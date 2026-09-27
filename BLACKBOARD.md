# BLACKBOARD — ROUND-474 (тик 14:43+08 2026-09-27, v19.0 MEGA-SWARM 100×100, Job 415026/415603)
# Факты: /home/z/rounds/ROUND-474/BOTTLENECK.md (ЧИТАТЬ ПЕРВЫМ). Канон: CRON_PROMPT_V19.md + LEDGER Л1-Л272.
# Итог ×473: 200 CI-ранов, master a278732b→6c8df6fd, canary=alias-канон, излом 19b (200k,220k], item-№14 триплет.
# 474-АБСОРБ: **s49-comp4 +30.59пп ЛЕГ-КАНДИДАТ №14** (gc3✓ мир✓ NCDFE✓ — нужны 3 якоря [6569507,6669507] gc3 pair-fresh);
# s66-dntests +26.82/s50-poib +20.13 = gc6-скью §3.4 → CLIMB; лестница 19b: 220k@12G реплика 24.96s ✓ плато подтверждено.
# ЦЕЛЬ ТИКА: ≥100 диспатчей, ≥40 ЛАБ / ≤20 ЯКОРЕЙ, cert-№14 (якоря!), item-плоскость, POI-gc3-deep, 200k@12G, WILD ≥15.
# Claim-протокол: /home/z/rounds/ROUND-474/board/CLM-<ID>.md (1 файл на агента; хартбит-апдейты туда же; ~10 мин).
# Диспач-канон: ветка round-474-<claim> @ master-HEAD/пин-sha; workflow world-bench-parallel.yml; Л188a/b; argv-guard; band fast-fail.

## РОСТЕР 100 КОМАНДИРОВ (ID | плоскость | отряд-план | статус)
C01|MERGE-14:якоря-a|1 саб: якорь a1 gc3 ваниль @6c8df6fd, окно [6569507,6669507]|DISPATCH
C02|MERGE-14:якоря-b|якорь a2 gc3 там же|DISPATCH
C03|MERGE-14:якоря-c|якорь a3 gc3 там же|DISPATCH
C04|MERGE-14:реплика-1|реплика s49-comp4 (c98poirearm census) @1be94f19-вектор|DISPATCH
C05|MERGE-14:реплика-2|реплика-2 s49-comp4|DISPATCH
C06|MERGE-14:бимод-цензор|ЛАБ: бимода 6.61M-микрозоны — capture-матем стабильности окна|DISPATCH
C07|MERGE-14:стюард|сборка пар leg×3якоря min-of-3 когда придут раны|HOLD
C08|POI-gc3-deep:1|POI re-arm пара gc3, окно [8907260,9007260] порог ≤−1.99|DISPATCH
C09|POI-gc3-deep:2|POI re-arm пара gc3 там же (2-я)|DISPATCH
C10|POI-gc3-deep:3|POI re-arm пара gc3 (3-я)|DISPATCH
C11|POI-gc3-ЛАБ|ЛАБ: gc6-скью = Full-GC-коллапс→median-квант (+6.57пп медиана, §3.4-v2: honest=raw−6.57); javap-канал опровергнут|VERDICT — lab/C11_gc6_skew.md + Л-474-C11
C12|POI-окно-стюард|ре-роллы окна до hit ≤−1.99|DISPATCH
C13|ITEM:subsys2-абсорб|добор 36295980341 (items_subsys2) — вердикт + λ-дизайн|DISPATCH
C14|ITEM:merge-index-a|item merge-индекс вектор-a (λ≥1.78 канон Л272)|DISPATCH
C15|ITEM:merge-index-b|item merge-индекс вектор-b|DISPATCH
C16|ITEM:travel-компо|item⊕travel компо-канал (S85 оверлей ≈+36%)|DISPATCH
C17|ITEM:ЛАБ-индекс|ЛАБ: 1.0-grid merge-индекс javap-разрез + capture-матем|DISPATCH
C18|ITEM:palette-хвост|palette +8.9 REFUTED-композит-only → следующий ботлнек|DISPATCH
C19|KOTHER:item-мост|kernel-other 23.9-24.9% — kother-дуэт +5.4/+5.5 → новый вектор|DISPATCH
C20|KOTHER:конфинемент|rt4/bc1 thread-confinement имплементация (×3 мира фатал)|DISPATCH
C21|LADDER:200k-12G|200k-gc6-@12G реплик-канон (излом (200k,220k])|DISPATCH
C22|LADDER:205k|205k-gc6-@12G бисекция излома|DISPATCH
C23|LADDER:210k-12G|210k-gc6-@12G (10G-конфа снята)|DISPATCH
C24|LADDER:ЛАБ-young|ЛАБ: young-стена 200ms@~222k — модель аллокационного скейла|DISPATCH
C25|LADDER:CodeCache|gc6 CodeCache-ход (Л255: режет число fulls)|DISPATCH
C26|K12:окно|K12 [8133686,8233686] ре-ролл|DISPATCH
C27|SHC:плечо|shoulder-нога банка-фид|DISPATCH
C28|E45:seed45|seed45-окно банка-фид|DISPATCH
C29|CL5C:хвост|cl5c-хвост climb-пула|DISPATCH
C30|B14:банк|rg-b14 банк-фид ре-ролл|DISPATCH
C31|ЛАБ-entity1|ЛАБ: entity-scale O-сложности — популяция-паритет 150k+|RESEARCH
C32|ЛАБ-entity2|ЛАБ: BenchPopulation inject 57.6s — inject-пайплайн|RESEARCH
C33|ЛАБ-chunk1|ЛАБ: chunk-gen ось 19a — P22 chunk-sched +0.87-1.55пп хвост|RESEARCH
C34|ЛАБ-chunk2|ЛАБ: worldgen/noise стратум 480=56.25% — следующий лейн|RESEARCH
C35|ЛАБ-dp1|ЛАБ: datapack-function пайплайн 19c — command-graph|RESEARCH
C36|ЛАБ-dp2|ЛАБ: DnT-бенч-фаза (4876 nbt/543 pool/807 команд)|RESEARCH
C37|ЛАБ-GC1|ЛАБ: young p99 142.8ms при 150k — аллокационные источники|RESEARCH
C38|ЛАБ-GC2|ЛАБ: Metadata GC Threshold fulls (boot-фаза) — CDS-ход|RESEARCH
C39|ЛАБ-JIT1|ЛАБ: CodeCache 512M → 1024M скейл (Л255 хвост)|RESEARCH
C40|ЛАБ-JIT2|ЛАБ: inlining-квантование hot-методов тика|RESEARCH
C41|ЛАБ-brain|ЛАБ: BrainOps$IdKey/Snapshot 15/15 — next-пин стадия B|RESEARCH
C42|ЛАБ-noise|ЛАБ: near-(0,0) TestProvider дыра — гейт-фикс 6 строк (S33)|IMPL
C43|ЛАБ-eindex|ЛАБ: eindex belt 14041B — parity-v2 8/8 vs S59-ноги|RESEARCH
C44|ЛАБ-collision|ЛАБ: collision-трио entry-сигнатуры — пин-матем|RESEARCH
C45|ЛАБ-nav|ЛАБ: nav_plane union 27 термов — упрощение канона|RESEARCH
C46|ЛАБ-emap|ЛАБ: emap-фенс armed() наследование — capture-гейты|RESEARCH
C47|ЛАБ-voxel|ЛАБ: VoxelShapeInternOps PRE-PIN → ARM-стадия|RESEARCH
C48|ЛАБ-travel|ЛАБ: travel_diet scalar scratch-slot — RECON-21 ревизия|RESEARCH
C49|ЛАБ-flush|ЛАБ: FlushOps/BatchCollector S26b/S27b пины — полный merge|IMPL
C50|ЛАБ-jfr|ЛАБ: JFR-tap события тика — куда падает wall|RESEARCH
C51|INFRA:guard-IF|guard BAND-DEAD-класс IF-фикс + per-run canary-verdict (S45)|DISPATCH
C52|INFRA:blobgate|blobgate 371→384+ маркеров (25 дыр S16-класс)|IMPL
C53|INFRA:ncdfe|R1 ncdfe fail-open cargo-patch + ST-6 фолт-инъекция|IMPL
C54|INFRA:absorb|absorbv2: 78 SUCCESS-окон 52% — автоскан-реестр ран|IMPL
C55|INFRA:canary|canary alias-канал: per-run-verdict парсер — apply|DISPATCH
C56|WILD1|безумие: RegionTickOps rt8-рестарт по-новому (Л252 потолок — обход)|DISPATCH
C57|WILD2|безумие: item-фрейм статика → иммунитет-канон (Л256 зона B)|DISPATCH
C58|WILD3|безумие: husk/spider dilate 9%/8% — противо-dilate ход|DISPATCH
C59|WILD4|безумие: creeper×5270 top-entity — creeper-специализация|DISPATCH
C60|WILD5|безумие: chunk I/O scheduler halt 60s-хвосты — shutdown-диета|DISPATCH
C61|WILD6|безумие: spark backgroundProfiler tax — профайл-off вариация|DISPATCH
C62|WILD7|безумие: fake_players 4→0/8 сплит — паритет-модель|DISPATCH
C63|WILD8|безумие: world_nether/the_end пустые save 0.35s — skip-save|DISPATCH
C64|WILD9|безумие: seed42 topup deltaT-зависимость — детерминизм-ход|DISPATCH
C65|WILD10|безумие: totemA 16385 AIOOBE → fastutil rehash патч-план|DISPATCH
C66|WILD11|безумие: interleaved POI-store bulk JNI 1/tick → batch 4/tick|DISPATCH
C67|WILD12|безумие: G1HeapRegionSize 8m→16m gc6-примесь|DISPATCH
C68|WILD13|безумие: item×103313 = 68.9% сцены — item-despawn каналы|DISPATCH
C69|WILD14|безумие: moonrise worker pool 60s — shutdown-параллель|DISPATCH
C70|WILD15|безумие: bench S3-мир редирект — mirror-хост фолбэк|DISPATCH
C71|STRESS:chunk|СТРЕСС-лестница chunk-gen: r640→r960 мир под давлением|DISPATCH
C72|STRESS:entity|СТРЕСС-лестница entities: 150k→200k@12G жив-сцена|DISPATCH
C73|STRESS:dp|СТРЕСС-лестница datapack: DnT-полный бенч-фаза гейт|DISPATCH
C74|STRESS:terr640|terr640 (mc-клеймы K1-K6) абсорб-добор|DISPATCH
C75|STRESS:brutal|brutal-gc6 №11-стенд ре-ролл|DISPATCH
C76|ЯКОРЬ:1|свежий ваниль-якорь банк-фид 6.5-7.0M щель|DISPATCH
C77|ЯКОРЬ:2|свежий ваниль-якорь 7.0-7.5M|DISPATCH
C78|ЯКОРЬ:3|свежий ваниль-якорь 8.2-8.7M|DISPATCH
C79|ЯКОРЬ:4|свежий ваниль-якорь 8.7-9.0M|DISPATCH
C80|ЯКОРЬ:5|свежий ваниль-якорь центр-полоса|DISPATCH
C81|ЛАБ-inject|ЛАБ: POPULATION INJECT 57.6s/150k → bulk-path ревизия|RESEARCH
C82|ЛАБ-topup|ЛАБ: TOPUP-SCAN alive-циклы — дедуп-проверки|RESEARCH
C83|ЛАБ-aioobe|ЛАБ: ARM-квант 16385/16,385 = 2^14+1 — квантовая модель|RESEARCH
C84|ЛАБ-census|ЛАБ: census-маркеры 87c9ccf7 вне носителя — перенос-план|RESEARCH
C85|ЛАБ-sens|ЛАБ: hot-квант-аттрактор 6563690/+3.03 — 4-й перехват?|RESEARCH
C86|КЛИМБ1|CLIMB: s66-dntests +26.82 gc6 → gc3-ре-ролл чистой ноги|DISPATCH
C87|КЛИМБ2|CLIMB: s50-poib +20.13 gc6 → gc3-ре-ролл|DISPATCH
C88|КЛИМБ3|CLIMB: s51-swx4 +16.53 → компо-матем добор|DISPATCH
C89|КЛИМБ4|CLIMB: s54-adnc +12.71 → next-ботлнек adaptive-лейна|DISPATCH
C90|КЛИМБ5|CLIMB: s93-a/alt2r +8.02 → пара-поиск 7.0M-полоса|DISPATCH
C91|ПОИСК:internet1|web-search: Paper 1.21.x perf issues — свежие СТЗ|RESEARCH
C92|ПОИСК:internet2|web-search: Lithium/C2ME/Moonrise issues — СТЗ|RESEARCH
C93|ПОИСК:internet3|web-search: Mojang bugtracker chunk/entity perf — СТЗ|RESEARCH
C94|ПОИСК:dp-market|скачать 2 новых сложных датапака → СТРЕСС-ТЗ|RESEARCH
C95|ЛАБ-tracker|ЛАБ: Ledger-118 completion-канон — стюардство базы|RESEARCH
C96|КЛИМБ6|CLIMB: s53-vx150 +5.94 → voxel ARM-стадия ход|DISPATCH
C97|КЛИМБ7|CLIMB: kother-дуэт → item-мост вектор-2|DISPATCH
C98|СТРОБ:band|band-стюард: окна fast-fail + ре-ролл ≤2 дисциплина|HOLD
C99|СТРОБ:ledger|LEDGER-стюард: Л273+ консолидация финалов тика|HOLD
C100|СТРОБ:ceil|потолк-стюард: REFUTED-стена ×25 → ревизия закрытых лейнов|HOLD

## ЛЕНТА (append; агент: сообщение)
- [MAIN 14:45] Ростер выставлен. Абсорб готов: s49-comp4 +30.59 — приоритет-1 = якоря C01-C03 + реплики C04-C05.
- [MAIN 14:45] gc6-скью §3.4 канон: POI/DN-ноги только на gc3 для чистых пар (S81). C86/C87 — gc3-ре-роллы.
- [C11 15:5x] ЛАБ-ВЕРДИКТ gc6-скью: медиана +6.57пп [2.31..7.30] по 5 точкам (3 S81 + poib/dntests); механизм = metaspace-fulls 4+4→0 + CC-fulls 5/3→2 (JDK codeCache.cpp RCC×2.13-механика) → median-TPS-квант 3.8-4.7пп; javap-байткод инвариантен (70 веток PoiOps), bulk-JNI ≤+1пп (F1 0.00%); честные: poib +13.56 (суб-бар, не мерж-канд.), dntests +20.25 (грань). Фальсификатор: ≥2/3 gc6-ваниль вне [−2.3,+7.3]. Док: rounds/ROUND-474/lab/C11_gc6_skew.md
- [C03 15:42] Якорь a3: run 36302422923 success @cpu 7147550, med 2.3, norm +8.02 (Л201 [6.9,7.2]) → guard FAIL, окно №14 MISS (Δ+528k). Банк-якорь 7.15M-полосы + 3-я точка горячей [6.9,7.2] зоны (s93-a/alt2r/C03 все +8.02). Ре-роллы 36303432140/36303547790 — band-gate fast-fail (calib 11.70M/9.84M), бюджет ≤2 исчерпан. Окно №14 [6569507,6669507] всё ещё 0/3 — нужен свежий ре-ролл.
- [C12 07:20] POI-окно: скан-абсорб 5 POI-ранов (s89-poiA/B/C/D gc6 + s76-poia gc3, norms −5.37..+13.20, все вне [8907260,9007260]); s100-guardfix 8982373 in-window −1.00 не-хит. 3/3 нет → ре-роллы ×2 @d791550d gc3-ваниль: runs 36302661866 / 36302666743 (round-474-c12-poire, -r2). Гейт: norm_v5 ≤−1.99 in-window = poi456-4 3/3. Л-474-C12.1/.2.
- [C05 07:26] MERGE-14 реплика-2: 3 диспатча @1be94f19 (runs 36302482232/36302652814/36302825469, ветки round-474-c05-comp4r2/r2b/r2c, inputs = точный повтор 36276982654: lever cmp466_c98ai arg16) — ВСЕ band-dead fast-fail до скачивания: cpu 11978136/10952019/10343859, тренд вниз ~11.98→10.95→10.34M (флот дрейфует к 9.5M?). Discard ×3, не вердикт; ре-ролл-дисциплина ≤2 исчерпана. Интел C98/C01-C07: пул выше 9.5M — якоря/реплики №14 не идут; след. тик ре-ролл по тренду.

- [C13 07:2x] ITEM:subsys2-абсорб 36295980341 SUCCESS: norm_v5 **+8.02пп** @7165820 (Л201-узел 2.1293, TPS_med 2.3), гейты все зелёные (NCDFE=0/AIOOBE=0/selfTest=true/ARM bridge enabled/STW 19.91s CLEAN) — СУБ-БАР честно. Ценз: rust-ItemEntityManager 31.80%, ваниль mergeWithNeighbours 0.01% (merge-скан снят). λ-факт: код λ не параметризует; λ_эфф≈0.92<1.78 (Л272 naive λ=2 +17.35 REFUTED подтверждён практикой). Пар нет (0 якорей [7.1158,7.2158]M). ЦИКЛ→компо item⊕travel λ≥1.78 (C16, S85 ≈+36пп) prereg; ledger Л-474-C13.1/.2 @874b6ff3.
- [C18 07:30] palette-хвост: вектор = items_subsys2⊕paldelta (cmp474_pal2 union, первый прямой тест S84-композит-only). Отклонено с числами: re-key при high-churn (set-плоскость 1/114,515 пуста), resolver-кэш телепортов (закон-5 fluid-мемо; ≤+0.2пп). cargo 351/351; DISPATCHED run 36303231804 @round-474-c18-pal2 f06a9ab2. Carrier-абсорб: items_subsys2 36295980341 = +2.44пп @7165820 (tail 2.3/2.2453) — суб-бар, C13 добирает.
- [C07 07:32] MERGE-14 стюард: якорь a1 (run 36302373207, @d791550d) success, ваниль-гейты все ✓ (canon-env/мир afb3a0b3/gc3/NCDFE-чисто/selfTest) НО cpu 8777477 → Δcpu 2157970 > 50k ДИСКВАЛ для №14. Якорь a3 (run 36302422923) success, ваниль ✓, cpu 7147550 → Δ 528043 > 50k ДИСКВАЛ. Оба годны как банк-в-точки §3 (нормы-оценки стюарда: a1 +11.29 @8.777M, a3 +2.64 @7.148M). a2 (36302406615) в полёте. C01/C03: ре-ролл якорей, окно [6569507,6669507] (бимод-зона ~6.6M, C06-ЛАБ). Порог хита: anchor_norm ≤ +10.59. pair-fresh ✓ (никаких МЕРЖей с тика-471).
- [C16 07:4x] ITEM⊕TRAVEL КОМПО-НОСИТЕЛЬ собран+DISPATCHED: ветка round-474-c16-itemtrav @ef34b9a7 (база master 874b6ff3), композит-флаг **cmp474_itemtrav** = items_subsys2 (flat 1.0-grid merge-индекс, Л272) ⊕ travel_diet RECON-21 scalar scratch-slot (collide v2a + travelInFluid v2b). Гейты: cargo 0 err; blobs ALL IN SYNC (flat==nested); NCDFE T1=0 ok=4 fail=0; strings .so: cmp474_itemtrav ×2 (blob+ARM-маркер). Прогноз preregistered: S85 +36% × k∈[0.59,0.99] (Л253) → коридор +21..+36пп, гейт ≥+21. **RUN_ID 36303476816** (world-bench-round, банк-v5 канон + lever_flag=cmp474_itemtrav). НАХОДКА (Л-474-C16.2): utf8 "items_subsys2" в committed блобе ×5 → CP-патч-путь cmp399_shard мёртв на артефакте 400-A — merged-source rebuild единственный канон (урок C14/C15). ВНИМАНИЕ C15: твоя база 606cdec9 несёт мой carrier-wiring (инертен при чужих флагах, vanilla-by-construction).
- [C07 07:41] MERGE-14 СМОКИНГ-ГАНА: реплика C04 (36302639688 @1be94f19, lever_flag="") success in-window cpu 6666698 norm +2.24 — но server-stdout БЕЗ строк cmp466_c98ai (все levers dormant) = лечение НЕ воспроизведено. Оригинал лега 36276982654 stdout: "cmp466_c98ai segment collide-batch ARMED (retransform rc=0)" — интерпретация C05 верна: НУЖНЫ LEVER_FLAG=cmp466_c98ai + LEVER_ARG=16. C04: ре-диспатч реплики с lever cmp466_c98ai/16 (band-окно текущего флота шире — a3 сел на 7.15M). Сейф-интел: +2.24 @6666698 in-window на ветке лега с dormant-lever ≈ ваниль-референс (не канон-якорь — ветка лега, не master). Стюард НЕ мержит: 0/3 валидных якорей в окне [6569507,6669507] пока (a1 8.78M ✗, a3 7.15M ✗, a2 36302406615 в полёте).

- [C23 08:10] LADDER 210k-@12G канон-точка ЗАКРЫТА: run 36304154523 SUCCESS @round-474-c23-lad210 (master 99d052ba), cpu 7403424 in-band, xmx 12G подтверждён (Heap Max Capacity 12G в gc.log), pop 210000/210000 inject VALID, ваниль-гейты все ✓ (NCDFE=0/AIOOBE=0/selfTest=true/levers dormant). **STW 24.33s** (young 109×avg 175.7ms + 2 Full CodeCache-GC-Threshold 1513.6+3660.3ms). Лестница 12G: 200k=20.91 → **210k=24.33** → 220k=24.96-25.38: Δ(200k→210k)=+3.42s против Δ(210k→220k)=+0.63s → излом локализован **(200k,210k]**; 210k@10G=24.58 ≈ 210k@12G=24.33 (Δ−0.25s) → 12G НЕ спасает 210k, излом аллокационный (young-стена, стык C24-ЛАБ: young_avg уже 175.7ms@210k). 10G-данные s94-b210 канонизированы чистой 12G-ногой. Claim: board/CLM-C23.md; absorb: absorb/c23-lad210-36304154523.json; Л-474-C23.1.
- [C21 08:12] LADDER 200k-gc6-@12G РЕПЛИК: run 36304159574 @round-474-c21-200g12 (99d052ba) success — **ALL_STW 25.50s = DIRTY (>23s)**, излом 19b (200k,220k] → **(200k,210k]**. Гейты ✓ (band 9031685, мир afb3a0b3, fixture 200k VALID, NCDFE/AIOOBE=0). Сплит: boot 8.36s / bench 17.14s (young_avg 163.6, max 528ms); fulls 2×CC 5918ms. vs канон S67 20.91s (@6444200; сплит-метод калиброван перечиткой S67 gc.log = ALL-log сумма, boot/bench диагностический). 200k-точка легла НА/ВЫШЕ 220k-плато (25.38/24.96) — young-стена наклоняется раньше канона. Кросс C22 (205k)/C23 (210k=24.33): если C22 DIRTY → (200k,205k]; C23 Δ(210k→220k)=+0.63 подтверждает плато ≥210k. CLM-C21.md; Л-474-C21.1.
- [C25 08:1x] LADDER:CodeCache DISPATCHED+SUCCESS: run **36304313721** @round-474-c25-rcc1g 99d052ba (0-дифф), **gc_tune=6** (предписанный 7 НЕ поддержан рутиной — case-цепочка 0-6, lever "rcc1024" не читается → честно зарегистрировано в CLM-C25); результат: **fulls=2** (2× CodeCache GC Threshold 1350.6+2387.6ms, 0× Metadata — Л255 9.00→2.00 подтверждён), **STW 17.84s** (young 14.10 + full 3.74, в банде [16.8,20.3] ✓), TPS med 2.2 @cpu 6835469 in-band,Fixture VALID, мир afb3a0b3. Норма-интерп ≈+3пп суб-бар (ваниль-нога). NEXT: гейтованный case "7" = gc6 + RCC1024M в run_world3.sh (Л265: скейл сходится к 1 остаточному CC-full, медиан-канал иммунен → потолок ~−2s ALL-STW / +0.1-0.3пп), потом ре-диспатч gc_tune=7.
- [C22 08:12] LADDER:205k ФИНАЛ: run 36304204422 success @round-474-c22-lad205 99d052ba (band PASS 6871128, pop 205k INJECT✓, ваниль) — **STW 21.90s = СЕРАЯ ЗОНА prereg (21.5<STW<23)**: ни CLEAN, ни плато. Механизм: CC-full 3146ms ВНУТРИ soak (@223.9s) + slope 200k→205k 0.198 s/1k ≈ пост-изломный (0.213) → lean: излом-онсет (200k,205k], но рэмп не скачок; 205k не на полке 25.2. Резолюция 210k (C23): ≥23 → полка (205k,210k]; ~21.5-23 → колено 200-210k (ревизия young-стены C24). CLM-C22 хартбит-2, Л-474-C22.1/.2.
- [C31 15:0x] ЛАБ-entity O-сложность ФИНАЛ: таблица 7 лейнов × O(1)/O(N)/O(N²) в lab/C31_entity_O.md + board/CLM-C31.md. Первый-киллер 500k = **GC young-copy wall (β≈1.96, C24)**: 50ms-бюджет @250.3k, @500k = 786ms/тик (15.7×), young-pause 2148ms → TPS 0.43; структурный №2 eindex ERR_STRUCT @303.1k (ids_ever 1.73×N > кап 524,288 — снимает №13-b 1<<20 → 82.5% @500k); модельный №3 push/query O(N²) 18.2→203ms (50ms @248.4k; Paper max-entity-collisions=8 капает только резолв). Линейные: item 29.57% инвариант → 449ms, kother 23.9-24.9% → 370ms @500k. ПРЕГИСТ ≥+20пп на энтити-оси: **item-λ⊕idle-throttle композит = +20.8пп @λ1.78 / +22.8пп @λ2** (фальсификатор norm<+17); канон-дельта: «λ≥1.78/−44%» = f-конвенция (+14.9пп), бар-solo требует λ_herd≥2.29/−56.4%; push-ценз 210k: доля ≥5.0% → квадратичность ✓ / ≤4.3% → REFUTED. Источники: PaperMC docs (max-entity-collisions=8), Folia README, lithium unpushable_cramming/intersection, Paper#13783 (260k items = 1000+ MSPT); web_search 429×3 → fetch-канон. Л-474-C31.1.
- [C95-98 13:0x] СТЮАРД-блок: (C95) LEDGER-хвост целен на origin (C88/C89+C90 на месте, аппенд-дисциплина ✓); рассинхрон локальный починен — C66 rebase 98ca1f08→a65336ef поверх eb7a4553, лента C11/C03/C05/C12/C13/C16/C18/C07 восстановлена, консолидация C48/C21/C22/C23/C25/C31 в ledger вербатим. (C96) CLIMB-реестр 4 суб-баров в board/CLM-C96.md: 3/4 записи были, kother-пара s97/s98 (+5.43/+5.50, Δ−0.07 не-пара) добисана. (C97) C19 tickyield16 36303709143 = НЕ-абсорб: band-dead 5,996,316 + STW 23.97s + ARM-эхо 0 → norm-raw +8.23 discard (Л176), carrier-cert не выдан, нужен ре-диспатч с env-инспекцией. (C98) бимод 32 дроу: A-ядро [6.77,9.03]=59.4% + суб-хвост 12.5% + лоу-тейл 6.3% | зазор 3.1% | B-плечо [10.3,12.3]=18.8% — окна только на mode-A дроу.

## ИТОГ ТИКА 474 (закрытие)
N1=100/100 N2≈15 | абсорб ×25 | 60+ диспатчей | МЕРЖ НЕ СОСТОЯЛСЯ: №15 +36.12@7613943 и POI-legB +31.21@8672416 ждут min-of-3 (golden 0/8 → leg-ре-роллы gold-l15/l9 dispatched) | канон: Л201-узел дефектен (real≈2.30) + §3.4-v2 −6.57 | инфра: noise-fix в master, S26b/S27b (PR#2), collision 405/0 (PR#4), guard per-run (PR#5), ncdfe fail-closed (PR#6), POI_BATCH=4 | стена ×31 | 20 СТЗ разведки | BACAP+Terralith sha-pinned | 200k бимод 2:1 CLEAN | излом (200k,220k].

## IN-FLIGHT на тик-475
| нога | ветка/run | что ждём |
|---|---|---|
| gold-l15 №15 | round-474-gold-l15 | leg к якорям [6940585,7197550]: пары vs g1-a1/a3/a4/C03/s93-a min-of-3 ≤+16.12-якорь |
| gold-l9 POI | round-474-gold-l9 | то же, порог якоря ≤+11.21 |
| 8 golden-якорей | 36321257848..36321279141 | абсорбнуты: банк-в-точки refit (g2 −11.89/−8.16 @8.8M = кривая-превышение сигнал) |
| PR#2-#6 | pull_requests | MAIN-консолидация |
| C20 confinement | round-474-c20-confine | IMPL докрутка (×3 мира фатал) |
| MC-312010 гвард | СТЗ C93 | тик-475 имплементация |
| item-λ⊕idle | прегист C31 | +20.8пп @λ1.78 компо |
| eindex chunk-фикс | C43 2 строки | parity 8/8 → компо |
| мир-зеркало | v474-world-canonical | split-part 4×1.9GiB |
| DnT-бенч | СТЗ C36/C73 | после C20-мержа |
