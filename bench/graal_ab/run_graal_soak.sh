#!/usr/bin/env bash
# TASK-117 — GRAAL SOAK server-wide: reentrant chunk rig (phase 1)
# agent-7625532f, 2026-09-09. See docs/GRAAL_SOAK_DESIGN.md (pre-registered).
# One chunk per invocation: boot operator-of-record config (GraalVM CE 21.0.2 +
# -XX:+UseJVMCICompiler + production runtime agent env-unset), then up to
# $WAVES waves of 64-fresh-chunk forceload bands (deterministic fixed band
# schedule => per-wave work identical across chunks), time-bounded by
# LOAD_SECONDS. Appends bench/graal_ab/RAW_SOAK/{waves.tsv,state.tsv}.
# LAWS: foreground only (nohup dies with tool-call); linear script => globals,
# no loop-var re-entrancy; J0 burst-start CPU law; rm-FIRST seed restore (tar
# is overlay); 9>&- ghost-lock; journal start/done pair; AGENT_ARGS variable
# (unquoted semicolons in inline agentpath = command separators).
set -uo pipefail
cd /home/z/c-crussty
JDK_GRAAL=/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT_ROOT="$PWD/bench/graal_ab/RAW_SOAK"; mkdir -p "$OUT_ROOT"
WAVES="${WAVES:-12}"
LOAD_SECONDS="${LOAD_SECONDS:-420}"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 120 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK117-graal-soak-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK117-graal-soak-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT_ROOT/soak.log"; }

RDIR="$OUT_ROOT/chunk_$STAMP"; mkdir -p "$RDIR"
IDLE_RATE=15                  # jiffies/0.5s (2 cores full=100) below = idle
IDLE_STREAK=6
HS_BEFORE=$(ls "$SERVER"/hs_err_* 2>/dev/null | wc -l)
[ -f "$OUT_ROOT/waves.tsv" ] || echo -e "chunk\twave\tband\tw_wall\tw_cpu\trss" > "$OUT_ROOT/waves.tsv"

# --- seed anchor (rm FIRST: tar is an overlay, not a snapshot) ---
SEED="$SERVER/world_graal_seed.tar.gz"
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed

rss_mb() { awk '/VmRSS/ {printf "%.1f", $2/1024}' "/proc/$1/status" 2>/dev/null || echo "NA"; }

log "TASK-117 SOAK chunk $STAMP begin: $( $JDK_GRAAL/bin/java -version 2>&1 | head -2 | tail -1 ) WAVES=$WAVES LOAD_SECONDS=$LOAD_SECONDS"

AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
FIFO="$RDIR/console.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" 9>&- & HOLDER=$!
( cd "$SERVER" && exec "$JDK_GRAAL/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
    -XX:+UseJVMCICompiler "-Xlog:gc:file=$RDIR/gc.log" 9>&- $AGENT_ARGS \
    -jar "$JAR" --nogui <"$FIFO" >"$RDIR/boot.log" 2>&1 ) &
SP=$!
D=0
for i in $(seq 1 120); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then log "FATAL: chunk $STAMP no Done( in 120s"; kill "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1; fi
BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1 | grep -o '[0-9.]*')
log "boot Done ${BOOT_T}s"

# idle gate pre-load (S initialized BEFORE loop: resetting inside kills the streak)
sleep 8
S=0
for i in $(seq 1 60); do
    J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
    sleep 0.5; J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
    DELTA=$(( J2 - J1 ))
    if [ "$DELTA" -le "$IDLE_RATE" ]; then S=$((S+1)); else S=0; fi
    [ "$S" -ge "$IDLE_STREAK" ] && break
done

exec 3>"$FIFO"
T_LOAD0=$(date +%s.%N)
NW=0
i=0
while [ "$i" -lt "$WAVES" ]; do
    i=$((i+1))
    X1=$((4000 + 64*(i-1) ))
    NOW=$(date +%s)
    ELAPSED=$(echo "$NOW $T_LOAD0" | awk '{printf "%d", $1-$2}')
    [ "$ELAPSED" -ge "$LOAD_SECONDS" ] && { log "load budget reached (${ELAPSED}s) after $NW waves"; break; }
    echo "forceload add $X1 3200 $((X1+7)) 3207" >&3
    T0=$(date +%s.%N); J0=$(awk '{print $14+$15}' "/proc/$SP/stat"); JPREV=$J0; FIN=0
    for j in $(seq 1 240); do     # 120s hard cap per wave (hung-wave guard)
        sleep 0.5
        JN=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$JPREV")
        D2=$(( JN - JPREV ))
        JPREV=$JN
        if [ "$D2" -le "$IDLE_RATE" ]; then FIN=$((FIN+1)); else FIN=0; fi
        [ "$FIN" -ge "$IDLE_STREAK" ] && break
    done
    T1=$(date +%s.%N); JEND=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo "$J0")
    W_WALL=$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}')
    W_CPU=$(echo "$JEND $J0" | awk '{printf "%.1f", ($1-$2)/100.0}')
    W_RSS=$(rss_mb "$SP")
    NW=$((NW+1))
    echo -e "chunk_$STAMP\t$NW\tx=${X1}..$((X1+7)),z=3200\t${W_WALL}s\t${W_CPU}s\t${W_RSS}" | tee -a "$OUT_ROOT/waves.tsv"
    log "wave $NW done: wall=${W_WALL}s cpu=${W_CPU}s rss=${W_RSS}MB"
done
T_LOAD1=$(date +%s.%N)
LOAD_W=$(echo "$T_LOAD1 $T_LOAD0" | awk '{printf "%.1f", $1-$2}')

echo "stop" >&3
for i in $(seq 1 25); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
kill -9 "$SP" 2>/dev/null || true
exec 3>&-; kill "$HOLDER" 2>/dev/null || true
restore_seed

HS_AFTER=$(ls "$SERVER"/hs_err_* 2>/dev/null | wc -l)
HS_DELTA=$(( HS_AFTER - HS_BEFORE ))

# --- per-chunk summary -> state.tsv (columns documented in design doc) ---
python3 - "$STAMP" "$BOOT_T" "$NW" "$LOAD_W" "$HS_DELTA" "$OUT_ROOT" <<'PYEOF'
import sys, statistics as st
stamp, boot_t, nw, load_w, hs_d, root = sys.argv[1], float(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
waves = []
for l in open(root + "/waves.tsv"):
    f = l.rstrip("\n").split("\t")
    if f[0] == "chunk_" + stamp and f[1] != "wave":
        waves.append((float(f[3].rstrip("s")), float(f[4].rstrip("s")), float(f[5])))
cpu = [w[1] for w in waves]; rss = [w[2] for w in waves]
h1 = st.median(cpu[:len(cpu)//2]) if len(cpu) >= 4 else float("nan")
h2 = st.median(cpu[len(cpu)//2:]) if len(cpu) >= 4 else float("nan")
drift = (h2 - h1) / h1 * 100 if h1 == h1 and h1 > 0 else float("nan")
r4 = rss[3] if len(rss) >= 4 else (rss[-1] if rss else float("nan"))
rl = rss[-1] if rss else float("nan")
rgrow = (rl - r4) / r4 * 100 if r4 == r4 and r4 > 0 else float("nan")
flags = []
if drift == drift and drift > 15: flags.append("INVESTIGATE-cpu-drift")
if rgrow == rgrow and rgrow > 25: flags.append("INVESTIGATE-rss-growth")
if hs_d != 0: flags.append("HARD-hs_err")
row = (f"chunk_{stamp}\tboot={boot_t:.1f}s\twaves={nw}\tload_wall={load_w:.1f}s\t"
       f"cpu_total={sum(cpu):.1f}s\tcpu_h1_med={h1:.1f}\tcpu_h2_med={h2:.1f}\t"
       f"cpu_drift_pct={drift:+.1f}\trss_w4={r4:.0f}MB\trss_last={rl:.0f}MB\t"
       f"rss_grow_pct={rgrow:+.1f}\ths_err_delta={hs_d}\t"
       f"flags={'+'.join(flags) if flags else 'none'}\n")
open(root + "/state.tsv", "a").write(row)
# cumulative soak status (pre-registered quota: >=40 min load across >=5 chunks)
tot_load = 0.0; nch = 0
for l in open(root + "/state.tsv"):
    if not l.strip(): continue
    kv = dict(f.split("=", 1) for f in l.strip().split("\t")[1:])
    tot_load += float(kv["load_wall"].rstrip("s")); nch += 1
print(f"CHUNK SUMMARY: {row.strip()}")
print(f"CUMULATIVE: chunks={nch} load_wall_total={tot_load/60:.1f}min (quota: >=40min across >=5 chunks)")
PYEOF

log "TASK-117 SOAK chunk $STAMP complete (hs_err delta=$HS_DELTA)"
