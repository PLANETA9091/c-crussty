#!/usr/bin/env bash
# build_460_swarx_blobs.sh — TASK-460-34: rebuild the 8 lever blobs whose
# source gates gained cmp458_swar/cmp457_paldelta at union-widen d22835bd
# (step-1b) but whose tracked .class blobs were NOT rebuilt — the exact
# agent-N F-1 finding for the paldelta family (BLACKBOARD 22:4x: "tracked
# build-блобы НЕ пересобраны (strings: 0/10 содержат cmp457_paldelta), CI
# собирает только Rust => java-гейты ENABLED=false = СПЯЩИЕ ГЕЙТЫ",
# урок 408 / placebo ×425). check_blobs_sync.sh at d22835bd: 8 classes FAIL.
#
# Lesson ×93 discipline: ONE javac pass (inter-source deps resolved by javac
# itself), full cp = kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0 (+ joml for ItemEntityManager type-linkage),
# --release 21 (major 65), NESTED path installed FIRST (include_bytes!
# contract) then flat legacy copy; flat==nested equality enforced by
# scripts/check_blobs_sync.sh after this script.
#
# Usage: scripts/build_460_swarx_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || KERNEL=/home/z/tools/patched-kernel.jar
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityinside/build"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done
JOML="${JOML_JAR:-/home/z/tools/joml-1.10.7.jar}"
[ -f "$JOML" ] && CP="$CP:$JOML"

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  colpush/net/minecraft/world/entity/ColpushOps.java \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java

install_nested_glob() { # outdir fqcn(slash-form) — nested (include_bytes!) + flat (legacy)
  local outdir="$1" cls="$2"
  local base nested
  base=$(basename "$cls")
  nested="$outdir/$cls.class"
  mkdir -p "$(dirname "$nested")"
  cp "$ALL_BUILD/$cls.class" "$nested"
  cp "$ALL_BUILD/$cls.class" "$outdir/$base.class"
  echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $outdir/$base.class"
}

install_nested_glob colpush/build        net/minecraft/world/entity/ColpushOps
install_nested_glob mobpush/build        net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build          net/minecraft/world/entity/MobScanOps
install_nested_glob mobai/build          net/minecraft/world/entity/MobAiOps
install_nested_glob entityinside/build   net/minecraft/world/entity/ItemEntityManager
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob goalops/build        net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob queryplane/build     net/minecraft/world/entity/QueryPlaneOps

echo "build_460_swarx_blobs: 8 blobs rebuilt (one javac pass, major 65) — run scripts/check_blobs_sync.sh next"
