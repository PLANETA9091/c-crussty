#!/usr/bin/env bash
# TASK-108 v3 phase-3 step 2 — THE FINAL NOISE-CHANNEL A/B (agent-7625532f).
# Pre-registered gates (TASK-105 law, unchanged through PROGRESS-1..7):
#   warm AND cold both measured, n>=3 within-session ABBA pairs,
#   bit-exact output gate ALREADY GREEN (PROBE PASS-ALL + V3_SHADOW PASS).
#   GO  iff warm median cpu_burst delta <= -3% AND cold delta not-negative
#           (<= 0%) AND full separation in warm (max(armed) < min(dormant)).
#   else honest NULL -> the noise channel CLOSES PERMANENTLY.
# Regimes (REGIME=warm|cold):
#   warm: boot -> idle gate -> WARM forceload 3400 band (JIT warmup, disjoint
#         from measured) -> idle gate -> MEASURED burst on fresh 3200 band.
#   cold: boot -> idle gate -> MEASURED burst on fresh 3200 directly
#         (noise classes + JIT compile inside the burst, TASK-106 semantics).
# Arms: CD = dormant (NO CRUSSTY_* env, exact TASK-100 dormant state),
#       CA = armed (CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off,
#       NO shadow — diagnostic re-advance forbidden in A/B by law).
# Primary metric: cpu_burst (child utime+stime jiffies delta over burst).
# Secondary: t_burst wall. Per-run rm+untar seed restore; 9>&- flock-fd law;
# BENCH-MUTEX + journal; armed post-hoc evidence gate ABORTS the run LOUDLY.
# Re-entrant: results.tsv skip; RUNS="i-j" fits tool-call budgets.
set -uo pipefail
cd /home/z/c-crussty
JDK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
REGIME="${REGIME:-warm}"
N_REPS="${N_REPS:-3}"
IDLE_RATE=15
IDLE_STREAK=6
BURST_TIMEOUT=240
WARM_TIMEOUT=120
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK108_V3_AB/$REGIME"; mkdir -p "$OUT"

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK108-v3-ab-$REGIME-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK108-v3-ab-$REGIME-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

WARM_FL="forceload add 3400 3400 3463 3463"
MEAS_FL="forceload add 3200 3200 3327 3327"
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
DORMANT_ENV=""   # exact dormant state: no CRUSSTY_* env
ARMED_ENV="CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off"

SEED="$SERVER/world_graal_seed.tar.gz"
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed

log "TASK-108 v3 FINAL A/B regime=$REGIME n=$N_REPS/arm; JDK: $($JDK/bin/java -version 2>&1 | head -1)"

arm_specs() { # rep odd: dormant,armed ; rep even: armed,dormant (position-balanced ABBA)
    local rep
    for ((rep=1; rep<=N_REPS; rep++)); do
        if [ $((rep % 2)) -eq 1 ]; then echo CD; echo CA;
        else echo CA; echo CD; fi
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

burst_fl() { # $1=pid $2=forceload $3=timeout -> echoes "t_burst cpu_burst"
    local SP=$1 FL="$2" TMO=$3 i J0 JP JN D2 FIN=0 T0 T1 JEND
    T0=$(date +%s.%N)
    exec 3>"$FIFO_GLOBAL"
    echo "$FL" >&3
    J0=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
    JP=$J0
    for i in $(seq 1 $((TMO*2))); do
        sleep 0.5
        JN=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JP")
        D2=$(( JN - JP )); JP=$JN
        if [ "$D2" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        [ "$FIN" -ge "$IDLE_STREAK" ] && break
    done
    T1=$(date +%s.%N)
    JEND=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JP")
    echo "$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}') $(echo "$JEND $J0" | awk '{printf "%.1f", ($1-$2)/100.0}')"
}

FIFO_GLOBAL=""
run_one() { # $1=ARM $2=idx
    local ARM="$1" IDX="$2" RID="${1}${2}" i
    awk -F'\t' -v r="$RID" '$1==r{found=1} END{exit !found}' "$OUT/results.tsv" 2>/dev/null && { log "skip $RID (already done)"; return 0; }
    local ENVV ARMED
    if [ "$ARM" = "CD" ]; then ENVV="$DORMANT_ENV"; ARMED=0;
    elif [ "$ARM" = "CA" ]; then ENVV="$ARMED_ENV"; ARMED=1;
    else log "FATAL: unknown arm $ARM"; return 1; fi
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    FIFO_GLOBAL="$RDIR/console.fifo"; mkfifo "$FIFO_GLOBAL"
    sleep 3600 3>"$FIFO_GLOBAL" 9>&- & local HOLDER=$!
    ( cd "$SERVER" && exec env $ENVV "$JDK/bin/java" 9>&- -Xms512M -Xmx2G -XX:+UseG1GC \
        $AGENT_ARGS -jar "$JAR" --nogui <"$FIFO_GLOBAL" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then
        log "FATAL: $RID no Done( in 120s"; tail -15 "$RDIR/boot.log" >> "$OUT/run.log"
        kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; return 1
    fi
    local BOOT_T; BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
    sleep 8; wait_idle "$SP"
    # WARM regime only: warm forceload on the disjoint 3400 band
    local TW="cold(none)"
    if [ "$REGIME" = "warm" ]; then
        local W=$(burst_fl "$SP" "$WARM_FL" "$WARM_TIMEOUT")
        TW=$(echo "$W" | awk '{print $1}')
        log "$RID warm band done t=${TW}s"
    fi
    # measured burst on the fresh 3200 band
    local RES=$(burst_fl "$SP" "$MEAS_FL" "$BURST_TIMEOUT")
    local T_BURST=$(echo "$RES" | awk '{print $1}')
    local CPU_BURST=$(echo "$RES" | awk '{print $2}')
    # arming evidence, post-hoc (armed runs only) — LOUD abort if absent
    local ARM_NOTE="dormant"
    if [ "$ARMED" = 1 ]; then
        local G1=0 G2=0 G3=0 G4=0
        grep -q 'noise_fill: NoiseInterpolator hook armed, retransform rc=0' "$RDIR/boot.log" && G1=1
        grep -q 'noise_fill: Noise hook armed, retransform rc=0' "$RDIR/boot.log" && G2=1
        grep -q 'CRUSSTY_NOISE_FILL SELFTEST PASS' "$RDIR/boot.log" && G3=1
        grep -q 'noise_fill: dormant' "$RDIR/boot.log" && G4=1
        if [ "$G1" = 1 ] && [ "$G2" = 1 ] && [ "$G3" = 1 ] && [ "$G4" = 0 ]; then
            ARM_NOTE="armed-verified"
        else
            log "INVALID: $RID armed evidence unmet (interp=$G1 noise=$G2 selftest=$G3 dormleak=$G4) — LOUD ABORT"
            grep -E 'noise_fill:|CRUSSTY_NOISE_FILL|CRUSSTY_V3' "$RDIR/boot.log" | tail -8 >> "$OUT/run.log" || true
            echo "stop" >&3 2>/dev/null || true
            sleep 3; kill -9 "$SP" 2>/dev/null || true; kill "$HOLDER" 2>/dev/null || true
            restore_seed; return 1
        fi
        grep -q 'CRUSSTY_V3_SHADOW' "$RDIR/boot.log" && { log "INVALID: $RID shadow ran in A/B boot — law violation"; }
    else
        grep -q 'noise_fill: dormant (set CRUSSTY_NATIVE_NOISE_FILL=1 to enable)' "$RDIR/boot.log" || log "WARN: $RID no dormant line (boot may predate module init)"
    fi
    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" "$RID" "$ARM" "$BOOT_T" "$ARM_NOTE" "t_warm=${TW}s" "t_burst=${T_BURST}s" "cpu_burst=${CPU_BURST}s" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $BOOT_T $ARM_NOTE t_warm=$TW t_burst=${T_BURST}s cpu_burst=${CPU_BURST}s"
}

mapfile -t SEQ < <(arm_specs)
RANGE="${RUNS:-1-${#SEQ[@]}}"
I0=${RANGE%%-*}; I1=${RANGE##*-}
log "A/B begin: ${#SEQ[@]} runs total, executing $I0..$I1"
for ((i=I0; i<=I1; i++)); do
    ARM=${SEQ[$((i-1))]}
    run_one "$ARM" "$i" || true
done

log "=== SUMMARY ($REGIME) ==="
[ -s "$OUT/results.tsv" ] && python3 - "$OUT/results.tsv" <<'PYEOF'
import sys
rows=[l.strip().split('\t') for l in open(sys.argv[1]) if l.strip()]
CD=[r for r in rows if r[1]=='CD']; CA=[r for r in rows if r[1]=='CA']
def med(v): v=sorted(v); return v[len(v)//2] if v else float('nan')
def col(rs,tag):
    out=[]
    for r in rs:
        for f in r[4:]:
            if f.startswith(tag): out.append(float(f.split('=')[1].rstrip('s')))
    return out
cd_, ca_ = col(CD,'cpu_burst'), col(CA,'cpu_burst')
td, ta = col(CD,'t_burst'), col(CA,'t_burst')
print(f"dormant n={len(CD)} cpu_all={[f'{x:.1f}' for x in sorted(cd_)]} cpu_med={med(cd_):.1f} t_med={med(td):.1f}")
print(f"armed   n={len(CA)} cpu_all={[f'{x:.1f}' for x in sorted(ca_)]} cpu_med={med(ca_):.1f} t_med={med(ta):.1f}")
if cd_ and ca_:
    d=(med(ca_)-med(cd_))/med(cd_)*100
    sep = "YES" if max(ca_) < min(cd_) else "no"
    print(f"warm-metric delta: {d:+.1f}% (negative = armed cheaper); full-separation: {sep}")
PYEOF
log "A/B $REGIME range $I0..$I1 complete"
