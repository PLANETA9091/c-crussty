# RESEARCH-459-P25 — Noise-router 2D-cache (TASK-459-65, WILD закон-11, тик-459)

Карточка: ID-P25 из RESEARCH-458-P.md (origin/round-458p-ideas): флет-кэш 2D-срезов
континентальность/эрозия/температура (XZ→double[]), переиспользуемый между 3D-колонками
одной чанк-колонки; чистый java-мемо + epoch-инвалидация; бит-в-байт значения (тот же
порядок вызовов шума); шум не мутируется миром → инвалидация только seed/размер;
burst-генерация −20-30% шума; резервный GC-debt член пары. Паттерн кэша:
src/proto_blend_cache.rs (B10, observation-only prototype). Ось: gen-axis (инертно на soak).

## 1. Механика (по источникам)

**Noise router** [S1, minecraft.wiki]: NoiseRouter — коллекция density-функций; поля
`temperature`, `vegetation`, `continents`, `erosion`, `depth`, `ridges` обслуживают ТОЛЬКО
biome placement / aquifer и НЕ влияют на форму рельефа (рельеф = `final_density`).
Значения считаются per block position ⇒ для климатических полей функция зависит только
от (x,z) внутри фиксированного NoiseRouter (seed+настройки): y-координата не входит в
аргументы climate-семплера — это и есть окно для 2D-мемоизации.

**Ядро** [S2, ForgeJavadoc NoiseChunk]: в NoiseChunk уже живут вложенные кэш-обёртки:
`Cache2D` (однослотовый мемо на экземпляр функции — поля `lastSamplingColumnPos` +
`lastSamplingResult`, видно по @Shadow в C2ME-миксине [S3]), `CacheAllInCell`
(заполняет double[] на интерполированную ячейку), `CacheOnce`, `FlatCache`
(флет double[] на функцию; поля `blendAlpha`/`blendOffset` суть `NoiseChunk.FlatCache`),
`NoiseInterpolator`; плюс `NoiseRouter.cachedClimateSampler(Climate.Sampler, ...)`.
Итого: флет-срез (XZ→double[]) в ядре ЕСТЬ только для blend-пары; климатические 2D
функции обходят мир через однослотовый Cache2D ⇒ thrash при (а) чередовании разных
2D-функций в одном стеке вычислений, (б) повторных заходах в колонку: интерполированные
ячейки пересчитывают угловые колонки ×8, blend/density хвосты семплируют те же (x,z)
повторно ⇒ каждый промах = полный пересчёт октав.

**C2ME** [S3, RelativityMC/C2ME-fabric]: направление индустрии — (1) выставить кэш
density-функций наружу duck-интерфейсом `IFastCacheLike` (`c2me$getCached/c2me$cache`
per-колонка + БАТЧ-вариант `c2me$getCached(double[] res, int[] x, int[] y, int[] z)`),
sentinel промаха `CACHE_MISS_NAN_BITS` (MixinChunkNoiseSamplerCache2D.java); (2) собрать
density-функции в AST и генерить батч-кернелы вплоть до OpenCL (McToAst,
CacheLikeNodeOpenCLCEmitter). Yarn-имена: ChunkNoiseSampler = Mojang NoiseChunk,
DensityFunctionTypes.Wrapping = обёртка кэша (MixinDFTWrapping.java). P25 берёт то же
семантическое ядро (флет-срез + промах-сентинел), но БЕЗ AST-компиляции: чистый
java-мемо поверх существующих ядерных обёрток, бит-в-байт.

## 2. P25-дизайн (scaffold src/noise2d_cache.rs)

- Скоуп: на чанк-колонку (NoiseChunk) 5 слотов (temperature/vegetation/continents/
  erosion/ridges — поля роутера по [S1]); каждый слот = флет double[16*16+] срез
  локальной колонки, индекс `(local_z << 4) | local_x`, битовая карта заполненности.
- Первый заход (MISS) = ванильный пересчёт через delegate (рекординг = тот же порядок
  вызовов шума, что ваниль на первом вычислении), повторные заходы (HIT) = чтение
  битов из среза БЕЗ вызовов шума. Потребитель видит ту же последовательность double,
  что ваниль: бит-в-байт (законы кэша как в proto_blend_cache §self-test).
- Переиспользование между 3D-колонками: все y-слои и все интерполированные ячейки
  чанк-колонки читают один срез ⇒ на (функцию, колонку) ровно 1 пересчёт вместо ~8-24.
- Чистый java-мемо: bridge-класс `Noise2DMemoOps` живёт в kernel loader (как
  ImprovedNoiseNativeOps — resolve ядровых классов, parent-first), JNI в hot path нет.
- **Epoch-инвалидация**: ключ кэша = (seed, dimension min_y/height, router-identity);
  шум не мутируется миром ⇒ инвалидация ТОЛЬКО при смене seed/размера (перезапуск
  сервера / смена измерения) — один long-epoch на чанк-колонку, проверка одним int-cmp.
- **NCDFE-канон**: EARLY-define Noise2DMemoOps (+nested) в kernel loader ДО первого
  обращения (паттерн improved_noise/area_map: define на тихом activation-воркере после
  load ядрового класса, class-version guard `--release 8`); НОЛЬ class definitions внутри
  retransform-callback (ClassReader COMPUTE_FRAMES → Class.forName → deadlock, см.
  заголовок src/improved_noise.rs); fail-closed: любой сбой — ванильный Cache2D путь.

## 3. Parity: порядок вызовов шума

1) Рекординг-дисциплина как в src/noise_fill.rs: MISS-путь — ванильный
   `fillAllDirectly`/`compute` эквивалент (delegate.compute(ctx)) на реальном провайдере
   ⇒ паритет по построению на первом заходе; HIT-путь — возврат сохранённых битов,
   вызовов шума ноль (значение уже бит-в-байт из MISS).
2) Rust-референс-модель мемо (в scaffold, фаза-1 selftest): детерминированный xorshift
   прогон прямых вызовов vs мемо-реплей, сравнение `f64::to_bits` 1:1 + счётчик HIT/MISS;
   ядерная паритет-проверка (фаза-2, TODO): 10k случайных (slot,x,z) через живой мост
   old-vs-new по образцу proto_blend_cache::blend_parity_selftest.
3) Порядок СЛОТОВ не меняется: мемо сидит в Cache2D-слое (выше batch-fill noise_fill.rs),
   не дублирует его кэш; координация с noise_fill.rs задокументирована как риск R1.

## 4. Δ-оценка

- Карточка: burst-генерация −20-30% вызовов шума; инертно на soak (ген-ось).
- Адресуемый лейн: 2D-климатика в ген-бёрсте (пересчёты октав на Cache2D-промахах);
  консервативный прогноз нога +0.3-0.8пп на gen-burst (chunk-gen CPU) при захвате
  30-50% пересчётов; потолок = доля 2D-пересчётов в шуме бёрста (замер — фаза-2 A/B
  min-of-2/3 по закону-16). GC-debt: минус повторные промах-вычисления, аллокаций мемо
  почти ноль (флет double[] на колонку ~10KB на 5 слотов) ⇒ резервный член GC-debt пары.

## 5. Риски

- R1: двойной кэш с noise_fill.rs (DensityFunctions$Noise.fillArray уже батчит) — мемо
  обязан жить СТРОГО выше fillArray-слоя, флаги кэша совпадать; иначе двойной учёт.
- R2: тред-модель — NoiseChunk однопоточный на генерацию ⇒ мемо без CHM/атомиков на
  hot path; чужой тред (spawn chunk prefill) → per-NoiseChunk владение, без шаринга.
- R3: маппинги Paper 1.21.x Mojang — точную поверхность (NoiseChunk.Cache2D поля,
  NoiseRouter-акцессоры) подтвердить `javap -p -c` на живом ядре ДО любого патча
  (TODO(B10-стиль, фаза-2); в фазе-1 только probe существования классов/методов).
- R4: кэш по колонке = тривиальный ключ [карточка], но blend-хвост (Blender) тоже семплирует
  климат — убедиться, что срез не протекает через границу чанка (blendOffset уже FlatCache).

## 6. Источники

- S1: https://minecraft.wiki/w/Noise_router — поля роутера (temperature/vegetation/
  continents/erosion/depth/ridges), «не влияют на форму рельефа», per-block-position семантика.
- S2: https://nekoyue.github.io/ForgeJavaDocs-NG/javadoc/1.19.3/net/minecraft/world/level/levelgen/NoiseChunk.html —
  вложенные Cache2D/CacheAllInCell/CacheOnce/FlatCache/NoiseInterpolator; blendAlpha/blendOffset = FlatCache; cachedClimateSampler.
- S3: https://github.com/RelativityMC/C2ME-fabric — c2me-opts-dfc MixinChunkNoiseSamplerCache2D.java
  (IFastCacheLike, однослотовый мемо + батч-вариант, CACHE_MISS_NAN_BITS), MixinDFTWrapping.java, McToAst/OpenCL-emitters.
- S4: https://maven.fabricmc.net/docs/yarn-1.21.4+build.1/net/minecraft/world/gen/noise/NoiseRouter.html —
  yarn-поверхность NoiseRouter (erosion()/etc. record-компоненты).

## 7. Что в этом коммите (фаза-1 scaffold, dormant)

- `src/noise2d_cache.rs` — observation-only scaffold по паттерну src/proto_blend_cache.rs:
  gate `CRUSSTY_NATIVE_NOISE2D_CACHE` OFF-by-default (dormant-дисциплина 3a270ee),
  byte-hook = capture pristine NoiseChunk байт, PATCH_ENABLED=false (патч не подаётся
  никогда в фазе-1), probe ядровых классов (NoiseChunk/NoiseRouter/Climate$Sampler),
  Rust-референс-мемо SliceCache2D + детерминированный bit-exact selftest.
- java-мемо stub: в заголовке модуля (Noise2DMemoOps, EARLY-define, NCDFE-канон).
- Wiring в src/lib.rs: `mod noise2d_cache;` + `noise2d_cache::register()/activate()`
  рядом с proto_blend_cache — оба no-op без гейта ⇒ dormant = byte-identical классы.
