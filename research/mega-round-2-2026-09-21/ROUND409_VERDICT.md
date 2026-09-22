# ROUND409_VERDICT — TASK-409 (тик 07:08→08:3x +08, cron 403695)

## Диспатчи верхнего
- Якоря-408 ×3 @010a07e (lever=""): anchora 35666723979 → 2.25@6833921 PARITY ✓; anchorb 35666743738 BAND-DISCARD; anchorc 35666762102 → 2.30@6846036 PARITY ✓. Пул-408: {2.25@6833921, 2.30@6846036} + банк-407 {2.4@6604167, 2.2@6976430}.
- МУЛЬТИКОМПОЗИТ cmp409_multi @f7d04d2 (round-408-f-multi = dac1140 ⊕ e-sscan 4e5b915): merge 15 конфликтов union-разрешены (rust 10 гейт-сайтов + java 6: MobPushOps/MobAiOps/MobScanOps/ItemEntityManager), дубли методов удалены, классы пересобраны (javac cp = полный kernel round409eleg2+fastutil+paper-api+adventure), cargo check PASS, cargo test 228/228. Ноги: multi1 35670197810, multi2 35670218104 (lever=cmp409_multi, f7d04d2).

## Вердикты агентов (R-векторы закона 6)
- NAVPLANE (409-A, cmp405_navplane): aleg3 35667055638 → 2.00@6021509 = −3.2pp RED/outlier (раннер 6.02M in-band, но ниже калибровки банка 6.65M; пары-якоря рядом нет). Серия min-of-3: +9.5/+10.6/−3.2 → медиана +9.5pp, 2/3 GREEN. k4-рисёрч (RESEARCH-A-k4, 6593b85): RNG-PARK подтверждён (Pufferfish DAB), deferred-wantedPos = парити-ловушка (same-point per-entity JNI запрещён), k5-кандидат = A* node-pool (Lithium #743 прецедент, потолок ~1.2% wall). Хэндофф findTarget/getEntities 5.95% → eindex.
- EINDEX (409-C, cmp405_eindex): cleg2b 35667449721 2.30 = PLACEBO пойман ARM-пруфом (from=addEntity хардкод на removeEntity-строках → патч отброшен → гейт спал). ROOT-CAUSE + фикс d05c930 (per-row from_add/from_rem, 4 сайта verified vs BOOTED kernel: addEntity(ZZ)@291→add, removeEntity(Entity)V@119→rem, moveEntity@145→rem, moveEntity@177→add). cleg3 35667240319 (leg-2 серии, дормант-эра — невалиден как armed), cleg4 k2 71cce9f (a_bb hull + empty-batch early-return), cleg4-reroll 35668283159, cleg5 35669081899 failure (причина в RESEARCH-C-k2/RESULT).
- SSCAN (409-E, cmp406_sscan): eleg1/eleg2/eleg3 ПЕРЕКЛАССИФИЦИРОВАНЫ INVALID TREATMENT (register_natives sig '(III[D[I)I' != java '(II[D[I)I' → NoSuchMethodError, мост спал; прежний +15.9pp = comp-база под sscan-флагом). Фикс b405fe1 → ре-роллы eleg2b 35669097430 / eleg3b 35669112587 @b405fe1 — вердикты в RESULT.json агента (write-through).

## БАР 80%: НЕ ВЗЯТ — МЕРЖ НЕТ (next-410 в CLAIMS)

## ФИНАЛ МУЛЬТИКОМПОЗИТА (абсорб верхнего, 08:2x)
- **multi1 35670197810: 2.9@6642199 T1/T2 PASS, ARM ×16 (soa+grid+shardgrid+stagger+collide+heap+ai-window EFFECT tick38+sscan-despawn EFFECT tick20, bulk 1/tick)**
- **multi2 35670218104: 2.9@6507416 T1/T2 PASS, ARM ×16 (ai-window tick30 + sscan-despawn tick11)**
- PAIR-by-runner: +20.8 (vs 2.4@6604167 Δ38k) / +20.8 (Δ97k); vs якоря-408 +26..+29. МЕДИАНА ≈+21pp ×2 GREEN.
- GC 19.1-19.4s / Full 9 = норма; parity PASS; pop VALID; NCDFE=0.
- ВЕРДИКТ: comp⊕aibatch⊕sscan ≈ +21pp pair — маргинал sscan поверх comp⊕aibatch ≈ +1пп (перекрытие mob-плоскости, третий случай не-аддитивности эры). 2.9 TPS ×2 = лучший pair-подтверждённый абсолют эры. БАР 80% НЕ ВЗЯТ — МЕРЖ НЕТ.
