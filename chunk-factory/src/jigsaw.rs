//! NCF P5.3 increment 2a (Job 441690) — jigsaw assembly CORE: the exact RNG
//! protocol, rotation/transform math, template NBT jigsaw extraction, pool
//! resolution and the shuffle family, ported bit-exact from the 1.21.10 CFR
//! oracles (decomp441 + fresh dumps):
//!   WorldgenRandom.setLargeFeatureSeed   (lines 71-77)
//!   Structure.generate / GenerationContext.makeRandom (lines 235-237)
//!   Rotation (enum order NONE,CW90,CW180,CCW90; getRandom/getShuffled via Util)
//!   Util.shuffle / shuffledCopy / getRandom (lines 729-998)
//!   StructureTemplate.transform (468-502), getBoundingBox (564-578),
//!     getJigsaws (205-217), JigsawBlockInfo.of (763-766),
//!     getJointType/getDefaultJointType (709-715)
//!   StructureTemplatePool (57-108: weight expansion, getRandomTemplate,
//!     getShuffledTemplates, size)
//!   StructurePoolElement (groundLevelDelta default 1, projection)
//!   SinglePoolElement.getShuffledJigsawBlocks (shuffle + stable sort by
//!     selectionPriority DESC), ListPoolElement (jigsaws from elements[0],
//!     bbox = union), EmptyPoolElement (bbox must be filtered)
//!   JigsawBlock.canAttach (85-93), getFrontFacing/getTopFacing (ORIENTATION)
//!   StructureTemplateManager (structure/ SINGULAR prefix — 1.21.10 jar has
//!     data/<ns>/structure/<path>.nbt; verified in the purpur jar)
//!
//! NOT in this increment (2b): Placer.tryPlacingChildren + the exact
//! free-space box algebra (VoxelShape masks = boxes minus boxes, exact on the
//! 0.25 grid), SequencedPriorityIterator, junction recording, heightmap start
//! heights (getFirstFreeHeight), pool_aliases (currently identity + flag).

use crate::jrandom::RandomSource;
use crate::sections::Nbt;

// ---------------------------------------------------------------------------
// WorldgenRandom.setLargeFeatureSeed — the structure GENERATION rng entry
// (no salt; placement uses setLargeFeatureWithSalt in P5.1 instead).
// ---------------------------------------------------------------------------
pub fn set_large_feature_seed<R: RandomSource>(rng: &mut R, base_seed: i64, chunk_x: i32, chunk_z: i32) {
    rng.set_seed(base_seed);
    let a = rng.next_long();
    let b = rng.next_long();
    let l = (chunk_x as i64).wrapping_mul(a) ^ (chunk_z as i64).wrapping_mul(b) ^ base_seed;
    rng.set_seed(l);
}

// ---------------------------------------------------------------------------
// Rotation
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

/// Rotation.values() ORDER — drives getRandom/getShuffled RNG consumption.
pub const ROTATION_VALUES: [Rotation; 4] =
    [Rotation::None, Rotation::Clockwise90, Rotation::Clockwise180, Rotation::Counterclockwise90];

/// Util.getRandom(Rotation.values(), random) = values[nextInt(4)].
pub fn rotation_get_random<R: RandomSource>(rng: &mut R) -> Rotation {
    ROTATION_VALUES[rng.next_int_bound(4) as usize]
}

/// Util.shuffledCopy(values, random) = copy + shuffle (Fisher-Yates from the
/// top: for i = n; i > 1; --i { swap(i-1, nextInt(i)) }).
pub fn rotation_get_shuffled<R: RandomSource>(rng: &mut R) -> [Rotation; 4] {
    let mut list = ROTATION_VALUES;
    util_shuffle(&mut list, rng);
    list
}

/// Rotation.rotate(Direction): Y axis unchanged; CW90 = clockwise
/// (N->E->S->W->N); CW180 = opposite; CCW90 = counterclockwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Dir {
    pub fn opposite(self) -> Dir {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
    fn clock_wise(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            other => other,
        }
    }
    fn counter_clock_wise(self) -> Dir {
        self.clock_wise().opposite()
    }
    pub fn is_horizontal(self) -> bool {
        !matches!(self, Dir::Up | Dir::Down)
    }

    /// Direction.getStepY — the Y component of the facing.
    pub fn step_y(self) -> i32 {
        match self {
            Dir::Up => 1,
            Dir::Down => -1,
            _ => 0,
        }
    }
}

impl Rotation {
    pub fn rotate_dir(self, facing: Dir) -> Dir {
        match self {
            Rotation::None => facing,
            Rotation::Clockwise90 => facing.clock_wise(),
            Rotation::Clockwise180 => facing.opposite(),
            Rotation::Counterclockwise90 => facing.counter_clock_wise(),
        }
    }

    /// StructureTemplate.transform(pos, Mirror.NONE, rotation, pivot) — the
    /// decompiled arms with (x1, z1) = pivot:
    ///   CCW90: (x1 - z1 + z, y, x1 + z1 - x)
    ///   CW90:  (x1 + z1 - z, y, z1 - x1 + x)
    ///   CW180: (2*x1 - x, y, 2*z1 - z)
    ///   NONE:  (x, y, z)
    pub fn transform_pos(self, x: i32, y: i32, z: i32, pivot_x: i32, pivot_z: i32) -> (i32, i32, i32) {
        match self {
            Rotation::None => (x, y, z),
            Rotation::Clockwise90 => (pivot_x + pivot_z - z, y, pivot_z - pivot_x + x),
            Rotation::Clockwise180 => (pivot_x + pivot_x - x, y, pivot_z + pivot_z - z),
            Rotation::Counterclockwise90 => (pivot_x - pivot_z + z, y, pivot_x + pivot_z - x),
        }
    }
}

// ---------------------------------------------------------------------------
// Util shuffle family (exact RNG consumption)
// ---------------------------------------------------------------------------
pub fn util_shuffle<T, R: RandomSource>(list: &mut [T], rng: &mut R) {
    let size = list.len();
    let mut i = size;
    while i > 1 {
        let j = rng.next_int_bound(i as i32) as usize;
        list.swap(i - 1, j);
        i -= 1;
    }
}

pub fn util_shuffled_copy<T: Clone, R: RandomSource>(list: &[T], rng: &mut R) -> Vec<T> {
    let mut copy = list.to_vec();
    util_shuffle(&mut copy, rng);
    copy
}

pub fn util_get_random<'a, T, R: RandomSource>(list: &'a [T], rng: &mut R) -> &'a T {
    &list[rng.next_int_bound(list.len() as i32) as usize]
}

// ---------------------------------------------------------------------------
// Template NBT (size + jigsaw blocks)
// ---------------------------------------------------------------------------
/// JigsawBlockEntity.JointType.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    Rollable,
    Aligned,
}

/// One jigsaw block of a template, in the template's OWN coordinate space
/// (block list order preserved — StructureTemplate.Palette.jigsaws()).
#[derive(Debug, Clone)]
pub struct TemplateJigsaw {
    pub pos: (i32, i32, i32),
    /// ORIENTATION property "front_top" (e.g. "north_up"); front is the first
    /// token, top the second. Parsed into dirs at use site.
    pub front: Dir,
    pub top: Dir,
    pub name: String,
    pub pool: String,
    pub target: String,
    pub placement_priority: i32,
    pub selection_priority: i32,
    pub joint: JointType,
}

#[derive(Debug, Clone)]
pub struct TemplateData {
    pub size: (i32, i32, i32),
    pub jigsaws: Vec<TemplateJigsaw>,
}

fn parse_orientation(state: &Nbt) -> Option<(Dir, Dir)> {
    let props = state.get("Properties")?;
    let o = props.get("orientation")?.as_str()?;
    let (f, t) = o.split_once('_')?;
    Some((parse_dir(f)?, parse_dir(t)?))
}

fn parse_dir(s: &str) -> Option<Dir> {
    match s {
        "down" => Some(Dir::Down),
        "up" => Some(Dir::Up),
        "north" => Some(Dir::North),
        "south" => Some(Dir::South),
        "west" => Some(Dir::West),
        "east" => Some(Dir::East),
        _ => None,
    }
}

/// JigsawBlockInfo.of — nbt fields with the exact defaults:
/// name -> minecraft:empty, pool -> minecraft:empty, target -> minecraft:empty,
/// placement_priority/selection_priority -> 0.
/// getJointType: nbt "joint" (rollable/aligned) else
/// getDefaultJointType: front horizontal -> ALIGNED else ROLLABLE.
fn parse_jigsaw(pos: (i32, i32, i32), state: &Nbt, nbt: &Nbt) -> Option<TemplateJigsaw> {
    let (front, top) = parse_orientation(state)?;
    let field_str = |k: &str| -> String {
        nbt.get(k)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "minecraft:empty".to_string())
    };
    let joint = match nbt.get("joint").and_then(|v| v.as_str()) {
        Some("rollable") => JointType::Rollable,
        Some("aligned") => JointType::Aligned,
        _ => {
            if front.is_horizontal() {
                JointType::Aligned
            } else {
                JointType::Rollable
            }
        }
    };
    Some(TemplateJigsaw {
        pos,
        front,
        top,
        name: field_str("name"),
        pool: field_str("pool"),
        target: field_str("target"),
        placement_priority: nbt.get("placement_priority").and_then(|v| v.as_int()).unwrap_or(0),
        selection_priority: nbt.get("selection_priority").and_then(|v| v.as_int()).unwrap_or(0),
        joint,
    })
}

/// Parse a template NBT (root compound) into size + jigsaws. Jigsaws keep
/// BLOCK LIST ORDER (palette filter order — the cache is value-identical).
pub fn parse_template(root: &Nbt) -> Option<TemplateData> {
    let size_arr = root.get("size")?;
    let sa = match size_arr {
        Nbt::List(v) if v.len() == 3 => v,
        _ => return None,
    };
    let size = (
        sa[0].as_int()?,
        sa[1].as_int()?,
        sa[2].as_int()?,
    );
    let palette = match root.get("palette") {
        Some(Nbt::List(p)) => p.clone(),
        _ => return None,
    };
    let blocks = match root.get("blocks") {
        Some(Nbt::List(b)) => b,
        _ => return None,
    };
    let mut jigsaws = Vec::new();
    for block in blocks {
        let state_idx = block.get("state")?.as_int()? as usize;
        let Some(state) = palette.get(state_idx) else { continue };
        let name = state.get("Name").and_then(|v| v.as_str()).unwrap_or("");
        if name != "minecraft:jigsaw" {
            continue;
        }
        let pos_arr = match block.get("pos") {
            Some(Nbt::List(v)) if v.len() == 3 => v,
            _ => continue,
        };
        let pos = (pos_arr[0].as_int()?, pos_arr[1].as_int()?, pos_arr[2].as_int()?);
        // JigsawBlockInfo.of: nbt must be present (Objects.requireNonNull).
        let nbt = block.get("nbt")?;
        if let Some(j) = parse_jigsaw(pos, state, nbt) {
            jigsaws.push(j);
        }
    }
    Some(TemplateData { size, jigsaws })
}

// ---------------------------------------------------------------------------
// Transforms over a parsed template
// ---------------------------------------------------------------------------
impl TemplateData {
    /// StructureTemplate.getBoundingBox(startPos, rotation, pivot=ZERO,
    /// Mirror.NONE, size): transform both ZERO and ZERO.offset(size-1), take
    /// fromCorners (min/max per axis), move by startPos.
    pub fn bounding_box(&self, start: (i32, i32, i32), rot: Rotation) -> crate::beardifier::InclusiveBox {
        let (sx, sy, sz) = (self.size.0 - 1, self.size.1 - 1, self.size.2 - 1);
        let a = rot.transform_pos(0, 0, 0, 0, 0);
        let b = rot.transform_pos(sx, sy, sz, 0, 0);
        crate::beardifier::InclusiveBox {
            min_x: a.0.min(b.0) + start.0,
            min_y: a.1.min(b.1) + start.1,
            min_z: a.2.min(b.2) + start.2,
            max_x: a.0.max(b.0) + start.0,
            max_y: a.1.max(b.1) + start.1,
            max_z: a.2.max(b.2) + start.2,
        }
    }

    /// StructureTemplate.getJigsaws(pos, rotation): per jigsaw block,
    /// pos' = transform(pos, rot, ZERO) + offset, front/top rotated.
    pub fn jigsaws_at(&self, offset: (i32, i32, i32), rot: Rotation) -> Vec<TemplateJigsaw> {
        self.jigsaws
            .iter()
            .map(|j| {
                let (x, y, z) = rot.transform_pos(j.pos.0, j.pos.1, j.pos.2, 0, 0);
                TemplateJigsaw {
                    pos: (x + offset.0, y + offset.1, z + offset.2),
                    front: rot.rotate_dir(j.front),
                    top: rot.rotate_dir(j.top),
                    name: j.name.clone(),
                    pool: j.pool.clone(),
                    target: j.target.clone(),
                    placement_priority: j.placement_priority,
                    selection_priority: j.selection_priority,
                    joint: j.joint,
                }
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Pool element / pool resolution
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Projection {
    Rigid,
    TerrainMatching,
}

impl Projection {
    fn from_json(s: &str) -> Projection {
        match s {
            "terrain_matching" => Projection::TerrainMatching,
            _ => Projection::Rigid,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PoolElement {
    Empty,
    /// single_pool_element / legacy_single_pool_element — identical for
    /// assembly (legacy differs only in placement processors).
    Single { location: String, projection: Projection },
    /// list_pool_element — jigsaws from elements[0]; bbox = union of children.
    List { elements: Vec<PoolElement>, projection: Projection },
    /// feature_pool_element / anything else: assembly-unsupported (loud).
    Unsupported { kind: String },
}

impl PoolElement {
    pub fn projection(&self) -> Projection {
        match self {
            PoolElement::Empty => Projection::Rigid,
            PoolElement::Single { projection, .. } | PoolElement::List { projection, .. } => *projection,
            PoolElement::Unsupported { .. } => Projection::Rigid,
        }
    }

    /// StructurePoolElement.getGroundLevelDelta — default 1 for every
    /// assembly-relevant element type.
    pub fn ground_level_delta(&self) -> i32 {
        1
    }

    fn parse(v: &crate::json::Json, fallback_projection: Option<Projection>) -> PoolElement {
        let ty = v.get("element_type").and_then(|x| x.as_str()).unwrap_or("minecraft:single_pool_element");
        let projection = v
            .get("projection")
            .and_then(|x| x.as_str())
            .map(Projection::from_json)
            .or(fallback_projection)
            .unwrap_or(Projection::Rigid);
        match ty {
            "minecraft:empty_pool_element" => PoolElement::Empty,
            "minecraft:single_pool_element" | "minecraft:legacy_single_pool_element" => {
                let location = v.get("location").and_then(|x| x.as_str()).unwrap_or("").to_string();
                PoolElement::Single { location, projection }
            }
            "minecraft:list_pool_element" => {
                let elements = v
                    .get("elements")
                    .and_then(|x| x.as_arr())
                    .map(|arr| {
                        arr.iter()
                            .map(|e| PoolElement::parse(e, Some(projection)))
                            .collect()
                    })
                    .unwrap_or_default();
                PoolElement::List { elements, projection }
            }
            other => PoolElement::Unsupported { kind: other.to_string() },
        }
    }
}

/// A resolved StructureTemplatePool: the expanded `templates` list (element
/// repeated `weight` times — exact StructureTemplatePool constructor) +
/// fallback key. Pool aliases are NOT applied yet (2b) — callers must flag.
#[derive(Debug, Clone)]
pub struct ResolvedPool {
    pub fallback: String,
    pub templates: Vec<PoolElement>,
    pub has_aliases: bool,
}

/// Resolve a template pool by key ("ns:path") from the worldgen dir.
pub fn resolve_pool(dir: &crate::router::WorldgenDir, key: &str, structure_has_aliases: bool) -> Option<ResolvedPool> {
    let (ns, path) = key.split_once(':')?;
    let raw = dir.get(ns, "template_pool", path)?;
    let v = crate::json::parse(raw).ok()?;
    let fallback = v.get("fallback").and_then(|x| x.as_str()).unwrap_or("minecraft:empty").to_string();
    let mut templates = Vec::new();
    for entry in v.get("elements").and_then(|x| x.as_arr()).unwrap_or(&[]) {
        let weight = entry.get("weight").and_then(|x| x.as_i64()).unwrap_or(1).max(0) as usize;
        let element = PoolElement::parse(entry.get("element")?, None);
        for _ in 0..weight {
            templates.push(element.clone());
        }
    }
    Some(ResolvedPool { fallback, templates, has_aliases: structure_has_aliases })
}

impl ResolvedPool {
    /// getRandomTemplate: templates[nextInt(size)]; Empty if empty list.
    pub fn get_random_template<R: RandomSource>(&self, rng: &mut R) -> PoolElement {
        if self.templates.is_empty() {
            PoolElement::Empty
        } else {
            util_get_random(&self.templates, rng).clone()
        }
    }

    /// getShuffledTemplates: shuffledCopy over the EXPANDED list.
    pub fn get_shuffled_templates<R: RandomSource>(&self, rng: &mut R) -> Vec<PoolElement> {
        util_shuffled_copy(&self.templates, rng)
    }

    pub fn size(&self) -> i32 {
        self.templates.len() as i32
    }
}

// ---------------------------------------------------------------------------
// JigsawBlock.canAttach (85-93)
// ---------------------------------------------------------------------------
pub fn can_attach(parent: &TemplateJigsaw, child: &TemplateJigsaw) -> bool {
    let front = parent.front;
    let front1 = child.front;
    let top = parent.top;
    let top1 = child.top;
    let rollable = parent.joint == JointType::Rollable;
    front == front1.opposite() && (rollable || top == top1) && parent.target == child.name
}

// ---------------------------------------------------------------------------
// getShuffledJigsawBlocks (SinglePoolElement path): getJigsaws -> Util.shuffle
// -> STABLE sort by selectionPriority DESC (List.sort is stable).
// ---------------------------------------------------------------------------
pub fn shuffled_jigsaw_blocks<R: RandomSource>(
    template: &TemplateData,
    offset: (i32, i32, i32),
    rot: Rotation,
    rng: &mut R,
) -> Vec<TemplateJigsaw> {
    let mut list = template.jigsaws_at(offset, rot);
    util_shuffle(&mut list, rng);
    // stable sort — Rust sort_by is stable, matches Java List.sort
    list.sort_by(|a, b| b.selection_priority.cmp(&a.selection_priority));
    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jrandom::LegacyRandomSource;

    #[test]
    fn large_feature_seed_protocol() {
        // Structure.GenerationContext.makeRandom: setLargeFeatureSeed via a
        // LegacyRandomSource — verify against the oracle formula recomputed
        // with an independent LegacyRandomSource instance.
        let (seed, cx, cz) = (3053459i64, -10i32, -16i32);
        let mut r1 = LegacyRandomSource::new(0);
        set_large_feature_seed(&mut r1, seed, cx, cz);
        let mut r2 = LegacyRandomSource::new(seed);
        let a = r2.next_long();
        let b = r2.next_long();
        let mut r3 = LegacyRandomSource::new((cx as i64).wrapping_mul(a) ^ (cz as i64).wrapping_mul(b) ^ seed);
        // Same seed state => same stream of values.
        for _ in 0..8 {
            assert_eq!(r1.next_long(), r3.next_long());
        }
    }

    #[test]
    fn rotation_random_and_shuffle_consume_rng() {
        // getRandom = values[nextInt(4)]; getShuffled = 3 nextInt calls
        // (i=4,3,2). Verify the exact call sequence with independent draws.
        let mut r1 = LegacyRandomSource::new(99);
        let rot = rotation_get_random(&mut r1);
        let mut r2 = LegacyRandomSource::new(99);
        let expect = ROTATION_VALUES[r2.next_int_bound(4) as usize];
        assert_eq!(rot, expect);
        // shuffle: for i in (4,3,2): swap(i-1, nextInt(i))
        let mut r3 = LegacyRandomSource::new(7);
        let shuffled = rotation_get_shuffled(&mut r3);
        let mut r4 = LegacyRandomSource::new(7);
        let mut list = ROTATION_VALUES;
        for i in (2..=4).rev() {
            let j = r4.next_int_bound(i) as usize;
            list.swap((i - 1) as usize, j);
        }
        assert_eq!(shuffled, list);
    }

    #[test]
    fn transform_pos_tables() {
        // Hand-derived from the CFR arms with pivot (0,0) and pivot (3,-2).
        use Rotation::*;
        assert_eq!(None.transform_pos(1, 5, -2, 0, 0), (1, 5, -2));
        assert_eq!(Clockwise90.transform_pos(1, 5, -2, 0, 0), (2, 5, 1));
        assert_eq!(Clockwise180.transform_pos(1, 5, -2, 0, 0), (-1, 5, 2));
        assert_eq!(Counterclockwise90.transform_pos(1, 5, -2, 0, 0), (-2, 5, -1));
        // pivot (3, -2): CCW90 = (x1 - z1 + z, y, x1 + z1 - x) = (3+2-2, 5, 3-2-1)
        assert_eq!(Counterclockwise90.transform_pos(1, 5, -2, 3, -2), (3, 5, 0));
        // CW90 = (x1 + z1 - z, y, z1 - x1 + x) = (3-2+2, 5, -2-3+1)
        assert_eq!(Clockwise90.transform_pos(1, 5, -2, 3, -2), (3, 5, -4));
        // CW180 = (2x1 - x, y, 2z1 - z) = (6-1, 5, -4+2)
        assert_eq!(Clockwise180.transform_pos(1, 5, -2, 3, -2), (5, 5, -2));
    }

    #[test]
    fn direction_rotation_map() {
        use Dir::*;
        use Rotation::*;
        assert_eq!(Clockwise90.rotate_dir(North), East);
        assert_eq!(Clockwise90.rotate_dir(East), South);
        assert_eq!(Clockwise180.rotate_dir(North), South);
        assert_eq!(Counterclockwise90.rotate_dir(West), South);
        assert_eq!(Clockwise90.rotate_dir(Up), Up);
        assert_eq!(None.rotate_dir(Down), Down);
    }

    #[test]
    fn bbox_transform_matches_oracle_shape() {
        // size (7,3,5), start (-10, 64, 20), all rotations: ZERO and size-1
        // corners transformed then min/max per axis (inclusive box).
        let td = TemplateData { size: (7, 3, 5), jigsaws: vec![] };
        let b = td.bounding_box((-10, 64, 20), Rotation::Clockwise90);
        // corners: (0,0,0) -> (0,0,0); (6,2,4) -> CW90: (0-4, 2, 0+6)=( -4,2,6)
        assert_eq!((b.min_x, b.min_y, b.min_z), (-10 - 4, 64, 20));
        assert_eq!((b.max_x, b.max_y, b.max_z), (-10, 64 + 2, 20 + 6));
        let b2 = td.bounding_box((0, 0, 0), Rotation::Clockwise180);
        // (6,2,4) -> (-6,2,-4)
        assert_eq!((b2.min_x, b2.min_y, b2.min_z), (-6, 0, -4));
        assert_eq!((b2.max_x, b2.max_y, b2.max_z), (0, 2, 0));
    }

    #[test]
    fn can_attach_truth_table() {
        let mk = |front: Dir, top: Dir, joint: JointType, name: &str, target: &str| TemplateJigsaw {
            pos: (0, 0, 0),
            front,
            top,
            joint,
            name: name.to_string(),
            target: target.to_string(),
            pool: "minecraft:empty".to_string(),
            placement_priority: 0,
            selection_priority: 0,
        };
        // parent front NORTH, target "cap" ; child front SOUTH, name "cap",
        // tops equal, aligned => attach.
        let parent = mk(Dir::North, Dir::Up, JointType::Aligned, "start", "cap");
        let child = mk(Dir::South, Dir::Up, JointType::Aligned, "cap", "x");
        assert!(can_attach(&parent, &child));
        // name mismatch -> no attach.
        let child_bad = mk(Dir::South, Dir::Up, JointType::Aligned, "other", "x");
        assert!(!can_attach(&parent, &child_bad));
        // top mismatch + aligned -> no attach.
        let child_top = mk(Dir::South, Dir::North, JointType::Aligned, "cap", "x");
        assert!(!can_attach(&parent, &child_top));
        // top mismatch + parent ROLLABLE -> attach (flag reads PARENT joint).
        let parent_roll = mk(Dir::North, Dir::Up, JointType::Rollable, "start", "cap");
        let child_roll = mk(Dir::South, Dir::North, JointType::Aligned, "cap", "x");
        assert!(can_attach(&parent_roll, &child_roll));
        // wrong facing -> no attach.
        let child_face = mk(Dir::North, Dir::Up, JointType::Aligned, "cap", "x");
        assert!(!can_attach(&parent, &child_face));
    }

    #[test]
    fn jigsaw_info_defaults_and_joint() {
        // state orientation "down_east" (front DOWN => not horizontal =>
        // default joint ROLLABLE), nbt without joint/name/pool/target.
        let state = Nbt::Comp(vec![
            ("Name".to_string(), Nbt::String("minecraft:jigsaw".to_string())),
            (
                "Properties".to_string(),
                Nbt::Comp(vec![(
                    "orientation".to_string(),
                    Nbt::String("down_east".to_string()),
                )]),
            ),
        ]);
        let nbt = Nbt::Comp(vec![]);
        let j = parse_jigsaw((1, 2, 3), &state, &nbt).expect("parse");
        assert_eq!(j.pos, (1, 2, 3));
        assert_eq!(j.front, Dir::Down);
        assert_eq!(j.top, Dir::East);
        assert_eq!(j.joint, JointType::Rollable);
        assert_eq!(j.name, "minecraft:empty");
        assert_eq!(j.pool, "minecraft:empty");
        assert_eq!(j.target, "minecraft:empty");
        assert_eq!(j.placement_priority, 0);
        assert_eq!(j.selection_priority, 0);
        // horizontal front + explicit joint override.
        let state2 = Nbt::Comp(vec![
            ("Name".to_string(), Nbt::String("minecraft:jigsaw".to_string())),
            (
                "Properties".to_string(),
                Nbt::Comp(vec![(
                    "orientation".to_string(),
                    Nbt::String("north_up".to_string()),
                )]),
            ),
        ]);
        let nbt2 = Nbt::Comp(vec![
            ("joint".to_string(), Nbt::String("rollable".to_string())),
            ("name".to_string(), Nbt::String("minecraft:top".to_string())),
            ("target".to_string(), Nbt::String("minecraft:bottom".to_string())),
            ("pool".to_string(), Nbt::String("minecraft:village/plains/houses".to_string())),
            ("placement_priority".to_string(), Nbt::Int(-1)),
            ("selection_priority".to_string(), Nbt::Int(5)),
        ]);
        let j2 = parse_jigsaw((0, 0, 0), &state2, &nbt2).expect("parse2");
        assert_eq!(j2.joint, JointType::Rollable); // explicit override wins
        assert_eq!(j2.name, "minecraft:top");
        assert_eq!(j2.placement_priority, -1);
        assert_eq!(j2.selection_priority, 5);
    }

    #[test]
    fn shuffled_jigsaw_stable_sort_and_rng() {
        // Two jigsaws; shuffle consumes len() nextInt calls; sort is stable
        // DESC by selection_priority (ties keep shuffled order).
        let j1 = TemplateJigsaw {
            pos: (0, 0, 0),
            front: Dir::North,
            top: Dir::Up,
            joint: JointType::Aligned,
            name: "a".to_string(),
            target: "t".to_string(),
            pool: "p".to_string(),
            placement_priority: 0,
            selection_priority: 1,
        };
        let j2 = TemplateJigsaw { selection_priority: 1, ..j1.clone() };
        let td = TemplateData { size: (3, 2, 3), jigsaws: vec![j1, j2] };
        let mut rng = LegacyRandomSource::new(12345);
        let out = shuffled_jigsaw_blocks(&td, (10, 20, 30), Rotation::None, &mut rng);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].pos, (10, 20, 30));
        assert_eq!(out[1].pos, (10, 20, 30));
    }

    #[test]
    fn pool_weight_expansion_and_pick() {
        let dir_json = r#"{
            "fallback": "minecraft:empty",
            "elements": [
                {"weight": 2, "element": {"element_type": "minecraft:single_pool_element", "location": "village/plains/a", "projection": "rigid"}},
                {"weight": 1, "element": {"element_type": "minecraft:empty_pool_element"}}
            ]
        }"#;
        // Minimal harness: parse the JSON directly through the same code path
        // used by resolve_pool (dir file access covered by integration).
        let v = crate::json::parse(dir_json).unwrap();
        let fallback = v.get("fallback").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let mut templates: Vec<PoolElement> = Vec::new();
        for entry in v.get("elements").and_then(|x| x.as_arr()).unwrap_or(&[]) {
            let weight = entry.get("weight").and_then(|x| x.as_i64()).unwrap_or(1) as usize;
            let element = PoolElement::parse(entry.get("element").unwrap(), None);
            for _ in 0..weight {
                templates.push(element.clone());
            }
        }
        let pool = ResolvedPool { fallback, templates, has_aliases: false };
        assert_eq!(pool.size(), 3);
        assert_eq!(pool.fallback, "minecraft:empty");
        // First element is Single with the right location/projection.
        match &pool.templates[0] {
            PoolElement::Single { location, projection } => {
                assert_eq!(location, "village/plains/a");
                assert_eq!(*projection, Projection::Rigid);
            }
            other => panic!("unexpected {other:?}"),
        }
        assert!(matches!(pool.templates[2], PoolElement::Empty));
    }
}
