# RESEARCH-459-L04 — eqsnap2+H03 композиция: capture-матем (TASK-459-L04)

Агент: TASK-459-L04 (ЛАБ ×459). Слот: L04 = eqsnap2+H03 композиция. Носитель round-457c-eqsnap2 @9d71b461
(leg-1 +2.6@6619122, REFUTED_CENS: захват 0/2.3% среза). Гипотеза H03: eqsnap2-ноги + Hilbert interval-tree
targeting R3 (план cmp458_hilbert agent-J ×458, geo-index 0.4.0) → пара ≥+20.
Вопрос брифа: какие слайсы забирает Hilbert-порядок; честный вывод — композиция достижима или потолок <20пп.

## 1. ГРАУНД: профили leg-1 (round-*eqsnap* в gc-recon ОТСУТСТВУЕТ — проверено ls; ближайшие leg-профили = chkmono457-11/14/16 на родственной несущей с тем же entity-query слоем)

CPU-collapsed (лист-ранжирование broadphase/target-слайсов, доля от тотала):

| фрейм | chk-11 (leg-1, 104163 сэмплов, runner 7040413, norm −3.6 депресс) | chk-14 (монстр, 103062, 8687055, +21.7) | chk-16 (104735, 6951662, +12.7) | anchor458-33 (ваниль, 115503, 6973621, −11.8) |
|---|---|---|---|---|
| `ChunkEntitySlices$EntityCollectionBySection.getEntities` | **964 = 0.93%** | 1120 = 1.09% | 879 = 0.84% | 2390 = 2.07% |
| `EntityLookup.getHardCollidingEntities` | **800 = 0.77%** | 926 = 0.90% | 799 = 0.76% | 749 = 0.65% |
| `AABB.intersects` | **727 = 0.70%** | 871 = 0.85% | 795 = 0.76% | 2466 = 2.14% |
| `ChunkEntitySlices.getEntities` (внешний слой) | — | — | — | 2143 = 1.85% |
| `EntityGoalQueryOps.snapshotQuery` (собств. цена eqsnap-плоскости) | 3246 = 3.12% | 2754 = 2.67% | 3128 = 2.99% | 0 (нет в ванили) |

Лейны ABSORB: broadphase 15.66% (базлайн) → 8.82/9.36% на ногах chk-11/14 → 14.68% на anchor458-33;
nav_ai 14.16 → 2.75/3.31% на ногах. Ваниль-якорь тратит на 4 broadphase-листа ≈ 6.7пп CPU; ноги уже сидят
на несущей и платят ≈ 2.3-2.8пп за те же слайсы.

## 2. JAVAP-КОНТРАКТЫ (3, jar = research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar, jdk 21.0.12.1)

1. `ChunkEntitySlices` — 4 перегрузки `getEntities(Entity|EntityType, AABB, List, Predicate[, int])` +
   `getHardCollidingEntities(Entity, AABB, List, Predicate)`; поля `allEntities/hardColliding/entitiesByClass/
   entitiesByType` (Reference2ObjectOpenHashMap — pre-class-индексы Lithium-класса). Это ТОЧКА ЗАМЕНЫ энумерации.
2. `ChunkEntitySlices$EntityCollectionBySection.getEntities(AABB, List, Predicate)` — байткод горячего листа:
   цикл по секциям `aaload` → `BasicEntityList.size/has` → `Entity.getBoundingBox()` → **`AABB.intersects`** →
   `Predicate.test` → `List.add`. Энумерация (секции+списки) и точный тест СМЕЖНЫ, но РАЗДЕЛИМЫ.
3. `AABB.intersects(AABB)` → делегирует `intersects(DDDDDD)`: 6 сравнений dcmpg/dcmpl, без аллокаций —
   дешёвый листовой тест. **Ключевое: в дизайн-контракте Hilbert-плейна (RESEARCH-458-J §2 п.4) точный
   `live bb.intersects` сохраняется в strict-хвосте пари → слайс AABB.intersects композиция НЕ забирает.**

Сигнатуры eqsnap2-опов (source entitygoalquery/.../EntityGoalQueryOps.java): `entitiesOfClassGate → snapshotQuery`
(расширенный прямоугольник с маржой), natives `eqProbe/eqEpoch/senseArena` — платформа композиции уже в банке.

## 3. CAPTURE-МАТЕМ КОМПОЗИЦИИ (lane% × захват% = Δ-прогноз; потолок = lane% × 100%)

**Какие слайсы забирает Hilbert-порядок (по leg-1, chk-11):**
- ЗАБИРАЕТ (энумерация → arena seq-read в Hilbert-порядке, leaf-first flatbush-контракт):
  `EntityCollectionBySection.getEntities` 0.93пп + `getHardCollidingEntities` 0.77пп = **лейн 1.70пп**.
  Замена стоит ~0.1-0.3пп (bulk-load O(n) 0.2-0.5ms/тик ≈ 0.05-0.12% тика 400мс + arena walk).
- НЕ ЗАБИРАЕТ: `AABB.intersects` 0.70пп — strict-хвост (javap-контракт п.2.3, пари);
  `snapshotQuery` 3.12пп — собственная цена eqsnap-плоскости, уже оплачена leg-базой +2.6.
- Δ-прогноз H03 = 1.70 × захват 40-60% = **+0.7-1.0пп**; потолок H03 = 1.70 × 100% = **+1.7пп**
  (оценка agent-J +1.3-1.9пп включала nav-хвост 1.2-1.5пп чистой навигации — вне broadphase-слайсов брифа).
- eqsnap2-ценз: срез 2.3% × захват 0% = 0 (REFUTED_CENS ×457-C2, item_frame×2714 hard-colliding → ваниль-фолбэк).

**Δ-прогноз композиции (нога):** +2.6 (leg-1 eqsnap2, замер) ⊕ +0.7-1.0 (H03) = **+3.3-3.6пп**;
АБСОЛЮТНЫЙ потолок ноги (100% энумерации, нулевой оверхед) = 2.6 + 1.7 = **+4.3пп**;
fantasy-bound (+ценз 2.3% вопреки REFUTED) = +6.6пп.

**Пара (pair = leg_norm − anchor_norm ≥ +20, Δ≤50k pair-fresh, min-of-3):**
- Требуемый якорь: ≤ leg−20 = **−16.4..−15.7** (реалистично); fantasy-bound ≤ −13.6.
- Банк ×459 (12 раннов): ≤−16 только **poi-14 −18.1@7223047** (1/12 = 8.3%); второй по глубине a33 −11.8@6973621
  даёт max +16.1пп < 20 ДАЖЕ при потолке ноги +4.3.
- Медиана пар-способного пула {−18.1, −5.0 (a36@7204933), +0.3 (chk-18), +0.7 (a37)} = −2.35 → **медианная пара
  ≈ +5.7..+6.0пп**; min-of-3 (консервативно min) ≈ +3.3..+3.6пп.
- Лотерея poi-14: окно [7173047,7273047] (Δ50k ≈ 2.9% бенда 6.0-9.5M) × класс ≤−16 (8.3%) → P ≈ **0.24%/ролл**,
  EV ≈ 0.07 попадания при норме ≥30 якорных роллов; min-of-3 → EV ≈ 1e-4. Недостижимо как поставка.

## 4. ЧЕСТНЫЙ ВЕРДИКТ

**ПОТОЛОК КОМПОЗИЦИИ < 20пп → ЗАКРЫТИЕ (закон 13a, REFUTED_CENS-класс).**
Числами: нога-потолок +4.3пп; медианная пара +5.7..+6.0пп; лучшая не-лотерейная пара (a33 −11.8) +16.1пп;
единственный пересекающий бар тикет (poi-14 −18.1) — джойнт-лотерея EV 0.07/тик. Три структурных блокера:
(1) база eqsnap2 всего +2.6 и её целевой срез 2.3% захвачен 0% (hard-colliding фолбэк);
(2) Hilbert-порядок забирает ТОЛЬКО энумерацию 1.70пп — AABB.intersects 0.70пп выживает по пари-контракту;
(3) якорная глубина: 1/12 банка ≤−16, min-of-3 недостижим.
H07/Hilbert-плоскость остаётся микроридером (+0.7-1.0пп к любой несущей; код cargo-ready @round-458j-hilbert),
но НЕ пара-анлокером: для монстров chk-14 +21.7 добавка не меняет якорное окно B (якорь ≤+1.7 в
[8637055,8737055] — уже цель ×459). Реальные ≥+20 пути — окна B/E монстров, независимо от H03.

## 5. PREREGISTERED ГЕЙТЫ G1-G6 (для любого будущего ридера-эксперимента на этой платформе)

- G1 ARM/эффект-маркеры: HilbertTargetOps EARLY-define (канон 43a83186/d73758a3/5ecd841a/9d71b461), NCDFE=0
  до вердикта, ht-эффект queries/tick > 0, маркер ДО пурджа.
- G2 lockstep бит-в-байт: пустой флаг = ваниль бит-в-байт; hilbert-порядок выдачи = документированный
  дельта-класс (потребители getNearestEntity строго-ближайшие).
- G3 GC: young/Full в банк-коридоре (Full ≤ 9, total ≈ 18.8s±) на целевой ноге.
- G4 популяция-паритет 140-165k живых, entity-totals polls стабильны, spawnable 289.
- G5 TPS-бар vs банк v4: norm = median/TPS_exp(runner) − 1; band 6.0-9.5M, ре-ролл ≤2.
- G6 fail-closed disarm: холодная SoA / ERR_RANGE / структурный дрейф → ваниль на вызов; selfTest=false →
  дизарм навсегда; javap flat==nested + strings блобов после каждого вайринга (урок-408 ×2).

## 6. ИСТОЧНИКИ (≥3)

1. **Moonrise Optimizations** (Spottedleaf) — chunk_system EntityLookup/ChunkEntitySlices секционный
   entity-индекс; взято: разделение enumeration/exact-test и javap-контракт перегрузок getEntities.
2. **Lithium** (CaffeineMC, entity.class mixing / SectionedEntityCache) — byClass/byType пре-индексы
   (видны в javap ChunkEntitySlices как Reference2ObjectOpenHashMap); взято: понимание, что Hilbert-арена
   конкурирует с уже существующими pre-class-индексами, а не с полным сканом → малый остаток захвата.
3. **geo-index 0.4.0 / flatbush** (kylebarron/mourner) — bulk-load-only immutable Hilbert R-tree,
   leaf-first layout → Hilbert-порядок выдачи бесплатно (ID-H07); взято: контракт арены (RESEARCH-458-J).
4. **rstar** (georust) — PARK-обоснование: динамический R-tree медленнее packed static (README geo-index).
5. **C2ME / Pufferfish** — просмотрены по памяти: C2ME = многопоточный entity-query вне тик-потока
   (закон-5-смежное, не наш путь), Pufferfish DAB = деактивация сущностей (меняет поведение — закон 4);
   оба не применимы → взято только отрицательное знание.

## 7. ЧТО ДАЛЬШЕ (CLIMB-заметка вместо мёртвой гипотезы)

Композиция закрыта, ось жива через ДРУГИЕ лейны: (а) окно B [8637055,8737055] якорь ≤+1.7 → пара с chk-14
+21.7 — диспатч-норма якорных роллов (не eqsnap2-семья); (б) H05/H07 свободны как ридеры к SWAR/roar-носителям
(broadphase-лейн ног 8.8-9.4пп — на 5× больше лейна H03); (в) hilbert-плейн как +0.7-1.0пп ридер поверх
будущего chk-климба (P31/P32+P36) — окупается только как подмешивание, не как носитель пары.
