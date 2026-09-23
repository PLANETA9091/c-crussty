# NOISEFILL_ROOTCAUSE — TASK-421-C (тик 08:08 +08, v17, base 2d23f45)

Мандат (владелец 2026-09-23): ≥+20% pair-stable, закон 8 (ось chunk/worldgen/noise
видима игроку). Вопрос раунда: почему noise-fill/parse-плоскость нестабильна
(cha −4.5 RED … chb +20.8 exact-pair, Δrunner 4k)?

## 1. Данные (RUN-файлы round-419/420, ветки round-419-c-cha/chb/chc, round-420-c-cha/chb/chc)

| нога | код | runner_cpu_index | TPS med | ближайший якорь (runner) | pair Δ |
|---|---|---|---|---|---|
| 419 chunka | f7b0be4 | 7,098,787 | 2.30 | 2.35 @ 6,971,994 (Δ127k) | **−2.1%** |
| 419 chunkb | f7b0be4 | 8,582,258 | 2.90 | 2.40 @ 8,541,939 (Δ40k) | **+20.8% exact-pair** |
| 419 chunkc | f7b0be4 | 7,026,302 | 2.45 | 2.35 @ 6,971,994 (Δ54k) | +4.3% |
| 420 ch420a | 85f74b8 | 6,758,845 | 2.10 | 2.20 @ 6,833,921 (Δ75k) | **−4.5% RED** |
| 420 ch420b | 85f74b8 | 7,097,920 | 2.45 | 2.20 @ 6,833,921 (Δ264k) | +11.4% |
| 420 ch420c | 85f74b8 | 6,686,983 | 2.30 | 2.20 @ 6,833,921 (Δ147k) | +4.5% |

Разброс ОДНОГО И ТОГО ЖЕ кода: −4.5 … +20.8. Медианы тиров: ~7.0M-тир = +4.3..+11.4;
8.5M-тир = +20.8 (одна нога).

## 2. Root-cause (каждый пункт — из артефактов, не гипотеза)

**RC1 — GEN-ось инертна на фикстуре.** BUCKETS_3 ch420b: `worldgen/noise (kernel) = 42
сэмпла = 0.0%` soak-CPU. Мир MineShield-3 ПРЕГЕНЕРИРОВАН: forceload 9216 чанков =
PARSE сохранённых чанков, НЕ worldgen. TASK-108 PROGRESS-4 (b4ecb3c) уже фиксирoвал:
production noise идёт через NoiseChunk.fillSlice→NoiseInterpolator, а на прегене не
идёт вообще. noise-fill патчи вычисляются и ре-transform'ятся (маркеры ×3), но работы
не имеют → вклад ≈ 0. «Arm-timing vs GEN-фаза»: GEN-фазы в soak НЕТ.

**RC2 — parse-бурст BOOT-only, поллы его не перекрывают.** Хронология ch420b:
22:13:14 Done(15.6s)+join 4 фейк-игроков → 22:13:27-29 forceload 36 команд →
22:13:43-45 parse-бурст (stats sections=8192 @22:13:43, 16384 @22:13:45 — ВСЕ 16384
миссов за ~2-3 сек) → 22:14:42 population inject done (56.3s) → ПЕРВЫЙ TPS-полл
22:15:55. Поллы (6 шт, 22:15:55→22:21) весь parse-бурст пропускают. Третья stats-строка
(24576 миссов) не наступила НИ РАЗУ → soak-миссов <8192, путь тих.

**RC3 — плато hit-rate 63-65% СТРУКТУРНОЕ, не cap-thrash.** ch420a/b/c: rate 63%/64%/64%,
`evicted=0` при cap 16384. Гипотеза 420-C («плато 64% = cap-1024 thrash») ОПРОВЕРГНУТА
её же логами: переполнений нет, миссы = уникальные теги (поверхностные/краевые секции,
каждая декодируется 1 раз). Кэш работает как задуман; добить hit-rate выше — нечем.

**RC4 — ramp-шум поллов ±5-8пп доминирует.** Soak-TPS — это РАМПА 1.6→2.7 (выгорание
103k item-сущностей, age 6000t): ch420a [1.6,1.9,2.1,2.4,2.5], ch420b [1.7,2.0,2.3,2.6,2.7],
ch420c [2.0,2.3,2.6,2.6] — ОДИН И ТОТ ЖЕ код. Медиана 5-6 поллов зависит от положения
рампы в окне опроса; в 7.0M-тире это ±5-8пп — БОЛЬШЕ эффекта рычага. Корреляция
TPS↔runner_cpu_index внутри тира: 2.10@6.76M … 2.45@7.10M.

**RC5 — порядок union-гейтов {cmp420_chunk2, cmp420_colpush} исключён.** Все три гейта
(noise_fill.rs lever_gate, chunk_parse.rs enabled(), run_world3.sh case) — независимые
проверки строкового равенства на register-времени; упорядочивания нет.

**RC6 — кросс-чек мегой: сигнал РЕАЛЕН и ≈+6-8пп, когда доходит до поллов.** cp420
(colpush-only) 2.55-2.65 @ 6.7-7.0M vs mg-ноги (colpush ⊕ chunk-parse ⊕ noise) 2.75/2.75/2.85
@ 6.4-7.0M → надбавка chunk-союза ≈ +0.15-0.2 TPS, СТАБИЛЬНО ×3. Механизм: parse-бурст
33.38% alloc-байт окна → его срез = меньше young-давления в population-инжект (56s) и
ранних поллах (ParallelGC, gc_tune=3) → ранняя рампа выше → медиана выше.

## 3. Вердикт root-cause

Нестабильность = (RC1+RC2) эффект вообще не доходит до поллов + (RC4) ramp-шум тира
±5-8пп > эффект; (RC3) добивание кэша бесполезно; (RC5) гейты ни при чём; (RC6) СТАБИЛЬНАЯ
составляющая = GC-debt relief ≈ +6-8пп на носителе, где scene ускорен мерж-1/2.

## 4. Фиксы TASK-421-C (stabilization + новый срез)

- **S1**: cmp421_chunk STRICT-OR во ВСЕ три гейта (один id раунда, никакого рассева).
- **S2**: check_blobs_sync теперь покрывает noise-блобы (GEN-ось = единственное
  семейство мостов, не аудируемое гейтом — дыра ×93-класса закрыта; major-52 пин +
  маркеры методов).
- **S3**: cap 16384/evict-half остаются (безвредны), плато задокументировано как
  структурное — кап-тюнинг больше НЕ трогаем (теория исчерпана).
- **S4 — НОВЫЙ СРЕЗ** (перенос эффекта в поллы): (а) biomes-сайт parse-кэша (второй
  decode-сайт lambda$parse$7 → второй кэш, MISS через reflection-replica) — добор
  GC-debt relief в бурсте; (б) chunk-send serialization cache (PlayerChunkSender/
  ChunkMap срез — revision-keyed переиспользование сериализованных чанк-пакетов;
  единственный chunk-lane, живущий В soak: трекер+отправка 2.0-2.2% сцены, RECON-36).

## 5. TASK-430-A: root-cause wgen-l3 CI-fail (run 35865655409) — RC7 BOOT-POISON

Симптом: SEEN_DONE=0, boot не дошёл до Done за 600s, FIXTURE-VALIDITY INVALID. server-stdout:
- последний прогресс: "[13:20:56 INFO]: [PluginInitializerManager] Bukkit plugins (2):" — main умер
  на следующем шаге.
- java.lang.NoClassDefFoundError: Could not initialize class net.minecraft.core.registries.BuiltInRegistries
  at net.minecraft.server.Bootstrap.bootStrap(Bootstrap.java:47) ... Caused by:
  ExceptionInInitializerError: IllegalArgumentException: Not bootstrapped (called from registry
  minecraft:game_event) [in thread "Thread-22"] ... at DensityFunctions.<clinit>(DensityFunctions.java:32)
  at Class.forName0 ... — т.е. <clinit>-цепочку DensityFunction→DensityFunctions→BuiltInRegistries
  запустил ФОНОВЫЙ поток noise_fill early-arm.

Механизм: TASK-429-A early-arm сменил каденс с "sleep 10s → force" на "force с первого прохода, 2s poll".
force_load_kernel_class зовёт Class.forName(target, initialize=TRUE) → если полл успевает ДО того как
main дойдёт до Bootstrap.bootStrap() (окно 4-13s после attach), BuiltInRegistries.<clinit> падает на
poll-потоке (checkBootstrapCalled → false), класс ЯДОВИТ навсегда (ExceptionInInitializerError), main
умирает NoClassDefFoundError → JVM висит на фоновых потоках. l1/l2 выиграли гонку, l3 проиграл (1/3).

Фикс (rust-only, блобы НЕ тронуты): improved_noise::force_load_kernel_class_lazy — Class.forName
initialize=FALSE (define есть → ClassFileLoadHook ловит pristine-байты → phase-0 по find_class
(только INITIALIZED-классы) разрывается, когда СЕРВЕР сам инициализирует DensityFunctions в
bootstrap-фазе — в l1 sightings на 8s post-attach, до Done). <clinit> навсегда остаётся за main.
Побочный бонус: arm-латентность ≤ boot marker (не Done+15-25s как до 429-A).

Валидация: cargo check --lib PASS, cargo test --lib 302/0. Ветка: round-429-a-wgen (след. коммит).
