# ENTITY-QUERY ЛЕЙН — STEP-0 КОНТРАКТ + ПАРИТИ-РАЗБОРКА (S7-140, 2026-09-18)

База: census run 35275967738 (X150K, все рычаги off), ядро /tmp/kmat purpur-1.21.10
(29386794B байт-в-бит), javap -p -c офлайн. CPU 56113 self-сэмплов.

## 1. Расклад непокрытого (после учёта готовых рычагов: DEMUX / INSIDE-CACHE / FLUSH-DIET / FLUID-FREE / ALLOC-DIET)

| лейн | CPU self | статус |
|---|---|---|
| ChunkEntitySlices$EntityCollectionBySection.getEntities 784 + ChunkEntitySlices.getEntities 599 | 2.5% | **НЕ ПОКРЫТ — разобран ниже** |
| AABB.intersects 847 | 1.5% | доминантно внутри getEntities-скана + CollisionUtil |
| CollisionUtil.getCollisionsForBlocksOrWorldBorder 510 | 0.9% | блок-часть: закрыт (см. §4) |
| SynchedEntityData.getValue 615 + getItem 428 + VarHandle 605 | 3.0% | закрыт (см. §5) |
| GC G1 суммарно | ~30% | бьётся диспатчами ALLOC-DIET + INSIDE-CACHE (churn ↓ → барьеры/сканы ↓); прямого законного рычага нет (JVM-флаги запрещены) |

## 2. Callers entity-query (javap-ценз живого ядра)

- **LivingEntity.pushEntities()** (protected, вызов из LivingEntity.aiStep offset 850):
  - гварды: isPushable() → team collision rule != NEVER → RULE_MAX_ENTITY_CRAMMING/maxEntityCollisions > 0
  - ЕДИНСТВЕННЫЙ тяжёлый вызов: `level.getPushableEntities(this, getBoundingBox())`
    = `getEntities(entity, box, EntitySelector.pushableBy(entity))` → moonrise
    ChunkEntitySlices по секциям AABB + intersects + predicate; возврат java.util.List
    (одноразовый: итерируется в цикле push + cramming count, не хранится).
  - 50k LivingEntity × каждый тик = 50k query/тик.
- **ItemEntity.mergeWithNeighbours()** (ItemEntity.tick offset 471):
  - ГЕРДЫ: `int i = movedCrossBlock ? 2 : 40; tickCount % i == 0 && !isClientSide && isMergable()`
    — movedCrossBlock = floor(pos) != floor(prevPos). Предметы спавн-волн СИНХРОННЫ
    по tickCount ⇒ каждые 40 тиков (или 2 у движущихся) — thundering herd: ~100k сканов
    в ОДИН тик (периодические MSPT-пики каждые 2с). Скан: getEntitiesOfClass(ItemEntity,
    box.inflate(itemMerge, [merge-0.5|0], itemMerge), lambda) + итерация/tryToMerge.
- **Mob.aiStep looting-скан** (offset 106): при canPickUpLoot() && alive && !dead &&
  mobGriefing: `getEntitiesOfClass(ItemEntity, getBoundingBox().inflate(reach), ...)` ;
  AABB.inflate-аллокация каждый тик каждого пикапера (census 289 сэмплов = 3.9% churn
  по этой единственной ветке).
- AI goals (LookAtPlayerGoal/AvoidEntityGoal/...) — периодические getEntitiesOfClass,
  мелкая доля.

## 3. ПАРИТИ-СТЕНА (почему кэши/индексы соседей отвергнуты)

1. **Порядок кандидатов значим**: выходной List питает цикл push (кто первый — тот
   толкнул первым) и cramming count; ванильный порядок = порядок EntityCollectionBySection
   (порядок добавления в секцию). Любой реордеринг (bucketing/R-tree обход) меняет
   наблюдаемую последовательность push'ей ⇒ НЕ median-exact.
2. **Порядок-сохраняющий суб-индекс** («только pushable в порядке добавления»):
   predicate pushableBy(this) — ПАРНЫЙ (Team collision rules: allied/never/push-own),
   зависит от (this, candidate), а не от кандидата одного ⇒ суб-индекс по флагу
   кандидата консистентен только при team==null/общих правилах; инвалидация по
   смене team/пассажира/умирания — события без единого хука. Отвергнуто.
3. **Event-driven кэш «соседей в AABB»**: сущности двигаются каждый тик (position
   меняет пересечения AABB, не обязательно секцию) ⇒ gen по секционным мутациям НЕ
   ловит движение ⇒ кэш мёртв всегда (устаревает за тик). Отвергнуто.
4. **Остаточный честный ход**: порядок-сохраняющий intersects-прескрин с сортировкой
   совпадений по исходному индексу (O(m log m) поверх) — экономит только
   intersects-фазу непересекающихся, добавляет sort-work; оценка нетто-выигрыша
   < 1% CPU. МИКРО-класс — ЗАПРЕЩЁН директивой (не 2-3%).
5. **Thundering-herd размазывание item-merge** (менять `% i` фазу): меняет ТИМИНГ
   merge ⇒ наблюдаемое поведение. ЗАПРЕЩЕНО (parity).

ВЕРДИКТ по лейну: архитектурного ×150000-класса рычага с median-exact parity
НЕ СУЩЕСТВУЕТ на данном ядре. Лейн закрывается как «парити-заблокирован»;
переоткрытие возможно только с новым moonrise-субстратом (изменение ядра).

## 4. Блок-коллизии — закрыт

CollisionUtil.getCollisionsForBlocksOrWorldBorder УЖЕ имеет: hasOnlyAir() skip
секции (offset 417), moonrise$hasSpecialCollidingBlocks спец-путь (428),
CollisionBlockState.moonrise$emptyContextCollisionShape отсечки форм (751).
Идея «COLLISION-FREE-SECTION» (сестра FLUID-FREE: секция без коллайдеров по
палитре) добирает только над-полные травяные секции: оценка ≤0.6% CPU — микро,
ЗАПРЕЩЕНО. После DEMUX внутрян PalettedContainer.get ускорена; остаток self
(510) = floor-математика/чанк-фетчи/шапки циклов — не-цель.

## 5. SynchedEntityData — закрыт

itemsById — УЖЕ DataItem<?>[] массив (O(1) по id); getValue = volatile-чтение
(VarHandle backbone, 605) — неизбежен. Рычага нет.

## 6. Следствие для конвейера

Все крупные лейны ценза 35275967738: покрыты готовыми рычагами (5 шт.,
офлайн-верифицированы, ждут диспатча) или закрыты настоящим документом
(парити-стена/микро/JVM-внутреннее). Офлайн-фаза конвейера ИСЧЕРПАНА.
Критический путь проекта = ДИСПАТЧИ + absorb-вердикты по preregistered-гейтам
(§156 INSIDE-CACHE; §S7-138 FLUSH-DIET; §S7-139 FLUID-FREE; ALLOC-DIET) —
каждый следующий рычаг обязан впитывать уроки живых A/B (урок leg #2:
непроверенный на сцене рычаг = риск коллапса). Дальнейшая слепая накопительная
инженерия 6-го рычага ВРЕДНА: увеличивает очередь непроверенных правок без
измеримой отдачи.

Артефакты: /tmp/collutil.j, /tmp/itementity.j, /tmp/mob.j, /tmp/syncdata.j,
/tmp/zombie.j (javap-срезы; ядро /tmp/kmat/server/versions/1.21.10/purpur-1.21.10.jar
совместимо с census 35275967738). Не хранятся в git (перегенерируемы; команды в §2).
