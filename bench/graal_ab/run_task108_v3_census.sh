#!/usr/bin/env bash
# TASK-108 v3 phase-3 step 1 — FULL-DISTRIBUTION interpreter census (agent-7625532f).
# PROGRESS-7 lesson: the 32-check shadow budget exhausted during early-boot
# spawn-prep, so its census (t3.FALLBACK.BlendDensity=4 RangeChoice=28) is NOT
# the production distribution. Fix: a diagnostic armed boot with a LARGE
# shadow budget (CRUSSTY_V3_SHADOW_N=20000) so the one-shot report fires DEEP
# into production forceload traffic; census captured then = real distribution
# across spawn-prep + fresh-chunk bands. Diagnostic-only boot (shadow
# re-advances provider state — never legal in A/B boots); world restored.
# Bands 3200 (measured-band twin) then 3400 then 3600 until the report lands.
set -uo pipefail
cd /home/z/c-crussty
JDK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
RUNTIME="$SERVER/libcrussty_runtime.so"
MODULE="$SERVER/modules/crussty/libcrussty.so"
BUILT="$PWD/target/release/libcrussty.so"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_TASK108_V3/census"; mkdir -p "$OUT"

[ -f "$RUNTIME" ] || { echo "FATAL: runtime .so missing"; exit 1; }
[ "$(strings "$BUILT" | grep -c 'DensityArrayInterpreter')" -gt 0 ] || { echo "FATAL: no v3 embed in built module"; exit 1; }
if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 180 9 || { echo "LOCK-BUSY"; exit 42; }
echo "start-TASK108-v3-census-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK108-v3-census-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/census.log"; }

SEED="$SERVER/world_graal_seed.tar.gz"
restore_seed() { ( cd "$SERVER" && rm -rf world world_nether world_the_end && tar -xzf world_graal_seed.tar.gz ); }
restore_seed

wait_idle() { # $1=pid  -> returns when cpu floor holds (same discipline as prior rigs)
    local SP=$1 i J1 J2 D S
    for i in $(seq 1 60); do
        J1=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
        S=0
        for i in $(seq 1 6); do
            sleep 0.5
            J2=$(awk '{print $14+$15}' "/proc/$SP/stat" 2>/dev/null || echo 0)
            D=$(( J2 - J1 )); J1=$J2
            [ "$D" -le 15 ] && S=$((S+1)) || S=0
        done
        [ "$S" -ge 4 ] && return 0
    done
    return 0
}

# ---- armed boot, BIG shadow budget ------------------------------------------
AGENT_ARGS="-agentpath:$RUNTIME=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
FIFO="$OUT/console.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" & HOLDER=$!
( cd "$SERVER" && exec env CRUSSTY_NATIVE_NOISE_FILL=1 CRUSSTY_KERNEL_POLICY=off CRUSSTY_V3_SHADOW_N=20000 \
    "$JDK/bin/java" 9>&- -Xms512M -Xmx2G -XX:+UseG1GC $AGENT_ARGS \
    -jar "$JAR" --nogui <"$FIFO" >"$OUT/boot.log" 2>&1 ) &
SP=$!
D=0
for i in $(seq 1 150); do grep -q 'Done (' "$OUT/boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then
    log "FATAL: armed boot no Done( in 150s"; tail -20 "$OUT/boot.log" | tee -a "$OUT/census.log"
    kill -9 "$SP" 2>/dev/null; kill "$HOLDER" 2>/dev/null; restore_seed; exit 1
fi
log "armed boot $(grep -o 'Done ([0-9.]*s)' "$OUT/boot.log" | tail -1) with SHADOW_N=20000"
sleep 8; wait_idle "$SP"

exec 3>"$FIFO"
BANDS=0
LINE=""
for BAND in "3200 3200 3327 3327" "3400 3400 3463 3463" "3600 3600 3663 3663"; do
    BANDS=$((BANDS+1))
    log "band #$BANDS: forceload add $BAND"
    echo "forceload add $BAND" >&3
    for i in $(seq 1 240); do
        LINE=$(grep 'CRUSSTY_V3_SHADOW ' "$OUT/boot.log" | tail -1)
        [ -n "$LINE" ] && break
        sleep 1
    done
    [ -n "$LINE" ] && break
    wait_idle "$SP"
done

if [ -n "$LINE" ]; then
    log "SHADOW REPORT CAPTURED: $LINE"
    echo "$LINE" > "$OUT/census_line.txt"
    grep -E 'noise_fill:|CRUSSTY_NOISE_FILL|CRUSSTY_V3_SHADOW' "$OUT/boot.log" > "$OUT/markers.txt" || true
    VERDICT="CENSUS-CAPTURED"
else
    log "budget 20000 NOT exhausted across 3 bands — census incomplete (report honestly)"
    VERDICT="CENSUS-INCOMPLETE"
fi

echo "stop" >&3
for i in $(seq 1 20); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
kill -9 "$SP" 2>/dev/null || true
exec 3>&-; kill "$HOLDER" 2>/dev/null || true
restore_seed
log "world restored; verdict=$VERDICT bands=$BANDS"
exit $([ "$VERDICT" = "CENSUS-CAPTURED" ] && echo 0 || echo 3)
