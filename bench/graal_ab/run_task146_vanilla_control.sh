#!/usr/bin/env bash
# TASK-146 — VANILLA CONTROL (old-kernel A/B side, P500-style per owner standing instruction)
# agent-7625532f, 2026-09-09.
# DERIVED COPY of bench/graal_ab/run_task129_pure_inject.sh (canonical, drift-guard 8ba2473c…)
# with EXACTLY TWO changes (canonical rig bytes UNTOUCHED):
#   (1) AGENT_ARGS empty — launch = stock JDK + NOTHING (old kernel, NO inject).
#       This is a CONTROL measurement, not a product run; the owner INJECTS-ONLY law
#       (docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md) governs PRODUCT launches, untouched.
#   (2) OUT dir RAW_VANILLA_CONTROL_* ; verdict=CONTROL (v2 gate is defined for agent runs
#       only — informational gate reading reported as v2_gate_ref, NON-binding).
# Pre-registered DIFFERENCE (TASK-146 claim 3e5d9cc): UNCONDITIONAL diagnostic GC.run at
#   R2+15s — the vanilla post-full-GC live set is the primary anchor (compare vs agent
#   248.5 / 249.4 / 250.0 MB).
# Same: seed restore, fifo launch, wait_settle, BAND, legs R0/R1/R2, traj, histogram,
#       BENCH.lock flock + journal, hs_err counting. N=1.
set -uo pipefail
cd /home/z/c-crussty
JDK_STOCK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="${RAWDIR:-$PWD/bench/graal_ab/RAW_VANILLA_CONTROL_$STAMP}"; mkdir -p "$OUT"
N_RUNS="${N:-1}"
IDLE_RATE=15
IDLE_STREAK=6
REMOVE_SETTLE="${REMOVE_SETTLE:-120}"
BAND="3200 3200 3327 3327"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-vanilla-control-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-vanilla-control-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed
# CHANGE (1): no inject — old kernel control launch
AGENT_ARGS=""

rss_of() { awk '/VmRSS/ {printf "%.0f", $2/1024}' "/proc/$1/status" 2>/dev/null || echo NA; }
heap_info() {
    "$JDK_STOCK/bin/jcmd" "$1" GC.heap_info 2>/dev/null \
      | awk '/heap / {for(i=1;i<=NF;i++){if($i=="used")u=$(i+1);if($i=="total")t=$(i+1)}; print "used="u" committed="t; exit}' \
      || echo "used=NA committed=NA"
}
smap_rss() { awk '/^Rss:/ {r+=$2} /^Pss:/ {p+=$2} END{printf "smapRss=%.0fMB Pss=%.0fMB", r/1024, p/1024}' "/proc/$1/smaps_rollup" 2>/dev/null || echo "smapRss=NA"; }
v2_threshold_of() { awk -v b="$1" 'BEGIN{t=b*1.10; f=b+100; printf "%.1f", (t>f)?t:f}'; }

wait_settle() {
    local P="$1" CAP="$2" i S=0 J1 J2 D
    for i in $(seq 1 $((CAP*2))); do
        J1=$(awk '{print $14+$15}' "/proc/$P/stat" 2>/dev/null || echo 0)
        sleep 0.5
        J2=$(awk '{print $14+$15}' "/proc/$P/stat" 2>/dev/null || echo 0)
        D=$(( J2 - J1 ))
        if [ "$D" -le "$IDLE_RATE" ]; then S=$((S+1)); else S=0; fi
        [ "$S" -ge "$IDLE_STREAK" ] && return 0
    done
    return 0
}

run_one() { # $1=idx
    local IDX="$1" RID="V$1" i
    if [ -f "$OUT/results.tsv" ] && awk -F'\t' -v r="$RID" '$1==r{f=1} END{exit !f}' "$OUT/results.tsv"; then
        log "skip $RID (already done — reentrant)"; return 0
    fi
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    local FIFO="$RDIR/console.fifo"; mkfifo "$FIFO"
    sleep 3600 3>"$FIFO" 9>&- & local HOLDER=$!
    # OLD-KERNEL CONTROL LAUNCH: stock JDK, NO agent, NO options (change 1)
    ( cd "$SERVER" && exec "$JDK_STOCK/bin/java" $AGENT_ARGS \
        -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then
        log "FATAL: $RID no Done( in 120s (boot.log tail below)"
        tail -5 "$RDIR/boot.log" | tee -a "$OUT/run.log"
        kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; return 1
    fi
    log "$RID boot done: $(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1) (VANILLA CONTROL — stock kernel, no inject)"
    grep -m1 'crussty-runtime' "$RDIR/boot.log" >> "$OUT/run.log" 2>/dev/null && log "WARN: crussty marker present in VANILLA boot?!" || log "confirm: no crussty-runtime marker (clean old-kernel launch)"
    wait_settle "$SP" 60
    local R0=$(rss_of "$SP") H0=$(heap_info "$SP") S0=$(smap_rss "$SP")
    log "$RID baseline: R0=${R0}MB $H0 $S0"

    exec 3>"$FIFO"
    echo "forceload add $BAND" >&3
    wait_settle "$SP" 90
    local R1=$(rss_of "$SP") H1=$(heap_info "$SP")
    log "$RID post-add: R1=${R1}MB $H1"

    echo "forceload remove $BAND" >&3
    local t
    for t in $(seq 0 10 "$REMOVE_SETTLE"); do
        echo "$(date +%H:%M:%S) $(rss_of "$SP")" >> "$RDIR/rss_after_remove.traj"
        sleep 10
    done
    local R2=$(rss_of "$SP") H2=$(heap_info "$SP") S2=$(smap_rss "$SP")
    "$JDK_STOCK/bin/jcmd" "$SP" GC.class_histogram > "$RDIR/hist_R2.txt" 2>&1 || true
    # CHANGE (2): verdict=CONTROL + UNCONDITIONAL diagnostic GC.run (pre-registered)
    local VERDICT="CONTROL" R3 H3
    local V2TREF=$(v2_threshold_of "$R0")
    log "$RID R2=${R2}MB (control — informational v2 ref ${V2TREF}MB, non-binding) -> UNCONDITIONAL diagnostic jcmd GC.run (pre-registered)"
    "$JDK_STOCK/bin/jcmd" "$SP" GC.run >> "$RDIR/jcmd.log" 2>&1 || true
    sleep 15
    R3=$(rss_of "$SP"); H3=$(heap_info "$SP")
    printf '%s\tR0=%sMB\tR1=%sMB\tR2=%sMB\tR3=%s\tverdict=%s\theap@base=%s\theap@add=%s\theap@remove_end=%s\theap@final=%s\tv2_gate_ref=%sMB\tnote=vanilla-no-inject\n' \
        "$RID" "$R0" "$R1" "$R2" "$R3" "$VERDICT" "$H0" "$H1" "$H2" "$H3" "$V2TREF" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $VERDICT (R0=$R0 R1=$R1 R2=$R2 R3=$R3)"
}

i=0
while [ $((i+1)) -le "$N_RUNS" ]; do i=$((i+1)); run_one "$i" || true; done

HSC=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
log "hs_err delta: $((HSC - HS0)) (must be 0)"
log "=== SUMMARY ==="
awk -F'\t' 'NR>0 {print}' "$OUT/results.tsv" 2>/dev/null | tee -a "$OUT/run.log"
log "TASK-146 vanilla control complete (old-kernel A/B side; canonical rig 8ba2473c untouched; product lane law INJECTS-ONLY untouched)"
