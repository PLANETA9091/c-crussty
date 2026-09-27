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
C11|POI-gc3-ЛАБ|ЛАБ: почему gc6-скью только у POI/DN — javap-контраст|DISPATCH
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
- [C12 07:20] POI-окно: скан-абсорб 5 POI-ранов (s89-poiA/B/C/D gc6 + s76-poia gc3, norms −5.37..+13.20, все вне [8907260,9007260]); s100-guardfix 8982373 in-window −1.00 не-хит. 3/3 нет → ре-роллы ×2 @d791550d gc3-ваниль: runs 36302661866 / 36302666743 (round-474-c12-poire, -r2). Гейт: norm_v5 ≤−1.99 in-window = poi456-4 3/3. Л-474-C12.1/.2.
- [C05 07:26] MERGE-14 реплика-2: 3 диспатча @1be94f19 (runs 36302482232/36302652814/36302825469, ветки round-474-c05-comp4r2/r2b/r2c, inputs = точный повтор 36276982654: lever cmp466_c98ai arg16) — ВСЕ band-dead fast-fail до скачивания: cpu 11978136/10952019/10343859, тренд вниз ~11.98→10.95→10.34M (флот дрейфует к 9.5M?). Discard ×3, не вердикт; ре-ролл-дисциплина ≤2 исчерпана. Интел C98/C01-C07: пул выше 9.5M — якоря/реплики №14 не идут; след. тик ре-ролл по тренду.
