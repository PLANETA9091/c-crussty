#!/usr/bin/env bash
# TASK-100 — TASK-96 follow-up bundle (agent-7625532f, 2026-09-09)
# (a) version-confound third arm: GraalVM-build C2 (-XX:-UseJVMCICompiler) vs
#     Temurin C2 — isolates the banked -12.5% JIT variable from the 21.0.2-vs-
#     21.0.12 build delta.
# (b) Graal x native-kernel composability: ARMED module (agentpath runtime +
#     CRUSSTY_NATIVE_IMPROVED_NOISE=1 + CRUSSTY_KERNEL_POLICY=audit) under C2
#     vs under Graal on the same 64-chunk fresh worldgen burst rig.
# Protocol: 5 arms x n=2, position-balanced reverse rep, uniform WARM forceload
# for every arm (armed arms need noise classes loaded before the measured
# burst; arming-evidence gate, loud INVALID abort), per-run rm+untar restore,
# BENCH-MUTEX + journal, foreground-in-call discipline.
# Re-entrant: runs already present in results.tsv are skipped; RUNS="i-j"
# selects an index range so one session fits several tool calls.
set -uo pipefail
cd /home/z/c-crussty
JDK_TEMU=/home/z/jdk21
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK100"; mkdir -p "$OUT"   # STABLE dir: cross-call re-entrancy via results.tsv

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK100-bundle-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK100-bundle-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

N_REPS="${N_REPS:-2}"
IDLE_RATE=15
IDLE_STREAK=6
BURST_TIMEOUT=75
WARM_TIMEOUT=60
ARMING_TIMEOUT=90
# warm region W1 = 3400..3463 (fresh, far from measured); measured = 3200..3327 (TASK-96 region)
WARM_FL="forceload add 3400 3400 3463 3463"
MEAS_FL="forceload add 3200 3200 3327 3327"
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"

SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }

log "JDKs: TEMU=$($JDK_TEMU/bin/java -version 2>&1 | head -1) | GRAAL=$($JDK_GRAAL/bin/java -version 2>&1 | head -1)"

arm_specs() { # balanced 9-arm sequence (7 base + TC2I/TC2P disambiguation); rep2 = reverse
    local rep
    for ((rep=1; rep<=N_REPS; rep++)); do
        if [ $((rep % 2)) -eq 1 ]; then
            for ARM in TC2D GC2D GD TC2AD GAD TC2A GA TC2I TC2P; do echo "$ARM"; done
        else
            for ARM in TC2P TC2I GA TC2A GAD TC2AD GD GC2D TC2D; do echo "$ARM"; done
        fi
    done
}

wait_idle() { # $1=server pid — wait for CPU floor
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

force_fl() { # $1=pid $2=fl-command $3=timeout_s $4=label -> echoes wall seconds
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
    local JDKP FLAGS ARMED AGENT ENVV=""
    case "$ARM" in
        TC2D)  JDKP=$JDK_TEMU;  FLAGS="";                      ARMED=0; AGENT=0 ;;
        GC2D)  JDKP=$JDK_GRAAL; FLAGS="-XX:-UseJVMCICompiler"; ARMED=0; AGENT=0 ;;
        GD)    JDKP=$JDK_GRAAL; FLAGS="-XX:+UseJVMCICompiler"; ARMED=0; AGENT=0 ;;
        TC2AD) JDKP=$JDK_TEMU;  FLAGS="";                      ARMED=0; AGENT=1 ;;
        GAD)   JDKP=$JDK_GRAAL; FLAGS="-XX:+UseJVMCICompiler"; ARMED=0; AGENT=1 ;;
        TC2A)  JDKP=$JDK_TEMU;  FLAGS="";                      ARMED=1; AGENT=1 ;;
        GA)    JDKP=$JDK_GRAAL; FLAGS="-XX:+UseJVMCICompiler"; ARMED=1; AGENT=1 ;;
        TC2I)  JDKP=$JDK_TEMU;  FLAGS="";                      ARMED=2; AGENT=1 ;;
        TC2P)  JDKP=$JDK_TEMU;  FLAGS="";                      ARMED=3; AGENT=1 ;;
        *) log "FATAL: unknown arm $ARM"; return 1 ;;
    esac
    case "$ARMED" in
        1) ENVV="CRUSSTY_NATIVE_IMPROVED_NOISE=1 CRUSSTY_NATIVE_PERLIN_NOISE=1 CRUSSTY_KERNEL_POLICY=audit" ;;
        2) ENVV="CRUSSTY_NATIVE_IMPROVED_NOISE=1 CRUSSTY_KERNEL_POLICY=audit" ;;
        3) ENVV="CRUSSTY_NATIVE_PERLIN_NOISE=1 CRUSSTY_KERNEL_POLICY=audit" ;;
    esac
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    FIFO_GLOBAL="$RDIR/console.fifo"; mkfifo "$FIFO_GLOBAL"
    sleep 3600 3>"$FIFO_GLOBAL" 9>&- & local HOLDER=$!
    ( cd "$SERVER" && exec env $ENVV "$JDKP/bin/java" 9>&- $FLAGS -Xms512M -Xmx2G -XX:+UseG1GC \
        $( [ "$AGENT" = 1 ] && echo "$AGENT_ARGS" ) \
        -jar "$JAR" --nogui <"$FIFO_GLOBAL" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: $RID no Done( in 120s"; kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; return 1; fi
    local BOOT_T; BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
    sleep 8; wait_idle "$SP"
    # uniform warm forceload (triggers worldgen noise classes; armed hook captures/retransforms here)
    local TW; TW=$(force_fl "$SP" "$WARM_FL" "$WARM_TIMEOUT" warm)
    # arming evidence gate for armed arms (per-arm marker set)
    local ARM_NOTE="dormant"
    if [ "$ARMED" != 0 ]; then
        local ok=0 t=0
        while [ "$t" -le "$ARMING_TIMEOUT" ]; do
            local G1=0 G2=0 G3=0
            grep -q 'improved_noise: hook armed, retransform rc=0' "$RDIR/boot.log" 2>/dev/null && G1=1
            grep -q 'self-test passed' "$RDIR/boot.log" 2>/dev/null && G2=1
            grep -q 'kernel-policy: WIRE PerlinNoise.getValueWholeBody' "$RDIR/boot.log" 2>/dev/null && G3=1
            if   [ "$ARMED" = 1 ] && [ "$G1" = 1 ] && [ "$G2" = 1 ] && [ "$G3" = 1 ]; then ok=1
            elif [ "$ARMED" = 2 ] && [ "$G1" = 1 ] && [ "$G2" = 1 ]; then ok=1
            elif [ "$ARMED" = 3 ] && [ "$G3" = 1 ] && [ "$G2" = 1 ]; then ok=1
            fi
            [ "$ok" = 1 ] && break
            sleep 2; t=$((t+2))
        done
        if [ "$ok" != 1 ]; then
            log "INVALID: $RID arming evidence absent after ${ARMING_TIMEOUT}s — LOUD ABORT (no silent armed run)"
            grep -E 'improved_noise:|kernel-policy:' "$RDIR/boot.log" | tail -5 >> "$OUT/run.log" || true
            echo "stop" >&3 2>/dev/null || true
            sleep 3; kill -9 "$SP" 2>/dev/null || true; kill "$HOLDER" 2>/dev/null || true
            restore_seed
            return 1
        fi
        ARM_NOTE="armed-verified(${t}s)"
    fi
    wait_idle "$SP"
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
    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" "$RID" "$ARM" "$BOOT_T" "$ARM_NOTE" "t_warm=${TW}s" "t_burst=${T_BURST}s" "cpu_burst=${CPU_BURST}s" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $BOOT_T $ARM_NOTE t_warm=${TW}s t_burst=${T_BURST}s cpu_burst=${CPU_BURST}s"
}

# build balanced sequence and execute selected range
mapfile -t SEQ < <(arm_specs)
RANGE="${RUNS:-1-${#SEQ[@]}}"
I0=${RANGE%%-*}; I1=${RANGE##*-}
log "TASK-100 bundle begin: ${#SEQ[@]} runs total, executing $I0..$I1"
for ((i=I0; i<=I1; i++)); do
    ARM=${SEQ[$((i-1))]}
    run_one "$ARM" "$i" || true
done

log "=== SUMMARY (partial view) ==="
[ -s "$OUT/results.tsv" ] && column -t -s'      ' "$OUT/results.tsv"
log "TASK-100 bundle range $I0..$I1 complete"
