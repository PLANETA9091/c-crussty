#!/usr/bin/env python3
"""RECON-21: свежая аллок-атрибуция travel-листьев (Vec3.add / makeBoundingBox /
AABB.inflate / Vec3.<init> / AABB.<init>) по alloc-окну s7177 (банк v3, живое
окно AP) — вход для javap-контракта #14 v2 TravelDietOps."""
import os, re
from collections import Counter

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7177-steal-v2"
SRC = os.path.join(BASE, "alloc-collapsed.txt")

LEAF_TYPE_PAT = re.compile(r"\.(\w+)_\[[ik]\]$")
# метод-аллокатор: фрейм НАД маркером типа (Vec3_[i] = тип аллокации)
LEAF_SITE_PAT = re.compile(
    r"(Vec3\.add|Vec3\.multiply|Vec3\.subtract|Vec3\.scale|Vec3\.normalize|Vec3\.relative|"
    r"makeBoundingBox|AABB\.inflate|AABB\.expandToFront|AABB\.expandTowards|"
    r"Vec3\.<init>|AABB\.<init>|Vec3i\.<init>|Vec3\.length|Vec3\.normalize)")

# travel-чейн якоря: ближайший net/minecraft/cru.sty фрейм НАД leaf
TRAVEL_ANCHORS = re.compile(
    r"(LivingEntity\.travel|Entity\.move|Entity\.collide|collideBoundingBox|"
    r"addCollisionsAlongTravel|EntityDimensions\.makeBoundingBox|"
    r"Entity\.travel|moveRelative|getAllowedMovement|BlockGetter\.clip)")

tot = 0
tot_leaf = 0
sites = Counter()
anchors = Counter()
travel_leaf = 0
with open(SRC, errors="replace") as f:
    for line in f:
        try:
            stack, cnt = line.rstrip("\n").rsplit(" ", 1)
            c = int(cnt)
        except ValueError:
            continue
        tot += c
        frames = stack.split(";")
        leaf = frames[-1]
        # метод-аллокатор: при маркере типа X_[i] — фрейм выше; иначе сам leaf
        site_frames = frames[:-1] if LEAF_TYPE_PAT.search(leaf) else frames
        site = site_frames[-1].split("(")[0] if site_frames else leaf
        m = LEAF_SITE_PAT.search(site)
        if not m:
            continue
        tot_leaf += c
        sites[site.rsplit("/", 1)[-1]] += c
        anchor = "?"
        is_travel = False
        for fr in reversed(site_frames[:-1]):
            if "net/minecraft" in fr or "cru/sty" in fr or "ca/spottedleaf" in fr:
                anchor = fr.split("(")[0]
                break
        for fr in site_frames:
            if TRAVEL_ANCHORS.search(fr):
                is_travel = True
                break
        if is_travel:
            travel_leaf += c
            anchors[anchor.rsplit("/", 1)[-1]] += c

print(f"== alloc-window total samples: {tot} | leaf-match: {tot_leaf} ({100*tot_leaf/tot:.2f}%)")
print("-- TOP leaf sites (samples, % of window) --")
for k, v in sites.most_common(14):
    print(f"{v:8d}  {100*v/tot:6.2f}%  {k}")
print(f"-- travel-чейн доля листьев: {travel_leaf} ({100*travel_leaf/tot:.2f}%) --")
print("-- anchors внутри travel-чейна --")
for k, v in anchors.most_common(12):
    print(f"{v:8d}  {100*v/tot:6.2f}%  {k}")
