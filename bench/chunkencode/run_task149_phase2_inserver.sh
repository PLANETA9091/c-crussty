#!/usr/bin/env bash
# TASK-149 phase-2a — IN-SERVER SectionData gate probe (agent-7625532f)
# Carries the phase-1 pure-array matrix INTO the live server via attach+loadAgent
# (bench-lane instrumentation of a running subject). Server boot stays PURE-INJECT:
# stock JDK + ONLY -agentpath — INJECTS-ONLY canon untouched, zero product changes.
# Pre-registration: bench/chunkencode/results/TASK149_PHASE2A_DESIGN_2026-09-09.md
#   RIG-INVALID    : no SHADOW_ENV row OR light row ULINK/THROW (channel itself broken)
#   CHANNEL-CLOSED : >=1 non-null section row rc=-3 (§97 verbatim, STRICT reading)
#   GATE-OPEN      : >=1 non-null section row rc>=0  -> phase-2b parity sweep + P500
#   ARG-LAYER      : all non-null rows in {-1,-2}, no -3, no >=0 (gate lifted, wire wrong)
set -uo pipefail
cd "$(dirname "$0")"    # bench/chunkencode
REPO_ROOT="$(cd ../.. && pwd)"

STAMP=$(date +%Y%m%d_%H%M%S)
JDK=/home/z/jdk21
if [ -x "$JDK/bin/java" ]; then JAVA="$JDK/bin/java"; JAVA_TAG="jdk21"
else JAVA=/usr/bin/java; JAVA_TAG="system-$(/usr/bin/java -version 2>&1 | head -1 | awk '{print $3}')"; fi
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
MODULE="$SERVER/modules/crussty/libcrussty.so"
SEED="$SERVER/world_graal_seed.tar.gz"
LOGDIR="$REPO_ROOT/bench/chunkencode/logs/task149_phase2a_$STAMP"
RAW="$REPO_ROOT/bench/chunkencode/results/CHUNKENCODE_SECTION_PHASE2A_RAW.tsv"

[ -f "$JAR" ] || { echo "FATAL: kernel jar missing ($JAR) — provisioning incomplete"; exit 1; }
[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing ($RUNTIME) — provisioning incomplete"; exit 1; }
[ -f "$MODULE" ] || { echo "FATAL: module not deployed ($MODULE) — provisioning incomplete"; exit 1; }
[ -f "$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so" ] || { echo "FATAL: repo chunk .so missing"; exit 1; }
if pgrep -f 'purpur-1\.21\.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi

exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-task149p2a-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task149p2a-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
mkdir -p "$LOGDIR"; : > "$RAW"

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOGDIR/rig.log"; }

# ---- deploy bench-lane native libs (repo = source of truth; mirrors TASK-108 S1) ----
mkdir -p "$SERVER/modules/crussty/native"
cp -f "$REPO_ROOT/native/libpaper_native_jni.so" "$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so" \
      "$SERVER/modules/crussty/native/" || { echo "FATAL: native deploy failed"; exit 1; }
log "deploy: repo native/*.so -> $SERVER/modules/crussty/native/ (chunk lib = optional dlopen source)"

restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
if [ -f "$SEED" ]; then restore_seed; log "world seed restored"
else ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); log "seed snapshot created"; fi

# ---- PURE boot (stock JDK + ONLY -agentpath; NO other flags) ----
FIFO="$LOGDIR/console.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" & HOLDER=$!
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
log "boot: java=$JAVA_TAG args=[$AGENT_ARGS] (pure-inject)"
( cd "$SERVER" && exec "$JAVA" $AGENT_ARGS -jar "$JAR" --nogui <"$FIFO" >"$LOGDIR/boot.log" 2>&1 ) &
SP=$!
echo "$SP" > "$LOGDIR/pid"

D=0
for i in $(seq 1 150); do grep -q 'Done (' "$LOGDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then
    log "FATAL: no Done( in 150s — RIG-INVALID (boot failed)"
    tail -30 "$LOGDIR/boot.log" | tee -a "$LOGDIR/rig.log"
    echo -e "VERDICT\tRIG-INVALID\tboot-no-done" >> "$RAW"
    kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1
fi
log "boot $(grep -o 'Done ([0-9.]*s)' "$LOGDIR/boot.log" | tail -1)"

S=0
for i in $(seq 1 90); do grep -q 'native surface live' "$LOGDIR/boot.log" 2>/dev/null && { S=1; break; }; sleep 1; done
SURF=$(grep 'native surface live' "$LOGDIR/boot.log" | tail -1)
if [ "$S" != 1 ]; then
    log "FATAL: surface-live marker absent in 90s — RIG-INVALID (module surface not initialized)"
    echo -e "VERDICT\tRIG-INVALID\tsurface-absent" >> "$RAW"
    echo "stop" >&3; sleep 5; kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1
fi
log "surface: $SURF"
sleep 5   # settle: let live_proof + activate threads land

# ---- attach shadow (source-mode runner; server pid $SP) ----
log "attach+loadAgent shadow (pid $SP)"
timeout 120 "$JAVA" --add-modules jdk.attach \
    "$REPO_ROOT/bench/chunkencode/Task149Attach.java" \
    "$SP" "$REPO_ROOT/bench/chunkencode/ChunkEncodeSectionShadow.java" \
    "$LOGDIR/shadow.tsv" >"$LOGDIR/attach.out" 2>"$LOGDIR/attach.err"
ARC=$?
grep -E "^SHADOW" "$LOGDIR/attach.out" >> "$RAW" 2>/dev/null || true
if [ $ARC -ne 0 ]; then
    log "ATTACH-FAILED rc=$ARC — RIG-INVALID (attach channel broken)"
    tail -20 "$LOGDIR/attach.err" | sed 's/^/  err> /' | tee -a "$LOGDIR/rig.log"
    echo -e "VERDICT\tRIG-INVALID\tattach-rc=$ARC" >> "$RAW"
    echo "stop" >&3; sleep 5; kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1
fi

# ---- verdict (pre-registered tree; rows verbatim, attribution separate) ----
SECTION_ROWS=$(grep -E '^SHADOW\t' "$LOGDIR/attach.out" | grep -vE '^SHADOW_ENV|^SHADOW_SINK' | grep -vP '^SHADOW\tlight_sanity' || true)
CLOSE=$(echo "$SECTION_ROWS" | grep -cP 'rc=-3\t' || true)
OPEN=$(echo "$SECTION_ROWS" | grep -cP 'rc=[0-9]+\t' || true)
LIGHT=$(grep -E '^SHADOW\tlight_sanity' "$LOGDIR/attach.out" | head -1 || true)
LIGHT_OK=$(echo "$LIGHT" | grep -cP 'rc=-?[0-9]+\t' || true)
ARGONLY=$(echo "$SECTION_ROWS" | grep -vE '^SHADOW\tnull_' | grep -vcE 'rc=-3\t' || true)

if [ "$LIGHT_OK" = "0" ]; then V="RIG-INVALID"; VR="light-row-not-numeric: $LIGHT"
elif [ "$CLOSE" -ge 1 ]; then V="CHANNEL-CLOSED"; VR="strict §97: >=1 non-null section rc=-3 ($CLOSE rows)"
elif [ "$OPEN" -ge 1 ]; then V="GATE-OPEN"; VR=">=1 section rc>=0 ($OPEN rows) -> phase-2b parity sweep + P500 pre-registered"
elif [ "$ARGONLY" = "0" ]; then V="ARG-LAYER"; VR="all non-null rows rc in {-1,-2}: gate lifted, hypothesis v1 arg-rejected"
else V="RIG-INVALID"; VR="unclassifiable row set"; fi
log "VERDICT: $V — $VR"
echo -e "VERDICT\t$V\t$VR" >> "$RAW"
echo -e "BOOT\tjava=$JAVA_TAG\tsurface=$SURF" >> "$RAW"

# ---- stop server, restore world ----
echo "stop" >&3
for i in $(seq 1 25); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
kill -9 "$SP" 2>/dev/null || true
kill "$HOLDER" 2>/dev/null || true
restore_seed
log "server stopped, seed restored"
[ -f "$SERVER/hs_err_pid"*.log ] && log "WARNING: hs_err present: $(ls "$SERVER"/hs_err_pid*.log | tr '\n' ' ')"
log "RAW: $RAW"
cat "$RAW"
exit 0
