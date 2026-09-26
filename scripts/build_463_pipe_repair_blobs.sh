#!/usr/bin/env bash
# build_463_pipe_repair_blobs.sh — TASK-463-88a repair: the x463 merge 887c4641
# union-glued the paldelta+eqsnap2 flag terms INSIDE one equals() string
# ("cmp457_paldelta|cmp457_eqsnap2", pipe in the CP string) at 11 java-gate
# sites × 7 gate classes — LEDGER-46 §3. Rust repair 9df38ba1 fixed only the
# 61 rust ` || `-strings; the java layer kept sleeping (7/10 gate classes
# never match a real flag) and carried an NCDFE landmine if anyone ever
# dispatches with a pipe-joined lever id.
#
# This script rebuilds the 7 affected blobs (mobpush, entitygoalquery,
# queryplane, sscan MobScanOps + the sscan MobPushOps double, sense, mobai,
# goalops) from the repaired standalone-term sources: ONE javac pass,
# cp = kernel round-396-a + entityinside/build + mobpush/build
# (+ fastutil/paper/adventure, canon round-457c), --release 21 (major 65),
# NESTED path installed FIRST (include_bytes! contract) then flat legacy
# copy; sscan MobPushOps double kept byte-identical to the mobpush copy.
#
# ItemEntityManager + ColpushOps join the pass as sync-fix riders: the swar
# union 85aea399 added cmp458_swar to their SOURCES but never rebuilt their
# tracked blobs (pre-existing check_blobs_sync FAIL on master 0359140d, found
# by this gate run) — rebuilding from current sources keeps the standalone
# paldelta/eqsnap2 CP shape AND restores source<->blob flag sync.
#
# Usage: scripts/build_463_pipe_repair_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || KERNEL=/home/z/tools/patched-kernel.jar
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityinside/build:mobpush/build"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

# ONE javac pass (inter-source deps resolved by javac itself)
"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  mobpush/net/minecraft/world/entity/MobPushOps.java \
  entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java \
  sscan/net/minecraft/world/entity/MobScanOps.java \
  sense/net/minecraft/world/entity/SenseOps.java \
  mobai/net/minecraft/world/entity/MobAiOps.java \
  goalops/net/minecraft/world/entity/ai/goal/GoalOps.java \
  entityinside/net/minecraft/world/entity/ItemEntityManager.java \
  colpush/net/minecraft/world/entity/ColpushOps.java

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

install_nested_glob mobpush/build         net/minecraft/world/entity/MobPushOps
install_nested_glob sscan/build           net/minecraft/world/entity/MobPushOps   # sscan double, byte-identical
install_nested_glob entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
install_nested_glob queryplane/build      net/minecraft/world/entity/QueryPlaneOps
install_nested_glob sscan/build           net/minecraft/world/entity/MobScanOps
install_nested_glob sense/build           net/minecraft/world/entity/SenseOps
install_nested_glob mobai/build           net/minecraft/world/entity/MobAiOps
install_nested_glob goalops/build         net/minecraft/world/entity/ai/goal/GoalOps
install_nested_glob entityinside/build    net/minecraft/world/entity/ItemEntityManager  # swar sync-fix rider
install_nested_glob colpush/build         net/minecraft/world/entity/ColpushOps         # swar sync-fix rider

# CP-EXACT pre-gate: NO combined pipe literal may survive in any rebuilt blob,
# and BOTH standalone constants must be present (LEDGER-46 §3 canonical lesson:
# substring-grep of the combined literal is what let this ship in 887c4641).
echo "== CP-EXACT: combined pipe literal must be ABSENT, standalone terms PRESENT =="
for b in \
  mobpush/build/net/minecraft/world/entity/MobPushOps.class \
  sscan/build/net/minecraft/world/entity/MobPushOps.class \
  entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class \
  queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class \
  sscan/build/net/minecraft/world/entity/MobScanOps.class \
  sense/build/net/minecraft/world/entity/SenseOps.class \
  mobai/build/net/minecraft/world/entity/MobAiOps.class \
  goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class \
  entityinside/build/net/minecraft/world/entity/ItemEntityManager.class \
  colpush/build/net/minecraft/world/entity/ColpushOps.class
do
  python3 scripts/check_cp_exact.py "$b"
done

echo "build_463_pipe_repair_blobs: 9 blobs rebuilt (one javac pass, major 65) — run scripts/check_blobs_sync.sh next"
