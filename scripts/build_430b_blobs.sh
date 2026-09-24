#!/usr/bin/env bash
# build_430b_blobs.sh — TASK-430-B: rebuild every lever blob whose source gate
# gained cmp430_inside (STRICT-OR plumbing) + the NEW inside-plane subsystem
# bridges (InsideSnapOps + $Snap; InsideBitmaskOps freshness rebuild).
#
# Lesson ×93 discipline: one javac pass (inter-source deps resolved by javac
# itself), full cp = kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar),
# --release 21 (major 65), NESTED path installed FIRST (include_bytes!
# contract) then flat legacy copy, then flat==nested byte-equality gate.
#
# Usage: scripts/build_430b_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityinside/build"
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
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  entityinside/net/minecraft/world/entity/InsideBitmaskOps.java \
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
# TASK-449-C δ-gate (урок ×448 NCDFE): inner classes ride the blob-сборка
# class-list — ColpushOps$ConstSlot (TASK-448-A cycle-2 constants plane).
install_nested_glob colpush/build        'net/minecraft/world/entity/ColpushOps$ConstSlot'
install_nested_glob mobpush/build        net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build          net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build          net/minecraft/world/entity/MobScanOps
install_nested_glob mobai/build          net/minecraft/world/entity/MobAiOps
install_nested_glob entityinside/build   net/minecraft/world/entity/ItemEntityManager
install_nested_glob entityinside/build   net/minecraft/world/entity/InsideSnapOps
install_nested_glob entityinside/build   'net/minecraft/world/entity/InsideSnapOps$Snap'
install_nested_glob entityinside/build   net/minecraft/world/entity/InsideBitmaskOps
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob goalops/build        net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob queryplane/build     net/minecraft/world/entity/QueryPlaneOps

# javap gate: flat==nested byte-equality (lesson ×93) for every touched class
gate_fe() { cmp -s "$1/$3.class" "$1/$2/$3.class" || { echo "GATE FAIL: $2/$3 flat != nested" >&2; exit 1; }; }
gate_fe colpush/build        net/minecraft/world/entity ColpushOps
gate_fe colpush/build        net/minecraft/world/entity 'ColpushOps$ConstSlot'
gate_fe mobpush/build        net/minecraft/world/entity MobPushOps
gate_fe sscan/build          net/minecraft/world/entity MobPushOps
gate_fe sscan/build          net/minecraft/world/entity MobScanOps
gate_fe mobai/build          net/minecraft/world/entity MobAiOps
gate_fe entityinside/build   net/minecraft/world/entity ItemEntityManager
gate_fe entityinside/build   net/minecraft/world/entity InsideSnapOps
gate_fe entityinside/build   net/minecraft/world/entity InsideBitmaskOps
gate_fe entitygoalquery/build net/minecraft/world/entity EntityGoalQueryOps
gate_fe goalops/build        net/minecraft/world/entity/ai/goal GoalOps
gate_fe queryplane/build     net/minecraft/world/entity QueryPlaneOps
gate_fe entityinside/build   net/minecraft/world/entity 'InsideSnapOps$Snap'
# shellcheck disable=SC2181
echo "flat==nested gates: OK"

# raw-byte gate: bridges must not reference LambdaMetafactory (no indy lambdas
# on the bridge surface; selfTest bodies are plain bytecode).
for f in \
  entityinside/build/net/minecraft/world/entity/InsideSnapOps.class \
  entityinside/build/net/minecraft/world/entity/InsideBitmaskOps.class \
  queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class; do
  if rg -a -q "LambdaMetafactory" "$f" 2>/dev/null; then
    echo "INDY GATE WARN: $f references LambdaMetafactory (expected none)" >&2
  fi
done

echo "build_430b_blobs: OK"
