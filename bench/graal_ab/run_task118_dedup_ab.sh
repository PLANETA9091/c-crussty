#!/usr/bin/env bash
# TASK-118 phase 1 — UseStringDeduplication A/B on the operator-of-record config
# agent-7625532f, 2026-09-09. OPT_ARCH §7 R3#8. One variable per A/B.
# A = GraalVM CE 21.0.2 + UseJVMCICompiler + production runtime agent (env unset)
# B = A + -XX:+UseStringDeduplication
# Warm ABBA n=3/arm, 3200-band 64-chunk bursts. Gates pre-registered in CLAIM:
# GO = cpu delta >=3% DOWN + max-pause regression <=20% + RSS not worse.
# LAWS: local i; J0 burst-start CPU; rm-FIRST restore; 9>&- ghost-lock;
# journal pair; AGENT_ARGS variable (inline semicolons = separators); foreground.
set -uo pipefail
cd /home/z/c-crussty
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK118_DEDUP_$STAMP"; mkdir -p "$OUT"
N_PER_ARM="${N:-3}"
IDLE_RATE=15
IDLE_STREAK=6
BURST_TIMEOUT=75
FL="forceload add 3200 3200 3327 3327"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK118-dedup-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK118-dedup-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }

SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"

run_one() { # $1=arm(A|B) $2=idx
    local ARM="$1" IDX="$2" RID="${1}${2}" i j
    local EXTRA=""
    [ "$ARM" = "B" ] && EXTRA="-XX:+UseStringDeduplication"
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
    local BOOT_T; BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1)
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
    echo -e "$RID\t$ARM\t$BOOT_T\tt_burst=${T_BURST}s\tcpu_burst=${CPU_BURST}s\trss=${RSS}MB\tmaxpause=${MAXPAUSE}ms" | tee -a "$OUT/results.tsv"
    echo "stop" >&3
    for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    exec 3>&-; kill "$HOLDER" 2>/dev/null || true
    restore_seed
    log "run $RID done: $BOOT_T t=${T_BURST}s cpu=${CPU_BURST}s rss=${RSS}MB pause=${MAXPAUSE}ms"
}

SEQ=""
for k in $(seq 1 "$N_PER_ARM"); do SEQ="$SEQ A B"; done
log "TASK-118 dedup A/B begin: n=$N_PER_ARM/arm, order:$SEQ"
i=0
for ARM in $SEQ; do i=$((i+1)); run_one "$ARM" "$i" || true; done

log "=== SUMMARY ==="
python3 - "$OUT/results.tsv" <<'PYEOF'
import sys
rows=[l.strip().split('\t') for l in open(sys.argv[1]) if l.strip()]
def med(v): v=sorted(v); return v[len(v)//2] if v else float('nan')
def col(rs,tag,suf):
    out=[]
    for r in rs:
        for f in r[3:]:
            if f.startswith(tag): out.append(float(f.split('=')[1].rstrip(suf)))
    return out
A=[r for r in rows if r[1]=='A']; B=[r for r in rows if r[1]=='B']
ca,cb=col(A,'cpu_burst','s'),col(B,'cpu_burst','s'); ta,tb=col(A,'t_burst','s'),col(B,'t_burst','s')
ra,rb=col(A,'rss','MB'),col(B,'rss','MB'); pa,pb=col(A,'maxpause','ms'),col(B,'maxpause','ms')
print(f"A op-config      n={len(A)} cpu_med={med(ca):.1f}s t_med={med(ta):.1f}s rss_med={med(ra):.0f}MB pause_med={med(pa):.1f}ms all_cpu={[f'{x:.1f}' for x in sorted(ca)]}")
print(f"B +StringDedup   n={len(B)} cpu_med={med(cb):.1f}s t_med={med(tb):.1f}s rss_med={med(rb):.0f}MB pause_med={med(pb):.1f}ms all_cpu={[f'{x:.1f}' for x in sorted(cb)]}")
if ca and cb:
    d=(med(cb)-med(ca))/med(ca)*100
    print(f"cpu delta med: {d:+.1f}% (negative = dedup cheaper) | rss delta med: {(med(rb)-med(ra))/med(ra)*100:+.1f}% | pause delta med: {(med(pb)-med(pa))/med(pa)*100:+.1f}%")
    print(f"GATE: {'GO' if d<=-3.0 else 'NULL (not adopted)'} (pre-registered: >=3% DOWN + pause reg <=20% + rss not worse)")
PYEOF
log "TASK-118 dedup A/B complete"
