#!/usr/bin/env bash
# TASK-50 (D1) paired probe — shape-B staging old vs new (agent-7625532f).
#
# Arms: --old / --new prebuilt module libcrussty.so (same wire ABI 131086;
# the diff between arms is EXACTLY the D1 staging change, commit d84e405).
# Cells: 64 shape-B ops x len {64,1024,4096}, 4 long-lived threads,
# 20 warm + 41 timed batches/cell/thread, medians + RSS retention block.
# The closed kernels reject synthesized input (ret=8/op) — identical across
# arms, so the arm diff isolates the dispatcher staging path.
#
# Output: bench/batch/results/D1_SHAPEB_RAW.tsv + per-arm logs.
# Exclusivity: full BENCH.lock (flock 9).

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RES="$ROOT/bench/batch/results"
JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
LOCK=/home/z/BENCH.lock

OLD_LIB="${OLD_LIB:?set OLD_LIB}"
NEW_LIB="${NEW_LIB:?set NEW_LIB}"
[[ -x "$JDK/bin/java" ]] || { echo "JDK not found at $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }
[[ -f "$OLD_LIB" && -f "$NEW_LIB" ]] || { echo "arm lib missing" >&2; exit 2; }

CLASSES="$ROOT/bench/batch/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/java/D1StagingProbe.java

mkdir -p "$RES"
RAW="$RES/D1_SHAPEB_RAW.tsv"
: > "$RAW"

exec 9>"$LOCK"
flock 9
echo "# BENCH.lock acquired $(date -u +%FT%TZ) by agent-7625532f"

run_arm() { # $1 = arm tag, $2 = lib path
  local tag="$1" lib="$2"
  local log="$RES/D1_SHAPEB_${tag}.log"
  echo "== arm=$tag lib=$lib =="
  LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$JDK/lib/server" \
  CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" \
  "$JDK/bin/java" -Xms128m -Xmx128m -XX:+UseSerialGC \
    -cp "$CLASSES" D1StagingProbe --lib "$lib" --native "$NATIVE_LIB" 2>&1 | tee "$log"
  grep '^RESULT' "$log" | sed "s/^RESULT/${tag}/" >> "$RAW"
  grep '^# RSS_KB' "$log" | sed "s/^# /${tag}\t/" >> "$RAW"
}

run_arm old "$OLD_LIB"
sleep 2
run_arm new "$NEW_LIB"

echo "== done $(date -u +%FT%TZ), rows=$(wc -l < "$RAW") =="
