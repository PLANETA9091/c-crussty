#!/usr/bin/env bash
# Build the SCHED-DEFER bridge set (C30, ROUND-475 — rt4/bc1
# thread-confinement SITE-A): BlockScheduleOps (NEW, zero nested) +
# RegionTickOps (+ phase-4c drainScheduledTicks).
#
# Compiled offline against the runtime kernel jar (Mojang-mapped) and defined
# into the KERNEL loader at region_threads activation time. Class-file major
# pinned to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_blocksched_ops.sh [javac|ecj-wrapper]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/tmp/toolchain/jdk-21.0.12.1+1/bin/javac
  elif [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1)" >&2; exit 1; fi
fi

SERVER_JAR="${SERVER_JAR:-/home/z/tools/patched-kernel.jar}"
JOML_JAR="${JOML_JAR:-/home/z/tools/joml-1.10.7.jar}"
FASTUTIL_JAR="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/world/level" "$OUT_DIR/net/minecraft/world/entity"

# Fresh delivery set: RegionTickOps + its two nested classes + BlockScheduleOps.
rm -f "$OUT_DIR/net/minecraft/world/entity/RegionTickOps"*.class
rm -f "$OUT_DIR/net/minecraft/world/level/BlockScheduleOps"*.class

$JAVAC --release 21 -nowarn \
  -cp "$SERVER_JAR:$JOML_JAR:$FASTUTIL_JAR:$OUT_DIR:colpush/build" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/level/BlockScheduleOps.java \
  entityinside/net/minecraft/world/entity/RegionTickOps.java

PRODUCED=$(ls "$OUT_DIR/net/minecraft/world/level/BlockScheduleOps"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "1" ]; then
  echo "SCHED-DEFER delivery guard FAILED: BlockScheduleOps produced $PRODUCED (expected 1)" >&2
  exit 2
fi
NESTED=$(ls "$OUT_DIR/net/minecraft/world/entity/RegionTickOps"*.class 2>/dev/null | wc -l)
if [ "$NESTED" != "3" ]; then
  echo "SCHED-DEFER delivery guard FAILED: RegionTickOps set = $NESTED (expected 3: +Mut +GuardedNavigatingMobs)" >&2
  exit 2
fi
echo "blocksched_ops build OK:"
ls "$OUT_DIR/net/minecraft/world/level/BlockScheduleOps"*.class \
   "$OUT_DIR/net/minecraft/world/entity/RegionTickOps"*.class
