#!/usr/bin/env bash
# Build the FluidPushOps bridge class (S7-151 / TASK-290, fluid-dirty).
#
# PURE JAVA bridge (no JNI natives), same delivery pattern as
# InsideBlockOps (S7-135/S7-148): compiled offline against the runtime jar
# (purpur-1.21.10.jar, Mojang-mapped) and defined into the KERNEL loader
# at activation time (same package net.minecraft.world.entity as Entity).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_fluid_push_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/tmp/toolchain/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1)" >&2; exit 1; fi
fi

SERVER_JAR="${SERVER_JAR:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime jar not found" >&2; exit 1; fi

# joml needed on the compile classpath (AABB/VoxelShape signatures pull org.joml)
JOML_JAR="${JOML_JAR:-$(find /tmp/pdec/matsrv/libraries/org/joml /tmp/s7147mat/server/libraries/org/joml /tmp/s7151mat/server/libraries/org/joml -name 'joml-*.jar' 2>/dev/null | head -1 || true)}"
if [ -z "$JOML_JAR" ]; then echo "joml jar not found" >&2; exit 1; fi
FASTUTIL_JAR="${FASTUTIL_JAR:-$(find /tmp/pdec/matsrv/libraries/it/unimi/dsi/fastutil /tmp/s7147mat/server/libraries/it/unimi/dsi/fastutil /tmp/s7151mat/server/libraries/it/unimi/dsi/fastutil -name 'fastutil-*.jar' 2>/dev/null | head -1 || true)}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 \
  -cp "$SERVER_JAR:$JOML_JAR${FASTUTIL_JAR:+:$FASTUTIL_JAR}" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/FluidPushOps.java

echo "build output:"
find "$OUT_DIR" -name 'FluidPushOps*.class' -exec ls -la {} \;
