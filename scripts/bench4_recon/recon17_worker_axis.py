#!/usr/bin/env python3
"""recon17_worker_axis.py — RECON-17 (TASK-337): decompose the WORKER-thread
wall axis (phase-3 critical path from RECON-15: workers 651 vs main 478) on
the clean bank s7169 profile, and attribute the BU-DEFER replay cost on the
s7177 (steal v2) main thread. Answers: why park-kill did not convert to TPS.

Usage: python3 recon17_worker_axis.py
"""
import os, re, sys
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"

DEEP_FAM = [
    ("fluid-sim", re.compile(
        r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState;|collidedWithFluid")),
    ("travel-collide", re.compile(
        r"Entity;travel|Entity;move|Entity;collide|addCollisionsAlongTravel|AABB|Vec3|CollisionUtil|Cursor3D|betweenClosed")),
    ("inside-blocks", re.compile(r"checkInsideBlocks|InsideBlockOps|betweenCornersInDirection")),
    ("push-physics", re.compile(r"pushEntities|getPushableEntities")),
    ("ai/goals/nav", re.compile(r"aiStep|GoalSelector|PathNavigation|Brain;|NodeEvaluator|Goal;")),
    ("tracking", re.compile(r"ServerEntity|sendChanges|SynchedData")),
    ("chunk-access", re.compile(r"PalettedContainer|getBlockState|getEntities|EntityGetter")),
    ("mob-tick-core", re.compile(r"doTick|mobTick")),
    ("planner-frame", re.compile(r"EntityTickList|forEach|bucket")),
    ("GC/JVM", re.compile(r"G1|JVM_|garbage|ZPage|PSWeak")),
]


def load(path):
    rows = []
    if not os.path.isfile(path):
        return rows
    for line in open(path, errors="ignore"):
        stack, _, cnt = line.rstrip("\n").rpartition(" ")
        if stack:
            try:
                rows.append((stack, int(cnt)))
            except ValueError:
                pass
    return rows


def classify(rows, restrict_thread=None):
    fam = Counter()
    tot = 0
    for stack, n in rows:
        if restrict_thread and not re.search(restrict_thread, stack.split(";")[0]):
            continue
        tot += n
        hit = None
        for name, rx in DEEP_FAM:  # leaf-first
            if rx.search(stack):
                hit = name
                break
        fam[hit or "other"] += n
    return fam, tot


def main():
    bank = os.path.join(RESDIR, "run-s7169-parse-diag")
    v2 = os.path.join(RESDIR, "run-s7177-steal-v2")

    # stack classes: main = contains spin root; worker = RegionTickOps-lambda
    # root without spin (the steal pool run loop).
    SPIN = "MinecraftServer.lambda$spin$2"
    WROOT = re.compile(r"RegionTickOps\$\$Lambda")

    for tag, d in (("BANK s7169", bank), ("V2 s7177", v2)):
        rows = load(os.path.join(d, "wall-collapsed.txt"))
        main_rows = [(s, n) for s, n in rows if SPIN in s]
        wk_rows = [(s, n) for s, n in rows if WROOT.search(s) and SPIN not in s]
        fm, tm = classify(main_rows)
        fw, tw = classify(wk_rows)
        print(f"== ({tag}) MAIN spin-wall {tm} samples ==")
        for name, n in fm.most_common(10):
            print(f"  {n:5d} {100.0*n/max(tm,1):6.2f}% {name}")
        print(f"-- ({tag}) WORKER wall {tw} samples (phase-3 critical path) --")
        for name, n in fw.most_common(10):
            print(f"  {n:5d} {100.0*n/max(tw,1):6.2f}% {name}")

    print("== (C) BU-DEFER/ops frame attribution (v2) ==")
    pat = re.compile(
        r"BlockUpdateOps|deferBlockUpdate|handleBlockUpdate|drainDeferred|"
        r"replayDeferred|RegionTickOps\.(steal|runTickOps|snapshot)")
    for tag, d in (("bank", bank), ("v2", v2)):
        cnt = Counter()
        for stack, n in load(os.path.join(d, "wall-collapsed.txt")):
            if pat.search(stack):
                where = "main" if SPIN in stack else (
                    "worker" if WROOT.search(stack) else "other")
                cnt[where] += n
        print(f"  {tag}: {dict(cnt)}")

    print("== (E) s7177 vs bank: CPU-collapsed total (work-growth check) ==")
    for tag, d in (("bank", bank), ("v2", v2)):
        rows = load(os.path.join(d, "cpu-collapsed.txt"))
        tot = sum(n for _, n in rows)
        print(f"  {tag}: cpu samples total = {tot}")


if __name__ == "__main__":
    main()
