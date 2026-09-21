#!/usr/bin/env python3
"""recon-c-navtax.py — TASK-401-C: таксономия nav_ai-лейна по классам.
Ответ на вопрос 'главный массовый nav-путь': в какой именно подсистеме
(PathFinder A*/PathNavigation/GoalSelector/Brain/Sensor/target goals) сидает
14% wall и что там индексируется.
"""
import re, sys, collections

PATH = sys.argv[1]
tot = 0
nav_total = 0
by_class = collections.Counter()   # first nav-frame class -> weight (any depth)
leaf_by_class = collections.Counter()
RX_NAV = re.compile(r"(PathNavigation|GoalSelector|\.Brain|behavior|PathFinder|Path\.|PathType|Sensor|MemoryModule|targeting|goal\.|nav)")

for ln in open(PATH, errors="ignore"):
    ln = ln.rstrip("\n")
    if not ln:
        continue
    parts = ln.rsplit(" ", 1)
    if len(parts) != 2 or not parts[1].isdigit():
        continue
    stack, w = parts[0], int(parts[1])
    tot += w
    if not RX_NAV.search(stack):
        continue
    nav_total += w
    frames = stack.split(";")
    # walk from leaf up; attribute weight to the FIRST nav-subsystem class met
    cls = None
    for f in reversed(frames):
        fm = (re.search(r"(net/minecraft/world/entity/ai/\w+)", f)
              or re.search(r"(net/minecraft/world/level/pathfinder/\w+)", f)
              or re.search(r"(net/minecraft/world/entity/Mob|net/minecraft/world/entity/monster/\w+|net/minecraft/world/entity/animal/\w+)", f))
        if fm:
            cls = fm.group(1)
            break
    if cls:
        by_class[cls] += w
    leaf_by_class[frames[-1][:80]] += w

print(f"TOTAL {tot}; nav-attributed {nav_total} = {nav_total/tot:.2%}")
print("\n== nav mass by subsystem class (first nav frame from leaf up) ==")
for k, v in by_class.most_common(30):
    print(f"  {v/tot:6.2%}  {k}")
print("\n== top nav leaves ==")
for k, v in leaf_by_class.most_common(25):
    print(f"  {v/tot:6.2%}  {k}")
