#!/usr/bin/env bash
# Build the InsideBlockOps bridge class (S7-135 / TASK-271, inside-cache).
#
# PURE JAVA bridge (no JNI natives), same delivery pattern as
# EntityQueryOps (S7-133): compiled offline against the real runtime jar
# (purpur-1.21.10.jar, Mojang-mapped) and defined into the KERNEL loader
# at activation time (same package net.minecraft.world.entity as Entity).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/inside_cache.rs refuses to arm if the embedded major exceeds the
# live JVM's (same guard as fluid_guard / alloc_diet).
#
# Usage: scripts/build_inside_block_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

SERVER_JAR="${SERVER_JAR:-/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then echo "runtime jar not found: $SERVER_JAR" >&2; exit 1; fi

# joml needed on the compile classpath (AABB/VoxelShape signatures pull org.joml)
JOML_JAR="$(find /tmp/pdec/matsrv/libraries/org/joml -name 'joml-*.jar' 2>/dev/null | head -1 || true)"

OUT_DIR=entityinside/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

# EntityQueryOps classes are referenced by InsideBlockOps (mutablePos ring)
EQ_DIR=entityquery/build
if [ ! -f "$EQ_DIR/net/minecraft/world/entity/EntityQueryOps.class" ]; then
  echo "EntityQueryOps classes missing — build entityquery/ first (scripts/build_entity_query_ops.sh)" >&2
  exit 1
fi

"$JAVAC" --release 21 \
  -cp "$SERVER_JAR:$EQ_DIR:$JOML_JAR" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/InsideBlockOps.java

echo "build output:"
find "$OUT_DIR" -name '*.class' -exec ls -la {} \;
