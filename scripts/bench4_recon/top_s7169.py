#!/usr/bin/env python3
"""Актуальный ТОП-лейны s7169 v2: корректные паттерны (точки), leaf-топы, GC-фракция."""
import re
from collections import Counter

D = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7168-parse-diag"

LANES = {
    "entity-bucket(RegionTickOps)": r"RegionTickOps",
    "entity-travel(AABB|Vec3|travel)": r"Entity\.travel|AABB|Vec3",
    "entity-tick-other": r"Entity\.tick|LivingEntity\.tick|Mob\.tick",
    "navigation/path": r"Navigation|PathFinder|Path\.|Goal",
    "lighting": r"LightEngine",
    "chunk-parse": r"SerializableChunkData|MapDecoder",
    "collision": r"BlockCollisions|CollisionSpliterator",
    "hopper/redstone": r"Hopper|Redstone",
    "sentinel/paper-tickloop": r"MinecraftServer\.tickServer|tickChildren|LevelTicks",
    "cru-sty-native": r"cru|sty|Crussty",
}

def analyze(name, skip_native=False):
    tot = 0
    hit = Counter()
    leaves = Counter()
    with open(f"{D}/{name}", "r", errors="replace") as f:
        for line in f:
            try:
                stack, cnt = line.rstrip("\n").rsplit(" ", 1)
                c = int(cnt)
            except ValueError:
                continue
            if skip_native and (stack.startswith("/usr/lib") or stack.startswith("/jdk") or "libjvm" in stack[:120]):
                continue
            tot += c
            for k, pat in LANES.items():
                if re.search(pat, stack):
                    hit[k] += c
            leaf = stack.split(";")[-1]
            leaves[leaf] += c
    print(f"== {name} == total={tot}" + (" (java-only)" if skip_native else " (все треды)"))
    for k, v in hit.most_common():
        print(f"  {k}: {v} ({100.0*v/tot:.2f}%)")
    print("  --- top-12 leaf frames ---")
    for leaf, v in leaves.most_common(12):
        print(f"  {v:7d} ({100.0*v/tot:5.2f}%) {leaf[:150]}")
    print()
    return tot, hit, leaves

analyze("alloc-collapsed.txt")
analyze("cpu-collapsed.txt", skip_native=False)
analyze("cpu-collapsed.txt", skip_native=True)
