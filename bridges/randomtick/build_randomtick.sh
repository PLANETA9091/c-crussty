#!/bin/bash
# Build F1 BATCH-RNG helper (RandomTickOps) — randomtick/ module member.
#
# Requirements: a JRE (java on PATH) + ECJ jar (auto-downloaded on first run)
# + the materialized kernel jar (mojang-mapped Purpur 1.21.10) as the
# compile classpath. Produces randomtick/build/net/minecraft/server/level/
# RandomTickOps.class for include_bytes! by the byte hook (next tick).
#
# NOTE: no server boot here — javac-equivalent offline compile (INJECTS-ONLY).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
ECJ="$HERE/ecj.jar"

if [ ! -f "$KERNEL_JAR" ]; then
  echo "KERNEL_JAR not found: $KERNEL_JAR (set KERNEL_JAR=<materialized purpur jar>)" >&2
  exit 1
fi
if [ ! -f "$ECJ" ]; then
  echo "downloading ECJ (offline compiler, runs on plain JRE)" >&2
  curl -sL -o "$ECJ" "https://repo1.maven.org/maven2/org/eclipse/jdt/ecj/3.36.0/ecj-3.36.0.jar"
fi

mkdir -p "$HERE/build"
java -jar "$ECJ" -source 21 -target 21 -cp "$KERNEL_JAR" -d "$HERE/build" \
  "$HERE/src/RandomTickOps.java"
echo "built: $(find "$HERE/build" -name '*.class')"
