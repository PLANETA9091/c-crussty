#!/usr/bin/env bash
# build_activation_range_ops.sh — compile the ACTRANGE bridge class
# (TASK-400-H, vector H — activation-range / DAB port, lever cmp399_dab).
# Usage: build_activation_range_ops.sh <javac> <kernel.jar> [fastutil.jar] [out_dir]
# Compiles ActivationRangeOps.java against the materialized mojang-mapped
# kernel (PURE kernel surface: Entity.activationType / defaultActivationState,
# ActivationType enum, TickRateManager — all public in purpur-1.21.10; no
# patched-classpath stub needed, the cargo<->javac cycle stays cut).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
FASTUTIL="${3:?fastutil jar (kernel libraries)}"
OUT="${4:-/home/z/rounds/ROUND-400/agent-h/actrange/build}"
SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/actrange"

mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$FASTUTIL" -d "$OUT" \
  "$SRC_DIR/net/minecraft/world/entity/ActivationRangeOps.java"

echo "built: $OUT/net/minecraft/world/entity/ActivationRangeOps.class"
sha256sum "$OUT/net/minecraft/world/entity/ActivationRangeOps.class"
