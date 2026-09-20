#!/usr/bin/env bash
# build_items_index_ops.sh — compile the ITEMS-INDEX bridge trio (TASK-395
# mega-round, agent-A lever `items_index`): ItemMergeIndexOps + $Cell + $Shard.
# Usage: build_items_index_ops.sh <javac> <kernel.jar> [fastutil.jar] [out_dir]
# Compiles against the materialized mojang-mapped kernel (PURE kernel — the
# ops class reads only public surface: level(), getBoundingBox(), getX/Y/Z,
# isRemoved(), Level.spigotConfig.itemMerge, Level.paperConfig()).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
FASTUTIL="${3:?fastutil jar (kernel libraries)}"
OUT="${4:-/home/z/rounds/ROUND-395/agent-A/itemsindex/build}"

mkdir -p "$OUT/net/minecraft/world/entity/item"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$FASTUTIL" -d "$OUT" \
  /home/z/rounds/ROUND-395/agent-A/itemsindex/net/minecraft/world/entity/item/ItemMergeIndexOps.java

echo "built:"
ls -la "$OUT/net/minecraft/world/entity/item/"
sha256sum "$OUT"/net/minecraft/world/entity/item/*.class
