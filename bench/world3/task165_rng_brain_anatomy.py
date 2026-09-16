#!/usr/bin/env python3
"""task165 STEP-0 anatomy: BATCH-RNG kill-gate + Brain.tick cluster mining.

Owner north-star (2026-09-16): 20 TPS / minimal MSPT on MineShield-3
(force-loaded chunks, live mob spawn/despawn as-if-players). Owner directive:
NO FALLBACKS - real native acceleration only, config-only "wins" forbidden.

Data: run#11 cpu-collapsed (first FULL v2 run; 3-window asprof) + alloc-collapsed
for the GC/F2 tie-in. Preregistered models (declared BEFORE reading results):

BATCH-RNG replaceable base (random-tick lane, stack ancestry
ServerLevel.tickChunk -> ServerLevel.optimiseRandomTick):
  + RNG machinery leaves: RandomSource.advanceSeed / next* / setSeed /
    ThreadLocalRandom math -> the whole per-position LCG advance batchable
    into ONE JNI call per chunk-section (48-bit LCG, median-exact, positions
    precomputed in Rust; block .tick() bodies stay JVM)
  + SELF loop overhead of optimiseRandomTick (position math, pick loop) ->
    bounded both ways: lower bound = RNG-only, upper bound = RNG+SELF
  - irreducible: block randomTick bodies (do-not-duplicate law), blockstate
    property accessors (StateHolder.getValue / ZeroCollidingReferenceStateTable),
    world reads (PalettedContainer/SimpleBitStorage/LevelChunk), JVM infra
  KILL-GATE: replaceable >= 3.0% of TOTAL tick CPU => GO (bytecode contract
  from booted kernel, then Rust core + parity); < 3.0% => REFUTED pre-code.

Brain.tick cluster (F3 discovery 6.85% presence) - anatomy only this round:
  split presence into SENSORS / BEHAVIOR-START / BEHAVIOR-TICK / SELF /
  STREAM-overhead / world reads / navigation / per-entity-class top-N.
  Purpose: pre-size the next lever (BRAIN-LENS) with the same honesty as BOAT.
"""
import collections
import sys

CPU = sys.argv[1] if len(sys.argv) > 1 else \
    "/home/z/my-project/scripts/bench3_research/run11/cpu-collapsed.txt"
ALLOC = sys.argv[2] if len(sys.argv) > 2 else \
    "/home/z/my-project/scripts/bench3_research/run11/alloc-collapsed.txt"

RNG_LANE = "net/minecraft/server/level/ServerLevel.optimiseRandomTick"
SELF = "net/minecraft/server/level/ServerLevel.optimiseRandomTick"
RNG = ("advanceSeed", "RandomSource", "LegacyRandomSource", "BitRandomSource",
       "WorldgenRandom", "ThreadLocalRandom", "Xoroshiro",
       "java/util/random")
BLOCK_BODY_MARK = ("$BlockStateBase.randomTick", "blockstate/StateHolder.getValue")
WORLD_READ = ("PalettedContainer", "SimpleBitStorage", "LevelChunk",
              "getBlockState", "getBlockStateFinal", "ZeroCollidingReferenceStateTable",
              "FlowingFluid", "getFluid", "BlockBehaviour", "CollisionUtil",
              "getCollisionsForBlocks", "LevelReader", "ChunkAccess",
              "LevelChunkSection", "PalettedContainerRO")
STREAM = ("java/util/stream", "ReferencePipeline", "AbstractPipeline",
          "ReduceOps", "MatchOps", "ImmutableCollections", "CollSer")

BRAIN = "net/minecraft/world/entity/ai/Brain.tick"
BRAIN_SENSORS = "Brain.tickSensors"
BRAIN_START = "Brain.startEachNonRunningBehavior"
BRAIN_TICKB = "Brain.tickEachRunningBehavior"
NAV = ("PathNavigation", "GroundPathNavigation", "trimPath", "moveTo",
       "PathFinder", "Path")

total = 0
# random-tick lane
rng_self = 0
rng_rng = 0
rng_block = 0
rng_world = 0
rng_other = collections.Counter()
lane_total = 0
# brain lane
b_self = 0
b_sens = 0
b_start = 0
b_tickb = 0
b_stream = 0
b_world = 0
b_nav = 0
b_other_leaf = collections.Counter()
b_entity = collections.Counter()
b_total = 0
# alloc signal for brain + rng lanes
alloc_rng = 0
alloc_brain = 0
alloc_total = 0
alloc_brain_sites = collections.Counter()

def entity_class(stack):
    # top-most concrete mob class under tickNonPassenger
    for mark in ("tickNonPassenger", "tickPassenger"):
        if mark not in stack:
            continue
        tail = stack.split(mark, 1)[1]
        for tok in tail.split(";"):
            if tok.startswith("net/minecraft/world/entity/") and ".tick" not in tok \
               and ".aiStep" not in tok and ".customServerAiStep" not in tok \
               and ".inactiveTick" not in tok and "$" not in tok \
               and "Lambda" not in tok and "level/" not in tok \
               and "/ai/" not in tok:
                return tok.split("/")[-1]
    return "?"

def has(s, *keys):
    return any(k in s for k in keys)

with open(CPU) as f:
    for line in f:
        line = line.rstrip("\n")
        if not line:
            continue
        sp = line.rfind(" ")
        try:
            n = int(line[sp + 1:])
        except ValueError:
            continue
        stack = line[:sp]
        total += n

        if RNG_LANE in stack:
            lane_total += n
            leaf = stack.split(";")[-1]
            if leaf == SELF:
                rng_self += n
            elif has(leaf, *RNG):
                rng_rng += n
            elif has(stack, *WORLD_READ):
                rng_world += n
            elif "randomTick" in stack and has(stack, *BLOCK_BODY_MARK):
                rng_block += n
            else:
                rng_other[leaf] += n

        if BRAIN in stack:
            b_total += n
            leaf = stack.split(";")[-1]
            seg = stack.split(BRAIN, 1)[1]
            b_entity[entity_class(stack)] += n
            if has(seg, BRAIN_SENSORS):
                b_sens += n
            elif has(seg, BRAIN_START):
                b_start += n
            elif has(seg, BRAIN_TICKB):
                b_tickb += n
            elif leaf == BRAIN:
                b_self += n
            if has(leaf, *STREAM):
                b_stream += n
            if has(leaf, *WORLD_READ):
                b_world += n
            if has(leaf, *NAV):
                b_nav += n
            if leaf != BRAIN and not has(leaf, *WORLD_READ):
                b_other_leaf[leaf] += n

try:
    with open(ALLOC) as f:
        for line in f:
            sp = line.rfind(" ")
            try:
                n = int(line[sp + 1:])
            except ValueError:
                continue
            stack = line[:sp]
            alloc_total += n
            if RNG_LANE in stack:
                alloc_rng += n
            if BRAIN in stack:
                alloc_brain += n
                leaf = stack.split(";")[-1]
                alloc_brain_sites[leaf] += n
except FileNotFoundError:
    print(f"[warn] alloc file absent: {ALLOC}")

pct = lambda x: 100.0 * x / total if total else 0.0

print(f"TOTAL cpu samples run#11: {total}")
print()
print("=" * 72)
print("A) RANDOM-TICK LANE (BATCH-RNG STEP-0 kill-gate, prereg >= 3.0%)")
print("=" * 72)
print(f"lane presence           : {lane_total:8d}  {pct(lane_total):6.2f}%")
print(f"  SELF loop overhead    : {rng_self:8d}  {pct(rng_self):6.2f}%")
print(f"  RNG machinery (leaf)  : {rng_rng:8d}  {pct(rng_rng):6.2f}%")
print(f"  world reads           : {rng_world:8d}  {pct(rng_world):6.2f}%")
print(f"  block bodies (JVM law): {rng_block:8d}  {pct(rng_block):6.2f}%")
oth = sum(rng_other.values())
print(f"  other leaves          : {oth:8d}  {pct(oth):6.2f}%")
for leaf, n in rng_other.most_common(8):
    print(f"      {pct(n):5.2f}%  {leaf[:110]}")
rng_only = pct(rng_rng)
rng_up = pct(rng_rng + rng_self)
print(f"REPLACEABLE lower bound (RNG-only)      : {rng_only:5.2f}%  "
      f"{'PASS' if rng_only >= 3.0 else 'FAIL'} vs 3.0% gate")
print(f"REPLACEABLE upper bound (RNG+SELF loop) : {rng_up:5.2f}%  "
      f"{'PASS' if rng_up >= 3.0 else 'FAIL'} vs 3.0% gate")
print(f"alloc in lane           : {alloc_rng}  ({pct(alloc_rng):.2f}% of tick allocs)")
print()
print("=" * 72)
print("B) Brain.tick CLUSTER (anatomy for BRAIN-LENS pre-sizing)")
print("=" * 72)
print(f"cluster presence        : {b_total:8d}  {pct(b_total):6.2f}%")
print(f"  tickSensors           : {b_sens:8d}  {pct(b_sens):6.2f}%")
print(f"  startEachNonRunning   : {b_start:8d}  {pct(b_start):6.2f}%")
print(f"  tickEachRunning       : {b_tickb:8d}  {pct(b_tickb):6.2f}%")
print(f"  self                  : {b_self:8d}  {pct(b_self):6.2f}%")
print(f"  stream leaves (any)   : {b_stream:8d}  {pct(b_stream):6.2f}%")
print(f"  world-read leaves     : {b_world:8d}  {pct(b_world):6.2f}%")
print(f"  navigation leaves     : {b_nav:8d}  {pct(b_nav):6.2f}%")
print(f"alloc in cluster        : {alloc_brain}  ({pct(alloc_brain):.2f}% of tick allocs)")
print("top entity classes driving Brain.tick:")
for e, n in b_entity.most_common(10):
    print(f"    {pct(n):5.2f}%  {e}")
print("top non-world leaves:")
for leaf, n in b_other_leaf.most_common(12):
    print(f"    {pct(n):5.2f}%  {leaf[:110]}")
print()
print(f"alloc total samples     : {alloc_total}")
