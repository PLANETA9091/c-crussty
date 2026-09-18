# RECON-5 (S7-163 design) — FLAT-TRAVERSAL: рычаг #9, полный javap-контракт снят

Артефакты: CONTRACT_traversal_javap.txt (полный дизассембл BlockGetter/BlockPos/BlockPos$6/Direction/Vec3/AABB-clip из kernel 1.21.10, leg5 artifact); RECON4_S7162.md (лейн 7.27% CPU).
Инструмент: jdk.jdeps javap через `java -m jdk.jdeps/com.sun.tools.javap.Main` (в песочнице javap отсутствует как бинарь, но модуль жив — новый инструмент тика).

## Ванильная структура (бит-в-бит контракт)
Владелец вызова: `Entity.checkInsideBlocks(Vec3 from, Vec3 to, StepBasedCollector, LongSet visited, int step)` — ЕДИНСТВЕННЫЙ вызов `BlockGetter.forEachBlockIntersectedBetween(from, to, box=makeBoundingBox(to).deflate(9.999999747378752E-6), visitor)` (invokestatic, strict sites=1 — javap-доказано в Entity.class байткоде).

`forEachBlockIntersectedBetween(Vec3 from, Vec3 to, AABB box, BlockStepVisitor visitor) : boolean`:
1. `delta = to - from`; если `delta.lengthSqr() < square(1.0E-5f)` (СТАЦИОНАРНЫЙ путь):
   `for pos in BlockPos.betweenClosed(box): if (!visitor.visit(pos, 0)) return false;` → true.
2. Иначе (ДВИЖУЩИЙСЯ путь — горячий):
   - `visited = new LongOpenHashSet()` (per-call аллокация — 369 сэмплов RECON-4);
   - ФАЗА A (стартовый бокс): `for pos in betweenCornersInDirection(box.move(delta.scale(-1.0)), delta)`: `visit(pos, 0)` (false→false), `visited.add(pos.asLong())`;
   - ФАЗА B (DDA): `steps = addCollisionsAlongTravel(visited, delta, box, visitor)`; `<0 → false`;
   - ФАЗА C (конечный бокс): `for pos in betweenCornersInDirection(box, delta)`: если `visited.add(...)`: `visit(pos, steps+1)` (false→false) → true.

`addCollisionsAlongTravel(LongSet visited, Vec3 delta, AABB box, visitor) : int`:
- sizes (dx,dy,dz)=box X/Y/Zsize; `furthest = getFurthestCorner(delta)`; `outer = center + 0.5*size*furthest(per-axis)`; `travel = outer - delta`;
- базовый DDA: `bx,by,bz = floor(travel)`; `sign* = Mth.sign(delta.*)`; `step* = sign==0 ? MAX : sign/delta.*`;
- `tMax* = step* * (sign>0 ? 1.0 - frac(travel.*) : frac(travel.*))`;
- цикл `while (tMaxX<=1 && tMaxY<=1 && tMaxZ<=1)`: шаг минимального tMax (строго <, else-ветки: X<Y→X? else Z; Y<Z→Y else Z — байткод 341-428), затем `Optional<Vec3> hit = AABB.clip(bx,by,bz,bx+1,by+1,bz+1, travel, outer)` — если empty: cell SKIP (visit НЕТ), continue;
- hit есть → `stepIdx++`; `clamp* = Mth.clamp(hit.*, (float)cell + 1.0E-5f, cell+1 - 9.999999747378752E-6)` (float-литерал внутри!); `off* = floor(clamp* - size* * furthest.*)`;
- ФАЗА B-visit: `for pos in betweenCornersInDirection(bx,by,bz, offX,offY,offZ, delta)`: если `visited.add`: `visit(pos, stepIdx)` (false→return -1);
- return stepIdx.

`getFurthestCorner(delta) : Vec3i`: ax=|x|, ay=|y|, az=|z|; sx,sy,sz = (comp>=0 ? 1 : -1); ax<=ay && ax<=az → `(-sx, -sz, -sy)`; ay<=az → `(sz, -sy, -sx)`; else `(-sy, sx, -sz)` (перестановка с инверсией — копировать ВЕРБАТИМ).

`betweenCornersInDirection(AABB, Vec3)`: floor(min/max) → int-версия. `betweenCornersInDirection(x0..z0, x1..z1, delta)`:
- extents d* = |max-min|; start* = (delta.* >= 0 ? min : max);
- `order = Direction.axisStepOrder(delta)`: `(|x| < |z|) ? YZX : YXZ` (Y всегда внешний);
- dirs d* = sign по каждой оси (unit step ±1);
- вызов BlockPos$6(d0,d1,d2, s0,s1,s2, dim2,dim1,dim0) — guava AbstractIterator:
  `computeNext: cursor.set(s0 + d0.x*fI + d1.x*sI + d2.x*tI, s1 + ..., s2 + ...)`; инкремент: tI<i8→tI++; else sI<i7→sI++,tI=0; else fI<i6→fI++,tI=0,sI=0; else end. (f=внешний=Y, s=средний, t=внутренний).

## Где именно orchestration (RECON-4 tail 2900-3400 сэмплов ≈ 2.3-2.7% CPU)
- guava AbstractIterator hasNext/next: 1443 (три фазы через Iterable);
- betweenCornersInDirection bodies: 765; betweenClosed-лямбды: 473;
- LongOpenHashSet add+<init> вне visit: 369; AABB.move/Vec3.scale/subtract/Optional-clip — аллокационная семья (gc-axis: 27.38% young-gen был RECON-3, из них inside-pipeline — значимая доля).

## Дизайн рычага #9 TraverseOps.forEachFlat (Java, entityinside/)
Точная копия алгоритма БЕЗ абстракций: примитивный long[]-маршрут, плоские вложенные циклы (f,s,t) вместо guava-итератора, inline-dedupe на open-addressing long-таблице СОВМЕСТИМОЙ семантики (add-возврат = was-new), inline AABB.clip (ray-box на doubles — вербатим формулы static clip), без Vec3/AABB аллокаций (только локальные doubles/int'ы). Сигнатура ДОЛЖНА совпадать с ванильной для 1:1 call-site ретаргета: `public static boolean forEachFlat(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor)` — visitor остаётся ванильной лямбдой (parity surface = порядок/шаги visit).
Rust-сторона: 6-я стадия entity_compose («traversal») — ретаргет invokestatic BlockGetter.forEachBlockIntersectedBetween → TraverseOps.forEachFlat в Entity.checkInsideBlocks (strict sites=1, по образцу stage batch/inside); env CRUSSTY_FLAT_TRAVERSAL (input flat_traversal); TraverseOps define в kernel loader + BRIDGE_READY + wait_bridge_ready (прецедент batch_collector.rs).
Верификация: (1) LockstepHarness — random (from,to,box) × N тысяч, vanilla vs flat: последовательность (posLong, stepInt) бит-в-бит + return-значение + было-ли короткое замыкание на false-visitor'е; (2) cargo suite (+тест стадии: strict sites=1, композит парсится); (3) javap-контракт — этот документ.

## Preregister гейты S7-163 (объявлены ДО диспатча; банк определяется исходом leg#2 35399980345)
- PG2: 0 NCDFE + pop 150k VALID + «entity_compose: ARMED chain [...->traversal], retransform rc=0» (стадия traversal в цепи ПОСЛЕ batch) + region ARMED + «traverse_ops: defined» + lockstep-харнесс PASS в CI-логе прогона suite.
- PG3 (non-regression TPS): медиана ≥ банка (v2-банк 1.60; если leg#2 забанчит v3 — против медианы v3).
- PG4 (экономика рычага): traversal-лейн (forEachBlockIntersectedBetween family: 9286 сэмплов = 7.27% CPU в leg#5-профиле) снижен ≥50% per-work; orchestration-хвост (guava+BlockPos-итераторы+LongOpenHashSet-init ~2900-3400) ≥ -70%; young GC ≤ банка.
- CRASH-FREE прежние (0 tracker-NPE, 0 uuid-dup, navigatingMobs).
- Банкинг: PASS → CUMULATIVE (+flat_traversal=1); FAIL → REFUTED + rollback flat_traversal=0 (TraverseOps остаётся инфраструктурой).
- ОСТОРОЖНО (из RECON-4): collidedWithFluid/getBlockState (2541 сэмплов под visit-лямбдой) — СЛЕДУЮЩИЙ слой, не трогать в #9; кэш-классы там REFUTED ×3.

## NEXT тик (S7-163 реализация)
1. entityinside/net/minecraft/world/level/TraverseOps.java — порт по этому контракту (вербатим формулы).
2. LockstepHarness.java + build-скрипт (прецедент build_region_tick_ops.sh) — 10k+ random сценариев бит-в-бит (включая degenerate: |delta|≈0 граница 1.0E-5f, sign==0 оси, AABB.clip empty-случаи).
3. src/traversal.rs + стадия в entity_compose.rs + пломбинг flat_traversal (workflow/run_world3.sh/config-echo).
4. cargo suite + preregister-правка в GOAL + диспатч после absorb leg#2.
