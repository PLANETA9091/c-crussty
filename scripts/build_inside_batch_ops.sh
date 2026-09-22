#!/usr/bin/env bash
# build_inside_batch_ops.sh — compile the INSIDE-BATCH bridge class (TASK-411-B R3).
#
# Usage: build_inside_batch_ops.sh [javac]
# Compiles InsideRustOps.java against the materialized Mojang-mapped kernel
# (research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar) + fastutil
# with --release 21 (JVM major guard checked at define time inside
# inside_batch.rs). Run after EVERY .java edit (blob write-through).
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  else echo "no javac found (pass one as arg 1)" >&2; exit 1; fi
fi

KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
if [ ! -f "$KERNEL" ]; then
  KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
fi
if [ ! -f "$KERNEL" ]; then echo "kernel jar not found" >&2; exit 1; fi
FASTUTIL="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/world/entity"

"$JAVAC" --release 21 -proc:none \
  -cp "$KERNEL:$FASTUTIL" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/InsideRustOps.java

echo "built: $OUT_DIR/net/minecraft/world/entity/InsideRustOps.class"
sha256sum "$OUT_DIR/net/minecraft/world/entity/InsideRustOps.class"
