# RESEARCH-459-P22 — Chunk-tick eligibility flat-плоскость (ID-P22)

Агент: TASK-459-59 (WILD, закон 11, тик-459, c-crussty v18.2/18.3 законы 12e/13-16).
Ось: chunk/worldgen. Статус оси: ServerChunkCache scheduling = единственный живой на soak
закон-8 сайт вне GEN (×421-C: ген-идеи P24/P25 soak-инертны).
Скоуп коммита: scaffold (плоскость + java-мост, lever dormant, NCDFE-канон).

## 1. Механика (плоскость ID-P22)

Flat-плоскость eligibility тика чанков: java-мост собирает ПЛОСКИЕ массивы предикатов тика
чанков за тик (packed chunkXZ keys, dist/flags, playerXZ-близость) → ОДИН bulk-JNI → Rust
считает ТОЛЬКО математику решений и возвращает битмаску eligible-чанков (9216 чанков =
144×long = 1152 Б = 1 JNI-блок) → java строгий хвост: итерация ticking-чанков в ВАНИЛЬНОМ
порядке (ReferenceList raw-order), маска = предварительный superset-гейт (бит=0 ⇒ пропуск
только для ТОЧНЫХ битов), порядок/тайминг вызовов не меняются. Закон 6: подсистема целиком
(gate + bulk-решение + strict-хвост). v1 scope = distance-гейт only (биты B5/B6: playerTicking
∧ hasAnyNearbyNarrow — точные по K6/K7); супerset-односторонние биты B2/B3 (random-tick
sections/fluid) в v1 НЕ включаются (цена false-positive недопустима).

## 2. Сайты (kernel javap ground truth, jar round-chkmono457-19/patched-kernel.jar, RESEARCH-459-L02 §3)

- K3 `ServerChunkCache.iterateTickingChunksFaster()V` (Moonrise chunk_tick_iteration):
  ReferenceList<LevelChunk> → getRawDataUnchecked → for i: tickChunk(raw[i], speed);
  if ((i&7)==0) moonrise$executeMidTickTasks(). Eligibility уже плоский, но ПОЛНАЯ итерация
  каждый тик + mid-tick каждые 8.
- K6 `ChunkMap.collectSpawningChunks(List<LevelChunk>)V`: playerTickingChunks raw-проход +
  isChunkNearPlayer на каждый чанк (NaturalSpawner.createState 24 wall-сэмпла — нагруженный
  сосед) — холостой проход, съедаемый маской B5.
- K7 `DistanceManager implements ChunkTickDistanceManager`: PositionCountingAreaMap
  spawnChunkTracker + moonrise$hasAnyNearbyNarrow(II)Z — близость игрока уже O(1) на чанк
  (fake_players=4 soak), годится в супerset-маску.
- K2 `ServerChunkCache.moonrise$setFullChunk(IILevelChunk)V` — ЕДИНСТВЕННАЯ точка мутации
  fullChunks (javap C2-верифицировано) = bit-в-байт feed-точка маски.
- K1 `ServerChunkCache.getChunkNow(II)`: чистый CHM.get по CoordinateUtils key, O(1) —
  выигрыш ТОЛЬКО в устранении самих вызовов (bulk-предвычисление), не в их удешевлении.

Лейн-декомпозиция (RESULT-c2 ×456/H, L02 §2.5): getChunkNow 0.6-0.8 + CLLRCHT.getNode 1.2
(shared) + ServerChunkCache$$Lambda 0.8-1.0 + off-main 0.9-1.4 = 4.6-5.2пп; после C2 на
chk-14 остаток 4.3-4.9пп (видимая top-40 = 2.7).

## 3. Web-источники (≥2)

1. https://github.com/Tuinity/Moonrise — README: "Optimisation mod for the dedicated/integrated
   server"; портирует Paper-патчи: "Chunk system rewrite", "Random ticking optimisations";
   "Chunk ticking/loading/generation/saving … measurably lower tick times"; цель — "without
   changing Vanilla behavior" (собственная планка парити Moonrise); C2ME "fundamentally
   incompatible" (Moonrise заменяет чанк-систему целиком ⇒ вся модификация scheduling-lane
   обязана сидеть ПОВЕРХ moonrise-списков, не вместо них — K8 lockstep-контракт).
2. https://bugs.mojang.com/browse/MC/issues/MC-310372 — "Tripwires depowering breaks
   cross-chunk block tick processing order" (Priority: Important, Fixed 26.3): чанк-тик
   планирование критично к порядку (DRAIN_ORDER / INTRA_TICK_DRAIN_ORDER, chunk tick
   container выбирается по приоритету) — любое переупорядочивание обработки чанков рвёт
   cross-chunk контракты. Прямое подтверждение P22-дисциплины: маска НЕ меняет порядок
   итерации и тайминг (strict vanilla-order хвост), отсекает только заведомо ineligible.
3. https://minecraft.wiki/w/Tick — механика чанк-тика Java Edition: чанки уровня entity
   ticking тикаются каждый game tick; mob spawning/lightning требуют игрока в 8 чанках;
   1.21.5 (25w06a): random ticks получили ВСЕ fully-loaded чанки (legacy 128-блочный гейт
   снят) ⇒ distance-гейт v1 обязан зеркалить КЕРНЕЛ-предикаты (playerTicking +
   hasAnyNearbyNarrow area-map, K6/K7), а не legacy-128 — иначе маска не superset.
4. (kernel-внутренний) /home/z/rounds/ROUND-459/RESEARCH-459-L02.md §3-5 — javap-контракты
   K1-K8 + матрица битов B0-B7 + capture-матем (лейн 4.3-4.9пп после C2).

## 4. Parity-контракт

- Бит-в-байт: маска — ТОЛЬКО предварительный гейт; бит=1 ⇒ java-хвост обязан проверить
  ванильный предикат; бит=0 ⇒ пропуск только когда бит построен из ТОГО ЖЕ детерминированного
  знания (B1/B5/B6 точные; в v1 включаются только B5/B6). Lockstep-оракул G2: flat-маска ==
  nested-пересчёт (ReferenceList + holder status) бит-в-байт каждый тик; drift>0 = fail-open
  (дормант-дизарм).
- Порядок: хвост итерирует ReferenceList raw-order как ваниль (MC-310372 — порядок
  чанк-обработки паритичен критично); i&7-mid-tick плотность сохраняется (маска не режет
  индексы, только пропуски тела tickChunk при точном бите=0 — в v1 таких skip-ов даёт только
  distance-гейт на spawn-проходе K6).
- NCDFE-канон: ChunkSchedOps define_class в РАННЕМ arm-хуке (cplugin_init_impl →
  chunk_sched_flat::register() — до любого ретаргета и до первого тика); lever dormant ⇒
  класс defined, ретаргетов НЕТ, путь бит-в-бит ваниль (dormant-invisible).
- Fail-closed: define/selfTest/байт-контракт дефект ⇒ BRIDGE_READY не ставится, хуки не
  ставятся, WARN-маркер, ваниль.

## 5. Δ-прогноз и потолок (L02 §5, LEDGER ID-P22)

- Захват: iterateTickingChunksFaster i&7-планирование + collectSpawningChunks холостые
  проходы 25-35% лейна; getChunkNow-пробы 10-15%.
- Δ = lane% × захват% = 4.3×0.25 ≈ +1.1 … 4.9×0.40 ≈ +2.0; центр 4.6×0.30 = +1.4пп —
  совпадает с LEDGER-прогнозом +1.2-2пп.
- Потолок = +4.6пп ⇒ НЕ самостоятельный носитель; стековый слой климба chk-14 (+21.7):
  P22 (+1.4) ⊕ P31 INSIDE-BATCH (+5-8) ⊕ P32/P36 SNAP sidecar (+1.5-2.5) → нога +29.6…+33.7
  → пара с a26 (+12.4@8671791, Δ15k) ≈ +20.2…+21.3 ≥ +20 (закон 13c закрыт числами).
- carrier-член: GC-debt relief пары P21/P24/P27 инертен на soak — P22 несёт живую CPU-ногу оси.

## 6. Риски

1. Feed-drift ReferenceList ↔ маска (add/remove вне feed-точки) — lockstep G2 fail-open;
   в v1 маска rebuild каждый тик из flat-снимков (stateless, дрейф невозможен в пределах тика).
2. JNI-дисциплина (закон 6): ровно 1 bulk-crossing/тик, java-хвост без нативных вызовов.
3. B2/B3 супerset-биты недопустимы в v1 (лишний fluid-скан/пустой tickChunk = разрыв parity).
4. Отрицательный прецедент roar-2 (eindex seed failed → dormant): маркеры ARM обязаны
   грепаться; scaffold armState() возвращает DORMANT_SCAFFOLD до Wiring-стадии.
5. Конкуренция в leak-плоскости: getChunkNow L1-shadow уже занят mono-plane C2 — маска не
   дублирует C2 (снимает вызовы, не кэширует).

## 7. Скоуп scaffold (этот коммит) и CI

- src/chunk_sched_flat.rs — плоскость: STRICT-eq lever cmp459_chunksched-mask, dormant by
  default; register() = ранний arm-хук (NCDFE-канон), blob-плейсхолдер (wiring-стадия
  подставит chunksched/build/*.class через include_bytes!), fail-closed при пустом blob.
- chunksched/net/minecraft/server/level/ChunkSchedOps.java — java-мост stub: flat-collector
  контракт (chunkXZ[], flags[], playerXZ[]) + ОДИН native bulk-JNI evaluateEligibility
  (подпись зафиксирована, вызовов нет) + selfTest()=false (scaffold ⇒ дормант, fail-closed) +
  strict-tail контракт в javadoc.
- lib.rs: mod chunk_sched_flat + register() в cplugin_init_impl (после chunk_send5 — ранний
  arm-хук).
- ci.yml (только в ветке round-459-p22): + workflow_dispatch (иначе dispatch ci = HTTP 422,
  прецедент L01 round-459-lab-01) + javac-степ chunksched stub (--release 8, major 52 ≤ 65)
  + артефакт chunksched/build. Master не тронут.
