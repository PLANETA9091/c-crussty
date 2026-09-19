# RECON-30 — офлайн-верификация broadphase-мемоизации (NEXT 353): exact-AABB кэш мёртв на движущейся сцене, звуковой вариант = layout-диета с потолком ≤3%

Тик TASK-353 (2026-09-20 06:43 +08, Job 398847). Статус: решение владельца по
эскалации A/B/C НЕ поступило → ветка NEXT 353 «без решения»: javap-контракт +
поверхность мутаций + модель hit-rate. Восстановление среды: воркспейс был
откачен к старому снапшоту (репо и /tmp стёрты) — оба репо recloned с GitHub,
тулчейн JDK21 восстановлен в /tmp/jdk21 (Temurin 21.0.5).

## 1. javap-контракт (jar валидной ноги s7194, дампы в contract-broadphase-s7194/)

`ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate)`:

- early-return при `count == 0` (пустая коллекция чанка);
- секционный диапазон: `[Mth.floor(minY - 2.0) >> 4 .. Mth.floor(maxY + 2.0) >> 4]`,
  кламп в `[minSection, maxSection]` (±2 блока вертикального запаса — константа
  контракта, зашита в байткод);
- для каждой секции: `entitiesBySection[sec - minSection]` (BasicEntityList),
  линейный обход `storage[0 .. min(storage.length, size))`;
- на кандидата: skip null → skip `entity == excluded (acmpeq)` → skip
  `!entity.getBoundingBox().intersects(queryAABB)` → skip predicate false →
  `list.add(entity)`;
- **порядок результата детерминирован**: секции по возрастанию Y, внутри
  секции — по индексу storage (median-exact воспроизводим);
- `getEntitiesLimited(...)` — тот же цикл с ранним выходом по лимиту.

Поля-носители: `slices.allEntities`, `hardCollidingEntities`,
`entitiesByClass`/`entitiesByType` (Reference2ObjectOpenHashMap) — каждый со
СВОИМ EntityCollectionBySection (у каждого свой storage/секции).

## 2. Кенсус поверхности мутации (кто и когда инвалидирует кэш)

**Членство** (bump секционных version-счётчиков достаточен):
- `EntityCollectionBySection.addEntity/removeEntity(Entity, int)` — вызывается
  из `ChunkEntitySlices.addEntity/removeEntity` (sectionIdx от Y);
- `ChunkEntitySlices.addEntity/removeEntity(Entity, int)` — from EntityLookup.

**Геометрия** (критично — упущено в RECON-29):
- `Entity.setPos(double,double,double)` → `makeBoundingBox(position)` →
  `setBoundingBox(bb)`: **каждый тик каждого движущегося моба** (X150K = все
  150k мобов имеют движение-AI каждый тик);
- то же через `setPosRaw`/`moveTo`/`absMoveTo`/`refreshDimensions`;
- поле BB приватное в Entity, саб-классы не переопределяют storage — все
  мутации через setBoundingBox (одна точка, но 150k вызовов/тик).

## 3. Модель hit-rate на X150K: exact-AABB мемоизация мертва

Кэш `key = (AABB-биты запроса, excluded, version-снимок затронутых секций)`:

1. **Запросы движутся вместе с сущностями**: collision-scan (hard-colliding
   push) строит AABB от текущего BB сущности → биты меняются каждый тик.
   Targeting/scan-запросы меняют AABB вслед за источником. Стационарных
   повторяющихся AABB в сцене — ничтожная доля.
2. **Даже при идентичном AABB кэш нечестен без BB-версии**: результат зависит
   от ЧУЖИХ BB (intersects). Моб, вошедший в AABB после fill, должен попасть в
   результат — stale-кэш его потеряет (median-exact НАРУШЕН, vanila-парити
   сломана).
3. Честный ключ обязан включать версию геометрии ВСЕХ кандидатов-секций;
   геометрия мутирует 150k раз/тик → версия меняется каждый тик → **hit-rate
   ≈ 0** (кэш живёт <1 тика, стоимость поддержки version-bump на
   setBoundingBox — чистый оверхед на весь тик).

**ВЕРДИКТ: секционно-версионная exact-мемоизация getEntities на сцене X150K
структурно мертва** (0 пользы + отрицательный оверхед). Оговорка RECON-29
«архитектурно законна» верна только для стационарных сцен — на нашей сцене
опровергнута до реализации (пятая док-верификация закрытия лейна после
RECON-17/20/23/26).

## 4. Звуковой вариант = layout-диета (flat live-list), потолок ≤3%

Честная альтернатива — НЕ кэш результата, а диета раскладки: на каждую
EntityCollectionBySection — плоский append-only массив живых ссылок
(инкрементальный append на add, ленивые tombstone-компакции на remove,
membership-version вместо per-query секционного dispatch):

- экономится ТОЛЬКО контур: count-check, расчёт секционного диапазона,
  per-section dispatch, storage.length/size min, null-checks — по RECON-29 это
  ~71-77% лейна (getEntities 3850/4115 против intersects 1125/1080);
- per-candidate `intersects` (23-29% лейна) и predicate ОБЯЗАТЕЛЬНЫ каждый
  вызов (геометрия чужих BB меняется) — не убираются никак;
- потолок = ~60-70% от 4.90/5.14% сцены ≈ **3.0-3.5% сцены** — микро-зона
  чартера, самостоятельный рычаг ЗАПРЕЩЁН; в составе кумулятивной ЭПОХИ-2
  (вариант A) даёт ВКЛАД ≤3% при риске регресса от tombstone-компакций
  (spawn/despawn-чурн сцены).

## 5. Последствия для эскалации владельцу (уточнение варианта A)

- Компонент «broadphase-мемоизация ~5%» в оценке A РЕАЛЬНО СТОИТ ≤3% (layout-
  диета), а не 5%: exact-кэш = 0, intersects/predicate неубираемы.
- Кумулятивный потенциал системной группы пересчитан: **~10-13% сцены** вместо
  12-16% (broadphase ≤3% + volatile ~3.4% + inside-gate ~3.1% + контур ~1.1% +
  chunk-read ~3.6%), и это ВЕРХНЯЯ граница при нулевой TPS-конверсии по
  эмпирике 6 REFUTED.
- Рекомендация усиливается в сторону **B (коррекция протокола: min-MSPT)**
  либо **C (смена уровня: Rust/JNI / off-thread)**; вариант A — только при
  явной санкции владельца на ожидаемый ~0 TPS-эффект.
- Прототип-патчер НЕ реализуется: единственный корректный вариант ниже порога
  чартера (микро-фикс запрещён), мёртвый вариант не кодим. Диспатчи остаются
  остановлены.

## 6. NEXT (id 354)

Решение владельца A/B/C. Без решения — продолжение офлайн-верификации
компонентов A по одному за тик: **volatile-диета ~3.4%** (javap-контракт
RegionTickOps/Entity volatile-полей + модель конверсии в MSPT) — следующий
кандидат на док-закрытие до реализации.
