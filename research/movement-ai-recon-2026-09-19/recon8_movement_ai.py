#!/usr/bin/env python3
"""recon8_movement_ai.py — RECON-8: decompose the movement/AI surface
(TOP-1 sub-lane of the entity phase, ~20% CPU) into BEHAVIOR CLASSES
per owner directive «ТОП-ПОЖИРАТЕЛЬ -> ∞»: goal-selector / targeting /
navigation / controls / sensing / travel-physics / broadphase / ...

Scope  : entity phase = stacks containing RegionTickOps.tickBucket or
         tickNonPassenger (classifier identical to RECON-6/7 lineage).
Method : deepest-match attribution — scan each stack from the DEEPEST
         frame to the root; the first frame matching a class pattern
         names the class. Behavioral context frames (targeting, goals,
         navigation) therefore win over their own callees (getEntities,
         AABB.intersects), which is the economically honest attribution
         for AI-driven scans.
Inputs : leg#2 profile (fresh, 35410873485, 125610 samples) as primary;
         v3 profile (35399980345, 128124) as stability cross-check.
Outputs: class table (CPU), per-class top leaf frames, sub-splits of the
         attackable classes (navigation / goal-selector / targeting /
         travel), alloc-axis class table, >=5% sub-lane candidates.
"""
import collections
import os
import re
import sys

LEG2 = "/home/z/c-crussty/research/flat-traversal-2026-09-19/run-s7163-leg2-artifact"
V3 = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"

# class -> regex (checked deepest-first; first match wins)
PATTERNS = [
    ("fluid-push", re.compile(r"updateInWaterStateAndDoFluidPushing|updateFluidHeightAndDoFluidPushing|getFlow")),
    ("inside-pipeline", re.compile(r"checkInsideBlocks|applyEffectsFromBlocks|TraverseOps\.|forEachBlockIntersectedBetween|collidedWithFluid|collidedWithShapeMovingFrom")),
    ("targeting", re.compile(r"TargetGoal|TargetingConditions")),
    ("navigation", re.compile(r"PathNavigation|pathfinder/|PathFinder")),
    ("goal-selector", re.compile(r"GoalSelector|WrappedGoal|ai/goal/")),
    ("brain", re.compile(r"Brain\.|behavior/")),
    ("sensing", re.compile(r"Sensing|getMaxLocalRawBrightness|canSee")),
    ("controls", re.compile(r"MoveControl|LookControl|JumpControl|BodyRotationControl")),
    ("travel-physics", re.compile(r"LivingEntity\.travel|travelInAir|handleRelativeFrictionAndCalculateMovement|/Entity\.move|/Entity\.collide|collidedAlongVector|collideWithShapes")),
    ("broadphase", re.compile(r"pushEntities|getPushableEntities|getEntities|getEntitiesOfClass|getNearestPlayer|AABB\.intersects")),
    ("item-entity", re.compile(r"ItemEntity")),
    ("baseTick-other", re.compile(r"baseTick")),
]

SUBSPLITS = {
    "navigation": [
        ("createPath", re.compile(r"PathNavigation\.createPath")),
        ("findPath(A*)", re.compile(r"PathFinder\.findPath")),
        ("node-evaluator", re.compile(r"NodeEvaluator|Node\.|TargetNode|WalkNodeEvaluator")),
        ("nav-tick/recompute", re.compile(r"PathNavigation\.tick|recomputePath|PathNavigation\.moveTo")),
        ("done-navigation", re.compile(r"PathNavigation|pathfinder/|PathFinder")),
    ],
    "goal-selector": [
        ("canUse", re.compile(r"WrappedGoal\.canUse|Goal\.canUse|canUse\(")),
        ("canContinueToUse", re.compile(r"canContinueToUse")),
        ("goal-tick", re.compile(r"WrappedGoal\.tick|Goal\.tick|requiresUpdateEveryTick")),
        ("selector-tick", re.compile(r"GoalSelector\.tick")),
        ("done-goalsel", re.compile(r"GoalSelector|WrappedGoal|ai/goal/")),
    ],
    "targeting": [
        ("nearest-attackable", re.compile(r"NearestAttackableTargetGoal")),
        ("hurt-by", re.compile(r"HurtByTargetGoal")),
        ("targeting-cond", re.compile(r"TargetingConditions")),
        ("done-targeting", re.compile(r"TargetGoal|TargetingConditions")),
    ],
    "travel-physics": [
        ("travel", re.compile(r"LivingEntity\.travel")),
        ("travelInAir", re.compile(r"travelInAir")),
        ("friction/movement", re.compile(r"handleRelativeFrictionAndCalculateMovement")),
        ("Entity.move", re.compile(r"/Entity\.move")),
        ("Entity.collide", re.compile(r"/Entity\.collide")),
        ("collidedAlongVector", re.compile(r"collidedAlongVector")),
        ("done-travel", re.compile(r"travel|handleRelativeFrictionAndCalculateMovement|/Entity\.move|/Entity\.collide|collidedAlongVector|collideWithShapes")),
    ],
    "broadphase": [
        ("pushEntities", re.compile(r"pushEntities")),
        ("getPushableEntities", re.compile(r"getPushableEntities")),
        ("getEntities", re.compile(r"Level\.getEntities|getEntitiesOfClass|getNearestPlayer")),
        ("AABB.intersects", re.compile(r"AABB\.intersects")),
        ("done-broadphase", re.compile(r"pushEntities|getPushableEntities|getEntities|getEntitiesOfClass|getNearestPlayer|AABB\.intersects")),
    ],
}


def cpu_stacks(d):
    for line in open(os.path.join(d, "cpu-collapsed.txt"), errors="replace"):
        line = line.rstrip()
        if not line:
            continue
        m = line.rsplit(" ", 1)
        if len(m) != 2:
            continue
        try:
            w = int(m[1])
        except ValueError:
            continue
        yield w, m[0].split(";")


def alloc_stacks(d):
    p = os.path.join(d, "alloc-collapsed.txt")
    if not os.path.exists(p):
        return
    for line in open(p, errors="replace"):
        line = line.rstrip()
        if not line:
            continue
        m = line.rsplit(" ", 1)
        if len(m) != 2:
            continue
        try:
            w = int(m[1])
        except ValueError:
            continue
        yield w, m[0].split(";")


def classify_stack(frs):
    """deepest-match: scan from deepest frame to root; first pattern hit wins."""
    for f in reversed(frs):
        for cls, rx in PATTERNS:
            if rx.search(f):
                return cls
    return None


def subsplit(frs, cls):
    for name, rx in SUBSPLITS.get(cls, []):
        for f in reversed(frs):
            if rx.search(f):
                return name
    return "other"


def run(d, tag):
    total = 0
    ent = collections.Counter()
    ent_leaf = collections.defaultdict(collections.Counter)
    ent_sub = collections.defaultdict(collections.Counter)
    alloc_cls = collections.Counter()
    alloc_total = 0
    for w, frs in cpu_stacks(d):
        total += w
        if not any("RegionTickOps.tickBucket" in f or "tickNonPassenger" in f for f in frs):
            continue
        cls = classify_stack(frs) or "aiStep/entity-other"
        ent[cls] += w
        ent_leaf[cls][frs[-1]] += w
        if cls in SUBSPLITS:
            ent_sub[cls][subsplit(frs, cls)] += w
    for w, frs in alloc_stacks(d):
        alloc_total += w
        cls = classify_stack(frs)
        if cls:
            alloc_cls[cls] += w
    ent_sum = sum(ent.values())
    print(f"\n===== {tag} ({d}) =====")
    print(f"total CPU samples: {total}; entity phase: {ent_sum} ({100.0*ent_sum/total:.2f}%)")
    print("--- behavior classes (share of TOTAL CPU) ---")
    for k, v in ent.most_common():
        print(f"  {k:22s} {v:7d}  {100.0*v/total:5.2f}% CPU   ({100.0*v/ent_sum:5.1f}% of entity phase)")
    rest = ent_sum - sum(ent.values())
    if rest:
        pass  # 'aiStep/entity-other' already in ent
    print("--- alloc axis: class shares of total alloc samples ---")
    for k, v in alloc_cls.most_common(10):
        print(f"  {k:22s} {v:7d}  {100.0*v/alloc_total:5.2f}%")
    for cls in ("navigation", "goal-selector", "targeting", "travel-physics", "broadphase"):
        if cls not in ent_sub or not ent_sub[cls]:
            continue
        print(f"--- subsplit {cls} ---")
        for k, v in ent_sub[cls].most_common():
            print(f"  {k:22s} {v:7d}  ({100.0*v/ent[cls]:5.1f}% of class)")
    for cls in ("navigation", "goal-selector", "targeting", "travel-physics"):
        if cls not in ent_leaf:
            continue
        print(f"--- top leafs {cls} ---")
        for f, w in ent_leaf[cls].most_common(6):
            print(f"  {w:6d} {100.0*w/ent[cls]:5.1f}%  {f[-120:]}")
    return ent, total


def main():
    print("=" * 78)
    print("RECON-8: movement/AI surface -> behavior classes (deep-match)")
    print("=" * 78)
    run(LEG2, "LEG#2 35410873485 (fresh bank-base profile)")
    run(V3, "V3 35399980345 (cross-check)")


if __name__ == "__main__":
    main()
