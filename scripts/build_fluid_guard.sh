#!/usr/bin/env bash
# Build the fluid-push guard hook class (TASK-80).
#
# Unlike the noise trio, this bridge is PURE JAVA (no JNI natives): the guard
# + slow-path reimplementation live entirely in FluidPushGuardHook, so it
# compiles against the real runtime jar (purpur-1.21.10.jar, Mojang-mapped) +
# guava (MapMaker) + fastutil (Object2DoubleMap via fluidHeight field type).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/fluid_guard.rs refuses to arm if the embedded major exceeds the live
# JVM's (same guard as improved_noise/perlin_noise).
#
# Usage: scripts/build_fluid_guard.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

SERVER_JAR=/home/z/server/versions/1.21.10/purpur-1.21.10.jar
if [ ! -f "$SERVER_JAR" ]; then echo "runtime jar not found: $SERVER_JAR" >&2; exit 1; fi

GUAVA=$(ls /home/z/server/libraries/com/google/guava/guava/*/guava-*.jar 2>/dev/null | head -1)
FASTUTIL=$(ls /home/z/server/libraries/it/unimi/dsi/fastutil/*/fastutil-*.jar 2>/dev/null | head -1)
[ -n "$GUAVA" ] || { echo "guava jar not found" >&2; exit 1; }
[ -n "$FASTUTIL" ] || { echo "fastutil jar not found" >&2; exit 1; }

OUT_DIR=fluid/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$SERVER_JAR:$GUAVA:$FASTUTIL" \
  -d "$OUT_DIR" \
  fluid/net/minecraft/world/entity/FluidPushGuardHook.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/
