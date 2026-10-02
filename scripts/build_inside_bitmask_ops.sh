#!/usr/bin/env bash
# build_inside_bitmask_ops.sh — compile the INSIDE-BITMASK bridge class (TASK-357).
# Usage: build_inside_bitmask_ops.sh <javac> <kernel.jar> [out_dir]
# Compiles InsideBitmaskOps.java against the materialized mojang-mapped kernel
# with --release 21 (JVM major guard checked at define time inside inside_bitmask.rs).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
OUT="${3:-/home/z/c-crussty/entityinside/build}"

mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL" -d "$OUT" \
  /home/z/c-crussty/entityinside/net/minecraft/world/entity/InsideBitmaskOps.java

echo "built: $OUT/net/minecraft/world/entity/InsideBitmaskOps.class"
sha256sum "$OUT/net/minecraft/world/entity/InsideBitmaskOps.class"
