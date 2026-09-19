#!/usr/bin/env python3
"""Jar-wide scan: which classes/methods invoke SerializableChunkData.parse.
Usage: python3 recon13g_scan_all.py kernel.jar"""
import zipfile, struct, sys
sys.path.insert(0, "/home/z/c-crussty/scripts/bench4_recon")
from recon13g_callsite import class_info

TARGET_CLASS = "net/minecraft/world/level/chunk/storage/SerializableChunkData"
TARGET_NAME = "parse"


def scan_code(code, hits):
    out = []
    j = 0
    while j < len(code):
        op = code[j]
        if op in (0xB6, 0xB7, 0xB8, 0xB9):
            cpidx = struct.unpack_from(">H", code, j + 1)[0]
            if cpidx in hits:
                kind = {0xB6: "invokevirtual", 0xB7: "invokespecial",
                        0xB8: "invokestatic", 0xB9: "invokeinterface"}[op]
                out.append((j, kind))
            j += 3 if op != 0xB9 else 5
        elif op == 0xAB:
            pad = (j + 1) % 4
            base = j + 1 + (4 - pad)
            np = struct.unpack_from(">i", code, base + 4)[0]
            j = base + 8 + np * 8
        elif op == 0xAA:
            pad = (j + 1) % 4
            base = j + 1 + (4 - pad)
            low, high = struct.unpack_from(">ii", code, base + 4)
            j = base + 12 + (high - low + 1) * 4
        elif op == 0xC4:
            j += 4 if code[j + 1] != 0x84 else 6
        else:
            j += 1
    return out


def main():
    jar = zipfile.ZipFile(sys.argv[1])
    found = 0
    for cn in jar.namelist():
        if not cn.endswith(".class"):
            continue
        data = jar.read(cn)
        if b"SerializableChunkData" not in data:
            continue
        try:
            name, fields, methods, cp = class_info(data)
        except Exception as e:
            print(f"{cn}: class_info fail: {e}")
            continue
        hits = {}
        for idx, e in enumerate(cp):
            if not e or e[0] not in (9, 10, 11):
                continue
            cls = cp[e[1][0]]
            cls_name = None
            if cls and cls[0] == 7:
                cn2 = cp[cls[1]]
                cls_name = cn2[1] if cn2 and cn2[0] == "Utf8" else None
            nt = cp[e[1][1]]
            if cls_name and TARGET_CLASS in cls_name and nt and nt[0] == 12:
                name2 = cp[nt[1][0]]
                desc2 = cp[nt[1][1]]
                if name2 and name2[0] == "Utf8" and name2[1] == "parse":
                    hits[idx] = desc2[1] if desc2 and desc2[0] == "Utf8" else "?" 
        if not hits:
            continue
        found += 1
        print(f"\n== {cn}")
        for idx in sorted(hits):
            print(f"   Methodref idx={idx} desc={hits[idx]}")
        for mname, mdesc, code in methods:
            for off, kind in scan_code(code, hits):
                print(f"   CALL {mname}{mdesc}: {kind}@{off}")
    print(f"\nclasses with parse refs: {found}")


if __name__ == "__main__":
    main()
