#!/usr/bin/env bash
# TASK-108 v3 smoke — the REAL-router gate (agent-7625532f, PROGRESS-7).
# Armed boot wires DensityArrayInterpreter into NoiseChunk$NoiseInterpolator
# .fillArray; the forceload drives REAL chunkgen, and the env-gated SHADOW
# mode (CRUSSTY_V3_SHADOW_N) compares the first N bridged column fills
# raw-bits against the EXACT vanilla fill (provider.fillAllDirectly) on the
# server's actual RandomState router — the pre-registered v3 gate 1.
# Gate order:
#   S1 deploy: modules/crussty/libcrussty.so <- target/release/libcrussty.so
#   S2 armed:  CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off
#              CRUSSTY_V3_SHADOW_N=32 ; REQUIRED markers:
#        - pristine sighting Noise / ShiftNoise / NoiseInterpolator
#        - defined NormalNoiseBatchOps (kernel loader)
#        - 'X hook armed, retransform rc=0' for all three targets
#        - CRUSSTY_NOISE_FILL SELFTEST PASS        (v1 leaf gate, still hard)
#        - CRUSSTY_V3_SHADOW PASS checks=32 ...    (v3 REAL-router gate, hard)
#   S3 dormant: no env -> dormant line, no hooks, boot reaches Done.
# World seed restored before and after (canonical env for the A/B tick).
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
OUT="$PWD/bench/graal_ab/RAW_TASK108_V3"; mkdir -p "$OUT"

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
[ -f "$BUILT" ] || { echo "FATAL: built module missing (cargo build --release)"; exit 1; }
[ "$(strings "$BUILT" | grep -c 'DensityArrayInterpreter')" -gt 0 ] || { echo "FATAL: built module has no v3 interpreter embed"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK108-v3-smoke-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK108-v3-smoke-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/smoke.log"; }

SEED="$SERVER/world_graal_seed.tar.gz"
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
[ -f "$SEED" ] || { ( cd "$SERVER" && tar -czf world_graal_seed.tar.gz world world_nether world_the_end ); }

AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
SMOKE_FL="forceload add 3600 3600 3663 3663"

log "S1 deploy: $BUILT -> $MODULE"
cp "$MODULE" "$MODULE.pre_task108v3.$$" 2>/dev/null || true
cp "$BUILT" "$MODULE" || { echo "FATAL: deploy cp failed"; exit 1; }
restore_seed
log "deploy ok; world seed restored"

# ---- S2: armed smoke (shadow-gated) -----------------------------------------
mkdir -p "$OUT/smoke_armed" "$OUT/smoke_dormant"
FIFO="$OUT/smoke_armed.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" & HOLDER=$!
( cd "$SERVER" && exec env CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off CRUSSTY_V3_SHADOW_N=32 \
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
exec 3>"$FIFO"
echo "$SMOKE_FL" >&3
# shadow needs bridged column fills: boot spawn-prep already fills some, the
# forceload adds a 64x64 band; give 75s for fills to land + shadow report
sleep 75
G1=$(grep -c 'noise_fill: pristine sighting Noise (' "$OUT/smoke_armed/boot.log" || true)
G2=$(grep -c 'noise_fill: pristine sighting ShiftNoise (' "$OUT/smoke_armed/boot.log" || true)
G3=$(grep -c 'noise_fill: pristine sighting NoiseInterpolator (' "$OUT/smoke_armed/boot.log" || true)
G4=$(grep -c 'noise_fill: defined net/minecraft/world/level/levelgen/NormalNoiseBatchOps in kernel loader' "$OUT/smoke_armed/boot.log" || true)
G5=$(grep -c 'noise_fill: Noise hook armed, retransform rc=0' "$OUT/smoke_armed/boot.log" || true)
G6=$(grep -c 'noise_fill: ShiftNoise hook armed, retransform rc=0' "$OUT/smoke_armed/boot.log" || true)
G7=$(grep -c 'noise_fill: NoiseInterpolator hook armed, retransform rc=0' "$OUT/smoke_armed/boot.log" || true)
G8=$(grep -c 'CRUSSTY_NOISE_FILL SELFTEST PASS' "$OUT/smoke_armed/boot.log" || true)
SHADOW_LINE=$(grep 'CRUSSTY_V3_SHADOW ' "$OUT/smoke_armed/boot.log" | tail -1)
G9=0; echo "$SHADOW_LINE" | grep -q 'CRUSSTY_V3_SHADOW PASS' && G9=1
log "markers: pristineN=$G1 pristineS=$G2 pristineI=$G3 defined=$G4 armedN=$G5 armedS=$G6 armedI=$G7 leaf_selftest=$G8 shadow='$SHADOW_LINE'"
echo "stop" >&3
for i in $(seq 1 25); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
kill -9 "$SP" 2>/dev/null || true
exec 3>&-; kill "$HOLDER" 2>/dev/null || true
grep -E 'noise_fill:|CRUSSTY_NOISE_FILL|CRUSSTY_V3_SHADOW' "$OUT/smoke_armed/boot.log" > "$OUT/smoke_armed/markers.txt" || true

if [ "$G1" -ge 1 ] && [ "$G2" -ge 1 ] && [ "$G3" -ge 1 ] && [ "$G4" -ge 1 ] && [ "$G5" -ge 1 ] && [ "$G6" -ge 1 ] && [ "$G7" -ge 1 ] && [ "$G8" -ge 1 ] && [ "$G9" -ge 1 ]; then
    S2_OK=1
else
    S2_OK=0
fi

# ---- S3: dormant smoke -------------------------------------------------------
( cd "$SERVER" && exec "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC $AGENT_ARGS -jar "$JAR" --nogui \
    >"$OUT/smoke_dormant/boot.log" 2>&1 ) &
SP=$!
i=0; D=0
for i in $(seq 1 150); do grep -q 'Done (' "$OUT/smoke_dormant/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
DG=$(grep -c 'noise_fill: dormant (set CRUSSTY_NATIVE_NOISE_FILL=1 to enable)' "$OUT/smoke_dormant/boot.log" || true)
DS=$(grep -c 'CRUSSTY_NOISE_FILL SELFTEST\|CRUSSTY_V3_SHADOW' "$OUT/smoke_dormant/boot.log" || true)
if [ "$D" = 1 ]; then kill "$SP" 2>/dev/null; sleep 3; kill -9 "$SP" 2>/dev/null || true; else kill -9 "$SP" 2>/dev/null || true; fi
log "S3 dormant: Done=$D dormant_line=$DG selftest_lines=$DS"
S3_OK=0; [ "$D" = 1 ] && [ "$DG" -ge 1 ] && [ "$DS" = 0 ] && S3_OK=1

restore_seed
log "world seed restored (canonical for A/B)"

if [ "$S2_OK" = 1 ] && [ "$S3_OK" = 1 ]; then
    log "SMOKE VERDICT: PASS-ALL (S1 deploy; S2 armed: 3 targets + leaf selftest + REAL-router shadow $SHADOW_LINE; S3 dormant)"
    exit 0
elif [ "$S2_OK" = 1 ]; then
    log "SMOKE VERDICT: PASS-ARMED-ONLY (S3 dormant anomaly — inspect before A/B)"
    exit 2
else
    log "SMOKE VERDICT: FAIL — armed gate unmet; A/B forbidden by pre-registered gate"
    exit 1
fi
