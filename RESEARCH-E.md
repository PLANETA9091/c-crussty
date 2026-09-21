# RESEARCH-E — TASK-406-E (vector R4 sscan, lever cmp406_sscan)

## Ground truth (javap, patched-kernel.jar round-round405anchorc, moonrise 289 entries)

1. **ДесAWN-СКАН САЙТ (единственный)**: `net/minecraft/world/entity/Mob.checkDespawn()V`
   содержит РОВНО ОДИН `invokevirtual net/minecraft/world/level/Level.findNearbyPlayer:
   (Lnet/minecraft/world/entity/Entity;DLjava/util/function/Predicate;)Lnet/minecraft/world/entity/player/Player;`
   @ offset 55 (grep mob bytecode: 1 совпадение на весь Mob.class).
   Вызов идёт с `distance = -1.0`, предикат `EntitySelector.PLAYER_AFFECTS_SPAWNING`.
2. **Ванильная семантика скана** (`EntityGetter.getNearestPlayer(DDDD,Predicate)` default-метод,
   javap): итерация `level.players()` ПО ПОРЯДКУ; на каждого игрока, прошедшего предикат:
   `d = player.distanceToSqr(mobX, mobY, mobZ)`; селекция `(best == -1.0 || d < best)` —
   ПЕРВЫЙ строго-ближайший выигрывает, при равенстве остаётся более ранний.
   `Entity.distanceToSqr(DDD)`: `dx = getX() - x; dy = getY() - y; dz = getZ() - z;
   return dx*dx + dy*dy + dz*dz` (this = player, аргументы = координаты моба).
   Фильтр по distance НЕ применяется (distance < 0).
3. **Тело checkDespawn после скана** (Paper "despawn ranges"): hard/soft диапазоны
   `paperConfig.entities.spawning.despawnRanges[category].hard()/soft().shouldDespawn(shape, dx2, dy2, dz2, |dy|)`,
   `removeWhenFarAway(d)`, `noActionTime > 600 && random.nextInt(800) == 0`, `discard(cause)`.
   ВСЁ это остаётся нетронутым ванильным байткодом — мост возвращает ТОГО ЖЕ игрока,
   которого вернул бы ванильный скан, тело читает координаты игрока виртуально.

## Дизайн (R4 RUST-FIRST, один bulk-JNI на тик)

- **Retarget**: сайт из (1) → `MobScanOps.findNearbyPlayerGate(Level,Entity,D,Predicate) → Player`
  (desc = receiver-класс Level, препендированный к virtual desc — contract
  `retarget_virtual_to_static`; хук на Mob.class, метод-скоуп checkDespawn()V).
- **Батч**: `sscanEpoch(tick, idTop, players[D], nearest[I]) → rc` — DOD-проход по SoA
  позициям mobs_soa (f64 x/y/z, seqlock-бракет even v1 → scan → even v2, QRETRY 128)
  пишет колонку `nearest[denseId] = индекс ближайшего qualifying-игрока | -1` в общий
  java int[] (GetPrimitiveArrayCritical). Пер-моб решение = O(1) чтение колонки
  (MobPushOps.idBoxOf → плотный id). Ноль per-entity JNI (закон 6).
- **Ванильность бит-в-байт**: игрок = тот же объект; порядок players() сохранён; при
  равенстве d остаётся более ранний игрок; порядок компонент и суммирование f64 1:1;
  предикат применяется 1× к игроку на снапшот (зависит только от игрока); пустой
  qualifying-набор → null (= ванильный null); hard/soft-решения считаются нетронутым
  ванильным байткодом. Погрешность: игрок, сместившийся МЕЖДУ снапшотом эпохи и
  поздним тиком моба в пределах одного тика (в бенче fake_players статичны → ноль).
- **Fail-closed лестница**: не-Mob / не в плоскости (idBoxOf null) / id вне эпохи →
  ваниль `level.findNearbyPlayer(...)` на вызов; sscanProbe mismatch / sscanEpoch
  ERR_STRUCT → дизарм навсегда; ERR_RANGE → ваниль тик, ретрай следующего.
- **Активация**: ждёт boot-маркер, guard class-major блоба (урок 408), define
  MobScanOps в kernel loader + RegisterNatives (sscanProbe 0x5353 / sscanEpoch),
  ARM-маркер в stdout, retransform Mob.class. Mob.class свободен от других хуков
  под этим флагом (prepare_index моб-хук спит вне cmp401_offthread) — stash-serve безопасен.

## Гейт-чеклист (mandate: все сайты расширяют cmp405_stagtick || cmp406_sscan)

rust: collide_batch.rs ×2, items_index.rs, items_manager.rs ×2, mobs_grid.rs,
mobs_manager.rs ×2 (GATE_LEVER_SSCAN + java_gate_matches + arm-marker),
mobs_soa.rs (lever_mode + sscan_snapshot), stagger.rs, tickplane.rs,
mobs_sscan.rs (STRICT eq cmp406_sscan only) — 35 вхождений cmp406_sscan по java+rust.
java: MobPushOps.java ×2 (leverEnabled + compositeEnabled), ItemEntityManager.java ×3
(ITEMS_ARMED + REST_PLANE + DESPAWN2), MobScanOps.java (новый STRICT eq only).
Классы ПЕРЕСОБРАНЫ из merged-исходников: scripts/build_sscan_ops.sh (--release 21,
cp = patched-kernel.jar + fastutil + adventure-api + adventure-key + paper-api),
блобы закоммичены (урок 408: устаревший блоб = спящий гейт).

## Билд

- cargo check --lib PASS 4.4s (61 warning — прe-существующие; cargo test --lib
  не компилируется на этом worktree из-за отсутствующего bench/p500/java/p500/
  groups.tsv fixture — PRE-EXISTING, не от ног E).
- javac OK: MobPushOps.class 10359B + MobScanOps.class 5512B.

## Прыжок веры → пул pair

Банк-якоря pair-пул: {2.35@6992836, 2.10@6851475, 2.70@8841704, 2.10@6845027}.
Лейн: despawn-скан = 150k мобов × O(players) java-итерации/тик → Rust DOD
(~6M f64-оп/тик) + устранение 600k предикат-вызовов/тик. Ожидаемый потолок
умеренный (скан ~2-4% wall cpu-collapsed comp-сцены) — цель pair ≥ 80% от пула.

## K2-RESEARCH (TASK-409-E tick, веб ≥3 пруфа + javap ground truth нашего kernel-jar) — расширение окна sscanEpoch

### Ground truth из round-round406eleg1/patched-kernel.jar (наш leg1 kernel, purpur-1.21.10) + entity-recon.txt
1. **DESPAWN-СКАН (закрыт leg1)**: `Mob.checkDespawn` → 1 сайт `Level.findNearbyPlayer(Entity,D,Predicate)`
   (javap @55, RESEARCH-E) — O(мобы×игроки)/тик, 150k мобов bench-сцены.
2. **ACTIVATION-СКАН (новый таргет k2)**: javap `io.papermc.paper.entity.activation.ActivationRange`
   из kernel-jar:
   - `ServerLevel.tickNonPassenger` @72: `ActivationRange.checkIfActive(Entity)` на КАЖДОГО
     энтити КАЖДЫЙ тик — НО это лишь чтение `Entity.activatedTick >= currentTick` (скана игроков нет).
   - ВЕСЬ player-скан концентрирован в `ActivationRange.activateEntities(Level)` (вызов из
     ServerLevel @517 recon): на КАЖДОГО игрока за тик: 7 inflated AABB (по ActivationType:
     misc/raider/animal/monster/water/flying-monster/villager) + **ОДИН broadphase-запрос
     `Level.getEntities(null, maxBB, predicate)`** (maxBB = union) + `activateEntity(e)` на каждый
     хит (= `e.activatedTick = currentTick`, идемпотентный сет, порядок НЕ важен).
   - Следствие k2: retarget единственного сайта `getEntities(null,maxBB,pred)` внутри
     activateEntities на java-мост, читающий из той же epoch-снапшот-плоскости rust-колонку
     `activation_hits[I]` (SoA box-overlap entity-BB ⊆ players' maxBB в ТОМ ЖЕ bulk-JNI проходе) →
     java-итерация ТОЛЬКО по хитам с нетронутым ванильным `activateEntity` (все java-состояния
     immunities/wakeup/afk/spectator/markers сохраняются бит-в-байт — решение activateEntity
     идемпотентно, порядок обхода не наблюдаем).
3. **SPAWN-СКАН (деприоритизирован)**: natural-spawn `getNearestPlayer` проверки не входят в топ-лейны
   BOTTLENECK-409 (fluid 16.9-18.5 / broadphase 14.3 / inside 13.3 / nav_ai 9.6-10.1 / JNI wall 12-17);
   фиксед в design-note, не в k2-коде.

### Веб-пруфы (≥3, industry precedent)
- **Paper/Spigot EAR** (docs.papermc.io spigot.yml, патч 0014-Entity-Activation-Range.patch github):
  entities вне activation-range тикают на 1/4 rate (изначально 5%) — само существование плоскости
  «решение по дистанции до игрока» = отраслевой стандарт.
- **Airplane DEAR** (blog.airplane.gg/dear-configuration): «limits how often a mob decides to do
  something based on how far away they are from a player»; per-entity freq = dist²/2^mod, 1/1@22blk
  … 1/20@101blk — подтверждает кэшируемость/огрубляемость per-entity player-дистанции без ломания
  механик (entities всё ещё двигаются/действуют 100% времени).
- **Pufferfish DAB** (docs.pufferfish.host): градиентное тикирование brain/goals по дистанции
  (villager/axolotl brains limited) — тот же принцип «меньше решений на далёких мобах».
- ВАЖНО для ванильности: наши ноги НЕ меняют частоты решений (это был бы геймплейный сдвиг
  = запрет на изменение значений). k2 устраняет ТОЛЬКО дублирующий поиск (broadphase-запрос/игрок/тик
  и O(P) итерации/моб), сохраняя бит-в-байт решения тика — категория «сколько работы», не «что решается».

### K2-дизайн (ОДИН bulk-JNI/батч — закон 6)
- sscanEpoch расширяем выход: `nearest[I]` (leg1) + `act_hits` (пер-игрок список denseId внутри
  maxBB-игрока; box-overlap по entity.getBoundingBox ⊣ inflate типов — union maxBB как в ванили;
  полуинтервалы идентичны `AABB.intersects`).
- Java-мост `MobScanOps.activateEntitiesBridge(Level)` ретаргетит call-site
  `ActivationRange.activateEntities` (invokestatic, единственный @ServerLevel @517): при STRICT-флаге
  и живом снапшоте — итерация хитов + ванильный `ActivationRange.activateEntity(e)`; при любом
  промахе (эпоха/структура/magic) — ванильный `getEntities`-путь на этот тик.
- Экономика: 4 игрока × getEntities(maxBB≈250blk) broadphase-запрос/тик уходит из ChunkEntitySlices
  (broadphase-лейн 14.3% BOTTLENECK-409) → DOD-проход по уже резидентным SoA-позициям.
- Классы: MobScanOps.java расширяем (новый мост + ретаргет-контракт), javac rebuild + commit
  ВСЕГДА (урок 408). ItemEntityManager.class блоб (гейты ×3 из 63bf3b3) — rebuild В k2-коммит.
