#!/usr/bin/env bash
# build_460_pdclimb_blobs.sh — TASK-460-03 climb (round-460-pdclimb-1): rebuild
# ALL lever blobs after the cmp457_paldelta_p31 family wiring (lesson-408 x2:
# stale/tracked blob = sleeping gate = placebo; the served .so embeds the
# NESTED blob via include_bytes! — every bridge class is recompiled here with
# the climb lever strings in place, then installed nested+flat).
#
# Also stages the FLAT javap dir (/tmp/agent-460-03-javap/flat) consumed by
# scripts/cert458n/javap_flat_nested.py (FLAT_CP env) — flat vs tracked-blob
# svorta incl. the 11th pair InsideBatchOps.
#
# Usage: scripts/build_460_pdclimb_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }
FASTUTIL=/home/z/tools/fastutil.jar
PAPER_API=/home/z/tools/paper-api-1.21.10.jar
ADV_API=/home/z/tools/adventure-api-4.24.0.jar
ADV_KEY=/home/z/tools/adventure-key-4.24.0.jar
JOML=/home/z/tools/joml-1.10.7.jar
CP="$KERNEL"
for j in "$FASTUTIL" "$PAPER_API" "$ADV_API" "$ADV_KEY" "$JOML"; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
FLAT_DIR=/tmp/agent-460-03-javap/flat
mkdir -p "$FLAT_DIR"
trap 'rm -rf "$ALL_BUILD"' EXIT

# one javac pass, full cp: every lever source touched by the climb wiring
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
  entityinside/net/minecraft/world/entity/InsideBlockOps.java \
  entityinside/net/minecraft/world/entity/InsideBatchOps.java \
  entityinside/net/minecraft/world/entity/InsideBitmaskOps.java \
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  entityinside/net/minecraft/util/RngOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  sense/net/minecraft/world/entity/SenseOps.java \
  randomtick/src/BrainOps.java

install_nested_glob() { # outdir slashpath — nested + flat + all $-nested classes
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

install_nested_glob colpush/build      net/minecraft/world/entity/ColpushOps
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob entityinside/build net/minecraft/world/entity/ItemEntityManager
install_nested_glob goalops/build      net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob mobai/build        net/minecraft/world/entity/MobAiOps
install_nested_glob mobpush/build      net/minecraft/world/entity/MobPushOps
install_nested_glob queryplane/build   net/minecraft/world/entity/QueryPlaneOps
install_nested_glob randomtick/build   net/minecraft/world/entity/ai/BrainOps
install_nested_glob sense/build        net/minecraft/world/entity/SenseOps
install_nested_glob sscan/build        net/minecraft/world/entity/MobScanOps
install_nested_glob entityinside/build net/minecraft/world/entity/InsideBatchOps
install_nested_glob entityinside/build net/minecraft/world/entity/InsideBlockOps
install_nested_glob entityinside/build net/minecraft/world/entity/InsideBitmaskOps
install_nested_glob entityinside/build net/minecraft/world/entity/InsideSnapOps

# sscan legacy flat duplicates (MobPushOps/MobScanOps flat mirrors, CERT-FIX canon)
cp "$ALL_BUILD/net/minecraft/world/entity/MobPushOps.class" sscan/MobPushOps.class 2>/dev/null || true
cp "$ALL_BUILD/net/minecraft/world/entity/MobScanOps.class" sscan/MobScanOps.class 2>/dev/null || true

# flat javap stage (gate FLAT_CP): whole ALL_BUILD tree as one classpath root
cp -r "$ALL_BUILD/." "$FLAT_DIR/"
echo "flat javap dir staged: $FLAT_DIR ($(find "$FLAT_DIR" -name '*.class' | wc -l) classes)"
echo "DONE 460-pdclimb blobs"
