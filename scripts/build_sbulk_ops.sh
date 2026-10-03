#!/usr/bin/env bash
# Build the SelectorBulkOps bridge class (ROUND-486 C07, lever cmp486_sbulk1).
#
# Bulk-JNI bridge (natives in src/selector_bulk.rs: sbProbe/sbEnumerate/
# sbStats), same delivery pattern as EntityGoalQueryOps (TASK-410-C):
# compiled offline against the real runtime jar (purpur-1.21.10.jar,
# Mojang-mapped) and defined into the KERNEL loader at activation time
# (package net.minecraft.world.entity.selector — kernel-internal types
# EntitySelector/Level/AABB are public here).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21).
# NCDFE canon (x456): the wiring tick MUST EARLY-define this class BEFORE the
# first retarget resolves it (define-before-retransform, T1=0 gate).
#
# SKELETON STATUS: not invoked this tick (local javap/javac unavailable —
# JRE-only sandbox; canon CANON-COMMANDER.md line 24: CI gate self-verifies
# javap flat==nested on the built class via scripts/cert458n/javap_flat_nested.py).
#
# Usage: scripts/build_sbulk_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /tmp/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/tmp/jdk-21.0.12.1+1/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

SERVER_JAR="${SERVER_JAR:-/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then echo "runtime jar not found: $SERVER_JAR" >&2; exit 1; fi

OUT_DIR=bulkjni/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn \
  -classpath "$SERVER_JAR" \
  -d "$OUT_DIR" \
  bulkjni/net/minecraft/world/entity/selector/SelectorBulkOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/selector/

# javap flat==nested gate (sleeping-blob canon ×425): the built .class must
# match this exact source — lever-in-SOURCES-without-rebuild = placebo.
if command -v javap > /dev/null 2>&1; then
  javap -p -c "$OUT_DIR/net/minecraft/world/entity/selector/SelectorBulkOps.class" \
    > "$OUT_DIR/SelectorBulkOps.javap.txt"
  echo "javap flat dump: $OUT_DIR/SelectorBulkOps.javap.txt"
else
  echo "javap unavailable locally — CI gate self-verifies (canon line 24)"
fi
