#!/usr/bin/env python3
"""Dump CONSTANT_Long / CONSTANT_Double entries (raw bits) from a JVM classfile.

Byte-level verification for NCF T35 (task: f64 bits of BiomeManager.getFiddle
constant + LinearCongruentialGenerator LCG constants must match the Rust port
bit-for-bit, not just CFR's printed decimal).

Usage: python3 cpool_dump.py <path.class> [...]
"""
import struct
import sys


def dump(path: str) -> None:
    with open(path, "rb") as f:
        data = f.read()
    magic, minor, major = struct.unpack_from(">IHH", data, 0)
    assert magic == 0xCAFEBABE, f"{path}: not a classfile (magic={magic:#x})"
    off = 8
    (count,) = struct.unpack_from(">H", data, off)
    off += 2
    print(f"== {path} (classfile {major}.{minor}, {count - 1} constant-pool slots)")

    # Constant pool is 1-indexed; long/double take TWO slots.
    entries: dict[int, tuple[int, int]] = {}
    i = 1
    while i < count:
        tag = data[off]
        off += 1
        if tag == 1:  # Utf8
            (ulen,) = struct.unpack_from(">H", data, off)
            off += 2 + ulen
        elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Package
            off += 2
        elif tag == 15:  # MethodHandle-less: Methodparam? (15 = MethodParameter)
            off += 3
        elif tag in (3, 4):  # Integer/Float
            off += 4
        elif tag in (5, 6):  # Long/Double — 8 bytes, occupies TWO pool slots
            raw = data[off:off + 8]
            entries[i] = (tag, raw)
            off += 8
            i += 1  # extra phantom slot
        elif tag in (9, 10, 11, 12, 17, 18):  # refs / name+type / dynamic
            off += 4
        elif tag == 14:  # MethodHandle
            off += 3
        else:
            raise RuntimeError(f"{path}: unknown cp tag {tag} at pool index {i}")
        i += 1

    for idx in sorted(entries):
        tag, raw = entries[idx]
        kind = "long" if tag == 5 else "double"
        if kind == "long":
            val = struct.unpack(">q", raw)[0]
            print(f"  #{idx}: long {val}  (bits {raw.hex()})")
        else:
            (val,) = struct.unpack(">d", raw)
            print(f"  #{idx}: double {val!r}  (bits {raw.hex()})")


if __name__ == "__main__":
    for p in sys.argv[1:]:
        dump(p)
