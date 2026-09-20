#!/usr/bin/env bash
# build_item_merge_ops.sh — compile the ITEMS-INDEX bridge class
# (ROUND-396 / TASK-396-A vector A, lever_flag="items_index").
# Usage: build_item_merge_ops.sh <javac> <kernel.jar> [fastutil.jar] [guava.jar] [out_dir]
# Compiles ItemMergeIndexOps.java against a materialized mojang-mapped
# kernel (pure kernel — no patched-classpath stub). Exactly ONE classfile
# is produced (no nested classes; the single lambda = invokedynamic).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
FASTUTIL="${3:-/home/z/tools/fastutil.jar}"
GUAVA="${4:-/home/z/tools/guava.jar}"
OUT="${5:-/home/z/rounds/ROUND-396/agent-a/items/build}"
SRC_DIR="$(cd "$(dirname "$0")/.." && pwd)/items/net/minecraft/world/entity/item"

mkdir -p "$OUT/net/minecraft/world/entity/item"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$FASTUTIL:$GUAVA" -d "$OUT" \
  "$SRC_DIR/ItemMergeIndexOps.java"

echo "built: $OUT/net/minecraft/world/entity/item/ItemMergeIndexOps.class"
sha256sum "$OUT/net/minecraft/world/entity/item/ItemMergeIndexOps.class"
