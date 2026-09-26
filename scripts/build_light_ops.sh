#!/usr/bin/env bash
# build_light_ops.sh — cmp466_light bridge class (round-466 C21 light plane).
#
# Compiles LightOps (net.minecraft.world.entity, Monster.updateNoActionTime
# whole-body census bridge) against the real runtime kernel jar
# (patched-kernel.jar, Mojang-mapped). --release 21 pins class major 65
# (lesson 408: stale class blob = sleeping gate — ALWAYS rebuild from merged
# sources before dispatch).
#
# Usage: scripts/build_light_ops.sh [javac] [kernel.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL_JAR="${2:-$(find research /home/z/tools -maxdepth 3 -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }
echo "kernel jar: $KERNEL_JAR"

OUT_DIR=light/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -proc:none -cp "$KERNEL_JAR" -d "$OUT_DIR" \
  light/net/minecraft/world/entity/LightOps.java

# flat sibling (legacy path of check_blobs_sync flat==nested discipline)
cp "$OUT_DIR/net/minecraft/world/entity/LightOps.class" "$OUT_DIR/LightOps.class"

echo "build output:"
find "$OUT_DIR" -name '*.class' -exec ls -la {} \;
