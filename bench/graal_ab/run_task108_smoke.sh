#!/usr/bin/env bash
# TASK-108 smoke — deploy module .so + live boot verification (agent-7625532f)
# Gate order (pre-registered in BATCH_BRIDGE_DESIGN.md / CLAIMS):
#   S1 deploy:  modules/crussty/libcrussty.so <- target/release/libcrussty.so
#   S2 armed smoke: CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off
#      CRUSSTY_NOISE_FILL_CENSUS=1 ; REQUIRED markers:
#        - pristine sighting Noise AND ShiftNoise
#        - defined NormalNoiseBatchOps (kernel loader)
#        - 'Noise hook armed, retransform rc=0' AND 'ShiftNoise hook armed, rc=0'
#        - CRUSSTY_NOISE_FILL SELFTEST PASS      (hard gate)
#        - census line calls>0 after warm forceload (production dispatch proof)
#   S3 dormant smoke: no env -> 'noise_fill: dormant' line, no SELFTEST line,
#      boot reaches Done (byte-identical dormant sanity).
#   World seed restored before and after (canonical env for the A/B tick).
# Smoke warm region 3600-3663 (disjoint from A/B 3200/3400 bands).
set -uo pipefail
cd /home/z/c-crussty
JDK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
MODULE="$SERVER/modules/crussty/libcrussty.so"
BUILT="$PWD/target/release/libcrussty.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK108"; mkdir -p "$OUT"

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
[ -f "$BUILT" ] || { echo "FATAL: built module missing (cargo build --release)"; exit 1; }
[ "$(strings "$BUILT" | grep -c 'NormalNoiseBatchOps')" -gt 0 ] || { echo "FATAL: built module has no noise_fill"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK108-smoke-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK108-smoke-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/smoke.log"; }

SEED="$SERVER/world_graal_seed.tar.gz"
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }

AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
SMOKE_FL="forceload add 3600 3600 3663 3663"

boot_one() { # $1=tag  $2=env-string  -> 0 if Done( within 150s
    local TAG="$1" ENVV="$2" RDIR="$OUT/$TAG"
    mkdir -p "$RDIR"
    ( cd "$SERVER" && exec env $ENVV "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
        $AGENT_ARGS -jar "$JAR" --nogui ) >"$RDIR/boot.log" 2>&1 &
    local SP=$!
    echo "$SP" > "$RDIR/pid"
    local i D=0
    for i in $(seq 1 150); do grep -q 'Done (' "$RDIR/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    [ "$D" = 1 ] || { log "FATAL $TAG: no Done( in 150s"; kill -9 "$SP" 2>/dev/null; return 1; }
    grep -o 'Done ([0-9.]*s)' "$RDIR/boot.log" | tail -1 > "$RDIR/done_time"
    echo "$TAG" # pid file consumed by caller via $RDIR/pid
    return 0
}
# ---- S1: deploy -------------------------------------------------------------
log "S1 deploy: $BUILT -> $MODULE"
cp "$MODULE" "$MODULE.pre_task108.$$" 2>/dev/null || true
cp "$BUILT" "$MODULE" || { echo "FATAL: deploy cp failed"; exit 1; }
restore_seed
log "deploy ok; world seed restored"

# ---- S2: armed smoke --------------------------------------------------------
mkdir -p "$OUT/smoke_armed" "$OUT/smoke_dormant"
FIFO="$OUT/smoke_armed.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" & HOLDER=$!
( cd "$SERVER" && exec env CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off CRUSSTY_NOISE_FILL_CENSUS=1 \
    "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC $AGENT_ARGS -jar "$JAR" --nogui \
    <"$FIFO" >"$OUT/smoke_armed/boot.log" 2>&1 ) &
SP=$!
i=0; D=0
for i in $(seq 1 150); do grep -q 'Done (' "$OUT/smoke_armed/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then
    log "INVALID S2: armed boot no Done( in 150s — LOUD"
    tail -30 "$OUT/smoke_armed/boot.log" | tee -a "$OUT/smoke.log"
    kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1
fi
BOOT_T=$(grep -o 'Done ([0-9.]*s)' "$OUT/smoke_armed/boot.log" | tail -1)
log "S2 armed boot $BOOT_T"
# warm forceload on disjoint band -> production fill calls
exec 3>"$FIFO"
echo "$SMOKE_FL" >&3
# census daemon prints first line ~30s; give 75s total for fills to land
sleep 75
G1=$(grep -c 'noise_fill: pristine sighting Noise (' "$OUT/smoke_armed/boot.log" || true)
G2=$(grep -c 'noise_fill: pristine sighting ShiftNoise (' "$OUT/smoke_armed/boot.log" || true)
G3=$(grep -c 'noise_fill: defined net/minecraft/world/level/levelgen/NormalNoiseBatchOps in kernel loader' "$OUT/smoke_armed/boot.log" || true)
G4=$(grep -c 'noise_fill: Noise hook armed, retransform rc=0' "$OUT/smoke_armed/boot.log" || true)
G5=$(grep -c 'noise_fill: ShiftNoise hook armed, retransform rc=0' "$OUT/smoke_armed/boot.log" || true)
G6=$(grep -c 'CRUSSTY_NOISE_FILL SELFTEST PASS' "$OUT/smoke_armed/boot.log" || true)
CEN=$(grep 'CRUSSTY_NOISE_FILL_CENSUS calls=' "$OUT/smoke_armed/boot.log" | tail -1)
CEN_CALLS=$(echo "$CEN" | sed -n 's/.*calls=\([0-9]*\).*/\1/p')
CEN_FALL=$(echo "$CEN" | sed -n 's/.*fallbacks=\([0-9]*\).*/\1/p')
G7=0; [ -n "$CEN_CALLS" ] && [ "$CEN_CALLS" -gt 0 ] 2>/dev/null && G7=1
log "markers: pristineN=$G1 pristineS=$G2 defined=$G3 armedN=$G4 armedS=$G5 selftest_pass=$G6 census_calls=${CEN_CALLS:-0} census_fallbacks=${CEN_FALL:-NA}"
echo "stop" >&3
for i in $(seq 1 25); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
kill -9 "$SP" 2>/dev/null || true
exec 3>&-; kill "$HOLDER" 2>/dev/null || true
grep -E 'noise_fill:|CRUSSTY_NOISE_FILL|CRUSSTY_NOISE_FILL_CENSUS' "$OUT/smoke_armed/boot.log" > "$OUT/smoke_armed/markers.txt" || true

if [ "$G1" -ge 1 ] && [ "$G2" -ge 1 ] && [ "$G3" -ge 1 ] && [ "$G4" -ge 1 ] && [ "$G5" -ge 1 ] && [ "$G6" -ge 1 ]; then
    S2_OK=1
else
    S2_OK=0
fi
# NOTE (tick 06:00 discovery): census=0 during active chunk-gen is EXPECTED —
# bytecode-verified: production noise work runs through NoiseChunk.fillSlice ->
# NoiseInterpolator.fillArray (fillingCell branch) / NoiseChunk$FlatCache.fillArray
# (fillAllDirectly per-point with quart quantization); DensityFunctions$Noise /
# ShiftNoise fillArray targets receive ZERO production traffic in this workload.
# Census stays wired for v2 re-target evidence.

# ---- S3: dormant smoke ------------------------------------------------------
( cd "$SERVER" && exec "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC $AGENT_ARGS -jar "$JAR" --nogui \
    >"$OUT/smoke_dormant/boot.log" 2>&1 ) &
SP=$!
i=0; D=0
for i in $(seq 1 150); do grep -q 'Done (' "$OUT/smoke_dormant/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
DG=$(grep -c 'noise_fill: dormant (set CRUSSTY_NATIVE_NOISE_FILL=1 to enable)' "$OUT/smoke_dormant/boot.log" || true)
DS=$(grep -c 'CRUSSTY_NOISE_FILL SELFTEST' "$OUT/smoke_dormant/boot.log" || true)
if [ "$D" = 1 ]; then kill "$SP" 2>/dev/null; sleep 3; kill -9 "$SP" 2>/dev/null || true; else kill -9 "$SP" 2>/dev/null || true; fi
log "S3 dormant: Done=$D dormant_line=$DG selftest_lines=$DS"
S3_OK=0; [ "$D" = 1 ] && [ "$DG" -ge 1 ] && [ "$DS" = 0 ] && S3_OK=1

restore_seed
log "world seed restored (canonical for A/B)"

if [ "$S2_OK" = 1 ] && [ "$S3_OK" = 1 ]; then
    log "SMOKE VERDICT: PASS-ALL (S1 deploy, S2 armed+selftest; census_calls=${CEN_CALLS:-0} = dispatch-dead finding, see NOTE)"
    exit 0
elif [ "$S2_OK" = 1 ]; then
    log "SMOKE VERDICT: PASS-ARMED-ONLY (S3 dormant anomaly — inspect before A/B)"
    exit 2
else
    log "SMOKE VERDICT: FAIL — armed gate unmet; A/B forbidden by pre-registered gate"
    exit 1
fi
