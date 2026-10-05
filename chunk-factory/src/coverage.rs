//! Tiny reporting types for the fallback law (mandate invariant I8):
//! native coverage % is ALWAYS reported, never implied. Every pregeneration
//! run -- and every live batch, once ladders exist -- emits one of these and
//! logs its `summary()` line.
//!
//! Rendering decision: `summary()` (process gate mode via `gate::mode()`)
//! is THE canonical one-line report for logs; `summary_in(mode)` is its
//! pure twin taking the mode explicitly (used by tests and by callers that
//! already hold the mode -- it never reads env). `summary` delegates to
//! `summary_in`, so the format string lives in exactly one place.

use crate::gate::{coverage_ratio, mode, PolicyMode};

/// Native-vs-Java coverage tally for a batch of generated chunks.
///
/// Invariants the REPORTER is expected to maintain (the type is a plain
/// data holder; fields are public for telemetry wiring):
/// `native_chunks + java_fallback_chunks == total_chunks`, and the
/// `by_step` rows are `(chunk_status_lane, native_chunks,
/// java_fallback_chunks)` tallies per lane. A chunk that fell back because
/// of ANY unsupported content (uncovered structure/feature, blending,
/// foreign BlockPopulator) counts as `java_fallback` -- per the fallback
/// law (I8) it is Java's chunk, not a partial native one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoverageReport {
    /// Chunks requested in the batch (native + fallback).
    pub total_chunks: u64,
    /// Chunks produced by the Rust factory (gate-allowed lanes only).
    pub native_chunks: u64,
    /// Chunks produced by the vanilla Java path (fallback law applies).
    pub java_fallback_chunks: u64,
    /// Per-lane breakdown: `(lane, native_chunks, java_fallback_chunks)`.
    pub by_step: Vec<(&'static str, u64, u64)>,
}

impl CoverageReport {
    /// Native coverage percentage, `0.0..=100.0`. Fraction form is
    /// `gate::coverage_ratio`; this is the report-level form used by the
    /// summary line. A zero-total report is `0.0` -- never NaN, never a
    /// silent 100% claim about an empty batch (I8: report, don't imply).
    pub fn coverage_pct(&self) -> f64 {
        coverage_ratio(self.native_chunks as usize, self.total_chunks as usize) * 100.0
    }

    /// One-line renderer under an explicit gate mode (pure; test-friendly,
    /// no env read). Example:
    /// `native 1234/20000 (6.17%), fallback 18766, gate=Strict`
    pub fn summary_in(&self, gate: PolicyMode) -> String {
        format!(
            "native {}/{} ({:.2}%), fallback {}, gate={:?}",
            self.native_chunks,
            self.total_chunks,
            self.coverage_pct(),
            self.java_fallback_chunks,
            gate
        )
    }

    /// One-line renderer using the process gate mode (`gate::mode()`).
    /// This is the string that must appear in run logs / reports next to
    /// every batch (invariant I8: coverage % always reported).
    pub fn summary(&self) -> String {
        self.summary_in(mode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_renders_one_line_with_coverage_and_gate() {
        let r = CoverageReport {
            total_chunks: 20_000,
            native_chunks: 1_234,
            java_fallback_chunks: 18_766,
            by_step: vec![("noise", 1_234, 18_766)],
        };
        assert_eq!(
            r.native_chunks + r.java_fallback_chunks,
            r.total_chunks,
            "reporter invariant: native + fallback == total"
        );
        let s = r.summary_in(PolicyMode::Strict);
        assert_eq!(s, "native 1234/20000 (6.17%), fallback 18766, gate=Strict");
        assert_eq!(s.lines().count(), 1, "summary must stay a single line");
    }

    #[test]
    fn empty_report_is_zero_not_nan() {
        // The zero-total case: 0/0 must render 0.00% and 0.0 pct, never
        // NaN and never an implied full-coverage claim.
        let r = CoverageReport::default();
        assert_eq!(r.coverage_pct(), 0.0);
        let s = r.summary_in(PolicyMode::Off);
        assert_eq!(s, "native 0/0 (0.00%), fallback 0, gate=Off");
        assert!(!s.contains("NaN"));
    }

    #[test]
    fn all_fallback_reported_honestly() {
        let r = CoverageReport {
            total_chunks: 512,
            native_chunks: 0,
            java_fallback_chunks: 512,
            by_step: vec![],
        };
        assert_eq!(r.coverage_pct(), 0.0);
        let s = r.summary_in(PolicyMode::Strict);
        assert_eq!(s, "native 0/512 (0.00%), fallback 512, gate=Strict");
    }

    #[test]
    fn coverage_pct_matches_summary_fraction() {
        let r = CoverageReport {
            total_chunks: 20_000,
            native_chunks: 1_234,
            java_fallback_chunks: 18_766,
            by_step: vec![],
        };
        assert!((r.coverage_pct() - 6.17).abs() < 1e-9);
        assert!((0.0..=100.0).contains(&r.coverage_pct()));

        let full = CoverageReport {
            total_chunks: 64,
            native_chunks: 64,
            java_fallback_chunks: 0,
            by_step: vec![],
        };
        assert_eq!(full.coverage_pct(), 100.0);
    }
}
