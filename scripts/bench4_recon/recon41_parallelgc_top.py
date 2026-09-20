#!/usr/bin/env python3
"""RECON-41: свежий ранк лейнов на БАЗЕ ParallelGC (банк v4) — leg#2
(run 35512885689, median5 2.2 @ 6653417, файлы распакованы absorb_s7201).
Вопрос: куда сместился ТОП-1 после освобождения ядер от G1-concurrent (37%->~6%)?
Атрибуция по вход-пути стека (профили без имён потоков, паттерн RECON-36/37):
  WORKER = стек через RegionTickOps$$Lambda (region-threaded entity ticking)
  MAIN   = стек через MinecraftServer$$Lambda / tickServer (сервер-тред)
  GC-VM  = нативные стеки GCTask/ParallelGC/libjvm-участие
  JIT    = CompileBroker/C1/C2
"""
import re
from collections import Counter

D = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7201-parallelgc"

def classify(stack):
    if "RegionTickOps$$Lambda" in stack or "RegionTickOps" in stack:
        return "WORKER"
    if "MinecraftServer$$Lambda" in stack or "tickServer" in stack or "tickChildren" in stack:
        return "MAIN"
    if re.search(r"GCTask|ParallelGC|PSYoung|PSMarkSweep|Promotion", stack):
        return "GC-PARALLEL"
    if re.search(r"CompileBroker|C2 Compiler|CompileTask|C1_", stack):
        return "JIT"
    if re.search(r"libjvm|interpreter|_thread_entry|start_thread|JavaThread", stack[:200]):
        return "VM-NATIVE"
    return "OTHER-JAVA"

LANES = {
    "fluid-push": r"Fluid|fluid",
    "broadphase/pushable/findTarget": r"getPushableEntities|findTarget|EntityLookup|getEntities",
    "travel/collide": r"Entity\.travel|AABB|Vec3|BlockCollisions|collide",
    "movement/ai": r"LivingEntity\.tick|Mob\.tick|Entity\.tick|aiStep|Goal|Navigation",
    "paletted/chunk-get": r"PalettedContainer|LevelChunkSection|ChunkAccess",
    "volatile/inside": r"checkInsideBlocks|InsideBlock",
    "collections/fastutil": r"it\.unimi\.dsi",
    "scheduler/bucket": r"tickBucket|BucketOps|EntityTickList",
    "tracker/sendChanges": r"ChunkMap|Sender|sendChanges|tracker",
    "random": r"Random",
}

def main():
    tot = 0
    grp = Counter()
    lane_hit = {g: Counter() for g in LANES}
    leaves = {g: Counter() for g in ("WORKER", "MAIN")}
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
            for k, pat in LANES.items():
                if re.search(pat, stack):
                    lane_hit[k][g] += c
            leaves[g][stack.split(";")[-1]] += c
    print(f"TOTAL samples: {tot}")
    for g, v in grp.most_common():
        print(f"  {g}: {v} ({100.0*v/tot:.1f}%)")
    print("\n== Лейны внутри WORKER+MAIN (java-сцена) ==")
    jscene = sum(grp[g] for g in ("WORKER", "MAIN"))
    for k in LANES:
        w = lane_hit[k]["WORKER"]; m = lane_hit[k]["MAIN"]
        s = w + m
        if s:
            print(f"  {k}: {s} ({100.0*s/jscene:.1f}% java) [worker {100.0*w/max(1,lane_hit[k]['WORKER']+lane_hit[k]['MAIN']):.0f}%]")
    for g in ("WORKER", "MAIN"):
        print(f"\n== top-14 leaf: {g} ==")
        for leaf, v in leaves[g].most_common(14):
            print(f"  {v:7d} ({100.0*v/max(1,grp[g]):5.2f}% grp) {leaf[:130]}")

if __name__ == "__main__":
    main()
