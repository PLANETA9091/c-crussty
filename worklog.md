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
