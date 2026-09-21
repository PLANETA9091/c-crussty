#!/usr/bin/env bash
# Build the ItemLifetimeOps bridge class (TASK-397-H / items_despawn_heap).
#
# PURE JAVA bridge (no JNI natives), same delivery pattern as RegionTickOps
# (S7-156): compiled offline against the kernel jar and defined into the
# KERNEL loader at activation time (same package as ItemEntity).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_item_lifetime_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  elif [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/tmp/toolchain/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1)" >&2; exit 1; fi
fi

KERNEL_JAR="${KERNEL_JAR:-research/gc-recon-2026-09-19/run-s7204-bitmask/patched-kernel.jar}"
if [ ! -f "$KERNEL_JAR" ]; then echo "kernel jar not found: $KERNEL_JAR" >&2; exit 1; fi
FASTUTIL_JAR="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"

OUT_DIR=items_despawn_heap/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 \
  -cp "$KERNEL_JAR:$FASTUTIL_JAR" \
  -d "$OUT_DIR" \
  items_despawn_heap/src/net/minecraft/world/entity/item/ItemLifetimeOps.java

echo "built: $OUT_DIR/net/minecraft/world/entity/item/ItemLifetimeOps.class"
sha256sum "$OUT_DIR/net/minecraft/world/entity/item/ItemLifetimeOps.class"
