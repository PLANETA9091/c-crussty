#!/usr/bin/env python3
"""Precompute / pre-validate c-crussty hot-patch inputs at BUILD time.

Today both hot patches are computed at ACTIVATION time inside the live server:

  * src/improved_noise.rs  — ASM `ReplaceBody` rewrite of
    `ImprovedNoise.noise(DDDDD)D` (bridge: ImprovedNoiseNativeOps), computed on
    a background worker from bytes captured by the JVMTI byte hook.
  * src/area_map.rs + src/classfile.rs — deterministic byte-splice of
    `SingleUserAreaMap.update(III)Z` into an 82-byte
    `invokestatic SingleUserAreaMapOps.run(...)` body, computed inside the
    class-file hook callback.

Every failure of that pipeline currently surfaces as a log line on a live
server (or worse: a dormant hook nobody noticed — see the major-69 bridge
incident in the worklog). This script moves what CAN move to build time:

  * extracts the target kernel classes from a Paper/Purpur jar (handles the
    paperclip launcher format, producing versions/<v>/<server>.jar itself),
  * parses them with a minimal pure-stdlib JVM classfile reader,
  * locates the target methods and dumps their bytecode + structure,
  * validates the exact preconditions the Rust patchers assume
    (method present, fields present, class-version guard rule),
  * computes BOTH patch bodies at build time when possible:
      - area_map: deterministic Python port of src/classfile.rs::patch_update
        (append-only cp splice, byte-identical to the Rust output),
      - improved_noise: the real ASM REPLACE_BODY rewrite, executed through
        the repo's own SdkAsmHelper + asm jar on a plain JDK (no kernel
        loader needed), spec serialized in cplug-sdk/src/asm.rs wire format,
  * emits machine-readable artifacts (patches/*.json + manifest.json +
    patched/*.class) that CI can gate on and the runtime can consume.

stdlib only. No third-party imports. A JDK is used (java/javac via PATH,
JAVA_HOME or /home/z/jdk21) only for the optional noise precompute.

Usage:
    python3 scripts/precompute_patches.py --self-test
    python3 scripts/precompute_patches.py --kernel-jar purpur-1.21.10.jar \
            --out reports/precompute_out [--max-class-major 65] [--strict]

Exit codes: 0 = ok (warnings allowed), 1 = usage/runtime error,
            2 = validation failure (bad class, missing method/field, ...).
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import struct
import sys
import zipfile
from datetime import datetime, timezone

TOOL = "precompute_patches.py"
TOOL_VERSION = "1.0.0"

# ---------------------------------------------------------------------------
# Targets — mirrors the constants in src/improved_noise.rs / src/classfile.rs
# ---------------------------------------------------------------------------

NOISE_CLASS = "net/minecraft/world/level/levelgen/synth/ImprovedNoise"
NOISE_METHOD = ("noise", "(DDDDD)D")
# ReplaceBody spec exactly as built in improved_noise.rs::activate().
# Args must match bridge_desc parameter types one-to-one.
NOISE_REPLACE_BODY = {
    "method_name": "noise",
    "method_desc": "(DDDDD)D",
    "bridge_owner": "net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps",
    "bridge_name": "noise",
    "bridge_desc":
        "(Lnet/minecraft/world/level/levelgen/synth/ImprovedNoise;[BDDDDDDDD)D",
    "args": [
        {"kind": "local", "slot": 0, "ty": "L"},            # this (handle key)
        {"kind": "this_field", "name": "p", "desc": "[B"},  # this.p
        {"kind": "this_field", "name": "xo", "desc": "D"},  # this.xo
        {"kind": "this_field", "name": "yo", "desc": "D"},  # this.yo
        {"kind": "this_field", "name": "zo", "desc": "D"},  # this.zo
        {"kind": "local", "slot": 1, "ty": "D"},            # x
        {"kind": "local", "slot": 3, "ty": "D"},            # y
        {"kind": "local", "slot": 5, "ty": "D"},            # z
        {"kind": "local", "slot": 7, "ty": "D"},            # yScale
        {"kind": "local", "slot": 9, "ty": "D"},            # yMax
    ],
}
# Fields the rewritten noise() body reads from `this`.
NOISE_REQUIRED_FIELDS = {"p": "[B", "xo": "D", "yo": "D", "zo": "D"}

MAP_CLASS = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap"
MAP_METHOD = ("update", "(III)Z")
# Fields the spliced update() body touches (src/classfile.rs).
MAP_REQUIRED_FIELDS = {
    "lastChunkX": "I",
    "lastChunkZ": "I",
    "distance": "I",
    "parameter": "Ljava/lang/Object;",
}
OPS_CLASS = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps"

TARGETS = [
    {
        "key": "improved_noise",
        "class_internal": NOISE_CLASS,
        "simple_name": "ImprovedNoise",
        "method": NOISE_METHOD,
        "required_fields": NOISE_REQUIRED_FIELDS,
        "guard": "noise",          # class-version guard rule from improved_noise.rs
    },
    {
        "key": "area_map",
        "class_internal": MAP_CLASS,
        "simple_name": "SingleUserAreaMap",
        "method": MAP_METHOD,
        "required_fields": MAP_REQUIRED_FIELDS,
        "guard": "area_map",       # same ceiling rule; patch spliced here
    },
]

# Java N <-> class-file major (JVM reads java.class.version "65.0" on Java 21).
JAVA_MAJOR = {8: 52, 11: 55, 17: 61, 21: 65, 22: 66, 23: 67, 24: 68, 25: 69}
# improved_noise.rs guard: the embedded bridge is compiled with `--release 8`,
# i.e. class-file major 52. If the bundled bridge is newer than the running
# JVM's max major, define_class dies with UnsupportedClassVersionError.
NOISE_BRIDGE_MAX_MAJOR = 52
NOISE_BRIDGE_RELPATH = os.path.join(
    "noise", "build", "net", "minecraft", "world", "level", "levelgen",
    "synth", "ImprovedNoiseNativeOps.class",
)

# ---------------------------------------------------------------------------
# Minimal classfile parser (JVMS chapter 4). Sufficient to: walk the constant
# pool (all 17 tags, long/double double-slotting), resolve names, walk the
# field/method tables, and locate + decode the Code attribute of a method.
# ---------------------------------------------------------------------------

MAGIC = 0xCAFEBABE

TAG_NAMES = {
    1: "Utf8", 3: "Integer", 4: "Float", 5: "Long", 6: "Double", 7: "Class",
    8: "String", 9: "Fieldref", 10: "Methodref", 11: "InterfaceMethodref",
    12: "NameAndType", 15: "MethodHandle", 16: "MethodType", 17: "Dynamic",
    18: "InvokeDynamic", 19: "Module", 20: "Package",
}
# payload length after the tag byte; None = variable (Utf8).
TAG_PAYLOAD_LEN = {
    1: None, 3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4, 11: 4, 12: 4,
    15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2,
}


class ParseError(Exception):
    pass


class Reader:
    __slots__ = ("b", "p")

    def __init__(self, b: bytes, p: int = 0):
        self.b = b
        self.p = p

    def u1(self) -> int:
        if self.p >= len(self.b):
            raise ParseError(f"truncated at {self.p}")
        v = self.b[self.p]
        self.p += 1
        return v

    def u2(self) -> int:
        if self.p + 2 > len(self.b):
            raise ParseError(f"truncated at {self.p}")
        v = (self.b[self.p] << 8) | self.b[self.p + 1]
        self.p += 2
        return v

    def u4(self) -> int:
        if self.p + 4 > len(self.b):
            raise ParseError(f"truncated at {self.p}")
        v = int.from_bytes(self.b[self.p:self.p + 4], "big")
        self.p += 4
        return v

    def take(self, n: int) -> bytes:
        if self.p + n > len(self.b):
            raise ParseError(f"truncated at {self.p} (want {n})")
        d = self.b[self.p:self.p + n]
        self.p += n
        return d


class Member:
    """A field or method table entry with absolute file offsets."""

    __slots__ = ("access", "name", "desc", "start", "end", "attrs", "code")

    def __init__(self, access, name, desc, start, end, attrs, code):
        self.access = access      # u16 flags
        self.name = name          # resolved utf8
        self.desc = desc          # resolved utf8
        self.start = start        # offset of access_flags
        self.end = end            # offset just past the last attribute
        self.attrs = attrs        # [(name, offset_of_name_idx, length)]
        self.code = code          # CodeBody or None


class CodeBody:
    __slots__ = ("max_stack", "max_locals", "code", "code_offset",
                 "exception_count", "attr_count", "attrs")

    def __init__(self, **kw):
        for k, v in kw.items():
            setattr(self, k, v)


class ClassFile:
    """Parsed classfile. `entries` mirrors src/classfile.rs::Pool: a list of
    (apparent_index, tag, payload) where payload excludes the tag byte."""

    def __init__(self, raw: bytes):
        if len(raw) < 10:
            raise ParseError("class file shorter than the fixed header")
        if int.from_bytes(raw[0:4], "big") != MAGIC:
            raise ParseError("bad magic (not 0xCAFEBABE)")
        self.raw = raw
        self.minor = int.from_bytes(raw[4:6], "big")
        self.major = int.from_bytes(raw[6:8], "big")
        r = Reader(raw, 8)
        cp_count = r.u2()          # entries + 1 (index 0 reserved)
        self.entries = []          # (index, tag, payload)
        self.cp_end = 0
        self._parse_pool(r, cp_count)
        self.cp_end = r.p
        self.access_flags = r.u2()
        this_idx = r.u2()
        super_idx = r.u2()
        self.this_class = self._class_name(this_idx)
        self.super_class = self._class_name(super_idx) if super_idx else None
        self.interfaces = [self._class_name(r.u2()) for _ in range(r.u2())]
        self.fields = self._parse_members(r)
        self.methods = self._parse_members(r)
        # class-level attributes are not needed by any patcher; validated only
        # structurally in the synthetic self-test, skipped here on purpose.

    # -- constant pool ------------------------------------------------------

    def _parse_pool(self, r: Reader, cp_count: int) -> None:
        # JVMS: constant_pool_count = apparent slots + 1 (index 0 reserved);
        # Long/Double rows occupy TWO apparent slots. Loop on the apparent
        # index, not on a row count.
        index = 1
        while index < cp_count:
            off = r.p
            tag = r.u1()
            ln = TAG_PAYLOAD_LEN.get(tag)
            if ln is None:  # Utf8
                length = r.u2()
                payload = r.take(length)
                # keep the length prefix in the payload (Rust Pool symmetry)
                payload = length.to_bytes(2, "big") + payload
            else:
                payload = r.take(ln)
            self.entries.append((index, tag, payload))
            index += 2 if tag in (5, 6) else 1
        self.cp_count = cp_count
        self.next_cp_index = index

    def entry(self, idx: int):
        for e in self.entries:
            if e[0] == idx:
                return e
        return None

    def utf8_value(self, idx: int):
        e = self.entry(idx)
        if not e or e[1] != 1:
            return None
        n = int.from_bytes(e[2][0:2], "big")
        return e[2][2:2 + n].decode("utf-8", "replace")

    def _class_name(self, idx: int):
        e = self.entry(idx)
        if not e or e[1] != 7:
            return None
        return self.utf8_value(int.from_bytes(e[2], "big"))

    # -- members ------------------------------------------------------------

    def _parse_members(self, r: Reader):
        out = []
        for _ in range(r.u2()):
            start = r.p
            access = r.u2()
            name = self.utf8_value(r.u2())
            desc = self.utf8_value(r.u2())
            attrs = []
            code = None
            for _ in range(r.u2()):
                an_off = r.p
                aname = self.utf8_value(r.u2())
                alen = r.u4()
                body_off = r.p
                body = r.take(alen)
                attrs.append((aname, an_off, alen))
                if aname == "Code":
                    code = self._parse_code(body, body_off)
            out.append(Member(access, name, desc, start, r.p, attrs, code))
        return out

    def _parse_code(self, body: bytes, body_off: int) -> CodeBody:
        r = Reader(body)
        max_stack = r.u2()
        max_locals = r.u2()
        clen = r.u4()
        code = r.take(clen)
        exc = r.u2()
        r.take(8 * exc)
        n_attrs = r.u2()
        inner = []
        for _ in range(n_attrs):
            an = self.utf8_value(r.u2())
            alen = r.u4()
            r.take(alen)
            inner.append((an, alen))
        return CodeBody(max_stack=max_stack, max_locals=max_locals, code=code,
                        code_offset=body_off + 8, exception_count=exc,
                        attr_count=n_attrs, attrs=inner)

    # -- lookups ------------------------------------------------------------

    def find_method(self, name: str, desc: str):
        for m in self.methods:
            if m.name == name and m.desc == desc:
                return m
        return None

    def find_field(self, name: str, desc: str):
        for f in self.fields:
            if f.name == name and f.desc == desc:
                return f
        return None


# ---------------------------------------------------------------------------
# Best-effort bytecode instruction count (summary aid only — no verification).
# ---------------------------------------------------------------------------

_LEN_ONE = set(range(0x00, 0x10)) | set(range(0x1A, 0x36)) | set(range(0x3B, 0x78)) \
    | set(range(0x78, 0x84)) | set(range(0x85, 0x99)) | set(range(0xA5, 0xA7)) \
    | set(range(0xAC, 0xB2)) | {0xBE, 0xBF, 0xC2, 0xC3, 0xCA, 0xFE, 0xFF}
_LEN2 = {0x10, 0x12, 0x15, 0x16, 0x17, 0x18, 0x19, 0x36, 0x37, 0x38, 0x39,
         0x3A, 0xA9, 0xBC}
_LEN3 = {0x11, 0x13, 0x14, 0x84} | set(range(0x99, 0xA5)) | {0xA7, 0xA8} \
    | set(range(0xB2, 0xB9)) | {0xBB, 0xBD, 0xC0, 0xC1, 0xC6, 0xC7}
_LEN4 = {0xC5}
_LEN5 = {0xB9, 0xBA, 0xC8, 0xC9}


def count_instructions(code: bytes):
    """Returns (count, exact). Stops at the first undecodable byte."""
    n = 0
    p = 0
    while p < len(code):
        op = code[p]
        if op in _LEN_ONE:
            step = 1
        elif op in _LEN2:
            step = 2
        elif op in _LEN3:
            step = 3
        elif op in _LEN4:
            step = 4
        elif op in _LEN5:
            step = 5
        elif op == 0xC4:  # wide
            if p + 1 >= len(code):
                return (n, False)
            step = 6 if code[p + 1] == 0x84 else 4
        elif op == 0xAA:  # tableswitch
            pad = (4 - ((p + 1) % 4)) % 4
            if p + 1 + pad + 12 > len(code):
                return (n, False)
            lo = int.from_bytes(code[p + 1 + pad + 4:p + 1 + pad + 8], "big", signed=True)
            hi = int.from_bytes(code[p + 1 + pad + 8:p + 1 + pad + 12], "big", signed=True)
            if hi < lo:
                return (n, False)
            step = 1 + pad + 12 + 4 * (hi - lo + 1)
        elif op == 0xAB:  # lookupswitch
            pad = (4 - ((p + 1) % 4)) % 4
            if p + 1 + pad + 8 > len(code):
                return (n, False)
            npairs = int.from_bytes(code[p + 1 + pad + 4:p + 1 + pad + 8], "big")
            step = 1 + pad + 8 + 8 * npairs
        else:
            return (n, False)
        if p + step > len(code):
            return (n, False)
        p += step
        n += 1
    return (n, True)


# ---------------------------------------------------------------------------
# Deterministic area_map patch — faithful port of src/classfile.rs::patch_update
# (append-only constant pool, find-or-push dedup in the SAME call order, fixed
# 82-byte body, fixed StackMapTable, splice at the update() method entry).
# ---------------------------------------------------------------------------

TAG_UTF8, TAG_INTEGER, TAG_CLASS = 1, 3, 7
TAG_FIELDREF, TAG_METHODREF, TAG_NAMEANDTYPE = 9, 10, 12


class Pool:
    def __init__(self, entries, next_index):
        self.entries = list(entries)   # (index, tag, payload)
        self.next = next_index

    @staticmethod
    def _payload_utf8(s: str) -> bytes:
        raw = s.encode("utf-8")
        return len(raw).to_bytes(2, "big") + raw

    def find(self, tag, payload):
        for (i, t, d) in self.entries:
            if t == tag and d == payload:
                return i
        return None

    def push(self, tag, payload, slots):
        idx = self.next
        self.entries.append((idx, tag, payload))
        self.next += slots
        return idx

    def utf8(self, s):
        p = self._payload_utf8(s)
        i = self.find(TAG_UTF8, p)
        return i if i is not None else self.push(TAG_UTF8, p, 1)

    def int_const(self, v):
        p = v.to_bytes(4, "big", signed=True)
        i = self.find(TAG_INTEGER, p)
        return i if i is not None else self.push(TAG_INTEGER, p, 1)

    def class_of(self, utf8_idx):
        p = utf8_idx.to_bytes(2, "big")
        i = self.find(TAG_CLASS, p)
        return i if i is not None else self.push(TAG_CLASS, p, 1)

    def name_and_type(self, name, desc):
        n = self.utf8(name)
        d = self.utf8(desc)
        p = n.to_bytes(2, "big") + d.to_bytes(2, "big")
        i = self.find(TAG_NAMEANDTYPE, p)
        return i if i is not None else self.push(TAG_NAMEANDTYPE, p, 1)

    def _ref(self, tag, owner, name, desc):
        owner_utf8 = self.utf8(owner)
        c = self.class_of(owner_utf8)
        nat = self.name_and_type(name, desc)
        p = c.to_bytes(2, "big") + nat.to_bytes(2, "big")
        i = self.find(tag, p)
        return i if i is not None else self.push(tag, p, 1)

    def field_ref(self, owner, name, desc):
        return self._ref(TAG_FIELDREF, owner, name, desc)

    def method_ref(self, owner, name, desc):
        return self._ref(TAG_METHODREF, owner, name, desc)

    def serialize(self):
        out = bytearray()
        for (_, tag, payload) in self.entries:
            out.append(tag)
            out += payload
        return bytes(out)


def patch_area_map(cf: ClassFile):
    """Returns (patched_bytes, info dict). Raises ParseError/KeyError on any
    structural surprise — the runtime Rust patcher would refuse the same
    input, so refusing here is the correct precompute semantics."""
    if cf.this_class != MAP_CLASS:
        raise ParseError(f"unexpected class {cf.this_class!r} (want {MAP_CLASS!r})")
    pool = Pool(cf.entries, cf.next_cp_index)

    # Rust calls pool.utf8("update")/utf8("(III)Z") before the fieldrefs; both
    # already exist, so these are pool no-ops — kept for exact call-order
    # parity with classfile.rs.
    _update_name = pool.utf8("update")
    _update_desc = pool.utf8("(III)Z")
    m = cf.find_method(*MAP_METHOD)
    if m is None:
        raise ParseError("update(III)Z not found")

    # ---- constant refs needed by the new body (append when absent) --------
    # ORDER MATTERS: this mirrors classfile.rs line-for-line so the output is
    # byte-identical to the runtime Rust patch.
    f_last_x = pool.field_ref(cf.this_class, "lastChunkX", "I")
    f_last_z = pool.field_ref(cf.this_class, "lastChunkZ", "I")
    f_dist = pool.field_ref(cf.this_class, "distance", "I")
    f_param = pool.field_ref(cf.this_class, "parameter", "Ljava/lang/Object;")
    NIE = "java/lang/IllegalArgumentException"
    INTEGER = "java/lang/Integer"
    nie_utf8 = pool.utf8(NIE)
    cls_nie = pool.class_of(nie_utf8)
    m_nie_init = pool.method_ref(NIE, "<init>", "(Ljava/lang/String;)V")
    m_to_str = pool.method_ref(INTEGER, "toString", "(I)Ljava/lang/String;")
    min_int = pool.int_const(-(2 ** 31))
    run_desc = f"(L{cf.this_class};IIIIIILjava/lang/Object;)V"
    m_run = pool.method_ref(OPS_CLASS, "run", run_desc)

    def u2(out, v):
        out += v.to_bytes(2, "big")

    # ---- 82-byte replacement body (see classfile.rs doc comment) ----------
    code = bytearray()
    code.append(0x1D)                       # iload_3
    code += bytes([0x9C, 0x00, 0x0F])       # ifge +15 -> 16
    code.append(0xBB)                       # new
    u2(code, cls_nie)
    code.append(0x59)                       # dup
    code.append(0x1D)                       # iload_3
    code.append(0xB8)                       # invokestatic
    u2(code, m_to_str)
    code.append(0xB7)                       # invokespecial
    u2(code, m_nie_init)
    code.append(0xBF)                       # athrow
    assert len(code) == 16
    code.append(0x2A)                       # aload_0
    code.append(0xB4)                       # getfield
    u2(code, f_last_x)
    code += bytes([0x36, 0x04])             # istore 4
    code.append(0x2A)
    code.append(0xB4)
    u2(code, f_last_z)
    code += bytes([0x36, 0x05])             # istore 5
    code.append(0x2A)
    code.append(0xB4)
    u2(code, f_dist)
    code += bytes([0x36, 0x06])             # istore 6
    code += bytes([0x15, 0x04])             # iload 4
    code.append(0x13)                       # ldc_w
    u2(code, min_int)
    code += bytes([0xA0, 0x00, 0x05])       # if_icmpne +5 -> 44
    code.append(0x03)                       # iconst_0
    code.append(0xAC)                       # ireturn
    assert len(code) == 44
    code.append(0x2A)                       # aload_0
    code.append(0x1B)                       # iload_1
    code.append(0xB5)                       # putfield
    u2(code, f_last_x)
    code.append(0x2A)
    code.append(0x1C)                       # iload_2
    code.append(0xB5)
    u2(code, f_last_z)
    code.append(0x2A)
    code.append(0x1D)                       # iload_3
    code.append(0xB5)
    u2(code, f_dist)
    code.append(0x2A)
    code.append(0xB4)                       # getfield parameter
    u2(code, f_param)
    code += bytes([0x3A, 0x07])             # astore 7
    code.append(0x2A)                       # aload_0
    code += bytes([0x15, 0x04])             # iload 4
    code += bytes([0x15, 0x05])             # iload 5
    code += bytes([0x15, 0x06])             # iload 6
    code.append(0x1B)                       # iload_1
    code.append(0x1C)                       # iload_2
    code.append(0x1D)                       # iload_3
    code += bytes([0x19, 0x07])             # aload 7
    code.append(0xB8)                       # invokestatic
    u2(code, m_run)
    code.append(0x04)                       # iconst_1
    code.append(0xAC)                       # ireturn
    assert len(code) == 82

    # ---- StackMapTable: same_frame @16, append_frame -> @44 ---------------
    stackmap = bytes([0x00, 0x02,
                      0x10,
                      0xFE, 0x00, 0x1B, 0x01, 0x01, 0x01])

    # ---- Code attribute ----------------------------------------------------
    code_attr = bytearray()
    u2(code_attr, pool.utf8("Code"))
    body = bytearray()
    u2(body, 8)                             # max_stack
    u2(body, 8)                             # max_locals
    body += len(code).to_bytes(4, "big")
    body += code
    body += bytes([0, 0])                   # exception_table_length
    body += (1).to_bytes(2, "big")          # attributes_count
    u2(body, pool.utf8("StackMapTable"))
    body += len(stackmap).to_bytes(4, "big")
    body += stackmap
    code_attr += len(body).to_bytes(4, "big")
    code_attr += body

    # ---- replacement method entry + splice ---------------------------------
    method = bytearray()
    u2(method, m.access)
    u2(method, cf.entry_lookup_name_desc(m))
    u2(method, cf.entry_lookup_desc(m))
    u2(method, 1)                           # attributes_count
    method += code_attr

    raw = cf.raw
    out = bytearray()
    out += raw[0:8]                         # magic, minor, major (preserved)
    u2(out, pool.next)                      # new cp_count
    out += pool.serialize()
    out += raw[cf.cp_end:m.start]
    out += method
    out += raw[m.end:]
    info = {
        "code_len": len(code),
        "stackmap_bytes": len(stackmap),
        "cp_entries_after": len(pool.entries),
        "cp_count_after": pool.next,
        "old_update_code_len": len(m.code.code),
        "class_len": len(raw),
        "patched_len": len(out),
    }
    return bytes(out), info


def _member_name_idx(self: ClassFile, m: Member) -> int:
    """Recover the name_idx the member uses in the raw file. The splice must
    copy the ORIGINAL cp indices (classfile.rs does exactly that), even though
    our parser resolves strings."""
    return Reader(self.raw, m.start + 2).u2()


def _member_desc_idx(self: ClassFile, m: Member) -> int:
    return Reader(self.raw, m.start + 4).u2()


ClassFile.entry_lookup_name_desc = _member_name_idx
ClassFile.entry_lookup_desc = _member_desc_idx


# ---------------------------------------------------------------------------
# Class-version guard (mirrors improved_noise.rs::jvm_max_class_major +
# the activate() guard), evaluated at build time.
# ---------------------------------------------------------------------------

def version_guard(major: int, max_class_major: int, is_noise: bool, strict: bool):
    """Returns (problems[], warnings[]). Rule from improved_noise.rs:
    any class bytes we ask the JVM to define must have major <= the JVM's
    java.class.version major (65 on Java 21). The bridge is `--release 8`
    (major 52); a kernel class's major is inherited by the patched bytes, so
    a kernel newer than the runtime JVM can never be patched."""
    problems, warnings = [], []
    if major > max_class_major:
        msg = (f"class-file major {major} > target runtime max {max_class_major} "
               f"(java.class.version ceiling; Java 21 = 65) — the live JVM would "
               f"reject these bytes (UnsupportedClassVersionError); improved_noise.rs "
               f"guard would keep the hook dormant")
        (problems if strict else warnings).append(msg)
    if is_noise and major > NOISE_BRIDGE_MAX_MAJOR:
        # informational echo of the bridge rule: the BRIDGE itself must be 52
        # (--release 8); kernel classes are naturally newer (65 on Purpur 1.21.x)
        warnings.append(
            f"noise kernel class major {major} > {NOISE_BRIDGE_MAX_MAJOR}: expected for "
            f"a modern kernel (patched bytes inherit this major, runtime JVM must be "
            f">= {[j for j, mm in JAVA_MAJOR.items() if mm == major][0] if major in JAVA_MAJOR.values() else '?'}); "
            f"the ImprovedNoiseNativeOps bridge itself must stay major <= {NOISE_BRIDGE_MAX_MAJOR} "
            f"(--release 8) per improved_noise.rs"
        )
    return problems, warnings


def check_noise_bridge_bytes(repo_root: str):
    """Parse the bundled noise bridge classes if present and apply the
    improved_noise.rs guard statically: the embedded bytes are define_class'ed
    INTO the kernel JVM, so their major must never exceed the kernel JVM's max
    (65 = Java 21, the only JVM that can run a 1.21 kernel). History: a
    Java-25 toolchain once shipped major-69 bytes -> UnsupportedClassVersionError
    on the live server; scripts/build_noise.sh now pins --release (major 55
    baseline, Cleaner needs Java 9+). We warn per the stricter historical rule
    (52 = --release 8) so CI notices drift, and hard-fail above 65."""
    paths = [os.path.join(repo_root, "noise", "build", "net", "minecraft",
                          "world", "level", "levelgen", "synth", n)
             for n in ("ImprovedNoiseNativeOps.class",
                       "ImprovedNoiseNativeOps$Handle.class")]
    warnings, infos = [], []
    if not any(os.path.exists(p) for p in paths):
        warnings.append(f"noise bridge classes not found under {os.path.dirname(paths[0])} "
                        f"(run scripts/build_noise.sh; runtime guard compares their "
                        f"major to the kernel JVM's java.class.version)")
        return warnings, None
    for path in paths:
        if not os.path.exists(path):
            warnings.append(f"expected bridge class missing: {path} "
                            f"(improved_noise.rs defines BOTH classes; a missing one "
                            f"= NoClassDefFoundError at first noise() call)")
            continue
        try:
            with open(path, "rb") as fh:
                b = fh.read()
            major = int.from_bytes(b[6:8], "big")
            info = {"path": os.path.relpath(path, repo_root),
                    "major": major, "sha256": hashlib.sha256(b).hexdigest(),
                    "size": len(b)}
            infos.append(info)
            if major > 65:
                warnings.append(
                    f"embedded noise bridge {info['path']} is class-file major {major} "
                    f"> 65 (Java 21 kernel max) — the improved_noise.rs runtime guard "
                    f"will keep the hook dormant; rebuild noise/ with scripts/build_noise.sh")
            elif major > NOISE_BRIDGE_MAX_MAJOR:
                warnings.append(
                    f"embedded noise bridge {info['path']} is class-file major {major} "
                    f"> {NOISE_BRIDGE_MAX_MAJOR}: acceptable ONLY because every 1.21 "
                    f"kernel JVM is Java 21 (major 65) and build_noise.sh pins "
                    f"--release 11 (major 55, Cleaner needs Java 9+); the historical "
                    f"--release 8 (major 52) floor is exceeded — CI heads-up per the "
                    f"improved_noise.rs guard")
        except Exception as e:  # noqa: BLE001
            warnings.append(f"cannot parse noise bridge {path}: {e}")
    return warnings, (infos[0] if infos else None)


# ---------------------------------------------------------------------------
# Jar extraction
# ---------------------------------------------------------------------------

def find_jar_entry(zf: zipfile.ZipFile, class_internal: str):
    """Locate the entry for a class. Prefers the exact base-path entry;
    rejects META-INF/versions (multi-release overlays) unless it is all we
    have (and says so)."""
    want = class_internal + ".class"
    exact = []
    suffixed = []
    for name in zf.namelist():
        if name == want:
            exact.append(name)
        elif name.endswith("/" + want) and not name.startswith("META-INF/versions/"):
            suffixed.append(name)
    if exact:
        return exact[0], False
    if suffixed:
        # shortest path wins (shaded jars embed the class under odd roots)
        return sorted(suffixed, key=len)[0], True
    return None, False


def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def is_paperclip(zf: zipfile.ZipFile) -> bool:
    """Modern Paper/Purpur download jars are 'paperclip' launchers: the actual
    server classes are NOT inside; they are produced at first boot into
    versions/<v>/<server>.jar (vanilla download + bsdiff patches + remap)."""
    return any(n == "paperclip/" or n.startswith("paperclip/")
               for n in zf.namelist())


def paperclip_extract(launcher_jar: str, workdir: str, timeout: int = 420):
    """Run the paperclip launcher headlessly so it materializes the real
    server jar. eula=false is pre-seeded: the launcher exits right after
    'Applying patches' + remap, before any server thread starts."""
    import subprocess
    os.makedirs(workdir, exist_ok=True)
    eula = os.path.join(workdir, "eula.txt")
    if not os.path.exists(eula):
        with open(eula, "w") as fh:
            fh.write("eula=false\n")
    cmd = ["java", "-Dpaperclip.extractOnly=true", "-jar",
           os.path.abspath(launcher_jar), "--nogui"]
    try:
        proc = subprocess.run(cmd, cwd=workdir, capture_output=True,
                              text=True, timeout=timeout)
        log = (proc.stdout + proc.stderr)[-2000:]
    except subprocess.TimeoutExpired as e:
        log = ((e.stdout or b"").decode("utf-8", "replace")
               + (e.stderr or b"").decode("utf-8", "replace"))[-2000:]
    candidates = sorted(glob_versions_jars(workdir))
    if not candidates:
        raise ParseError("paperclip extraction produced no versions/*/*.jar; log tail:\n"
                         + log)
    # prefer a jar that actually contains every target class
    for cand in candidates:
        try:
            with zipfile.ZipFile(cand) as zf:
                if all(find_jar_entry(zf, t["class_internal"])[0] for t in TARGETS):
                    return cand
        except zipfile.BadZipFile:
            continue
    return candidates[-1]


def glob_versions_jars(workdir: str):
    out = []
    vroot = os.path.join(workdir, "versions")
    if os.path.isdir(vroot):
        for sub in sorted(os.listdir(vroot)):
            d = os.path.join(vroot, sub)
            if os.path.isdir(d):
                for f in sorted(os.listdir(d)):
                    if f.endswith(".jar"):
                        out.append(os.path.join(d, f))
    return out


# ---------------------------------------------------------------------------
# Per-target processing
# ---------------------------------------------------------------------------

def method_record(cf: ClassFile, m: Member):
    count, exact = count_instructions(m.code.code)
    return {
        "name": m.name,
        "desc": m.desc,
        "access_flags": f"0x{m.access:04x}",
        "method_start_offset": m.start,
        "method_end_offset": m.end,
        "code_attr_offset": (m.attrs[0][1] if m.attrs else None),
        "code_offset": m.code.code_offset,
        "code_len": len(m.code.code),
        "max_stack": m.code.max_stack,
        "max_locals": m.code.max_locals,
        "instruction_count": count,
        "instruction_count_exact": exact,
        "exception_table_entries": m.code.exception_count,
        "code_attributes": [a[0] for a in m.code.attrs],
        "bytecode_sha256": hashlib.sha256(m.code.code).hexdigest(),
        "bytecode_hex": m.code.code.hex(),
    }


def process_target(t, zf, out_dir, max_class_major, strict, emit_patched,
                   asm_ctx=None):
    key = t["key"]
    entry, relocated = find_jar_entry(zf, t["class_internal"])
    if entry is None:
        raise KeyError(f"{t['class_internal']} not found in kernel jar")
    raw = zf.read(entry)
    cf = ClassFile(raw)
    problems, warnings = [], []

    m = cf.find_method(*t["method"])
    if m is None or m.code is None:
        problems.append(
            f"target method {t['method'][0]}{t['method'][1]} not found (or has no Code) "
            f"in {t['class_internal']} — the runtime patcher would bail; kernel shape changed?")
    fields = {}
    for fname, fdesc in sorted(t["required_fields"].items()):
        f = cf.find_field(fname, fdesc)
        fields[fname] = {"desc": fdesc, "present": f is not None,
                         "access_flags": f"0x{f.access:04x}" if f else None}
        if f is None:
            problems.append(
                f"required field {fname}:{fdesc} missing in {t['class_internal']} "
                f"(ReplaceBody ThisField / spliced body reads it)")

    p_problems, p_warnings = version_guard(cf.major, max_class_major,
                                           key == "improved_noise", strict)
    problems += p_problems
    warnings += p_warnings
    if relocated:
        warnings.append(f"entry located at non-canonical path {entry!r} "
                        f"(shaded/relocated jar?)")

    rec = {
        "target": key,
        "class_internal": cf.this_class,
        "expected_class_internal": t["class_internal"],
        "jar_entry": entry,
        "class_file": {
            "minor": cf.minor,
            "major": cf.major,
            "java_hint": {v: k for k, v in JAVA_MAJOR.items()}.get(cf.major),
            "size": len(raw),
            "sha256": hashlib.sha256(raw).hexdigest(),
        },
        "fields": fields,
        "cp": {"count": cf.cp_count, "entries": len(cf.entries)},
        "method": method_record(cf, m) if (m and m.code) else None,
        "warnings": warnings,
        "problems": problems,
    }

    if key == "improved_noise":
        rec["replace_body_spec"] = NOISE_REPLACE_BODY
        if asm_ctx and not problems:
            # JVM-backed precompute: run the SAME SdkAsmHelper REPLACE_BODY
            # rewrite the runtime worker runs, but here, at build time.
            try:
                patched = asm_ctx["rewrite"](raw)
                pcf = ClassFile(patched)
                pm = pcf.find_method(*NOISE_METHOD)
                pinfo = {}
                if pm is None or pm.code is None:
                    problems.append("internal: patched noise class failed re-parse")
                else:
                    pinfo = {"code_len": len(pm.code.code),
                             "max_stack": pm.code.max_stack,
                             "max_locals": pm.code.max_locals}
                    if len(pm.code.code) >= len(m.code.code):
                        problems.append(
                            f"patched noise() body did not shrink "
                            f"({len(m.code.code)} -> {len(pm.code.code)}B) — "
                            f"rewrite looks wrong")
                rel = os.path.join("patches", "patched", "ImprovedNoise.patched.class")
                os.makedirs(os.path.join(out_dir, "patches", "patched"), exist_ok=True)
                with open(os.path.join(out_dir, rel), "wb") as fh:
                    fh.write(patched)
                rec["patched"] = {
                    "file": rel,
                    "sha256": hashlib.sha256(patched).hexdigest(),
                    "size": len(patched),
                    "baseline_sha256": rec["class_file"]["sha256"],
                    "reparse_major": pcf.major,
                    "reparse_this_class": pcf.this_class,
                    "details": pinfo,
                    "pipeline": (f"JVM REPLACE_BODY via SdkAsmHelper "
                                 f"(classpath: {asm_ctx['cp']}); spec = "
                                 f"replace_body_spec (wire format of "
                                 f"cplug-sdk/src/asm.rs::serialize)"),
                }
                rec["patch_pipeline"] = (
                    "PRECOMPUTED at build time (ASM ReplaceBody via SdkAsmHelper); "
                    "runtime serves the bytes after baseline-sha256 check")
            except RuntimeError as e:
                warnings.append(f"JVM-backed noise precompute unavailable: {e}")
                rec["patch_pipeline"] = (
                    "validation-only at build time (no JVM/ASM available); "
                    "patch computed at activation exactly as today")
        else:
            rec["patch_pipeline"] = (
                "validation-only at build time (pass --no-noise-asm off and have "
                "cplug-sdk/asm-lib + a JDK for JVM-backed precompute); patch "
                "computed at activation as today")
    else:
        rec["patch_pipeline"] = (
            "deterministic byte-splice ported from src/classfile.rs::patch_update; "
            "patched class bytes emitted at build time")
        if emit_patched and not problems:
            patched, pinfo = patch_area_map(cf)
            pcf = ClassFile(patched)   # re-parse: must survive our own reader
            pm = pcf.find_method(*MAP_METHOD)
            if pm is None or len(pm.code.code) != 82:
                problems.append("internal: patched class failed re-parse/self-check")
            rel = os.path.join("patches", "patched", "SingleUserAreaMap.patched.class")
            os.makedirs(os.path.join(out_dir, "patches", "patched"), exist_ok=True)
            with open(os.path.join(out_dir, rel), "wb") as fh:
                fh.write(patched)
            rec["patched"] = {
                "file": rel,
                "sha256": hashlib.sha256(patched).hexdigest(),
                "size": len(patched),
                "baseline_sha256": rec["class_file"]["sha256"],
                "code_len": 82,
                "reparse_major": pcf.major,
                "reparse_this_class": pcf.this_class,
                "details": pinfo,
            }
    return rec


# ---------------------------------------------------------------------------
# Synthetic self-test class — built byte-by-byte, one entry per cp tag.
# ---------------------------------------------------------------------------

def build_synthetic_class() -> bytes:
    """A tiny valid-ish class 'Simple' with field 'counter:I' and method
    'run(II)I' (code: iload_1, iload_2, iadd, ireturn). Contains at least one
    constant-pool entry of EVERY tag (1,3..20). Structure is valid enough for
    our parser and for javap -v (minus verification-level details we don't
    emit: no StackMapTable needed at major 52... javac emits none for 49-)."""
    b = bytearray()

    def u1(v):
        b.append(v & 0xFF)

    def u2(v):
        b.extend((v & 0xFFFF).to_bytes(2, "big"))

    def u4(v):
        b.extend((v & 0xFFFFFFFF).to_bytes(4, "big"))

    def utf8(s):
        raw = s.encode()
        b.append(1)
        u2(len(raw))
        b.extend(raw)

    # We must know indices up-front; lay out the pool deterministically.
    # slots: 1 Utf8"Simple" | 2 Class(#1) | 3 Utf8"java/lang/Object"
    # 4 Class(#3) | 5 Utf8"counter" | 6 Utf8"I" | 7 NameAndType(#5,#6)
    # 8 Fieldref(#2,#7) | 9 Utf8"run" | 10 Utf8"(II)I" | 11 Utf8"Code"
    # 12 Integer 0x7FFFFFFF | 13 Float 1.5 | 14 Long 0x0123456789ABCDEF
    # (15 unused phantom) | 16 Double 2.5 (17 phantom) | 18 String(#1)
    # 19 Utf8"<init>" | 20 Utf8"()V" | 21 NameAndType(#19,#20)
    # 22 Methodref(#4,#21) | 23 MethodType(#10) | 24 MethodHandle(6REF#22)
    # 25 Utf8"BootstrapMethods-y" | 26 NameAndType(#9,#10)
    # 27 InvokeDynamic(0,#26) | 28 InterfaceMethodref(#4,#26)
    # 29 Utf8"pkg" | 30 Utf8"Simple" | 31 NameAndType(#29,#30)
    # 32 Package(#31) | 33 Module(#29) | 34 Dynamic(0,#26)
    # cp_count = 35 (indices 1..34, with 15 and 17 phantom-slotted)
    b.extend(bytes([0xCA, 0xFE, 0xBA, 0xBE]))
    u2(3)    # minor
    u2(52)   # major (Java 8)
    u2(35)   # constant_pool_count

    utf8("Simple")                    # 1
    b.append(7); u2(1)                # 2 Class -> Simple
    utf8("java/lang/Object")          # 3
    b.append(7); u2(3)                # 4 Class -> Object
    utf8("counter")                   # 5
    utf8("I")                         # 6
    b.append(12); u2(5); u2(6)        # 7 NameAndType
    b.append(9); u2(2); u2(7)         # 8 Fieldref
    utf8("run")                       # 9
    utf8("(II)I")                     # 10
    utf8("Code")                      # 11
    b.append(3); b.extend((0x7FFFFFFF).to_bytes(4, "big"))   # 12 Integer
    b.append(4); b.extend(struct.pack(">f", 1.5))            # 13 Float
    b.append(5); b.extend((0x0123456789ABCDEF).to_bytes(8, "big"))  # 14 Long (2 slots: 14+15)
    b.append(6); b.extend(struct.pack(">d", 2.5))            # 16 Double (2 slots: 16+17)
    b.append(8); u2(1)                # 18 String -> Simple
    utf8("<init>")                    # 19
    utf8("()V")                       # 20
    b.append(12); u2(19); u2(20)      # 21 NameAndType
    b.append(10); u2(4); u2(21)       # 22 Methodref
    b.append(16); u2(10)              # 23 MethodType -> (II)I
    b.append(15); u1(6); u2(22)       # 24 MethodHandle (6=REF_invokeSpecial)
    utf8("bsm")                       # 25
    b.append(12); u2(9); u2(10)       # 26 NameAndType run:(II)I
    b.append(18); u2(0); u2(26)       # 27 InvokeDynamic
    b.append(11); u2(4); u2(26)       # 28 InterfaceMethodref
    utf8("pkg")                       # 29
    utf8("Simple")                    # 30
    b.append(12); u2(29); u2(30)      # 31 NameAndType pkg.Simple
    b.append(20); u2(31)              # 32 Package
    b.append(19); u2(29)              # 33 Module
    b.append(17); u2(0); u2(26)       # 34 Dynamic

    u2(0x0021)  # access: ACC_PUBLIC|ACC_SUPER
    u2(2)       # this_class
    u2(4)       # super_class
    u2(0)       # interfaces_count

    u2(1)       # fields_count
    u2(0x0002)  # ACC_PRIVATE
    u2(5)       # name counter
    u2(6)       # desc I
    u2(0)       # attrs

    u2(2)       # methods_count: <init> + run
    # <init>()V
    u2(0x0001)  # ACC_PUBLIC
    u2(19)      # name
    u2(20)      # desc
    u2(1)       # attrs
    u2(11)      # "Code"
    u4(17)      # attribute_length = 2+2+4+5+2+2
    u2(1)       # max_stack
    u2(1)       # max_locals
    u4(5)       # code_len
    b.extend(bytes([0x2A, 0xB7, 0x00, 0x16, 0xB1]))  # aload_0; invokespecial #22; return
    u2(0)       # exception_table_length
    u2(0)       # code attributes_count
    # run(II)I
    u2(0x0001)
    u2(9)
    u2(10)
    u2(1)
    u2(11)
    u4(16)      # attribute_length = 2+2+4+4+2+2
    u2(2)       # max_stack
    u2(3)       # max_locals
    u4(4)       # code_len
    b.extend(bytes([0x1A, 0x1B, 0x60, 0xAC]))        # iload_1; iload_2; iadd; ireturn
    u2(0)       # exception_table_length
    u2(0)       # code attributes_count

    u2(0)       # class attributes_count
    return bytes(b)


# ---------------------------------------------------------------------------
# Self-test
# ---------------------------------------------------------------------------

class SelfTest:
    def __init__(self):
        self.passed = 0
        self.failed = 0

    def check(self, name, cond, detail=""):
        if cond:
            self.passed += 1
            print(f"  ok    {name}")
        else:
            self.failed += 1
            print(f"  FAIL  {name} {detail}")

    def run(self, repo_root: str) -> bool:
        print("[1/3] synthetic classfile (constructed byte-by-byte)")
        self._synthetic()

        fx_map = os.path.join(repo_root, "tests", "fixtures", "SingleUserAreaMap.class")
        if os.path.exists(fx_map):
            print("[2/3] real fixture tests/fixtures/SingleUserAreaMap.class")
            self._map_fixture(fx_map)
        else:
            print("[2/3] SingleUserAreaMap fixture not present — skipped")

        fx_noise = os.path.join(repo_root, "cplug-sdk", "asm-src", "fixtures",
                                "ImprovedNoise.class")
        if os.path.exists(fx_noise):
            print("[3/3] real fixture cplug-sdk/asm-src/fixtures/ImprovedNoise.class")
            self._noise_fixture(fx_noise)
        else:
            print("[3/3] ImprovedNoise fixture not present — skipped")

        golden = os.environ.get("CRUSSTY_GOLDEN_PATCHED",
                                "/tmp/opencode/patchcheck/patched.class")
        if os.path.exists(fx_map) and os.path.exists(golden):
            print("[+] golden cross-check vs Rust classfile.rs output")
            self._golden(fx_map, golden)
        print(f"self-test: {self.passed} passed, {self.failed} failed")
        return self.failed == 0

    def _synthetic(self):
        raw = build_synthetic_class()
        cf = ClassFile(raw)
        self.check("magic/version", (cf.minor, cf.major) == (3, 52), f"got {cf.minor}.{cf.major}")
        self.check("this_class", cf.this_class == "Simple", f"got {cf.this_class!r}")
        self.check("super_class", cf.super_class == "java/lang/Object")
        # every cp tag parsed
        tags = {t for (_, t, _) in cf.entries}
        want = set(TAG_NAMES)
        self.check("all 17 cp tags present", want <= tags, f"missing {want - tags}")
        # long/double double-slotting: entry after Long(#14) has index 16
        e_after_long = cf.entry(16)
        self.check("long double-slot accounting",
                   e_after_long is not None and e_after_long[1] == 6,
                   f"idx16={e_after_long}")
        self.check("cp_count matches slots", cf.next_cp_index == cf.cp_count)
        f = cf.find_field("counter", "I")
        self.check("field counter:I located", f is not None and f.access == 0x0002)
        m = cf.find_method("run", "(II)I")
        self.check("method run(II)I located", m is not None)
        if m and m.code:
            self.check("run() code hex", m.code.code.hex() == "1a1b60ac",
                       m.code.code.hex())
            self.check("run() max_stack/max_locals",
                       (m.code.max_stack, m.code.max_locals) == (2, 3))
            n, exact = count_instructions(m.code.code)
            self.check("run() instruction count == 4 (exact)", n == 4 and exact)
            seg = raw[m.code.code_offset:m.code.code_offset + len(m.code.code)]
            self.check("run() code_offset points at the code bytes",
                       seg.hex() == "1a1b60ac", seg.hex())
        init = cf.find_method("<init>", "()V")
        self.check("<init>()V located with aload_0/special/return",
                   init is not None and init.code.code.hex() == "2ab70016b1",
                   init.code.code.hex() if init and init.code else "?")
        # negative paths
        try:
            bad = bytes(raw)
            bad = bad[:9] + bytes([bad[9] ^ 0xFF]) + bad[10:]
            ClassFile(bad)
            self.check("corrupted pool rejected", False, "no exception")
        except ParseError:
            self.check("corrupted pool rejected", True)
        try:
            ClassFile(b"not a class file at all")
            self.check("bad magic rejected", False, "no exception")
        except ParseError:
            self.check("bad magic rejected", True)
        try:
            ClassFile(raw[:30])
            self.check("truncated class rejected", False, "no exception")
        except ParseError:
            self.check("truncated class rejected", True)

    def _map_fixture(self, path):
        with open(path, "rb") as fh:
            raw = fh.read()
        cf = ClassFile(raw)
        self.check("fixture this_class", cf.this_class == MAP_CLASS, cf.this_class)
        m = cf.find_method(*MAP_METHOD)
        self.check("update(III)Z located", m is not None and m.code is not None)
        if m and m.code:
            n, exact = count_instructions(m.code.code)
            print(f"        update(): {len(m.code.code)} bytes, {n} insns "
                  f"(exact={exact}), max_stack={m.code.max_stack}, "
                  f"max_locals={m.code.max_locals}")
        for fn, fd in MAP_REQUIRED_FIELDS.items():
            self.check(f"field {fn}:{fd}", cf.find_field(fn, fd) is not None)
        patched, info = patch_area_map(cf)
        self.check("patched starts with CAFEBABE",
                   patched[:4] == bytes([0xCA, 0xFE, 0xBA, 0xBE]))
        self.check("patched preserves header (major/minor)",
                   patched[:8] == raw[:8])
        pcf = ClassFile(patched)
        self.check("patched re-parses; this_class intact",
                   pcf.this_class == MAP_CLASS)
        pm = pcf.find_method(*MAP_METHOD)
        self.check("patched update() body is 82 bytes",
                   pm is not None and pm.code is not None and len(pm.code.code) == 82,
                   f"got {len(pm.code.code) if pm and pm.code else 'none'}")
        self.check("patched update() carries StackMapTable",
                   pm is not None and any(a[0] == "StackMapTable" for a in pm.code.attrs))
        self.check("patched field table untouched",
                   pcf.find_field("parameter", "Ljava/lang/Object;") is not None)
        self.check("patched is smaller than original",
                   len(patched) < len(raw),
                   f"{len(raw)} -> {len(patched)}")
        print(f"        patch: {len(raw)} -> {len(patched)} bytes, "
              f"cp {cf.cp_count} -> {pcf.cp_count}")

    def _noise_fixture(self, path):
        with open(path, "rb") as fh:
            raw = fh.read()
        cf = ClassFile(raw)
        self.check("noise fixture this_class", cf.this_class == NOISE_CLASS,
                   cf.this_class)
        m = cf.find_method(*NOISE_METHOD)
        self.check("noise(DDDDD)D located", m is not None and m.code is not None)
        if m and m.code:
            n, exact = count_instructions(m.code.code)
            print(f"        noise(): {len(m.code.code)} bytes, {n} insns "
                  f"(exact={exact}), max_stack={m.code.max_stack}, "
                  f"max_locals={m.code.max_locals}")
            self.check("noise() max_locals >= 10 (ReplaceBody slots 0..9)",
                       m.code.max_locals >= 10, f"got {m.code.max_locals}")
        for fn, fd in NOISE_REQUIRED_FIELDS.items():
            self.check(f"field {fn}:{fd}", cf.find_field(fn, fd) is not None)
        major = cf.major
        self.check("noise fixture is a modern kernel class (major>=61)",
                   major >= 61, f"major={major}")

    def _golden(self, fixture, golden):
        with open(fixture, "rb") as fh:
            cf = ClassFile(fh.read())
        patched, _ = patch_area_map(cf)
        with open(golden, "rb") as fh:
            gold = fh.read()
        same = patched == gold
        self.check("python patch == Rust patch_update output (byte-identical)",
                   same,
                   f"py={hashlib.sha256(patched).hexdigest()[:12]} "
                   f"rust={hashlib.sha256(gold).hexdigest()[:12]} len "
                   f"{len(patched)} vs {len(gold)}")


# ---------------------------------------------------------------------------
# JVM-backed noise precompute (ASM REPLACE_BODY outside the live server).
#
# The runtime computes the noise patch on a background worker because ASM's
# COMPUTE_FRAMES resolves frame types through a classloader. It does NOT need
# the kernel loader for ImprovedNoise (all referenced types are JDK), so the
# exact same SdkAsmHelper.rewrite([B,[B)[B runs fine on a plain JDK at build
# time. The wire spec below mirrors cplug-sdk/src/asm.rs::serialize() 1:1.
# ---------------------------------------------------------------------------

ASM_DRIVER_JAVA = """
import java.nio.file.*;

/** Generated by scripts/precompute_patches.py: build-time REPLACE_BODY
 *  rewrite through the repo's SdkAsmHelper (ASM). */
public class PrecomputeNoise {
    public static void main(String[] a) throws Exception {
        byte[] src = Files.readAllBytes(Paths.get(a[0]));
        byte[] spec = Files.readAllBytes(Paths.get(a[1]));
        byte[] out = dev.dist.SdkAsmHelper.rewrite(src, spec);
        if (out == null) {
            System.err.println("rewrite returned null; lastError="
                + dev.dist.SdkAsmHelper.lastError());
            System.exit(3);
        }
        Files.write(Paths.get(a[2]), out);
    }
}
"""


def serialize_replace_body(spec: dict) -> bytes:
    """Wire format of cplug-sdk/src/asm.rs::serialize():
    u8 version=1, u8 op=1, str method_name, str method_desc, str bridge_owner,
    str bridge_name, str bridge_desc, u8 argc, then per arg:
    Local{u2 slot, u1 ty} | ThisField{u2 0, u1 '@', str name, str desc}.
    str = u2 len + bytes."""

    def push_str(out, s):
        raw = s.encode()
        out += len(raw).to_bytes(2, "big")
        out += raw

    out = bytearray()
    out.append(1)   # version
    out.append(1)   # op REPLACE_BODY
    for k in ("method_name", "method_desc", "bridge_owner", "bridge_name",
              "bridge_desc"):
        push_str(out, spec[k])
    args = spec["args"]
    out.append(len(args))
    for a in args:
        if a["kind"] == "local":
            out += a["slot"].to_bytes(2, "big")
            out.append(ord(a["ty"]))
        else:
            out += (0).to_bytes(2, "big")
            out.append(ord("@"))
            push_str(out, a["name"])
            push_str(out, a["desc"])
    return bytes(out)


def _find_java_tool(name):
    import shutil
    p = shutil.which(name)
    if p:
        return p
    home = os.environ.get("JAVA_HOME", "")
    if home and os.path.isfile(os.path.join(home, "bin", name)):
        return os.path.join(home, "bin", name)
    for cand in ("/home/z/jdk21/bin", "/usr/lib/jvm/*/bin"):
        import glob as _g
        for hit in _g.glob(os.path.join(cand, name)):
            if os.path.isfile(hit):
                return hit
    return None


def make_asm_context(repo_root, workdir, cp_override=None):
    """Returns {'cp': ..., 'rewrite': fn(bytes)->bytes} or None when the repo
    has no ASM setup and no override was given."""
    import glob as _g
    import subprocess
    if cp_override:
        cp = cp_override
    else:
        jars = sorted(_g.glob(os.path.join(repo_root, "cplug-sdk", "asm-lib",
                                           "asm-*.jar")))
        build = os.path.join(repo_root, "cplug-sdk", "asm-build")
        if not jars or not os.path.isdir(os.path.join(build, "dev", "dist")):
            return None
        cp = os.pathsep.join(jars + [build])
    if not os.path.isdir(os.path.join(workdir)):
        os.makedirs(workdir, exist_ok=True)
    driver_cp_entry = workdir
    marker = os.path.join(workdir, "PrecomputeNoise.class")
    if not os.path.exists(marker):
        javac = _find_java_tool("javac")
        java = _find_java_tool("java")
        if not javac:
            raise RuntimeError("javac not found (needed once to compile the "
                               "PrecomputeNoise driver); set JAVA_HOME or install a JDK")
        if not java:
            raise RuntimeError("java not found")
        src = os.path.join(workdir, "PrecomputeNoise.java")
        with open(src, "w") as fh:
            fh.write(ASM_DRIVER_JAVA)
        subprocess.run([javac, "--release", "11", "-cp", cp, "-d", workdir, src],
                       check=True, capture_output=True, text=True)
    else:
        java = _find_java_tool("java")
        if not java:
            raise RuntimeError("java not found")

    def rewrite(src_bytes: bytes) -> bytes:
        import tempfile
        with tempfile.TemporaryDirectory(dir=workdir) as td:
            s = os.path.join(td, "src.class")
            sp = os.path.join(td, "spec.bin")
            o = os.path.join(td, "out.class")
            with open(s, "wb") as fh:
                fh.write(src_bytes)
            with open(sp, "wb") as fh:
                fh.write(serialize_replace_body(NOISE_REPLACE_BODY))
            proc = subprocess.run(
                [java, "-cp", os.pathsep.join([driver_cp_entry, cp]),
                 "PrecomputeNoise", s, sp, o],
                capture_output=True, text=True, timeout=120)
            if proc.returncode != 0 or not os.path.exists(o):
                raise RuntimeError(f"SdkAsmHelper rewrite failed rc={proc.returncode}: "
                                   f"{proc.stderr[-400:]}")
            with open(o, "rb") as fh:
                return fh.read()

    return {"cp": cp, "rewrite": rewrite}


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        description="Precompute/pre-validate c-crussty hot-patch inputs at build time")
    ap.add_argument("--kernel-jar", metavar="PATH",
                    help="Paper/Purpur server jar to extract target classes from")
    ap.add_argument("--out", metavar="DIR", default="precompute_out",
                    help="output directory (default: ./precompute_out)")
    ap.add_argument("--max-class-major", type=int, default=JAVA_MAJOR[21],
                    help="target runtime's max class-file major (default 65 = Java 21)")
    ap.add_argument("--strict", action="store_true",
                    help="treat class-version ceiling warnings as failures (exit 2)")
    ap.add_argument("--no-emit-patched", action="store_true",
                    help="skip emitting the spliced SingleUserAreaMap patch bytes")
    ap.add_argument("--extract-paperclip", action="store_true",
                    help="if the kernel jar is a paperclip launcher (modern Paper/"
                         "Purpur: classes live in versions/<v>/<server>.jar produced "
                         "at first boot), run 'java -Dpaperclip.extractOnly=true' "
                         "in --workdir and read classes from the extracted server jar")
    ap.add_argument("--workdir", metavar="DIR", default=None,
                    help="workdir for paperclip extraction + ASM driver (default: <out>/work)")
    ap.add_argument("--no-noise-asm", action="store_true",
                    help="skip the JVM-backed noise patch precompute (validation only; "
                         "default: use repo cplug-sdk/asm-lib + a JDK when available)")
    ap.add_argument("--asm-cp", metavar="CP", default=None,
                    help="override the Java classpath holding dev/dist/SdkAsmHelper "
                         "and org.objectweb.asm (default: cplug-sdk/asm-lib/*.jar:"
                         "cplug-sdk/asm-build)")
    ap.add_argument("--self-test", action="store_true",
                    help="run the embedded self-test and exit")
    args = ap.parse_args(argv)

    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    if args.self_test:
        ok = SelfTest().run(repo_root)
        return 0 if ok else 2

    if not args.kernel_jar:
        ap.print_help()
        print("\nerror: --kernel-jar is required (or use --self-test)", file=sys.stderr)
        return 1
    if not os.path.isfile(args.kernel_jar):
        print(f"error: kernel jar not found: {args.kernel_jar}", file=sys.stderr)
        return 1

    out_dir = os.path.abspath(args.out)
    patches_dir = os.path.join(out_dir, "patches")
    os.makedirs(patches_dir, exist_ok=True)

    jar_sha = sha256_file(args.kernel_jar)
    jar_size = os.path.getsize(args.kernel_jar)
    print(f"input jar  : {args.kernel_jar} ({jar_size} bytes, sha256 {jar_sha[:16]}…)")

    # -- resolve the jar that actually carries the server classes -----------
    provenance = {"input_jar": {"path": os.path.abspath(args.kernel_jar),
                                "name": os.path.basename(args.kernel_jar),
                                "size": jar_size, "sha256": jar_sha}}
    server_jar = args.kernel_jar
    with zipfile.ZipFile(server_jar) as zf:
        have_all = all(find_jar_entry(zf, t["class_internal"])[0] for t in TARGETS)
        paperclip = not have_all and is_paperclip(zf)
    if not have_all:
        if not paperclip:
            print("error: target classes not found in the given jar and it is not a "
                  "paperclip launcher — is this the right kernel jar?", file=sys.stderr)
            return 2
        print("kernel jar is a paperclip launcher (server classes are produced at "
              "first boot in versions/<v>/<server>.jar)")
        if not args.extract_paperclip:
            print("  re-run with --extract-paperclip, or extract manually:")
            print(f"    mkdir /tmp/kernel-extract && cd /tmp/kernel-extract && "
                  f"echo eula=false > eula.txt && java -jar {args.kernel_jar}")
            print("  then pass versions/<ver>/<server>.jar as --kernel-jar")
            return 2
        workdir = os.path.abspath(args.workdir or os.path.join(out_dir, "work"))
        server_jar = paperclip_extract(args.kernel_jar, workdir)
        print(f"extracted  : {server_jar}")
        provenance["server_jar"] = {
            "path": server_jar, "name": os.path.basename(server_jar),
            "size": os.path.getsize(server_jar), "sha256": sha256_file(server_jar),
            "via": "paperclip extract (eula=false -> exits after remap, no boot)",
            "workdir": workdir,
        }
    else:
        provenance["server_jar"] = {"path": os.path.abspath(server_jar),
                                    "name": os.path.basename(server_jar),
                                    "size": os.path.getsize(server_jar),
                                    "sha256": jar_sha, "via": "direct"}

    warnings_all = []
    bridge_warn, bridge_info = check_noise_bridge_bytes(repo_root)
    warnings_all += bridge_warn

    # -- optional JVM-backed noise precompute (ASM REPLACE_BODY) ------------
    workdir = os.path.abspath(args.workdir or os.path.join(out_dir, "work"))
    asm_ctx = None
    asm_note = "disabled (--no-noise-asm)"
    if not args.no_noise_asm:
        try:
            asm_ctx = make_asm_context(repo_root, workdir, args.asm_cp)
            asm_note = f"classpath {asm_ctx['cp']}" if asm_ctx else \
                       "unavailable (no cplug-sdk/asm-lib or asm-build in repo)"
        except RuntimeError as e:
            asm_note = f"unavailable ({e})"
            warnings_all.append(f"JVM-backed noise precompute skipped: {e}")
    print(f"asm setup  : {asm_note}")

    targets_out = {}
    any_problems = []
    with zipfile.ZipFile(server_jar) as zf:
        for t in TARGETS:
            rec = process_target(t, zf, out_dir, args.max_class_major,
                                 args.strict, not args.no_emit_patched,
                                 asm_ctx=asm_ctx)
            fname = f"{t['simple_name']}.json"
            with open(os.path.join(patches_dir, fname), "w") as fh:
                json.dump(rec, fh, indent=2)
                fh.write("\n")
            targets_out[t["key"]] = {"json": fname, **{k: rec[k] for k in
                                                       ("class_internal", "jar_entry")}}
            mj = rec["class_file"]["major"]
            nm = rec["method"]["name"] + rec["method"]["desc"] if rec["method"] else "?"
            print(f"target     : {t['key']}: {rec['class_internal']} major={mj} "
                  f"method={nm} code={rec['method']['code_len'] if rec['method'] else '?'}B "
                  f"insns={rec['method']['instruction_count'] if rec['method'] else '?'}")
            if "patched" in rec:
                print(f"             patched -> {rec['patched']['file']} "
                      f"({rec['class_file']['size']} -> {rec['patched']['size']} bytes, "
                      f"sha256 {rec['patched']['sha256'][:16]}…)")
            for w in rec["warnings"]:
                print(f"  WARN     [{t['key']}] {w}")
                warnings_all.append(f"[{t['key']}] {w}")
            for p in rec["problems"]:
                print(f"  PROBLEM  [{t['key']}] {p}")
                any_problems.append(f"[{t['key']}] {p}")

    manifest = {
        "tool": TOOL,
        "tool_version": TOOL_VERSION,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "provenance": provenance,
        "kernel_jar": provenance["input_jar"],
        "server_jar": provenance["server_jar"],
        "targets": targets_out,
        "guards": {
            "max_class_major": args.max_class_major,
            "java_hint": {v: k for k, v in JAVA_MAJOR.items()}.get(args.max_class_major),
            "noise_bridge_max_major": NOISE_BRIDGE_MAX_MAJOR,
            "noise_bridge": bridge_info,
            "noise_asm": asm_note,
            "strict": args.strict,
        },
        "consumption_contract": (
            "runtime serves patched/ bytes only after sha256(bytes delivered by the "
            "byte hook) == patched.baseline_sha256; on mismatch fall back to the "
            "runtime patcher (kernel build drift)"),
        "warnings": warnings_all,
        "problems": any_problems,
        "files": [],
    }
    # include emitted files with hashes (self-describing artifact bundle)
    for root, _dirs, files in os.walk(out_dir):
        for f in sorted(files):
            p = os.path.join(root, f)
            if os.path.abspath(p) == os.path.abspath(os.path.join(patches_dir, "manifest.json")):
                continue
            manifest["files"].append({
                "path": os.path.relpath(p, out_dir),
                "size": os.path.getsize(p),
                "sha256": sha256_file(p),
            })
    with open(os.path.join(patches_dir, "manifest.json"), "w") as fh:
        json.dump(manifest, fh, indent=2)
        fh.write("\n")

    print(f"outputs    : {patches_dir}/ "
          f"({len(targets_out)} target json + manifest"
          f"{'' if args.no_emit_patched else ' + patched/'}); "
          f"warnings={len(warnings_all)} problems={len(any_problems)}")
    if any_problems:
        return 2
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except ParseError as e:
        print(f"error: {e}", file=sys.stderr)
        sys.exit(2)
    except BrokenPipeError:
        sys.exit(0)
