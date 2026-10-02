#!/usr/bin/env bash
# run_inside_bitmask_lockstep.sh — INSIDE-BITMASK offline oracle (RECON-33
# contract, lever #15, TASK-358): hull-superset vs REAL traversal + section
# truth. Precedent: run_travel_diet_lockstep.sh.
# Evidence (first PASS run 2026-09-20 ~09:3x +08):
#   [1] structural+arm: InsideBitmaskOps ARMED on real kernel jar (s7194)
#   [2] 1,000,000 randomized/adversarial cases, 210,513,534 positions visited
#       by the REAL BlockGetter.forEachBlockIntersectedBetween — every one
#       inside the REAL InsideBitmaskOps.sweptHullInto hull block-domain
#   [3] 20,000 section sequences / 249,217 real setBlockState ops:
#       hasOnlyAir() == manual all-air scan AND nonEmptyBlockCount == manual
#       !isAir recount (fluids are NOT-air -> water sections never skippable)
#   ALL PASS, RC=0.
set -euo pipefail
cd "$(dirname "$0")/.."
JAVA=/tmp/jdk21/bin/java
JAVAC=/tmp/jdk21/bin/javac
# MUST be the same jar the RECON-33 contract was dumped from (s7194):
KERNEL_JAR="research/gc-recon-2026-09-19/run-s7194-zeroalloc-v1/patched-kernel.jar"
# Kernel deps (mojang serialization etc.): paperclip INSTALL-ONLY bootstrap
# (no server boot — INJECTS-ONLY discipline):
#   mkdir -p /tmp/purpurhome && cd /tmp/purpurhome && \
#   java -Dpaperclip.install=true -jar /tmp/purpur.jar   # from api.purpurmc.org/v2/purpur/1.21.10/latest/download
if [ ! -d /tmp/purpurhome/libraries ]; then
  echo "missing /tmp/purpurhome/libraries — run the paperclip install-only command above" >&2
  exit 2
fi
LIBS=$(find /tmp/purpurhome/libraries -name "*.jar" | tr '\n' ':')
SERVER_JAR=$(ls /tmp/purpurhome/versions/*/purpur-*.jar 2>/dev/null | head -1)
CP="$KERNEL_JAR:entityinside/build:$LIBS$SERVER_JAR"
$JAVAC --release 21 -proc:none -nowarn -cp "$CP" -d entityinside/build \
  entityinside/harness/InsideBitmaskLockstepHarness.java
$JAVA -Xmx4G -cp "$CP" net.minecraft.world.entity.InsideBitmaskLockstepHarness
