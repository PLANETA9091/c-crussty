#!/usr/bin/env bash
# TASK-47-w7 batch-rollout A/B bench — CRUSSTY_BATCH paired sweep
# (agent-7625532f).
#
# Arms: CRUSSTY_BATCH unset (= dormant state: no consumer routes through the
# dispatcher; harness measures both arms regardless, the env selects the
# ROUTED arm) vs CRUSSTY_BATCH=on (B.6 "on": force batch at any K; the old
# `CRUSSTY_BATCH=1` phrasing is SUPERSEDED — parse_rollout("1") -> Off,
# test-pinned in batch_api.rs).
# The product code DOES read CRUSSTY_BATCH now (G1 gate, batch_api.rs) —
# the harness implements the B.6 call-site semantics; this script proves
# env-neutrality of the measurement by interleaving A/B/A/B per group.
#
# Coverage: full batch table (ids 0-11, 7 groups) + --controls direct-only
# floor probe (g42/g35/g33/g30 x2/g2 — the shape-coverage gap groups).
#
# Exclusivity: full BENCH.lock (flock 9) for the whole run, like P500.
# Output: bench/batch/results/BATCH_ROLLOUT_{A,B}_{rep}.log + BATCH_ROLLOUT_RAW.tsv

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RES="$ROOT/bench/batch/results"
JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
DISPATCH_LIB="${DISPATCH_LIB:-$ROOT/target/release/libcrussty.so}"
LOCK=/home/z/BENCH.lock
REPS="${REPS:-2}"

[[ -x "$JDK/bin/java" ]] || { echo "JDK not found at $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }
[[ -f "$DISPATCH_LIB" ]] || { echo "dispatch lib missing: $DISPATCH_LIB (cargo build --release)" >&2; exit 2; }

# ---- build classes on the fly (never committed; .gitignore covers this) ----
CLASSES="$ROOT/bench/batch/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/java/BatchFloorBench.java \
  "$ROOT"/bench/batch/java/PaperNativeAquiferIndexStride.java \
  "$ROOT"/bench/batch/java/BatchRolloutBench.java \
  "$ROOT"/bench/batch/java/net/minecraft/world/level/biome/PaperNativeClimateRTree.java \
  "$ROOT"/bench/p500/java/PaperNativeTicketSetSearch.java \
  "$ROOT"/bench/p500/java/PaperNativeChunkDependencies.java \
  "$ROOT"/bench/p500/java/PaperNativeDensitySplineContext.java \
  "$ROOT"/bench/p500/java/PaperNativeEntityLookupStatus.java \
  "$ROOT"/bench/p500/java/PaperNativeNoiseInterpolatorFractions.java \
  "$ROOT"/bench/p500/java/PaperNativeClimateRTree.java \
  "$ROOT"/bench/p500/java/PaperNativeStaticCacheGet.java \
  "$ROOT"/bench/p500/java/PaperNativeRangeChoice.java \
  "$ROOT"/bench/p500/java/PaperNativePluginStartupRollup.java \
  "$ROOT"/bench/p500/java/PaperNativePluginLoadingAllocation.java \
  "$ROOT"/bench/p500/java/PaperNativeAquiferSurfaceSampling.java

mkdir -p "$RES"
RAW="$RES/BATCH_ROLLOUT_RAW.tsv"
: > "$RAW"

exec 9>"$LOCK"
flock 9
echo "# BENCH.lock acquired $(date -u +%FT%TZ) by agent-7625532f (TASK-47-w7)"

run_one() { # $1=arm(A|B) $2=rep $3=mode(control|table)
  local arm="$1" rep="$2" mode="$3"
  local log="$RES/BATCH_ROLLOUT_${arm}${rep}_${mode}.log"
  local env_mode=( )
  if [[ "$arm" == "B" ]]; then env_mode=(CRUSSTY_BATCH=on); fi
  local extra=(); [[ "$mode" == "control" ]] && extra=(--controls)
  env -u CRUSSTY_BATCH "${env_mode[@]}" \
  LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$JDK/lib/server" \
  CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" \
  "$JDK/bin/java" -Xms256m -Xmx512m -cp "$CLASSES" BatchRolloutBench \
      --lib "$DISPATCH_LIB" --native "$NATIVE_LIB" --rounds 7 --ops 40000 \
      "${extra[@]}" 2>&1 | tee "$log"
  grep '^RESULT\|^ROUTE_COST' "$log" | sed "s/^RESULT/${arm}${rep}\t/" >> "$RAW" || true
}

# A/B/A/B per arm-rep: A(rep1) B(rep1) A(rep2) B(rep2) — table sweep first,
# then the direct-only controls (same A/B cadence; B re-measures direct and
# doubles as the env-neutrality check on the non-dispatch path).
for rep in $(seq 1 "$REPS"); do
  for arm in A B; do
    run_one "$arm" "$rep" table
  done
done
for rep in $(seq 1 "$REPS"); do
  for arm in A B; do
    run_one "$arm" "$rep" control
  done
done

flock -u 9
echo "# BENCH.lock released $(date -u +%FT%TZ)"
echo "# raw rows: $(grep -c . "$RAW" || true) -> $RAW"
