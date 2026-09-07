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
static slices scanned linearly — 4 + 11 entries):

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

* **live** wirings (verified on Purpur 1.21.10, must keep working):
  `PaperNativeAreaMap.nativeUpdateOpsBatch` (area_map hook, 64-rect self-test
  == naive set difference) and
  `PaperNativeImprovedNoise.nativeNoise` / `nativeBuildHandle` /
  `nativeFreeHandle` (improved_noise hot-patch v2, self-test PASSED,
  worklog session 003).
* **P500 WIN** verdicts (promotion candidates, wiring-eligible):
  `NoiseChunkBlendCache.newEmptyBlenderSummary` (244×),
  `NoiseInterpolatorSlice.flatSummary` (3.29×),
  `PluginLoadingAllocation.newLazyValidateSummary` (1.55×),
  `PluginLoadingAllocation.newLazyMissingSetSummary` (1.53×),
  `ImprovedNoiseInline.switchGradientSummary` (1.22×),
  `PalettedReencodeScratch.scratchThreadLocalSummary` (1.20×),
  `AquiferSurfaceSampling.newBatchSummary` (1.15×) —
  evidence: `bench/p500/results/P500_REPORT_v2.md`.

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
* **Demote to `DO_NOT_WIRE`**: a **REGRESSION** verdict (ratio ≥ 1.18)
  confirmed by a rerun and (for hot-path candidates) the scale-invariance
  probe; add a `RegressedKernel` entry with ratio, source tag and reason. The
  policy test suite iterates both registries, so any edit is checked
  automatically (`cargo test`, see `src/kernel_policy.rs#tests`).

Everything stays uncommitted until the orchestrator decides; see
`docs/OPTIMIZATION_ROADMAP.md` §2 for the sibling work items.
