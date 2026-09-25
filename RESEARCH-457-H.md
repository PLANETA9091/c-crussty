# RESEARCH-457-H — безумие-фабрика (agent-H, закон 11г)

Ветка: round-457h-ideas @ origin/master (ea1ccfa1). Роль: НЕ бенчить, НЕ имплементить —
находить/изобретать реально безумные идеи, доводить до готовности-к-имплементации,
выкладывать в BLACKBOARD «ИДЕИ-НА-ПОХИЩЕНИЕ». Фильтры: закон 4 (пустой lever = ваниль
бит-в-байт; отклонения только с parity-планом), закон 5 (запрещённый список), закон 6
(подсистема целиком, ОДИН bulk-JNI/тик), мёртвые вектора НЕ воскрешать (items/gsel/
fluid/mega/travel_diet/fluid_dirty_ledger), полные юнионы НЕ предлагать.

Метод: web-search (z-ai web_search CLI, ~45 запросов, сырые JSON в /tmp/457h/) по
F3-форкам (Folia/Pufferfish/Leaves/Luminol/C2ME/Krypton/Lithium/Hydrogen/Starlight/
Moonrise/FasterRandom/Noisium/VMP) + Rust game-dev (bevy, arc-swap, rayon+SoA,
parking_lot, crossbeam-epoch, io_uring, SWAR/SIMD, interval-tree/geo-index, roaring,
papaya/dashmap, flume MPMC, zstd-словари, SAP/box2d, NVIDIA broad-phase) + чужие
JVM-бенчи/issue-трекеры (bugs.mojang.com MC-311224, admincraft-треды).

Лейн-цели (профиль ×456, мастер f44a831e): broadphase 15.3 (остаток 9.4-10.9 на ногах),
nav_ai 13.7 (остаток 3.2), fastutil 8.7, java_util 6.5, paletted 6.0, chunk/worldgen ось
(ServerChunkCache scheduling 4.6-5.2, chunk-send, RegionFile-IO), poi (носитель B),
entity-query слой (носитель C eqsnap2).

Формат идеи: [ID-H##] название | лейн-цель % | механизм (2 строки) | сайты в
/home/z/c-crussty | parity-план | прогноз Δ и почему может ≥+20 | риск | источник.

---

## ПАКЕТ 1 — BROADPHASE/QUERY-ОСЬ (H01-H07)

### [ID-H01] SWAR/SIMD batch-AABB broadphase (box2d-техника) | ARCH
- **Лейн-цель**: broadphase остаток 9.4-10.9пп (mob-push хвост + getEntities-хвост).
- **Механизм**: Rust-плоскость принимает батч запросных боксов + SoA-массив центров/радиусов
  (mobs_soa уже плоский) и прогоняет 8-лановый AVX2/SWAR-intersect (два пакета min/max на
  8 кандидатов за инструкцию), возвращая битмаску кандидатов ОДНИМ bulk-JNI на батч.
  Java re-validate строгим AABB.intersects как сейчас —candidate-set не меняется.
- **Сайты**: src/mobs_soa.rs (массивы x/y/z/hw/hh уже в SoA), src/collide_batch.rs
  (прототип batch-call), java mobpush/net/minecraft/world/entity/MobPushOps.java:467
  (маршрут батча, NCDFE-канон define в раннем arm), src/classfile.rs (patch-секция MOBPUSH).
- **Parity**: strict-неравенства реплицируются целочисленным сравнением битовых паттернов
  f64 (NaN→false как dcmpg); candidate-set = superset → java strict-хвост = бит-в-байт;
  oracle-харнес уже есть (superset-доказательство в шапке mobs_soa.rs — расширить на SIMD).
- **Δ-прогноз**: захват 25-40% остатка 9.4-10.9 → нога +2.3-4.3пп поверх текущих плоскостей;
  при паре с якорем a15 +9.7@6813110 (Δ≤50k окно) → 12-14; самостоятельный монстр маловероятен,
  но в носителе chunkmono/poi как R2-плейн (sub-additive добавка кcheduling-плейна) даёт шанс ≥+20.
- **Риск**: AVX2 недоступен на runner → fallback SWAR (u64×4) — тот же прогноз −15%;
  партиционирование батча по 8 ломает strict-порядок обхода (порядок не наблюдаем — ок).
- **Источник**: https://box2d.org — Erin Catto, «SIMD for Collision» (batch SIMD broad-phase,
  2026-07); подтверждено поиском /tmp/457h/q12_simd.json.

### [ID-H02] Монотонный sweep-and-prune (sort&sweep) для push-хвоста | ARCH
- **Лейн-цель**: broadphase остаток 9.4-10.9пп (сеточный walk 3×3 ячеек на моба).
- **Механизм**: держим в SoA монотонно поддерживаемый массив (id, xmin, xmax), отсортированный
  один раз/тик инкрементально (почти-отсортирован между тиками — insertion sort O(n) амортиз.);
  push-запрос = binary-search по xmin/xmax вместо walk сетки; ответы пакетом в одном bulk-JNI.
- **Сайты**: src/mobs_soa.rs (flatten-массивы), src/mobs_grid.rs (замещаемый fallback-плейн),
  mobpush/MobPushOps.java (переключатель плейна per-call ERR_RANGE-паттерн уже в кодовой базе).
- **Parity**: SAP-теорема — пересечение по ВСЕМ осям влечёт пересечение по x → candidate-set
  superset; strict-фильтр java остаётся; порядок выдачи = порядок сетки не требуется
  (java сортирует/фльтрует сам — сверить на харнесе).
- **Δ-прогноз**: push-хвост (MobPushOps scan ~3-4пп на ногах) захват 30-50% → +1-2пп
  сверх сидящих плоскостей; комбинируется с H01 в один bulk-JNI (не юнион — один механизм
  два прохода). Пара-потенциал через носитель chunkmono (ветка a: ноги уже +11.5/+9.1).
- **Риск**: вырожденные кластеры (ферма мобов в 1 ячейке) → O(n²) хвост; лечится
  grid-hybrid порогом (rare-case → vanilla сетка).
- **Источник**: NVIDIA GPU Gems 3, гл.32 «Broad-Phase Collision Detection» (sort-and-sweep
  канон) https://developer.nvidia.com/gpugems/gpugems3/part-iv-image-effects/chapter-32-broad-phase-collision-detection-gpu;
  + gamedev.stackexchange «Sweep and prune vs quad tree» (/tmp/457h/q13_sap.json).

### [ID-H03] Interval-tree таргетинг: geo-index packed Hilbert R-tree, bulk-load 1р/тик | ARCH
- **Лейн-цель**: nav_ai 13.7 (остаток 3.2) + broadphase getEntities-хвост (target-цели).
- **Механизм**: из SoA-масштаба собираем СТАТИЧЕСКИЙ packed Hilbert R-tree (geo-index crate,
  bulk-load за O(n log n) в Rust-памяти, без аллокаций в тике) один раз в начале тика;
  все NearestAttackableTarget/AvoidEntity-запросы тика бьются по дереву ОДНИМ bulk-JNI
  (список id-кандидатов на запрос) — java делает строгий хвост.
- **Сайты**: src/mobs_sense.rs (SenseOps-оракул), sense/net/minecraft/world/entity/SenseOps.java,
  entitygoalquery/ (носитель C — отдать ему как R3-плейн!), src/entity_index_manager.rs (точка
  инкремента после мутаций).
- **Parity**: candidate-superset → strict-хвост бит-в-байт; тик-выравнивание: дерево строится
  ПОСЛЕ всех тик-мутаций прошлых стадий и ДО target-стадии — на снапшоте тика N; отклонение
  только для сущностей, заспавненных МЕЖДУ — parity-план: dirty-append list (позаписной
  перехват add/removeEntity уже есть в entity_index.rs — добавить в дерево инкрементально
  или пометить dirty и включить в запрос).
- **Δ-прогноз**: target-запросы = NearestAttackableTargetGoal 0.81 + AvoidEntity 0.22 +
  vanilla-хвост getEntitiesOfClass 1.85 (профиль ×456/412-B) — остаток на ногах ~3.2пп;
  захват 40-60% → +1.3-1.9пп к ноге; у носителя eqsnap2 (agent-C leg-1 completed) это
  готовый R3 — его ноги +9-12 + этот плейн → пара ≥+20 реально.
- **Риск**: rebuild дорог при 47k мобов (меряем на харнесе; packed-Hilbert bulk-load ~1мс
  на 50k точек — приемлемо); мутации-между-фазами (dirty-план обязателен).
- **Источник**: https://lib.rs/crates/geo-index (packed Hilbert R-tree, static bulk-load) +
  https://github.com/georust/rstar (R*-tree альтернатива) — /tmp/457h/q23_rtree.json.

### [ID-H04] Roaring-bitmap occupancy: секция→множество живых id | ARCH
- **Лейн-цель**: broadphase 9.4-10.9 + fastutil спилловер (секционные списки = java-объекты).
- **Механизм**: на каждый chunk-section (16³) — roaring bitmap живых id (сжатие диапазонов
  плотных id-пространств); mirror-апдейты уже льются через сущ-плоскости — добавить 3 операции
  (add/remove/section-move), запрос «перечисли id секций-кандидатов» уходит в общий bulk-JNI.
  Roaring даёт O(1) AND/OR/счёт — гейт «секция пуста» без java-аллокаций.
- **Сайты**: src/entity_index.rs (home-chunk mirror), src/mobs_grid.rs (64-shard ключи секций),
  entityquery/net/minecraft/world/entity/EntityIndexOps.java.
- **Parity**: битмап — superset-плейн (те же id из тех же вызовов add/removeEntity) → strict
  tail java; AIOOBE-гейт не трогается; паразитных отклонений нет — плоскость только чтение.
- **Δ-прогноз**: «пустая секция» гейты убирают java-итерацию списков (ChunkEntitySlices
  section walk — часть 9.4-10.9 остатка) — захват 15-25% → +1.4-2.7пп; как add-on к
  entity_index (уже в мастере) — дешёвый носитель: ноги entity_index-семьи +12-14 + H04
  → пара-шанс ≥+20.
- **Риск**: double-bookkeeping (id в 2 структурах) — расхождение ловит selfTest
  (битмап-count vs цепочка-count per секция — дёшево, раз в 100 тиков).
- **Источник**: https://docs.rs/roaring (RoaringBitmap) + debian/roaring подтверждение
  /tmp/457h/q41_obits.json.

### [ID-H05] papaya-style read-mostly concurrent map (для agent-F shardlock) 
- **Лейн-цель**: broadphase остаток (шард-локи contention).
- **Механизм**: сущ-шард-таблица (F-вектор cmp457_shardlock) переезжает с per-shard
  Mutex+seqlock на papaya-семантику: per-shard epoch-guarded мутируемая сторона + ПОЛНОСТЬЮ
  lock-free читатели (старая версия живёт до passage of grace) — Java-мутатор один/тик.
- **Сайты**: src/mobs_grid.rs (Shard struct — заменить wlock:Mutex на epoch), кросс-реф:
  src/entity_index_manager.rs; работа агента-F (round-457g/457f ветки — НЕ трогать, идея
  для похищения).
- **Parity**: seqlock-гейты (QRETRY=256, ERR_RANGE fallback) заменяются epoch-retry —
  оракул тот же (rand-популяции, superset-проверка), ERR_RANGE частота должна упасть.
- **Δ-прогноз**: contention-хвост broadphase (шард-миссии) 1-2пп + убирает редкие
  fallback-затраты; носитель F-ветки + цифры их ног; шанс ≥+20 только в паре с носителем.
- **Риск**: ABA на переиспользовании id — epoch-tag на id (старший бит поколения).
- **Источник**: https://docs.rs/papaya («lock-free reads … per-shard mutexes» — snippet
  подтверждён) + https://docs.rs/dashmap — /tmp/457h/q20_dashmap.json.

### [ID-H06] Bloom-фильтр парности секций (visibility prune)
- **Лейн-цель**: broadphase getEntities-хвост (дальние target/collision запросы).
- **Механизм**: на тик собираем 64-битный bloom «какие секции вообще заняты» (fpr <2% при
  47k id в 4КБ); запросы уровня Level.getEntitiesOfClass на дальних боксов отсеиваются ДО
  java-обхода секций одним bulk-вызовом (батч всех запросов тика — integer filter).
- **Сайты**: src/mobs_grid.rs (масштабирование секций), src/queryplane.rs (сайт-3 гейт
  HARD_ADDS — тот же паттерн «пустой fast-path»), entityquery/EntityIndexOps.java.
- **Parity**: bloom отсеивает только ПУСТЫЕ секции (false-positive допустим — кандидат
  пройдёт в java) → candidate-set идентичен; QUERYPLANE STRICT-оракул расширяется на bloom.
- **Δ-прогноз**: на факел-мире большая часть секций пуста → доля короткозамкнутых запросов
  40-70%; lane getEntitiesOfClass 1.85пп − 50% = +0.9пп дёшево; добавка к queryplane-носителю.
- **Риск**: пересборка bloom каждый тик — 47k вставок ~микросекунды (u64-хэш) — ок;
  false-negative запрещены — детерминированный хэш-набор + selfTest-инвариант.
- **Источник**: классика bloom-фильтров — подтверждение применения в пространственных
  гейтах: /tmp/457h/q15_bloom.json (поиск-контекст), docs.rs/bloom.
  Примечание: источник слабый — идея банальна в реализации, риск низкий.

### [ID-H07] Hilbert-порядок section-walk (cache locality, без изменения множества)
- **Лейн-цель**: broadphase 9.4-10.9 (memory-bound хвост обхода секций).
- **Механизм**: java-обход ChunkEntitySlices идёт по линейному cx/cz-порядку; mirror-плоскость
  отдаёт кандидатов в Hilbert-порядке (плотная кривая = соседние секции в соседних кэш-линиях);
  это НЕ изменение candidate-set, только порядок выдачи + prefetch-хинты в Rust.
- **Сайты**: src/entity_index.rs (порядок выдачи цепочек), src/entity_compose.rs.
- **Parity**: порядок обхода в vanilla частично влияет на tie-breaking (nearest-pick!) —
  parity-план: STRICT-режим по умолчанию ВЫКЛ, включается только если оракул-харнес
  докажет эквивалентность tie-break на 10k случайных сцен (или отдаёт порядок = vanilla).
- **Δ-прогноз**: 0.3-0.8пп чисто кэш-эффект — копеечная идея, держим в списке как
  «бесплатный» пункт пары.
- **Риск**: tie-break отклонения = видимая разница выбора цели — без оракула НЕ выпускать.
- **Источник**: geo-index Hilbert packing https://lib.rs/crates/geo-index; суперскрафт-обзор
  соседних техник /tmp/457h/q23_rtree.json.

---
(пакеты 2-4 добавляются ниже, коммит на каждый пакет)
