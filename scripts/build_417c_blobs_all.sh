#!/usr/bin/env bash
# build_417c_blobs_all.sh — TASK-417-C: rebuild ALL lever blobs after gate
# edits (lesson ×93 ×N: stale blob = sleeping plane; flat AND nested paths
# both installed — include_bytes! embeds the NESTED path, legacy scripts
# sometimes refreshed only the flat copy).
#
# One javac pass, full cp: kernel round-396-a + fastutil + paper-api 1.21.10
# + adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar).
# Bridges whose SOURCES this round touched:
#   MobPushOps / MobScanOps / MobAiOps / ItemEntityManager /
#   EntityGoalQueryOps / QueryPlaneOps (NEW on this carrier)
# Usage: scripts/build_417c_blobs_all.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }
FASTUTIL=/home/z/tools/fastutil.jar
PAPER_API=/home/z/tools/paper-api-1.21.10.jar
ADV_API=/home/z/tools/adventure-api-4.24.0.jar
ADV_KEY=/home/z/tools/adventure-key-4.24.0.jar
CP="$KERNEL"
for j in "$FASTUTIL" "$PAPER_API" "$ADV_API" "$ADV_KEY"; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java

install_blob() { # outdir fqcn...
  # TASK-417-C ×93 discipline: update BOTH paths — nested (include_bytes!
  # contract) + flat (legacy sibling).
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
install_blob sscan/build net/minecraft/world/entity/MobScanOps \
                         net/minecraft/world/entity/MobPushOps
install_blob mobai/build net/minecraft/world/entity/MobAiOps
install_blob entityinside/build net/minecraft/world/entity/ItemEntityManager
install_blob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_blob queryplane/build net/minecraft/world/entity/QueryPlaneOps

echo "== 417c blob rebuild OK (6 bridges, one javac pass, cp=full, major 65) =="
