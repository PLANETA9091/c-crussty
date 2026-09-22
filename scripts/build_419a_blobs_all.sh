#!/usr/bin/env bash
# build_419a_blobs_all.sh — TASK-419-A: rebuild ALL lever blobs touched by the
# colpush carrier after gate edits (lesson ×93 ×N: stale blob = sleeping plane;
# flat AND nested paths both installed — include_bytes! embeds the NESTED path).
#
# One javac pass, full cp: kernel round-396-a + fastutil + paper-api 1.21.10
# + adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar).
# Sources touched this round:
#   ColpushOps (NEW) / MobPushOps (boxFor+colpushSweep+gate) / MobScanOps /
#   MobAiOps / ItemEntityManager / RegionTickOps (+companions) /
#   EntityGoalQueryOps / QueryPlaneOps
# Usage: scripts/build_419a_blobs_all.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
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
  colpush/net/minecraft/world/entity/ColpushOps.java \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  entityinside/net/minecraft/world/entity/RegionTickOps.java \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java \
  entityinside/net/minecraft/world/entity/BatchCollector.java \
  entityinside/net/minecraft/server/level/TrackerTickOps.java \
  entityinside/net/minecraft/util/RngOps.java

install_blob() { # outdir fqcn... — nested (include_bytes! contract) + flat (legacy)
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

install_nested_glob() { # outdir fqcn — nested + flat + all $-nested classes
  local outdir="$1" cls="$2"
  local dir base
  base=$(basename "$cls")
  dir=$(dirname "$cls")
  mkdir -p "$outdir/$dir"
  local copied=0
  for f in "$ALL_BUILD/$dir/$base"*.class; do
    [ -f "$f" ] || continue
    local name
    name=$(basename "$f")
    cp "$f" "$outdir/$dir/$name"
    cp "$f" "$outdir/$name"
    copied=$((copied + 1))
  done
  echo "blob-glob: $outdir/$cls.class (+$((copied-1)) nested) — flat copies installed"
}

install_nested_glob colpush/build  net/minecraft/world/entity/ColpushOps
install_nested_glob mobpush/build  net/minecraft/world/entity/MobPushOps
install_nested_glob mobai/build    net/minecraft/world/entity/MobAiOps
install_nested_glob sscan/build    net/minecraft/world/entity/MobScanOps
install_nested_glob sscan/build    net/minecraft/world/entity/MobPushOps
install_nested_glob entityinside/build net/minecraft/world/entity/ItemEntityManager
install_nested_glob entityinside/build net/minecraft/world/entity/RegionTickOps
install_nested_glob entityinside/build net/minecraft/server/level/BlockUpdateOps
install_nested_glob entityinside/build net/minecraft/world/entity/BatchCollector
install_nested_glob entityinside/build net/minecraft/server/level/TrackerTickOps
install_nested_glob entityinside/build net/minecraft/util/RngOps
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob queryplane/build net/minecraft/world/entity/QueryPlaneOps

echo "== 419a blob rebuild OK (one javac pass, cp=full, major 65) =="
