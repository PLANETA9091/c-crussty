#!/usr/bin/env bash
# G-ABI-2 (TASK-108): decode the NormalNoise BATCH FILL family ABI
# (nativeFillPositions / nativeFillScaledPositions / nativeFillShiftA/B)
# via parity-driven candidate sweep against the real 1.21.10 kernel classes.
# TASK-79 G-NORMAL discipline; CPU-only, no server, no .so deploy.
# Caller holds BENCH.lock (one run per tool call).
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

mkdir -p classes_fill logs
"$JAVAC" -nowarn -cp "$PAPER_PATCHED:$FASTUTIL" -d classes_fill \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.java \
  src/net/minecraft/world/level/levelgen/synth/Step0NormalBench.java \
  src/net/minecraft/world/level/levelgen/synth/Step0NormalFillBench.java

SERVER_LIBS="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"

"$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch \
  -Dp500.libs="$MAIN_SO" \
  -cp "classes_fill:$PAPER_PATCHED:$FASTUTIL:$SERVER_LIBS" \
  net.minecraft.world.level.levelgen.synth.Step0NormalFillBench "$MAIN_SO" \
  2>&1 | tee logs/fill_abi_$$.log
