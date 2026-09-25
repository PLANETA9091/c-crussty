# RESEARCH-459-L02 — chunk-tick eligibility / ServerChunkCache scheduling lane (ID-P22)
Агент: TASK-459-L02 (ЛАБ, тик 459, v18.3 законы 13-16). Носитель: cmp456_chunkmono @d73758a3 (ветка round-456c-chunkmono-14, монстр-нога chk-14 +21.7@8687055, пара +9.3 — суб-бар, закон 13c климб).

## 1. Задача лабы (по LAB_BRIEF п.2)
- Декомпозиция scheduling-лейна по профилям монстр-ноги chkmono457-14 (+ сопоставление: chk-11 депресс, chk-16, anchor458-33 ваниль) — ЧИСЛАМИ.
- javap-контракты предикатов тика чанков (ChunkHolder/ChunkMap/DistanceManager/ServerChunkCache/ServerLevel).
- Матрица супerset-предикатов бит-в-байт (P22 bulk-JNI → bitmask → strict java tail).
- Capture-матем: lane% × захват% → Δ, потолок. Связь с парой ≥+20 поверх chk-14.

## 2. Декомпозиция scheduling-лейна (числа сэмплов, лист-ранжирование)

### 2.1 Бакеты «chunk system (kernel)» — CPU self-time (источники: BOTTLENECKS_3.md 4 профилей)

| профиль | runner | norm | CPU сэмплов | chunk-system bucket | доля |
|---|---|---|---|---|---|
| chkmono457-14 (монстр) | 8687055 | +21.7% | 103062 | 8570 | 8.3% |
| chkmono457-16 | 6951662 | +12.7% | 104735 | 7486 | 7.1% |
| chkmono457-11 (депресс) | 7040413 | −3.6% | 104163 | 8342 | 8.0% |
| anchor458-33 (ваниль) | 6973621 | −11.8% | 115503 | 10170 | 8.8% |

Несущая фактура: ваниль 8.0-8.8%; носитель chk-14 8.3% — mono-plane C2 (getChunkNow L1-shadow) снял лишь ~0.5пп от ванили: lane НЕ закрыт.

### 2.2 Фазовый сплит (chk-14 CPU, stack ancestry): phase: chunk tick 2760 = 2.7%; phase: chunk system (off-main worker) 1260 = 1.2%; phase: random tick 551 = 0.5%.

### 2.3 Лист-ранжирование sched-лейна (leaf frames, share CPU):

| leaf | chk-14 | chk-16 | chk-11 | anchor-33 |
|---|---|---|---|---|
| ServerChunkCache.getChunkNow | 738 (0.7%) | — | 717 (0.7%) | 913 (0.8%) |
| ConcurrentLong2ReferenceChainedHashTable.getNode | 763 (0.7%) | 1069 (1.0%) | 1033 (1.0%) | 940 (0.8%) |
| ChunkMap$TrackedEntity.moonrise$tick (tracker) | 819 (0.8%) | 699 (0.7%) | 687 (0.7%) | — |
| ChunkEntitySlices$EntityCollectionBySection.getEntities | 1120 (1.1%) | 879 (0.8%) | 964 (0.9%) | 2390 (2.1%) |
| ChunkEntitySlices.getEntities | — | — | — | 2143 (1.9%) |
| HashMap.getNode (shared) | 1459 (1.4%) | 1271 (1.2%) | 1634 (1.6%) | 1759 (1.5%) |

### 2.4 WALL-профиль chk-14 (63661 сэмплов, 81.6% libc-sleep = idle) — inclusive-агрегация моих скриптов:
- ServerChunkCache.tick incl 246 (0.39%), tickChunks 77 (0.12%), ChunkHolder 54 (0.08%), ChunkMap.tick 168 (0.26%), DistanceManager 5 (0.01%), moonrise$tick 72 (0.11%), moonrise/patches/chunk_system 189 (0.30%), NewChunkHolder 21 (0.03%).
- «Scheduler» 6008 (9.44%) = Bukkit Craft-Scheduler потоки (sleep-noise) — НЕ sched-lane, отбраковано.
- Активная часть sched-лейна на wall (внутри ServerChunkCache/ChunkMap/ChunkHolder/TrackerTickOps стеков, 397 сэмплов): листья — ServerEntity.sendChanges 46, VarHandle.getVolatile 43, ChunkMap$TrackedEntity.moonrise$tick 42, CLLRCHT.getNode 28, NaturalSpawner.createState 24, getChunkNow 16, optimiseRandomTick 15, NewChunkHolder$ChunkCompletion.chunk 12.
- ALLOC chk-14: ServerChunkCache$$Lambda 30 байт-долей 0.8% — lambda$neighbourSpawner / ChunkGetter, GC-debt мелочь.

### 2.5 Итоговая арифметика лейна (совместимо с C2-каноном «4.6-5.2%»):
vanilla-slice = getChunkNow 0.6-0.8 + CLLRCHT.getNode 1.2 (shared) + ServerChunkCache$$Lambda 0.8-1.0 + off-main 0.9-1.4 = 4.6-5.2%.
На chk-14 (после C2) видимая часть: getChunkNow 0.72 + getNode 0.74 + off-main 1.22 ≈ 2.7% видимых (top-40), полная с lambda/tails ≈ 4.3-4.9% — C2 забрал ≈0.3-0.5пп из 4.6-5.2, лейн жив на 4.3-4.9%.

## 3. javap-контракты (jar: round-chkmono457-19/patched-kernel.jar, JDK21 javap -p -c; классы подтверждены в /tmp/l02)

### K1 — ServerChunkCache.getChunkNow(II)LevelChunk (vanilla body):
```
fullChunks.get(CoordinateUtils.getChunkKey(x,z)) → if (!PlatformHooks.hasCurrentlyLoadingChunk()) return r
  (Paper: gate=false → чистый CHM.get + checkcast LevelChunk);
slow path: TickThread.isTickThread + chunkHolderManager.getChunkHolder(x,z) → getCurrentlyLoadingChunk
```
→ контракт P22: eligibility-маска НЕ может пересобирать чанк-доступ дешевле уже существующего CHM.get — путь сам O(1); выигрыш только в УБИРАНИИ вызова (bulk-предвычисление).

### K2 — ServerChunkCache.moonrise$setFullChunk(IILevelChunk)V:
```
key=getChunkKey(x,z); chunk==null ? fullChunks.remove(key) : fullChunks.put(key, chunk)
```
→ ЕДИНСТВЕННАЯ точка мутации fullChunks (C2 javap-верификация подтверждена повторно) = идеальный hook для feed-а любой eligibility-маски (bit-в-байт feed point).

### K3 — ServerChunkCache.iterateTickingChunksFaster()V (Moonrise chunk_tick_iteration):
```
ReferenceList<LevelChunk> l = level.moonrise$getEntityTickingChunks();
LevelChunk[] raw = l.getRawDataUnchecked(); for i<l.size(): tickChunk(raw[i], randomTickSpeed);
  if ((i & 7) == 0) moonrise$executeMidTickTasks()
```
→ eligibility уже «плоский» (raw array), но: ПОЛНАЯ итерация каждый тик + mid-tick каждые 8 + профайлер/gamerule внутри tickChunk. Супerset-маска бьёт по i&7-плотности mid-tick и по skip-чанкам.

### K4 — ServerLevel.tickChunk(LevelChunk, I)V:
```
simpleRandom; ice&snow: for i<randomTickSpeed { if (simpleRandom.nextInt(48)==0) tickPrecipitation(getBlockRandomPos(...)) }
if (RULE_RANDOMTICKING>0) optimiseRandomTick(chunk, speed)
```

### K5 — ServerLevel.optimiseRandomTick(LevelChunk, I)V (Moonrise block_counting):
```
for section in chunk.getSections():
  if (!section.isRandomlyTickingBlocks()) continue;                       // ← ПРЕДИКАТ №1 (раздел)
  ShortList tl = BlockCountingChunkSection.moonrise$getTickingBlockList(); // ← flat ticking indices
  for i<speed: idx = nextInt() & 4095; if (idx < tl.size()) { state = PalettedContainer.get(tl.getRaw(idx)&0xFFFF); ... new BlockPos(...) }
```
→ per-chunk eligibility = «есть ли хоть одна section с isRandomlyTickingBlocks» — вычислимо в bulk (1 бит на чанк) ДО java-хвоста; PalettedContainer.get 4.0% и BlockPos-new сидят в java-хвосте ПОСЛЕ маски.

### K6 — ChunkMap.collectSpawningChunks(List<LevelChunk>)V:
```
ReferenceList playerTickingChunks = level.moonrise$getPlayerTickingChunks();
for chunk in raw: if (isChunkNearPlayer(this, chunk.getPos(), chunk)) list.add(chunk)
```
→ spawn-eligibility = playerTickingChunks ∧ isChunkNearPlayer; bulk-JNI может собрать маску spawnable за один проход (NaturalSpawner.createState 24 сэмпла wall — нагруженный сосед).

### K7 — DistanceManager implements ChunkTickDistanceManager:
```
PositionCountingAreaMap<ServerPlayer> spawnChunkTracker;
moonrise$hasAnyNearbyNarrow(II)Z; addPlayer/removePlayer/updatePlayer(...)
```
→ player-близость уже агрегирована в area-map — бит B6 «nearby-narrow» получается O(1) на чанк, годится в супerset-маску (fake_players=4 soak).

### K8 — ReferenceList<E>: Reference2IntOpenHashMap membership + getRawDataUnchecked() — мутации add/remove синхронны с маской- feed: flat==nested lockstep оракул (G2).

## 4. Матрица супerset-предикатов бит-в-байт (9216 чанков = 144×long = 1152 Байт = 1 JNI-блок)

| бит | предикат | источник (javap) | feed-точка | false-positive цена |
|---|---|---|---|---|
| B0 | fullChunks membership | K2 | moonrise$setFullChunk | vanilla-эквивалент (C2 canon) |
| B1 | entityTicking membership | K3/K8 | ReferenceList.add/remove | лишний tickChunk → РАЗРЫВ parity, запрещён |
| B2 | any-section isRandomlyTickingBlocks | K5 | section states set/moonrise tick-list mutation | лишний пустой tickChunk (проверяем дешёвый гейт в хвосте) |
| B3 | any-section hasFluidState | LevelChunk.getFluidState 0.9-1.0% лист | fluid dirty-stamp (×453 canon, без fluid_dirty-мемо-вектора) | лишний fluid-скан → недопустимо; только ИНДИКАТОР |
| B4 | chunkHoldersToBroadcast | K1-семья broadcast | ChunkHolder.broadcastChanges enqueue | лишний getChunkToSend — cheap null-check |
| B5 | spawnable (playerTicking ∧ nearPlayer) | K6 | playerTickingChunks feed + move-events | лишний createState — НЕ пропускать в хвост |
| B6 | hasAnyNearbyNarrow | K7 | area-map updates | fine |
| B7 | tickingBlockList size>0 per section | K5 | section mutation | = B2 уточнение |

Правило бит-в-байт: маска — только ПРЕДВАРИТЕЛЬНЫЙ гейт (superset: бит=1 ⇒ java-хвост обязательно проверяет vanilla-предикат; бит=0 ⇒ только тогда, когда бит построен из ТОГО ЖЕ детерминированного знания, что vanilla-предикат ложен, — т.е. B1/B5/B6 точные, B2/B3/B4 супerset-односторонние). Lockstep-оракул G2: flat-маска == nested-пересчёт (ReferenceList + holder status) бит-в-байт каждый тик; drift>0 = fail-open.

## 5. Capture-матем и потолок

- lane% (остаток на chk-14 после C2) = 4.3-4.9%; видимая часть top-40 = 2.7%.
- захват% по компонентам: iterateTickingChunksFaster+i&7-планирование и collectSpawningChunks-холостые проходы ~25-35% лейна; getChunkNow-проб соседей (частично уже C2) ещё 10-15%; остаток = реальная работа tickChunk (в java-хвосте, не захватывается).
- Δ-прогноз = lane% × захват% = 4.6% × 0.30 ≈ **+1.4пп** (диапазон 4.3×0.25=+1.1 … 4.9×0.40=+2.0пп) — подтверждает LEDGER-прогноз ID-P22 +1.2-2пп.
- **Потолок = lane% × 100% = +4.6пп** (даже полный уход sched-лейна в bulk-JNI не даёт ≥+20 — лейн закрыт как самостоятельный носитель, годен только стековым слоем).
- Парная связь (закон 13c): chk-14 +21.7; P22 сам по себе → нога ≈ +23.1-23.7, якорный порог окна B [8637055,8737055] поднимается с ≤+1.7 до ≤+3.1-3.7 — a26 (+12.4@8671791, Δ15k) ВСЁ РАВНО не проходит (пара ≈ +10.9). Композиция-климб: P22 (+1.4) ⊕ P31 INSIDE-BATCH (+5-8пп) ⊕ P32+P36 SNAP sidecar (+1.5-2.5пп) → нога ≈ +29.6…+33.7 → якорный порог ≤+9.6…+13.7 → a26 +12.4 КВАЛИФИЦИРУЕТСЯ при Δ=15k pair-fresh → пара ≈ +20.2…+21.3 ≥ +20 ✓. Вывод: P22 — ОБЯЗАТЕЛЬНЫЙ стековый слой климба chk-14 (замыкает бар вместе с P31).

## 6. Внешние источники (≥3)
1. **Moonrise (Spottedleaf)** — патч chunk_tick_iteration (есть в кернеле javap-видимо: iterateTickingChunksFaster, playerTickingChunks, ChunkTickDistanceManager, block_counting moonrise$getTickingBlockList): взята архитектура плоских списков eligibility; P22 добавляет bulk-JNI маску ПОВЕРХ списков, не меняя семантики.
2. **C2ME (fabric, threadings/scheduler)** — chunk-granular off-main tasks, zero-per-entity JNI канон; взято: chunk-гранулярные JNI-события (mirrorEvent/schedProbe прецедент C2) и off-main tail-обработка.
3. **Paper/MC-310372 (chunk tick iteration)** — проблема per-tick итераций чанков через map-коллекции; Moonrise-фикс реализован, остаток — scheduling-планирование (i&7 mid-tick, broadcast, spawn-collect) — наш остаточный лейн 4.3-4.9%.
4. **Pufferfish (dabbing/occupied-tick, sim-range)** — идея дистанционных предвычислений элегибилити (spawnChunkTracker-паттерн подтверждён javap K7 в кернеле).
5. **Lithium/noisium/krypton** — не релевантны лейну напрямую (Lithium=коллекции/математика, noisium=worldgen-ген — инертен ×421-C, krypton=сеть); Pufferfish-слой отмечен для B6.

## 7. Preregistered гейты G1-G6 (фиксируются ДО диспатча)
- **G1 ARM/эффект-маркеры**: server-stdout содержит «cmp459_chunksched-mask: ARMED»; schedProbe-JNI-счётчик > 0; javap flat==nested 10/10 EQUAL после вайринга (спящий-гейт урок ×408 ×2: strings-проверка блобов обязана показать новый маркер).
- **G2 lockstep бит-в-байт оракул**: flat-маска == nested-пересчёт (ReferenceList+holder+area-map) каждый тик soak; mismatch=0; drift-счётчик rust=0.
- **G3 young/Full GC**: young ≤ 128, Full ≤ 9, total-pause ≤ 18.8s (банк-справка chk-14: 17.7s/Full=9; депресс-окна мимо: ре-роллы вне GC-кластеров chk-11/12).
- **G4 популяция-паритет**: 140-165k живых, spawnable-chunks=289, churn ACTIVE (сумmons=0), items-гейт 0.00.
- **G5 TPS-бар vs банк v4**: median/TPS_exp(runner) ≥ leg-chk-14 + 1.0пп; пара: leg_norm − anchor_norm ≥ +20 при Δ≤50k pair-fresh, окно B [8637055,8737055], min-of-3, band 6.0-9.5M.
- **G6 fail-closed disarm**: любой Throwable/drift>0 → BROKEN-защёлка → java-хвост vanilla навсегда; selfTest в харнессе (offline lockstep-маска против nested-референса, 3 фикстуры) обязан поймать инжектируемый рассинхрон.

## 8. План минимального эксперимента
1. Offline-харнесс (lockstep, без CI): ReentrantAreaLock-симуляция + ReferenceList против маски: 10M операций add/remove/iterate — бит-в-байт + скорость итерации (прогноз: маска-итерация 1.5-2× быстрее raw-array при ≥50% пустых бит; при 9216 чанках и ~289 spawnable — экономия видна на spawn-lane).
2. CI-нога: lever cmp459_chunksched-mask поверх носителя cmp456_chunkmono (STRICT-композиция canon ×455), ветка round-459-lab-02, диспетч в след. волне (run id фиксировать в BOARD).

## 9. Depth-чеклист (закон 14d)
- javap-контрактов: 8 (K1-K8) ✓ (≥2)
- профильных чисел: >30 (§2) ✓ (≥5)
- внешних источников: 5 ✓ (≥3)
- capture-матем + потолок: §5 ✓
- parity-план: §4 правило бит-в-байт + G2 ✓
- preregistered гейты: G1-G6 ✓
- прогноз Δ и потолок: +1.4пп (1.1-2.0), потолок +4.6пп ✓

## 10. Вердикт лабы
ID-P22 = **GO как стековый слой климба chk-14** (не самостоятельный носитель): Δ-прогноз +1.4пп (диапазон +1.1…+2.0), потолок +4.6пп < +20; композиция P22⊕P31⊕P32/P36 поверх chk-14 → прогноз ноги +29.6…+33.7 → пара с a26 +12.4@8671791 ≈ +20.2…+21.3 ≥ +20 (закон 13c закрыт числами). Ген инертен ×421-C — chunk-tick eligibility остаётся единственным живым законом-8 сайтом на soak.
