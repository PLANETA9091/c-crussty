#!/usr/bin/env python3
"""javap_lite.py — минимальный дизассемблер одного метода class-файла (Java 21, no flags).
Контракт перед ретаргетами (S7-закон): точный список getfield/invokevirtual для sendBlockUpdated."""
import struct, sys

class CP:
    def __init__(self, data):
        self.cp = {}
        cnt, o = struct.unpack_from(">H", data, 8)[0], 10
        self.count = cnt
        i = 1
        while i < cnt:
            tag = data[o]
            if tag == 1:  # Utf8
                ln = struct.unpack_from(">H", data, o+1)[0]
                self.cp[i] = ("utf8", (data[o+3:o+3+ln].decode("utf-8", "replace"),)); o += 3 + ln
            elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Pkg refs
                self.cp[i] = ("ref", struct.unpack_from(">H", data, o+1)[0]); o += 3
            elif tag == 15:  # MethodHandle-ish small
                self.cp[i] = ("mh", data[o+1], struct.unpack_from(">H", data, o+2)[0]); o += 4
            elif tag in (3, 4):  # int/float
                self.cp[i] = ("num", struct.unpack_from(">i" if tag == 3 else ">f", data, o+1)[0]); o += 5
            elif tag in (5, 6):  # long/double
                self.cp[i] = ("num", 0); o += 9; i += 1
            elif tag in (9, 10, 11, 12, 17, 18):  # refs
                self.cp[i] = ("mref", struct.unpack_from(">HH", data, o+1)); o += 5
            else:
                raise ValueError(f"tag {tag} at {i}")
            i += 1
        self.end = o

    def utf8(self, i): return self.cp[i][1][0]
    def cls(self, i): return self.utf8(self.cp[i][1])
    def mref(self, i):
        c, nt = self.cp[i][1]
        n2 = self.cp[nt][1]
        return f"{self.cls(c)}.{self.utf8(n2[0])} {self.utf8(n2[1])}"

def disasm_method(path, owner_hint, method_name):
    data = open(path, "rb").read()
    cp = CP(data)
    o = cp.end
    # skip access/this/super
    o += 6
    ifc, = struct.unpack_from(">H", data, o); o += 2 + 2*ifc
    # fields
    fc, = struct.unpack_from(">H", data, o); o += 2
    for _ in range(fc):
        o += 6
        ac, = struct.unpack_from(">H", data, o); o += 2
        for _ in range(ac):
            o += 2; ln = struct.unpack_from(">I", data, o)[0]; o += 4 + ln
    # methods
    mc, = struct.unpack_from(">H", data, o); o += 2
    for _ in range(mc):
        af, ni, di = struct.unpack_from(">HHH", data, o); o += 6
        ac, = struct.unpack_from(">H", data, o); o += 2
        for _ in range(ac):
            an = struct.unpack_from(">H", data, o)[0]; al = struct.unpack_from(">I", data, o+2)[0]; o += 6
            attr = data[o:o+al]
            if cp.utf8(an) == "Code" and cp.utf8(ni) == method_name:
                clen, mxl, mxv = struct.unpack_from(">HHI", attr, 0)
                code = attr[8:8+clen]
                print(f"# {owner_hint}.{method_name} {cp.utf8(di)} maxstack={mxl} maxlocals={mxv} len={clen}")
                i = 0
                while i < len(code):
                    op = code[i]
                    start = i
                    if op == 0xb4 or op == 0xb2:  # getfield/getstatic
                        idx = struct.unpack_from(">H", code, i+1)[0]
                        kind = "mref" if op == 0xb4 else "mref"
                        try: ref = cp.mref(idx)
                        except Exception: ref = f"?{idx}"
                        print(f"  {start:4d} {'getfield ' if op==0xb4 else 'getstatic'} {ref}")
                        i += 3
                    elif op in (0xb6, 0xb7, 0xb8):  # invokevirtual/special/static
                        idx = struct.unpack_from(">H", code, i+1)[0]
                        try: ref = cp.mref(idx)
                        except Exception: ref = f"?{idx}"
                        print(f"  {start:4d} invoke{'virtual' if op==0xb6 else 'special' if op==0xb7 else 'static'} {ref}")
                        i += 3
                    elif op == 0xbb:  # new
                        idx = struct.unpack_from(">H", code, i+1)[0]
                        print(f"  {start:4d} new {cp.cls(idx)}"); i += 3
                    elif op in (0x19, 0xb4 + 0x100):  # aload n
                        print(f"  {start:4d} aload {code[i+1]}"); i += 2
                    elif 0x1a <= op <= 0x2d:
                        print(f"  {start:4d} load/store local {op:#x}"); i += 1
                    elif op == 0xb0: print(f"  {start:4d} areturn"); i += 1
                    elif op == 0x01: print(f"  {start:4d} aconst_null"); i += 1
                    elif op == 0xa7:  # goto
                        off = struct.unpack_from(">h", code, i+1)[0]
                        print(f"  {start:4d} goto {start+off}"); i += 3
                    elif op == 0xc6:  # ifnull
                        off = struct.unpack_from(">h", code, i+1)[0]
                        print(f"  {start:4d} ifnull {start+off}"); i += 3
                    elif op == 0xc7:  # ifnonnull
                        off = struct.unpack_from(">h", code, i+1)[0]
                        print(f"  {start:4d} ifnonnull {start+off}"); i += 3
                    elif op == 0x59: print(f"  {start:4d} dup"); i += 1
                    elif op == 0x57: print(f"  {start:4d} pop"); i += 1
                    elif op == 0x4c: print(f"  {start:4d} astore_1-ish {op:#x}"); i += 1
                    elif op == 0xb9:  # invokeinterface
                            idx = struct.unpack_from(">H", code, i+1)[0]
                            try: ref = cp.mref(idx)
                            except Exception: ref = f"?{idx}"
                            print(f"  {start:4d} invokeinterface {ref}"); i += 5
                    else:
                        print(f"  {start:4d} op {op:#04x}"); i += 1
                return
            o += al
    print(f"# method {method_name} NOT FOUND in {owner_hint}")

if __name__ == "__main__":
    disasm_method(sys.argv[1], sys.argv[2], sys.argv[3])
