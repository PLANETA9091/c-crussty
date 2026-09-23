# CHUNK_ROOTCAUSE — TASK-424-C (тик-423, ROUND-423, Job 406609, эра v17)

Мандат: ≥+20% pair-stable ОБЯЗАТЕЛЬНО; ось chunk-loading/worldgen/noise
(закон 8, директива владельца 2026-09-23 01:4x «оптимизация шума в генерации
мира — игроки видят разницу, чанки медленно грузятся»).
База-носитель: master 4789ca7 (vanilla + банк-плоскости), якорные артефакты
research/gc-recon-2026-09-19/round-anchor422b (world3-bench, bank inputs,
runner_cpu_index 6724897 — в бан-диапазоне 6.0-9.5M).

## 1. Игрок-видимая хронология ванильного якоря (server-stdout.log, anchor422b)

| окно | факт | число |
|---|---|---|
| BOOT | bootstrap → Done | **14.784s** (02:18:50 → 02:19:04; «Done preparing level» 1.507s) |
| JOIN | 4 fake players joined | до/в момент Done (02:19:03-04) |
| FORCELOAD | 36 команд × 256-тайлы, radius 640 | **9216 чанков** parse-бурст ~2-3s ПОСЛЕ Done |
| POPULATION | inject 150k (items 105k/hostiles 30k/passives 15k) | 9954 loadedChunks к старту, ~66s |
| SOAK | TPS-поллы (первое значение окна) | **[20.6 pre-pop; 1.7, 1.8, 2.0, 2.4, 2.5]** |
| MSPT | spark tick-monitor | avg 439.65ms / max 615.99 |
| SAVE | shutdown save | 10000 block chunks, 24.16s |

## 2. Рут-кауз осей (все числа из anchor422b-артефактов, soak-CPU 116234 сэмплов)

**RC-A (повтор тика-421, подтверждено на 4789ca7): GEN-ось noise ИНЕРТНА.**
worldgen/noise bucket = **34 сэмпла = 0.03%** soak-CPU. Мир MineShield-3
ПРЕГЕНЕРИРОВАН (world_sha256 afb3a0b3…): 9216 forceloaded чанков = PARSE
сохранённых секций, не worldgen. NoiseChunk/PerlinNoise/ImprovedNoise-семья
не работает в soak ни на тике 421, ни сейчас. → **R5a (noise-ядро в Rust)
на ЭТОЙ фикстуре = плацебо-класс**: реализация не дойдёт ни до одного полла.

**RC-B: chunk-SEND плоскость МЕРТВА в soak.** Лейн players_packets
(ServerGamePacketListener|Clientbound|PlayerChunkSender) = **12/115655 = 0.01%**
(абсорб-гвард lane_map). Fake-players fixture (task170) не тянет клиентские
чанк-пакеты как реальный клиент. ChunkMap.tick под ServerChunkCache.tick =
3184 сэмпла, из них **3180 (99.9%) = TrackerTickOps.newTrackerTick** — это
уже регион-тредовая трекер-плоскость (bank region_threads=4), не chunk-send.
→ **R5b (chunk-serialization/send slice) = мёртвый лейн** в soak-окне;
единственная его жизнь — boot-окно, которое TPS-поллы не накрывают.

**RC-C: блок-рид-семья = REFUTED-зона.** PalettedContainer.get 4483 (3.9%) +
SimpleBitStorage.get 1462 + readPalette 1045 + LevelChunk.getFluidState 1032 +
getBlockStateFinal 891 ≈ 7.7% — RECON-35: 100% инфраструктура потребителей
(fluid-sim 50% = ванильная семантика, movement-collision 22% = лейн #14
REFUTED); мемоизация чтений = потолок <7.6% сцены < двойного бара, TPS-
конверсия диет-семейства ×6 = 0. → НЕ банкингуется (9-е док-закрытие).

**RC-D: touchingUnloadedChunk 2.24% (2600) — 100% из fluid-push, 79% под ним
= Level.hasChunksAt → moonrise$areChunksLoaded → N× ServerChunkCache.hasChunk
= ConcurrentLong2ReferenceChainedHashTable.getNode (1442).** Оба вызывающих
семейства (fluid-мемо «fluid_dirty», dirty-stamp ledger) — в списке ЗАПРЕТОВ
эфры; безмемная ускорялка цикла N=4-16 хэш-лукапов бит-в-бит невозможна
(каждый лукап уникален по ключу). → микро-зона, чартер RECON-35/31.

**RC-E: hot chunk-lane, который ЖИВ и НЕ закрыт — entity-query/broadphase
через chunk-slices: EntityLookup.getEntities 8758 = 7.54% + ChunkEntitySlices.
getEntities 2044 + EntityCollectionBySection.getEntities 1979 + getHard-
CollidingEntities 697 ≈ 11.6% soak-CPU.** Вызывающие: 52% Level.getEntities
(EntityTypeTest-оверлоад), 48% Level.getEntitiesOfClass. Это лейн ЗАМЕРЖЕНОГО
вектора эры QUERYPLANE (5db1090 = первый мерж эпохи; cv⊕queryplane медиана
pair **+36.2%** ночью, selfTest ×6) — на носителе 4789ca7 он В КОДЕ, но
ДОРМАНТНТ (гейт строго eq на чужие lever-id). → закон-7 композиция.

**RC-F: parse-бурст = boot-window; конверсия в поллы = GC-debt relief.**
Parse 9216 чанков ≈ 2-3s после Done; RECON-13b: decode-путь = 33.38% alloc
байт бурст-окна (codec 19.06% + paletted 13.98%). На ParallelGC (gc_tune=3)
якоря: young n=106 × avg 172ms, full n=9 (CodeCache/Metadata-триггеры) —
механизм RC6 тика-421: срез parse-аллокаций = меньше young-давления в
population-инжекте (66s) и ранних поллах = ранняя рампа выше. Вклад chunk-
союза ≈ **+6-8пп стабильных** (кросс-чек cp420 vs mg-ноги ×3). Блок-lambda
кэш (cmp420_chunk2, мерж 85f74b8) в master ЕСТЬ, но дорамантен на чужих
флагах; biomes-lambda (lambda$parse$7, ТОТ ЖЕ канонический дескриптор,
javap-verified: codec.parse(NbtOps,tag).promotePartial(logErrors).getOrThrow
(new ChunkReadException)) НЕ кэшируется вовсе — остаток codec-машинерии.

**RC-G: BOOT-DONE 14.784s не имеет кэш-рычага.** Parse в boot = ПЕРВИЧНОЕ
декодирование (кэш по определению мимо); JIT/classload/CodeCache-полные GC
не управляемы из bridge-плоскости. Честная метрика was→became, рычага нет.

## 3. Вердикт для вектора C (что реализуем)

- **R5a noise-ядро** — НЕ реализуем в этом тике: RC-A, 0.03% soak = плацебо
  (закон против анти-плацебо-гейта: ретаргет с 0 горячих сайтов = sites>0
  формально, эффект 0).
- **R5b chunk-send slice** — НЕ реализуем в soak: RC-B, 0.01% лейн.
- **R5c biomes-parse cache** — РЕАЛИЗУЕМ (RC-F): вторая section-lambda
  (lambda$parse$7) получает зеркальный template-кэш в ChunkParseOps
  (biomes-секции низко-энтропийны: 1-2 биома на секцию → hit-rate выше
  блок-lambda; MISS = in-bridge reflection-реплика ванильного тела,
  бит-в-бит; самотест расширен). Продолжение 600e3cc, которое
  root-кауз теперь подтверждает (parse остаётся единственной живой
  осью-срезом в бурст-окне).
- **Композиция закона 7 (носитель cmp423_wgen)** — РЕАЛИЗУЕМ (RC-E, RC-F):
  платформа chunk-pipeline ⊕ queryplane (замерженный вектор эры,
  broadphase-лейн 11.6%, +36.2% на своём носителе) ⊕ chunk-parse cache
  (замерженный, +6-8пп RC6) — оба догружаются STRICT-OR в гейты чужих
  флагов (прецедент cmp417_bq, cmp420_colpush/round-421-c-chunk S1).
  Ожидание по лейнам: broadphase −5..-9пп, parse-бурст relief → ранняя
  рампа; честный потолок >20% только в композиции — одиночные рычаги
  оси по числам RC-A..RC-F бар не берут.

## 4. Метрики игрока-видимости (фиксируются в RESULT.json was→became)

1. boot-Done time: was 14.784s (anchor422b) → стал (×3 ноги, + anchors).
2. Ramp TPS-поллы (первые 3 soak-полла): was [1.7, 1.8, 2.0] → стал.
3. chunk-send счётчики: players_packets лейн = 12/115655 (0.01%) was;
   стал = аналогичный lane_map по ногам (фиксация мёртвого лейна).
4. soak TPS (медиана поллов): was 2.0-2.5 (якоря-423 банк) → стал (min-of-3).
