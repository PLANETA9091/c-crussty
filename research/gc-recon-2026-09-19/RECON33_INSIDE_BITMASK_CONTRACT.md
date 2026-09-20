# RECON-33 · JAVAP-КОНТРАКТ INSIDE-BITMASK-ФЛАГМАНА (option-B pre-work)

**TASK-356 · 2026-09-20 · офлайн, без диспатча.** Источник: patched-kernel.jar s7194,
javap-дампы `research/gc-recon-2026-09-19/contract-inside-bitmask-s7194/`
(Entity_full, LevelChunkSection_full, LevelChunk_full, BlockGetter через javap run).
Скрипт: `scripts/bench4_recon/recon33_inside_bitmask_contract.py` (воспроизводимо).
Назначение: довести вердикт RECON-32 (inside-семья 11.28/11.65% сцены, флагман
option-B при выборе владельца) до коммит-пригодного контракта реализации.

## 1. Полная цепочка входов (bytecode-доказано)

Семья: `applyEffectsFromBlocks()` → `applyEffectsFromBlocks(Vec3,Vec3)` →
`applyEffectsFromBlocks(List<Entity$Movement>)` → `checkInsideBlocks(List<Movement>, StepBasedCollector)`
→ (по Movement) `checkInsideBlocks(Vec3,Vec3,Collector,LongSet,int)` →
`BlockGetter.forEachBlockIntersectedBetween(from,to,aabb,BlockStepVisitor)` →
`lambda$checkInsideBlocks$2(pos,step)`.

ВНЕШНИЕ per-tick входы (все — через no-arg `applyEffectsFromBlocks()`, ref-скан пулов классов):
- **ItemEntity.tick** (bc 273: invokevirtual) — главный источник на сцене X150K
- ExperienceOrb.tick, FallingBlockEntity.tick, PrimedTnt.tick, EndCrystal.tick
- EnderDragon.aiStep; AbstractBoat.tick (×2 call-site); AbstractMinecart.move + собственный override
- **LivingEntity / Mob: ПУСТО** — ни деклараций, ни вызовов (javap -p -c, ref-скан).
  КОРРЕКТИРОВКА RECON-32: «Mob-муверы» в внутри-семье не участвуют; внутри-gate =
  item-класс + спец-классы. Число 9.08/9.36% не меняется (те же стеки), меняется владелец механизма.
- Внутри Entity сам no-arg AFB никем не вызывается (baseTick/move/tick — чисто);
 Master-gate `isAffectedByBlocks()` = `!isRemoved() && !noPhysics` (для сцены всегда true).

## 2. Семантика visit-лямбды (lambda$checkInsideBlocks$2) — где легален skip

Порядок в лямбде (bc): `isAlive()` → step-budget (AtomicInteger get/set, icmp) →
`level.getBlockState(pos)` → **`BlockState.isAir()` → ifeq 66: air-ветка ВОЗВРАЩАЕТ true
(continue) СРАЗУ**, до любых наблюдаемых действий (только debug-запись при активных
подписчиках) → non-air: `getEntityInsideCollisionShape` → `!= Shapes.block()` →
`collidedWithShapeMovingFrom(...)` → `getFluidState` → эффекты (entityInside-семейство).

**Ключевые пины:**
1. **Air-ветка не трогает visitedBlocks** (тело bc 34..65: только debug-флаг) —
   visitedBlocks LongSet на air-пути не читается и не пишется. Skip all-air секций
   оставляет поле бит-в-бит.
2. Эффекты существуют только на non-air ветке → «в секции нет не-air блоков» ⟹
   visit каждого pos внутри неё эквивалентен no-op в терминах эффектов, бюджета
   шагов (AtomicInteger не инкрементится на air — проверка идёт ДО) и отладочного
   состояния (при выключенных подписчиках).
3. **Прецедент в кернеле**: `LevelChunk.getFluidState` УЖЕ использует fast-path
   `getSection(i).hasOnlyAir()` (bc 13..38) — предлагаемый паттерн нативен кодовой базе.
4. Носитель истины: `LevelChunkSection.nonEmptyBlockCount` (short), поддерживается
   vanilla `setBlockState(IIII,BlockState,Z)` (getAndSet + инкремент/декремент),
   `recalcBlockCounts()` (предикат `!isAir` — lambda$recalcBlockCounts$0) и `read(FriendlyByteBuf)`.
   `hasOnlyAir()` = public, `nonEmptyBlockCount == 0` ⟺ все состояния секции air.
   Fluid-блоки (вода/лава) — НЕ-air BlockState → счётчик их считает → водные секции
   не скипаются → эффекты жидкости (пузырьки/течение) сохраняются по построению.

## 3. Спецификация pre-gate (flagman-кандидат, реализация при option B)

Точка вставки: `Entity.checkInsideBlocks(Vec3,Vec3,Collector,LongSet,int)`, сразу после
`makeBoundingBox(to).deflate(1e-5)` (bc 11), ДО invokedynamic forEachBlockIntersectedBetween.

Алгоритм:
- S = множество (cx,cz,secY) клеток, покрываемых deflated AABB: cx∈[floor(minX)>>4..floor(maxX)>>4],
  аналогично cz; secY = [clamp(floor(minY)>>4, minSection..maxSection) .. clamp(floor(maxY)>>4, ...)].
  Типично ≤2×2 клетки × 1-2 секции.
- Для каждой клетки: `Level.getChunk(cx,cz)` (существующий chunk-map — та же инфраструктура,
  что у getBlockState) → `chunk.getSections()[secY - minSection].hasOnlyAir()`.
- Если ВСЕ клетки air И debugSubscribers НЕ подписаны (ENTITY_BLOCK_INTERSECTIONS):
  return немедленно (0 visited). Иначе — полный vanilla-путь без изменений.
- Оверхед неудачного skip: 1-4 chunk-lookup + 2-4 чтения short-поля — пренебрежимо против
  сохраняемой машинерии (gate 791 + PalettedContainer.get 855 + visit-set 819 + flushStep 806
  + init 747 + applier 658 + guava-iterator 616 + lambda 457 сэмплов, s7194/s7189).

## 4. Median-exact гарантия (против протокола)

- Гейм-состояние: effects-поток, AtomicInteger-бюджет, visitedBlocks, collector-шаги —
  все либо недостижимы на air-пути (п.2.1-2.2), либо идентичны в skip и vanilla.
- Отладочный поток debugBlockIntersection: единственное наблюдаемое расхождение —
  отсутствуют записи для air-блоков all-air секций при АКТИВНЫХ подписчиках; гейтится
  условием `!debugActive` → расхождение исключено по построению.
- Fail-safe: любой non-air в проверяемых клетках → vanilla-путь без модификаций.
- Доверие счётчику = тому же уровню, на котором уже стоит fast-path getFluidState (прецедент).

## 5. Геометрия сцены и потолок

X150K: предметы стоят/плывут над плоским полом, deflated BB верхних предметов целиком в
all-air секциях → skip срабатывает практически на каждый tick каждого движущегося item.
Сырой потолок семейства подтверждён RECON-32: ~9.1% CPU (11.28/11.65% минус effects-apply
2.20/2.29% — парити-семантика, не скипается). TPS-конверсия по эмпирике диет ~0 (slack
region-threads) → под ДВОЙНОЙ БАР не банкингуется — вердикт RECON-32 не пересматривается.
Данный контракт = готовая к реализации спецификация при выборе владельцем option B (min-MSPT).

## 6. Статус

- RECON-33 = 8-е документальное усиление закрытия «диеты» (RECON-17/20/23/26/30/31/32/33),
  но ПЕРВОЕ с реализационным контрактом флагмана: точки вставки, пины семантики,
  fail-safe, прецедент паттерна.
- Диспатчи остаются остановленными (ожидание решения владельца B/C). Данный док —
  офлайн-артефакт NEXT 356, коммит-пригоден.
