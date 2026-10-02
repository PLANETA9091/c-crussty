#!/usr/bin/env python3
"""BOAT STEP-0 kill-gate anatomy (task164): exact-frame mining of AbstractBoat.tick
from run#10 raw collapsed (224,660 samples, byte-same artifact as the ranked report).

Preregistered (TASK-230/S7-93): replaceable base >= 4% of TOTAL tick CPU => GO
(boat_native.rs core + parity ladder); < 4% => REFUTED pre-implementation, next
lever = BATCH-RNG (same round allowed).

Replaceable-base model (declared BEFORE reading results):
  + leaf == AbstractBoat.tick (own body: float physics, status switches)
  + boat callees that are local math or LOCAL region reads (status/friction/
    underwater/inWater = small neighborhood block scans -> snapshot-able in
    ONE upcall per boat per tick; physics float math -> SIMD-able in Rust)
  + world-read leaves inside boat stacks (PalettedContainer.get /
    getBlockStateFinal / SimpleBitStorage.get / BlockBehaviour* / CollisionUtil
    / getCollisionsForBlocks* / fluid) -> become a per-tick LOCAL SNAPSHOT
  + broadphase callee ChunkEntitySlices$EntityCollectionBySection.getEntities
    (entity_mirror infrastructure, task161)
  - irreducible: SynchedEntityData / VarHandle / ServerEntity.sendChanges /
    networking / moonrise scheduling glue (stays JVM-side even after patch)
"""
import collections
import sys

PATH = sys.argv[1] if len(sys.argv) > 1 else \
    "/home/z/my-project/scripts/bench3_research/run10/cpu-collapsed.txt"
TOTAL = 224660  # run#10 canonical

BOAT = "net/minecraft/world/entity/vehicle/AbstractBoat.tick"
WORLD_READ = ("PalettedContainer", "LevelChunk", "SimpleBitStorage",
              "BlockBehaviour", "CollisionUtil", "getCollisionsForBlocks",
              "FlowingFluid", "updateFluidHeightAndDoFluidPushing",
              "getFluidHeight", "BlockState")
BROADPHASE = ("ChunkEntitySlices", "getEntities")
IRREDUCIBLE = ("SynchedEntityData", "ServerEntity", "sendChanges",
               "VarHandle", "Packet", "send")
BOAT_SUB = ("AbstractBoat.push", "AbstractBoat.getStatus",
            "AbstractBoat.getGroundFriction", "AbstractBoat.isUnderwater",
            "AbstractBoat.checkInWater", "AbstractBoat.checkFallDamage",
            "AbstractBoat.floatBoat", "AbstractBoat.lerp",
            "AbstractBoat.getWaterLevelAbove", "AbstractBoat.isEyeInFluid")

presence = 0
self_t = 0
leaf_by_class = collections.Counter()
sub_presence = collections.Counter()
irred = 0
world_read = 0
broad = 0
other_leaf = 0

def classify(leaf: str) -> str:
    if leaf == BOAT:
        return "SELF"
    if any(k in leaf for k in IRREDUCIBLE):
        return "IRREDUCIBLE"
    if any(k in leaf for k in WORLD_READ):
        return "WORLD_READ"
    if any(k in leaf for k in BROADPHASE):
        return "BROADPHASE"
    return "OTHER"

with open(PATH, encoding="utf-8", errors="replace") as f:
    for line in f:
        line = line.rstrip("\n")
        if not line or " " not in line:
            continue
        stack, _, cnt = line.rpartition(" ")
        try:
            n = int(cnt)
        except ValueError:
            continue
        if BOAT not in stack:
            continue
        presence += n
        frames = stack.split(";")
        leaf = frames[-1]
        if leaf == BOAT:
            self_t += n
        cls = classify(leaf)
        leaf_by_class[cls] += n
        if cls == "IRREDUCIBLE":
            irred += n
        elif cls == "WORLD_READ":
            world_read += n
        elif cls == "BROADPHASE":
            broad += n
        else:
            other_leaf += n
        for fr in frames:
            for sub in BOAT_SUB:
                if sub in fr:
                    sub_presence[sub.split(".")[-1]] += n
                    break
        # top OTHER leaves for transparency
        if cls == "OTHER":
            leaf_by_class["leaf:" + leaf.split("(")[0][:80]] += n

print(f"BOAT presence: {presence} = {100.0*presence/TOTAL:.2f}% of total CPU")
print(f"  SELF (leaf=AbstractBoat.tick): {self_t} = {100.0*self_t/TOTAL:.2f}%")
print(f"  leaf class split within boat stacks:")
for cls in ("SELF", "WORLD_READ", "BROADPHASE", "IRREDUCIBLE", "OTHER"):
    n = leaf_by_class.get(cls, 0)
    print(f"    {cls:<12} {n:>6} = {100.0*n/TOTAL:.2f}% total")
print(f"  boat sub-method presence (any frame):")
for sub, n in sub_presence.most_common(10):
    print(f"    {n:>6} = {100.0*n/TOTAL:.2f}%  {sub}")
print("\n-- top OTHER leaves (within boat stacks) --")
for fr, n in [(k, v) for k, v in leaf_by_class.items() if k.startswith("leaf:")][:15]:
    print(f"  {n:>6} = {100.0*n/TOTAL:.2f}%  {fr[5:]}")

replaceable = leaf_by_class.get("SELF", 0) + world_read + broad + \
    (other_leaf - irred)  # OTHER minus irreducible = math/misc native-able
irr = leaf_by_class.get("IRREDUCIBLE", 0)
print(f"\nREPLACEABLE-BASE MODEL (pre-declared):")
print(f"  self + world-read + broadphase + other-math = "
      f"{self_t}+{world_read}+{broad}+{max(other_leaf,0)} = {replaceable}")
print(f"  = {100.0*replaceable/TOTAL:.2f}% of TOTAL tick CPU")
print(f"  irreducible (stays JVM): {irr} = {100.0*irr/TOTAL:.2f}%")
gate = 100.0 * replaceable / TOTAL
print(f"\nSTEP-0 KILL-GATE (>= 4% replaceable): "
      f"{'GO' if gate >= 4.0 else 'REFUTED'} ({gate:.2f}%)")
