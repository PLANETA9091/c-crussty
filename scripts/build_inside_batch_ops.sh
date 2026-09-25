#!/usr/bin/env bash
# build_inside_batch_ops.sh — TASK-460-01 (ID-P31 INSIDE-BATCH wiring on the
# cmp456_chunkmono carrier, lever cmp456_chunkmono_p31snap): rebuild the
# InsideBatchOps bridge blob.
#
# Урок-408/425: lever в SOURCES без пересборки tracked-блобов = ПЛАЦЕБО —
# блоб обязан перегенериться и закоммититься. One javac pass (--release 21,
# major 65) against the real patched kernel (Mojang-mapped Entity/Level/AABB).
# TASK-462-61: EVERY emitted class (top + nested $QuantumVisitor) is installed
# to the nested include_bytes! path AND the flat legacy copy, flat==nested
# byte-equality gate per file (javap/cmp, lesson ×93).
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

# TASK-462-61 (chkclimb-12 cert-fix, LEDGER-24 ×461): the old 2-cp-single-file
# install shipped ONLY InsideBatchOps.class and left
# InsideBatchOps$QuantumVisitor.class inside the trap-removed ALL_BUILD tmpdir
# => lazy resolution NCDFE ×38 at the first quantum SERVE (boats, run
# 36174354289, verdict INVALID). Canon `install_nested_glob` (lesson ×93
# «flat AND nested paths both installed», build_432b_blobs.sh): install EVERY
# javac-emitted $CLS*.class to BOTH the nested include_bytes! path and the
# flat legacy copy, cmp flat==nested for EACH file, FATAL if javac produced
# < 2 class files (nested companion missing => the cert hole reopens).
CLS=net/minecraft/world/entity/InsideBatchOps
BASE=$(basename "$CLS")
shopt -s nullglob
blobs=("$ALL_BUILD/$CLS"*.class)
shopt -u nullglob
if [ "${#blobs[@]}" -lt 2 ]; then
  echo "FATAL: javac emitted ${#blobs[@]} class file(s) for $CLS — nested companion missing (cert canon: >= 2)" >&2
  exit 1
fi
mkdir -p "entityinside/build/$(dirname "$CLS")"
pkgdir=$(dirname "$CLS")
for f in "${blobs[@]}"; do
  name=$(basename "$f")
  nested="entityinside/build/$pkgdir/$name"
  flat="entityinside/build/$name"
  cp "$f" "$nested"
  cp "$f" "$flat"
  if ! cmp -s "$nested" "$flat"; then
    echo "FATAL: flat!=nested for $CLS/$name" >&2
    exit 1
  fi
  echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $flat (flat==nested OK)"
done
echo "inside_batch blob set installed: ${#blobs[@]} class file(s) (top + nested companions)"
