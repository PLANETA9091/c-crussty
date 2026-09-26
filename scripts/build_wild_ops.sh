#!/usr/bin/env bash
# build_wild_ops.sh — cmp466_c97 census bridge class (round-466 C97 WILD,
# WILD-3D "fiber" vector: per-tick per-worker flat integer section planes).
#
# Compiles WildOps (net.minecraft.world.level.chunk, LevelChunk
# getBlockStateFinal(III) whole-body census bridge) against the real runtime
# kernel jar (patched-kernel.jar, Mojang-mapped, pinned e2992d63).
# --release 21 pins class major 65 (lesson 408: stale class blob = sleeping
# gate — ALWAYS rebuild from merged sources before dispatch).
#
# Usage: scripts/build_wild_ops.sh [javac] [kernel.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL_JAR="${2:-$(find /home/z/tools research -maxdepth 3 -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }
echo "kernel jar: $KERNEL_JAR"

OUT_DIR=wild/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -proc:none -cp "$KERNEL_JAR" -d "$OUT_DIR" \
  wild/net/minecraft/world/level/chunk/WildOps.java

# flat sibling (legacy path of check_blobs_sync flat==nested discipline)
cp "$OUT_DIR/net/minecraft/world/level/chunk/WildOps.class" "$OUT_DIR/WildOps.class"

echo "build output:"
find "$OUT_DIR" -name '*.class' -exec ls -la {} \;
