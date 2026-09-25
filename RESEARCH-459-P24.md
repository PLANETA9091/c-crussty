# RESEARCH-459-P24 — Noise octave scratch-pool (thread-лок пул флет-аккумуляторов)

TASK-459-64 · WILD-агент закона 11 · тик-459 · c-crussty v18.2 · ветка `round-459-p24`
Идея: [ID-P24] из RESEARCH-458-P.md (origin/round-458p-ideas). Класс: moonshot / gen-alloc carrier
(0% soak-CPU; GC Full=9 vs банк 7 — инертно на soak ×421-C).

---

## 1. Механика

Thread-лок пул флет-аккумуляторов (`double[]`/`Vec<f64>`) для октавных проходов
ImprovedNoise/PerlinNoise/NormalNoise: октавные интерполяционные массивы переиспользуются
между октавами/колонками одной ген-колонки вместо свежих аллокаций на каждый вызов.

* **НЕ дублирует noisesimd** (cmp457_noisesimd): тот ускоряет ВЫЧИСЛЕНИЕ октав (SIMD-kernels),
  этот убирает АЛЛОКАЦИЮ вокруг октав. Семейства lever'ов не пересекаются.
* **НЕ дублирует fill-family** (agent-D, src/noise_fill.rs): тот делает whole-body свопы
  `fillArray` (DensityFunctions$Noise / ShiftNoise), этот не трогает fill-сайты вообще.
* Дисциплина lever_scoped_allow у noisesimd принята: строгий env-гейт, всё спит при пустом
  окружении (dormant = бит-в-байт ваниль, hopper-jar rule).
* **Бит-в-байт**: буфер заполняется ЦЕЛИКОМ до первого чтения — переиспользуемый блок не
  может протечь в результат ни при каком порядке октав (никаких «хвостов от прошлой колонки»).
  Значения шума совпадают побитно; меняется только происхождение памяти.
* GC-debt механизм: gen-сцены (pregen/exploration) — единственный сценарий, где столбец
  NoiseChunk рождает сотни тысяч короткоживущих массивов в TLAB-младших поколениях;
  Full GC счётчик (9) уже за банком (7) на soak-профиле ×421-C. Снятие доли alloc-потока
  на ген-сценах = monotone-член оси на прегене, на soak-CPU инертно (0% soak-CPU).

## 2. Сайты (по карточке)

| Сайт | Роль |
|---|---|
| `src/noise_scratch_pool.rs` | **гейт + сам пул** (v0 scaffold — этот тик) |
| `src/improved_noise.rs` | интеграция-точка: ветка bridge-хэндлов ImprovedNoiseNativeOps/nativeNoise — октавные аккумуляторы колонки (заглушка-описание — этот тик) |
| `src/perlin_noise.rs` | будущая фаза-2: тот же ретаргет для PerlinNoise-октав (не тронуто) |
| `noise/` kernel-shapes | будущая фаза-2: kernels берут scratch из пула вместо fresh Vec (не тронуто) |

## 3. Web-рисёрч (3 источника, проверены fetch'ем)

1. **Noisium** (Steveplays28) — конкурент-мод, оптимизирует worldgen-функции, которые не
   трогают другие моды: `NoiseChunkGenerator#populateNoise` (прямая запись block states в
   palette storage в обход абстракций), + «block state sampling speed» оптимизация.
   Ключевое заявление: **«full 1:1 parity with vanilla Minecraft world generation»** —
   подтверждает канон проекта: worldgen-плоскости обязаны сохранять побитную генерацию,
   ускорять можно только ОРГАНИЗАЦИЮ работы (память/аллокация/обход абстракций), не числа.
   URL: https://github.com/Steveplays28/noisium (README, raw-fetch 2026-09-25)
2. **J.-P. Bempel, «When Escape Analysis fails you?»** — разбор, почему C2 escape-analysis
   НЕ скаляризует массивы (varargs-массив `Objects.hash`, массивы с независимо
   отслеживаемыми элементами) в реальных горячих местах. Прямо закрывает риск-вопрос
   карточки «escape-analysis мог уже скаляризовать»: для МАССИВов это не гарантировано —
   сначала alloc-профиль (v0 = observation-only + счётчики), не вслепую.
   URL: https://jpbempel.github.io/2020/08/02/when-escape-analysis-fails-you.html
3. **J. Humesine, «Object Pool Design Pattern»** — канонические trade-offs пула:
   + «GC pressure reduced, especially in GC languages»; − memory bloat (idle-объекты
   удерживают память), pool exhaustion, забытые release = утечки, overuse на дешёвых
   объектах → «перед внедрением — измерить узкие места». Отсюда: кап по байтам на тред,
   failure-open (переполнение пула = обычная аллокация), счётчики в v0.
   URL: https://jhumelsine.github.io/2025/11/28/object-pool.html

Кэш выдачи поиска: /tmp/p24_q1..q8.json (сессия тик-459).

## 4. Parity

* Значения шума **бит-в-байт**: порядок/формулы октав не меняются; переиспользуемый буфер
  перед чтением переписывается целиком (dispose-путь без zeroing легален именно поэтому).
* Fill-family (noise_fill) не затрагивается; ванильный порядок вызовов шума сохраняется
  (тот же инвариант, что у noisium: parity-by-construction, spark-профили в README).
* Self-проверка фазы-2: A/B min-of-2/3 lockstep-прегена (канон закона 16) + счётчик
  re-use должен показывать >=1 re-use на колонку, иначе lever выключается (мёртвый пул).

## 5. Δ (прогноз, канон ×421-C)

* **GC-debt на ген-сценах: +6-8пп** к носителю (octave-аккумуляторы — доминирующий
  короткоживущий объект NoiseChunk-колонки; снятие 30-60% alloc-байтов сцены).
* Monotone-член оси на прегене (pregen-сцена = чистый ген-поток, 0 soak-CPU-цены).
* v0 (этот тик) НЕ даёт Δ — scaffold + гейт + счётчики; Δ появляется после фазы-2
  (ретаргет сайтов) и только при живом alloc-профиле.

## 6. Риски

* **EA-скаляризация**: если C2 уже устраняет массивы на наших сайтах — пул мёртвый.
  Порог: счётчики re-use в v0; ноль живых re-use в прегене → lever закрыт (карточка
  требовала «сначала alloc-профиль» — v0 это и есть профильный инструмент).
* **Memory bloat / exhaustion / leaks** (источник 3): кап пула на тред
  (MAX_POOLED_BYTES), переполнение = failure-open (обычная аллокация, без lock-очередей).
* **ThreadLocal × region-threads**: worker-треды живут долго → пул не вымеряет; при
  рестартах тредов память уходит вместе с тредом (не течёт, но стагнирует) — кап решает.
* **NCDFE-канон**: в v0 никаких java-классов не определяется. Фаза-2: ops-бридж
  определяется EARLY на тихом activation-worker в loader кернела (паттерн
  improved_noise: define -> compute patch -> один retransform; ноль определений классов
  внутри byte-hook callback — COMPUTE_FRAMES deadlock).
* Конфликт с noisesimd-блобами: фазы независимы (pool ≠ kernel math), но общий файл
  `noise/` — мерж-порядок: этот lever идёт последним и не трогает существующие блобы.

## 7. Гейты (v0)

* env `CRUSSTY_NOISE_SCRATCH_POOL` (1/true/on/yes → on). **OFF by default** — спящий
  модуль: register() не регистрирует хуков (урок dormant-gate-leak 3a270ee), activate()
  не делает ничего. Kernel-policy ключ в v0 не вводится (нет арма сайтов).
* Wire: `mod noise_scratch_pool;` + `register()`/`activate()` в lib.rs по образцу
  proto_blend_cache (B10-прототип, observation-only by construction).

## 8. Что попадает в коммит (12e wiring)

1. `src/noise_scratch_pool.rs` — гейт + функциональный thread-лок пул (ScratchGuard,
   re-use счётчики, кап) + dormant register/activate.
2. `src/lib.rs` — регистрация модуля (register/activate), dormant при пустом env.
3. `src/improved_noise.rs` — интеграция-заглушка: документированная точка фазы-2
   (октавные аккумуляторы bridge-хэндлов), NCDFE-канон, без изменения логики.
4. `RESEARCH-459-P24.md` — этот файл.

## 9. Факт самотеста v0 (rustc -O, изолированный бинарь, 2026-09-25)

* mode=off (гейт пуст): stats=(3, 0, 0, 0, 0) — счётчики заморожены, пула нет, паритет
  обычной аллокации (dormancy zero-cost подтверждена).
* mode=on: stats=(19, 1, 18, 9, 9) — тёплый 64-len буфер переиспользован (reuse>0),
  9 возвратов в пул, 9 кап-дропов (16×128KiB одновременных против капа 1 MiB),
  oversize 200k в пул не возвращается (plain free).
* cargo check --lib на worktree: 0 errors (после коммита — CI прогон).
