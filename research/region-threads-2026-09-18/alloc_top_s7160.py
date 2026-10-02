#!/usr/bin/env python3
"""alloc_top_s7160.py — alloc-axis TOP for leg #5 (CUMULATIVE v2).
Groups alloc-collapsed.txt by TLAB type leaf (last frame before the
typed leaf `net.minecraft.X_Y` / `X_Y_[i]`), domain-level rollup."""
import sys
import re
import collections

PATH = sys.argv[1] if len(sys.argv) > 1 else (
    "/home/z/c-crussty/research/region-threads-2026-09-18/"
    "run-s7159-leg5-artifact/alloc-collapsed.txt")

rows = []
total = 0
for ln in open(PATH):
    ln = ln.rstrip("\n")
    if not ln:
        continue
    stack, _, w = ln.rpartition(" ")
    try:
        w = int(w)
    except ValueError:
        continue
    total += w
    rows.append((stack, w))

print(f"total={total} rows={len(rows)}")

# typed leaf = last segment of stack (e.g. net.minecraft...Vec3_[i], int_[i])
type_w = collections.Counter()
for s, w in rows:
    segs = s.split(";")
    type_w[segs[-1]] += w
print("\n-- top-25 allocation sites by type leaf:")
for t, w in type_w.most_common(25):
    print(f"   {w:6d}  {100.0*w/total:5.2f}%  {t[:100]}")

# domain rollup over FULL stacks (first-match)
DOMAINS = [
    ("fluid-scan family", ["updateFluidHeightAndDoFluidPushing", "FluidPushOps",
                           "FluidState", "FlowingFluid", "getFlow"]),
    ("inside-pipeline (traversal+collector)", ["checkInsideBlocks",
        "forEachBlockIntersectedBetween", "StepBasedCollector",
        "RecordedEffect", "applyEffectsFromBlocks"]),
    ("movement/collision geometry", ["Entity.move", ".collide",
        "performCollisions", "collidedWithFluid", "collidedAlongVector",
        "travel", "moveRelative", "makeBoundingBox", "AABB"]),
    ("AI/sensing/goals", ["goalSelector", "world/entity/ai/", "Brain",
        "Sensor", "getEntitiesOfClass", "findTarget"]),
    ("tracker/network sync", ["TrackerTickOps", "ServerEntity",
        "sendChanges", "SynchedEntityData"]),
    ("GC/JIT infra", ["G1", "CompileBroker"]),
    ("region-threads infra", ["RegionTickOps", "bucketOf"]),
    ("despawn/lifecycle", ["checkDespawn", "remove", "discard"]),
    ("data-sync reads", ["SynchedEntityData"]),
]
dom_w = collections.Counter()
for s, w in rows:
    for name, keys in DOMAINS:
        if any(k in s for k in keys):
            dom_w[name] += w
            break
    else:
        dom_w["other"] += w
print("\n-- domain rollup (first-match):")
for n, w in dom_w.most_common():
    print(f"   {w:6d}  {100.0*w/total:5.2f}%  {n}")

print("\n-- top-18 full alloc stacks:")
sw = collections.Counter()
for s, w in rows:
    # rollup to meaningful tail: last 6 frames
    segs = s.split(";")
    sw[";".join(segs[-6:])] += w
for s, w in sw.most_common(18):
    print(f"   {w:6d}  {100.0*w/total:5.2f}%  ...{s[-200:]}")
