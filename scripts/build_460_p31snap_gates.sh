#!/usr/bin/env bash
# build_460_p31snap_gates.sh — TASK-460-01 (lever cmp456_chunkmono_p31snap,
# P31 INSIDE-BATCH + P32 snapreg sidecar on the cmp456_chunkmono carrier):
# rebuild EVERY lever blob whose source gate gained the composite id
# (mirror-drift lesson ×452: prod gates and blobs move synchronously;
# lesson-408/425: lever в SOURCES без пересборки tracked-блобов = ПЛАЦЕБО).
#
# One javac pass (inter-source deps resolved by javac itself), full cp =
# kernel (tools fallback) + fastutil + paper-api 1.21.10 + adventure 4.24.0,
# --release 21 (major 65), NESTED path installed FIRST (include_bytes!
# contract) then flat legacy copy, then flat==nested byte-equality gate.
#
# Sources touched this round (composite-id gate edits):
#   MobAiOps / ColpushOps / MobPushOps / MobScanOps / ItemEntityManager /
#   EntityGoalQueryOps / QueryPlaneOps / SenseOps / GoalOps
#   (+ InsideBatchOps NEW bridge; InsideSnapRegistryOps rebuilt separately)
# + the entityinside companion set (RegionTickOps et al.) for deps.
#
# Usage: scripts/build_460_p31snap_gates.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="/home/z/tools/patched-kernel.jar"
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL"
for d in entityinside/build mobpush/build sscan/build colpush/build mobai/build \
         mobai/build sense/build goalops/build entitygoalquery/build \
         queryplane/build chunkparse/build chunksend/build chunksched/build; do
  [ -d "$d" ] && CP="$CP:$d"
done
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  colpush/net/minecraft/world/entity/ColpushOps.java \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  sense/net/minecraft/world/entity/SenseOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  entityinside/net/minecraft/world/entity/InsideBatchOps.java \
  entityinside/net/minecraft/world/entity/InsideSnapRegistryOps.java \
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  entityinside/net/minecraft/world/entity/RegionTickOps.java \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java \
  entityinside/net/minecraft/world/entity/BatchCollector.java \
  entityinside/net/minecraft/server/level/TrackerTickOps.java \
  entityinside/net/minecraft/util/RngOps.java

install_nested_glob() { # outdir fqcn — nested + flat + all $-nested + equality gate
  local outdir="$1" cls="$2"
  local dir base name copied=0
  base=$(basename "$cls")
  dir=$(dirname "$cls")
  mkdir -p "$outdir/$dir"
  for f in "$ALL_BUILD/$dir/$base"*.class; do
    [ -f "$f" ] || continue
    name=$(basename "$f")
    cp "$f" "$outdir/$dir/$name"
    cp "$f" "$outdir/$name"
    cmp -s "$outdir/$dir/$name" "$outdir/$name" || { echo "FATAL: flat!=nested for $cls" >&2; exit 1; }
    copied=$((copied + 1))
  done
  echo "blob-glob: $outdir/$cls.class (+$((copied-1)) nested) — flat copies installed, flat==nested OK"
}

install_nested_glob colpush/build       net/minecraft/world/entity/ColpushOps
install_nested_glob mobpush/build       net/minecraft/world/entity/MobPushOps
install_nested_glob mobai/build         net/minecraft/world/entity/MobAiOps
install_nested_glob sscan/build         net/minecraft/world/entity/MobScanOps
install_nested_glob sscan/build         net/minecraft/world/entity/MobPushOps
install_nested_glob sense/build         net/minecraft/world/entity/SenseOps
install_nested_glob goalops/build       net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob entityinside/build  net/minecraft/world/entity/ItemEntityManager
install_nested_glob entityinside/build  net/minecraft/world/entity/InsideBatchOps
install_nested_glob entityinside/build  net/minecraft/world/entity/InsideSnapRegistryOps
install_nested_glob entityinside/build  net/minecraft/world/entity/InsideSnapOps
install_nested_glob entityinside/build  net/minecraft/world/entity/RegionTickOps
install_nested_glob entityinside/build  net/minecraft/server/level/BlockUpdateOps
install_nested_glob entityinside/build  net/minecraft/world/entity/BatchCollector
install_nested_glob entityinside/build  net/minecraft/server/level/TrackerTickOps
install_nested_glob entityinside/build  net/minecraft/util/RngOps
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob queryplane/build    net/minecraft/world/entity/QueryPlaneOps

echo "== 460 p31snap gate-blob rebuild OK (one javac pass, cp=full, major 65) =="
