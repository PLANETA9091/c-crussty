#!/usr/bin/env bash
# S7-170 NAV-MOBS-GUARD offline lockstep (RECON-22 / TASK-348).
# Evidence (first PASS run 2026-09-20 ~04:3x +08):
#   [1] structural link OK (kernel jar s7178-recal, Unsafe offset=308)
#   [2] 240,000-op deterministic replay: vanilla ObjectOpenHashSet vs
#       GuardedNavigatingMobs — iteration sequences BIT-EXACT at every 4k
#       checkpoint (fastutil clone preserves the hash table order)
#   [3] snapshot semantics: frozen view, no CME, fresh iterator sees adds
#   [4] concurrency stress: 8 threads x 50k ops, 4 snapshot readers,
#       0 exceptions on the guard; RAW control detonated with the EXACT
#       production signature: NPE ObjectOpenHashSet$SetIterator
#       ("wrapped" is null) — the s7176/s7180/s7186/s7189 crash class
#   [5] final consistency: walk==size(), all real-Mob markers
set -euo pipefail
cd "$(dirname "$0")/.."
JAVA=/tmp/jdk21/bin/java
JAVAC=/tmp/jdk21/bin/javac
KERNEL_JAR="$(ls research/gc-recon-2026-09-19/run-s7178-recal/patched-kernel.jar 2>/dev/null || ls research/*/run-*/patched-kernel.jar | tail -1)"
LIBS="/tmp/alllibs2"   # full server library set (fastutil/joml/purpur-libs)
BUILD=/tmp/s7170build
HARN=/tmp/s7170harness
mkdir -p "$BUILD" "$HARN"

# 1) ops classes (RegionTickOps carries the S7-170 NAV-MOBS-GUARD)
$JAVAC --release 21 -proc:none -cp "$KERNEL_JAR:$LIBS/*" -d "$BUILD" \
  entityinside/net/minecraft/world/entity/RegionTickOps.java \
  entityinside/net/minecraft/world/entity/BatchCollector.java \
  entityinside/net/minecraft/server/level/TrackerTickOps.java \
  entityinside/net/minecraft/util/RngOps.java \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java

# 2) harness
$JAVAC --release 21 -proc:none -cp "$KERNEL_JAR:$LIBS/*:$BUILD" -d "$HARN" \
  entityinside/harness/NavMobsLockstepHarness.java

# 3) run
$JAVA -Xmx2g -cp "$KERNEL_JAR:$LIBS/*:$BUILD:$HARN" harness.NavMobsLockstepHarness
