#!/usr/bin/env bash
# ======================================================================# Benchmark 3.0 — REAL-WORLD no-player load benchmark in GitHub CI
# (owner directive, 2026-09-16: MineShield-3 world, forceload everything,
# farms/mobs/entities tick, zero players, detailed bottleneck output).
#
# This is a DIAGNOSTIC boot: spark + async-profiler are measurement
# instruments attached to the run. Per project rules (INJECTS-ONLY), numbers
# produced here are NOT parity/A-B evidence — they rank hotspots so the next
# research rounds have a data-driven front. Parity claims stay on the
# sandbox rigs.
#
# Honesty notes (documented, not hidden):
#  - MineShield-3 Min zip is 6.68 GB, Full is 43.4 GB. Standard GH runners
#    fit Min only; Full requires a larger runner (owner dispatches with the
#    full URL on a self-hosted/bigger runner).
#  - Vanilla natural mob spawning is player-proximity-gated. With zero
#    players: forceloaded chunks fully tick (redstone, villagers, item
#    entities, existing mobs, block entities — farms built on those run),
#    but spawner blocks idle past 16 blocks and natural spawns idle.
#    SUMMON_SWEEPS=1 adds periodic console summon sweeps (documented
#    deviation) to exercise the spawn/tick pipeline anyway.
#  - If the closed-source natives are absent, the module degrades to
#    hotpatch-only mode (its own graceful path) — the report LABELS the mode.
#
# Disk economy: reclaim preinstalled toolchains first, delete the zip right
# after extraction, keep only report artifacts.
# ======================================================================set -uo pipefail

WORLD_URL="${WORLD_URL:-https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip}"
RUN_SECONDS="${RUN_SECONDS:-900}"
FORCELOAD_RADIUS="${FORCELOAD_RADIUS:-640}"
SUMMON_SWEEPS="${SUMMON_SWEEPS:-0}" # legacy; input dropped TASK-327
# BENCH-4 fake players (task170, S7-99): N>0 injects N real ServerPlayers via
# the BenchFakePlayers bench-only plugin (research/bench4-recon-2026-09-17 is
# the STEP-0 contract; docs/BENCH4_FAKE_PLAYERS_DESIGN.md the preregistration).
# 0 => bench-3 mode, byte-identical behavior to the no-player runs.
FAKE_PLAYERS="${FAKE_PLAYERS:-0}"
# GUARD-WAVE wave-1 (TASK-80; S7-128 ARCH-ATTACK, owner directive 2026-09-17:
# «смотри на боттлнеки и сделай чтобы они не нагружали») — same-state
# fluid-push guard на Entity.updateFluidHeightAndDoFluidPushing: negative-only
# skip при неизменном жидкостном окружении (identity re-read канонических
# FluidState-синглтонов), медленный путь = точная реимплементация ванильного
# тела. Live-verified era TASK-80: hit-rate 96.4%, JFR-proof цепочки.
# Эра ARCH (S7-128): на bench ARMED по умолчанию (архитектурный буст в паке);
# pre-guard A/B нога = CRUSSTY_FLUID_PUSH_GUARD=0 в inputs workflow.
FLUID_GUARD="${FLUID_GUARD:-1}"
# PALETTED-DEMUX (S7-131, ARCH-ATTACK lever #1 — the owner's top-1 function):
# 1 = demux patch served at PalettedContainer first load (field-inject +
# fast-path get + guarded mutators); 0 = vanilla-palette A/B leg.
PALETTED_DEMUX="${PALETTED_DEMUX:-0}"
# ALLOC-DIET (S7-133, TASK-269, ARCH-ATTACK lever #2 — the allocation lane):
# 1 = zero-alloc entity-query diet ARMED (LivingEntity.pushEntities wrapper
# -> EntityQueryOps.pushables rotating pool; CollisionUtil MutableBlockPos
# ctor -> EntityQueryOps.mutablePos ring); 0 = vanilla-alloc A/B leg.
# Targets G1 GC + oop barriers ~27% CPU (allocation-rate derivative).
ALLOC_DIET="${ALLOC_DIET:-0}"
# GC-TUNE TASK-375 (парадокс-фикс: здесь −CPU = +TPS 1:1 по построению):
# G1 STW-паузы ВНУТРИ тикового wall-clock (база s7198: 246 пауз/300s-окно,
# 19.5s суммарно = 6.5% окна, avg 79ms, max 178ms) и параллельный GC-CPU
# (37% сэмплов) на 4-ядерном раннере отбирает ядра у тик-воркеров.
# Флаги ТОЛЬКО JVM-уровня — семантика игры не трогается (parity бит-в-бит):
# MaxGCPauseMillis=40 (цель паузы vs default 200) + IHOP=35 (раньше
# concurrent-цикл → mixed-GC держит old-gen до young-давления; high-water
# 6.3G/10G=63% vs default IHOP 45%) + G1HeapRegionSize=8m (меньше карт/refine
# bookkeeping: авто при 10G = 4M) + AlwaysPreTouch (pre-commit страниц при
# буте — убирает soft-fault латентность из тиков).
GC_TUNE="${GC_TUNE:-0}"
INSIDE_CACHE="${INSIDE_CACHE:-0}"
FLUSH_DIET="${FLUSH_DIET:-0}"
# FLUID-DIRTY S7-151/TASK-290 ARCH-ATTACK lever #6 (1 = ARMED: fluid-scan
# memoization via FluidPushOps.scan + event-driven dirty-stamp ledger on
# LevelChunk.setBlockState — kills the ~10% CPU fluid-push family on static
# entities; 0 = vanilla-scan A/B leg).
FLUID_DIRTY="${FLUID_DIRTY:-0}"
FLUID_DIRTY_LEDGER="${FLUID_DIRTY_LEDGER:-0}"
FLUID_BITMASK="${FLUID_BITMASK:-0}"
REGION_THREADS="${REGION_THREADS:-0}"
# BATCH-COLLECTOR S7-160 ARCH-ATTACK lever #8 (1 = ARMED: zero-map flat
# StepBasedCollector replacement, lazy per-entity swap in RegionTickOps
# .tickBucket — requires region_threads>=2; 0 = vanilla collector A/B leg).
BATCH_COLLECTOR="${BATCH_COLLECTOR:-0}"
# FLAT-TRAVERSAL S7-163 ARCH-ATTACK lever #9 (1 = ARMED: flat bit-exact
# TraverseOps.forEachFlat replacing the guava-iterator orchestration of
# BlockGetter.forEachBlockIntersectedBetween via the entity_compose stage 6
# retarget — requires region_threads>=2; 0 = vanilla traversal A/B leg).
FLAT_TRAVERSAL="${FLAT_TRAVERSAL:-0}"
TRAVEL_DIET="${TRAVEL_DIET:-0}"
INSIDE_BITMASK="${INSIDE_BITMASK:-0}"
ZERO_ALLOC="${ZERO_ALLOC:-0}"
ZERO_CURSOR="${ZERO_CURSOR:-0}"
SKIP_STORE_BB="${SKIP_STORE_BB:-0}"
REGION_STEAL="${REGION_STEAL:-0}"
BU_DEFER="${BU_DEFER:-0}"
# TASK-391 INFRA-ROOT-CAUSE (s7206#2 35527308614: "run_world3.sh: line 182:
# PARSE_DIAG: unbound variable" под set -euo pipefail — лег умер после
# 4.5-минутной загрузки мира, артефакт = run-env-only): PARSE_DIAG был
# ЕДИНСТВЕННОЙ ручкой без default-init (echo @182 + export @437), workflow
# world-bench.yml её никогда не биндит. FLUID_FREE — того же класса страховка
# (сегодня workflow пинит '0', дефолта тут не было).
PARSE_DIAG="${PARSE_DIAG:-0}"
FLUID_FREE="${FLUID_FREE:-0}"
# BENCH-X150K population fixture (S7-129, docs/BENCH_X150K_SCENARIO.md §2):
# deterministic living-scene injection AFTER forceload, BEFORE the profiler
# window (harness waits for the POPULATION INJECT DONE marker). 0 = off.
# The mix is 70% items / 20% hostiles / 10% passives; a topup task re-injects
# vanilla-despawned items every 600 ticks so item lanes stay continuously hot.
POPULATION_TARGET="${POPULATION_TARGET:-0}"
POPULATION_SEED="${POPULATION_SEED:-42}"
SERVER_XMX="${SERVER_XMX:-6G}" # S7-130: prime-scale (150k entities) needs ~10G; 6G = historical default
NATIVES_TGZ="${NATIVES_TGZ:-https://github.com/PLANETA9091/c-crussty/releases/download/v0.1.0/crussty-v0.1.0-linux-x64.tar.gz}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/latest/download}"
WORK="${WORK:-$PWD/world3-run}"
SERVER="$WORK/server"
BOOT_TIMEOUT="${BOOT_TIMEOUT:-600}"
NATIVES_MODE="unknown"
# Resolve script dir BEFORE any cd (run #3 lesson: cd $SERVER broke
# relative "$(dirname "$0")" lookups for report_world3.py / module.json)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

log() { echo "[world3 $(date -u +%H:%M:%SZ)] $*"; }
die() { # die <msg> — still emit a diagnostic report so artifacts ship (evidence >= silence)
  log "FATAL: $*"
  python3 "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/report_world3.py" "$WORK" "$NATIVES_MODE" "0" || true
  exit 1
}
mkdir -p "$WORK" "$SERVER"

# --- 0. disk reclaim (standard GH-runner trick, ~25 GB back) ---------------
if [ -d /usr/local/lib/android ]; then sudo rm -rf /usr/local/lib/android; fi
if [ -d /usr/share/dotnet ]; then sudo rm -rf /usr/share/dotnet; fi
if [ -d /opt/ghc ]; then sudo rm -rf /opt/ghc; fi
df -h / | tail -1

# --- 1. kernel + world + natives + profilers (retries x5, exp backoff) ------
fetch() { # fetch <url> <dest>
  local url="$1" dest="$2" i wait_s
  # TASK-393 root-cause (s7207#1 35530317923): storage.shield.land отдал 503
  # на все 3 ретрая за 36с → FATAL → лег сгорел на доставке. Внешние
  # недоступности живут минутами — терпим до ~3 мин: 5 попыток с
  # экспоненциальным бэкоффом 10/20/40/80с.
  for i in 1 2 3 4 5; do
    # Run#5 lesson: NO --fail meant a GitHub-404 BODY (9 bytes "Not Found") was
    # saved as a "successful" fetch — asprof then "not found" in a non-tar file.
    curl -sSfL --retry 2 -o "$dest" "$url" && return 0
    wait_s=$(( 10 << (i - 1) ))
    log "retry $i for $url (next in ${wait_s}s)"; sleep "$wait_s"
  done
  return 1
}

log "downloading purpur kernel"
fetch "$PURPUR_URL" "$WORK/purpur.jar" || die "purpur download failed from $PURPUR_URL"
mkdir -p "$SERVER/versions" && cp "$WORK/purpur.jar" "$SERVER/versions/purpur-1.21.10.jar"

log "downloading world"
fetch "$WORLD_URL" "$WORK/world.zip" || die "world download failed from $WORLD_URL"
# S7-96d pairing hardening (run#15/#16 lesson): IDENTICAL world snapshot
# (item_frame 2714 in both) + identical inputs still produced 20.0 vs 12.5 TPS
# => shared-runner CPU variance DOMINATES cross-run baselines. Every run must
# publish its world hash + a fixed-work CPU index so research rounds can pair
# runs by (snapshot, runner-speed) before trusting any cross-run delta.
WORLD_SHA="$(sha256sum "$WORK/world.zip" 2>/dev/null | cut -d' ' -f1 || echo unknown)"
RUNNER_CPU_IDX="$(python3 -c '
import time
t=time.time(); x=1
for _ in range(6000000):
    x=(x*1103515245+12345)&0x7fffffff
print(f"{6000000/(time.time()-t):.0f}")' 2>/dev/null || echo unknown)"
{
  echo "date_utc: $(date -u +%FT%TZ)"
  echo "world_url: $WORLD_URL"
  echo "world_sha256: $WORLD_SHA"
  echo "runner_cpu_index: $RUNNER_CPU_IDX (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)"
  echo "nproc: $(nproc 2>/dev/null || echo unknown)"
  echo "summon_sweeps: $SUMMON_SWEEPS"
  echo "fake_players: $FAKE_PLAYERS (BENCH-4 fixture: N real ServerPlayers, task170)"
  echo "fluid_guard: $FLUID_GUARD (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)"
  echo "paletted_demux: $PALETTED_DEMUX (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)"
  echo "alloc_diet: $ALLOC_DIET (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)"
  echo "gc_tune: $GC_TUNE (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)"
  echo "inside_cache: $INSIDE_CACHE (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)"
  echo "flush_diet: $FLUSH_DIET (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)"
  echo "fluid_free: $FLUID_FREE (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)"
  echo "fluid_dirty: $FLUID_DIRTY (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)"
  echo "fluid_dirty_ledger: $FLUID_DIRTY_LEDGER (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)"
  echo "fluid_bitmask: $FLUID_BITMASK (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)"
  echo "region_threads: $REGION_THREADS (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)"
  echo "batch_collector: $BATCH_COLLECTOR (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)"
  echo "flat_traversal: $FLAT_TRAVERSAL (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)"
  echo "travel_diet: $TRAVEL_DIET (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)"
  echo "inside_bitmask: $INSIDE_BITMASK (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)"
  echo "zero_alloc: $ZERO_ALLOC (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)"
  echo "parse_diag: $PARSE_DIAG (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)"
  echo "zero_cursor: $ZERO_CURSOR (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos\$6+MutableBlockPos churn; TASK-330)"
  echo "skip_store_bb: $SKIP_STORE_BB (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)"
  echo "region_steal: $REGION_STEAL (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)"
  echo "bu_defer: $BU_DEFER (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)"
  echo "population_target: $POPULATION_TARGET (BENCH-X150K living-scene injection, S7-129; 0 = off)"
  echo "population_seed: $POPULATION_SEED (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)"
  echo "server_xmx: $SERVER_XMX (S7-130; 150k-scale runs use 10G)"
  echo "server_xms: $SERVER_XMS (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)"
  echo "seconds: $RUN_SECONDS (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)"
} > "$WORK/run-env.txt"
log "run-env: world_sha256=$WORLD_SHA runner_cpu_index=$RUNNER_CPU_IDX fake_players=$FAKE_PLAYERS"
log "extracting world"
# Run #1 lesson (world-bench-3 run 35106393250): the MineShield-3 zip IS the world
# directory itself (level.dat/region//DIM-1//DIM1/ at zip ROOT, no wrapper folder) —
# the old maxdepth-1 name heuristic found nothing. Robust protocol: extract to a
# staging dir, locate level.dat (any depth), pick the dir that also has region/,
# normalize it to $SERVER/world so level-name is deterministic. Handles BOTH
# wrapped (server-root-style zips) and bare-world zips.
unzip -q -o "$WORK/world.zip" -d "$WORK/worldx" && rm -f "$WORK/world.zip"
LEVELDAT="$(find "$WORK/worldx" -maxdepth 3 -type f -name level.dat | sort | head -1)"
if [ -z "$LEVELDAT" ]; then
  die "no level.dat in zip — staging top-level: $(find "$WORK/worldx" -maxdepth 2 -type d 2>/dev/null | head -40 | tr '\n' ' ')"
fi
WORLD_SRC="$(dirname "$LEVELDAT")"
[ -d "$WORLD_SRC/region" ] || die "level.dat parent has no region/: $WORLD_SRC"
rm -rf "$SERVER/world"; mkdir -p "$SERVER"
if [ "$WORLD_SRC" = "$WORK/worldx" ]; then
  # bare-world zip: level.dat at staging root — move the staging dir itself
  mv "$WORK/worldx" "$SERVER/world"
else
  mv "$WORLD_SRC" "$SERVER/world"
fi
rm -rf "$WORK/worldx"
LEVEL_NAME="world"
log "world dir: $LEVEL_NAME (from $WORLD_SRC)"

NATIVES_MODE="module-hotpatch-only"
if [ -n "$NATIVES_TGZ" ] && fetch "$NATIVES_TGZ" "$WORK/natives.tar.gz"; then
  tar xzf "$WORK/natives.tar.gz" -C "$WORK" 2>/dev/null || true
  FOUND="$(find "$WORK" -name 'libpaper_native*.so' -not -path '*/server/*' | head -2)"
  if [ -n "$FOUND" ]; then
    mkdir -p "$SERVER/native"
    find "$WORK" -name 'libpaper_native*.so' -not -path '*/server/*' -exec cp {} "$SERVER/native/" \;
    NATIVES_MODE="full-bridge"
    log "natives: $(ls "$SERVER/native")"
  fi
fi
log "NATIVES_MODE=$NATIVES_MODE"

# Run#1 lesson: resolve the script dir ABSOLUTELY once — the harness cd's into
# $SERVER before launch, and "$(dirname "$0")" stays relative after that (run#4:
# report never ran, gate failed on a missing BOTTLENECKS_3.md).
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# async-profiler (native + Rust frames via asprof attach — no JVM flag).
# Run#4 lesson: a failed fetch was logged as an EMPTY string and silently
# produced a profile-less run — the bottleneck report NEEDS collapsed stacks,
# so a pinned-version fallback is tried before giving up (loudly).
ASPROF=""
# Run#5 lesson: the "latest/download/async-profiler-linux-x64.tgz" asset name
# does NOT exist (v4.5 ships async-profiler-4.5-linux-x64.tar.gz) — pinned
# correct names first, latest/download last resort.
for APURL in \
  "https://github.com/async-profiler/async-profiler/releases/download/v4.5/async-profiler-4.5-linux-x64.tar.gz" \
  "https://github.com/async-profiler/async-profiler/releases/download/v4.1/async-profiler-4.1-linux-x64.tar.gz" \
  "https://github.com/async-profiler/async-profiler/releases/latest/download/async-profiler-linux-x64.tar.gz"; do
  if fetch "$APURL" "$WORK/ap.tgz"; then
    mkdir -p "$WORK/ap" && tar xzf "$WORK/ap.tgz" -C "$WORK/ap" --strip-components=1
    ASPROF="$(find "$WORK/ap" -type f -name asprof 2>/dev/null | head -1)"
    # run#6 lesson: the 'asprof*' prefix glob matched include/asprof.h and the
    # harness tried to EXECUTE the C header (Permission denied => zero profiles
    # with a green run). Exact name + executability check.
    if [ -n "$ASPROF" ] && [ ! -x "$ASPROF" ]; then
      chmod +x "$ASPROF" 2>/dev/null || ASPROF=""
    fi
  if [ -z "$ASPROF" ]; then
    log "asprof not found; tar top entries: $(tar tzf "$WORK/ap.tgz" 2>/dev/null | head -8 | tr '\n' ' ')"
  fi
    [ -n "$ASPROF" ] && { log "async-profiler: $ASPROF (from ${APURL##*/download/})"; break; }
  fi
  log "async-profiler source failed: $APURL — trying fallback"
 done
[ -n "$ASPROF" ] || log "WARN: async-profiler UNAVAILABLE — cpu-collapsed.txt will be absent (spark still runs)"

# --- 2. module assembly (CI prebuilds; fall back to build here) ------------
MODULE_DIR="$SERVER/modules/crussty"
mkdir -p "$MODULE_DIR"
if [ -f "$SERVER/modules/crussty/.built" ]; then
  log "module prebuilt by CI"
else
  log "building libcrussty.so from source"
  (cd "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../.." && cargo build --release) || die "cargo build failed"
  cp "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../../target/release/libcrussty.so" "$MODULE_DIR/"
fi
cp "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../../module.json" "$MODULE_DIR/" 2>/dev/null || true
# closed-source natives live INSIDE the module dir (run #2 + TASK-86 lesson:
# the loader expects libpaper_native_jni.so in modules/crussty/, not in a
# server-level native/ dir — module logged "missing libpaper_native_jni.so"
# when they were staged to $SERVER/native only)
if compgen -G "$SERVER/native/libpaper_native*.so" > /dev/null; then
  cp "$SERVER/native"/libpaper_native*.so "$MODULE_DIR/"
  log "natives staged into module dir: $(ls "$MODULE_DIR" | tr '\n' ' ')"
fi

RUNTIME_SO="${RUNTIME_SO:-$WORK/libcrussty_runtime.so}"
test -s "$RUNTIME_SO" || die "libcrussty_runtime.so not staged at $RUNTIME_SO"

# --- 3b. BENCH-4 fake-player fixture (task170) -----------------------------
# eula-less paperclip pass materializes the mojang-mapped kernel the plugin
# compiles against; the server main exits BEFORE boot on the missing eula
# (sanctioned precedent: paperclip materialization killed pre-main is NOT a
# boot). Then javac the bench plugin and stage it into plugins/.
MAX_PLAYERS=0
# S7-129: EITHER bench fixture needs the materialized mojang-mapped kernel
# (fake players always; population fixture only when target > 0).
if [ "$FAKE_PLAYERS" -gt 0 ] || [ "$POPULATION_TARGET" -gt 0 ]; then
  if [ "$FAKE_PLAYERS" -gt 0 ]; then
    MAX_PLAYERS=$((FAKE_PLAYERS + 8))
  else
    MAX_PLAYERS=8
  fi
  log "bench fixture: materializing kernel (eula-less paperclip pass, exits pre-main — NOT a boot)"
  rm -f "$SERVER/eula.txt"
  ( cd "$SERVER" && timeout 300 java -jar "versions/purpur-1.21.10.jar" --nogui \
      > "$WORK/pclip-materialize.log" 2>&1 || true )
  KERNEL_JAR="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
  [ -s "$KERNEL_JAR" ] || die "bench fixture kernel materialization failed (no $KERNEL_JAR; see $WORK/pclip-materialize.log)"
  log "bench fixture: kernel materialized ($(stat -c%s "$KERNEL_JAR") B)"
  command -v javac >/dev/null || die "javac not found (setup-java must expose JDK 21)"
  command -v jar >/dev/null || die "jar tool not found"
  FIX_CP="$KERNEL_JAR"
  while IFS= read -r j; do FIX_CP="$FIX_CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' 2>/dev/null)
  mkdir -p "$SERVER/plugins"
fi
if [ "$FAKE_PLAYERS" -gt 0 ]; then
  log "bench-4: compiling BenchFakePlayers plugin (javac only — compile is not a boot)"
  FP_SRC="$SCRIPT_DIR/fakeplayers"
  [ -f "$FP_SRC/BenchFakePlayersPlugin.java" ] || die "bench-4 plugin source missing at $FP_SRC"
  FP_CLASSES="$WORK/fpclasses"
  rm -rf "$FP_CLASSES" && mkdir -p "$FP_CLASSES"
  javac --release 21 -proc:none -cp "$FIX_CP" -d "$FP_CLASSES" \
    "$FP_SRC/BenchFakePlayersPlugin.java" || die "bench-4 plugin compile failed"
  cp "$FP_SRC/plugin.yml" "$FP_CLASSES/"
  ( cd "$FP_CLASSES" && jar cf "$SERVER/plugins/BenchFakePlayers.jar" . ) || die "bench-4 plugin packaging failed"
  log "bench-4: plugin staged ($(stat -c%s "$SERVER/plugins/BenchFakePlayers.jar") B); N=$FAKE_PLAYERS"
fi
# --- 3b-bis. BENCH-X150K population fixture (S7-129) ------------------------
if [ "$POPULATION_TARGET" -gt 0 ]; then
  log "x150k: compiling BenchPopulation plugin (javac only — compile is not a boot)"
  POP_SRC="$SCRIPT_DIR/population"
  [ -f "$POP_SRC/BenchPopulationPlugin.java" ] || die "x150k plugin source missing at $POP_SRC"
  POP_CLASSES="$WORK/popclasses"
  rm -rf "$POP_CLASSES" && mkdir -p "$POP_CLASSES"
  javac --release 21 -proc:none -cp "$FIX_CP" -d "$POP_CLASSES" \
    "$POP_SRC/BenchPopulationPlugin.java" || die "x150k plugin compile failed"
  cp "$POP_SRC/plugin.yml" "$POP_CLASSES/"
  ( cd "$POP_CLASSES" && jar cf "$SERVER/plugins/BenchPopulation.jar" . ) || die "x150k plugin packaging failed"
  log "x150k: plugin staged ($(stat -c%s "$SERVER/plugins/BenchPopulation.jar") B); target=$POPULATION_TARGET seed=$POPULATION_SEED"
fi

# --- 3. server config ------------------------------------------------------
echo "eula=true" > "$SERVER/eula.txt"
cat > "$SERVER/server.properties" <<EOF
online-mode=false
level-name=$LEVEL_NAME
motd=crussty world3 bench
view-distance=10
simulation-distance=10
spawn-monsters=true
spawn-animals=true
max-players=$MAX_PLAYERS
enable-command-block=false
white-list=false
EOF
# S7-162 leg#1 DUD lesson (run 35395826385): a slow GitHub runner pushed the
# forceload sync-load tick past the spigot watchdog timeout (default 60s) —
# the watchdog dump-storm then hard-stopped the server mid-inject, fixture
# gates failed and the leg was lost (profile of the dying server = 95%
# identity-map probing on the registration path). The watchdog is a HARNESS
# covariate, not engine behavior: keep the monitor, remove the kill
# (measured-window ticks are 0.4-1.2s — two orders below any of these caps).
cat > "$SERVER/spigot.yml" <<EOF
settings:
  timeout-time: 86400
EOF
log "bench harness: spigot watchdog timeout-time=86400 monitor-only (S7-162 leg#1 DUD hardening)"

# --- 4. launch with console fifo ------------------------------------------
# Run #2 lesson (run 35107535812): Paper resolves eula.txt/server.properties/
# world/ against CWD — launching from the repo root made eula.txt invisible
# ("Failed to load eula.txt") and would have re-created a fresh world outside
# $SERVER. cd into the server dir first.
cd "$SERVER"
# run#7 pre-registration: asprof attach needs ptrace; some runner images ship
# yama restricted-ptrace (scope=1) — lift it while sudo is passwordless here.
# Cheap insurance: a refused attach = another profile-less run.
log "ptrace_scope(pre)=$(cat /proc/sys/kernel/yama/ptrace_scope 2>/dev/null || echo unknown)"
sudo sysctl -w kernel.yama.ptrace_scope=0 >/dev/null 2>&1 \
  && log "ptrace_scope set to 0" \
  || log "WARN: could not set ptrace_scope (attach may fail)"
mkfifo "$WORK/console.in" 2>/dev/null || true
# BENCH-4 fixture env read by BenchFakePlayersPlugin (0 = no-op)
export BENCH_FAKE_PLAYERS="$FAKE_PLAYERS"
export BENCH_FORCELOAD_RADIUS="$FORCELOAD_RADIUS"
# GUARD-WAVE wave-1 gate (fluid_guard.rs reads it at register time)
export CRUSSTY_FLUID_PUSH_GUARD="$FLUID_GUARD"
# PALETTED-DEMUX gate (paletted.rs reads it at register time; S7-131)
export CRUSSTY_PALETTED_DEMUX="$PALETTED_DEMUX"
# ALLOC-DIET gate (alloc_diet.rs reads it at register time; S7-133/TASK-269)
export CRUSSTY_ALLOC_DIET="$ALLOC_DIET"
# INSIDE-CACHE gate (inside_cache.rs reads it at register time; S7-135/TASK-271)
export CRUSSTY_INSIDE_CACHE="$INSIDE_CACHE"
# FLUSH-DIET gate (flush_diet.rs reads it at register time; S7-137)
export CRUSSTY_FLUSH_DIET="$FLUSH_DIET"
# FLUID-FREE-SECTION gate (fluid_free.rs reads it at register time; S7-143;
# requires CRUSSTY_PALETTED_DEMUX=1 — the verdict epoch is the demux counter)
export CRUSSTY_FLUID_FREE="$FLUID_FREE"
# FLUID-DIRTY gate (fluid_dirty.rs reads it at register time; S7-151/TASK-290;
# the entity retarget composes through the inside_cache chain — requires
# CRUSSTY_INSIDE_CACHE=1 for scan memoization to arm)
export CRUSSTY_FLUID_DIRTY="$FLUID_DIRTY"
export CRUSSTY_FLUID_DIRTY_LEDGER="$FLUID_DIRTY_LEDGER"
export CRUSSTY_FLUID_BITMASK="$FLUID_BITMASK"
# REGION-THREADS gate (region_threads.rs reads it at register time; S7-156/
# TASK-295; integer >= 2 arms the tick-segment splice + guard retargets;
# the Ops re-parses the same env at class-init — 1 degrades to vanilla
# forEach INSIDE the bridge, parity intact)
export CRUSSTY_REGION_THREADS="$REGION_THREADS"
# BATCH-COLLECTOR gate (batch_collector.rs reads it at define time and
# RegionTickOps re-parses it at class-init; S7-160; requires region_threads
# >= 2 — the swap site lives in RegionTickOps.tickBucket)
export CRUSSTY_BATCH_COLLECTOR="$BATCH_COLLECTOR"
# FLAT-TRAVERSAL gate (traversal.rs reads it at define time; S7-163;
# requires region_threads >= 2 — the retarget composes through the
# entity_compose chain)
export CRUSSTY_FLAT_TRAVERSAL="$FLAT_TRAVERSAL"
# TRAVEL-DIET v2a gate (RECON-21, lever #14; travel_diet.rs reads it at
# define time; requires region_threads >= 2 — the redirect composes through
# the entity_compose chain stage 10)
export CRUSSTY_TRAVEL_DIET="$TRAVEL_DIET"
# INSIDE-BITMASK gate (RECON-33, lever #15; inside_bitmask.rs reads it at
# register time AND the entity_compose stage-1b polls enabled_pub; the bridge
# is fail-closed: armState()!=ARMED -> BRIDGE_READY never set -> chain
# continues WITHOUT inside_bitmask)
export CRUSSTY_INSIDE_BITMASK="$INSIDE_BITMASK"
export CRUSSTY_ZERO_ALLOC="$ZERO_ALLOC"
# SKIP-STORE-BB gate (#13-SBB, S7-166; skip_store.rs reads it at define
# time; requires region_threads >= 2 — the redirect composes through the
# entity_compose chain stage 8)
export CRUSSTY_SKIP_STORE_BB="$SKIP_STORE_BB"
# CHUNK-PARSE-DIAG gate (RECON-13d, TASK-327; parse_diag.rs reads it at
# register time): passive per-chunk parse census (chunk-parse = TOP-1 alloc
# lane 33.38% ap, RECON-13b) — decides cache-vs-ticket-churn for lever #12.
# Diagnostics-not-config: O(1) map note per parse, no behavior change.
export CRUSSTY_PARSE_DIAG="$PARSE_DIAG"
export CRUSSTY_PARSE_DIAG_FILE="$WORK/chunk-parse-diag.txt"
# ZERO-CURSOR gate (lever #11 v1, TASK-330; zero_cursor.rs reads it at activation)
export CRUSSTY_ZERO_CURSOR="$ZERO_CURSOR"
export CRUSSTY_REGION_STEAL="$REGION_STEAL"
export CRUSSTY_BU_DEFER="$BU_DEFER"
# MEGA-ROUND generic lever gate (TASK-395, world-bench-parallel.yml): empty
# flag = exact vanilla path (parity by construction); branch bridges/loaders
# read these at registration time to ARM their architecture lever.
export CRUSSTY_LEVER_FLAG="${LEVER_FLAG:-}"
export CRUSSTY_LEVER_ARG="${LEVER_ARG:-}"
# cmp420_chunk2 arming (TASK-420-C stability iteration; wave-419 base
# cmp419_chunk, chunk-pipeline law 8): the noise-fill GEN-axis
# (noise_fill.rs STRICT-OR gate) is NOT in PROVEN_WINS, so the
# kernel-policy two-key rule must be overridden for the A/B leg — the
# documented benchmarking override (kernel_policy.rs: off = A/B only).
# The chunk-parse plane needs no policy key (byte redirect, zero natives).
# Empty lever_flag = vanilla bit-in-bit, no policy change.
case "${LEVER_FLAG:-}" in
  cmp419_chunk|cmp420_chunk2|cmp420_colpush|cmp421_chunk|cmp421_brain)
    export CRUSSTY_KERNEL_POLICY="off"
    log "${LEVER_FLAG} armed: chunk-parse section-cache (deep: cap 16384, evict-half, lock-free probe) + noise-fill GEN-axis (CRUSSTY_KERNEL_POLICY=off — documented two-key A/B override; noiseFillArrayWholeBody not in PROVEN_WINS); cmp421_chunk = TASK-421-C stabilized chunk-axis round (NOISEFILL_ROOTCAUSE.md)"
    ;;
  cmp450_chunk)
    # TASK-455-B rebaze-3: cmp450_chunk arms chunk4 send-snapshot + chunk5
    # packet-encode cache + chunkparse section/biomes codec caches + the
    # ins4-carrier mob stack (STRICT-OR union of round-454b/454c gate lists).
    # GEN-axis (noise_fill.rs) restored: cmp434/435/437/444/450 ids were
    # missing from this policy case since round-422 = the noise plane never
    # fired on chunk legs (research RESEARCH-455-B.md finding #1). The
    # documented TASK-108 A/B override (kernel_policy.rs: off = benchmarking
    # only) applies to the lever-armed GEN leg; empty flag = vanilla bit-in-bit.
    export CRUSSTY_KERNEL_POLICY="off"
    log "${LEVER_FLAG} armed: CHUNK-UNION rebaze-3 = chunk4 send-snapshot ⊕ chunk5 packet-encode cache ⊕ chunkparse section/biomes codec caches ⊕ noise-fill GEN-axis (CRUSSTY_KERNEL_POLICY=off — documented two-key A/B override) ⊕ ins4-carrier mob stack STRICT-OR (TASK-455-B, закон 6+8)"
    ;;
  cmp457_paldelta)
    # TASK-457-G (закон 11, закон 8-ось): paletted-delta вектор — ЗАВЕРШЕНИЕ
    # спящего lever #1 (PALETTED-DEMUX S7-131, закон 11г «идеи-на-похищение»)
    # НА современном сертифицированном носителе: demux read-snapshot
    # PalettedContainer.get (топ-1 leaf на cert-ногах 3.7%, ~6% lane ванили)
    # STRICT-OR поверх cert-стека (ins4 ⊕ senseins ⊕ chunk-comp; гейты
    # расширены add_paldelta_gates_457.py, NCDFE-канон flag_enabled).
    # Wire-формат НЕ меняется: демукс — read-only снапшот внутри серверной
    # стадии, наружу бит-в-байт. GEN-axis two-key override как cmp450_chunk.
    export CRUSSTY_PALETTED_DEMUX="1"
    export CRUSSTY_KERNEL_POLICY="off"
    log "${LEVER_FLAG} armed: PALDELTA = PALETTED-DEMUX completion (per-container demux read fast-path, write-gen invalidation) на cert-стек носителе ⊕ noise-fill GEN-axis (CRUSSTY_KERNEL_POLICY=off) — paletted lane ~6% ванили, top-1 leaf 3.7% на cert-ногах (TASK-457-G, закон 11г)"
    ;;
esac
# TASK-430-B inside-plane subsystem (cmp430_inside) + TASK-434-C chunk-pipeline
# R5 (cmp434_chunkpl): inside_bitmask pre-gate (RECON-33 option-B flagman,
# stage-1b) + inside_snap snapshot plane (modulo decoder, ONE bulk-JNI per
# collect, stage-1c) + the A2 mobsoa carrier stack via STRICT-OR; chunkpl adds
# the biomes-parse cache (second section-decode site, 558fd1d port) + the
# block_states deep cache. No kernel-policy change (byte redirects + bulk-JNI
# only — same two-key behavior as cmp424_mobfeed legs; section-cache needs no
# key; GEN-axis stays dormant under chunkpl, kernel-policy untouched).
case "${LEVER_FLAG:-}" in
  cmp430_inside|cmp434_chunkpl)
    log "${LEVER_FLAG} armed: inside-plane subsystem = inside_bitmask all-air pre-gate (median-exact, entity_compose stage-1b) + inside_snap per-section BlockState[4096] snapshots (ONE bulk-JNI per collect, event-driven secWrite invalidation, entity_compose stage-1c) + mobsoa/colpush/queryplane/goal/items carrier stack via STRICT-OR (TASK-430-B, закон 6); cmp434_chunkpl adds chunk-parse deep cache + biomes-parse cache (TASK-434-C R5, both section lambdas)"
    ;;
  cmp432_inside2)
    log "${LEVER_FLAG} armed: inside-plane DEEPENING = inside_bitmask pre-gate + inside_snap snapshot plane with tick-stamped serve fastpath (chunk/sec memo) + inside_cache gate fused onto the snapshot plane (SNAP_ARMED reads) + slot space 2^18 (full-population memo) + mobsoa carrier stack via STRICT-OR (TASK-432-B, закон 6)"
    ;;
  cmp451_senseins)
    log "${LEVER_FLAG} armed: senseins composite = ins4-carrier (inside_snap v4 serve-plane closure + goalquery sense-arena + SoA/colpush/cvs/bq stack STRICT-OR) + sense/brain family (SenseOps getNearestEntity body-swap + BrainOps.tickEachRunning tick2 + goal-selector ops) — law-6 subsystem migration on the merge-carrier (TASK-451-D, закон 7)"
    ;;
  cmp452_mega)
    log "${LEVER_FLAG} armed: MEGA-COMPOSITION cmp452_mega = cmp451_senseins (ins4-carrier inside_snap v4 + SoA/colpush/cvs/bq stack + SenseOps body-swap + BrainOps tick2 + goalquery sense-arena) ⊕ cmp450_chunk (chunk4 send-snapshot ⊕ chunk5 packet encode-cache ⊕ chunkparse section/biomes codec caches) — STRICT-OR мульти-семейный композит: ОБЕ семьи живут на каждом гейте, пустой флаг = ваниль бит-в-байт (TASK-452-C, закон 7, лесенка эры вверх)"
    ;;
  cmp453_diet)
    log "${LEVER_FLAG} armed: DIET-COMPOSITE cmp453_diet = ins4-carrier (inside_snap v4 + SoA/colpush/cvs/bq/items stack STRICT-OR) ⊕ sense-ядро (SenseOps getNearestEntity body-swap + goalquery sense-arena/entity_query senseMode) ⊕ chunk4 send-snapshot — БЕЗ хвостов: chunk5 encode-cache ВЫРЕЗАН (players_packets lane 0.00-0.01% FLAT ×452 = нулевая маржа, MISS-диета), BrainOps tick2 ВЫРЕЗАН (Brain-mobs=0 в фикстуре, срез мёртв), section-codec chunkparse ОСТАВЛЕН (не перекрыт chunk4: parse=disk-load vs send=player path, часть серта) — закон 7 субаддитивность: диета против склейки (TASK-453-C)"
    ;;
  cmp456_chunkmono)
    # TASK-456-C chunkmono carrier: STRICT-композиция-носитель = master cert
    # stack (ins4 ⊕ senseins ⊕ diet ⊕ chunk4 ⊕ chunk5 ⊕ chunkparse ⊕ noise-GEN)
    # ⊕ NEW chunk6-sched scheduling mono-plane (getChunkNow fast-path over the
    # direct-mapped L1 shadow fed by moonrise$setFullChunk + rust L2 key-mirror,
    # chunk-granular JNI events — law 6 legal; scheduling slice 4.6-5.2% wall).
    # noise-GEN carries with the master stack => CRUSSTY_KERNEL_POLICY=off
    # (documented TASK-108 A/B override, cmp450_chunk canon; fillFromNoise 0.0%
    # in soak — boot-parity only). Empty flag = vanilla bit-in-byte.
    export CRUSSTY_KERNEL_POLICY="off"
    log "${LEVER_FLAG} armed: CHUNKMONO-CARRIER cmp456_chunkmono = master cert stack (ins4 ⊕ senseins ⊕ diet ⊕ chunk4 send-snapshot ⊕ chunk5 encode-cache ⊕ chunkparse section/biomes codec caches ⊕ noise-GEN GEN-axis; STRICT-OR) ⊕ chunk6-sched scheduling mono-plane (getChunkNow -> ChunkSchedOps.getNow fast-path + moonrise\$setFullChunk shadow feed + rust L2 key-mirror; BOTH-or-none; scheduling slice 4.6-5.2% in scope) — закон 7 субаддитивность на серт-носителе f44a831e (TASK-456-C, RESEARCH-456-C GO)"
    ;; # P0-fix x466-C02: MERGE #9 line-union ate the case-arm terminator — bash -n FAIL line 532, canary-405 36226176808 failure @07:21:11Z
  cmp456_poi)
    # TASK-456-B: POI подсистема целиком (закон 6) ⊕ ПОЛНЫЙ НОСИТЕЛЬ ЭРЫ
    # (STRICT-OR union of the certified master composite). GEN-axis
    # (noise_fill.rs) needs the documented kernel-policy A/B override — same
    # two-key rule as cmp450_chunk legs (RESEARCH-455-B finding #1). Empty
    # flag = vanilla bit-in-bit, no policy change.
    export CRUSSTY_KERNEL_POLICY="off"
    log "${LEVER_FLAG} armed: POI-SUBSYSTEM = POI-плоскость целиком в Rust (Level.notifyAndUpdatePhysics updatePOIOnBlockStateChange site -> PoiOps.updatePoiGate POI-mask fast-path; ChunkMap.tick PoiManager.tick site -> PoiOps.poiTickGate epoch flush; rust PoiStore mirror via ONE bulk poiEpoch JNI/tick) ⊕ ПОЛНЫЙ НОСИТЕЛЬ ЭРЫ cmp456_poi STRICT-OR (ins4 ⊕ senseins ⊕ chunk4-send ⊕ chunk5-encode ⊕ chunkparse ⊕ noise-GEN, KERNEL_POLICY=off documented A/B override) — TASK-456-B, закон 6+7+8"
    ;;
esac
# RECON_DIAG (TASK-317, instrument-гейт рычага #13 SKIP-STORE-DIET): чистая
# наблюдаемость — 0 поведения. GC-политика/heap не трогаются (логирование ≠
# config-win, вердикт NEXT TASK-316): remset/refine debug-логи (агрегатная
# интенсивность old->young карт) + JFR profile recording (jdk.OldObjectSample
# активен в profile.jfc JDK21: memory-leaks default=stack-traces — объекты,
# достигшие старого гена, с allocation stack = producer-атрибуция;
# jdk.ObjectAllocationSample 300/s — контрольный alloc-профиль). Числа этого
# лега НЕ используются для CPU/TPS-гейтов (JFR overhead смещает профиль) —
# только атрибуция store-firehose (доля entity-полей в old->young записях).
# bash-массив (НЕ строка): word-splitting на $@ сохраняет цельность аргументов;
# пути $WORK без пробелов, но массив паритетен java-строке по построению.
EXTRA_JVM_DIAG=()
# GC-TUNE TASK-375/376/380 flag array (armed only at gc_tune=1/2; vanilla JVM args
# otherwise — historical legs bit-exact via GC_TUNE default 0)
EXTRA_JVM_GC=()
# GC-COLLECTOR switch (TASK-380 autonomous A/B, owner directive 2026-09-20 20:08
# "непривычный но быстрее = ставь"): default G1 (historical parity); 3 = ParallelGC
# (throughput-play: G1 concurrent machinery = 37% CPU overhead on 4 cores),
# 4 = ZGC generational (sub-ms pauses vs concurrent CPU price).
GC_COLLECTOR=("-XX:+UseG1GC")
if [ "${GC_TUNE:-0}" = "1" ]; then
  EXTRA_JVM_GC=(
    "-XX:MaxGCPauseMillis=40"
    "-XX:InitiatingHeapOccupancyPercent=35"
    "-XX:G1HeapRegionSize=8m"
    "-XX:+AlwaysPreTouch"
  )
  log "gc_tune=1: MaxGCPauseMillis=40 + IHOP=35 + RegionSize=8m + AlwaysPreTouch (TASK-375)"
elif [ "${GC_TUNE:-0}" = "2" ]; then
  EXTRA_JVM_GC=(
    "-XX:InitiatingHeapOccupancyPercent=35"
    "-XX:G1HeapRegionSize=8m"
    "-XX:+AlwaysPreTouch"
  )
  log "gc_tune=2: IHOP=35 + RegionSize=8m + AlwaysPreTouch, NO pause-target (TASK-376 v2: s7199 показал pause-target=40 токсичен — eden 180M, 621 эвакуаций x ~60ms fixed = 44.4s; v2 = чистый A/B без токсичной кнопки)"
elif [ "${GC_TUNE:-0}" = "3" ]; then
  GC_COLLECTOR=("-XX:+UseParallelGC")
  log "gc_tune=3: COLLECTOR SWAP G1->ParallelGC (TASK-380 autonomous A/B: throughput-play на 4 ядрах — отказ от G1 concurrent/refinement CPU 37% в обмен на редкие длинные STW)"
elif [ "${GC_TUNE:-0}" = "4" ]; then
  GC_COLLECTOR=("-XX:+UseZGC" "-XX:+ZGenerational")
  log "gc_tune=4: COLLECTOR SWAP G1->ZGC generational (TASK-380 autonomous A/B: sub-ms паузы против concurrent CPU-цены на 4 ядрах)"
elif [ "${GC_TUNE:-0}" = "5" ]; then
  GC_COLLECTOR=("-XX:+UseParallelGC" "-XX:+UseTransparentHugePages" "-XX:+AlwaysPreTouch")
  log "gc_tune=5: ParallelGC + THP + AlwaysPreTouch (TASK-384 autonomous A/B: RECON-41 — профиль memory-bound [PalettedContainer.get 4.3% + SimpleBitStorage 1.7% воркеров, HashMap.getNode], THP режет TLB-miss и в сцене и в GC-copy 4.35GB/s)"
fi
if [ "${RECON_DIAG:-0}" = "1" ]; then
  EXTRA_JVM_DIAG=(
    "-Xlog:gc+remset=debug:file=$WORK/remset.log:time,uptime,level,tags"
    "-Xlog:gc+refine=debug:file=$WORK/refine.log:time,uptime,level,tags"
    "-XX:StartFlightRecording=filename=$WORK/recon.jfr,settings=profile,dumponexit=true"
  )
  log "recon_diag=1: remset/refine debug-логи + JFR profile (diagnostic leg — NOT a gate leg)"
fi
# BENCH-X150K population fixture env (0 = no-op; S7-129)
export BENCH_POPULATION_TARGET="$POPULATION_TARGET"
export BENCH_POPULATION_SEED="$POPULATION_SEED"
# S7-157 incident fix: `tail -f console.in | java` leaves an ORPHANED tail -f
# holding the CI step's stderr pipe open when the JVM dies — the runner then
# waits on the open pipe until the 75-min job timeout (run 35353820223 burned
# 68 min post-crash). FIFO pattern: tail's lifetime is bounded and killed at
# shutdown.
rm -f "$WORK/console.pipe"; mkfifo "$WORK/console.pipe"
( tail -f "$WORK/console.in" > "$WORK/console.pipe" 2>/dev/null ) &
TAIL_PID=$!
java \
  "-agentpath:$RUNTIME_SO=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar" \
  -Xms"$SERVER_XMS" -Xmx"$SERVER_XMX" "${GC_COLLECTOR[@]}" -Dfile.encoding=UTF-8 \
  "${EXTRA_JVM_GC[@]}" \
  -Xlog:gc*:file="$WORK/gc.log":time,uptime,level,tags \
  "${EXTRA_JVM_DIAG[@]}" \
  -jar "$SERVER/versions/purpur-1.21.10.jar" --nogui \
  < "$WORK/console.pipe" \
  > "$WORK/server-stdout.log" 2>&1 &
SERVER_PID=$!
server_died() { ! kill -0 "$SERVER_PID" 2>/dev/null; }
log "server pid $SERVER_PID (console tail pid $TAIL_PID) — waiting for Done (<=${BOOT_TIMEOUT}s)"

SEEN_DONE=0
for i in $(seq 1 "$BOOT_TIMEOUT"); do
  if grep -qF "Done (" "$WORK/server-stdout.log" 2>/dev/null; then SEEN_DONE=1; break; fi
  if grep -qiE "Failed to start|Exception in thread .main." "$WORK/server-stdout.log" 2>/dev/null; then break; fi
  if server_died; then log "FATAL: server process died during boot — aborting waits (crash artifacts preserved)"; break; fi
  sleep 1
done
log "SEEN_DONE=$SEEN_DONE"

# S7-158a (leg #2 35363758352 incident, 59-min post-artifact burn): `echo >
# console.in` OPENS a FIFO — if the reader (tail) is gone (it SIGPIPEs on the
# first write into console.pipe once java died) the open() blocks FOREVER and
# the job sits until the 75-min timeout. Every console write is now bounded
# by timeout(1): a dead console channel costs 5s per call, never the job.
# Same bound applies to report_world3.py (last unbounded op before exit).
cmd() { timeout 5 sh -c 'printf "%s\n" "$1" > "$2"' _ "$*" "$WORK/console.in" 2>/dev/null || true; }

if [ "$SEEN_DONE" = "1" ]; then
  # --- 5. forceload sweep (overworld tiles of 16x16 chunks <= 256/command) --
  STEP=256
  R="$FORCELOAD_RADIUS"
  TILES=$(( (R + STEP - 1) / STEP ))
  log "forceload sweep: radius ${R} blocks, $((2*TILES))x$((2*TILES)) tiles"
  for tx in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
    for tz in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
      cmd "forceload add $tx $tz $((tx + STEP - 1)) $((tz + STEP - 1))"
      # S67 (round-467): sweep-protocol knob. Canon 0.4 = A-r640 afb3a0b3
      # bit-exact (wall 26.0s = 36*0.4 floor + 11.6s server gen => 354 c/s).
      # Floor N*sleep dominates wall: sleep 0.05 -> 687 c/s additive /
      # 748 c/s overlap-model vs BAR 708 (S67 model, /tmp/s67/model.py).
      # STEP stays 256: (128,64) shrink coverage 9216->6400 and multiply
      # the floor (400 cmds @ STEP=64) — strictly dominated at any sleep.
      sleep "${COMMAND_SLEEP:-0.4}"
    done
  done
  cmd "tps"
  cmd "paper debug chunks"
  sleep 10

  # --- 5b. BENCH-X150K population injection (S7-129) -------------------------
  # The living-scene fixture injects BEFORE any profiler starts: the harness
  # triggers the plugin via console and waits for the DONE marker, so the
  # measured window always sees the full injected population (spec §2:
  # «инъекция до старта окна замера»). Item topups DURING the window are
  # part of the scene model (continuous item lanes), not window pollution.
  if [ "$POPULATION_TARGET" -gt 0 ]; then
    POP_TIMEOUT="${POP_INJECT_TIMEOUT:-900}"
    log "x150k: benchpop inject target=$POPULATION_TARGET seed=$POPULATION_SEED (waiting <= ${POP_TIMEOUT}s for DONE marker)"
    cmd "benchpop inject $POPULATION_TARGET $POPULATION_SEED"
    POP_WAITED=0
    while ! grep -q "POPULATION INJECT DONE" "$WORK/server-stdout.log" 2>/dev/null; do
      if [ "$POP_WAITED" -ge "$POP_TIMEOUT" ]; then
        log "WARN: x150k injection DONE marker NOT seen in ${POP_TIMEOUT}s — continuing (fixture gate will fail the run)"
        break
      fi
      if server_died; then log "FATAL: server process died during population injection — aborting waits (crash artifacts preserved)"; SEEN_DONE=0; break; fi
      sleep 10
      POP_WAITED=$((POP_WAITED + 10))
      if [ $((POP_WAITED % 60)) -eq 0 ]; then
        log "x150k: still injecting... waited=${POP_WAITED}s"
      fi
    done
    grep "POPULATION INJECT DONE" "$WORK/server-stdout.log" | tail -1 || true
    grep "POPULATION FIXTURE-VALIDITY" "$WORK/server-stdout.log" | tail -1 || true
    sleep 5  # settle injection tail before profilers attach
  fi

  # --- 6. profilers (v3: stop-based sequential windows) --------------------
  # run#9 lesson: asprof v4.x allows ONE active session per target — so the
  # soak is split into three windows. cpu (0..55%) ranks hotspots; wall
  # (55..80%) exposes JNI/lock/IO waits cpu hides; alloc (80..100%) names the
  # allocation offenders feeding G1 (alloc weights = BYTES — the churn
  # ranking the S7-134 old-gen census needs).
  # S7-134 ROOT-CAUSE FIX (v2 bug): asprof v4.x `dump` does NOT stop the
  # session — only `stop` does. v2 chained dump->start, so the FIRST cpu
  # session stayed alive for the whole soak (ap.log: 3x "[ERROR] Profiler
  # already started"), wall/alloc never started, and cpu-/wall-/alloc-
  # collapsed.txt were cumulative CPU re-dumps. Consequence: the alloc
  # profile was NEVER collected in any past run (S7-131..133 lane analyses
  # remain valid — cpu+gc lanes only; wall/alloc BOTTLENECKS_3 sections were
  # CPU-contaminated). v3: `stop` ends each window (stop == stop+dump), and
  # every start is guarded by an orphan rescue so a stuck session can never
  # silently swallow a window again.
  END=$(( SECONDS + RUN_SECONDS ))
  CPU_END=$(( SECONDS + RUN_SECONDS * 55 / 100 ))
  WALL_END=$(( SECONDS + RUN_SECONDS * 80 / 100 ))
  PROF_PHASE=cpu

  asprof_guard_start() { # asprof_guard_start [event args...] — orphan-safe start
    local out rc
    out="$("$ASPROF" start "$@" "$SERVER_PID" 2>&1)"; rc=$?
    if [ $rc -ne 0 ] || echo "$out" | grep -qi "already started"; then
      log "asprof: orphan session before start($*) — rescuing into orphan-collapsed.txt"
      "$ASPROF" stop -o collapsed -f "$WORK/orphan-collapsed.txt" "$SERVER_PID" >>"$WORK/ap.log" 2>&1 || true
      out="$("$ASPROF" start "$@" "$SERVER_PID" 2>&1)"
    fi
    if echo "$out" | grep -qi "error"; then log "asprof start($*) FAILED: $out"; else echo "$out" >>"$WORK/ap.log"; fi
  }
  asprof_stop_dump() { # asprof_stop_dump <file> <format> <label> — stop == stop+dump in asprof 4.x
    local file="$1" fmt="$2" label="$3"
    "$ASPROF" stop -o "$fmt" -f "$file" "$SERVER_PID" >>"$WORK/ap.log" 2>&1 || log "asprof $label stop failed"
    [ -s "$file" ] && log "$label: $(wc -l < "$file") stacks" || log "WARN: $file EMPTY"
  }

  if [ -n "$ASPROF" ]; then
    asprof_guard_start -e cpu,interval=5ms
  fi
  cmd "spark profiler start --timeout $RUN_SECONDS"

  while [ $SECONDS -lt $END ]; do
    if server_died; then log "FATAL: server process died mid-soak — ending soak early (crash artifacts preserved)"; break; fi
    sleep 60
    cmd "tps"
    # run#12 root-cause (S7-96b): `paper mspt` does NOT exist on Purpur 1.21.10
    # (every poll answered Usage-error since run#10) — replaced with mobcaps,
    # which IS in the command set and gives the spawn-lane observability the
    # owner's as-if-players condition needs. MSPT comes from tickmonitor [⚡].
    cmd "paper mobcaps world"
    # run#12 root-cause (S7-96b): `paper entity list` needs filter+worldName,
    # bare call returned Usage-error every poll since run#10 (0 entity data)
    cmd "paper entity list * world"
    cmd "spark tickmonitor --threshold 50"
    if [ "$SUMMON_SWEEPS" = "1" ]; then
      for k in 1 2 3 4 5; do
        X=$(( (RANDOM % (2 * R)) - R )); Z=$(( (RANDOM % (2 * R)) - R ))
        cmd "execute in minecraft:overworld run summon minecraft:zombie $X 100 $Z"
      done
    fi
    if [ "$PROF_PHASE" = "cpu" ] && [ $SECONDS -ge $CPU_END ]; then
      PROF_PHASE=wall
      if [ -n "$ASPROF" ]; then
        asprof_stop_dump "$WORK/cpu-collapsed.txt" collapsed "cpu-collapsed"
        # RECON-36 P2-pre-gate (TASK-363): the wall session runs THREADED (-t)
        # so wall-collapsed.txt carries per-thread stacks: duty of each
        # crussty-region-worker-* (tickBucket-active vs CyclicBarrier park)
        # + Server-thread DONE-wait -> worker imbalance I (RECON-36 fork:
        # I<=1.15 offload-ready / I>=1.3 rebalance). Thread-merged wall had no
        # thread identity (dead weight). Fallback: plain merged wall if this
        # asprof build rejects -t.
        tout="$("$ASPROF" start -t -e wall "$SERVER_PID" 2>&1)"; trc=$?
        if [ $trc -eq 0 ] && ! echo "$tout" | grep -qi "error"; then
          echo "$tout" >>"$WORK/ap.log"
          log "wall session: THREADED (-t -e wall) — per-thread duty/imbalance enabled (RECON-36 P2-pre-gate)"
        else
          echo "threaded start rc=$trc: $tout" >>"$WORK/ap.log"
          log "asprof threaded start FAILED — fallback to merged wall session"
          asprof_guard_start -e wall
        fi
      fi
    fi
    if [ "$PROF_PHASE" = "wall" ] && [ $SECONDS -ge $WALL_END ]; then
      PROF_PHASE=alloc
      if [ -n "$ASPROF" ]; then
        asprof_stop_dump "$WORK/wall-collapsed.txt" collapsed "wall-collapsed"
        asprof_guard_start -e alloc
      fi
    fi
  done

  # --- 7. final captures + shutdown ---------------------------------------
  cmd "paper debug chunks"
  cmd "spark gc"
  if [ -n "$ASPROF" ]; then
    # v3: the session active at soak end is ALLOC (windows above) — stopping
    # it into alloc-collapsed.txt (weights = BYTES). Then a SHORT fresh cpu
    # session produces the flamegraph for humans (20s sampling tail).
    asprof_stop_dump "$WORK/alloc-collapsed.txt" collapsed "alloc-collapsed"
    asprof_guard_start -e cpu,interval=5ms
    sleep 20   # give the flamegraph window real samples (v2 got an orphan re-dump here)
    "$ASPROF" stop -o flamegraph -f "$WORK/cpu-flamegraph.html" "$SERVER_PID" >>"$WORK/ap.log" 2>&1 || true
  fi
  cmd "spark profiler --stop"
  sleep 15
  # spark stores raw .sparkprofile blobs under plugins/spark; /paper debug
  # chunks dumps tables under debug/ — collect BOTH trees (both small)
  mkdir -p "$WORK/spark-report" "$WORK/debug-dumps"
  [ -d "$SERVER/plugins/spark" ] && cp -r "$SERVER/plugins/spark/." "$WORK/spark-report/" 2>/dev/null || true
  [ -d "$SERVER/debug" ] && cp -r "$SERVER/debug/." "$WORK/debug-dumps/" 2>/dev/null || true
  log "spark/debug artifacts: $(find "$WORK/spark-report" "$WORK/debug-dumps" -type f 2>/dev/null | wc -l)"
fi
cmd "stop"
sleep 30
kill "$SERVER_PID" 2>/dev/null || true
sleep 10
kill -9 "$SERVER_PID" 2>/dev/null || true
kill "$TAIL_PID" 2>/dev/null || true
sleep 2
kill -9 "$TAIL_PID" 2>/dev/null || true
rm -f "$WORK/console.pipe"
# --- 8. bottleneck report ---------------------------------------------------
if [ "$SEEN_DONE" != "1" ]; then
  log "WARN: SEEN_DONE=0 — last 40 server lines for in-log diagnosis (no artifact archaeology):"
  tail -40 "$WORK/server-stdout.log" 2>/dev/null | sed 's/^/[srv] /'
fi
timeout 180 python3 "$SCRIPT_DIR/report_world3.py" "$WORK" "$NATIVES_MODE" "$SEEN_DONE" || true
log "harness complete; artifacts in $WORK"
exit 0
