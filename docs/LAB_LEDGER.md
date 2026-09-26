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
- Л463-46 dormant-страж на 97f1c9c3: гейтов живо **24/31 (77.4%)** — rust 21/21 файлов/61 токен целы (cargo check 0 err), java **3/10** (ItemEntityManager/ColpushOps/BrainOps-зеркало); **7 java-классов × 11 сайтов несут не-матчимый pipe-литерал equals("cmp457_paldelta\|cmp457_eqsnap2")** (merge 887c4641 склеил standalone-термы 9d71b461+96cc2704; 9df38ba1 чинил только rust; javac на merge-коммите не гонялся); блобы: 22/242,244B с флагом, check_blob_sync OK, javap flat==nested 10/10 byte-md5 10/10; NCDFE-landmine: pipe-флаг → java-alive/rust-dead = класс ×456; ремонт = 11 строк×7 файлов + rebuild + **CP-EXACT-гейт** (substring-grep пропускает). Детали: ROUND-463/LAB-STAGE/LEDGER-46.md.

## ПОДСИСТЕМА: broadphase (8.8-10.9% на ногах)
- x458 агенты: I cmp458_swar @cee4992c (SWAR-CSR + zero-JNI feed; КРИТ-ФАКТ: push-плоскость 131k upsert = 100% ваниль на серт-ногах — drain-гейт мёртв, потенциал выше оценки); K cmp458_roar @eb47e869 (per-секционные цепочки + 64-bit occupancy + 4KB bloom pre-gate k=4 splitmix64; bloom bit-variance фикс обязателен — фикс-бит давал FPR 99%); J cmp458j-hilbert (geo-index 0.4.0, HilbertSort leaf order); H05 papaya-lockfree шард-ридеры СВОБОДЕН; H07 Hilbert-порядок выдачи СВОБОДЕН.
- REFUTED ранее: кэш-классы ×2 (RECON-39/40) — new-attack только не-кэш-класс (batching/O(1)-индексы/layout).
- Л463-45 roar-3 ПЕРЕКВАЛИФИЦИРОВАН VALID→PLACEBO: stdout:973 «eindex: seed failed (0)» + armed=пусто — зеркало не ARM (seedAll: 4 пути return-0, silent catch(Throwable) без диагностики; одноразовый seed на hook-фазе, late-seed нет) → блум-гипотеза +2-8пп НЕ тестирована. Лейн-ценз 113,583 сэмплов: sendBlockUpdated 0.926%, shouldRecomputePath 0.813%, PathNavigation 4.83%, getEntities 8.93% → P45-соло потолок 0.926×0.4-0.6 = **+0.37..0.56пп < 1пп** (chkclimb-16 −2.2@6407285 armed=TRUE подтверждает нуль-эффект); javac-rebuild блоба снят ×462 (16c310c6, NavPlaneOps.class 6477→6719B); FPR: 64KB/k8 = 3.31e-4@30k, оптимальный k=12 = 2.26e-4 (Л35 2.0e-4 = оптимальный-k); ≥+1пп только компо-полка nav (P45⊕navplane⊕P44⊕P48 +1.3-2.5пп) или roar-ARM-fix query-плейн (8.93%×10-30% = +0.9-2.7пп); re-audit триггеры R1-R5 (ARM-гейт вердикта в absorb, seed-fail маркер, pregate-hit-rate, CP-EXACT, bloom bit-variance). Детали: ROUND-463/LAB-STAGE/LEDGER-45.md.

## ПОДСИСТЕМА: paletted (6.4-7.0%)
- cmp457_paldelta @29876077 (В master, CERT-FIX блобы): PalettedContainer.get 4.48% + SimpleBitStorage.get 1.50% ваниль; TOP-1 3.7% на серт-ногах. Ноги 2/3 после фикса — вердикты ×459.
- P21 parse-cache widen biomes+light (twin lambda$parse$7 без кэша; +1-2пп reload + GC-relief).
- **×463 Л62 demux REFUTED-страж (полный разбор: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-33.md)**: лейн-ценз 12 профилей ×462 — ваниль PC.get+SBS mean **5.57% CPU** (TOP-1 leaf 4.4-4.6%) / **0.26-0.32% WALL**; demux-ноги 5.47/5.64% = конверсия стола 1:1 (Ops.get +2.06-2.18пп возвращает снятый бит-декод −2.30..−2.65пп), capture 0/5.7 → norm −0.3 (31acaf0a). Стол декоративен числами. Re-audit триггер: WALL-лейн ≥3.0% (сейчас 0.2-0.4%, дефицит 10-13×) ИЛИ CPU ≥8.0% + SBS-инлайн-деградация (leaf>4%). CLIMB-PLAN формулировка «лейн ≥3.0%» в CPU несамостоятельна — материал norm = WALL.
- **×463 Л63 P21 re-scope — УСЛОВНЫЙ вектор (полный разбор: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-34.md)**: javap λ$parse$5/$7 twin = 36 опкодов, canon-desc ровно 2 метода; codec.parse-сайтов в parse() всего 2 — light = DataLayer::new НЕ-codec → widen-плоскость пуста; biomes-кэш УЖЕ доставлен R5c (redirect 2/2, ChunkParseOps 12380B в блобе). Reload-сцена: 442,368 parse-вызовов (9216×24×2), hit ≥64% → ≤21-33% burst-alloc срез; на bank-soak capture ≈0 (медиана из steady). Актуален только при owner reload-метрике (гейты G1-G7 бит-в-байт); иначе PARK, TASK-463-73 = reload-верификация.

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

## ТИК-460 СВОДКА ЛАБЫ (v18.3; полные вердикты: /home/z/rounds/ROUND-460/ABSORB-WILD.jsonl + GOAL ×460)
- P31+P32/P36 на cmp456_chunkmono (round-460-chkclimb-1 @2d6c7ed6, agent-01): VALID +15.9@8904014, STW-CLEAN 21.54s/157.2ms/137 пауз, NCDFE=0, selfT=9 — НИЖЕ банка chk-14 +21.7 (capture P31 ниже прогноза L01 66%×15.36%); лейн-состав lever: p31snap×3 + snap + inside2 + collide + sscan + aibatch + colpush + brain2.
- swarx-1 (agent-34, swar push-plane ARM-ремонт, round-460-swarx-1 @33939931): +18.0@8682604 = 2-й монстр эры после chk-14; root-cause мёртвого drain-гейта задокументирован агентом; needs anchor ≤−2.0 в [8632604,8732604] для пары.
- eqsnap2-3 ре-ролл (cmp457_eqsnap2 @9d71b461): +12.0@6805816 → 2/3 пары {a28 Δ23k +22.6, a41 Δ8k +26.9} — кластер-кандидат мержа.
- P31⊕paldelta REFUTED: pdclimb-1 −13.3@6613699 (agent-03), paldelta-10 −4.7 — P31 не переносится на paldelta-носитель (inside-лейн paldelta иной структуры).
- P26-send на poi (agent-38, poisend-1): +0.5@7071067 — нейтраль; joins-burst lane на poi-профиле не активируется.
- P34-quant на климб-дереве (agent-40, chkquant-1): INVALID гейты (selfTest) — P34 требует отдельного серта; papaya-соло на chunkmono (chkswing-1): +3.1 (изолированная база для swing-композиции).
- Семейные ре-роллы: chkmono-30/31/32 +7.0/−13.7/−1.9; poi-21/22 +8.9/+9.6; noisesimd-5 +0.1; swar-2 −12.0; WILD-ноги ×459: p42 +4.2, p34b +1.4, p21 +0.6, p43 −3.0, p44 −10.6, cx4 −15.1, p25 −14.8, p26 −12.6.
- **КЛАСТЕР 6.80M = НОВАЯ ЯКОРНАЯ ШАХТА**: a22 −9.7@6729244, a53 −9.8@6737567, a4 −5.9@6802561, a6 −6.0@6819953, a28 −10.6@6829058, a41 −14.9@6813994 (все STW-ценз честен кроме a10/a19 — выбыли) — нога norm ≥+14.1 в [6779058,6852561] получает min-of-3 = leg+5.9 ≥+20 мгновенно; окно E chk-19 закрыто наполовину (a22+a53).
- G3-STW LIVE-эффект: a19 −28.8@8883590 INVALID-STW-HOST (24.4s/203.3ms) — без гейта дал бы фейковую пару +44.7; a10 VOID (23.0s); канон подтверждён на живых кейсах ×2.
- ПАР-СОСТОЯНИЕ ×460 (все кандидаты 2/3): chk-19 +12.8@6733439 {a22 +22.5, a53 +22.6}; chkclimb-1 +15.9@8904014 {a24-456w2 +26.3, a20 +26.5}; poi456-4 +15.4@8957260 {a24-456w2 +25.8, a20 +26.0}; eqsnap2-3 +12.0@6805816 {a28 +22.6, a41 +26.9}; swarx-1 +18.0@8682604 {0/1: нужен ≤−2.0 в B}.
- Волна-2 (закон 15a): chkclimb-5, swarx-2, eqsnap2-4, якоря 81-88 диспатчены; next-tick абсорб.

## ТИК-461 СВОДКА ЛАБЫ (v18.3; полные вердикты: /home/z/rounds/ROUND-461/LEDGER-*.md + ABSORB-WILD.jsonl ×122)
- Л21 P31 THRESH-скан: THRESH=512 в живом блобе 0 вхождений (javap 4215B @4bcabb2f, 6/6 jar один md5); матрица T∈[128,1024] ≤1.5пп@c=3µs — миф «+2..4пп» рефютирован; T=512 праймери (74 вызова/воркер/тик, dispatch 0.44пп); прогноз chkclimb-6-реинкарнации 25.4 (23.0-29.3) поверх +15.6.
- Л22 snapreg P32: capture-гейты flatHits/fallbacks ≥95/5, NCDFE=0, вердикт-бар +0.5пп; прогноз chkclimb-5⊕P32 +16.5-17.6; v1-носители = parity-леги Δ≈0, capture решает v2-lockstep 10⁶×3.
- Л23 getAcquire-demotion: 4 сайта демот-таблицы (serve/serve4, PalettedContainer.data, crusstySnap×3); бар +0.6пп; chkclimb-10 (P33⊕P32-план) = +16.2-16.8.
- Л24 cert-fix root-cause: chkquant-1 (×460) INVALID = NCDFE=38/threw=38 @ARMED p31quant, pop 160493 VALID — P34 требует серт-пересборку блобов (рецепт в L24); selfF=1 у cmp412_b2p1 — чужой спящий рычаг.
- Л25 DDA-v2: shadow feed capture +1.19пп, P(no-detect/5мин)=1.9e-12 @0.18% MSPT; компо-вертикаль P31+v2+P32+P36 = нога 35.1 / пара +22.7↔a26 (консерв +20.9).
- Л26 chunk-sched: NewChunkHolder 66291B vs ServerChunkCache 39350B; маска 6561 чанков = 824B/тик zero-alloc; P22 capture = срезы 1.5-2.5% (due-ness + queue + Long2Ref 2.21%).
- Л27 attribution probe: NOT-A-BENCH-инструмент (факторные оценки пар-шума, интерфейс к ABSORB-WILD.jsonl).
- Л30 swar drain-gate после ARM-repair: ЖИВОЙ (EFFECT-маркер +18.0@8682604), контракты 8/8 in sync, потолок swarx-семьи +24±1; компо-план swarx-3 = graft 89f90d50 + add_swar_gates_458.py.
- Л31 navmath IEEE754 re-verified (10/10 констант вербатим, FRAC_BIAS=2^44); MovePlaneOps ready-to-wire; ядро +0.50-0.90, семья +0.83-1.28, потолок +2.24/+3.2.
- Л32 pathnode-cache: харнесс 10^4 путей×3, path bit-in-bit + superset ≤10%, hit 20-30%; Δ +0.6-1.0пп.
- Л33 goal sense-memo: единственный parity-чистый путь (DAB/every-other-tick REJECT ×2 по behavior-видимости); +0.4-0.8пп (Sensing 0.6%).
- Л34 brain flat-memory: mutation-site единственный @693, изоляция от cmp421/422; +0.3-0.5пп. Л35 navgate roaring: 636 LOC 336/336 тестов, блокер iter-2 = javac-rebuild блоба; roar-3 сначала bloom-фикс.
- Л36 tick-deadband REFUTED-as-carded: потолок +0.011пп (лейн 0.0105% CPU, INTERVAL=100 ваниль) — слот 81 не тратить.
- Л37 parse-cache widen: twin λ$parse$7 байт-идентичен, 3/4 сайтов без кэша; мерить только на reload/INJECT-сцене (на soak parse-лейн ≈0).
- Л38 POI union-widening: НЕ capture-лейн (POI 0.23пп потолок); poi456-5 −26.4 = compose-дилуция −11.7..−31 → в пары не брать, ре-ролл R17 с recal.
- Л39 paldelta-diff: InsideBatchOps 4215→5529B (+31%), batchGate TRUE-ветвь = quantum без JNI → damage = wiring; P31 не переносим без P32/P36; bank-bias −4.67пп обязателен в нормах.
- Л40 hard-colliding census: item_frame×2714 (94.1% idle) стабилен → HARD_ADDS>0 перманентно, empty fast-path мёртв (capture 0/2.3%); фикс = per-region occupancy-ключ (SHIFT=5).
- Л41 банк-v4 рекалибровка: гипотеза HOST-шума не опровергнута, n=5 в банк не вшить; нужно stw-поля в ABSORB-пайплайн (стык с TASK-461-92) → CLEAN-only банк; slope-bias −4.67пп в окне 6.5-9.3M.
- Л43 Moonrise/MC-310372 скан: baseline уже == Moonrise; гейт MC-310372 ordering-hash обязателен; DRAIN_ORDER неприкосновенен; 8 гейтов preregistered, носитель = chk-ось.
- Л44 C2ME/Lithium скан: новая нога P48 cmp461_colllazy +0.5-1.2пп (потолок 2.2, young-gen −2.8-3.7пп), композ с P31 → +12.4-15.2пп.
- Л45 Paper/Pufferfish broadphase: H05⊕H07 = единый lever cmp461_papaya_hilbert (суб-аддитивность 0.6); swarx-5 drain-batch +2-4пп = приоритетный carrier; Krypton/flush/async-tracker = 0 capture на банке.
- Л46 pop-165k: flips VALID↔INVALID = 0 под v4/v5a/v5b → сертификат живучести банка 140-165k.
- Л47 brain MEM-квантование: гейт-бар +0.5пп (закон 13a), составной lever-id канон. Л48 chunk-tick budget decay: P8-бар +1.2пп → chk-14⊕P22-композиция окна B/E.
- Л49 sense interval-tree R3: REFUTED ×2 = закон-5 мертвец, не воскрешается. Л50 paletted demux: ар-existence ДА, конфликтов НЕТ; стол +2.3-3.4пп = крупнейшая env-дельта эры, очередь после swarx-3.
- Л51 young-gen: Full==9 бит-в-бит инвариант (5×CodeCache+4×Metadata); цель young −5-8.5% (108-112), alloc-дельта ≤+0.3%.
- Л52 idle-goal suppression: plane ОТКРЫТА — единственный не-REFUTED ≥3%-семейный кандидат (cmp461_idlesleep, композиция с P31/P32 ортогональна).
- Л53 JFR-pipeline GO Secondary (замена spark-report, 0 CI-диспатчей). Л54 blockentity budget: hopper-mirror REFUTED числами, соло-рычаг <1.5пп — слот не открывать.
- Л55 pathfinding stream-соло REFUTED (SUB-BAR); GO только как carrier в закон-6 подсистеме. Л56 Vec3/AABB 28-29% alloc decomposed (4 плеча) — соло REFUTED GC-физикой, Tier-B агрегат.
- Л57 capture-cadence ЗДОРОВА: добавляется дисциплина (пре-цензус, STW-CLEAN-окна only, alloc-окна депресс-ранов выбраковывать).
- Л58 entity-lookup safe = S1 enumeration + S2 caller-decimation (+2.4-4.4пп CPU → norm +5-12 при k≈1.5-3); кэш-класс RECON-39/40 по построению неприменим.
- Л59 netty watermark: REFUTED_CENS если syscall-хвост <0.1% wall — реальный capture только на ЦЕЛЬ-ДЕПЛОЙ профиле (RECON-43 player-concurrency).
- Л60 Int2Object layout: ровно одна горячая цель ChunkMap.entityMap (13 getfield-сайтов, tracker 4.6%) — dense-window flat + CHM-overflow; потолок оси +0.4-1.1пп CPU (norm +0.6-1.6), carrier-класс.
- КЛИМБ тика: chkclimb-5 +15.6@6477199; chkclimb-6 A/A +8.4@7160059 (плацебо-реплика @0061cc52 — дисперсия канон-банка −7.2пп, THRESH-ручек нет, L61); chkclimb-7 −2.8@6468251; chkclimb-9 +0.6@6593727.
- swarx-2b NO-ARTIFACT (infra), swarx-3 не диспатчена (компо-план готов), swarx-4 INVALID AIOOBE=1 (4× дубль lever → канон одиночного id); eqsnap2-4b NO-ARTIFACT (infra); poi456-5 −26.4@7169524 REFUTED (parity-лега, дилуция); paldelta-13/14 — слоты 70/71 живы, судьба = iter-2 после reload-дрилла/серта P34, диспатчей тика 0.
- roar-3 −1.9@8786643 VALID (блум-фикс ≈паритет, +2-8пп не подтверждён; ран #1 36189945821 fixture-flake); chdelta-2 +9.1@7518771 — вторая нога тика (send-lane ID-M1).
- МАСТЕР-ИНЦИДЕНТ: merge eqsnap2-3 887c4641 (21:01:58Z) вставил 61 битую union-строку (` || `-префиксы/`|| ||`/`|| |`) в 21 src/*.rs → rustc unexpected-token (queryplane.rs:106, mobs_manager.rs:213), 0/34 зелёных (якоря 71-102 CI-red 21:02-21:07Z); ремонт 9df38ba1 (pre-merge гейт-строки + eqsnap2-union правильными термами, verified 1:1:61, token-delta {cmp457_eqsnap2:61}×21, cargo check -p crussty PASS, canary 36191158970 green). УРОК-КАНОН: cargo check на merge-коммите ОБЯЗАТЕЛЕН до push.

## ТИК-463 WILD-СРЕЗ (ростер-строки 50/51; полные доки: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-50.md + LEDGER-51.md)
- Строка-50 HEIGHTMAP: REFUTED_CENS — 19 soak-профилей/2155475 сэмплов: read 0.039% + write 0.002% (29× ниже порога 1%); шторм setBlockState 0.86% CPU при 92% DiodeBlock (красный камень same-Y), heightmap-write = 0.22% шторма; нативные высоты уже DO_NOT_WIRE ratio 5.70/1.78 (P500 49гр/70пар) — двойной мертвец; чтения живут в precip 54.5%/spawn 33.6%. Потолок +0.09 norm.
- Строка-51 BLOCKSTATE-FLATTENING: REFUTED_CENS — SH.getValue-лейн 0.529% (11402 сэмпла), 98.6% уже флет через moonrise ZeroCollidingReferenceStateTable (мемо Property.Value поставлено апстримом); не-флет хвост 0.17пп → потолок +0.26..+0.45 < бара +0.5пп; mojang Pair#get 0.0001%, Property$Value alloc 0; demux-1 прецедент подтверждён (armed 5.1%-лейн → −0.3 norm).
- Урок-атрибуция: setBlockState-шторм канон-фикстуры = краснокаменный (DiodeBlock 843/917 = 92%), не chunk6-sched — TASK-463-93 трейс целиться в redstone-квант; владелец 0.53%-лейна blockstate-резолва = fluid-push item-entity (getFlow 50.3% + LiquidBlock collision 18%) — рычаг в fluid-плоскости, fluid-класс REFUTED ×2 (закон 5).
## ТИК-463 WILD-СРЕЗ доб.2 (ростер-строки 61/62; полные доки: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-61.md + LEDGER-62.md; инструменты lane_probe_refined.py + выводы в той же папке)
- Строка-61 ADV/STAT: REFUTED — 19 soak-профилей/2,351,676 CPU+918,903 WALL сэмплов: advtick (PlayerAdvancements.*+CriteriaTriggers+StatsCounter) = 2/2.35M = **0.000085%** (STW-CLEAN якоря 7 ранов = 0/805,543 бит-ноль), src/ 0 сайтов; «advancement-рейсы от deaths» = mirage: death-лейн 0.055%, trigger-фреймов в death-стэках 0/2.35M, critereon-«попадания» 0.00085% = LootItemEntityPropertyCondition→EntityPredicate (loot/damage переиспользуют critereon-классы как предикат-либ — ловушка grep-среза); flushDirty-сэмпл = JIT IC-resolve артефакт; ванильный flushDirty = уже batch-deferred (GO-идея ростера реализована Mojang) — deploy P=100 → ≤0.1-0.5% < гейта. Re-audit: деплой-JFR advtick ≥0.5% tick-thread.
- Строка-62 LEVELTICKS/REDSTONE/WEATHER: **REFUTED-on-bank / GO-on-deploy(carrier-gated)** — lt_drain 21,049/2.35M = 0.895% all-CPU / 2.081% main / **2.89% tick-thread** (полоса 2.99-4.78% по ранам); premиса «фикстура без редстоун-машин» ОПРОВЕРГНУТА: 94.1% стэков дрена ∩ CollectingNeighborUpdater, wire-eval 0.655% main (cross-val LEDGER-50: DiodeBlock 92% шторма) — фикс-мир = снапшот деплой-мира MineShield-3 S3 (run-env sha afb3a0b3); очередь LevelTicks 0.02% main мертва, тела tickBlock 1.938% main; weather 0.048% (norm ≤+0.08); pair-матем: 100%-capture norm +5.6-12.9 SUB-PAIR, реалистичный +1.7-5.2 → банк 0 диспатчей (13b); деплой-гейты G-D1..D4: деплой-JFR lt_drain tick-thread ≥3.0% И wire-eval ≥1.5% → family-bank как ⊕-ингредиент (потолок norm +5-11 при старой эре 8.28-11.55% — carrier, не pair-maker); компо с 37 существующими F3-хуками LevelTicks в src/ (javap flat==nested, NCDFE T1=0), DRAIN_ORDER/moonrise mid-tick неприкосновенны.
- Урок-срез (новая дисциплина): в эру region-воркеров (54-62% сэмплов = воркеры) all-CPU-срез занижает main-thread лейны ×2.2 (lt_drain 0.895%→2.89% tick-thread) — MSPT-верикты обязаны иметь tick-thread-знаменатель; старая эра task167 (lane 8.28-11.55%) vs банк 0.895% = 9-13× разрыв — мир/эра фикстуры менялись, старые числа не канон банка, это верхняя граница деплой-риска.
