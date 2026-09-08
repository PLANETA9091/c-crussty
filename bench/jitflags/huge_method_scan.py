#!/usr/bin/env python3
"""TASK-85 static recon: scan a server jar for methods whose Code.length > LIMIT.

HugeMethodLimit default is 8000 bytecodes on HotSpot (develop flag, internal).
With DontCompileHugeMethods=true (product default) such methods never reach C2.

Minimal classfile parser: walks constant pool (utf8 strings only),
fields, methods, and Code attribute headers. No bytecode disassembly.
"""
import struct
import sys
import zipfile
from collections import namedtuple

LIMIT = 8000
Method = namedtuple("Method", "clazz name desc code_len")

def parse_class(data, classes_out, methods_out):
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
            try:
                cp[i] = data[off:off + ln].decode("utf-8", "replace")
            except Exception:
                cp[i] = "?"
            off += ln
        elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Pkg: u2
            off += 2
        elif tag in (15,):  # MethodHandle: u1+u2
            off += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):  # int/float/fieldref/... : u4
            off += 4
        elif tag in (5, 6):  # long/double: u8, 2 slots
            off += 8
            i += 1
        else:
            return  # unknown tag -> bail on this class
        i += 1

    def utf8(idx):
        s = cp[idx] if idx < len(cp) else None
        return s if isinstance(s, str) else "?"

    def classname(idx):
        # CONSTANT_Class -> name_index -> utf8
        entry = cp[idx] if idx < len(cp) else None
        if isinstance(entry, str):
            return entry
        return f"cls#{idx}"

    # We stored raw utf8 in cp; Class entries hold name_index, we skipped storing them.
    # To keep it simple: re-scan is unnecessary; approximate class name from filename.
    off += 2  # access_flags
    this_idx = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2  # this_class, super_class
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc

    # pre-resolve Class entries: we need cp[this_idx] as utf8 idx; we skipped
    # storing Class->name mapping. Store it on the fly instead: handled by
    # classes_out fallback below (filename), good enough for reporting.
    fname = classes_out

    def skip_attributes(off):
        n = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(n):
            off += 2  # name idx
            alen = struct.unpack_from(">I", data, off)[0]
            off += 4 + alen
        return off

    for section in ("field", "method"):
        cnt = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(cnt):
            off += 6  # access, name_idx, desc_idx
            name_idx = struct.unpack_from(">H", data, off - 4)[0]
            desc_idx = struct.unpack_from(">H", data, off - 2)[0]
            nattr = struct.unpack_from(">H", data, off)[0]
            off += 2
            for _ in range(nattr):
                aname_idx = struct.unpack_from(">H", data, off)[0]
                alen = struct.unpack_from(">I", data, off + 2)[0]
                body_off = off + 6
                aname = utf8(aname_idx) if aname_idx < len(cp) else ""
                if section == "method" and aname == "Code" and alen >= 8:
                    code_len = struct.unpack_from(">I", data, body_off + 4)[0]
                    if code_len > LIMIT:
                        methods_out.append(Method(fname, utf8(name_idx), utf8(desc_idx), code_len))
                off = body_off + alen

def main(jar_path, out_path):
    methods = []
    with zipfile.ZipFile(jar_path) as z:
        names = [n for n in z.namelist() if n.endswith(".class")]
        for idx, n in enumerate(names):
            try:
                data = z.read(n)
            except Exception:
                continue
            cls_name = n[:-6].replace("/", ".")
            parse_class(data, cls_name, methods)
            if idx % 4000 == 0:
                print(f"  ... {idx}/{len(names)}", file=sys.stderr)
    methods.sort(key=lambda m: -m.code_len)
    with open(out_path, "w") as f:
        f.write(f"# Huge-method scan: {jar_path}\n")
        f.write(f"# limit: Code.length > {LIMIT} bytecodes (HotSpot HugeMethodLimit default)\n")
        f.write(f"# total .class parsed: {len(names)}; huge methods found: {len(methods)}\n\n")
        f.write("code_len\tclass\tmethod\tdescriptor\n")
        for m in methods:
            f.write(f"{m.code_len}\t{m.clazz}\t{m.name}\t{m.desc}\n")
    print(f"DONE: {len(methods)} huge methods in {len(names)} classes -> {out_path}")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
