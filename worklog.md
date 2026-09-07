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
