# RESEARCH-445-A — COLLIDE+BROADPHASE+PUSH плоскость (cmp445_collide)

Agent-A, round-445, ветка `round-445-a-collide` @ master 8eded686.
Мандат: ≥+20% pair-stable (закон 6 R1 — ПОДСИСТЕМА ЦЕЛИКОМ, один bulk-JNI/тик).

## 1. JAVAP-ЦЕНЗ hot path (kernel jar round-396-a/patched-kernel.jar, purpur 1.21.10)

Сайты подтверждены javap -c на живом ядре:

| Сайт | Сигнатура/вызов | Роль |
|---|---|---|
| `LivingEntity.pushEntities` | → `Level.getPushableEntities(Entity, AABB)` (offset 75 invokevirtual) | per-entity push-broadphase fetch, КАЖДЫЙ aiStep |
| `Entity.move` | → `Level.noCollision(Entity, AABB)` (offset 6 invokevirtual в move-гейте) | block+hard-entity коллизия на каждый мув |
| `CollisionUtil.getCollisionsForBlocksOrWorldBorder` | static scan, CB_SCAN_DESC | block-collision перечисление секций |
| `Level.getEntitiesOfClass(Class, AABB, Predicate)` | final generic | findTarget/Avoid-семейство |
| `RegionTickOps.forEach` | `COLPUSH_ON` volatile gate → `ColpushOps.bulkTick()` | ОДИН bulk-JNI/тик триггер (main, до GO-барьера) |

## 2. ДОКАЗАННОЕ ДОМИНИРОВАНИЕ ЛЕЙНА (свежий якорь ×444, round-round-442-anchor-10, 117 264 wall-сэмпла)

| Семейство | % wall |
|---|---|
| query engine (ChunkEntitySlices/$EntityCollectionBySection.getEntities) | **7.90** |
| block collide (CollisionUtil/BlockCollisions) | **7.75** |
| push lane (pushEntities/getPushableEntities/doPush) | **4.64** |
| findTarget/TargetingConditions/Avoid | **3.35** |
| noCollision | **3.33** |
| AABB.intersects/inflate/expandTowards | **2.99** |
| hard-colliding (EntityLookup) | **1.73** |
| Sensing.tick | 0.46 |
| **UNION (с перекрытиями)** | **32.15** |

Перекрытия честны: getPushableEntities сидит ВНУТРИ query-engine стека; даже при
консервативном разбореперекрытий ~19-24% wall обслуживаются сертифицированной
колпаш-плоскостью (CSR push) + collide-batch (block-план) + queryplane (2-phase
query). При MSPT 397ms/tick (насыщенный main) снятые X% wall ≈ X% TPS-выигрыша.

## 3. ЭРА-ИСТОРИЯ (worklog)

- **cmp420_colpush мега (4ab7306, ×420)**: колпаш-носитель = colpush-fix ⊕
  chunk2 ⊕ полный carrier stack; ноги mga/mgc/mgd = **+25.0/+31.0/+29.5 pair**
  — сильнейшая сертифицированная пара эры. Механизм: pushEntities whole-body
  redirect → ColpushOps (буферы COL_D/COL_I пишутся per-entity простыми
  store, 0 JNI; ОДИН bulk colpushTick/тик строит uniform-grid broadphase
  (cell 4.0, pad 1) + точный AABB-overlap + CSR кандидатов; java-хвост
  ваниль бит-в-байт). SoA-семья (per-entity upsert-конвой/глобальный мьютекс)
  МЕРТВА (×427) — в носителе спит (colpush_plane_refresh кормит колонки
  одним WLOCK), НЕ возрождается.
- С ×420 колпаш-носитель НИ РАЗУ не ехал в CI-ноги: все победители
  (ins4d/ins4/chunk4/sense/mega) сидели на чужих лейнах. Лейн на якорях
  ×444 = по-прежнему ваниль (32% union).
- Соседние носители НЕ трогаются: inside_snap (ins4-вектор), sscan2-полка
  (despawn+spawn — в колпаш-носителе segment-срез уже сертифицирован),
  queryplane merged (Level-сайты).

## 4. ДИЗАЙН cmp445_collide (STRICT-OR добавки, урок ×438)

1. **CERTIFIED CARRIER BASE**: cmp445_collide добавляется В ХВОСТЫ рядом с
   каждым `cmp420_colpush` (rust 17 файлов, java 8 мостов) — композиция ноги
   = сертифицированный колпаш-мега-носитель ×420 (+29.5 pair) поверх
   сегодняшнего master. Ничего не удаляется/не заменяется: пустой/чужой флаг
   = ваниль бит-в-байт, прежние флаги несут прежние точные хвосты.
2. **НОВЫЙ ДЕЛЬТА-МЕХАНИЗМ (моя подсистема)**: `colpushTick2` — тот же
   контракт (сигнатура/CSR/semantics), что сертифицированный `colpushTick`,
   + wide-phase prune по чанковым бакетам: активные ряды партиционируются в
   16.0-бакеты (chunk-ключи), по каждому бакету строится AABB его рядов
   (min/max, расширенный на 2×RADIUS_GATE=4.0); пара (бакет A, бакет B)
   перечисляется ТОЛЬКО если их swept-AABB пересекаются — консервативный
   early-out: отсечённые бакет-пары не могут дать ни одной пары-кандидата
   (доказывается: reach пары ≤ hx_i+hx_j ≤ 4.0 ⊆ expansion). CSR ВЫХОД
   БИТ-В-БАЙТ равен colpushTick (oracle в cargo test: brute-force O(N²)
   AABB-энумерация == CSR обоих натативов на рандомизированных сценах).
   Развилка НАТИВА — rust-стороне (OnceLock флага): java не меняет сигнатуру,
   вызывает colpushTick как раньше.
3. **tick-stamped кэш**: активный список = fresh == tick-1 (end-of-previous-
   tick снапшот, сертифицированный ghost-контракт ≤1 тик) — без изменений.
4. **ОДИН bulk-JNI/тик**: colpushTick/colpushTick2 (общая точка bulkTickImpl)
   — collide_batch (0 JNI) и queryplane (0 JNI)JNI не добавляют. Per-entity
   JNI = дизайн-ошибка (закон 6) — не появляется.

## 5. ГЕЙТЫ/МАРКЕРЫ

- STRICT-OR хвосты: `cmp420_colpush || cmp445_collide` (+ прежние) —
  существующие хвосты не ломаются (урок ×438), альтернативы ДОБАВЛЯЮТСЯ.
- java-гейты двойные где применимо: ColpushOps (FLAG4) + ChunkParseOps
  (CARRIER_UNION) + MobPushOps/MobAiOps/MobScanOps/EntityGoalQueryOps/
  ItemEntityManager/QueryPlaneOps.
- Маркер-протокол: selfTest=true ДО ARM (fail-closed) → ARMED → EFFECT
  (first hit tick N); мой семейный маркер `cmp445_collide: carrier stack
  ARMED ...` печатает новый модуль src/collide445.rs (регистрация/порядок
  не меняются — colpush остаётся последним хуком LivingEntity).
- check_blobs_sync: gate-flag consistency loop автоматически требует
  cmp445_collide в cp каждого отредактированного блоба (raw-byte grep);
  check_class-списки ColpushOps/MobPushOps дополняются маркером.

## 6. ПОТЕНЦИАЛ (честно)

- Сертифицированный носитель: pair +25..+31 на базе ×420. Лейн на ×444 не
  изменился (32% union wall). Ожидание: +18..+28 pair — бар ≥+20% достижим,
  но в нижней зоне ⇒ нужен min-of-3 и оба плеча окна.
- Мой дельта-механизм (bucket prune) — экономия CPU rust-bulk тика
  (кластерные сцены: mob-cap кольца вокруг 4 fake-players ⇒ пустые
  бакет-пары), а НЕ замена носителя.
