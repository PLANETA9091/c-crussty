#!/usr/bin/env bash
# Build & run the REGION-THREADS PG1 lockstep harness (S7-157).
#
# Gate preregistered in S7-155/GOAL: per-entity state bit-exact between
# vanilla-sequential (W=1) and region-parallel (W=2/W=4) on a deterministic
# 60-tick scenario with a mid-tick mutation storm (7 removals + 20 adds
# through the REAL retargeted guard sites).
#
# INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

JAVA="${JAVA:-/tmp/toolchain/jdk-21.0.12.1+1/bin/java}"
JAVAC="${JAVAC:-/tmp/toolchain/jdk-21.0.12.1+1/bin/javac}"
if [ ! -x "$JAVAC" ] && [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac; fi
if [ ! -x "$JAVA" ] && [ -x /tmp/jdk21/bin/java ]; then JAVA=/tmp/jdk21/bin/java; fi

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')

HARNESS_BUILD=entityinside/build-harness-regionlockstep
mkdir -p "$HARNESS_BUILD"

"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$LIBCP" -d "$HARNESS_BUILD" \
  entityinside/harness/RegionLockstepHarness.java

exec "$JAVA" -cp "$HARNESS_BUILD:$KERNEL:$LIBCP" \
  net.minecraft.world.entity.RegionLockstepHarness "$@"
