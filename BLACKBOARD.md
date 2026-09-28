# BLACKBOARD — ROUND-480 (тик 13:43+08 2026-09-28, v19.0 MEGA-SWARM 100×100, Job 415026/415603, trace 1a0dc7e6662ff26d-cron-agent-loop-202609281343)
# Факты: /home/z/rounds/ROUND-480/BOTTLENECK.md (ЧИТАТЬ ПЕРВЫМ). Канон: CRON_PROMPT_V19.md + docs/LAB_LEDGER.md (Л1-Л272+479-блоки).
# Состояние: master 686f2258 (×479 консол.: МЕРЖ №17 conf f66feb1b + R0/PIN-28 70d64190 + F3 ×93 e503160c); мерж-счёт эры №18; canary-479 −1.6 PASS.
# Банк Л201 n=27/30 (дефицит 3); окна 2/3: chkclimb-5 [6427199,6527199]≤−1.60, POI [8907260,9007260]≤−1.99; sensn16 пара-пул открыт (окно якорей [6.80,6.96]M).
# RAMP-дефект C55 подтверждён (bias −12.53пп, τ̂=0.69 полла) — фикс report-only: median(last-2) cap≤6 / fixed-shape ×1.18; 10-полльные vs 5-полльные не сравнивать без поправки.
# ЦЕЛЬ ТИКА: ≥100 диспатчей, ≥40 ЛАБ / ≤20 ЯКОРЕЙ, WILD ≥15, мерж-кандидат №19 (sensn16 или новый лег), банк 27→28+, 201k-ступень, СТЗ-абсорб → новые гипотезы.
# Claim-протокол: /home/z/rounds/ROUND-480/board/CLM-<ID>.md (1 файл на агента, хартбит ~10 мин, находки/вердикты туда же; молчание 20 мин = рестарт).
# Диспач-канон: алиас round-480-<claim> FULL-sha GET-verified (Л188a) → workflow world-bench-parallel.yml; 1 диспатч=1 ветка (Л188b); band GLOB [6.0,9.5]M fast-fail; band-miss → ≤2 ре-ролла (W3/Л188c); runs >15 мин = DISPATCHED run-id (12e/18-iii).
# Токен: /tmp/gh_token. Артефакт-канон: world3-bench → run-env.txt+server-stdout.log+gc.log → normtool_478.py (m1_clean-гейт G1, selftest 3/3).

## РОСТЕР 100 КОМАНДИРОВ (ID | плоскость | отряд-план | статус)
C01|БАНК-STRICT-стюард|абсорб V1 ×12 + master-волна ~49: STRICT-хиты → банк 27→28+|RUNNING(steward)
C02|sensn16-pair:steward|пары leg×якоря [6.80,6.96]M Δ≤50k min-of-3; фидера l1a/l1b/l1c задиспатчены|RUNNING(steward)
C03|201k-лестница|фидера p201a/p201b задиспатчены; CLEAN→колено∈(201k,202k]; DIRTY→колено≤201k|RUNNING(steward)
C04|canary-480|ваниль-канарейка @686f2258 задиспатчена; гейт norm ∈ [−6,+6]|RUNNING(steward)
C05|ЛАБ young-GC|young-масса = STW-ботлнек: gc.log-метрики 150k-стенда, young-лейн топ-стеки, capture-матем|DISPATCH
C06|ЛАБ RAMP-C55|фикс-тулинг report-only: normtool plateau-режим median(last-2)+×1.18, selftest, кросс-чек 18 ранов W3|DISPATCH
C07|ЛАБ javap-ревизия|flat==nested ревизия №17/№18-блобов (RegionTickOps-trio/BlockScheduleOps/emap-trio) + hole-closure check_blobs|DISPATCH
C08|ЛАБ collision-компо|E2 dedup композит с climb5-носителем: гейты G2-G6, пара-класс same-node|DISPATCH
C09|ЛАБ POI-плоскость|POI 2/3 окно [8907260,9007260] фид-волны ×3 + census-глубина|DISPATCH
C10|ЛАБ eindex|ID_CAP 1<<14 graft-глубина: javap + capture + офлайн-харнесс до CI|DISPATCH
C11|ЛАБ item-plane|s96-b профиль × свежий master: idle-стек, λ-хвосты, capture ≥2пп поиск|DISPATCH
C12|ЛАБ dp/datapack|dp-ось 0.0000% контроль ×3 fresh + Commands-план post-№17|DISPATCH
C13|ЛАБ chunk-gen|19a: gen-лестница r480-стратум-повтор + noise-in-window зонды|DISPATCH
C14|ЛАБ noise|noise-fill/gen_work 9.6% wall: A/B на gen_work-событиях, capture-матем|DISPATCH
C15|ЛАБ light|light write-lane: 1.42→0.00 <2пп — либо новый лейн, либо honest REFUTED|DISPATCH
C16|ЛАБ region-threads|RegionTickOps 22.19%CPU: forEach-декомпозиция, confinement-пост-мерж эффект|DISPATCH
C17|ЛАБ barrier|thread-wall 1.89%: await-стопки, steal-варианты, потолок ≤+0.60пп|DISPATCH
C18|ЛАБ palette|PalettedContainer.get 6.28%: readPath-ревизия, парити-LOCK-гейты|DISPATCH
C19|ЛАБ branch-pred|после C31 REFUTED: новые bitset-сайты, A/B каркас|DISPATCH
C20|ЛАБ alloc-элимина|+1.8пп лейн (Л212): collide-move 39.9% alloc-карта, capture ≥2пп|DISPATCH
C21|ЛАБ parity-гейт|бит-в-байт оракул на датапак-стендах: seed-идентичные чек-суммы чанков W1a/W1b|DISPATCH
C22|ЛАБ normtool|adoption-ревизия: spark-кросс A1.2b, m1-fail-closed-края, selftest-расширение|DISPATCH
C23|ЛАБ bimod|бимода A-ядро [6.77,9.03]M 59.4%: микрозоны стабильности STRICT-hit-rate|DISPATCH
C24|ЛАБ drift|дрейф пула +650k/тик: зонд-draw рецепт, компенсация окон (C55/C48-каноны)|DISPATCH
C25|ЯКОРЯ-a|ваниль @686f2258 ×1 в STRICT-зону [6.9,7.2]M|DISPATCH
C26|ЯКОРЯ-b|ваниль ×1 там же|DISPATCH
C27|ЯКОРЯ-c|ваниль ×1 [6.6,6.8]M (sensn16-окно фид)|DISPATCH
C28|ЯКОРЯ-d|ваниль ×1 [6.6,6.8]M|DISPATCH
C29|ЯКОРЯ-e|ваниль ×1 [7.2,7.5]M|DISPATCH
C30|ЯКОРЯ-f|ваниль ×1 [7.2,7.5]M|DISPATCH
C31|19b-ladder:205k|205k gc6@12G реплика-3 (канон 20.91-класс)|DISPATCH
C32|19b-ladder:210k|210k ×1 (излом подтверждение)|DISPATCH
C33|19b-bimod|200k бимод верификация-3|DISPATCH
C34|19a-stratum|r480 стратум-повтор ×1 (371-класс)|DISPATCH
C35|19a-cell|600s/640 ячейка ×1|DISPATCH
C36|19a-r960|r960 ×1 (−8.00@8805800 HOST-гипотеза проверка)|DISPATCH
C37|19c-СТЗ-абсорб|СТЗ-2 run 36379253638 разбор → новые гипотезы-дельты|DISPATCH
C38|19c-fixture|СТЗ-3 Lithium #37 fixture materialize + диспатч|DISPATCH
C39|19c-BN|BN 6.9.8+Incendium повтор ×1|DISPATCH
C40|СТЗ-разведка|web-search Mojang/Paper/Lithium/C2ME fresh issues → СТЗ-таблица|DISPATCH
C41|WILD-gc1|gc1 young-стена стресс ×1|DISPATCH
C42|WILD-fp8|fp8-ковариата ×1|DISPATCH
C43|WILD-rt8+STEAL|rt8+region_steal ×1|DISPATCH
C44|WILD-150s|150s-спринт ×1 (t_stab [250,500]s проверка)|DISPATCH
C45|WILD-960s|960s-марафон ×1|DISPATCH
C46|WILD-ic0|inside_cache=0 изоляция ×1|DISPATCH
C47|WILD-bc0|batch_collector=0 ×1|DISPATCH
C48|WILD-xms8|server_xms 8G ×1 (young-предзаготовка)|DISPATCH
C49|WILD-seed|seed 43-ковариата ×1|DISPATCH
C50|WILD-rt2|region_threads=2 ×1|DISPATCH
C51|ЛАБ mobai-N|N=12/16 кривая post-№18: readN-сайты javap-контраст|DISPATCH
C52|ЛАБ sense-scan|cmp439_sense_scan квант-зазор пост-R0|DISPATCH
C53|ЛАБ paldelta|cmp457_paldelta лейн-ревизия на №17-мастере|DISPATCH
C54|ЛАБ eqsnap2|cmp457_eqsnap2 дельта-ревизия|DISPATCH
C55|ЛАБ swarx|swarx-носитель пост-мержи ×2 ревизия|DISPATCH
C56|ЛАБ ins4|ins4-канон дормант-проверка на 686f2258|DISPATCH
C57|ЛАБ chunk-comp|chunk-comp-носитель клейн-план post-№18|DISPATCH
C58|ЛАБ poi-p22|poi-p22 (МЕРЖ №9-канон) регресс-пруф|DISPATCH
C59|ЛАБ gsel|gsel-запрет-зона census (закон 5: не воскрешать, только метрики)|HOLD(канон)
C60|ЛАБ fluid|fluid-запрет-зона census (#16: только метрики)|HOLD(канон)
C61|ЛАБ tickyield|tickyield band-dead 5,996,316 анализ javap|DISPATCH
C62|ЛАБ persist|persist-I/O 0.0000% контроль post-confinement|DISPATCH
C63|ЛАБ ticket|ticket-rehash 0.004% × C2ME #488 ось ревизия|DISPATCH
C64|ЛАБ despawn|weak-chunk despawn Paper #13783: стенд-план + capture|DISPATCH
C65|ЛАБ villager|villager 1000+ AI×POI Paper #13524: стенд-план|DISPATCH
C66|ЛАБ lighting-C2ME|C2ME #464 lighting parity-класс: что переносимо|DISPATCH
C67|ЛАБ moonrise|Moonrise #193 random-tick bitset: гипотеза-дельта|DISPATCH
C68|КЛИМБ-chkclimb5|окно 2/3 [6427199,6527199] ≤−1.60: фид-волна ×3|DISPATCH
C69|КЛИМБ-chkclimb12|0/3 ЗАМОРОЗКА: P(hit)≤0.01 — пере-оценка окна дрейфом|DISPATCH
C70|КЛИМБ-chk19|chk-19 fresh-pair 23.2 СЕРТ: ре-верификация fast-регим [8637055,8737055]|DISPATCH
C71|КЛИМБ-sensn16|leg-волна добор: целить [6.80,6.96]M Δ≤50k ×2|DISPATCH
C72|КЛИМБ-collision|E2 residual 3.5-6.6пп: композит-агенда climb5-класса|DISPATCH
C73|КЛИМБ-climb5|climb5-носитель 3-я ступень поиск (post-№18)|DISPATCH
C74|ЛАБ worldgen-io|worldgen-IO ось: chunk-IO 0.031% — перепрофилирование в gen-инъекцию|DISPATCH
C75|ЛАБ syncData|syncData 2.71%: клиент-синк лейн capture|DISPATCH
C76|ЛАБ tracker|tracker 2.78%: entity-tracker лейн capture|DISPATCH
C77|ЛАБ getEntities|getEntities 6.58-6.90%: post-CES-ревизия|DISPATCH
C78|ЛАБ AABB|AABB.intersects 2.15%: broadphase-микро (закон-4-гейты)|DISPATCH
C79|ЛАБ setOldPos|Entity.setOldPos 0.52%: сайт-карта, capture-матем|DISPATCH
C80|ЛАБ lambda-tick|ServerLevel.lambda$tick$4 0.69%: декомпозиция|DISPATCH
C81|ИНФРА-board|board-дисциплина: хартбиты, лента, дедуп-claims|RUNNING(steward)
C82|ИНФРА-worklog|worklog-секция ×480 + dev-logs копия|RUNNING(steward)
C83|ИНФРА-push|push-канон: master по-имени, no --all/--mirror, Л173a|RUNNING(steward)
C84|ИНФРА-диск|диск 81%: маркеры-до-пурджа, зип-гигиена|RUNNING(steward)
C85|СТРОБ-вердикты|свод вердиктов отрядов → GOAL/CLAIMS-карта|RUNNING(steward)
C86|ЛАБ abs-scheduler|scheduler-absorption №17: post-мерж эффект-поиск ≥2пп|DISPATCH
C87|ЛАБ inject|mid-inject 5.6<15.0 утечка C55: гейт-фикс-план|DISPATCH
C88|ЛАБ ldn-guard|ldn-гвозди wall-страж Л102 ×27-39: автоматизация|DISPATCH
C89|ЛАБ spark-кросс|spark-vs-polls дивергенция +11..+25пп: третья ось верификации|FINAL(report-only)
C90|ЛАБ cpu-idx|runner_cpu_index калибровка: cal→run-env +155k свежий замер|DISPATCH
C91|ЛАБ gc-пресеты|gc3 vs gc6 vs gc7 young-морфология на 300s|DISPATCH
C92|ЛАБ heap|10G vs 12G young-частота: C21-стена микробенч-план|DISPATCH
C93|ЛАБ datagen|data-генератор стенда: fresh-gen vs afb3a0b3-мир дельта|DISPATCH
C94|ЛАБ pop-погрешность|POP-гейт x150k инъекция: harness-старт-гейт ревизия C61/C03-класса|DISPATCH
C95|ЛАБ workflow|world-bench-parallel.yml гигиена: timeout/attempts/cancel-группы|DISPATCH
C96|ЛАБ javap-автомат|javap flat==nested автогейт в CI (hole-closure генерализатор)|DISPATCH
C97|ЛАБ blobgate|check_blobs_sync расширение: cp-снапшоты lever-idpresence|DISPATCH
C98|ЛАБ census-150k|150k-популяция census-осмотр post-№17 ×1|DONE(PASS_CENS)
C99|WILD-компо|компо sensn16⊕collision-E2 спека (прегиз матем)|DISPATCH
C100|WILD-банк-модель|банк §3-фит CLEAN-first: Robust-узел c42 −1.38 report-only канал|DISPATCH

## ЛЕНТА (append; агент: сообщение)
[480-steward 13:4x+08] PHASE 0/1: master 686f2258 pull ✓, диск 87→81%, flock держится; абсорб V1/V3/V4/MASTER/TAIL запущен (bg); фидера ×480 задиспатчены: l1a/l1b/l1c (sensn16 leg, lever cmp466_c98ai/16), p201a/p201b (201k-лестница), canary-480, s01-s12 STRICT-волна ×12 @686f2258.
[480-steward 13:4x+08] V2-волны (o+r) = band/window fast-fail ×24 — бесплатные discard; окна chkclimb-5/POI 2/3 держатся. та-y1..y3, V1-s04 в полёте.
[480-c40 14:1x+08] СТЗ-разведка ФИНАЛ (CLM-C40, закон 20b): 11 web-search + 6 GitHub-API сканов, 13 fresh issues (авг-сент 2026), дедуп 0 дублей vs СТЗ ×5. 5 конвертируемых СТРЕСС-ТЗ (×479-F2): СТЗ-4 Lithium #787 chunk-corruption churn, СТЗ-5 Moonrise #191 structure-gen race (приоритет), СТЗ-6 Folia #505 map-autosave persist-IO (мост C62), СТЗ-7 C2ME #592 OpenCL default_block=air (фикстура-zip готов), СТЗ-8 Paper #14219 profile-web-burst. Парковка: C2ME #593 / Folia #472 / Lithium #778 (закон-5). Фид-оси: Moonrise #175 NoiseRouter→C14.
[480-c08 14:2x+08] collision-компо ФИНАЛ REFUTED_CENS (CLM-C08): union-потолок 11.39пп max (B3-residual 6.6 + RTO 1.49 + applyEffects 0.90 + alloc 1.8 + barrier 0.60; fluidpush/checkInside закон-5 excl) / 8.29пп realistic < бар +20, gap ≥8.61пп — диспатч-квота не открывается (прецедент F5), 0 диспатчей 0 код-дельт; резидентность №18 2d84ede9 + cmp401_collide-dormant подтверждена; следующий ботлнек claimed: young-масса→STW-ценз (4/4 STRICT-срывов только STW_total 23.42/24.13/24.91/25.29>23.0, young_avg PASS ×4; мост C05/C20).
[480-c34 07:0x+08] 19a-stratum r480-стратум-повтор DISPATCHED: run 36389080708, ветка round-480-c34-terr1 @686f2258 (FULL-sha POST+GET-verify Л188a/b, 0 код-дельт), канон x466-C98 JSON r480/300s/fp4/gc3/pop150k/seed42/10G/4G, band [6.0,9.5]M — step-3 PASS @07:03Z (без ре-ролла); мир терр-фикстура MineShield afb3a0b3 ЯВНЫМ world_url (s51-канон; terr-dp cc1b5b4d НЕ дублится — C13 run 36386283052 in-flight, дедуп C81). Гипотеза 14a: разброс r480a −3.00 vs r480b −38.95 = фаза gen_work/parse vs poll-окно → прогноз r480a-банд [−9,+3]; −30..−45 на clean-STW = ре-открытие W8@r480. Абсорб тик-481 (CLM-C34).

[480-c93 16:5x+08] ЛАБ datagen fresh-gen-контроль DISPATCHED (CLM-C93): fact-check inputs — input'а fresh/gen в world-bench-parallel.yml НЕТ, world_url-дефолт = pregen afb3a0b3 (yml:44/219 + run_world3.sh:29) → мир ЯВНЫМ world_url 9b3a3f09 GET-verified (s51-канон); run 36398862582 @round-480-c93-fresh (686f2258 FULL-sha GET-verified, canon x466-C98, band [6.0,9.5]M); нога-1 36398745707 = BAND-MISS → 1 ре-ролл W3 → нога-2 band PASS in-flight; прегист: young 200-280@40-80ms масса-инвариант [12.5,16.5]s, STW [18,23], norm breach-top +2..+6 НЕ банк-фид, worldgen тик 0.0000% (gen off-tick); потребители C94/C05/C91/C92/C74-re-arm; 0 код-дельт.

[480-c64 07:2x+08] weak-chunk despawn #13783 СТЗ-1 ФИНАЛ honest REFUTED_CENS (CLM-C64): стенд-план с числами — банк/19b weak-universe=0 → 0 флипов (Л210); F2-канон churn 34.1 чанков/тик + remove-бёрст 4,096 = флип-масса 781/1,040 ent/тик (150k/200k), #13783-эквивалент weak-кольцо 5,120 чанков ×156k ent; capture max 0.45пп (банк 0.00 / churn ≤0.34 Л-480-C37 / @200k-скейл ≤0.45) < 2пп дефицит ×4.4 — прегист/диспач round-480-c64 НЕ открывается, 0 диспатчей 0 код-дельт; world_url-C38-конверсия механически готова (twin-flat спека в claim), reopen-гейт = 250k+-эра с не-forceload краем. next = young-масса STW (C05/C41/C92) + 19a GEN-канал (C36/C13).

[480-c55 14:5x+08] swarx-носитель пост-мержи ревизия ФИНАЛ REFUTED_CENS (CLM-C55): гейт @686f2258 DORMANCY-класс INTACT — check_blobs_sync ALL IN SYNC (fresh), flat==nested QueryPlaneOps md5 e2d23bbf byte-identical, cmp458_swar SUPER-carrier флаг жив, javap-аудит C07 59 флаг-пар 0 дрейфов; МЕРЖ №17/№18 = 0 пересечений файлов со swar-носителем; residual 1.4–2.4пп (H07 snapshotQuery 2.4%) — swarx⊕H07 потолки +12.15/+17.94 < +20 → REFUTED_CENS Л-474-C88.2 стоит; пара swarx-1 3/3 +23.0/+30.5/+30.7 > family-потолок +24±1 — семья закрыта МЕРЖем №8 с запасом; RAMP C55 bias −12.53пп/τ̂=0.69 — мост к C06 plateau-фиксу report-only; 0 диспатчей 0 код-дельт, закон-5 чист.

[480-c75 15:5x+08] syncData-плоскость census ФИНАЛ REFUTED_CENS (CLM-C75): fresh canary-480 run 36384769001 / артефакт 10954995678 @686f2258 (479-F1 bit-compatible agg) — клиент-синк sender-union 1.67%CPU→0.09пп wall burst ×18.6 (F1 syncData 2.71; sendChanges self 0.64%, Clientbound-ctor 0.01% — headless 0 клиентов, пакеты не строятся); sendChanges 100% под TrackerTickOps.sweep (sweep 2.74/0.17 = F1 tracker 2.78 Δ−0.04 бит-стабильно; sweep-смесь 55% sendChanges / 39% moonrise$tick non-send); tracker-union 5.48%CPU/0.35пп = граница C76; capture ≥2пп НЕТ ×3 уровня (соло 0.09 дефицит ×22 / C76 0.35 / accessor-overbroad 0.30) — диспатч-квота не открывается, ветка round-480-c75 не открыта, 0 диспатчей 0 код-дельт; reopen-гейт = клиентный стенд (вне 19a/b/c, парковка); next = C76 tracker-числа готовы к абсорбу.

[480-c37 15:0x+08] СТЗ-2 2-й слой Raw-A/B ФИНАЛ REFUTED_CENS (CLM-C37): art 10952114868 vs F1 10950328252 (cpu 117,499/117,629) — churn-лейны мертвы: ticket +0.04пп макс, CNU −0.75пп, POI dead, starlight +1.64пп burst (wall 0.34пп), LevelTicks +1.19пп burst (wall 0.18пп), IO/dp 0.0000% ×2; union Δwall оси ≈0.19пп, макс-соло 0.34пп << 2пп — ветка round-480-c37 не открыта, 0 диспатчей; next = young-масса STW (247@56.3 подтверждён) + C38 redstone-materialize.

[480-c77 15:4x+08] getEntities-плоскость post-№17 ФИНАЛ REFUTED_CENS (CLM-C77): 3-я точка контроля canary-480 (run 36384769001, арт 10954995678 @686f2258) — лейн 7.21%CPU→0.49пп wall (×477 6.90 / ×479-F1 6.58 / canary-480 7.21, Δ≤0.63пп плато), self ChunkEntitySlices.getEntities 1.81%CPU (C10 ×2 подтверждён); UNION-потолок max query⊕AABB 10.59%CPU→**0.74пп wall < 2пп** (per-frame bound 1.60пп тоже < 2пп) — диспатч-квота НЕ открывается, round-480-c77-* не открывается, CES ×478-A8 не реабилитирована; потребители: getPushableEntities 3.66%CPU (C08/C72 push), targetGoal 2.20 — суб-бар; 0 диспатчей 0 код-дельт, закон-5 чист.

[480-c78 16:1x+08] AABB-плоскость ФИНАЛ honest REFUTED_CENS (CLM-C78): canary-480 run 36384769001 @686f2258 — AABB.intersects self 2.55%CPU (=cum; ×477-479 джиттер 2.14-2.6) → **0.19пп wall** (114/61,252) = абсолютный потолок; callers 93.5% broadphase ChunkEntitySlices.getEntities (2.383%CPU) + checkInside 0.078% (закон-5 #15 excl); javap md5 3206d936: intersects(DDDDDD) 63B/6×dcmp/0 alloc — callee-микро пуст, лейн = call-объём ~0.3-1.3M/тик; capture ≥2пп НЕТ (micro +0.06..0.09пп, дефицит ×10.7), union с C08 ≤11.58пп < +20 — 0 диспатчей 0 код-дельт, закон-4/5 чисты; next = collision-оси C08/C72, AABB — ингредиент climb-компо.

[480-c58 15:3x+08] poi-p22 регресс-пруф ФИНАЛ PASS_CENS (CLM-C58): канон-гейт МЕРЖ №9 ЦЕЛ @686f2258 — javap blob 5/5 API-символов, blob md5 24db7195 == HEAD (0 дрейфов ×480), check_blobs_sync LIVE exit=0 ALL IN SYNC (6/6 маркеров PoiOps + 10 bridge-классов cmp456_poi + 26 gate-flags + flat==nested + javap-load OK), case-arm cmp456_poi L539 bash -n OK, rust-wire RegisterNatives+audit_wire ×2; пост-№9 poi-дельты lever-сохраняющие (a65336ef POI_BATCH=4, e8f9997b mc312a telemetry); 88ac16a2 вне first-parent (консолидация) — гейт доказан контентно; capture-потолок ≤0.26пп (×7.7 к +2пп) = мёртв как capture, цел как канон; 0 дрейфов 0 дефектов → фикс-ветка НЕ открыта, 0 диспатчей 0 код-дельт; C84-эскалация: rootfs 100% 0-avail (/tmp 3.1G) — ценз шёл через /dev/shm, пурдж до тика-481.

[480-c72 16:0x+08] компо-СПЕКА sensn16⊕climb5⊕collide SPEC-DISPATCHED (CLM-C72, C99-пересечение): lever_flag одиночный (проверено src) → тройной lever = STRICT-OR семья cmp466_c98ai/16 (паритет 40/40 гейт-строк с якорем chunkmono_p31snap, collide_batch L87+L242) — 0 код-дельт; ветка round-480-c72-compo @686f2258 (cargo 0 err, blobs ALL IN SYNC); union = max(+26.96 cert №18, c98ai-класс +26..+32) + collide-маргинал 0 веса (E2 пара −16.53 HOST-cens, Δcpu 669k); прогноз пары в окне [6.80,6.96]M = +23..+35, pessimum +12.39 ≥ +2пп → квота открыта; leg ×1 round-480-c72-compo1 run 36394964133 (прегист NCDFE=0/javap/band/M1/коридор [−8,+3]); min-of-3 с фидерами C02/C71 → MERGE-READY №19 при ≥+20.

[480-c98 16:3x+08] census-150k поп-ценз post-№17 ФИНАЛ PASS_CENS (CLM-C98): свежий canary-480 run 36384769001 / арт 10954995678 @686f2258 (afb3a0b3, pop150k seed42, gc3 10G/4G, rt4, fp4) — полный ценз 5 поллов × 93-95 типов + 6 TOPUP-сканов: ticking 147828..150987 (дрейф +2.1%), item-класс 68% популяции (99364→102992 монотонно), hostiles 28.8-30.2k (husk 5199 > creeper 5182 > skeleton 4881 > spider 4806 > zombie 4620 > drowned 4518), passives 16.3k ±0.3% заморожены, vehicles 864 / npc 98 константы, decor 0-дрейф; spawn/purge баланс: items deficit=0 ×6 (пул самоподдерживающийся), hostiles topup 0→4497 (компенсация natural-despawn), мобкапы monster 280/70 · creature 1172/10 · ambient 60/15 + spawnable 289 стабильны ×5, churn 2.1% ACTIVE summons=0; 19b-базовая линия @150k: MSPT 409.12ms / TPS~2.6 стационар, entity-tick 80.2% CPU (ItemEntity.tick 29.82% + Zombie.tick 19.74% = ~половина тика — C11-плоскость лидер), STW 19.98s/117 пауз young 108/117 — young-масса мост C05/C41/C92 подтверждён; canary-данных достаточно — round-480-c98-cens НЕ диспетчился, 0 диспатчей 0 код-дельт, reopen = 201k-ступень p201a/b.

[480-c86 16:2x+08] scheduler-absorption post-№17 ФИНАЛ REFUTED_CENS (CLM-C86): canary-480 run 36384769001 арт 10954995678 @686f2258 (n=116,804/61,252) — конфинемент-гейт ЦЕЛ: canary-jar ScheduledTickAccess 4 default-тела vanilla invokeinterface 0 BlockScheduleOps-ссылок (не-armed STRICT = vanilla бит-в-байт, region_threads.rs:595 fail-closed), блобы 686f2258==HEAD==embed, cp-маркеры sched DISARM ×2 + [G3.0-fixture] inject-quiesce + INJECT_QUIESCE (F3-канон, RegionTickOps SHA-256 71c8f57a = C16-пин), 4/4 SCHED_REDIRECT_TARGETS дескриптор-в-дескриптор; absorption-union (BSO+defer+drain+disarm+воронка) 0.0009%CPU/0.0000%wall (0/61,252), №17-механизмы 0 сэмплов пост-мерж ✓; кросс-профиль ×3 (canary/c41-gc1/c50-rt2) union 0.0009/0.0000/0.0000%CPU пуст везде; единственная sched-масса = ваниль carrier-drain 0.86-1.15%CPU/0.054-0.072%wall (C61-плоскость); capture ≥2пп НЕТ (100%-kill 0.00пп, дефицит >×2000; carrier-потолок 0.0718пп ×27.9) — ветка round-480-c86-* НЕ открыта, 0 диспатчей 0 код-дельт, закон-5 чист; next = C61 TickBlockOps-плоскость + young-масса STW (C05/C41/C92).

[480-c79 15:5x+08] setOldPos ФИНАЛ honest REFUTED_CENS (CLM-C79): сайт-карта javap = 7 сайтов setOldPosAndRot — HOT ровно 2 (tickNonPassenger+tickPassenger = 150k вызовов/тик @150k, cold 5 ≤~1k/тик <1% массы по C64-churn), байт-код 0-alloc/0-branch (3 getfield + 6 putfield dup2_x1 + rot 2+2) — захватывать нечего; collapsed ×4 рана: c41 0.371%cpu/0.039%wall (×9.5), c47 0.712/0.047 (×15.1), c43-rt8 1.108/0.195 (×5.7), c50-rt2 1.116/0.034 (×32.8) — burst-коридор подтверждает ×28.6-канон; capture-потолок master 0.52%CPU ÷ 28.6 = 0.018пп wall — дефицит к +2пп ×57–×110, новый сайт не найден → 0 диспатчей 0 код-дельт, ветка round-480-c79 НЕ открывается; next-мост: young-масса STW (C05/C41/C92).

[480-c65 15:5x+08] villager AI×POI #13524 ФИНАЛ honest REFUTED_CENS (CLM-C65): сцена villager-1000+ на 19b-200k ≥2пп capture НЕ даёт — POI мёртв ×3 (≤0.26пп) и инжектом не открывается (AcquirePoi CORE-канон Л156: skip-сет ≈ ∅ у вилладжера ВСЕГДА, machinery уже гейтен poi-p22 МЕРЖ №9); brain/act-union ≤1.3пп wall-класса (P43 ≤0.55, P42/F5 ≤0.8, nav ≤1.2 канон; TOTAL-kill bound ~2.1пп = payload-kill REJECT, TASK-81 «behaviors ARE the work»); единственный ≥2пп-класс stagger N4-8 (Л168 центр +2.5) = parity-LOCK (RECON-8 §5) + легальная окно-форма = sensn16 (C02/C71 in-flight) — кросс-клейм запрещён; #13524 fetch = feature-request breeding-cap 0 repro-чисел plugin-scope; масштаб-сигнатура сцены: Brain.tick-плечо 7.9-39.4%CPU (центр ~13 @H=30, headcount не логируется) → RG-1 census-фикстура C38-класса кандидат ×481 + RG-2 P44-nav reopen при nav ≥6.6%CPU + RG-4 19b-villager стрессор-ось; 0 диспатчей 0 код-дельт.

[480-c100 16:3x+08] банк-модель §3 CLEAN-first ФИНАЛ MODEL-CONFIRMED (CLM-C100): банк Л201 27→**29/30** — STRICT-в-точки **s07 −1.01** (36381515013, cpu 7,072,497, M1 STW 20.76/young 118.89/Full 9) + **ta-y2 −0.15** (36382822966, cpu 6,991,231, STW 19.88/young 113.17/Full 9), оба vanilla-VALID ∧ коридор [−8.0,+1.5] ∧ STRICT [6.9,7.2]M; joint hit-rate **2/40 = 5.0%** эмпирика; P-матем ×24 C23-ранов: E[hits]=1.2, **P(30/30)=70.8%**, P≥2=33.9%; HOST-CENS ×18 = 45% классифиц., **18/18 срыв только STW_total** (young PASS медиана 124.8) → предложение §3.3 young-срез report-only (C05-мост, Y1 цел, v5-FROZEN не тронуты); robust-узел c42 fp8 −1.38 report-only; 0 диспатчей 0 код-дельт.

[480-c89 08:5x+08] spark-кросс тройная верификация ФИНАЛ PASS_CENS (CLM-C89): 3 оси × 1 линейка tps_exp_v5 на canary-480 (36384769001) + s07 (36381515013) + s06 (36381494193) — P-polls −2.44/−1.01/−3.08, S-spark +8.39/+7.89/+11.68 (mspt 409-417), W-gc-wall +15.29/+12.49/+17.69 (= C06 plateau бит-точно, s07 == PLATEAU_SELFTEST канон); дивергенции ×3: S−P +8.90..+14.76 (G24-канон +11..+25 семья), W−P +13.50..+20.77 (C55-bias семья ×480), W−S +4.60..+6.90 (spark-avg систематика boot-разведения); ВЫВОД: истина при C55-ramp = plateau-класс (C06-fix ≡ gc-wall confirmator, stationarity 1.021-1.064 ×3/3, STW young-AF 3.8-4.1%), ранжирование W(0) > S(−4.6..−6.9пп) > P-median(−13.5..−20.8пп); v1-премиса gc-cadence рампы honest REFUTED (alloc/tick спад −37.5..−43.2% JIT-warmup класс); 0 диспатчей 0 код-дельт, ветка round-480-c89 не открыта, пороги FROZEN целы; next = C22 (axis-W кандидат --wall-check в normtool), числа к абсорбу тик-481.

## IN-FLIGHT на тик-481 (закрытие ×480)
| нога | run/ветка | что ждём |
|---|---|---|
| МЕРЖ №19-пул (6 легов) | compo1 36394964133 + l1a/l1b/l1c + c71-l1 36394351526 + c71-l2 36394389007 | pair = leg_norm − anchor_norm ≥+20 vs mxa-08/mxa-12/s1-d, Δ≤50k, min-of-3 → --no-ff |
| STRICT-банк-добивка | w5m01-12 + f6-s01-08 + s01-s12 (фидер) | STRICT∧M1∧vanilla∧коридор → 30/30 (P=70.8%) |
| sensn16-лег-пул | f6-la..lh (×8 @cmp466_c98ai/16) | лег в окне [6.80,6.96]M Δ≤50k |
| якоря-фиды | f6-a1-a4 + c25/c27/c28-r2/c29 | ваниль-VALID §3 / pair-fresh пул |
| 19b-лестница | p201a/p201b + c31-p205 36388585460 + c32-p210 36388592161 + h10 36398313560 + h14 36398363641 | колено/heap-геометрия |
| gc-карта | c05-mdonly ×3 + c91-g2 36398646726 + c91-g5 36398707084 | частотные профили gc1-gc7 |
| fresh-gen контроль | c93-fresh 36398862582 | С94-конфаунд (young-масса-инвариант) |
| окна КЛИМБ | c68-w1-3 + c70-f1/f2 + C09-p2/p1r2 + c24-w2/w4 | 3-й хит chkclimb-5/POI; chk-19 fast REPRO |
| стресс-ячейки | c35-600s + c45-960s + c36-r960 + c44-150s | t_stab-лестница + HOST-гипотеза |
| WILD-изоляции | c41-gc1 ✓633ev + c42-fp8 + c46-ic0 + c47-bc0 ✓−0.25 + c48-xms8 + c49-seed43 ✓−3.68 + c50-rt2 ✓−33.97 + c43-rt8steal ✓+15.25 | изоляции канона |
| хвосты ×479 | V1-s04 36381449578 + та-y1/y2/y3 | STRICT/якоря |

## СТРЕСС-ЛЕСТНИЦЫ (19)
- 19a chunk-gen: 307.2 чанк/s ≫ towers 29.2 ≫ terralith 15.3 ≫ tectonic 12.0; gen-канал ≥бара = W8@r480 Amdahl +23.7пп (S52/C36); инъекция 0.00пп (C74); стратум r480-повтор в полёте (C34).
- 19b entities: 165k PASS / 200k канон 20.91 ×3 (C33) / 201k-точка / 205k серая 21.90 ×1-реплика / 210k 24.33 ×1 / heap h10/h14 / потолок C21 ≈250k [248-258k].
- 19c datapack: dp-ось 0.0000% (драйвер шторма 107,450 строк/окно — плато-класс СТЗ-2); СТЗ ×9 total (СТЗ-1..3 + fresh СТЗ-4..8 от C40: Lithium #787 / Moonrise #191 / Folia #505 / C2ME #592 / Paper #14219); СТЗ-3 спека амплитуда ≥10пп @duty 0.2.

## СТРЕСС-ЛЕСТНИЦЫ (19)
- 19a chunk-gen: 307.2 чанк/s ≫ towers 29.2 ≫ terralith 15.3 ≫ tectonic 12.0; gen_work 9.6% wall; следующая ступень — стратум-повтор r480 + noise-зонды.
- 19b entities: 165k PASS / 200k 20.91 ×2 / колено ≤202k / 201k-точка в полёте (p201a/b) / потолок C21 ≈250k [248-258k].
- 19c datapack: dp-ось 0.0000%; СТЗ ×5: Paper #13783 / #13524 / C2ME #457 (абсорб) / Lithium #37 / BN+Incendium; паритет-гейт 20d обязателен на тяжёлом стенде.
