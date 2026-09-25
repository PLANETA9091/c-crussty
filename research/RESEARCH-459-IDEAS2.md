# RESEARCH-459-IDEAS2 — crazy-пакет C-X7..C-X15 (TASK-459-92, инфра-агент закона 11, тик-459 ideas-scout-2)

Миссия: второй WILD-пакет безумий (продолжение C-X1..C-X6) на internet-рисёрче радикальных
техник: data-oriented design/SoA, JVM escape analysis, off-heap storage, field relocation,
invokedynamic caching, Valhalla-подобное флаттенинг-эмулирование. Все идеи STRICT dormant
(lever `cmp459_cxN`), порядок выдачи бит-в-байт, NCDFE-канон, javap-грунт, preregistered гейты.

## 0. Лейн-карта входа (GOAL ×459 / LAB_LEDGER)

| Лейн | Число | Вектор роста | Направление пакета |
|---|---|---|---|
| inside_volatile | **16.6% РОСТ** (12.01→16.63/16.30 на RED chkmono457-11/12) | гипотеза B: secWrite-бампы → volatile MISS ↑ | C-X7, C-X11, C-X14 |
| java_util | **9% РОСТ** (7.01→8.96/8.73) | CHM.get кормит invalidation-волна | C-X8, C-X10, C-X12, C-X15 |
| broadphase | 8.8-10.9% | push/collide/query секции | C-X13, C-X8, C-X14 |
| paletted | 5-7% (6.4-7.0: PC.get 4.48 + SimpleBitStorage 1.50) | читатель горячий | C-X9 |
| items | 0.00-гейт | ЗАКРЫТ (канон) — пакет НЕ трогает items-плоскости | — |

Web-рисёрч: 7 поисковых сессий (z-ai web_search), 15 URL верифицированы curl 200 (§12).

## 1. C-X7 — SoA INSIDE-probe batch (data-oriented transform volatile-лейна) ★ТОП-1
- **Механика**: внутри checkInsideBlocks-батч candidate-superset конвертируется в
  struct-of-arrays: packedX/Y/Z → int[], blockStateId → int[]; все probe-чтения становятся
  plain array-loads по последовательным индексам (1 кэш-линия = 16 проб), volatile/CHM-чтения
  выносятся в строгий java-хвост по dirty-list мутантов. Композируется с P31 INSIDE-BATCH
  (P31 = bulk-JNI transport, C-X7 = layout внутри плоскости — дизъюнкт оси).
- **Сайты**: checkInsideBlocks (TOP-1 компо-носителя 12-16.6%), соискатели — те же, что P31
  (THRESH=512/bucket, buildPlan-порт CollideBatchOps); БЛОБ-гейт javap flat==nested 10/10.
- **Parity-план**: STRICT superset-гейт кандидатов (как CX4-канон FL_EVENT_ABORT/UNKNOWN refusals);
  dirty-list мутантов обязателен (P31-канон); бит-в-байт порядок выдачи; offline lockstep A/B min-of-2.
- **Прогноз Δ**: лейн 16.6 × захват 15-25% = **+2.5..+4.1пп** к ноге; потолок +5.0пп
  (композиция P31 ⊕ C-X7); на ванильных якорях 0 (плоскость dormant).
- **Риск**: hard-colliding фикстура (item_frame×2714) глушит fast-path (eqsnap2-ценз) — гвардия
  в перманентном ваниль-фолбэке; AIOOBE=0 гейт на SoA-индексы; band [6.0-9.5]M.
- **URL**: https://www.dataorienteddesign.com/dodbook/ (SoA/контакт данных, гл.3-4) ·
  https://generalistprogrammer.com (SoA/ECS ~6x entity updates, кэш-промахи)

## 2. C-X8 — Off-heap section-реестр (FFM MemorySegment, JEP 454)
- **Механика**: индекс EntitySection-реестра (секция→entities) шардируется в off-heap
  MemorySegment-арену (long-слоты: base+offset+count+epoch); читатели = plain segment loads с
  Acquire/Release семантикой вместо CHM volatile — GC не сканирует, кэш-плотность 2x (64B = 8
  слотов). ARM из rust-плоскости, GC-минус: java_util CHM-хвост лейна 9% питается CHM.get.
- **Сайты**: EntityLookup/EntitySectionIndex (broadphase 8.8-10.9%); upsert-плоскость 131k
  мертва на серт-ногах (cmp458_swar крит-факт: 100% ваниль) — читатель is the boss.
- **Parity-план**: latch-fail-closed при ANY-рассинхроне epoch (CX3-канон BROKEN-латч);
  arena lifecycle = тик-эпоха; STRICT dormant lever cmp459_cx8; selfTest-оракул против
  round-396-a kernel jar.
- **Прогноз Δ**: broadphase 8.8 × 10-20% = **+0.9..+1.8пп**; потолок +2.0пп.
- **Риск**: FFM-сейфти-цена vs Unsafe (inside.java: bounded lifetime стоит на hot path);
  NCDFE-канон (EARLY-define); dual-GC-взаимодействие (канон ZGC-фальсификации — не рычаг).
- **URL**: https://openjdk.org/jeps/454 · https://inside.java/2025/06/12/ffm-vs-unsafe/ ·
  https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/MethodHandles.html

## 3. C-X9 — Valhalla-флаттенинг эмуляция (tuple→packed-long)
- **Механика**: Project Valhalla «codes like a class, works like an int» — эмулируем ДО релиза:
  горячие тьюплы (SectionPos/BlockPos координаты, palette-entry индексы) пакуются в
  single-long поля/long[] слоты (x:26|y:12|z:26 классика MC-упаковки), убивают pointer-chase и
  заголовки объектов (Lilliput-эффект ~х2 плотность, header 64→32б-класс). Читатель paletted
  получает flat int palette-id → прямой state-лукап без ре-аллокаций тьюплов.
- **Сайты**: PalettedContainer.get 4.48% + SimpleBitStorage.get 1.50% (TOP-1 3.7% серт-ног);
  readPalette на КАЖДЫЙ вызов (javap-грунт CX5); Vec3i-поля x/y/z раздельные.
- **Parity-план**: identity-sensitive сайты (== на тьюплах) под STRICT-гвардией → фолбэк;
  javap flat==nested 10/10 + strings-проверка блобов (анти-placebo ×425/×458-F1);
  preregistered гейт: identity-миссии = 0.
- **Прогноз Δ**: paletted 6.4-7.0 × 12-20% = **+0.8..+1.4пп**; потолок +1.6пп.
- **Риск**: JDK28 value objects ещё preview (off by default) — ручная упаковка не должна
  течь в identity-путь; boxing-ловушки.
- **URL**: https://github.com/openjdk/valhalla-docs ·
  https://cr.openjdk.org/~briangoetz/valhalla/sov/01-background.html

## 4. C-X10 — EA-friendly cursor rewrite (scalar replacement java_util)
- **Механика**: горячие аллокационные walker-пути (iterator/Map.Entry/cursor в CHM-чтениях
  java_util 9%) переписываются в переиспользуемый курсор-объект, который HotSpot EA
  scalar-replaces (Whaley: EA элиминирует 24-67% sync + stack-alloc); метод режется до
  EA-дружественного размера (8000-bytecode лимит инлайн-окна).
- **Сайты**: CHM.get/value-итерации, кормящие java_util 8.96/8.73 РОСТ (RED-ноги);
  инвал-волна hypothesis B.
- **Parity-план**: pure-refactor внутри dormant-плоскости, порядок выдачи бит-в-байт;
  offline lockstep min-of-2; NOT-A-BENCH маркер для микро-оракула.
- **Прогноз Δ**: java_util 9 × 7-15% = **+0.6..+1.4пп**.
- **Риск**: EA хрупка (мега-методы/глубина инлайна ломают) — валидация scalarity оракулом;
  JIT-версии.
- **URL**: https://shipilev.net/jvm/anatomy-quarks/ · http://minborgsjavapot.blogspot.com
  (Java 21 perf, поисковая выдача) · https://daily.dev (JDK21→25 +10% latency upgrade)

## 5. C-X11 — False-sharing паддинг горячих счётчиков (атака гипотезы B РОСТА) ★ТОП-3
- **Механика**: per-worker секционные secWrite-эпохи/счётчики, оседающие на одной 64B кэш-
  линии разных ядер → инвал-шторм (MESI ping-pong) = механика РОСТА inside_volatile
  12.01→16.63 (+25% абс) и java_util 7.01→8.96. Фикс: @Contended/ручной 64B-паддинг
  (long p0..p7) или разъезд счётчиков по per-worker слотам long[] (индекс=worker id).
- **Сайты**: secWrite-эпохи (C-X1 LCG-эпоха рядом — дизъюнкт: CX1 меняет протокол, C-X11 —
  ЛИНИЮ размещения), RegionTickOps счётчики (main bucket-0 478/651 сэмплов RECON-15).
- **Parity-план**: layout-only, порядок выдачи бит-в-байт; JOL-дамп до/после (гейт: счётчики
  в разных линиях = OK); STRICT dormant lever cmp459_cx11.
- **Прогноз Δ**: условный РОСТ-убийца: если hypothesis B верна, съедает до 30-50% РОСТ-дельты
  = **+1.0..+2.5пп на RED-ногах** (16.6-лейн), ~0 на ванильных.
- **Риск**: паддинг-дилюция кэша (psy-lob-saw: «padding every little piece dilutes»); JIT
  умеет убирать мёртвые поля — паддинг только «живой» записью; needs perf-counters валидация.
- **URL**: https://openjdk.org/jeps/142 (@Contended) ·
  http://psy-lob-saw.blogspot.com/2014/06/notes-on-false-sharing.html ·
  https://alidg.me/blog/2020/5/1/false-sharing · https://www.baeldung.com/java-false-sharing-contended

## 6. C-X12 — Megamorphic→mono repair (LambdaMetafactory frozen constants)
- **Механика**: мега-морфные interface-сайты (Comparator/Runnable/iterator в goalops/CHM-
  хвостах java_util 9%) склеиваются в мономорфные через статик-финал поля, инициализированные
  LambdaMetafactory.metaFactory/MethodHandle-константами — JIT инлайнит без inline-cache poll;
  фиксация констант при БУТЕ (не indy-lazy — NCDFE-канон: bootstrap в hot-path запрещён).
- **Сайты**: goalops-хвосты (nav_ai/goalops остаток 2.75-3.2%), CHM-компараторы, bucket
  callable-цепочки RegionTickOps.
- **Parity-план**: javap-гейт: 0 invokedynamic в горячем классе (прецедент P33: getAcquire
  signature-polymorphic без indy = NCDFE-безопасен); selfTest live JVM accept/reject.
- **Прогноз Δ**: java_util 9 × 5-12% = **+0.5..+1.1пп**.
- **Риск**: bootstrap-time NCDFE-класс риска при неаккуратном indy → всё через eager static;
  LambdaMetafactory-аргументы frozen-проверки.
- **URL**: https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/LambdaMetafactory.html ·
  https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/MethodHandles.html

## 7. C-X13 — SoA AABB broadphase sweep (layout-only расширение swar-семьи) ★ТОП-2
- **Механика**: AABB кандидатов broadphase (push/collide/query) хранятся SoA: min/max packed
  int[] (x26|y12|z26), CSR-справочник секций; свип = векторные сравнения по массивам без
  разыменования Entity-объектов (по dodbook: «целая кэш-линия должна использоваться целиком»).
  Дизъюнкт с cmp458_swar (там SWAR-CSR по upsert — который мёртв на серт-ногах; тут — layout
  читателя sweep-плоскости).
- **Сайты**: broadphase 8.8-10.9% (push ~1 блок, merge ~0.5, collide AABB vs 8x8-чанк регионы,
  javap-грунт RegionTickOps:821-829).
- **Parity-план**: бит-в-байт порядок кандидатов (снапшот-порядок оракул, как CX6 G2);
  interleave-класс S7-155 GREEN; bloom-урок roar-2 (bit-variance фикс) учтён в гейт-наборе.
- **Прогноз Δ**: broadphase 8.8-10.9 × 15-25% = **+1.5..+2.7пп**; потолок +3.5пп.
- **Риск**: RECON-39/40 кэш-классы REFUTED ×2 — защита: это layout-атака (не-кэш-класс
  разрешён каноном: batching/O(1)-индексы/layout); overflow x26-упаковки — AIOOBE=0 гейт.
- **URL**: https://www.dataorienteddesign.com/dodbook/ · https://generalistprogrammer.com ·
  https://github.com/CaffeineMC/lithium (миксин-прецеденты layout-оптимизаций) ·
  https://github.com/RelativityMC/C2ME-fabric

## 8. C-X14 — Lilliput-sidecar: single-long section metadata (field relocation)
- **Механика**: «field relocation» в DOD-стиле: горячие поля EntitySection (flags|epoch|
  y-level|lock-bits) переезжают в один long слот параллельного long[] sidecar (64-бит
  компактный «заголовок» = Lilliput-эмуляция), pointer-chase секция→метаданные 3 deref → 1
  array-load. Vanilla-объекты остаются (GC-безопасно — sidecar только long[]).
- **Сайты**: EntitySection-чтения в inside/broadphase путях (volatile-лейн 16.6 РОСТ —
  MISS-часть читать из sidecar plain-загрузкой).
- **Parity-план**: STRICT двойное-чтение (sidecar vs vanilla field, расхождение → fail-closed
  латч CX3-канон); javap flat==nested; min-of-2 lockstep.
- **Прогноз Δ**: inside+broadphase совм. 16.6+8.8 × 5-9% = **+1.2..+2.2пп**.
- **Риск**: рассинхрон sidecar/vanilla при GC-ре-аллокации java-ссылок (sidecar — примитивы,
  ссылки не пакуются); epoch-гонка = C-X1-совместимость (CX1 протокол, C-X14 хранилище).
- **URL**: https://github.com/openjdk/valhalla-docs (Lilliput) ·
  https://cr.openjdk.org/~briangoetz/valhalla/sov/01-background.html

## 9. C-X15 — indy-frozen dispatch table (scheduling callable-плоскость)
- **Механика**: bucket-dispatch RegionTickOps (bucketOf w=4 checkerboard, 478/651/2432
  RECON-15, main PARKED 13.4%) — callable-цепочка на тик собирается из мега-морфных
  site-вызовов; заменяем на frozen-таблицу MethodHandle-констант, собранную один раз при
  boot (LambdaMetafactory+MethodHandles.insertArguments), инвариант w — const-folded.
- **Сайты**: RegionTickOps.forEach:573,579-583 (единственный сайт распределения),
  bucketOf:821-829.
- **Parity-план**: порядок тиков бит-в-байт (партитивность снапшота сохраняется — CX6-канон);
  javap 0 indy в hot class; STRICT dormant cmp459_cx15.
- **Прогноз Δ**: scheduling/java_util 9 × 4-10% = **+0.4..+0.9пп**; композиция с CX6-affinity.
- **Риск**: indy-bootstrap NCDFE-канон (eager-инициализация обязательна); profile-pollution.
- **URL**: https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/LambdaMetafactory.html ·
  https://cr.openjdk.org (Static Dynamic JVM, выдача поиска) · https://shipilev.net/jvm/anatomy-quarks/

## 10. ТОП-3 по Δ (вердикт-число пакета)

| # | ID | Δ (к ноге) | Потолок | Лейн-мишень |
|---|---|---|---|---|
| 1 | **C-X7** SoA INSIDE-probe batch | **+2.5..+4.1пп** | +5.0пп (⊕P31) | inside_volatile 16.6 РОСТ |
| 2 | **C-X13** SoA AABB broadphase sweep | +1.5..+2.7пп | +3.5пп | broadphase 8.8-10.9 |
| 3 | **C-X11** false-sharing паддинг (РОСТ-убийца) | +1.0..+2.5пп (RED-ноги, условный) | — | inside_volatile/java_util РОСТ |

Остальное: C-X14 +1.2..2.2 | C-X8 +0.9..1.8 | C-X9 +0.8..1.4 | C-X10 +0.6..1.4 |
C-X12 +0.5..1.1 | C-X15 +0.4..0.9. Сумма пакета (субаддитивный потолок, компо-канон ×455):
+6..+12пп к серт-ноге при композиции поверх P31/P32-климб-пути.

## 11. Preregistered гейты пакета (канон волны)
1. STRICT dormant lever cmp459_cx7..cx15, ARM-маркеры ДО пурджа; 2. NCDFE=0 T1 до вердикта
(EARLY-define); 3. javap flat==nested 10/10 + strings-блобов (анти-placebo ×425/×458-F1);
4. AIOOBE=0; 5. items 0.00-гейт не трогаем; 6. band [6.0-9.5]M runner_cpu_index;
7. pair = leg_norm − anchor_norm ≥ +20, Δ≤50k, min-of-3, pair-fresh; 8. offline lockstep
A/B min-of-2 (дизъюнктность с C-X1..X6 задокументирована в §1-9).

## 12. Источники (15 URL, все curl 200 верифицированы)
1. https://www.dataorienteddesign.com/dodbook/ — DOD-канон (SoA/AoS, layout)
2. https://generalistprogrammer.com — SoA/ECS game perf (поиск #1: ~6x entity updates)
3. https://openjdk.org/jeps/454 — FFM/MemorySegment (C-X8)
4. https://inside.java/2025/06/12/ffm-vs-unsafe/ — FFM vs Unsafe safety cost (C-X8)
5. https://github.com/openjdk/valhalla-docs — State of Valhalla/Lilliput (C-X9/C-X14)
6. https://cr.openjdk.org/~briangoetz/valhalla/sov/01-background.html — Valhalla background
7. https://openjdk.org/jeps/142 — @Contended cache-line padding (C-X11)
8. http://psy-lob-saw.blogspot.com/2014/06/notes-on-false-sharing.html — false-sharing notes
9. https://alidg.me/blog/2020/5/1/false-sharing — @Contended паддинг полей
10. https://www.baeldung.com/java-false-sharing-contended — практикум паддинга
11. https://shipilev.net/jvm/anatomy-quarks/ — EA/megamorphic/инлайн-каноны (C-X10/12/15)
12. https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/LambdaMetafactory.html
13. https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/invoke/MethodHandles.html
14. https://github.com/CaffeineMC/lithium — mixin-прецеденты optimization-гостей
15. https://github.com/RelativityMC/C2ME-fabric — region/thread паритет-прецеденты
Поисковые сессии: 7 (DOD/SoA ×1, escape ×2, off-heap ×1, indy ×1, Valhalla ×1, false-sharing ×1).
