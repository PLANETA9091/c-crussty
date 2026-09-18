# S7-150 RECON — ранжирование пост-эры по CUMULATIVE 35330129145

- вход: `run-s7149b-cumulative/cpu-collapsed.txt`, всего self-time сэмплов: **52341**
- JVM-Java (attackable): **33393** (63.8%) · native/JVM-internal: **18948** (36.2%)
- entity-фаза (стеки через tickNonPassenger): **30625** (58.5%)
- unclassified-фаза (не-entity стеки): **21716** (41.5%)

## 1. Топ-25 attackable kernel-функций (self-time, без native/JVM)

| leaf frame | samples | %% of total |
|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 1688 | 3.23% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | 1368 | 2.61% |
| `net/minecraft/world/phys/AABB.intersects` | 981 | 1.87% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | 840 | 1.60% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | 712 | 1.36% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | 628 | 1.20% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | 541 | 1.03% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | 524 | 1.00% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | 501 | 0.96% |
| `net/minecraft/util/SimpleBitStorage.get` | 492 | 0.94% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | 466 | 0.89% |
| `net/minecraft/util/Mth.floor` | 443 | 0.85% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | 428 | 0.82% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f663d9d8d80.accept` | 410 | 0.78% |
| `jdk/internal/util/ArraysSupport.mismatch` | 362 | 0.69% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | 354 | 0.68% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | 338 | 0.65% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | 338 | 0.65% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | 333 | 0.64% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | 329 | 0.63% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | 328 | 0.63% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | 327 | 0.62% |
| `net/minecraft/world/entity/Entity.setOldPos` | 322 | 0.62% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | 299 | 0.57% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | 295 | 0.56% |

## 2. Entity-фаза по классам сущностей (stack presence под tickNonPassenger)

| класс-tick | samples | % entity-фазы | % всего CPU |
|---|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12261 | 40.0% | 23.43% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6873 | 22.4% | 13.13% |
| `net/minecraft/world/entity/Mob.tick` | 4307 | 14.1% | 8.23% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2101 | 6.9% | 4.01% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1864 | 6.1% | 3.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1522 | 5.0% | 2.91% |
| `entity-loop (no class tick)` | 514 | 1.7% | 0.98% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 276 | 0.9% | 0.53% |
| `net/minecraft/world/entity/npc/Villager.tick` | 166 | 0.5% | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 138 | 0.5% | 0.26% |
| `net/minecraft/world/entity/animal/horse/AbstractHorse.tick` | 99 | 0.3% | 0.19% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | 82 | 0.3% | 0.16% |
| `net/minecraft/world/entity/animal/frog/Frog.tick` | 77 | 0.3% | 0.15% |
| `net/minecraft/world/entity/animal/sniffer/Sniffer.tick` | 63 | 0.2% | 0.12% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 43 | 0.1% | 0.08% |

## 3. Под-лейны ItemEntity.tick (топ-1 класс, presence 12261 = 23.43% CPU)

| под-лейн | samples | % лейна |
|---|---|---|
| fluid-push/height | 4466 | 36.4% |
| move/collision | 2816 | 23.0% |
| inside-blocks | 2670 | 21.8% |
| base-tick misc | 988 | 8.1% |
| data-watcher/sync | 592 | 4.8% |
| merge-search (getEntities/broadphase) | 265 | 2.2% |
| sub-lane misc (unanchored) | 233 | 1.9% |
| item/stack logic | 141 | 1.1% |
| self (tick body) | 64 | 0.5% |
| palette/blockstate reads | 26 | 0.2% |

## 4. Под-лейны Zombie.tick (presence 6875 = 13.14% CPU)

| под-лейн | samples | % лейна |
|---|---|---|
| merge-search (getEntities/broadphase) | 1515 | 22.0% |
| base-tick misc | 1426 | 20.7% |
| AI/goals/brain | 1418 | 20.6% |
| move/collision | 793 | 11.5% |
| fluid-push/height | 674 | 9.8% |
| inside-blocks | 537 | 7.8% |
| palette/blockstate reads | 241 | 3.5% |
| data-watcher/sync | 198 | 2.9% |
| item/stack logic | 43 | 0.6% |
| sub-lane misc (unanchored) | 29 | 0.4% |
| self (tick body) | 1 | 0.0% |

## 5. Unclassified-фаза: группировка по корням (21716 = 41.5% CPU)

| группа | samples | % фазы | % всего CPU |
|---|---|---|---|
| native/JVM (GC, runtime, threads) | 15090 | 69.5% | 28.83% |
| ServerLevel.tick (вне entity-цикла) | 4221 | 19.4% | 8.06% |
| JIT | 2064 | 9.5% | 3.94% |
| other threads/misc | 103 | 0.5% | 0.20% |
| chunk system | 101 | 0.5% | 0.19% |
| tickChildren (прочие уровни/ветки) | 70 | 0.3% | 0.13% |
| network | 57 | 0.3% | 0.11% |
| main-tick misc (server) | 10 | 0.0% | 0.02% |

### 5b. main-tick misc (10 = 0.0% CPU): топ-12 якорей и топ-12 листьев

| якорь (net/minecraft кадры, stack presence) | samples |
|---|---|
| `net/minecraft/world/entity/LivingEntity.isAlive` | 4 |
| `net/minecraft/world/entity/LivingEntity.getHealth` | 4 |
| `net/minecraft/network/syncher/SynchedEntityData.get` | 3 |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | 3 |
| `net/minecraft/server/MinecraftServer.buildServerStatus` | 1 |
| `net/minecraft/network/protocol/status/ServerStatus$Version.current` | 1 |

| лист | samples |
|---|---|
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | 3 |
| `org/bukkit/craftbukkit/entity/CraftEntity.isValid` | 3 |
| `net/minecraft/network/protocol/status/ServerStatus$Version.current` | 1 |
| `org/bukkit/craftbukkit/CraftWorld.getTileEntityCount` | 1 |
| `I2C/C2I adapters` | 1 |
| `net/minecraft/world/entity/LivingEntity.getHealth` | 1 |

