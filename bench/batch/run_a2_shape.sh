#!/usr/bin/env bash
# TASK-48 Phase 1 bench — wave-1 shape A' (III[J)I) dispatch cost vs direct
# (agent-7625532f).
#
# Single arm = the TASK-48 commit (arg, default: the sha this script ships
# with). Drives kernels 12,13 (DensityAp2MinMaxFill old/newSummary, A') AND
# kernels 2,3 (AquiferIndexStride old/newBatchSummary, shape A) in the SAME
# JVM process: the A-vs-A' delta isolates the marginal cost of the 3-long
# packed scalar plane, and batch-vs-direct per kernel gives the breakeven K.
#
# Usage: bench/batch/run_a2_shape.sh [--commit <sha>] [--sizes 1,8,16,64,256]
# Output: bench/batch/results/A2_SHAPE_<ts>.log + A2_SHAPE_RAW.tsv
#
# Exclusivity: full BENCH.lock (flock 9) for the whole run, like TASK-24.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RES="$ROOT/bench/batch/results"
JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
LOCK=/home/z/BENCH.lock
COMMIT=""
SIZES="1,8,16,64,256"
OPS="${OPS:-60000}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit) COMMIT="$2"; shift 2;;
    --sizes)  SIZES="$2";  shift 2;;
    *) echo "unknown arg $1" >&2; exit 2;;
  esac
done
[[ -n "$COMMIT" ]] || { echo "usage: $0 --commit <sha> (the TASK-48 code state to bench)" >&2; exit 2; }
[[ -x "$JDK/bin/java" ]] || { echo "JDK not found at $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }

# ---- build classes on the fly (never committed; .gitignore covers this) ----
CLASSES="$ROOT/bench/batch/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/java/PaperNativeAquiferIndexStride.java \
  "$ROOT"/bench/batch/java/PaperNativeDensityAp2MinMaxFill.java \
  "$ROOT"/bench/batch/java/BatchFloorBench.java

# ---- detached worktree at the commit (never touch the shared main WIP) ----
WT="/tmp/w-t48"
rm -rf "$WT"
(cd "$ROOT" && git worktree add --detach "$WT" "$COMMIT") >/dev/null 2>&1
trap '(cd "$ROOT" && git worktree remove --force "$WT" >/dev/null 2>&1 || true)' EXIT
( cd "$WT" && source "$HOME/.cargo/env" && cargo build --release 2>&1 | tail -1 )
LIB="$WT/target/release/libcrussty.so"
[[ -f "$LIB" ]] || { echo "build failed: $LIB missing" >&2; exit 2; }

mkdir -p "$RES"
TS="$(date -u +%H%M%S)"
LOG="$RES/A2_SHAPE_${TS}.log"
RAW="$RES/A2_SHAPE_RAW.tsv"

exec 9>"$LOCK"
flock 9
echo "# BENCH.lock acquired $(date -u +%FT%TZ) by agent-7625532f (commit=$COMMIT)"

# LD_LIBRARY_PATH + CRUSSTY_BATCH_NATIVE_LIB: same rationale as TASK-24.
LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$JDK/lib/server" \
CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" \
"$JDK/bin/java" -Xms256m -Xmx512m -cp "$CLASSES" BatchFloorBench \
    --lib "$LIB" --native "$NATIVE_LIB" --kernels 2,3,12,13 --sizes "$SIZES" \
    --rounds 11 --ops "$OPS" 2>&1 | tee "$LOG"

grep '^RESULT' "$LOG" | sed 's/^RESULT/a2\t/' >> "$RAW" || true
flock -u 9
echo "# BENCH.lock released $(date -u +%FT%TZ)"
echo "# log: $LOG"
echo "# raw rows appended: $(grep -c . "$RAW" || true) -> $RAW"
