#!/usr/bin/env python3
"""recon13g_callsite.py — find SerializableChunkData.parse call sites in the
kernel jar via constant-pool scan (javap unavailable in the sandbox).

For each candidate class: parse cp (JVMS 4.4/4.5/4.6), list Methodref entries
whose class = SerializableChunkData and name = parse, and which method of the
class contains the invokestatic (needs code scan for the cp index).
Usage: python3 recon13g_callsite.jar kernel.jar
"""
import struct, sys, zipfile

TARGET_CLASS = "net/minecraft/world/level/chunk/storage/SerializableChunkData"
TARGET_NAME = "parse"


def cp_parse(data):
    n = struct.unpack_from(">H", data, 8)[0]
    cp = [None] * n  # 1-based, long/double take TWO slots
    i = 10
    j = 1
    while j < n:
        tag = data[i]
        if tag == 1:  # Utf8
            ln = struct.unpack_from(">H", data, i + 1)[0]
            cp[j] = ("Utf8", data[i + 3:i + 3 + ln].decode("utf-8", "replace"))
            i += 3 + ln
        elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Pkg
            cp[j] = (tag, struct.unpack_from(">H", data, i + 1)[0])
            i += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            cp[j] = (tag, struct.unpack_from(">HH", data, i + 1))
            i += 5
        elif tag in (5, 6):  # long/double: slot + phantom
            cp[j] = (tag, struct.unpack_from(">Q", data, i + 1))
            cp[j + 1] = None
            j += 1  # extra slot
            i += 9
        elif tag == 15:  # MethodHandle
            cp[j] = (tag, struct.unpack_from(">BH", data, i + 1))
            i += 4
        else:
            raise ValueError(f"bad cp tag {tag} at {i} (slot {j})")
        j += 1
    return cp, i


def utf8(cp, idx):
    e = cp[idx]
    return e[1] if e and e[0] == "Utf8" else None


def entry_str(cp, idx):
    e = cp[idx] if 0 < idx < len(cp) else None
    if not e:
        return "?"
    if e[0] == 7:
        return utf8(cp, e[1])
    if e[0] in (9, 10, 11):
        cls = utf8(cp, e[1][0])
        nt = cp[e[1][1]]
        return f"{cls}.{utf8(cp, nt[1][0])}:{utf8(cp, nt[1][1])}"
    if e[0] == 12:
        return f"{utf8(cp, e[1][0])}:{utf8(cp, e[1][1])}"
    return str(e)


def class_info(data):
    """-> (name, fields[], methods[(name, desc, code)]) minimal."""
    cp, i = cp_parse(data)
    acc, this = struct.unpack_from(">HH", data, i)
    name = entry_str(cp, this)
    i += 2  # access_flags
    i += 2  # this_class
    i += 2  # super_class
    ifc = struct.unpack_from(">H", data, i)[0]
    i += 2 + 2 * ifc
    fields = []
    fn = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(fn):
        fa, fni, fdi = struct.unpack_from(">HHH", data, i)
        i += 6
        at = struct.unpack_from(">H", data, i)[0]
        i += 2
        for _ in range(at):
            ani = struct.unpack_from(">H", data, i)[0]
            al = struct.unpack_from(">I", data, i + 2)[0]
            i += 6 + al
        fields.append((utf8(cp, fni), utf8(cp, fdi)))
    methods = []
    mn = struct.unpack_from(">H", data, i)[0]
    i += 2
    for _ in range(mn):
        ma, mni, mdi = struct.unpack_from(">HHH", data, i)
        i += 6
        at = struct.unpack_from(">H", data, i)[0]
        i += 2
        code = b""
        for _ in range(at):
            ani = struct.unpack_from(">H", data, i)[0]
            al = struct.unpack_from(">I", data, i + 2)[0]
            body = data[i + 6:i + 6 + al]
            if utf8(cp, ani) == "Code":
                cl = struct.unpack_from(">I", body, 4)[0]
                code = body[10:10 + cl]
            i += 6 + al
        methods.append((utf8(cp, mni), utf8(cp, mdi), code))
    return name, fields, methods, cp


def main():
    jar = sys.argv[1]
    cands = [
        "ca/spottedleaf/moonrise/patches/chunk_system/scheduling/task/ChunkLoadTask$ChunkDataLoadTask.class",
        "ca/spottedleaf/moonrise/patches/chunk_system/scheduling/task/ChunkLoadTask$CallbackDataLoadTask.class",
        "ca/spottedleaf/moonrise/patches/chunk_system/scheduling/task/GenericDataLoadTask$LoadDataFromDiskTask.class",
        "ca/spottedleaf/moonrise/patches/chunk_system/scheduling/task/GenericDataLoadTask$ProcessOffMainTask.class",
        "ca/spottedleaf/moonrise/patches/chunk_system/scheduling/task/GenericDataLoadTask.class",
        "net/minecraft/world/level/chunk/storage/SerializableChunkData.class",
    ]
    with zipfile.ZipFile(jar) as z:
        for cn in cands:
            try:
                data = z.read(cn)
            except KeyError:
                continue
            name, fields, methods, cp = class_info(data)
            # find cp indexes of Methodref SerializableChunkData.parse
            hits = {}
            for idx, e in enumerate(cp):
                if e and e[0] == 10:
                    cls = utf8(cp, e[1][0])
                    nt = cp[e[1][1]]
                    if cls and TARGET_CLASS in cls and utf8(cp, nt[1][0]) == TARGET_NAME:
                        hits[idx] = utf8(cp, nt[1][1])
            print(f"\n== {cn} ({len(fields)} fields, {len(methods)} methods)")
            if not hits:
                print("   no parse refs in cp")
                continue
            for desc in hits.values():
                print(f"   Methodref -> {TARGET_CLASS}.{TARGET_NAME}{desc}")
            # scan Code of each method for the invokestatic (0xB8) / invokevirtual(0xB6) with those indexes
            for mname, mdesc, code in methods:
                found = []
                j = 0
                while j < len(code):
                    op = code[j]
                    if op in (0xB6, 0xB7, 0xB8, 0xB9):
                        cpidx = struct.unpack_from(">H", code, j + 1)[0]
                        if cpidx in hits:
                            kind = {0xB6: "invokevirtual", 0xB7: "invokespecial",
                                    0xB8: "invokestatic", 0xB9: "invokeinterface"}[op]
                            found.append((j, kind))
                        j += 3 if op != 0xB9 else 5
                    elif op in (0xC4,):  # wide
                        j += 4 if code[j + 1] != 0x84 else 6
                    elif op == 0xAB:  # lookupswitch (variable)
                        pad = (j + 1) % 4
                        base = j + 1 + (4 - pad)
                        npairs = struct.unpack_from(">i", code, base + 4)[0]
                        j = base + 8 + npairs * 8
                    elif op == 0xAA:  # tableswitch
                        pad = (j + 1) % 4
                        base = j + 1 + (4 - pad)
                        low, high = struct.unpack_from(">ii", code, base + 4)
                        j = base + 12 + (high - low + 1) * 4
                    else:
                        j += 1
                if found:
                    print(f"   CALLER: {mname}{mdesc} -> " +
                          ", ".join(f"{k}@{o}" for o, k in found))
            # field list for the clone design
            print("   fields:")
            for fn, fd in fields:
                print(f"     {fd} {fn}")


if __name__ == "__main__":
    main()
