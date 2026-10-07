//! P5.3 increment 1 (Job 441690) — Beardifier: structure terrain-adaptation
//! contribution to the density function, bit-exact port of CFR decomp441
//! net/minecraft/world/level/levelgen/Beardifier.java (Purpur 1.21.10,
//! DataVersion 4556).
//!
//! Oracle contract (Beardifier.java):
//! - BEARD_KERNEL[24*24*24]: built once, entry (a,b,c) with a=z+12, b=x+12,
//!   c=y+12 is `(float)exp(-lengthSquared(x, y+0.5, z)/16)` — the f32 CAST of
//!   the f64 exp is part of Java semantics; reads widen the f32 back to f64.
//! - getBeardContribution(x,y,z,height) = (-(height+0.5) *
//!   fastInvSqrt(lengthSquared(x, height+0.5, z)/2) / 2) * kernel, 0 when any
//!   of x/y/z+12 falls outside [0,24).
//! - getBuryContribution(x,y,z) = clampedMap(length(x,y,z), 0, 6, 1, 0).
//! - compute(context): sums over RIGID pool pieces and jigsaw junctions with
//!   the per-TerrainAdjustment distance arms (BEARD_THIN/BEARD_BOX *0.8,
//!   BURY, ENCAPSULATE */2 *0.8, junctions *0.4), early-out on the union box
//!   inflated by 24 (INCLUSIVE isInside bounds).
//!
//! This increment is STANDALONE (no tree wiring): Df::Beardifier still
//! evaluates to 0.0 — the piece feed (jigsaw assembly → Rigid/junction lists
//! per chunk) lands in the next increments. Zero behavior change by design.

use crate::mth;

pub const BEARD_KERNEL_RADIUS: i32 = 12;
const BEARD_KERNEL_SIZE: usize = 24;
const BEARD_KERNEL_LEN: usize = BEARD_KERNEL_SIZE * BEARD_KERNEL_SIZE * BEARD_KERNEL_SIZE;

/// TerrainAdjustment (structure JSON `terrain_adaptation` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainAdjustment {
    None,
    Bury,
    BeardThin,
    BeardBox,
    Encapsulate,
}

impl TerrainAdjustment {
    /// Parse the vanilla JSON enum name (data/minecraft/worldgen/structure/*.json).
    pub fn from_json(name: &str) -> Option<TerrainAdjustment> {
        match name {
            "none" => Some(TerrainAdjustment::None),
            "bury" => Some(TerrainAdjustment::Bury),
            "beard_thin" => Some(TerrainAdjustment::BeardThin),
            "beard_box" => Some(TerrainAdjustment::BeardBox),
            "encapsulate" => Some(TerrainAdjustment::Encapsulate),
            _ => None,
        }
    }
}

/// A RIGID pool piece (Beardifier.Rigid): inclusive bounding box + the
/// STRUCTURE-level terrain adjustment (not piece-level — forStructuresInChunk
/// takes it from StructureStart.getStructure().terrainAdaptation()) + the
/// piece's groundLevelDelta.
#[derive(Debug, Clone)]
pub struct BeardRigid {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
    pub adjustment: TerrainAdjustment,
    pub ground_level_delta: i32,
}

impl BeardRigid {
    pub fn new(
        min_x: i32,
        min_y: i32,
        min_z: i32,
        max_x: i32,
        max_y: i32,
        max_z: i32,
        adjustment: TerrainAdjustment,
        ground_level_delta: i32,
    ) -> BeardRigid {
        BeardRigid { min_x, min_y, min_z, max_x, max_y, max_z, adjustment, ground_level_delta }
    }
}

/// JigsawJunction reduced to the three fields Beardifier reads.
#[derive(Debug, Clone, Copy)]
pub struct BeardJunction {
    pub source_x: i32,
    pub source_ground_y: i32,
    pub source_z: i32,
}

/// Inclusive box (Java BoundingBox semantics: min/max are INCLUSIVE on every
/// axis; isInside uses <=).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InclusiveBox {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}

impl InclusiveBox {
    pub fn is_inside(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y && z >= self.min_z && z <= self.max_z
    }

    /// BoundingBox.encapsulating — union of two inclusive boxes.
    pub fn encapsulating(a: &InclusiveBox, b: &InclusiveBox) -> InclusiveBox {
        InclusiveBox {
            min_x: a.min_x.min(b.min_x),
            min_y: a.min_y.min(b.min_y),
            min_z: a.min_z.min(b.min_z),
            max_x: a.max_x.max(b.max_x),
            max_y: a.max_y.max(b.max_y),
            max_z: a.max_z.max(b.max_z),
        }
    }

    /// BoundingBox.inflatedBy(n) — extends by n on EVERY side (inclusive
    /// bounds move out by n in both directions).
    pub fn inflated_by(&self, n: i32) -> InclusiveBox {
        InclusiveBox {
            min_x: self.min_x - n,
            min_y: self.min_y - n,
            min_z: self.min_z - n,
            max_x: self.max_x + n,
            max_y: self.max_y + n,
            max_z: self.max_z + n,
        }
    }
}

/// The Beardifier density node: pieces + junctions + the affected box
/// (union of contributions inflated by 24) — `None` = EMPTY (always 0.0).
#[derive(Debug, Clone)]
pub struct Beardifier {
    pieces: Vec<BeardRigid>,
    junctions: Vec<BeardJunction>,
    affected: Option<InclusiveBox>,
}

impl Beardifier {
    pub fn empty() -> Beardifier {
        Beardifier { pieces: Vec::new(), junctions: Vec::new(), affected: None }
    }

    /// VisibleForTesting constructor — caller supplies already-filtered pieces
    /// (RIGID projection only, isCloseToChunk 12) and junctions (window
    /// [min-12, min+27) on x/z) plus the union box BEFORE inflation; this
    /// inflates by 24 exactly like forStructuresInChunk.
    pub fn new(pieces: Vec<BeardRigid>, junctions: Vec<BeardJunction>, union: Option<InclusiveBox>) -> Beardifier {
        Beardifier { pieces, junctions, affected: union.map(|b| b.inflated_by(24)) }
    }

    pub fn is_empty(&self) -> bool {
        self.affected.is_none()
    }

    /// Beardifier.compute(FunctionContext) — bit-exact.
    pub fn compute(&self, x: i32, y: i32, z: i32) -> f64 {
        let Some(affected) = &self.affected else { return 0.0 };
        if !affected.is_inside(x, y, z) {
            return 0.0;
        }
        let mut d = 0.0f64;
        for rigid in &self.pieces {
            let max0 = 0.max((rigid.min_x - x).max(x - rigid.max_x));
            let max1 = 0.max((rigid.min_z - z).max(z - rigid.max_z));
            let i3 = rigid.min_y + rigid.ground_level_delta;
            let i4 = y - i3;
            let i5 = match rigid.adjustment {
                TerrainAdjustment::None | TerrainAdjustment::Bury | TerrainAdjustment::BeardThin => i4,
                TerrainAdjustment::BeardBox => 0.max((i3 - y).max(y - rigid.max_y)),
                TerrainAdjustment::Encapsulate => 0.max((rigid.min_y - y).max(y - rigid.max_y)),
            };
            d += match rigid.adjustment {
                TerrainAdjustment::None => 0.0,
                TerrainAdjustment::Bury => get_bury_contribution(max0 as f64, i5 as f64 / 2.0, max1 as f64),
                TerrainAdjustment::BeardThin | TerrainAdjustment::BeardBox => {
                    get_beard_contribution(max0, i5, max1, i4) * 0.8
                }
                TerrainAdjustment::Encapsulate => {
                    get_bury_contribution(max0 as f64 / 2.0, i5 as f64 / 2.0, max1 as f64 / 2.0) * 0.8
                }
            };
        }
        for junction in &self.junctions {
            let dx = x - junction.source_x;
            let dy = y - junction.source_ground_y;
            let dz = z - junction.source_z;
            d += get_beard_contribution(dx, dy, dz, dy) * 0.4;
        }
        d
    }
}

/// Beardifier#getBuryContribution.
#[inline]
pub fn get_bury_contribution(x: f64, y: f64, z: f64) -> f64 {
    let len = mth::length3(x, y, z);
    mth::clamped_map(len, 0.0, 6.0, 1.0, 0.0)
}

/// Beardifier#getBeardContribution — kernel read index (z+12)*576 + (x+12)*24
/// + (y+12); the kernel holds f32 values (Java float[]), reads widen to f64.
#[inline]
pub fn get_beard_contribution(x: i32, y: i32, z: i32, height: i32) -> f64 {
    let i = x + 12;
    let i1 = y + 12;
    let i2 = z + 12;
    if is_in_kernel_range(i) && is_in_kernel_range(i1) && is_in_kernel_range(i2) {
        let d = height as f64 + 0.5;
        let d1 = mth::length_squared3(x as f64, d, z as f64);
        let d2 = -d * mth::fast_inv_sqrt(d1 / 2.0) / 2.0;
        let k = BEARD_KERNEL[(i2 as usize) * 24 * 24 + (i as usize) * 24 + (i1 as usize)];
        d2 * (k as f64)
    } else {
        0.0
    }
}

#[inline]
fn is_in_kernel_range(value: i32) -> bool {
    value >= 0 && value < 24
}

/// Kernel entry value BEFORE the f32 cast: exp(-lengthSquared(x, y+0.5, z)/16).
#[inline]
fn compute_beard_contribution(x: i32, y: f64, z: i32) -> f64 {
    let d = mth::length_squared3(x as f64, y, z as f64);
    (-d / 16.0).exp()
}

/// BEARD_KERNEL — built lazily on first use, single thread; entry
/// [z+12][x+12][y+12] = (f32)computeBeardContribution(x, y+0.5, z).
/// Build-loop provenance (CFR lines 38-46): array[i*576 + i1*24 + i2] =
/// compute(i1-12, (i2-12)+0.5, i-12) — with read indices a=i2=z+12,
/// b=i=x+12, c=i1=y+12 this is compute(x, y+0.5, z) at [a][b][c].
static BEARD_KERNEL: std::sync::LazyLock<[f32; BEARD_KERNEL_LEN]> = std::sync::LazyLock::new(|| {
    let mut array = [0.0f32; BEARD_KERNEL_LEN];
    for i in 0..BEARD_KERNEL_SIZE {
        for i1 in 0..BEARD_KERNEL_SIZE {
            for i2 in 0..BEARD_KERNEL_SIZE {
                array[i * 24 * 24 + i1 * 24 + i2] =
                    compute_beard_contribution(i1 as i32 - 12, (i2 as i32 - 12) as f64 + 0.5, i as i32 - 12) as f32;
            }
        }
    }
    array
});

// ---------------------------------------------------------------------------
// Tests — oracle math (CFR Beardifier.java + Mth.java), no live server needed.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_inv_sqrt_matches_java_formula() {
        // Hand-executed Java for 4.0: bits 0x4010000000000000, >>1 =
        // 0x2008000000000000, 6910469410427058090 - that = 0x3FEFF55D... —
        // verify via the formula recomputed independently (i64 arithmetic).
        let number = 4.0f64;
        let d = 0.5 * number;
        let l = number.to_bits() as i64;
        let expected_y = f64::from_bits((6910469410427058090i64 - (l >> 1)) as u64);
        let expected = expected_y * (1.5 - d * expected_y * expected_y);
        assert_eq!(mth::fast_inv_sqrt(4.0).to_bits(), expected.to_bits());
        // Quake-approx sanity: within ~0.18% of 1/sqrt (one-Newton-step magic
        // constant bound; worst measured input 1.0 -> 1.69e-3 relative).
        for v in [1.0f64, 2.0, 0.5, 1024.0, 3.141592653589793] {
            let exact = 1.0 / v.sqrt();
            let approx = mth::fast_inv_sqrt(v);
            assert!((approx - exact).abs() / exact < 1.8e-3, "fastInvSqrt({v}) = {approx} vs {exact}");
        }
    }

    #[test]
    fn kernel_entry_is_f32_cast_of_exp() {
        // kernel[z+12][x+12][y+12] == (f32) exp(-lsq(x, y+0.5, z)/16)
        for (x, y, z) in [(0i32, 0i32, 0i32), (1, -2, 3), (-11, 7, 5), (11, -12, -12), (-12, 11, 0)] {
            let expect = (-mth::length_squared3(x as f64, y as f64 + 0.5, z as f64) / 16.0).exp() as f32;
            let got = BEARD_KERNEL[((z + 12) as usize) * 24 * 24 + ((x + 12) as usize) * 24 + ((y + 12) as usize)];
            assert_eq!(got.to_bits(), expect.to_bits(), "kernel at {x},{y},{z}");
        }
        // Center of the kernel: exp(-(0.5)^2/16) = exp(-0.015625).
        let c = BEARD_KERNEL[12 * 24 * 24 + 12 * 24 + 12];
        let expect_c = ((-(0.5f64 * 0.5) / 16.0).exp()) as f32;
        assert_eq!(c.to_bits(), expect_c.to_bits());
    }

    #[test]
    fn beard_contribution_matches_oracle_formula() {
        // Hand-assembled oracle for (x=3, y=-5, z=-2, height=4):
        // d = 4.5; d1 = 9 + 20.25 + 4; d2 = -4.5 * fastInvSqrt(33.25/2)/2;
        // value = d2 * (f64)kernel[(-2+12)*576 + (3+12)*24 + (-5+12)].
        let d1 = mth::length_squared3(3.0, 4.5, -2.0);
        let d2 = -4.5 * mth::fast_inv_sqrt(d1 / 2.0) / 2.0;
        let expected = d2 * (BEARD_KERNEL[(10) * 24 * 24 + (15) * 24 + (7)] as f64);
        assert_eq!(get_beard_contribution(3, -5, -2, 4).to_bits(), expected.to_bits());
        // Out-of-kernel arms -> exactly 0.0.
        assert_eq!(get_beard_contribution(13, 0, 0, 0).to_bits(), 0.0f64.to_bits());
        assert_eq!(get_beard_contribution(0, -13, 0, 0).to_bits(), 0.0f64.to_bits());
        assert_eq!(get_beard_contribution(0, 0, 12, 0).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn bury_contribution_clamped_map() {
        // clampedMap(len, 0, 6, 1, 0): len 0 -> 1, len >= 6 -> 0, linear
        // between: len 1.5 -> lerp(1, 0, 0.25) = 0.75.
        assert_eq!(get_bury_contribution(0.0, 0.0, 0.0).to_bits(), 1.0f64.to_bits());
        assert_eq!(get_bury_contribution(6.0, 0.0, 0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(get_bury_contribution(2.0, 5.0, 14.0).to_bits(), 0.0f64.to_bits()); // len > 6
        let mid = get_bury_contribution(1.5, 0.0, 0.0);
        assert_eq!(mid.to_bits(), 0.75f64.to_bits());
    }

    #[test]
    fn compute_beard_thin_piece_and_junction() {
        // One BEARD_THIN piece (0..10, 60..70, 0..10, delta 0) + one junction
        // at (5, 70, 5). Point (5, 66, 5): max0 = max1 = 0, i3 = 60, i4 = 6,
        // i5 = 6 (BEARD_THIN) -> beard(0, 6, 0, 6)*0.8 + junction
        // beard(0, -4, 0, -4)*0.4.
        let pieces = vec![BeardRigid::new(0, 60, 0, 10, 70, 10, TerrainAdjustment::BeardThin, 0)];
        let junctions = vec![BeardJunction { source_x: 5, source_ground_y: 70, source_z: 5 }];
        let union = InclusiveBox::encapsulating(
            &InclusiveBox { min_x: 0, min_y: 60, min_z: 0, max_x: 10, max_y: 70, max_z: 10 },
            &InclusiveBox { min_x: 5, min_y: 70, min_z: 5, max_x: 5, max_y: 70, max_z: 5 },
        );
        let b = Beardifier::new(pieces, junctions, Some(union));
        assert!(!b.is_empty());
        let expected = get_beard_contribution(0, 6, 0, 6) * 0.8 + get_beard_contribution(0, -4, 0, -4) * 0.4;
        assert_eq!(b.compute(5, 66, 5).to_bits(), expected.to_bits());
        // Outside the inflated box -> 0.0 even with pieces present.
        assert_eq!(b.compute(5, 200, 5).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn compute_adjustment_arms() {
        // BURY: contribution = bury(max0, i5/2, max1) with i5 = i4 (NOT
        // clamped at 0) — verify at a point above the box.
        let bury = Beardifier::new(
            vec![BeardRigid::new(0, 60, 0, 10, 70, 10, TerrainAdjustment::Bury, 0)],
            vec![],
            Some(InclusiveBox { min_x: 0, min_y: 60, min_z: 0, max_x: 10, max_y: 70, max_z: 10 }),
        );
        let expected_bury = get_bury_contribution(0.0, (78 - 60) as f64 / 2.0, 0.0);
        assert_eq!(bury.compute(5, 78, 5).to_bits(), expected_bury.to_bits());
        // ENCAPSULATE at an INSIDE point: max0 = max1 = 0, i5 = max(0, max(
        // minY - y, y - maxY)) = 0 -> bury(0,0,0)*0.8 = 0.8.
        let enc = Beardifier::new(
            vec![BeardRigid::new(0, 60, 0, 10, 70, 10, TerrainAdjustment::Encapsulate, 0)],
            vec![],
            Some(InclusiveBox { min_x: 0, min_y: 60, min_z: 0, max_x: 10, max_y: 70, max_z: 10 }),
        );
        assert_eq!(enc.compute(5, 65, 5).to_bits(), 0.8f64.to_bits());
        // BEARD_BOX above the box: i5 = max(0, max(60 - y, y - 70)) = y - 70,
        // contribution = beard(0, i5, 0, i4) * 0.8 with i4 = y - 60.
        let boxp = Beardifier::new(
            vec![BeardRigid::new(0, 60, 0, 10, 70, 10, TerrainAdjustment::BeardBox, 0)],
            vec![],
            Some(InclusiveBox { min_x: 0, min_y: 60, min_z: 0, max_x: 10, max_y: 70, max_z: 10 }),
        );
        let expected_box = get_beard_contribution(0, 3, 0, 13) * 0.8;
        assert_eq!(boxp.compute(5, 73, 5).to_bits(), expected_box.to_bits());
        // NONE piece: inside the box but contributes exactly 0.
        let none = Beardifier::new(
            vec![BeardRigid::new(0, 60, 0, 10, 70, 10, TerrainAdjustment::None, 0)],
            vec![],
            Some(InclusiveBox { min_x: 0, min_y: 60, min_z: 0, max_x: 10, max_y: 70, max_z: 10 }),
        );
        assert_eq!(none.compute(5, 65, 5).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn empty_beardifier_is_zero_everywhere() {
        let b = Beardifier::empty();
        assert!(b.is_empty());
        assert_eq!(b.compute(-160, 66, -256).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn terrain_adjustment_json_names() {
        assert_eq!(TerrainAdjustment::from_json("beard_thin"), Some(TerrainAdjustment::BeardThin));
        assert_eq!(TerrainAdjustment::from_json("encapsulate"), Some(TerrainAdjustment::Encapsulate));
        assert_eq!(TerrainAdjustment::from_json("nonsense"), None);
    }
}
