#!/usr/bin/env bash
# build_fluid_ops.sh — compile the FLUID-FREE-SECTION bridge class (S7-143
# re-implementation of S7-139).
# Usage: build_fluid_ops.sh <javac> <kernel.jar> [fastutil.jar] [out_dir]
# Compiles FluidOps.java against the materialized mojang-mapped kernel
# (PURE kernel — the injected fields are resolved via Unsafe at runtime, so
# no patched-classpath stub is needed and the cargo<->javac cycle stays cut).
set -euo pipefail
JAVAC="${1:?javac path}"
KERNEL="${2:?materialized kernel jar}"
FASTUTIL="${3:?fastutil jar (kernel libraries)}"
OUT="${4:-/home/z/c-crussty/entityinside/build}"

mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$FASTUTIL" -d "$OUT" \
  /home/z/c-crussty/entityinside/net/minecraft/world/entity/FluidOps.java

echo "built: $OUT/net/minecraft/world/entity/FluidOps.class"
sha256sum "$OUT/net/minecraft/world/entity/FluidOps.class"
