#!/usr/bin/env bash
# G-NORMAL (TASK-79, Session-0): decode the heritage whole-Noise composition
# kernel PaperNativeNormalNoise.nativeGetValue(JJDDDD)D of the closed lib via
# parity-driven candidate sweep + full parity + three-arm P500 timing
# (J pristine / N2 armed-today 2-crossing / N1 one-crossing target).
# CPU-only, no server, no .so deploy.
#
# BENCH.lock is held by the caller (one run per tool call — reaper lesson F4).
#
# Usage: ./run_normal_abi.sh
set -euo pipefail
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
PAPER_PATCHED="/home/z/server/versions/1.21.10/purpur-1.21.10.jar"
FASTUTIL="$(find /home/z/server/libraries -name 'fastutil*.jar' | head -1)"
[ -f "$MAIN_SO" ] || { echo "missing $MAIN_SO" >&2; exit 1; }
[ -f "$PAPER_PATCHED" ] || { echo "missing $PAPER_PATCHED" >&2; exit 1; }
[ -n "$FASTUTIL" ] || { echo "missing fastutil jar" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

mkdir -p classes_normal
"$JAVAC" -nowarn -cp "$PAPER_PATCHED:$FASTUTIL" -d classes_normal \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  src/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.java \
  src/net/minecraft/world/level/levelgen/synth/Step0NormalBench.java

LIBS_CP="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"
exec "$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC \
  -cp "classes_normal:$PAPER_PATCHED:$LIBS_CP" \
  net.minecraft.world.level.levelgen.synth.Step0NormalBench "$MAIN_SO"
