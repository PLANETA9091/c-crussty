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

# TASK-449-A ×449 (recipe б): every class installed by this script is
# recorded here and MUST pass the javap LOADABILITY gate at the end
# (javap -p -cp <blobs-dir> <fqcn> — gate fails if the class is not
# locatable/parseable through a real classpath). This catches a blob-set
# hole BEFORE dispatch: a missing inner-class blob = kernel-loader
# NoClassDefFoundError at runtime (×448 collide-2: ColpushOps$ConstSlot
# NCDFE ×4 → cmp420_colpush disarm → vanilla-fallback + young-GC storm 529).
LOADED_CLASSES=()

gate_load() { # outdir slash-fqcn — javap must locate+parse the class
  local outdir="$1" fq="$2" fq_dots
  fq_dots="${fq//\//.}"
  if "$JAVAP" -p -cp "$outdir" "$fq_dots" >/dev/null 2>&1; then
    echo "javap-load OK: $fq_dots (cp=$outdir)"
  else
    echo "JAVAP-GATE FAIL: $fq_dots not loadable from $outdir (blob-set hole → runtime NCDFE)" >&2
    exit 1
  fi
}
JAVAP="${JAVAP:-/home/z/tools/jdk-21.0.12.1+1/bin/javap}"
[ -x "$JAVAP" ] || JAVAP=$(command -v javap)

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

install_nested_glob() { # outdir fqcn(slash-form) — nested (include_bytes!)
                        # + flat (legacy) + ALL $-inner siblings (×449: inner
                        # classes are part of the delivery surface — a
                        # flat-only install leaves the kernel loader without
                        # the companion = NCDFE on first resolution)
  local outdir="$1" cls="$2"
  local dir base f name copied=0
  base=$(basename "$cls")
  dir=$(dirname "$cls")
  mkdir -p "$outdir/$dir"
  for f in "$ALL_BUILD/$dir/$base"*.class; do
    [ -f "$f" ] || continue
    name=$(basename "$f")
    cp "$f" "$outdir/$dir/$name"
    cp "$f" "$outdir/$name"
    LOADED_CLASSES+=("$outdir|$dir/${name%.class}")
    copied=$((copied + 1))
  done
  [ "$copied" -gt 0 ] || { echo "GATE FAIL: javac produced no $base*.class under $ALL_BUILD/$dir" >&2; exit 1; }
  echo "blob: $outdir/$cls.class (+$((copied-1)) inner) + flat copies"
}

install_nested_glob colpush/build        net/minecraft/world/entity/ColpushOps
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
gate_fe colpush/build        net/minecraft/world/entity 'ColpushOps$ConstSlot'
# shellcheck disable=SC2181
echo "flat==nested gates: OK"

# javap LOADABILITY gate (recipe б): every installed class — outer AND inner —
# must be locatable+parseable through its blobs dir (javap -p -cp). A missing
# inner blob fails HERE, not as NCDFE ×4 after dispatch.
for entry in "${LOADED_CLASSES[@]}"; do
  gate_load "${entry%%|*}" "${entry#*|}"
done

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
