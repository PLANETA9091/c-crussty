#!/usr/bin/env bash
# G-BODY (TASK-71): standalone whole-body dispatch prototype.
# Java arm = pristine JVM (no agent); patched arm = agent attached, baseline
# computed pre-retransform, whole-body swap applied mid-JVM, bit-parity on
# the same deterministic stream, then P500 timing. CPU-only, no server.
#
# BENCH.lock is held by the caller (one run per tool call — reaper lesson F4).
#
# Usage: ./run_body.sh
set -euo pipefail
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
PAPER_PATCHED="/home/z/server/versions/1.21.10/purpur-1.21.10.jar"
FASTUTIL="$(find /home/z/server/libraries -name 'fastutil*.jar' | head -1)"
ASM="$(find /home/z/server/libraries/org/ow2/asm -name 'asm-9*.jar' | head -1)"
[ -f "$MAIN_SO" ] || { echo "missing $MAIN_SO" >&2; exit 1; }
[ -f "$PAPER_PATCHED" ] || { echo "missing $PAPER_PATCHED" >&2; exit 1; }
[ -n "$FASTUTIL" ] || { echo "missing fastutil" >&2; exit 1; }
[ -n "$ASM" ] || { echo "missing asm" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAR=/home/z/jdk21/bin/jar; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAR=jar; JAVA=java; fi

mkdir -p classes_body
"$JAVAC" -nowarn -cp "$ASM:$PAPER_PATCHED:$FASTUTIL" -d classes_body \
  src/bodyagent/BodyAgent.java \
  src/bodyagent/AgentProbe.java \
  src/net/minecraft/world/level/levelgen/synth/BodyDispatch.java \
  ../../p500/java/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  src/net/minecraft/world/level/levelgen/synth/Step0BodyBench.java

cat > classes_body/manifest.mf << 'MF'
Premain-Class: bodyagent.BodyAgent
Can-Retransform-Classes: true
MF
"$JAR" cfm bodyagent.jar classes_body/manifest.mf -C classes_body bodyagent -C classes_body net

CP="classes_body:bodyagent.jar:$ASM:$PAPER_PATCHED:$FASTUTIL"
LIBS_CP="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"
MODE="${1:-both}"
if [ "$MODE" = "java" ] || [ "$MODE" = "both" ]; then
  echo "=== ARM J (pristine JVM, no agent) ==="
  "$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC -cp "$CP:$LIBS_CP" \
    net.minecraft.world.level.levelgen.synth.Step0BodyBench "$MAIN_SO" java
fi
if [ "$MODE" = "patched" ] || [ "$MODE" = "both" ]; then
  echo "=== ARM P (agent attached, mid-JVM whole-body swap) ==="
  "$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC -javaagent:bodyagent.jar -cp "$CP:$LIBS_CP" \
    net.minecraft.world.level.levelgen.synth.Step0BodyBench "$MAIN_SO" patched
fi
