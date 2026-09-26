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
- **×463 P42 iter-2 javap-контракт (полный разбор: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-39.md)**: canUse = 1 сайт (GoalSelector.tick @166, 282B, 3 пре-гейта @137/@148/@162); start/stop 19B вне мемо (1 start @265 + 3 stop @67/@240/@45) — behavior-видимая поверхность; Paper-сплит чёт/нечёт ⇒ canUse ≤1/goal/чётный тик. ПОПРАВКИ канона ×461: Sensing.tick 0.60% не капчурен (18B = 2×clear, tickSensors-плоскость) → честный лейн 0.70-0.79% (потолок 0.79пп); RNG-ценз: NearestAttackableTargetGoal.canUse не-чист (nextInt@18) → whitelist +0.3-0.5пп, phase-split iter-3 = +0.4-0.8пп. DAB/every-other-tick/ActivationRange REJECT ×3 (behavior-видимость start/stop-сдвига). Компо: на cmp421/422 vanilla tick()V dead code → GoalOps.tickGate g.canUse()@266.
- **×463 P43 iter-2 верификация сайтов (полный разбор: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-40.md)**: memories-map = 1 писатель (setMemoryInternal 57B, Map.put @51; Map.remove НЕ существует — erase = tombstone-put, коррекция BrainOps.java @30), 6 входов → 1 воронка, getMemories() 0 внешних вызовов ⇒ write-through 1 сайт = 100% покрытие; реестр: писатель @64 + bulk-clear @1 (×462 подтверждён); Brain.class = 3 мутационных сайта, изоляция от cmp421/422 дизъюнктностью классов. Flat-зеркало Object[60] = +264B/brain резидент (честная цена, 11.9MB @45k), экономия = young-чурн 96-224B/brain/тик = 172-288MB/s; capture: банк +0.3-0.5пп (канон), ×462-профиль (Brain.tick 6.85%) +0.4-0.7пп; TTL-канон ExpirableValue (17/14/11B, Long.MAX-сентинел) неприкосновенен; новая харнесс-строка write-through-immediate-read.

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

## ТИК-463 СВОДКА ЛАБЫ (v18.3; MERGE №8 swarx-1 85aea399; полные доки: /home/z/rounds/ROUND-463/LAB-STAGE/LEDGER-25..64.md)
- ДУАЛЬ-ЗАКРЫТИЕ: anchor-536 (norm −12.7@8712040, run 36206683361, STW 22.9s/175ms CLEAN, NCDFE=0) = 3-я пара chk-14 (+34.4) И swarx-1 (+30.7) одновременно; chk-14 = verdict-closure (носитель уже union через №6), swarx-1 = МЕРЖ №8 85aea399 (9 блобов пересобраны, cargo 0 err, ALL IN SYNC).
- chkclimb-5 2/3: a239 −11.8@6433490 → pair +27.4 (run 36198879422 ×461 добор); нужен 1 якорь ≤−4.4 в [6427199,6527199].
- Л62 P31-v1 strict-tail: JNI 296/тик vs 375k superset (×1266.9) = 63.6µs/тик 0.032пп; capture +9.8raw−0.73dispatch = нетто +9.5пп; прогноз ноги 25.1 (22.9-27.0); batchGate=8 инстр (не 11), TL 496KB/поток (не 512); selfTest-дыра в InsideBatchOps (0 самотестов) — C5b в чеклист.
- Л63 P32-v2: sidecar 6072B/13 полей/0 native, serveFlat 74 инстр; CHM-хоп 0.64% CPU; dΔ/dh=0.011пп/пп; chkclimb-5⊕P32 → 3-й якорь ≤−2.4..−3.5 (смягчение порога E-окна).
- Л64 getAcquire: 0 байт-сайтов VarHandle — всё ACC_VOLATILE GETFIELD; x86 цена = C2 redundant-load-запрет; capture +0.41-0.76пп; crusstySnap мёртв 0/4 живых kernel-jars (4-й сайт = инкремент к paldelta).
- Л65 NCH-декомпозиция: 56 полей/89 методов/3903 инстр, горячая поверхность 60 инстр; net-new +0.87-1.55пп; P22 = ОДИН JNI/тик → long[103] маска.
- Л66 ATTRIBUTION ×463: 225 VALID-точек, OLS R²=0.154; шум = 3-модовая смесь хостов (67%/14%/19% deep N(−13.32)); DEEP-HOST-гейт |norm|≥10.1 → pair_honest = pair−(|norm|−4.67): 6/6 пар выживают (a536-пары 30.7→22.7, 34.4→26.4); порог B' ≤−2.0 остаётся (искажение 0.065пп при Δ29k).
- Л67 swar capture: 9 natives, 2 bulk/тик, 0 per-query; broadphase 15.66→4.7% = −10.1пп CPU, k=1.78; остаток до +24 = 1.4-2.4пп (H07 на snapshotQuery 2.4%); swarx-6 прогноз нога +20.5-22.6 (P50).
- Л68 H05/H07: reader-core 6.63-7.22% CPU (71% лейна), k=0.57; H05⊕chk-14 → пара +20.8-21.4; H07 соло +0.43-0.87пп; компо-прогнозы семейства легальны ТОЛЬКО как маржа к сертованному монстру (×0.4-нож).
- Л69 demux REFUTED-страж: wall-лейн 0.23-0.32% (дефицит 10-13× до 3.0%); «стол» = CPU-бёрст-профиль; re-audit: WALL≥3.0%.
- Л70 P21: widen-плоскость ПУСТА (biomes уже кэш ×424, light=DataLayer::new не codec); 442,368 codec.parse/рестарт, hit ≥64% → только reload-метрика; PARK до санкции владельца.
- Л71 БАНК-V5 ADOPT: 0/35 флипов, MAE 5.91 vs 6.20, bias −0.16 vs −2.76; ×463 vanilla-срез MAE 8.59 vs 10.43; young-only +0.10-0.28пп, young⊕CC-kill +0.87-1.09пп; CC-джиттер 5↔6 = функция скорости хоста (сепарация 8/8); канон: STW-slope неидентифицируем кросс-хост (R²=0.86 хост-конфаунд).
- Л72 NAVMATH: 0/10 констант в src — MovePlaneOps НЕ существует (contract-ready ≠ source-ready); ASIN_TAB 9/257 ±1 ULP (glibc vs fdlibm — генератор запрещён); честный коридор ядра +0.27-0.38пп (P44-срез 0.90-0.95 после lane-дрейфа); Mth-оракул 100000/100000 бит-в-байт.
- Л73 P41: getNeighbors 92.7-93.8% findPath; Δ +0.55-1.03пп двойная сходимость; кэш 2^17×14B=1.75MB.
- Л74 P42/P43: canUse = 1 сайт (GoalSelector.tick@166); RNG-ценз (nextInt в canUse целевой семьи — phase-split обязателен); Brain мутаций = 3 (memories 1, реестр 2); резидент +264B/brain против чурн-среза 172-288MB/s young.
- Л75 P48: P48⊥P31 доказан байтами (0 общих); alloc-плечо REFUTED (375-566× ниже гейта); CPU-плечо effects-apply +0.66-1.15пп; chkclimb-13⊕P48 → 25.8-26.5.
- Л76 netty REFUTED_CENS ×7: 0/428,776 wall-сэмплов (writev+flush0+processSelectedKeys); epoll_wait 1.96% = 100% idle-park; re-audit: players≥16.
- Л77 P45: roar-3 PLACEBO (seed failed 0 — hook dormant, Armed=пусто); Capture-потолок P45-соло +0.37-0.56пп <1пп; roar-4 (ARM-fix+bloom 64KB/k12) +0.9-2.7пп.
- Л78 EQSNAP-СТРАЖ: 24/31 гейтов живо; ГЛАВНОЕ: 7/10 java-гейт-классов × 11 сайтов несут pipe-литерал equals("cmp457_paldelta|cmp457_eqsnap2") — java-слой юниона полукорруптен (merge 887c4641 склейка, 9df38ba1 чинил только rust); NCDFE-landmine; ремонт iter-3: 11 строк × 7 файлов + rebuild 7 блобов + CP-EXACT-гейт.
- Л79 INT2OBJ: overflow 0.000% ровно (P0) на 320 профилях; G4 запас 1.41× на 165k-крае (сцеплен с pop-сертификатом); 12 getfield+1 putfield.
- Л80 alloc_budget.rs СКЕЛЕТ: ветка round-463-alloc-budget @18ca090e, cargo 0 err, тесты 8/8, 0 heap-alloc, fail-closed 1 строка.
- Л81 JFR: TAP СУХОЙ 0/8 (recon.jfr за RECON_DIAG=1 гейтом — yaml-строка 341 фантом); jdk.jfr-парсер StwCensusJfr.java 6/6, cross-val gc.log Δ0.22%; наивный парсер = +49% STW-инфляция (вложенные фазы) — M1 = gc.log-primary.
- Л82 Long2Ref REFUTED: 1 сайт (коммент), остаток 0.1-0.7% после монетизации chkmono; центр capture 0.27% < гейта.
- Л83/84 WILD heightmap/blockstate REFUTED: 0.039%/0.53% лейны; шторм 92% = DiodeBlock (redstone-квант, не chunk6-sched); ZCRST уже мемо апстримом.
- Л85/86 WILD NBT/POI REFUTED: NbtIo 0.001-0.003% (IO-воркеры Moonrise вне тик-плоскости); POI 100% в startEachNonRunning (subsumed P49, аддитивность 0.00пп); P49-СТРАЖ: пустой running-сет у безработного вилладжера — guard обязан нести bypass по pending-стартаперам (behavior-visibility ×52.8% villager-фреймов).
- Л87/88 WILD noise/broadcast: P24 пул нуля (0 new в октав-цикле), c8156a69 NOT-ancestor (noise-ARM = миф); broadcast REFUTED-on-bank 0.01% / GO-on-deploy players≥48-64.
- Л89/90 WILD biome/feature REFUTED: 0/2.88M сэмплов; park-ловушка (Reaper-нити 3603 wall-сэмплов); «9.2k чанков» = миф (6,561 = 81²).
- Л91/92 WILD structure/loot REFUTED: structure 0 сайтов в kernel-src, 0.000% ×14; loot 62% = LootParams.Builder.create (аллокационная цель), ролл 11.6%.
- Л93/94 WILD advancement/redstone: advtick 2/2.35M сэмплов (11,760× ниже гейта); НО lt_drain 2.08% main / 2.89% tick-thread, 94.1% ∩ CollectingNeighborUpdater — фикс-мир несёт живые redstone-схемы (deploy-порог lt_drain tick-thread ≥3.0%).
- Л95/96 WILD random-tick/sleep REFUTED: bulk-JNI убыль (sync 215ns/хит > лейн); sleep-check уже кернел (SleepStatus event-driven); thread-разрез дисциплина: all-CPU занижает main-лейны ×2.2 в эру region-воркеров.

## ТИК-464 АБСОРБ (28 ранов волны-2 ×463 + добор; ключевые вердикты)
- navmath-1 (36211663003): PLACEBO — lever_flag=cmp463_move не дошёл до inputs (run-env без lever, ARM-маркер 0, норма −0.2 v4/+2.31 v5 = A/A). УРОК: диспатч-скрипт обязан верифицировать lever в inputs ДО запуска; ре-диспатч navmath-2 с lever.
- chkclimb-12 (36201153518): VALID ARMED cmp456_chunkmono_p31quant, +14.97 v4 / +17.64 v5 @8183686 (в K12), NCDFE=0, STW 20.0s/151ms CLEAN; AIOOBE=1 = biomes-selftest-throwable (bits=0 pos=-5,3 пустая секция) = fixture-шум канона «selfTest ×1-2 FAIL», НЕ hard-гейт. Суб-бар → CLIMB ⊕P32.
- swarx-3 (36200685322): ARMED-VALID swar_papaya компо (papaya shard-readers + brain-tick2 + soa + collide-batch + stagger + goal-query), +7.51 v4 / +10.48 v5 @6806616, STW 19.9s/175ms CLEAN. Суб-бар → CLIMB H07/drain-batch (+2-4пп, потолок семьи +24±1 Л30).
- canary-403 (36211037572): INVALID-STW-HOST 23.8s>23.0 (гейт G3), +3.97 v4 — вне ±2 паритета; canary-404 ре-диспатч.
- Якоря волны-2 (641-663): пар-хитов 0; E-окно свежие a643 +6.86/a645 +0.56 v5 — мимо порога ≤−1.60; POI a660 +7.08 v5 мимо ≤−1.99; K12 якорей 0; band-out a656/a659/a662 (12.1M/10.5M/10.6M). Вывод: глубокие хвосты в ±50k-окнах ~3-5%/диспатч → CLIMB-стратегия подъёма ног до shallow-якорного порога.
- Механика ABSORB: lever-детект из run-env строк "lever_flag:" ненадёжен (ARMED ноги с lever в stdout-маркерах) → детект по "lever_flag=X ARMED" в server-stdout; AIOOBE-счётчик должен исключать "selftest FAIL (throwable" строки.

## ТИК-464 БАНК-V5 FREEZE (TASK-464-03, 2026-09-26; док: docs/BANK_V5_FREEZE.md)
- Л97 FREEZE: банк v5 (rolling ±1.5M OLS, CLEAN-first, MIN_N=3, LOO-дисциплина, 191 ваниль-VALID якорь) заморожен 2026-09-26 ДО минирования E/K12/POI — ре-литигация порогов после старта минирования = подгонка, запрещена. Числа adoption: MAE 5.91пп vs v4 6.20 (−0.29), bias −0.16пп vs −2.76 (+2.60), флипы 0/35 (35 pair-комбо × 6 ног).
- Л98 ПОРОГИ-V5 6 НОГ (anchor ≤ leg_v5−20, LEDGER-35 §4): chk-14 ≤+3.22 (leg +23.22), swarx-1 ≤−0.47 (+19.53), chkclimb-5 ≤−1.60 (+18.40) в E=[6427199,6527199], chkclimb-12 ≤−2.57 (+17.43; x464 ре-мер ноги +17.64) в K12=[8133686,8233686], poi456-4 ≤−1.99 (+18.01) в POI=[8907260,9007260], chk-19 ≤−3.89 (+16.11).
- Л99 ПРАВИЛО В-ТОЧЕК: только vanilla VALID-якоря (NO-TPS/NO-ARTIFACT отфильтрованы), band 6.0-9.5M (приоритет 8.5-9.3M — 8.7M-кластер n=32, LOO-плечо максимальное), HOST-excl STW>23s/avg>200ms из фитов (CLEAN-first), deep-host |norm|≥10.1 → pair_honest-гейт.
- Л100 ИНВАРИАНТ: v4-терминалы порогов НЕизменны (порог и якорь сдвигаются вместе — пар-решётки не пересчитываются, потому 0/35 флипов); tps_exp_v5: 6.5M→2.1252, 7.0M→2.2047, 7.5M→2.3271, 8.2M→2.4732, 8.7M→2.5981, 9.0M→2.6280.
- Л101 СТРАЖ: near-bar 4 якоря E-окна (a59/a297/a237/a268) = m5 +19.5..+19.7 — в 0.3-0.5пп от бара, банк-выбор материален; canary-402 v5 −3.94 остаётся RED (head-дрифт, банк-инвариантно); MAE-флор ±0.57пп CC-джиттера — кандидат v6 = stw-ковариата (стык TASK-463-87).

## ТИК-464 ЛАБ-АГЕНТ-48 (TASK-464-48, 2026-09-26; док: /home/z/rounds/ROUND-464/LAB-STAGE/LEDGER-48.md)
- Л102 paletted-demux RE-AUDIT (WALL≥3.0% правило): x464-лейны по raw-collapsed navmath1 (=run 36211663003, PLACEBO=чистая vanilla-база; cpu 116,469 / wall 61,254 сэмплов; r*-collapsed не сохранились, 39/39 дайджестов BOTTLENECKS_3 живы): CPU self 4.857% (5,657) / incl 6.095% (7,099), WALL self 0.256% (157) / incl 0.328% (201); биом-лейн CPU incl 0.063% / WALL incl 0.007%; кросс-ран 39/39: PalettedContainer.get CPU 3.1–4.8% (медиана 4.0%), WALL 0.1–0.3% (медиана 0.2%), wall≥3.0% = **0/39**. Бёрст-фактор cpu/wall = **18.97× self / 18.58× incl** при базовом тотал-ратио 1.90× → лейн бёрст-экстракласса (~10× бёрстier базы). ВЕРДИКТ: **REFUTED-guard** — WALL-страж закрыл demux-лейн на x464-мастере (дефицит 9.1–11.7× до 3.0%); «стол» +2.3–3.4пп = CPU-норма, wall-конверсия ≤0.33пп (7–10× инфляция стола), потолок ниже закона-13a бара 0.5пп. Флаг cmp457_paldelta жив в блобах (javap ALL IN SYNC: standalone-терм в 9 гейт-классах, pipe-литерал 0/335 cp-exact; Rust-гейт paletted.rs:85) — рычаг спит до wall-сигнала. Диспатч round-464-lab48-v1 НЕ производится. Внешние: Lithium chunk_palette (порт-прецедент items_oss), C2ME BiomeContainer-тракты (зона = worldgen, тик-лейн 0.063% пуст), Paper/Purpur data-palette (уже в ядре 1.21.10, wire-эталон RESULTS_LEDGER:974).
