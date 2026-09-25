#!/usr/bin/env bash
# build_457c_blobs_all.sh — TASK-455-A: rebuild ALL lever blobs touched by the
# R4 despawn+spawn+activation carrier union (cmp457_eqsnap2 added STRICT-OR into
# java flag-lists; ×93 lesson: stale blob = sleeping plane; flat AND nested
# paths both installed — include_bytes! embeds the NESTED path).
#
# One javac pass, full cp: kernel (round-396-a mandate /home/z/tools/patched-kernel.jar)
# + fastutil + paper-api 1.21.10 + adventure-api/key 4.24.0 (NEVER round-j2b-jar).
# Sources touched this round: MobScanOps (despawn plane lever list), MobAiOps,
# MobPushOps, QueryPlaneOps, ColpushOps (FLAG7), GoalOps, SenseOps,
# EntityGoalQueryOps (+FLAG_LABEL), ItemEntityManager, BrainOps (TICK2_FLAGS)
# + companions (RegionTickOps/BlockUpdateOps/BatchCollector/TrackerTickOps/RngOps).
# Usage: scripts/build_457c_blobs_all.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-/home/z/tools/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL=$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)
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
  mobai/net/minecraft/world/entity/MobAiOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  colpush/net/minecraft/world/entity/ColpushOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  sense/net/minecraft/world/entity/SenseOps.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entityinside/net/minecraft/world/entity/RegionTickOps.java \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java \
  entityinside/net/minecraft/world/entity/BatchCollector.java \
  entityinside/net/minecraft/server/level/TrackerTickOps.java \
  entityinside/net/minecraft/util/RngOps.java \
  randomtick/src/BrainOps.java

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

install_nested_glob mobai/build           net/minecraft/world/entity/MobAiOps
install_nested_glob sscan/build           net/minecraft/world/entity/MobScanOps
install_nested_glob sscan/build           net/minecraft/world/entity/MobPushOps
install_nested_glob mobpush/build         net/minecraft/world/entity/MobPushOps
install_nested_glob queryplane/build      net/minecraft/world/entity/QueryPlaneOps
install_nested_glob colpush/build         net/minecraft/world/entity/ColpushOps
install_nested_glob goalops/build         net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob sense/build           net/minecraft/world/entity/SenseOps
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob entityinside/build    net/minecraft/world/entity/ItemEntityManager
install_nested_glob entityinside/build    net/minecraft/world/entity/RegionTickOps
install_nested_glob entityinside/build    net/minecraft/server/level/BlockUpdateOps
install_nested_glob entityinside/build    net/minecraft/world/entity/BatchCollector
install_nested_glob entityinside/build    net/minecraft/server/level/TrackerTickOps
install_nested_glob entityinside/build    net/minecraft/util/RngOps
install_nested_glob randomtick/build      net/minecraft/world/entity/ai/BrainOps

# javap ground-truth: every rebuilt blob must show cmp457_eqsnap2 in its constant
# pool (raw-byte grep — indy recipes hide concat flags from javap -c, x93).
echo "== raw-byte cp grep cmp457_eqsnap2 =="
for b in \
  mobai/build/net/minecraft/world/entity/MobAiOps.class \
  sscan/build/net/minecraft/world/entity/MobScanOps.class \
  mobpush/build/net/minecraft/world/entity/MobPushOps.class \
  queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class \
  colpush/build/net/minecraft/world/entity/ColpushOps.class \
  goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class \
  sense/build/net/minecraft/world/entity/SenseOps.class \
  entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class \
  entityinside/build/net/minecraft/world/entity/ItemEntityManager.class \
  randomtick/build/net/minecraft/world/entity/ai/BrainOps.class
do
  python3 - "$b" <<'PY'
import sys
b = open(sys.argv[1], 'rb').read()
assert b'cmp457_eqsnap2' in b, f"STALE: {sys.argv[1]} lacks cmp457_eqsnap2"
print(f"  OK {sys.argv[1]}: cmp457_eqsnap2 in cp")
PY
done

echo "== 457c blob rebuild OK (one javac pass, cp=full, major 65) =="
