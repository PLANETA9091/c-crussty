#!/usr/bin/env bash
# TRAVEL-DIET v2a offline lockstep (RECON-21): bit-exact mirrors + step ladder.
# Evidence (first PASS run 2026-09-20 ~04:1x +08):
#   [1] structural link OK (kernel jar s7178-recal)
#   [2] 1,000,000 expand/move mirror cases bit-exact (corners: +-0.0, NaN,
#       +-Inf, 1e-300, 1e300, eps -9.999999747378752E-6)
#   [3] 50,000 calculateStepHeights lockstep scenarios bit-exact vs the
#       REAL private static Entity method (order-sensitive float ladder)
set -euo pipefail
cd "$(dirname "$0")/.."
JAVA=/tmp/jdk21/bin/java
JAVAC=/tmp/jdk21/bin/javac
# MUST be the same jar the RECON-21 contract was dumped from (s7178-recal):
# a stale kernel (s7160 era) misses moonrise$ shape hooks -> link failure.
KERNEL_JAR="$(ls research/gc-recon-2026-09-19/run-s7178-recal/patched-kernel.jar 2>/dev/null || ls research/*/run-*/patched-kernel.jar | tail -1)"
FJ=$(find /tmp/my-project/scripts/f1_batch_rng /tmp/pdec /tmp/s7147mat -maxdepth 4 -name "fastutil*.jar" 2>/dev/null | head -1 || true)
# Entity.<clinit> pulls brigadier/authlib/datafixer/purpur-libs: /tmp/alllibs2
# carries the full server library set (alllibs2/* wildcard on the runtime cp).
LIBS="/tmp/alllibs2"
JOML=$(find /tmp/my-project/scripts/f1_batch_rng /tmp/pdec /tmp/s7147mat -maxdepth 5 -name "joml*.jar" 2>/dev/null | head -1 || true)
OUT=/tmp/traveldiet-harness
mkdir -p "$OUT"
$JAVAC -proc:none -cp "$KERNEL_JAR:$LIBS/*:$FJ:entityinside/build" -d "$OUT" entityinside/harness/TravelDietLockstepHarness.java
$JAVA -cp "$KERNEL_JAR:$LIBS/*:$FJ:$JOML:entityinside/build:$OUT" TravelDietLockstepHarness
