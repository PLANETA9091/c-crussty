#!/usr/bin/env bash
# build_inside_batch_ops.sh — TASK-460-01 (ID-P31 INSIDE-BATCH wiring on the
# cmp456_chunkmono carrier, lever cmp456_chunkmono_p31snap): rebuild the
# InsideBatchOps bridge blob.
#
# Урок-408/425: lever в SOURCES без пересборки tracked-блобов = ПЛАЦЕБО —
# блоб обязан перегенериться и закоммититься. One javac pass (--release 21,
# major 65) against the real patched kernel (Mojang-mapped Entity/Level/AABB).
# NESTED path installed FIRST (include_bytes! contract, inside_batch.rs),
# then flat legacy copy, then flat==nested byte-equality gate (javap/сmp).
#
# Usage: scripts/build_inside_batch_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="/home/z/tools/patched-kernel.jar"
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityinside/build"
for j in /home/z/tools/fastutil.jar /home/z/tools/joml-1.10.7.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  entityinside/net/minecraft/world/entity/InsideBatchOps.java

CLS=net/minecraft/world/entity/InsideBatchOps
BASE=$(basename "$CLS")
nested="entityinside/build/$CLS.class"
cp "$ALL_BUILD/$CLS.class" "$nested"
cp "$ALL_BUILD/$CLS.class" "entityinside/build/$BASE.class"
if ! cmp -s "$nested" "entityinside/build/$BASE.class"; then
  echo "FATAL: flat!=nested for $CLS" >&2
  exit 1
fi
echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat (flat==nested OK)"
