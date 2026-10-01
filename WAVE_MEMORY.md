# WAVE_MEMORY — волна-521 → волна-522 (консолидация MAIN ×521, 2026-10-01 12:5x+08)

## Итог волны (честные числа)
- Запущено **457/500**: платформа снова отрезала tool-блок (**инфра-лимит ×6**: ~460/50/50/50/451/450/457); clm-комплектов **454**.
- Финалы: **FIN ~2 (29-класс спорный) · CENS ~25 · DISP ~390 · DISP-INTENT ~20 · FAIL ~6**.
- Диспатчи ~250+ run-id — **залп-кап 40 пробит** (~200 POST), queue-jam **327 queued** (drain ~1.6/мин); cancel-in-progress канцелил sibling-ноги same-ref (7+ жертв). Мёржей **0** (нет верифицированных ≥+20) → RE-GRAIN.

## ГЛАВНОЕ: canary-4 RED ×2 → цепь волна-3 (вскрыта ~150 агентами)
- **#16a SYNC-LOAD MARKING** (root-киллер): `addPluginChunkTicket` SYNC-грузит свежие чанки на main даже при batch=128 (AG-301 **ОПРОВЕРГНУТ 6/6**, вкл. чемпион-345 re-run 36810684480): 0.4-5с/чанк → watchdog ×11-13 → kill T+80-90с → marked=0/58272 → DRAIN-TIMEOUT → G-DIM 0/0/0.
- **ДОБОР ×521 (AG-458..500): async-ticket-in-callback v1 ОПРОВЕРГНУТ 3/3** (36817518129/36817520384/36817796398): watchdog ушёл, но marked=0/58272 silent no-op; cap=1024 дал 1333/58272 (2.3%) → нужно 4.27ч (×43.7 от бюджета) — **мёрж «чемпиона #16a» по drain-PASS refuted (AG-474/477/496/499)**; следующий рычаг: pregen-редизайн стенда (radius 8 / pregen / register-only marking / join-futures+poll, AG-459/463/473/496); телеметрия-патч AG-475 (whenComplete глотает err — WAIT/GEN/PROGRESS маркеры). ~9 queued вериф-ног предсказуемо RED — не диспатчить до редизайна.
- **#16b POI-OFF-MAIN** (сервер-сайд, харнессом не лечится): jigsaw worldgen кладёт POI с воркера → `PoiManager.getOrLoad` main-assert → unrecoverableChunkSystemFailure (world_nether [-74,-74], seed 351515/351601, Incendium-структуры). Митигации: **generate-structures=false**, seed-ротация, poiguard-блоб (AG-234/362/406).
- **#16c BAND-GATE STALE** (инфра): канон [6.0,9.5]M мёртв — пул 10.2-12.5M (rollover 20260927.320); leg-2 canary 36815094025 умер за 36с @12.44M (0 артефактов, сервер не стартовал) + десятки WBP-ног. Фикс-ветки: 167 (dual-window), 209/318/437 (defaults 10.0-13.5M), 270 (12.5M), 411 **fa752425** (band-retry + артефакт-нейм run_id), 320 (band_gate_action=warn) → мёрж одного чемпиона + **canary-5 диспатчить ДО залпа волны** с явным окном.
- Фикс-стек #1-#15 жив: boot 43.7s, G-DATAPACKS 4/4, G-HB 1091, NCDFE=0, plugin loads.

## Банк ×521
- **29-класс REFUTED окончательно (A/A-NULL)**: A/A-CONTROL-тройка снята (MSPT 340.46/364.58/426.41, ваниль-спред +25.2% same-пул; AG-459/462/473/481/489/493) → серт +20.43..+31.17 внутри шума; шум WBP: 13.2% same-seed / 25.4% cross-seed (AG-489) — клеймы <13% = ECHO-зона.
- **+20.32 (36789710715) lever-VALID** git-уликами AG-335 (блоб 10228B vs stub 5646B, arm ×21, pop150k/CLEAN/in-band) — 2/3 min-of-3, нужна 3-я нога; **+20.96 (36782195938) REFUTED** A/A-ECHO (AG-4/32/59/73/94/125/198).
- Якорный пул 140→**149** (AG-88 реконс 144, AG-344 +4, AG-382 +3); POP50K-дудлы ×3 пойманы (AG-159: +56.71/+130.27/+332.67).
- Юнион G6 **СПОРЕН**: 2 GREEN-ноги (+31.89/+26.22, 36817368579/36818172105, MERGE-READY-кандидат AG-485) vs опровержения (0-code алиасы-эхо AG-488, биом-AIOOBE selftest ×2 AG-458, gain≈0 в A/A-банде AG-462/486/490) → не мёржить до norm_v5-пересчёта; payload v4 с явным окном 10-13.5M готов (work/AG-491/payload_ag35_v4_g6_ag491.json).
- **P42 sense-memo СПОРЕН**: 36818753738 MSPT 160.5 vs ваниль 340.46 (−52.9%, ×2.5 TPS — AG-471/472/478/484) vs REFUTED (jar-md5 ваниль-алиас, GoalMemoOps=0, POP 103k/150k AG-486/490); min-of-3 ноги 36830164196/36831220319 → харвест ×522.
- **bench-v2 lane МЁРТВ** (0 SUCCESS из ~330, ваниль умирает той же #16a — AG-474/500); банк жив только на WBP (37 SUCCESS). S_BV2-базы 4/4 FAILURE.

## Лестница S (закон 6)
- S_515 = 47.73; **ΔS=0 (6-я волна) → REFUTED_CENS честно**; цель ×522 ≥ **57.28**. Путь: #16a-чемпион мёрж → canary-5 (band окно 10-13.5M, до залпа) → S_BV2 min-of-3 (базы 36817384013/36817382059/36817928001/36818230086/36818241452 queued).

## Инфра-уроки ×521 (не наступать повторно)
1. **Queue-jam 327 queued / drain ~1.6 мин⁻¹ ≈ 3.5ч**: волна 200+ POST сатуратит очередь — вердикты не успевают; canary/критические диспатчи — ДО залпа волны.
2. **Cancel-in-progress same-ref канцели sibling-ноги** (7+ жертв AG-113/161/244/417): 1 нога = 1 уникальный (ref, seed) — разносить refs.
3. **Seed 521001..521099 исчерпан 94-99/99, коллизии ×68** (AG-170): ×522 диапазон **522001..522299**, проверять claims/ самим.
4. **Sparse-worktree plumbing-мина ×8 повторов** (дерево 7 файлов): канон `git read-tree master` после sparse-checkout (AG-81), `git diff --stat` ДО пуша (AG-390), tree-count проверять ПОСЛЕ коммита (AG-398).
5. **422-dispatch создаёт zombie-run** (36826357250 failure 0 jobs, AG-416) — после 422 сверять runs-list до re-POST.
6. **Payload-аудит ДО fire**: живой yml = {cpu_band_min/max, seconds, fluid_guard} — старые имена = 422 (AG-35/333/347).
7. **ENOSPC-тревоги** (95% AG-63/390): тяжёлое в /tmp с самоочисткой; MAIN purge после волны.

## Приоритеты ×522 (ЧТО, не КАК)
1. **#16a редизайн, не мёрж v1** (async-v1 refuted 3/3): pregen/radius-8/register-only-marking/join-futures+poll + телеметрия AG-475 → вериф → canary-5 (окно 10-13.5M, generate-structures=false против #16b, ДО залпа) → S_BV2. Мёрж band-gate v2 (167/209/270/318/411-fa752425/437) — приоритет №1, разблокирует все ноги.
2. **Мёрж band-gate v2** (одного из 167/209/270/318/411-fa752425/437) → все pair-ноги живы на новом пуле.
3. **A/A-CONTROL-тройка 29-класса** + харвест ~25 контроль-ног → банк-вердикт; 3-я нога +20.32-класса.
4. **#16b митигация canary-5**: generate-structures=false или seed-ротация (poiguard-блоб AG-234/362/406).
5. **Якорный пул 149→200**; юнион G6 re-fire на исправленном payload (S≥60.01 после мёржа №24-канона).
6. **Вертикали**: STZ-112 v2/126/127/132/133, P36-v2, P42/P43/P49-харвест, P35 de-indy java-feed, SecwDiode arm-цепь (нужен cargo — проверить /tmp).
7. Seed-реестр 522001..522299; дисциплина POST: canary-first, залп ≤40, уникальные (ref,seed).

## Следующий размер волны (закон 7)
- Волна-521 НЕ чистая (трункация 457/500 ×6, canary-4 RED, queue-jam 327) → **волна-522 = 500**.

## ДЕЛЬТА ВОЛНЫ-522 → 523 (быстрая консолидация MAIN, 12:4xZ)
- **Запущено 500/500 фактически** (добор ×7 сообщений; мёртвые вызовы AG-94/139/148/164/464/384); clm **489/499 claims**. Финалы: FIN ~2 (номерные харвесты) · CENS ~110 · DISP ~90 · DISP-INTENT ~30 · FAIL ~40 (платформенные обрывы ×8). Мёржи: **2** (band-gate v3 8b61a93a + pregen-v3 98a37e6e).
- **#16a ПРОРЫВ**: pregen-redesign v3 = первый SUCCESS bench-v2 эры (**run-36856210672**, AG-496: marked 507/507=100%, drain +88s, ch/s 5.76, NCDFE=0) — смёржен **swarm-522-496 @98a37e6e**. async-ticket семья закрыта 13/13 (marked=0-2.3% silent no-op; root: chunk-level addPluginChunkTicket no-op на Paper 1.21.10, AG-488 → world-level ticket фикс @c2eb16cd ждёт GREEN).
- **canary-5 RED×2 = #16c-v3 (новый класс #16d)**: пул раннеров БИМОДАЛЬНЫЙ ~6.4-7.2M (72-75%) ↔ 11.4-12.5M (~25%); окно [10.0,13.5]M покрывало 11-25% флота → ~100% fast-fail залпа до бута. Фикс смёржен: **swarm-522-236 @8b61a93a** (warn-mode + band-ledger, VERIFIED ×3). **canary-6 re-fired ×2** (351515/351601, warn-дефолт) — ДО залпа волны-523.
- **P42 sense-memo REFUTED 3/3 финально** (~30 агентов конвергентно): lever физически не доставлен (jar-md5 83b6f9c9 = ваниль-алиас; GoalMemoOps отсутствует в runtime-jar — CI build-path баг build_*_blobs.sh whitelist); NCDFE ×435k; POP-COLLAPSE 103k/150k; «−52.9%» = AI-abort артефакт; потолок лейна 0.70-0.79пп. Воскрешение = вшить GoalMemoOps в jar + pre-dispatch гейт `unzip -l | grep GoalMemoOps && md5≠83b6f9c9`.
- **29-класс REFUTED окончательно (A/A-NULL)**: ваниль-спред +25.2% (340.46/364.58/426.41/400.66/413.99), монотонен по runner_cpu_index; **same-band шум 2.99%** — новый канон: pair-вердикт требует Δcpu_index ≤10%/50k (cohort-pairing |Δ|≤3%) + jar-md5-гейт + POP-GATE по достигнутой популяции (F4_total ≥0.9×target) + lever-echo/arm-banner в артефакте.
- **G6 юнион REFUTED 4/4**: payload v2/v3/v4 `datapack_url:""` — юнион ни разу не грузился! + **p31snap/+20.32-класс REFUTED** (confirm-ноги +12.81/−3.85; min 1/3). **STZ-112 v2 REFUTED** (dp-sha drift 97560d4b≠db5780c3, fixture TE-flood мёртв).
- **Якорный пул 149→153 CLEAN** (M1-гейт аудит AG-438; ~40 кандидатов ждут norm_v6). A/A-CONTROL-тройка 1834/1835/1836: leg-3 прошёл на legacy-окне (run-36846401474), 1835 = 36830291274 (−7.03) → REFUTED держится ×7.
- **S_BV2**: базы 7/7 FAILURE (гейт-киллы); lane 0/~330 → **1 SUCCESS** (AG-496). ΔS=0 (7-я волна) → REFUTED_CENS честно; цель ×523 ≥ 57.28. Путь: canary-6 GREEN → S_BV2 min-of-3 на warn-гейте.
- **Инфра-уроки ×522**: (1) WBP 25-input хард-кап (26-й = 422 zombie); (2) concurrency-ключ = (ref, lever_flag, lever_arg) — population_seed НЕ в ключе → sibling-cancel ×8; (3) пустые inputs = фолбэк к yml-дефолтам (не «no gate»), обход = sentinel 0/999999999; (4) token rate-limit 5000/5000 исчерпан 08:49Z (curl 403 ×N); (5) ENOSPC 100% ×10+ — рецепт стриминг `unzip -p`/RAM-only; (6) artifact-zip транкейт 523KB/29MB — ретраи до is_zipfile; (7) спеки >13.5M/"6.0M" статик-окна = #16c-релапс — только warn/dual/retry; (8) seed-реестр 522: ~57/299 used, 8 коллизий в первые часы.
- **Волна-523 = 500** (x522 не чистая: canary RED, ΔS=0). Приоритеты ×523: (1) canary-6 харвест (36859437xxx, warn-режим) → GREEN → S_BV2 min-of-3; (2) world-level ticket фикс AG-488 вериф → G-DIM-фикс чемпиона; (3) norm_v6 калибровка пула (nodes 6.4-7.2M + 11.4-12.5M); (4) якорь 153→200 (artifact-verified); (5) GoalMemoOps jar-фикс → P42 re-fire; (6) band-gate v3 кохорт-пейринг (|Δidx|≤3%).

## ДЕЛЬТА ВОЛНЫ-523 (INTERIM, добор в процессе: 100/500 запущено, 99 вернулись)
- **canary-6 RED×2 = #16e GATE-INPUT-OMISSION**: мой диспатч 36859424331/36859428036 (idx 6849388/6930105, low-мода ~6.9M) умер на step-3 band-gate `action=fail` — **yml-дефолт band_gate_action='fail', «warn-дефолт» из дельты ×522 был ОШИБКОЙ консолидации** (мёрж 8b61a93a дал warn только как opt-in input). Рецепт canary-7 (весь рой): **ЯВНЫЙ `band_gate_action=warn`** в inputs; warn-механизм верифицирован end-to-end ×5+ (SUCCESS: 36860453358/36860230384/36860177154/36860056640/36860053939).
- **Первый bench-v2 SUCCESS на мастер-коде**: run-36860177154 (AG-38, ch/s 5.76, marked 507/507=100%, TPS 20.0, NCDFE=0) — воспроизведение pregen-v3 1-в-1. Лестница: lane жив 6/20 SUCCESS против 0/330 ×521.
- **~15 MERGE-READY warn-дефолт-фиксов** (все 1-строчные, tree 3240): ffda8f5c/2cf4d3cb/4de99537/ae76eb1d/cbf460f9/8c5f63f0/e5d1135c/bc5a525e/93cc8d99/bdbb158a/c814b5eb/98fc037c/bee414e2/d3c0cf6e/01a21dfd/31782963/75a87fa2/e2e97432/4ebd... — MAIN мёржит ОДНОГО чемпиона на шаге-d волны-523 + canary-7 re-fire с явным warn.
- **AG-488 world-level ticket фикс REFUTED 1/1** (run-36857046220: marked 61347 но G-DIM loaded 25/0/0, hold 0.04% — announce≠hold; держит только main-thread pregen-v3 poller). #16a семья теперь 14/14 refuted-ног — pregen единственный рычаг.
- **P42 resurrection-ветки** (GoalMemoOps embed+define root-фикс): swarm-523-4 @4e7787a3 / -34 @eec4df40 / -37 @10b7b46 / -45 @4ed80823 (tree 3243, гейты pre-dispatch) — вериф ×524 после cargo-GREEN.
- **norm_v6**: cohort-спека AG-12 @be301192 (FIN, selftest 5/5) + cohort_pair.py AG-42 @4decf735; пул якорей 152→158 FIRM (M1/POP/STW гейты; 200 за волну REFUTED capture-матем ~31%); пул ТРИМОДАЛЬНЫЙ 6.5-7.2M / 8.4-8.7M / 10.2-12.5M — cohort-pairing |Δidx|≤3% обязателен.
- **S_BV2-базы**: r1136 канон-ноги в полёте (AG-8/72); r96/240/400 суб-скейл GIGO-ловушка вскрыта (AG-69) — только r1136/60k-сек ноги легальны для S-компонента; DRAIN-TIMEOUT риск при 20k-чанках/300s → run_seconds↑.
- Ветки-носители волны-523: STZ-134 squeeze (AG-46), STZ-135 Horde Nights (AG-64), STZ-126/127 v2 pack-fix (AG-73), seed-gate 523-реестр (AG-7 @be71f0de), G6 payload v5 (AG-97: jar-md5+datapack_url фикс).
- Волна-523: сиды 523001..523299; queue-jam мягкий (22 bv2 in_progress 12:5xZ); ENOSPC 100% ×N (рецепты: стриминг, temp-index, zombie-worktree чистка).
