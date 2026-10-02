#!/usr/bin/env python3
"""RECON-6 addendum: pin callers of the dominant allocation leaves
(AABB, Vec3, BlockPos family) — attack surface for lever #10 ZERO-ALLOC."""
import re
import collections

ART = ("/home/z/c-crussty/research/batch-collector-2026-09-19/"
       "run-s7162-leg2-artifact")


def load(path):
    rows = []
    with open(path, errors="replace") as f:
        for ln in f:
            m = re.match(r"^(.*) (\d+)$", ln.rstrip("\n"))
            if m:
                rows.append((m.group(1).split(";"), int(m.group(2))))
    return rows


rows = load(ART + "/alloc-collapsed.txt")
tot = sum(w for _, w in rows)

LEAVES = ["net.minecraft.world.phys.AABB_[i]",
          "net.minecraft.world.phys.Vec3_[i]",
          "net.minecraft.core.BlockPos_[i]",
          "net.minecraft.core.BlockPos$MutableBlockPos_[i]",
          "net.minecraft.core.BlockPos$6_[i]",
          "net.minecraft.core.BlockPos$4_[i]"]
for leaf in LEAVES:
    sub = [(fs, w) for fs, w in rows if fs[-1] == leaf]
    wsum = sum(w for _, w in sub)
    print(f"\n=== {leaf}: {wsum} = {100.0*wsum/tot:.2f}% alloc ===")
    agg = collections.Counter()
    for fs, w in sub:
        # skip self; take the deepest 6 minecraft/ca frames above leaf
        mcs = [f for f in fs[:-1] if ("net/minecraft" in f or "ca/spottedleaf"
                                      in f or "RegionTickOps" in f)]
        key = ";".join(mcs[-6:]) if mcs else "(native/jdk)"
        agg[key] += w
    for k, w in agg.most_common(10):
        print(f"  {w:5d} ({100.0*w/wsum:5.1f}%): {k}")

# Phase-root aggregation: which top-level lane drives AABB+Vec3 combined
print("\n=== AABB+Vec3+BlockPos family by lane root ===")
family = ("net.minecraft.world.phys.AABB_[i]",
          "net.minecraft.world.phys.Vec3_[i]",
          "net.minecraft.core.BlockPos_[i]",
          "net.minecraft.core.BlockPos$MutableBlockPos_[i]",
          "net.minecraft.core.BlockPos$6_[i]",
          "net.minecraft.core.BlockPos$4_[i]",
          "net.minecraft.core.Vec3i_[i]")
LANES = [
    ("inside-pipeline(checkInsideBlocks)", lambda s: "checkInsideBlocks" in s),
    ("traversal(forEachBlockIntersected)", lambda s:
        "forEachBlockIntersectedBetween" in s),
    ("broadphase/push", lambda s: any(
        k in s for k in ("getCollisions", "CollisionGetter", "EntityGetter",
                         "push", "collide"))),
    ("movement/travel", lambda s: any(
        k in s for k in ("travel", "moveRelative", "AiStep", "moveControl"))),
    ("AI/brain/sensing/goal", lambda s: any(
        k in s for k in ("Brain", "Behavior", "GoalSelector", "sensing",
                         "Sensing"))),
    ("fluid", lambda s: "collidedWithFluid" in s or "FluidState" in s),
    ("tickBucket/other-orch", lambda s: "RegionTickOps" in s),
    ("other", lambda s: True),
]
lane_w = collections.Counter()
for fs, w in rows:
    if fs[-1] in family:
        s = ";".join(fs)
        for name, fn in LANES:
            if fn(s):
                lane_w[name] += w
                break
fw = sum(lane_w.values())
for name, w in lane_w.most_common():
    print(f"  {w:5d} ({100.0*w/fw:5.1f}% of family): {name}")
print(f"  family total = {fw} = {100.0*fw/tot:.1f}% of all alloc")
