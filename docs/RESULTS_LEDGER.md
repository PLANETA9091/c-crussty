# RESULTS_LEDGER — executive summary of the CRUSSTY optimization campaign (TASK-49)

* Author: agent-7625532f (TASK-49-w8), 2026-09-08. **DOCS ONLY — no code, no `.so`, no gameplay,
  no live-server contact.** Single executive summary for the repo owner; every claim carries its
  sha(s) and a doc pointer, labeled by evidence class.
* Sources mined (read, not recalled): `docs/OPTIMIZATION_ROADMAP.md` (5c8e66b+839aefc),
  `docs/BATCH_ADOPTION_MATRIX.md` (d02fbc2), `docs/BOOST_SWEEP.md` (f55f9c9),
  `docs/HOTSPOT_CANDIDATES.md` (0dcfa7b) + `_V2` (0939257), `bench/p500/results/P500_REPORT_v2.md`,
  `bench/areamap/results/APPLY_BENCH.md` + `docs/AREAMAP_COALESCING_FEASIBILITY.md` (94c4891) +
  `TASK30_ORACLE.md` (125e648), `bench/lifecycle/results/LIFECYCLE_REPORT.md` + `SOAK_REPORT.md`
  (0200e7b), `bench/bootab/results/BOOTAB_REPORT.md` (e6a030d) + `TASK45_SDK_STATS_AB.md`
  (54c8638 → d176e46), `bench/batch/results/BATCH_ROLLOUT_AB.md` (c351b46+90829f4) +
  `A2_SHAPE_REPORT.md` (dc434d8), `docs/BLEND_CACHE_PATCHER_DESIGN.md` (a7e3967, §9 NO-GO 1c5eefb),
  `src/kernel_policy.rs` PROVEN_WINS/DO_NOT_WIRE (db820b1), `docs/KERNEL_POLICY_COVERAGE.md`
  (66fbced), dev-logs `CLAIMS.md` + `worklog.md` sessions 003–008.

**Evidence-class legend.** LIVE = runs on the live server path, verified there. MEASURED = an
initial ESTIMATE converted to numbers by a dedicated A/B on the live-path mechanism. BENCH-ONLY =
measured kernel pair, no live routing. MODELED = arithmetic over measured inputs (no direct run).
ESTIMATE = order-of-magnitude, pending bench. Negative results are first-class: §3 exists because
a measured "no" is a shipped fact (nothing regressed), not a failure.

---

## §1 Verdict

After eight waves and the closure of every claimed task TASK-01…49 (§6), the campaign has taken
the CRUSSTY plugin from an unmeasured JNI surface to an exhaustively classified one: all 70
old/alt kernel pairs are timed and parity-classified against the canonical P500 rerun (49 groups /
129 kernels / 0 crashes, `P500_REPORT_v2.md` @ 3baa0f7), the two live wirings (area-map,
improved-noise) carry proven >100x-class and lifecycle wins backed by a parity oracle and a
no-leak soak, and every hypothesis that did not survive measurement — batch dispatcher, blend-cache
realization, same-tick coalescing, boot-latency A/B — is closed with an evidence-linked negative
result rather than a shipped regression. The remaining distance to "15000x everywhere" is not an
engineering shortfall but physics plus closed-source boundaries: the measured 35–90 ns JNI
transition floor *is itself* the per-op cost for 32 floor kernels in 13 groups, and the aggregate
plugin-side batch ceiling is ~0.047 ms/tick in the worldgen-burst scenario (≤0.13% of one tick,
`BATCH_ADOPTION_MATRIX.md` §5.1). The project now sits at an honest steady state: shipped wins are
live and verified, negatives are documented first-class results, dormant-but-correct assets (batch
dispatcher, blend-cache designs) sit behind explicit re-bench gates, and the only remaining >100x
levers require ENGINE-TOUCH (`.so`) work that is out of plugin scope by rule. Verification
infrastructure — CI ratio-gate, differential-fuzz job, bootab, soak, aggregator `--check`/`--strict`,
live E2E with hot-reload verification — is in place so that any future claim must arrive with this
same evidence discipline or not at all.

---

## §2 PROVEN WINS

| # | Win | Surface | Magnitude | Evidence class | sha(s) + pointer |
|--:|---|---|---|---|---|
| 1 | **Area-map same-state fast path** — O(1) field-compare skips the whole enumerate+JNI apply loop on unchanged maps; 0 native calls per idle update | `SingleUserAreaMap.update()` on the live server (USER-VISIBLE) | **1,945x–170,612x** per idle update (fast 24.7–25.3 ns vs REAL apply 48.6 µs @d=63 → 4.25 ms @d=511); full-bench headline reaches ~1.8e6x shape-dependent (APPLY_BENCH.md) | **LIVE** + parity oracle: TASK-30 `125e648` — 268/268 per-call multiset parity in both bench modes, "anomaly" replayed with 0 free parameters | hook live-verified (worklog 001–003); smoke `532597b`; benches `beaf374`+`2997d2f`; oracle `125e648` → `bench/areamap/results/TASK30_ORACLE.md` VERDICT: PARITY |
| 2 | **Noise-handle lifecycle: phantom-reaper + 16 identity stripes** (`finalize()` killed; releaseHandle CAS at-most-once) | every `ImprovedNoise` handle in worldgen (USER-VISIBLE) | quiet reclaim **>12,000 ms (timeout) → 21 ms (>571x)**; GC collections under churn pressure 512→21 (**24.4x**), gc_time 523→51 ms (10.2x) | **LIVE** (live E2E x2) | `e59201d`+`99dd17e` (+ releaseHandle/CAS adopted from `e2ec502`) → `bench/lifecycle/results/LIFECYCLE_REPORT.md` |
| 2b | **…plus soak proof (TASK-17)**: 10-min churn under GC pressure | same path | leak **NO** — built == freed == **26,450,000** exactly, `native_unfreed=0`, live=0 after settle; **0 guard trips** (no double-free/CAS violation) across 529 waves; reclaim p50 4 ms pressure / 1.6 s quiet, 0 timeouts, drift 1.09x/1.51x | **LIVE** (MEASURED, `SOAK_VERDICT: PASS`) | `0200e7b` → `bench/lifecycle/results/SOAK_REPORT.md` |
| 3 | **improved_noise lifecycle A/B** — hot path under contention | `ImprovedNoiseNativeOps.noise()` (live wiring path) | **~2x** under contention: t2 150.2→79.6 ns, t4 136.8→70.4 ns (t8 1.57x; t1 parity 1.14x — honest) | **LIVE** (MEASURED A/B; live E2E x2) | same shas as #2 → `LIFECYCLE_REPORT.md` M1 table |
| 4 | **TASK-22 find_class early-exit + ClassFileLoadHook sighting feed + poller backoff** | boot/class-load path (gates activation of both live wirings) | **MEASURED**: 7 scans avoided vs 2 scans per hook = **77.8% of hook calls scan-free**, deterministic (2×2 hooks @120 s window); boot-window counters: scan work **9.3x lower** (156,690→16,822 classes walked), scans **5.3x fewer** (16→3), **63% of misses scan-free** (5/8); ≈0.13–0.26 s scan work removed from the boot window. >100x upper end **NOT demonstrated** (workload too small) — honest floor only | **LIVE (MEASURED)** — converted from ESTIMATE by TASK-45 | `e9405d1` (+tests `584f94a`,`5e9cff6`); counters `53578ba`; report `54c8638`→`d176e46` → `bench/bootab/results/TASK45_SDK_STATS_AB.md` |
| 5 | **Blend-cache kernel pair** (`oldEmptyBlenderSummary` → `newEmptyBlenderSummary`) | g21, both kernels in `libpaper_native_jni.so`; registered, **nothing routes to them** | **316.45x** (95.3 µs → 301.1 ns, ratio 0.003, stability 0.0%, baseline drift 0.0%; refreshed from v2-era 244x) | **BENCH-ONLY** — live realization measured **NO-GO** (see §3.2) | `P500_REPORT_v2.md` §21/§Wins; registry `PROVEN_WINS "P500 WIN (316x)"` @ `db820b1`; designs `4fb9d12`+`a7e3967` |
| 6 | **Kernel-policy remap gate** (`registration_fallback`, `CRUSSTY_KERNEL_PREF=old`) — insurance wiring: the 4 measured regressions (1.78x–5.70x slower) are swap-remapped to their old kernels at registration | registration-time, live-verified on the closed `.so` | **4/4 remap candidates rescued to ≤1% parity** (16/16 measured pairs, 100% coverage of the remap-able surface); live-verified 4 remaps / 0 unresolved | **LIVE** (correctness/insurance win, not a speed win) | TASK-04 `8ec63b9`; TASK-13 `66fbced` → `docs/KERNEL_POLICY_COVERAGE.md`; TASK-28 gate `ERR_KERNEL_REFUSED=-10` in `397856c` |
| 7 | **C1–C8 hot-path hygiene pack** — all landed or closed | cplug-sdk + src hot paths | deliberately sub-10x, bounded by sweep v1: COW readers remove the JVM-wide serialization point (0 alloc/read); serve branch `Arc<[u8]>` (~200–500 ns × 1–3 firings); method-ID cache 300–900 ns → 20–50 ns/hit; batch control-plane scratch 8 allocs→0 per `run()` (K=1 **−9%** measured, K≥8 parity) | **LIVE** (shipped; each item bounded, none >10x — per BOOST_SWEEP §1 "not counted" list) | C1 `e9405d1` · C2 `54a6724` · C3 `28ad646`(+`1449f7f`) · C4 leave-as-is (sub-noise, cleared) · C5 `f86c517` · C6 `106bb73` · C7 `f542d02` · C8 `397856c` |

| 8 | **FlatCacheContext promotion (TASK-53)** — first full §Lifecycle application: twice-reproduced P500 WIN pair, semantic parity proven, promoted to `PROVEN_WINS` + env-gated promotion binding (`CRUSSTY_KERNEL_PROMOTE`, default OFF, fail-safe) with a live-armed self-test | `PaperNativeNoiseChunkFlatCacheContext` old/new × true/false `(II[J)I` — registration-time impl-pointer rebind of the original bridge names to the WIN symbols | **1.24x / 1.17x** (23.5→18.9 µs true, 21.8→18.6 µs false; ratios 0.804/0.853 on the 2026-09-09 fresh full rerun, 0.805/0.846 on 2026-09-08) | **LIVE-armed self-test + offline gate**: parity old≡new byte-exact 3648/3648 inputs per pair (result + full dst, cross-JVM deterministic fixtures); armed boot: rebind lines + SELF-TEST PASS (fixtures 8/8 vs offline old-impl expectations, bridge parity 8/8); unarmed boot: 0 marker lines | `bench/p500/parity/results/FLATCACHE_PARITY.md` + `PROMOTE_E2E_2026-09-09.md`; registry `kernel_policy.rs` PROMOTE_PAIRS + 2 ProvenKernel entries (verdict "P500 WIN + live-verified") |
| 9 | **Promotion lifecycle wave 2 (TASK-54)** — the three remaining wire-eligible WIN pairs through the full §Lifecycle; self-test generalized to multi-shape signatures | `PaperNativeNoiseInterpolatorSlice` oldJaggedSummary→flatSummary `(IIII[J)I`, `PaperNativeImprovedNoiseInline` oldPMethodSummary→switchGradientSummary `([BI[J)I`, `PaperNativePalettedReencodeScratch` oldNewArraySummary→scratchThreadLocalSummary `(I[J)I` — `PROMOTE_PAIRS` 2 → 5 | **3.32x / 1.22x / 1.20x** (6.3 ms→1.9 ms, 9.3→7.6 µs, 493.6→411.9 µs; ratios 0.301, 0.819, 0.834 on both the 2026-09-08 rerun and 2026-09-09 fresh rerun) | **LIVE-armed self-test + offline gate wave 2**: parity old≡new byte-exact 1600/3036/3072 inputs per pair (result + full dst, two identical runs = cross-JVM fixture determinism); armed boot: 5 pairs re-bound + generalized SELF-TEST PASS (fixtures 20/20, bridge parity 20/20); unarmed boot: 0 marker lines | `bench/p500/parity/results/WINPAIR_PARITY_RAW.tsv` + `PROMOTE_E2E_2026-09-09.md` (TASK-54 section); registry verdicts → "P500 WIN + live-verified (TASK-54)", Allow set +3 (`PROVEN_WINS_SYNC` §4 item 5 / §5) |

**Also landed (correctness-class wins, not perf claims).** TASK-43 `11b19c3` (D2 POLL_STATE bounded
65,536 + no-alloc-on-hit; D3 MAIN_IDS survives the pre-boot null-server loop; D4 poison-recovery)
and TASK-46 `ef2754e` (7 JNI-reachable lock-`.unwrap()` sites → `into_inner`, +3 headless tests)
closed the "panic unwinding through JNI = VM abort" hazard class repo-wide — the exact mechanism
root-caused for the live P0 (armed-SIGSEGV, `28cdc09` → closed `b002d9c`). Verification: cplug-sdk
20/20 + crussty 24/24, clippy Δ0. LIVE-path correctness hardening.

---

## §3 MEASURED NEGATIVES (equally valuable — each closed by measurement, nothing shipped)

| # | Negative | Evidence | Why it is the *right* answer | sha(s) + pointer |
|--:|---|---|---|---|
| 1 | **Batch dispatcher is net-negative for every tested cell** — 6 groups × K≤256 | best **+11%** overhead (g10, K=256), worst **+168%** (g14, K=1); overhead 30–270 ns/call = shape-B double-copy (TASK-39 D1 signature); **no K\* crossover ≤256**; threshold-T = OFF | **Parity is bit-exact everywhere** (batch == direct on every cell, every lane) and error paths work (`ret=-3` refused probe) — the dispatcher is CORRECT, just unprofitable at current marshalling economics. The BOOST-sweep "≤40x dormant" line is **REFUTED by measurement**, not pessimism | `c351b46`+`90829f4` → `bench/batch/results/BATCH_ROLLOUT_AB.md` |
| 1b | **A' (single-copy) shape does not rescue it** — TASK-48 Phase 1 landed the A' machinery (TABLE_VERSION 2, KERNEL_COUNT 14) and measured it | dispatch overhead **+40.5 ns/op @K=256** (shape A was +43.1 → the 3-scalar plane is free); g9 batch **never** beats direct (116 ns < breakeven ~160 ns; K=1 3.07x worse → K=256 1.35x worse); the wave-2 matrix "19.2x" projection **REFUTED**; whole wave-1 list (g42 34.6 / g35 81.4 / g39 87.7 / g40 88.6 ns direct) **NO-GO-by-measurement** | **Combined verdict stands post-D1-fix** (stronger than either report alone); re-bench gate (`run_batch_rollout.sh`) is the only entry criterion for a revisit | `b4a5c9b`+`de81bcf`+`dc434d8` → `bench/batch/results/A2_SHAPE_REPORT.md`; cross-ref `90829f4` |
| 2 | **Blend-cache impl is NO-GO** (TASK-32 Phase-1 probe, gate `CRUSSTY_BLEND_CACHE`-class env default OFF) | item 4 decisive: JFR post-boot attach, 252 samples over forceload+settle — per-column blend mechanics **0/180** worldgen-worker samples; single hit 0.40% = one-time `Blender.of` setup; NO-GO threshold <0.1% breached. H2 ("already folded") was refuted at bytecode level (javap: EMPTY path NOT folded, 3× MutableDouble + BlendingOutput alloc/call) — but the machinery is **irrelevant**: fresh worlds take `Blender.EMPTY` → cheap map-probe path; the 316x pair models the DENSE branch (upgrade worlds) which this server never walks | Honest negative with proof: proto stays `PATCH_ENABLED=false`, dormant; `kernel_policy` untouched; the pair remains BENCH-ONLY (§2.5). Item1/3/5 gates (probe safety, N-scaling, V1 parity 10,000/10,000) all PASS — the *probe* worked, the *premise* failed | probe `cedc9df`; verdict `1c5eefb` (§9 final in `docs/BLEND_CACHE_PATCHER_DESIGN.md`); session worklog `587fd1b` |
| 3 | **Area-map same-tick coalescing is structurally moot** | javap call-graph of the REAL remapped purpur jar (paperclip patch+reobf on throwaway copies; `/home/z/server` never written): ≤1 **native** call per map-instance per tick — first non-same-state update pays, same-tick repeats hit the 0-native fast path (24.7–25.3 ns) | Nothing to coalesce at the Java/plugin layer; the rare multi-native tick is semantically non-coalescible (mid-tick readers would see deferred state — forbidden); the impossible 6→1 cross-instance merge would save ~4 µs/player-tick (**~0.008% of a tick**) and needs bridge+`.so`. No implementation shipped, per investigate-first gate | `94c4891` → `docs/AREAMAP_COALESCING_FEASIBILITY.md` |
| 4 | **Boot-latency module A/B is directional, NOT proven** | A (pre-wave `4fb9d12`) 3.10 s ±0.04 vs B (master `217e3e8`) 3.02 s ±0.05 to "native surface live" = **−2.6%, n=3, ranges overlap**; secondary `Done(` indistinguishable (31.03 vs 31.21 s) | Recorded as DIRECTIONAL ONLY everywhere — the honesty guard (TASK-40 `5c8e66b`) found **0 overclaims** repo-wide; no regression exists either. Engine-agent `.so` A/B remains pending a runtime rebuild (recipe: BOOTAB_REPORT §7) | `e6a030d` → `bench/bootab/results/BOOTAB_REPORT.md` |
| 5 | **Stale "wins" reclassified by evidence-sync** (TASK-31) | 27 registry entries rechecked against the canonical rerun: PluginLoadingAllocation 1.55x → **0.993/0.996 parity**; AquiferSurfaceSampling 1.15x → **0.906**; blend-cache 244x → 316x (number refresh); hot-path WIN set 7→4; gate behavior unchanged | The registry (`PROVEN_WINS`/`DO_NOT_WIRE`) is now synced to one canonical dataset — wiring decisions read only reproduced facts | `db820b1` → `docs/PROVEN_WINS_SYNC.md`; full-rerun basis `e5c4fad` |
| 6 | **BE-tick shouldTickBlocksAt guard closed at the static layer** (TASK-89, census-first, 0 boots) — the check is already one lock-free hash-get (`ConcurrentLong2ReferenceChainedHashTable.get`) + flag, invoked **once per DUE chunk-container per tick** (single `tickCheck` site in `LevelTicks.sortContainersToTick`), not per scheduled tick | ceiling: 10,000 due containers/tick × ~40 ns = **0.8%** of a 50 ms tick (realistic ≤0.04%) vs pre-registered **>3%** GO gate — unreachable 4-75×; stale-accumulation refuted (`removeContainer` on unload; loaded-non-ticking containers bounded and predicate-skipped) | A guard replaces one O(1) lookup with another cache lookup — the TASK-80 "mechanism works, economics do not pay" shape; Moonrise already superseded the upstream SparklyPaper lever on this stack | this audit → `docs/BETICK_STATIC_AUDIT_2026-09-08.md`; ADDENDUM-6 §11 |

| 1c | **Wave-1 shapes D/E/F measured: batch never wins on the probe bodies** — the descriptor-parser port (BATCH_API_PROPOSAL §4/§5) + wire v3 ref plane completed the shape surface (G3 FULLY closed: g35/g39/g40 now expressible, ids 15/16/17), and the floor bench closed the economics question | g35 direct 77 ns vs batch op 594-808 ns (**5.9-10.6x worse at every K**, per-op GetObjectArrayElement+delete_local_ref cost); g39/g40 direct 18.4 µs / 7.4 µs on String-object inputs (input-domain dependent; batch delta honest +100-150 ns/op); parity OK on every row (count-written contract probe-verified, MUTATION NONE) | **G5 consequence: measured-T still exists for NO wave-1 candidate** — Stage-1 stays demonstrator-terminal; the shape surface is complete for any future in-engine kernel body | `449ebe8`+`ed845d9` → `bench/batch/results/WAVE1_V3_SHAPES_REPORT.md` |
| 1d | **G9 whole-method hook: amplification blocker measured MARGINAL** — first live JFR measurement of the `Ap2.fillArray` loop (1049-chunk forceload gen, dormant hooks) | fillArray = 5.12% of async-worker samples during the ~200 s gen burst ≈ 1.0-1.5 ms/tick ≈ 19-29k calls/tick (band 12k-40k, at the §4.2 bar) **BURSTS ONLY; steady-state ≈ 0; 307/307 samples on the Paper async worker, 0 on the Server thread** | Blocker (b) (undocumented `(III[J)I` semantics) still fatal → **G9 stays NO-GO, upgraded to never-hook at steady state**; gen-burst potential parked behind blocker (b) | `1a1fe6b` → `reports/G9_JFR_AMPLIFICATION_PROBE.md` |

---

## §4 PHYSICAL LIMITS — why "15000x everywhere" is not physics

| # | Limit | Numbers | Class |
|--:|---|---|---|
| 1 | **The JNI transition floor IS the per-op cost for floor kernels.** Per-op cost = T = F(transition) + B(body); for floor kernels B ≈ 0, so the 35–90 ns transition is not overhead to remove — it is the operation | floor 35–90 ns measured (anchors: StaticCacheGet **34.6 ns** = global minimum, RangeChoice 81.4 ns, `(I)D` @N=1 19.9–31.2 ns — `P500_REPORT_v2.md` + `P500_SCALING.md`, canon via 3baa0f7/TASK-10 errata). A >100x per-call speedup would need ≤0.35–0.9 ns/op — below one L1 access, 25–100x below the cheapest observable transition | MEASURED |
| 2 | **32 floor kernels / 13 groups are blocked-by-`.so`** for the >100x class: the only true fix is fewer transitions per op, i.e. batch entries *inside* the closed `libpaper_native_jni.so` — not editable. The 33 body-dominated groups are blocked by their own bodies (CaveCarverSkip 34.2–34.6 ms, BlendedNoise 50.9 µs) — only in-`.so` algorithmic replacement moves them | groups g9, g18, g24, g28, g30–g33, g35, g36, g39, g40, g42 (`BOOST_SWEEP.md` §3; `BATCH_ADOPTION_MATRIX.md` §1/§4) | MEASURED + MODELED |
| 3 | **Remap-able surfaces are exhausted.** Of 70 pairs exactly 2 have alt ≥2x (g21 316x — live realization measured NO-GO, §3.2; g23 3.32x — needs a new byte-hook, unowned). The `DO_NOT_WIRE` remap surface is fully covered 4/4 at ≤1% parity (`66fbced`). After TASK-32's measured NO-GO, **the >100x pipeline is empty without ENGINE-TOUCH (`.so`) work** — measured, not assumed | `BOOST_SWEEP.md` §2.2/§3; `1c5eefb` | MEASURED |
| 4 | **Aggregate plugin-side batch-API ceiling is ~0.047 ms/tick** in the S2 worldgen-burst scenario (envelope 0.028–0.066) = **≤0.13% of a 50 ms tick**; E boot wave ≈ 0.83 ms/event. The dispatcher's theoretical T(K)=m+C/K caps (11.5–40x naive, 1.4–4.8x ref-adjusted) are themselves **refuted for the as-built code** by the measured net-negative sweep (§3.1) — reaching even this ceiling requires in-`.so` marshalling economics that do not exist today | `BATCH_ADOPTION_MATRIX.md` §5.1 (`d02fbc2`); refutation `c351b46`+`dc434d8` | MODELED (ceiling) + MEASURED (refutation) |

**The >100x class that DOES exist** — and all shipped exemplars of it: skip per-call machinery with
an O(1) guard instead of making the machinery faster (same-state/constant-fold guards → area-map
1,945x–170,612x, blend-cache 316x bench-only; lifecycle hygiene → >571x reclaim; boot-path caching →
TASK-22). Every one of these three classes is now either shipped (LIVE) or measured-inapplicable
(TASK-32 NO-GO).

---

## §5 WHAT REMAINS

**Engine-domain (ENGINE-TOUCH rules apply; nothing actionable in-scope today):**
- TASK-05/TASK-21 minor follow-ups: no live npm bug exists (e2e install→run verified, `02b0451`);
  F1 stale-pins publish-trap + F2 wrapper exit-code/musl are documented minors
  (`task05-npm-precheck.md`).
- g38 `ServerEntityDeltaIdentity` has a single unpaired old kernel — needs an alt kernel to exist
  first (`BATCH_ADOPTION_MATRIX.md` row 38).
- Engine-agent `.so` rebuild A/B: recipe ready in `BOOTAB_REPORT.md` §7 (same harness; copy the new
  runtime into the throwaway root). Boot-path wave is module-side measured (directional), engine-side
  unmeasured.
- Area-map diff-budget window (~8·d bytes vs cap=2·px, 64–256x fewer JNI bytes/call): the REAL apply
  is JNI-copy-bound (slower than a pure-Java reference 1.38–4.56x, `APPLY_BENCH.md`) — the fix needs
  a bridge+`.so` change (out of scope; pointer in `HOTSPOT_CANDIDATES_V2.md` D1-note).
- Upstream engine batch-API promotion (roadmap §6.2) is the only lever that changes the §4.2 floor
  arithmetic — and only after a marshalling redesign beats the measured D1-class overhead.

**Dormant-but-correct assets (shipped, gated, zero traffic by design):**
- Batch dispatcher: as-built, bit-exact parity, 12–14 kernels policy-gated (`ERR_KERNEL_REFUSED`),
  zero consumers, env default off; revisit ONLY through the re-bench gate in
  `BATCH_ROLLOUT_AB.md` (≥1 group beats direct at some K with parity, no >5% regression).
- Blend-cache: two convergent design docs (`4fb9d12`, `a7e3967`), observation-only proto
  (`PATCH_ENABLED=false`), NO-GO verdict recorded; poison-recovery for its 2 lock sites is
  report-only (`ef2754e`) — apply before ever enabling.
- PREF-inversion ("fast-by-default") spec: one paragraph in `BOOST_SWEEP.md` §2.3, useful only when
  future callers exist.

**Verification infrastructure now in place (every future claim must pass through it):**
- CI: p500.yml ratio-gate (1.2x per-kernel paired vs checked-in `baseline.json`, `f04a170`+`61c9aad`),
  aggregator fixture `--check` + fresh-TSV `--strict` (`278dcf0`), areamap-fuzz job (`6c751b7`,
  crate `b6bb359`), smoke-gate PASS on the b1c22f5 stack (`7ba92c2`).
- Benches: P500 canon aggregator (self-test 51, medians stable 0/129, `ed27eb0`), bootab
  (`e6a030d`+`977a339`), lifecycle soak (`0200e7b`), batch floor/rollout/A'-shape harnesses
  (`1449f7f`, `c351b46`, `de81bcf`), blend-probe (`cedc9df`).
- Live: E2E orchestration scripts with hot-reload verification — first live hot-reload PASS,
  dormant boot PASS 15/15 (`28cdc09`); P0 armed-SIGSEGV root-caused (pre-TASK-43 lock-`.unwrap()`
  unwind through JNI) and closed (`b002d9c`); integration GREEN on post-wave master (TASK-31-w3).

---

## §6 TASK-12…49 closure table

(One line each; shas are c-crussty unless noted. Source of truth: dev-logs `CLAIMS.md` @ eff6431.)

| Task | Status | sha(s) | One-line outcome |
|---|---|---|---|
| TASK-12 | done | `d02fbc2` | BATCH_ADOPTION_MATRIX: all 49 groups rated (HIGH×5/MEDIUM×10/LOW×34); ceiling S1≈0.003 / S2≈0.047 ms/tick; 6 anomalies (floor anchor 34.6 ns) |
| TASK-13 | done | `66fbced` | verify_kernel_pref.sh — 16/16 measured pairs; 4/4 remap candidates rescued to ≤1% parity (100% remap-surface coverage) |
| TASK-14 | done | `f04a170`+`61c9aad` | CI ratio-gate in p500.yml + baseline.json (5 JNI-floor parity groups, verbatim medians) + self-testing gate script |
| TASK-15 | done | `b6bb359` | area-map differential fuzz crate (seeded, deterministic, ≥10k-class parity; workspace-excluded) |
| TASK-16 | dup-done | `e5c4fad` | full P500 rerun 49 groups / 70 pairs / 0 missing / 0 CRASH (repeat cancelled by precedent) |
| TASK-17 | done | `0200e7b` | 10-min soak: leak NO (26.45M==26.45M), 0 guard trips, SOAK_VERDICT PASS |
| TASK-18 | done | `0dcfa7b` | HOTSPOT_CANDIDATES.md — C1–C8 ranked + NOT-hot/cleared list |
| TASK-19 | done | `ed8ff1a`+`d3b3ce4` | roadmap refresh to canon P500 v2 (floor 35–90 ns errata) |
| TASK-20 | done | `beaf374`+`2997d2f` | APPLY_BENCH: fast 0.20–1.97 ns/update vs apply 21.8 µs–3.59 ms; REAL apply JNI-copy-bound (1.38–4.56x slower than pure-Java ref) → diff-budget candidate |
| TASK-21 | done | `02b0451` | npm crussty pre-check: NO live bug; F1/F2 minor, non-ENGINE-TOUCH |
| TASK-22 | done | `e9405d1` (+`584f94a`,`5e9cff6`) | C1: find_class break-on-first-match + sighting feed + 2s/10s poller backoff (leak fix included) |
| TASK-23 | done | `54a6724` | C2: COW lock-free hook readers (Arc snapshot swap; order contract; rebased over TASK-22) |
| TASK-24 | done | `28ad646`+`1449f7f` (+`29aa1c7`) | C3: batch control-plane → per-thread SCRATCH (8 allocs→0); K=1 −9%, K≥8 parity |
| TASK-25 | done (dup) | — (verified @`125e648`) | C5–C8 pack already on master before R2 start; SKIP by already-fixed rule |
| TASK-26 | done | `f86c517` | C5: serve branch Arc<[u8]> + one-shot version parse + one-shot serve log |
| TASK-27 | done | `106bb73` (C6) + `f542d02` (C7) | method-ID cache + conservative invalidation + drain-8; log copy-out under lock |
| TASK-28 | done | `397856c` (+`db7cf27`) | batch wiring as-built (mod declaration, gate `ERR_KERNEL_REFUSED=-10`, 12 PROVEN_WINS batch entries, drift-guards) + rollout design (auto-threshold T, CRUSSTY_BATCH off default, stage gates) |
| TASK-29 | done | `4fb9d12`+`a7e3967` | two convergent blend-cache designs (cross-linked); per-column cache rejected on measured evidence (g3 0.980, MarkerCache 4.54x regression) |
| TASK-30 | done | `125e648` | ops-count anomaly oracle: VERDICT PARITY (268/268 both modes; replay 0 free parameters; signed ERRATUM) |
| TASK-31 | done | `db820b1` | PROVEN_WINS evidence-sync: 27 entries, 3 stale WIN→PARITY, blend-cache 316x, Allow-set unchanged. (TASK-31-w3 = integration GREEN, report-only) |
| TASK-32 | done — **NO-GO** | probe `cedc9df`, verdict `1c5eefb` (worklog `587fd1b`) | Phase-1 probe: JFR 0/180 per-column samples on live worldgen → impl never started; proto stays OFF; pair BENCH-ONLY (§2.5) |
| TASK-33 | done | `f55f9c9` | BOOST_SWEEP: >10x/>100x ledger, binding truth (283 registered ≠ wired; 2 live wirings), physical limits. (Its "≤40x dormant" batch row is refuted by §3.1 — read together) |
| TASK-33-w4 | done | `217e3e8` | canon 35–90 ns errata across live docs (history not rewritten) |
| TASK-34 | done | `ed27eb0` | aggregator dedup/unpaired/--strict/--check; medians stable 0/129 changed |
| TASK-35 | done | `97afe20` | untrack 8 accidentally-committed bench .class files |
| TASK-36 | done | `6c751b7` | areamap-fuzz CI job (loud parity gate, artifact on failure) |
| TASK-37 | done | `278dcf0` | aggregator canon prose (future reports emit 35–90 ns) + CI fixture --check/--strict wiring |
| TASK-38 | done — **NEGATIVE** | `94c4891` | coalescing moot: ≤1 native call/map/tick (javap-verified); no impl per investigate-first gate |
| TASK-39 | done | `0939257` | HOTSPOT_CANDIDATES_V2: D1–D5 (D1 = batch double-copy = named prerequisite in §3.1; D2–D4 → TASK-43) |
| TASK-40 | done | `5c8e66b`+`839aefc` | roadmap waves 2–4 sync with shas + honesty guard (0 overclaims found) |
| TASK-41 | done | `28cdc09` (P0 closure `b002d9c`) | first live E2E: dormant boot PASS 15/15, first hot-reload PASS; armed-SIGSEGV P0 root-caused to lock-unwrap-unwind class, discriminatively closed |
| TASK-42 | not assigned | — | suggested ID (D1/D2 pack) in HOTSPOT_CANDIDATES_V2; D2–D4 executed as TASK-43, D1 remains open (§5) |
| TASK-43 | done | `11b19c3` | D2+D3+D4: POLL_STATE bounded 65,536 + no-alloc-on-hit; MAIN_IDS survives pre-boot loop; CACHE/QUEUE poison-recovery; +4 tests |
| TASK-44 | not assigned | — | no CLAIMS row ever existed |
| TASK-45 | done | `54c8638` → `53578ba`+`977a339`+`d176e46` | bootab phase-2: scan-avoidance ESTIMATE→measured (§2.4); 60s-window finding → 120s minimum |
| TASK-46 | done | `ef2754e` | poison-recovery sweep: 7 JNI-reachable sites fixed + 3 tests; 2 report-only (blend proto, gated OFF) |
| TASK-47 | done — **NO-GO** | `c351b46`+`90829f4` | rollout A/B: net-negative 6 groups × K≤256 (+11..+168%), parity bit-exact, T=OFF; "≤40x dormant" refuted |
| TASK-48 | done — **NO-GO** | `b4a5c9b`+`de81bcf`+`dc434d8` | A' shape landed + measured: +40.5 ns/op, g9 never wins; wave-1 NO-GO-by-measurement; Phase 2 closed |
| TASK-49 | done | this commit | RESULTS_LEDGER.md — this document |

Pre-wave-1 closure (TASK-01…11, for completeness): phantom-reaper+stripes `e59201d`/`99dd17e` (01/09),
P500 CI `6134cfb` (02), batch proposals `e0d6e06`/`bcb71bf` (03), remap gate `8ec63b9` (04), npm
closed-no-bug via 21 (05), O(N)@262144 `bfdbf87` (06), full P500 v2 rerun `3baa0f7` (07), runtime
class-name fix CRUSSTY `66ff504` (08), area-map smoke `532597b` (11), review/errata TASK-10.

---

## §7 Doc inconsistencies flagged during this consolidation (NOT edited — listed per task rules)

1. **BOOST_SWEEP.md (`f55f9c9`) row 5 "batch ≤40x dormant" is refuted by later measurement**
   (`c351b46`/`dc434d8`); the sweep predates the A/B and was intentionally not rewritten. Read
   §2.2/§3 of BOOST_SWEEP together with `BATCH_ROLLOUT_AB.md` (which says this explicitly).
2. **Area-map fast-path ratio has two canonical ranges**: 1,945x–170,612x (BOOST_SWEEP #1, resize-mix
   basis) vs up to ~1.8e6x shape-dependent (APPLY_BENCH.md headline, 1-move mix). Same surface,
   different mixes — not a contradiction, but a misquote hazard; this ledger quotes the conservative
   range as canonical and labels the larger one shape-dependent.
3. **CLAIMS.md TASK-45 row cites `54c8638` as done-sha** while the measured A/B artifacts are
   `53578ba`+`977a339`+`d176e46` (counters landed after the first phase-2 report; the row text
   contains both). Canonical evidence for the measured numbers = `TASK45_SDK_STATS_AB.md` @
   `d176e46`.
4. **Known open errata already registered in `BOOST_SWEEP.md` §5**: `PROVEN_WINS_SYNC.md` §4.4
   (claims report files differ; they are byte-identical at HEAD), `HOOK_BLEND_CACHE.md` (stale 244x /
   ~115 ns), `BLEND_CACHE_DESIGN.md` §4 (244x), `OPTIMIZATION_ROADMAP.md` §1 (area-map activation
   cadence staleness). None affect verdicts; all are number/cadence staleness.
5. **ID collision**: TASK-32 (blend-cache impl, NO-GO `1c5eefb`) vs TASK-32-w4 (bootab harness,
   `e6a030d`) — different tasks sharing a numeric prefix; disambiguated throughout this ledger.

---

*TASK-49-w8 · agent-7625532f · 2026-09-08 · docs-only; /home/z/c-crussty working tree untouched
(written in a detached worktree); no `.so`, no gameplay, no live-server contact; no invented numbers —
every magnitude above traces to a committed report or registry line.*

---

## §7 ADDENDUM (TASK-70…74, 2026-09-08) — worldgen noise batching layer: all five gates GO, implementation landed

* Author: agent-7625532f. Evidence class: **LIVE** (G-AB on the deployed module
  .so, production class bytes) for the headline row; the pipeline behind it is
  MEASURED/BENCH-ONLY per row.

| Item | Result | Evidence |
|---|---|---|
| G-STEP0 (octave-kernel breakeven) | GO — NB16 3-arg 0.59× Java, breakeven any N≥2 | `bench/p500/results/STEP0_NOISE_CORE_2026-09-09.md` |
| G-RECON (owner loops) | GO — PerlinNoise.getValue = one whole-method octave loop; whole-object kernels already ship in the closed lib | `bench/p500/results/GRECON_OWNERS_2026-09-09.md` |
| G-ABI (handle ABI decode) | GO — ABI decoded empirically, parity 0/51000 bit-exact, whole-getValue 0.825× | `bench/p500/results/GABI_HANDLE_2026-09-09.md` |
| G-BODY (whole-body swap) | GO — real retransform, parity 0/20000, 0.815×, dispatch ≤ noise | `bench/p500/results/GBODY_DISPATCH_2026-09-09.md` |
| **G-AB (decisive live A/B)** | **GO — cpu_burst −11.1 % median (perfect separation, exact p_two=0.0079), wall −12.3 % median (p_two=0.0952), JFR engagement under load; effect 5–8× prediction (inlining-barrier removal)** | `bench/e2e/results/PERLIN_AB_2026-09-09.md` |
| Implementation | Session-1 bridge landed dormant-invisible (`src/perlin_noise.rs`, env-gated), armed boots verified, deployed with backup | c-crussty becd2f3, commits this session |

Caveats (honest): wall p_two=0.151 at n=5 (one-sided 0.075 — the gate's
directional criterion); rollout-grade promotion (kernel-policy whitelist,
NormalNoise/BlendedNoise owners, B.2.2 runbook) deliberately deferred — the
bridge stays default-OFF until that lands.

## §7 ADDENDUM-2 (TASK-79, 2026-09-08) — noise channel extension probes: measured shut

* Author: agent-7625532f. Evidence class: MEASURED (CPU-only rig) + LIVE (combo A/B).

| Item | Result | Evidence |
|---|---|---|
| G-NORMAL (heritage whole-Noise kernel ABI) | DECODE GO — `PaperNativeNormalNoise.nativeGetValue(JJDDDD)D` = `(A(x,y,z)+B(x·1.0181268882175227, y·F, z·F))·valueFactor`, bit-exact 0/20000 on two real NormalNoise objects (20-hypothesis formula sweep × 2 handle orders); three-arm P500: J 1036.4 / N2 655.6 / N1 615.9 ns/call → N1/J 0.594×, **N1/N2 0.939× (−39.7 ns) marginal** | `bench/step0_noise/run_normal_abi.sh`, CSV inline |
| NormalNoise whole-body bridge | **PARKED — marginal ~0.05–0.1 CPU-s/burst (≈0.1%) does not justify a new hook module**; decode + timing recorded for any future batching consumer | commit 47d451b |
| BlendedNoise whole-object kernel | ABSENT from the closed lib (only batch summaries `oldBatchSummary/cachedBatchSummary`); a whole-body compute swap would need a new engine-side kernel — out of module scope | jni_table.rs rows 249–250 |
| G-COMBO (dual arming, live A/B) | **NO-GO — cpu −1.2% p_two=0.84, wall −2.3% p_two=0.69 (n=5/arm ABBA, protocol v2)**; both arms marker-verified; world byte-identical; mechanism: whole-swap ROI inversely related to inlineability (small noise bodies inline; 11KB barrier body was the win) | `bench/e2e/results/COMBO_AB_2026-09-09.md` |
| Channel status | worldgen noise channel measured shut on ALL fronts: PerlinNoise GO (−11.1% live), NormalNoise parked, BlendedNoise kernel-absent, COMBO NO-GO. Next >100x-class work: guard-wave (neighbor TASK-78 census: fluid-push 5.7% top-1, checkInsideBlocks 3.4% #2) | this addendum |

## §8 ADDENDUM-3 (TASK-85, 2026-09-08) — JIT-heuristics / huge-method channel: static-census NO-GO

* Author: agent-7625532f. Evidence class: STATIC CENSUS (full-jar classfile scan, javap-validated) + JDK21 flag archaeology. Zero boots — falsified at the static layer.

| Item | Result | Evidence |
|---|---|---|
| HugeMethodLimit (TASK-83 research prescription) | **UNEXECUTABLE on JDK21 product** — develop-only flag (`VM option 'HugeMethodLimit' is develop...`) | bench/jitflags/results/JITFLAGS_2026-09-08.md §1 |
| DontCompileHugeMethods=false (executable knob) | product, command-line writable — BUT **NO-GO / DO-NOT-FLIP**: full-jar census of the running server jar (9,809 classes) shows 16 methods > 8000 bytecodes, ALL cold (boot-once datafixer/registry `<clinit>`s + datagen-only providers); ZERO in any measured hot domain | bench/jitflags/HUGE_METHODS_SCAN_2026-09-08.txt |
| Dynamics | Tier3InvocationThreshold=200 (measured): single-shot methods never compile regardless of flag → flip is a no-op by construction; a boot A/B would measure noise | §3 |
| Channel status | JIT-heuristics branch of the x1000 hunt CLOSED (9th refuted branch, cheapest refutation: 0 boots, 0 src/). Re-open criteria: scanner re-run shows a >8000-byte method on a measured-hot path | bench/jitflags/huge_method_scan.py |

## §9 ADDENDUM-4 (TASK-87, S7-31, 2026-09-08) — AppCDS v2: S030 refutation OVERTURNED, first boot-channel measured GO

* Author: S7-31 main. Evidence class: MEASURED LIVE A/B (n=5/4 arms, byte-identical seed anchor restored before EVERY boot, BENCH-MUTEX, hs_err 4/0 across 9 boots, dynamic-archive mapping verified per-run).

| Item | Result | Evidence |
|---|---|---|
| S030 "AppCDS REFUTED by own weaving" | **OVERTURNED — experiment-design artifact**: JDK only forbids ArchiveClassesAtExit WITH an agent (that is all R2 proved); vanilla-dump → agent-use never tested until now; agent's woven set is tiny, all other class bytes match the archive | docs/BOOT_SUBSECOND_FEASIBILITY.md ADDENDUM; bench/boot/cds_v2.sh |
| Census corrections to S030 composition | noise-kernel CONSTRUCTORS = 0 samples (S030's ctor lever dead by its own ≥1.5s gate); Climate RTree.build = 0.06s (dead); SHA2 jar-verify = 0.05s (S030 pre-log attribution corrected); worldgen-family in-boot = 1.0-1.25s level prep only (321/322 noise-family samples POST-Done lazy) | /tmp/boot_census JFR 585 samples, analyzer docs |
| AppCDS v2 (vanilla-dump 124MB → agent + SharedArchiveFile) | **GO: mean 16.668s → 13.597s = −3.07s (−18.4%), full sample separation, Mann-Whitney exact p ≈ 0.0079**; baselines 16.038/16.882/16.627/16.501/17.291; CDS 13.838/13.347/14.013/13.190; `Mapped dynamic region` ×3 every use-run | bench/boot/cds_v2.sh + /tmp/boot_cds_v2/*.log |
| Safe degradation | missing/stale archive → baseline-speed boot, no crash (16.520s with nonexistent path) | d9_missing run |
| Operator runbook | dump once (NO agent, bench/boot/cds_rebuild.sh) → append -XX:SharedArchiveFile to every boot; re-dump after Paper/engine updates; CLI-only flags, nothing committed to server.properties | docs/BOOT_SUBSECOND_FEASIBILITY.md ADDENDUM §runbook |
| Honest floor | ~13.2-13.6s; residual = Paper-internal payload (ENGINE-TOUCH) + JVM/bundler startup; **<1s still unreachable without snapshot/restore (CRaC env-blocked)** | ADDENDUM |
| Harness fixes | kill_stdin_holders sleep-child leak (orphaned fd-holders deadlocked BENCH-MUTEX ×3 today) patched to kill children first; cds_v2.sh literal case arms generalized | scripts/e2e_orchestrate.sh |

## §10 ADDENDUM-5 (TASK-86, 2026-09-08, agent-7625532f) — whole-body promotion policy + B.2.2 runbook LANDED

* Author: agent-7625532f. Evidence class: LANDED CODE + measured gates (numbering note: originally written as ADDENDUM-4; renumbered to ADDENDUM-5 on rebase over S7-31's AppCDS ADDENDUM-4. Claim-label collision: S7-30's boot-feasibility task also carried a "TASK-86" label — resolution recorded in dev-logs CLAIMS.md.)

| Item | Result | Evidence |
|---|---|---|
| Kernel-policy whitelist for the whole-body class | LANDED — `("PerlinNoise","getValueWholeBody")` + `("ImprovedNoise","noiseWholeBody")` in `PROVEN_WINS` (evidence: G-AB live −11.1% cpu p=0.0079 / v2 self-test); arming in both modules consults `decide()` — two-key gate (env AND policy, either side = kill-switch) | src/kernel_policy.rs, src/perlin_noise.rs, src/improved_noise.rs |
| B.2.2 whole-body runbook | LANDED — boot sequence (dormant/armed/refusal gates), markers, PASS criteria, abort ladder (env → registry entry → .so revert) | docs/BATCH_ROLLOUT_RUNBOOK.md §9, docs/KERNEL_POLICY.md §whole-body |
| Gates (src/ touched) | cargo test 65/65 (was 64 + new `whole_body_bridge_wirings_are_policy_gated`), clippy Δ0 (12), P500 FULL 70/70 0 CRASH/SKIP, 4 known regressions unchanged | bench/p500/results/P500_REPORT.md (this session's rerun) |
| Live boot validation (post-reset) | ALL THREE §9 runbook gates GREEN on reconstructed env: dormant ALL PASS; armed+audit full trace (policy line "WIRE PerlinNoise.getValueWholeBody: allowed (proven)" + 4 DO_NOT_WIRE surface-only); refusal rig keeps dormant despite env gate (drift-guard test 64/1); 0 hs_err across 5 boots; promoted build md5-verified | docs/BATCH_ROLLOUT_RUNBOOK.md §9 live-table (commit 2016ce8) |

---

## §11 ADDENDUM-6 (TASK-89, 2026-09-08, agent-7625532f) — BE-tick shouldTickBlocksAt guard: NO-GO at the static layer

* Author: agent-7625532f. Evidence class: **javap-fidelity static audit of the running deployment jar** (census-first discipline, TASK-85 precedent — cheapest refutation, 0 boots, 0 src/).

| Item | Result | Evidence |
|---|---|---|
| The predicate | `ServerLevel.shouldTickBlocksAt(long)` = Moonrise `ChunkHolderManager.getChunkHolder(long)` (single lock-free `ConcurrentLong2ReferenceChainedHashTable.get(J)`) + `NewChunkHolder.isTickingReady()` flag — already the minimal O(1) primitive | javap -c on versions/1.21.10/purpur-1.21.10.jar |
| Call frequency | `tickCheck` has exactly ONE invocation site in `LevelTicks.sortContainersToTick(long)`: once **per DUE chunk-container per game tick** (not per scheduled tick); not-due containers skip it entirely | LevelTicks bytecode, single getfield tickCheck |
| Ceiling vs GO gate | 10,000 due containers/tick × ~40 ns = 0.8% of a 50 ms tick (realistic ≤0.04%) vs pre-registered >3% gate — unreachable by 4-75×; stale-accumulation refuted (removeContainer on unload; loaded-non-ticking bounded and predicate-skipped) | docs/BETICK_STATIC_AUDIT_2026-09-08.md §2-§3 |
| Upstream lever | "SparklyPaper mode" reduces a heavier vanilla check path; on this stack Moonrise already reduced it to a lock-free hash-get — lever architecturally superseded (upstream internals not audited; verdict rests on this jar's bytecode) | audit §4 |
| Verdict | **NO-GO / DO-NOT-BUILD** — a guard replaces one hash-get with another cache lookup (TASK-80 economics shape); 10th statically/measured-refuted x1000 branch (slot freed by TASK-87's overturn of the S030 AppCDS refutation) | re-open criteria: audit §5 (JFR ≥3% tick share or engine change to the lookup) |

## §12 ADDENDUM-7 (TASK-94, 2026-09-09, agent-7625532f) — D6 TPS-accounting: OUT at the static layer

* Author: agent-7625532f. Evidence class: javap static anatomy of the remapped runtime classpath + arithmetic on the already-measured JFR profile (TASK-92 discipline, 0 boots, 0 src/).

| Item | Result | Evidence |
|---|---|---|
| Mechanism | per-tick: `addTickTime` = 7×`addDataFrom` + `clearTickTimeStatistics()` (cache invalidation every tick) → `C()` → `getTPS()` → guaranteed miss → `computeTPS()` = **4 full deque walks** (5s/1m/5m/15m ≈ 25,300 entries @ 20 TPS; 15m window = 71%) | MinecraftServer + TickData bytecode, audit §2 |
| Measured match | JFR 142+17+23 samples ≈ 0.13–0.17 ms/tick; model: 130 µs / 25,300 visits = 5.1 ns/visit — mechanism fully explains measurement | JFR_PROFILE_2026-09-09 §4, audit §3 |
| Load-independence correction | window sizes are time-bounded → cost is a **constant** at 20 TPS idle or loaded; only relative share dilutes | audit §3 |
| Verdict | **OUT / DO-NOT-BUILD** — 0.26–0.34% of tick budget vs ≥3% gate (fails 9–23×); perfect O(1) rolling-sum patch removes ≤0.34% = 2+ orders below >100x bar; no config knob exists | audit §4 |
| Design retained | rolling-sum O(1) aggregate strictly correct (per-entry deltas now-independent except boundary); retransform vehicle same as area_map | audit §5 |
| Status | 12th closed x1000 branch; D6 resolved (was open since first live JFR profile) | re-open: audit §6 |
