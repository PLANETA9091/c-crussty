# RESEARCH-459-H07 — ID-H07 Hilbert-порядок ХРАНЕНИЯ секций (cmp-заготовка, STRICT dormant)

Агент: TASK-459-77 (WILD, закон 11 тика-459, v18.2/12e). Ветка `round-459-h07` из master `0d147876`.
Идея свободна из ×457 (ростер ×459 строка 77). Тема: Hilbert-порядок сущностей в storage-сканах
секций для локальности кэша. ВЕРДИКТ-КОНТРАКТ: порядок ВЫДАЧИ остаётся ванильным ⇒ H07 =
ПРЕД-СОРТИРОВКА индексов ВНУТРИ секции БЕЗ изменения последовательности вызовов (reorder
storage, НЕ выдачу). STRICT dormant: scaffold без define/хуков/JNI.

## 1. ИНТЕРНЕТ-РИСЁРЧ (curl, 2026-09-25; ≥2 URL)

### U1 https://en.wikipedia.org/wiki/Hilbert_R-tree
- Hilbert R-tree использует space-filling curve (Hilbert), чтобы навязать ЛИНЕЙНЫЙ порядок
  многомерным объектам внутри узлов: «use space-filling curves, and specifically the Hilbert
  curve, to impose a linear ordering on the data rectangles … better ordering of multidimensional
  objects in the node».
- LHV (Largest Hilbert Value) кластеризует прямоугольники в узлах: объекты с близкими Hilbert-
  значениями лежат в одном узле/странице (диск-локальность канона; для нас — кэш-локальность).
- Packed-вариант (bulk-load) = все узлы полной ёмкости → минимум памяти, максимум локальности.

### U2 https://en.wikipedia.org/wiki/Hilbert_curve
- «Both the true Hilbert curve and its discrete approximations are useful because they give a
  mapping between 1D and 2D space that preserves locality fairly well» — соседние d → соседние
  клетки (чебышёв-дистанция 1); обратное не всегда верно. Наш selfTest проверяет оба свойства
  (биекция 256 клеток bits=4 + windows(2) чебышёв-гейт).

### U3 https://docs.rs/geo-index/latest/geo_index/ (+ module rtree)
- geo-index = «packed, immutable, zero-copy spatial indexes»; «tends to be faster than dynamic
  implementations like rstar»; bulk-load ONLY («As an immutable index, only bulk loading is
  supported»); координаты i8..f64 (2D), индексы u16/u32.
- API: `RTreeBuilder::<f64>::new(n)` → `add(min_x,min_y,max_x,max_y)` → `finish::<HilbertSort>()`;
  сортировки HilbertSort/STRSort(+rayon). МОДУЛЬ rtree 0.4.0 подтверждён.
- Insertion-index поверхность: `search` возвращает insertion-индексы; leaf-уровень хранится
  первым → `indices()[0..num_items]` = insertion-индексы в Hilbert-порядке (leaf-layout
  flatbush-канона, сверено agent-J в src/rtree/builder.rs finish + sort/hilbert.rs).

### U4 https://github.com/mourner/flatbush
- «An efficient implementation of the packed Hilbert R-tree algorithm … fast spatial queries on a
  very large number of objects (e.g. millions)» — upstream-канон сортировки, который портирует
  geo-index (mourner = автор RBush/flatbush).

### U5 https://crates.io/crates/geo-index (агент-J ×458, RESEARCH-458-J.md)
- 0.4.0, 159k DL, обновлён 2026-09-15; cargo add + cargo check = 0 err (22:2x тика-458) —
  КОНСУМАЦИЯ: компиляция geo-index 0.4.0 уже проверена в проекте (RTreeBuilder + HilbertSort,
  leaf indices = hilbert-порядок) — база будущего bulk-load плана в wiring-фазе.

## 2. ДЖАВАП-КОНТРАКТ ЦЕЛИ (LEDGER-459-L11, ≥5 чисел)
- EntityCollectionBySection.getEntities = plain count early-out + y-clamp (minY−2.0/maxY+2.0) +
  storage-СКАН с null-дырами — последовательный обход массива слотов. getHardCollidingEntities =
  z-outer/x-inner region-grid (SHIFT=5) + FULL-гейт.
- Ванильный EntitySectionStorage = minY−4.0/maxY+0.0 LongSortedSet-порядок секций.
- Лейн broadphase-ридеров = 9.36% chk-14 (9647/103062 сэмплов; 16-нога 9.87%, депресс 8.82%,
  anchor33 14.68%).
- Reader-core = getEntities 1120 + getHardColliding 926 + intersects 871 + CHM/VarHandle/getNode
  ≈820 = 4.0% CPU = 43% лейна; voxel-хвост 57% вне компетенции.
- Pop бенча: SoA-живые ~38-40k мобов (eqsnap2 ×457-C2: mobSlots 274 → senseSlots 38540); плотность
  ~0.023/м², бокс follow-range 80×80 ≈ 50-150 кандидатов.

## 3. ДИЗАЙН (reorder storage, НЕ выдачу)
- Ключ слота секции: `key(x,y,z) = hilbert2D(x,z,bits=4) << 4 | (y&0xF)` — 2D-мажор (geo-index
  2D-only контракт; broadphase-боксы прунятся по x/z, вертикаль не маржируется), y — минор.
- План секции = СТАБИЛЬНАЯ сортировка слот-индексов по ключу: `order[r]` = слот на ранге r скана.
- Паритет-гарды (бит-в-байт оракул): (1) биекция перестановки; (2) мульти-множество ключей до/после
  == (same entities, same counts, count-early-out не меняется); (3) стабильность двойников
  (одинаковые координаты не переупорядочиваются); (4) порядок выдачи ВЫЗОВОВ не меняется вовсе —
  меняется только то, какой слот лежит на каком физическом индексе.
- Rust: src/hilbert_storage_order.rs (план+гарды+статы+selfTest×3). Java-зеркало (stub, vanilla-
  free, plain-паттерн без indy): hilbert/java/net/minecraft/world/entity/HilbertStorageOps.java —
  будущая lockstep-цеза план-rust == план-java индекс-в-индекс ДО любого define (NCDFE-канон).
- STRICT DORMANT: env-lever `CRUSSTY_HILBERT_STORAGE` OFF by default; register() = dormant-notice;
  ноль JNI/define/байт-хуков — модуль невидим в рантайме по построению.

## 4. CAPTURE-МАТ (прогноз, dormant — не эффект)
- Адресуемая база: reader-core 4.0пп CPU (getEntities-скан — главный последовательный потребитель).
- Захват Hilbert-реордера на кэш-миссиях последовательного скана: 15-30% (U1/U2 locality; packed-
  layout канон) → Δ +0.6..+1.2пп; РЕАЛИСТИЧНЫЙ +0.8пп. Потолок левериджа = +4.0пп (весь
  reader-core; 100% захват недостижим — AABB-математика доминирует, ×458-J: speedup 0.73-1.35×).
- STRICT dormant: тик-эффект = 0пп по построению (ни один вызов не меняется); числа — прогноз для
  wiring-фазы с ARM-маркером.

## 5. PREREGISTERED ГЕЙТЫ (wiring-фаза)
G1 lockstep-цеза: план rust == java бит-в-байт на ≥1000 детерминированных секциях (вкл. двойников,
null-дыры); G2 ARM-маркер hilbert_storage в логе до первого реордера (урок-408: спящие гейты);
G3 NCDFE T1=0 (EARLY-define, fa9054d9 ARM-AFTER-DEFINE прецедент); G4 fail-closed: гард-отказ ⇒
план не публикуется (ваниль), GUARD_REJECTS счётчик; G5 A/B min-of-3 band 6.0-9.5M, Δ≥+0.8пп
гейт-бар; G6 популяция-паритет 140-165k + NCDFE=0.

## 6. СТАТУС
SCAFFOLD (law-11 финал = {run id, ветка+SHA, вердикт-число}): rust-модуль + java-стаб + wiring
lib.rs (register-only) + cargo test 332 passed (3 новых) + javac --release 21 selfTest=true.
Wiring-фаза (отдельный коммит): bulk-load план из живых слотов, java-лестница реордера, lockstep.

## ИСТОЧНИКИ
U1 https://en.wikipedia.org/wiki/Hilbert_R-tree
U2 https://en.wikipedia.org/wiki/Hilbert_curve
U3 https://docs.rs/geo-index/latest/geo_index/
U4 https://github.com/mourner/flatbush
U5 https://crates.io/crates/geo-index
