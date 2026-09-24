# RESEARCH-450-B — items root-cause (lane 31.17% ARMED-но-не-двигается) + cycle-4 план

Task: TASK-450-B (продолжение 447-B/448-B/449-B). Ветка: round-450b-items (master d02a2977 + round-446-items @a6648df2).
Метод: артефакты готовых ранов (world3-bench: server-stdout.log + cpu-collapsed.txt) — СВОИ и золотые чужие, числа только из них.

## 0. Артефакт-база (все скачаны и разобраны)

| ран | ветка | SHA/lever | лейн items | источник |
|---|---|---|---|---|
| 36043768180 | round-443g-anchor-5 | master vanilla | 28.98% | golden ×449 |
| 36038486355 | round-443g-ins4-1 | cmp436_ins4 @07078007 | **0.00%** | golden ×449 |
| 36015023714 | round-448b-items-1 | 1f73d9f8 (cmp446_items) | 25.26% | тик ×448 |
| 36023838241 | round-449b-items-2 | a6648df2 (cmp446_items) | 30.07% | тик ×449 |

Ключевой факт: лейн-items в absorb_round.py — это РЕГЭКС `ItemEntity\.tick` по cpu-collapsed
(строки стека, весь вес строки в лейн при любом совпадении кадра). Это лейн-ИНКЛЮЗИВНАЯ
метрика имени кадра, не подсистема.

## 1. ROOT-CAUSE #0: «items-плоскость в ins4/mega4 роняет лейн в 0.00» — МИРАЖ (опровергнуто)

В ins4-1 cpu-collapsed: `net/minecraft/world/entity/ItemEntityManager.tickOne` = **30.35%**
всех сэмплов (через tickOne→dispatch→tickBody→baseTick/applyEffects/fluid/collision — ТА ЖЕ
работа, дети поддерева пропорциональны ванильному ItemEntity.tick). Кадр просто ПЕРЕИМЕНОВАН
(items_subsys2 defines ItemEntityManager в kernel loader + tickplane-скелет cmp403_tickplane),
регэкс `ItemEntity\.tick` не матчится → лейн 0.00.

**Стоимость items НЕ изменилась: 30.07% (cmp446_items) ≈ 30.35% (ins4 tickOne) ≈ 28.98-31.17% (ваниль).**
«Рабочая items-плоскость в lineage ins4/mega» — НЕ существует; TPS-победы ins4-семьи дают
nav_ai (14.2→3.1-4.2) и broadphase (15.7→9.1-9.6), не items.

## 2. ROOT-CAUSE #A (главный механический блокер ×449): append-gating баг cycle-3

Diff 1f73d9f8→a6648df2 (tick()): cycle-3 ввёл drain-throttle и ЗАГЕЙТИЛ append на drainTick:

```java
boolean drainTick = st - meta[6] >= DRAIN_EVERY;   // =4
if (drainTick && ...) { drain(meta, st); ... }
if (drainTick) { append(meta, e); }                // ← БАГ: append ТОЛЬКО на drainTick
```

drain() в конце ставит meta[6]=st → до следующего окна 4 тика append НИКОГДА не зовётся.
Первое существо тика на потоке аппендится, остальные ~26к — НЕТ. Снапшот = 1 item/4 тика/поток
(≈0.01% популяции), hash решений пуст для всех остальных → lookup=FULL (fail-open) → 100% ванильный
реплик-путь. Плоскость мертва при живом ARM.

**Профайл-пруф (items-2, a6648df2)**: поддерево ItemEntity.tick = vanillaTick 99.4%,
append 0.0%, drain 0.0%, lookup 0.3% (мисс на каждой сущности). Лейн 30.07% = ваниль.
**Контраст (items-1, 1f73d9f8, кормленая плоскость)**: vanillaTick 91.7%, append 1.06%,
drain 0.61% (внутри — planeDecide→decide_batch 593 сэмплов: rust РЕАЛЬНО решал батчи),
lookup 0.27%. Оверхед плоскости ≈2-3% total CPU.

## 3. ROOT-CAUSE #B: EFFECT/X-ray маркер НЕПЕЧАТАЕМ в бенче (инструмент молчал)

Гейт EFFECT-лога: `st - meta[5] >= 1200` (рассчёт на 20 TPS = 60с). Бенч 300с при ~2.2 TPS =
**~660 серверных тиков на весь ран** → EFFECT-строка не может напечататься НИ В ОДНОМ бенч-леге.
Греп по обоим items-легам (×448 items-1, ×449 items-2): EFFECT=0, severe=0. Поэтому X-ray
счётчики причин (a6648df2 stats[3..7]: ground/fluid/hdsqr/pd/other) НИ РАЗУ не отчитались —
«ARMED-но-почему-FULL» вопрос оставался без инструмента.

## 4. ROOT-CAUSE #C (глубокий слой, требует X-ray с ноги): кормленая плоскость решает ~всё FULL

items-1 (плоскость кормлена КАЗДЫМ тиком): decide_batch реально выполнялся, buildHash строил
решения, НО inactiveTick = 0.01% сэмплов (≈НОЛЬ REST-исполнений), vanillaTick = 91.7%.
Т.е. даже кормленая плоскость НЕ отдыхает ни один item этой фикстуры. Кандидаты причины:
- fixture-мисс: BenchPopulation.spawnItem = `dropItem(getHighestBlockYAt+1.0)` — на водных
  колонках это ПОВЕРХНОСТЬ ВОДЫ → item плавает вечно (isInWater, onGround=false, дрейф
  hdsqr>1e-5) → системно ineligible по «сухому» предикату planeResting-реплики. Предикат
  верен по парити (отдыхающий water-item = видимая дивергенция), лезвие = срез фикстуры.
- pd=32767 (ITEM_PICKUP_DELAY) — предикатом принят ✓, не блокер.
- возможен тайминг-эффект server-thread vs region-worker (getTickCount видимость) —
  Explain-кандидат для «почему EFFECT не печатался даже при n=1» в items-2; лег cycle-4
  с безусловным маркером даст эмпирику.

Якорный бэнд лейна 28.98-31.17 (2.2пп разброс) — прошлые «лейн сдвинулся −5.9/−6.6пп»
(24.57/25.26) частично внутри бэнда+шума; честный сигнал придёт с X-ray суммарными счётчиками.

## 5. CYCLE-4 ПЛАН (имплементация)

RUST-FIRST, минимальный диф, всё под флаг-гейтом cmp446_items (пустой = ваниль бит-в-байт):
1. **Java ItemBatchOps**: (a) append КАЖДОЙ сущности КАЖДЫЙ тик (гейт drainTick снят),
   drain по факту данных (meta[1]>0 && st!=meta[0]) — восстановление ДОКАЗАННО-кормленого
   потока 1f73d9f8; (b) EFFECT/X-ray: безусловная печать ПЕРВОГО датадрейна (meta[5]==0)
   + каждые 300 тиков; (c) CUM-ThreadLocal: кумулятивные full/rest/rechecks/reasons за ран
   → греппуемый SUMMARY; (d) канон n-contract (doubles→items), stride-гейт fail-closed ДО
   натива, JNI_ABORT hardening — сохранены (урок ×447).
2. **Rust**: логика decide_batch НЕ меняется (X-ray уже в языке; минимальный диф = быстрый
   зелёный гейт), cargo check --lib + cargo test (311) зелёные.
3. Блобы: build_itemsbatch_ops.sh → flat==nested + raw-маркеры + javap-LOADABILITY.
4. Диспатч world-bench-parallel.yml ref=round-450b-items, lever_flag=cmp446_items, банк-inputs,
   скрипт с argv-guard (--dry-run канон ×447). Вердикт pair-by-runner vs round-450-anchor-*,
   band 6.0-9.5M, депресс-гейт norm≥−2, pair Δ≤50k, AIOOBE=0, selfTest==true.
5. Чтение лега: X-ray buckets (fluid vs ground vs hdsqr) = окончательный вердикт RC-C;
   лейн был→стал = механический эффект #A. Δ<+20% → новый research-виток (срез/расширение
   или честное LOW-POTENTIAL закрытие с числами — по канону wgen).

## 6. Что это даёт эре

- Снимает проклятие «ARMED-но-лейн-не-двигается»: известны все три слоя (баг-гейтинг,
  немой маркер, срез фикстуры).
- Возвращает в эру честную метрику: лейн-items в ins4/mega4-носителях = артефакт регэкспа
  (ItemEntityManager.tickOne 30.35%) — «0.00» никогда не был эффектом плоскости.
- X-ray-счётчики (первый раз печатаемые) = карта «сколько из 31% лейна вообще отдыхаемо»
  в этой фикстуре → потолок вектора числом, а не гипотезой.
