# RESEARCH-446-EQ — ENTITY-QUERY СЛОЙ: javap-ценз + лейн-декомпозиция остатка (TASK-446-EQ)

> Цикл закона 3, PHASE RESEARCH (ДО кода — мандат-гейт «доказать лейн-доминирование
> ДО имплементации»). База: ветка round-446-eq = носитель origin/round-444-c-sensemega
> @5cfafaba (sense-семья ⊕ ins4d-диета ⊕ chunk4 chunk-send, cmp444_sensemega STRICT-OR).
> Вердикт: **LOW-POTENTIAL — имплементация и ноги НЕ оправданы числами** (см. §6).

## 0. Метод

1. javap -p -c офлайн-ценз живого ядра (patched-kernel.jar round-396-a, purpur-1.21.10,
   sha 29386794B байт-в-бит) — ВСЕ сайты Level.getEntities / Level.getEntitiesOfClass /
   EntityGetter.getEntitiesOfClass / Level.getPushableEntities / findNearbyPlayer /
   ServerEntityGetter.getNearest*.
2. Свежие vanilla-якоря ВОЛНЫ ×446 (master 4ab73061, lever_flag пуст, мир MineShield-3
   world_sha256 afb3a0b3…, population 150k seed 42, fake_players 4, band-раннеры):
   **anchor-2** (run 35992154085, 114263 cpu-сэмплов, runner 8.65M) и **anchor-5**
   (run 35992226151, 112975 сэмплов) — артефакты скачаны до абсорба, collapsed-профили
   разобраны root-unique атрибуцией (лейн = union включительных долей корневых
   query-методов; двойной счёт исключён разбором стека до ПЕРВОГО корня).

## 1. Лейн целиком (свежие ×446-якоря): 8.5-8.9% wall — подтверждено ×2

| корень (javap-метод) | anchor-2 | anchor-5 |
|---|---|---|
| Level.getPushableEntities | 4.68% | 4.01% |
| Level.getEntitiesOfClass | 2.94% | 2.81% |
| EntityGetter.getEntitiesOfClass | 0.61% | 0.58% |
| Level.getEntities | 0.31% | 0.28% |
| ServerEntityGetter.getNearestPlayer | 0.21% | 0.16% |
| ServerEntityGetter.getNearestEntity | 0.09% | 0.08% |
| **UNION лейна** | **8.83%** | **8.52%** |

Числа C-445 (8.31%/7.80% на ×436c) воспроизводятся на ×436c → лейн реален, НО
декомпозиция ниже показывает: **остатка, адресуемого новой плоскостью, нет**.

## 2. Атрибуция по вызывателям (обе ноги согласованы ±0.1пп)

| вызыватель → сайт | a2 | a5 | статус на носителе cmp444_sensemega |
|---|---|---|---|
| LivingEntity.pushEntities → getPushableEntities | 4.68 | 4.01 | НЕ покрыт (allocdiet cmp408 не в носителе) — **парити-стена S7-140** (§4.1) |
| NearestAttackableTargetGoal.findTarget → getEntitiesOfClass | 2.36 | 2.26 | **ПОКРЫТ** eindexq (entity_query.rs + EntityGoalQueryOps: cmp444_sensemega в OR, chain-скан SoA, eqEpoch bulk-JNI) |
| AvoidEntityGoal.canUse → getEntitiesOfClass | 0.46 | 0.46 | **ПОКРЫТ** eindexq ⊕ queryplane (Player.class fast path) |
| Mob.aiStep (offset 109) → getEntitiesOfClass(ItemEntity, inflate(reach)) 2-arg | 0.60 | 0.58 | НЕ покрыт — **items-субстрат граница** (§4.3) |
| AbstractBoat.tick (offset 536) → getEntities(Entity,AABB,Pred) | 0.22 | 0.21 | НЕ покрыт — **universe-граница** (§4.4) |
| TemptGoal.canUse → getNearestPlayer | 0.17 | 0.14 | **ПОКРЫТ** sense (4 getNearestPlayer-варианта депегаются в getNearestEntity body-swap) |
| findTarget → getNearestEntity/getNearestPlayer (player-target ветка) | 0.10 | 0.10 | **ПОКРЫТ** sense |
| ServerLevel.tick → getEntities | 0.05 | 0.05 | микро (<0.05%, запрещено директивой) |
| HopperBlockEntity.pushItemsTick → getEntitiesOfClass | 0.05 | 0.06 | items-граница + микро |
| OldMinecartBehavior.pushAndPickupEntities → getEntities | 0.02 | 0.02 | universe-граница + микро |
| NearestLivingEntitySensor/NearestItemSensor.doTick | 0.03 | 0.04 | universe-граница (LivingEntity/ItemEntity-универсум) + микро |
| LookAtPlayerGoal/armorstand/minecart-hopper/merge/прочее | ≤0.03 | ≤0.03 | микро |

## 3. javap ground truth ключевых тел (purpur-1.21.10)

- `Mob.aiStep` @109: `invokevirtual Level.getEntitiesOfClass(Class,AABB)` — 2-arg БЕЗ
  предиката, класс = ItemEntity.class, box = `getBoundingBox().inflate(getPickupReach())`
  (purpur-хвост `entitiesPickUpLootMobGriefingOverride`); далее ИТЕРАЦИЯ ВСЕХ кандидатов:
  skip isRemoved/emptyItem/hasPickUpDelay → wantsToPickUp → canMobPickup → pickUpItem.
  Порядок = section-walk (EntityCollectionBySection); выбор «кто первым» — при
  контеншене двух мобов за предмет решает ПОРЯДОК ТИКОВ мобов, не порядок списка.
- `LivingEntity.pushEntities` @75: `getPushableEntities(this, getBoundingBox())` — БЕЗ
  inflate; пусто → return; cramming-hurt по list.size() (порядок-инвариантен); затем
  `for j: list.get(j).push(this)` — порядок пушей = порядок списка → последовательность
  setDeltaMovement наблюдаема (траектории).
- `AbstractBoat.tick` @536: `getEntities(this, bb, Predicate)` — кандидат-универсум =
  ВСЕ сущности (пассажир-пикап).
- `EntityGetter` (javap -p): getEntities(Entity,AABB,Pred) abstract;
  getEntities(EntityTypeTest,AABB,Pred) abstract; getEntitiesOfClass 3-arg/2-arg —
  default (депегаются в EntityLookup.getEntities → ChunkEntitySlices → секции).

## 4. Парити/субстратные стены (почему «просто покрыть» нельзя)

### 4.1 pushEntities 4.0-4.7% — парити-стена S7-140 (подтверждена байткодом §3)
- Реказ candidates → меняет последовательность `list.get(j).push(this)` →
  setDeltaMovement-последовательность → траектории 150k мобов (НЕ ties-класс
  items_subsys2, а систематическая дивергенция). S7-140 §3.1: отвергнуто.
- Порядок-сохраняющий prescreen (rust-бит «соседство пусто» / intersect-прескрин с
  сортировкой по исходному индексу): S7-140 §3.4 оценивал net <1%; РЕАЛЬНОСТЬ хуже —
  при 150k населения и ~2 моба/секцию-ячейку честный «соседство-пусто»-бит требует
  pair-enumeration O(n·k): 150k × ~9 ячеек × ~35 соседей ≈ 45M pair-checks/тик в rust
  ≈ 100-300мс — НА МНОГО порядков отрицательный net. Cell-emptiness 3×3 (airtight для
  мобов: push-box без inflate ≤4 ячеек) даёт экономию только на доле пустых окрестностей
  (неизвестна, верхняя оценка ~30% × 1.5µs/вызов ≈ 1.0-1.5% wall MINUS rust-pass
  ~0.15% MINUS players-corner (4 фейк-игрока — pushable-кандидаты!) MINUS
  non-SoA-pushables (лодки/шалкеры — нужен HARD_ADDS-style probe) —
  **итог <1.5% best-case, ниже микро-директивы 2-3%**, при порядочном
  corner-case риске. ЗАПРЕЩЕНО директивой.
- Tick-stamped AABB-кэш списков кандидатов: валидность требует отслеживания движений
  соседей (= та же pair-enumeration) — S7-140 §3.3 «кэш мёртв всегда» остаётся в силе
  для любого ген-триггера дешевле O(n·k).
- Tiered spatial index (иерархия ячеек): сама по себе не убирает ни pair-enumeration
  (валидность), ни порядок section-walk — только пересортировка того же перечисления.

### 4.2 findTarget+avoid 2.8-2.9% + nearest 0.3% — УЖЕ ПОКРЫТО носителем
entity_query.rs (eindexq, eqEpoch chain-hash over mobs_soa — ОДИН bulk-JNI/тик,
пер-entity JNI отсутствует) и mobs_sense.rs (senseEpoch) ОR-ят cmp444_sensemega:
носитель sensemega @5cfafaba несёт этот срез. Повторное покрытие = ноль дельты.

### 4.3 Mob.aiStep looting 0.6% — items-субстратная граница
Кандидаты = ItemEntity — НЕ в mobs_soa (мобы/игроки only). Честный сервис требует
items-SoA (upsert из ItemEntity.tick) = items-лейн (агент-B-445, живой вектор,
items 29.7% — лан №1 эры) + collides с items_oss (ItemMergeOps) механикой мержа.
Чистая доля скана ≈ 0.3-0.4% (предикат/итерация остаются) — микро даже при нулевой
стоимости субстрата.

### 4.4 Лодки/вагонетки/сенсоры ~0.3% — universe-граница
Кандидаты = все сущности (или LivingEntity/ItemEntity-классы): superset-контракт
eindexq не выполним из mobs_soa (нет не-мобов в плоскости); без superset гейт
некорректен по построению. Микро-доля даже в идеале.

### 4.5 SchedmuleUtil / NavigationPortal — мёртвые под-лейны на фикстуре
- «SchedmuleUtil»: класса НЕТ в ядре (jar-tf ценз: только net/minecraft/world/entity/schedule/
  (вилладжер-schedule — сканов не делает) и ai/sensing/Sensing (LOS-кэш — в топе
  атрибуции отсутствует = <0.01%)). Интерпретация брифа = sensing/schedule-соседства —
  микрофракция, ниже ценза.
- NavigationPortal-поиски (PortalForcer/PoiManager): POI-лейн 0.06%/0.06% (C-445) —
  мёртвый лейн; порталы в фикстуре не используются.

## 5. Сводка адресуемости cmp446_eq

| кусок лейна | доля (a2/a5) | вердикт |
|---|---|---|
| pushEntities | 4.68/4.01 | парити-стена (reorder = геймплей-дивергенция; prescreen/кэш/индекс = net<1.5% best-case = микро-директива) |
| findTarget+avoid (eindexq) | 2.82/2.72 | уже покрыто носителем |
| nearest-players (sense) | 0.30/0.24 | уже покрыто носителем |
| looting | 0.60/0.58 | items-субстратная граница (вектор агента-B) |
| boat/minecarts | 0.24/0.23 | universe-граница |
| hoppers/sensors/ServerLevel.tick/прочее | 0.15/0.17 | микро |
| POI/порталы | 0.06/0.06 | мёртвый лейн |
| **ЧИСТО адресуемо cmp446_eq** | **≈0.0-0.15%** | ниже микро-директивы на порядок |

## 6. ВЕРДИКТ: LOW-POTENTIAL — имплементация/ноги НЕ оправданы (честный финиш PHASE RESEARCH)

- Лейн 8.5-8.9% подтверждён ×2 свежими якорями ×446, НО: ~3.1пп уже покрыты носителем
  sensemega (eindexq+sense), ~4.0-4.7пп = pushEntities за парити-стеной S7-140
  (подтверждена javap: push-цикл порядок-наблюдаем; все order-preserving варианты =
  pair-enumeration O(n·k) при 150k населения или <микро net), ~1.0пп = субстратные
  границы (items/universe) и микро.
- Потолок ИДЕАЛЬНОЙ cmp446_eq-плоскости ≈ 0.0-0.3% TPS — ожидаемая нога = шум
  (≈±2-4пп батч-ноайз волны) ≪ мандата ≥+20% pair. Диспатч ног при недоказанном
  потенциале ЗАПРЕЩЁН (BOTTLENECK-446 TOP-2; прецедент ×445-C wgen: «диспатч
  разрешён только при доказанном потенциале»).
- Экономия: 2 ноги бюджета волны сохранены для реальных серт-шансов (ins4d 3-я пара,
  sensemega-банк, golden 02:08).
- Рекомендация эре: рекомендацию «entity-query слой 8.31% = реальный хедрум» ×445-C
  СНЯТЬ — лейн-тотал вводил в заблуждение; остаток вне носителя/стен <1.5%.

## 7. Условия переоткрытия (track, не забыть)

1. items-вектор агента-B посадит rust items-SoA → looting (0.6%) становится сервисным
   тем же eq-chain-hash — ДОБАВКА к его носителю, не соло-вектор.
2. Владелец санкционирует push-order дельту (аномалия траекторий ~1e-16/оп, растущая
   хаотически) → открывается 4.0-4.7пп — НО это геймплей-закон, не моё решение.
3. Смена фикстуры (меньше мобов/больше игроков рядом) перекашивает доли —
   перепровести ценз на новой фикстуре.

— agent-eq, tick-446, 2026-09-24; evidence: /tmp/eq446/a2/cpu-collapsed.txt,
/tmp/eq446/a5/cpu-collapsed.txt (до конца тика), ветка round-446-eq.
