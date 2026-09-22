# RESEARCH-A-419 — COLPUSH: подсистема collide+push целиком Java→Rust (TASK-419-A)

Автор: agent-a, ROUND-419. Ветка round-419-a-colpush @ fbb06e3 (master = cvs⊕queryplane +36.2%).

## 1. Ботлнек (BOTTLENECK-419, тик-417 абсорб)

- collide+push family: **collide ~8.7% + MobPushOps.push/move ~7-9% (java)** = ~16-18% java-семплов.
- На master под cmp417_bq push-путь = per-entity лестница: каждый living entity В КАЖДОМ aiStep:
  1. `upsertSelf` → **per-entity JNI mobUpsert** (eqsnap: shard-append, но JNI-переход остаётся) ~40-48k/тик;
  2. `EntityGoalQueryOps.pushCandidates` → chain-скан снапшота (per-query JNI-проход, java-фильтрация);
  3. `collect`: точные ванильные фильтры + ArrayList/RING/predicate-аллокации на КАЖДЫЙ запрос;
  4. ванильный хвост: cramming + numCollisions + doPush.

## 2. javap-контракт ванильного pushEntities (kernel round-396-a, точный)

```
protected void pushEntities():           # LivingEntity, вызывается из aiStep @850 ПОСЛЕ travel/move
  if (!isPushable()) return;
  team = getTeam(); if (team != null && team.getCollisionRule() == NEVER) return;
  cramming = level.getGameRules().getInt(RULE_MAX_ENTITY_CRAMMING);
  if (cramming <= 0 && paperConfig.collisions.maxEntityCollisions <= 0) return;
  list = level.getPushableEntities(this, this.getBoundingBox());   # ← retarget-сайт round-401
  if (list.isEmpty()) return;                                      # ← декремента НЕТ
  if (level instanceof ServerLevel sl && cramming > 0 && list.size() > cramming-1
      && this.random.nextInt(4) == 0) {                            # ← RNG = per-entity RandomSource
      nonPass = count(!e.isPassenger()); if (nonPass > cramming-1) hurtServer(sl, cramming, 6.0f);
  }
  this.numCollisions = max(0, this.numCollisions - maxEntityCollisions);   # кредит/тик
  for (e : list) {
      if (this.numCollisions >= maxEntityCollisions) break;        # cap-выход
      e.numCollisions++; this.numCollisions++;                     # оба счётчика
      doPush(e);                                                   # = e.push(this)
  }
```

`Entity.push(Entity other)` (this = кандидату-получатель, other = пушер):
- гейты: isPassengerOfSameVehicle / noPhysics×2 / onlyPlayersCollide;
- dx = other.x − this.x; dz = other.z − this.z; d = Mth.absMax; if (d < 0.01) return;
- d = sqrt(d); dx/=d; dz/=d; f = min(1/d, 1.0); dx *= f*0.05; dz *= f*0.05; (0.05 = 0.05000000074505806)
- если !this.isVehicle() && this.isPushable() → this.push(−dx, 0, −dz);   # кандидат ОТ пушера
- если !other.isVehicle() && other.isPushable() → other.push(+dx, 0, +dz); # пушер ОТ кандидата
- push(DDD) → push(DDDD, null): **source==null → Bukkit-событие НЕ вызывается** (дешёвый путь), deltaMovement += v, hasImpulse = true.
- Математика пуша зависит ТОЛЬКО от позиций ⇒ дельту можно считать по снапшоту, а применять в живом java-состоянии бит-в-байт.

Кандидат-предикат `EntitySelector.pushableBy(self)` = lambda$pushable$10:
`candidate.isCollidable(false) && candidate.canCollideWithBukkit(self) && self.canCollideWithBukkit(candidate) && …`
`Entity.isCollidable(false)` = **false**; `LivingEntity.isPushable()` = isCollidable(fixClimbingBypassingCrammingRule) = isAlive && !isSpectator && …
⇒ вселенная кандидатов пуша = **ЖИВЫЕ LivingEntity ровно** (items/frames НЕ кандидаты — RECON-39 подтверждает: item-сущности скан не делают). Это в точности SoA-вселенная round-401.

## 3. Архитектура COLPUSH-плоскости (ОДИН bulk-JNI/тик)

**Инвариант честности**: вся математика пуша/cramming/numCollisions остаётся ВАНИЛЬНОЙ в java
(живые поля, живой RNG, живые события hasImpulse, бит-точное округление per-pair add).
Rust = bulk-broadphase: один проход по всей популяции → CSR списки кандидатов.

Поток данных:
- **Входной буфер** (мандат: позиции/AABB/velocity, пополненный из mobs_soa-снапшота): персистентные
  java-массивы COL_D double[idTop×6] (cx,cy,cz,hx,hz,hh — ТОЧНЫЕ полуэкстенты по осям, не max-superset)
  + COL_I int[idTop×3] (lid, flags, freshTick). Пишутся per-entity прямо в aiStep (простые store,
  0 JNI, 0 аллокаций), id = плотный id mobs_soa (MobPushOps.idBoxOf/ensureBox).
- **ОДИН bulk-JNI/тик** `colpushTick(tick, idTop, COL_D, COL_I, OFF, IDS)`: триггерится main-потоком
  в RegionTickOps.forEach ДО GO-барьера фазы воркеров (0 гонок/конвоя; lazy tryLock-фоллбек без блокировок).
  Rust: актив = fresh==tick−1 && flags.pushable && конечные координаты → грид (ячейка 4.0, pad 1,
  reach ≤ hx_i+hx_j ≤ 4.0) → точный AABB-overlap (строгие <, эквивалент AABB.intersects, y = |Δcy| < hh_i+hh_j)
  → CSR кандидатов по возрастанию id → запись в java-массивы. Плюс ОДИН WLOCK-рефреш колонок
  mobs_soa (sscan/ai-плоскости читают свежие x/y/z/hw/hh — питание sscan сохранено БЕЗ per-entity JNI).
- **Выходной буфер**: OFF int[idTop+1] + IDS int[cap] (CSR). Java apply:
  ```
  n = OFF[id+1] − OFF[id]; if (n == 0) return;                 # ваниль: пустой список = ничего
  cramming-ветка бит-в-байт (RNG self.random.nextInt(4), nonPass по моим кандидатам, hurtServer)
  numCollisions = max(0, nc − maxCol)                          # живое поле
  for cid in IDS[off..off+n]:
      e = byId[cid]; live-ревалидация: e != null, e != self, e.level()==level,
      e.isPushable(), e.getBoundingBox().intersects(bb), (team != null → EntitySelector.pushableBy)
      if (self.numCollisions >= maxCol) break;                 # ванильный cap
      e.numCollisions++; self.numCollisions++;
      e.push(self);                                            # ванильный doPush: живые позиции/округление
  ```

**Дельты (все задокументированы, под флагом, невидимы игроку — класс ghost-контракта eqsnap)**:
1. кандидат-множество из end-of-previous-tick позиций (≤1 тик ghost для уже тикнувших соседей) —
   ТОТ ЖЕ контракт, что eqsnap-плоскость master;
2. порядок кандидатов = возрастание плотного id (вместо live chain-порядка) — документированная
   дельта items_subsys2/soa;
3. passengers/vehicles/teamed-сущности исключены из плоскости (fail-closed per-entity ваниль для
   себя; в bench-популяции ≈ 0);
4. cramming-count по снапшоту (граничные ghost-случаи ±1 у порога RNG-branch);
5. velocity НЕ нужен для математики пуша (только позиции) ⇒ вход не носит velocity, дельта
   применяется в живое deltaMovement ванильным Entity.push.

**Parity**: пустой флаг → бридж не определён, хук спит, LivingEntity бит-в-байт ванильный (закон 4).

## 4. Гейт-архитектура (STRICT-OR)

- Раст-модуль `colpush`: lever_flag_matches STRICT eq `cmp419_colpush` (СТРОГО только мой флаг —
  прежние флаги бит-в-байт прежнее поведение).
- Все носитель-мосты master (mobs_soa/mobs_manager/mobs_ai/mobs_sscan/mobs_grid/collide_batch/
  nav_plane/nav_pool/entity_query/area_map/batch_collector/item planes/… + java leverEnabled)
  получают STRICT-OR добавку `|| v == "cmp419_colpush"` рядом с cmp417_bq (носитель эры).
- Живые ревалидации в apply = fail-closed: любое расхождение флагов → ванильный кандидат/сущность.

## 5. Источники

- javap дизасемблирование реального kernel round-396-a (pushEntities/Entity.push/pushableBy/isPushable — точные контракты выше);
- src/mobs_soa.rs + EntityGoalQueryOps.java (eqsnap-протокол, seqlock/WLOCK, shard-drain);
- collide_batch.rs + classfile.rs::redirect_method_body_to_static (whole-body static redirect, приёмник в начале);
- EntityGoalQueryOps.maybeEpoch (double-checked volatile публикация эпохи — образец для BULK_TICK);
- RegionTickOps.forEach (фаза-1 main-потока до GO-барьера — точка тика до тикнинга сущностей);
- интернет-контекст: batch broad-phase (uniform grid + chained cells — стандарт DOD-практик,
  Fabian "Data-Oriented Design" ch. SoA; Unity DOTS/Bevy ECS columnar storage — те же источники,
  что в доке mobs_soa), pair-by-runner ночная дисциплина эры.
