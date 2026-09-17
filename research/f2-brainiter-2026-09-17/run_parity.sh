#!/bin/bash
# F2 BRAIN-ITERATORS parity bank runner (TASK-249 / S7-113).
#
# Prereqs (all cached locally by prior runs):
#   - run21 patched-kernel.jar (bench artifact, INJECTS-ONLY absorb)
#   - randomtick/build (BrainOps classes; built by randomtick/build_randomtick.sh
#     pattern — see run_parity.sh env KERNEL_JAR)
#   - mojang-libs/ (44 jars extracted from the official 1.21.10 server bundler:
#     vanilla-1.21.10.jar via piston-meta, sha 95495a7f485eedd84ce928cef5e223b757d2f764)
#   - clip/ libraries (purpur paperclip set, extracted previously)
#   - slf4j (f1_batch_rng workspace)
#
# Sandbox safety: Bootstrap.bootStrap() + SharedConstants.tryDetectVersion()
# initialize static data ONLY (no Main, no worlds, no tick loop) — INJECTS-ONLY
# compliant (non-boot class of init). ServerLevel instance is created via
# Unsafe.allocateInstance (no constructor), levelData = WritableLevelData proxy.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
F=/home/z/my-project/scripts/f1_batch_rng
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
ML=$(ls "$F"/mojang-libs/*.jar | tr '\n' ':')
CP="$KERNEL_JAR:/home/z/c-crussty/randomtick/build:$HERE/pbuild:$ML$F/slf4j-api.jar:$F/slf4j-jdk.jar:$(find $F/clip -name '*.jar' | tr '\n' ':')"

# 1. compile the helper (idempotent, ECJ offline compiler)
java -jar /home/z/c-crussty/randomtick/ecj.jar -source 21 -target 21 -nowarn \
  -cp "$KERNEL_JAR" -d /home/z/c-crussty/randomtick/build \
  /home/z/c-crussty/randomtick/src/BrainOps.java

# 2. compile + run the bank
java -jar /home/z/c-crussty/randomtick/ecj.jar -source 21 -target 21 -nowarn \
  -cp "$CP" -d "$HERE/pbuild" "$HERE/ParityTest.java"
java -cp "$CP" ParityTest
