#!/usr/bin/env bash
# build_mcomp_blobs_all.sh — TASK-415-A: rebuild ALL lever blobs after gate
# edits (lesson ×93 ×3: stale blob = lever-BUG/plane-sleep/mega-плоскости уснуть).
# Full cp per mandate: kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0. All edited bridges in ONE javac pass.
# Usage: scripts/build_mcomp_blobs_all.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || { echo "kernel jar not found: $KERNEL" >&2; exit 1; }
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
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java \
  items/net/minecraft/world/entity/item/ItemMergeOps.java

install_blob() { # outdir fqcn...
  # TASK-416-A ×93-гейт КРИТИЧНО: rust include_bytes! читает ВЛОЖЕННЫЙ путь
  # (build/net/minecraft/...), легаси-скрипт обновлял ТОЛЬКО плоский
  # build/<Name>.class — «пересборка» оставляла реально-вшитый блоб устаревшим
  # (второй корень franken-merge mc1-3: плоский свежий, вложенный спит).
  # Обновляем ОБА пути: вложенный (include_bytes! contract) + плоский (legacy).
  local outdir="$1"; shift
  mkdir -p "$outdir"
  local cls base nested
  for cls in "$@"; do
    base=$(basename "$cls")
    nested="$outdir/$cls.class"
    mkdir -p "$(dirname "$nested")"
    cp "$ALL_BUILD/$cls.class" "$nested"
    cp "$ALL_BUILD/$cls.class" "$outdir/$base.class"
    echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $outdir/$base.class"
  done
}

install_blob mobpush/build net/minecraft/world/entity/MobPushOps
install_blob sscan/build net/minecraft/world/entity/MobScanOps
install_blob mobai/build net/minecraft/world/entity/MobAiOps
install_blob entityinside/build net/minecraft/world/entity/ItemEntityManager
install_blob queryplane/build net/minecraft/world/entity/QueryPlaneOps
install_blob gsel/build net/minecraft/world/entity/ai/goal/GoalBatchOps
install_blob items/build net/minecraft/world/entity/item/ItemMergeOps

echo "== mcomp blob rebuild OK (7 bridges, one javac pass, cp=full, major 65) =="
