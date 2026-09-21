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
