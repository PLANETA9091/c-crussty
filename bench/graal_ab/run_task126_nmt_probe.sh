#!/usr/bin/env bash
# TASK-126 — Native-residue attribution probe (TASK-125 pre-registered protocol)
# agent-7625532f, 2026-09-09.
# Derivative of run_task119_c3_probe.sh (TASK-121/125 cadence skeleton UNCHANGED):
#   + -XX:NativeMemoryTracking=summary on the JVM
#   + jcmd VM.native_memory BASELINE at R0 (pre-churn)
#   + jcmd VM.native_memory summary.diff + GC.class_histogram at R2-end
#   + on FAIL path: summary.diff + histogram again after the GC.run leg (R3)
# DIAGNOSTIC-CLASS: NMT overhead is excluded from gate semantics — the <=10%
# reclaim gate stays owned by the un-instrumented cadence rig (TASK-119 rig).
# Attribution tree (TASK125_C3_HEALTH_CADENCE2 doc):
#   committed-heap-not-uncommitted  => G1PeriodicGCInterval flag A/B candidate
#   JVMCI/code-cache growth         => benign capacity, close
#   agent-arena growth              => twin escalation (shared .so)
# REENTRANT: RAWDIR override skips done runs. N=2 default.
# LAWS: local i; rm-FIRST restore; 9>&- ghost-lock; journal pair;
# AGENT_ARGS variable; foreground-only; printf '\t' for TSV (Write tab law).
set -uo pipefail
cd /home/z/c-crussty
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="${RAWDIR:-$PWD/bench/graal_ab/RAW_TASK126_NMT_$STAMP}"; mkdir -p "$OUT"
N_RUNS="${N:-2}"
IDLE_RATE=15
IDLE_STREAK=6
REMOVE_SETTLE="${REMOVE_SETTLE:-120}"
BAND="3200 3200 3327 3327"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK126-nmt-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK126-nmt-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"

rss_of() { awk '/VmRSS/ {printf "%.0f", $2/1024}' "/proc/$1/status" 2>/dev/null || echo NA; }
lastheap() { grep -o '[0-9]*M->[0-9]*M([0-9]*M)' "$1" 2>/dev/null | tail -1 || echo none; }
over_threshold() { awk -v a="$1" -v b="$2" 'BEGIN{exit !(a > b*1.10)}'; }

wait_settle() { # $1=pid -> waits until rolling cpu delta idle, cap $2s
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
    local IDX="$1" RID="R$1" i
    if [ -f "$OUT/results.tsv" ] && awk -F'\t' -v r="$RID" '$1==r{f=1} END{exit !f}' "$OUT/results.tsv"; then
        log "skip $RID (already done — reentrant)"; return 0
    fi
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    local FIFO="$RDIR/console.fifo"; mkfifo "$FIFO"
    sleep 3600 3>"$FIFO" 9>&- & local HOLDER=$!
    ( cd "$SERVER" && exec "$JDK_GRAAL/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
        -XX:NativeMemoryTracking=summary $EXTRA_JVM_FLAGS \
        -XX:+UseJVMCICompiler "-Xlog:gc:file=$RDIR/gc.log" 9>&- $AGENT_ARGS \
        -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: $RID no Done( in 120s"; kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; return 1; fi
    log "$RID boot done: $(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)"
    wait_settle "$SP" 60
    local R0=$(rss_of "$SP") H0=$(lastheap "$RDIR/gc.log")
    "$JDK_GRAAL/bin/jcmd" "$SP" VM.native_memory baseline > "$RDIR/jcmd_baseline.log" 2>&1 || true
    log "$RID baseline: R0=${R0}MB heap_last=$H0 nmt_baseline=$([ -s "$RDIR/jcmd_baseline.log" ] && echo ok || echo failed)"

    exec 3>"$FIFO"
    echo "forceload add $BAND" >&3
    wait_settle "$SP" 90
    local R1=$(rss_of "$SP") H1=$(lastheap "$RDIR/gc.log")
    log "$RID post-add: R1=${R1}MB heap_last=$H1"

    echo "forceload remove $BAND" >&3
    local t
    for t in $(seq 0 10 "$REMOVE_SETTLE"); do
        echo "$(date +%H:%M:%S) $(rss_of "$SP")" >> "$RDIR/rss_after_remove.traj"
        sleep 10
    done
    local R2=$(rss_of "$SP") H2=$(lastheap "$RDIR/gc.log")
    # diagnostics at R2-end (NMT diff + histogram) — best-effort
    "$JDK_GRAAL/bin/jcmd" "$SP" VM.native_memory summary.diff > "$RDIR/nmt_diff_R2.txt" 2>&1 || true
    "$JDK_GRAAL/bin/jcmd" "$SP" GC.class_histogram > "$RDIR/hist_R2.txt" 2>&1 || true
    local VERDICT R3=NA H3=NA
    if over_threshold "$R2" "$R0"; then
        log "$RID R2=${R2}MB > R0*1.10 -> diagnostic jcmd GC.run"
        "$JDK_GRAAL/bin/jcmd" "$SP" GC.run >> "$RDIR/jcmd.log" 2>&1 || true
        sleep 15
        R3=$(rss_of "$SP"); H3=$(lastheap "$RDIR/gc.log")
        "$JDK_GRAAL/bin/jcmd" "$SP" VM.native_memory summary.diff > "$RDIR/nmt_diff_R3.txt" 2>&1 || true
        "$JDK_GRAAL/bin/jcmd" "$SP" GC.class_histogram > "$RDIR/hist_R3.txt" 2>&1 || true
        if over_threshold "$R3" "$R0"; then VERDICT="FAIL-leak-signature"; else VERDICT="PASS-LAZY"; fi
    else
        VERDICT="PASS"
    fi
    printf '%s\tR0=%sMB\tR1=%sMB\tR2=%sMB\tR3=%s\tverdict=%s\theap@add=%s\theap@remove_end=%s\theap@final=%s\n' \
        "$RID" "$R0" "$R1" "$R2" "$R3" "$VERDICT" "$H1" "$H2" "$H3" | tee -a "$OUT/results.tsv"
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
log "TASK-126 NMT attribution probe complete (diagnostic-class; gate owned by task119 rig)"
