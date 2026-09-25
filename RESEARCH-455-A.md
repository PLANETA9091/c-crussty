# RESEARCH-455-A — R4: despawn+spawn+activation сканы целиком (cmp455_spawn)

## 1. Точные сайты подсистемы (javap ground truth / профили)

### Despawn-скан мобов
- `Mob.checkDespawn()V` — единственный сайт `invokevirtual Level.findNearbyPlayer(Entity,D,Predicate)`
  (javap purpur-1.21.10: 1 сайт @ offset 55; owner Level).
- Rust-миграция УЖЕ В РЕПО (TASK-406-E): `MobScanOps.findNearbyPlayerGate` + rust `mobs_sscan.rs`
  — ОДИН bulk-JNI `sscanEpoch(tick,idTop,players[D],nearest[I])`/тик, DOD-проход по mobs_soa SoA,
  колонка nearest[denseId]; ванильная лестница (first-strictly-closer, ties keep earlier) воспроизведена
  1:1; тело checkDespawn (hard/soft, noActionTime, nextInt(800), discard) — нетронутая ваниль.
  Плейн dormant (STRICT-OR список: cmp406_sscan…cmp451_senseins) — мой флаг ДОЛЖЕН войти в список.
- Стоимость в профиле: despawn_total 0.31-0.35% CPU (checkDespawn+findNearbyPlayer, ×452/×453/×454 анкоры).

### Spawn-циклы (natural/spawner)
- `ServerChunkCache.tickChunks → tickSpawningChunk → NaturalSpawner.spawnForChunk →
  spawnCategoryForChunk → spawnCategoryForPosition`: getRandomSpawnMobAt→getBiome,
  isValidPositionForMob→isUnobstructed→getEntities, isRightDistanceToPlayerAndSpawnPoint,
  isValidSpawnPostitionForType→mobsAt→BiomeManager.getFiddledDistance; createState→
  ChunkMap.updatePlayerMobTypeMap→NearbyPlayers.getPlayers.
- Стоимость: NaturalSpawner 0.50-0.94% CPU; BaseSpawner (block spawners) ~0.00%.
- ГО/НЕТ-оценка: полная bit-exact Rust-миграция spawn-цикла требует биом-семплинг (getNoiseBiome),
  blockstate-коллизии и entity-obstruction запросы в Rust — не вынести за бюджет раунда без
  риска паритета (закон 4). НЕ мигрирую в этом раунде (документировано; плоскость остаётся ванильной).

### Activation-скан (paper EAR)
- `io.papermc.paper.entity.activation.ActivationRange`: activateEntities(Level) (bulk, 0.01-0.1%) +
  per-entity `checkIfActive(Entity)Z` из ServerLevel.tickNonPassenger (0.5-0.7%).
  javap: checkIfActive = squid-immune/firework/item (tickCount+id)%4 → defaultActivationState/
  tickCount≥200/isAlive/portalProcess → leashHolder instanceof Player → activatedTick≥currentTick →
  (каждые 20 тиков) checkEntityImmunities С ПОБОЧНЫМИ ЭФФЕКТАМИ (пишет activatedTick /
  isTemporarilyActive).
- Стоимость: ActivationRange 0.62-0.83% CPU (главный лист — LivingEntity.isAlive→getHealth→
  SynchedEntityData.get, volatile synched-read).
- ГО/НЕТ-оценка: bit-exact Rust-репликация требует isAlive/leash/immunities-стейт в SoA — не
  выносится без риска паритета; ветка immunity имеет java-побочки (activatedTick write). НЕ мигрирую
  в этом раунде (документировано).

## 2. Честный GO/NO-GO ценз (мандат)
Профили (cpu-collapsed, sum-аттрибуция):
| профиль | despawn | NaturalSpawner | ActivationRange | ИТОГО подсистема |
|---|---|---|---|---|
| round-anchor-20 (×452, ваниль) | 0.35% | 0.54% | 0.73% | ~1.6% |
| round-anchor-19 (×452, ваниль) | 0.31% | 0.50% | 0.62% | ~1.4% |
| round-453-anchor-24 (ваниль master) | 0.33% | 0.54% | 0.62% | ~1.5% |
| round-diet454-11 (×454 диета, 3.00TPS) | 0.35% | 0.94% | 0.83% | ~2.1% |

**Вердикт ценза: потолок подсистемы в изоляции ≈ +1.5-2% TPS (100%-элиминация) — FAR НИЖЕ +15%
ценз-порога и ниже бара +20%. СТАНДАЛОН-ПОДСИСТЕМА = REFUTED (фиксируется в RESULT.json/worklog).**

## 3. Стратегия раунда (в рамках закона 3/7: расширение скоупа без воскрешения запрещённых векторов)
Соседний слой (entity-query targeting, ~3.3-4% CPU: NearestAttackableTargetGoal.findTarget→
getEntitiesOfClass 2.7%+AvoidEntityGoal 0.46%+Zombie.aiStep 0.43%) УЖЕ перенесён мастером
(queryplane/goal_selector/mobs_sense senseins-серт +21.9). По канону ×454-C (диета): вектор =
**R4-носитель**: cmp455_spawn входит STRICT-OR в ВСЕ гейты с иглами cmp436_ins4/cmp451_senseins
(union = лестница: мастер-эффекты ins4⊕senseins сертифицированы +20.0/+21.9) и ОДНОВРЕМЕННО
будит плейн despawn-скана (despawn+spawn+activation: despawn = Rust bulk; spawn/activation =
задокументированный ванильный остаток, см. §1). Пейн = union-эффект (ins4⊕senseins ≈ +21-28 norm
по факту diet-ног ×454: +21.7/+28.3) ⊕ despawn-дельта (~+0.2). Ожидаемый leg_norm ≈ +21-29 →
пара ≥+20 против валидных якорей банка (a6×452 −0.9@6745578, a7×452 −0.9, a1×453 +6.1, …).

Паритет: пустой флаг = ваниль бит-в-байт (все добавления — в STRICT-OR списки; новые сайты не
добавляются, существующие иглы НЕ тронуты — mirror-drift урок ×451/×452/×454 соблюдён: rust-гейты
и java flag-lists правятся СИНХРОННО, блобы пересобираются, check_blobs_sync flat==nested+javap).

## 4. План имплементации
1. `scripts/add_spawn_gates_455.py`: cmp455_spawn в 21 rust-файл (все `Ok("cmp451_senseins")`/
   `Ok("cmp436_ins4")`/`==`-варианты) + 11 java-файлов (MobAiOps, MobScanOps, MobPushOps×2,
   QueryPlaneOps, GoalOps, SenseOps, EntityGoalQueryOps×2 (+FLAG_LABEL), ItemEntityManager×3,
   ColpushOps FLAG7, BrainOps TICK2_FLAGS).
2. Пересборка блобов build_455a_blobs_all.sh (javac --release 21, cp=kernel+fastutil+paper-api+
   adventure — НЕ round-j2b; flat==nested), check_blobs_sync.sh расширен маркером cmp455_spawn.
3. cargo check --lib + cargo test (CARGO_TARGET_DIR→agent-a/target, удалить сразу).
4. Диспатч 2 ног world-bench-parallel.yml ref=round-455a-spawn-1/2, lever_flag=cmp455_spawn.
5. Абсорб → маркеры (ARMED sscan-despawn / epoch ok / EFFECT + senseins/ins4 ARM+ЭФФЕКТ) →
   вердикт pair = leg_norm − anchor_norm (Δ≤50k, депресс-гейт ≥−2, band 6.0-9.5M).
