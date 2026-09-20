#!/usr/bin/env bash
# Build & run the ITEMS-MONO offline harness (TASK-396-F, MEGA-ROUND-1
# vector F — lever_flag="items_mono").
#
# Prereqs:
#   - entityinside/build/RegionTickOps.class (javac of the edited bridge
#     source; the class the rust patcher retargets tickNonPassenger INTO).
#
# INJECTS-ONLY: plain JVM, real kernel jar, NO server boot. Three gates:
#   1. STRUCTURAL  — bridge bytes define+link against the real kernel,
#      entityTick is public static (Lnet/minecraft/world/entity/Entity;)V.
#   2. WIRING      — raw byte audit of the split shape in the bridge bytes.
#   3. BEHAVIORAL  — the REAL entityTick bytecode on a child-first shadow
#      loader (stub Entity/ItemEntity/OtherEntity) reproduces the vanilla
#      virtual-dispatch sequence EXACTLY on a deterministic 70/30 layout.
#
# The run carries CRUSSTY_LEVER_FLAG="" (dormant): entityTick is
# flag-independent by design (the retarget itself is the flag's arm).
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

JDK="${JDK:-/home/z/tools/jdk-21.0.12.1+1}"
JAVA="${JAVA:-$JDK/bin/java}"
JAVAC="${JAVAC:-$JDK/bin/javac}"
KERNEL="${KERNEL:-research/gc-recon-2026-09-19/run-s7204-bitmask/patched-kernel.jar}"

SHADOW_BUILD=entityinside/build-harness-itemsmono-shadow
HARNESS_BUILD=entityinside/build-harness-itemsmono
rm -rf "$SHADOW_BUILD" "$HARNESS_BUILD"
mkdir -p "$SHADOW_BUILD" "$HARNESS_BUILD"

# Stage 1: shadow stubs — compiled with NO kernel jar on the classpath so
# Entity/ItemEntity resolve to the STUB sources (child-first shadow set).
"$JAVAC" --release 21 -proc:none -d "$SHADOW_BUILD" \
  entityinside/harness/itemsmono/net/minecraft/world/entity/Entity.java \
  entityinside/harness/itemsmono/net/minecraft/world/entity/OtherEntity.java \
  entityinside/harness/itemsmono/net/minecraft/world/entity/item/ItemEntity.java \
  entityinside/harness/itemsmono/net/minecraft/server/level/ServerLevel.java

# Stage 2: the harness itself (reflection-only; kernel jar for GATE1).
"$JAVAC" --release 21 -proc:none -cp "$KERNEL" -d "$HARNESS_BUILD" \
  entityinside/harness/ItemsMonoHarness.java

# Stage 3: run — dormant flag (vanilla clinit), three gates. The shadow
# loader also defines a minimal ServerLevel stub, so the bridge's S7-170
# Unsafe static block never pulls the real kernel ServerLevel clinit (the
# harness stays free of the full server library set).
export CRUSSTY_LEVER_FLAG=""
export CRUSSTY_LEVER_ARG=""
exec "$JAVA" -cp "$HARNESS_BUILD:$KERNEL" \
  net.minecraft.world.entity.ItemsMonoHarness \
  entityinside/build "$SHADOW_BUILD" "$KERNEL" "$@"
