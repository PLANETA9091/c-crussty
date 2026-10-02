#!/usr/bin/env bash
# Build the InsideEpochGate bridge class (ID-P36 / TASK-459-58, inside-epoch fast-gate).
#
# SCAFFOLD NOTE: the bridge source is deliberately vanilla-free (only
# java.lang / java.util.concurrent) so this script compiles it with a BARE
# javac — no runtime jar, no joml, no cross-bridge compile deps
# (SELF-CONTAINED discipline, S7-148 lesson: compile-dep на чужой build =
# NCDFE-шторм при выключенном соседе).
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# src/inside_epoch_gate.rs refuses to arm if the embedded major exceeds the
# live JVM's (same guard as inside_cache / fluid_guard / alloc_diet).
#
# Wiring phase (after this script): include_bytes! the produced blob into
# src/inside_epoch_gate.rs (GATE_BYTES) and flip BLOB_EMBEDDED to true.
#
# Usage: scripts/build_inside_epoch_gate.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/InsideEpochGate.java

echo "inside_epoch_gate: built $OUT_DIR/net/minecraft/world/entity/InsideEpochGate.class"
