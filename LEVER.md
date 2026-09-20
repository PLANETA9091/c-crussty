# LEVER — items_manager (agent J, TASK-395)

## Идентификатор
- `CRUSSTY_LEVER_FLAG=items_manager`, `CRUSSTY_LEVER_ARG=1`
- Требует `CRUSSTY_REGION_THREADS>=2` и `REGION_STEAL=0` (статический режим
  параллельного тика — конфиг банка v4). Пустой/чужой флаг = точный vanilla.

## Механизм (полная замена подсистемы)
ItemEntity больше не идут через общий entity-tick dispatch:
`ServerLevel.lambda$tick$4 → Level.guardEntityTick → ServerLevel.tickNonPassenger
→ Entity.tick → ItemEntity.tick`. В фазе 1-2 RegionTickOps (снапшот
EntityTickList) items маршрутизируются в per-slot ПЛОТНЫЕ массивы
(`itemArr`, grow-on-demand, персистентные — тот же zero-alloc протокол, что
`bucketArr`, S7-158c). В фазе 3 каждый бакет-поток перед общей фазой исполняет
`ItemEntityManager.tickSlot(slot)`:

1. **gather** — плотные массивы в ванильном порядке снапшота (порядок items
   внутри секции сохранён).
2. **batch-движение** — побайтная реплика `ItemEntity.tick` (javap
   purpur-1.21.10, offsets 0..588) через ванильные Entity-методы:
   baseTick, move(MoverType.SELF), applyEffectsFromBlocks, applyGravity,
   updateInWaterStateAndDoFluidPushing, moveTowardsClosestSpace, noCollision,
   friction/bounce, discard. `getItem()` hoisted 1x/тик (SynchedEntityData
   не мутирует внутри тела). Гейты dispatch-цепочки реплицированы по javap:
   lambda$tick$4 (isRemoved/isEntityFrozen/checkDespawn/vehicle-гейт),
   tickNonPassenger (setOldPosAndRot, tickCount++/totalEntityAge++,
   ActivationRange.checkIfActive → tick/inactiveTick), guardEntityTick catch
   (log + ServerExceptionEvent + discard(DISCARD)).
   Опущены только не имеющие игровой семантики: profiler push/incrementCounter
   и ServerLevel.currentlyTickingEntity (package-private диагностика крашей).
3. **merge — ВАНИЛЬНЫЙ**: приватный `ItemEntity.mergeWithNeighbours()`
   вызывается по MethodHandle (privateLookupIn, резолв 1x) в точном ванильном
   месте цикла (`tickCount % (moved?2:40) == 0 && isMergable()`) — ноль
   репликации мердж-логики, порядок и позиции бит-в-бит. Данные профиля
   (run 35528326290): merge = 0.09% items-лейна — батчить его бессмысленно.
4. **despawn/age** — в том же проходе: `age++`, ванильный
   `CraftEventFactory.callItemDespawnEvent` → cancel? age=0 : discard(DESPAWN);
   despawnRate (private) — по MethodHandle findGetter, как и
   `Entity.despawnTime` (Purpur-гейт в Entity.tick).
5. **применение к ядру** — ТОЛЬКО ванильные методы/коллбеки; удаления через
   discard (ванильный EntityRemoveEvent flow), мутации списков сущностей —
   через существующие ретаргеченные EntityCallbacks.

Пассажирные ItemEntity (патология) НЕ маршрутизируются — остаются в ванильном
consumer-пути. STEAL-путь не модифицирован (lever в нём неактивен).

## Диспетч-структура, которую устраняет lever (items-лейн 31.17%)
per-item: lambda$tick$4 invoke, guardEntityTick accept (try/catch),
tickNonPassenger (AtomicReference get/lazySet x2, profiler push(Supplier)-
 invokedynamic + incrementCounter), Entity.tick+ItemEntity.tick виртуальный
dispatch, EntityTickList hash-итерация, множественные getItem()→
SynchedEntityData.get. Замена: плотный массив + прямые вызовы.

## Деливери (паттерн batch_collector, define-only мост)
- `entityinside/net/minecraft/world/entity/ItemEntityManager.java` — новый
  bridge-класс (пакет net.minecraft.world.entity = доступ к protected
  Entity-методам).
- `RegionTickOps.java` — маршрутизация в fill + item-фаза в tickBucket +
  hygiene. Сигнатуры существующих retarget-сайтов не менялись.
- `src/items_manager.rs` — define ItemEntityManager в KERNEL loader
  (BRIDGE-READY паттерн, fail-closed), вызов из lib.rs после batch_collector.
- Компиляция: javac --release 21 -cp <kernel.jar:paper-api:adventure-api:
  adventure-key:examination-api:fastutil> -d entityinside/build
  ItemEntityManager.java RegionTickOps.java; .class коммитятся.

## Fail-closed матрица
- класс не определён / MethodHandle не резолвится → itemsManagerArmed()=false
  → items идут ванильным consumer'ом бит-в-бит;
- пустой CRUSSTY_LEVER_FLAG → armed()=false по построению;
- region_threads<2 или REGION_STEAL=1 → lever неактивен.

## Паритет
- Ванильные результаты: пары (merge — ванильный код), пикап (не в конвейере:
  playerTouch остаётся в тике игрока), деспавн (ванильные события/гейты).
- Порядок items внутри секции — сохранён (порядок снапшота).
- Item-vs-моб интерлив внутри бакета — принятый interleave-класс
  кросс-бакетного параллельного тика (S7-155/RECON-15).
