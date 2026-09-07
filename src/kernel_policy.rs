//! Kernel selection policy — the enforced do-not-wire gate.
//!
//! P500 (`bench/p500/aggregate_p500.py`) measures every native kernel
//! (`alt`) against its paired old Java-side kernel (`old`), ratio = alt/old,
//! and classifies: WIN <= 0.85, REGRESSION >= 1.18, parity in between.
//! Four kernels are CONFIRMED genuine, scale-invariant regressions
//! (`bench/p500/results/P500_SCALING.md`): wiring them into a hot path would
//! slow the kernel down. Until now that knowledge lived only in prose
//! (README, docs/OPTIMIZATION_ROADMAP.md); this module makes it ENFORCED
//! infrastructure: every future wiring site must ask `decide()` before
//! routing a hot path to a native kernel, and the policy refuses anything
//! that is not explicitly proven.
//!
//! Policy (strict, the default):
//!   * kernel is in `DO_NOT_WIRE`            -> KeepJava (refused, with reason)
//!   * kernel is in `PROVEN_WINS`            -> Allow    (live-verified or P500 WIN)
//!   * anything else (unknown / unproven)    -> KeepJava ("not proven")
//!
//! The policy never changes CURRENT behavior: all live wirings (area_map,
//! improved_noise) are whitelisted in `PROVEN_WINS`, and the full 283-native
//! surface injection in `lib.rs` is untouched (registration is NOT wiring —
//! a registered-but-unrouted kernel is callable, nothing hot-path routes to
//! it; the 4 regressed kernels stay exactly there: registered, never routed).
//!
//! Env override `CRUSSTY_KERNEL_POLICY` (read once, cached):
//!   * `strict` (default) — enforce, silent.
//!   * `audit`            — enforce, plus log every decision (registration
//!                          chokepoint + `audit_wire` call sites). Never
//!                          crashes: a refusal is a returned Decision, the
//!                          caller keeps the Java path.
//!   * `off`              — DANGEROUS: gate bypassed (decide() always
//!                          Allow). Only for A/B benchmarking of a suspect
//!                          registry entry; never on a live server.
//!   * any other value    — treated as `strict` (fail-safe).
//!
//! Registry lookups are allocation-free and lock-free: static slices,
//! linear scan over a handful of entries, mode cached in a `OnceLock`.

use std::sync::OnceLock;

/// P500 verdict thresholds, mirrored from `bench/p500/aggregate_p500.py`
/// (WIN_MAX = 0.85, REG_MIN = 1.18). Documented here so the policy and the
/// aggregator cannot drift apart silently — the aggregator stays the source
/// of truth; these exist for prose reference in decisions/docs only.
#[allow(dead_code)] // doc/test-facing mirror of the aggregator thresholds
pub const WIN_MAX: f64 = 0.85;
#[allow(dead_code)] // doc/test-facing mirror of the aggregator thresholds
pub const REG_MIN: f64 = 1.18;

/// Env var consulted (once) for the policy mode.
const MODE_ENV: &str = "CRUSSTY_KERNEL_POLICY";

/// Log prefix shared with the rest of the plugin (`lib.rs` et al.).
const LOG_PREFIX: &str = "[crussty-plugin] kernel-policy:";

/// How the gate behaves. Default `Strict`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyMode {
    /// Enforce silently (default).
    Strict,
    /// Enforce AND log every decision (experiments; never crashes).
    Audit,
    /// DANGEROUS: bypass the gate entirely (A/B benchmarking only).
    Off,
}

/// Parse a mode from a raw env value. `None`/empty/unrecognized -> Strict
/// (fail-safe: an unknown value must never widen the gate).
fn parse_mode(raw: Option<&str>) -> PolicyMode {
    match raw.map(|v| v.trim().to_ascii_lowercase()) {
        Some(v) if v == "off" => PolicyMode::Off,
        Some(v) if v == "audit" => PolicyMode::Audit,
        Some(v) if v == "strict" => PolicyMode::Strict,
        // unset, empty, garbage -> strict
        _ => PolicyMode::Strict,
    }
}

/// The process-wide mode, read from `CRUSSTY_KERNEL_POLICY` exactly once.
pub fn mode() -> PolicyMode {
    static MODE: OnceLock<PolicyMode> = OnceLock::new();
    *MODE.get_or_init(|| {
        let m = parse_mode(std::env::var(MODE_ENV).ok().as_deref());
        if m == PolicyMode::Off {
            eprintln!(
                "{LOG_PREFIX} CRUSSTY_KERNEL_POLICY=off — do-not-wire gate BYPASSED \
                 (dangerous: A/B benchmarking only, never run live)"
            );
        } else if m == PolicyMode::Audit {
            eprintln!("{LOG_PREFIX} audit mode: decisions are logged (still enforced)");
        }
        m
    })
}

/// A known-regressed kernel: measured slower than its paired old kernel,
/// scale-invariantly (P500 SCALING probe), therefore NEVER to be routed into
/// a hot path. `ratio` is alt/old (>1 means the native alt kernel is slower).
#[allow(dead_code)] // registry metadata is self-documenting; fields read by tests/audit
pub struct RegressedKernel {
    /// Short bridge class name (the P500 report form, e.g.
    /// `PaperNativeMarkerCache`; full internal names are normalized to this).
    pub class: &'static str,
    /// Kernel (native method) name, e.g. `cachedSummary`.
    pub kernel: &'static str,
    /// The paired old kernel it lost to.
    pub paired_old: &'static str,
    /// Measured alt/old ratio (>1 = slower than old Java logic).
    pub ratio: f64,
    /// Where the measurement comes from.
    pub source: &'static str,
    /// Short human reason for the refusal.
    pub reason: &'static str,
}

/// Registry of known-regressed kernels (the enforced do-not-wire list).
/// Ratios from the canonical P500 full rerun (report generated
/// 2026-09-07T16:46Z, a.k.a. 2026-09-08 in UTC+8 logs; 49 groups / 70 pairs
/// / 0 CRASH) — re-verified entry-by-entry by the TASK-31 evidence-sync
/// (docs/PROVEN_WINS_SYNC.md); scale-invariance was
/// proven separately (`bench/p500/results/P500_SCALING.md`, N=16/256/4096).
/// All four are part of the REGISTERED native surface (callable through
/// their bridge classes) and must STAY unwired: no hot path may route to
/// them. Append here when the aggregator promotes a REGRESSION verdict
/// (ratio >= REG_MIN = 1.18) confirmed by a rerun.
pub static DO_NOT_WIRE: &[RegressedKernel] = &[
    RegressedKernel {
        class: "PaperNativeLevelChunkHeightmap",
        kernel: "newCombinedUpdateSummary",
        paired_old: "oldFourUpdateSummary",
        ratio: 5.70,
        source: "P500 2026-09-08 full rerun (49 groups, 70 pairs)",
        reason: "combined single-call update loses to the old four-call path at every N (scale-invariant); see bench/p500/results/P500_SCALING.md",
    },
    RegressedKernel {
        class: "PaperNativeMarkerCache",
        kernel: "cachedSummary",
        paired_old: "oldSummary",
        ratio: 4.54,
        source: "P500 2026-09-08 full rerun (49 groups, 70 pairs)",
        reason: "per-call cache bookkeeping dominates; old cache-free summary wins scale-invariantly (P500_SCALING)",
    },
    RegressedKernel {
        class: "PaperNativePalettedReencodeScratch",
        kernel: "directPackedSummary",
        paired_old: "oldNewArraySummary",
        ratio: 2.35,
        source: "P500 2026-09-08 full rerun (49 groups, 70 pairs)",
        reason: "packed-direct shape loses to the old array path at every N (scale-invariant, P500_SCALING)",
    },
    RegressedKernel {
        class: "PaperNativeProtoChunkHeightmap",
        kernel: "newCachedContainsSummary",
        paired_old: "oldEnumSetForeachSummary",
        ratio: 1.78,
        source: "P500 2026-09-08 full rerun (49 groups, 70 pairs)",
        reason: "cached-contains shape loses to the old EnumSet foreach at every N (scale-invariant, P500_SCALING)",
    },
];

/// A kernel the policy explicitly ALLOWS for hot-path routing, with the
/// evidence that earned it. Two kinds (`verdict`):
///   * "live"  — already wired & live-verified on a real server (the
///               wirings that exist today; must keep working).
///   * "P500 WIN" — genuine win verdict (ratio <= WIN_MAX = 0.85) from the
///               P500 report: wiring-eligible, promotion candidate.
///   * "P500 PARITY (...)" — reclassified/parity entry (TASK-31 evidence
///               sync): still Allow (batch-dispatch infrastructure), but
///               NOT a hot-path swap/promotion candidate.
#[allow(dead_code)] // registry metadata is self-documenting; fields read by tests/audit
pub struct ProvenKernel {
    pub class: &'static str,
    pub kernel: &'static str,
    /// "live" or "P500 WIN (ratio 244x)" etc.
    pub verdict: &'static str,
    /// Evidence pointer.
    pub evidence: &'static str,
}

/// Whitelist of proven kernels. The two live wirings MUST stay listed —
/// `improved_noise.rs` debug-asserts its entry before arming the patch.
pub static PROVEN_WINS: &[ProvenKernel] = &[
    // --- live wirings (registered, routed, verified on Purpur 1.21.10) ---
    ProvenKernel {
        class: "PaperNativeAreaMap",
        kernel: "nativeUpdateOpsBatch",
        verdict: "live",
        evidence: "area_map hook: patch 5075->3320 bytes, 64-rect self-test == naive set difference (docs/ARCHITECTURE.md)",
    },
    ProvenKernel {
        class: "PaperNativeImprovedNoise",
        kernel: "nativeNoise",
        verdict: "live",
        evidence: "improved_noise hot-patch v2: self-test PASSED on live Purpur 1.21.10 (worklog session 003)",
    },
    ProvenKernel {
        class: "PaperNativeImprovedNoise",
        kernel: "nativeBuildHandle",
        verdict: "live",
        evidence: "improved_noise bridge: handle build/sample/free round-trip self-test on live server",
    },
    ProvenKernel {
        class: "PaperNativeImprovedNoise",
        kernel: "nativeFreeHandle",
        verdict: "live",
        evidence: "improved_noise bridge: handle free verified by self-test + Cleaner lifecycle",
    },
    // --- P500 WIN verdicts (promotion candidates, wire-eligible) ------------
    // Evidence synced to the canonical 2026-09-08 full rerun
    // (bench/p500/results/P500_REPORT.md) by TASK-31; see
    // docs/PROVEN_WINS_SYNC.md. 4 of the 7 previous v2 WIN verdicts
    // reproduce here; the 3 that did not are reclassified PARITY below.
    ProvenKernel {
        class: "PaperNativeNoiseChunkBlendCache",
        kernel: "newEmptyBlenderSummary",
        verdict: "P500 WIN (316x)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (95.3 us -> 301.1 ns, ratio 0.003, stability 0.0%); v2 said 244x (67.0 us -> 274.5 ns) — direction unchanged, number refreshed",
    },
    ProvenKernel {
        class: "PaperNativeNoiseInterpolatorSlice",
        kernel: "flatSummary",
        verdict: "P500 WIN (3.32x)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (6.3 ms -> 1.9 ms, ratio 0.301, stability 0.0%); v2 said 3.29x (6.2 ms)",
    },
    ProvenKernel {
        class: "PaperNativeImprovedNoiseInline",
        kernel: "switchGradientSummary",
        verdict: "P500 WIN (1.22x)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (9.3 us -> 7.6 us, ratio 0.819, stability 0.0%) — reproduced from v2",
    },
    ProvenKernel {
        class: "PaperNativePalettedReencodeScratch",
        kernel: "scratchThreadLocalSummary",
        verdict: "P500 WIN (1.20x)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (493.6 us -> 411.9 us, ratio 0.834, stability 0.0%) — reproduced from v2",
    },
    // --- P500 PARITY reclassifications (TASK-31 evidence-sync) --------------
    // Previously listed as "P500 WIN" on the v2 report; NOT reproducible on
    // the canonical 2026-09-08 rerun (ratio in the 0.8-1.25 parity band).
    // Kept inside PROVEN_WINS so the Allow set is UNCHANGED (no gate
    // behavior change; batch-dispatch eligibility per
    // docs/BATCH_WIRING_PLAN.md B.2.3 precondition (a) "parity or better"
    // still holds) — but they are parity kernels, NOT hot-path swap or
    // promotion candidates.
    ProvenKernel {
        class: "PaperNativePluginLoadingAllocation",
        kernel: "newLazyValidateSummary",
        verdict: "P500 PARITY (2026-09-08 rerun)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (116.1 ns -> 115.3 ns, ratio 0.993); v2 'WIN (1.55x) 217.9 ns -> 140.6 ns' did not reproduce",
    },
    ProvenKernel {
        class: "PaperNativePluginLoadingAllocation",
        kernel: "newLazyMissingSetSummary",
        verdict: "P500 PARITY (2026-09-08 rerun)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (115.1 ns -> 114.6 ns, ratio 0.996); v2 'WIN (1.53x) 218.3 ns -> 142.5 ns' did not reproduce",
    },
    ProvenKernel {
        class: "PaperNativeAquiferSurfaceSampling",
        kernel: "newBatchSummary",
        verdict: "P500 PARITY (2026-09-08 rerun)",
        evidence: "bench/p500/results/P500_REPORT.md 2026-09-08 rerun (6.0 us -> 5.5 us, ratio 0.906); v2 'WIN (1.15x) 6.3 us -> 5.5 us' did not reproduce",
    },
    // --- batch-dispatch surface (src/batch_table.rs, caller-initiated) -------
    // These 12 are the batch dispatcher's table: P500 PARITY-floor kernels
    // (neither wins nor regressions) whose bridge classes are ALREADY part of
    // the registered callable surface — any Java caller could invoke them
    // directly today. Batch dispatch executes the SAME registered kernel
    // function pointer, just with one Java->native transition for N ops; it
    // is infrastructure, not hot-path routing. They are listed here because
    // PROVEN_WINS is the SINGLE source of truth for what the batch gate may
    // execute: batch-table membership alone never grants allowance (a future
    // table edit adding a do-not-wire kernel is refused at dispatch time and
    // caught by the drift-guard tests below / in batch_api).
    ProvenKernel {
        class: "PaperNativeTicketSetSearch",
        kernel: "binarySummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id0 (jni_table.rs:15); registered surface, caller-initiated dispatch",
    },
    ProvenKernel {
        class: "PaperNativeTicketSetSearch",
        kernel: "uncheckedBinarySummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id1 (jni_table.rs:16)",
    },
    ProvenKernel {
        class: "PaperNativeAquiferIndexStride",
        kernel: "oldBatchSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id2 (jni_table.rs:128)",
    },
    ProvenKernel {
        class: "PaperNativeAquiferIndexStride",
        kernel: "newBatchSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id3 (jni_table.rs:129); parity pair (1.07x on the 2026-09-08 rerun; the old '1.15x-win stem family' note was a stale v2 reference — TASK-31 sync)",
    },
    ProvenKernel {
        class: "PaperNativeChunkDependencies",
        kernel: "oldImmutableListSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id4 (jni_table.rs:175)",
    },
    ProvenKernel {
        class: "PaperNativeChunkDependencies",
        kernel: "arraySummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id5 (jni_table.rs:176)",
    },
    ProvenKernel {
        class: "PaperNativeDensitySplineContext",
        kernel: "oldWrapperSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id6 (jni_table.rs:30)",
    },
    ProvenKernel {
        class: "PaperNativeDensitySplineContext",
        kernel: "newDirectSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id7 (jni_table.rs:31); parity (1.00x, 2026-09-08 rerun); the 3.29x win note belonged to NoiseInterpolatorSlice.flatSummary (3.32x on the rerun, different class) — TASK-31 sync",
    },
    ProvenKernel {
        class: "PaperNativeEntityLookupStatus",
        kernel: "oldStatusSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id8 (jni_table.rs:242)",
    },
    ProvenKernel {
        class: "PaperNativeNoiseInterpolatorFractions",
        kernel: "divisionSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id9 (jni_table.rs:155)",
    },
    ProvenKernel {
        class: "PaperNativeClimateRTree",
        kernel: "buildTreeHandle",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id10 (jni_table.rs:89), shape B ([J[J)J",
    },
    ProvenKernel {
        class: "net/minecraft/world/level/biome/PaperNativeClimateRTree",
        kernel: "nativeBuildTreeHandle",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id11 (jni_table.rs:92), shape B ([J[J)J",
    },
    // TASK-48 wave-1 shape A' (III[J)I): the P500 g9 density min/max-fill
    // pair. Honest framing mirrors ids 2/3: the pair's old-vs-new ratio is
    // PARITY (1.003x, canonical 2026-09-08 rerun — the wave-2 matrix's
    // "19.2x" was a BATCH-dispatch projection, not a kernel win); batch
    // membership here is about amortizing the ~120ns direct JNI transition,
    // gated per-site later (runbook G4/G5).
    ProvenKernel {
        class: "PaperNativeDensityAp2MinMaxFill",
        kernel: "oldSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id12 (jni_table.rs:173), shape A' (III[J)I; direct 119.8 ns (P500_REPORT.md §9)",
    },
    ProvenKernel {
        class: "PaperNativeDensityAp2MinMaxFill",
        kernel: "newSummary",
        verdict: "P500 PARITY (batch surface)",
        evidence: "src/batch_table.rs id13 (jni_table.rs:174), shape A' (III[J)I; parity pair 1.003x on the 2026-09-08 rerun, TASK-31 sync",
    },
];

/// The policy verdict for one kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Route to the native kernel (explicitly proven: live or P500 WIN).
    Allow,
    /// Keep the Java-side logic. `reason` says why (regressed / unproven).
    KeepJava { reason: &'static str },
}

impl Decision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Decision::Allow)
    }
}

/// Normalize a class name to the registry's short form: strip any package
/// path (full internal names use '/', e.g.
/// `net/minecraft/.../PaperNativeImprovedNoise`). No allocation.
fn short_class(class: &str) -> &str {
    match class.rfind('/') {
        Some(i) => &class[i + 1..],
        None => class,
    }
}

/// Look a kernel up in the do-not-wire registry. Exact (short-class, method)
/// match; allocation-free linear scan.
pub fn do_not_wire_entry(class: &str, kernel: &str) -> Option<&'static RegressedKernel> {
    let class = short_class(class);
    DO_NOT_WIRE
        .iter()
        .find(|r| r.class == class && r.kernel == kernel)
}

/// Look a kernel up in the proven/whitelist registry. Registry entries may
/// be written in either short or full internal form — BOTH sides are
/// normalized to the short form before comparing (the batch table carries a
/// full `net/minecraft/...` entry).
pub fn proven_entry(class: &str, kernel: &str) -> Option<&'static ProvenKernel> {
    let class = short_class(class);
    PROVEN_WINS
        .iter()
        .find(|p| short_class(p.class) == class && p.kernel == kernel)
}

/// The decision function under an explicit mode (pure; used by tests and by
/// `decide`). No allocation, no locks (mode is passed in).
pub fn decide_in(mode: PolicyMode, class: &str, kernel: &str) -> Decision {
    if mode == PolicyMode::Off {
        // Documented dangerous bypass (A/B benchmarking only).
        return Decision::Allow;
    }
    if let Some(r) = do_not_wire_entry(class, kernel) {
        return Decision::KeepJava {
            reason: r.reason,
        };
    }
    if proven_entry(class, kernel).is_some() {
        return Decision::Allow;
    }
    // Default-safe: unknown / unproven kernels stay on the Java side until a
    // P500 WIN verdict + live-server verification promote them.
    Decision::KeepJava {
        reason: "kernel is not in PROVEN_WINS (no P500 WIN verdict / live verification) — default-safe KeepJava",
    }
}

/// The decision function (uses the cached env mode).
/// `class` may be the short bridge class name (`PaperNativeMarkerCache`) or a
/// full internal name (`net/minecraft/.../PaperNativeImprovedNoise`).
pub fn decide(class: &str, kernel: &str) -> Decision {
    decide_in(mode(), class, kernel)
}

/// Convenience form taking a `Class.method` id (the P500 report form).
/// A bare method name (no '.') counts as unknown class -> unproven.
#[allow(dead_code)] // convenience API for future wiring sites (id form)
pub fn decide_id(id: &str) -> Decision {
    match id.rsplit_once('.') {
        Some((class, kernel)) => decide(class, kernel),
        None => decide("", id),
    }
}

/// Audit hook for WIRING sites (hot-path routing decisions). Call where a
/// hot path starts routing to a native kernel: in `Audit` mode logs the
/// decision; otherwise silent. Never panics. Pair with a
/// `debug_assert!(decide(..).is_allowed(), ..)` if the wiring is meant to be
/// whitelisted — debug builds then catch an accidental demotion.
pub fn audit_wire(class: &str, kernel: &str, site: &str) {
    if mode() != PolicyMode::Audit {
        return;
    }
    let sc = short_class(class);
    match decide_in(PolicyMode::Audit, class, kernel) {
        Decision::Allow => {
            eprintln!("{LOG_PREFIX} WIRE {sc}.{kernel} at {site}: allowed (proven)")
        }
        Decision::KeepJava { reason } => {
            eprintln!("{LOG_PREFIX} WIRE {sc}.{kernel} at {site}: REFUSED — {reason}")
        }
    }
}

/// Audit hook for the SURFACE-registration chokepoint
/// (`lib.rs::define_and_register`, which every native flows through).
/// Registration is NOT wiring: a do-not-wire kernel may legally be
/// registered (callable surface) as long as nothing hot-path routes to it.
/// In `Audit` mode logs; otherwise silent; never changes behavior.
pub fn audit_registered(class: &str, kernel: &str) {
    if mode() != PolicyMode::Audit {
        return;
    }
    if do_not_wire_entry(class, kernel).is_some() {
        let sc = short_class(class);
        eprintln!(
            "{LOG_PREFIX} REGISTER {sc}.{kernel}: do-not-wire kernel — \
             registered as callable surface ONLY, never route a hot path to it"
        );
    }
}

// --- TASK-04: conservative surface binding -----------------------------------

/// Env var for the operator opt-in (read once, cached). Separate concern from
/// `CRUSSTY_KERNEL_POLICY` (which gates WIRING decisions): this one swaps the
/// IMPLEMENTATION POINTER of known-regressed bridge methods to their paired
/// old kernel at registration time.
const PREF_ENV: &str = "CRUSSTY_KERNEL_PREF";

/// Conservative surface binding is on when the operator sets
/// `CRUSSTY_KERNEL_PREF=old|conservative|safe|1`. Anything else (including
/// unset) keeps the native alt bindings — default behavior is unchanged.
fn conservative_pref() -> bool {
    static PREF: OnceLock<bool> = OnceLock::new();
    *PREF.get_or_init(|| {
        matches!(
            std::env::var(PREF_ENV).ok().as_deref(),
            Some("old") | Some("conservative") | Some("safe") | Some("1")
        )
    })
}

/// If conservative binding is on and (class, kernel) is a confirmed
/// regression, returns the `Java_*` symbol of the PAIRED OLD kernel — same
/// bridge method name, same signature, same semantics (it is the original
/// Java-side algorithm the alt kernel was meant to replace), just the
/// implementation that is actually faster. Callers of the bridge method can
/// never execute the slow implementation. `None` = keep the requested symbol.
///
/// The fallback symbol name is DERIVED from the registry entry
/// (`Java_{class}_{paired_old}`) and cross-checked against `jni_table` by a
/// unit test, so registry and table cannot drift apart silently.
pub fn registration_fallback(class: &str, kernel: &str) -> Option<String> {
    if !conservative_pref() {
        return None;
    }
    let r = do_not_wire_entry(class, kernel)?;
    Some(format!("Java_{}_{}", r.class, r.paired_old))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- registry integrity -------------------------------------------------

    #[test]
    fn do_not_wire_registry_has_the_four_p500_regressions() {
        let names: Vec<_> = DO_NOT_WIRE.iter().map(|r| (r.class, r.kernel)).collect();
        assert_eq!(
            names,
            [
                ("PaperNativeLevelChunkHeightmap", "newCombinedUpdateSummary"),
                ("PaperNativeMarkerCache", "cachedSummary"),
                ("PaperNativePalettedReencodeScratch", "directPackedSummary"),
                ("PaperNativeProtoChunkHeightmap", "newCachedContainsSummary"),
            ]
        );
        for r in DO_NOT_WIRE {
            assert!(r.ratio >= REG_MIN, "{}: ratio below REG_MIN", r.kernel);
            assert!(!r.source.is_empty() && !r.reason.is_empty());
            // A regressed kernel must never also be whitelisted.
            assert!(proven_entry(r.class, r.kernel).is_none());
        }
    }

    // --- core decisions (strict) ---------------------------------------------

    #[test]
    fn strict_refuses_known_regressed_kernels() {
        for r in DO_NOT_WIRE {
            let d = decide_in(PolicyMode::Strict, r.class, r.kernel);
            assert!(!d.is_allowed(), "{} must be refused", r.kernel);
            if let Decision::KeepJava { reason } = d {
                assert!(!reason.is_empty());
            }
            // Class name may come in full internal form too.
            let full = format!("some/pkg/{}", r.class);
            assert!(!decide_in(PolicyMode::Strict, &full, r.kernel).is_allowed());
        }
    }

    #[test]
    fn strict_allows_proven_and_live_kernels() {
        for p in PROVEN_WINS {
            assert!(
                decide_in(PolicyMode::Strict, p.class, p.kernel).is_allowed(),
                "{}.{} must be allowed",
                p.class,
                p.kernel
            );
        }
        // Spot-check the two live wirings explicitly.
        assert!(decide_in(PolicyMode::Strict, "PaperNativeAreaMap", "nativeUpdateOpsBatch").is_allowed());
        assert!(decide_in(
            PolicyMode::Strict,
            "net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise",
            "nativeNoise"
        )
        .is_allowed());
    }

    #[test]
    fn strict_defaults_unknown_kernels_to_keep_java() {
        let d = decide_in(PolicyMode::Strict, "PaperNativeSomeFutureKernel", "newFastSummary");
        assert!(!d.is_allowed());
        if let Decision::KeepJava { reason } = d {
            assert!(reason.contains("not in PROVEN_WINS"), "reason: {reason}");
        } else {
            panic!("expected KeepJava");
        }
        // Same-name-different-class collisions stay unproven unless the exact
        // (class, kernel) pair is whitelisted: `cachedSummary` is a REGRESSION
        // on MarkerCache but merely unproven anywhere else.
        assert!(!decide_in(PolicyMode::Strict, "PaperNativeSomeOther", "cachedSummary").is_allowed());
        assert!(!decide_in(PolicyMode::Strict, "", "cachedSummary").is_allowed());
    }

    // --- env override modes ---------------------------------------------------

    #[test]
    fn parse_mode_is_fail_safe() {
        assert_eq!(parse_mode(None), PolicyMode::Strict);
        assert_eq!(parse_mode(Some("")), PolicyMode::Strict);
        assert_eq!(parse_mode(Some("strict")), PolicyMode::Strict);
        assert_eq!(parse_mode(Some("audit")), PolicyMode::Audit);
        assert_eq!(parse_mode(Some("off")), PolicyMode::Off);
        // Robustness: whitespace + case.
        assert_eq!(parse_mode(Some("  AUDIT ")), PolicyMode::Audit);
        assert_eq!(parse_mode(Some("Off")), PolicyMode::Off);
        // Unknown values must never widen the gate.
        assert_eq!(parse_mode(Some("bogus")), PolicyMode::Strict);
        assert_eq!(parse_mode(Some("0")), PolicyMode::Strict);
        assert_eq!(parse_mode(Some("allow_all")), PolicyMode::Strict);
    }

    #[test]
    fn audit_mode_enforces_the_same_decisions_as_strict() {
        for r in DO_NOT_WIRE {
            assert!(!decide_in(PolicyMode::Audit, r.class, r.kernel).is_allowed());
        }
        assert!(decide_in(PolicyMode::Audit, "PaperNativeAreaMap", "nativeUpdateOpsBatch").is_allowed());
        assert!(!decide_in(PolicyMode::Audit, "PaperNativeUnknown", "newThing").is_allowed());
    }

    #[test]
    fn off_mode_bypasses_the_gate() {
        for r in DO_NOT_WIRE {
            assert!(decide_in(PolicyMode::Off, r.class, r.kernel).is_allowed());
        }
        assert!(decide_in(PolicyMode::Off, "PaperNativeUnknown", "newThing").is_allowed());
    }

    // --- id form --------------------------------------------------------------

    #[test]
    fn decide_id_splits_class_and_method() {
        assert!(decide_id("PaperNativeMarkerCache.cachedSummary").is_allowed() == false);
        assert!(decide_id("PaperNativeAreaMap.nativeUpdateOpsBatch").is_allowed());
        // Bare method name: no class -> unproven (default-safe).
        assert!(!decide_id("cachedSummary").is_allowed());
        // Full internal class name in the id is normalized.
        assert!(decide_id(
            "ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap.nativeUpdateOpsBatch"
        )
        .is_allowed());
    }

    // --- batch-dispatch surface drift guards ----------------------------------

    #[test]
    fn every_batch_table_kernel_is_policy_allowed() {
        // The shipped batch surface must be live: if this fails, every batch
        // referencing the kernel returns ERR_KERNEL_REFUSED and the surface
        // is dead. Add an honest PROVEN_WINS entry (with evidence) or remove
        // the kernel from batch_table — never special-case the gate.
        for k in crate::batch_table::BATCH_KERNELS {
            let d = decide_in(PolicyMode::Strict, k.class, k.method);
            assert!(
                d.is_allowed(),
                "batch kernel {}.{} refused by policy (batch surface dead)",
                k.class,
                k.method
            );
        }
    }

    #[test]
    fn batch_table_never_carries_a_do_not_wire_kernel() {
        for r in DO_NOT_WIRE {
            for k in crate::batch_table::BATCH_KERNELS {
                let same =
                    k.class == r.class || short_class(k.class) == r.class;
                assert!(
                    !(same && k.method == r.kernel),
                    "batch table id {} carries do-not-wire kernel {}.{}",
                    k.id,
                    r.class,
                    r.kernel
                );
            }
        }
    }

    #[test]
    fn proven_wins_has_no_duplicate_class_kernel_pairs() {
        // The registry is scanned linearly; a duplicate would shadow evidence
        // and make audits lie. Cheap O(n^2) guard over a ~30-entry list.
        let mut pairs: Vec<(&str, &str)> =
            PROVEN_WINS.iter().map(|p| (p.class, p.kernel)).collect();
        let n = pairs.len();
        pairs.sort_unstable();
        pairs.dedup();
        assert_eq!(pairs.len(), n, "duplicate (class, kernel) in PROVEN_WINS");
    }

    #[test]
    fn fallback_symbols_exist_in_jni_table_with_matching_sigs() {
        // Registry <-> table drift guard: every DO_NOT_WIRE pair must exist
        // in the injected surface with the SAME signature on both sides, and
        // the derived `Java_{class}_{paired_old}` fallback symbol must be the
        // real symbol jni_table registers for the old kernel.
        for r in DO_NOT_WIRE {
            let alt = crate::jni_table::MAIN_JNI_TABLE
                .iter()
                .find(|e| e.class == r.class && e.method == r.kernel)
                .unwrap_or_else(|| panic!("{}:{} missing from jni_table", r.class, r.kernel));
            let old = crate::jni_table::MAIN_JNI_TABLE
                .iter()
                .find(|e| e.class == r.class && e.method == r.paired_old)
                .unwrap_or_else(|| panic!("{}:{} missing from jni_table", r.class, r.paired_old));
            assert_eq!(alt.sig, old.sig, "{}: sig drift between alt and old", r.class);
            assert_eq!(
                old.symbol,
                format!("Java_{}_{}", r.class, r.paired_old),
                "{}: fallback symbol derivation mismatch",
                r.class
            );
        }
    }

    // --- decision must not regress the live wiring contract -------------------

    #[test]
    fn default_mode_never_breaks_the_two_live_wirings() {
        // If the real env is unset (CI default) mode() is Strict; the live
        // wirings must be Allow there. (We do not mutate the real env in
        // tests; Off/Audit behavior is covered by the *_in tests above.)
        if mode() == PolicyMode::Strict {
            assert!(decide("PaperNativeAreaMap", "nativeUpdateOpsBatch").is_allowed());
            assert!(decide("PaperNativeImprovedNoise", "nativeNoise").is_allowed());
        }
    }
}
