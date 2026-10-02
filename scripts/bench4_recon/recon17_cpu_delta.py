#!/usr/bin/env python3
"""recon17_cpu_delta.py — куда ушла +35% CPU-работы на тик (bank vs v2,
deep-first семейства на cpu-collapsed, окно одинаково ~300s)."""
import os, re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"

DEEP_FAM = [
    ("GC/JVM-G1", re.compile(r"G1|JVM_|garbage|ZPage|PSWeak|StringDedup")),
    ("fluid-sim", re.compile(r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState;|collidedWithFluid")),
    ("travel-collide", re.compile(r"Entity;travel|Entity;move|Entity;collide|addCollisionsAlongTravel|AABB|Vec3|CollisionUtil|Cursor3D|betweenClosed")),
    ("inside-blocks", re.compile(r"checkInsideBlocks|InsideBlockOps|betweenCornersInDirection")),
    ("push-physics", re.compile(r"pushEntities|getPushableEntities")),
    ("ai/goals/nav", re.compile(r"aiStep|GoalSelector|PathNavigation|Brain;|NodeEvaluator|Goal;")),
    ("tracking", re.compile(r"ServerEntity|sendChanges|SynchedData")),
    ("chunk-access", re.compile(r"PalettedContainer|getBlockState|getEntities|EntityGetter")),
    ("mob-tick-core", re.compile(r"doTick|mobTick")),
    ("planner-frame", re.compile(r"EntityTickList|forEach|bucket")),
    ("BU-DEFER/ops", re.compile(r"BlockUpdateOps|deferBlockUpdate|handleBlockUpdate|drainDeferred|replayDeferred|RegionTickOps")),
]


def load(p):
    rows = []
    if os.path.isfile(p):
        for line in open(p, errors="ignore"):
            s, _, c = line.rstrip("\n").rpartition(" ")
            if s:
                try:
                    rows.append((s, int(c)))
                except ValueError:
                    pass
    return rows


for tag, d in (("BANK s7169", "run-s7169-parse-diag"), ("V2 s7177", "run-s7177-steal-v2")):
    rows = load(os.path.join(RESDIR, d, "cpu-collapsed.txt"))
    fam = Counter()
    tot = 0
    for s, n in rows:
        tot += n
        hit = None
        for name, rx in DEEP_FAM:
            if rx.search(s):
                hit = name
                break
        fam[hit or "other"] += n
    print(f"== {tag} CPU total {tot} ==")
    for name, n in fam.most_common():
        print(f"  {n:6d} {100.0*n/tot:6.2f}% {name}")
