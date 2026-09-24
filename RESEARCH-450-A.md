# RESEARCH-450-A — расширение коллизионной плоскости ЦЕЛИКОМ (cycle-1 ×450)

Agent-A (TASK-450-A-RESTORE), ветка `round-450a-collide`, носитель = merge
round-449a-collide-1 @acd0f36f (cmp420 ⊕ collide-batch ⊕ queryplane ⊕ eqsnap ⊕
chunk-parse ⊕ sscan/ai/items + NCDFE-фикс ConstSlot 7edda662, javap-LOADABILITY
гейты ×3 скриптов). Lever = `cmp445_collide` (STRICT-OR additive, пустой флаг =
ваниль бит-в-байт). Мандат ≥+20% pair-stable, цикл закона 3.

## 0. ЛАН-ЧИСЛА ТИКА (BOTTLENECK ×450, золотые абсорбы ×449)

| Срез | Ваниль | Под носителями | Статус покрытия |
|---|---|---|---|
| items/ItemEntity-slice | 31.17% wall | → 0.00 на ins4/mega4 (lane-regex; ItemEntityManager.tickOne 30.35% — artifact ×450-B #0) | rest-plane несёт tickBody |
| collision-срез ВНУТРИ items-лейна (×446 лан-сплит: fluid 32/inside 28/**collision 25**) | ~25% × 31.2% ≈ **7.8% wall** | НЕ покрыт (ваниль Entity.move/noCollision + collide-batch section-plan) | цель cycle-1 анализа |
| colpush/broadphase | **15.7-18.1%** | 9.1-9.6 (ins4-семья); на colpush-носителе = bulk-ядро + java-хвост | цель cycle-1 дельты |
| nav_ai | 14.16% | 3.1-4.2 | чужая плоскость |

Банк collide-линии: +10.2 (×448 collide-1), +4.0 (×447 1r2), +4.1 (×449 1).
Для пары ≥+20% дельта должна дать +10-16pp — цикл (3) продолжается витками.

## 1. JAVAP-ЦЕНЗ collision-среза items-лейна (kernel round-445/assets, purpur 1.21.10)

tickBody rest-plane (ItemEntityManager, offsets 140..270) зовёт ваниль:

1. `e.level().noCollision(e, bb.deflate(1.0E-7))` — КАЖДЫЙ item КАЖДЫЙ тик.
   javap Level.noCollision(Entity,AABB): ArmorStand-гейт (paperConfig chain) →
   flags = entity!=null ? **8** : 12 → `!CollisionUtil.getCollisionsForBlocksOrWorldBorder(
   level, entity, box, null, null, 8, null)` → redirect → CollideBatchOps.blockCollisions
   (checkOnly, списки NULL — аллокаций списков НЕТ; на вызов: getChunk-lookup ×1-2,
   section-loop, findSlot, epoch-гейт, LazyEntityCollisionContext alloc, plan walk).
2. `e.move(SELF, delta)` — только НЕ-resting (гейт offsets 226..270: onGround &&
   hdSqr ≤ 9.999999747378752E-6 && (tickCount+id)%4 != 0 → skip). Move-оркестрация
   (RECON-10: 560 юнитов, Vec3 ×11) + collide-сканы через тот же redirect.

**Оценка остатка**: 20-30k items × 1 noCollision/тик × ~0.4-0.8µs (с温план) ≈
8-20ms/тик ≈ 2.5-6% wall — совпадает с ланом 7.8%. Лестница (0.37% CPU, RECON-10)
и сбор секций уже под collide-batch; остаток = per-query скаффолдинг.

## 2. ТЕХНИКА-ОБЗОР (мандат: collision-sweep батчем, contact-diet, AABB-сорт+прунь)

| Техника | Анализ числами | Вердикт |
|---|---|---|
| **collision-sweep батчем (item noPhysics pre-pass по бакету)** | перестройка tickBucket: прe-проход собирает боксы, батч-вычисление noPhysics, потом tick-проход. Дивергенция: блок может измениться МЕЖДУ pre-pass и tickBody item i в том же проходе (creeper-explode в aiStep, FallingBlock, TNT) — ванильный ответ увидел бы кратер. Без ревалидации = parity-риск; с ревалидацией = та же стоимость. Амортизация (списки NULL и так, context ~30ns) потолок ~1ms/тик < шум 2-3% | **ОТКЛОНИТЬ** (parity + ниже шума) |
| **memo ответа noCollision для resting items** | box не меняется тик-к-тику, но инвалидация требует block-change трекинга = RECON-42-класс механик (запрещён владельцем, закон 5) | **ЗАПРЕЩЕНО законом 5** |
| **ghost-candidate diet (дожим java-ревалидации CSR)** | ревалидация живыми боксами обязана видеть СУПЕРМНОЖЕСТВО: live-боксы могут СБЛИЗИТЬСЯ (снапшот tick-1). Урезать кандидатов по снапшот-зазору нельзя; порядок хвоста = стена S7-140 | **ОТКЛОНИТЬ** (superset-требование) |
| **contact-diet: x-window по сорту бакета** | необходимые условия точного теста: \|cx_a−cx_b\| < hx_a+hx_b ≤ ra+bmaxh[slot]. Бакет-сегмент, отсортированный по cx → бинарный поиск окна → перебор ТОЛЬКО окна вместо всей цепи. В плотном кластере (440-1200 рядов/бакет вокруг 4 fake-players) окно ≈ 8-блочная x-полоса | **ИМПЛЕМЕНТИРОВАТЬ** |
| **contact-diet: y-band бакета** | то же по y: \|cy_a−cy_b\| < hh_a+hh_b ≤ hha+bmaxhh[slot]; бакеты с y-разносом (поверхность/пещеры/крыши) отсекаются ЦЕЛИКОМ до бинарного поиска | **ИМПЛЕМЕНТИРОВАТЬ** |
| **AABB-сорт+прунь (broadphase)** | подсчётная сортировка по бакету O(n+16K) + сортировка сегмента по (cx,id) (total_cmp, детерминизм) + окно. Цепи bhead/next уходят: O(9·m) на ряд → O(9·(log m + window)). CSR-выход бит-в-байт (per-row sort по id остаётся; оракул O(N²) equality) | **ИМПЛЕМЕНТИРОВАТЬ** |
| SAP по всем осям / quadtree / hash cell 2.0/8.0 | ×448-CYCLE2 уже отклонены (сортировка id-требования, константы, walk-цена) | НЕ реанимировать |

## 3. ДЕЛЬТА CYCLE-1 (rust-only, src/colpush.rs, ветка colpushTick2)

1. **BulkState v3**: +`brows` (IDS_CAP i32 — бакет-группировка), +`bstart/bcnt`
   (BUCKET_CAP i32), +`bminy/bmaxy/bmaxhh` (BUCKET_CAP f64). Сбросы безусловные
   (ERR_RANGE-ретрай идемпотентен, канон range_retry_is_idempotent).
2. **PASS 1**: вместо цепей — счёт бакетов + AABB-копилка (xz + **y-полоса** +
   bmaxhh); PASS 1.5: префикс-сумма bstart, раскладка brows, per-segment sort по
   (cx: total_cmp, id) — детерминизм при равных cx.
3. **FUSED walk**: на ряд: 9 соседей (dedup) → empty/bucket-xz-prune/**y-band-prune**
   → бинарный поиск x-окна [cx−mh, cx+mh] (mh = ra+bmaxh+EPS) → точный strict-<
   тест (x→z→y, бит-в-байт с ×420/×448) на строках окна. per-row сортировка CSR
   по id — без изменений. Копия out = buckets_copy_out без изменений.
4. **Маркер**: EFFECT-строка colpushTick2 расширяется тегом `sorted-windows=1`
   (grep-канон абсорба видит свежий ×450 маркер ДО purge).
5. **Оракул**: +тяжёлые плотные сцены (сотни рядов в 4×4/8×8 кубах, y-слои,
   граничные касания) — equality brute-force обязана держаться бит-в-байт.

Парити-стены НЕ трогаются: java-блобы НЕ пересобираются (в classpath ноль
правок — check_blobs_sync обязан пройти как есть), порядок push (S7-140),
CSR-семантика ×420. ОДИН bulk-JNI/тик/поток сохранён (colpushTick2).

## 4. ЧЕСТНЫЙ ПОТЕНЦИАЛ

- sorted windows + y-band: dense-cluster chain-visit'ы 300M+/тик → окно+log;
  оценка −2-4ms main-потока при MSPT 300-450 ⇒ **+0.5-1.2% TPS** поверх банка
  +4-10. НЕ закрывает +20% соло — цикл (3) продолжается витком-2
  (кандидаты: push-хвост diet в рамках wall-допусков, quad-slice плоскость
  noCollision item'ов через rust-снапшот секций, композит с ins4-носителем
  — закон 7, мега-композиция).
- Подтверждение эффекта: lane broadphase в лей-профиле ноги + EFFECT-маркер
  + pair vs свежие якоря round-450-anchor-*.
