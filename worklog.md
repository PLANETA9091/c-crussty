## SESSION wave-2 — TASK-12 done

**Task ID: TASK-12-sub (wave-2, proc-1)** · Agent: agent-7625532f (TASK-12-sub) · 2026-09-08

Work Log:
- Двойная доставка обнаружена ДО push: docs/BATCH_ADOPTION_MATRIX.md (TASK-12-w1, d02fbc2) уже в origin/master. По прецеденту TASK-03/TASK-11 объединение: оба документа сохранены, мой — компаньон docs/BATCH_ADOPTION_MATRIX_wave2.md с перекрёстными ссылками; расхождений в цифрах нет (одна каноничная база).
- База: P500_REPORT.md @ HEAD ≡ P500_REPORT_v2.md байт-в-байт (diff) — один каноничный перегон 2026-09-08 (3baa0f7): 49 групп / 129 кернелов / 70 пар / 0 CRASH. Stem-pairing воспроизведён офлайн (70 пар, 1 unpaired, 9 multi-pair — точное совпадение с агрегатором).
- Floor-инвентарь по ПАРАМ: 13 групп / 17 old↔alt пар / 32 кернела ≤225ns (в полосе 35-90ns строго — 6 кернелов: g35, g39, g40; нижний якорь g42 34.6ns). Все 17 ratio — parity (0.94-1.05) = «сидят на полу».
- Грейды H/M/L по сигнатурам JNI_EXPORTS.manifest: H=4 кернела (g9 (III[J)I, g35 ([D[I[I[II[J)I — плоские примитив-арены), M=26 (объектные [Lobj;/String слоты), L=2 (g42 <40ns — амортизировать нечего). Сверка с wave-1 (g42 у них HIGH как калибровочный) — обе линзы описаны.
- Модель: T(K)=m+C/K, C=35-90ns, m=3-9ns → T(64)=3.5/7.0/10.4, T(256)=3.1/6.2/9.4, T(4096)=3.0/6.0/9.0 ns. Добавлена M-поправка: m_M = m + 6ns×(ref-слоты) — воспроизводит errata TASK-10 (~1.3-3x для ref-тяжёлых); H-пары дают 11-19x @K=256.
- Top-10 wire-кандидатов (по риск-скорректированной экономии @K=256): g9 113.6ns (19.2x), g32 91.3 (4.8x), g30×3 84.9-86.0 (3.8x), g18 85.4 (4.5x), g31 84.5-84.8 (3.3x), g35 74.2 (11.3x), g39 69.5 (4.8x). DO-NOT-WIRE: 4 масштабно-инвариантные регрессии (5.70/4.54/2.35/1.78) + L-исключение g42.
- ms/tick: параметрическая модель f×(T1−T(K)); при f=1k → 0.065-0.094 ms/tick, f=10k → 0.65-0.94, f=100k → 6.5-9.4. Сверилась с wave-1 S1/S2/E — согласование в пределах ~±50%. Честный вывод: на референсных нагрузках ≤0.1% тика; ≥1 ms/tick требует ≥12k-40k вызовов/тик (JFR-профиль обязателен до вайринга).
- План вайринга через kernel_policy::decide_id(): пререквизит TASK-24 (8 allocs/call в batch_api.rs ≈ пол на K=16), гейт на build+serve, ERR_POLICY_DENIED, промоция в PROVEN_WINS только по BatchFloorBench A/B, тесты (a)-(d), порядок шейпов A′→D→refArgs.
- НОВЫЕ инконсистентности: (1) PROVEN_WINS evidence протух против перегона-2026-09-08 — 3 из 7 «P500 WIN» больше не воспроизводятся (PluginLoadingAllocation 1.55x/1.53x → parity 0.993/0.996; AquiferSurfaceSampling 1.15x → 0.917 parity; blend-cache 244x → 316x) — DO_NOT_WIRE синхронизирован в e5c4fad, PROVEN_WINS нет; (2) baseline.tsv самоссылочен (все drifts ±0.0% — гейт TASK-14 осмыслен только со СЛЕДУЮЩего прогона); (3) P500_REPORT.md ≡ v2 (один датасет); (4) UTC vs UTC+8 в датах; (5) batch_table.rs ids 0-11 — все µs-масштаба, ноль floor-покрытия.

Stage Summary:
- Документ: docs/BATCH_ADOPTION_MATRIX_wave2.md (компаньон wave-1 d02fbc2; union сохранён) — коммит docs(TASK-12) в c-crussty, хэш в CLAIMS.
- Топ-кандидат: PaperNativeDensityAp2MinMaxFill oldSummary→newSummary — 19.2x @K=256 (113.6ns/op, grade H), следующий RangeChoice 11.3x (74.2ns).
- Грейды: H 4 / M 26 / L 2 кернела (13 групп / 17 пар ≤225ns).
- Главный инконсистент-файндинг: PROVEN_WINS evidence stale (3 из 7 WIN-вердиктов не воспроизводятся на каноничном перегоне 2026-09-08) — нужен evidence-sync коммит (не делал: analysis only).

---

## SESSION wave-2 — 2026-09-07T17:47Z — TASK-20 done: area-map apply-loop micro-bench

**Task ID: TASK-20**, Agent: agent-7625532f (proc-1)

Work Log:
- AreaMapBench.java (bench/areamap/benchjava/): тайминг-компаньон смока TASK-11 — те же фикстуры/конвенции RNG (xorshift64 0x9E3779B97F4A7C15), но no-log CountAreaMap (без аллокаций) и 3 фазы: CHANGED (move+resize, каждый вызов гарантированно меняет состояние — 1 натив на вызов, верифицировано), SAME (fast path), NAIVE (java diff-скан = нижняя граница). Размеры side 127/511/1023 px, median-of-3 раундов по >=150ms.
- run_apply_bench.sh: собственные classes-bench-fake/real (не трогает смоковые), REAL-режим через realdecl .so-биндинг. FAKE+REAL прогоны: ВСЕ ВЕРИФИКАЦИИ PASS (native=1/changed-call, native=0 на fast path).
- РЕЗУЛЬТАТЫ (results/APPLY_BENCH.md): fast path 24.7-25.3ns/вызов 0 нативных — в 2000x-170000x дешевле apply-цикла (48.6µs-4.25ms REAL); РЕАЛЬНЫЙ натив 6.18-6.44x МЕДЛЕННЕЕ java-скана per-px (2.0 vs 0.30-0.35 ns/px), per-op 435-851ns — 90%+ стоимости внутри .so (JNI-пол 35-90ns), wave-3 candidate: engine-side батчинг; small-grid overhead 2.31x (side 127) сходится к 1.08x (side 1023).
- АНОМАЛИЯ -> TASK-22: REAL ops/call d=63 = 645 vs FAKE 374 на идентичном RNG-стриме (d=255/511 сходятся в 4.6%) — возможны дубли/стейл-ряды скретча в реальном кернеле на быстрых move+resize стримах. Нужен oracle-прогон до любых wave-3 кернел-работ.
- BENCH.lock соблюдён (solo-окно), soak-сервер агента-2 работал фоном (раскрыто в отчёте).

Stage Summary:
- TASK-20 closed: fast-path прайсинг + первый per-px профиль реального apply-кернела + найдена корректностная аномалия (TASK-22). Артефакты: bench/areamap/{benchjava/,run_apply_bench.sh,results/APPLY_BENCH.md,results/apply_bench_raw.tsv}.

---
## SESSION wave-2 — TASK-30 done (ops-count oracle) — agent-7625532f (TASK-30-sub)

Work Log:
- OracleBench.java (bench/areamap/benchjava/...): per-call multiset parity vs naive set difference on deterministic streams — S1 pure 1-chunk moves (d=63, 64 calls), S2 same-center ±1 resizes (64), S3 anomaly mix move+resize (d=63 ×128, d=255 ×8, d=511 ×4); capacity probe (reflection into bridge ThreadLocal Scratch) vs maxOps grow contract; REPLAY of the bench's exact CHANGED RNG stream (geometric O(1) diff, cross-checked vs enumeration).
- RESULT (both modes, JDK21, native/libpaper_native_jni.so): 268/268 parity PASS in FAKE and in REAL, 0 duplicates / 0 stale / 0 missing / 0 cap violations, exit 0. TASK-30 VERDICT: PARITY.
- Root cause of the 645-vs-374 anomaly: NOT stub-drop (bridge grows scratch to maxOps = old²+new² >= diff BEFORE every native call; stub cap never binds), NOT duplicates/stale — the CHANGED radius is a ±1 random walk and the time-bounded windows consumed different stream lengths (FAKE 52108 vs REAL ~10003 calls at d=63); ops/call scales with the LIVE radius, and the walk's mean over the long FAKE window drifted to 37.3 (min 1) vs 64.4 over the short REAL window. Replay reproduces BOTH bench numbers parameter-free: 374.30 over 52108 iters, 644.9 over 9996..10010 iters. d=255/511 agree because walk σ/d there is 1-8%.
- Deliverables: results/TASK30_ORACLE.md (method/evidence/verdict/wave-3 statement), results/task30_oracle_raw.tsv, run_oracle.sh (classes-oracle-* file-disjoint), signed ERRATUM appended to APPLY_BENCH_RESIZE_MIX.md (anomaly refuted, not rewritten). AreaMapBench.java NOT modified (root cause is not stub-drop; bench-hygiene recommendation documented in TASK30_ORACLE.md instead).
- Kernel correctness: closed .so emitted exactly the naive set difference on every tested transition shape — wave-3 gate passed.

---
## SESSION wave-2 — TASK-31 done (PROVEN_WINS evidence-sync to 2026-09-08 rerun) — agent-7625532f (TASK-31-sub)

Work Log:
- Method: every registry entry carrying a P500 verdict/number (src/kernel_policy.rs: 4 DO_NOT_WIRE + 23 PROVEN_WINS = 4 live + 7 "P500 WIN" + 12 batch-surface PARITY) re-checked group-by-group against the canonical rerun bench/p500/results/P500_REPORT.md (generated 2026-09-07T16:46Z = 2026-09-08 UTC+8; 49 groups / 70 pairs / 0 CRASH — the only authoritative ns/op source). Canonical aggregator rubric (WIN <= 0.85 / REG >= 1.18) applied; historical v2 numbers kept only as labelled history.
- RECLASSIFIED 3 stale WINs -> PARITY (verdict strings now "P500 PARITY (2026-09-08 rerun)", kept in PROVEN_WINS so the Allow set is UNCHANGED — no gate behavior change): PluginLoadingAllocation.newLazyValidateSummary 1.55x -> 0.993 (116.1->115.3 ns); newLazyMissingSetSummary 1.53x -> 0.996 (115.1->114.6 ns); AquiferSurfaceSampling.newBatchSummary 1.15x -> 0.906 (6.0->5.5 us; TASK-12's 0.917 = rounded-median artifact, report's own pair table says 0.906 — same verdict).
- Number refreshes (verdict unchanged): NoiseChunkBlendCache.newEmptyBlenderSummary 244x -> 316.45x (95.3 us -> 301.1 ns, ratio 0.003); NoiseInterpolatorSlice.flatSummary 3.29x -> 3.32x; switchGradientSummary 1.22x and scratchThreadLocalSummary 1.20x REPRODUCED (absolutes refreshed 493.6->411.9 us). DO_NOT_WIRE (e5c4fad) re-verified against the rerun: 5.70/4.54/2.35/1.78 — zero drift; only its doc comment (old "2026-09-07 120ms-batch" label) harmonized. Batch-surface evidence strings fixed where stale: id3 "1.15x-win stem family" -> 1.07x parity; id7 "newDirect 3.29x stem" -> parity 1.00x (that win belongs to NoiseInterpolatorSlice, different class).
- Wiring-eligibility delta: hot-path WIN promotion set 7 -> 4 (g30 x2 + g2 lose hot-path-swap candidacy, stay batch-dispatch-eligible per BATCH_WIRING_PLAN B.2.3(a) "parity or better" — matches MATRIX_wave2 #4/#5/#7 "wire for batching, not kernel swap"); batch Allow set unchanged (12 ids); no DO_NOT_WIRE change; net gate-widening zero. decide()/decide_id()/ERR_KERNEL_REFUSED semantics identical; cargo test kernel_policy: 13 passed.
- OPEN (documented, not fixed — would need a bench run): 5 batch-surface entries (TicketSetSearch x2, NoiseInterpolatorFractions.divisionSummary, ClimateRTree x2) have NO pair in the canonical rerun (0 mentions) — verdicts rest on pre-rerun evidence, flagged for a future calibration rerun; 2 NEW rerun WINs deliberately NOT added to the registry (would widen the gate): NoiseChunkFlatCacheContext.newTrueContextSummary 1.24x (0.805) / newFalseContextSummary 1.18x (0.846) — recorded as top promotion candidates in docs/PROVEN_WINS_SYNC.md §4.
- Deliverables: src/kernel_policy.rs (registry evidence strings/comments only), docs/KERNEL_POLICY.md (PROVEN_WINS section -> synced tables, 4+11 -> 4+23 entries), docs/PROVEN_WINS_SYNC.md (method, before/after for all 27 entries, eligibility delta, risk note). Files of other agents untouched.

---
## SESSION wave-3 — TASK-33 done (BOOST sweep: >100x ledger, missed bindings, physical limits) — agent-7625532f (TASK-33-sub)

Work Log:
- Trigger: messagesFromUser.md — user demands >100x ("маленькие ускорения <100x надо сделать >100x"). ANALYSIS ONLY (box busy: soak+benches) — no runs, no product code; sole write = docs/BOOST_SWEEP.md + this append. Worktree /home/z/w-t33 detached @ origin/master d87067e.
- Deliverable docs/BOOST_SWEEP.md, 5 sections: (1) shipped >10x ledger — 5 mechanisms ≥10x with commit refs + measurement + USER-VISIBLE/BENCH-ONLY split: area-map same-state fast path 1,945x–170,612x LIVE (~25 ns/0-native vs 48.6 µs–4.25 ms apply, APPLY_BENCH_RESIZE_MIX), blend-cache kernel pair 316.45x BENCH-ONLY (95.3 µs→301.1 ns, 0.0% stability), TASK-01/09 lifecycle quiet reclaim >571x LIVE (12 s timeout→21 ms) + GC churn 24x, TASK-22 find_class sighting-feed >10x–>100x ESTIMATE LIVE (10–60 ms/scan → ~0), batch-dispatch amortization DORMANT ≤40x naive best (≥10x only H-shapes g9/g35) + <10x rerun WIN pairs (3.32x/1.24x/1.22x/1.20x) and cleared hygiene (TASK-23/24/26/27).
- (2) Missed-binding sweep: binding truth = all 283 natives registered (registration ≠ wiring); ONLY live wirings area_map.nativeUpdateOpsBatch + improved_noise.nativeNoise/BuildHandle/FreeHandle; batch dispatcher armed (POLICY_ALLOWED + ERR_KERNEL_REFUSED=-10) with ZERO consumers. Of 70 pairs exactly 2 have alt ≥2x: g21 blend-cache 316.45x (live pays vanilla; H1/H2 open — TASK-32 remap, conditional) and g23 interpolator 3.32x (live pays jagged; hook design exists, no owner). Soft: g22 1.24x/1.18x promotion candidates. PREF mechanism today supports ONLY old-under-alt rescue (registration_fallback ← DO_NOT_WIRE); inversion spelled out in one paragraph (paired_old field on ProvenKernel + 3-state env parse + swap-up in registration_fallback + TASK-13 symbol cross-check extension + output-parity evidence field; unset-env behavior byte-identical). Floor: 17 parity pairs — transition-dominated, batch-remap via decide_id/B.2.3, 0 sites armed.
- (3) Physical limits: floor 35–90 ns ⇒ >100x per-call needs ≤0.9 ns/op ⇒ impossible plugin-side; batch caps 11.5–40x naive / 1.4–4.8x M-adj ⇒ 32 floor kernels / 13 groups BLOCKED-BY-.so; 33 body-dominated groups blocked by bodies. >100x CLASS = same-state/constant-fold guards + lifecycle hygiene + boot caching (all three shipped exemplars skip machinery with O(1) guards). Next >100x: blend-cache guard is the LAST identified candidate on a live surface (HOOK_BLEND_CACHE P1–P4, PATCHER_DESIGN C0/C1, C1 cache rejected on measured evidence); BLEND_CACHE_DESIGN evidence leans H2 (Paper already folds EMPTY) ⇒ TASK-32 honest outcome may be documented NO-GO; after it the >100x pipeline is empty without in-.so engine work (upstream batch API, ENGINE-TOUCH).
- (4) User-facing ledger table for messagesFromUser.md: already-live >100x (①②), TASK-32-conditional >100x (③, with H1/H2 caveat), physics-capped (④⑤⑥ with exact numbers). (5) Doc errata (not edited): PROVEN_WINS_SYNC §4.4 wrong that P500_REPORT.md ≠ P500_REPORT_v2.md (diff -q: IDENTICAL @ d87067e); HOOK_BLEND_CACHE.md stale 244x/~115 ns; BLEND_CACHE_DESIGN quotes "WIN (244x)"; ROADMAP §1 stale area-map poll cadence (60 s/500 ms vs 180 s/2 s/10 s).
- Signed: agent-7625532f (TASK-33-sub), 2026-09-07T18:55Z.

---
## SESSION cron 03:20+08 — TASK-24 (C3) закрыт: bench tail (BatchFloorBench before/after) — 2026-09-07T19:45Z — agent-7625532f

Work Log:
- Открытый хвост TASK-24 (C3): код 28ad646 на master (scratch reuse, prefix-sum pre-size, in_starts reuse), но doc batch_api.rs:90-92 ссылался на bench/batch/ — файл не был поставлен (subagent-3b SESSION 005 погиб без артефактов). Клейм CLAIMS f73ace8.
- Доставлен bench/batch/: BatchFloorBench.java (standalone: System.load closed lib P500-real-mode + module cdylib; abiVersion gate 65548; P500 G0 shape-A ids 2/3, scalar=16, dst=64; parity gate direct-vs-batch 4 lanes PASS; settle ~1M ops; 11 rounds medians; direct per-op reference), run_batch_floor.sh (detach-worktree arm builds, полный BENCH.lock, TSV raw), classes/ в .gitignore.
- Полный прогон 19:31-19:33Z под BENCH.lock, пары BEFORE=db7cf27 vs AFTER=master: k2 K=1 1096.6→997.2 (0.909), k3 K=1 1042.3→951.3 (0.913); K>=8 паритет в шуме (0.989-1.015); direct/op drift <=0.6% между руками (module-независимость).
- Декомпозиция (AFTER): fixed preamble ~200ns/batch, per-op marginal +242-244ns @K=1 → +40-49ns @K=256. Report-only вывод: batch бьёт direct только у потолка 90ns-перехода при больших K (~2.2x асимптота для 0-body); для 35ns-floor и body-dominated ядер — никогда. Registry/kernel_policy НЕ менялись.
- Отчёт: bench/batch/results/BATCH_FLOOR_REPORT.md (+RAW.tsv, per-arm logs). cargo test 22/22 + cplug-sdk 15/15, clippy 12 = baseline. Push 1449f7f (race attempt 1), CLAIMS done (re-pull verified).
- Cleanup: /home/z/w-t24 удалён; live server не тронут; токен не экспонирован.

Stage Summary:
- TASK-24 (C3) полностью закрыт (код 28ad646 + измерение 1449f7f). Константы диспетчера теперь эмпирические — вход для wave-1 batch adoption. Open: TASK-32 (blend-cache impl, вероятный NO-GO по H2), TASK-13/17/20/22/23/25/28-36/w3/w4 done.

---
## SESSION cron 03:40+08 — TASK-32 Phase-1 probe: items 1/3/5 done, items 2/4 in setsid-фоне — 2026-09-07T20:05Z — agent-7625532f

Work Log:
- TASK-32 takeover (клейм мой, субагент-имплементатор мёртв). Item 1 javap LIVE runtime jar: H2 ОПРОВЕРГНУТ — blendOffsetAndFactor vanilla-структура, EMPTY-путь НЕ свёрнут (3x MutableDouble + forEach + new BlendingOutput(1.0,0.0) alloc/call; patches.list 0 хитов; blender set-once/final confirmed; сайты резолвятся).
- Item 3 g21 N-scaling (BENCH.lock): old linear CONFIRMED; new flat N≤16, сублинейный рост далее; канон 313.6x @N=256 ≈ registry 316.45x.
- Item 5 V1 parity: bench/blendprobe/V1BlendParity.java 10000/10000 PASS (16 quart-классов, радиусы ≤60, fresh-dst, ret+lanes).
- Items 2+4: run_blend_probe.sh — throwaway default-world бут (CRUSSTY_NATIVE_BLEND_CACHE=1 observation-only + JFR profile + forceload 2x17x17) в setsid-фоне, harvest следующего тика.
- §9 running addendum в docs/BLEND_CACHE_PATCHER_DESIGN.md; c-crussty cedc9df; CLAIMS takeover-строка; registry НЕ менялся.

Stage Summary:
- 3/5 гейтов PASS; решающий item 4 (JFR share) — harvest далее. Прото PATCH_ENABLED=false; Phase 2 (V2-V4) обязателен до env-gated serve.

---
## SESSION cron 03:40+08 (продолжение) — TASK-32 Phase-1 probe VERDICT: NO-GO — 2026-09-07T20:33Z — agent-7625532f

Work Log:
- Boot-диагностика: 4 тихие смерти (без hs_err) на craftbukkit.Main — **F2: стартовый JFR + JVMTI-агент несовместимы в class-load шторме (4/4; gate=1, gate=0, SerialGC, разные heap)**; hook exonerated gate=0-контролем; workaround: jcmd JFR.start post-boot (бут #6, Done 33.147s, rec.jfr 1.44MB).
- Stray bootab-JVM (PID 12499, run-B-p2, 344MB@126%CPU) найден и убит — НЕ live-сервер.
- Item 4 (JFR, 252 сэмпла, строгий матч): per-column blend-механика 0/180 worldgen-worker сэмплов; единственный хит 0.40% = разовый Blender.of setup (первичный «18.25%» — парсерный false-positive на doFill(Blender,...), честно перепроверено). NO-GO порог <0.1% пробит.
- ВЕРДИКТ TASK-32 NO-GO: Blender.EMPTY путь дёшев/нерелевантен на свежем мире; 316x = DENSE-ветка (upgrade-миры) — сервер не ходит. Прото dormant, BENCH-ONLY, kernel_policy не тронут. >100x pipeline пуст ИЗМЕРЕННО.
- c-crussty: 4da13af, 1c5eefb, 2fbe52c; CLAIMS done; race-safe пуши, re-pull verified.

Stage Summary:
- TASK-24 и TASK-32 — оба хвоста wave-3 закрыты. Открытых клеймов нет.

---
## SESSION cron 04:20+08 — TASK-45 done: scan-avoidance ESTIMATE → measured — 2026-09-08T21:16Z — agent-7625532f

Work Log:
- TASK-45 (клейм моей линии, w6-субагент мёртв): cplug-sdk 53578ba — AtomicU64-счётчики find_class + CRUSSTY_SDK_STATS-дампер (t=25/65/120/180s); arm A' = 4fb9d12 + идентичный hand-patch; bootab +977a339 BOOTAB_POST_HOLD_S.
- Замер boot-window (t=25s): A' 16 сканов/156,690 walked/0 skips vs B 3 скана/16,822/5 skips → **9.3x scan-work↓, 5.3x scans↓, 63% miss scan-free**; премиса 10-30k классов/скан подтверждена. >10x live-claim — ESTIMATE с измеренным полом (flat-boot трафик мал). Boot n=2 перекрывается = directional.
- d176e46: отчёт bench/bootab/results/TASK45_SDK_STATS_AB.md + 4 TSV. Инфра: /tmp вычищается между тиками (scratch не выживает); BOOTAB_ROOT должен содержать rt/; stdout bash-сессии деградировал после kill — вывод через файлы+Read.
- CLAIMS e983b0d done; re-pull verified; worktree /tmp/w-a45 удалён; live server был выключен весь тик (не моя зона, не трогал).

Stage Summary:
- TASK-45 закрыт. Инструмент sdk-stats + POST_HOLD остаётся для будущих A/B. Открытых клеймов нет.

## SESSION cron 05:00+08 — TASK-48 Phase 1: wave-1 shape A' (III[J)I) implemented + measured — 2026-09-08T21:4xZ — agent-7625532f

Work Log:
- Взят открытый blocker runbook §8 G3 (wave-1 shapes, S7-5declared "not attempted"). Клейм TASK-48 в CLAIMS (863d7a6), re-pull verify ок.
- Код (b4a5c9b): Shape::APrime (III[J)I) + scalar_width() в batch_table; kernels id 12/13 = PaperNativeDensityAp2MinMaxFill old/newSummary (jni_table.rs:173-174, P500 g9); TABLE_VERSION 1->2 (args0 = shape-packed scalar plane: A/Z=1, A'=3, B=0 longs/op, prefix-of-widths layout — v1 all-A batches wire-идентичны); batch_api: ShapeAPrimeFn/KernelFn::APrime, layout в том же prefix-проходе что in_starts (0 доп. проходов), args0 length validated post-layout; kernel_policy: PROVEN_WINS id12/13 вердикт "P500 PARITY (batch surface)" — честно: пара old-vs-new = 1.003x паритет (2026-09-08 rerun), "19.2x" wave-2 матрицы был BATCH-проекцией.
- Тесты: crussty 29/29 (+2: ABI pins 131086, scalar-layout worked example), cplug-sdk 20/20, clippy Δ0.
- Бенч (de81bcf runner, detached worktree @b4a5c9b, BENCH.lock 21:1x-21:22:58Z): kernels 2,3,12,13 same-process, K={1,8,16,64,256}, 11-round medians. Parity gates PASS (4 lanes, все 4 kernel). A' id12: K=1 361.3ns/op (3.07x worse than direct 117.7), K=256 156.5ns/op (1.35x worse). Декомпозиция: fixed ~205ns/batch (= TASK-24 ~200ns), dispatch-only overhead +40.5ns/op @K=256 == shape A +43.1ns/op → 3-long packed plane стоит ~0 доп.
- ВЕРДИКТ: batch g9 НИКОГДА не бьёт direct (нужен direct >= ~160ns, g9 = 116ns); wave-2 матрица §6.6 (6.2ns/op, 19.2x) ОПРОВЕРГНУТА измерением. Обобщение по структуре: overhead shape-независим (~+40ns/op) → весь wave-1 список (g42 34.6 / g35 81.4 / g39 87.7 / g40 88.6ns) никогда не окупится → Phase 2 (shape D/refArgs) = NO-GO-by-measurement, G3 закрыт измеренно. Batch остаётся рациональным только для body-dominated (>=700ns) kernel на больших K (паритет 1.06x). Registry/site-arming НЕ менялись (G4 открыто, скоуп теперь честный).
- Cross-validation: k2 K=1 998.8 vs 997.2 TASK-24 (0.2%), k2 direct 756.4 vs 757.1 — машина/JIT согласованы.
- Отчёт bench/batch/results/A2_SHAPE_REPORT.md (6 секций) + A2_SHAPE_RAW.tsv (40 строк) + лог. Worktree /tmp/w-t48 удалён trap-ом; main WIP и live server не тронуты.

Stage Summary:
- G3 (wave-1 shapes) ЗАКРЫТ: A' machinery в master (инфраструктура Stage-1 готова для ЛЮБЫХ будущих shape-расширений), но весь wave-1 candidate list измеренно NO-GO для batch adoption. Открытый скоуп: G4 (site-arming) — теперь только shape-A body-dominated kernel; G5 auto-threshold получил эмпирические константы (fixed ~205ns, marginal ~+40ns/op независимо от shape). Ближайшие кандидаты: G8 (refused-id e2e fixture), либо новые hotspot'ы dump->analyze->optimize.

---
## SESSION cron 05:20+08 — TASK-50 done: TASK-39 D1 closed (shape-B double-copy eliminated + measured) — 2026-09-08T22:1xZ — agent-7625532f

Work Log:
- Старт: worklog+CLAIMS прочитаны (открытых клеймов моей линии нет; второй контур: TASK-47 rollout A/B NO-GO + свежий G3 shape-C spike 2bd43c4). Взят named prerequisite батч-трека: TASK-39 D1 (HOTSPOT_CANDIDATES_V2 P2, dormant). Клейм: сначала TASK-49 → КОЛЛИЗИЯ с чужим клеймом TASK-49 (ledger) — перенумерован в TASK-50, push+verify.
- Код d84e405: arena удалена; per-op staging args1→in_stage (src-offset GetLongArrayRegion)→in_arr; in_stage lazy high-water, единственный контракт = per-op len ≤ IN_SCRATCH_CAP (ERR_INPUT_CAPACITY без изменений); shape-A/A' путь bit-identical (total_in=0); wire не менялся (TABLE_VERSION 2). Найдено по ходу: старый путь делал ПОЛНЫЙ memset total_in КАЖДЫЙ батч (arena.clear+resize) + bulk copy, и bulk-копировал ВСЕ ops даже при break на op 0.
- Проба bench/batch/{D1StagingProbe.java,run_d1_probe.sh}: paired old/new .so arms, 64 shape-B ops, len {64,1024,4096}, 4 threads, BENCH.lock. Инфра-уроки: System.load(closed→module) обязателен; CRUSSTY_BATCH_NATIVE_LIB = путь к ЗАКРЫТОЙ lib (dlopen ядер), не к модулю; RSS-измерение ПОСЛЕ join() маскирует retention (thread-local деструктурируется) — протокол переведён на alive-parked threads.
- Результат: 2.07x faster @2 MiB (338→163 µs), 1.37x @512 KB, ~1.07x slower @32 KB (честно, per-op вызовы дороже снятого bulk на малых len; win от ~512 KB); RSS alive Δ: old +24 024 KB vs new +14 064 KB = −9.7 MB ≈ 4×2 MiB арены. run1-агreement: 2.27x/1.32x/parity.
- РЕБЕЙЗ-КОЛЛИЗИЯ: push словал конфликт с 2bd43c4 (G3 shape-C spike: g42 (IIIII[I[J)I, их C-arm ТОЖЕ ел арену). Разрешено семантически: D1 расширен на C-путь (in_stage → narrow jint → SetIntArrayRegion), merged 43275e1+e06ab4d+0c86f10+5da5bf1. Post-rebase revalidation: abi 131087, 2.17x @2 MiB, RSS −7.5 MB, tests 34/34 (вкл. их C-pins), clippy 13 = baseline Δ0.
- Вывод для трека: TASK-47 post-D1 re-bench gate VACUOUS для shape-A таблицы (bit-identical path — их ячейки никогда не исполняли удалённый код); объединённый batch NO-GO стоит С закрытым prerequisite. Addendum в BATCH_ROLLOUT_AB.md + статус-аддендум в HOTSPOT_CANDIDATES_V2.md (D2-D4 уже закрыты TASK-43, D5 observation-only). CLAIMS done + re-pull verify. Live server не тронут; токен не экспонирован; worktree /home/z/w-t50 (push, к следующему тику можно удалить).

Stage Summary:
- TASK-50 (D1) ЗАКРЫТ: двойное копирование shape-B/C устранено, неограниченная per-thread retention заменена конрактом 32KB, измерения приложены. Батч-трек: G1-G8 закрыты/измерены, D1 закрыт — диспетчер остаётся transition-амортизатором (не ускоритель floor-ядер), приговор TASK-47/48 теперь полный. Открытые направления: их G3 spike продолжение (второй контур), G4 site-arming (только shape-A body-dominated ≥700ns — но таких в таблице нет, кроме id 0 ~505µs где route невидим), новые hotspot'ы по dump→analyze→optimize, PROVEN_WINS_SYNC §4 (5 batch-surface записей вне канона).
