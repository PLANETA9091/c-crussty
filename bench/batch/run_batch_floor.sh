#!/usr/bin/env bash
# TASK-24 (C3) batch-floor bench — dispatcher before/after (agent-7625532f).
#
# BEFORE = db7cf27 (control-plane Vecs allocated per run())
# AFTER  = master    (28ad646: per-thread scratch reuse, steady state 0 allocs)
#
# Usage: bench/batch/run_batch_floor.sh [--before <path/to/libcrussty.so>]
#                                       [--after  <path/to/libcrussty.so>]
#                                       [--kernels <id[,id..]>]
# Missing arms are built on the fly into /tmp worktrees (detached).
# --kernels passes through to BatchFloorBench (default 2,3 = shape A;
# G3 spike: 14 = shape C g42 StaticCacheGet newBatchSummary).
# Output: bench/batch/results/BATCH_FLOOR_<arm>.log + BATCH_FLOOR_RAW.tsv
# (aggregated medians land in BATCH_FLOOR_REPORT.md, committed next to this).
#
# Exclusivity: full BENCH.lock (flock 9) for the whole run, like P500.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RES="$ROOT/bench/batch/results"
JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
LOCK=/home/z/BENCH.lock
ARMS=()
KERNELS="2,3"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --before) ARMS+=("before"); BEFORE_LIB="$2"; shift 2;;
    --after)  ARMS+=("after");  AFTER_LIB="$2";  shift 2;;
    --kernels) KERNELS="$2"; shift 2;;
    *) echo "unknown arg $1" >&2; exit 2;;
  esac
done

[[ -x "$JDK/bin/java" ]] || { echo "JDK not found at $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }

# ---- build classes on the fly (never committed; .gitignore covers this) ----
CLASSES="$ROOT/bench/batch/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/java/PaperNativeAquiferIndexStride.java \
  "$ROOT"/bench/batch/java/PaperNativeStaticCacheGet.java \
  "$ROOT"/bench/batch/java/BatchFloorBench.java

# ---- resolve arm libs: use --before/--after if given, else build worktrees ----
build_arm() { # $1 = arm name, $2 = commit (empty for master)
  local arm="$1" commit="${2:-}"
  local wt="/tmp/w-batch-$arm"
  rm -rf "$wt"
  if [[ -n "$commit" ]]; then
    (cd "$ROOT" && git worktree add --detach "$wt" "$commit") >/dev/null 2>&1
  else
    (cd "$ROOT" && git worktree add --detach "$wt" origin/master) >/dev/null 2>&1
  fi
  (cd "$wt" && cargo build --release 2>&1 | tail -1)
  echo "$wt/target/release/libcrussty.so"
}

if [[ ${#ARMS[@]} -eq 0 ]]; then ARMS=(before after); fi
for arm in "${ARMS[@]}"; do
  var="${arm^^}_LIB"
  if [[ -z "${!var:-}" ]]; then
    if [[ "$arm" == "before" ]]; then build_commit="db7cf27"; else build_commit=""; fi
    # shellcheck disable=SC2116
    lib="$(build_arm "$arm" "$build_commit")"
    printf -v "$var" '%s' "$lib"
  fi
  [[ -f "${!var}" ]] || { echo "arm $arm: lib missing: ${!var}" >&2; exit 2; }
done

mkdir -p "$RES"
RAW="$RES/BATCH_FLOOR_RAW.tsv"
: > "$RAW"

exec 9>"$LOCK"
flock 9
echo "# BENCH.lock acquired $(date -u +%FT%TZ) by agent-7625532f"

for arm in "${ARMS[@]}"; do
  var="${arm^^}_LIB"
  lib="${!var}"
  log="$RES/BATCH_FLOOR_${arm}.log"
  echo "== arm=$arm lib=$lib =="
  # LD_LIBRARY_PATH: libpaper_native_jni.so may carry libjvm DT_NEEDED; the
  # Rust-side dlopen (self_init) resolves through the process scope, this
  # keeps the fallback path robust. CRUSSTY_BATCH_NATIVE_LIB: absolute path
  # for batch_api::self_init (standalone bench mode, no engine).
  LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$JDK/lib/server" \
  CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" \
  "$JDK/bin/java" -Xms256m -Xmx512m -cp "$CLASSES" BatchFloorBench \
      --lib "$lib" --native "$NATIVE_LIB" --kernels "$KERNELS" --sizes 1,8,16,64,256 \
      --rounds 11 --ops 60000 2>&1 | tee "$log"
  grep '^RESULT' "$log" | sed "s/^RESULT/${arm}\t/" >> "$RAW" || true
done

flock -u 9
echo "# BENCH.lock released $(date -u +%FT%TZ)"
echo "# raw rows: $(grep -c . "$RAW" || true) -> $RAW"
