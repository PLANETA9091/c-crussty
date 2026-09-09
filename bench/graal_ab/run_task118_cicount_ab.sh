#!/usr/bin/env bash
# TASK-118 phase 2 — CICompilerCount A/B on the operator-of-record config
# agent-7625532f, 2026-09-09. OPT_ARCH §7 R3#8, boot-compile-stall class.
# Ground truth: default CICompilerCount=2 {ergonomic} on this 2 vCPU box
# (PrintFlagsFinal, CICompilerCountPerCPU=true => explicit value honored).
# A = GraalVM CE 21.0.2 + UseJVMCICompiler + runtime agent env-unset (SOAK-PASS config)
# B = A + -XX:CICompilerCount=3  (=> 1 C1 + 2 JVMCI split)
# PRIMARY metric: boot-wall "Done(s)"; GUARD: warm burst cpu (not worse +3%).
# Gates pre-registered in CLAIM (dev-logs a32d9d3):
#   GO = boot-wall >=3% DOWN AND burst cpu <= +3% AND hs_err=0.
#   boot faster + burst worse >3% = TRADE-OFF NULL; boot parity/worse = NULL.
# REENTRANT: pass RAWDIR=/path/to/old/dir to skip already-done runs.
# LAWS: local i; J0 burst-start CPU; rm-FIRST restore; 9>&- ghost-lock;
# journal pair; AGENT_ARGS variable; foreground-only.
set -uo pipefail
cd /home/z/c-crussty
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="${RAWDIR:-$PWD/bench/graal_ab/RAW_TASK118_CICOUNT_$STAMP}"; mkdir -p "$OUT"
N_PER_ARM="${N:-5}"
IDLE_RATE=15
IDLE_STREAK=6
BURST_TIMEOUT=75
FL="forceload add 3200 3200 3327 3327"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK118-cicount-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK118-cicount-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"

run_one() { # $1=arm(A|B) $2=idx
    local ARM="$1" IDX="$2" RID="${1}${2}" i j
    if [ -f "$OUT/results.tsv" ] && grep -q "^$RID      " "$OUT/results.tsv"; then
        log "skip $RID (already done — reentrant)"; return 0
    fi
    local EXTRA=""
    [ "$ARM" = "B" ] && EXTRA="-XX:CICompilerCount=3"
    local RDIR="$OUT/run_$RID"; mkdir -p "$RDIR"
    local FIFO="$RDIR/console.fifo"; mkfifo "$FIFO"
    sleep 3600 3>"$FIFO" 9>&- & local HOLDER=$!
    ( cd "$SERVER" && exec "$JDK_GRAAL/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
        -XX:+UseJVMCICompiler $EXTRA "-Xlog:gc:file=$RDIR/gc.log" 9>&- $AGENT_ARGS \
        -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &
    local SP=$!
    local D=0
    for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: $RID no Done( in 120s"; kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; return 1; fi
    local BOOT_RAW; BOOT_RAW=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
    local BW; BW=$(echo "$BOOT_RAW" | grep -o '[0-9.]*')
    sleep 8
    local S=0
    for i in $(seq 1 60); do
        local J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        sleep 0.5; local J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        local DELTA=$(( J2 - J1 ))
        if [ "$DELTA" -le "$IDLE_RATE" ]; then S=$((S+1)); else S=0; fi
        [ "$S" -ge "$IDLE_STREAK" ] && break
    done
    exec 3>"$FIFO"
    echo "$FL" >&3
    local T0=$(date +%s.%N) J0 JPREV FIN=0
    J0=$(awk '{print $14+$15}' "/proc/$SP/stat")
    JPREV=$J0
    for j in $(seq 1 $((BURST_TIMEOUT*2))); do
        sleep 0.5
        local JN=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JPREV")
        local D2=$(( JN - JPREV ))
        JPREV=$JN
        if [ "$D2" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        [ "$FIN" -ge "$IDLE_STREAK" ] && break
    done
    local T1=$(date +%s.%N) JEND
    JEND=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$J0")
    local T_BURST=$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}')
    local CPU_BURST=$(echo "$JEND $J0" | awk '{printf "%.1f", ($1-$2)/100.0}')
    local RSS=$(awk '/VmRSS/ {printf "%.0f", $2/1024}' "/proc/$SP/status" 2>/dev/null || echo NA)
    local MAXPAUSE=$(grep 'Pause' "$RDIR/gc.log" 2>/dev/null | grep -o '[0-9.]*ms' | sed 's/ms//' | sort -n | tail -1)
    echo -e "$RID       $ARM    boot_wall=${BW}s        t_burst=${T_BURST}s     cpu_burst=${CPU_BURST}s rss=${RSS}MB    maxpause=${MAXPAUSE}ms" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: boot=${BW}s t=${T_BURST}s cpu=${CPU_BURST}s rss=${RSS}MB pause=${MAXPAUSE}ms"
}

SEQ=""
for k in $(seq 1 "$N_PER_ARM"); do SEQ="$SEQ A B"; done
log "TASK-118 CICompilerCount A/B begin: n=$N_PER_ARM/arm, order:$SEQ (A=default cc2, B=cc3)"
i=0
for ARM in $SEQ; do i=$((i+1)); run_one "$ARM" "$i" || true; done

HS1=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
log "hs_err delta: $((HS1 - HS0)) (must be 0)"

log "=== SUMMARY ==="
python3 - "$OUT/results.tsv" <<'PYEOF'
import sys
rows=[l.strip().split('\t') for l in open(sys.argv[1]) if l.strip()]
def med(v): v=sorted(v); return v[len(v)//2] if v else float('nan')
def col(rs,tag,suf):
    out=[]
    for r in rs:
        for f in r[2:]:
            if f.startswith(tag): out.append(float(f.split('=')[1].rstrip(suf)))
    return out
A=[r for r in rows if r[1]=='A']; B=[r for r in rows if r[1]=='B']
ba,bb=col(A,'boot_wall','s'),col(B,'boot_wall','s')
ca,cb=col(A,'cpu_burst','s'),col(B,'cpu_burst','s')
pa,pb=col(A,'maxpause','ms'),col(B,'maxpause','ms')
print(f"A default cc2  n={len(A)} boot_med={med(ba):.2f}s cpu_med={med(ca):.1f}s pause_med={med(pa):.1f}ms all_boot={[f'{x:.1f}' for x in sorted(ba)]} all_cpu={[f'{x:.1f}' for x in sorted(ca)]}")
print(f"B CICompilerCount=3 n={len(B)} boot_med={med(bb):.2f}s cpu_med={med(cb):.1f}s pause_med={med(pb):.1f}ms all_boot={[f'{x:.1f}' for x in sorted(bb)]} all_cpu={[f'{x:.1f}' for x in sorted(cb)]}")
if ba and bb:
    db=(med(bb)-med(ba))/med(ba)*100
    dc=(med(cb)-med(ca))/med(ca)*100
    print(f"boot delta med: {db:+.1f}% (negative = cc3 boots faster) | burst cpu delta med: {dc:+.1f}%")
    if db <= -3.0 and dc <= 3.0:
        print("GATE: GO — CICompilerCount=3 ADOPTED")
    elif db <= -3.0 and dc > 3.0:
        print("GATE: TRADE-OFF NULL (boot faster but burst regression >3%) — NOT adopted, config unchanged")
    else:
        print("GATE: NULL (boot parity or worse) — NOT adopted, config unchanged")
PYEOF
log "TASK-118 CICompilerCount A/B complete"
