#!/usr/bin/env bash
# build_sscan_ops.sh — cmp406_sscan bridge classes (TASK-406-E).
#
# Compiles MobPushOps (net.minecraft.world.entity, gate extension + id
# accessors) and MobScanOps (net.minecraft.world.entity, despawn-scan bridge)
# against the real runtime kernel jar (patched-kernel.jar, Mojang-mapped).
# --release 21 pins class major 65 (lesson 408: stale class blob = sleeping
# gate — ALWAYS rebuild from merged sources before dispatch).
#
# Usage: scripts/build_sscan_ops.sh [javac] [kernel.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL_JAR="${2:-$(find research -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }

# Full compile classpath: kernel + fastutil + adventure + paper-api (the
# round-406-E mandate; round-j2b is NOT acceptable as a cp source).
FASTUTIL="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"
ADVENTURE="${ADVENTURE_API_JAR:-/tmp/adventure-api.jar}"
ADVENTURE_KEY="${ADVENTURE_KEY_JAR:-/tmp/adventure-key.jar}"
PAPER_API="${PAPER_API_JAR:-/tmp/paper-api.jar}"

CP="$KERNEL_JAR"
for j in "$FASTUTIL" "$ADVENTURE" "$ADVENTURE_KEY" "$PAPER_API"; do
    [ -f "$j" ] && CP="$CP:$j"
done

OUT_DIR=sscan/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -cp "$CP" -d "$OUT_DIR" \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java

echo "build output:"
find "$OUT_DIR" -name '*.class' -exec ls -la {} \;
