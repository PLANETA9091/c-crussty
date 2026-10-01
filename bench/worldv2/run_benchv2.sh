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
TECTONIC_URL="https://cdn.modrinth.com/data/lWDHr9jE/versions/CmzMQNDL/tectonic-datapack-3.0.25.zip"
TECTONIC_SHA512="7b3c5dee391337b21bdfee5362f4a986417b4fffd6aa617dddaf3444b729b326e3651e7cd87cd7d68d39c301d28767648753784cd50f0a94eda904f85cd6d5bd"
INCENDIUM_URL="https://cdn.modrinth.com/data/ZVzW5oNS/versions/gBoadsBv/Incendium_1.21.5_v5.4.9_UNSUPPORTED.zip"
INCENDIUM_SHA512="b8983657dae93206203422bf0d5365f26ba212caf018b743d96b0438ba3f16c1471473e37377ca3437b791e631ad42e62e8e29e87a4081b774771ba939cc0104"
STELLARITY_URL="https://cdn.modrinth.com/data/bZgeDzN8/versions/zudQ7s97/Stellarity-5.1.3.zip"
STELLARITY_SHA512="adb87f4e429086a66f4bb120a1e464514285a1ee1ecd2642f7d9e37c0346dc8c4818d38237c500d2c1ad8b6dca4e5e79e76e8ad0c080afcb36bdb63327cf12d9"

RADIUS_BLOCKS="${RADIUS_BLOCKS:-1136}"   # 1136 => 143x143 = 20449 chunks per dim
SEED="${BENCH_SEED:-351515}"
RUN_SECONDS="${RUN_SECONDS:-300}"
FAKE_PLAYERS="${FAKE_PLAYERS:-0}"        # AG-342 spawn-lane leg: 0 = canon (vacuum), N>0 = N real ServerPlayers
XMX="${SERVER_XMX:-10G}"
DIMS="${BENCH_DIMS:-minecraft:overworld,minecraft:the_nether,minecraft:the_end}"
STEP=256                                  # vanilla forceload cap: 16x16 chunks/cmd
FAIL=0

cat > "$WORK/run-env.txt" <<EOF
bench=v2 agent=AG-12 wave=516 canon=AG-433/104+93async+248plugindim+342fakeplayers
purpur_url=$PURPUR_URL purpur_md5=$PURPUR_MD5 purpur_sha256=$PURPUR_SHA256
terralith=$TERRALITH_URL tectonic=$TECTONIC_URL
incendium=$INCENDIUM_URL stellarity=$STELLARITY_URL
radius_blocks=$RADIUS_BLOCKS seed=$SEED run_seconds=$RUN_SECONDS xmx=$XMX dims=$DIMS
fake_players=$FAKE_PLAYERS
runner_cpu_index=${RUNNER_CPU_INDEX:-0}
EOF

# --- G1/G2: downloads + hash pins -------------------------------------------
# x519 canary RED postmortem (run-36773277359): transient CDN drop on tectonic.zip
# killed the whole run (exit 42 before boot). Retry x3 with backoff — infra flakes
# must not burn a bench slot in a saturated queue.
dl() { local url="$1" out="$2" n; for n in 1 2 3; do curl -sS -m 300 -L -o "$out" "$url" && return 0; log "G-DL retry$n $url"; sleep $((n*5)); done; log "G-DL FAIL $url (3 attempts)"; FAIL=1; }
dl "$PURPUR_URL" purpur.jar
md5now=$(md5sum purpur.jar | cut -d' ' -f1); sha256now=$(sha256sum purpur.jar | cut -d' ' -f1)
[ "$md5now" = "$PURPUR_MD5" ] && [ "$sha256now" = "$PURPUR_SHA256" ] && log "G-PURPUR PASS" || { log "G-PURPUR FAIL md5=$md5now sha256=$sha256now"; FAIL=1; }
dl "$TERRALITH_URL" terralith.zip;    echo "$TERRALITH_SHA512  terralith.zip"  | sha512sum -c - || FAIL=1
dl "$TECTONIC_URL"  tectonic.zip;     echo "$TECTONIC_SHA512  tectonic.zip"   | sha512sum -c - || FAIL=1
dl "$INCENDIUM_URL" incendium.zip;    echo "$INCENDIUM_SHA512  incendium.zip"  | sha512sum -c - || FAIL=1
dl "$STELLARITY_URL" stellarity.zip;  echo "$STELLARITY_SHA512 stellarity.zip" | sha512sum -c - || FAIL=1
[ "$FAIL" = "0" ] || { log "PIN-GATES FAILED — aborting before boot"; exit 42; }

# --- world fixture -----------------------------------------------------------
mkdir -p world/datapacks
cp terralith.zip tectonic.zip world/datapacks/
cp incendium.zip world/datapacks/
cp stellarity.zip world/datapacks/
MAX_PLAYERS=$(( FAKE_PLAYERS > 0 ? FAKE_PLAYERS + 8 : 10000000 ))
cat > server.properties <<EOF
level-seed=$SEED
view-distance=32
simulation-distance=32
online-mode=false
spawn-protection=0
max-tick-time=1800000
enable-command-block=false
max-players=$MAX_PLAYERS
motd=BENCH-V2 AG-433
EOF
echo "eula=true" > eula.txt
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

# --- AG-12 canon: kernel materialize pre-pass (plugin compilation, both legs) -
KERNEL_JAR="versions/1.21.10/purpur-1.21.10.jar"
if [ ! -s "$KERNEL_JAR" ]; then
  log "benchv2-ag12: materializing kernel (eula-less paperclip pass, exits pre-main — NOT a boot)"
  rm -f eula.txt
  ( timeout 300 java -jar purpur.jar --nogui > "$WORK/pclip-materialize.log" 2>&1 || true )
  [ -s "$KERNEL_JAR" ] || { log "G-KERNEL FAIL (no $KERNEL_JAR)"; exit 44; }
  echo "eula=true" > eula.txt
fi

# --- AG-342: fake-player spawn-lane fixture (BENCH-4 pattern, sanctioned) ----
# Without players natural spawning is structurally off (0 spawnable chunks,
# task170/S7-99 precedent) -> the directive's "spawn in ceiling" leg is
# vacuous in canon. FAKE_PLAYERS>0 injects N REAL ServerPlayers (eula-less
# paperclip materialization pass exits PRE-main — NOT a boot), compile is
# javac-only. FAKE_PLAYERS=0 skips all of this (byte-identical canon).
if [ "$FAKE_PLAYERS" -gt 0 ]; then
  log "benchv2-ag342: materializing kernel (eula-less paperclip pass, exits pre-main — NOT a boot)"
  rm -f eula.txt
  ( timeout 300 java -jar purpur.jar --nogui > "$WORK/pclip-materialize.log" 2>&1 || true )
  KERNEL_JAR="versions/1.21.10/purpur-1.21.10.jar"
  [ -s "$KERNEL_JAR" ] || { log "G-KERNEL FAIL (no $KERNEL_JAR)"; exit 44; }
  command -v javac >/dev/null || { log "G-JAVAC FAIL"; exit 44; }
  FP_SRC="$GITHUB_WORKSPACE/bench/worldv2/fakeplayers"
  [ -f "$FP_SRC/BenchFakePlayersPlugin.java" ] || { log "G-FPSRC FAIL"; exit 44; }
  FIX_CP="$PWD/$KERNEL_JAR"
  while IFS= read -r j; do FIX_CP="$FIX_CP:$j"; done < <(find libraries -name '*.jar' 2>/dev/null)
  FP_CLASSES="$WORK/fpclasses"; rm -rf "$FP_CLASSES"; mkdir -p "$FP_CLASSES"
  javac --release 21 -proc:none -cp "$FIX_CP" -d "$FP_CLASSES" "$FP_SRC/BenchFakePlayersPlugin.java" || { log "G-FPCOMPILE FAIL"; exit 44; }
  cp "$FP_SRC/plugin.yml" "$FP_CLASSES/"
  mkdir -p plugins
  SRV_PLUGINS="$PWD/plugins"   # AG-395: capture BEFORE cd — $PWD inside subshell = $FP_CLASSES after cd, jar landed in fpclasses/plugins/ (blocker #5, run-36794417339: Initialized 0 plugins)
  ( cd "$FP_CLASSES" && jar cf "$SRV_PLUGINS/BenchFakePlayers.jar" . ) || { log "G-FPJAR FAIL"; exit 44; }
  export BENCH_FAKE_PLAYERS="$FAKE_PLAYERS" BENCH_FORCELOAD_RADIUS="$RADIUS_BLOCKS" BENCH_FAKE_DISTRIBUTE=1
  log "benchv2-ag342: plugin staged ($(stat -c%s plugins/BenchFakePlayers.jar) B); N=$FAKE_PLAYERS distribute=3-dim"
  echo "eula=true" > eula.txt
fi

# --- AG-12 canon: DimForceload plugin — cross-dim chunk tickets (AG-248 F1) --
# vanilla /forceload is Overworld-only and console sweep blocks the main thread
# (FAKE-GREEN class F, run 36756489239). Plugin marks tickets via
# World.addPluginChunkTicket from dimload.start — async marking, main thread
# free, all three dims simultaneously; /dimchunks = G-DIM census gate.
command -v javac >/dev/null || { log "G-JAVAC FAIL"; exit 44; }
DF_SRC="$GITHUB_WORKSPACE/bench/worldv2"
[ -f "$DF_SRC/DimForceloadPlugin.java" ] || { log "G-DFSRC FAIL"; exit 44; }
DF_CP="$PWD/$KERNEL_JAR"
while IFS= read -r j; do DF_CP="$DF_CP:$j"; done < <(find libraries -name '*.jar' 2>/dev/null)
DF_CLASSES="$WORK/dfclasses"; rm -rf "$DF_CLASSES"; mkdir -p "$DF_CLASSES"
javac --release 21 -proc:none -cp "$DF_CP" -d "$DF_CLASSES" "$DF_SRC/DimForceloadPlugin.java" || { log "G-DFCOMPILE FAIL"; exit 44; }
cp "$DF_SRC/dimforceload-plugin.yml" "$DF_CLASSES/"
mkdir -p plugins
SRV_PLUGINS="$PWD/plugins"   # AG-395: capture BEFORE cd — $PWD inside subshell = $DF_CLASSES after cd (blocker #5: jar landed in dfclasses/plugins/, stat cannot statx, Initialized 0 plugins, /dimchunks unknown -> G-DIM empty FAIL; run-36794417339)
( cd "$DF_CLASSES" && jar cf "$SRV_PLUGINS/DimForceload.jar" . ) || { log "G-DFJAR FAIL"; exit 44; }
[ -s plugins/DimForceload.jar ] || { log "G-DFJAR FAIL (jar not in server plugins/ — instrumentation would run DEAD)"; exit 44; }   # AG-395 hard gate: silent plugin absence = FAKE-GREEN class F
export DIM_RADIUS_CHUNKS=$(( (RADIUS_BLOCKS + 15) / 16 ))
export DIM_WORLDS="world,world_nether,world_end"
log "benchv2-ag12: DimForceload staged ($(stat -c%s plugins/DimForceload.jar) B) radius_chunks=$DIM_RADIUS_CHUNKS worlds=$DIM_WORLDS"

# --- AG-12: async wall-clock heartbeat sampler (AG-234 principle) ------------
# samples wall-clock every 2s INDEPENDENT of server responsiveness — 0 lines
# while run claims success = FAKE-GREEN class F detector (heartbeat lives).
HEARTBEAT="$PWD/BENCHV2_HEARTBEAT.log"; rm -f "$HEARTBEAT"
touch .hb.keep
( while [ -f .hb.keep ]; do echo "t=$(date +%s.%N) hb=1" >> "$HEARTBEAT"; sleep 2; done ) &
HB_PID=$!

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
# AG-395 fix (blocker #3): grep -c counts LINES; Paper lists all 4 packs on ONE line
# ("There are 7 data pack(s) enabled: [vanilla], [file/bukkit], [file/terralith.zip (world)]...")
# -> enabled-markers=1 false-fail. Count OCCURRENCES (grep -o | wc -l); run-36794417339 proof.
DP_ENABLED=$(grep -oE "\[(file/)?(terralith|tectonic|incendium|stellarity)" server-stdout.log | wc -l)
log "G-DATAPACKS enabled-markers=$DP_ENABLED (expect 4)"
[ "$DP_ENABLED" -ge 4 ] || { log "G-DATAPACKS FAIL (datapacks not all enabled)"; FAIL=1; }

# --- idle baseline -----------------------------------------------------------
for k in 1 2 3; do cmd "spark mspt"; sleep 4; done

# --- GEN phase: forceload sweep, ALL dims simultaneously ---------------------
FIRST_TS=$(date +%s)
touch dimload.start
log "GEN_FIRST_TS=$FIRST_TS dims=$DIMS marked=tickets x$((DIM_RADIUS_CHUNKS*2+1))^2/dim (async, AG-93/248 principle)"
sleep 5
cmd "dimchunks"; sleep 3   # G-DIM early census from plugin

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
# AG-342 census instrument (both legs, identical): per-dim entity counts once
# per 60s via scoreboard census (console-only, no plugin dependency) — proves
# or refutes the spawn-vacuum hypothesis with the SAME instrument in A and B.
cmd "scoreboard objectives add benchv2c dummy"; sleep 2
LAST_CENSUS=0
END=$(( $(date +%s) + RUN_SECONDS ))
while [ "$(date +%s)" -lt "$END" ]; do
  cmd "spark tps"; cmd "spark mspt"
  NOW=$(date +%s)
  if [ $((NOW - LAST_CENSUS)) -ge 60 ]; then
    log "CENSUS round (t=$NOW)"
    for pair in "minecraft:overworld:#c_ov" "minecraft:the_nether:#c_ne" "minecraft:the_end:#c_en"; do
      DIM="${pair%:*}"; CNT="${pair##*:}"   # AG-395 fix (blocker #6): %%:* cut at FIRST colon -> DIM="minecraft" -> execute in minecraft => "Unknown dimension 'minecraft:minecraft'" (run-36794417339); %:* keeps "minecraft:overworld"
      cmd "scoreboard players set $CNT benchv2c 0"
      cmd "execute in $DIM as @e[type=!minecraft:player] run scoreboard players add $CNT benchv2c 1"
      cmd "scoreboard players get $CNT benchv2c"
    done
    cmd "list"
    LAST_CENSUS=$NOW
  fi
  sleep 15
done
cmd "spark profiler stop"; sleep 8
cmd "forceload remove all"; sleep 3
cmd "stop"
for i in $(seq 1 60); do kill -0 $SERVER_PID 2>/dev/null || break; sleep 2; done

rm -f .hb.keep; kill $HB_PID 2>/dev/null || true

# --- report ------------------------------------------------------------------
python3 "$GITHUB_WORKSPACE/bench/worldv2/report_benchv2.py" "$WORK/server" "$FIRST_TS" "$DRAIN_TS" || FAIL=1
python3 "$GITHUB_WORKSPACE/bench/worldv2/census_ag342.py" "$WORK/server" || true

# --- AG-12 canon gates: G-DIM (3-dim loaded) + G-HB (async heartbeat) --------
DIMGATE=$(grep -oE "G-DIM world=[a-z_]+ loaded=[0-9]+" server-stdout.log | tail -6)
log "G-DIM census: $DIMGATE"
OV=$(echo "$DIMGATE" | grep -oE "world=world loaded=[0-9]+" | grep -oE "[0-9]+" | tail -1); OV=${OV:-0}
NE=$(echo "$DIMGATE" | grep -oE "world=world_nether loaded=[0-9]+" | grep -oE "[0-9]+" | tail -1); NE=${NE:-0}
EN=$(echo "$DIMGATE" | grep -oE "world=world_end loaded=[0-9]+" | grep -oE "[0-9]+" | tail -1); EN=${EN:-0}
TOTAL=$((OV + NE + EN))
if [ "$OV" -ge 19000 ] && [ "$NE" -ge 19000 ] && [ "$EN" -ge 19000 ] && [ "$TOTAL" -ge 60000 ]; then
  log "G-DIM PASS ov=$OV ne=$NE en=$EN total=$TOTAL"
else
  log "G-DIM FAIL ov=$OV ne=$NE en=$EN total=$TOTAL (silent-empty dims impossible gate)"; FAIL=1
fi
HB_LINES=$(wc -l < "$HEARTBEAT" 2>/dev/null || echo 0)
if [ "$HB_LINES" -ge 60 ]; then
  log "G-HB PASS heartbeat_lines=$HB_LINES"
else
  log "G-HB FAIL heartbeat_lines=$HB_LINES (<60 — sampler starved, FAKE-GREEN class F)"; FAIL=1
fi
{ echo ""; echo "## AG-12 canon gates x516"; echo "G-DIM: ov=$OV ne=$NE en=$EN total=$TOTAL (gate per-dim>=19000 total>=60000)"; echo "G-HB: heartbeat_lines=$HB_LINES (gate >=60, wall-clock async sampler)"; echo "G-TECTONIC: 3.0.25 sha512 7b3c5dee repinned (FATAL-alias 3.0.29 purged)"; echo "G-FP: fake_players=$FAKE_PLAYERS (0=canon vacuum byte-identical)"; } >> BENCHV2.md 2>/dev/null || true
log "FAIL=$FAIL (0 = all prereg gates passed)"
exit $FAIL
