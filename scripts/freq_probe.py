#!/usr/bin/env python3
"""Independent replication of java's Random for the structure frequency reducers."""
MASK = (1 << 48) - 1
MUL = 0x5DEECE66D
ADD = 0xB

class JRandom:
    def __init__(self, seed):
        self.set_seed(seed)
    def set_seed(self, seed):
        self.seed = (seed ^ MUL) & MASK
    def next(self, bits):
        self.seed = (self.seed * MUL + ADD) & MASK
        return self.seed >> (48 - bits)
    def next_int(self):
        v = self.next(32)
        return v - (1 << 32) if v >= (1 << 31) else v
    def next_int_bound(self, bound):
        r = self.next(31)
        m = bound - 1
        if (bound & m) == 0:
            r = (bound * r) >> 31
        else:
            u = r
            r = u % bound
            while u - r + m < 0:
                u = self.next(31)
                r = u % bound
        return r
    def next_float(self):
        return self.next(24) / float(1 << 24)

def i32(v):
    v &= 0xFFFFFFFF
    return v - (1 << 32) if v >= (1 << 31) else v

def legacy_type_1(level_seed, cx, cz, frequency=0.2):
    i = i32(cx) >> 4
    i1 = i32(cz) >> 4
    base = i32(i ^ (i1 << 4))          # int xor (i1<<4 is int arithmetic)
    seed64 = (base & 0xFFFFFFFFFFFFFFFF) ^ (level_seed & 0xFFFFFFFFFFFFFFFF)
    # sign-extend semantics: (long)(int) ^ long — xor of 64-bit patterns
    r = JRandom(seed64)
    r.next_int()
    return r.next_int_bound((int)(1.0 / frequency)) == 0

def villages_candidate(seed, cx, cz, spacing=34, separation=8, salt=10387312):
    i = cx // spacing if (cx >= 0 or cx % spacing == 0) else -( ( -cx + spacing - 1)//spacing )
    # floorDiv
    i = (cx - (cx % spacing if cx >= 0 else spacing + (cx % spacing))) // spacing if cx < 0 else cx // spacing
    # simpler: python floor division == floorDiv
    i = cx // spacing
    j = cz // spacing
    r = JRandom(0)
    # setLargeFeatureWithSalt(seed, i, j, salt): setLargeFeatureSeed = setSeed(seed + i*25214903917? no:
    # setLargeFeatureSeed(baseSeed, x, z): setSeed(baseSeed + (x*25214903917? ...
    return None

def set_large_feature_with_salt(r, base, x, z, salt):
    # WorldgenRandom.setLargeFeatureWithSalt(baseSeed, regionX, regionZ, salt):
    #   setSeed(baseSeed + 341873128712L * (long)regionX + 132897987541L * (long)regionZ + (long)salt)  -- that's setRegionSeed? hmm
    pass

if __name__ == "__main__":
    # the outpost cell at seed 90210, chunk (8,-9)
    print("legacy_type_1(90210, 8, -9, 0.2):", legacy_type_1(90210, 8, -9, 0.2))
    # a sweep for sanity: how often does it pass?
    p = sum(1 for x in range(20) for z in range(20) if legacy_type_1(90210, x, z, 0.2)) / 400
    print("pass rate over 400 chunks:", p)
