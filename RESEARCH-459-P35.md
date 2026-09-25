# RESEARCH-459-P35 — StepBasedCollector гибрид DDA-v2 (WILD-агент закона 11, тик-459)

Ветка: `round-459-p35` @ origin/master 0d147876. Карточка ID-P35 (RESEARCH-458-P.md,
agent-P): развитие inside_diet; разбор провала full-DDA 21/350k. Лейн: **inside alloc
31.18% alloc-семплов окна (RECON14C, s7173) / 9.20% CPU**. Карточка-прогноз: **нога
+0.5-1пп** (java_util/GC-debt −), верификатор жрёт CPU → **1/200 и только на
ноги-прогоны**. Скаффолд: `src/inside_dda_v2.rs` + P35-стаб в
`entityinside/net/minecraft/world/entity/InsideDietOps.java` (NCDFE-канон, dormant).

## ГРУНТ: что уже есть (не дублирую)

- **v1 INSIDE-DIET (TASK-332)**: body-redirect 5-arg
  `Entity.checkInsideBlocks(...)I -> InsideDietOps.checkInsideBlocks(Entity,...)I`;
  клей-диета = 1 box + 1 visitor, сам walk — РЕАЛЬНАЯ ванильная статика
  `BlockGetter.forEachBlockIntersectedBetween` (bit-exact ПО КОНСТРУКЦИИ). Деливери
  s7174/75 (min-of-2): inside-alloc 31.18->16.48% окна (−47.2%), inside-CPU −65-67%,
  НО TPS median 1.50/1.60 < harm-floor 1.60 -> REFUTED-по-TPS, rollback
  `inside_diet=0`; мосты+stage-9 = verified-инфраструктура. ГЛАВНАЯ НАХОДКА: аллок-ось
  ≠ TPS-ось — диеты продолжать только как GC-debt relief сопутствующее (закон 8).
- **DDA-v1 провал (задокументирован в InsideDietOps.java doc)**: полная walk+DDA
  транскрипция разошлась на **21/350k** random-lockstep-сцен в DDA-порядке -> REJECTED.
  Вербатим-ловушки канона (GOAL_20TPS §983): getCenter = Mth.lerp(0.5,min,max) =
  max + 0.5*(min−max) (РЕВЕРС-форма); стационарный путь = BlockPos$4 index/%-итерация
  (int-переполнение паритетно); corner-итератор Y-внешний, (|dx|<|dz|)?YZX:YXZ, старт
  по знаку дельты, extents+1 включительно; DDA-цикл пока ЛЮБОЙ tMax <= 1; выбор оси
  строго-меньше (X iff tMaxX<tMaxY && tMaxX<tMaxZ); clamp low = (double)((float)cell +
  1.0E-5f) (FLOAT-сложение), high = (cell+1.0) − 9.999999747378752E-6; stepX =
  sign==0 ? Double.MAX_VALUE : sign/delta.x; AABB.clip — статик звать вербатим.

## V2-ГИБРИД (архитектура скаффолда)

Принцип карточки: **ванильный walk = источник истины** (не трогается вообще — v1
redirect остаётся единственным исполнителем эффектов), visit-транскрипция (DDA-модель)
идёт **ПАРАЛЛЕЛЬНО как тень** с **выборочной верификацией 1/200** (offline 1M-сцен +
online на ногах), расхождение -> **one-shot disarm-латч** (паттерн batchOk nav_plane).

1. **Тень (shadow)**: на верифицируемых вызовах DDA-модель пересчитывает
   visit-транскрипцию — последовательность ячеек (walkStep-порядок) + гейт-предикаты
   per-ячейку (movedThrough/inFluid-видимость, membership в visited-сете) — и
   сравнивается с трассой, снятой с ванильного walk'а (visitor уже транскрибирует
   visit-семантику lambda$2 — см. InsideDietVisitor). Тень НИКОГДА не применяет
   эффекты -> наблюдаемая разница 0 по построению.
2. **Выборочная верификация 1/200**: детерминированный сэмплер
   `(call_ordinal % 200 == 0)` (лок-степ-воспроизводимый, без RNG-движка) —
   прецедент: JFR adaptive sampler / ObjectAllocationSample throttling
   (foojay, источник 2) — «reduce the number of events per time unit while
   maintaining statistical relevancy», онлайн-решение без отложенной буферизации.
   Вердикт-статистика (verified/matched/diverged) — DATA-PLAN для гейта ноги.
3. **One-shot disarm (batchOk-паттерн)**: один volatile-латч; первая же дивергенция /
   ERR -> тень выключается НАВСЕГДА (после disarm накладных 0), громкий лог + счётчик;
   ваниль продолжает быть исполнителем -> fail-closed. Прецедент опасности агрессивного
   редиректа этого же лейна — источник 1: lithium 0.5.3 `entity.fast_suffocation_check`
   (полный redirect suffocation-пути Entity) сломал Pehkui -> откат на менее агрессивный
   redirect (0.5.4, коммиты 7f0e2bb/548bbf1 «use less aggressive redirect»). Вывод:
   полный транскрипт-исполнитель в этом лейне имеет документированную историю
   совместимостных дивергенций — поэтому v2 держит ваниль исполнителем, а транскрипт
   растит до исполнителя ТОЛЬКО после offline 1M-сцен proof + online 1/200 без disarm.
4. **NCDFE-канон**: пока lever dormant — классы НЕ определяются, ретаргет НЕ
   компонуется, хуки НЕ регистрируются (ваниль бит-в-байт по построению); java-стаб
   добавляет ТОЛЬКО dormant-секцию P35 (флаг `P35_LEVER=false`), класс НЕ пересобран —
   include_bytes! внутри inside_diet.rs носит текущий байткод, javac-rebuild = шаг
   следующей ноги; будущий JNI-батч регистрируется RegisterNatives на
   только-что-определённый класс (define-ДО-arm).

## Сайты (следующая нога — javap-декомпиляция обязательна)

- Rust: `src/inside_dda_v2.rs` — DDA-модель-референс (вербатим-ловушки §983) +
  сэмплер + one-shot латч + selftest'ы (offline-инварианты: ось строго-меньше, tie
  tMaxX==tMaxY -> НЕ X, step=MAX_VALUE, float-clamp, stationary-путь, step-cap
  fail-closed). Модель offline-only (без JNI) — среда для 1M-сцен харнесса.
- Java: `InsideDietOps.java` — P35-секция: `static volatile boolean ddav2Ok = true`
  (batchOk-паттерн), `ddaV2Sample()` (1/200), `ddaV2Disarm(reason)` — dormant (флаг
  `P35_LEVER=false`, ни одного вызова из активного байта).
- Wiring: `src/lib.rs` — mod + activate() no-op (dormant без CRUSSTY_INSIDE_DDA_V2).

## Δ-матем (прогноз карточки, preregistered гейт ноги)

- Лейн: inside 31.18% alloc-окна / 9.20% CPU (s7173); v1-диета сняла клей (−47% alloc),
  остаток — внутренние аллокации walk'а (corner-итераторы, BlockPos, AABB.clip-буферы).
- V2-fork (после 1M-proof): транскрипт-исполнитель вместо клей+lambda-объектов
  (1 visitor+2 AABB на вызов ↓) -> java_util/GC-debt хвост −; прогноз карточки
  **+0.5-1пп к ноге** (legs-only, min-of-2, harm-floor канон v7).
- Стоимость верификатора: 1/200 × O(walk) ≈ <=0.5% лейн-CPU worst-case на верифицируемых
  прогонах; после disarm — 0. Гейт: «disarm-fires=0 на 1M offline-сцен» обязателен до
  вектор-ноги (DDA-v1 провалил именно этот гейт: 21/350k).

## Источники (реальные URL, сняты page_reader/web_search тик-459)

1. https://github.com/CaffeineMC/lithium/issues/125 — «Entity suffocation optimizations
   in Lithium 0.5.3 are incompatible with Pehkui»: агрессивный редирект
   fast_suffocation_check ломает внешние модификаторы метода; исправлен менее
   агрессивным redirect'ом (7f0e2bb/548bbf1) -> канон «ваниль = исполнитель истины,
   оптимизация — гость с disarm».
2. https://foojay.io/today/improved-jfr-allocation-profiling-in-jdk-16/ — JFR adaptive
   sampler / ObjectAllocationSample: троттлинг онлайн-сэмплера при сохранении
   статистической релевантности -> канон выборочной верификации 1/200 без буферизации.
