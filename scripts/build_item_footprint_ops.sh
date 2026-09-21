#!/usr/bin/env bash
# build_item_footprint_ops.sh — compile the ITEM-FOOTPRINT bridge class
# (round-397-g-footprint).
# Usage: build_item_footprint_ops.sh <javac> <kernel.jar> [out_dir]
# Compiles ItemFootprintOps.java against the materialized mojang-mapped
# kernel (bridge needs the ItemEntity type in signatures). NOTE: NOT
# --release (the bridge pokes sun.misc.Unsafe via jdk.unsupported, which the
# ct.sym snapshot of --release hides); the class-major guard is checked at
# define time inside item_footprint.rs.
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
OUT="${3:-/home/z/c-crussty/entityinside/build}"

mkdir -p "$OUT/net/minecraft/world/entity/item"
"$JAVAC" -source 21 -target 21 -proc:none -cp "$KERNEL" -d "$OUT" \
  "$(dirname "$0")/../entityinside/net/minecraft/world/entity/item/ItemFootprintOps.java"

echo "built: $OUT/net/minecraft/world/entity/item/ItemFootprintOps.class"
sha256sum "$OUT/net/minecraft/world/entity/item/ItemFootprintOps.class"
