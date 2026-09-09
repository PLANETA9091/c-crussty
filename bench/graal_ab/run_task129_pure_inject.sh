#!/usr/bin/env bash
# TASK-129 — PURE-INJECT canonical config validation probe
# agent-7625532f, 2026-09-09.
# TASK-133 (2026-09-09): verdict tree flipped to gate v2 (TASK-130 §5 pre-registration,
#   binding since TASK-132 first v2-gated run). Legacy v1 kept as dual-report class only
#   (v1_class/v1_final columns). Measurement (R0/R1/R2/R3 sampling, settle, traj) untouched.
#   v2_thresh >= v1_thresh always => GC.run branch still executes on any v2-miss;
# Owner law 14:45+08 (docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md):
#   launch = stock JDK + ONLY -agentpath. ZERO other JVM options.
# Derivative of run_task119_c3_probe.sh skeleton (flock/journal/seed/settle/traj/tree UNCHANGED).
# Flag-free diagnostics only (law §3): jcmd GC.heap_info, GC.class_histogram, /proc smaps_rollup.
# Inject functional parity: crussty-runtime lines in boot.log + 'Done (' wall time.
# REENTRANT: RAWDIR override + results.tsv skip. N=1 default.
# LAWS: local i; rm-FIRST restore; 9>&- ghost-lock; journal pair; foreground-only;
# printf '\t' for TSV (Write tab law); AGENT_ARGS variable.
set -uo pipefail
cd /home/z/c-crussty
JDK_STOCK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="${RAWDIR:-$PWD/bench/graal_ab/RAW_TASK129_PURE_$STAMP}"; mkdir -p "$OUT"
N_RUNS="${N:-1}"
IDLE_RATE=15
IDLE_STREAK=6
REMOVE_SETTLE="${REMOVE_SETTLE:-120}"
BAND="3200 3200 3327 3327"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK129-pure-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK129-pure-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"

rss_of() { awk '/VmRSS/ {printf "%.0f", $2/1024}' "/proc/$1/status" 2>/dev/null || echo NA; }
heap_info() { # $1=pid -> flag-free jcmd GC.heap_info used/committed of the G1 heap line
    "$JDK_STOCK/bin/jcmd" "$1" GC.heap_info 2>/dev/null \
      | awk '/heap / {for(i=1;i<=NF;i++){if($i=="used")u=$(i+1);if($i=="total")t=$(i+1)}; print "used="u" committed="t; exit}' \
      || echo "used=NA committed=NA"
}
smap_rss() { awk '/^Rss:/ {r+=$2} /^Pss:/ {p+=$2} END{printf "smapRss=%.0fMB Pss=%.0fMB", r/1024, p/1024}' "/proc/$1/smaps_rollup" 2>/dev/null || echo "smapRss=NA"; }
# legacy v1 class (dual-report only, non-binding since TASK-133):
over_threshold() { awk -v a="$1" -v b="$2" 'BEGIN{exit !(a > b*1.10)}'; }
# gate v2 (pre-registered docs/C3_GATE_V2_PROTOCOL_2026-09-09.md §5): R2 <= max(R0*1.10, R0+100MB)
#   crossover R0=1000MB: fat baselines bit-identical to v1, lean baselines get 100MB absolute floor
#   = upper edge of NMT-attributed benign residue band (~50-90MB structural C2+metaspace).
over_threshold_v2() { awk -v a="$1" -v b="$2" 'BEGIN{t=b*1.10; f=b+100; exit !(a > ((t>f)?t:f))}'; }
v2_threshold_of() { awk -v b="$1" 'BEGIN{t=b*1.10; f=b+100; printf "%.1f", (t>f)?t:f}'; }

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
    # CANONICAL OWNER LAUNCH: stock JDK + ONLY the inject. No other JVM options.
    ( cd "$SERVER" && exec "$JDK_STOCK/bin/java" $AGENT_ARGS \
        -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then
        log "FATAL: $RID no Done( in 120s (default-heap boot risk realized — boot.log tail below)"
        tail -5 "$RDIR/boot.log" | tee -a "$OUT/run.log"
        kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; return 1
    fi
    log "$RID boot done: $(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1) (pure-inject stock Temurin 21.0.12.1)"
    grep -m1 'crussty-runtime.*loaded' "$RDIR/boot.log" >> "$OUT/run.log" 2>/dev/null || log "WARN: no crussty-runtime marker in boot.log"
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
    local VERDICT R3=NA H3=NA V2T V1_CLASS V1_FINAL="NA"
    V2T=$(v2_threshold_of "$R0")
    if [ "$R2" -gt "$(awk -v b="$R0" 'BEGIN{printf "%.0f", b*1.10}')" ]; then V1_CLASS="exceeds-v1"; else V1_CLASS="within-v1"; fi
    if over_threshold_v2 "$R2" "$R0"; then
        log "$RID R2=${R2}MB > v2 gate ${V2T}MB (max(R0*1.10,R0+100)) -> diagnostic jcmd GC.run"
        "$JDK_STOCK/bin/jcmd" "$SP" GC.run >> "$RDIR/jcmd.log" 2>&1 || true
        sleep 15
        R3=$(rss_of "$SP"); H3=$(heap_info "$SP")
        if over_threshold_v2 "$R3" "$R0"; then VERDICT="FAIL-leak-signature"; else VERDICT="PASS-LAZY"; fi
        if over_threshold "$R3" "$R0"; then V1_FINAL="FAIL"; else V1_FINAL="PASS-LAZY"; fi
    else
        VERDICT="PASS"
    fi
    printf '%s\tR0=%sMB\tR1=%sMB\tR2=%sMB\tR3=%s\tverdict=%s\theap@base=%s\theap@add=%s\theap@remove_end=%s\theap@final=%s\tv2_gate=%sMB\tv1_class=%s\tv1_final=%s\n' \
        "$RID" "$R0" "$R1" "$R2" "$R3" "$VERDICT" "$H0" "$H1" "$H2" "$H3" "$V2T" "$V1_CLASS" "$V1_FINAL" | tee -a "$OUT/results.tsv"
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
log "TASK-129 pure-inject canonical validation complete (owner law 14:45+08; gate v2 binding per TASK-130 §5 — TASK-133 rig flip)"
