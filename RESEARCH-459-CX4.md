# RESEARCH-459-CX4 — WILD-агент закона 11 (карточка ID-83): spawn-block cohort bitset — однотиковая memoization повторных spawn-проверок идентичных блоков в NaturalSpawner-циклах

Агент: TASK-459-83 · закон 11 безумие C-X4 · подсистема: natural spawn (NaturalSpawner.spawnForChunk → spawnCategoryForChunk → spawnCategoryForPosition → isValidSpawnPostitionForType) · lever `cmp459_cx4` STRICT eq, dormant по умолчанию · STRICT superset-гейт: реюз вердикта только при доказанной эквивалентности входов; поведение спавна бит-в-байт (lever off = ваниль по построению).

## 0. Отправная точка (числа ×459)
- Лейн direct-CPU: **phase: mob spawning = 0.31% chk-14** (321/103062 сэмплов, монстр-нога +21.7@8687055); chk-16 = 0.3% (293); anchor-33 (ваниль-класс) = 0.1% (132). Лейн мал — это честно зафиксировано ДО старта (закон 16: сначала потолок, потом код).
- Alloc-хвост лейна: spawn-фреймы **11/3161 = 0.35% alloc-сэмплов** chk-14 (кадры: `PreCreatureSpawnEvent` alloc, `BlockPos.above` в `SpawnPlacementTypes.lambda$static$1`, 3× `NaturalSpawner$$Lambda` indy `allocateInstance`, `Collections$UnmodifiableMap$Entry.next` в mobsAt) — сопутствующая экономия, не самостоятельная ось.
- Активность спавна (owner-условие): spawnable-chunk polls **289** ×5 постоянны; churn **delta 5499 / total 148094..153593 = 3.6%** ACTIVE, summons=0 → натуральный spawn/despawn цикл exercised; фиксстура 150k pop.
- Контекст чейна: дефицит пары chk-14 = +10.7пп (закон 13a CLIMB); C-X4 — micro-swing слой, НЕ pair-maker (см. §5).

## 1. javap-контракты (канон round-396-a patched-kernel.jar, javap -p -c JDK-21)

1. **`NaturalSpawner.spawnCategoryForChunk(category, level, chunk, predicate, callback)`** (6-arg): тело = `getRandomPosWithin(level, chunk)` ОДИН раз → гейт `pos.getY() < level.getMinY()+1 → return` (offsets 7-18) → делегация в 8-arg `spawnCategoryForPosition(category, level, chunk, pos, predicate, callback, 0, null)`. Т.е. на каждый chunk×category спавн-цикла — **1 случайная позиция**; циклы попыток живут выше (ServerChunkCache.tickSpawningChunk).
2. **`NaturalSpawner.isValidSpawnPostitionForType(...)` (private, → PreSpawnStatus)** — точная цепочка проверок (offsets из javap -c): `PreCreatureSpawnEvent.<init>+callEvent+shouldAbortSpawn` (14→40, Paper-хук) → `canSpawnMobAt` (113: biome/mobsAt-плоскость) → `SpawnPlacements.isSpawnPositionOk` (124: `getPlacementType` → `SpawnPlacementType.isSpawnPositionOk` — block/fluid-гейты) → `SpawnPlacements.checkSpawnRules` (142: свет, monster light-level) → `noCollision(EntityType.getSpawnAABB(x,y,z))` (177-180). Смешанные входы: чисто блочные компоненты (state, light, collision) эквивалентны для одинаковых (stateId, light, AABB-сигнатуры); Paper-ивент — позиционно-зависимый (Location), НЕ когорт-переиспользуемый.
3. **`SpawnPlacements.isSpawnPositionOk(EntityType, LevelReader, BlockPos)`**: единственный вызов `getPlacementType(type)` (invokestatic) → `SpawnPlacementType.isSpawnPositionOk(reader, pos, type)` (invokeinterface) — т.е. таксономия проверок определяется placement-type'ом ⇒ cohort-ключ обязан включать класс placement×category, иначе superset нарушается.

**Ключевой вывод для дизайна:** повторные проверки одного чанка за тик падают в однотипные (state, light, collision) классы — на однородной фиксстуре (150k, однотипные поверхности) доля идентичных когорт среди 289×N попыток высока; но ивент-компонента (`PreCreatureSpawnEvent`) и canSpawnMobAt-плоскость не когорт-переиспользуемы → STRICT superset-гейт обязан их исключать из memo.

## 2. Идея C-X4: однотиковый cohort-bitset
- **Cohort** = класс эквивалентности spawn-проверок: ключ `u64 = stateId(24b) | light(4b) | collFlags(4b: noCollision-класс+fluid-класс) | catClass(8b: placement×category) | reserved`. Одинаковый ключ за тик = идентичный исход блочной части цепочки (контракт §1.2-1.3).
- **Bitset-memoization**: на тик (per level) фиксированная таблица 4096 слотов (open-addressing, u64-ключи) + bitset `checked[64×u64]`; probe(key): бит поднят → **skip повторной блочной проверки** (java гейтится ДО isSpawnPositionOk/checkSpawnRules/noCollision, ПОСЛЕ canSpawnMobAt+event); бит опущен → полный ваниль-чекаут и set-бит. Reset на границе тика = `fill(0)` 512B, нулевой alloc.
- **STRICT superset-гейт** (сердце безопасности): результат memoизируется ТОЛЬКО когда полная блочная цепочка выполнена до конца (флаги event-abort / unknown-light / unknown-collision → refusal, реюз запрещён навсегда для этой когорты); любое OOB/hash-аномалия → one-shot BROKEN латч → disarm в ваниль до рестарта JVM. При lever off класс не гейтится вообще → бит-в-байт.
- **Fail-closed во всех направлениях**: нет натива/нет маркера/переполнение таблицы/расхождение epoch → ваниль-путь; пропущенная проверка невозможна при refusals>0 на этой когорте.

## 3. Прецеденты (URL-рисёрч, 3 верифицированы curl)
1. **Paper per-player-mob-spawns** — https://docs.papermc.io/paper/reference/world-configuration/ (HTTP 200; флаг `per-player-mob-spawns` в world-defaults): легальный прецедент смены домена spawn-проверки (глобальный mobcap-проход → пер-плеер аккаунтинг) без изменения выдачи спавна — C-X4 повторяет паттерн «та же выдача, другой исполнитель/кэш».
2. **PostgreSQL enable_memoize** — https://www.postgresql.org/docs/current/runtime-config-query.html (HTTP 200): «caching results from parameterized scans inside nested-loop joins … allows scans to the underlying plans to be skipped when the results for the current parameters are already in the cache» — канон skip-repeated-identical-parameterizations, прямой аналог probe/skip по ключу параметров.
3. **Roaring bitmaps** — https://roaringbitmap.org/ (HTTP 202): bitset-формат для occupancy-индексации (прецедент у P45: roaring pre-gate). По памяти: Lithium (jellysquid) — секционные кэши коллизий/flat-AABB префильтры; Moonrise (Spottedleaf) — single-writer тик-тред владелец (наш bitset = tick-thread-local, без синхронизации); C2ME — vanilla-parity границы конкурентности.

## 4. Офлайн-эксперимент (закон 16, lockstep в юнит-тестах Rust + java selfTest)
- G2-lockstep: детерминированный реплей on/off → identical skip-решения; ложный реюз при дрейфе light/state = тест-фейл (тесты в `cargo test spawn_cohort`).
- java-твин selfTest (source-run, без блоб-ребилда): selfTestCohort=true — отказ реюза на event-abort-классах, skip только точных ключей, reset-семантика.

## 5. CAPTURE-МАТЕМ (честная, закон 14)
- lane 0.31% (chk-14) × захват 20-40% (доля когорт-хитов на однотипной фиксстуре при 289 спавн-чанках × 1 попытка/chunk/category; event+mobcap-плоскость не кэшируется) → **Δ +0.06..+0.12пп**; потолок компоненты **+0.31пп** (100% захват лейна).
- Alloc сопутствующий: 0.35% spawn-alloc-сэмплов × 30-50% = −0.1..−0.2% alloc-лейна (вторично).
- Роль в лестнице chk-14 (+21.7, дефицит пары +10.7): P31 INSIDE-BATCH (+5-8) ⊕ P32+P36 SNAP (+1.5-2.5) ⊕ papaya (+0.8-1.4) ⊕ **cx4 (+0.06-0.12)** — бортовой шум; ценность = дешёвая (no-JNI-хот-хвост) унификация плоскости memoization + отрицательное знание потолка лейна (если когорт-хиты < 20% — вектор закрыт числом).

## 6. PREREGISTERED ГЕЙТЫ G1-G6 (CI-нога round-459-cx4)
1. **G1 ARM**: boot-маркер `cx4_cohort ARMED` при lever=cmp459_cx4; counters checks_total>0, skips>0, refusals>=0; lever off → skips==0 (placebo-гейт).
2. **G2 lockstep бит-в-байт**: офлайн-реплей идентичен (см. тесты); CI-ценз: emission id-sequence спавна lever-on/off идентична (spawn-delta/churn в band).
3. **G3 GC**: young ≤ 128+10%, Full ≤ 9; cohort-структуры fixed-alloc (4096×8B таблица + 512B bitset), reset fill(0) — Δalloc ≈ 0.
4. **G4 популяция-паритет**: pop 140-165k; churn band (chk-14 справка: delta 5499, 3.6%).
5. **G5 TPS-бар**: банк v4 norm; Δ ≥ +0.06пп медиана min-of-3 band 6.0-9.5M; регрессия STRICT: Δ < −0.5пп = REFUTED (микро-лейн не вправе стоить дороже когорт-учёта).
6. **G6 fail-closed**: NCDFE=0 (EARLY-define SpawnCohortBitsetOps в arm-хуке ДО первого spawn-цикла, прецедент MobPushOps.pushables:467 / d73758a3-класс носителей), threw=0, AIOOBE=0, selfTest==true; BROKEN-латч/epoch-drift → авто-ваниль.

## 7. Вердикт-число и следующий шаг
- **0.31% × 20-40% → Δ +0.06..+0.12пп; потолок +0.31пп** (плюс alloc-хвост −0.1-0.2% 0.35%-лейна). Wave-2 (если G1-G6 зелёные): EARLY-define + вайринг probe в isValidSpawnPostitionForType после canSpawnMobAt, javap flat==nested ценз, min-of-3; иначе закрытие вектора отрицательным числом (когорт-хиты < 20% ⇒ REFUTED_CENS по потолку 0.31пп).
