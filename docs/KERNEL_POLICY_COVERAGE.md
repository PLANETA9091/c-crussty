# Kernel-policy gate coverage — `verify_kernel_pref.sh` (TASK-13)

**Date:** 2026-09-07T18:52Z · **commit tested:** `d87067e` · **agent:** subagent-3f (SESSION 005, wave-3 BOOST)
**Scope:** every remap candidate registered in `src/kernel_policy.rs` (`DO_NOT_WIRE` — the only set
`kernel_policy::registration_fallback` ever remaps). **Generator:** `scripts/verify_kernel_pref.sh`
(re-runnable; runtime artifacts in `/tmp/crussty_kp_pref`, nothing else committed).

## Verdict

**4/4 remap candidates verified live = 100% coverage of the remap-able kernel surface.**
All 4 default-native bindings reproduce their P500 REGRESSION (ratio >= 1.18) and all 4 re-bound
bridges (`CRUSSTY_KERNEL_PREF=old`) are bit-exact vs the paired old kernel AND run at old-kernel speed
(regression neutralized). No engine/.so/gameplay files touched.

| # | bridge class.method (gid) | default fnPtr | fnPtr under `CRUSSTY_KERNEL_PREF=old` | bit-exact parity (re-bound bridge == old) | alt==old outputs | ns/op default (alt) | ns/op old | ns/op `PREF=old` bridge | ratio default (alt/old) | ratio under `PREF=old` | registry ratio (P500 v2) | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `PaperNativeLevelChunkHeightmap.newCombinedUpdateSummary` (g19) | `Java_PaperNativeLevelChunkHeightmap_newCombinedUpdateSummary` | `Java_PaperNativeLevelChunkHeightmap_oldFourUpdateSummary` (re-bound under the alt method name) | PASS (3/3 seeds) | DIFFERS (see notes) | 12.34 ms | 2205.1 us | 2215.9 us | 5.594 | 1.005 | 5.700 | OK — regression visible; old path restores baseline |
| 2 | `PaperNativeMarkerCache.cachedSummary` (g20) | `Java_PaperNativeMarkerCache_cachedSummary` | `Java_PaperNativeMarkerCache_oldSummary` (re-bound under the alt method name) | PASS (3/3 seeds) | DIFFERS (see notes) | 551.2 us | 118.9 us | 119.6 us | 4.634 | 1.005 | 4.540 | OK — regression visible; old path restores baseline |
| 3 | `PaperNativePalettedReencodeScratch.directPackedSummary` (g27) | `Java_PaperNativePalettedReencodeScratch_directPackedSummary` | `Java_PaperNativePalettedReencodeScratch_oldNewArraySummary` (re-bound under the alt method name) | PASS (3/3 seeds) | DIFFERS (see notes) | 1133.3 us | 488.5 us | 491.2 us | 2.320 | 1.006 | 2.350 | OK — regression visible; old path restores baseline |
| 4 | `PaperNativeProtoChunkHeightmap.newCachedContainsSummary` (g34) | `Java_PaperNativeProtoChunkHeightmap_newCachedContainsSummary` | `Java_PaperNativeProtoChunkHeightmap_oldEnumSetForeachSummary` (re-bound under the alt method name) | PASS (3/3 seeds) | DIFFERS (see notes) | 2062.2 ns | 1163.8 ns | 1166.7 ns | 1.772 | 1.002 | 1.780 | OK — regression visible; old path restores baseline |

*alt/old ns/op are independent kernels in the same JVM; the `PREF=old` bridge column is the alt bridge
method after the registration-time re-bind — i.e. exactly what a caller of e.g.
`PaperNativeMarkerCache.cachedSummary` executes under the env.*

## Coverage statement

- Remap-able surface = `DO_NOT_WIRE` registry entries (4). Verified: **4/4 (100%)** — parity + timing + env semantics, live on the closed `native/libpaper_native_jni.so`.
- Not remap-able by design (`registration_fallback` maps only `DO_NOT_WIRE`): the 10 `PROVEN_WINS` entries and the 270+ other registered natives — out of scope for the gate (they are wiring-policy, `decide()`, not kernel-pref).
- Static drift guards, all checked per candidate: `bench/p500/java/p500/groups.tsv` row contains alt+old under one sig (gid derived: 19/20/27/34); `native/JNI_EXPORTS.manifest` exports both symbols with EQUAL sigs; `nm -D` exports both symbols from the closed `.so`; Rust unit test `kernel_policy::tests::fallback_symbols_exist_in_jni_table_with_matching_sigs` (registry<->jni_table, sig equality + `Java_{class}_{paired_old}` derivation): **PASS (`cargo test --release kernel_policy`)** test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s
- Env parsing mirror: accepted values verified against the Rust source (`conservative_pref`): `old|conservative|safe|1`; negative probe `CRUSSTY_KERNEL_PREF=bogus` on g20 leaves alt bindings: ratio bogus/default = 1.016 (PASS, default behavior unchanged).

## Method

1. **Enumeration (no hardcoded list):** `DO_NOT_WIRE` parsed from `src/kernel_policy.rs` at runtime; the fallback symbol is derived as `Java_{class}_{paired_old}` — verbatim the Rust `format!` in `registration_fallback`. Any registry change re-runs through the same cross-checks (fail loudly on drift).
2. **Mechanism:** TASK-04 (`8ec63b9`) swaps the implementation pointer at REGISTRATION time via JNI `RegisterNatives` with the derived old symbol. The verifier performs the same primitive on the same closed `.so` in a standalone JVM (test-only `dlsym`+`RegisterNatives` shim, built into `/tmp`, never shipped), then delegates measurement to the **unchanged** P500 harness (`p500.Bench`: REAL 120ms batches, median-of-5, min-of-two-passes, strategy ladder, fresh args per method; JVM flags identical to `run_p500.sh`). Groups: 19/20/27/34 only — short subset, not the 49-group suite.
3. **Parity:** per kernel, 3 seeds of identical inputs (xorshift-filled); checksum folds return value + mutated dst arrays (FNV). Gate: re-bound bridge == paired old kernel, bit-exact on all seeds. Informational: alt vs old outputs (same-semantics claim of the registry).
4. **Timing:** one JVM per group per mode (default / `CRUSSTY_KERNEL_PREF=old`), 8 JVMs total + 1 env fail-safe probe. Ratios compared against the registry ratios from the 2026-09-08 full rerun (5.70/4.54/2.35/1.78).
5. **Cross-check:** `bench/p500/aggregate_p500.py` (canon aggregator, untouched) run over the verifier's own raw TSVs: default run classifies all 4 pairs REGRESSION; `PREF=old` run classifies all re-bound bridges PARITY vs the old kernel.

## Interference & environment

- live Purpur 1.21.10 server (dedicated ~28-36% CPU on 2 vCPU) throughout; TASK-17 30s lifecycle soak (~32% CPU) observed just before the timed window and excluded from it.
- All timed runs + the cargo guard held `flock /tmp/crussty_bench.lock` (single window, `-w 2400`).
- 2 vCPU sandbox, JDK 21 (/home/z/jdk21, bench-standard); `p500.n` default 256 (canon). Paired-ratio methodology (canon P500) keeps verdicts robust to this shared-CPU noise; absolute ns/op are NOT comparable to P500_REPORT_v2 absolutes, ratios are.

## Verified live vs not verified here

- **Verified live (this run, closed `.so`, headless):** the re-bind primitive with the derived fallback symbols (all 4 succeed, `rc=0`), bit-exact output parity of every re-mapped bridge with its paired old kernel, perf restoration to old-kernel baseline, and the exact `conservative_pref` env semantics incl. fail-safe on garbage values.
- **Not re-verified here (already covered elsewhere):** the engine-side plumbing inside the live `libcrussty_runtime` on the running Purpur server (TASK-04's own live verification, commit `8ec63b9`: all 4 remaps logged via `CRUSSTY_KERNEL_POLICY=audit`, 98/283/0 unresolved); the closed engine runtime is not exercised by this script by design (no server restarts).
- Findings: alt-vs-old output parity — PaperNativeLevelChunkHeightmap: alt and old outputs differ (informational); PaperNativeMarkerCache: alt and old outputs differ (informational); PaperNativePalettedReencodeScratch: alt and old outputs differ (informational); PaperNativeProtoChunkHeightmap: alt and old outputs differ (informational).

## Raw evidence (runtime, `/tmp/crussty_kp_pref`)

`results/parity.tsv` (per-seed checksums), `results/timing_default.tsv` / `timing_prefold.tsv` / `timing_prefbogus.tsv` (P500 RESULT lines), `results/aggregate_default.md` / `aggregate_prefold.md` (canon aggregator output), `logs/*.log` (incl. engine-format `kernel_pref: ... bound to old kernel (...)` arm lines), `results/cargo_test_summary.txt`, `results/interference_start.txt` / `interference_end.txt`.
