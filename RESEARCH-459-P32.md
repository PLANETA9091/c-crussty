# RESEARCH-459-P32 — SNAP sidecar-реестр (ID-P32, TASK-459-57, закон 11 тик-459)

Lever: `cmp459_snapreg` (STRICT eq) · carrier: inside_snap (cmp432_inside2/cmp436_ins4 · 16.6 / 9 РОСТ java_util)
Прогноз (карточка ID-P32): java_util −40-60% CHM-части → **+0.8-1.2пп**; HIT-путь дешевеет → inside −1-2пп косвенно.
Статус: **scaffold v1 DORMANT** — мост + флет-реестр + epoch int-cmp + fail-closed CHM fallback; ретаргетов в v1 НЕТ.

## 1. Механика (что положено в scaffold)

`CHM<LevelChunkSection, Snap>` гейта inside_snap заменяется на **флет-индекс по стабильным
per-section int-индексам**:

| элемент | v1 (этот scaffold) | v2 (после оффлайн-оракула) |
|---|---|---|
| реестр | `Snap[] REG` — зеркала ССЫЛОК тех же Snap (sidecar Snap сам не создаёт; источник истины — collect-план CHM-плоскости) | без изменений |
| индекс | стабильный монотонный int idx; identity-карта `SEC2IDX` только на WRITE-пути (1 раз/жизнь секции — каденс существующего `register()`) | idx вписывается в секцию на перехваченных сайтах создания (splice-паттерн fluid_free; перехват создания — паттерн entity_index.rs), SEC2IDX уходит |
| fresh-гейт | `REG_EPOCH[idx]` int-зеркало Snap.gen → **epoch одним int-cmp**; fresh-якорь — ВСЕГДА длинное `builtAtGen == gen` (int-wrap не может дать wrong serve) | epochBump из retarget secWrite; сэмплированный selfTest 1/100 на реальных HIT |
| miss | fail-closed CHM fallback `InsideSnapOps.snapGet` → ваниль (секции вне перехваченных сайтов, незарегистрированный idx, кап — всё через fallback) | без изменений |

Незыблемые контракты: тот же объект Snap, та же инвалидация secWrite (НЕ тронута), тот же
collect-план; HIT = ТОТ ЖЕ BlockState-объект, что и CHM-плоскость.

## 2. Web-рисёрч (4 источника; механика → как легла в дизайн)

1. **https://www.burnison.ca/articles/the-concurrency-of-concurrenthashmap** (R. Burnison,
   «The Concurrency Of ConcurrentHashMap», 2013) — разбор read-пути CHM: hash-spread +
   прохождение segment/bin + CAS-стампы при lock-free чтении. → Это и есть per-visit
   стоимость, которую снимает флет-индекс: `REG[idx]` = один array-read без хеширования
   (оценка: CHM.get ~40-80 нс при боксированном/identity ключе против ~1-3 нс array-read).
2. **https://skypjack.github.io/2020-08-02-ecs-baf-part-9/** (ECS back and forth, part 9,
   sparse set / EnTT) — каноническая форма «стабильный int-индекс объекта → плотный флет-
   массив»: identity→index один раз (write-путь), lookups по индексу навсегда (read-путь).
   → Дизайн SEC2IDX (write) vs REG[idx] (serve) — прямое приложение паттерна.
3. **https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/VarHandle.html**
   — JMM-семантика access-мод: `getAcquire`: «Returns the value of a variable, and ensures
   that subsequent loads and stores are not reordered before this access»; `getOpaque`:
   «accessed in program order, but with no assurance of memory ordering effects». → REG_EPOCH
   int-зеркало обновляется ПОСЛЕ публикации `REG[idx]` (reader order ref→epoch, release/acquire
   по наблюдаемости через volatile-поля Snap.gen), fast-gate никогда не serve-ит сам по себе.
4. **https://gee.cs.oswego.edu/dl/html/j9mm.html** (D. Lea, «Using JDK 9 Memory Order Modes»)
   — канон Acquire-режима: достаточно для наблюдаемости epoch/int-зеркал, x86 = plain load с
   compiler-barrier (C2-перестановки контролируемы); volatile long gen остаётся якорем. →
   Обоснование «epoch одним int-cmp» без фенс-налога на HIT-пути (для полного getAcquire-
   плана — смежная идея ID-P33, здесь НЕ смешивается).

## 3. Паритет

- HIT-serve: тот же BlockState-объект (флет-массив держит ссылки тех же Snap, что CHM).
- miss/стейл/вне-реестра/кап: `InsideSnapOps.snapGet` (действующая CHM-плоскость) → ваниль —
  бит-в-байт продолжение контракта cmp432_inside2.
- Инвалидация: secWrite InsideSnapOps НЕ модифицирован; epochBump — зеркало (v2 wiring).
- int-wrap epoch: не может дать wrong serve — длинный gen-якорь всегда ре-проверяется
  (fast-gate только в fail-closed сторону).
- selfTest (карточка: «count раз/100»): полный при арме (selfTest()==1 до ARM) + сэмплированный
  1/100 в v2 на реальных HIT; проверяет fresh-identity, epoch-mismatch→null, bounds→null, кап-гварды.

## 4. Δ числом

- Адресуемая база: inside/java_util хвосты; java_util 9% РОСТ с CHM-частью (RECON-карточка P32).
- CHM.get → REG[idx]: −40-60% CHM-части java_util (карточка) → **+0.8-1.2пп** тика;
  HIT-путь дешевле → inside −1-2пп косвенно (не засчитываем в scaffold-вердикт).
- Стоимость регистрации: 1 CHM-put на секцию за жизнь (≈ существующий register() каденс),
  AMORTIZED ZERO на serve-пути.

## 5. Риски (и чем закрыто в scaffold)

| риск | закрытие |
|---|---|
| секции вне перехваченных сайтов создания | fail-closed CHM fallback (карточка явно разрешает) |
| int-wrap REG_EPOCH | длинный `builtAtGen == gen` якорь всегда (fast-gate fail-closed) |
| NCDFE-шторм (canon ×93-indy, run 35902792520) | define-order $Snap → $Lane → sidecar в rust; <clinit> без indy/method-ref (NCDFE-канон: define в раннем arm-хуке, lever dormant) |
| selfTest false / define fail | BRIDGE_READY не публикуется; ваниль без хуков (probe-then-patch) |
| смешанные флаги (dup-define $Snap с inside_snap.rs) | fail-closed dormant; lever id непересекающийся (cmp459_snapreg STRICT eq) |
| кап 2^15 секций | clampAssign → -1 → fallback (зеркало CAP CHM-плоскости) |

## 6. Сайты (точки входа)

- v1: НЕТ байтовых ретаргетов — только define_class в kernel loader + selfTest-проба + arm
  (ранний arm-хук по канону; класс не определяется без lever-флага — ваниль бит-в-байт).
- v2 (план, вне scaffold): (a) перехват сайтов создания LevelChunkSection → регистрация idx
  (паттерн entity_index.rs / splice fluid_free); (b) wide-гейт 4B→3B в
  Entity.lambda$checkInsideBlocks$2-цепочке через entity_compose; (c) epochBump из secWrite;
  (d) selfTestSampled 1/100.

## 7. Sources

1. https://www.burnison.ca/articles/the-concurrency-of-concurrenthashmap
2. https://skypjack.github.io/2020-08-02-ecs-baf-part-9/
3. https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/VarHandle.html
4. https://gee.cs.oswego.edu/dl/html/j9mm.html

Карточка-источник: origin/round-458p-ideas:RESEARCH-458-P.md [ID-P32] (+ комбо-партнёр ID-P36
epoch fast-gate — REG_EPOCH есть его носитель).
