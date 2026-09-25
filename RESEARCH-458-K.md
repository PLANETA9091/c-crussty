# RESEARCH-458-K — ID-H04 roaring-occupancy секций + ID-H06 bloom занятости (agent-K, тик-458)

Ветка: `round-458k-roaring` @origin/master 96cc2704. Лейн: broadphase-остаток (8.8% на
компо-носителе; getEntities-хвост 1.85) + fastutil спилловер. Носитель: существующий
eindex counts-skip plane (cmp405_eindex infra: mirror src/entity_index.rs, bridge
entityquery/EntityIndexOps.java, wiring src/entity_index_manager.rs) — новый флаг
`cmp458_roar` (пустой/чужой = ваниль бит-в-байт, закон 4).

## Факты (javap patched-kernel.jar 289 moonrise, verified 2026-09-25)

1. `ChunkEntitySlices$EntityCollectionBySection.getEntities`: early-return `count==0`
   (вся коллекция); секции итерируются по y-диапазону запроса:
   `secMin = clamp(Mth.floor(box.minY - 2.0) >> 4, minSection, maxSection)`,
   `secMax = clamp(Mth.floor(box.maxY + 2.0) >> 4, minSection, maxSection)`;
   на секцию: `storage[0..min(storage.length, size)]`, per-entity
   `getBoundingBox().intersects(box)` strict.
2. `EntityLookup.addEntity(E,ZZ)`: sectionY = `clamp(blockPosition().getY() >> 4,
   WorldUtil.getMinSection, getMaxSection)` — ИДЕНТИЧНО формуле seedAll в
   EntityIndexOps.java (pos.getY()>>4 с тем же clamp). moveEntity пере-хоумит той же
   формулой (remove+add пара на КАЖДОЙ смене секции, вкл. y-only в одном чанке);
   removeEntity берёт `moonrise$getSectionY` (вычисленный так же). ⇒ sectionY из
   note-сайтов и seed бит-в-бит совпадает с ванильным размещением → per-секционный
   гейт НЕ может дать false-negative по построению (контракт superset сохраняется,
   дополнительно сужая его: секции вне y-диапазона ваниль НЕ сканирует, значит
   сегодняшние count>0 от y-чужих entity — чистые false positives, которые
   per-секционный гейт убирает БЕСПЛАТНО для parity).
3. Текущий mirror (entity_index.rs): chain per-ЧАНК (не per-секция), k2-monotone hull,
   count_chunk идёт по ВСЕЙ цепочке чанка независимо от y-диапазона запроса.
   secY уже приходит в note (cell[3i+2]) — rust его отбрасывает (Op без secY).

## Интернет-рисёрч (закон 11)

- docs.rs/roaring 0.11.5 (MIT/Apache-2.0): RoaringBitmap = BTreeMap<u16, container>,
  container = Array(≤4096) | Bitmap(65536 bit); u32-ключи; Treemap для u64.
  КРИТИЧНО для нас: heap-контейнеры + reader-snapshot семантика несовместимы с
  фиксированным BSS/seqlock-дисциплиной mirror'а (нулевые аллокации в горячем пути,
  lock-free читатели без epoch-копий). Отказ от крейта — осознанный: мы берём
  *bitmap-container половину* roaring inline: 64-секционное окно чанка = один bitmap
  контейнер (AtomicU64), per-секционные singly-linked цепочки = «array-контейнеры»
  ключей-id (уже есть в slot-пуле). Кардинальность per-секция не нужна (см. K-MAD-3).
- docs.rs/bloom 0.3.2: GPL-2.0 (!) + bit-vec/rand зависимости — лицензионное
  заражение репо + аллокатор внутри. Отказ: 40-строчный inline bloom на
  детерминированном splitmix64 (no deps, no alloc, no seed → false-negative
  невозможны по построению: insert-only, no delete, insert строго ДО publish под
  WLOCK/seqlock-брекетом).
- Формулы Bloom (wikipedia/классика): ε = (1-e^(-kn/m))^k; для m=32768 (4КБ), k=4:
  n=3000 → 0.88% < 2% (спека идеи H06 выполняется до ~3k занятых чанков; плотный
  9.2k-чанковый фиксатор деградирует gracefully в exact-путь — bloom-hit это просто
  fallthrough, FN невозможен, FP безвреден по контракту superset).

## §БЕЗУМИЕ (≥3 идеи: почему ≥+20 / риск / parity-план / GO-or-PARK)

### K-MAD-1 — Per-СЕКЦИОННЫЕ цепочки mirror'а + 64-бит occupancy bitmap (roaring-bitmap-container inline)
- **Почему**: count_chunk сегодня идёт по всей цепочке чанка (в среднем 16.3 entity/чанк
  на 150k/9.2k-фикстуре, поверхностные запросы качают ВСЕ секции). Секционный гейт =
  walk только секций y-диапазона ванильного скана (2-4 секции из 24) → ~5-10× меньше
  bb-тестов в count-плейне broadphase, плюс УДАРЕНИЕ false positives (y-чужие entity
  больше не считаются) → меньше напрасных ванильных сканов java-хвоста. Захват 15-25%
  остатка broadphase 8.8% + getEntities-хвост 1.85 → ~+1.4-2.7пп к ноге; в паре с
  якорями банка (a26 +12.4, a15 +9.7, chunkmono-ноги +11.5) пара ≥+20 реальна
  (субаддитивность ×3 учтена: наш вклад самостоятельный broadphase-лейн).
- **Риск**: bookkeeping add/remove/section-move (ghost-ссылки при расхождении секций).
  Ловится selfTest-инвариантом bitmap-count vs chain-count раз в 100 тиков → ERR_STRUCT
  → java broken=true → вечная ваниль (fail-closed). Out-of-window секции → OVERFLOW
  цепочка (walk всегда) — escape hatch от modded-гиперзначений y.
- **parity-план**: секции note-сайтов бит-в-бит ванильные (факт 2 выше); гейт только
  сужает superset со стороны y; x/z/y bb-тест остаётся relaxed; порядок выдачи java не
  тронут (counts-skip семантика прежняя).
- **GO** (основная нога).

### K-MAD-2 — 4КБ глобальный chunk-occupancy bloom pre-gate (ID-H06)
- **Почему**: 512×AtomicU64 слов, k=4 splitmix64, детерминированный (никаких seed'ов),
  insert-only → FN невозможен; probe в count_chunk ДО open-addressed chunk probe:
  bloom-miss (=1-0.9% на ≤3k занятых) срезает probe+hull (2-3 холодных кэш-линии) для
  никогда-не-занятых чанков (края загруженного мира, чужие dim-рексты). Один bulk
  вызов eidxFlushQuery уже фильтрует батч запросов тика ДО java-обхода — bloom
  встраивается внутрь него (закон 6: подсистема целиком, один bulk-JNI/тик,
  java-аллокаций ноль — java видит только готовые int[] counts).
- **Риск**: fpr растёт с n (плотный фиксатор) → fallthrough в exact-путь (деградация
  до сегодняшней скорости, не хуже). Стоимость на hit-пути: 4 загрузки bloom-слов ≈
  0.5 кэш-линии — шум на фоне walk цепочки.
- **parity-план**: bloom только ДОБАВЛЯЕТ rejects; каждый reject — «точно 0» по
  построению (insert-before-publish), selfTest каждые 100 тиков: chain_len>0 чанки
  обязаны probe-иться true (FN-инвариант), иначе ERR_STRUCT → disarm.
- **GO** (дешёвая добавка к K-MAD-1, один и тот же bulk-JNI).

### K-MAD-3 — Count-only per-секционный битмап БЕЗ секционных цепочек (light-вариант)
- **Почему**: только монотонный бит-флаг занятости секции (set on ADD, never clear) +
  reject «все секции диапазона пусты»; цепочка остаётся чанковой. ~40% выигрыша
  K-MAD-1 за 20% работы.
- **Риск**: монотонные биты после вымирания чанка дают ложные walk'и пустых цепочек
  (дёшево: head=0), но НЕ дают главного — walk'а только y-диапазона.
- **parity**: тот же superset.
- **PARK**: K-MAD-1 подмножает его (bitmap у K-MAD-1 и так есть); сохраняем как
  fallback-план Б при struct-отказах ноги.

### K-MAD-4 — Java-хвост: per-slices section-occupancy bitmask перед storage-loop (ванильный tail rewrite)
- **Почему**: патч EntityCollectionBySection.getEntities: перед циклом секций
  консультация java-side long[1] bitmask'а (заполняется rust'ом в том же bulk-JNI)
  → пропуск null/size-итераций. Хвост 1.85% → верх +0.3-0.5пп.
- **Риск**: второй канал данных java↔rust per-chunk (массив per-slices — либо java
  static double-bookkeeping (рассинхрон → FN → бит-в-бит ПОЛНОСТЬЮ риск), либо
  второй JNI per-chunk (нарушает экономику JNI-переходов, урок alloc_diet REFUTED)).
- **parity**: без оракула на порядок выдачи секций — риск неоправдан.
- **PARK** (пересмотреть, если после K-MAD-1/2 java-хвост станет TOP плейна).

## Дизайн имплементации (GO: K-MAD-1 + K-MAD-2)

Rust (src/entity_index.rs):
- Shard += `s_head: [AtomicI32; 64*CHUNK_CAP]` (per-секционные цепочки, окно
  SEC_OFF=32 → secY∈[-32..31] → [-512..511] блоков, vanilla dims -4..19 внутри),
  `s_sec: [AtomicI32; SLOT_CAP]` (biased+1; 0 = overflow), `sec_bits: [AtomicU64;
  CHUNK_CAP]` (монотонный occupancy bitmap), `chain_len: [AtomicI32; CHUNK_CAP]`
  (для selfTest), overflow = старый `head` (per-ki).
- ADD: same-cell+same-sec → bb update; same-cell+new-sec → SECTION-MOVE (unlink старой
  секции, push новой); new-cell → как сегодня + push в секционную цепочку.
- REMOVE: s_sec → unlink из секционной цепочки (s_sec авторитетен, note-sec
  информативен); slot в free-pool.
- BB: bb+agg update, секция не меняется (vanilla-семантика: секция меняется только
  через moveEntity-пару).
- count_chunk: bloom probe → chunk probe → sec_bits range-mask (1 AND) → hull → walk
  y-диапазонных секций (clamp(floor(minY-2)>>4..floor(maxY+2)>>4, minSec, maxSec),
  репликация ванильного байткода) + overflow.
- selfTest: каждый 100-й eidxFlushQuery — полный проход mirror'а: Σ walked ==
  chain_len per entry, bloom FN-инвариант; нарушение → статический BROKEN →
  ERR_STRUCT навсегда (java broken=true).
Java (EntityIndexOps.java): leverEnabled() принимает cmp405_eindex|cmp458_roar;
noteRemove теперь пишет реальный secY (информативно). Больше НИЧЕГО — java-плоскость
не меняется (counts-гейт тот же, NCDFE-канон: класс+Buf дефайнятся до arm, как в
cmp405 — EARLY-define паттерн d73758a3/5ecd841a соблюдён: define строго до первого
touch из retarget'ов, gate в manager несёт cmp458_roar).
Wiring (src/entity_index_manager.rs): gate принимает оба флага; ARM-маркер v2 громкий
(roaring секции/bloom/selfTest параметры).
