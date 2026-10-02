#!/usr/bin/env bash
# build_stagger_ops.sh — cmp401_stagger bridge classes (TASK-401-I).
#
# Compiles PushStaggerOps (net.minecraft.world.entity) and GoalStaggerOps
# (net.minecraft.world.entity.ai.goal.target) against the real runtime jar
# (purpur-1.21.10.jar, Mojang-mapped). --release 21 pins class major 65.
#
# Usage: scripts/build_stagger_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
SERVER_JAR="${SERVER_JAR:-/tmp/versions/1.21.10/purpur-1.21.10.jar}"
[ -f "$SERVER_JAR" ] || SERVER_JAR=/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar
[ -f "$SERVER_JAR" ] || { echo "runtime jar not found" >&2; exit 1; }

JOML_JAR="${JOML_JAR:-$(find /tmp/pdec/matsrv/libraries/org/joml /tmp/s7147mat/server/libraries/org/joml /tmp -maxdepth 6 -name 'joml-*.jar' 2>/dev/null | head -1 || true)}"
CP="$SERVER_JAR"
[ -n "$JOML_JAR" ] && CP="$SERVER_JAR:$JOML_JAR"

OUT_DIR=stagger/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -cp "$CP" -d "$OUT_DIR" \
  stagger/net/minecraft/world/entity/PushStaggerOps.java \
  stagger/net/minecraft/world/entity/ai/goal/target/GoalStaggerOps.java

echo "build output:"
find "$OUT_DIR" -name '*.class' -exec ls -la {} \;
