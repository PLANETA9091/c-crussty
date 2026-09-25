# RESEARCH-459-CX5 — TASK-459-84 (WILD закон-11, тик-459)

**C-X5: paletted zero-copy reader views.** Идея своя (не из карточки RESEARCH-458-P): массовые
чтения `PalettedContainer.get` (4.48% ваниль + `SimpleBitStorage.get` 1.50% = ~6.0% paletted-лейн,
round-a32-457 cpu-collapsed) заменяются **reader-view объектом**: одна выборка
`container.data → (storage, palette, bits)` на серию запросов, дальше N чтений без повторного
volatile-рида `data`, без interface-dispatch цепочки и без per-call palette-индирекции.
Одиночные чтения — ваниль (не трогаем). STRICT dormant (lever `cmp459_cx5`).

## 1. Профильные числа (грунт)

| метрика | значение | источник |
|---|---|---|
| PalettedContainer.get | 4.48% CPU | round-a32-457 cpu-collapsed (src/paletted.rs header) |
| SimpleBitStorage.get | 1.50% CPU | там же |
| суммарный paletted-лейн | ~6.0% (4.48+1.50) | там же; 3.7% top-1 leaf на cert-stack ногах (chkmono457-8 BOTTLENECKS_3) |
| PalettedContainerOps.get (DEMUX slow-path) | 2.2% (2352 сэмпла) | L06 leg-3 факт (BOARD 15:32) |
| serve4-лейн (leg-3 адресуемый) | 1.9% + SNAPS-доля 0.45% | L06 капчур-матем (BOARD 15:50) |

## 2. javap-грунт (patched-kernel.jar, round-396-a канон; снято 16:2x, TASK-459-84)

`PalettedContainer.get(int)` — 3 шага НА КАЖДЫЙ вызов:
```
0: getfield data : PalettedContainer$Data          // volatile read (каждый вызов!)
8: getfield Data.storage : BitStorage
12: invokeinterface BitStorage.get(I)I             // interface dispatch
17: invokevirtual readPalette(Data,I)Object        // palette-индирекция
```

`SimpleBitStorage.get(int)` — magic-деление на вызов:
```
2: j = magic * idx                       // imul (int wrapping)
10: k = j >>> 20                         // idx / valuesPerLong (magic division)
23: l = (j & 1048575) * mulBits >>> 20   // бит-офсет внутри long
39: return (int)(data[k] >>> l & mask)   // laload + lushr + land
```
Полная цепочка magic (javap, замкнута web-retrieval'ом):
- `magic = BETTER_MAGIC[bits]`, `mulBits = (64/bits)*bits` (javap ctor 168..193);
- `BETTER_MAGIC[bits] = (int) IntegerUtil.getUnsignedDivisorMagic(64/bits, 20)`
  (javap `<clinit>` 1174..1211);
- concurrentutil `IntegerUtil.getUnsignedDivisorMagic(d, 20) = ((1L<<20)-1)/d + 1`
  (github.com/Spottedleaf/ConcurrentUtil `IntegerUtil.java` L192-194) — т.е.
  `magic = ceil(2^20 / vpl)`;
- эквивалентность `magic*idx>>>20 == idx/vpl` и `(j&0xFFFFF)*mulBits>>>20 ==
  (idx%vpl)*bits` доказана и проверена программно на всём поддерживаемом
  диапазоне (bits 1..15 × idx<4096 = 61440 точек — транскрипт в selfTest).
- LIVE-верификация (офлайн-проба TASK-459-84: concurrentutil IntegerUtil +
  commons-lang3 Validate shim'ы на classpath кернел-jar): поле `magic` живого
  кернел-класса == транскрибированной формуле на всех bits 1..15 =
  **mismatches 0/15**; selfTest-итог: model parity 308160 точек +
  magic-division 61440 точек (JDK-only канон стабилен).
- ВАЖНО (moonrise-патч кернела): конструктор жёстко каппит
  `Size > 4096 not supported` (IllegalStateException, javap 197..215) —
  секции ровно 16³; объёмы 16384 через SimpleBitStorage не проходят.

Нулевой-копии лег: `SimpleBitStorage.getRaw()` = `getfield data; areturn` — возвращает ЖИВОЙ
`long[]` без клонов (javap, 4 байткода). `PalettedContainer$Data` — иммутабельный кортеж
`(configuration, storage, palette, moonrise$palette[])`; `storage()`/`palette()`/`moonrise$getPalette()`
публичные; `PalettedContainer.data` — public volatile. ⇒ читатель-вьюха, сэмплировавшая `data`
ОДИН раз, держит консистентную (storage, palette) пару: resize/swap публикуется заменой ссылки
`data`, а не мутацией содержимого Data — внутри серии запросов битовая математика не меняется.

## 3. Внешние источники (≥2 по миссии)

1. **C2ME-fabric** — https://github.com/RelativityMC/C2ME-fabric (и релизы
   https://github.com/RelativityMC/C2ME-fabric/releases): канон-прецедент, что paletted-слой —
   легитимная ось оптимизации chunk gen/I/O/loading; их подход — параллелизм + замена storage
   имплементаций, НЕ смена читаемого формата (парити к ваниль-битовой раскладке сохраняется).
2. **minecraft.wiki — Java Edition protocol/Chunk format** —
   https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format: секция = palette + packed data,
   `entriesPerLong = 64/bits`, значения НЕ пересекают границу long — именно эта инварианта делает
   div/mod-математику вьюхи (idx/valuesPerLong, (idx%valuesPerLong)*bits) бит-в-бит эквивалентной
   magic-делению SimpleBitStorage.
3. **NotEnoughPalette** — https://modrinth.com/mod/notenoughpalette: drop-in замена
   PalettedContainer, заменяющая bit-packed SimpleBitStorage на `byte[4096]+palette` — прецедент
   жизнеспособности swap-storage И его цена (мемори): C-X5 берёт противоположную половину выигрыша
   (массовые чтения) БЕЗ смены storage/формата и без Memory-налога на все контейнеры.
4. **ConcurrentUtil (Spottedleaf)** — https://github.com/Spottedleaf/ConcurrentUtil
   (`src/main/java/ca/spottedleaf/common/util/IntegerUtil.java` L192-194):
   `getUnsignedDivisorMagic(divisor, bits) = ((1L<<bits)-1)/divisor + 1` — замыкает
   javap-цепочку magic-деления кернела (BETTER_MAGIC), см. §2.

## 4. Батч-пути (где серия > 1 запроса) vs одиночные

Массовые: codec-parse/serialize (`Data.write`/`getSerializedSize`, `PalettedContainer.getAll`
лямбда-цикл), chunk-send encode (доля ваниль-циклов вокруг ChunkParseOps/сериализация секций —
см. L07: entity-sync send 0.68% wall, chunk-encode JNI уже нативный), countPaletteSizes,
DEMUX-материализация (сама строит снапшот 4096 циклом get — вьюха ускорила бы билд, но это
внутренняя ирония, не цель). Одиночные (ваниль остаётся): random-tick/ collision/ gameplay get.
Оценка батч-доли лейна: PalettedContainerOps.get 2.2% (чисто батч: materialize-циклы 4096/16384)
+ сериализационная доля ванильного get ~25-35% → адресуемая база ~1.4-2.0пп.

## 5. Капчур-матем (preregistered)

Убираем на батч-сериях: (а) volatile re-read `data` + ветки, (б) interface dispatch, (в)
per-call readPalette-индирекцию для raw-index потребителей (serialize требует индексы, не
объекты — ваниль делает object-resolve → hashCode/equals-free, но load из T[]), (г) magic-div
заменяем на div/mod по константе (javac/JIT то же, вычитаем nothing). Экономия на серии 4096:
~3 invoke/volatile-рида на чтение → консервативно 20-35% стоимости батч-чтений.
Δ = база 1.4-2.0пп × захват 30-45% = **+0.4..+0.9пп**; потолок = батч-база = **+2.0пп**.
Против P21 (+1-2пп reload) и P27 (alloc- relief) — комбинируется, пересечений по коду нет
(вьюха не трогает parse-cache и не аллоцирует в горячем цикле, кроме самой вьюхи 1/серию).

## 6. V1-ловушки (задокумечены до вайринга)

- **Torn-read во время мутации**: вьюха консистентна против resize (Data иммутабелен), но НЕ
  против in-place мутаций storage (`getAndSet` пишет data[k] на месте). Батч-пути parse/serialize
  выполняются на снапшотах/под секционной блокировкой — вайринг волны-2 обязан доказать hold;
  STRICT dormant до оракула. Fail-closed: identity re-check `container.data == sampled` до и
  после серии; любое расхождение → ваниль-цикл, счётчик fallbacks.
- **Generic BitStorage**: вьюха легальна только для SimpleBitStorage (magic-математика известна);
  `instanceof` guard, иначе ваниль. ZeroBitStorage/PalettedContainerRO — early-out.
- **NCDFE-канон**: java-хелпер (волна-2) определяется в kernel loader ДО первого use, ZERO
  определений внутри retransform (ClassReader → Class.forName → deadlock, src/improved_noise.rs
  header). Фаза-1 (эта) — observation-only scaffold: Rust-референс + JDK-only stub, ничего не
  определяется.

## 7. Гейты (preregistered, T1)

- Гейт STRICT eq: `CRUSSTY_LEVER_FLAG == cmp459_cx5`; пустой/чужой флаг = ваниль, hook не
  регистрируется (dormant discipline 3a270ee). Отдельной env-кнопки НЕТ (в отличие от
  paletted-demux — исторический ключ не наследуем).
- Парити: Rust `bit_exact_selftest` — вьюха div/mod ≡ magic-модель SimpleBitStorage ≡ ожидаемые
  значения, лестница bits 1..15 × sizes {64, 256, 4096, 16384} ≈ 10k+ проверок; java `selfTest()`
  — та же математика чисто на JDK (без кернел-классов).
- Бенч-гейты (волна-2): fallbacks <1%, ARM-маркер `cmp459_cx5` в логе, Δ≥+0.4пп min-of-3 band
  6.0-9.5M, young ≤128+10%, Full ≤9.
