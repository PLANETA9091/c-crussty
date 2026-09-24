# RESEARCH-448-A-CYCLE2 — collide-подсистема, виток-2 (cmp445_collide)

Agent-A, round-448 cycle-2, база `round-445-a-collide` @ 18f37d57 (step2).
Мандат: ≥+20% pair-stable (закон 3: Δ<+20% → research → имплементация → бенч).

## 0. ЧТО УЖЕ ЗАКРЕПЛЕНО НОСИТЕЛЕМ (не дублируется)

Carrier stack @18f37d57 = cmp420 ⊕ collide-batch ⊕ queryplane ⊕ eqsnap ⊕
chunk-parse ⊕ sscan/ai/items (ARM-чисто на leg round-447a-collide-1r2 +4.0).

| Vanilla-лейн (anchor-10, 117k samples) | % wall | Покрытие на носителе |
|---|---|---|
| query engine (ChunkEntitySlices.getEntities) | 7.90 | colpush CSR (push-fetch) + queryplane (Player-targeting, hard-colliding empty fast-path) |
| block collide (CollisionUtil/BlockCollisions) | 7.75 | collide-batch section-plan (per-tick per-worker dedup block-reads) |
| push lane (pushEntities/getPushableEntities/doPush) | 4.64 | colpush CSR: getPushableEntities снят; хвост = ваниль (парити-стена S7-140) |
| findTarget/Avoid | 3.35 | queryplane site 1 |
| noCollision | 3.33 | queryplane site 2 (entity-половина) + collide-batch (block-половина) |
| AABB.intersects/inflate/expandTowards | 2.99 | частично (revalidation живых боксов — обязана остаться) |
| hard-colliding (EntityLookup) | 1.73 | queryplane empty fast-path |
| Sensing.tick | 0.46 | вне скоупа |

## 1. JAVAP-ЦЕНЗ остаточного хвоста (kernel jar round-396-a, purpur 1.21.10)

Сайты, остающиеся на покрытом пути ПОСЛЕ носителя (проверено javap -c):

1. **`GameRules.getInt(Key)`** — вызывается на КАЖДОГО covered entity в
   pushEntities (`getGameRules().getInt(RULE_MAX_ENTITY_CRAMMING)`):
   getRule → map-lookup + IntegerValue.get — ~15-25ns × 44-48k/тик.
   КОНСТАНТА ВНУТРИ ТИКА (gamerules не меняются mid-tick: команды исполняются
   main-потоком вне фазы region-tick'ов ⇒ значение, прочитанное pre-GO
   bulkTick'ом, == значению любой per-entity чтения этого тика).
2. **`MinecraftServer.getServer().getTickCount()`** — static-цепочка ×2 на
   covered entity (fresh-стор + никак) — константа внутри тика.
3. **`Entity.push(Entity)`** (javap offsets 0..198): целиком СКАЛЯРНЫЙ
   (0 аллокаций Vec3 — гипотеза опровергнута цензом), но тянет
   `paperConfig().collisions.onlyPlayersCollide` (2 getfield по config-цепи)
   на КАЖДУЮ пару. Парити-зона (порядок/тайминг push наблюдаемы) —
   редирект тела возможен только бит-в-байт; выигрыш = config-цепь
   ~0.1-0.2% — РИСК/ВЫГОДА НЕ ЦЕЛЯ (не имплементируем в cycle-2).
4. **`paperConfig().collisions.maxEntityCollisions`** — читается только при
   cramming ≤ 0 (vanilla short-circuit iload_2 ifgt 66; в бенче cramming
   default > 0) — НЕ горячая.

## 2. ТЕХНИКА-ОБЗОР broadphase (интернет-ценз + локальная математика)

Источники: классика broad-phase (NVIDIA CUDA broad-phase chapter, gamedev
SAP-vs-quadtree консенсус, spatial-hashing литература) + собственная модель
сцены (44-48k активных LivingEntity, reach пары ≤ hx_a+hx_b ≤ 4.0,
RADIUS_GATE 2.0, кластеризация вокруг 4 fake-players).

| Техника | Оценка для сцены | Вердикт |
|---|---|---|
| Uniform grid cell=4.0 pad1 (×420, сертифицирован) | O(n) build/тик, 9 chain-walk/row | база; менять нельзя (бит-в-байт CSR) |
| Spatial hash крупнее (cell 8.0) | 4× рядов на cell → 4× точных тестов при том же 9-walk | ОТКЛОНИТЬ (плотнее = хуже) |
| Spatial hash мельче (cell 2.0) | окно ±3.0 < reach 4.0 → нужен 5×5 walk (25 hash/row) ради 4× меньших цепей | нейтрально-минус (hash-аллокация дороже отсечённых тестов) |
| Sweep-and-prune (оси) | O(n log n) сортировка 44-48k/тик; выигрыш только при высокой когерентности; списки кандидатов затем СОРТИРУЮТСЯ по id (требование CSR) | ОТКЛОНИТЬ: сортировка съедает выигрыш, бит-в-байт порядок требует post-sort всё равно |
| Quadtree/R-tree | логарифмические указательные структуры, worse constants при uniform-плотности + rebuild/тик | ОТКЛОНИТЬ (канонический результат) |
| Time-slicing коллизий по бакетам (группа i — тик i%K) | меняет семантику (реже push-события) = дивергенция траекторий 150k мобов = стена S7-140 | ЗАПРЕЩЕНО законом 4 (не парити-класс) |
| Инкрементальные dirty-сеты (пере-вставка только сдвинувшихся) | мобы бенча ходят каждый тик → dirty ≈ 100% активных; экономия build ≈ 0; усложнение идемпотентности ERR_RANGE-retry | ОТКЛОНИТЬ числом (fixture-специфично) |
| **Fused single-pass scan+fill** (главный cycle-2 дельта-механизм) | сейчас PASS2 (степени) и PASS4 (заполнение) ДВАЖДЫ проходят 9 бакетов и ДВАЖДЫ делают точный AABB-тест каждой пары-кандидата; слияние = −1 полный grid-walk и −50% точных тестов; выход бит-в-байт (сортировка per-row в scratch → memcpy) | **ИМПЛЕМЕНТИРОВАТЬ** (bulk идёт main-потоком pre-GO ⇒ экономия 1:1 в MSPT) |
| Per-(level,tick) constants plane (java-хвост) |getInt+getTickCount → 1 volatile-check; ключ (level,tick) — парити-экзакт по построению | **ИМПЛЕМЕНТИРОВАТЬ** |

## 3. ПОТЕНЦИАЛ (честно)

- Fused single-pass: pair-slots на leg ×447 = сотни тысяч (EFFECT-маркер
  first tick rows≈44k); двойной точный тест + двойной walk = ~2-4ms/тик
  main-потока при 300-400ms MSPT ⇒ ~0.7-1.2% TPS.
- Constants plane: 44-48k × (map-lookup 20ns + static-цепочка 10ns) ≈
  1.3-1.4ms/тик ⇒ ~0.3-0.4% TPS.
- Совокупный delta vs step2: ~+1-1.5% TPS. Мандат ≥+20% закрывается
  СЕРТИФИЦИРОВАННЫМ НОСИТЕЛЕМ (пары ×420: +25.0/+31.0/+29.5) при
  парной-валидации vs свежие якоря (round-448-anchor-1/2/3) — дельта
  cycle-2 повышает шансы пары через банку norm и держит STRICT-OR
  соседей нетронутыми.
- Парити-стены НЕ трогаются: порядок push (S7-140), time-slicing,
  reorder кандидатов (CSR уже sorted-by-id), Entity.push тело (ценз §1.3).

## 4. ДИЗАЙН ДЕЛЬТЫ cycle-2 (additive STRICT-OR, cmp445_collide)

1. **Rust `colpush_tick_buckets` → fused**: один проход по активным рядам:
   9-бакетный walk с per-(row,bucket) reach-prune, точный тест ОДИН раз,
   кандидаты пишутся в grow-once `st.csr` (bit-в-байт порядок: insertion-sort
   per-row в scratch), префикс-офсеты в `st.off_scratch`, total = cursor;
   ids_cap-гейт ДО записи в java (ERR_RANGE-ретрай идемпотентен как раньше —
   bhead/bAABB очищаются безусловно, csr/act пересобираются с нуля);
   затем один memcpy scratch→java-ids + off-заполнение (неактивные ряды —
   нулевые длины, бит-в-байт с ×420). PASS5 plane-refresh без изменений.
   Классическое тело colpushTick (cmp420_colpush) НЕ ТРОГАЕТСЯ.
2. **Java `ColpushOps`: per-(level,tick) constants plane** — bulkTickImpl
   (main, pre-GO, тот же тик) пре-популейлит слот: volatile
   {Level, tick, cramming}; pushEntitiesImpl: если (level,tick) совпали —
   читает cached cramming/tick; иначе пересчитывает ТОЧНО как ваниль и
   перезаписывает слот (worker-гонка = повторный идемпотентный recompute,
   значения детерминированы в тике). Пустой флаг = класс не определяется.
3. **Оракул**: cargo test расширяется сценой fused-vs-classic-vs-bruteforce
   (рандомизированные популяции, граничные касания strict-<, y-разнос,
   stale/flag-off/NaN, ERR_RANGE-идемпотентность).
4. **Маркеры**: EFFECT-строка первого bulk дополняется тегом `fused-single-pass`
   (grep-канон абсорба видит и прежний префикс `cmp445_collide: bulk EFFECT
   armed` — совместимо).

## 5. ГЕЙТЫ ЧИСТОТЫ (закон 5 / соседние STRICT-OR)

- Запреты cycle: ZGC/alloc_diet/zero_alloc/flat_traversal/fluid-*/inside_bitmask/
  THP/RECON-42/players-16/GC-лотерея/travel_diet/concurrency-гварды/gsel — не затронуты.
- Соседние плоскости (items/inside/sscan/ai/eqsnap/chunkparse/queryplane/
  collide_batch) — код не менялся; mobs_soa plane-refresh (PASS5) — бит-в-байт.
- Пустой/чужой CRUSSTY_LEVER_FLAG = ваниль бит-в-байт (bridge не определяется).
