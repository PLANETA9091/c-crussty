#!/usr/bin/env bash
# SAME-BOOT A/B PAIR-HARNESS (AG-377 w527).
#
# Recipe AG-210 w527 ("same-boot = только 2-бенч-в-1-job: 1 VM, 1 download,
# boots подряд") + cert law AG-164/AG-212 (кросс-раннер пары несудимы,
# sigma_d ~12пп >> гейт 2.3пп; серт = same-boot min-of-3).
#
# ONE job = ONE same-boot pair:
#   legA = LEVER_FLAG_A armed (empty => same-boot A/A sigma pair)
#   legB = vanilla control (LEVER_FLAG/LEVER_ARG forced empty)
# One VM, one build (.so shared), boots back-to-back; world re-downloaded from
# the SAME URL per leg => world_sha256 bit-equal (S7-96d pairing, AG-207).
# Legs differ ONLY in CRUSSTY_LEVER_FLAG/CRUSSTY_LEVER_ARG; the rest of the
# env vector is the frozen bank canon exported by world-bench-ab.yml.
#
# Output: $AB_ROOT/legA/, $AB_ROOT/legB/ (full world3-run layout each, since
# run_world3.sh is WORK-parametric and never mutated) + $AB_ROOT/PAIR-VERDICT.txt
# (tail(<20) TPS medians per leg, delta %, world-sha pair-eq, fixture gates,
# per-leg runner_cpu_index from run-env.txt).
#
# CERT RULE: one pair certifies nothing — verdict is a measurement quantum;
# merge gates need min-of-3 pairs (3 dispatches of this workflow).
set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
AB_ROOT="${AB_ROOT:-$PWD/ab-run}"
LEVER_A="${LEVER_FLAG_A:-}"
LEVER_ARG_A="${LEVER_ARG_A:-}"
mkdir -p "$AB_ROOT"

leg_verdict() { # $1=leg dir -> echoes "sha idx valid fixture"
  local w="$1" sha idx fx val
  sha="$(sed -n 's/^world_sha256: \([0-9a-f]\{8\}\).*/\1/p' "$w/run-env.txt" 2>/dev/null | head -1)"
  idx="$(sed -n 's/^runner_cpu_index: \([0-9]\{4,\}\).*/\1/p' "$w/run-env.txt" 2>/dev/null | head -1)"
  if grep -q "FIXTURE-VALIDITY: VALID" "$w/BOTTLENECKS_3.md" 2>/dev/null; then fx=1; else fx=0; fi
  if [ "${POPULATION_TARGET:-0}" != "0" ]; then
    val=0
    grep -q "POPULATION INJECT DONE" "$w/server-stdout.log" 2>/dev/null && \
    grep -q "POPULATION FIXTURE-VALIDITY: VALID" "$w/server-stdout.log" 2>/dev/null && val=1
    [ "$val" = "1" ] || fx=0
  fi
  echo "${sha:-none} ${idx:-none} $fx"
}

leg_tps() { # $1=leg dir -> tail(<20) median of first-of-window TPS polls
  python3 - "$1/BOTTLENECKS_3.md" <<'PY'
import re, sys, statistics
try:
    t = open(sys.argv[1], encoding='utf-8', errors='replace').read()
    m = re.search(r'first-of-window values: \[([^\]]*)\]', t)
    if not m:
        print('none'); raise SystemExit
    vals = [float(x) for x in m.group(1).replace(',', ' ').split()]
    tail = [v for v in vals if v < 20.0] or vals
    print(f'{statistics.median(tail):.3f}' if tail else 'none')
except Exception:
    print('none')
PY
}

run_leg() { # $1=leg id  $2=lever  $3=lever_arg
  local id="$1" work="$AB_ROOT/leg$1" t0
  echo "=== SAME-BOOT leg$id lever='$2' arg='$3' work=$work $(date -u +%FT%TZ) ==="
  rm -rf "$work"; mkdir -p "$work/server/modules/crussty"
  cp "$RUNNER_TEMP/libcrussty.so" "$work/server/modules/crussty/" \
    || { echo "PAIR-FATAL: module .so missing"; return 1; }
  touch "$work/server/modules/crussty/.built"
  t0=$(date +%s)
  WORK="$work" LEVER_FLAG="$2" LEVER_ARG="$3" \
    bash "$SCRIPT_DIR/run_world3.sh" || echo "WARN: leg$id run_world3 rc=$? (pair continues)"
  echo "leg$id wall: $(( $(date +%s) - t0 ))s"
}

run_leg A "$LEVER_A" "$LEVER_ARG_A"
run_leg B "" ""

AV="$(leg_verdict "$AB_ROOT/legA")"; BV="$(leg_verdict "$AB_ROOT/legB")"
AT="$(leg_tps "$AB_ROOT/legA")";     BT="$(leg_tps "$AB_ROOT/legB")"
SHA_A="${AV%% *}"; SHA_B="${BV%% *}"
DELTA="none"
if [ "$AT" != "none" ] && [ "$BT" != "none" ]; then
  DELTA="$(python3 -c "a=$AT; b=$BT; print(f'{(b-a)/a*100:+.2f}' if a else 'none')" 2>/dev/null || echo none)"
fi

{
  echo "SAME-BOOT A/B PAIR-VERDICT $(date -u +%FT%TZ)"
  echo "mode: $([ -z "$LEVER_A" ] && echo 'A/A sigma-pair (lever empty)' || echo "A/B lever=$LEVER_A arg=$LEVER_ARG_A")"
  echo "legA(lever): sha=$SHA_A runner_idx=$(echo $AV | awk '{print $2}') fixture_valid=$(echo $AV | awk '{print $3}') tps_med_tail20=$AT"
  echo "legB(vanilla): sha=$SHA_B runner_idx=$(echo $BV | awk '{print $2}') fixture_valid=$(echo $BV | awk '{print $3}') tps_med_tail20=$BT"
  echo "world_sha_pair_eq: $([ -n "$SHA_A" ] && [ "$SHA_A" = "$SHA_B" ] && echo YES || echo NO)"
  echo "pair_delta_pct(legB vs legA): $DELTA"
  echo "cert: min-of-3 same-boot pairs required (AG-164/AG-212); single pair = measurement quantum"
} | tee "$AB_ROOT/PAIR-VERDICT.txt"

[ "$SHA_A" = "$SHA_B" ] && [ "${AV##* }" = "1" ] && [ "${BV##* }" = "1" ] && [ "$AT" != "none" ] && [ "$BT" != "none" ]
