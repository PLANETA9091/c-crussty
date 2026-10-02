#!/usr/bin/env bash
# build_collidebatch_ops.sh — cmp401_collide bridge class (TASK-401-B).
#
# Compiles CollideBatchOps (net.minecraft.world.entity) against the real
# runtime jar (purpur-1.21.10.jar, Mojang-mapped). --release 21 pins class
# major 65. Output MUST stay a SINGLE classfile (S7-163: no nested classes —
# pinned by the collidebatch_source_declares_no_nested_classes test).
#
# Usage: scripts/build_collidebatch_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
SERVER_JAR="${SERVER_JAR:-/tmp/versions/1.21.10/purpur-1.21.10.jar}"
[ -f "$SERVER_JAR" ] || SERVER_JAR=/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar
[ -f "$SERVER_JAR" ] || { echo "runtime jar not found" >&2; exit 1; }

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/world/entity"

"$JAVAC" --release 21 -cp "$SERVER_JAR" -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/CollideBatchOps.java

echo "build output:"
ls -la "$OUT_DIR/net/minecraft/world/entity/CollideBatchOps.class"
# S7-163 guard at build time: exactly one classfile must come out.
N=$(find "$OUT_DIR" -name 'CollideBatchOps*' -type f | wc -l)
[ "$N" -eq 1 ] || { echo "nested classfile detected ($N outputs) — bridge would NCDFE" >&2; exit 1; }
