# ROUND-435-B RESEARCH — inside2-плоскость: следующий hot-lane (глубина 4)

Автор: TASK-435-B (cron tick-435). База: round-434-b-inside4 @92cd1e5 (NCDFE=0 ×3 впервые).
Вопрос рисёрча: внутри-плоскость впервые ПОЛНОСТЬЮ живая (stage-1c + inside2);
inside_volatile 15.9% (leg 432-ins-l1r3) — что под ним ТЕПЕРЬ, при живой снап-плоскости?

## 1. Данные (локальные collapsed-профили + top-40 ног ×434)

Лейн-дефиниция (absorb_round.py:19): `inside_volatile = checkInsideBlocks|collidedWithShapeMovingFrom`
(по collapsed-стекам). Разбор anchor-o2 (master, 117279 сэмплов, full-bridge, inside_cache=1):

- checkInsideBlocks subtree = **10.83%** total CPU; collidedWithShapeMovingFrom subtree = 0.64% (отдельный fluid-лейн, НЕ мой).
- Анатомия checkInsideBlocks (o2, ванильный лейн):
  - `Entity.lambda$checkInsideBlocks$2` (per-visit тело, ~5.5%):
    - `Entity.collidedWithFluid` **1.69%** (FluidState.getAABB→Fluid.getAABB 1.02 + AABB-математика) — **ЧУЖОЙ лейн** (fluid_guard/fluid_bitmask, спека inside2: «the fluid sub-lane — other levers' lanes»)
    - `BatchCollector.advanceStep→flushStep` **1.08%** (лист 0.81 + ArrayList.isEmpty 0.60 inline) — batch_collector lever #8, чужой
    - `Level.getBlockState` **0.90%** — ЦЕЛЬ снап-плоскости (ретаргет на snapGet)
    - `LongOpenHashSet.add` 0.50% (visited-dedup), leaf 0.39%, shape/effect ~0.7%
  - `InsideBlockOps.gate` (мемо inside_cache) **1.61%**: leaf 0.90 + getBlockState 0.18 + mp/mirror/advanceStep
  - traversal `forEachBlockIntersectedBetween` ~1.6% (betweenCorners 0.77, iterator 0.53, addCollisionsAlongTravel 0.64) — flat_traversal #9 ЗАПРЕЩЁН (ghost≠snapshot)
- Дельта leg-и ×434 (cmp432_inside2 live, top-40 из BOTTLENECKS_3.md) vs anchor-o2:
  - PalettedContainer.get total 3.8→3.3, readPalette 1.0→0.7, getBlockStateFinal 0.8→вне top-40
  - **InsideSnapOps.serve self 0→1.9% (1995 сэмплов) = НОВЫЙ top-6 лист**

## 2. Вердикт рисёрча: следующий hot-lane = САМ serve()-fastpath

Объём драйвит InsideBlockOps.gate HIT-верификация (inside_cache=1): ~107k статичных
item-сущностей × ~10-30 visited-позиций КАЖДЫЙ тик → bstate→snapGet→serve. Бёрсты
same-chunk/same-section доминируют. Текущий serve() на КАЖДЫЙ визит платит:

1. `level.getGameTime()` volatile-read
2. TL.get (lanes())
3. **scan 8 лейнов (до 8×5 полевых сравнений)**
4. `((LevelHeightAccessor) ch).getSectionIndex(y)` — **2 itable-dispatch + 1 static** за визит (javap: getSectionIndex → blockToSectionCoord (i>>4) + getSectionIndexFromSectionY → getMinSectionY)
5. `sec.hasOnlyAir()` per-visit
6. SNAPS CHM.get при смене секции в лейне
7. 2 volatile (gen/builtAtGen)
8. **LongAdder.increment() на КАЖДЫЙ hit — striped-cell contention 4 region-воркеров на самом горячем пути**

== serve честно стоит ~1.9% self и съедает заметную часть того, что снап-плоскость экономит.

## 3. Дизайн cmp435_inside3 = INSIDE-SNAP SERVE-V3 (STRICT-OR поверх cmp432_inside2)

Закон 6: подсистема целиком = весь serve-путь, один флаг-гейт, без чужих лейнов:

1. **L0-прямой лейн**: lanes[LANES] (размер 9, слот 8 = L0, климы masked по 8) —
   check (tick,level,cx,cz) 5 сравнений; на промах — прежний 8-слотовый scan +
   **copy-on-match в L0** (7 сторов на смену чанка); на полный промах — claim как V2.
2. **Inline section-index**: lane.minSecY = getMinSectionY() один раз на claim;
   si = (y>>4) - minSecY (javap-эквивалентность: blockToSectionCoord = i>>4).
   **Runtime-парити-проба на каждый claim** (3 значения y против живого
   getSectionIndex того же ch): mismatch → lane.inlineOk=false → itable-путь
   (fail-open, median-exact).
3. **Air-маска**: long на claim (secs.length ≤ 64; иначе conservative all-non-air);
   секции null/hasOnlyAir → бит → return null БЕЗ загрузки sec.
4. **Пер-тредовые hit-счётчики**: lanes[0].hitSub (plain int, zero contention),
   STAT_HITS.add(64) на переполнении (точность в пределах 63/тред; hits() = диагностика,
   прод-потребителя нет; гейт-абсорб смотрит маркер "first gate HIT served" — он остаётся).
5. **V2 НЕ тронут**: `serve() = V3 ? serveV3 : serveV2`; V3=false при
   lever cmp432_inside2/cmp430_inside (ноги r1..r4 = чистый контроль).

Парити-контракт: V3 возвращает ТОТ ЖЕ объект BlockState на тех же позициях, те же
null/miss-ветки, тот же stale-miss continuation `sec.states.get(packed)`, тот же
seqlock (gen/builtAtGen), fail-dominant try/catch — без изменений.

Rust-гейт: lever_flag_matches() += "cmp435_inside3"; при v3_requested → static call
`InsideSnapOps.v3()V` ДО selfTest/arm; фейл резолва → dormant (fail-closed для нового
рычага). Resolution closure += ("", "", "v3", "()V"). Левер-списки STRICT-OR:
mobs_manager.rs ×3, mobs_grid.rs, MobPushOps.java, GoalOps.java, QueryPlaneOps.java.

Маркеры абсорба сохраняются с каноническим префиксом cmp432_inside2 (грепы T1);
добавлен громкий маркер `cmp435_inside3: V3-serve enabled`.

## 4. Что НЕ трогаем (чужие лейны, дисциплина вектора)

- collidedWithFluid/FluidState.getAABB 1.69+1.02 — fluid-лейн (fluid_guard/fluid_bitmask)
- BatchCollector.flushStep/advanceStep — batch_collector lever #8
- traversal/iterator — flat_traversal #9 запрещён, zero_cursor #11 — чужой
- InsideBlockOps.gate мемо-машина — inside_cache lever #3 (фьюжн только через snapGet)

## 5. Ожидание и цикл

serve 1.9→~0.5 self + снятие contention-хвоста → надежду даёт объём: если legs r1..r4
(cmp432_inside2) в золотом окне дадут базу B, ноги r5+ (cmp435_inside3) = B + V3-эффект.
Δ<+20% pair → цикл закона 3 продолжается (новый рисёрч: валидация статов MISS/INVALIDATION
по сервер-логам следующей ноги; кандидат-2 = memo-гейт HIT-верификация topN-сэмплинг).
