#!/usr/bin/env python3
"""recon14c_alloc_callers.py — attribute the TOP alloc leaves (AABB/Vec3/
BlockPos/MutableBlockPos/long[]) and CPU sub-lanes to their caller families
across the WHOLE s7173 window (fresh top, bank v3+zero_cursor=1).

Buckets (first match wins, top-down priority):
  inside-blocks : Entity.checkInsideBlocks (list/5-arg) + StepBasedCollector
  travel-collide: Entity.travel/move/collide + BlockCollisions/collideWithShapes
  push          : pushEntities/getPushableEntities/cramming
  fluid         : updateFluidHeightAndDoFluidPushing/collidedWithFluid
  other
"""
import os, re
from collections import Counter, defaultdict

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7171-zero-cursor"

BUCKETS = [
    ("inside-blocks", re.compile(
        r"checkInsideBlocks|InsideBlockEffectApplier|StepBasedCollector|InsideBlockOps")),
    ("travel-collide", re.compile(
        r"Entity\.travel|LivingEntity\.travel|Entity\.move\(|Entity;move|collide"
        r"|BlockCollisions|collideWithShapes|getCollisions|addCollisionsAlongTravel")),
    ("push", re.compile(r"pushEntities|getPushableEntities|cramming|Player;aiStep|pushableBy")),
    ("fluid", re.compile(r"updateFluidHeightAndDoFluidPushing|collidedWithFluid|FluidState|fluidPush")),
]

LEAVES = [
    ("AABB", re.compile(r"phys\.AABB_\[i\]")),
    ("Vec3", re.compile(r"phys\.Vec3_\[i\]")),
    ("BlockPos", re.compile(r"core\.BlockPos(?:_\[i\]|\$MutableBlockPos_\[i\])")),
    ("long[]", re.compile(r"^long\[]_\[i\]")),
    ("char[]", re.compile(r"char\[]_\[k\]")),
]


def bucket(frames):
    for name, rx in BUCKETS:
        if rx.search(frames):
            return name
    return "other"


def main():
    out = ["# RECON-14c — атрибуция аллок-листьев и CPU по вызывателям (s7173, всё окно)\n"]
    path = os.path.join(BASE, "alloc-collapsed.txt")
    leaf_tot = defaultdict(Counter)
    buck_tot = Counter()
    grand = 0
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                n = int(p[2])
            except ValueError:
                continue
            grand += n
            b = bucket(p[0])
            buck_tot[b] += n
            for lname, lrx in LEAVES:
                if lrx.search(p[0].split(";")[-1]):
                    leaf_tot[lname][b] += n
    out.append(f"## ALLOC окно: {grand:,} сэмплов; по вызывателям:")
    for b, n in buck_tot.most_common():
        out.append(f"- {b}: {n:,} = {n/grand*100:.2f}%")
    out.append("\n## Топ-листья по вызывателям:")
    for lname, _ in LEAVES:
        c = leaf_tot[lname]
        s = sum(c.values())
        if not s:
            continue
        parts = "; ".join(f"{k}={v} ({v/s*100:.0f}%)" for k, v in c.most_common())
        out.append(f"- {lname}: {s:,} = {s/grand*100:.2f}% окна | {parts}")

    # CPU: same buckets
    path = os.path.join(BASE, "cpu-collapsed.txt")
    buck_tot = Counter()
    grand = 0
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                n = int(p[2])
            except ValueError:
                continue
            grand += n
            buck_tot[bucket(p[0])] += n
    out.append(f"\n## CPU окно: {grand:,} сэмплов; по вызывателям:")
    for b, n in buck_tot.most_common():
        out.append(f"- {b}: {n:,} = {n/grand*100:.2f}%")

    text = "\n".join(out) + "\n"
    dest = "/home/z/c-crussty/research/gc-recon-2026-09-19/RECON14C_ALLOC_CALLERS.md"
    with open(dest, "w") as f:
        f.write(text)
    print(text)
    print(f"written: {dest}")


if __name__ == "__main__":
    main()
