#!/usr/bin/env python3
"""recon17_worker_axis2.py — уточнение (C): только BU-DEFER-специфичные кадры
(BlockUpdateOps/deferBlockUpdate/handle/drain/replay), не корень run-loop;
+ парки воркеров (CyclicBarrier) bank vs v2.
"""
import os, re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
SPIN = "MinecraftServer.lambda$spin$2"
WROOT = re.compile(r"RegionTickOps\$\$Lambda")
BU = re.compile(r"BlockUpdateOps|deferBlockUpdate|handleBlockUpdate|drainDeferred|replayDeferred|BU_DEFER|buDefer")
BARRIER = re.compile(r"CyclicBarrier|LockSupport.park|RegionTickOps\.await|awaitPhase")


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


bank = os.path.join(RESDIR, "run-s7169-parse-diag")
v2 = os.path.join(RESDIR, "run-s7177-steal-v2")

for tag, d in (("BANK s7169", bank), ("V2 s7177", v2)):
    rows = load(os.path.join(d, "wall-collapsed.txt"))
    bu = Counter()
    park = Counter()
    for s, n in rows:
        where = "main" if SPIN in s else ("worker" if WROOT.search(s) else "other")
        if BU.search(s):
            bu[where] += n
        if BARRIER.search(s):
            park[where] += n
    print(f"{tag}: BU-DEFER-кадры {dict(bu)} | барьер/парк {dict(park)}")
