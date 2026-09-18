# S7-154 RECON-2 — раскладка non-entity main-tick (CUMULATIVE 35330129145)

- total CPU: **52341**
- non-entity main-tick (tickServer без entity-цикла): **4304 = 8.22% CPU**

## Якоря (первый mc-кадр ниже main-loop)

| элемент | samples | % фазы | % всего CPU |
|---|---|---|---|
| `net/minecraft/world/level/entity/EntityTickList.forEach` | 614 | 14.3% | 1.17% |
| `net/minecraft/world/level/Level.setBlock` | 50 | 1.2% | 0.10% |
| `net/minecraft/commands/Commands.performPrefixedCommand` | 43 | 1.0% | 0.08% |
| `net/minecraft/world/level/Level.tickBlockEntities` | 32 | 0.7% | 0.06% |
| `net/minecraft/world/level/block/piston/PistonBaseBlock.moveBlocks` | 30 | 0.7% | 0.06% |
| `net/minecraft/world/level/Level.removeBlock` | 15 | 0.3% | 0.03% |
| `org/bukkit/craftbukkit/scheduler/CraftScheduler.mainThreadHeartbeat` | 14 | 0.3% | 0.03% |
| `io/papermc/paper/SparksFly.executeMainThreadTasks` | 9 | 0.2% | 0.02% |
| `io/papermc/paper/entity/activation/ActivationRange.activateEntities` | 5 | 0.1% | 0.01% |
| `net/minecraft/server/network/ServerCommonPacketListenerImpl.resumeFlushing` | 1 | 0.0% | 0.00% |
| `java/util/concurrent/CopyOnWriteArrayList.forEach` | 1 | 0.0% | 0.00% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet.removeFirst` | 1 | 0.0% | 0.00% |
| `net/minecraft/util/debug/ServerDebugSubscribers.tick` | 1 | 0.0% | 0.00% |
| `net/minecraft/util/debug/LevelDebugSynchronizers.tick` | 1 | 0.0% | 0.00% |
| `net/minecraft/server/MinecraftServer.buildServerStatus` | 1 | 0.0% | 0.00% |
| `java/lang/invoke/Invokers$Holder.linkToTargetMethod` | 1 | 0.0% | 0.00% |
| `net/minecraft/server/ServerFunctionManager.tick` | 1 | 0.0% | 0.00% |
| `io/papermc/paper/SparksFly.tickEnd` | 1 | 0.0% | 0.00% |

## Листья (последний mc-кадр)

| элемент | samples | % фазы | % всего CPU |
|---|---|---|---|
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f663d9d8d80.accept` | 410 | 9.5% | 0.78% |
| `net/minecraft/world/entity/Mob.checkDespawn` | 96 | 2.2% | 0.18% |
| `net/minecraft/commands/execution/tasks/ExecuteCommand.execute` | 40 | 0.9% | 0.08% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | 36 | 0.8% | 0.07% |
| `net/minecraft/world/level/Level.tickBlockEntities` | 32 | 0.7% | 0.06% |
| `net/minecraft/world/entity/ai/navigation/PathNavigation.shouldRecomputePath` | 31 | 0.7% | 0.06% |
| `net/minecraft/world/entity/Entity.isRemoved` | 30 | 0.7% | 0.06% |
| `net/minecraft/world/level/pathfinder/Path.getEndNode` | 26 | 0.6% | 0.05% |
| `net/minecraft/world/level/pathfinder/Path.isDone` | 22 | 0.5% | 0.04% |
| `net/minecraft/world/entity/Entity.checkDespawn` | 13 | 0.3% | 0.02% |
| `net/minecraft/server/MinecraftServer.tickChildren` | 10 | 0.2% | 0.02% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | 8 | 0.2% | 0.02% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | 6 | 0.1% | 0.01% |
| `net/minecraft/server/MinecraftServer.tickServer` | 6 | 0.1% | 0.01% |
| `net/minecraft/server/level/ServerLevel.tick` | 5 | 0.1% | 0.01% |
| `net/minecraft/server/level/ServerLevel.sendBlockUpdated` | 4 | 0.1% | 0.01% |
| `net/minecraft/world/TickRateManager.runsNormally` | 4 | 0.1% | 0.01% |
| `net/minecraft/world/entity/Entity.getVehicle` | 4 | 0.1% | 0.01% |
| `net/minecraft/world/entity/Entity.level` | 3 | 0.1% | 0.01% |
| `net/minecraft/server/level/ServerPlayerGameMode.getGameModeForPlayer` | 3 | 0.1% | 0.01% |
| `net/minecraft/world/entity/Entity.getControlledVehicle` | 2 | 0.0% | 0.00% |
| `net/minecraft/core/Vec3i.distToCenterSqr` | 2 | 0.0% | 0.00% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | 2 | 0.0% | 0.00% |
| `net/minecraft/world/entity/Mob.getNavigation` | 2 | 0.0% | 0.00% |
| `net/minecraft/world/level/LevelSettings.difficulty` | 1 | 0.0% | 0.00% |
| `net/minecraft/world/phys/AABB.intersects` | 1 | 0.0% | 0.00% |
| `net/minecraft/world/level/pathfinder/Path.getNodeCount` | 1 | 0.0% | 0.00% |
| `net/minecraft/world/entity/Entity.getBukkitEntity` | 1 | 0.0% | 0.00% |
| `net/minecraft/world/level/storage/PrimaryLevelData.getDifficulty` | 1 | 0.0% | 0.00% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f663d9d8fb8.accept` | 1 | 0.0% | 0.00% |

## broadphase/getEntities-семейство (все стеки)

| элемент | samples | % фазы | % всего CPU |
|---|---|---|---|
| `ChunkEntitySlices$EntityCollectionBySection.getEntities` | 2090 | 4.0% | 3.99% |
| `CollisionUtil.getCollisions` | 1962 | 3.7% | 3.75% |
| `ChunkEntitySlices.getEntities` | 952 | 1.8% | 1.82% |
| `pushEntities` | 457 | 0.9% | 0.87% |
| `AABB.intersects` | 26 | 0.0% | 0.05% |
| `EntityGetter.getEntities` | 19 | 0.0% | 0.04% |

---

## ВЕРДИКТ S7-154 (RECON-2): пул attackable ≥5% исчерпан; следующий ×N-класс = region-threaded entity ticking (S7-155+ proposal)

### 1. Non-entity main-tick (4304 сэмпла = 8.22% CPU): доминанты НЕТ

Разложен до якорей/листьев: пассажиры (EntityTickList.forEach под lambda, 614 = 1.17% CPU),
команды консоли (0.08%), block entities (0.06%), поршни (0.06%), scheduler/spark (<0.05%).
Ни одной функции ≥2% CPU — по отдельности всё микро-класс.

### 2. Broadphase/entity-query семейство: 5461 сэмплов = 10.43% CPU (лег 35341241628: 9.57%)

Крупнейший оставшийся когерентный блок, НО раздроблен по семантике вызовов:

| вызыватель (ближайший выше) | сэмплы | % fam | семантика |
|---|---|---|---|
| ChunkEntitySlices.getEntities (итерация индекса) | 1688 | 30.9% | per-query обход секций moonrise |
| Entity.collide | 1049 | 19.2% | движение: разрешение коллизий с сущностями |
| Level.noCollision | 913 | 16.7% | дешёвые проверки пути/шага |
| LivingEntity.aiStep | 457 | 8.4% | pushEntities (расталкивание соседей) |
| ChunkEntitySlices.getHardCollidingEntities | 362 | 6.6% | жёсткие коллизии |
| EntityLookup.getEntities (обёртка) | 952 | 17.4% | (перекрыто с итерацией) |

По классам: Zombie 31.8% fam, ItemEntity 28.1%, Mob 15.6%, Skeleton 6.6%, Spider 6.5%, Creeper 6.1%
— размазано по ВСЕМ сущностям. Ванильная семантика требует каждый запрос каждый тик (расталкивание,
мердж, коллизии, sensing — решения принимаются по текущим позициям). Уроки FLUID-FREE/FLUID-DIRTY:
позиционно-нестабильная популяция (все сущности X150K движутся) делает per-entity кэш результата
бессмысленным; батч-амортизация (запрос раз в N тиков) меняет наблюдаемую логику = запрещено;
O(n)→O(1) уже сделан moonrise (индекс по секциям); JNI-порт per-call перекрывается стоимостью
перехода (урок ALLOC-DIET: даже invokestatic-мост экономически отрицателен).

### 3. Ядерная карта (4-ядерный раннер): последний ×N-рычаг

| поток | доля CPU | состояние |
|---|---|---|
| main (entity-цикл + уровень) | 66.9% | насыщает ~1 ядро полностью |
| native (GC workers 28.8% + JIT 3.9%) | 32.7% | уже параллелен (G1 workers) |
| chunk-system workers (PrioritisedQueue) | 0.1% | ПРОСТАИВАЮТ |
| прочие | 0.3% | — |

58.5% entity-фазы исполняется на ОДНОМ потоке; 2-2.5 ядра из 4 в среднем свободны.
Единственный оставшийся ×N-класс рычаг из списка владельца («планировщики»):
**region-threaded entity ticking (Folia-модель)** — тик независимых регионов на параллельных
воркерах. Сохраняет per-entity семантику (сущности разных регионов не взаимодействуют внутри
тика), parity-риски = порядок тиков/РНГ между регионами. Это мега-проект (не одиночный тик):
 feasibility-гейт S7-155 → прототип планировщика → A/B на X150K.

### 4. Финальная карта эры ARCH-ATTACK (все ноги на валидной базе base-b, протокол S7-148)

- **GREEN (забанковано в CUMULATIVE)**: INSIDE-CACHE + FLUSH-DIET — flushStep −100%,
  inside-blocks −18.9% отн., young GC −5.6% (единственная комбинация с чистым A/B).
- **REFUTED (экономика)**: PALETTED-DEMUX+FLUID-FREE (1.8% CPU лейн, TPS −20%),
  ALLOC-DIET ×2 (мосты дороже экономии), FLUID-DIRTY (hit-rate 0% — движущаяся популяция).
- **Закрыто вне досягаемости паритета**: GC native (производная семантической работы),
  JIT, индексные запросы (moonrise уже O(log)).
- **Честная граница**: TPS X150K 0.8-0.9 упирается в СЕМАНТИЧЕСКИЙ минимум тика 150k сущностей
  (скан/движение/расталкивание/синхронизация обязаны выполняться каждый тик на ванильной логике).
  Одиночные кэш-рычаги исчерпаны; ×N-класс достижим только планировщиком (region-threading).

### 5. Инструменты

- s7154_recon2.py (анализатор: non-entity раскладка + broadphase drill + потоки + кросс-чек лега);
  база: run-s7149b-cumulative/cpu-collapsed.txt (52341), лег: run-s7153-fluid-dirty (54711).
