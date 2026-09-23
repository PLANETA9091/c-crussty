#!/usr/bin/env bash
# build_432b_blobs.sh — TASK-432-B: rebuild every lever blob whose source gate
# gained cmp430_inside (STRICT-OR plumbing) + the inside-plane subsystem
# bridges (InsideSnapOps + $Snap; InsideBitmaskOps; TASK-432-B adds InsideBlockOps
# + $Recorder: gate fusion + 2^18 slot space) — ONE javac pass (lesson ×93).
#
# Lesson ×93 discipline: one javac pass (inter-source deps resolved by javac
# itself), full cp = kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar),
# --release 21 (major 65), NESTED path installed FIRST (include_bytes!
# contract) then flat legacy copy, then flat==nested byte-equality gate.
#
# Usage: scripts/build_432b_blobs.sh [javac]
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
# TASK-432-B: InsideBlockOps ($Recorder toAabbs linkage) needs org.joml types
# (the old build_inside_block_ops.sh pulled them from /tmp/pdec matsrv libs —
# purged since). joml 1.10.7 in /home/z/tools (type-linkage only, never shipped).
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
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  entityinside/net/minecraft/world/entity/InsideBlockOps.java \
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
install_nested_glob mobpush/build        net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build          net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build          net/minecraft/world/entity/MobScanOps
install_nested_glob mobai/build          net/minecraft/world/entity/MobAiOps
install_nested_glob entityinside/build   net/minecraft/world/entity/ItemEntityManager
install_nested_glob entityinside/build   net/minecraft/world/entity/InsideSnapOps
install_nested_glob entityinside/build   'net/minecraft/world/entity/InsideSnapOps$Snap'
# ROUND-3 NCDFE FIX (run 35902792520): $Lane (serve-fastpath lanes) was added
# by inside2 but never installed as a blob => kernel loader could not resolve
# `[Lnet/.../InsideSnapOps$Lane;` => clinit NCDFE => permanently erroneous
# class => 200k NCDFE storm. Install nested+flat and gate it below.
install_nested_glob entityinside/build   'net/minecraft/world/entity/InsideSnapOps$Lane'
install_nested_glob entityinside/build   net/minecraft/world/entity/InsideBlockOps
install_nested_glob entityinside/build   'net/minecraft/world/entity/InsideBlockOps$Recorder'
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
gate_fe entityinside/build   net/minecraft/world/entity InsideBlockOps
gate_fe entityinside/build   net/minecraft/world/entity 'InsideBlockOps$Recorder'
gate_fe entityinside/build   net/minecraft/world/entity InsideBitmaskOps
gate_fe entitygoalquery/build net/minecraft/world/entity EntityGoalQueryOps
gate_fe goalops/build        net/minecraft/world/entity/ai/goal GoalOps
gate_fe queryplane/build     net/minecraft/world/entity QueryPlaneOps
gate_fe entityinside/build   net/minecraft/world/entity 'InsideSnapOps$Snap'
gate_fe entityinside/build   net/minecraft/world/entity 'InsideSnapOps$Lane'
gate_fe entityinside/build   net/minecraft/world/entity 'InsideBlockOps$Recorder'
# shellcheck disable=SC2181
echo "flat==nested gates: OK"

# ROUND-3 GATE (canon ×93-indy): InsideSnapOps <clinit> must be indy-FREE.
# An indy bootstrap inside <clinit> resolves the call-site's descriptor types
# EAGERLY at class-init time — any custom type there (e.g. a nested class not
# defined in the kernel loader) permanently poisons the bridge (run 35902792520:
# ExceptionInInitializerError @ InsideSnapOps.java:220 on
# ThreadLocal.withInitial(InsideSnapOps::newLanes) — return type Lane[]).
# Other bridges: advisory warn (InsideBitmaskOps clinit indy #0/#1 are
# String-concat + primitive-array supplier — resolvable, production-proven
# ×200+ legs — but flagged so any NEW custom-typed clinit indy gets eyes).
JAVAP_BIN="${JAVAC:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"; JAVAP_BIN="${JAVAP_BIN%javac}javap"
clinit_indy() { "$JAVAP_BIN" -p -c "$1" 2>/dev/null | awk '/^  static \{\};$/{flag=1; next} /^  [a-zA-Z].*\(.*\);$/{flag=0} flag' | rg -q "invokedynamic"; }
if clinit_indy entityinside/build/net/minecraft/world/entity/InsideSnapOps.class; then
  echo "CLINIT-INDY GATE FAIL: InsideSnapOps.class has invokedynamic in <clinit> (round-3 NCDFE hazard)" >&2
  exit 1
fi
for f in \
  entityinside/build/net/minecraft/world/entity/InsideBitmaskOps.class \
  entityinside/build/net/minecraft/world/entity/InsideBlockOps.class; do
  if clinit_indy "$f"; then
    echo "CLINIT-INDY GATE WARN: $f has invokedynamic in <clinit> (resolvable today; custom-typed bootstrap targets would poison <clinit> — see run 35902792520)" >&2
  fi
done
echo "clinit-indy gates: OK"

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

echo "build_432b_blobs: OK"
