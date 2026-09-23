#!/usr/bin/env python3
"""TASK-120 — OPT_ARCH §6 closure: JIT-heuristic matrix scanner.

Extends TASK-85's huge_method_scan.py (which covered ONLY Code.length > 8000
= HugeMethodLimit) with:
  * threshold BANDS: HUGE >8000 (never C2-compiled, DontCompileHugeMethods),
    NEAR_HUGE (4000..8000] (C2-compiles but inline-tree pressured),
    FAT (2000..4000] (well within C2),
  * worldgen-scope pass (net.minecraft.world.level.levelgen.** / chunk.**)
    where FAT is also reported (our measured-hot area),
  * cross-reference against the MEASURED hot census (DFC_STATIC_AUDIT +
    TASK-108 v3 census classes) with decision rules:
      HUGE + hot            -> NATIVE-SWAP CANDIDATE (TASK-74 law: 11KB
                               barrier body -> native swap 5-8x)
      NEAR_HUGE + hot       -> INLINE-PRESSURE WATCH (consider inline-barrier)
      FAT + hot             -> NO-ACTION (well within C2)
      HUGE + not-hot        -> NO-ACTION (one-time init/bootstrap cost)
  * documented scope boundary: megamorphic-callsite half of §6
    (TypeProfileWidth=2) is RUNTIME-measurable only (agent instrumentation
    lane) — static scan cannot count receivers; NOT attempted here.

Same minimal classfile parser discipline as TASK-85 (no bytecode
disassembly; Code.length attribute only).
"""
import struct
import sys
import zipfile
import re
from collections import namedtuple

MIN_BAND = 2000          # report threshold (FAT band floor)
HUGE = 8000              # HotSpot HugeMethodLimit default
NEAR = 4000              # near-huge floor
Method = namedtuple("Method", "clazz name desc code_len band hot decision")

HOT_CENSUS_TOKENS = [
    # measured-hot classes: DFC_STATIC_AUDIT table + TASK-108 v3 census
    "level.levelgen.NoiseChunk",        # NoiseChunk$NoiseInterpolator.fillArray / interpFillArray
    "levelgen.synth.PerlinNoise",
    "levelgen.synth.ImprovedNoise",
    "levelgen.synth.BlendedNoise",
    "levelgen.synth.NormalNoise",
    "DensityFunctions",                 # interpreter tree (Ap2/Mapped/Clamp/...)
    "levelgen.DensityFunction",         # interfaces/impls incl. wrappers
    "ChunkNoiseFiller",                 # if present
]
WORLDGEN_SCOPE = re.compile(
    r"^net\.minecraft\.world\.level\.(levelgen|chunk)\."
)

def band_of(code_len):
    if code_len > HUGE: return "HUGE"
    if code_len > NEAR: return "NEAR_HUGE"
    return "FAT"

def decision_of(code_len, hot, name):
    if hot:
        if code_len > HUGE: return "NATIVE-SWAP-CANDIDATE"
        if code_len > NEAR: return "INLINE-PRESSURE-WATCH"
        return "NO-ACTION"
    # not hot: huge clinit/bootstrap = one-time init cost, not hot-path
    return "NO-ACTION"

def is_hot(clazz, name):
    for tok in HOT_CENSUS_TOKENS:
        if tok in clazz:
            return True
    return False

def parse_class(data, cls_name, methods_out, scope_only):
    if len(data) < 10 or data[:4] != b"\xca\xfe\xba\xbe":
        return
    if scope_only and not WORLDGEN_SCOPE.match(cls_name):
        return
    off = 8
    cp_count = struct.unpack_from(">H", data, off)[0]
    off += 2
    cp = [None] * cp_count
    i = 1
    while i < cp_count:
        tag = data[off]
        off += 1
        if tag == 1:
            ln = struct.unpack_from(">H", data, off)[0]
            off += 2
            try:
                cp[i] = data[off:off + ln].decode("utf-8", "replace")
            except Exception:
                cp[i] = "?"
            off += ln
        elif tag in (7, 8, 16, 19, 20):
            off += 2
        elif tag == 15:
            off += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            off += 4
        elif tag in (5, 6):
            off += 8
            i += 1
        else:
            return
        i += 1

    def utf8(idx):
        s = cp[idx] if idx < len(cp) else None
        return s if isinstance(s, str) else "?"

    off += 2  # access_flags
    off += 4  # this_class, super_class
    ifc = struct.unpack_from(">H", data, off)[0]
    off += 2 + 2 * ifc

    for section in ("field", "method"):
        cnt = struct.unpack_from(">H", data, off)[0]
        off += 2
        for _ in range(cnt):
            off += 6
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
                    if code_len >= MIN_BAND:
                        hot = is_hot(cls_name, utf8(name_idx))
                        methods_out.append(Method(
                            cls_name, utf8(name_idx), utf8(desc_idx),
                            code_len, band_of(code_len), hot,
                            decision_of(code_len, hot, utf8(name_idx))))
                off = body_off + alen

def scan(jar_path, scope_only):
    methods = []
    with zipfile.ZipFile(jar_path) as z:
        names = [n for n in z.namelist() if n.endswith(".class")]
        for idx, n in enumerate(names):
            try:
                data = z.read(n)
            except Exception:
                continue
            cls_name = n[:-6].replace("/", ".")
            parse_class(data, cls_name, methods, scope_only)
    methods.sort(key=lambda m: -m.code_len)
    return len(names), methods

def emit(out, jar, total_classes, methods, header):
    out.write(f"# {header}\n")
    out.write(f"# jar: {jar}\n# classes parsed: {total_classes}; methods >= {MIN_BAND}B: {len(methods)}\n\n")
    out.write("band\tcode_len\tclass\tmethod\tdescriptor\thot\tdecision\n")
    for m in methods:
        out.write(f"{m.band}\t{m.code_len}\t{m.clazz}\t{m.name}\t{m.desc}\t{'HOT' if m.hot else '-'}\t{m.decision}\n")

def main(jar_path, out_path_full, out_path_worldgen):
    total, full = scan(jar_path, scope_only=False)
    full_band = [m for m in full if m.band in ("HUGE", "NEAR_HUGE")]
    _, wg = scan(jar_path, scope_only=True)
    with open(out_path_full, "w") as f:
        emit(f, jar_path, total, full_band, "FULL-JAR scan: HUGE + NEAR_HUGE bands (TASK-85 covered only HUGE)")
    with open(out_path_worldgen, "w") as f:
        emit(f, jar_path, total, wg, "WORLDGEN-SCOPE scan (levelgen|chunk): all bands FAT+")
    print(f"full-jar: HUGE={sum(1 for m in full_band if m.band=='HUGE')} NEAR_HUGE={sum(1 for m in full_band if m.band=='NEAR_HUGE')}")
    print(f"worldgen-scope: total={len(wg)} HUGE={sum(1 for m in wg if m.band=='HUGE')} NEAR_HUGE={sum(1 for m in wg if m.band=='NEAR_HUGE')} FAT={sum(1 for m in wg if m.band=='FAT')}")
    hot = [m for m in wg if m.hot]
    print(f"worldgen HOT-listed methods: {len(hot)}")
    for m in hot:
        print(f"  HOT {m.band} {m.code_len}B {m.clazz}.{m.name} -> {m.decision}")
    cand = [m for m in wg if m.decision != "NO-ACTION"]
    print(f"decision != NO-ACTION in worldgen scope: {len(cand)}")
    near_full = [m for m in full_band if m.band == "NEAR_HUGE"]
    print(f"full-jar NEAR_HUGE top-10:")
    for m in near_full[:10]:
        print(f"  {m.code_len}B {m.clazz}.{m.name}{' HOT' if m.hot else ''}")

if __name__ == "__main__":
    jar = sys.argv[1] if len(sys.argv) > 1 else "/home/z/server/versions/purpur-1.21.10.jar"
    out_full = sys.argv[2] if len(sys.argv) > 2 else "bench/jitflags/JIT_HEURISTIC_SCAN_FULL_2026-09-09.txt"
    out_wg = sys.argv[3] if len(sys.argv) > 3 else "bench/jitflags/JIT_HEURISTIC_SCAN_WORLDGEN_2026-09-09.txt"
    main(jar, out_full, out_wg)
