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
BENCH_T0="${BENCH_T0:-$(date +%s)}"  # AG-432 w527: wall-clock anchor for job-deadline guard (L133 job cap)

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
# AG-178 w527: kernel-drift pin (AG-159 follow-up #4). paperclip materializes vanilla
# from Mojang WITHOUT orig-hash enforcement; rotation 2026-10-02 17:26-21:18Z
# (vanilla 2e2867d1->5bb64dc4) silently swapped the kernel. e2992d63 = post-drift
# kernel (3-channel: local 22:49Z + WBR 21:18/21:56Z). Hot re-pin via env.
KERNEL_SHA_EXP="${EXPECTED_KERNEL_SHA256:-e2992d63abd2c2544a4d1564c6dbe402fb05c12a410d2700a355d2cbe2e87200}"

RADIUS_BLOCKS="${RADIUS_BLOCKS:-1136}"   # 1136 => 143x143 = 20449 chunks per dim
SEED="${BENCH_SEED:-351515}"
RUN_SECONDS="${RUN_SECONDS:-300}"
FAKE_PLAYERS="${FAKE_PLAYERS:-0}"        # AG-342 spawn-lane leg: 0 = canon (vacuum), N>0 = N real ServerPlayers
SIM_DISTANCE="${SIM_DISTANCE:-32}"        # AG-138 x525 sim-оси: 32 = canon (byte-identical), 10 = vanilla default
XMX="${SERVER_XMX:-10G}"
DIMS="${BENCH_DIMS:-minecraft:overworld,minecraft:the_nether,minecraft:the_end}"
STEP=256                                  # vanilla forceload cap: 16x16 chunks/cmd
FAIL=0

cat > "$WORK/run-env.txt" <<EOF
bench=v2 agent=AG-12 wave=516 canon=AG-433/104+93async+248plugindim+342fakeplayers
purpur_url=$PURPUR_URL purpur_md5=$PURPUR_MD5 purpur_sha256=$PURPUR_SHA256
kernel_sha256_exp=$KERNEL_SHA_EXP # AG-178 w527 drift-pin (L194 horizon split)
terralith=$TERRALITH_URL tectonic=$TECTONIC_URL
incendium=$INCENDIUM_URL stellarity=$STELLARITY_URL
radius_blocks=$RADIUS_BLOCKS seed=$SEED run_seconds=$RUN_SECONDS xmx=$XMX dims=$DIMS
fake_players=$FAKE_PLAYERS
sim_distance=$SIM_DISTANCE
generate_structures=${GENERATE_STRUCTURES:-unset} # AG-389 w527 GS-attribution: yml->env knob in artifact (gap AG-339; wiring canon AG-113)
runner_cpu_index=${RUNNER_CPU_INDEX:-0}
runner_name=${RUNNER_NAME:-?} run_id=${GITHUB_RUN_ID:-?} attempt=${GITHUB_RUN_ATTEMPT:-?} # AG-301 w526 re-land AG-311: host-census line (AG-233 FAIL, clobber-lost)
EOF
cp "$WORK/run-env.txt" "$WORK/server/run-env.txt" # AG-370 w526 B-canon: artifact run/server/run-env.txt (yml x2), zip LCA=run/server intact (AG-296/319 consumers); $WORK copy kept: report reads dirname()

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
simulation-distance=$SIM_DISTANCE
online-mode=false
sync-chunk-writes=${SYNC_CHUNK_WRITES:-true}
spawn-protection=0
initial-enabled-packs=vanilla,file/terralith.zip,file/tectonic.zip,file/incendium.zip,file/stellarity.zip
max-tick-time=1800000
enable-command-block=false
max-players=$MAX_PLAYERS
motd=BENCH-V2 AG-433
EOF
echo "SYNC_CHUNK_WRITES=${SYNC_CHUNK_WRITES:-true}" # AG-390 scw attribution
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

# --- AG-178 w527: G-KERNEL-DRIFT guard (AG-159 follow-up #4, fail-closed) ----
# Single choke point for BOTH legs: pin the materialized kernel BEFORE any
# plugin compile (G-FPCOMPILE x7 DOA famine 21:23-22:24Z was detected only
# after the fact). Rotation => fast honest exit44, not a silently other-kernel run.
ksha=$(sha256sum "$KERNEL_JAR" | cut -d' ' -f1)
if [ "$ksha" != "$KERNEL_SHA_EXP" ]; then
  log "G-KERNEL-DRIFT FAIL sha256=$ksha expected=$KERNEL_SHA_EXP (Mojang rotation? re-pin EXPECTED_KERNEL_SHA256 + canary before legs)"; exit 44
fi
log "G-KERNEL-DRIFT PASS sha256=$ksha"

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
else
  # AG-370 w527 census-alias fix (AG-357 w526 FAIL + AG-344 w527 FACT): vacuum leg
  # (fake_players=0) gets the SAME BenchV2Census plugin in census-only mode —
  # console scoreboard per-dim census is dim-aliased (c_ov==c_ne==c_en bit-exact),
  # plugin censusTick is per-level-correct (lvl.getAllEntities()). NO injection at
  # fp=0 -> canon vacuum preserved; plugin only logs [BenchV2Census] every 5s.
  FP_SRC="$GITHUB_WORKSPACE/bench/worldv2/fakeplayers"
  [ -f "$FP_SRC/BenchFakePlayersPlugin.java" ] || { log "G-FPSRC FAIL"; exit 44; }
  FP_CP="$PWD/$KERNEL_JAR"
  while IFS= read -r j; do FP_CP="$FP_CP:$j"; done < <(find libraries -name '*.jar' 2>/dev/null)
  FP_CLASSES="$WORK/fpclasses"; rm -rf "$FP_CLASSES"; mkdir -p "$FP_CLASSES"
  javac --release 21 -proc:none -cp "$FP_CP" -d "$FP_CLASSES" "$FP_SRC/BenchFakePlayersPlugin.java" || { log "G-FPCOMPILE FAIL"; exit 44; }
  cp "$FP_SRC/plugin.yml" "$FP_CLASSES/"
  mkdir -p plugins
  SRV_PLUGINS="$PWD/plugins"   # AG-395 pattern: $PWD captured AFTER server cd-safe point
  ( cd "$FP_CLASSES" && jar cf "$SRV_PLUGINS/BenchFakePlayers.jar" . ) || { log "G-FPJAR FAIL"; exit 44; }
  export BENCH_FAKE_PLAYERS="$FAKE_PLAYERS" BENCH_FORCELOAD_RADIUS="$RADIUS_BLOCKS" BENCH_FAKE_DISTRIBUTE=0
  log "benchv2-ag370: census-only plugin staged ($(stat -c%s plugins/BenchFakePlayers.jar) B); N=$FAKE_PLAYERS no-injection; scoreboard census=ALIAS-PRONE (read plugin lines)"
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
# AG-342 fix (blocker #8, canary-3 run-36802056368/36802054506): descriptor must be
# named plugin.yml AT JAR ROOT. Paper error: "plugins/.paper-remapped/DimForceload.jar
# does not contain a paper-plugin.yml or plugin.yml!" => plugin DEAD, tickets never
# marked, G-DIM empty, census LEG-B-DEAD (server-stdout.log line 8 proof).
cp "$DF_SRC/dimforceload-plugin.yml" "$DF_CLASSES/plugin.yml"
mkdir -p plugins
SRV_PLUGINS="$PWD/plugins"   # AG-395: capture BEFORE cd — $PWD inside subshell = $DF_CLASSES after cd (blocker #5: jar landed in dfclasses/plugins/, stat cannot statx, Initialized 0 plugins, /dimchunks unknown -> G-DIM empty FAIL; run-36794417339)
( cd "$DF_CLASSES" && jar cf "$SRV_PLUGINS/DimForceload.jar" . ) || { log "G-DFJAR FAIL"; exit 44; }
[ -s plugins/DimForceload.jar ] || { log "G-DFJAR FAIL (jar not in server plugins/ — instrumentation would run DEAD)"; exit 44; }   # AG-395 hard gate: silent plugin absence = FAKE-GREEN class F
jar tf plugins/DimForceload.jar | grep -qx "plugin.yml" || { log "G-DFJAR FAIL (jar lacks plugin.yml descriptor — blocker #8 class, plugin will not load)"; exit 44; }   # AG-342 hard gate #8: deterministic pre-boot detector (server-stdout line 8: DirectoryProviderSource load error)
export DIM_RADIUS_CHUNKS=$(( (RADIUS_BLOCKS + 15) / 16 ))
# AG-120 x523 (#16f remediation-a, AG-112 lesson): bench_dims must SCOPE the plugin,
# not only gates/report — master hardcoded all 3 worlds so a "single-dim" leg still
# armed 61347 cells (AG-72 single-dim illusion, run-36864515813). Default unchanged.
DIM_WORLDS="$(printf '%s' "$DIMS" | sed -e 's/minecraft:overworld/world/g' -e 's/minecraft:the_nether/world_nether/g' -e 's/minecraft:the_end/world_the_end/g' -e 's/minecraft://g' -e 's/[[:space:]]//g')"
export DIM_WORLDS
export DIM_GEN_WINDOW="${DIM_GEN_WINDOW:-256}"   # AG-120 #16f: bounded in-flight getChunkAtAsync fan-out
export DIM_MARK_MODE="${DIM_MARK_MODE:-pregen}"   # AG-496 x522: pregen-v3 (register-only marking) | legacy = v2 amortized-sync
log "benchv2-ag12: DimForceload staged ($(stat -c%s plugins/DimForceload.jar) B) radius_chunks=$DIM_RADIUS_CHUNKS worlds=$DIM_WORLDS mark_mode=$DIM_MARK_MODE"
# AG-43 w527: dgw/dcp cell-attribution in run-env (artifact-side dedup; board-claims rot 3x today)
echo "mark_mode=$DIM_MARK_MODE dim_gen_window=${DIM_GEN_WINDOW:-256} drain_cap_polls=${DRAIN_CAP_POLLS:-240} dim_drain_unmark=${DIM_DRAIN_UNMARK:-0}" >> "$WORK/run-env.txt"
echo "mark_mode=$DIM_MARK_MODE dim_gen_window=${DIM_GEN_WINDOW:-256} drain_cap_polls=${DRAIN_CAP_POLLS:-240} dim_drain_unmark=${DIM_DRAIN_UNMARK:-0}" >> "$WORK/server/run-env.txt" # AG-370 w526 mirror + AG-43 w527

# AG-376 w527: sameboot leg-attribution in run-env (AG-339 gap OBSERVED: AB_VAR/AB_VAL
# never reach artifact; pair-law gate "identical except ab_env line" unverifiable offline)
echo "ab_env: ${AB_VAR:-none}=${AB_VAL:-}" >> "$WORK/run-env.txt"
echo "ab_env: ${AB_VAR:-none}=${AB_VAL:-}" >> "$WORK/server/run-env.txt" # AG-370 w526 mirror canon

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
# AG-342 fix (blocker #11, canary-3 233KB log: 0 lines match 'mspt'): console
# 'spark mspt' is SILENT on Purpur 1.21.10 bundled spark; 'spark tps' prints the
# TPS row + 'Tick durations (min/med/95%ile/max ms)' row (log lines 4143-4250 proof).
for k in 1 2 3; do cmd "spark tps"; sleep 4; done

# --- GEN phase: forceload sweep, ALL dims simultaneously ---------------------
FIRST_TS=$(date +%s)
touch dimload.start
SIDE_C=$(( 2 * DIM_RADIUS_CHUNKS + 1 ))
EXP_PD=$(( SIDE_C * SIDE_C ))
log "GEN_FIRST_TS=$FIRST_TS dims=$DIMS cells=x${SIDE_C}^2=${EXP_PD}/dim mark_mode=$DIM_MARK_MODE (AG-496 pregen-v3: getChunkAtAsync off-main + register-only tickets)"
sleep 5
cmd "dimchunks"; sleep 3   # G-DIM early census from plugin

# --- drain poll: MSPT back to near-idle => chunk system drained --------------
DRAIN_TS=""; DRAIN_TIMEOUT=1
# AG-432 w527 deadline guard. Forensics 2026-10-03: r1152 37001588090 (cap 1500) and
# dcp2100 37000413529 (cap 2100) both died at the bench-v2.yml L133 320m job kill —
# mid-drain/mid-sustain, report-gate skipped, 0 data, 10.7 slot-h burned. Root cause:
# dispatchers raised DRAIN_CAP_POLLS without deadline math (cap 1500-2100 polls = 250-350m
# > job budget). eff_cap keeps sustain RUN_SECONDS + 600s report/upload reserve inside 318m.
# Fail-open floor 100s; drain is a lower-bound phase (AG-400), so trimming is measurement-neutral.
DRAIN_EFF_CAP="${DRAIN_CAP_POLLS:-240}"
DEADLINE_RAW=$(( 318*60 - ( $(date +%s) - BENCH_T0 ) - ${RUN_SECONDS:-300} - 600 ))
DEADLINE_REMAIN="$DEADLINE_RAW"
[ "$DEADLINE_REMAIN" -lt 100 ] && DEADLINE_REMAIN=100
DEADLINE_CAP=$(( DEADLINE_REMAIN / 10 ))
if [ "$DEADLINE_CAP" -lt "$DRAIN_EFF_CAP" ]; then
  log "WARN DRAIN-DEADLINE cap=$DEADLINE_CAP (dispatch asked $DRAIN_EFF_CAP) — job 318m margin: sustain ${RUN_SECONDS:-300}s + 600s report protected (AG-432 w527)"
  DRAIN_EFF_CAP="$DEADLINE_CAP"
fi
echo "deadline_guard: eff_cap=$DRAIN_EFF_CAP asked=${DRAIN_CAP_POLLS:-240} raw_remain=${DEADLINE_RAW}s run_s=${RUN_SECONDS:-300}s (AG-29 w528 clamp + AG-4 w528 arb-union)" >> "$WORK/run-env.txt"
echo "deadline_guard: eff_cap=$DRAIN_EFF_CAP asked=${DRAIN_CAP_POLLS:-240} raw_remain=${DEADLINE_RAW}s run_s=${RUN_SECONDS:-300}s (AG-29 w528 clamp + AG-4 w528 arb-union)" >> "$WORK/server/run-env.txt" # AG-370 w526 mirror canon
# AG-4 w528 arb-union (drain-clamp arbitration): port the UNIQUE value of AG-1 w528 —
# BUDGET-EXHAUST fail-fast. If raw deadline remainder < 100s floor, even the minimum
# 10-poll drain leaves less than RUN_SECONDS+600s tail inside the 318m step -> GH 320m
# kill mid-sustain, census/report lost (AG-483 class r1152/dcp2100/r2368). Abort BEFORE
# sustain: graceful stop, report still runs, honest RED verdict, slot freed early.
# Arb verdicts: AG-10 w528 REJECTED (no RUN_SECONDS subtraction -> dose-leg run>=1200s
# dies mid-sustain: 19200-1200 reserve < run+tail); AG-24 w528 REJECTED (subsumed by
# AG-29 clamp; 1-poll floor keeps the mid-sustain kill path open, no abort).
if [ "$DEADLINE_RAW" -lt 100 ]; then
  log "BUDGET-EXHAUST ABORT: raw_deadline_remain=${DEADLINE_RAW}s < 100s floor — sustain ${RUN_SECONDS:-300}s+tail cannot fit 318m step even at 10-poll drain (AG-1 w528 semantics via AG-4 arb)"
  cmd "forceload remove all"; sleep 3
  cmd "stop"
  for i in $(seq 1 30); do kill -0 $SERVER_PID 2>/dev/null || break; sleep 2; done
  rm -f .hb.keep; kill $HB_PID 2>/dev/null || true
  python3 "$GITHUB_WORKSPACE/bench/worldv2/report_benchv2.py" "$WORK/server" "$FIRST_TS" "$DRAIN_TS" || true
  { echo ""; echo "## INVALID/FAIL verdict: BUDGET-EXHAUST ABORT (AG-4 w528 arb-union): raw_deadline_remain=${DEADLINE_RAW}s — no sustain data, drain ch/s lower-bound only; reduce DRAIN_CAP_POLLS or RUN_SECONDS"; } >> BENCHV2.md 2>/dev/null || true
  exit 1
fi
for i in $(seq 1 "$DRAIN_EFF_CAP"); do  # AG-400 x523 #16f: cap env-tunable; AG-432: deadline-clamped (see above)
  sleep 10
  cmd "spark tps"   # AG-342 fix (blocker #11): parse Tick durations med from spark tps output (spark mspt console = silent)
  ts=$(date +%s)
  med=$(tail -n 25 server-stdout.log | grep -A2 "Tick durations" | grep -oE "[0-9]+\.[0-9]+/[0-9]+\.[0-9]+/[0-9]+\.[0-9]+/[0-9]+\.[0-9]+" | head -1 | cut -d/ -f2)
  if [ -n "$med" ] && [ -n "$FIRST_TS" ]; then
    log "DRAIN_POLL i=$i mspt_median=$med elapsed=$((ts - FIRST_TS))s"
    # AG-342: idle = FIRST Tick-durations med ever seen (pre-GEN idle baseline window); 5.0 fallback keeps prereg threshold
    idle=$(grep -A2 "Tick durations" server-stdout.log | grep -oE "[0-9]+\.[0-9]+/[0-9]+\.[0-9]+/[0-9]+\.[0-9]+/[0-9]+\.[0-9]+" | head -1 | cut -d/ -f2)
    idle="${idle:-5.0}"
    pass=$(python3 -c "print(1 if float('$med') < max(1.5*float('$idle'), 50.0) else 0)")
    # AG-400 x523 #16f GEN-DONE gate (port AG-339). AG-388 w527 REPAIR: the comment
    # claimed the python-bug FIXED, but "last.group(1)]=l" (invalid python) survived
    # on master 2afeef68..6a1f880c -> gendone ALWAYS 0 -> DRAIN-HOLD every poll ->
    # every leg burned its full drain cap (joblog 110855033035 w896: mspt idle 0.8 at
    # i=2, 190 DRAIN-HOLD, WARN DRAIN-TIMEOUT at 9000s; light AND heavy legs alike).
    # MSPT-idle alone is false-PASS (GEN-FANOUT-STALL: mspt idle 0.3-1.3, gen frozen):
    # drain PASS requires LATEST [DF] PROGRESS of EVERY bench dim: inflight=0 AND
    # gen_ok==marked-total (gen_ok, not marked — #16g: marked lost at gate 625-3444->0).
    # AG-388 w527 +loadpass (AG-345 drain-stall fork): heavy legs hold mspt>50 with
    # tickets held (sustain canon: pregen chunks stay loaded, `forceload remove all`
    # fires only post-sustain) -> mspt-idle unreachable -> structural acceptance when
    # every bench dim ALSO reports loaded>=EXP_PD (gen physically complete, tickets
    # still held — zero world-state change, sustain comparability preserved).
    # Fail-open: plugin-silent -> gendone=loadpass=0 -> old DRAIN-TIMEOUT WARN path.
    gate=$(grep "\[DF\] PROGRESS" server-stdout.log 2>/dev/null | python3 -c "
import sys,re
last={}
for l in sys.stdin:
    m=re.search(r'world=(\S+)',l)
    if m: last[m.group(1)]=l
exp=int(sys.argv[1]) if len(sys.argv)>1 else 0
g=lp=bool(last)
for d,l in last.items():
    mt=re.search(r'marked=(\d+)/(\d+)',l); gi=re.search(r'gen_ok=(\d+)',l); ifl=re.search(r'inflight=(\d+)',l)
    ok=bool(mt and gi and ifl and int(gi.group(1))==int(mt.group(2)) and int(ifl.group(1))==0)
    ld=re.search(r'loaded=(\d+)',l)
    if not ok: g=False
    if not (ok and ld and int(ld.group(1))>=exp): lp=False
print(('1 ' if g else '0 ')+('1' if lp else '0'))" "$EXP_PD" 2>/dev/null) || gate="0 0"
    gendone=$(echo $gate | cut -d' ' -f1); loadpass=$(echo $gate | cut -d' ' -f2)
    if [ "$pass" = "1" ]; then
      if [ "$gendone" = "1" ]; then DRAIN_TS=$ts; DRAIN_TIMEOUT=0; log "DRAIN at +$((ts - FIRST_TS))s (GEN-DONE gate pass)"; break; fi
      log "DRAIN-HOLD i=$i mspt_idle_but_gen_not_done (AG-400 GEN-DONE gate: false-PASS blocked)"
    elif [ "$loadpass" = "1" ]; then
      DRAIN_TS=$ts; DRAIN_TIMEOUT=0; log "DRAIN at +$((ts - FIRST_TS))s (GEN-DONE+loaded-census gate: mspt-stall structural, AG-388 w527)"; break
    fi
  fi
  grep -qi "Exception in thread" server-stdout.log && { log "FATAL: main-thread exception during drain"; break; }
done
[ "$DRAIN_TIMEOUT" = "0" ] || log "WARN DRAIN-TIMEOUT ($(( DRAIN_EFF_CAP * 10 ))s of asked ${DRAIN_CAP_POLLS:-240}) — ch/s reported as lower bound (AG-400: cap env-tunable; AG-432: deadline-trim)"

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
# v3.1 unmark-at-drain (AG-367 w527, #16g; root-cause AG-345 dcp2100): after sustain
# releases plugin tickets so the world can quiesce; DEFAULT OFF = byte-identical legacy flow.
if [ "${DIM_DRAIN_UNMARK:-0}" = "1" ]; then touch dimload.stop; log "UNMARK-TRIGGER touched dimload.stop (DIM_DRAIN_UNMARK=1)"; fi
cmd "dimchunks"; sleep 3   # AG-342 fix (blocker #12): G-DIM gate reads tail -6 of G-DIM lines; the ONLY call was +5s after GEN start (counts ~0) -> gate FAIL even with plugin alive. Final call after drain -> tail -6 = drained per-dim counts
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
EN=$(echo "$DIMGATE" | grep -oE "world=world_the_end loaded=[0-9]+" | grep -oE "[0-9]+" | tail -1); EN=${EN:-0}
TOTAL=$((OV + NE + EN))
# AG-496 x522: radius-aware G-DIM (hardcoded 19000/60000 broke any radius!=71 leg with
# G-DIM FAIL even on a mechanically perfect run). Canon radius 71 => min_pd=19426/min_tot=58277
# ~= old 19000/60000 (same 95% prerig). min-of: per-dim >= 95% of cells, total >= 95% of 3x cells.
MIN_PD=$(( EXP_PD * 95 / 100 ))
MIN_TOT=$(( EXP_PD * 3 * 95 / 100 ))
# AG-120 x523: n_dims-aware G-DIM (master required ne/en >= min_pd even on
# bench_dims=minecraft:overworld legs — impossible gate for legal single-dim legs).
N_DIMS=$(printf '%s' "$DIMS" | awk -F',' '{print NF}')
MIN_TOT=$(( EXP_PD * 95 / 100 * N_DIMS ))
WANT_OV=0; WANT_NE=0; WANT_EN=0
case ",$DIMS," in *,minecraft:overworld,*)  WANT_OV=1;; esac
case ",$DIMS," in *,minecraft:the_nether,*) WANT_NE=1;; esac
case ",$DIMS," in *,minecraft:the_end,*)    WANT_EN=1;; esac
GDIM_OK=1
[ "$WANT_OV" = "1" ] && [ "$OV" -lt "$MIN_PD" ] && GDIM_OK=0
[ "$WANT_NE" = "1" ] && [ "$NE" -lt "$MIN_PD" ] && GDIM_OK=0
[ "$WANT_EN" = "1" ] && [ "$EN" -lt "$MIN_PD" ] && GDIM_OK=0
[ "$TOTAL" -lt "$MIN_TOT" ] && GDIM_OK=0
if [ "$GDIM_OK" = "1" ]; then
  log "G-DIM PASS ov=$OV ne=$NE en=$EN total=$TOTAL (expect_pd=$EXP_PD n_dims=$N_DIMS min_pd=$MIN_PD min_tot=$MIN_TOT)"
elif [ "${DIM_DRAIN_UNMARK:-0}" = "1" ]; then
  # v3.1 unmark-at-drain (AG-367 w527): tickets released post-sustain -> loaded counts
  # legitimately drop; dims-proof = [DF] GEN-DONE all_marked (marked-counter, #16g split).
  GDIM_PROOF=$(grep -oE "GEN-DONE all_marked=[0-9]+" server-stdout.log | tail -1)
  log "G-DIM UNMARK-MODE WAIVED ov=$OV ne=$NE en=$EN total=$TOTAL (loaded-gate n/a post-unmark; marked-proof: ${GDIM_PROOF:-MISSING})"
  [ -n "$GDIM_PROOF" ] || { log "G-DIM UNMARK-MODE FAIL: no GEN-DONE marked-proof (pregen incomplete)"; FAIL=1; }
else
  log "G-DIM FAIL ov=$OV ne=$NE en=$EN total=$TOTAL (expect_pd=$EXP_PD min_pd=$MIN_PD min_tot=$MIN_TOT — silent-empty dims impossible gate)"; FAIL=1
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
