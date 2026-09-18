#!/usr/bin/env bash
# build_flush_ops.sh — compile the FLUSH-DIET bridge class (S7-137).
# Usage: build_flush_ops.sh <javac> <kernel.jar> [out_dir]
# Compiles FlushOps.java against the materialized mojang-mapped kernel with
# --release 21 (JVM major guard checked at define time inside flush_diet.rs).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
OUT="${3:-/home/z/c-crussty/entityinside/build}"

mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL" -d "$OUT" \
  /home/z/c-crussty/entityinside/net/minecraft/world/entity/FlushOps.java

echo "built: $OUT/net/minecraft/world/entity/FlushOps.class"
sha256sum "$OUT/net/minecraft/world/entity/FlushOps.class"
