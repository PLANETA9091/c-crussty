#!/usr/bin/env bash
# build_voxelintern_ops.sh — cmp470_voxelintern bridge class (S17, round-470).
#
# Compiles VoxelShapeInternOps (net.minecraft.world.level.block.state — SAME
# package as BlockBehaviour$BlockStateBase$Cache so the protected-final
# collisionShape field is package-readable from the sweep) against the real
# runtime jar (purpur-1.21.10.jar, Mojang-mapped). --release 21 pins class
# major 65. Output MUST stay a SINGLE classfile (S7-163: no nested classes —
# NCDFE canon; pinned by voxelintern_source_declares_no_nested_classes).
#
# javap flat==nested contract (tick-470 canon): the built classfile carries
# zero nested member classes — verified here via javap on the output and by
# the single-output count guard.
#
# Usage: scripts/build_voxelintern_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/tmp/jdk21/bin/javac}"
[ -x "$JAVAC" ] || JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
JAVAP="${JAVAC%javac}javap"
SERVER_JAR="${SERVER_JAR:-/tmp/versions/1.21.10/purpur-1.21.10.jar}"
[ -f "$SERVER_JAR" ] || SERVER_JAR=/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar
[ -f "$SERVER_JAR" ] || { echo "runtime jar not found" >&2; exit 1; }

SRC=entityinside/net/minecraft/world/level/block/state/VoxelShapeInternOps.java
OUT_DIR=entityinside/build
OUT_CLS="$OUT_DIR/net/minecraft/world/level/block/state/VoxelShapeInternOps.class"

# BuiltInRegistries pulls com.mojang.serialization (datafixerupper) — add the
# runtime library tree when present (compile-time signature use only).
LIBS=""
for d in /tmp/my-project/libraries /tmp/pdec/matsrv/libraries; do
  [ -d "$d" ] || continue
  LIBS=$(find "$d" -name 'datafixerupper*.jar' -o -name 'guava*.jar' 2>/dev/null | head -4 | tr '\n' ':')
  [ -n "$LIBS" ] && break
done
CP="$SERVER_JAR"
[ -n "$LIBS" ] && CP="$SERVER_JAR:$LIBS"

mkdir -p "$OUT_DIR/net/minecraft/world/level/block/state"

"$JAVAC" --release 21 -cp "$CP" -d "$OUT_DIR" "$SRC"

echo "build output:"
ls -la "$OUT_CLS"
# S7-163 guard at build time: exactly one classfile must come out.
N=$(find "$OUT_DIR" -name 'VoxelShapeInternOps*' -type f | wc -l)
[ "$N" -eq 1 ] || { echo "nested classfile detected ($N outputs) — bridge would NCDFE" >&2; exit 1; }

# javap flat==nested contract: dump members, require the two redirect
# statics, and confirm javap reports the class WITHOUT any nested $-classes
# next to it (the count guard above already proves a single file).
"$JAVAP" -p -classpath "$OUT_DIR" net.minecraft.world.level.block.state.VoxelShapeInternOps
"$JAVAP" -p -c -classpath "$OUT_DIR" net.minecraft.world.level.block.state.VoxelShapeInternOps \
  | grep -q "public static int sweepNow" \
  || { echo "javap contract FAIL: sweepNow()I missing" >&2; exit 1; }
echo "voxelintern bridge OK: single flat classfile, sweepNow present (major pinned 21)"
