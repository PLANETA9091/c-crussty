# RECON-6 — разложение фазы GC/JIT-native (ТОП-2, 35.49% CPU) — v3 bank leg 35399980345

Тик 08:08 +08 (2026-09-19), Job 396026, TASK-308. Методика «ТОП-ПОЖИРАТЕЛЬ → ∞»:
ТОП-1 = entity-фаза (59.06%), её attackable под-лейн inside-pipeline traversal = рычаг #9
FLAT-TRAVERSAL — **лег в полёте** (run 35407788083, absorb — следующий тик). Пока лег
летит, выполняем RECON крупнейшей НЕРАЗЛОЖЕННОЙ фазы: GC/JIT-native 45470/128124 =
35.49% CPU. Артефакт: `research/batch-collector-2026-09-19/run-s7162-leg2-artifact/`
(CUMULATIVE v3 = inside_cache + flush_diet + region_threads=4 + batch_collector).
Инструменты: `recon6_gcjit.py` (три оси), `recon6_alloc_callers.py` (пинование
вызывающих), сырой вывод `RECON6_GCJIT_raw.txt`.

## (a) Ось CPU — под-лейны фазы (классификатор фазы идентичен absorb_s7162.py)

| под-лейна | сэмплы | % CPU total | % фазы |
|---|---|---|---|
| GC-concurrent REFINEMENT (G1ConcurrentRefineThread) | 22917 | **17.89%** | **50.40%** |
| GC-concurrent MARKING (G1CMTask) | 6857 | 5.35% | 15.08% |
| GC STW-worker EVACUATION (scan_roots/trim_queue) | 6186 | 4.83% | 13.60% |
| GC STW-worker REBUILD-RS/scrub | 5377 | 4.20% | 11.83% |
| JIT-compile (C1/C2/broker) | 2611 | 2.04% | 5.74% |
| GC other STW + service + VM-thread + unclassified | 1522 | 1.19% | 3.35% |

Половина фазы — ОДИН конкурентный refine-тред (G1DirtyCardQueueSet::refine_buffer =
23098 вхождений = 50.8% фазы): переклассификация dirty-card от write-барьеров. На боксе
4 vCPU это ~0.7-0.9 ядра, отобранного у тик-тредов (main + 4 region worker'а). Marking —
следствие аллокационного давления (IHOP). STW evac+rebuild ≈ 9% CPU в сэмплах +
прямая заморозка тик-тредов (см. ось b). JIT 2.04% — стартовый шум, НЕ цель.

## (b) Ось gc.log — паузы и аллокационный темп

| метрика | значение |
|---|---|
| young GC (весь ран) | **154** события, 0 Full |
| паузы | total 21127 ms; mean 137.2 / **median 156.0** / p90 175.8 / **max 190.4 ms** |
| причины | 124×G1 Evacuation (Normal/Concurrent Start/Mixed/Prepare-Mixed), 6×Metadata, 2×CodeCache, 1×GCLocker |
| живой сегмент (t≥22:15:47 UTC) | 77 событий; total **12684 ms**; mean 164.7 / median 161.4 / max 190.4 |
| **STW wall-share живого сегмента** | **4.29%** стены (42.9 ms паузы на секунду стены; span 296 s) |
| аллокационный темп между young GC | **median 1640 MB/s** (mean 1634, max 2037) |
| меж-GC интервал | min 2.82 / **median 3.85** / max 5.21 s |

Интерпретация: при медиане тика ~525 ms (1.9 TPS) каждая young-пауза 156-190 ms — это
~30-36% медианного MSPT, съеденного ЗАМОРОЗКОЙ. Плюс refine-тред непрерывно конкурирует
за CPU вне пауз. Фаза GC/JIT — полностью downstream от **аллокационного темпа 1.6 GB/s**:
 Eden (2 GB) наполняется за ~3.9 s → evac; write-барьеры от ref-записей → card churn →
refine; occupancy → marking.

## (c) Ось alloc-collapsed — кто генерирует мусор

Топ-листва (доля всего аллока): **AABB 20.19%**, **Vec3 19.89%**, BlockPos 6.03%,
MutableBlockPos 5.47%, long[] 4.77%, BlockPos$6 (guava-итератор) 3.62%, byte[] 3.11%.

Семья AABB+Vec3+BlockPos(+Vec3i) = **56.2% всего аллокационного давления**; по лейнам:
inside-pipeline(checkInsideBlocks) 42.0% семьи, tickBucket-orch 25.0%, broadphase/push
13.4%, movement/travel 10.4%, fluid 4.6%.

Пиновка вызывающих (главное):
- AABB: collidedWithShapeMovingFrom→makeBoundingBox 17.3%, FluidState.getAABB (visit-
  лямбда inside) 16.7%, checkInsideBoxes deflate/makeBoundingBox 16.2%, move/collide
  inflate/expandTowards ~9.7%, setPos 3.4%, isInWall 2.5%.
- Vec3: collidedWithFluid→AABB.collidedAlongVector (add/getCenter) 21.0%, 
  updateFluidHeightAndDoFluidPushing (add/scale/getFlow) 21.0%, traversal-temps
  (subtract/add, addCollisionsAlongTravel) 9.7%, ItemEntity-ветки 6.0%.
- BlockPos$6/BlockPos$4: 99.5%+96.6% = forEachBlockIntersectedBetween (guava-итераторы)
  — **уже атакованы рычагом #9 (в полёте, zero-alloc по дизайну)**.
- MutableBlockPos: FlowingFluid.getFlow 15.7%+12.5%+5.7%, guava-итераторы 18.2%,
  pathfinding ~11%.

## ВЕРДИКТ RECON-6

1. Фаза GC/JIT — НЕ самостоятельный пожиратель, а **downstream аллокационного давления
   1.6 GB/s** из entity-путей. JVM-флаги/GC-тюнинг запрещены (config-wins) — рычаг
   только один: **убирать мусор в источнике** (zero-alloc тик-пути).
2. Иерархия атаки внутри фазы: (i) kill leaf-мусора AABB/Vec3/BlockPos в inside-pipeline
   и fluid-push — снижает И аллок-темп (паузы/marking), И часть CPU entity-фазы
   (двойной рычаг по двум осям сразу); (ii) ref-запись churn (refine) — сокращается
   опосредованно через меньше временных объектов/записей; прямой рычаг на G1 запрещён.
3. Классы-кэши в collidedWithFluid/getBlockState — REFUTED ×3 (позиционная мемоизация
   бьётся о нестабильное население; третье независимое подписание). Zero-alloc — ДРУГОЙ
   класс рычага: не запоминает ничего между вызовами, только устраняет аллокацию
   временных объектов при той же бит-в-бит double-математике и том же control-flow
   (vanilla-parity по построению).

## КАНДИДАТ РЫЧАГА #10 — ZERO-ALLOC-INSIDE (следующий после absorb #9)

Скоп: внутри inside-pipeline (Entity.checkInsideBlocks + Entity.collidedWithFluid +
collidedWithShapeMovingFrom) и fluid-push lane (updateFluidHeightAndDoFluidPushing /
FlowingFluid.getFlow) заменить аллоцирующие векторные вычисления на примитивные/
scratch-переиспользуемые (long-packed BlockPos, mutable AABB-поля, double-примитивы
вместо Vec3.add/scale/subtract temps) с сохранением ТОЧНОЙ последовательности операций
над double (бит-в-бит; Vec3.add = (x+dx,y+dy,z+dz) — примитивы эквивалентны по IEEE754).

Методика — как #9: javap-контракт бит-в-бит → lockstep-оракул-харнесс (десятки тысяч
сценариев × все visitor-политики) → rust-стадия entity_compose (stage 7?) → прeregister
гейты (per-work снижение alloc-листв семьи inside ≥50%, young GC ≤ 154, TPS ≥ 1.60,
0 NCDFE/ARMED-цепь) → диспатч после absorb #9 по свежему ТОПу.

ЗАМЕТКА О ПОРЯДКЕ (методика «ТОП-ПОЖИРАТЕЛЬ → ∞»): пересортировка ТОПа после absorb #9
может сместить порядок (traversal-лейн просядет, inside-семья AABB/Vec3 станет видимой
частью entity-фазы, GC-оси проседают следом). Диспатч #10 — только после absorb #9 и
свежего замера, по фактической сортировке.

Файлы: recon6_gcjit.py, recon6_alloc_callers.py, RECON6_GCJIT_raw.txt (сырой вывод).
