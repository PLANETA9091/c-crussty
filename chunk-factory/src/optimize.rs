//! NCF P2.14 — interval cutoff (интервальная отсечка).
//!
//! Build-time rewrites of the wired density tree that SKIP work whose result
//! provably cannot affect any output value. Soundness contract (I2/I3):
//! every rewrite uses the SAME bounds Java itself attaches at construction
//! (`TwoArgumentSimpleFunction.create` / per-node min/max), so a rewritten
//! tree returns BIT-IDENTICAL f64 for every (x, y, z). Branches of RangeChoice
//! are pure (no cache state on the Df level), so an untaken branch is free to
//! drop — Java evaluates it, but its value is never observed when the guard
//! decides the other side for ALL possible inputs.
//!
//! Rewrites (conservative set):
//!   R1 RangeChoice: input.max < minInclusive OR input.min >= maxExclusive
//!      -> the guard can never be true -> out-of-range branch.
//!      input.min >= minInclusive AND input.max < maxExclusive
//!      -> guard always true -> in-range branch.   (Java: d >= min && d < max)
//!   R2 Clamp: input bounds within [min, max] -> drop the clamp
//!      (Mth.clamp(v, min, max) == v for v in [min, max]; NaN cannot occur:
//!      noise/grad nodes are finite and their bounds finite).
//!   R3 MulOrAdd mul with argument == 1.0 -> input (x*1.0 == x bitwise for
//!      all finite/±0 values; NaN cannot occur per the bounds contract).
//!      Add with argument == 0.0 is NOT folded: (-0.0) + 0.0 == +0.0 but
//!      passing -0.0 through would flip the sign bit — NOT bit-identical.
//!
//! Every other node recurses with its children optimized; stored bound fields
//! stay untouched (rewrites preserve value ranges exactly).
//!
//! Gates that pin bit-exactness: veccheck all modes (13171+786432+...),
//! stagediff 225x3 statuses, ncf-gate-p2 matrix 36015 chunks.

use crate::density::{Ap2Type, Df, NoiseBank};

/// Trusted bounds: ONLY the node classes whose min/max Java itself computes
/// identically (TwoArgumentSimpleFunction.create, Clamp/Mapped/MulOrAdd
/// constructors, YClampedGradient, ±noise.maxValue). Spline/WeirdScaled/
/// Blended bounds come from OUR port — a slightly-too-narrow bound would
/// make a fold UNSOUND (CI surface gate caught exactly that: podzol/grass
/// biome flips). Unknown => refuse to fold anything upstream.
fn trusted_bounds(df: &Df, bank: &NoiseBank) -> Option<(f64, f64)> {
    let r = match df {
        // TRUSTED LEAVES (bounds are structural facts, not ported math)
        Df::Const(v) => (*v, *v),
        Df::YClampedGradient { from_value, to_value, .. } => {
            (from_value.min(*to_value), from_value.max(*to_value))
        }
        Df::Noise(idx, ..) => {
            let m = bank.noises[*idx].max_value();
            (-m, m)
        }
        Df::ShiftA(idx) | Df::ShiftB(idx) | Df::Shift(idx) => {
            let m = bank.noises[*idx].max_value() * 4.0;
            (-m, m)
        }
        Df::Clamp { min, max, .. } => (*min, *max), // JSON-given constants
        Df::BlendAlpha => (1.0, 1.0),
        Df::BlendOffset => (0.0, 0.0),
        Df::Beardifier => (0.0, 0.0),
        // COMPOSITION over trusted children (formulas = Java constructors
        // applied to OUR child bounds — sound iff children are sound)
        Df::Marker { wrapped: w, .. } => return trusted_bounds(w, bank),
        Df::Mapped { ty, input, .. } => {
            let (lo, hi) = trusted_bounds(input, bank)?;
            // transform may be non-monotone over a straddling box (abs/square):
            // box = min/max over endpoints + the 0 critical point if inside
            let mut cands: Vec<f64> = vec![ty.transform(lo), ty.transform(hi)];
            if lo < 0.0 && hi > 0.0 {
                cands.push(ty.transform(0.0));
            }
            let mut blo = f64::INFINITY;
            let mut bhi = f64::NEG_INFINITY;
            for c in cands {
                blo = blo.min(c);
                bhi = bhi.max(c);
            }
            (blo, bhi)
        }
        Df::MulOrAdd { is_add, input, argument, .. } => {
            let (lo, hi) = trusted_bounds(input, bank)?;
            if *is_add {
                (lo + argument, hi + argument)
            } else {
                (lo * argument, hi * argument)
            }
        }
        Df::Ap2 { ty, a1, a2, .. } => {
            let (x1, x2) = trusted_bounds(a1, bank)?;
            let (y1, y2) = trusted_bounds(a2, bank)?;
            match ty {
                Ap2Type::Add => (x1 + y1, x2 + y2),
                Ap2Type::Mul => {
                    let cands = [x1 * y1, x1 * y2, x2 * y1, x2 * y2];
                    (cands.iter().cloned().fold(f64::INFINITY, f64::min),
                     cands.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
                }
                Ap2Type::Min => (x1.min(y1), x2.min(y2)),
                Ap2Type::Max => (x1.min(y1).max(x2.min(y2)), x2.max(y2)),
            }
        }
        Df::RangeChoice { when_in_range, when_out_of_range, .. } => {
            let (a_lo, a_hi) = trusted_bounds(when_in_range, bank)?;
            let (b_lo, b_hi) = trusted_bounds(when_out_of_range, bank)?;
            (a_lo.min(b_lo), a_hi.max(b_hi))
        }
        // UNTRUSTED: any node whose range depends on OUR ported bounds math
        // (spline min/max, weird-scaled rarity, blended max, end islands,
        // find-top-surface, blend density)
        _ => return None,
    };
    Some(r)
}

pub fn optimize_tree(df: Df, bank: &NoiseBank) -> Df {
    match df {
        Df::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } => {
            let input = optimize_tree(*input, bank);
            let in_range = optimize_tree(*when_in_range, bank);
            let out_of_range = optimize_tree(*when_out_of_range, bank);
            if let Some((lo, hi)) = trusted_bounds(&input, bank) {
                if hi < min_inclusive || lo >= max_exclusive {
                    // guard false for every input -> the out branch is the only
                    // observable value (Java short-circuit order preserved: the
                    // guard input is still evaluated first in Java, but it is
                    // pure — dropping it changes no observable state).
                    return out_of_range;
                }
                if lo >= min_inclusive && hi < max_exclusive {
                    return in_range;
                }
            }
            Df::RangeChoice {
                input: Box::new(input),
                min_inclusive,
                max_exclusive,
                when_in_range: Box::new(in_range),
                when_out_of_range: Box::new(out_of_range),
            }
        }
        Df::Clamp { input, min, max } => {
            let input = optimize_tree(*input, bank);
            if let Some((lo, hi)) = trusted_bounds(&input, bank) {
                if lo >= min && hi <= max {
                    return input;
                }
            }
            Df::Clamp { input: Box::new(input), min, max }
        }
        Df::MulOrAdd { is_add, input, min, max, argument } => {
            let input = optimize_tree(*input, bank);
            if !is_add && argument == 1.0 {
                return input;
            }
            Df::MulOrAdd { is_add, input: Box::new(input), min, max, argument }
        }
        Df::ShiftedNoise { shift_x, shift_y, shift_z, xz_scale, y_scale, noise } => Df::ShiftedNoise {
            shift_x: Box::new(optimize_tree(*shift_x, bank)),
            shift_y: Box::new(optimize_tree(*shift_y, bank)),
            shift_z: Box::new(optimize_tree(*shift_z, bank)),
            xz_scale,
            y_scale,
            noise,
        },
        Df::BlendDensity(input) => Df::BlendDensity(Box::new(optimize_tree(*input, bank))),
        Df::Marker { ty, wrapped } => Df::Marker { ty, wrapped: Box::new(optimize_tree(*wrapped, bank)) },
        Df::WeirdScaledSampler { input, noise, rarity } => {
            Df::WeirdScaledSampler { input: Box::new(optimize_tree(*input, bank)), noise, rarity }
        }
        Df::Mapped { ty, input, min, max } => {
            Df::Mapped { ty, input: Box::new(optimize_tree(*input, bank)), min, max }
        }
        Df::Ap2 { ty, a1, a2, min, max } => Df::Ap2 {
            ty,
            a1: Box::new(optimize_tree(*a1, bank)),
            a2: Box::new(optimize_tree(*a2, bank)),
            min,
            max,
        },
        Df::FindTopSurface { density, upper_bound, lower_bound, cell_height } => Df::FindTopSurface {
            density: Box::new(optimize_tree(*density, bank)),
            upper_bound: Box::new(optimize_tree(*upper_bound, bank)),
            lower_bound,
            cell_height,
        },
        // leaves and structurally opaque nodes: nothing to rewrite
        other => other,
    }
}

/// Optimize all 15 router fields (used from RandomState::build AFTER the
/// spec hash is taken over the unoptimized shape — the shape text therefore
/// stays stable across sessions).
pub fn optimize_router_fields(
    barrier: Df,
    floodedness: Df,
    spread: Df,
    lava: Df,
    temperature: Df,
    vegetation: Df,
    continents: Df,
    erosion: Df,
    depth: Df,
    ridges: Df,
    preliminary_surface_level: Df,
    final_density: Df,
    vein_toggle: Df,
    vein_ridged: Df,
    vein_gap: Df,
    bank: &NoiseBank,
) -> (
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
    Df,
) {
    (
        optimize_tree(barrier, bank),
        optimize_tree(floodedness, bank),
        optimize_tree(spread, bank),
        optimize_tree(lava, bank),
        optimize_tree(temperature, bank),
        optimize_tree(vegetation, bank),
        optimize_tree(continents, bank),
        optimize_tree(erosion, bank),
        optimize_tree(depth, bank),
        optimize_tree(ridges, bank),
        optimize_tree(preliminary_surface_level, bank),
        optimize_tree(final_density, bank),
        optimize_tree(vein_toggle, bank),
        optimize_tree(vein_ridged, bank),
        optimize_tree(vein_gap, bank),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::density::Ap2Type;

    #[test]
    fn rangechoice_out_branch_when_input_below_min() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        // input = const -5; guard [0..10) -> never true -> out branch (const 7)
        let df = Df::RangeChoice {
            input: Box::new(Df::Const(-5.0)),
            min_inclusive: 0.0,
            max_exclusive: 10.0,
            when_in_range: Box::new(Df::Const(3.0)),
            when_out_of_range: Box::new(Df::Const(7.0)),
        };
        let opt = optimize_tree(df, &bank);
        assert!(matches!(opt, Df::Const(v) if v == 7.0));
    }

    #[test]
    fn rangechoice_in_branch_when_input_inside_window() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        let df = Df::RangeChoice {
            input: Box::new(Df::Const(4.0)),
            min_inclusive: 0.0,
            max_exclusive: 10.0,
            when_in_range: Box::new(Df::Const(3.0)),
            when_out_of_range: Box::new(Df::Const(7.0)),
        };
        let opt = optimize_tree(df, &bank);
        assert!(matches!(opt, Df::Const(v) if v == 3.0));
    }

    #[test]
    fn clamp_dropped_when_bounds_inside() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        let df = Df::Clamp { input: Box::new(Df::Const(0.5)), min: -1.0, max: 1.0 };
        let opt = optimize_tree(df, &bank);
        assert!(matches!(opt, Df::Const(v) if v == 0.5));
    }

    #[test]
    fn clamp_kept_when_bounds_exceed_window() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        // YClampedGradient bounds span [-2..2] -> clamp [-1..1] must stay
        let df = Df::Clamp {
            input: Box::new(Df::YClampedGradient {
                from_y: 0,
                to_y: 16,
                from_value: -2.0,
                to_value: 2.0,
            }),
            min: -1.0,
            max: 1.0,
        };
        let opt = optimize_tree(df, &bank);
        assert!(matches!(opt, Df::Clamp { .. }));
    }

    #[test]
    fn mul_by_one_folded_add_by_zero_kept() {
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        let df = Df::MulOrAdd {
            is_add: false,
            input: Box::new(Df::Const(2.5)),
            min: 2.5,
            max: 2.5,
            argument: 1.0,
        };
        assert!(matches!(optimize_tree(df, &bank), Df::Const(v) if v == 2.5));
        // add-by-zero: NOT folded (-0.0 sign hazard)
        let df = Df::MulOrAdd {
            is_add: true,
            input: Box::new(Df::Const(2.5)),
            min: 2.5,
            max: 2.5,
            argument: 0.0,
        };
        assert!(matches!(optimize_tree(df, &bank), Df::MulOrAdd { .. }));
    }

    #[test]
    fn optimize_preserves_values_on_a_mixed_tree() {
        // pure functions: rewrite must be value-neutral for every probe
        let bank = NoiseBank { noises: Vec::new(), blended: Vec::new() };
        let mk = || Df::Ap2 {
            ty: Ap2Type::Add,
            a1: Box::new(Df::RangeChoice {
                input: Box::new(Df::Const(11.0)),
                min_inclusive: 0.0,
                max_exclusive: 10.0,
                when_in_range: Box::new(Df::Const(100.0)),
                when_out_of_range: Box::new(Df::Const(-100.0)),
            }),
            a2: Box::new(Df::Const(0.25)),
            min: -100.0,
            max: 100.25,
        };
        let opt = optimize_tree(mk(), &bank);
        for x in -3..3 {
            for y in -3..3 {
                for z in -3..3 {
                    let raw_v = mk().compute(&bank, x, y, z);
                    let opt_v = opt.compute(&bank, x, y, z);
                    assert_eq!(raw_v.to_bits(), opt_v.to_bits(), "divergence at {x},{y},{z}");
                }
            }
        }
    }
}
