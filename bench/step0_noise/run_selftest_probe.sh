#!/usr/bin/env bash
# TASK-108 smoke forensics: run Step0SelfTestProbe — replicate the SERVER
# selfTest failing case (seed 1234L, n=1 (1000,-64,2000), sxz=2 sy=3) outside
# the server against the SAME closed-lib natives. Isolates fill-kernel
# divergence vs server-side bridge bug. CPU-only, no deploy.
# Caller holds BENCH.lock (one run per tool call).
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

mkdir -p classes_fill logs
"$JAVAC" -nowarn -cp "$PAPER_PATCHED:$FASTUTIL" -d classes_fill \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  ../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.java \
  src/net/minecraft/world/level/levelgen/synth/Step0SelfTestProbe.java

SERVER_LIBS="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"

"$JAVA" -Xms1g -Xmx1g \
  -cp "classes_fill:$PAPER_PATCHED:$FASTUTIL:$SERVER_LIBS" \
  net.minecraft.world.level.levelgen.synth.Step0SelfTestProbe "$MAIN_SO" \
  2>&1 | tee logs/selftest_probe_$$.log
