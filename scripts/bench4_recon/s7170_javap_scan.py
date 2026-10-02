#!/usr/bin/env python3
"""s7170_javap_scan.py — RECON-22 / S7-170 контракт: инвентарь ВСЕХ методов class-файла,
которые читают/пишут поле navigatingMobs (и isUpdatingNavigations), с типом доступа
(getfield/putfield) и списком вызовов. S7-закон: javap-доказательство до ретаргета.

Usage: python3 s7170_javap_scan.py <path-to.class> [fieldName ...]
"""
import struct, sys


class CP:
    def __init__(self, data):
        self.cp = {}
        cnt, o = struct.unpack_from(">H", data, 8)[0], 10
        self.count = cnt
        i = 1
        while i < cnt:
            tag = data[o]
            if tag == 1:
                ln = struct.unpack_from(">H", data, o + 1)[0]
                self.cp[i] = ("utf8", data[o + 3:o + 3 + ln].decode("utf-8", "replace"))
                o += 3 + ln
            elif tag in (7, 8, 16, 19, 20):
                self.cp[i] = ("ref", struct.unpack_from(">H", data, o + 1)[0]); o += 3
            elif tag == 15:
                self.cp[i] = ("mh", data[o + 1], struct.unpack_from(">H", data, o + 2)[0]); o += 4
            elif tag in (3, 4):
                self.cp[i] = ("num", struct.unpack_from(">i" if tag == 3 else ">f", data, o + 1)[0]); o += 5
            elif tag in (5, 6):
                self.cp[i] = ("num", 0); o += 9; i += 1
            elif tag in (9, 10, 11, 12, 17, 18):
                self.cp[i] = ("mref", struct.unpack_from(">HH", data, o + 1)); o += 5
            else:
                raise ValueError(f"tag {tag} at {i}")
            i += 1
        self.end = o

    def utf8(self, i):
        v = self.cp[i]
        return v[1] if v[0] == "utf8" else self.utf8(v[1])

    def cls(self, i):
        return self.utf8(self.cp[i][1])

    def mref(self, i):
        c, nt = self.cp[i][1]
        n2 = self.cp[nt]
        return f"{self.cls(c)}.{self.utf8(n2[0] if isinstance(n2[1], tuple) is False and False else n2[1] if isinstance(n2[1], tuple) else n2[1])}"
        # (n2 handled below in scan)


# Canonical JVM instruction lengths (opcode -> total bytes); negative = special.
_OPSZ = [1] * 0x100
for _i in list(range(0x10, 0x12)) + [0x12] + list(range(0x15, 0x1a)) + list(range(0x36, 0x3b)) + [0xa9, 0xbc]:
    _OPSZ[_i] = 2                                  # bipush, ldc, iload..aload, istore..astore, ret, newarray
for _i in list(range(0x99, 0xa9)) + [0xc6, 0xc7] + list(range(0xb2, 0xb9)) + [0xbb, 0xbd, 0xc0, 0xc1]:
    _OPSZ[_i] = 3                                  # branches, getstatic..invokestatic, new, anewarray, checkcast, instanceof
for _i in [0x11, 0x13, 0x14, 0x84]:
    _OPSZ[_i] = 3                                  # sipush, ldc_w, ldc2_w, iinc
_OPSZ[0xc5] = 4                                    # multianewarray
_OPSZ[0xb9] = 5                                    # invokeinterface
_OPSZ[0xba] = 5                                    # invokedynamic
_OPSZ[0xc4] = -1                                   # wide
_OPSZ[0xaa] = -2                                   # tableswitch
_OPSZ[0xab] = -3                                   # lookupswitch
_OPSZ = tuple(_OPSZ)


def _step(code, i):
    """Return next index after the instruction at i."""
    op = code[i]
    n = _OPSZ[op]
    if n >= 0:
        return i + n
    if n == -1:  # wide
        return i + (6 if code[i + 1] == 0x84 else 4)
    # switches: pad to 4-byte alignment from i+1
    p = (i + 4) & ~3
    if op == 0xaa:
        lo, hi = struct.unpack_from(">ii", code, p + 4)
        return p + 12 + (hi - lo + 1) * 4
    npairs = struct.unpack_from(">i", code, p + 4)[0]
    return p + 8 + npairs * 8


def scan(path, fields):
    data = open(path, "rb").read()
    cp = CP(data)

    def name_of(nt_idx):
        n2 = cp.cp[nt_idx]
        # NameAndType stored as ("mref", (name_idx, desc_idx))
        a, b = n2[1]
        return cp.utf8(a), cp.utf8(b)

    def ref_str(idx):
        c, nt = cp.cp[idx][1]
        n, d = name_of(nt)
        return f"{cp.cls(c)}.{n} {d}"

    o = cp.end
    o += 6  # access/this/super
    ifc, = struct.unpack_from(">H", data, o); o += 2 + 2 * ifc
    fc, = struct.unpack_from(">H", data, o); o += 2
    for _ in range(fc):
        o += 6
        ac, = struct.unpack_from(">H", data, o); o += 2
        for _ in range(ac):
            o += 2
            ln = struct.unpack_from(">I", data, o)[0]; o += 4 + ln
    mc, = struct.unpack_from(">H", data, o); o += 2
    print(f"# {path} methods={mc} scan_fields={fields}")
    hits = 0
    for _ in range(mc):
        af, ni, di = struct.unpack_from(">HHH", data, o); o += 6
        mname, mdesc = cp.utf8(ni), cp.utf8(di)
        ac, = struct.unpack_from(">H", data, o); o += 2
        for _ in range(ac):
            an = struct.unpack_from(">H", data, o)[0]
            al = struct.unpack_from(">I", data, o + 2)[0]; o += 6
            attr = data[o:o + al]
            o += al
            if cp.utf8(an) != "Code":
                continue
            clen, mxl, mxv = struct.unpack_from(">HHI", attr, 0)
            code = attr[8:8 + clen]
            found = []
            i = 0
            while i < len(code):
                op = code[i]
                if op in (0xb2, 0xb3, 0xb4, 0xb5):  # getstatic/putstatic/getfield/putfield
                    idx = struct.unpack_from(">H", code, i + 1)[0]
                    try:
                        rs = ref_str(idx)
                    except Exception:
                        rs = f"?{idx}"
                    for f in fields:
                        if "." in rs and rs.split(".", 1)[1].split(" ")[0] == f:
                            kind = {0xb2: "getstatic", 0xb3: "putstatic",
                                    0xb4: "getfield", 0xb5: "putfield"}[op]
                            found.append(f"{kind} {rs}")
                    i += 3
                else:
                    i = _step(code, i)
            if found:
                hits += 1
                flags = []
                if af & 0x0001: flags.append("public")
                if af & 0x0002: flags.append("private")
                if af & 0x0004: flags.append("protected")
                if af & 0x0008: flags.append("static")
                if af & 0x0010: flags.append("final")
                if af & 0x0100: flags.append("synchronized")
                print(f"\n## {mname} {mdesc}  [{' '.join(flags)}] codelen={clen}")
                for f in found:
                    print(f"   {f}")
    print(f"\n# total methods touching scan fields: {hits}")


if __name__ == "__main__":
    scan(sys.argv[1], sys.argv[2:] or ["navigatingMobs", "isUpdatingNavigations"])
