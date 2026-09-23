# ABSORB S7-163 leg#1 (run 35407788083) — TECH-DUD: дефект доставки вложенного класса

Тик 08:43 +08 (2026-09-19), TASK-309. Лега: v4-кандидат = CUMULATIVE v3 + flat_traversal=1
(head 37a0072), старт 08:00:07 +08, FAILURE 08:31:42 +08.

## Хронология

- 00:09:08 UTC: бут готов, TPS 21.4 (до инжекта)
- 00:09:09 UTC: POPULATION INJECT START (target 150000, seed 42, loadedChunks 9949,
  farmClusters 497); entity_compose compose-события этой же секунды
- ARMED-цепь жива: «entity_compose: ARMED chain [inside->rng->batch->traversal],
  205458 -> 205677 bytes, retransform rc=0» (4 стадии composed, strict); region_threads
  ARMED (ServerLevel/EntityCallbacks/Level/ChunkMap rc=0); traverse_ops/batch_collector
  defined; flush_diet armed
- 00:09:13 → 00:09:17: инжект 6000 → 12000/150000; затем шторм
  «Entity threw exception … NoClassDefFoundError: net/minecraft/world/level/TraverseOps$LongTable
  at TraverseOps.forEachFlat(TraverseOps.java:83) ← Entity.checkInsideBlocks ←
  applyEffectsFromBlocks ← ItemEntity.tick» на КАЖДОЙ сущности с блоками под боксом
- 00:09:18 UTC: ReportedException «Exception while updating neighbours»
  (TrapDoorBlock.neighborChanged → setBlock → sendBlockUpdated → fastutil
  ObjectOpenHashSet$SetIterator NPE) → «This crash report has been saved …
  crash-2026-09-19_00.09.18-server.txt» → Stopping server. До профайл-окна не дошло;
  все гейты неизмеримы.

## Корень (инженерный, агентский)

traversal.rs embed'ил и определял в kernel loader ТОЛЬКО верхнеуровневый
`net/minecraft/world/level/TraverseOps.class`. Вложенный класс
`TraverseOps$LongTable` (open-addressing long-таблица dedupe, private static final,
TraverseOps.java:356) компилируется в ОТДЕЛЬНЫЙ classfile
`TraverseOps$LongTable.class` и при первом же NEW внутри forEachFlat резолвится
против ТОГО ЖЕ kernel loader, где его не было → ClassNotFoundException →
NoClassDefFoundError. Оффлайн-локстеп-оракул этот класс дефекта ЛОВИТЬ НЕ МОЖЕТ
по построению: обычный classpath резолвит вложенные классы неявно.

Вторичная амортизация (fastutil NPE на соседних обновлениях) — следствие шторма
ошибок тиков сущностей во время инжекта, не отдельный дефект; атрибуция одна.

## Вердикт: TECH-DUD (не REFUTED, не DUD-среда)

- НЕ REFUTED-BY-ECONOMICS: экономика рычага не измерена (падение ДО окна).
- НЕ средовой DUD (прецедент S7-162 leg#1): crash 100% воспроизводится нашим
  дефектом доставки, стек указывает точно в наш класс.
- Алгоритм не пострадал: classfile-ы байт-в-байт воспроизводимы
  (35192a1b…, 7157df0c…), локстеп-оракул 60106×4 PASS после пересборки.

## Фикс доставки (этот же тик)

1. `src/traversal.rs`: TRAVERSE_NESTED = [(«net/minecraft/world/level/TraverseOps$LongTable»,
   include_bytes!(…TraverseOps$LongTable.class))]; вложенные классы определяются в
   ТОТ ЖЕ kernel loader в ТОМ ЖЕ with_attached-замыкании, что и верхнеуровневый
   класс, ДО BRIDGE_READY (провал любого define → хук dormant, fail-closed цел).
2. Guard-тесты (cargo): (a) source-parse TraverseOps.java — каждый вложенный класс
   обязан быть в TRAVERSE_NESTED; (b) build-dir classfile set == embedded set.
   Сьют 157 passed / 0 failed / 1 ignored.
3. `scripts/build_traverse_ops.sh`: барьер — если javac произвёл ≠2 classfile-ов
   TraverseOps* → FAIL до диспатча (пересборка проверена: «nested-delivery guard:
   2/2 classfiles OK»).
4. `scripts/run_traverse_lockstep_harness.sh`: fallback для module-javac (тот же
   паттерн, что в build-скрипте); оракул повторно прогнан — 60106 сценария × 4
   visitor-политики, (posLong, step) последовательность + return бит-в-бит — PASS.

## Следствие

Редиспатч леги #2 тем же тиком (dispatch_s7163.py, те же прereg-входы, head =
фикс-коммит); absorb леги #2 — следующий тик по НЕИЗМЕННЫМ гейтам
PG2/PG3/PG4/CRASH-FREE. Пометка для PG2 leg#2: маркер «traverse_ops: defined …
(+1 nested)» (новая строка лога — «traverse_ops: defined nested
net/minecraft/world/level/TraverseOps$LongTable in kernel loader»).

## Урок эры (5-й о доставке)

Kernel-loader доставка байткода — это ДОСТАВКА ГРАФА КЛАССОВ, а не одного файла:
каждый вложенный/вспомогательный класс должен быть определён явно, а тесты сборки
обязаны сверять множество produced classfiles с embedded-множеством. Оффлайн-оракул
на обычном classpath слеп к этой категории дефектов — нужна либо CI-проверка
loader-семантики, либо статический guard на produced-vs-embedded (реализован).
