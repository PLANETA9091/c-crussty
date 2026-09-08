#!/usr/bin/env bash
# STEP-0 (TASK-67): G-STEP0 gate of docs/WORLDGEN_BATCHING_LAYER_DESIGN.md.
# Per-sample cost: REAL JIT'd Java ImprovedNoise (deployed paperclip-patched
# jar, mojang-mapped — the exact production code) vs the closed lib native
# noise core (nativeNoise / nativeNoiseNoYScale = the live-verified wiring
# core) vs the ALREADY-SHIPPED batch fill kernels (nativeFill /
# nativeFillNoYScale, N samples per ONE crossing).
#
# CPU-only session: no server boot, no .so deploy, no world I/O. BENCH.lock
# taken inline by the caller (one run per tool call — reaper lesson F4).
#
# Usage: ./run_step0.sh
set -euo pipefail
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
PAPER_PATCHED="/home/z/server/versions/1.21.10/purpur-1.21.10.jar"
JOML="/home/z/server/libraries/org/joml/joml/1.10.8/joml-1.10.8.jar"
[ -f "$MAIN_SO" ] || { echo "missing $MAIN_SO" >&2; exit 1; }
[ -f "$PAPER_PATCHED" ] || { echo "missing $PAPER_PATCHED" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

mkdir -p classes
"$JAVAC" -nowarn -cp "$PAPER_PATCHED" -d classes \
  src/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.java \
  src/net/minecraft/world/level/levelgen/synth/Step0NoiseBench.java

# -cp: our classes FIRST (the stub), then the patched server jar (real
# ImprovedNoise + RandomSource), then every bundled library jar (RandomSource's
# init path touches joml, guava, etc.). The stub shadows nothing the driver
# uses: PaperNativeImprovedNoise exists ONLY in the closed .so/JNI naming,
# the server jar has no such class.
LIBS_CP="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"
exec "$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC \
  -cp "classes:$PAPER_PATCHED:$LIBS_CP" \
  net.minecraft.world.level.levelgen.synth.Step0NoiseBench "$MAIN_SO"
