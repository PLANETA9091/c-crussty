#!/usr/bin/env python3
"""Read biome palette cells for a chunk from anvil region (java-saved chunk).
usage: chunk_biomes.py <region_dir> <cx> <cz> [y]
Prints the biome palette and, for each Y section requested, the 8x8x8 cell grid biomes
(sampled at cell centers) as compact rows.
"""
import sys, os, zlib, struct

def read_region(path, cx, cz):
    rx, rz = cx >> 5, cz >> 5
    fp = os.path.join(path, f"r.{rx}.{rz}.mca")
    if not os.path.exists(fp):
        print("no region", fp); return None
    with open(fp, "rb") as f:
        data = f.read()
    lx, lz = cx & 31, cz & 31
    idx = 4 * ((lx & 31) + (lz & 31) * 32)
    off = int.from_bytes(data[idx:idx+3], "big")
    cnt = data[idx+3]
    if off == 0:
        print("chunk not present in region"); return None
    chunk_data = data[off*4096:(off+cnt)*4096]
    length = struct.unpack(">I", chunk_data[:4])[0]
    comp = chunk_data[4]
    body = chunk_data[5:4+length]
    if comp == 2:
        return zlib.decompress(body)
    elif comp == 1:
        import gzip
        return gzip.decompress(body)
    else:
        return body  # uncompressed

# --- minimal NBT parser (network-safe, big-endian) ---
class NBT:
    def __init__(self, data):
        self.d = data; self.i = 0
    def u1(self): v=self.d[self.i]; self.i+=1; return v
    def i1(self): v=struct.unpack_from(">b",self.d,self.i)[0]; self.i+=1; return v
    def i2(self): v=struct.unpack_from(">h",self.d,self.i)[0]; self.i+=2; return v
    def i4(self): v=struct.unpack_from(">i",self.d,self.i)[0]; self.i+=4; return v
    def i8(self): v=struct.unpack_from(">q",self.d,self.i)[0]; self.i+=8; return v
    def f4(self): v=struct.unpack_from(">f",self.d,self.i)[0]; self.i+=4; return v
    def f8(self): v=struct.unpack_from(">d",self.d,self.i)[0]; self.i+=8; return v
    def s(self):
        n=self.i2(); v=self.d[self.i:self.i+n].decode('utf-8','replace'); self.i+=n; return v
    def payload(self, t):
        if t==0: return None
        if t==1: return self.i1()
        if t==2: return self.i2()
        if t==3: return self.i4()
        if t==4: return self.i8()
        if t==5: return self.f4()
        if t==6: return self.f8()
        if t==7:
            n=self.i4(); v=self.d[self.i:self.i+n]; self.i+=n; return bytearray(v)
        if t==8: return self.s()
        if t==9:
            it=self.u1(); n=self.i4(); return [self.payload(it) for _ in range(n)]
        if t==10:
            out={}
            while True:
                st=self.u1()
                if st==0: break
                name=self.s(); out[name]=self.payload(st)
            return out
        if t==11:
            n=self.i4(); v=list(struct.unpack_from(f">{n}i",self.d,self.i)); self.i+=4*n; return v
        if t==12:
            n=self.i4(); v=list(struct.unpack_from(f">{n}q",self.d,self.i)); self.i+=8*n; return v
        raise ValueError(f"tag {t}")

def main():
    region_dir, cx, cz = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    ylist = [int(y) for y in sys.argv[4:]] or [50]
    raw = read_region(region_dir, cx, cz)
    if raw is None: sys.exit(1)
    nbt = NBT(raw)
    root = nbt.payload(nbt.u1())
    secs = {s["Y"]: s for s in root.get("sections", [])}
    # biome palette per section: biomes: {palette: [...], data: longs?}
    def biome_at(x, y, z):
        # chunk-local x,z 0..15, world y
        sy = (y >> 4) + (-4 if y < 0 else 0)  # sections listed with Y = -4..19 for -64..320
        sec = secs.get(y >> 4)
        if sec is None: return "?"
        bio = sec["biomes"]
        pal = bio["palette"]
        if len(pal) == 1: return pal[0]
        # 4^3 = 64 cells per section
        bits = max(4, (len(pal)-1).bit_length())
        per_long = 64 // bits
        lx, lz = x & 15, z & 15
        cell = ((y & 15) >> 2) * 16 + (lz >> 2) * 4 + (lx >> 2)
        long_i = cell // per_long
        bit_i = (cell % per_long) * bits
        data = bio["data"]
        v = (data[long_i] >> bit_i) & ((1 << bits) - 1)
        if v >= len(pal): return f"OOR({v})"
        return pal[v]
    pal_all = set()
    for sy, sec in secs.items():
        for p in sec["biomes"]["palette"]:
            pal_all.add(p)
    print("palette union:", sorted(pal_all))
    for y in ylist:
        row = []
        for lx in (0, 7, 15):
            for lz in (0, 7, 15):
                row.append(biome_at(lx, y, lz).replace("minecraft:", ""))
        print(f"y={y}: " + " | ".join(row))

if __name__ == "__main__":
    main()
