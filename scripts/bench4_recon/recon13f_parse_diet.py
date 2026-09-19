#!/usr/bin/env python3
"""recon13f_parse_diet.py — RECON-13f: decomposition of the TOP-1 chunk-parse
alloc lane (33.38% of ap-window, RECON-13b/13e) into sub-lanes >= 5%.

Directive "ТОП-1 ОБЯЗАН УПАСТЬ" (owner 2026-09-19 ~18:3x): a REFUTED/defect
verdict on the first lever does NOT close the lane — the lane yields until a
>=10% boost is gate-verified or the lane vanishes from fresh profiles.
This tool feeds the next-lever choice for BOTH census branches:
  - repeat_share < 10% (cache REFUTED) -> attack the largest parse sub-lane
  - repeat_share >= 30% (GO decode-cache) -> cache design input (what to cache)

Method (alloc-collapsed.txt = ap-truth for classes/callers/volumes):
  lane filter : stack contains a parse-path marker (ChunkDataLoadTask /
                GenericDataLoadTask / SerializableChunkData.parse / ChunkSerializer)
  attribution : DEEPEST frame matching a sub-lane pattern wins (right-to-left)
  sub-lanes   : codec-машинерия | nbt-io | paletted-decode | io-stream |
                chunk-struct | other-parse
  output      : ranked sub-lanes (% of TOTAL alloc bytes + % of parse lane),
                top-3 call chains per sub-lane (leaf <- ... display),
                cross-run comparison (s7168 fresh vs s7165 base)

Usage: python3 scripts/bench4_recon/recon13f_parse_diet.py [collapsed.txt ...]
"""
import os, re, sys
from collections import Counter, defaultdict

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
DEFAULTS = [
    f"{RESDIR}/run-s7168-parse-diag/alloc-collapsed.txt",   # fresh (s7168, bank v3 + parse_diag)
    f"{RESDIR}/run-s7165-recon-diag/alloc-collapsed.txt",   # diag base (s7165, recon_diag=1)
]

LANE_MARKERS = re.compile(
    r"ChunkDataLoadTask|GenericDataLoadTask|SerializableChunkData\.parse|ChunkSerializer")

SUB_LANES = [
    # (name, deepest-frame pattern)  — first match wins (list order = priority)
    ("codec-машинерия", re.compile(
        r"DataResult|MapDecoder|MapCodec|Codec\.|Codec\$|RecordCodec|CompressedDecode"
        r"|PartialResult|DeferredHolder|Encoders|DecoderContext")),
    ("nbt-io", re.compile(
        r"NbtIo|NbtAccounter|NbtInput|TagParser|CompoundTag\.|ListTag|ByteArrayTag"
        r"|IntArrayTag|StringTag|readNamedTag|readAnyTag|readTag|CompoundInput"
        r"|NbtOps|StringTagVisitor|TagWriter")),
    ("paletted-decode", re.compile(
        r"PalettedContainer|PalettedContainerRO|Palette|PackedIntegerArray"
        r"|SimpleBitStorage|ZeroBitStorage|Strategy|GlobalPalette|LinearPalette"
        r"|HashMapPalette|SingleValuePalette")),
    ("io-stream", re.compile(
        r"FastBufferedInputStream|ByteArrayInputStream|DataInputStream|Inflater"
        r"|GZIP|ZoneRegion|FileChannel|RegionFile|readChunk|StreamUpgrade")),
    ("chunk-struct", re.compile(
        r"SerializableChunkData|ChunkDataSerial|LevelChunkSection|LightEngine"
        r"|BlendingData|Heightmap|UpgradeData|ChunkAccess|ChunkPos")),
]
OTHER = "other-parse"


def classify_deepest(frames):
    """deepest (right-most) frame matching any sub-lane pattern; priority order."""
    for fr in reversed(frames):
        for name, rx in SUB_LANES:
            if rx.search(fr):
                return name
    return OTHER


def top_chains(stacks, n=3, depth=6):
    out = []
    for cnt, frames in stacks[:n]:
        chain = " <- ".join(frames[::-1][:depth])
        out.append((cnt, chain))
    return out


def analyze(path):
    total = 0
    lane = 0
    subs = Counter()
    sub_stacks = defaultdict(list)  # name -> [(count, frames)]
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                cnt = int(p[2])
            except ValueError:
                continue
            total += cnt
            frames = p[0].split(";")
            if not any(LANE_MARKERS.search(fr) for fr in frames):
                continue
            lane += cnt
            name = classify_deepest(frames)
            subs[name] += cnt
            sub_stacks[name].append((cnt, frames))
    for k in sub_stacks:
        sub_stacks[k].sort(key=lambda t: -t[0])
    return {"total": total, "lane": lane, "subs": subs, "stacks": sub_stacks}


def report(tag, a):
    print(f"\n### {tag}: parse-лейн {a['lane']:,}/{a['total']:,} = "
          f"{100*a['lane']/max(a['total'],1):.2f}% от всех аллок-байтов")
    ranked = a["subs"].most_common()
    for name, cnt in ranked:
        pct_lane = 100 * cnt / max(a["lane"], 1)
        pct_total = 100 * cnt / max(a["total"], 1)
        flag = " **>=5% ТОП**" if pct_total >= 5.0 else ""
        print(f"  {name:16s} {cnt:>10,} = {pct_lane:5.1f}% лейна / {pct_total:5.2f}% total{flag}")
        for c, chain in top_chains(a["stacks"][name]):
            print(f"      [{c:>8,}] {chain[:200]}")
    return ranked


def main():
    paths = [p for p in (sys.argv[1:] or DEFAULTS) if os.path.isfile(p)]
    if not paths:
        print("нет collapsed-файлов", file=sys.stderr)
        return 2
    results = {}
    for p in paths:
        tag = os.path.basename(os.path.dirname(p))
        results[tag] = a = analyze(p)
        report(tag, a)
    # cross-run delta on the big sub-lanes
    if len(results) == 2:
        (t1, a1), (t2, a2) = results.items()
        print(f"\n### кросс-ран {t1} vs {t2} (доля sub-лейна от total):")
        names = set(a1["subs"]) | set(a2["subs"])
        rows = []
        for n in names:
            p1 = 100 * a1["subs"].get(n, 0) / max(a1["total"], 1)
            p2 = 100 * a2["subs"].get(n, 0) / max(a2["total"], 1)
            rows.append((n, p1, p2, p1 - p2))
        for n, p1, p2, d in sorted(rows, key=lambda r: -max(r[1], r[2])):
            print(f"  {n:16s} {p1:5.2f}% vs {p2:5.2f}%  (Δ{d:+.2f}п.п.)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
