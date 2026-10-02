#!/usr/bin/env python3
# RECON-10: INCLUSIVE-census поддеревьев collide-пути на банковом v3-профиле.
# Ответ на вопрос GO/NO-GO: сколько CPU реально абсорбирует Rust-лестница (Option A)
# vs сколько остаётся в world-сборе (Option B = INFEASIBLE-зона).
import re, sys
from collections import defaultdict

BASE = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact/cpu-collapsed.txt"

def inclusive(marker_rx, name):
    tot = 0
    matched = 0
    for line in open(BASE, encoding="utf-8", errors="replace"):
        m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
        if not m:
            continue
        stack, n = m.group(1), int(m.group(2))
        tot += n
        if marker_rx.search(stack):
            matched += n
    print(f"{name:52s} inclusive = {matched:6d} = {100*matched/tot:5.2f}% CPU")
    return matched, tot

t = 0
tot = 0
for pat, name in [
    (r"CollisionUtil\.performCollisions", "лестница performCollisions (вся X/Y/Z+collide*)"),
    (r"CollisionUtil\.performVoxelCollisions\b", "  ветка voxel"),
    (r"CollisionUtil\.performAABBCollisions\b", "  ветка AABB"),
    (r"CollisionUtil\.getCollisionsForBlocksOrWorldBorder", "сбор getCollisionsForBlocksOrWorldBorder"),
    (r"CollisionUtil\.getCollisions\b", "getCollisions (обёртка)"),
    (r"CollisionUtil\.getEntityHardCollisions", "сущностные hard-collisions"),
    (r"net/minecraft/world/entity/Entity\.collide\(", "Entity.collide (весь)"),
    (r"net/minecraft/world/entity/Entity\.move\(", "Entity.move (весь)"),
    (r"net/minecraft/world/entity/Entity\.makeStuckInBlock|StuckInBlock", "stuckInBlock"),
    (r"PalettedContainer\.get", "PalettedContainer.get (все потребители)"),
    (r"CollisionUtil\.isCollidingWithBorder", "isCollidingWithBorder"),
]:
    a, b = inclusive(re.compile(pat), name)
    if pat == r"CollisionUtil\.performCollisions": t = a
    tot = b
print(f"\nБАЗА: {tot} сэмплов")
