#!/usr/bin/env python3
"""recon44_postj.py — RECON-44 (TASK-399): декомпозиция post-J профиля.
Вопросы:
 1. Кто драйвер broadphase-лейна на J-ветке (merge-candidates умерли — grid обслуживает)?
    split: getEntities/EntitySectionStorage vs collide/performCollisions/getCollisions vs другое
 2. Состав nav_ai (14.5%): листья PathNavigation/GoalSelector/Brain/behavior
 3. Состав fluid (15.6%) на post-J
 4. rust-сторона видна ли в collapsed (crussty frames)
 5. top-40 leaf-фреймов всего профиля
"""
import re, sys, collections

PATH = sys.argv[1] if len(sys.argv) > 1 else \
    "/home/z/c-crussty/research/gc-recon-2026-09-19/round-j2b/cpu-collapsed.txt"

tot = 0
leaf_self = collections.Counter()          # leaf frame -> weight
frame_hit = collections.Counter()          # frame-substr -> weight (ancestry-aware)
crussty = collections.Counter()
broad_split = collections.Counter()
nav_leaves = collections.Counter()
fluid_leaves = collections.Counter()

RX_BROAD_GETENT = r"EntitySectionStorage|getEntities|EntityLookup|getEntitiesOfClass"
RX_BROAD_COLLIDE = r"CollisionUtil|getCollisionsForBlocksOrWorldBorder|performCollisions|collide|noCollision|BlockCollisions"
RX_NAV = r"PathNavigation|GoalSelector|\.Brain|behavior|PathFinder|Goal|Sensor|MemoryModule"
RX_FLUID = r"updateFluidHeightAndDoFluidPushing|FluidPushGuardHook|collidedWithFluid|getFluidState|FlowingFluid|FluidState"

for ln in open(PATH, errors="ignore"):
    ln = ln.rstrip("\n")
    if not ln:
        continue
    parts = ln.rsplit(" ", 1)
    if len(parts) != 2 or not parts[1].isdigit():
        continue
    stack, w = parts[0], int(parts[1])
    tot += w
    frames = stack.split(";")
    leaf = frames[-1]
    leaf_self[leaf] += w
    if "crussty" in stack.lower():
        for f in frames:
            if "crussty" in f.lower():
                crussty[f.split("(")[0][-60:]] += w
                break
    joined = stack
    if re.search(RX_BROAD_GETENT, joined):
        broad_split["getEntities/EntitySectionStorage"] += w
    elif re.search(RX_BROAD_COLLIDE, joined):
        broad_split["collide/performCollisions/BlockCollisions"] += w
    if re.search(RX_NAV, joined):
        # leaf of the AI stack
        nav_leaves[re.sub(r"^\d+ ", "", leaf)[:90]] += w
    if re.search(RX_FLUID, joined):
        fluid_leaves[re.sub(r"^\d+ ", "", leaf)[:90]] += w

print(f"TOTAL {tot}")
print("\n== broadphase split ==")
for k, v in broad_split.most_common():
    print(f"  {k}: {v} = {v/tot:.2%}")
print("\n== nav_ai top-15 leaves ==")
for k, v in nav_leaves.most_common(15):
    print(f"  {v:6d} {v/tot:6.2%}  {k}")
print("\n== fluid top-10 leaves ==")
for k, v in fluid_leaves.most_common(10):
    print(f"  {v:6d} {v/tot:6.2%}  {k}")
print("\n== crussty (rust) frames ==")
for k, v in crussty.most_common(10):
    print(f"  {v:6d} {v/tot:6.2%}  {k}")
print("\n== top-30 leaves overall ==")
for k, v in leaf_self.most_common(30):
    print(f"  {v:6d} {v/tot:6.2%}  {re.sub(chr(39), '', k)[:110]}")
