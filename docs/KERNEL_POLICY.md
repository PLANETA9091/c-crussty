# Kernel selection policy (do-not-wire gate)

Every native kernel we expose can be *registered* (callable bridge surface)
without being *wired* (routed into a live hot path). Wiring decisions are
entirely ours — we generate the bridge bytes and bind the natives
(`src/bridge_class.rs`, `src/lib.rs::define_and_register`, byte-hook modules).
P500 compares each optimized (`alt`) kernel against its paired old Java-side
kernel (`old`), `ratio = alt/old`, and classifies in
[`bench/p500/aggregate_p500.py`](../bench/p500/aggregate_p500.py):

* **WIN** — ratio ≤ 0.85 (`WIN_MAX`)
* **REGRESSION** — ratio ≥ 1.18 (`REG_MIN`)
* **PARITY** — in between (shared-hardware noise band, ±15%)

Four kernels are *confirmed genuine, scale-invariant regressions*
(`bench/p500/results/P500_SCALING.md`, N=16/256/4096 probe). Wiring any of
them into a hot path would slow the kernel down. That rule used to live only
in prose (README, [OPTIMIZATION_ROADMAP.md](OPTIMIZATION_ROADMAP.md)); it is
now **enforced, tested Rust infrastructure**: `src/kernel_policy.rs`.

## The decision function

```rust
use crate::kernel_policy::{decide, Decision};

match decide("PaperNativeMarkerCache", "cachedSummary") {
    Decision::Allow => { /* route the hot path to the native kernel */ }
    Decision::KeepJava { reason } => { /* keep the Java-side logic; `reason` says why */ }
}
```

Strict policy (the default), evaluated in order:

| Condition | Decision |
|---|---|
| `CRUSSTY_KERNEL_POLICY=off` | `Allow` — **DANGEROUS bypass, A/B benchmarking only** |
| kernel in `DO_NOT_WIRE` | `KeepJava` (measured regression, with reason) |
| kernel in `PROVEN_WINS` | `Allow` (live-verified or P500 WIN verdict) |
| anything else (unknown / unproven) | `KeepJava` — "not proven" (default-safe) |

API surface (all allocation-free; mode cached in a `OnceLock`; registries are
static slices scanned linearly — 4 + 25 entries; plus the TASK-53/54 `PROMOTE_PAIRS` slice (5 pairs, wave 2 = TASK-54) for the env-gated WIN-direction registration binding):

| Signature | Purpose |
|---|---|
| `pub fn decide(class: &str, kernel: &str) -> Decision` | the gate; `class` may be short (`PaperNativeMarkerCache`) or a full internal name (`net/minecraft/.../PaperNativeImprovedNoise`) |
| `pub fn decide_id(id: &str) -> Decision` | convenience `"Class.method"` id form (P500 report form) |
| `pub fn decide_in(mode: PolicyMode, class: &str, kernel: &str) -> Decision` | pure variant under an explicit mode (used by tests) |
| `pub fn mode() -> PolicyMode` | `Strict` (default) / `Audit` / `Off`, parsed once from env |
| `pub fn audit_wire(class, kernel, site)` | wiring-site audit hook: logs the decision in audit mode, silent otherwise |
| `pub fn audit_registered(class, kernel)` | registration chokepoint hook (`lib.rs::define_and_register`): in audit mode flags do-not-wire kernels being registered as surface-only |
| `pub fn do_not_wire_entry(class, kernel)` / `proven_entry(class, kernel)` | direct registry lookups |

## Registries

### `DO_NOT_WIRE` — known regressions (ratio = alt/old, >1 = slower)

| Class | Kernel | Paired old | Ratio | Source |
|---|---|---|---:|---|
| `PaperNativeLevelChunkHeightmap` | `newCombinedUpdateSummary` | `oldFourUpdateSummary` | 5.70× | P500 2026-09-08 full rerun (49 groups, 70 pairs) |
| `PaperNativeMarkerCache` | `cachedSummary` | `oldSummary` | 4.54× | P500 2026-09-08 full rerun (49 groups, 70 pairs) |
| `PaperNativePalettedReencodeScratch` | `directPackedSummary` | `oldNewArraySummary` | 2.35× | P500 2026-09-08 full rerun (49 groups, 70 pairs) |
| `PaperNativeProtoChunkHeightmap` | `newCachedContainsSummary` | `oldEnumSetForeachSummary` | 1.78× | P500 2026-09-08 full rerun (49 groups, 70 pairs) |

All four remain part of the *registered* 283-native surface (callable through
their bridge classes — registration is not wiring) and are **never routed**
into a hot path. Scale-invariance: `bench/p500/results/P500_SCALING.md`.

### `PROVEN_WINS` — whitelisted (allowed for hot-path routing)

Evidence synced to the canonical 2026-09-08 full rerun
(`bench/p500/results/P500_REPORT.md`; 49 groups / 70 pairs / 0 CRASH) by
TASK-31 — full before/after audit in
[PROVEN_WINS_SYNC.md](PROVEN_WINS_SYNC.md).

* **live** wirings (verified on Purpur 1.21.10, must keep working):
  `PaperNativeAreaMap.nativeUpdateOpsBatch` (area_map hook, 64-rect self-test
  == naive set difference) and
  `PaperNativeImprovedNoise.nativeNoise` / `nativeBuildHandle` /
  `nativeFreeHandle` (improved_noise hot-patch v2, self-test PASSED,
  worklog session 003).
* **P500 WIN** verdicts — promotion candidates, wiring-eligible (4 of the
  previous 7 reproduce on the rerun):

  | Kernel | 2026-09-07 v2 (old) | 2026-09-08 rerun | Verdict |
  |---|---|---|---|
  | `NoiseChunkBlendCache.newEmptyBlenderSummary` | 244× (67.0 µs → 274.5 ns) | **316×** (95.3 µs → 301.1 ns, ratio 0.003) | WIN (number refreshed) |
  | `NoiseInterpolatorSlice.flatSummary` | 3.29× (6.2 ms → 1.9 ms) | 3.32× (6.3 ms → 1.9 ms, ratio 0.301) | WIN (reproduced) |
  | `ImprovedNoiseInline.switchGradientSummary` | 1.22× (9.3 → 7.6 µs) | 1.22× (9.3 → 7.6 µs, ratio 0.819) | WIN (reproduced) |
  | `PalettedReencodeScratch.scratchThreadLocalSummary` | 1.20× (483.9 → 403.2 µs) | 1.20× (493.6 → 411.9 µs, ratio 0.834) | WIN (reproduced) |
* **P500 PARITY** reclassifications (were "P500 WIN" on v2; NOT reproducible
  on the rerun — kept in `PROVEN_WINS` so the Allow set is unchanged, but
  they are NOT hot-path swap/promotion candidates):

  | Kernel | 2026-09-07 v2 (old) | 2026-09-08 rerun | Verdict |
  |---|---|---|---|
  | `PluginLoadingAllocation.newLazyValidateSummary` | "WIN 1.55×" (217.9 → 140.6 ns) | 0.993 (116.1 → 115.3 ns) | **PARITY** (reclassified) |
  | `PluginLoadingAllocation.newLazyMissingSetSummary` | "WIN 1.53×" (218.3 → 142.5 ns) | 0.996 (115.1 → 114.6 ns) | **PARITY** (reclassified) |
  | `AquiferSurfaceSampling.newBatchSummary` | "WIN 1.15×" (6.3 → 5.5 µs) | 0.906 (6.0 → 5.5 µs) | **PARITY** (reclassified) |
* **P500 PARITY (batch surface)** — the batch-dispatch table kernels
  (`src/batch_table.rs` ids 0-17, `docs/BATCH_WIRING_PLAN.md` §A.4; ref updated
  S7-12/S7-14):
  caller-initiated infrastructure, not hot-path routing. Pairs covered by
  the rerun stay parity (AquiferIndexStride 1.07×, ChunkDependencies 1.03×,
  DensitySplineContext 1.00×, EntityLookupStatus 1.00×, DensityAp2MinMaxFill
  1.003×); the 5 entries that had **no pair in the canonical report**
  (`TicketSetSearch` ×2, `NoiseInterpolatorFractions.divisionSummary`,
  `ClimateRTree` ×2) are now **calibrated** under canonical methodology —
  see `bench/p500/results/BATCH_SURFACE_CALIBRATION.md` (TASK-51;
  PROVEN_WINS_SYNC §4.1 resolved). Wire-v3 wave-1 additions (S7-14):
  `RangeChoice.optimizedFillArraySummary` (id 15, 1.0025),
  `SpigotLoadOrderDependency.newLoadAfterBuildSummary` (id 16, 1.0034),
  `SpigotLoadOrderDependency.newRemovedCountSummary` (id 17, 0.9955) —
  shapes D/E/F, contract probe-verified, PROVEN_WINS_SYNC §8.

Note: whitelist keys are exact `(class, kernel)` pairs. A kernel *name* that
collides across classes (`cachedSummary` is a regression on `MarkerCache`,
unproven elsewhere) does **not** leak between entries.

## Env override: `CRUSSTY_KERNEL_POLICY`

| Value | Behavior |
|---|---|
| unset / `strict` | enforce silently (default) |
| `audit` | enforce **and** log every decision (`[crussty-plugin] kernel-policy: …`); never crashes — a refusal is a returned `Decision`, the caller keeps the Java path |
| `off` | gate bypassed (`Allow` for everything) — **dangerous**, for A/B benchmarking of a suspect registry entry only; never on a live server |
| anything else | treated as `strict` (fail-safe: an unknown value never widens the gate) |

## Wiring-site integration (the contract for future kernels)

* **Registration** (`lib.rs::define_and_register`) — every surface native
  flows through `kernel_policy::audit_registered(class, method)`. Audit-only;
  behavior never changes. Generated `jni_table.rs` stays untouched.
* **Hot-path wiring** (byte-hook / patch modules like `area_map.rs`,
  `improved_noise.rs`) — before arming a route to a native kernel, the module
  must consult the policy. Pattern (see `improved_noise.rs`, Phase 3):

  ```rust
  debug_assert!(
      crate::kernel_policy::decide(CLASS, KERNEL).is_allowed(),
      "kernel policy refused this routing"
  );
  crate::kernel_policy::audit_wire(CLASS, KERNEL, "site description");
  ```

  `debug_assert` costs nothing in release; `audit_wire` is a no-op outside
  audit mode. Debug builds fail fast if someone demotes a live-wired kernel.

## Lifecycle: promotion & demotion

* **Promote to `PROVEN_WINS`**: a P500 **WIN** verdict (ratio ≤ 0.85, same
  stem pairing, min-of-medians, `aggregate_p500.py`) **plus** live-server
  verification of the actual wiring (self-test / profile evidence), then add
  a `ProvenKernel` entry with the evidence pointer.
  First application — **TASK-53** (`NoiseChunkFlatCacheContext`
  `newTrueContextSummary` / `newFalseContextSummary`, 2026-09-09): the full
  chain was P500 WIN (twice reproduced) → offline semantic-parity gate
  (`bench/p500/parity/`, 3648 inputs/pair byte-exact) → env-gated promotion
  binding (`CRUSSTY_KERNEL_PROMOTE`, default OFF) → live-armed self-test
  through the real bridge (fixtures 8/8, bridge parity 8/8). The operator
  arms the swap via `CRUSSTY_KERNEL_PROMOTE=1`; the registry entries make
  the pair policy-`Allow` either way.
  Second application — **TASK-54 wave 2** (`NoiseInterpolatorSlice`
  `flatSummary` 3.32×, `ImprovedNoiseInline` `switchGradientSummary` 1.22×,
  `PalettedReencodeScratch` `scratchThreadLocalSummary` 1.20×): parity gate
  1600/3036/3072 inputs per pair byte-exact (`WINPAIR_PARITY_RAW.tsv`, two
  identical runs), self-test generalized to multi-shape signatures
  (`(II[J)I` / `(IIII[J)I` / `([BI[J)I` / `(I[J)I`), live-armed e2e
  fixtures 20/20, bridge parity 20/20 — `PROMOTE_PAIRS` 2 → 5 pairs.
* **Demote to `DO_NOT_WIRE`**: a **REGRESSION** verdict (ratio ≥ 1.18)
  confirmed by a rerun and (for hot-path candidates) the scale-invariance
  probe; add a `RegressedKernel` entry with ratio, source tag and reason. The
  policy test suite iterates both registries, so any edit is checked
  automatically (`cargo test`, see `src/kernel_policy.rs#tests`).

Everything stays uncommitted until the orchestrator decides; see
`docs/OPTIMIZATION_ROADMAP.md` §2 for the sibling work items.
