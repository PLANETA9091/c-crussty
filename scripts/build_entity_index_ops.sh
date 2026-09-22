#!/usr/bin/env bash
# Build the EntityIndexOps bridge class (TASK-405-C, vector eindex).
#
# Same delivery pattern as EntityQueryOps/MobPushOps: compiled offline
# against the runtime kernel jar (Mojang-mapped purpur + moonrise, plus
# fastutil), defined into the KERNEL loader at activation time by
# src/entity_index_manager.rs (same package net.minecraft.world.entity).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# the manager refuses to arm if the embedded major exceeds the live JVM's.
#
# Usage: scripts/build_entity_index_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

# The sparse worktree has no research/ tree — use a FULL moonrise kernel
# (round-j2b has 0 moonrise entries and CANNOT compile this bridge; verified:
# round406dleg3 jar has 289). Override with KERNEL_JAR=... if needed.
KERNEL_JAR="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-round406dleg3/patched-kernel.jar}"
FASTUTIL_JAR="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"
ADVENTURE_API_JAR="${ADVENTURE_API_JAR:-/tmp/adventure-api.jar}"
ADVENTURE_KEY_JAR="${ADVENTURE_KEY_JAR:-/tmp/adventure-key.jar}"
PAPER_API_JAR="${PAPER_API_JAR:-/tmp/paper-api.jar}"
CP="$KERNEL_JAR:$FASTUTIL_JAR:$ADVENTURE_API_JAR:$ADVENTURE_KEY_JAR:$PAPER_API_JAR"
if [ ! -f "$KERNEL_JAR" ]; then echo "kernel jar not found: $KERNEL_JAR" >&2; exit 1; fi

OUT_DIR=entityquery/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$CP" \
  -d "$OUT_DIR" \
  entityquery/net/minecraft/world/entity/EntityIndexOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/
