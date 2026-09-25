#!/usr/bin/env bash
# build_456b_blobs_all.sh — TASK-456-B: rebuild ALL widened lever bridges +
# the new POI bridge in ONE javac pass (×93 canon: one pass, nested+flat
# install, cp = full kernel jar + fastutil [+paper-api/adventure]; the
# round-j2b jar is BANNED, lesson x93).
#
# Widened set (poi_widen.py): mobpush, sscan, mobai, entitygoalquery,
# queryplane, goalops, sense + NEW poi/PoiOps.
#
# Usage: scripts/build_456b_blobs_all.sh
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${JAVAC_BIN:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-$(find research -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || KERNEL=/home/z/tools/patched-kernel.jar
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

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
  entityinside/net/minecraft/world/entity/RegionTickOps.java \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java \
  entityinside/net/minecraft/world/entity/BatchCollector.java \
  entityinside/net/minecraft/server/level/TrackerTickOps.java \
  entityinside/net/minecraft/util/RngOps.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  sense/net/minecraft/world/entity/SenseOps.java \
  poi/net/minecraft/world/entity/ai/village/poi/PoiOps.java \
  colpush/net/minecraft/world/entity/ColpushOps.java

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
install_blob goalops/build net/minecraft/world/entity/ai/goal/GoalOps
install_blob sense/build net/minecraft/world/entity/SenseOps
install_blob poi/build net/minecraft/world/entity/ai/village/poi/PoiOps
install_blob colpush/build net/minecraft/world/entity/ColpushOps

# PoiOps must declare NO nested classes (flat==nested trivial; the checker
# also verifies byte identity below via check_blobs_sync.sh).
if ls "$ALL_BUILD"/net/minecraft/world/entity/ai/village/poi/ | grep -q '\$'; then
  echo "FATAL: nested class detected in PoiOps build" >&2
  exit 1
fi

echo "== 456b blob rebuild OK (10 bridges, one javac pass, cp=full, major 65) =="
