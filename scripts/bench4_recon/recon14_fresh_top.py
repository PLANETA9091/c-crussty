#!/usr/bin/env python3
"""recon14_fresh_top.py — rebuild the actual TOP from the FRESHEST run
(s7169: bank v3 + parse_diag + recon_diag=1) across the three axes:
CPU% (cpu-collapsed), wall (wall-collapsed), alloc pressure (alloc-collapsed),
plus gc duty. Deepest-frame aggregation (right-most frame). Cross-run vs s7165.
Usage: python3 recon14_fresh_top.py [run_dir]
"""
import os, re, statistics, sys
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"

FAMILY = [
    ("entity-tick-core (travel/physics AABB-Vec3)", re.compile(
        r"Entity\.travel|Entity\.move|LivingEntity\.travel|aiStep|entityInside"
        r"|makeBoundingBox|setBoundingBox|collide|sweep|AABB\.|Vec3\.|CollisionGetter"
        r"|BlockCollisions|Cursor3D|betweenClosed|checkInsideBlocks")),
    ("navigation/pathfinding", re.compile(
        r"PathFinder|Path\.|Navigation|GoalSelector|NodeEvaluator|TargetFinder"
        r"|WalkNodeEvaluator|FlyNodeEvaluator|Brain\.|Behavior")),
    ("chunk-parse (SerializablerChunkData/codec)", re.compile(
        r"ChunkDataLoadTask|GenericDataLoadTask|SerializableChunkData|ChunkSerializer"
        r"|MapDecoder|DataResult|NbtOps")),
    ("remset/card-set (G1)", re.compile(
        r"G1CardSet|G1RemSet|G1ScanCardClosure|refine_card_concurrently"
        r"|G1ConcurrentRefine|G1UpdateBuffer|G1DirtyCardQueue|G1HotCardCache")),
    ("world-tick misc (block/fluid/random)", re.compile(
        r"ServerLevel\.tick|Level\.tick|RandomTick|FluidState|BlockState\.tick"
        r"|tickBlocks|chunk_system|RegionTickOps|tickBucket")),
]


def deepest(path, topn=25):
    agg = Counter()
    total = 0
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                n = int(p[2])
            except ValueError:
                continue
            total += n
            leaf = p[0].split(";")[-1]
            agg[leaf] += n
    return agg, total


def families(path):
    agg = Counter()
    total = 0
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                n = int(p[2])
            except ValueError:
                continue
            total += n
            frames = p[0]
            for name, rx in FAMILY:
                if rx.search(frames):
                    agg[name] += n
                    break
    return agg, total


def gc_stats(path):
    young = full = 0
    tot = mx = 0.0
    for line in open(path, errors="ignore"):
        m = re.search(r"Pause (Young|Full).*?([\d.]+)(ms|s)\s*$", line)
        if not m or "[gc,start" in line:
            continue
        ms = float(m.group(2)) * (1 if m.group(3) == "ms" else 1000)
        tot += ms
        mx = max(mx, ms)
        if m.group(1) == "Young":
            young += 1
        else:
            full += 1
    return young, full, tot / 1000.0, mx


def main():
    run = sys.argv[1] if len(sys.argv) > 1 else f"{RESDIR}/run-s7169-parse-diag"
    base = f"{RESDIR}/run-s7165-recon-diag"
    print(f"### FRESH TOP from {os.path.basename(run)}")
    for fname, axis in (("cpu-collapsed.txt", "CPU"), ("wall-collapsed.txt", "WALL"),
                        ("alloc-collapsed.txt", "ALLOC")):
        p = f"{run}/{fname}"
        if not os.path.isfile(p):
            continue
        agg, total = deepest(p, 15)
        print(f"\n-- {axis} (total samples {total:,}), deepest-frame top-15:")
        for leaf, n in agg.most_common(15):
            print(f"   {100*n/total:5.2f}%  {n:>7,}  {leaf[:150]}")
        fa, ft = families(p)
        print(f"   -- семейства:")
        for name, n in fa.most_common():
            print(f"      {100*n/ft:5.2f}%  {n:>7,}  {name}")
    bg = f"{base}/cpu-collapsed.txt"
    if os.path.isfile(bg):
        fa, ft = families(bg)
        print(f"\n### кросс-ран CPU-семейства vs s7165 (база):")
        for name, n in fa.most_common():
            print(f"   {100*n/ft:5.2f}%  {name}")
    gp = f"{run}/gc.log"
    if os.path.isfile(gp):
        y, fl, t, mx = gc_stats(gp)
        print(f"\n### GC {os.path.basename(run)}: young={y} full={fl} duty={t:.1f}s max={mx:.0f}ms")
    bp = f"{base}/gc.log"
    if os.path.isfile(bp):
        y, fl, t, mx = gc_stats(bp)
        print(f"### GC s7165 (база): young={y} full={fl} duty={t:.1f}s max={mx:.0f}ms")


if __name__ == "__main__":
    main()
