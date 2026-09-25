# LAB LEDGER — перманентная память лабы глубокой оптимизации (v18.3, закон 14c)
# Правила: каждый агент после каждого эксперимента ДОПИСЫВАЕТ находку (в т.ч. отрицательную) с числами + commit+push.
# Формат: | подсистема | статус | ключевые числа | открытые гипотезы | уроки |

## ПЕРМАНЕНТНЫЕ КАНОНЫ ДОСТАВКИ (все подсистемы)
- NCDFE-канон: перед вердиктом любого носителя T1 NCDFE=0 ОБЯЗАТЕЛЬНО. Гонка arm/define EntityGoalQueryOps @ MobPushOps.pushables:467; фикс = EARLY-define в раннем arm-хуке + mirror-drift иглы (прецеденты d73758a3 chunkmono, 5ecd841a poi, 9d71b461 eqsnap2). NCDFE>0 = DELIVERY-FAIL, пара аннулируется (отравленные +20.1/+41.3 ×456).
- Спящие гейты: lever, вшитый только в SOURCES без пересборки tracked build-блобов = placebo (×425, ×458-F1 agent-N: strings 0/10 блобов содержали флаг). После каждого вайринга: javap flat==nested 10/10 EQUAL + strings-проверка блобов.
- Band-закон: нога с runner_cpu_index вне [6.0M, 9.5M] = BAND-DISCARD, не вердикт (poi457-8 +13.5@10425941 — сгорела). Ре-ролл ≤2.
- Депресс-кластеры: системные GC-окна (Full=9) душат ноги кластером (chkmono-11/12 −3.6/−3.7 @7.04-7.05M) — ре-роллы в окно, а не рядом.
- Pair-математика: pair = leg_norm − anchor_norm ≥ +20, Δ≤50k, pair-fresh (якорь занят = только в предыдущем МЕРЖЕ), min-of-3 (нога парится с ≥3 якорями).
- Банк v4 (2 точки): (2.6 TPS @ 8551924), (2.2 @ 6653417), линейная TPS_exp(runner); norm = median/TPS_exp − 1.
- Гейты вердикта: ARM + эффект-маркеры ДО пурджа; AIOOBE=0; selfTest==true (×1-2 FAIL = fixture-шум); threw=1 = REFUTED; items 0.00-гейт на серт-ногах (28.7-30.8% ваниль на якорях); NOT-A-BENCH маркер; argv-guard.

## ПОДСИСТЕМА: inside_blocks / checkInsideBlocks (TOP-1 остаток компо-носителя 12-16.6%)
- Потолок: #15 inside_bitmask (закон 5) = паритет на обеих экономиках (×73: norm −0.5%), лейн закрыт для точечных рычагов; G1-вердикты: конверсия ~0.
- РОСТ на RED-ногах chkmono457-11/12: inside_volatile 12.01→16.63/16.30% (+22-25% АБСОЛЮТНО); java_util 7.01→8.96/8.73 (+12-15%); гипотеза B: invalidation-driven (chunk6-sched setBlockState-всплески → secWrite-бампы → volatile MISS-чаще, CHM.get кормит java_util).
- ОТКРЫТЫЕ (agent-P ×458): ID-P31 INSIDE-BATCH bulk-JNI (candidate-superset → strict java tail; прогноз +5-8пп к ноге; паттерн nav_plane/collide_batch; парити: dirty-list мутантов обязателен) — КЛИМБ-КАНДИДАТ #1. ID-P32+P36 SNAP sidecar (CHM→флет-реестр + epoch fast-gate; +1.5-2.5пп; 70% кода в inside_snap существует) — климб-кандидат #2. P33 VarHandle.getAcquire demotion (+0.6-1.2пп). P34 квант-гейт K-тик-покой (+2-4пп). P35 DDA-v2 гибрид (разбор провала 21/350k, верификатор 1/200). P37 attribution probe (NOT-A-BENCH, инструмент).
- Урок: hard-colliding масса фикстуры (item_frame×2714) глушит fast-path гвардии (eqsnap2-ценз ×457-C2) — в фикстурах с hard-colliding fast-path в перманентном ваниль-фолбэке.

## ПОДСИСТЕМА: ServerChunkCache scheduling / chunk-tick (закон-8 ось, носитель cmp456_chunkmono @d73758a3)
- Ноги эры: 11 валид (−0.8..+11.5), МОНСТР chk-14 +21.7@8687055 (band OK, NCDFE=0) — пара max +9.3↔a26 +12.4@8671791. Суб-бар → КЛИМБ: P31/P32+P36 поверх носителя (прогноз нога 16-21пп).
- Ген (worldgen) инертен на soak ×421-C (0.0%) — живой закон-8 сайт = chunk-tick eligibility (ID-P22: плоские предикаты тика чанков → один bulk-JNI → битмаска → строгий java хвост; +1.2-2пп; Moonrise/MC-310372).
- Окна-прицелы: [8637055,8737055] якорь norm ≤+1.7 → пара ≥+20 с chk-14.

## ПОДСИСТЕМА: POI (носитель cmp456_poi @5ecd841a)
- Ноги: max +16.9 (валид), poi-12 +18.5 BAND-OUT@9.83M. Пары до +10.9. Потолок пары требует якорь ≤−3.1 рядом — окна [8907260,9007260] ≤−4.6 (poi456-4 +15.4).

## ПОДСИСТЕМА: entity-query / eqsnap2 (носитель round-457c-eqsnap2 @9d71b461)
- REFUTED_CENS ×457-C2: захват 0/2.3% среза (players fast-path пуст на фикстуре 4; hard-colliding → ваниль-фолбэк). НОСИТЕЛЬ ЖИВ для композиций: H03 interval-tree targeting R3 ( Hilbert, agent-J ×458) — гипотеза «eqsnap2-ноги + H03 → пара ≥+20».

## ПОДСИСТЕМА: broadphase (8.8-10.9% на ногах)
- x458 агенты: I cmp458_swar @cee4992c (SWAR-CSR + zero-JNI feed; КРИТ-ФАКТ: push-плоскость 131k upsert = 100% ваниль на серт-ногах — drain-гейт мёртв, потенциал выше оценки); K cmp458_roar @eb47e869 (per-секционные цепочки + 64-bit occupancy + 4KB bloom pre-gate k=4 splitmix64; bloom bit-variance фикс обязателен — фикс-бит давал FPR 99%); J cmp458j-hilbert (geo-index 0.4.0, HilbertSort leaf order); H05 papaya-lockfree шард-ридеры СВОБОДЕН; H07 Hilbert-порядок выдачи СВОБОДЕН.
- REFUTED ранее: кэш-классы ×2 (RECON-39/40) — new-attack только не-кэш-класс (batching/O(1)-индексы/layout).

## ПОДСИСТЕМА: paletted (6.4-7.0%)
- cmp457_paldelta @29876077 (В master, CERT-FIX блобы): PalettedContainer.get 4.48% + SimpleBitStorage.get 1.50% ваниль; TOP-1 3.7% на серт-ногах. Ноги 2/3 после фикса — вердикты ×459.
- P21 parse-cache widen biomes+light (twin lambda$parse$7 без кэша; +1-2пп reload + GC-relief).

## ПОДСИСТЕМА: chunk-send / serialization (law-8 видимый лейн)
- agent-M cmp458_chdelta: ID-M1 GO (пер-секционная дельта extractChunkData, isUnsaved oracle; send lane 1.75%, serialize 0.00% на soak — carrier-зависимо). P26 send-burst coalescing (+0.3-0.8пп netty burst, krypton). P27 scratch-arena (GC-debt carrier).

## ПОДСИСТЕМА: noise / worldgen (ген инертен на soak — только GC-debt carriers)
- cmp457_noisesimd @c8156a69 (agent-D: lever-scoped arm NormalNoiseBatchOps C1a; SIMD-варианты: incubator=NCDFE-риск). P24 octave scratch-pool / P25 2D-router cache — ТОЛЬКО GC-debt relief механика, не прямые Δ.

## ПОДСИСТЕМА: nav_ai / goalops / brain (остаток 2.75-3.2%)
- P41 path-node neighbor cache (+0.6-1пп); P42 goal canUse sense-memo (+0.4-0.8); P43 brain flat-memory (+0.3-0.5); P44 MoveControl navmath bulk-JNI (+0.8-1.2, javap-транскрипция IEEE754 канон); P45 navigatingMobs pre-gate (roaring, +0.3-0.6); P46 tick-deadband; P47 GoalSelector transition-diff iter-3.

## ПОДСИСТЕМА: GC / экономика
- Банк v4 = ParallelGC (gc_tune=3); ZGC REFUTED −27% (фальсификация: STW-элиминация без освобождения CPU крадёт ядра воркеров); THP паритет; alloc-диеты REFUTED ×2. GC Full=9 на депресс-ногах vs банк 7 — мониторить как фактор, не как рычаг.

## УРОКИ ИНФРАСТРУКТУРЫ
- Task-инфра: мёртвый вызов ≠ мёртвый агент (живость по worktree/веткам/панам/RESULT.json; рестарт-канон ×5 подтверждён).
- Диск: маркеры ДО пурджа; joml.jar вне пурдж-зон (Maven Central восстановление); sparse-worktree ~30-40MB (worktree add без --no-checkout валится при соседях 3×932M); git gc 1.4G→850M.
- sha:literal в --leg ломал split ":" (фикс ×458); absorb-кэш склеивает одинаковые артефакты — проверять tag-vs-run_id.
- INFRA-DUP: артефакт-коллизия двух ранов одного коммита (a35) — разносить ре-роллы по хешам/коммитам.

## ТИК-459 СВОДКА ЛАБЫ (v18.3 первый мега-тик; полные доки: research/round-459/ + RESEARCH-459-L*.md)
- L01 P31 INSIDE-BATCH: lane 15.36% × захват 66% → Δ +10.2пп, потолок +14.1пп; THRESH=512/bucket, buildPlan-порт CollideBatchOps.
- L02 P22 chunk-sched: +1.4пп; КЛИМБ-ПУТЬ БАРА: chk-14 +21.7 ⊕P31 (+5-8) ⊕P32/P36 (+1.5-2.5) → нога +29.6…+33.7 → пара с a26 +12.4@8671791 = +20.2…+21.3 ≥+20 ✓.
- L03 POI-ценз: лейн сухой 0.15-0.23% CPU → потолок +0.23пп; сила poi = юнион-widening (mean +6.1пп diet-vs-poi n=8); живая пара poi456-4+a24-456w2 −10.4@8928192 = +25.8 (нужны ещё 2 якоря ≤−4.6 окна D); a35=фантом-склейка, a36/a37 не-ваниль (compose armed) — из pair-пула.
- L04 eqsnap2+H03 ЗАКРЫТО: медианная пара +6.0 < 20 (потолок +4.3пп) — закон 13a honest.
- L05/L09 GC-КАНОН НОВЫЙ: STW-slope −4.96пп/с ОПРОВЕРГНУТ (n=33: −1.09пп/с, 95%CI −3.11..+0.93 ns, R²=0.04); soak-STW потолок 2.78пп; Full=9 = 5×CodeCache+4×Metadata ДЕТЕРМИНИЗМ (не аллокация); депресс-кластер chk-11/12 = миф; G3-STW гейт: STW>23.0s ИЛИ avg>200ms → INVALID-STW-HOST (ре-ролл бесплатно), STW-CLEAN-RED, census-поля scavAvg/soakFull; STW-жертвы: a33/roar-2/chk-11 (якоря a33 — out pair-пула).
- L06 paldelta-климб: P32+P36 +1.0-1.5пп; pair-maker P31+P34 → нога ≥18.7-19.2 ↔ a4 −0.8@6737702 (Δ7.7k, pair-fresh).
- L07 chunk-send: joins-burst +1.5пп (потолок 1.75), на монстр-soak lane 0.00 — НЕ диспатчить на chk-оси; смежный entity-sync +0.9пп.
- L08 navmath: P44-ядро +0.83-1.28пп (потолок +3.2); ПОЛНАЯ IEEE754-таблица Mth (sin=SIN[65536]-таблица, atan2=fastInvSqrt 6910469410427058090L, FRAC_BIAS) — имплементация MovePlaneOps готова к вайрингу.
- L10 P24 noise PARK честно: Full GC 0×AllocFailure ×4 ног; октав-циклы javap 0×new; GC-носители → Vec3/AABB entity-чурн (28-29% alloc).
- L11 papaya H05: +0.8-1.4пп (reader-core потолок +4.1пп); офлайн lockstep 0/300×3 бит-в-байт GREEN — swing-слой климба chk-14: P31⊕P32/P36⊕PapayaReadOps → нога 29.0-33.6 → пара 16.6-21.2.
- L12 P35 DDA-v2: root-cause DDA-1 = inline-clip поверхность (0/200,736 verbatim-clip; брикет класса 0..21.5%); v2: vanilla-walk=истина + shadow feed + 1/200 верификатор P(no-detect/5мин)=1.9e-12, 0.18% MSPT; capture feed +1.19пп; пара-база +22.7↔a26.
