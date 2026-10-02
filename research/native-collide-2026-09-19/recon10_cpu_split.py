#!/usr/bin/env python3
# RECON-10: раскладка CPU внутри collide-семьи по листьям банкового v3-профиля (cpu-collapsed).
# Цель: разделить «лестница» (collideX/Y/Z, performVoxel/AABB) vs «сбор» (getCollisionsForBlocksOrWorldBorder)
# vs орchestration (Entity.move/collide) — решает GO/NO-GO рычага #12 NATIVE-COLLIDE.
import re
from collections import defaultdict

BASE = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact/cpu-collapsed.txt"
total = 0
self_by_leaf = defaultdict(int)
for line in open(BASE, encoding="utf-8", errors="replace"):
    line = line.rstrip("\n")
    if not line:
        continue
    m = re.match(r"^(.*) (\d+)$", line)
    if not m:
        continue
    stack, n = m.group(1), int(m.group(2))
    total += n
    frames = stack.split(";")
    if frames:
        self_by_leaf[frames[-1]] += n

def census(label, pat):
    rx = re.compile(pat)
    agg = defaultdict(int)
    for leaf, n in self_by_leaf.items():
        if rx.search(leaf):
            # берём минимальный осмысленный фрейм: последнее вхождение паттерна
            agg[leaf] += n
    top = sorted(agg.items(), key=lambda x: -x[1])[:12]
    s = sum(n for _, n in top)
    print(f"== {label}: top-leafs ({s}/{total} = {100*s/total:.2f}% CPU, top12)")
    for leaf, n in top:
        print(f"   {100*n/total:6.3f}%  {leaf[:150]}")
    return sum(n for _, n in top)

census("CollisionUtil ladder (collideX/Y/Z/perform*)", r"CollisionUtil\.(collide[XYZ]|performVoxelCollisions[XYZ]|performAABBCollisions[XYZ]|performCollisions|findFloor)")
census("CollisionUtil collection (getCollisions*)", r"CollisionUtil\.getCollisions")
census("Entity.move", r"Entity\.move\(")
census("Entity.collide", r"Entity\.collide\(")
census("collideWithShapes/collideBoundingBox (Level)", r"collideWithShapes|collideBoundingBox")
census("PalettedContainer.get (block reads)", r"PalettedContainer\.get")
print(f"\ntotal samples = {total}")
