#!/usr/bin/env python3
"""jvms_scan.py — correct JVM code walker with full operand-length table.
Finds NEW (0xBB) / invoke (0xB6-0xB9) sites and resolves their cp targets.
Usage: python3 jvms_scan.py <jar> <class> <method-filter>"""
import struct, sys, zipfile
sys.path.insert(0, "/home/z/c-crussty/scripts/bench4_recon")
from recon13g_callsite import class_info, utf8

# operand byte counts for the opcodes that HAVE operands (all others = 0)
LEN = {}
for ops, ln in (
    ((0x10, 0x11, 0x12, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
      0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0xa9, 0xbc), 1),
    ((0x11, 0x13, 0x14, 0x84, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
      0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xc6, 0xc7), 2),
    ((0x10, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
      0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0xa9), 1),
    ((0x13, 0x14, 0x84, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1,
      0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8,
      0xbb, 0xbd, 0xc0, 0xc1, 0xc6, 0xc7, 0xc5), 2),
    ((0xc8, 0xc9), 4),  # goto_w, jsr_w
    ((0x11, 0x12), 0),  # 0x11=sipush(2) 0x12=ldc(1) handled below
):
    for o in ops:
        LEN[o] = ln
LEN[0x10] = 1   # bipush
LEN[0x11] = 2   # sipush
LEN[0x12] = 1   # ldc
LEN[0x13] = 2   # ldc_w
LEN[0x14] = 2   # ldc2_w
LEN[0x15] = 1   # iload
LEN[0x19] = 1   # aload
LEN[0x32] = 0   # aaload
LEN[0x84] = 2   # iinc
LEN[0xa9] = 1   # ret
LEN[0xbc] = 1   # newarray
LEN[0xc5] = 3   # multianewarray
LEN[0xc6] = 2   # ifnull
LEN[0xc7] = 2   # ifnonnull
INV = {0xB6: "invokevirtual", 0xB7: "invokespecial", 0xB8: "invokestatic", 0xB9: "invokeinterface"}


def walk(code, cp):
    """yield (offset, opcode, resolved_cp_string) for NEW + invokes + getstatic/putstatic refs."""
    j = 0
    n = len(code)
    out = []
    while j < n:
        op = code[j]
        if op in (0xB6, 0xB7, 0xB8):
            idx = struct.unpack_from(">H", code, j + 1)[0]
            out.append((j, INV[op], resolve(cp, idx)))
            j += 3
        elif op == 0xB9:
            idx = struct.unpack_from(">H", code, j + 1)[0]
            out.append((j, "invokeinterface", resolve(cp, idx)))
            j += 5
        elif op == 0xBB:
            idx = struct.unpack_from(">H", code, j + 1)[0]
            out.append((j, "new", resolve_class(cp, idx)))
            j += 3
        elif op == 0xAA:  # tableswitch
            pad = (j + 1) % 4
            base = j + 1 + (4 - pad)
            low, high = struct.unpack_from(">ii", code, base + 4)
            j = base + 12 + (high - low + 1) * 4
        elif op == 0xAB:  # lookupswitch
            pad = (j + 1) % 4
            base = j + 1 + (4 - pad)
            np = struct.unpack_from(">i", code, base + 4)[0]
            j = base + 8 + np * 8
        else:
            j += 1 + LEN.get(op, 0)
    return out


def resolve_class(cp, idx):
    e = cp[idx]
    if e and e[0] == 7:
        n = cp[e[1]]
        return n[1] if n and n[0] == "Utf8" else "?"
    return str(e)


def resolve(cp, idx):
    e = cp[idx] if idx < len(cp) else None
    if not e or e[0] not in (9, 10, 11):
        return resolve_class(cp, idx) if e and e[0] == 7 else str(e)
    cls = resolve_class(cp, e[1][0])
    nt = cp[e[1][1]]
    if nt and nt[0] == 12:
        nm = utf8(cp, nt[1][0])
        ds = utf8(cp, nt[1][1])
        return f"{cls}.{nm}{ds}"
    return f"{cls}?{nt}"


def main():
    jar = zipfile.ZipFile(sys.argv[1])
    cn = sys.argv[2]
    filt = sys.argv[3] if len(sys.argv) > 3 else ""
    data = jar.read(cn)
    name, fields, methods, cp = class_info(data)
    for mn, md, code in methods:
        if filt and filt not in mn:
            continue
        print(f"== {mn} {md} code={len(code)}B")
        for off, kind, tgt in walk(code, cp):
            print(f"   @{off:<4} {kind:<16} {tgt}")


if __name__ == "__main__":
    main()
