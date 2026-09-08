#!/usr/bin/env bash
# G-RECON (TASK-69): owner-cost measurement — REAL PerlinNoise.getValue /
# NormalNoise.getValue / production-corner octave sample from the deployed
# patched jar. Pure Java, NO natives, no .so load, no server.
# Usage: ./run_recon.sh
set -euo pipefail
cd "$(dirname "$0")"

PAPER_PATCHED="/home/z/server/versions/1.21.10/purpur-1.21.10.jar"
FASTUTIL="/home/z/server/libraries/it/unimi/dsi/fastutil/8.5.15/fastutil-8.5.15.jar"
[ -f "$PAPER_PATCHED" ] || { echo "missing $PAPER_PATCHED" >&2; exit 1; }
[ -f "$FASTUTIL" ] || { echo "missing $FASTUTIL" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

mkdir -p classes_recon
"$JAVAC" -nowarn -cp "$PAPER_PATCHED:$FASTUTIL" -d classes_recon \
  src/net/minecraft/world/level/levelgen/synth/Step0ReconBench.java

LIBS_CP="$(find /home/z/server/libraries -name '*.jar' | tr '\n' ':')"
exec "$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC \
  -cp "classes_recon:$PAPER_PATCHED:$LIBS_CP" \
  net.minecraft.world.level.levelgen.synth.Step0ReconBench
