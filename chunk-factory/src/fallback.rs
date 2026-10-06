//! NCF P3.5 / P5.6 — the Java-fallback law (I8) as executable policy plus
//! coverage accounting. Any chunk whose content leaves the proven native
//! lane set is generated ENTIRELY by Java — never a partial mix.
//!
//! Lane status after session 6:
//!   native: NOISE + SURFACE + CARVERS (vanilla overworld, no structures,
//!           no features, no blending) — the staged zero-diff corpus.
//!   fallback: everything else, until its own zero-diff gate lands
//!           (features P4.x, light P4.6, structures P5.x, blending Q5).

use crate::coverage::CoverageReport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkDecision {
    pub native: bool,
    pub reason: FallbackReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FallbackReason {
    /// full native lane (currently: no features/structures/blending)
    Native,
    /// feature content present (Phase 4 not closed)
    Features,
    /// structure starts/references present (Phase 5 not closed)
    Structures,
    /// blending with an old world (Q5)
    Blending,
    /// foreign BlockPopulator / custom generator (Q4)
    ForeignPopulator,
    /// unsupported dimension/legacy random source
    UnsupportedSettings,
    /// unsupported carver ref in the biome's carver list
    UnsupportedCarver,
}

impl FallbackReason {
    pub fn as_str(self) -> &'static str {
        match self {
            FallbackReason::Native => "native",
            FallbackReason::Features => "features",
            FallbackReason::Structures => "structures",
            FallbackReason::Blending => "blending",
            FallbackReason::ForeignPopulator => "foreign_populator",
            FallbackReason::UnsupportedSettings => "unsupported_settings",
            FallbackReason::UnsupportedCarver => "unsupported_carver",
        }
    }
}

/// The decision function — pure, testable, and the ONLY place a chunk is
/// allowed to switch between Java and Rust (I9: callers additionally need a
/// gate/ledger promotion before native is even consulted).
pub fn decide_chunk(has_features: bool, has_structures: bool, blending: bool, foreign_populator: bool, unsupported_settings: bool, unsupported_carver: bool) -> ChunkDecision {
    if unsupported_settings {
        return ChunkDecision { native: false, reason: FallbackReason::UnsupportedSettings };
    }
    if blending {
        return ChunkDecision { native: false, reason: FallbackReason::Blending };
    }
    if foreign_populator {
        return ChunkDecision { native: false, reason: FallbackReason::ForeignPopulator };
    }
    if has_structures {
        return ChunkDecision { native: false, reason: FallbackReason::Structures };
    }
    if has_features {
        return ChunkDecision { native: false, reason: FallbackReason::Features };
    }
    if unsupported_carver {
        return ChunkDecision { native: false, reason: FallbackReason::UnsupportedCarver };
    }
    ChunkDecision { native: true, reason: FallbackReason::Native }
}

/// Rolling coverage tracker for a generation batch (P3.5: "метрика coverage %
/// обязательна в каждом отчёте").
#[derive(Default)]
pub struct CoverageLedger {
    pub report: CoverageReport,
    pub reasons: std::collections::HashMap<String, usize>,
}

impl CoverageLedger {
    pub fn record_native(&mut self) {
        self.report.native_chunks += 1;
    }

    pub fn record_fallback(&mut self, reason: FallbackReason) {
        self.report.java_fallback_chunks += 1;
        *self.reasons.entry(reason.as_str().to_string()).or_insert(0) += 1;
    }

    pub fn summary(&self) -> String {
        let mut s = self.report.summary();
        for (k, v) in &self.reasons {
            s.push_str(&format!("\n  fallback[{k}] = {v}"));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_lane_is_narrow() {
        let d = decide_chunk(false, false, false, false, false, false);
        assert!(d.native);
        assert_eq!(d.reason, FallbackReason::Native);
    }

    #[test]
    fn everything_else_falls_back() {
        assert!(!decide_chunk(true, false, false, false, false, false).native);
        assert_eq!(
            decide_chunk(true, false, false, false, false, false).reason,
            FallbackReason::Features
        );
        assert!(!decide_chunk(false, true, false, false, false, false).native);
        assert_eq!(
            decide_chunk(false, true, false, false, false, false).reason,
            FallbackReason::Structures
        );
        assert!(!decide_chunk(false, false, true, false, false, false).native);
        assert!(!decide_chunk(false, false, false, true, false, false).native);
        assert!(!decide_chunk(false, false, false, false, true, false).native);
        assert!(!decide_chunk(false, false, false, false, false, true).native);
    }

    #[test]
    fn structure_beats_feature_in_reason_precedence() {
        // The report should blame the FIRST blocker in the priority order
        // (structures before features — matches I8's listing order).
        let d = decide_chunk(true, true, false, false, false, false);
        assert!(!d.native);
        assert_eq!(d.reason, FallbackReason::Structures);
    }
}
