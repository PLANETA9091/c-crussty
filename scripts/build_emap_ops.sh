#!/usr/bin/env bash
# Build the EntityMap fence bridge classes (TASK-411-A k5b, cmp405_navplane).
#
# THREE flat top-level classes (ZERO nested — S7-163 discipline):
#   EntityMapOps        — fence helpers (containsKey/put/remove/get/values)
#   EntityMapSafeValues — values() view (iterator creation under map monitor)
#   EntityMapSafeItr    — bound-checked fail-dominant iterator
#
# Delivery pattern of NavPoolOps (TASK-410-A): compiled offline against the
# runtime kernel jar (Mojang-mapped) + fastutil, defined into the KERNEL
# loader at region_threads activation time BEFORE the ChunkMap retransform.
# Class-file major pinned to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_emap_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"

SERVER_JAR="${SERVER_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | tail -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

FASTUTIL_JAR="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/server/level"

# Remove any stale output first (delivery-set discipline).
rm -f "$OUT_DIR/net/minecraft/server/level/EntityMapOps"*.class \
      "$OUT_DIR/net/minecraft/server/level/EntityMapSafeValues"*.class \
      "$OUT_DIR/net/minecraft/server/level/EntityMapSafeItr"*.class

if ! $JAVAC --release 21 \
  -cp "$SERVER_JAR:$FASTUTIL_JAR" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/server/level/EntityMapOps.java \
  entityinside/net/minecraft/server/level/EntityMapSafeValues.java \
  entityinside/net/minecraft/server/level/EntityMapSafeItr.java 2>/dev/null; then
  $JAVAC \
    -cp "$SERVER_JAR:$FASTUTIL_JAR" \
    -d "$OUT_DIR" \
    entityinside/net/minecraft/server/level/EntityMapOps.java \
    entityinside/net/minecraft/server/level/EntityMapSafeValues.java \
    entityinside/net/minecraft/server/level/EntityMapSafeItr.java
fi

# Delivery-set guard: EXACTLY three classfiles must be produced (no nested).
PRODUCED=$(ls "$OUT_DIR/net/minecraft/server/level/EntityMap"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "3" ]; then
  echo "EMAP delivery guard FAILED: produced $PRODUCED classfiles (expected 3)" >&2
  ls -la "$OUT_DIR/net/minecraft/server/level/" >&2
  exit 2
fi
echo "emap build OK:"
ls "$OUT_DIR/net/minecraft/server/level/"EntityMap*.class
