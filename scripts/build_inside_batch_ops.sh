#!/usr/bin/env bash
# build_inside_batch_ops.sh — TASK-460-03 climb: compile the INSIDE-BATCH bridge
# (InsideBatchOps.java, ID-P31 bulk-JNI discovery plane) against the materialized
# mojang-mapped kernel with --release 21 (JVM major guard checked at define time
# inside inside_batch.rs). Installs BOTH paths:
#   nested  entityinside/build/net/minecraft/world/entity/InsideBatchOps.class
#           (include_bytes! contract — embedded into libcrussty.so at compile time)
#   flat    entityinside/InsideBatchOps.class (legacy javap gate path)
# Usage: scripts/build_inside_batch_ops.sh [javac] [kernel.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${2:-${KERNEL_JAR:-$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)}}"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

OUT="entityinside/build"
mkdir -p "$OUT/net/minecraft/world/entity"
"$JAVAC" --release 21 -proc:none -cp "$KERNEL" -d "$OUT" \
  entityinside/net/minecraft/world/entity/InsideBatchOps.java

NESTED="$OUT/net/minecraft/world/entity/InsideBatchOps.class"
cp "$NESTED" "entityinside/InsideBatchOps.class"
echo "built: $NESTED ($(stat -c%s "$NESTED") bytes)"
echo "flat:  entityinside/InsideBatchOps.class"
sha256sum "$NESTED"
