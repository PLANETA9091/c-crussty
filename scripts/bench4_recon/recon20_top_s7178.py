#!/usr/bin/env python3
"""recon20_top_s7178.py — свежий ТОП CPU-оси на якорном леге s7178
(банк v3, раннер 8.49M, cpu-collapsed 38.5MB жив при AP-PID-дефекте).
Это актуальный ТОП-1 для решения: paired GREEN → v4; иначе #14 из ТОПа.
"""
import os, re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN = os.path.join(RESDIR, "run-s7178-recal")

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
    ("RegionTickOps/BU", re.compile(r"RegionTickOps|BlockUpdateOps")),
]

rows = []
for line in open(os.path.join(RUN, "cpu-collapsed.txt"), errors="ignore"):
    s, _, c = line.rstrip("\n").rpartition(" ")
    if s:
        try:
            rows.append((s, int(c)))
        except ValueError:
            pass

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

print(f"== s7178 ЯКОРЬ CPU total {tot} ==")
for name, n in fam.most_common():
    print(f"  {n:6d} {100.0*n/tot:6.2f}% {name}")

# сплит GC-семьи: G1-потоки vs аллок-чумные стопы приложения (young паузы уже в gc.log)
g1_threads = Counter()
for s, n in rows:
    if re.search(r"G1|StringDedup|GC Thread|VM Thread", s):
        g1_threads[s.split(";")[0][:60]] += n
print("-- G1/JVM-потоки top3 --")
for t, n in g1_threads.most_common(3):
    print(f"  {n:6d}  {t}")

young = full = 0
gp = os.path.join(RUN, "gc.log")
if os.path.isfile(gp):
    for line in open(gp, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause (Young|Full).*", line)
        if m:
            if m.group(1) == "Young":
                young += 1
            else:
                full += 1
print(f"-- gc.log: young={young} Full={full} (давление аллок-оси) --")
