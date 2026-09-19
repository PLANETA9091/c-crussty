# RECON-22 — fastutil navigatingMobs race — LATENT в банке v3 (не STEAL-специфичен); s7186 разбор

TASK-345 (тик ~04:2x +08). Leg s7186 (run 35462686159 @ 9afd098, paired #14 v1: банк v3 +
zero_alloc=1 + skip_store_bb=1, медленный класс): SUCCESS, НО PG-T2 FAIL (threw=3) и
PG-T3 вне класса пары (runner 7072550 = +5.88% от якоря 6680195 — банд 6.2M..7.2M ШИРЕ
парного ±5% → окно диспатча допускает непарные лендинги; фикс: банд = якорь ±5% =
6.35M..7.01M).

## Стек (19:03:13, воркер-тред)
```
Entity threw exception (ReportedException "Exception while updating neighbours")
  ← Entity.lambda$checkInsideBlocks$2 → forEachBlockIntersectedBetween → checkInsideBlocks
  ← applyEffectsFromBlocks ← Skeleton.aiStep ... Skeleton.tick
  ← ServerLevel.tickNonPassenger ← guardEntityTick
  ← RegionTickOps.tickBucket(474) ← lambda$ensureHelpers$4 ← Thread.run   [WORKER]
Caused by: NPE fastutil ObjectOpenHashSet$SetIterator.next("wrapped" is null)
  ← ServerLevel.sendBlockUpdated(1883)            ← ВАНИЛЬНАЯ точка
  ← notifyAndUpdatePhysics ← setBlock ← setBlock
  ← DoorBlock.neighborChanged                      ← side-effect внутри checkInsideBlocks
```
Всего 3 throws (19:03:13 / 19:05:36 / 19:07:26), все пойманы per-entity catch, сервер
дожил до конца соака (5 TPS-поллов, median5=1.7).

## Выводы
1. **Race — латентный дефект БАНКА v3, а не только лейна STEAL**: воркеры region_threads
   (банк v3!) тикают entity → setBlock (DoorBlock от checkInsideBlocks) → ВАНИЛЬНЫЙ
   sendBlockUpdated:1883 итерирует navigatingMobs БЕЗ защиты от параллельной мутации
   (другой воркер: Navigation add/remove). Vanilla guard (isUpdatingNavigations) —
   однопоточная семантика, cross-thread не работает.
2. RECON-20 корректировка: s7176 (CRASH) и s7180 (CRASH в BU-DEFER реплее) — ЭТОТ ЖЕ
   фундаментальный race; STEAL/BU-DEFER лишь меняли ТОЧКУ итерации. S7-169-кандидат
   (isUpdatingNavigations-гейт на replay) НЕ лечит воркер-vs-воркер/воркер-vs-replay.
3. Частота: 3 catches / 300s соак на 150k сцене (первое наблюдение на 5 bank-v3 классов
   ног — вероятностное окно: DoorBlock × скелет × навигация). Эффект на median5 — шумовой
   (per-entity catch), НО хвостовой риск: fatal-путь (s7176) убивает лег целиком.
4. **S7-170-кандидат** (правильность, не ТОП-1-perf; к реализации ПОСЛЕ закрытия текущей
   пары v1/v2a): потокобезопасная канализация sendBlockUpdated с воркеров — инфраструктура
   BU-DEFER существует (BlockUpdateOps мосты); защитить ИТЕРАЦИИ navigatingMobs
   (гейт на всех путях: vanilla + replay), либо COW-снапшот на итерации. Parity-риск
   (порядок recomputePath) — задокументировать при прегистере.
5. Парность: банд диспатча ОБЯЗАН быть ±5% от якоря (6.35M..7.01M для anchor-slow), иначе
   банд-гейт пропускает непарные лендинги (доказано s7186: 7072550).

## Вердикт s7186
INVALID-PAIRING (+PG-T2 FAIL) — НЕ вердикт лейна #14 v1. Парная нога v1 остаётся
не-вердиктированной; парк-профиль leg (park 12.5%, young 136/Full 0, median5=1.7 @ 7072550)
— консистентен с anchor-slow классом (нормализованно ~1.7/7.07M vs 1.60/6.68M = −0.7%... в
допуске ±12% шума RECON-19; намёка на буст v1 НЕТ, но лег невалиден для пары).
