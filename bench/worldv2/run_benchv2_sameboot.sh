#!/usr/bin/env bash
# run_benchv2_sameboot.sh — SAME-BOOT A/B dual-bench harness (AG-361, wave-527).
# Cert-path enabler for the swarm sigma-census (AG-210/212/227 w527): cross-runner
# A/A sigma_d~12пп >> 2.3пп gate; 94/94 WBP-succ Oct2 = 94 unique runner-ids,
# 0 reuse (ephemeral VMs) => n=1 cross-runner pairs UNJUDGEABLE. The only
# judgeable quantum = same-boot: 1 job = 1 VM = 1 runner_cpu_index (|dIdx|=0 by
# construction), two sequential FULL bench cycles with an env-delta lever.
# Leg A = control (base env). Leg B = AB_LEG_B_VARS ("K=V K=V", space-separated)
# or byte-identical A when AB_NULL=1 (A/A null = harness sanity canary).
# Each leg is a complete run_benchv2.sh cycle in its own BENCH_WORK dir: downloads,
# G-PURPUR md5+sha256 pins, G-KERNEL-DRIFT fail-closed guard, pregen, sustain,
# report — pinned downloads make the two kernels byte-identical across legs
# (anti-drift canon AG-178 w527). Verdict: report_sameboot_ab.py merges both
# BENCHV2.md into BENCHV2_AB.md (prereg: AB-NULL |d|<=10% PASS / <=25% WARN).
set -uo pipefail
log() { echo "[sameboot $(date -u +%H:%M:%S)] $*"; }
REPO_ROOT="$PWD"
: "${AB_LEG_B_VARS:=}"          # e.g. 'DIM_GEN_WINDOW=6144' (space-separated K=V)
AB_NULL="${AB_NULL:-0}"
RUN_A="$REPO_ROOT/runA"; RUN_B="$REPO_ROOT/runB"

# --- leg A (control) ---------------------------------------------------------
log "LEG-A start (base env) BENCH_WORK=$RUN_A"
BENCH_WORK="$RUN_A" bash "$REPO_ROOT/bench/worldv2/run_benchv2.sh"; A_RC=$?
echo "ab_leg=A ab_null=$AB_NULL ab_vars=base" >> "$RUN_A/run-env.txt" 2>/dev/null || true
cp "$RUN_A/server/BENCHV2.md" "$REPO_ROOT/BENCHV2_LEG_A.md" 2>/dev/null || true
log "LEG-A rc=$A_RC"

# --- leg B (lever delta) -----------------------------------------------------
if [ "$AB_NULL" = "1" ]; then
  log "AB_NULL=1: leg-B env byte-identical leg-A (A/A null canary)"
else
  for kv in $AB_LEG_B_VARS; do
    export "${kv%%=*}=${kv#*=}"
    log "LEG-B lever export: ${kv%%=*}=${kv#*=}"
  done
fi
log "LEG-B start (BENCH_WORK=$RUN_B)"
BENCH_WORK="$RUN_B" bash "$REPO_ROOT/bench/worldv2/run_benchv2.sh"; B_RC=$?
echo "ab_leg=B ab_null=$AB_NULL ab_vars=${AB_LEG_B_VARS:-none}" >> "$RUN_B/run-env.txt" 2>/dev/null || true
cp "$RUN_B/server/BENCHV2.md" "$REPO_ROOT/BENCHV2_LEG_B.md" 2>/dev/null || true
log "LEG-B rc=$B_RC"

# --- merge verdict -----------------------------------------------------------
python3 "$REPO_ROOT/bench/worldv2/report_sameboot_ab.py" \
  "$RUN_A/server" "$RUN_B/server" "$REPO_ROOT" "$A_RC" "$B_RC" "$AB_NULL" || true
[ -s "$REPO_ROOT/BENCHV2_AB.md" ]; AB_RC=$?
log "sameboot done: A_RC=$A_RC B_RC=$B_RC AB_RC=$AB_RC"
# fail-closed (dud-gate canon x519): leg gate-fail or missing AB verdict = red job
[ "$A_RC" = "0" ] && [ "$B_RC" = "0" ] && [ "$AB_RC" = "0" ] || exit 1
exit 0
