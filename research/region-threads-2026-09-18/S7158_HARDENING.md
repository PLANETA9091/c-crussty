# S7-158 — REGION-THREADS HARDENING (S7-158a/b/c/d) — 2026-09-19

Цель: закрыть три блокера банкования рычага #7 (TPS +66.7% доказан в leg #2
35363758352, банкование отложено: tracker-race NPE + UUID-dup + PG4 GC
+31.4%) и вывести leg #3 min-of-2.

## S7-158a — CI-гигиена post-soak (наследовано от незакоммиченной работы
предыдущего тика, проверено и принято)

`bench/world3/run_world3.sh`:
- `cmd()` = `timeout 5 sh -c 'printf > console.in'`: `echo > FIFO` открывает
  FIFO БЕЗ reader'а → open() блокируется навсегда (59-мин ожог leg #2 после
  выгрузки артефакта: tail умер по SIGPIPE, консольный канал мёртв). Каждый
  консольный write теперь ограничен 5 с.
- `report_world3.py` = `timeout 180` (последняя неограниченная операция
  перед выходом).

## S7-158b — tracker-race: root-cause (javap, kernel e2992d63) + фикс

Крэш leg #2 (15:54:54, soak-хвост, ПОСЛЕ выгрузки spark — данные валидны):
`NullPointerException: ... "entity" is null` @ `ChunkMap.newTrackerTick:1017`
← `ChunkMap.tick:1033` ← `ServerChunkCache.tick:495` ← `ServerLevel.tick:815`
(main, `MinecraftServer.tickChildren`).

Разбор байткода `newTrackerTick`:
- line 1013-1015: `ServerEntityLookup entityLookup = level.moonrise$getEntityLookup();
  ReferenceList trackerEntities = entityLookup.trackerEntities;
  Entity[] raw = trackerEntities.getRawDataUnchecked(); int len = size();`
- line 1016-1017: `for (i=0; i<len; i++) { Entity entity = raw[i];
  entity.moonrise$getTrackedEntity(); }` — НЕпроверяемый обход (unchecked) с
  захваченной длиной и БЕЗ null-чека на `entity` (первый invoke по слоту).
- Parallel worker'ы удаляют сущности из мира в своей фазе (немедленная
  цепочка удаления) → `ReferenceList` (swap-remove) зануляет хвост того же
  backing-массива → main на следующем свипе, захватив len до сжатия, читает
  нулевой слот → NPE. (Крэш проявляется в окне, когда мутация попадает
  между `size()` и чтением слота; редкая, но фатальная.)

Фикс = «removal-safe итерация» (опция 1 preregistered плана S7-158):
`TrackerTickOps` (net.minecraft.server.level, kernel loader) — тело ванили
байт-в-байт (javap-сверка: те же интерфейсы/порядок вызовов; условие
sendChanges = `hasPlayers || status.isOrAfter(ENTITY_TICKING)`) + ровно одно
отличие: нулевой слот SKIP (ваниль бы упала — состояние недостижимо в
безгоночном мире, наблюдаемая семантика неизменна).
Ретаргет: единственный сайт `newTrackerTick` в kernel — `ChunkMap.tick()V`,
чьё тело = ровно этот один вызов (javap 0-4) → `TrackerTickOps.newTrackerTick(
ChunkMap)` (receiver-prepended 1:1). Strict Retargeted{1}, fail-closed.
Отклонённая опция (deferral EntityLookup-удалений в FIFO) меняет same-tick
broadphase-видимость — отвергнута в пользу сохранения ванильных таймингов
воркеров.

## S7-158d — UUID-сидирование: root-cause (javap) + фикс

Инцидент leg #2: 1× UUID-dup WARN (база 0×): worker-спавн Arrow (skeleton
выстрел) алиасил UUID с Rotten Flesh (~900 блоков, другой регион) →
EntityLookup «can't add» → спавн ПОТЕРЯН (реальное отклонение поведения).

Разбор:
- `PurpurWorldConfig.entitySharedRandom` — ДЕФОЛТ = TRUE (iconst_1 в
  javap-дизасме конфиг-дефолтов) → Entity ctor:
  `this.random = purpurConfig.entitySharedRandom ? Entity.SHARED_RANDOM :
  RandomSource.create(); this.uuid = Mth.createInsecureUUID(this.random);`
- `Entity.SHARED_RANDOM` = `new Entity$RandomRandomSource extends
  ThreadUnsafeRandom` (имя говорит само; setSeed-лок только для
  анти-эксплойта).
- `Mth.createInsecureUUID(RandomSource)` = 2× nextLong без синхронизации →
  два параллельных ctor'а читают одно состояние генератора → идентичная
  пара UUID.
- Census вызовов (бинарный скан CP всего jar): вариант (RandomSource) зовут
  ровно 2 класса — Entity (ctor) и сам Mth (no-arg обёртка над Mth.RANDOM =
  `createThreadSafe()`, потокобезопасен, не трогаем); ServerBossEvent —
  no-arg вариант. Единственный боевой сайт = Entity ctor.

Фикс: `RngOps` (net.minecraft.util, kernel loader) — ванильная математика
UUIDv4 бит-в-бит (most = nextLong() & -61441 | 16384; least = nextLong() &
4611686018427387903 | Long.MIN_VALUE) под `synchronized (random)` —
монитор ТОЛЬКО на время двух дров при КОНСТРУИРОВАНИИ сущности (спавны
редки против 150k тикающих; per-tick AI-дровы на общем источнике сохраняют
прежний доброкачественный lost-update рэс — без изменения поведения
относительно leg #2).
Ретаргет: `retarget_invokestatic` (static→static, ТОТ ЖЕ дескриптор —
стек-шейп идентичен) в `Entity.<init>(EntityType,Level)` →
`RngOps.createInsecureUUID(RandomSource)`. Strict Retargeted{1}.
Config-wins НЕ используется (запрещено владельцем) — фикс кодовый.

## S7-158c — GC-диета моста (наследовано, найден и исправлен баг)

`RegionTickOps.parallelTick` (zero-alloc steady state): персистентные
слотовые массивы (grow-on-overflow), общий consumer-константа, гигиена
хвоста (null за len после джойна). БАГ найден харнессом в parallel child:
`new Entity[w][]` оставляет null-слоты → NPE на `b.length` первого fill →
фикс: слоты инициализируются `new Entity[0]`. Ожидаемый эффект на leg #3:
young GC 155 → ≤135 (кап PG4 = база 118 +15%); механика: v1 аллоцировал
снапшот-лист 150k ref + W бакет-листов + consumer-массив КАЖДЫЙ тик.

## Верификация (весь офлайн-эшелон)

- cargo test --lib: **147 passed / 0 failed / 1 ignored** (+3: strict
  Retargeted{1} обоих новых патчей на реальных фикстурах ChunkMap_real/
  Entity_real + idempotent + fail-closed на мусоре/чужих классах).
- RegionThreadsHarness OFFLINE PASS: structural (верификатор принимает
  патченные ServerLevel/EntityCallbacks/Level/ChunkMap) + wiring (Methodref
  TrackerTickOps в ChunkMap, RngOps в Entity) + dormant + parallel child
  (exactly-once) + **S7-158b-регресс**: sweep переживает [e1, null, e2] при
  len=3 (ванильная форма крэша) + чистый список = no-op.
- **Урок харнесса**: патченный ENTITY нельзя оффлайн-дефайнить в child
  лоадере — parent-loader ItemEntity extends parent-Entity → сплит
  идентичности → VerifyError в spawnAtLocation (артефакт оффлайна; в
  runtime патч обслуживается ТЕМ ЖЕ лоадером, что и ItemEntity). Гейты
  Entity = wiring-Methodref + cargo-roundtrip + живой retransform rc.
- **S7-158d-регресс**: математика UUID бит-в-бит = Mth на одном сиде;
  2×2000 параллельных конструкций над ОДНИМ ThreadUnsafeRandom → 4000
  уникальных UUID, 0 дубликатов (ваниль без синка периодически алиасит).
- **PG1 LOCKSTEP PASS**: дайджест `61e3c374…941d5` БИТ-В-БИТ неизменён
  (W=1==W=2==W=4, 60 тиков, 400 сущностей, шторм мутаций, финал 413) —
  фиксы не трогают per-entity семантику.

## Watchlist leg #3 (не фиксится в этом тике)

`ServerLevel.sendBlockUpdated:1883` — итерация `navigatingMobs`
(ObjectOpenHashSet) на воркере против параллельной мутации навигации
(старт/стоп на владеющем воркере). Проявился 1 раз за 15 мин soak leg #2
(15:52:02), НЕ фатален (guardEntityTick контейнит, сущность пропускает
тик). Если частота > 1/час на leg #3 → S7-159: мост-сериализация путей
обновления блоков или копия множества. Ловится grep'ом stdout.

## Инъекционная поверхность (region_threads v3)

5 таргетов: ServerLevel Retargeted{1}; EntityCallbacks Retargeted{1}+{1};
Level Retargeted{1}; ChunkMap Retargeted{1}; Entity (ctor) Retargeted{1}.
Bridge'и в kernel loader: RegionTickOps + $Mut + TrackerTickOps + RngOps.
Audit: `forEach/onTickingStart/onTickingEnd/midTickTasks/trackerTick/
rngUUID`, «region_threads v3». Всё остальное (гейт CRUSSTY_REGION_THREADS,
дормант, WARN F1/F3) — без изменений.

INJECTS-ONLY цел: 0 sandbox-бутов; leg #3 = preregister A/B
(санкционированный CI-ран).
