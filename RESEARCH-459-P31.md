# RESEARCH-459-P31 — ID-P31 INSIDE-BATCH: discovery тика одним bulk-JNI

TASK-459-56, WILD-агент закона 11 тика-459 (v18.2). Карточка: RESEARCH-458-P.md §ПАКЕТ-3
`[ID-P31]`. Лейн-носитель: **inside_volatile 16.6пп — TOP-1 остаток компо-носителя chunkmono**
(абсолютный рост +25% на обеих ногах chkmono457-11/12, invalidation-driven гипотеза B:
secWrite-вампы ген-эпох inside_snap → ванильный volatile MISS-путь чаще, CHM.get кормит
java_util 9% РОСТ).

## 1. МЕХАНИКА (что режем)

Ванильный discovery: `Entity.checkInsideBlocks(List<Movement>, StepBasedCollector)` —
swept-обход `BlockGetter.forEachBlockIntersectedBetween(from, to, box.deflate(9.999999747378752E-6))`;
на каждую visit-позицию `getBlockState` (PalettedContainer.get, top-1 kernel CPU) +
`getEntityInsideCollisionShape` + `collidedWithFluid`. Две traversal за тик (main from-to +
финальная to-to с бюджетом 1). Ключевое наблюдение карточки: подавляющая часть позиций visit
падает в **заведомо пустые/не-влияющие секции** (air-only 16³, вне swept-бокса, без collision
и fluid) — всю их цену можно снять ОДНИМ батч-решением на тик вместо per-position
volatile-чтений и CHM-промахов inside_snap.

План-форма (закон 6 — подсистема целиком: gate+discovery+tail):

1. java-мост собирает батч ВСЕХ checkInsideBlocks-кандидатов тика:
   `(eid, x/y/z, bb-флет 6×f64, section-ключи 16³-секций, пересекаемых swept-боксом)`;
2. **ОДИН JNI** (`insideBatchMask`) — плоские SoA-массивы, ноль per-entity переходов
   (дисциплина nav_plane/navDecide + colpushTick: цена JNI-перехода амортизируется батчем);
3. Rust возвращает **битмаску секций-кандидатов per-entity (superset)**: отсекает заведомо
   пустые (non-air-count == 0, Moonrise `BlockCountingChunkSection`) и не пересекающие
   swept-бокс секции; НЕ-кандидат ≠ эффект-пусто → любой сомнительный бит ставится (fail-open
   на superset, fail-closed на parity);
4. java гонит **строгий ванильный хвост** visit-обхода ТОЛЬКО по секциям-кандидатам
   (порядок вызовов, шаги, advanceStep/entityInside/onInsideBlock — бит-в-байт ванили);
5. dirty-list секций-мутантов между сборкой батча и тиком (secWrite-бампы уже есть в
   inside_snap — паттерн entity_index.rs): мутировавшая секция ⇒ её бит форс-единичный.

## 2. JAVA-САЙТЫ

- `entityinside/net/minecraft/world/entity/InsideBlockOps.java` — гейт-паттерн
  (retarget `isAffectedByBlocks` @ offset 1 в `checkInsideBlocks` → `gate(Entity)Z`,
  3B→3B receiver-first). Новый мост-сиблинг: `entityinside/net/minecraft/world/entity/InsideBatchOps.java`
  (`batchGate(Entity)Z` + RegisterNatives `insideBatchMask`), scaffold положен в этом коммите.
- `src/classfile.rs` (Entity stage): `patch_inside_batch` — тот же единственный сайт,
  цель `(Entity,isAffectedByBlocks,()Z)` → `(InsideBatchOps,batchGate,(LEntity;)Z)`;
  владелец сайта ОДИН (S7-162 supersede-дисциплина entity_compose): armed inside_batch
  вытесняет inside_cache на этом сайте, иначе внутрикэш-хвост.
- `src/inside_batch.rs` (новый) — структуры батча, superset-ядро, JNI-вход; `src/inside_cache.rs`
  остаётся владельцем bridge-канала (define в kernel loader).
- NCDFE-канон: java Ops define в раннем arm-хуке (паттерн d73758a3/5ecd841a — NCDFE 2924/24158
  @ MobPushOps.pushables:467 в DELIVERY-FAIL chkmono456-1/2; fa9054d9 canon ARM-AFTER-DEFINE);
  lever dormant: флаг не стоит ⇒ класс не определяется, сайт не компонуется, ваниль бит-в-байт.

## 3. PARITY-ПЛАН (candidate-superset → strict tail бит-в-байт)

- Маска = **superset**: false-positive разрешён (лишний кандидат — просто ванильный хвост по
  нему), false-negative ЗАПРЕЩЁН (потерянный кандидат = потерянный effect). Неизвестная
  секция/переполнение CSR/ошибка структури ⇒ маска all-ones ⇒ чистая ваниль.
- Хвост строгий: visit-порядок, step-бюджет 16, intersected-флаги, fluid-ветка после block —
  транскрипция 1:1 c javap-контрактом InsideBlockOps.Recorder (visit 140-315);
  исполнитель эффектов — существующий ванильный вызыватель (applyAndClear не трогаем).
- Оракул: entityinside/harness расширяется lockstep-сценой (ваниль vs батч-хвост, бит-в-байт
  сравнение finalEffects; 10k+ сцен до STRICT-on).
- Бит-инвариант самтеста: `mask ⊇ vanillaVisitSections` на оракул-сценах (count-инвариант).

## 4. ПРОГНОЗ Δ (числом)

- Захват 30-50% discovery-части лейна 16.6пп ⇒ **+5-8пп к ноге chunkmono**
  (ноги +11.5/+9.1 → 16-19.5пп; банк якорей a15 +9.7@6813110 / a26 +12.4@8671791, Δ≤50k
  ⇒ пара ≥+20 — самый прямой путь волны). Проверка прогноза — канон-инпуты диспатчера
  сессии-B (cpu band 6.0-9.5M, population 150000, radius 640, 300s).

## 5. РИСКИ

1. Секции-мутанты между сборкой батча и хвостом ⇒ dirty-list обязателен (secWrite-бампы),
   иначе lost-effect — главный parity-риск.
2. Не-ванильные размеры/формы сущностей (моды) — прецеденты lithium: Pehkui-краш #125 и
   Origins-фазинг #336 ⇒ superset обязан считать фактический bb; неизвестная геометрия ⇒
   all-ones + ванильный хвост.
3. NCDFE: мост должен быть определён ДО первого retarget-вызова (early arm-hook define,
   паттерн d73758a3/5ecd841a); констант-пул ref резолвится только во взятой ветке.
4. JNI-батч переполнение (n > лимит) ⇒ ERR-код ⇒ disarm-латч batchOk ⇒ чистая ваниль
   (Err-ladder mobs_soa дисциплина).
5. inside_bitmask (закон 5) сознательно НЕ используется — запрещённый лейн, не воскрешаем.

## 6. SOURCES (web, проверены 2026-09-25)

1. https://github.com/CaffeineMC/lithium/issues/125 — «Entity suffocation optimizations in
   Lithium 0.5.3 are incompatible with Pehkui» (HTTP 200, title verified): lithium-подход к
   inside-block discovery (кэш-гейт suffocation-слоя) и его parity-ловушка с модифицированными
   размерами сущностей — обоснование superset+strict-tail вместо полной замены discovery.
2. https://github.com/CaffeineMC/lithium/issues/336 — «Suffocation Optimization incompatible
   with origins mod» (lithium 1.18.1, 2022-01-05): второй parity-прецедент (не-ванильные
   формы/фазинг) ⇒ fallback-требование для нестандартных bb.
3. https://bugs.mojang.com/browse/MC/issues/MC-260743 — ванильный тикет по внутри-блочной
   discovery-дорожке (карточка P31; HTTP 200).
4. https://modrinth.com/mod/c2me-fabric — C2ME батч-дисциплина (батч как единица планирования;
   референс формы «один батч — один нативный вызов»).
