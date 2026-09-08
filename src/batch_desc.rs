//! Batch descriptor parser — the wire-v3 port of the P500 descriptor grammar
//! (`bench/p500/gen_p500_bench.py::parse_params`, lines 48-66: dims `[`*,
//! primitive char, `L...;` objects), classified into the BATCH_API_PROPOSAL
//! encoding.
//!
//! # Why this module exists (docs/BATCH_API_PROPOSAL.md §4/§5)
//!
//! §4 requires the batch table's `scalars`/`refs` widths to be "precomputed
//! by porting the descriptor parser from
//! `bench/p500/gen_p500_bench.py::parse_params` (same grammar) ..., unit-tested
//! against all 49 group signatures in `bench/p500/java/p500/groups.tsv`".
//! This module IS that port: [`parse_sig`] classifies every parameter of a
//! JNI method descriptor into a [`ParamKind`], and [`slots`]/[`input_refs`]
//! derive the plane widths from the classification. The batch-table
//! cross-check (the `batch_api::resolve_fns` fail-closed path and the
//! `batch_table`/`batch_desc` tests) makes it LOAD-BEARING: init fails closed
//! if a table row's descriptor ever disagrees with its declared shape.
//!
//! # §5 encoding classification
//!
//! | descriptor param | [`ParamKind`] | scalar-plane slots | ref-plane slots |
//! |---|---|---:|---:|
//! | `I`/`Z`/`B`/`S`/`C` | [`ParamKind::JIntNarrow`] | 1 (sign-extended long, narrowed to `jint` at the C boundary) | 0 |
//! | `J` | [`ParamKind::JLong`] | 1 (raw) | 0 |
//! | `F` | [`ParamKind::JFloatBits`] | 1 (raw bits) | 0 |
//! | `D` | [`ParamKind::JDoubleBits`] | 1 (raw bits) | 0 |
//! | `[X` (any dims) / `L...;` | [`ParamKind::Ref`] | 0 | 1 (ZERO-COPY per proposal §5: "reference stored in `refArgs`, consumed by the kernel via JNI array access — no copy") |
//!
//! The current batch kernels only exercise `JIntNarrow` + `Ref` scalars, but
//! the full classification is pinned here so future shapes inherit the §5
//! rules without re-deriving them.
//!
//! # P500 dst convention (the trailing `[J`)
//!
//! Per the P500 generator rule ("the LAST `[J` of a descriptor is the
//! summary/dst array", `gen_p500_bench.py` header; restated in proposal §5),
//! a trailing array parameter is the kernel's summary/dst — it is the
//! dispatcher's shared `out_arr` scratch and NEVER rides the ref plane.
//! [`input_refs`] therefore subtracts one ref slot iff the last parameter is
//! a [`ParamKind::Ref`]: that slot is the dst, the rest are true inputs.
//!
//! All 49 group signatures in `bench/p500/java/p500/groups.tsv` are
//! unit-tested to parse below (the §4 requirement quoted above).

/// Encoding classification of one descriptor parameter (BATCH_API_PROPOSAL
/// §5 table; see the module docs for the slot arithmetic).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamKind {
    /// `I`/`Z`/`B`/`S`/`C` — one scalar-plane slot, sign-extended long on the
    /// wire, narrowed to `jint` at the C ABI boundary.
    JIntNarrow,
    /// `J` — one scalar-plane slot, raw `jlong`.
    JLong,
    /// `F` — one scalar-plane slot, raw `Float.floatToRawIntBits` bits.
    JFloatBits,
    /// `D` — one scalar-plane slot, raw `Double.doubleToRawLongBits` bits.
    JDoubleBits,
    /// Any array (`[`*) or object (`L...;`) — one ref-plane slot, passed
    /// ZERO-COPY (proposal §5).
    Ref,
}

/// Return-type classification of a descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetKind {
    /// `I` — jint return (count-carried shapes A/A′/C/D/E/F).
    Int,
    /// `J` — jlong return (shape B's handle).
    Long,
    /// `D` — jdouble return (raw bits ride the scalar plane when carried).
    Double,
    /// `V` — void.
    Void,
    /// `Z` — jboolean (0/1; reserved shape Z).
    Bool,
}

/// Parsed JNI method descriptor: classified parameters + return kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Desc {
    /// One [`ParamKind`] per parameter, in descriptor order.
    #[allow(dead_code)] // read via slots()/input_refs() and the encoding tests
    pub params: Vec<ParamKind>,
    /// Return-type classification.
    #[allow(dead_code)] // registry metadata: pinned by the encoding tests
    pub ret: RetKind,
}

/// Plane-slot counts of a parsed descriptor (proposal §5 stride arithmetic).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotCounts {
    /// Scalar-plane long slots per op (non-[`ParamKind::Ref`] params).
    pub scalars: usize,
    /// Ref-plane slots per op ([`ParamKind::Ref`] params, INCLUDING any
    /// trailing dst array — see [`input_refs`] for the input-only count).
    pub refs: usize,
}

/// Parse a JNI method descriptor into its [`ParamKind`] classification.
///
/// Same grammar as `bench/p500/gen_p500_bench.py::parse_params` (dims `[`*,
/// primitive char, `L...;` objects) plus the return type — but panic-free,
/// bounded (every loop advance is driven by the input length; the only
/// allocation is the result `Vec`, bounded by the descriptor length), and
/// FAIL-CLOSED: any malformed descriptor is an `Err`, never a guess.
///
/// Error paths (all pinned by the tests): empty input, missing `(`,
/// unterminated parameter list, a parameter char outside the primitive set
/// that is not an object `L...;`, an unterminated object class (no `;`), an
/// empty class name (`L;`) or one containing whitespace/control characters
/// (e.g. `(Lno Semi;)I` — illegal in a JVM binary name), a missing/invalid
/// return type, or trailing garbage after the return type.
pub fn parse_sig(sig: &str) -> Result<Desc, String> {
    let b = sig.as_bytes();
    if b.is_empty() {
        return Err("empty descriptor".to_string());
    }
    if b[0] != b'(' {
        return Err(format!("descriptor must start with '(': {sig:?}"));
    }
    let mut i = 1usize;
    let mut params = Vec::new();
    let mut closed = false;
    while i < b.len() {
        if b[i] == b')' {
            i += 1;
            closed = true;
            break;
        }
        // dims: `[`* (bounded by the input length)
        let mut dims = 0usize;
        while i < b.len() && b[i] == b'[' {
            dims += 1;
            i += 1;
        }
        if i >= b.len() {
            return Err(format!("unterminated descriptor (dangling '[' dims): {sig:?}"));
        }
        let c = b[i];
        i += 1;
        if c == b'L' {
            // Object class: everything up to the mandatory ';'.
            let Some(rel) = sig[i..].find(';') else {
                return Err(format!("object parameter without ';': {sig:?}"));
            };
            let name = &sig[i..i + rel];
            if name.is_empty() {
                return Err(format!("empty object class name: {sig:?}"));
            }
            if name.bytes().any(|b| b <= b' ') {
                // Whitespace/control characters are illegal in a JVM binary
                // name — fail closed instead of accepting a mangled class
                // (e.g. `(Lno Semi;)I`).
                return Err(format!("illegal character in object class name: {sig:?}"));
            }
            i += rel + 1;
            params.push(ParamKind::Ref);
        } else {
            let kind = match c {
                b'I' | b'Z' | b'B' | b'S' | b'C' => ParamKind::JIntNarrow,
                b'J' => ParamKind::JLong,
                b'F' => ParamKind::JFloatBits,
                b'D' => ParamKind::JDoubleBits,
                _ => return Err(format!("malformed parameter char {:?}: {sig:?}", c as char)),
            };
            params.push(if dims > 0 { ParamKind::Ref } else { kind });
        }
    }
    if !closed {
        return Err(format!("unterminated parameter list: {sig:?}"));
    }
    if i >= b.len() {
        return Err(format!("missing return type: {sig:?}"));
    }
    let ret = match b[i] {
        b'I' => RetKind::Int,
        b'J' => RetKind::Long,
        b'D' => RetKind::Double,
        b'V' => RetKind::Void,
        b'Z' => RetKind::Bool,
        _ => return Err(format!("malformed return type {:?}: {sig:?}", b[i] as char)),
    };
    if i + 1 != b.len() {
        return Err(format!("trailing garbage after return type: {sig:?}"));
    }
    Ok(Desc { params, ret })
}

/// Scalar/ref plane-slot counts of a parsed descriptor (proposal §5).
///
/// `scalars` = non-[`ParamKind::Ref`] params (one long slot each);
/// `refs` = [`ParamKind::Ref`] params (one ref slot each, INCLUDING the
/// trailing dst — [`input_refs`] is the input-only count).
pub fn slots(desc: &Desc) -> SlotCounts {
    let mut scalars = 0usize;
    let mut refs = 0usize;
    for p in &desc.params {
        if *p == ParamKind::Ref {
            refs += 1;
        } else {
            scalars += 1;
        }
    }
    SlotCounts { scalars, refs }
}

/// INPUT ref-plane slots of a parsed descriptor: [`slots`] refs minus one iff
/// the LAST parameter is a [`ParamKind::Ref`] (the P500 trailing-`[J`
/// summary/dst convention — see the module docs; the dst is the dispatcher's
/// shared `out_arr` scratch and never rides the ref plane).
///
/// This is the count the batch-table cross-check compares against the
/// shape's declared input-ref width: it equals the ref-plane width for the
/// ref-plane-native shapes (D/E/F), 0 for the dst-only shapes (A/Z/A′), and
/// 1 for the args1-native shapes B/C (whose single ref input rides the v2
/// packed arena, not the ref plane — `batch_table::Shape::refs` documents
/// the wire distinction).
pub fn input_refs(desc: &Desc) -> usize {
    let SlotCounts { scalars: _, refs } = slots(desc);
    match desc.params.last() {
        Some(ParamKind::Ref) => refs.saturating_sub(1),
        _ => refs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch_table::{Shape, BATCH_KERNELS};

    /// (a) §4 requirement verbatim: the parser is "unit-tested against all 49
    /// group signatures in bench/p500/java/p500/groups.tsv" — every sig column
    /// must parse Ok (embedded via include_str! so the test breaks loudly if
    /// the TSV moves or a new group introduces an unparseable descriptor).
    #[test]
    fn all_49_group_signatures_parse() {
        let tsv = include_str!("../bench/p500/java/p500/groups.tsv");
        let mut parsed = 0usize;
        for line in tsv.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            assert!(cols.len() >= 2, "groups.tsv line shape: {line:?}");
            let sig = cols[1];
            match parse_sig(sig) {
                Ok(d) => parsed += 1,
                Err(e) => panic!("groups.tsv sig {sig:?} failed to parse: {e}"),
            }
        }
        assert_eq!(parsed, 49, "expected all 49 group signatures to parse");
    }

    /// (b) Table cross-check: every batch-table kernel's descriptor parses to
    /// exactly the shape's plane widths. This is the same rule the
    /// `batch_api::resolve_fns` fail-closed path enforces at init (the parser
    /// is load-bearing, not decorative). Per-shape truth:
    ///   - A/Z `(I[J)[I` — scalars 1, refs 1 (trailing dst), input refs 0;
    ///   - A′ `(III[J)I` — scalars 3, refs 1 (dst), input refs 0;
    ///   - B `([J[J)J` — scalars 0, refs 2, input refs 1 (the packed args1
    ///     src rides the v2 arena, NOT the ref plane — wire refs() = 0);
    ///   - C `(IIIII[I[J)I` — scalars 5, refs 2, input refs 1 (the int[]
    ///     keys ride args1 packed — wire refs() = 0);
    ///   - D `([D[I[I[II[J)I` — scalars 1, refs 5 (4 inputs + dst),
    ///     input refs 4 == wire refs();
    ///   - E `(I[Ljava/lang/Object;[J)I` — scalars 1, refs 2, input refs 1;
    ///   - F `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`
    ///     — scalars 2, refs 4 (3 inputs + dst), input refs 3 == wire refs().
    #[test]
    fn batch_table_signatures_cross_check_against_the_parser() {
        assert_eq!(BATCH_KERNELS.len(), 18, "table grew without this test");
        for k in BATCH_KERNELS {
            let d = parse_sig(k.sig)
                .unwrap_or_else(|e| panic!("kernel id {} sig {:?}: {}", k.id, k.sig, e));
            let s = slots(&d);
            let (exp_scalars, exp_input_refs): (usize, usize) = match k.shape {
                Shape::A | Shape::Z => (1, 0),
                Shape::APrime => (3, 0),
                Shape::B => (0, 1),
                Shape::C => (5, 1),
                Shape::D => (1, 4),
                Shape::E => (1, 1),
                Shape::F => (2, 3),
            };
            assert_eq!(s.scalars, exp_scalars, "kernel id {} ({})", k.id, k.sig);
            assert_eq!(input_refs(&d), exp_input_refs, "kernel id {} ({})", k.id, k.sig);
            // Raw ref count pins (dst included) for the new shapes.
            if k.shape == Shape::D {
                assert_eq!(s.refs, 5);
            }
            if k.shape == Shape::E {
                assert_eq!(s.refs, 2);
            }
            if k.shape == Shape::F {
                assert_eq!(s.refs, 4);
            }
            // The ref-plane width equals the descriptor input refs exactly for
            // the ref-plane-native shapes; it is 0 for every args1/dst shape.
            assert_eq!(
                k.shape.refs(),
                match k.shape {
                    Shape::D => input_refs(&d),
                    Shape::E => input_refs(&d),
                    Shape::F => input_refs(&d),
                    _ => 0,
                },
                "ref-plane width drift for kernel id {}",
                k.id
            );
        }
    }

    /// (c) Encoding pins — the §5 classification is contract, not derivation.
    #[test]
    fn encoding_pins() {
        // The proposal §5's own example shape.
        let d = parse_sig("(ILjava/lang/String;[J)I").expect("doc example must parse");
        assert_eq!(
            d.params,
            vec![ParamKind::JIntNarrow, ParamKind::Ref, ParamKind::Ref]
        );
        assert_eq!(d.ret, RetKind::Int);
        // The F/D/J scalar-bit pins (raw-bit encodings, one long slot each).
        let d = parse_sig("(FDJ)V").expect("FDJ must parse");
        assert_eq!(
            d.params,
            vec![ParamKind::JFloatBits, ParamKind::JDoubleBits, ParamKind::JLong]
        );
        assert_eq!(d.ret, RetKind::Void);
        // The narrow-scalar set rides one sign-extended slot each.
        let d = parse_sig("(ZBSC)I").expect("ZBSC must parse");
        assert_eq!(
            d.params,
            vec![
                ParamKind::JIntNarrow,
                ParamKind::JIntNarrow,
                ParamKind::JIntNarrow,
                ParamKind::JIntNarrow
            ]
        );
        // Multi-dim arrays are still a single Ref slot.
        let d = parse_sig("([[J)I").expect("dimmed array must parse");
        assert_eq!(d.params, vec![ParamKind::Ref]);
    }

    /// (c) Malformed descriptors fail CLOSED (never guess): every pinned
    /// error path returns Err.
    #[test]
    fn malformed_signatures_are_rejected() {
        for bad in [
            "(",           // no params, no return
            "(X)I",        // invalid parameter char
            "(I",          // unterminated parameter list
            "()",          // missing return type
            "(Lno Semi;)I", // object class without ';'
            "",            // empty input
            "I(J)I",       // missing '('
            "([)I",        // dangling dims
            "(I)II",       // trailing garbage after the return type
            "(I)X",        // invalid return type
            "(L;)I",       // empty class name
            "(Lno Semi;)I", // whitespace inside a class name (illegal binary name)
            "([Lno Semi;)I", // dimmed object without ';'
        ] {
            assert!(parse_sig(bad).is_err(), "{bad:?} must be rejected");
        }
    }

    /// (d) The three wave-1 signatures' EXACT ParamKind sequences (wire-v3:
    /// ids 15/16/17, shapes D/E/F).
    #[test]
    fn wave1_signature_encodings() {
        // g35: PaperNativeRangeChoice.optimizedFillArraySummary
        let d = parse_sig("([D[I[I[II[J)I").expect("g35");
        assert_eq!(
            d.params,
            vec![
                ParamKind::Ref,       // [D fills
                ParamKind::Ref,       // [I hashed a
                ParamKind::Ref,       // [I hashed b
                ParamKind::Ref,       // [I hashed c
                ParamKind::JIntNarrow, // jint scalar
                ParamKind::Ref,       // [J dst (trailing summary convention)
            ]
        );
        assert_eq!(d.ret, RetKind::Int);
        assert_eq!(slots(&d), SlotCounts { scalars: 1, refs: 5 });
        assert_eq!(input_refs(&d), 4);

        // g39: PaperNativeSpigotLoadOrderDependency.newLoadAfterBuildSummary
        let d = parse_sig("(I[Ljava/lang/Object;[J)I").expect("g39");
        assert_eq!(
            d.params,
            vec![
                ParamKind::JIntNarrow, // jint scalar
                ParamKind::Ref,        // Object[] refs
                ParamKind::Ref,        // [J dst
            ]
        );
        assert_eq!(d.ret, RetKind::Int);
        assert_eq!(slots(&d), SlotCounts { scalars: 1, refs: 2 });
        assert_eq!(input_refs(&d), 1);

        // g40: PaperNativeSpigotLoadOrderDependency.newRemovedCountSummary
        let d = parse_sig("(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I")
            .expect("g40");
        assert_eq!(
            d.params,
            vec![
                ParamKind::JIntNarrow, // jint scalar a
                ParamKind::Ref,        // Object[] x
                ParamKind::Ref,        // Object[] y
                ParamKind::Ref,        // Object[] z
                ParamKind::JIntNarrow, // jint scalar b
                ParamKind::Ref,        // [J dst
            ]
        );
        assert_eq!(d.ret, RetKind::Int);
        assert_eq!(slots(&d), SlotCounts { scalars: 2, refs: 4 });
        assert_eq!(input_refs(&d), 3);
    }
}
