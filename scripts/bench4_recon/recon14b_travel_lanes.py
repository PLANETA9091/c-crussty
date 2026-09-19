#!/usr/bin/env python3
"""recon14b_travel_lanes.py — decompose the cursor/travel alloc+CPU lane
(s7173 fresh profile) into caller sub-lanes >=5% to pick the next TOP-1
sub-lane lever (TASK-332, v7 "ТОП-1 ОБЯЗАН УПАСТЬ").

Inputs: run-s7171-zero-cursor/{alloc,cpu}-collapsed.txt (fresh same-bank
profile, full delivery of lever #11 v1 infrastructure).

Needle family = travel/cursor walk (BlockPos$6 is dead; the walk now runs
through ZeroCursor pooled iterators but the LANE is the vanilla callers):
  betweenCornersInDirection | forEachBlockIntersectedBetween |
  addCollisionsAlongTravel | ZeroCursor
Sub-lane buckets keyed by caller frames found in the stack:
  travel-physics   : Entity.collide/addCollisionsAlongTravel/move
  push-physics     : pushEntities/EntityPush/natural pushing
  inside-blocks    : checkInsideBlocks/lambda$checkInsideBlocks
  fluid            : collideWithShapes fluid paths/updateFluidHeightAndDoFluidPushing
  navigation       : navigation/pathfinding walk families
  other            : everything else in the family
Leaves: top leaf classes per bucket (allocation churn atoms).
"""
import os, re, sys
from collections import Counter, defaultdict

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7171-zero-cursor"

FAMILY_RX = re.compile(
    r"betweenCornersInDirection|forEachBlockIntersectedBetween|addCollisionsAlongTravel|ZeroCursor")

BUCKETS = [
    ("travel-physics", re.compile(r"addCollisionsAlongTravel|Entity;collide|collide\(Lnet/minecraft/world/phys/Vec3|Entity;move\(|travel")),
    ("push-physics", re.compile(r"pushEntities|getPushableEntities|collideWithPushable|EntityPush")),
    ("inside-blocks", re.compile(r"checkInsideBlocks|lambda\$checkInsideBlocks")),
    ("fluid", re.compile(r"updateFluidHeightAndDoFluidPushing|collidedWithFluid|FluidState")),
    ("navigation", re.compile(r"navigation|PathFinder|PathNavigation")),
    ("ai-goal", re.compile(r"aiStep|goalSelector|MobTick")),
]


def classify(frames):
    for name, rx in BUCKETS:
        if rx.search(frames):
            return name
    return "other"


def main():
    out = ["# RECON-14b — декомпозиция cursor/travel-лейна (s7173, run 35444005075 @ 27ef945)\n"]
    for fname, axis in (("alloc-collapsed.txt", "ALLOC"), ("cpu-collapsed.txt", "CPU")):
        path = os.path.join(BASE, fname)
        total = 0
        fam = 0
        buckets = Counter()
        leaves = defaultdict(Counter)
        callers = defaultdict(Counter)
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
                if not FAMILY_RX.search(p[0]):
                    continue
                fam += n
                b = classify(p[0])
                buckets[b] += n
                leaf = p[0].split(";")[-1].split("$Lambda")[0]
                leaf = re.sub(r"0x[0-9a-f]+", "", leaf)
                leaves[b][leaf] += n
                # deepest caller below the needle family frame
                frames = p[0].split(";")
                idx = None
                for i, fr in enumerate(frames):
                    if FAMILY_RX.search(fr):
                        idx = i
                        break
                if idx is not None and idx > 0:
                    callers[b][frames[idx - 1].split("/")[-1]] += n
        pct = fam / total * 100 if total else 0
        out.append(f"\n## {axis}: family {fam:,}/{total:,} = {pct:.2f}%")
        for b, n in buckets.most_common():
            bp = n / fam * 100 if fam else 0
            top_leaves = "; ".join(f"{k.rsplit('/',1)[-1]}={v}" for k, v in leaves[b].most_common(4))
            top_callers = "; ".join(f"{k}={v}" for k, v in callers[b].most_common(3))
            out.append(f"- {b}: {n:,} = {bp:.1f}% of family | leaves: {top_leaves}")
            out.append(f"  - callers: {top_callers or '—'}")
    text = "\n".join(out) + "\n"
    dest = "/home/z/c-crussty/research/gc-recon-2026-09-19/RECON14B_TRAVEL_LANES.md"
    with open(dest, "w") as f:
        f.write(text)
    print(text)
    print(f"written: {dest}")


if __name__ == "__main__":
    main()
