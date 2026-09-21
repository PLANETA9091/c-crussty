#!/usr/bin/env bash
# Build the items_oss bridge class (TASK-396-H, Lithium item_entity_merging port).
#
# Pure-Java bridge (no JNI natives) compiled against the real runtime kernel
# jar (purpur-1.21.10, Mojang-mapped, Moonrise entity lookup included).
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/item_merge.rs refuses to arm if the embedded major exceeds the live
# JVM's (same guard as fluid_guard).
#
# Usage: scripts/build_items_oss.sh [javac] [kernel-jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

KERNEL_JAR="${2:-${SERVER_JAR:-/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar}}"
if [ ! -f "$KERNEL_JAR" ]; then
  # fallback: any captured patched-kernel.jar from the recon runs
  KERNEL_JAR=$(ls /home/z/c-crussty/research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)
fi
[ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }

OUT_DIR=items/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn -proc:none \
  -classpath "$KERNEL_JAR" \
  -d "$OUT_DIR" \
  items/net/minecraft/world/entity/item/ItemMergeOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/item/
