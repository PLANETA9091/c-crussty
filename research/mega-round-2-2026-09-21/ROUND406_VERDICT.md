# ROUND-406 VERDICT (тик 02:43→03:0x +08, cron 402447) — interim

База: master ca5e1a4; comp = cmp405_stagtick @ fef3746 (топ-комбо эры, медиана +23.8pp ×5).

## Якоря-406 (vanilla @ ca5e1a4, lever="")
- anchora run 35640571735: 2.35 TPS @ runner 6992836 (band OK), items 30.46% ваниль ✓, GC 21.2s/Full 9
- anchorb run 35640599131: 2.10 TPS @ runner 6851475 (band OK), items 30.9% ваниль ✓, GC 21.3s/Full 9
- Pair-пул волны-406: {2.35@6992836, 2.10@6851475, 2.70@8841704, 2.10@6845027}

## Волна-1 (запущена тиком 405, жива, не тронута)
- 405-A nav→Rust (round-405-a-nav @60fe902): имплементация
- 405-B fluid→Rust (round-405-b-fluid @60fe902): имплементация (FluidRustOps bridge)
- 405-C eindex→Rust (round-405-c-eindex): recon запушен 877077d (javap EntityLookup/ChunkEntitySlices)

## Волна-2 (этот тик, закон 6 RUST-FIRST, база comp fef3746)
- 406-D aibatch (R3 sense/AI→Rust, gate cmp406_aibatch STRICT ⊕ все stagtick-сайты): имплементация ГОТОВА (e1b3117: mob-ai_step batch plane, serverAiStep subtree 9.73% wall → golden-phase WINDOW N=4, ОДИН bulk-JNI DOD-проход по SoA-популяции, retarget LivingEntity.aiStep→MobAiOps.serverAiStepGate); dleg1 run 35643612437 = BAND-DISCARD (runner 11.66M вне 6.0-9.5M, fast-fail pre-download) — ре-ролл по BAND-закону ≤2.
- 406-E sscan (R4 despawn/spawn/activation→Rust, gate cmp406_sscan): имплементация в процессе (worktree жив, RESULT.json пишется).

## Вердикты legs: НЕТ (ноги волны-1/2 прилетят в тик 407)
БАР 80% НЕ ВЗЯТ — МЕРЖ НЕТ. NEXT-407: ре-ролл dleg1; абсорб dleg*/eleg* + волна-1 ноги; pair-вердикты vs пул-406; GREEN → композит comp⊕aibatch⊕sscan и волна-1 ⊕ comp.

## Обновление тик-407 (03:08-03:5x +08, pair-вердикты волны)
- **aleg1 (navplane, cmp405_navplane, run 35649236693): GREEN-CANDIDATE 2.3@6507187 ARMED ×7, items 30.27% ваниль ✓, GC 21.8s/9 → pair +9.5pp vs ближайший 2.10@6845027** (nav_ai 14.16→13.28 −0.9пп — срез скромный, вектор продолжает цикл min-of-3). Ветка round-405-a-nav rebased на master.
- **bleg1 (fluidplane, cmp405_fluidplane, run 35648244840): RED 0.4@6975266 ARMED ×7 — fluid-плейн ВЗОРВАН 16.72→71.48% (+54.8пп)** — анти-паттерн моста (переходы на единицу вместо батча / Critical-массив в горячем цикле); root-cause = B-цикл; вектор на паузе до leg2 с исправлением. «Invalid block entity»-исключения = норма харнесса (×10 в ваниль-якоре тоже).
- Pair-пул-406 полный: {2.35@6992836, 2.10@6851475, 2.70@8841704, 2.10@6845027, 2.30@6507187(aleg1)}.
- Волна-1 рестарты (03:3x): A — done leg1 GREEN; B — leg2 после root-cause; C — рестарт с rebase + имплементация entity_index. Волна-2: D — рестарт на ре-ролл dleg2/dleg3; E — рестарт с merge master + имплементация sscan.
- Диск-инцидент тика: 100% на абсорбе aleg1 (worktree-target purge 362M + /tmp 167M → 97%); урок: cargo target УДАЛЯТЬ немедленно после чека (все агенты мандат).

## Обновление тик-408 (04:43→05:5x +08): ГЕЙТ-ФИКС КОМПОЗИТА + ВОЛНА ВЕРДИКТОВ
- **ROOT-CAUSE гейт-бага dleg2/3 найден и закрыт (TASK-408-D самим верхним)**: .java-гейты были расширены, но .class-блобы (ItemEntityManager/MobPushOps/MobAiOps) не пересобраны (дата 18:49 до финала, строки cmp406_aibatch отсутствовали в байткоде) → items-гейт спал. Пересобраны javac --release 21 vs round-396-a/patched-kernel.jar (полный, moonrise 288 классов)+fastutil+adventure+paper-api (round-j2b-jar НЕ годится — без moonrise) → commit dac1140 → **items 31.17→0.00% в dleg4/6 — композит ПОЛНОСТЬЮ активен.**
- **КОМПОЗИТ comp⊕aibatch (cmp406_aibatch @dac1140): dleg4 2.8@7043848 pair +19.1 (vs 2.35@6992836); dleg5 BAND-DISCARD (9.98M); dleg6 2.9@6585981 pair +20.8 (vs 2.4@6604167) — серия медиана ≈+19.9pp ×2 GREEN, items 0.00 ×2, nav_ai→9.6-10.0. НЕ аддитивно к comp (+23.8): aibatch-окно перекрывает stagger на goal-плоскости. Абсолют 2.9 TPS = лучший pair-подтверждённый счёт эры.**
- **NAVPLANE серия: aleg1 +9.5 + aleg2 2.6@7237353 +10.6 (vs 2.35@6992836) → медиана +10.6 ×2, стабильный знак (aleg3 = min-of-3 закрытие).**
- **EINDEX (405-C): cleg1 2.35@6944967 PARITY (гейт-dormant найден самим агентом, фикс 36ea2a3); cleg2 CI-fail; цикл продолжается.**
- **SSCAN (406-E, R4): eleg1 2.55@6905070 pair +15.9 (vs 2.2@6976430) leg1 GREEN, items 34.55% = master-база (comp не влита — норма); серия продолжается.**
- Якоря-407: anchora 2.4@6604167 + anchorb 2.2@6976430 (ваниль ✓).
- Диск-дисциплина: purж tracked >20MB после каждого абсорб-батча (3× за тик).
