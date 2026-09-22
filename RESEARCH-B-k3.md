# RESEARCH-B-k3 (TASK-411-B) — посимвольный профиль inside-плоскости + таргет моста

**Дата:** tick-411, 12:08+08 · **Агент:** agent-b (новый хозяин `inside_volatile` после пивота TASK-410-B)
**Источники:** `research/gc-recon-2026-09-19/round-round410anchora/cpu-collapsed.txt` (115 385 сэмплов, ваниль-якорь),
`round-round409multi2/cpu-collapsed.txt` (110 274 сэмпла, multi-сцена). Скрипты: `lane_profile.py`, `lane_profile2.py`
(точные фреймы, не подстроки — `lambda$checkInsideBlocks$2` содержит имя лейна и ломает наивную атрибуцию).

## 1. Размер и состав лейна (регекс checkInsideBlocks|collidedWithShapeMovingFrom)

| сцена | лейн | checkInsideBlocks | collidedWithShapeMovingFrom |
|---|---|---|---|
| round410anchora (бокс X150K) | **11.76%** | 13 573 = 11.76% | 789 = 0.68% |
| round409multi2 (multi) | **12.77%** | 14 077 = 12.77% | 713 = 0.65% |

**ВЕРДИКТ по регексу:** `collidedWithShapeMovingFrom` = 0.65–0.68% total, и 100% его вызовов — из
`Entity.collidedWithFluid` внутри visit-лямбды checkInsideBlocks (0 сайтов вне лейна). Это НЕ
collide-поддерево broadphase (CollisionUtil 37% занято другим агентом — не дублируем). **Таргет строго
inside: `Entity.checkInsideBlocks`** — обе сцены согласованы (±1pp).

## 2. Кто зовёт лейн (доля лейна)

| entity-caller | round410anchora | round409multi2 |
|---|---|---|
| `ItemEntity.tick` / `ItemEntityManager.tickBody` | **60.27%** | **60.94%** |
| `LivingEntity.aiStep` (зомби/пауки/скелеты/криперы/овцы…) | 38.63% | 37.65% |
| boats/minecarts/arrows | ~1.1% | ~1.4% |

Цепочка (javap booted jar): `tick/aiStep → applyEffectsFromBlocks() → (Vec3,Vec3) → (List<Movement>) →
checkInsideBlocks(List,Collector)V → ×3 invokestatic-кандидата checkInsideBlocks(Vec3,Vec3,C,LongSet,I)I
(байткод-смещения 171/204/229 — ЕДИНСТВЕННЫЕ 3 сайта перегрузки в кернеле, javap-цензус) →
BlockGetter.forEachBlockIntersectedBetween → lambda$checkInsideBlocks$2 → листья.

## 3. Посимвольнаяheatmap тела visit-лямбды (доля лейна)

| лист | round410a | multi2 | смысл |
|---|---|---|---|
| forEachBlockIntersectedBetween (self+cursor) | 30.6% | 27.4% | обход-машины (guava-итераторы, betweenCorners) |
| **Entity.collidedWithFluid** | 14.8% | 14.0% | fluid-подветка visit (внутри неё B 0.68%) |
| InsideBlockOps.gate (inside_cache=1 банк) | 13.4% | 12.8% | существующий java-мемо-гейт |
| Level.getBlockState | 9.0% | 8.2% | per-position чтение |
| BatchCollector.advanceStep | 7.1% | 9.6% | коллектор эффектов (bank batch_collector=1) |
| LongOpenHashSet.add | 5.1% | 4.9% | visitedBlocks |
| AtomicInteger.set | 2.4% | 0.5% | step-бюджет |
| getEntityInsideCollisionShape | 1.3% | 1.4% | non-air ветка |
| ServerDebugSubscribers.hasAnySubscriberFor | — | 1.4% | air-ветка debug-записи |
| entityInside (freeze/honey/fire) | ~0.7% | ~0.7% | эффекты (парити — не трогаем) |

Машина обхода (traversal+set+budget+collector+getState+debug) ≈ **55–57% лейна**, fluid-подветка ≈ 15%,
эффекты-парити ≈ 2–3%. **Всё это стоит ЗА ОДНИМ условием: «позиция не-air».**

### 3b. Кросс-валидация на round-round410multi3 (110 026 сэмплов, R2-ревизия)

Лейн = **14 157 = 12.87%** (mandate-число подтверждено), item-доля 61.8%,
`collidedWithShapeMovingFrom` = 697 = **0.63%** total (100% из collidedWithFluid внутри visit —
не collide-поддерево). Три сцены согласованы: 11.76 / 12.77 / 12.87. Доминанта таргета = `Entity.checkInsideBlocks`.

## 4b. javap-контракт R2-ревизии (байткод round-396-a, Entity_javap.txt в worktree)

1. **List-тело** `checkInsideBlocks(List,StepBasedCollector)V`: gate isAffectedByBlocks bc 1–4 →
   `budget = 16` ЛОКАЛЬНЫЙ int (bc 64–66, НЕ shared-atomic) → per-Movement: axis-split (site#1 bc 171,
   только d≠0-сегменты) / прямой (site#2 bc 204) → `budget -= consumed` → `budget <= 0` → fallback
   site#3 bc 229 `(to,to,collector,visited,1)` с ПРОПУЩЕННЫМ результатом → `visited.clear()` bc 236.
   Три сайта — единственные вызовы overload'а в кернеле (opcodes **invokevirtual**, готов к
   receiver-prepended retarget_virtual_to_static, STRICT sites==3).
2. **int-overload** (bc 4409–4471): `box = makeBoundingBox(to).deflate(9.999999747378752E-6)`
   (= (double)(float)1.0E-5F) → `flagMoving = from.distanceToSqr(to) > Mth.square(0.9999900000002526)`
   → debugActive (ServerLevel→getServer().debugSubscribers().hasAnySubscriberFor(ENTITY_BLOCK_INTERSECTIONS))
   → `AtomicInteger steps = new` (bc 80, per-call alloc) → FBIB(from,to,box,visitor) → return
   **`steps.get()+1` БЕЗУСЛОВНО (bc 118–125)**.
3. **FBIB still-ветка**: `to.subtract(from).lengthSqr() < (double)Mth.square(1.0E-5F)` →
   `BlockPos.betweenClosed(box)` = cells `[floor(min)..floor(max)]` ВКЛЮЧИТЕЛЬНО (containing=min-floor),
   **visit step≡0 для ВСЕХ клеток still-запроса** ⇒ `steps.set(0)` на каждой ⇒ vanilla return still-запроса
   = **1 всегда** (поправка к §4 пин 2: atomic хранит ИНДЕКС последнего visit-attempt'а, а у still все
   attempt'ы = step 0). Fast-plane `return 1` = бит-в-бит ванильному возврату; бюджет-арифметика List-тела
   видит consumed=1 как ваниль.
4. **Visit-лямбда** (lambda$checkInsideBlocks$2, bc 15062–15228): isAlive → `step >= budget → false` →
   `steps.set(step)` (ДО air-проверки — но для still step≡0, см. п.3) → getBlockState → **air → true
   (только debug-запись при подписчиках)** → non-air: getEntityInsideCollisionShape →
   `inBlock = shape==Shapes.block() || collidedWithShapeMovingFrom(from,to,shape.move(new Vec3(pos)).toAabbs())`
   → `inFluid = collidedWithFluid(getFluidState,pos,from,to)` → `(!inBlock && !inFluid) → true` →
   `visited.add(asLong) false → true` → inBlock-ветка В try/catch(→CrashReport «Colliding entity with block»):
   `stuck = flagMoving || box.intersects(pos)`, advanceStep(step,pos), entityInside(level,pos,this,collector,stuck),
   onInsideBlock → вне try: inFluid-ветка advanceStep+FluidState.entityInside → debug-запись → true.
5. **Доступность для реплики** (javap): collidedWithFluid/collidedWithShapeMovingFrom public;
   makeBoundingBox/onInsideBlock/fillCrashReportCategory protected (same-package мост OK);
   StepBasedCollector.advanceStep public; debugBlockIntersection **private** → инлайн-реплика тела через
   public ServerLevel.debugSynchronizers().sendBlockValue + DebugSubscriptions.ENTITY_BLOCK_INTERSECTIONS +
   DebugEntityBlockIntersection (IN_FLUID/IN_BLOCK/IN_AIR, fluid-приоритет); BlockPos.betweenClosed(AABB)
   = containing(min)..containing(max) — секционный хулл `[floor(min)>>4..floor(max)>>4]` (×3 оси).
6. **Мmoving-flag** (flagMoving) влияет ТОЛЬКО на `stuck` внутри inBlock-ветки (bc 170–190) — на
   all-air hull недостижим ⇒ fast-plane не требует его репликации.

## 4. Ключевые пины семантики (первая ревизия, до полной javap-диспозиции)

1. **Still-ветка FBIB:** `delta.lengthSqr() < (double)Mth.square(1.0E-5F)` → `BlockPos.betweenClosed(bb)` —
   посещения все с step=0; visit-лямбда на air возвращает true ДО любых наблюдаемых действий
   (visitedBlocks не пишется — RECON-33 пин 1-2 подтверждён байткодом; УТОЧНЕНИЕ в §4b п.3:
   steps.set(0) на air-visit'ах ПРОИСХОДИТ, но return всё равно 1).
2. **Возврат per-movement метода = `steps.get()+1` БЕЗУСЛОВНО** (bc 118–125). Для still-запроса
   все visit-attempt'ы имеют step≡0 (FBIB still-ветка) ⇒ consumed = **1 всегда** (и для мёртвого
   entity: isAlive-аборт на step 0 → 0+1=1). Это делает still-путь **арифметически предсказуемым без обхода**
   (полный контракт — §4b).
3. List-тело: `stepBudget -= consumed; if (stepBudget > 0) next; else fallback checkInsideBlocks(to,to,1)`
   — fast-path, возвращающий ровно 1, сохраняет бюджет-арифметику бит-в-бит.
4. `collidedWithFluid(FluidState,BlockPos,Vec3,Vec3)`, `collidedWithShapeMovingFrom(Vec3,Vec3,List)`,
   `makeBoundingBox(Vec3)` (protected), `onInsideBlock` (protected) — доступны классу-мосту в пакете
   `net.minecraft.world.entity`; `BlockGetter.forEachBlockIntersectedBetween` public static — реплика тела
   возможна без приватных хуков.
5. Носитель истины «секция пустая»: `LevelChunkSection.hasOnlyAir()` (nonEmptyBlockCount) — прецедент
   `LevelChunk.getFluidState` fast-path (RECON-33 пин 3); fluid-блоки — non-air ⇒ водные секции честно
   дают negative verdict (эффекты жидкости не теряются).

## 5. Дизайн-решение (меню §мандата → финал)

**inside_volatile → Rust батч-вердикты секциями, negative fast-plane:**

- **RETARGET:** ровно 3 сайта `invokevirtual checkInsideBlocks(Vec3,Vec3,C,LongSet,I)I` в
  `checkInsideBlocks(List,...)V` → `invokestatic InsideRustOps.checkInside(Entity,Vec3,Vec3,C,LongSet,I)I`
  (receiver-prepended, 3B→3B length-preserving; STRICT ровно-3, иначе fail-dominant без стадии).
- **FAST-PLANE (только still-запросы, только !debugActive):** thread-confined вердикт-таблица
  {entity-ref, from×3, to×3, dims-bits} → вердикт. HIT + verdict=0 (all-air) → return 1, нулевой JNI,
  нулевой обход, нулевой alloc (джавa-цена ~25ns против ~350ns ванили).
- **GATHER (zero JNI):** miss still-запрос → ваниль-реплика тела + append record {entity, from, to,
  deflated bb(to), dedup-секции хулла ≤64} в thread-очередь. Движущиеся запросы НЕ собираются
  (урок bl2: gather дороже экономии, когда вердикт не повторяется).
- **FLUSH (ОДИН bulk-JNI на фазу на поток):** первый вызов с новым gameTime → `inside_batch_tick(n,
  metaI, metaD, secIdx, flags, out)`; java резолвит ДИСТИHKT секции хуллов всех записей
  (`getChunk→getSection().hasOnlyAir()`, unknown=2), Rust перечитывает metaD-геометрию (строгая
  структурная проверка floors/counts = ERR_STRUCT → перманентный disarm), комбинирует флаги секций
  **бит-в-байт в вердикт-массив** (out[i]=0 ⟺ все секции хулла пустые). Стамп gameTime, single-phase TTL,
  valve 8192 — как fluid_rust v2.
- **FAIL-CLOSED:** гейт `CRUSSTY_LEVER_FLAG == "cmp411_insidebat"` STRICT eq (пустой = ваниль бит-в-байт)
  с ДВУХ сторон (java-класс + rust-стадия); probe native; любое исключение/ошибка натива → disarm в
  точную ваниль-реплику навсегда (никогда не half-applied); unknown-секция/переполнение ячеек →
  verdict=1 → ваниль.
- **SUPERSEDE-аудит (урок eb7a870):** сайты ретаргета — только 3 внутри checkInsideBlocks(List);
  fluid_guard (fluid-wrapper сайты), inside_cache (gate-инъекция вне List-тела), flush_diet, batch_collector
  (полиморфный advanceStep — реплика зовёт тот же метод) — байт-сайтами не пересекаются; реплика зовёт
  публичные collidedWithFluid/collidedWithShapeMovingFrom — те же точки, что ваниль ⇒ guard-патчи тел
  ведут себя идентично. inside_cache-гейт на fast-hits superseded моим вердиктом (оба — мемо стационарных),
  на slow-path честная ваниль-дискавери; конфаунд в пользу ноги (не регресс).
- **Staleness-дисклеймер:** вердикт живёт ровно одну фазу и валидируется полным фингерпринтом запроса;
  гипотетическая правка блока между flush и использованием в той же фазе ⇒ пропуск эффектов ≤1 тик для
  still-сущностей (на бенч-сцене правок блоков нет; в проде — ставка на valve+TTL, документировано).

## 6. Потолок

Лейн 11.76%: хит-популяция (стационарные items ≈60% лейна + idle-мобы) убирает ~85% своего лейн-хвоста
(всё, кроме эффектов-парити ~3%); мобы-движущиеся (~38%) остаются ванилью. Ожидаемый лейн
**11.8% → ~5.5–6.5%** при цене flush ≈0 в стационаре (собираются только miss'ы первого тика).
Верхняя оценка TPS-конверсии умеренная (RECON-32 эмпирика диет ~0, но это не диета — это удаление
CPU из entity-фазы, где 82.9% фазовой доли). Чек-лист вердикта: ARM-маркер + EFFECT-маркер (первый
fast-hit в server-stdout) + grep AIOOBE=0 + **inside-лейн был→стал (РОСТ = RED)**.
