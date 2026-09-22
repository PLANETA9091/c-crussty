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
