#!/usr/bin/env bash
# TASK-108 v3 phase-1 probe (PROGRESS-6, agent-7625532f): run V3ArrayProbe —
# bit-exactness of DensityArrayInterpreter.eval vs the vanilla per-point
# oracle on REAL kernel noise instances, standalone, closed-lib natives.
# CPU-only, no deploy, no boot. Caller holds BENCH.lock (one run per call).
set -euo pipefail
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
PAPER_PATCHED="/home/z/server/versions/1.21.10/purpur-1.21.10.jar"
FASTUTIL="$(find /home/z/server/libraries -name 'fastutil*.jar' | head -1)"
[ -f "$MAIN_SO" ] || { echo "missing $MAIN_SO" >&2; exit 1; }
[ -n "$FASTUTIL" ] || { echo "missing fastutil jar" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

mkdir -p classes_v3 logs
"$JAVAC" -nowarn -cp "$PAPER_PATCHED:$FASTUTIL" -d classes_v3 \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.java \
  "$REPO_ROOT/noise/net/minecraft/world/level/levelgen/NormalNoiseBatchOps.java" \
  "$REPO_ROOT/noise/net/minecraft/world/level/levelgen/DensityArrayInterpreter.java" \
  src/net/minecraft/world/level/levelgen/V3ArrayProbe.java

SERVER_LIBS="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"

"$JAVA" -Xms1g -Xmx1g \
  -cp "classes_v3:$PAPER_PATCHED:$FASTUTIL:$SERVER_LIBS" \
  net.minecraft.world.level.levelgen.V3ArrayProbe "$MAIN_SO" \
  2>&1 | tee logs/v3_array_probe_$$.log
