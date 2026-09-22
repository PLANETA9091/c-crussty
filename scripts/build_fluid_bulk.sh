#!/usr/bin/env bash
# Build the FluidBulkOps bridge class (TASK-416-C fluid-bulk composite).
#
# Bridge defined into the KERNEL loader at activation time (same package
# net.minecraft.world.entity as Entity — protected fluidHeight/lastLavaContact
# access). Compiled against the FULL patched-kernel jar (round-396-a recon
# artifact = the live runtime) + fastutil (Object2DoubleMap fluidHeight field
# type) + paper-api/adventure per the round-416 mandate classpath — never the
# round-j2b jar (lesson ×93 family).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/fluid_bulk.rs refuses to arm if the embedded major exceeds the live
# JVM's (same guard as fluid_guard / mobpush / sscan bridges).
#
# Usage: scripts/build_fluid_bulk.sh [javac]
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
FASTUTIL=/home/z/tools/fastutil.jar
PAPER_API=/home/z/tools/paper-api-1.21.10.jar
ADV_API=/home/z/tools/adventure-api-4.24.0.jar
ADV_KEY=/home/z/tools/adventure-key-4.24.0.jar

OUT_DIR=fluid/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$KERNEL_JAR:$FASTUTIL:$PAPER_API:$ADV_API:$ADV_KEY" \
  -d "$OUT_DIR" \
  fluid/net/minecraft/world/entity/FluidBulkOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/ | grep FluidBulkOps
