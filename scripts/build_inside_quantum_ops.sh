#!/usr/bin/env bash
# Build + selfTest the P34 InsideBlockOps quantum-gate stub (ID-P34,
# TASK-459-74). JDK-only: NO kernel jar needed — the stub is a pure
# reference model (selfTest-canon, p41 precedent); kernel-facing JNI /
# flat-slot wiring is a next-leg step.
#
# --release 21 pins the class-file major to 65 = the kernel JVM (Java 21);
# canon scripts/build_inside_block_ops.sh. DORMANT: the produced class is
# NOT include_bytes!'d anywhere (NCDFE-canon, RESEARCH-459-P34.md §2).
#
# Usage: scripts/build_inside_quantum_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

SRC=entityinside/quantum/net/minecraft/world/entity/InsideBlockOps.java
OUT_DIR=entityinside/quantum/build

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -Xlint:-options -d "$OUT_DIR" "$SRC"

JAVA="${JAVAC%javac}java"
"$JAVA" -cp "$OUT_DIR" net.minecraft.world.entity.quantum.InsideBlockOps

echo "P34 quantum stub OK: $OUT_DIR (dormant, not woven)"
