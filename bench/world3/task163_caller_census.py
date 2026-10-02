#!/usr/bin/env python3
"""task163 caller-census: WHO calls the chunk-state read lane (run#10 exact data)?

Answers the twin's red-team objection (TASK-230: 'per-get JNI crossing eats the
win; lane viable ONLY as a batch lens') with numbers: for every sample whose
LEAF is in the lane (PalettedContainer.get / readPalette / SimpleBitStorage.get /
getBlockStateFinal), walk the ancestor stack and classify the nearest
meaningful caller. Shares of batchable (collision scans, BE loops, entity
movement = multi-block spans) vs scattered (random tick sampling) decide the
lever's fate: advance batch-lens ONLY if batchable share supports a >=3% gate.
"""
import collections
import re
import sys

path = sys.argv[1] if len(sys.argv) > 1 else \
    "/home/z/my-project/scripts/bench3_research/run10/cpu-collapsed.txt"

LANE_LEAVES = (
    "net/minecraft/world/level/chunk/PalettedContainer.get",
    "net/minecraft/world/level/chunk/PalettedContainer.readPalette",
    "net/minecraft/util/SimpleBitStorage.get",
    "net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal",
    "net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock",
)

# ancestor substring -> (label, batchable?)  first match from leaf upward wins
CALLERS = [
    ("getCollisionsForBlocksOrWorldBorder", ("collision: moonrise block scan", True)),
    ("CollisionUtil", ("collision: moonrise block scan", True)),
    ("findCollisions", ("collision: moonrise block scan", True)),
    ("collide", ("collision: entity movement", True)),
    ("AbstractBoat", ("boat movement/collision", True)),
    ("Entity.move", ("entity movement/collision", True)),
    ("move", ("entity movement/collision", True)),
    ("getBlockPathType", ("pathfinding", True)),
    ("getBlockCost", ("pathfinding/nav", True)),
    ("PathFinder", ("pathfinding", True)),
    ("tickBlockEntities", ("block entity tick loop", True)),
    ("BlockEntity", ("block entity tick loop", True)),
    ("HopperBlockEntity", ("hopper push/pull", True)),
    ("optimiseRandomTick", ("random tick sampling (scattered)", False)),
    ("randomTick", ("random tick sampling (scattered)", False)),
    ("tickSpawning", ("mob spawning scans", True)),
    ("Spawning", ("mob spawning scans", True)),
    ("getLightValue", ("light queries", False)),
    ("isRainingAtPos", ("environment queries", False)),
    ("mayInteract", ("protection checks", False)),
    ("destroyBlock", ("block destroy", True)),
    ("playerCheckBlock", ("player actions", False)),
    ("ServerEntity.sendChanges", ("network sync", True)),
    ("sendChanges", ("network sync", True)),
    ("SectionRenderCache", ("render", True)),
]

total_lane = 0
per_leaf = collections.Counter()
per_caller = collections.Counter()
per_caller_batchable = collections.Counter()
depth_hist = collections.Counter()
unmatched = collections.Counter()

with open(path, encoding="utf-8", errors="replace") as f:
    for line in f:
        line = line.rstrip("\n")
        if not line or " " not in line:
            continue
        stack, _, cnt = line.rpartition(" ")
        try:
            n = int(cnt)
        except ValueError:
            continue
        frames = stack.split(";")
        leaf = frames[-1]
        if not any(leaf.startswith(p) for p in LANE_LEAVES):
            continue
        total_lane += n
        per_leaf[leaf.split("(")[0]] += n
        depth_hist[len(frames)] += n
        label = None
        for fr in reversed(frames[:-1]):
            for marker, (lab, batchable) in CALLERS:
                if marker in fr:
                    label = (lab, batchable)
                    break
            if label:
                break
        if label is None:
            # drill-down: nearest ancestor OUTSIDE the lane's own plumbing
            SKIP = ("PalettedContainer", "LevelChunk", "SimpleBitStorage",
                    "BlockBehaviour", "ZeroCollidingReferenceStateTable",
                    "PalettedContainerRO", "LevelReader", "BlockGetter",
                    "net/minecraft/world/level/Level.", "getChunk", "ChunkAccess")
            for fr in reversed(frames[:-1]):
                if not any(s in fr for s in SKIP):
                    unmatched[fr.split("(")[0][:70]] += n
                    break
            label = ("caller-unmatched (see per-frame below)", False)
        per_caller[label[0]] += n
        if label[1]:
            per_caller_batchable[label[0]] += n

print(f"total lane samples: {total_lane} / 224660 = {100.0*total_lane/224660:.1f}%")
print("\n-- per leaf --")
for leaf, n in per_leaf.most_common():
    print(f"  {n:>7}  {100.0*n/total_lane:5.1f}%  {leaf.split('/')[-1]}")
print("\n-- per caller (nearest meaningful ancestor) --")
bat = sum(n for n in per_caller_batchable.values())
scat = total_lane - bat
for lab, n in per_caller.most_common():
    print(f"  {n:>7}  {100.0*n/total_lane:5.1f}%  {'BATCHABLE' if lab in per_caller_batchable and per_caller_batchable[lab]==n else 'scattered/mixed':<14} {lab}")
print(f"\nVERDICT: batchable {bat} ({100.0*bat/total_lane:.1f}%) vs scattered {scat} ({100.0*scat/total_lane:.1f}%)")
print(f"batchable share of TOTAL tick CPU: {100.0*bat/224660:.1f}%")
