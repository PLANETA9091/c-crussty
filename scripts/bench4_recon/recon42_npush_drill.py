#!/usr/bin/env python3
"""RECON-42: сверление n-push лейна (23.6% java на RECON-41) под ParallelGC-экономикой.
Угол (заявлен в GOAL x71/x72): scan-оркестрация vs чтения vs flow vs same-above.
База: run-s7201-parallelgc (leg#2 банк v4, median5 2.2 @ 6653417, свежий профиль RECON-41)
+ кросс-чек run-s7203-thp (банк v4+THP, median5 2.3 @ 7014414, ≈паритет-экономика).
Атрибуция: стек содержит updateFluidHeightAndDoFluidPushing -> вес стека в лейн;
под-лейн по ЛИСТУ стека (leaf = где реально прошёл сэмпл), вызовители по фрейму-входу.
Вопрос вердикта: однородный под-лейн >=5% java-сцены (рычаг) или потолок <10% (закрытие, класс RECON-17/20/38)?
"""
import re
from collections import Counter

RUNS = {
    "s7201-pgc": "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7201-parallelgc",
    "s7203-thp": "/home/z/my-project/c-crussty/research/gc-recon-2026-09-19/run-s7203-thp",
}

LANE_MARK = "updateFluidHeightAndDoFluidPushing"

# под-лейны по листу
SUB = [
    ("READS-paletted",  r"PalettedContainer|SimpleBitStorage|LevelChunkSection|readPalette"),
    ("READS-chunk-get", r"getFluidState|getBlockState|LevelChunk\.|ChunkAccess|getChunk"),
    ("FLOW",            r"FlowingFluid|FluidState|getFlow|getHeight\(|Spread"),
    ("SAME-ABOVE",      r"hasSameAbove"),
    ("ORCH-AABB",       r"AABB|intersects|Shapes\.|VoxelShape|BlockCollisions"),
    ("ORCH-entity",     r"Entity\.setOnGround|SynchedEntityData|getBoundingBox|move\("),
    ("INSIDE-bitmask",  r"checkInsideBlocks|InsideBlock"),
]
CALLERS = [
    ("baseTick",      r"Entity\.baseTick|updateInWaterStateAndDoFluidPushing"),
    ("move/travel",   r"Entity\.move|Entity\.travel"),
    ("aiStep/goal",   r"aiStep|Goal|Navigation"),
    ("spawn/track",   r"ChunkMap|sendChanges|tracker|addEntity"),
]

def classify(stack):
    if "RegionTickOps" in stack:
        return "WORKER"
    if "MinecraftServer$$Lambda" in stack or "tickServer" in stack or "tickChildren" in stack:
        return "MAIN"
    if re.search(r"GCTask|ParallelGC|PSYoung|PSMarkSweep|Promotion|ZGeneration|ZGC", stack):
        return "GC-VM"
    if re.search(r"CompileBroker|C2 Compiler|CompileTask|C1_", stack):
        return "JIT"
    return "OTHER"

def drill(tag, D):
    tot = 0; grp = Counter(); lane = 0
    sub = Counter(); leaves = Counter(); callers = Counter()
    jscene = 0
    for line in open(f"{D}/cpu-collapsed.txt", errors="replace"):
        try:
            stack, cnt = line.rstrip("\n").rsplit(" ", 1)
            c = int(cnt)
        except ValueError:
            continue
        tot += c
        g = classify(stack)
        grp[g] += c
        if g in ("WORKER", "MAIN"):
            jscene += c
        if LANE_MARK not in stack:
            continue
        if g not in ("WORKER", "MAIN"):
            continue
        lane += c
        leaf = stack.split(";")[-1]
        leaves[leaf] += c
        for k, pat in SUB:
            if re.search(pat, leaf):
                sub[k] += c
                break
        else:
            sub["ORCH-self/other"] += c
        for k, pat in CALLERS:
            if re.search(pat, stack):
                callers[k] += c
    print(f"\n===== {tag} =====")
    print(f"TOTAL {tot} | java-сцена {jscene} ({100.0*jscene/tot:.1f}%)")
    for g, v in grp.most_common():
        print(f"  {g}: {v} ({100.0*v/tot:.1f}%)")
    print(f"LANE n-push: {lane} = {100.0*lane/max(1,jscene):.1f}% java-сцены")
    print("-- под-лейны (по листу, % java-сцены):")
    for k, v in sub.most_common():
        print(f"   {k:18s} {v:6d} ({100.0*v/max(1,jscene):5.2f}% java | {100.0*v/max(1,lane):5.1f}% лейна)")
    print("-- вызовители лейна:")
    for k, v in callers.most_common():
        print(f"   {k:14s} {v:6d} ({100.0*v/max(1,lane):5.1f}% лейна)")
    print("-- top-16 листьев лейна:")
    for leaf, v in leaves.most_common(16):
        print(f"   {v:6d} ({100.0*v/max(1,lane):5.1f}% лейна | {100.0*v/max(1,jscene):5.2f}% java) {leaf[-120:]}")
    return jscene, lane, sub

def main():
    res = {}
    for tag, D in RUNS.items():
        try:
            res[tag] = drill(tag, D)
        except FileNotFoundError as e:
            print(f"\n===== {tag} ===== ФАЙЛЫ ОТСУТСТВУЮТ: {e}")
    # кросс-чек стабильности под-лейнов
    if len(res) == 2:
        print("\n===== КРОСС-ЧЕК (под-лейн % java, s7201 vs s7203) =====")
        keys = set(res["s7201-pgc"][2]) | set(res["s7203-thp"][2])
        for k in sorted(keys):
            a = 100.0*res["s7201-pgc"][2].get(k, 0)/max(1, res["s7201-pgc"][0])
            b = 100.0*res["s7203-thp"][2].get(k, 0)/max(1, res["s7203-thp"][0])
            print(f"   {k:18s} {a:5.2f}% | {b:5.2f}%")

if __name__ == "__main__":
    main()
