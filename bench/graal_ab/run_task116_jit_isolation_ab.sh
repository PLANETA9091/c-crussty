#!/usr/bin/env bash
# TASK-116 — Graal JIT-VARIABLE ISOLATION: Temurin-C2 vs C2-on-GraalVM-build (UseJVMCICompiler OFF)
# agent-7625532f, 2026-09-09. The research claim (OPT_ARCH §7 R3#2: +23% geomean,
# strongest on megamorphic dispatch) is a STEADY-STATE claim — boot-time alone
# cannot measure it (and structurally favors C2). This rig measures the real
# thing: a 64-chunk worldgen FORCELLOAD BURST on fresh chunks under each JIT,
# P500-style paired runs, ABBA, byte-identical world restore per run.
# Primary metric: cpu_burst (child utime+stime jiffies delta over the burst
# window — JIT-independent of wall-clock contention). Secondary: t_burst wall.
# Dormant module both arms — JVM-level comparison only. Pilot n=2/arm.
set -uo pipefail
cd /home/z/c-crussty
JDK_C2=/home/z/jdk21
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK116_$STAMP"; mkdir -p "$OUT"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK116-jit-isolation-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK116-jit-isolation-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

N_PER_ARM="${N:-2}"           # pilot: 2/arm ABBA
IDLE_RATE=15                  # jiffies/0.5s sample (2 cores: full=100) below = idle
IDLE_STREAK=6
BURST_TIMEOUT=75
FL="forceload add 3200 3200 3327 3327"   # 64 fresh chunks far from all prior rigs

# --- seed anchor (clean world, server stopped) ---
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }   # rm FIRST: tar is an overlay, not a snapshot (pilot-2 lesson: generated r.200.200.mca survived restore -> no-op bursts)
restore_seed

log "JDKs: C2=$($JDK_C2/bin/java -version 2>&1 | head -1) | GRAAL=$($JDK_GRAAL/bin/java -version 2>&1 | head -1)"

run_one() { # $1=arm(A|B) $2=idx
    local ARM="$1" IDX="$2" RID="${1}${2}" i
    awk -F'\t' -v r="$RID" '$1==r{found=1} END{exit !found}' "$OUT/results.tsv" 2>/dev/null && { log "skip $RID (already done)"; return 0; }
    local JDKP FLAGS
    if [ "$ARM" = "A" ]; then JDKP="$JDK_C2"; FLAGS=""
    else JDKP="$JDK_GRAAL"; FLAGS="-XX:-UseJVMCICompiler"; fi
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    local FIFO="$RDIR/console.fifo"; mkfifo "$FIFO"
    sleep 3600 3>"$FIFO" 9>&- & local HOLDER=$!   # holder write-end opens FIRST (server read-end then unblocks)
    ( cd "$SERVER" && exec "$JDKP/bin/java" 9>&- $FLAGS -Xms512M -Xmx2G -XX:+UseG1GC \
        -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &   # stdin=FIFO console; 9>&-: no flock fd inheritance (ghost-lock lesson)
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: $RID no Done( in 120s"; kill "$SP" 2>/dev/null; return 1; fi
    local BOOT_T; BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
    # idle gate: wait for CPU floor (same discipline as TASK-74 v2)
    local PREV J1
    sleep 8
    for i in $(seq 1 60); do
        J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        local J2 S=0
        sleep 0.5; J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        local DELTA=$(( J2 - J1 ))
        if [ "$DELTA" -le "$IDLE_RATE" ]; then S=$((S+1)); else S=0; fi
        [ "$S" -ge "$IDLE_STREAK" ] && break
    done
    # burst: forceload via the pre-opened console FIFO
    exec 3>"$FIFO"
    echo "$FL" >&3
    local T0=$(date +%s.%N) J0 JPREV
    J0=$(awk '{print $14+$15}' "/proc/$SP/stat")
    JPREV=$J0
    local FIN=0
    for i in $(seq 1 $((BURST_TIMEOUT*2))); do
        sleep 0.5
        local JN=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JPREV")
        local D2=$(( JN - JPREV ))   # ROLLING per-sample delta (cumulative-from-gate never decays)
        JPREV=$JN
        if [ "$D2" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        if [ "$FIN" -ge "$IDLE_STREAK" ]; then break; fi
    done
    local T1=$(date +%s.%N) JEND
    JEND=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$J0")
    local T_BURST=$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}')
    local CPU_BURST=$(echo "$JEND $J0" | awk '{printf "%.1f", ($1-$2)/100.0}')
    echo -e "$RID\t$ARM\t$BOOT_T\tt_burst=${T_BURST}s\tcpu_burst=${CPU_BURST}s" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $BOOT_T t_burst=${T_BURST}s cpu_burst=${CPU_BURST}s"
}

# --- ABBA ---
SEQ=""; for k in $(seq 1 "$N_PER_ARM"); do SEQ="$SEQ A B"; done
log "TASK-96 loaded A/B pilot begin: n=$N_PER_ARM per arm, order:$SEQ"
i=0
for ARM in $SEQ; do i=$((i+1)); run_one "$ARM" "$i" || true; done

log "=== SUMMARY ==="
python3 - "$OUT/results.tsv" <<'PYEOF'
import sys
rows=[l.strip().split('\t') for l in open(sys.argv[1]) if l.strip()]
A=[r for r in rows if r[1]=='A']; B=[r for r in rows if r[1]=='B']
def med(v): v=sorted(v); return v[len(v)//2] if v else float('nan')
def col(rs,tag):
    out=[]
    for r in rs:
        for f in r[3:]:
            if f.startswith(tag): out.append(float(f.split('=')[1].rstrip('s')))
    return out
ta, tb = col(A,'t_burst'), col(B,'t_burst')
ca, cb = col(A,'cpu_burst'), col(B,'cpu_burst')
print(f"C2   n={len(A)} t_med={med(ta):.1f}s cpu_med={med(ca):.1f}s cpu_all={[f'{x:.1f}' for x in sorted(ca)]}")
print(f"GRAAL n={len(B)} t_med={med(tb):.1f}s cpu_med={med(cb):.1f}s cpu_all={[f'{x:.1f}' for x in sorted(cb)]}")
if ca and cb: print(f"cpu delta med: {(med(cb)-med(ca))/med(ca)*100:+.1f}%  (negative = Graal cheaper)")
if ta and tb: print(f"wall delta med: {(med(tb)-med(ta))/med(ta)*100:+.1f}%")
PYEOF
log "TASK-96 loaded A/B pilot complete"
