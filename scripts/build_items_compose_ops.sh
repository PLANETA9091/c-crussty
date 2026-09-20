#!/usr/bin/env bash
# build_items_compose_ops.sh — compile the ITEMS-COMPOSE bridge class
# (MEGA-ROUND-2 / TASK-397-A, lever_flag="items_compose_ai").
# Usage: build_items_compose_ops.sh <javac> <kernel.jar> [fastutil.jar] [guava.jar] [out_dir]
# Compiles ItemsComposeOps.java against a materialized mojang-mapped kernel.
# Exactly ONE classfile is produced (no nested classes; the single lambda =
# invokedynamic).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
FASTUTIL="${3:-/home/z/tools/fastutil.jar}"
GUAVA="${4:-/home/z/tools/guava.jar}"
OUT="${5:-$(cd "$(dirname "$0")/.." && pwd)/items_compose/build}"
SRC_DIR="$(cd "$(dirname "$0")/.." && pwd)/items_compose/net/minecraft/world/entity/item"

mkdir -p "$OUT/net/minecraft/world/entity/item"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$FASTUTIL:$GUAVA" -d "$OUT" \
  "$SRC_DIR/ItemsComposeOps.java"

echo "built: $OUT/net/minecraft/world/entity/item/ItemsComposeOps.class"
sha256sum "$OUT/net/minecraft/world/entity/item/ItemsComposeOps.class"
