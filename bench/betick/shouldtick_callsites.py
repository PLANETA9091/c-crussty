#!/usr/bin/env python3
"""TASK-89 static recon: find every constant-pool call site of
`shouldTickBlocksAt` (and a configurable extra-method list) in a server jar.

Census-first discipline (TASK-85 lineage): the cheapest refutation/GO evidence
is static. A CP Methodref/InterfaceMethodref entry is the exact declared call
site the verifier links against; invokedynamic-embedded refs still appear via
BootstrapMethods MethodHandles. Zero boots, zero code changes.

Full CP resolution: Utf8/Class/NameAndType/Methodref(9,10,11).
Runs in seconds over the 9,809-class Purpur jar.
"""
import struct
import sys
import zipfile
from collections import Counter, namedtuple

TARGETS = {"shouldTickBlocksAt"}  # method names of interest (call targets)
Ref = namedtuple("Ref", "caller target_owner target_name target_desc kind")


def parse_class(data, cls_name, refs_out):
    if len(data) < 10 or data[:4] != b"\xca\xfe\xba\xbe":
        return
    off = 8  # magic(4) minor(2) major(2)
    cp_count = struct.unpack_from(">H", data, off)[0]
    off += 2
    cp = [None] * cp_count  # 1-based
    i = 1
    while i < cp_count:
        tag = data[off]
        off += 1
        if tag == 1:  # Utf8
            ln = struct.unpack_from(">H", data, off)[0]
            off += 2
            cp[i] = ("U", data[off:off + ln].decode("utf-8", "replace"))
            off += ln
        elif tag == 7:  # Class
            cp[i] = ("C", struct.unpack_from(">H", data, off)[0])
            off += 2
        elif tag in (9, 10, 11):  # Fieldref/Methodref/InterfaceMethodref
            c, n = struct.unpack_from(">HH", data, off)
            cp[i] = ("R", tag, c, n)
            off += 4
        elif tag == 12:  # NameAndType
            a, b = struct.unpack_from(">HH", data, off)
            cp[i] = ("N", a, b)
            off += 4
        elif tag == 8:  # String
            cp[i] = ("S", struct.unpack_from(">H", data, off)[0])
            off += 2
        elif tag == 15:  # MethodHandle
            off += 3
        elif tag == 16:  # MethodType
            off += 2
        elif tag in (17, 18):  # Dynamic/InvokeDynamic
            off += 4
        elif tag in (3, 4):  # int/float
            off += 4
        elif tag in (5, 6):  # long/double
            off += 8
            i += 1
        elif tag in (19, 20):  # Module/Package
            off += 2
        else:
            return  # unknown tag -> bail on this class
        i += 1

    # resolve refs — no closure indirection, direct tuple checks
    for e in cp:
        if e and e[0] == "R":
            _, tag, cidx, natidx = e
            cent = cp[cidx] if cidx < cp_count else None
            owner = cp[cent[1]][1] if cent and cent[0] == "C" and cp[cent[1]] else "?"
            nent = cp[natidx] if natidx < cp_count else None
            if not (nent and nent[0] == "N"):
                continue
            n1 = cp[nent[1]] if nent[1] < cp_count else None
            d1 = cp[nent[2]] if nent[2] < cp_count else None
            name = n1[1] if n1 and n1[0] == "U" else "?"
            desc = d1[1] if d1 and d1[0] == "U" else "?"
            if name in TARGETS:
                kind = "field" if tag == 9 else ("method" if tag == 10 else "imethod")
                refs_out.append(Ref(cls_name, owner, name, desc, kind))


def main(jar_path, out_path):
    refs = []
    n_cls = 0
    with zipfile.ZipFile(jar_path) as z:
        names = [n for n in z.namelist() if n.endswith(".class")]
        for idx, n in enumerate(names):
            try:
                data = z.read(n)
            except Exception:
                continue
            parse_class(data, n[:-6].replace("/", "."), refs)
            n_cls += 1
            if idx % 4000 == 0:
                print(f"  ... {idx}/{len(names)}", file=sys.stderr)
    with open(out_path, "w") as f:
        f.write(f"# shouldTickBlocksAt call-site census: {jar_path}\n")
        f.write(f"# classes parsed: {n_cls}; CP refs found: {len(refs)}\n")
        f.write("# caller_class\ttarget\tkind\tdescriptor\n")
        for r in sorted(refs):
            f.write(f"{r.caller}\t{r.target_owner}.{r.target_name}\t{r.kind}\t{r.target_desc}\n")
    by_caller = Counter(f"{r.caller} -> {r.target_owner}" for r in refs)
    print(f"classes={n_cls} refs={len(refs)}")
    for k, v in sorted(by_caller.items()):
        print(f"  {v:3d}x  {k}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
