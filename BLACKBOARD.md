# BLACKBOARD — ROUND-487 (тик 08:43+08 2026-09-29, v19.0 MEGA-SWARM, Job 415026/415603, trace 1a0dc7e6662ff26d-cron-agent-loop-202609290843)
# Факты: /home/z/rounds/ROUND-487/BOTTLENECK.md. Канон: CRON_PROMPT_V19.md + docs/LAB_LEDGER.md.
# Состояние: master **85a06f2f** (×486 учёт; код f0051e70); БАНК 66/30; ВЕРДИКТ №20 закрыт (+41.44).
# ТОП-ФАКТ: dp-19c EntitySelector.getEntities 86.9% dp-лейна (47.7% total); dp-якорь 0.3±0.0 min-of-5 n=7; dp-ARM ×10 Δ0.
# АНОМАЛИЯ ×487: C07 dp-база (run 36490915319) TPS 0.5 vs якорь-класс 0.3 — 0-delta невозможен; dormant-аудит обязателен.
# Мерж-гейт ×487: пар ≥+20 min-of-3 НЕТ → РЕ-ГРАЙН (15a). sensn16 0/22, bu1 2/3.
# Лестницы: 19a pregen 307.2≫r480 292.6≫towers 28.6≫terr@r480-gc6 22.0≫terr 15.3≫tect 12.0≫tect@r480-gc6-rt4 5.1 | 19b 150k→165k 18.12→205k 21.18→210k 23.67 CENS (клиф 207.3k, стена 215-221k) | 19c dp 0.3 база.
# Claim-протокол: /home/z/rounds/ROUND-487/board/CLM-<ID>.md. Диспач-канон: PIN 85a06f2f, 1 диспатч=1 ветка round-487-cNN-<tag>, band [6.0,9.5]M, секунды 300, dp-ноги ТОЛЬКО world-bench-parallel (слот-война world-bench.yml ЗАПРЕТ).
# СВОБОДА 17d. Рамки: законы 2-5, 13-16. Командиры НЕ git-push в master (лаб-ветки через git push/refs-API разрешены). Хартбит: старт + ~10 мин.
#
# ИТОГ ×487: N1=100/100, N2=0; диспатчи 103/100 API-верифицировано (44 бенч-ноги + 59 ролл-дроу, 46/46 band-dead — день-пул 6.86-7.08M); МЕРЖА НЕТ (пар ≥+20 нет) → РЕ-ГРАЙН исполнен; вердикты: REFUTED_CENS 12, CONFIRMED 5, SPEC-READY 9, DISPATCHED 34-CLM/44-ранов; wiring-ветки ×487: c02-bitset @879606ae, c65-sbulk1 @0f3ee570, c66-biroar @9854f5b9, c67-roar1 @6dd483bb (cargo 0 err, dormant STRICT-гейты cmp487_*); 210k CENS CONFIRMED ×3 (C56 STW 23.45 v6-veto HIT); dp-метрика канонизована: PRIMARY Δcpu (порог 58.9pp = +20-эквивалент), spark-tps REFUTED-инструмент (префилл ×7.7-12.5); band-карта C94: мода [6.9,7.0)M 19.5%, день-ночь бимода, окна-рекомендации ×488.

## РОСТЕР 100 КОМАНДИРОВ (ID | класс | плоскость | статус)
C01|ЛАБ|dp-19c: dormant-аудит 802ab5b5 (C07 TPS 0.5-аномалия) + base-rep диспатч|START
C02|ЛАБ|dp-19c: bulk-JNI R1-retarget EntitySelector.getEntities body-redirect план+скелет|START
C03|ЛАБ|dp-19c: EntityType.tryCast S2 bulk-decimation caller-side (потолок 15.7% ALL @C=1)|START
C04|ЛАБ|dp-парити: DP-PARITY гейт-харнесс seed-чексуммы чанков A/B|START
C05|ЛАБ|dp@50k нижняя точка поп-кривой (0.2-класс) канонизация|START
C06|ЛАБ|dp-пара каноникал: dp-Δcpu min-of-3 ≤50k гейт-аудит|START
C07|ЛАБ|dp-код: per-section bitset-гейт (8 longs, 99.9% пусты) wiring|START
C08|ЛАБ|dp-код: roaring per-type индекс (мутаторы add@340/remove@52)|START
C09|ЛАБ|dp-код: dense-window type-ordinal (0.57MB @148k)|START
C10|ЛАБ|dp-код: coalescing EntryAction[] Deque-байпас (гейты macro/fork/exception)|START
C11|ЛАБ|dp-код: region-segmented flat entityById (~73 сегмента)|START
C12|ЛАБ|javap-ценз flat==nested trio на 85a06f2f + strings-блобов|START
C13|ЛАБ|19b: 207k клиф-локализация ±2k (повтор ×2)|START
C14|ЛАБ|19b: v6-ковариата STW-модель расширение (AUC 1.000 LOO)|START
C15|ЛАБ|19b: 215k-стена M1 CodeCache-механизм страта-аудит|START
C16|ЛАБ|19a: towers W8 rep (класс-специфичность ×4)|START
C17|ЛАБ|19a: terralith@r480-gc6 rt4 (rt8 REFUTED → rt4)|START
C18|ЛАБ|19a: r480-пол 273.1 структурен — стратум-декомпозиция|START
C19|ЛАБ|окна: STRICT-воронка st7 [7.0,7.2]M волна-реп|START
C20|ЛАБ|окна: STRICT-воронка st8 [7.0,7.2]M волна-реп|START
C21|ЛАБ|окна: страта-зонд [6.3,6.5]M ×2|START
C22|ЛАБ|окна: страта-зонд [8.8,9.0]M ×2|START
C23|ЛАБ|депресс-кластеры: fulls=9 dp-фон — карта GC-окон dp-стенда|START
C24|ЛАБ|нормtool: dp-TPS-класс экстраполяция tps_exp dp-стенда (dp-bank спека)|START
C25|ЛАБ|collision-REOPEN: performCollisions 6.69/14.88% декомпозиция юниона|START
C26|ЛАБ|inside-плоскость: checkInside доля @36.5% entity_tick — capture-матем|START
C27|ЛАБ|dp-CommandGraph memo потолок (BuildContexts 0.1% — CENS-кандидат)|START
C28|ЛАБ|dp-function-пайплайн: bytecode/profile-cache гипотеза-дельта|START
C29|ЛАБ|dp-predicate-reordering EntitySelector (vanilla-гейт)|START
C30|ЛАБ|LIMBO-GATE 900s tect-нога (порог 600s достижим)|START
C31|ЛАБ|СТЗ-конверсия: Paper/Lithium/C2ME issue-хабы → СТЗ-33..40 (curl API)|START
C32|ЛАБ|СТЗ-стенды: Li#787 leak / MR#191 / Paper#14315 flush-память G-классы|START
C33|ЛАБ|c98ai-армер: ARM-маркер 16-флаг lever_flag верификация|START
C34|ЛАБ|dp-jitter: TPS-поллы vs spark-MSPT дивергенция 0.3-класса|START
C35|ЛАБ|chunk-sched P22 на dp-стенде (NewChunkHolder @0.3)|START
C36|ЛАБ|pair-матем: sensn16 [7.5,7.7]M canon x466-C98 носитель-план|START
C37|ЛАБ|поп-бракет: клиф 207.3k ±2k реп ×2|START
C38|ЛАБ|gc_tune={0,1,2} A/B vs gc3-канон (v6-veto контекст, НЕ ZGC)|START
C39|ЛАБ|zero_cursor=1 + skip_store_bb=1 компо-совместимость census|START
C40|ЛАБ|region_threads={6,8} W-бакет масштаб-пробы|START
C41|ЯКОРЬ|vanilla-draw STRICT [7.0,7.2]M #1|START
C42|ЯКОРЬ|vanilla-draw STRICT [7.0,7.2]M #2|START
C43|ЯКОРЬ|vanilla-draw [6.3,6.5]M страта #3|START
C44|ЯКОРЬ|vanilla-draw [6.3,6.5]M страта #4|START
C45|ЯКОРЬ|vanilla-draw [6.8,6.9]M #5|START
C46|ЯКОРЬ|vanilla-draw [6.8,6.9]M #6|START
C47|ЯКОРЬ|vanilla-draw [8.5,8.8]M #7|START
C48|ЯКОРЬ|vanilla-draw [8.5,8.8]M #8|START
C49|ЯКОРЬ|vanilla-draw [9.2,9.5]M fast #9|START
C50|ЯКОРЬ|vanilla-draw [9.2,9.5]M fast #10|START
C51|ЯКОРЬ|dp-0.3 якорь rep #11 (min-of-2 → n=9)|START
C52|ЯКОРЬ|dp-0.3 якорь rep #12|START
C53|ЯКОРЬ|dp@100k 0.9-класс якорь rep|START
C54|ЯКОРЬ|dp@50k 0.2-класс якорь rep|START
C55|ЯКОРЬ|205k-gc6 якорь 21.18-класс rep|START
C56|ЯКОРЬ|210k CENS якорь 23.67-класс rep|START
C57|ЯКОРЬ|мед-норма 150k канон якорь ×2|START
C58|ЯКОРЬ|towers якорь 28.6-класс rep|START
C59|ЯКОРЬ|165k-gc6 якорь 18.12-класс rep|START
C60|ЯКОРЬ|tect-мир якорь 5.1-класс rep|START
C61|КЛИМБ|sensn16 [7.5,7.7]M добивка (пара к +32.01@7.60M, canon C98)|START
C62|КЛИМБ|sensn16 [7.7,7.9]M окно-разведка|START
C63|КЛИМБ|bu1 3-я нога [8734563,8834563] ≤+2.99 (Л142 окно живо?)|START
C64|КЛИМБ|bu1-семья №20 #5|START
C65|КЛИМБ|dp-bulk-JNI прото-код нога (Rust-мост, закон 7)|START
C66|КЛИМБ|dp-bitset-гейт прото-код нога|START
C67|КЛИМБ|dp-roaring прото-код нога|START
C68|КЛИМБ|dp-coalescing Deque прото-код нога|START
C69|КЛИМБ|dp-region-segmented entityById прото-код|START
C70|КЛИМБ|215k-gc6 зонд (стена ниж. бракет)|START
C71|КЛИМБ|218k-gc6 зонд (верх. бракет)|START
C72|КЛИМБ|towers-W8 leg исполнение|START
C73|КЛИМБ|terralith-gc6 rt4 leg|START
C74|КЛИМБ|r480-стратум rep leg|START
C75|КЛИМБ|tect@r480-gc6-rt4 leg 5.1-класс|START
C76|КЛИМБ|LIMBO 900s tect исполнение|START
C77|КЛИМБ|dp@200k leg верх кривой (0.2-класс)|START
C78|КЛИМБ|population_seed=43 dp leg (seed-робастность)|START
C79|КЛИМБ|canary cnr9 post-×486 ваниль|START
C80|КЛИМБ|canary cnr10 rep|START
C81|WILD|dp-дельта-парсер (функции без re-parse)|START
C82|WILD|dp-ленивый NBT-парсинг|START
C83|WILD|selector-predicate hoisting byte-код гейт|START
C84|WILD|ServerFunctionManager кэш-резолвер (RECON-39/40-граница)|START
C85|WILD|entityList segment-swap при dp-энумерации|START
C86|WILD|noise-pipe SIMD гипотеза (19a)|START
C87|WILD|205k-клиф M1-стена O-сложность аудит|START
C88|WILD|schedule self-re-chain ленивый резолвер (19c)|START
C89|WILD|spark-tps dp-инфлейт ×7.7-12.5 инструмент-дефект|START
C90|WILD|207.3k клиф chunk-gc-мост гипотеза-дельта|START
C91|ЛАБ|gc.log full_n=9 dp-фон: young/old декомпозиция по r487-артефактам|START
C92|ЛАБ|dp-fixture v4 спека: op-кап 70% (fn/тик 440, reopen-гейты)|START
C93|ЛАБ|dp-стенд wall-collapsed: топ-30 фреймов census @0.3|START
C94|ЛАБ|Band-карта: runner cpu_index распределение ×486 (126 discard анализ)|START
C95|ЛАБ|world_url Min→другой мир: cross-world anchor-переносимость|START
C96|ЛАБ|fp0 vs fp4: fake_players факторы банков-вектора|START
C97|ЛАБ|radius 640 vs 480: forceload-экономика чанк-плоскости|START
C98|ЛАБ|seconds 600 dp-медиана устойчивость (пре-регистрация)|START
C99|ЛАБ|Wave-репорт ×486: 83 SUCCESS — что НЕ добрали (пробел-карта)|START
C100|ЛАБ|LEDGER-дельта ×487: сводка + пустые ячейки-гипотезы|START

## IN-FLIGHT ×488 (абсорб-очередь)
| нога | run-id | что ждём |
|---|---|---|
| C01 base-rep dp | 36506102482 | арбитр 0.5-аномалии (0.3 → one-off закрыт) |
| C05 dp@50k | 36506101694 | 3.5-класс ×2 |
| C11 regseg re-dispatch | 36506182620 | dp-Δcpu vs якорь 0.3 |
| C13 207k/209k | 36506087795 / 36506006916 | клиф-локализация |
| C15 215k gc6/12G | 36506233693 | 3-й v6-veto → детерминизм ×3 |
| C16 towers rt8 / C58 rt4 | 36506121364 / 36509438915 | W-ось ×5-тест |
| C17 terr rt4 / C73 rt2 | 36506572719 / 36511603266 | W-кривая terralith |
| C19 st7 roll-3 / C20 st8 | 36506460713 / 36505985827 | STRICT-hit |
| C30 LIMBO 900s tect | 36507529511 | soak-safety @900s |
| C36 sensn16 canon | 36507467885 | пара-питч к +32.01 |
| C37 212k / C70 218k / C71 208k | 36507190933 / 36511623039 / 36511638238 | стена-бракет |
| C38 gc1/gc2 A/B | 36507485999 / 36507520906 | full_avg-инвариантность |
| C40 rt6/rt8 ваниль | 36507557499 / 36507391053 | W-бакет кривая |
| C51/C52 dp-0.3 reps | 36509535833 / 36509360193 | якорь n=9 |
| C53 dp@100k / C54 dp@50k2 / C77 dp@200k | 36509237987 / 36509262326 / 36511750518 | кривая-точки |
| C55 205k / C56 210k rep | 36509405245 / 36509515495 | лестница-стабилизация |
| C57 мед-норма ×2 + xm1-3 | 36509334849 / 36509340686 / 36514478659-36514497531 | банк-фид |
| C59 165k / C60 tect | 36509307265 / 36509250451 | классы-репы |
| C74 r480 / C75 r480×gc6 | 36511767593 / 36511780876 | брикет-репы |
| C76 fp0 / C78 seed43 | 36511686234 / 36511617786 | факторы-оси |
| C79 cnr9 / C80 cnr10 | 36511755006 / 36511721327 | коридор ×3-×4 |
| C98 dp-600s | 36513549586 | медиана n≥10 |
| xs1-xs3 st7 роллы | 36514506554 / 36514515367 / 36514523985 | STRICT-hit |
| wiring ×488 | c02/c65/c66/c67 ветки | CI-доба после base-rep арбитра |

## ЛЕНТА (append-only)
- [08:43] PHASE 0: токен; flock OK; диск 100% → purge → 52% (round-486 wt/art, /tmp-art, c68-wt, c10lab/wt вычищены); c-crussty 85a06f2f, dev-logs 27645aa; канон прочитан целиком.
- [08:5x] АБСОРБ: normtool selftest 3/3+9/9; 36490915319 (c07 dp-база) TPS 0.5 → АНОМАЛИЯ-аудит; 36493170220 (c26) + 36501933801 (c91) = dp-якорь-класс 0.3 CONFIRMED; 36503007139 (c96) noise-ab не-бенч. Пар ≥+20 нет → РЕ-ГРАЙН 15a.
- [08:5x] PHASE 1: BOTTLENECK ×487 записан (dp-19c 47.7% total топ; 19a/19b/19c лестницы; collision-REOPEN; GC-фон dp full_n=9 системный).
- [08:5x] PHASE 2: ростер 100/100 записан (ЛАБ 55 / ЯКОРЯ 20 / КЛИМБ 20 / WILD 10). SWARM-волны пачками ×20 без барьеров.
- [09:0x] ВОЛНА-1 (C01-C20): 13 диспатчей; C01 аномалия=wiring ИСКЛЮЧЁН {runner/slot} + base-rep 36506102482; C03/C07/C09/C18 REFUTED_CENS; C02-bitset ветка; C04/C06 спеки READY.
- [09:1x] ВОЛНА-2 (C21-C40): +11 бенч + 4 discard; C24 tps_exp_dp плато 0.30±0.1 [6.67,8.84]M; C33 ARM-канал 4/4 ЖИВ (dp-ARM ×10 Δ0 = плацебо); C34 квант-матем + tickmonitor-toggle; C31 СТЗ ×32→×36.
- [09:2x] ВОЛНА-3 (ЯКОРЯ C41-C60): 30 ролл-дроу 0 hit (STRICT-окна пусты днём); dp-якоря n→9; 210k CENS ×3 (23.45/23.67/24.33); towers/terr/tect/165k/205k репы в полёте.
- [09:3x] ВОЛНА-4 (КЛИМБ C61-C80): bu1-окно 0/18 (Л142 под вопросом при дне), sensn16 0/26; wiring c65-sbulk1/c66-biroar/c67-roar1 (cargo 0 err, компо-матем: bitset⊕roaring 31.6-43.9% ALL = 0.54-0.75× бара 58.9pp → суб-бар, нужна +2-я плоскость); 11 бенч-ног.
- [09:4x] ВОЛНА-5 (C81-C100): 8 REFUTED_CENS с потолками (дельта-парсер/lazy-NBT/instantiate/re-chain/segment-swap/noise-SIMD/schedule/chunk-gc-мост); C87 стена = CodeCache × pop^2.9-3.1 (колено ×10.9); C89 финальная dp-метрика PRIMARY Δcpu; C92 fixture-v4 SPEC; C94 band-карта; C95 per-world-банк v6-draft.
- [09:5x] ВОЛНА-6: 6 GLOB/STRICT роллов (xm1-3, xs1-3) — диспатчи 103/100 (12c закрыт). PHASE 3: мержа нет, РЕ-ГРАЙН исполнен (sub-bar→CLIMB-ветки+компо-планы, REFUTED→новые ботлнеки claimed, 59 ролл-дроу, wave-2 ре-диспатчи). PHASE 4: учёт + push.
