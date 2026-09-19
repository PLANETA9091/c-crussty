#!/usr/bin/env python3
"""RECON-13: decompose the entity-behavior allocation family (other-entity
28.04% of ObjectAllocationSample count in the 10G diag base) into sub-lanes
>=5% using async-profiler alloc-collapsed stacks (s7165 10G clean bank).

Attribution: right-to-left scan (leaf = allocation site side first) — a stack
counts in the DEEPEST recognized family. Cross-check: top leaf classes.
Output: research/gc-recon-2026-09-19/RECON13_OTHER_ENTITY_LANES.md
"""
import re, sys, collections

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag"

FAMILIES = [
    ("navigation/pathfinding", re.compile(r"PathNavigation|PathFinder|Path\b|NodeEvaluator|WalkNodeEvaluator|FlyNodeEvaluator|SwimNodeEvaluator|PathFinder\b|TargetingNavigator|moveControl|MoveControl")),
    ("goals/behavior-selector", re.compile(r"GoalSelector|WrappedGoal|Goal\b|DefaultGoal|AvoidEntityGoal|RandomStrollGoal|RandomLookaroundGoal|FloatGoal|LookAtPlayerGoal|MeleeAttackGoal|WaterAvoidingRandomStrollGoal")),
    ("brain/sensing/behavior", re.compile(r"Brain\b|Sensor\b|Behavior\b|BehaviorControl|MemoryModule|LivingEntityReference|PosAndRot|EntityMemo")),
    ("look/attack-control", re.compile(r"LookControl|JumpControl|Attack|SmoothSwimmingMoveControl")),
    ("collision-context", re.compile(r"CollisionContext|LazyEntityCollisionContext|EntityCollisionContext")),
    ("spawn", re.compile(r"SpawnPlacements|NaturalSpawner|MobSpawnSettings|Spawner|SpawnGroup|WeightedList")),
    ("entity-tick-core", re.compile(r"Mob\.tick|LivingEntity\.tick|Entity\.tick|ServerLevel\.tick|tickEntity|Entity\.baseTick|aiStep|mobTick")),
    ("entity-lambda", re.compile(r"Entity\$\$Lambda|Mob\$\$Lambda|LivingEntity\$\$Lambda|ServerLevel\$\$Lambda|RegionTickOps")),
    ("entity-data/sync", re.compile(r"SynchedEntityData|DataWatcher|EntityDataAccessor")),
]


def leaf_class(frames):
    return frames[-1] if frames else "?"


def analyze(path):
    lanes = collections.Counter()
    leaves = collections.Counter()
    total = 0
    entity_total = 0
    for line in open(path, errors="ignore"):
        p = line.rstrip("\n").rsplit(" ", 1)
        if len(p) != 2:
            continue
        frames, cnt = p[0].split(";"), p[1]
        try:
            n = int(cnt)
        except ValueError:
            continue
        total += n
        leaves[leaf_class(frames)] += n
        for f in reversed(frames):
            hit = None
            for name, rx in FAMILIES:
                if rx.search(f):
                    hit = name
                    break
            if hit:
                lanes[hit] += n
                entity_total += n
                break
    return total, entity_total, lanes, leaves


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else f"{BASE}/alloc-collapsed.txt"
    total, entity_total, lanes, leaves = analyze(path)
    print(f"total alloc samples: {total:,}")
    print(f"entity-behavior family (deepest-frame attribution): {entity_total:,} = {100*entity_total/total:.2f}%")
    print("\n== entity sub-lanes ==")
    for name, n in lanes.most_common():
        print(f"{name:28s} {n:10,}  {100*n/total:5.2f}%  (of family {100*n/entity_total:5.1f}%)")
    print("\n== top-25 leaf allocation sites ==")
    for cls, n in leaves.most_common(25):
        print(f"{100*n/total:5.2f}%  {cls[:120]}")
    doc = [f"# RECON-13 — декомпозиция entity-аллокаций ({path})\n",
           f"- всего аллок-семплов: {total:,}; entity-семья (глубочайший фрейм): {entity_total:,} = {100*entity_total/total:.2f}%",
           "\n| под-лейн | семплов | % всего | % семьи |", "|---|---|---|---|"]
    for name, n in lanes.most_common():
        doc.append(f"| {name} | {n:,} | {100*n/total:.2f}% | {100*n/entity_total:.1f}% |")
    doc += ["", "| топ-25 листьев аллокации | % всего |", "|---|---|"]
    for cls, n in leaves.most_common(25):
        doc.append(f"| {cls[:120]} | {100*n/total:.2f}% |")
    open(f"{BASE}/../../gc-recon-2026-09-19/RECON13_OTHER_ENTITY_LANES.md", "w").write("\n".join(doc) + "\n")
    print("\nwritten: RECON13_OTHER_ENTITY_LANES.md")


if __name__ == "__main__":
    main()
