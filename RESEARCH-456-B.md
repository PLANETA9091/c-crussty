# RESEARCH-456-B — POI подсистема ЦЕЛИКОМ в Rust (закон 6) ⊕ полный носитель эры

Агент: TASK-456-B · ветка `round-456b-poi` · база: origin/master **0716075c** (f44a831e серт-мерж + docs).
Цель: ≥+20% pair-stable (мандат владельца 2026-09-23). Lever: `cmp456_poi` (пустой флаг = ваниль бит-в-байт, закон 4).

---

## 1. Карта POI-сайтов в патче/ядре (javap ground truth, patched-kernel 1.21.10, moonrise chunk-system)

Классы: `net/minecraft/world/entity/ai/village/poi/{PoiManager, PoiSection, PoiRecord, PoiTypes, PoiManager$Occupancy, PoiManager$DistanceTracker}` + moonrise `chunk_system/level/poi/PoiChunk`, `chunk_system/io/datacontroller/PoiDataController`, `ChunkLoadTask$PoiDataLoadTask`.

Горячие (main-thread, per-tick/per-setBlock) сайты:
1. **`Level.notifyAndUpdatePhysics` → `invokevirtual Level.updatePOIOnBlockStateChange(BlockPos,BS,BS)V`** — РОВНО 1 сайт в Level (javap: offset 256, единственный вызов). Virtual-dispatch на `ServerLevel.updatePOIOnBlockStateChange` (переопределение: тело = `PoiTypes.forState(old)` + `forState(newState)` + `Objects.equals` early-out + редкий exists/remove/add + debugSynchronizers). Вызывается на КАЖДЫЙ setBlock (fluid-апдейты, spawn-события) — 2 HashMap.get (`PoiTypes.TYPE_BY_STATE`, hashCode у BlockState кэширован) + 2 `Optional.ofNullable` на вызов.
2. **`ChunkMap.tick(BooleanSupplier)`** — фаза "poi": `invokevirtual PoiManager.tick` (РОВНО 1 сайт; тело = `villageDistanceTracker.propagateUpdates()`) + фаза "chunk_unload": `processUnloads`. Это часть lane "ServerChunkCache scheduling 4.6-5.2%" (BOTTLENECK-456, закон-8 ось).
3. Chunk load/unload POI: `PoiChunk.parse`/`moonrise$loadInPoiChunk`/`moonrise$checkConsistency`/`checkConsistencyWithBlocks` — OFF-MAIN (region-threads, ChunkLoadTask$PoiDataLoadTask) — не гейтит TPS, гейтит latency чанк-загрузки (закон 8, cycle-2).
4. Запросы POI (villager brains `AcquirePoi`/`NearestBedSensor`, `WanderingTraderSpawner`/`CatSpawner` 1/1200 тиков, `Raids`) — в сцене бенча ~незначимы (Villager.tick = 0.33% self, спавнеры раз в 1200 тиков).

## 2. Честный профиль-оценка POI-плоскости (из своих чисел)

Профиль серт-ноги мастера chk455-1 (BOTTLENECKS_3, 104197 self-samples, окно cpu 0-55%): top-40 leaf frames НЕ содержат НИ ОДНОГО POI-фрейма (PoiManager/PoiTypes/propagateUpdates/forState — отсутствуют); `Villager.tick` 0.33%; фазы "chunk tick" 2.5% CPU, "chunk system (off-main)" 1.1%; POI-составляющая размазана внутри "kernel: other" (21.1%). Верхняя граница main-thread POI (2×HashMap.get/setBlock + propagateUpdates + tick-оркестрация): **≈0.3-0.8% CPU**.

**GO/NO-GO ЦЕНЗ (закон 3): честный потолок ПОДСИСТЕМЫ POI СОЛО ≈ +0.5-1.5% pair < +15% → REFUTED-ценз для соло-скоупа** (прецедент agent-A ×455: despawn+spawn+activation соло REFUTED ~+1.5-2%, пивот в носитель). ПОЭТОМУ скоуп = **POI-подсистема целиком (Rust, закон 6) ⊕ ПОЛНЫЙ НОСИТЕЛЬ ЭРЫ (cmp456_poi STRICT-OR во ВСЕ гейты cmp450_chunk/cmp453_diet-семей = серт-композит мастера ins4⊕senseins⊕chunk4⊕chunk5⊕chunkparse⊕noise-GEN)** + второй слой закон-8: flush-plane на сайте ChunkMap.tick (PoiManager.tick-фаза scheduling-оркестрации). Прецедент носителя: диета cmp453_diet ноги +19.7..+28.3 norm при том же мастере — носитель = сертифицированный эффект, POI-дельта ≥0 сверху.

## 3. Имплементация (план)

**Java-мост `poi/net/minecraft/world/entity/ai/village/poi/PoiOps.java`** (blobs: poi/build, compile `--release 21`, cp = kernel-jar + fastutil, НЕ round-j2b — урок ×93):
- natives: `poiProbe()I` (magic 0x5049 "PI"), `poiBindMask([J)I` (маска POI-состояний → Rust-сторона, данные подсистемы в Rust), `poiEpoch(II[I[I)I` (tick, count, batch[5/event], out[1] = mirrorTotal) — **ОДИН bulk-JNI/тик** (flush на сайте ChunkMap.tick; пустой тик = 0 JNI).
- `updatePoiGate(Level,BlockPos,BS,BS)V` — статический stack-identical ретаргет сайта 1. Fast-path: маска POI-битов (Java long[] twin, построенная ОДИН раз из `PoiTypes.hasPoi` по `Block.BLOCK_STATE_REGISTRY`, НО данные = Rust-байнд) → оба состояния не-POI ⇒ return (ваниль в этом случае: 2×forState → 2×Optional.empty → equals true → no-op; бит-в-байт эквивалент, без Optional/map-get). Slow-path: append события в батч (levelHash, posLo, posHi, oldId, newId) + ванильный вызов `level.updatePOIOnBlockStateChange(pos, old, new)` — тело ванили НЕ тронуто (exact dispatch).
- `poiTickGate(PoiManager,BooleanSupplier)V` — ретаргет сайта 2 (flush эпохи: ОДИН bulk-JNI батча в Rust-зеркало PoiStore, затем ваниль `manager.tick(hasTimeLeft)`).
- FAIL-CLOSED: probe/binding/epoch-структурный отказ → broken=true навсегда → оба гейта = чистая ваниль; ERR_RANGE → ваниль этот тик.
- Маркеры: ARM (activate), `selfTest==true` (pre-ARM oracle: probe+mask+roundtrip), EFFECT (первый slow-path hit: "updatePOI EFFECT armed ... tick N"), "epoch ok tick=... events=... (bulk JNI 1/tick)".

**Rust `src/poi_plane.rs`**: register() = byte-hooks на `net/minecraft/world/level/Level` (после queryplane/region_threads — receive их chain-bytes, свой сайт + их патчи сохраняются) и `net/minecraft/server/level/ChunkMap` (после region_threads Hook 4; их сайт tick()V/newTrackerTick ≠ мой tick(BooleanSupplier)/PoiManager.tick — методно ортогонально); activate() = wait_for_boot → define PoiOps → RegisterNatives → selfTest==true → READY → retransform Level+ChunkMap → ARM-маркер. Natives: probe/bind/epoch (epoch = append в Vec<PoiEvent> под мьютексом, DOD-store: (levelHash, packedPos, oldId, newId)).

**STRICT-OR расширение (носитель, урок ×454-C MAIN FIX / зеркального дрейфа ×451/×452):** `cmp456_poi` добавлен в КАЖДЫЙ rust-гейт и java flag-list, где живёт cmp450_chunk или cmp453_diet (prod + test-helpers синхронно), + case `cmp456_poi` в run_world3.sh (KERNEL_POLICY=off — documented A/B override, RESEARCH-455-B finding #1 — noise_fill GEN-ось) + check_blobs_sync.sh маркеры/флаги + javap-loadability.

**Cargo:** `CARGO_TARGET_DIR=<worktree>/target cargo check --lib && cargo test` → target УДАЛИТЬ СРАЗУ (диск-закон).

## 4. Диспатч/вердикт (канон ×455/×456)

2 ноги: workflow world-bench-parallel.yml, ref round-456b-poi-1/2, inputs РОВНО канон (radius 640, seconds 300, fake_players 4, fluid_guard 1, gc_tune 3, inside_cache 1, flush_diet 1, fluid_dirty 0, fluid_bitmask 0, region_threads 4, batch_collector 1, inside_bitmask 0, skip_store_bb 0, region_steal 0, bu_defer 0, population_target 150000, population_seed 42, server_xmx 10G, server_xms 4G, cpu_band_min 6000000, cpu_band_max 9500000, lever_flag=cmp456_poi, lever_arg=1). НЕ слать travel_diet/fluid_dirty_ledger; concurrency-гвардов нет.
Вердикт: band 6.0-9.5M (BAND-DISCARD ре-ролл ≤2), threw=0, AIOOBE=0, selfTest==true, dep-гейт norm≥−2, ARM+ЭФФЕКТ маркеры, NOT-A-BENCH; пара = leg_norm − anchor_norm при Δ≤50k (кросс-тиковые валидны, канон ×455); якорный пул ×456 из ANCHORS.md, иначе исторический GOAL ×136: a3 +0.6@7061411, a6 +5.2@8395083, a7 −0.6@6720243, a9 +9.9@7438628, a13 +4.9@6619580, a14 −0.2@8580467, a15 +4.6@8914125, a20 −1.7@7079697, a24 +5.1@6601880, a25 +8.8@7551035, a27 −1.4@7044077, a28 +9.4@6190791, a32 −1.3@6791671. min-of-3 → вердикт. golden_443.py ЗАПРЕЩЁН.

## 5. Бюджет-таймплан (старт ~09:10 UTC)

- 09:10-09:30: research (этот файл) + worktree/hygiene. 09:30-09:50: STRICT-OR widening + cargo. 09:50-10:30: POI plane (java+rust+blobs+sync). 10:30-10:40: диспатч 2 ног. 10:40-11:40: полл/абсорб/маркеры. 11:40-12:00: вердикт, RESULT.json, worklog. Δ<+20% → цикл закона 3: расширение (layer-2 sched slice / sense-ядро добор) до бюджета.
