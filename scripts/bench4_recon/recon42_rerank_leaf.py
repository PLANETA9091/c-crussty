#!/usr/bin/env python3
"""RECON-42 annex: истинный ранк ТОП-1 под ParallelGC — группировка ВСЕХ java-сэмплов
по ЛИСТУ (each sample counted once), листья бугтятся в семьи. Убирает double-count
regex-подхода RECON-41 (Fluid|fluid матвил чужие стеки)."""
import re
from collections import Counter

RUNS = {
    "s7201-pgc": "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7201-parallelgc",
    "s7203-thp": "/home/z/my-project/c-crussty/research/gc-recon-2026-09-19/run-s7203-thp",
}

FAM = [
    ("n-push scan",        r"updateFluidHeightAndDoFluidPushing|updateInWaterState"),
    ("travel/collide",     r"Entity\.travel|Entity\.move$|BlockCollisions|collide|getCollisions"),
    ("volatile/inside",    r"checkInsideBlocks|InsideBlock|InsideBlockEffectType"),
    ("broadphase",         r"getPushableEntities|findTarget|getEntities|EntityLookup|EntitySlice"),
    ("paletted/chunk-get", r"PalettedContainer|SimpleBitStorage|LevelChunk|ChunkAccess|LevelChunkSection|getFluidState|getBlockState|getChunkNow"),
    ("ai/goal/nav",        r"aiStep|GoalSelector|WrappedGoal|Navigation|PathFinder|NodeEvaluator|goal"),
    ("scheduler/bucket",   r"tickBucket|RegionTickOps|EntityTickList|guardEntityTick"),
    ("data/sync/tracker",  r"SynchedEntityData|sendChanges|ChunkMap|tracker"),
    ("collections/fastutil", r"fastutil|concurrentutil|spottedleaf"),
    ("JIT/GC-VM",          r""),
]

def fam_of(leaf):
    for k, pat in FAM:
        if pat and re.search(pat, leaf):
            return k
    return "прочее-java"

def main():
    for tag, D in RUNS.items():
        tot = 0; jscene = 0
        fam = Counter(); leaves = Counter()
        for line in open(f"{D}/cpu-collapsed.txt", errors="replace"):
            try:
                stack, cnt = line.rstrip("\n").rsplit(" ", 1)
                c = int(cnt)
            except ValueError:
                continue
            tot += c
            if not ("RegionTickOps" in stack or "MinecraftServer$$Lambda" in stack
                    or "tickServer" in stack or "tickChildren" in stack):
                continue
            jscene += c
            leaf = stack.split(";")[-1]
            leaves[leaf] += c
            fam[fam_of(leaf)] += c
        print(f"\n===== {tag}: java-сцена {jscene} =====")
        for k, v in fam.most_common():
            print(f"   {k:22s} {v:7d} ({100.0*v/max(1,jscene):5.2f}% java)")
        print("   -- top-6 листьев вне n-push/inside (кандидаты-хвосты):")
        for leaf, v in leaves.most_common(40):
            if re.search(r"updateFluidHeightAndDoFluidPushing|PalettedContainer|SimpleBitStorage|InsideBlock|checkInside", leaf):
                continue
            print(f"      {v:6d} ({100.0*v/max(1,jscene):5.2f}% java) {leaf[-110:]}")

if __name__ == "__main__":
    main()
