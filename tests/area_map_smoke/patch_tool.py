#!/usr/bin/env python3
"""Byte-exact Python mirror of src/classfile.rs::patch_update.

The shipped Rust patcher cannot be called headlessly without cargo (and the
crussty crate only exposes patch_update through its JVMTI hook / unit tests),
so this tool reproduces the patch for the headless area-map smoke test
(tests/area_map_smoke/run.sh). The algorithm, constant-pool append order and
emitted bytecode are mirrors of the Rust source; run.sh --gold compares the
output byte-for-byte against the cargo-produced gold bytes
(cargo test classfile::tests::patch_roundtrip dumps them to
/tmp/ccrussty_patched_SingleUserAreaMap.class), so a silent drift between the
two implementations is impossible to miss.

Semantics (see src/classfile.rs module docs for the full annotated body):
  - constant pool is parsed and REUSED (append-only: new entries are added at
    the tail, existing (tag, payload) entries are shared),
  - the body of update(III)Z is replaced wholesale by an 82-byte body that
    preserves the original contract:
      negative newDistance -> IllegalArgumentException("...") + athrow,
      lastChunkX == NOT_SET (i32::MIN) -> return false, fields untouched,
      otherwise: write lastChunkX/lastChunkZ/distance, then invokestatic
      SingleUserAreaMapOps.run(LSingleUserAreaMap;IIIIIILjava/lang/Object;)V
      with the snapshot (fromX, fromZ, oldD) and the new state, return true,
  - the new Code carries a StackMapTable (same_frame @16, append_frame @44)
    as required for class-file major >= 51.

Usage:
    python3 patch_tool.py --fixture tests/fixtures/SingleUserAreaMap.class \
                          --out    tests/area_map_smoke/build/patched_SingleUserAreaMap.class \
                        [--verify /path/to/gold.class]   # exit 2 on byte mismatch
"""

from __future__ import annotations

import argparse
import hashlib
import struct
import sys

# ---------------------------------------------------------------------------
# Constant-pool tags (subset used here)
# ---------------------------------------------------------------------------
TAG_UTF8 = 1
TAG_INTEGER = 3
TAG_CLASS = 7
TAG_FIELDREF = 9
TAG_METHODREF = 10
TAG_NAMEANDTYPE = 12

MAP_CLASS = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap"
OPS_CLASS = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps"
NIE = "java/lang/IllegalArgumentException"
INTEGER = "java/lang/Integer"


class PatchError(Exception):
    pass


class Pool:
    """Mirror of classfile.rs::Pool (parse / find / push / serialize).

    entries: list of (apparent_index, tag, payload) in insertion order; long /
    double entries occupy two apparent slots but are a single list element.
    """

    __slots__ = ("entries", "next")

    def __init__(self) -> None:
        self.entries: list[tuple[int, int, bytes]] = []
        self.next = 1  # index the NEXT appended entry gets

    # -- parse ---------------------------------------------------------------
    @classmethod
    def parse(cls, data: bytes, cp_start: int, cp_count: int) -> tuple["Pool", int]:
        pool = cls()
        p = cp_start
        seen = 0
        entries_total = cp_count - 1  # index 0 is reserved
        if cp_count < 1:
            raise PatchError("bad constant_pool_count")
        while seen < entries_total:
            if p >= len(data):
                raise PatchError("truncated constant pool")
            tag = data[p]
            p += 1
            if tag == TAG_UTF8:
                if p + 2 > len(data):
                    raise PatchError("truncated utf8")
                (length,) = struct.unpack_from(">H", data, p)
                payload = data[p : p + 2 + length]
                if len(payload) != 2 + length:
                    raise PatchError("truncated utf8 body")
                p += 2 + length
                slots = 1
            elif tag in (TAG_INTEGER, 4):  # Integer | Float
                payload = data[p : p + 4]
                if len(payload) != 4:
                    raise PatchError("truncated int/float")
                p += 4
                slots = 1
            elif tag in (5, 6):  # Long | Double
                payload = data[p : p + 8]
                if len(payload) != 8:
                    raise PatchError("truncated long/double")
                p += 8
                slots = 2
            elif tag in (7, 8, 16, 19, 20):  # Class | String | (u2 payload kinds) | Module | Package
                payload = data[p : p + 2]
                if len(payload) != 2:
                    raise PatchError("truncated u2 entry")
                p += 2
                slots = 1
            elif tag in (9, 10, 11, 12, 17, 18):  # refs | NameAndType | Dynamic/InvokeDynamic
                payload = data[p : p + 4]
                if len(payload) != 4:
                    raise PatchError("truncated u4 entry")
                p += 4
                slots = 1
            elif tag == 15:  # MethodHandle
                payload = data[p : p + 3]
                if len(payload) != 3:
                    raise PatchError("truncated method handle")
                p += 3
                slots = 1
            else:
                raise PatchError(f"unknown cp tag {tag}")
            pool.entries.append((pool.next, tag, payload))
            if pool.next + slots > 0xFFFF:
                # Rust: checked_add -> None -> parse fails; mirror as error
                raise PatchError("constant pool index overflow")
            pool.next += slots
            seen += 1
        return pool, p

    # -- lookup / append ------------------------------------------------------
    def find(self, tag: int, payload: bytes) -> int | None:
        for idx, t, d in self.entries:
            if t == tag and d == payload:
                return idx
        return None

    def push(self, tag: int, payload: bytes, slots: int) -> int:
        idx = self.next
        self.entries.append((idx, tag, payload))
        self.next = min(self.next + slots, 0xFFFF + 1)  # saturate like A4-F3
        return idx

    def utf8(self, s: str) -> int:
        encoded = s.encode("utf-8")
        payload = struct.pack(">H", len(encoded)) + encoded
        idx = self.find(TAG_UTF8, payload)
        return idx if idx is not None else self.push(TAG_UTF8, payload, 1)

    def int_const(self, v: int) -> int:
        payload = struct.pack(">i", v)
        idx = self.find(TAG_INTEGER, payload)
        return idx if idx is not None else self.push(TAG_INTEGER, payload, 1)

    def class_of(self, utf8_idx: int) -> int:
        payload = struct.pack(">H", utf8_idx)
        idx = self.find(TAG_CLASS, payload)
        return idx if idx is not None else self.push(TAG_CLASS, payload, 1)

    def name_and_type(self, name: str, desc: str) -> int:
        n = self.utf8(name)
        d = self.utf8(desc)
        payload = struct.pack(">HH", n, d)
        idx = self.find(TAG_NAMEANDTYPE, payload)
        return idx if idx is not None else self.push(TAG_NAMEANDTYPE, payload, 1)

    def field_ref(self, owner: str, name: str, desc: str) -> int:
        owner_utf8 = self.utf8(owner)
        c = self.class_of(owner_utf8)
        nat = self.name_and_type(name, desc)
        payload = struct.pack(">HH", c, nat)
        idx = self.find(TAG_FIELDREF, payload)
        return idx if idx is not None else self.push(TAG_FIELDREF, payload, 1)

    def method_ref(self, owner: str, name: str, desc: str) -> int:
        owner_utf8 = self.utf8(owner)
        c = self.class_of(owner_utf8)
        nat = self.name_and_type(name, desc)
        payload = struct.pack(">HH", c, nat)
        idx = self.find(TAG_METHODREF, payload)
        return idx if idx is not None else self.push(TAG_METHODREF, payload, 1)

    def utf8_value(self, idx: int) -> str | None:
        for i, tag, payload in self.entries:
            if i == idx:
                if tag != TAG_UTF8:
                    return None
                (length,) = struct.unpack_from(">H", payload, 0)
                return payload[2 : 2 + length].decode("utf-8")
        return None

    def serialize(self) -> bytes:
        out = bytearray()
        for _, tag, payload in self.entries:
            out.append(tag)
            out += payload
        return bytes(out)


# ---------------------------------------------------------------------------
# Layout parsing (mirror of parse_layout / find_method, bounds-checked)
# ---------------------------------------------------------------------------
def u16_at(data: bytes, p: int) -> int:
    if p + 2 > len(data):
        raise PatchError(f"u16 read out of bounds at {p}")
    return struct.unpack_from(">H", data, p)[0]


def u32_at(data: bytes, p: int) -> int:
    if p + 4 > len(data):
        raise PatchError(f"u32 read out of bounds at {p}")
    return struct.unpack_from(">I", data, p)[0]


class Method:
    __slots__ = ("start", "end", "name_idx", "desc_idx", "access")

    def __init__(self, start: int, end: int, name_idx: int, desc_idx: int, access: int) -> None:
        self.start = start
        self.end = end
        self.name_idx = name_idx
        self.desc_idx = desc_idx
        self.access = access


class Layout:
    __slots__ = ("pool", "cp_end", "this_class_idx", "methods_start")

    def __init__(self, pool: Pool, cp_end: int, this_class_idx: int, methods_start: int) -> None:
        self.pool = pool
        self.cp_end = cp_end
        self.this_class_idx = this_class_idx
        self.methods_start = methods_start


def parse_layout(data: bytes) -> Layout:
    if len(data) < 10:
        raise PatchError("classfile shorter than 10 bytes")
    if struct.unpack_from(">I", data, 0)[0] != 0xCAFEBABE:
        raise PatchError("bad magic")
    if u16_at(data, 6) < 51:
        raise PatchError("major < 51: StackMapTable unsupported")
    cp_count = u16_at(data, 8)
    pool, cp_end = Pool.parse(data, 10, cp_count)
    this_class_idx = u16_at(data, cp_end + 2)
    p = cp_end + 6  # access_flags(2) + this_class(2) + super_class(2)
    iface_count = u16_at(data, p)
    p += 2 + 2 * iface_count
    fields_count = u16_at(data, p)
    p += 2
    for _ in range(fields_count):
        p += 6
        attr_count = u16_at(data, p)
        p += 2
        for _ in range(attr_count):
            length = u32_at(data, p + 2)
            p += 6 + length
    return Layout(pool, cp_end, this_class_idx, p)


def this_class_name(layout: Layout) -> str | None:
    for idx, tag, payload in layout.pool.entries:
        if idx == layout.this_class_idx:
            if tag != TAG_CLASS:
                return None
            (utf8_idx,) = struct.unpack_from(">H", payload, 0)
            return layout.pool.utf8_value(utf8_idx)
    return None


def find_method(data: bytes, methods_start: int, name_idx: int, desc_idx: int) -> Method:
    p = methods_start
    count = u16_at(data, p)
    p += 2
    for _ in range(count):
        start = p
        access = u16_at(data, p)
        n = u16_at(data, p + 2)
        d = u16_at(data, p + 4)
        p += 6
        attr_count = u16_at(data, p)
        p += 2
        for _ in range(attr_count):
            length = u32_at(data, p + 2)
            p += 6 + length
        if n == name_idx and d == desc_idx:
            return Method(start, p, n, d, access)
    raise PatchError("update(III)Z not found")


# ---------------------------------------------------------------------------
# The patch itself (mirror of classfile.rs::patch_update)
# ---------------------------------------------------------------------------
def patch_update(data: bytes) -> bytes:
    layout = parse_layout(data)
    this_name = this_class_name(layout)
    if this_name != MAP_CLASS:
        raise PatchError(f"unexpected class {this_name}")
    pool = layout.pool

    update_name = pool.utf8("update")
    update_desc = pool.utf8("(III)Z")
    m = find_method(data, layout.methods_start, update_name, update_desc)

    # Constant refs needed by the new body (appended when absent) -- the ORDER
    # of these calls mirrors patch_update exactly so appended cp indices match.
    f_last_x = pool.field_ref(this_name, "lastChunkX", "I")
    f_last_z = pool.field_ref(this_name, "lastChunkZ", "I")
    f_dist = pool.field_ref(this_name, "distance", "I")
    f_param = pool.field_ref(this_name, "parameter", "Ljava/lang/Object;")
    nie_utf8 = pool.utf8(NIE)
    cls_nie = pool.class_of(nie_utf8)
    m_nie_init = pool.method_ref(NIE, "<init>", "(Ljava/lang/String;)V")
    m_to_str = pool.method_ref(INTEGER, "toString", "(I)Ljava/lang/String;")
    min_int = pool.int_const(-2**31)
    run_desc = f"(L{this_name};IIIIIILjava/lang/Object;)V"
    m_run = pool.method_ref(OPS_CLASS, "run", run_desc)
    if pool.next > 0xFFFF - 16:
        raise PatchError("constant pool overflow: no index space left for patched refs")

    def u2(out: bytearray, v: int) -> None:
        out += struct.pack(">H", v)

    code = bytearray()
    # A: throw guard
    code.append(0x1D)  # iload_3
    code += bytes([0x9C, 0x00, 0x0F])  # ifge +15 -> 16
    code.append(0xBB)  # new
    u2(code, cls_nie)
    code.append(0x59)  # dup
    code.append(0x1D)  # iload_3
    code.append(0xB8)  # invokestatic
    u2(code, m_to_str)
    code.append(0xB7)  # invokespecial
    u2(code, m_nie_init)
    code.append(0xBF)  # athrow
    if len(code) != 16:
        raise PatchError(f"guard block is {len(code)} bytes, expected 16")
    # B: snapshot old state
    code.append(0x2A)  # aload_0
    code.append(0xB4)  # getfield
    u2(code, f_last_x)
    code += bytes([0x36, 0x04])  # istore 4
    code.append(0x2A)
    code.append(0xB4)
    u2(code, f_last_z)
    code += bytes([0x36, 0x05])
    code.append(0x2A)
    code.append(0xB4)
    u2(code, f_dist)
    code += bytes([0x36, 0x06])
    # C: NOT_SET check
    code += bytes([0x15, 0x04])  # iload 4
    code.append(0x13)  # ldc_w
    u2(code, min_int)
    code += bytes([0xA0, 0x00, 0x05])  # if_icmpne +5 -> 44
    code.append(0x03)  # iconst_0
    code.append(0xAC)  # ireturn
    if len(code) != 44:
        raise PatchError(f"pre-run block is {len(code)} bytes, expected 44")
    # D: write fields, run ops
    code.append(0x2A)  # aload_0
    code.append(0x1B)  # iload_1
    code.append(0xB5)  # putfield
    u2(code, f_last_x)
    code.append(0x2A)
    code.append(0x1C)  # iload_2
    code.append(0xB5)
    u2(code, f_last_z)
    code.append(0x2A)
    code.append(0x1D)  # iload_3
    code.append(0xB5)
    u2(code, f_dist)
    code.append(0x2A)
    code.append(0xB4)  # getfield parameter
    u2(code, f_param)
    code += bytes([0x3A, 0x07])  # astore 7
    code.append(0x2A)  # aload_0
    code += bytes([0x15, 0x04])  # iload 4
    code += bytes([0x15, 0x05])  # iload 5
    code += bytes([0x15, 0x06])  # iload 6
    code.append(0x1B)  # iload_1
    code.append(0x1C)  # iload_2
    code.append(0x1D)  # iload_3
    code += bytes([0x19, 0x07])  # aload 7
    code.append(0xB8)  # invokestatic
    u2(code, m_run)
    code.append(0x04)  # iconst_1
    code.append(0xAC)  # ireturn
    if len(code) != 82:
        raise PatchError(f"emitted code is {len(code)} bytes, expected 82")

    # StackMapTable: same_frame @16 + append_frame @44 (offset_delta 27)
    stackmap = struct.pack(">H", 2) + bytes([0x10]) + bytes([0xFE, 0x00, 0x1B, 0x01, 0x01, 0x01])

    # Code attribute
    body = bytearray()
    u2(body, 8)  # max_stack
    u2(body, 8)  # max_locals
    body += struct.pack(">I", len(code))
    body += code
    body += bytes([0, 0])  # exception_table_length
    body += struct.pack(">H", 1)  # attributes_count
    u2(body, pool.utf8("StackMapTable"))
    body += struct.pack(">I", len(stackmap))
    body += stackmap
    code_attr = bytearray()
    u2(code_attr, pool.utf8("Code"))
    code_attr += struct.pack(">I", len(body))
    code_attr += body

    # replacement method entry
    method = bytearray()
    u2(method, m.access)
    u2(method, m.name_idx)
    u2(method, m.desc_idx)
    u2(method, 1)  # attributes_count
    method += code_attr

    # splice: header + new cp + tail with the update method replaced
    out = bytearray()
    out += data[0:8]
    u2(out, pool.next)  # new cp_count
    out += pool.serialize()
    out += data[layout.cp_end : m.start]
    out += method
    out += data[m.end :]
    return bytes(out)


def main() -> int:
    ap = argparse.ArgumentParser(description="Headless reimplementation of classfile::patch_update")
    ap.add_argument("--fixture", required=True, help="original SingleUserAreaMap.class")
    ap.add_argument("--out", required=True, help="where to write the patched classfile")
    ap.add_argument("--verify", help="optional gold patched classfile to byte-compare against")
    args = ap.parse_args()

    data = open(args.fixture, "rb").read()
    patched = patch_update(data)
    with open(args.out, "wb") as f:
        f.write(patched)
    sha = hashlib.sha256(patched).hexdigest()
    print(f"patch_tool: {args.fixture} ({len(data)} B) -> {args.out} ({len(patched)} B)")
    print(f"patch_tool: patched sha256 {sha}")
    # provenance canary: the original must NOT reference the Ops class, the
    # patch must add exactly that reference.
    if b"SingleUserAreaMapOps" in data:
        print("patch_tool: WARNING original fixture already references SingleUserAreaMapOps", file=sys.stderr)
    if b"SingleUserAreaMapOps" not in patched:
        print("patch_tool: FATAL patched bytes lost the SingleUserAreaMapOps reference", file=sys.stderr)
        return 1

    if args.verify:
        gold = open(args.verify, "rb").read()
        if gold == patched:
            print(f"patch_tool: BYTE-PARITY with gold OK ({len(gold)} B, sha {hashlib.sha256(gold).hexdigest()})")
        else:
            print(
                f"patch_tool: BYTE MISMATCH vs gold: python {len(patched)} B / gold {len(gold)} B",
                file=sys.stderr,
            )
            return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
