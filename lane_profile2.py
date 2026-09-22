#!/usr/bin/env python3
"""STEP0 v2: exact-frame lane attribution for checkInsideBlocks family."""
import os, re
from collections import Counter

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"
A = "net/minecraft/world/entity/Entity.checkInsideBlocks"
VIS = "net/minecraft/world/entity/Entity.lambda$checkInsideBlocks$2"
B = "net/minecraft/world/entity/Entity.collidedWithShapeMovingFrom"
GATE = "net/minecraft/world/entity/InsideBlockOps.gate"
BC = "net/minecraft/world/entity/BatchCollector.advanceStep"
AFB = "net/minecraft/world/entity/Entity.applyEffectsFromBlocks"

def parse(path):
    rows, total = [], 0
    with open(path, errors="replace") as f:
        for line in f:
            m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
            if not m: continue
            rows.append((m.group(1).split(";"), int(m.group(2)))); total += int(m.group(2))
    return rows, total

def analyze(name):
    rows, total = parse(os.path.join(BASE, name, "cpu-collapsed.txt"))
    print(f"### {name}  total={total}")
    lane = 0; ent_caller = Counter(); under_vis = Counter(); vis_stack = Counter()
    gate_sites = Counter(); under_gate = Counter(); b_under = Counter()
    for fr, cnt in rows:
        if A not in fr and VIS not in fr: continue
        # find deepest exact lane frame
        idxs = [i for i, f in enumerate(fr) if f in (A, VIS)]
        lane += cnt
        di = max(idxs)
        top = fr[-1] if di == len(fr) - 1 else fr[di + 1]
        # entity caller: walk up to first *.tick/*.aiStep/*.move frame
        ent = "<none>"
        for f in reversed(fr[:di]):
            if f.startswith("net/minecraft/") and (".tick" in f or ".aiStep" in f or ".move" in f or ".inactiveTick" in f):
                ent = f; break
        ent_caller[ent] += cnt
        if di < len(fr) - 1 and fr[di] == VIS:
            under_vis[top] += cnt
        # where does the visitor chain pass gate?
        if GATE in fr:
            gi = fr.index(GATE)
            under_gate[fr[gi + 1] if gi + 1 < len(fr) else "<leaf=gate>"] += cnt
            gate_sites[fr[gi - 1] if gi else "<ROOT>"] += cnt
        if B in fr:
            bi = fr.index(B)
            b_under[fr[bi - 1] if bi else "<ROOT>"] += cnt
    print(f"LANE (checkInsideBlocks exact + visitor) = {lane} = {100*lane/total:.2f}%")
    print("-- top entity-level callers of the lane:")
    for k, v in ent_caller.most_common(10):
        print(f"   {v:7d} {100*v/lane:6.2f}% of lane  {k}")
    print("-- visitor lambda callees (hot body):")
    for k, v in under_vis.most_common(14):
        print(f"   {v:7d} {100*v/lane:6.2f}% of lane  {k}")
    print("-- InsideBlockOps.gate: callers / callees:")
    for k, v in gate_sites.most_common(4): print(f"   caller {v:7d} {k}")
    for k, v in under_gate.most_common(6): print(f"   callee {v:7d} {k}")
    print("-- collidedWithShapeMovingFrom parents:", dict(b_under.most_common(4)))
    print()
    return total, lane

if __name__ == "__main__":
    for n in ("round-round410anchora", "round-round409multi2"):
        analyze(n)
