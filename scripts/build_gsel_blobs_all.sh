#!/usr/bin/env bash
# build_gsel_blobs_all.sh — TASK-414-C2: rebuild ALL lever blobs after ANY
# .java edit (lesson ×93: stale blob = lever-BUG/plane-sleep) + build the NEW
# GoalBatchOps bridge (cmp415_gsel2 gsel-batch plane).
#
# Full cp per tick-414 mandate: kernel round-396-a + fastutil + paper-api
# 1.21.10 + adventure-api/key 4.24.0. Class major pinned 65 (--release 21).
# All 6 bridges compile in ONE javac pass (cross-references MobPushOps ↔
# EntityGoalQueryOps ↔ MobAiOps resolve from source), classes are then copied
# to their canonical per-plane build dirs (kernel include_bytes! paths).
#
# Usage: scripts/build_gsel_blobs_all.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"

KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
if [ ! -f "$KERNEL" ]; then
  KERNEL="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | tail -1 || true)"
fi
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

FASTUTIL=/home/z/tools/fastutil.jar
PAPER_API=/home/z/tools/paper-api-1.21.10.jar
ADV_API=/home/z/tools/adventure-api-4.24.0.jar
ADV_KEY=/home/z/tools/adventure-key-4.24.0.jar
for j in "$FASTUTIL" "$PAPER_API" "$ADV_API" "$ADV_KEY"; do
  [ -f "$j" ] || { echo "missing tool jar: $j" >&2; exit 1; }
done

CP="$KERNEL:$FASTUTIL:$PAPER_API:$ADV_API:$ADV_KEY"

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

$JAVAC --release 21 -cp "$CP" -d "$ALL_BUILD" \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java

install_blob() { # outdir fqcn...
  local outdir="$1"; shift
  mkdir -p "$outdir"
  local cls base
  for cls in "$@"; do
    base=$(basename "$cls")
    cp "$ALL_BUILD/$cls.class" "$outdir/$base.class"
    echo "blob: $outdir/$base.class ($(stat -c%s "$outdir/$base.class") bytes)"
  done
}

install_blob mobpush/build net/minecraft/world/entity/MobPushOps
install_blob sscan/build net/minecraft/world/entity/MobScanOps
install_blob mobai/build net/minecraft/world/entity/MobAiOps
install_blob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_blob entityinside/build net/minecraft/world/entity/ItemEntityManager
install_blob gsel/build/net/minecraft/world/entity/ai/goal net/minecraft/world/entity/ai/goal/GoalBatchOps

echo "== blob rebuild OK (6 bridges, one javac pass, cp=full, major 65) =="
