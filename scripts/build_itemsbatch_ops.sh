#!/usr/bin/env bash
# build_itemsbatch_ops.sh — cmp446_items bridge class (TASK-446-B).
#
# Compiles ItemBatchOps (net.minecraft.world.entity.item) against the real
# runtime kernel jar (purpur-1.21.10, Mojang-mapped). --release 21 pins the
# class-file major to 65 = the kernel JVM (Java 21); src/items_batch.rs
# refuses to arm if the embedded major exceeds the live JVM's.
#
# S7-163 guard at build time: exactly ONE classfile must come out (no nested
# classes — a nested class would NCDFE at serve time).
#
# Usage: scripts/build_itemsbatch_ops.sh [javac] [kernel-jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL_JAR="${KERNEL_JAR:-/tmp/patched-kernel-396a.jar}"
if [ ! -f "$KERNEL_JAR" ]; then
  KERNEL_JAR=/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar
fi
[ -f "$KERNEL_JAR" ] || KERNEL_JAR=/home/z/rounds/ROUND-405/lib/patched-kernel.jar
[ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }

# Canonical cp (lesson ×93 / build_430b_blobs.sh): kernel + fastutil +
# paper-api 1.21.10 + adventure-api/key 4.24.0. NEVER round-j2b-jar.
CP="$KERNEL_JAR"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

OUT_DIR=itemsbatch/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn -proc:none \
  -classpath "$CP" \
  -d "$OUT_DIR" \
  itemsbatch/net/minecraft/world/entity/ItemBatchOps.java

echo "build output:"
ls -la "$OUT_DIR/net/minecraft/world/entity/"
# S7-163 guard: exactly one classfile must come out.
N=$(find "$OUT_DIR" -name 'ItemBatchOps*' -type f | wc -l)
[ "$N" -eq 1 ] || { echo "nested classfile detected ($N outputs) — bridge would NCDFE" >&2; exit 1; }

# Install NESTED (the path include_bytes! embeds) + FLAT (legacy sibling) and
# gate flat==nested byte-equality (×93 blob-sync discipline).
NESTED="$OUT_DIR/net/minecraft/world/entity/ItemBatchOps.class"
FLAT="$OUT_DIR/ItemBatchOps.class"
cp "$NESTED" "$FLAT"
cmp -s "$NESTED" "$FLAT" || { echo "GATE FAIL: flat != nested" >&2; exit 1; }
echo "blob: $NESTED ($(stat -c%s "$NESTED") bytes) + flat $FLAT"

# Raw-byte gate: the bridge must carry the double gate + native surface +
# vanilla rest-path bodies in its constant pool.
for marker in cmp446_items planeProbe planeDecide selfTest inactiveTick mergeWithNeighbours; do
  if ! rg -a -q "$marker" "$NESTED"; then
    echo "MARKER GATE FAIL: '$marker' missing from $NESTED" >&2
    exit 1
  fi
done
if rg -a -q "LambdaMetafactory" "$NESTED"; then
  echo "INDY GATE WARN: $NESTED references LambdaMetafactory (expected none)" >&2
fi

echo "build_itemsbatch_ops: OK"
