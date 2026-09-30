#!/usr/bin/env bash
# run_benchv2.sh — BENCH-V2 heavy stand (AG-433, wave-515, owner directive 2026-10-01).
# 20k+ force-loaded chunks SIMULTANEOUSLY in ALL THREE dimensions (overworld +
# nether + end) on a fresh fixed-seed world with Terralith+Tectonic (overworld),
# Incendium (nether), Stellarity (end) datapacks pinned from Modrinth (sha512),
# view/sim distance 32, bukkit spawn-limits at ceiling + per-player-mob-spawns
# OFF. Metrics: ch/s (drain-definition), TPS/MSPT samples (spark), entity-tick
# share (spark profiler artifact). All gates preregistered in
# /home/z/rounds/ROUND-515/work/AG-433/SPEC-BENCH-V2.md (dispatch BEFORE run).
set -u
WORK="${BENCH_WORK:-$PWD/run}"
mkdir -p "$WORK/server"
cd "$WORK/server"
log() { echo "[benchv2 $(date -u +%H:%M:%S)] $*"; }

# --- pins (sha512, Modrinth API 2026-10-01; anti-alias lesson dp13-16) ------
PURPUR_URL="https://api.purpurmc.org/v2/purpur/1.21.10/2535/download"
PURPUR_MD5="d48ae0c35eee5dca1e476dd5dc2ce2ea"
PURPUR_SHA256="4159783677b08b6395782e6150cb28646c70ed988b7948c09e01aa5a5e90f548"
TERRALITH_URL="https://cdn.modrinth.com/data/8oi3bsk5/versions/RFNry3l0/Terralith_1.21.5_v2.5.13.zip"
TERRALITH_SHA512="aea0cc28ca47a18ce0d8c82e25af08b52af01da80409c71a005e5799ca8c490b4cc293043758e14b6817ce2c54cc4f3da9c79f0d52c15c96dddcd989c3ac0dd8"
TECTONIC_URL="https://cdn.modrinth.com/data/lWDHr9jE/versions/pxgiJaJp/tectonic-datapack-3.0.29.zip"
TECTONIC_SHA512="e425783bb29ef1633af881e0174f21d7f0039591a7ba1dd4686d016a373680b46c0117219a66651b05aa0924c933f03ef8ab2ed68c801e431207b260feb6bdb0"
INCENDIUM_URL="https://cdn.modrinth.com/data/ZVzW5oNS/versions/gBoadsBv/Incendium_1.21.5_v5.4.9_UNSUPPORTED.zip"
INCENDIUM_SHA512="b8983657dae93206203422bf0d5365f26ba212caf018b743d96b0438ba3f16c1471473e37377ca3437b791e631ad42e62e8e29e87a4081b774771ba939cc0104"
STELLARITY_URL="https://cdn.modrinth.com/data/bZgeDzN8/versions/zudQ7s97/Stellarity-5.1.3.zip"
STELLARITY_SHA512="adb87f4e429086a66f4bb120a1e464514285a1ee1ecd2642f7d9e37c0346dc8c4818d38237c500d2c1ad8b6dca4e5e79e76e8ad0c080afcb36bdb63327cf12d9"

RADIUS_BLOCKS="${RADIUS_BLOCKS:-1136}"   # 1136 => 143x143 = 20449 chunks per dim
SEED="${BENCH_SEED:-351515}"
RUN_SECONDS="${RUN_SECONDS:-300}"
XMX="${SERVER_XMX:-10G}"
DIMS="${BENCH_DIMS:-minecraft:overworld,minecraft:the_nether,minecraft:the_end}"
STEP=256                                  # vanilla forceload cap: 16x16 chunks/cmd
FAIL=0

cat > "$WORK/run-env.txt" <<EOF
bench=v2 agent=AG-433 wave=515
purpur_url=$PURPUR_URL purpur_md5=$PURPUR_MD5 purpur_sha256=$PURPUR_SHA256
terralith=$TERRALITH_URL tectonic=$TECTONIC_URL
incendium=$INCENDIUM_URL stellarity=$STELLARITY_URL
radius_blocks=$RADIUS_BLOCKS seed=$SEED run_seconds=$RUN_SECONDS xmx=$XMX dims=$DIMS
runner_cpu_index=$RUNNER_CPU_INDEX
EOF

# --- G1/G2: downloads + hash pins -------------------------------------------
dl() { curl -sS -m 300 -L -o "$2" "$1" || { log "G-DL FAIL $1"; FAIL=1; }; }
dl "$PURPUR_URL" purpur.jar
md5now=$(md5sum purpur.jar | cut -d' ' -f1); sha256now=$(sha256sum purpur.jar | cut -d' ' -f1)
[ "$md5now" = "$PURPUR_MD5" ] && [ "$sha256now" = "$PURPUR_SHA256" ] && log "G-PURPUR PASS" || { log "G-PURPUR FAIL md5=$md5now sha256=$sha256now"; FAIL=1; }
dl "$TERRALITH_URL" terralith.zip;    echo "$TERRALITH_SHA512  terralith.zip"  | sha512sum -c - || FAIL=1
dl "$TECTONIC_URL"  tectonic.zip;     echo "$TECTONIC_SHA512   tectonic.zip"   | sha512sum -c - || FAIL=1
dl "$INCENDIUM_URL" incendium.zip;    echo "$INCENDIUM_SHA512  incendium.zip"  | sha512sum -c - || FAIL=1
dl "$STELLARITY_URL" stellarity.zip;  echo "$STELLARITY_SHA512 stellarity.zip" | sha512sum -c - || FAIL=1
[ "$FAIL" = "0" ] || { log "PIN-GATES FAILED — aborting before boot"; exit 42; }

# --- world fixture -----------------------------------------------------------
mkdir -p world/datapacks
cp terralith.zip tectonic.zip world/datapacks/
cp incendium.zip world/datapacks/
cp stellarity.zip world/datapacks/
cat > server.properties <<EOF
level-seed=$SEED
view-distance=32
simulation-distance=32
online-mode=false
spawn-protection=0
max-tick-time=1800000
enable-command-block=false
motd=BENCH-V2 AG-433
EOF
cat > bukkit.yml <<EOF
settings:
  allow-end: true
spawn-limits:
  monsters: 2000
  animals: 200
  water-animals: 200
  water-ambient: 100
  ambient: 100
ticks-per:
  monster-spawns: 1
  animal-spawns: 1
  water-animal-spawns: 1
  water-ambient-spawns: 1
  ambient-spawns: 1
EOF
mkdir -p config
cat > config/paper-world-defaults.yml <<EOF
spawning:
  per-player-mob-spawns: false
  despawn-ranges:
    monster:
      hard: 128
      soft: 96
EOF
log "fixture ready: datapacks=$(ls world/datapacks | tr '\n' ' ')"

# --- boot (FIFO console, run_world3.sh pattern) ------------------------------
mkfifo console.in 2>/dev/null || true
touch server-stdout.log
( tail -f console.in ) | java -Xms4G -Xmx"$XMX" -XX:+UseG1GC -jar purpur.jar --nogui > server-stdout.log 2>&1 &
SERVER_PID=$!
cmd() { timeout 5 sh -c 'printf "%s\n" "$1" > "$2"' _ "$*" "$WORK/server/console.in" 2>/dev/null || true; }
SEEN_DONE=0
for i in $(seq 1 900); do
  grep -qF "Done (" server-stdout.log && { SEEN_DONE=1; break; }
  grep -qiE "Failed to start|Exception in thread .main." server-stdout.log && break
  sleep 1
done
log "SEEN_DONE=$SEEN_DONE"
if [ "$SEEN_DONE" != "1" ]; then cmd "stop"; sleep 10; exit 43; fi

# --- G3: datapack enablement gate -------------------------------------------
cmd "datapack list"; sleep 6
DP_ENABLED=$(grep -cE "\[(file/)?(terralith|tectonic|incendium|stellarity)" server-stdout.log || true)
log "G-DATAPACKS enabled-markers=$DP_ENABLED (expect 4)"
[ "$DP_ENABLED" -ge 4 ] || { log "G-DATAPACKS FAIL (datapacks not all enabled)"; FAIL=1; }

# --- idle baseline -----------------------------------------------------------
for k in 1 2 3; do cmd "spark mspt"; sleep 4; done

# --- GEN phase: forceload sweep, ALL dims simultaneously ---------------------
FIRST_TS=""
for dim in ${DIMS//,/ }; do
  TILES=$(( (RADIUS_BLOCKS + STEP - 1) / STEP ))
  log "forceload sweep $dim tiles=${TILES}x${TILES}"
  for tx in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
    for tz in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
      [ -z "$FIRST_TS" ] && FIRST_TS=$(date +%s)
      cmd "execute in $dim run forceload add $tx $tz $((tx + STEP - 1)) $((tz + STEP - 1))"
      sleep 0.3
    done
  done
done
log "GEN_FIRST_TS=$FIRST_TS marked-cmds sent=$(( $(echo "$DIMS" | tr ',' '\n' | wc -l) * TILES * TILES ))"

# --- drain poll: MSPT back to near-idle => chunk system drained --------------
DRAIN_TS=""; DRAIN_TIMEOUT=1
for i in $(seq 1 120); do           # 120 x 10s = 1200s cap
  sleep 10
  cmd "spark mspt"
  ts=$(date +%s)
  med=$(tail -n 25 server-stdout.log | grep -iE "mspt" | tail -1 | grep -oE "[0-9]+\.[0-9]+" | head -1)
  if [ -n "$med" ] && [ -n "$FIRST_TS" ]; then
    log "DRAIN_POLL i=$i mspt_median=$med elapsed=$((ts - FIRST_TS))s"
    idle=$(grep -iE "mspt" server-stdout.log | head -1 | grep -oE "[0-9]+\.[0-9]+" | head -1)
    idle="${idle:-5.0}"
    pass=$(python3 -c "print(1 if float('$med') < max(1.5*float('$idle'), 50.0) else 0)")
    if [ "$pass" = "1" ]; then DRAIN_TS=$ts; DRAIN_TIMEOUT=0; log "DRAIN at +$((ts - FIRST_TS))s"; break; fi
  fi
  grep -qi "Exception in thread" server-stdout.log && { log "FATAL: main-thread exception during drain"; break; }
done
[ "$DRAIN_TIMEOUT" = "0" ] || log "WARN DRAIN-TIMEOUT (1200s) — ch/s reported as lower bound"

# --- SUSTAIN phase: spawn-storm window, TPS/MSPT sampling + profiler ---------
cmd "spark profiler start"; sleep 3
END=$(( $(date +%s) + RUN_SECONDS ))
while [ "$(date +%s)" -lt "$END" ]; do
  cmd "spark tps"; cmd "spark mspt"; sleep 15
done
cmd "spark profiler stop"; sleep 8
cmd "forceload remove all"; sleep 3
cmd "stop"
for i in $(seq 1 60); do kill -0 $SERVER_PID 2>/dev/null || break; sleep 2; done

# --- report ------------------------------------------------------------------
python3 "$GITHUB_WORKSPACE/bench/worldv2/report_benchv2.py" "$WORK/server" "$FIRST_TS" "$DRAIN_TS" || FAIL=1
log "FAIL=$FAIL (0 = all prereg gates passed)"
exit $FAIL
