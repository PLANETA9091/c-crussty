#!/bin/bash
# F3 LEVELTICKS-READS parity bank runner (TASK-251 / S7-115).
# Mirrors research/f2-brainiter-2026-09-17/run_parity.sh (proven harness):
# kernel-first classpath, ECJ offline compile, INJECTS-ONLY static init.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
F=/home/z/my-project/scripts/f1_batch_rng
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
ML=$(ls "$F"/mojang-libs/*.jar | tr '\n' ':')
CP="$KERNEL_JAR:/home/z/c-crussty/randomtick/build:$HERE/pbuild:$ML$F/slf4j-api.jar:$F/slf4j-jdk.jar:$(find $F/clip -name '*.jar' | tr '\n' ':')"

# 1. compile the helper (kernel classpath + fastutil — same machine as production)
java -jar /home/z/c-crussty/randomtick/ecj.jar -source 21 -target 21 -nowarn \
  -cp "$KERNEL_JAR:$F/mojang-libs/fastutil-8.5.15.jar" -d /home/z/c-crussty/randomtick/build \
  /home/z/c-crussty/randomtick/src/TickBlockOps.java

# 2. compile + run the bank (same-package test: net.minecraft.server.level)
java -jar /home/z/c-crussty/randomtick/ecj.jar -source 21 -target 21 -nowarn \
  -cp "$CP" -d "$HERE/pbuild" "$HERE/ParityTest.java"
java -cp "$CP" net.minecraft.server.level.ParityTest
