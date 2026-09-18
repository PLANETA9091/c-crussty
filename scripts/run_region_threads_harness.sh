#!/usr/bin/env bash
# Build & run the REGION-THREADS offline harness (S7-156, TASK-295).
#
# Prereqs:
#   - entityinside/build/RegionTickOps*.class (scripts/build_region_tick_ops.sh)
#   - tests/out/{ServerLevel,EntityCallbacks}.regionthreads.patched.class
#     (cargo test --lib region_threads)
#
# INJECTS-ONLY: plain JVM, real kernel classes, NO server boot. The child
# run carries CRUSSTY_REGION_THREADS=2 (the parallel container gate).
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

JAVA="${JAVA:-/tmp/toolchain/jdk-21.0.12.1+1/bin/java}"
JAVAC="${JAVAC:-/tmp/toolchain/jdk-21.0.12.1+1/bin/javac}"
if [ ! -x "$JAVAC" ] && [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac; fi
if [ ! -x "$JAVA" ] && [ -x /tmp/jdk21/bin/java ]; then JAVA=/tmp/jdk21/bin/java; fi

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')

HARNESS_BUILD=entityinside/build-harness-regionthreads
mkdir -p "$HARNESS_BUILD"

"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$LIBCP" -d "$HARNESS_BUILD" \
  entityinside/harness/RegionThreadsHarness.java

# parent run: structural/wiring/dormant; it spawns the W=2 child itself
exec "$JAVA" -cp "$HARNESS_BUILD:$KERNEL:$LIBCP" \
  net.minecraft.world.entity.RegionThreadsHarness "$@"
