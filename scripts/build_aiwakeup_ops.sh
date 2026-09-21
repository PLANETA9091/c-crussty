#!/usr/bin/env bash
# build_aiwakeup_ops.sh — compile the EVENT-DRIVEN AI WAKEUP-LIST bridge class
# (TASK-399-I, lever cmp399_wakeup; AiWakeupOps receivers of the 4 retargeted
# Mob.serverAiStep goal-tick call sites).
# Usage: build_aiwakeup_ops.sh <javac> <kernel.jar> [out_dir]
# Compiled against the PURE materialized kernel (no patched-classpath stubs):
# AiWakeupOps touches only public/package vanilla shapes (Mob, GoalSelector,
# WrappedGoal, Goal$Flag), so the bridge can be defined into the Mob loader
# before the retransform (src/wakeup.rs wakeup protocol).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
OUT="${3:-/home/z/rounds/ROUND-399/agent-i/entityinside/build}"

mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL" -d "$OUT" \
  /home/z/rounds/ROUND-399/agent-i/entityinside/net/minecraft/world/entity/AiWakeupOps.java

echo "built: $OUT/net/minecraft/world/entity/AiWakeupOps.class"
sha256sum "$OUT/net/minecraft/world/entity/AiWakeupOps.class"
