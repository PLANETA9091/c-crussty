#!/usr/bin/env bash
# Build the EntityGoalQueryOps bridge class (TASK-410-C, K3 pivot eindexq).
#
# Bridge defined into the KERNEL loader at activation time (same package
# net.minecraft.world.entity as Entity — kernel-internal types are public;
# MobPushOps package-private accessors byIdArr/idCount/idCapacity/planeReady
# require the SAME package + SAME loader). Compiled against the FULL
# patched-kernel jar (round-396-a recon artifact = the live runtime).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/entity_query.rs refuses to arm if the embedded major exceeds the live
# JVM's (same guard as mobpush / sscan bridges).
#
# Usage: scripts/build_entitygoalquery_ops.sh [javac]
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

OUT_DIR=entitygoalquery/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$KERNEL_JAR:mobpush/build" \
  -d "$OUT_DIR" \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/
