#!/usr/bin/env bash
# TASK-61 old-member parity — parity-through-dispatcher for the wave-1 pairs
# g35/g39/g40 (old ids 18/19/20 vs optimized ids 15/16/17) + dispatcher-vs-
# direct loop closure to the P500 anchor surface. (agent-7625532f)
#
# Usage: bench/batch/run_oldmember_parity.sh [--module <libcrussty.so>]
# Output: bench/batch/results/OLD_MEMBER_PARITY_2026-09-09.log
#         bench/batch/results/OLD_MEMBER_PARITY_RAW.tsv (per-lane PARITY rows)
# Exclusivity: full BENCH.lock (flock 9) for the whole run, like P500.
#
# Domain pins (Wave1ContractProbe, S7-14): g35 n in {1,2} (>=4 refuses -6),
# g40 mode flag = 1 (else -3), g39 any n (String objects). The probe fails
# loudly on domain drift — a FAIL here is a REAL regression signal.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RES="$ROOT/bench/batch/results"
JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
MODULE_LIB="$ROOT/target/release/libcrussty.so"
LOCK=/home/z/BENCH.lock

while [[ $# -gt 0 ]]; do
  case "$1" in
    --module) MODULE_LIB="$2"; shift 2;;
    *) echo "unknown arg $1" >&2; exit 2;;
  esac
done

[[ -x "$JDK/bin/java" ]] || { echo "JDK not found at $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }
[[ -f "$MODULE_LIB" ]]   || { echo "module lib missing: $MODULE_LIB" >&2; exit 2; }

# ---- build classes on the fly (never committed; .gitignore covers this) ----
CLASSES="$ROOT/bench/batch/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/java/PaperNativeRangeChoice.java \
  "$ROOT"/bench/batch/java/PaperNativeSpigotLoadOrderDependency.java \
  "$ROOT"/bench/batch/java/OldMemberParityProbe.java

mkdir -p "$RES"
LOG="$RES/OLD_MEMBER_PARITY_2026-09-09.log"
RAW="$RES/OLD_MEMBER_PARITY_RAW.tsv"

(
  flock 9
  echo "== TASK-61 old-member parity probe =="
  echo "module: $MODULE_LIB"
  echo "native: $NATIVE_LIB"
  echo "start : $(date -Is)"
  CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" CRUSSTY_MODULE_LIB="$MODULE_LIB" \
    "$JDK/bin/java" -XX:+UseG1GC -cp "$CLASSES" OldMemberParityProbe 2>&1 | tee "$LOG"
  RC=${PIPESTATUS[0]}
  echo "end   : $(date -Is) rc=$RC"
  # RAW: one row per PARITY OK line (tag, ret, dsthash)
  grep -E 'PARITY OK|FAIL' "$LOG" | sed 's/^/ROW\t/' > "$RAW"
  echo "raw rows: $(wc -l < "$RAW") -> $RAW"
  exit "$RC"
) 9>"$LOCK"
