#!/usr/bin/env bash
# Build the MobPushOps bridge class (TASK-401-E soa / TASK-402-B composite;
# TASK-410-C adds the package-private accessors + cmp410_eindexq gate).
#
# Compiled against the FULL patched-kernel jar (round-396-a recon artifact =
# the live runtime). --release 21 pins the class-file major to 65.
#
# Usage: scripts/build_mobpush_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

KERNEL_JAR="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
if [ ! -f "$KERNEL_JAR" ]; then echo "kernel jar not found: $KERNEL_JAR" >&2; exit 1; fi

OUT_DIR=mobpush/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$KERNEL_JAR:entitygoalquery/build" \
  -d "$OUT_DIR" \
  mobpush/net/minecraft/world/entity/MobPushOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/
