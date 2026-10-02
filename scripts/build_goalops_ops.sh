#!/usr/bin/env bash
# build_goalops_ops.sh — TASK-421-A brain-slice: build the GoalOps bridge
# (goal-selector flat priority fast-path, lever cmp421_brain).
#
# Bridge defined into the KERNEL loader at activation time (same package
# net.minecraft.world.entity.ai.goal as GoalSelector — kernel-internal types
# are public; GoalSelector private fields are read via cached MethodHandle
# getters, see GoalOps.reflectSetup). Compiled against the FULL
# patched-kernel jar (round-396-a recon artifact = the live runtime).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_goalops_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
[ -x "$JAVAC" ] || JAVAC="$(command -v javac)"
[ -n "$JAVAC" ] || { echo "no javac found" >&2; exit 1; }
KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

OUT_DIR=goalops/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$KERNEL" \
  -d "$OUT_DIR" \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java

nested="$OUT_DIR/net/minecraft/world/entity/ai/goal/GoalOps.class"
flat="$OUT_DIR/GoalOps.class"
# ×93 discipline: BOTH paths installed — nested (include_bytes! contract) + flat
cp "$nested" "$flat"
echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $flat"
echo "== goalops blob build OK (major 65) =="
