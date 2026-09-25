# RESEARCH-459-P36 — Inside-epoch fast-gate (TASK-459-58, закон 11 v18.2)

Карточка [ID-P36] (origin/round-458p-ideas:RESEARCH-458-P.md):

> Inside-epoch fast-gate (комбо к P32) | ARCH | inside/java_util хвосты | per-section
> счётчик мутаций (bump уже есть в secWrite) → флет-массив эпох; checkInsideBlocks
> сначала сравнивает int-эпоху секции с эпохой слота кэша — несовпадение → полный путь |
> src/inside_snap.rs, entityinside/…InsideSnapOps.java, src/inside_cache.rs |
> miss → ваниль; hit-инвариант самтестом; эпоха 64-бит (ABA/wrap) |
> −0.5-1пп java_util/inside хвостов в связке с P32 | тривиальный

## 1. МЕХАНИКА

Подсистема inside-snap (cmp424_inside) уже несёт per-section эпоху мутаций:
`InsideSnapOps.Snap.gen` (volatile long), bump на РЕАЛЬНОМ изменении в
`InsideSnapOps.secWrite` (InsideSnapOps.java:505-518: делегат ванильного
`section.setBlockState` + ref-compare `old != newState` → `s.gen++; s.pending = true`;
volatile bump ПОСЛЕ write — seqlock-дисциплина уже применена).

P36 добавляет ДЕШЁВЫЙ ПРЕ-ГЕЙТ перед полным путём inside-snapshot serve:

```
snapGet(level, pos)
  └─ [P36 fast-gate] slot = slotOf(section); epoch секции vs EPOCHS[slot] (ОДИН long-cmp)
       ├─ совпало И слот warm → serve из слота (мимо lane-claim/CHM-обхода)
       └─ НЕ совпало (miss) → полный путь serve/serve4 → miss → ваниль level.getBlockState
```

- ФЛЕТ-МАССИВ ЭПОХ: плоский `long[CAP]` (CAP=1<<15, калиброван с InsideSnapOps.CAP),
  запись по слоту секции; secWrite-bump НЕ дублируется — P36 только ЧИТАЕТ уже
  существующий `Snap.gen` (bump уже в secWrite — карточка).
- ПОЛНЫЙ ПУТЬ (что пропускается на hit): serve4 lane-claim = lane-скан до 8 слотов
  (tick/level/cx/cz), `level.getChunkNow`, section-index арифметика, `SNAPS.get(sec)`
  CHM-lookup, gen/pending волатильные пары. На горячих статичных сущностях это
  повторяется на КАЖДУЮ позицию КАЖДОГО checkInsideBlocks-прохода.
- MISS-СЕМАНТИКА: несовпадение эпохи/слота → полный путь → его собственный miss →
  ваниль. P36 не добавляет НОВОГО источника истины — только порядок проверки.

## 2. САЙТЫ (javap-цели wiring-фазы)

| # | Сайт | Тип | Контракт |
|---|------|-----|----------|
| 1 | `InsideSnapOps.snapGet(Level,BlockPos)BlockState` | pre-gate вставка (HEAD метода) | receiver-first, длина сохранена; fast-gate до `serve/serve4` |
| 2 | `InsideSnapOps.secWrite(LevelChunkSection,int,int,int,BlockState)BlockState` | БЕЗ изменений (bump уже live, s.gen++) | read-only потребитель `Snap.gen` |
| 3 | `Entity.lambda$checkInsideBlocks$2` | НЕ трогается (уже retarget на snapGet, entity_compose stage 1c, S7-162) | ОДИН Entity-хук сохраняется |

NCDFE-канон (×93-indy, run 35902792520): `InsideEpochGate` (+ вложенные классы)
определяются в РАННЕМ arm-хуке (src/inside_epoch_gate.rs activate → kernel loader)
ДО первой gated-вызова; indy/`ThreadLocal.withInitial`-метод-ссылок в `<clinit>` НЕТ
(plain `new ThreadLocal<>()`-паттерн InsideSnapOps.java:241-254).

## 3. PARITY (бит-в-байт)

- MISS → ваниль: единственный эффект fast-gate-miss = исполнение существующего пути.
- HIT-ИНВАРИАНТ самтестом (при арме, до публикации BRIDGE_READY): N>=3 позиций на
  прогретую секцию — slot-serve обязан вернуть ТОТ ЖЕ ref, что полный serve
  (тот же объект, что ванильный readPalette — HIT-контракт InsideSnapOps).
  Расхождение → lever не публикуется (fail-dominant).
- ЭПОХА 64-БИТ: `long gen`, монотонный, per-section. ABA невозможен: перезапись
  2^63 реальных (ref-неравных) записей в ОДНУ секцию недостижима; wrap не меняет
  порядок (monotonic add до 2^63-1). Слот-ключ = identity секции (SNAPS не эвиктит —
  sec->snap immutable, InsideSnapOps.java:230), ложный hit по переиспользованному
  слоту исключён ref-привязкой слота к секции.
- MEMORY ORDERING: чтение `gen` volatile после контент-чтения (seqlock-пара;
  writer-side volatile-схема уже в secWrite).

## 4. Δ ПРОГНОЗ

- Карточка: −0.5-1пп java_util/inside хвостов В СВЯЗКЕ с P32 (inside_cache gate).
- База: inside-лейн ≈1.3-1.6% total CPU (PROFILE-B, inside_snap.rs:3-7);
  serve-хвост (lane-claim+CHM) ≈30-50% среза → 0.4-0.8пп независимого эффекта,
  связка с P32 (гейт целиком для статичных сущностей) даёт верх карточки.
- ЗАМЕР (wiring-фаза, law-16): min-of-2/3 A/B на carrier + P32, STRICT-off до оракула.

## 5. РИСКИ

| Риск | Митигация |
|------|-----------|
| NCDFE-шторм (×93-indy, 473412 повтор) | define в раннем arm-хуке, lever dormant до BRIDGE_READY; no-indy-clinit |
| Ложный hit (wrap/ABA) | 64-бит monotonic gen + ref-привязка слота; самтест hit-инварианта при арме |
| Порядок define vs retarget | arm-hook раньше entity_compose stage; секвенция как inside_cache (BRIDGE_READY поллинг) |
| Флет-слот вне CAP | сверх CAP — гейт молчит → ваниль/полный путь (cap-семантика SNAPS) |
| Регрессия связки с P32 | STRICT-OR композиция, отдельный env-флаг; miss-путь бит-в-байт ванильный |

## 6. СТАТУС СКАФФОЛДА (этот коммит)

- `src/inside_epoch_gate.rs` — модуль-каркас: env `CRUSSTY_INSIDE_EPOCH_GATE`
  (off = dormant-невидимость), register/activate-лестница, define-точка раннего
  arm-хука (blob не встроен — include_bytes! приземляется в wiring-фазе после
  scripts/build_inside_epoch_gate.sh), hit-инвариант самтеста в контракте.
- `entityinside/net/minecraft/world/entity/InsideEpochGate.java` — stub моста:
  флет `long[] EPOCHS`, epochRead/epochMatch/epochBump-механика, самтест, статы;
  vanilla-типы (Level/BlockPos/BlockState) — в wiring-фазе (stub сознательно
  vanilla-free = компилируется голым javac, SELF-CONTAINED-урок S7-148).
- `scripts/build_inside_epoch_gate.sh` — сборка блоба (javac --release 21, major <=65).

## 7. ИСТОЧНИКИ

1. Linux kernel docs — Sequence counters and sequential locks (seqlock: валидация
   чтений счётчиком, odd/even, write-side барьеры):
   https://docs.kernel.org/locking/seqlock.html
2. Wikipedia — Seqlock (механика sequence lock, retry читателей, порядок):
   https://en.wikipedia.org/wiki/Seqlock
3. Zuriel et al., Efficient Lock-Free Durable Sets (SOSP 2019) — Epoch Based
   Reclamation [Fraser 2004] как канон эпох-счётчиков против ABA:
   https://dl.acm.org/doi/10.1145/3341301.3359627
4. rigtorp/Seqlock — C++11 seqlock реализация (memory-order пары reader/writer):
   https://github.com/rigtorp/Seqlock

— TASK-459-58, закон 11 v18.2: финал = {run id, ветка+SHA, вердикт-число}.
