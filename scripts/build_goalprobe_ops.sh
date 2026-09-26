#!/usr/bin/env bash
# build_goalprobe_ops.sh — ROUND-468 S88: build the GoalProbeOps bridge
# (goal-selector branch-predicate probe, lever cmp468_s88probe).
#
# Bridge defined into the KERNEL loader at activation time (same package
# net.minecraft.world.entity.ai.goal as GoalSelector/WrappedGoal).
# Compiled against the FULL patched-kernel jar (live runtime).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_goalprobe_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
[ -x "$JAVAC" ] || JAVAC="$(command -v javac)"
[ -n "$JAVAC" ] || { echo "no javac found" >&2; exit 1; }
KERNEL="${KERNEL_JAR:-/home/z/tools/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find /home/z -maxdepth 4 -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

OUT_DIR=goalprobe/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$KERNEL" \
  -d "$OUT_DIR" \
  goalprobe/net/minecraft/world/entity/ai/goal/GoalProbeOps.java

nested="$OUT_DIR/net/minecraft/world/entity/ai/goal/GoalProbeOps.class"
flat="$OUT_DIR/GoalProbeOps.class"
# ×93 discipline: BOTH paths installed — nested (include_bytes! contract) + flat
cp "$nested" "$flat"
echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $flat"
echo "== goalprobe blob build OK (major 65) =="
