#!/usr/bin/env bash
# Build & run the FLUID-DIRTY behavioral lockstep harness (S7-152, TASK-291).
# Tier 2 of the offline verification (§S7-150-G5 core): bit-exact differential
# vanilla-scan vs FluidPushOps.scan on Unsafe scan-contract fixtures.
#
# INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

JAVA="${JAVA:-/tmp/toolchain/jdk-21.0.12.1+1/bin/java}"
JAVAC="${JAVAC:-/tmp/toolchain/jdk-21.0.12.1+1/bin/javac}"

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
JOML_JAR=$(find "$LIBS/org/joml" -name 'joml-*.jar' | head -1)
FASTUTIL_JAR=$(find "$LIBS/it/unimi/dsi/fastutil" -name 'fastutil-*.jar' | head -1)

HARNESS_BUILD=entityinside/build-harness-fluidpushops
mkdir -p "$HARNESS_BUILD"

"$JAVAC" -proc:none --release 21 \
  -cp "entityinside/build:$KERNEL:$JOML_JAR:$FASTUTIL_JAR:$LIBCP" \
  -d "$HARNESS_BUILD" \
  entityinside/harness/FluidDirtyLockHarness.java

# Classpath: bridge + harness first (shadow), then PATCHED kernel, then libs.
exec "$JAVA" \
  -cp "$HARNESS_BUILD:entityinside/build:$KERNEL:$LIBCP" \
  net.minecraft.world.entity.FluidDirtyLockHarness
