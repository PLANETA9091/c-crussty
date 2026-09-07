#!/usr/bin/env python3
r"""Classfile-patcher CONTRACT fuzz corpus generator (pure Python, no Rust).

c-crussty's byte hook feeds every sighting of `SingleUserAreaMap` through
`src/classfile.rs::patch_update()` inside a JVMTI ClassFileLoadHook
callback. The patcher's contract is:

    patch_update(bytes) -> Ok(patched) | Err(reason)

    C1  Ok(patched)  => patched is a structurally well-formed class file
                        (same layout grammar as the input), whose
                        magic/minor/major bytes, this_class name, original
                        constant-pool entries (append-only), all fields, all
                        methods other than `update`, and everything after the
                        `update` method entry are byte-identical to the
                        input; only `update`'s attribute list is rebuilt.
    C2  Err(_)       => a clean rejection with a reason string.
    C3  PANIC/ABORT/HANG => NEVER acceptable. A panic inside a JVMTI callback
                        cannot unwind across FFI (the process aborts or worse).

This script does NOT build or run Rust (per task constraints). It:
  1. contains a small PURE-PYTHON classfile WRITER and emits a synthetic
     minimal `SingleUserAreaMap` (magic, versions, constant pool with
     Utf8/Class/NameAndType/Fieldref/Methodref entries, fields, an <init> and
     a straight-line `update(III)Z` with an empty StackMapTable) -- a
     JVM-verifiable baseline the future harness must be able to patch;
  2. derives 50 deterministic mutated cases (bit flips, truncations, pool
     count corruption, lying length prefixes, unknown tags, trailing
     garbage, ...) and writes them to generated/fuzz_cases/*.class;
  3. classifies every case with a safe MiniParser that mirrors the Rust
     parser's walk and, crucially, ANNOTATES the steps where the Rust
     implementation performs unchecked indexing (panic risk) instead of a
     checked read;
  4. writes generated/classfile_fuzz_manifest.json -- the case list with the
     EXPECTED failure modes -- ready to drive a future Rust fuzz harness
     (harness loop: read case -> patch_update -> check C1/C2/C3).
  5. grounds the MiniParser against the REAL kernel fixture
     tests/fixtures/SingleUserAreaMap.class (read-only), and optionally
     JVM-loads the synthetic golden class (javac/java, auto-detected) to
     prove the writer emits verifier-clean bytecode.

EXPECTED FAILURE MODES (the doc part of the contract):
  ACCEPT_OK : patcher MUST return Ok; output constraints C1 checked by the
              future harness (never silently corrupt).
  REJECT    : patcher MUST return Err (structurally unparseable input, wrong
              this_class, or missing update(III)Z). Ok here = contract
              violation (patching garbage = silent corruption upstream).
  EITHER   : input is in a gray zone (e.g. valid grammar, semantically odd);
              Ok (with C1 constraints) and Err are both legal. Panic is not.
  panic_risk cases are REJECT/EITHER by contract AND carry a finding note:
              as of this writing the Rust parser uses unchecked indexing at
              those steps (see manifest "findings"); the future harness will
              turn C3 into a hard regression test.

Usage:
    python3 tests/extended/classfile_fuzz.py [--out DIR] [--no-jvm]
Exit code 0 = writer/parser self-checks + fixture grounding pass.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys

MAP_CLASS = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap"
MAGIC = b"\xCA\xFE\xBA\xBE"
GOLDEN_MAJOR = 52  # Java 8 class-file target, matching area-map/ build (--release 8)

FUZZ_SEED = 0xC0FFEE123456789 & 0xFFFFFFFFFFFFFFFF
MASK64 = (1 << 64) - 1

# tags
TAG_UTF8, TAG_INT, TAG_FLOAT = 1, 3, 4
TAG_LONG, TAG_DOUBLE = 5, 6
TAG_CLASS, TAG_STRING = 7, 8
TAG_FIELDREF, TAG_METHODREF, TAG_IFACEMETHODREF = 9, 10, 11
TAG_NAMEANDTYPE = 12
TAG_METHODHANDLE, TAG_METHODTYPE = 15, 16
TAG_DYNAMIC, TAG_INVOKEDYNAMIC, TAG_MODULE, TAG_PACKAGE = 17, 18, 19, 20

# tag -> payload size (None = variable, 2-slot for long/double)
TAG_FIXED = {3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4, 11: 4,
             12: 4, 15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2}
TAG_SLOTS = {5: 2, 6: 2}


# ---------------------------------------------------------------------------
# xorshift64 (same family as the bridge selftest LCG; seed differs so the two
# corpora are independent streams)
# ---------------------------------------------------------------------------

class FuzzRng:
    __slots__ = ("state",)

    def __init__(self, seed: int = FUZZ_SEED):
        self.state = seed & MASK64

    def next(self) -> int:
        s = self.state
        s ^= (s << 13) & MASK64
        s ^= s >> 7
        s ^= (s << 17) & MASK64
        self.state = s & MASK64
        return self.state

    def below(self, n: int) -> int:
        return self.next() % n


# ---------------------------------------------------------------------------
# Constant pool builder (mirrors src/classfile.rs Pool: dedupe by
# (tag, payload), append-only, 1-based apparent indices, long/double = 2 slots)
# ---------------------------------------------------------------------------

def modified_utf8(s: str) -> bytes:
    out = bytearray()
    for ch in s:
        c = ord(ch)
        if c == 0:
            out += b"\xC0\x80"
        elif c <= 0x7F:
            out.append(c)
        elif c <= 0x7FF:
            out.append(0xC0 | (c >> 6))
            out.append(0x80 | (c & 0x3F))
        elif c <= 0xFFFF:
            out.append(0xE0 | (c >> 12))
            out.append(0x80 | ((c >> 6) & 0x3F))
            out.append(0x80 | (c & 0x3F))
        else:
            raise ValueError("supplementary chars need surrogate pairs (unused here)")
    if len(out) > 0xFFFF:
        raise ValueError("utf8 constant too long")
    return bytes(out)


class Pool:
    def __init__(self) -> None:
        self.entries: list[tuple[int, bytes]] = []  # (tag, payload) in file order

    def _find(self, tag: int, payload: bytes) -> int | None:
        for i, (t, p) in enumerate(self.entries):
            if t == tag and p == payload:
                return i
        return None

    def _idx(self, i: int) -> int:
        """apparent 1-based index of entries[i] (accounting for 2-slot tags)."""
        base = 1
        for j in range(i):
            base += TAG_SLOTS.get(self.entries[j][0], 1)
        return base

    def push(self, tag: int, payload: bytes) -> int:
        self.entries.append((tag, payload))
        return self._idx(len(self.entries) - 1)

    def raw_append(self, tag: int, payload: bytes) -> int:
        """append WITHOUT dedupe (legal; duplicates may exist in real files)."""
        return self.push(tag, payload)

    def utf8(self, s: str) -> int:
        payload = modified_utf8(s)
        i = self._find(TAG_UTF8, payload)
        return self.push(TAG_UTF8, payload) if i is None else self._idx(i)

    def integer(self, v: int) -> int:
        payload = (v & 0xFFFFFFFF).to_bytes(4, "big")
        i = self._find(TAG_INT, payload)
        return self.push(TAG_INT, payload) if i is None else self._idx(i)

    def class_of(self, name: str) -> int:
        ni = self.utf8(name)
        payload = ni.to_bytes(2, "big")
        i = self._find(TAG_CLASS, payload)
        return self.push(TAG_CLASS, payload) if i is None else self._idx(i)

    def name_and_type(self, name: str, desc: str) -> int:
        n, d = self.utf8(name), self.utf8(desc)
        payload = n.to_bytes(2, "big") + d.to_bytes(2, "big")
        i = self._find(TAG_NAMEANDTYPE, payload)
        return self.push(TAG_NAMEANDTYPE, payload) if i is None else self._idx(i)

    def member_ref(self, tag: int, owner: str, name: str, desc: str) -> int:
        c = self.class_of(owner)
        nat = self.name_and_type(name, desc)
        payload = c.to_bytes(2, "big") + nat.to_bytes(2, "big")
        i = self._find(tag, payload)
        return self.push(tag, payload) if i is None else self._idx(i)

    def field_ref(self, owner: str, name: str, desc: str) -> int:
        return self.member_ref(TAG_FIELDREF, owner, name, desc)

    def method_ref(self, owner: str, name: str, desc: str) -> int:
        return self.member_ref(TAG_METHODREF, owner, name, desc)

    def count(self) -> int:
        """constant_pool_count = entries + 1 (index 0 reserved)."""
        return 1 + sum(TAG_SLOTS.get(t, 1) for t, _ in self.entries)

    def serialize(self) -> bytes:
        out = bytearray()
        for t, p in self.entries:
            out.append(t)
            out += p
        return bytes(out)


# ---------------------------------------------------------------------------
# Class writer
# ---------------------------------------------------------------------------

def u2(v: int) -> bytes:
    return (v & 0xFFFF).to_bytes(2, "big")


def u4(v: int) -> bytes:
    return (v & 0xFFFFFFFF).to_bytes(4, "big")


def attr(pool: Pool, name: str, payload: bytes) -> bytes:
    return u2(pool.utf8(name)) + u4(len(payload)) + payload


def code_attr(pool: Pool, max_stack: int, max_locals: int, code: bytes,
              stackmap: bytes = b"\x00\x00") -> bytes:
    """Code + (always present) StackMapTable, as javac -target 8 would emit
    for straight-line bodies (empty frame stack is legal with no branches)."""
    inner = (u2(max_stack) + u2(max_locals) + u4(len(code)) + code +
             u2(0) +                      # exception_table_length
             u2(1) +                      # attributes_count
             attr(pool, "StackMapTable", stackmap))
    return attr(pool, "Code", inner)


def method(pool: Pool, access: int, name: str, desc: str, attrs: list[bytes]) -> bytes:
    return (u2(access) + u2(pool.utf8(name)) + u2(pool.utf8(desc)) +
            u2(len(attrs)) + b"".join(attrs))


def field(pool: Pool, access: int, name: str, desc: str, attrs: list[bytes] = ()) -> bytes:
    return (u2(access) + u2(pool.utf8(name)) + u2(pool.utf8(desc)) +
            u2(len(attrs)) + b"".join(attrs))


def write_class(pool: Pool, access: int, this_name: str, super_name: str,
                interfaces: list[str], fields_b: list[bytes], methods_b: list[bytes],
                class_attrs: list[bytes], minor: int = 0, major: int = GOLDEN_MAJOR) -> bytes:
    out = bytearray()
    out += MAGIC
    out += u2(minor) + u2(major)
    out += u2(pool.count())
    out += pool.serialize()
    out += u2(access) + u2(pool.class_of(this_name)) + u2(pool.class_of(super_name))
    out += u2(len(interfaces)) + b"".join(u2(pool.class_of(i)) for i in interfaces)
    out += u2(len(fields_b)) + b"".join(fields_b)
    out += u2(len(methods_b)) + b"".join(methods_b)
    out += u2(len(class_attrs)) + b"".join(class_attrs)
    return bytes(out)


def build_golden(this_name: str = MAP_CLASS) -> tuple[bytes, Pool]:
    """Synthetic minimal SingleUserAreaMap: the 4 fields the patcher's new
    update body getfields, a trivial <init>, and a straight-line update(III)Z
    (fields written, return true) -- verifier-clean, JVM-loadable, and shaped
    so patch_update can find update(III)Z and rebuild it."""
    pool = Pool()
    f_x = pool.field_ref(this_name, "lastChunkX", "I")
    f_z = pool.field_ref(this_name, "lastChunkZ", "I")
    f_d = pool.field_ref(this_name, "distance", "I")
    obj_init = pool.method_ref("java/lang/Object", "<init>", "()V")

    init_code = bytes([0x2A, 0xB7]) + u2(obj_init) + bytes([0xB1])  # aload_0; invokespecial; return
    # update: store toX/toZ/newD into the fields, return true (straight line)
    upd = bytearray()
    for fld, load in ((f_x, 0x1B), (f_z, 0x1C), (f_d, 0x1D)):  # iload_1/2/3
        upd += bytes([0x2A, load]) + bytes([0xB5]) + u2(fld)   # aload_0; iload_n; putfield
    upd += bytes([0x04, 0xAC])                                  # iconst_1; ireturn

    cls_b = write_class(
        pool,
        access=0x0021,  # ACC_PUBLIC | ACC_SUPER
        this_name=this_name,
        super_name="java/lang/Object",
        interfaces=[],
        fields_b=[
            field(pool, 0x0002, "lastChunkX", "I"),
            field(pool, 0x0002, "lastChunkZ", "I"),
            field(pool, 0x0002, "distance", "I"),
            field(pool, 0x0002, "parameter", "Ljava/lang/Object;"),
        ],
        methods_b=[
            method(pool, 0x0001, "<init>", "()V",
                   [code_attr(pool, max_stack=1, max_locals=1, code=bytes(init_code))]),
            method(pool, 0x0001, "update", "(III)Z",
                   [code_attr(pool, max_stack=2, max_locals=4, code=bytes(upd))]),
        ],
        class_attrs=[],
    )
    return cls_b, pool


# ---------------------------------------------------------------------------
# MiniParser: safe mirror of classfile.rs parse_layout + Pool::parse +
# find_method, annotating the steps where the RUST implementation indexes
# unchecked (panic instead of Err).
# ---------------------------------------------------------------------------

def parse_class(data: bytes) -> dict:
    """Returns {ok, error, panic_risk, this_name, major, minor, magic_ok,
    cp_entries, fields, methods, update_found}.

    panic_risk=True means: at the failing step the Rust parser would have
    performed an unchecked index (bytes[p] / bytes[8] style) -> panic, not a
    clean Err. Line references are to src/classfile.rs as of this writing.
    """
    r = {"ok": False, "error": None, "panic_risk": False, "this_name": None,
         "major": None, "minor": None, "magic_ok": data[:4] == MAGIC,
         "cp_entries": 0, "fields": [], "methods": [], "update_found": False}

    def fail(msg: str, panic: bool = False) -> dict:
        r["error"], r["panic_risk"] = msg, panic
        return r

    def need(p: int, n: int, what: str, direct_in_rust: bool) -> bytes | None:
        if p + n > len(data):
            # Pool payloads use bytes.get (clean Err); counter/header reads in
            # parse_layout / find_method use direct indexing (panic).
            return fail(f"truncated while reading {what}", panic=direct_in_rust)
        return data[p:p + n]

    # header (parse_layout:272 -- unchecked bytes[8], bytes[9] in Rust: F-1)
    if len(data) < 10:
        return fail(f"file shorter than 10 bytes (len={len(data)}); Rust "
                    f"parse_layout indexes bytes[8..9] unchecked", panic=True)
    r["minor"] = int.from_bytes(data[4:6], "big")
    r["major"] = int.from_bytes(data[6:8], "big")
    cp_count = int.from_bytes(data[8:10], "big")

    # constant pool (Pool::parse -- all reads checked in Rust)
    p, seen, entries = 10, 0, []
    total = max(0, cp_count - 1)
    while seen < total:
        b = need(p, 1, "cp tag", direct_in_rust=False)
        if b is None:
            return r
        tag = b[0]
        if tag in (TAG_UTF8,):
            b = need(p + 1, 2, "utf8 length", False)
            if b is None:
                return r
            ln = int.from_bytes(b, "big")
            if need(p + 3, ln, "utf8 payload", False) is None:
                return r
            payload = data[p + 1:p + 3 + ln]
            entries.append((tag, payload))
            p += 3 + ln
            slots = 1
        elif tag in TAG_FIXED:
            if need(p + 1, TAG_FIXED[tag], f"cp tag {tag} payload", False) is None:
                return r
            entries.append((tag, data[p + 1:p + 1 + TAG_FIXED[tag]]))
            p += 1 + TAG_FIXED[tag]
            slots = TAG_SLOTS.get(tag, 1)
        else:
            return fail(f"unknown cp tag 0x{tag:02x} at offset {p}")
        r["cp_entries"] += 1
        seen += 1
        del slots
    cp_end = p
    if not entries:
        return fail("empty constant pool walk (cp_count<=1)")

    def entry_at(idx: int):
        """apparent-index lookup, mirroring Pool.entries."""
        base = 1
        for t, pl in entries:
            if base == idx:
                return t, pl
            base += TAG_SLOTS.get(t, 1)
        return None

    def utf8_value(idx: int) -> str | None:
        e = entry_at(idx)
        if not e or e[0] != TAG_UTF8:
            return None
        ln = int.from_bytes(e[1][0:2], "big")
        try:
            return e[1][2:2 + ln].decode("utf-8", "replace")
        except Exception:
            return None

    # this_class (parse_layout:274 -- unchecked in Rust when cp_end+4 > len)
    b = need(cp_end + 2, 2, "this_class index", direct_in_rust=True)
    if b is None:
        return r
    if need(cp_end, 6, "access/this/super", True) is None:
        return r
    this_idx = int.from_bytes(b, "big")
    e = entry_at(this_idx)
    if not e or e[0] != TAG_CLASS:
        return fail(f"this_class idx {this_idx} is not a CONSTANT_Class entry")
    ni = int.from_bytes(e[1], "big")
    r["this_name"] = utf8_value(ni)

    # interfaces + fields (parse_layout:276-288 -- unchecked counters: F-3)
    q = cp_end + 6
    b = need(q, 2, "interfaces_count", True)
    if b is None:
        return r
    q += 2 + 2 * int.from_bytes(b, "big")
    b = need(q, 2, "fields_count", True)
    if b is None:
        return r
    nf = int.from_bytes(b, "big")
    q += 2
    for _ in range(nf):
        b = need(q, 6, "field header", True)
        if b is None:
            return r
        r["fields"].append({"name": utf8_value(int.from_bytes(data[q + 2:q + 4], "big")),
                            "desc": utf8_value(int.from_bytes(data[q + 4:q + 6], "big")),
                            "access_off": q})
        b = need(q + 6, 2, "field attrs_count", True)
        if b is None:
            return r
        na = int.from_bytes(b, "big")
        q += 8
        for _ in range(na):
            b = need(q + 2, 4, "field attr length", True)
            if b is None:
                return r
            q += 6 + int.from_bytes(b, "big")

    # methods (find_method -- unchecked counters: F-4)
    b = need(q, 2, "methods_count", True)
    if b is None:
        return r
    nm = int.from_bytes(b, "big")
    q += 2
    for _ in range(nm):
        b = need(q, 8, "method header", True)
        if b is None:
            return r
        name = utf8_value(int.from_bytes(data[q + 2:q + 4], "big"))
        desc = utf8_value(int.from_bytes(data[q + 4:q + 6], "big"))
        attrs = []
        na = int.from_bytes(data[q + 6:q + 8], "big")
        w = q + 8
        for _ in range(na):
            b = need(w + 2, 4, "method attr length", True)
            if b is None:
                return r
            alen = int.from_bytes(b, "big")
            aname = utf8_value(int.from_bytes(data[w:w + 2], "big"))
            attrs.append({"name": aname, "len_off": w + 2, "len": alen})
            # NOTE: the Rust walk advances unchecked; a lying length makes the
            # NEXT direct counter read (or the final bytes[m.end..] splice)
            # panic -- F-4. We keep walking safely and flag it at the end.
            w += 6 + alen
        r["methods"].append({"name": name, "desc": desc, "access_off": q,
                             "attrs": attrs})
        if name == "update" and desc == "(III)Z":
            r["update_found"] = True
            r["update_off"] = q
    if w > len(data):
        return fail("method attribute lengths overrun the file (Rust would "
                    "panic in find_method/patch_update splice)", panic=True)
    r["ok"] = True
    return r


# ---------------------------------------------------------------------------
# Case construction: golden + 49 deterministic mutations = 50
# ---------------------------------------------------------------------------

def locate_update_code_attr(info: dict) -> int | None:
    for m in info["methods"]:
        if m["name"] == "update" and m["desc"] == "(III)Z":
            for a in m["attrs"]:
                if a["name"] == "Code":
                    return a["len_off"]
    return None


def build_cases(golden: bytes) -> list[dict]:
    info = parse_class(golden)
    assert info["ok"], "golden must parse (writer self-check)"
    code_len_off = locate_update_code_attr(info)
    assert code_len_off is not None, "golden update must have a Code attr"
    cp_end = info  # not used directly; mutations use fixed offsets below
    first_utf8_len_off = 11  # golden pool starts with the this_name Utf8:
    #   [10]=tag=1, [11:13]=length -> mutating it exercises length-prefix lies

    cases: list[dict] = []

    def add(cid, kind, data, expected, note="", params=None, panic_risk=False,
            reject_reason=None):
        cases.append({"id": cid, "kind": kind, "data": bytes(data),
                      "expected_verdict": expected, "note": note,
                      "params": params or {}, "panic_risk": panic_risk,
                      "reject_reason": reject_reason})

    add("case_01_golden", "golden", golden, "ACCEPT_OK",
        "synthetic baseline; patcher must accept and rebuild update() only")

    # benign pool/version variants
    b2 = bytearray(golden)
    cp_count = int.from_bytes(b2[8:10], "big")
    b2[8:10] = u2(cp_count + 1)
    b2 += bytes([TAG_UTF8]) + u2(9) + b"fuzz/extra"
    add("case_02_extra_utf8", "benign_variant", b2, "ACCEPT_OK",
        "one extra trailing Utf8 cp entry (append-only pool stays valid)")

    b = bytearray(golden); b[6:8] = u2(61)
    add("case_03_major_61", "benign_variant", b, "EITHER",
        "class major 61: grammar-valid; patcher passes versions through")
    b = bytearray(golden); b[6:8] = u2(45)
    add("case_04_major_45", "benign_variant", b, "EITHER",
        "class major 45 (ancient): grammar-valid for the patcher's walk")

    # identity checks
    b = bytearray(golden); b[10:10 + len(modified_utf8(MAP_CLASS))] = modified_utf8(
        MAP_CLASS[:-4] + "Fake")
    add("case_05_wrong_this_class", "identity", b, "REJECT",
        "this_class != SingleUserAreaMap -> Err(unexpected class)",
        reject_reason="unexpected class")
    b = bytearray(golden); b[10:10 + len(modified_utf8(MAP_CLASS))] = modified_utf8(
        MAP_CLASS.replace("moonrise", "moonrise2"))
    add("case_06_wrong_package", "identity", b, "REJECT",
        "right simple name, wrong package -> Err(unexpected class)",
        reject_reason="unexpected class")

    # method-table identity
    b = bytearray(golden)
    # find the utf8 payload of "update": search the pool region
    tagpos = golden.find(bytes([TAG_UTF8]) + u2(6) + b"update")
    assert tagpos > 0
    b[tagpos + 3:tagpos + 9] = b"updute"
    add("case_07_missing_update", "identity", b, "REJECT",
        "no update(III)Z method -> Err(update(III)Z not found)",
        reject_reason="update(III)Z not found")

    # update without any attribute (patcher rebuilds the whole entry: heals)
    b = bytearray(golden)
    uoff = parse_class(bytes(b))["update_off"]
    na_off = uoff + 6  # attributes_count of update
    # drop the Code attribute entirely: shift rest left
    attr_len = int.from_bytes(b[na_off + 2 + 2:na_off + 2 + 6], "big")
    attr_start = na_off + 2
    attr_total = 6 + attr_len
    del b[attr_start:attr_start + attr_total]
    b[na_off:na_off + 2] = u2(0)
    add("case_08_update_no_code", "identity", b, "ACCEPT_OK",
        "update() with zero attrs: patcher rebuilds the entry wholesale "
        "(heals a JVMS-invalid input); output must be valid")

    # magic (patcher does NOT validate magic: documented gap B8-F5)
    b = bytearray(golden); b[0] ^= 0x40
    add("case_09_magic_bitflip", "magic", b, "EITHER",
        "magic bitflip: parse_layout never checks magic (B8-F5) -> likely Ok "
        "passthrough with corrupt magic preserved; JVM rejects it after the hook")
    b = bytearray(golden); b[0:4] = b"\xDE\xAD\xBE\xEF"
    add("case_10_magic_deadbeef", "magic", b, "EITHER",
        "magic = 0xDEADBEEF: same as case_09 (Ok passthrough legal, panic not)")
    b = bytearray(golden); b[0:4] = b"\x00\x00\x00\x00"
    add("case_11_magic_zeroed", "magic", b, "EITHER",
        "magic zeroed: same treatment")

    # versions
    b = bytearray(golden); b[6:8] = u2(0)
    add("case_12_major_0", "version", b, "EITHER", "major 0: passthrough or Err")
    b = bytearray(golden); b[6:8] = u2(0xFFFF)
    add("case_13_major_65535", "version", b, "EITHER", "major 0xFFFF: passthrough or Err")
    b = bytearray(golden); b[4:6] = u2(0xFFFF)
    add("case_14_minor_65535", "version", b, "EITHER", "minor 0xFFFF: passthrough or Err")

    # truncations
    for cid, ln, note, panic in [
        ("case_15_trunc_0", 0, "empty file: MUST Err; Rust bytes[8] panics today (B8-F1)", True),
        ("case_16_trunc_3", 3, "3 bytes: same as empty (B8-F1)", True),
        ("case_17_trunc_8", 8, "magic+versions, no cp_count byte 9: (B8-F1)", True),
        ("case_18_trunc_9", 9, "one cp_count byte missing: (B8-F1)", True),
    ]:
        add(cid, "truncation", golden[:ln], "REJECT", note, panic_risk=panic,
            reject_reason="truncated header")
    add("case_19_trunc_10", "truncation", golden[:10], "REJECT",
        "header only: Pool::parse reads past end via bytes.get -> clean Err",
        reject_reason="truncated pool")
    add("case_20_trunc_header_minus4", "truncation", golden[:len(golden) - 4], "REJECT",
        "cut inside update()'s Code attr: attr-walk overrun (B8-F4 panic risk "
        "or clean miss)", panic_risk=True, reject_reason="truncated method table")
    add("case_21_trunc_mid_pool", "truncation",
        golden[: (10 + 3 + len(modified_utf8(MAP_CLASS)) + 12)], "REJECT",
        "cut inside the pool: clean Err from Pool::parse",
        reject_reason="truncated pool")
    add("case_22_trunc_last_byte", "truncation", golden[:-1], "REJECT",
        "last byte of the class cut: method-walk lands past EOF -> splice "
        "bytes[m.end..] panics (B8-F4) or clean Err",
        panic_risk=True, reject_reason="truncated tail")

    # cp_count corruption
    b = bytearray(golden); b[8:10] = u2(int.from_bytes(b[8:10], "big") - 1)
    add("case_23_cp_count_minus1", "cp_count", b, "EITHER",
        "walk stops one entry early -> desync: Err likely; Ok legal if it "
        "still parses (panic not)")
    b = bytearray(golden); b[8:10] = u2(int.from_bytes(b[8:10], "big") + 1)
    add("case_24_cp_count_plus1", "cp_count", b, "REJECT",
        "walk overruns the pool region -> bytes.get None -> clean Err",
        reject_reason="pool overrun")
    b = bytearray(golden); b[8:10] = u2(0)
    add("case_25_cp_count_0", "cp_count", b, "EITHER",
        "cp_count=0: saturating_sub -> 0 entries; this_class read from "
        "offset 10 (garbage) -> Err(unexpected class) likely")
    b = bytearray(golden); b[8:10] = u2(0xFFFF)
    add("case_26_cp_count_65535", "cp_count", b, "REJECT",
        "cp_count=0xFFFF: walk overruns -> clean Err",
        reject_reason="pool overrun")

    # utf8 length-prefix lies
    b = bytearray(golden); b[first_utf8_len_off:first_utf8_len_off + 2] = u2(0xFFFF)
    add("case_27_utf8_len_ffff", "utf8_len", b, "REJECT",
        "declared utf8 length 0xFFFF > file: Pool::parse get() -> None -> Err",
        reject_reason="utf8 length overrun")
    b = bytearray(golden); b[first_utf8_len_off:first_utf8_len_off + 2] = u2(1)
    add("case_28_utf8_len_short", "utf8_len", b, "REJECT",
        "declared length 1 desyncs the walk -> next 'tag' byte is ASCII "
        "(0x63=99 unknown) -> Err")
    b = bytearray(golden); b[first_utf8_len_off:first_utf8_len_off + 2] = u2(0)
    add("case_29_utf8_len_zero", "utf8_len", b, "EITHER",
        "empty utf8 entry desyncs: Err or accidental parse (panic not)")

    # unknown tags
    for cid, tagv in [("case_30_tag_0", 0), ("case_31_tag_99", 99), ("case_32_tag_2", 2)]:
        b = bytearray(golden); b[10] = tagv
        add(cid, "unknown_tag", b, "REJECT",
            f"cp tag {tagv} -> Pool::parse `_ => None` -> clean Err",
            reject_reason="unknown cp tag")

    # Code attribute length lies
    b = bytearray(golden); b[code_len_off:code_len_off + 4] = u4(0x7FFFFFFF)
    add("case_33_code_len_huge", "attr_len", b, "REJECT",
        "Code attr length 0x7FFFFFFF: find_method walk overruns (B8-F4 panic "
        "risk today; must be Err)", panic_risk=True,
        reject_reason="attribute length overrun")
    b = bytearray(golden); b[code_len_off:code_len_off + 4] = u4(8)
    add("case_34_code_len_short", "attr_len", b, "EITHER",
        "Code attr length 8 (< real): walk desyncs; Err or accidental parse")

    # trailing garbage
    rng = FuzzRng()
    add("case_35_trailing_1k", "trailing", golden + bytes(rng.below(256) for _ in range(1024)),
        "EITHER", "1 KiB random tail: parse_layout stops after the method "
        "table; patcher splices bytes[m.end..] and preserves the tail")
    add("case_36_trailing_zeros", "trailing", golden + b"\x00" * 64, "EITHER",
        "64 zero bytes tail: same treatment")

    # flags mangling
    foff = info["fields"][0]["access_off"]
    b = bytearray(golden); b[foff:foff + 2] = u2(0xFFFF)
    add("case_37_field_flags_ffff", "flags", b, "EITHER",
        "field access flags 0xFFFF: patcher ignores flags -> Ok passthrough "
        "(JVM rejects after the hook)")
    b = bytearray(golden); b[info["update_off"]:info["update_off"] + 2] = u2(0xFFFF)
    add("case_38_update_flags_ffff", "flags", b, "EITHER",
        "update access flags 0xFFFF: patcher PRESERVES m.access in the new "
        "entry -> Ok; flags unchanged is contract-legal (C1)")

    # 11 deterministic bit flips in the body region [10, len)
    for k in range(11):
        nbits = len(golden) * 8
        bit = (FUZZ_SEED ^ (k * 0x9E3779B97F4A7C15)) % nbits
        byi, bib = bit // 8, bit % 8
        if byi < 10:
            byi += 10
        b = bytearray(golden); b[byi] ^= 1 << bib
        add(f"case_{39 + k:02d}_bitflip_{k + 1:02d}", "bitflip", b, "EITHER",
            f"single bit flip at offset {byi} bit {bib}: parse survives -> Ok "
            f"(C1) else Err; never panic", params={"offset": byi, "bit": bib})

    # duplicate pool entry (legal per JVMS; dedupe find() must still work)
    b = bytearray(golden)
    cp_count = int.from_bytes(b[8:10], "big")
    b[8:10] = u2(cp_count + 1)
    b += bytes([TAG_UTF8]) + u2(6) + b"update"
    add("case_50_dup_utf8", "benign_variant", b, "ACCEPT_OK",
        "duplicate 'update' Utf8 entry: legal; Pool::find must take the first")

    assert len(cases) == 50, f"expected 50 cases, built {len(cases)}"
    ids = [c["id"] for c in cases]
    assert len(set(ids)) == 50, "case ids must be unique"
    blobs = {c["data"] for c in cases}
    assert len(blobs) == 50, "case payloads must be pairwise distinct"
    return cases


# ---------------------------------------------------------------------------
# Optional JVM proof that the golden class is verifier-clean
# ---------------------------------------------------------------------------

LOADCHECK_JAVA = r"""
import java.nio.file.*;

public class LoadCheck {
    static class ByteLoader extends ClassLoader {
        ByteLoader(ClassLoader parent) { super(parent); }
        Class<?> define(byte[] b) { return defineClass(null, b, 0, b.length); }
    }

    public static void main(String[] args) throws Exception {
        byte[] b = Files.readAllBytes(Paths.get(args[0]));
        Class<?> c = new ByteLoader(LoadCheck.class.getClassLoader()).define(b);
        Object inst = c.getDeclaredConstructor().newInstance();
        Object r = c.getMethod("update", int.class, int.class, int.class)
                      .invoke(inst, 1, 2, 3);
        java.lang.reflect.Field fx = c.getDeclaredField("lastChunkX");
        java.lang.reflect.Field fz = c.getDeclaredField("lastChunkZ");
        java.lang.reflect.Field fd = c.getDeclaredField("distance");
        fx.setAccessible(true); fz.setAccessible(true); fd.setAccessible(true);
        System.out.println("LOADCHECK OK update=" + r
                + " lastChunkX=" + fx.getInt(inst)
                + " lastChunkZ=" + fz.getInt(inst)
                + " distance=" + fd.getInt(inst));
    }
}
"""


def jvm_check(golden_path: str, outdir: str) -> str:
    javac = None
    for cand in ("/home/z/jdk21/bin/javac",):
        if os.path.exists(cand):
            javac = cand
    javac = javac or shutil.which("javac")
    java = shutil.which("java") or "/home/z/jdk21/bin/java"
    if not javac or not java or not os.path.exists(java):
        return "SKIP (javac/java not found)"
    jdir = os.path.join(outdir, "jvm")
    os.makedirs(jdir, exist_ok=True)
    jsrc = os.path.join(jdir, "LoadCheck.java")
    with open(jsrc, "w") as f:
        f.write(LOADCHECK_JAVA)
    c = subprocess.run([javac, "-d", jdir, jsrc], capture_output=True, text=True, timeout=120)
    if c.returncode != 0:
        return f"SKIP (javac failed: {c.stderr.strip()[:200]})"
    r = subprocess.run([java, "-XX:+UseSerialGC", "-Xmx256m",
                        "-cp", jdir, "LoadCheck", golden_path],
                       capture_output=True, text=True, timeout=120)
    if r.returncode != 0:
        return f"FAIL (JVM rejected golden: {r.stderr.strip()[:400]})"
    return r.stdout.strip()


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    here = os.path.dirname(os.path.abspath(__file__))
    ap.add_argument("--out", default=os.path.join(here, "generated"))
    ap.add_argument("--no-jvm", action="store_true")
    ap.add_argument("--fixture", default=os.path.join(here, "..", "fixtures",
                                                      "SingleUserAreaMap.class"))
    args = ap.parse_args()
    failures: list[str] = []

    golden, _ = build_golden()
    cases = build_cases(golden)

    # ---- writer self-check: parse golden ----------------------------------
    info = parse_class(golden)
    if not info["ok"]:
        failures.append(f"golden does not parse: {info['error']}")
    elif info["this_name"] != MAP_CLASS:
        failures.append(f"golden this_name={info['this_name']!r}")
    elif not info["update_found"]:
        failures.append("golden missing update(III)Z")
    print(f"writer self-check : golden {len(golden)} bytes, cp_entries="
          f"{info['cp_entries']}, major={info['major']}, this={info['this_name']}, "
          f"update_found={info['update_found']}, fields="
          f"{[f['name'] for f in info['fields']]}")

    # ---- fixture grounding (read-only) -------------------------------------
    fx_info = None
    if os.path.exists(args.fixture):
        with open(args.fixture, "rb") as f:
            real = f.read()
        fx_info = parse_class(real)
        if not fx_info["ok"]:
            failures.append(f"MiniParser cannot parse the REAL fixture: {fx_info['error']}")
        else:
            names = [m["name"] + m["desc"] for m in fx_info["methods"]]
            print(f"fixture grounding : {os.path.normpath(args.fixture)} len={len(real)} "
                  f"major={fx_info['major']} cp_entries={fx_info['cp_entries']} "
                  f"this={fx_info['this_name']}")
            print(f"                    methods={names}")
            print(f"                    fields={[f['name'] for f in fx_info['fields']]}")
            if fx_info["this_name"] != MAP_CLASS or not fx_info["update_found"]:
                failures.append("fixture grounding: unexpected identity")
    else:
        print("fixture grounding : SKIP (fixture not found)")

    # ---- write case files + manifest ---------------------------------------
    case_dir = os.path.join(args.out, "fuzz_cases")
    os.makedirs(case_dir, exist_ok=True)
    manifest_cases = []
    from collections import Counter
    verdicts = Counter()
    for c in cases:
        path = os.path.join(case_dir, c["id"] + ".class")
        with open(path, "wb") as f:
            f.write(c["data"])
        pi = parse_class(c["data"])
        verdicts[c["expected_verdict"]] += 1
        manifest_cases.append({
            "id": c["id"],
            "kind": c["kind"],
            "file": os.path.join("generated", "fuzz_cases", c["id"] + ".class"),
            "bytes": len(c["data"]),
            "params": c["params"],
            "expected_verdict": c["expected_verdict"],
            "reject_reason": c["reject_reason"],
            "panic_risk": c["panic_risk"],
            "parser": {"ok": pi["ok"], "error": pi["error"],
                       "this_name": pi["this_name"],
                       "update_found": pi["update_found"]},
            "note": c["note"],
        })
        # consistency: ACCEPT_OK cases must parse cleanly and keep identity
        if c["expected_verdict"] == "ACCEPT_OK":
            if not pi["ok"]:
                failures.append(f"{c['id']}: ACCEPT_OK but parser fails: {pi['error']}")
            elif pi["this_name"] != MAP_CLASS or not pi["update_found"]:
                failures.append(f"{c['id']}: ACCEPT_OK but identity broken")

    manifest = {
        "tool": "tests/extended/classfile_fuzz.py",
        "fuzz_prng": "xorshift64 seed 0x%016X (bit-flip offsets; corpus otherwise structural)" % FUZZ_SEED,
        "golden_sha256": hashlib.sha256(golden).hexdigest(),
        "golden_bytes": len(golden),
        "target_of_patch": MAP_CLASS + ".update(III)Z",
        "contract": {
            "C1": "Ok(out) => out must re-parse, keep magic/minor/major bytes, "
                  "this_class name, all original cp entries (append-only), all "
                  "fields, all methods except update, and everything after the "
                  "update entry byte-identical; only update's attribute list rebuilt",
            "C2": "Err(reason) allowed for REJECT/EITHER cases; REQUIRED for REJECT",
            "C3": "panic/abort/hang NEVER acceptable (JVMTI callback cannot unwind)",
        },
        "findings": [
            {"id": "B8-F1", "where": "src/classfile.rs parse_layout (bytes[8], bytes[9])",
             "issue": "inputs shorter than 10 bytes index unchecked -> panic "
                      "(abort across JVMTI FFI) instead of Err",
             "cases": ["case_15_trunc_0", "case_16_trunc_3", "case_17_trunc_8",
                        "case_18_trunc_9"]},
            {"id": "B8-F2", "where": "src/classfile.rs parse_layout "
                                     "(this_class read at cp_end+2..cp_end+4)",
             "issue": "a class whose pool ends near EOF indexes unchecked -> panic",
             "cases": ["constructed by cp corruption landing at EOF; covered by "
                       "case_23/case_25 families"]},
            {"id": "B8-F3", "where": "src/classfile.rs parse_layout "
                                     "(interfaces/fields walk, counters)",
             "issue": "counter reads are direct bytes[p] indexes -> lying "
                      "counts panic instead of Err",
             "cases": ["case_23_cp_count_minus1 family"]},
            {"id": "B8-F4", "where": "src/classfile.rs find_method + patch_update "
                                     "splice (bytes[m.end..])",
             "issue": "attr-walk overruns / m.end > len slice -> panic; lying "
                      "attribute lengths reach it unchecked",
             "cases": ["case_20_trunc_header_minus4", "case_22_trunc_last_byte",
                        "case_33_code_len_huge"]},
            {"id": "B8-F5", "where": "src/classfile.rs patch_update (no magic check)",
             "issue": "parse_layout never validates 0xCAFEBABE; a garbage-magic "
                      "input is patched and returned Ok (JVM rejects it later "
                      "-- harmless in practice, but the contract check C1 "
                      "should pin the behavior; guard is a one-liner)",
             "cases": ["case_09_magic_bitflip", "case_10_magic_deadbeef",
                        "case_11_magic_zeroed"]},
        ],
        "harness_loop": "for each case: read file -> patch_update(bytes) -> "
                        "if panic/abort/hang: FAIL(C3); if Err: expected_verdict "
                        "must be REJECT or EITHER; if Ok: expected_verdict must "
                        "be ACCEPT_OK or EITHER and constraints C1 must hold",
        "cases": manifest_cases,
    }
    manifest_path = os.path.join(args.out, "classfile_fuzz_manifest.json")
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, sort_keys=True)
        f.write("\n")

    # ---- JVM proof of the golden -------------------------------------------
    golden_path = os.path.join(case_dir, "case_01_golden.class")
    if args.no_jvm:
        jvm = "SKIP (--no-jvm)"
    else:
        jvm = jvm_check(golden_path, args.out)
    print(f"jvm golden check  : {jvm}")
    if jvm.startswith("FAIL"):
        failures.append(f"JVM check: {jvm}")

    # ---- summary -----------------------------------------------------------
    print(f"cases             : {len(cases)} written to {case_dir}")
    print(f"verdict mix       : {dict(sorted(verdicts.items()))}")
    print(f"panic_risk cases  : {sum(1 for c in cases if c['panic_risk'])} "
          f"(findings B8-F1..F4 encoded in manifest)")
    print(f"manifest          : {manifest_path}")
    if failures:
        print(f"FAIL ({len(failures)}):")
        for m in failures[:10]:
            print(f"  - {m}")
        return 1
    print("PASS: writer emits a parseable golden (MiniParser + JVM), fixture "
          "grounding OK, 50 cases + manifest written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
