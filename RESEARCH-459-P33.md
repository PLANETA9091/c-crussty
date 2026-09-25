# RESEARCH-459-P33 — Volatile-demotion: VarHandle.getAcquire на остаточных volatile-чтениях (TASK-459-73, закон 11 v18.2)

Карточка [ID-P33] (origin/round-458p-ideas:RESEARCH-458-P.md):

> Volatile-demotion: getAcquire на остаточных volatile-чтениях MISS-пути | ARCH |
> inside_volatile фенс-хвост | javap-таргет: в retarget-теле inside-сайта
> (Level.getBlockState→InsideSnapOps.snapGet уже есть) + PalettedContainerOps-классе
> читать поля через VarHandle.getAcquire (JMM-эквивалент volatile-read по
> наблюдаемости; x86 = plain load, C2-перестановки контролируемы) |
> src/inside_snap.rs, paletted/net/minecraft/world/level/chunk/PalettedContainerOps.java
> (класс уже определён), src/classfile.rs | getAcquire (НЕ opaque) в v1 — JMM-канон;
> single-writer region-модель; рандом-поп оракул | фенс-хвост MISS 2-4пп × 30% →
> +0.6-1.2пп | JMM-тонкости дольных полей — только acquire-режим до оракула

## 1. МЕХАНИКА (что демотируется и почему это JMM-легально)

Кернел несёт ДВА семейства горячих volatile-чтений в inside-лейне, где writer-сторона
ПОЛНОСТЬЮ контролируется нами (single-writer per region), а reader-стороне достаточно
release/acquire-пары — volatile-полная сила StoreLoad-драина не нужна, потому что
читатель никогда не пишет, а писатель один:

  1. INSIDE-SITE (retarget-тело): Entity.lambda$checkInsideBlocks$2 уже retarget на
     InsideSnapOps.snapGet (inside_snap.rs, cmp424_inside; entity_compose stage 1c,
     S7-162 — ОДИН Entity-хук). В serve/serve4 читается seqlock-пара
     `Snap.gen` (volatile long) + `Snap.pending` (volatile) и content-массив; на
     MISS-хвосте (гейт не сработал → полный путь → его miss → ваниль
     Level.getBlockState) каждая позиция каждого checkInsideBlocks-прохода платит
     повторные volatile-чтения пары. Демотирование: `gen/pending` читаются через
     VarHandle.getAcquire — acquire-читатель видит ВСЕ записи, сделанные до
     volatile-схемы writer-а (см. §2), т.е. наблюдаемость seqlock-пары сохраняется.
  2. PALETTED-SITE: PalettedContainerOps.get (vanilla-equivalent slow path demux,
     S7-131) читает `self.data` (PalettedContainer.data — public volatile field) и
     гейт-поля crusstySnap/crusstySnapGen/crusstyGen (publication-порядок snap
     первым, snapGen последним — оба volatile). Демотирование: все три гейт-чтения
     через getAcquire; publication-протокол writer-а (volatile store 1 = snap,
     volatile store 2 = snapGen) НЕ трогается — acquire-читатель, зафиксировавший
     snapGen, гарантированно видит snap (acquire load не может быть переставлен выше
     подчинённых ему загрузок; в терминах JSR-133 cookbook это LoadLoad+LoadStore
     после volatile load — на x86 обе no-op).

Карточка прямо фиксирует канон v1: **getAcquire, НЕ opaque** — opaque допускает
койлесcинг повторных чтений и не даёт synchronization-order-упорядочивания с
writer-ом; acquire даёт JVMM-эквивалент volatile-read по наблюдаемости (кумулятивная
лестница Lea: Plain < Opaque < Release/Acquire < Volatile — любая гарантия слабого
режима входит в сильный, поэтому замена volatile-load на acquire-load легальна:
все свойства читателя сохраняются, writer-сторона остаётся volatile).

## 2. ИСТОЧНИКИ (рисёрч ≥2 URL; все живы на момент записи, HTTP 200)

1. Doug Lea — «Using JDK 9 Memory Order Modes»
   https://gee.cs.oswego.edu/dl/html/j9mm.html
   → лестница режимов кумулятивна (Plain, Opaque, Release/Acquire, Volatile);
   «Any guaranteed property of a weaker mode, plus more, holds for a stronger mode»;
   канон чтения поля в Acquire-режиме: `int v = X.getAcquire(this)`; поля,
   предназначенные для конкурентного доступа, продолжают ДЕКЛАРИРОВАТЬСЯ volatile
   (демотирование меняет только чтение-сайт, не декларацию) — прямой JMM-мандат P33.
2. Marc Brooker — «Are volatile reads really free?»
   https://brooker.co.za/blog/2012/09/10/volatile.html
   → на x86 volatile-read не несёт инструкции-фенса: из JSR-133-барьеров (LoadLoad,
   LoadStore, StoreLoad, StoreStore) на x86 только StoreLoad (после volatile STORE)
   компилируется в реальный fence; LoadLoad/LoadStore после volatile LOAD — no-op.
   Именно поэтому volatile-demotion чтений на x86 = 0 инстр-овер хед, а выигрыш P33
   — НЕ в удалении инструкций, а в снятии C2-ограничений: volatile-load запрещает
   C2 подъём/койлесинг/скалярную замену, acquire-load снимает часть этих запретов
   и убирает запреты переупорядочивания в планировщике (x86-планировщик сам шире).
3. Jean-Philippe Bempel — «Volatile and Memory Barriers»
   https://jpbempel.github.io/2015/05/26/volatile-and-memory-barriers.html
   → x86-барьер = lock addl / mfence (drain store buffer), дорогой ТОЛЬКО на
   store-стороне; lazySet ≈ release-семантика; компиляторные перестановки — вторая
   половина цены volatile наравне с аппаратной.
4. JEP 193 — Variable Handles
   https://openjdk.org/jeps/193
   → access-модели: «…with acquire memory order effects for reading» — getAcquire
   является стандартизованной access-mode; signature-polymorphic вызов =
   invokevirtual (НЕ invokedynamic), поэтому <clinit>-инициализация VarHandle
   (findVarHandle в статическом финале) не создаёт indy-констант — NCDFE-канон
   кернела (×93-indy, run 35902792520) выполняется без спец-мер.
5. (карточный, бот-гейт 403 на curl) StackOverflow 58336714 — getAcquire vs
   getVolatile vs getOpaque: https://stackoverflow.com/questions/58336714

## 3. САЙТЫ (javap-цели wiring-фазы, НЕ этого коммита)

| # | Сайт | Поле(я) | Режим v1 | Контракт |
|---|------|---------|----------|----------|
| 1 | `InsideSnapOps.serve/serve4` (тело обслуживает retarget-сайт Entity.lambda$checkInsideBlocks$2 → snapGet) | `Snap.gen`, `Snap.pending` | getAcquire | seqlock-пара: acquire-порядок сохраняется (gen-read упорядочен относительно content-read); writer-сторона (secWrite, volatile s.gen++ ПОСЛЕ write) не трогается |
| 2 | `PalettedContainerOps.get` | `PalettedContainer.data` | getAcquire | vanilla-эквивалент slow path: одна загрузка data («volatile read (vanilla)») — наблюдаемость бит-в-бит; torn-read окно = то же, что у ванили |
| 3 | `PalettedContainerOps.get` (гейт) | `crusstySnap`, `crusstySnapGen`, `crusstyGen` | getAcquire | publication-порядок writer-а сохранён (snap → snapGen last); accept-условие snapGen==gen+1 не меняется; демотирование читателя НЕ ослабляет accept-протокол (acquire ⊇ все нужные читателю упорядочивания) |
| 4 | `PalettedContainerOps` стат-поля | `BUILDS/ABORTS/CAPPED`, `NEXT_LOG_AT` | v1 НЕ трогается | monotone-лог каждые 2^24 — не горячо; дольные long-тонкости (card: «JMM-тонкости дольных полей») — только acquire-режим до рандом-поп оракула |

НЕ-ЦЕЛИ (анти-плейсебо): ARMED-флаги, CHM-lookup (не volatile-поле), любая
writer-сторона (secWrite bump, PalettedContainerOps publication) — демотирование
писателей запрещено карточкой (там StoreLoad-драин и есть смысл seqlock).

## 4. PARITY (бит-в-байт)

- Наблюдаемость: acquire-load ⊇ volatile-load по JMM-гарантиям читателя
  (кумулятивность лестницы, Lea §modes) — единственная разница = ДОПОЛНИТЕЛЬНЫЕ
  свободы C2 на читателе. Значение каждого чтения из synchronization order
  не меняется.
- Дольные long: 32-бит JVM-расщепление long исключено самой фактурой VarHandle
  (getAcquire на long — атомарен по JEP 193) + поля кернела уже volatile
  (64-битные volatile всегда атомарны); v1 держит acquire-режим и не опускается
  в opaque/plain до рандом-поп оракула (карточка).
- MISS-путь: гейт-поля только ЧИТАЮТСЯ читателем; accept/miss-решение
  (snapGen==gen+1, gen-even) переиспользуется бит-в-бит — новых источников истины
  нет, меняется лишь инструкция загрузки.
- Самтест (wiring-фаза, при арме до BRIDGE_READY): N≥10^6 acquire-чтений пары
  (snap, snapGen) под конкурентным writer-ом — ни одного accept при
  несогласованной паре (рандом-поп оракул-префикс: single-writer region-модель
  делает race-окно доказуемо пустым; расхождение → lever не публикуется,
  fail-dominant).

## 5. Δ ПРОГНОЗ

- Карточка: фенс-хвост MISS 2-4пп × 30% → +0.6-1.2пп.
- База (PROFILE-B, inside_snap.rs:3-7): inside-лейн ≈1.3-1.6% total CPU
  (PalettedContainer.get 0.487 + SimpleBitStorage.get 0.126 + readPalette 0.099 +
  LevelChunk.getBlockStateFinal 0.498). Механизм выигрыша — не x86-фенсы (их нет
  на load, Brooker), а снятие C2-запретов на горячем читателе: планировщик/hoist
  acquire-загрузок вокруг content-массива в serve-теле (после acquire-gen остальные
  загрузки тела получают легальные перестановки; volatile-тело это запрещало) +
  отсутствие volatile-барьерных отметок на demux-гейте. Оценка среза:
  demux+serve volatile-чтения ≈ 25-40% среза лейна → 0.4-0.65пп независимого
  эффекта; связка с P32/P36-гейтами (меньше полный-путь срабатываний — остаток
  концентрируется в MISS-хвосте) даёт верх карточки.
- ЗАМЕР (wiring-фаза, law-16): min-of-2/3 A/B на carrier, lever STRICT,
  прогноз-дельта пререгистрируется в RESULT.json ноги.

## 6. РИСКИ

| Риск | Митигация |
|------|-----------|
| NCDFE-шторм (×93-indy) | getAcquire = signature-polymorphic invokevirtual, НЕ indy (JEP 193); <clinit> без лямбда/indy — findVarHandle-канон; define в раннем arm-хуке ДО первого gated-вызова |
| C2-койлесинг acquire-чтений (v1-ловушка opaque) | v1 ТОЛЬКО getAcquire (не opaque): acquire запрещает койлесинг через synchronization order; оракул-гейт до любых дальнейших демотирований |
| Дольные long-тонкости | 64-битные поля кернела уже volatile-декларированы; VarHandle.getAcquire(long) атомарен; v1 не трогает стат-счётчики |
| Writer-side деградация | писателей НЕ трогаем (volatile store + StoreLoad-драин остаётся); demotion только на чтениях |
| Регрессия связки с P32/P36 | независимый env-флаг; miss-путь бит-в-бит ванильный; STRICT eq lever cmp459_p33 |

## 7. СТАТУС СКАФФОЛДА (этот коммит)

- `src/inside_acquire.rs` — модуль-каркас: env `CRUSSTY_INSIDE_ACQUIRE` (off =
  dormant-невидимость), register/activate-лестница (activate = честный
  scaffold-stop, BLOB_EMBEDDED=false), таблица демот-сайтов §3 как константный
  контракт + юнит-тесты (лестница режимов, NCDFE-индуктивность без indy,
  fail-closed при выключенном флаге).
- `entityinside/net/minecraft/world/entity/InsideAcquireOps.java` — stub моста
  (vanilla-free = компилируется голым javac, урок S7-148): VarHandle-поля
  (findVarHandle в <clinit> — без indy), acquire-читатели пары gen/pending и
  publication-пары snap/snapGen на СОБСТВЕННЫХ полях-эмулях (реальные сайты
  InsideSnapOps.Snap / PalettedContainer — wiring-фаза), selfTest одно-writer
  наблюдаемости.
- `scripts/build_inside_acquire.sh` — сборка блоба (javac --release 21,
  hard-fail major > 65).

## 8. ИСТОЧНИКИ-URL (протокол закона 11: рисёрч ≥2 URL)

- https://gee.cs.oswego.edu/dl/html/j9mm.html
- https://brooker.co.za/blog/2012/09/10/volatile.html
- https://jpbempel.github.io/2015/05/26/volatile-and-memory-barriers.html
- https://openjdk.org/jeps/193
- https://stackoverflow.com/questions/58336714

— TASK-459-73, закон 11 v18.2: финал = {run id, ветка+SHA, вердикт-число}.
