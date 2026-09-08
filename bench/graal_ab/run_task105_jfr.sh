#!/usr/bin/env bash
# TASK-105 — JFR warm-burst mechanism diff: ARMED vs DORMANT (agent-7625532f)
# TASK-100 NEXT#1: confirm the +35% warm-burst armed-regression mechanism.
# Protocol: 2 arms x n=2, position-balanced (rep2 reversed), uniform WARM
# forceload, arming-evidence gate, per-run rm+untar restore, BENCH-MUTEX +
# journal, foreground-in-call. JFR = DYNAMIC jcmd JFR.start scoped to the
# measured burst window only (no boot/warm profile pollution); identical
# settings=profile both arms -> differential attribution valid. CPU/wall
# numbers are JFR-inflated vs TASK-100 baselines by design (not comparable
# across harnesses; within-harness comparison is the evidence).
# PRE-REGISTERED MECHANISM GATES (before any run; attribution evidence,
# n=2/arm sufficient for mechanism, NOT for effect-size banking):
#  MECH-CONFIRMED if >=2 of 3 hold in BOTH armed reps vs BOTH dormant reps:
#   (1) BRIDGE EXPOSURE: armed profiles show >=5% ExecutionSamples with a
#       crussty/JNI/MethodHandle/map-lookup frame in top-6 stack AND noise
#       leaf frames as separate top frames (non-inlined); dormant noise
#       frames rare/absent from top-6 (inlined into worldgen caller).
#   (2) ALLOCATION: armed TLAB-event rate >=1.2x dormant (events/burst-s)
#       or GC-pause count in window armed > dormant.
#   (3) PROFILE DIVERGENCE: armed-only hot entries in bridge path among
#       top-25 (rank-shift or exclusive).
#  Else: mechanism NOT established — honest report of what showed.
# Re-entrant: results.tsv skip; RUNS="i-j" fits 10-min tool-call budget.
set -uo pipefail
cd /home/z/c-crussty
JDK_TEMU=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK105"; mkdir -p "$OUT"

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
[ -x "$JDK_TEMU/bin/jcmd" ] || { echo "FATAL: jcmd missing"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK105-jfr-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK105-jfr-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

N_REPS="${N_REPS:-2}"
IDLE_RATE=15
IDLE_STREAK=6
BURST_TIMEOUT=75
WARM_TIMEOUT=60
ARMING_TIMEOUT=90
WARM_FL="forceload add 3400 3400 3463 3463"
MEAS_FL="forceload add 3200 3200 3327 3327"
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
DORMANT_ENV=""  # exact TASK-100 dormant state: no CRUSSTY_* env (default runtime behavior)
ARMED_ENV="CRUSSTY_NATIVE_IMPROVED_NOISE=1 CRUSSTY_NATIVE_PERLIN_NOISE=1 CRUSSTY_KERNEL_POLICY=audit"

SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }

log "JDK: $($JDK_TEMU/bin/java -version 2>&1 | head -1)"

arm_specs() { # rep1: dormant,armed ; rep2: armed,dormant (position-balanced)
    local rep
    for ((rep=1; rep<=N_REPS; rep++)); do
        if [ $((rep % 2)) -eq 1 ]; then echo JFRAD; echo JFRA;
        else echo JFRA; echo JFRAD; fi
    done
}

wait_idle() {
    local SP=$1 i J1 J2 D S
    for i in $(seq 1 60); do
        J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        S=0
        for i in $(seq 1 6); do
            sleep 0.5
            J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
            D=$(( J2 - J1 )); J1=$J2
            [ "$D" -le "$IDLE_RATE" ] && S=$((S+1)) || S=0
        done
        [ "$S" -ge 4 ] && return 0
    done
    return 0
}

force_fl() {
    local SP=$1 FL="$2" TMO=$3 LBL="$4" T0 T1 i J1 J2 D FIN=0
    T0=$(date +%s.%N)
    exec 3>"$FIFO_GLOBAL"
    echo "$FL" >&3
    J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
    FIN=0
    for i in $(seq 1 $((TMO*2))); do
        sleep 0.5
        J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$J1")
        D=$(( J2 - J1 )); J1=$J2
        if [ "$D" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        [ "$FIN" -ge "$IDLE_STREAK" ] && break
    done
    T1=$(date +%s.%N)
    echo "$T0 $T1" | awk '{printf "%.1f", $2-$1}'
}

FIFO_GLOBAL=""
run_one() { # $1=ARM $2=idx
    local ARM="$1" IDX="$2" RID="${1}${2}" i
    awk -F'\t' -v r="$RID" '$1==r{found=1} END{exit !found}' "$OUT/results.tsv" 2>/dev/null && { log "skip $RID (already done)"; return 0; }
    local ENVV ARMED
    if [ "$ARM" = "JFRAD" ]; then ENVV="$DORMANT_ENV"; ARMED=0;
    elif [ "$ARM" = "JFRA" ]; then ENVV="$ARMED_ENV"; ARMED=1;
    else log "FATAL: unknown arm $ARM"; return 1; fi
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    FIFO_GLOBAL="$RDIR/console.fifo"; mkfifo "$FIFO_GLOBAL"
    sleep 3600 3>"$FIFO_GLOBAL" 9>&- & local HOLDER=$!
    ( cd "$SERVER" && exec env $ENVV "$JDK_TEMU/bin/java" 9>&- -Xms512M -Xmx2G -XX:+UseG1GC \
        $AGENT_ARGS \
        -jar "$JAR" --nogui <"$FIFO_GLOBAL" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: $RID no Done( in 120s"; kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; return 1; fi
    local BOOT_T; BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
    sleep 8; wait_idle "$SP"
    # uniform warm forceload (noise classes load; armed hook retransforms here)
    local TW; TW=$(force_fl "$SP" "$WARM_FL" "$WARM_TIMEOUT" warm)
    # arming evidence gate (armed arms) — LOUD ABORT if absent
    local ARM_NOTE="dormant"
    if [ "$ARMED" = 1 ]; then
        local ok=0 t=0
        while [ "$t" -le "$ARMING_TIMEOUT" ]; do
            local G1=0 G2=0 G3=0
            grep -q 'improved_noise: hook armed, retransform rc=0' "$RDIR/boot.log" 2>/dev/null && G1=1
            grep -q 'self-test passed' "$RDIR/boot.log" 2>/dev/null && G2=1
            grep -q 'kernel-policy: WIRE PerlinNoise.getValueWholeBody' "$RDIR/boot.log" 2>/dev/null && G3=1
            [ "$G1" = 1 ] && [ "$G2" = 1 ] && [ "$G3" = 1 ] && { ok=1; break; }
            sleep 2; t=$((t+2))
        done
        if [ "$ok" != 1 ]; then
            log "INVALID: $RID arming evidence absent after ${ARMING_TIMEOUT}s — LOUD ABORT"
            grep -E 'improved_noise:|kernel-policy:' "$RDIR/boot.log" | tail -5 >> "$OUT/run.log" || true
            echo "stop" >&3 2>/dev/null || true
            sleep 3; kill -9 "$SP" 2>/dev/null || true; kill "$HOLDER" 2>/dev/null || true
            restore_seed; return 1
        fi
        ARM_NOTE="armed-verified(${t}s)"
    fi
    wait_idle "$SP"
    # JFR scoped to burst window: start recording immediately before the burst
    "$JDK_TEMU/bin/jcmd" "$SP" JFR.start name=t105 settings=profile >"$RDIR/jfr_start.out" 2>&1
    grep -q 'Started recording' "$RDIR/jfr_start.out" || log "WARN: $RID JFR.start output: $(cat "$RDIR/jfr_start.out" | head -2 | tr '\n' ' ')"
    # measured burst on fresh region
    local T0=$(date +%s.%N) J0 JPREV
    J0=$(awk '{print $14+$15}' "/proc/$SP/stat")
    JPREV=$J0
    local FIN=0
    exec 3>"$FIFO_GLOBAL"
    echo "$MEAS_FL" >&3
    for i in $(seq 1 $((BURST_TIMEOUT*2))); do
        sleep 0.5
        local JN=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JPREV")
        local D2=$(( JN - JPREV )); JPREV=$JN
        if [ "$D2" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        [ "$FIN" -ge "$IDLE_STREAK" ] && break
    done
    local T1=$(date +%s.%N) JEND
    JEND=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$J0")
    local T_BURST=$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}')
    local CPU_BURST=$(echo "$JEND $J0" | awk '{printf "%.1f", ($1-$2)/100.0}')
    "$JDK_TEMU/bin/jcmd" "$SP" JFR.dump name=t105 filename="$RDIR/burst.jfr" >"$RDIR/jfr_dump.out" 2>&1
    "$JDK_TEMU/bin/jcmd" "$SP" JFR.stop name=t105 >>"$RDIR/jfr_dump.out" 2>&1
    [ -s "$RDIR/burst.jfr" ] || log "WARN: $RID burst.jfr missing/empty: $(head -2 "$RDIR/jfr_dump.out" | tr '\n' ' ')"
    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" "$RID" "$ARM" "$BOOT_T" "$ARM_NOTE" "t_warm=${TW}s" "t_burst=${T_BURST}s" "cpu_burst=${CPU_BURST}s" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $BOOT_T $ARM_NOTE t_warm=${TW}s t_burst=${T_BURST}s cpu_burst=${CPU_BURST}s jfr=$([ -s "$RDIR/burst.jfr" ] && echo ok || echo MISSING)"
}

mapfile -t SEQ < <(arm_specs)
RANGE="${RUNS:-1-${#SEQ[@]}}"
I0=${RANGE%%-*}; I1=${RANGE##*-}
log "TASK-105 JFR diff begin: ${#SEQ[@]} runs total, executing $I0..$I1"
for ((i=I0; i<=I1; i++)); do
    ARM=${SEQ[$((i-1))]}
    run_one "$ARM" "$i" || true
done

log "=== SUMMARY (partial view) ==="
[ -s "$OUT/results.tsv" ] && cat "$OUT/results.tsv"
log "TASK-105 JFR diff range $I0..$I1 complete"
