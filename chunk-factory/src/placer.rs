//! NCF P5.3 increment 2b (Job 441690) — JigsawPlacement.addPieces +
//! Placer.tryPlacingChildren, ported bit-exact from the 1.21.10 CFR oracles:
//!   JigsawPlacement.java (65-105 addPieces, 107-114 isStartTooClose,
//!     116-122 getRandomNamedJigsaw, 124-131 addPieces, 152-161 stub lambda,
//!     167-310 Placer.tryPlacingChildren)
//!   Shapes.create (81-99): span < 1.0E-7 on ANY axis => EMPTY; world-range
//!     coords => findBits -1 => exact-double ArrayVoxelShape (single cell
//!     [min,max] per axis) — hence the exact box algebra below.
//!   BooleanOp: ONLY_FIRST = a && !b, ONLY_SECOND = b && !a.
//!   AABB.of(BoundingBox) = (minX,minY,minZ, maxX+1,maxY+1,maxZ+1);
//!     deflate(v) = inflate(-v) = min+v / max-v per axis.
//!   BoundingBox: moved (offset), isInside (all axes INCLUSIVE),
//!     getYSpan = maxY-minY+1, encapsulate(BlockPos) grows in place;
//!     StructurePiece.move MUTATES the box object in place — therefore the
//!     top-level free shape subtracts the POST-MOVE start bbox (lambda
//!     capture aliases the same object).
//!   StructureTemplatePool.getMaxSize (82-87): lazy-cached, over the
//!     WEIGHT-EXPANDED templates, filter != EmptyPoolElement, YSpan at
//!     BlockPos.ZERO / Rotation.NONE, max orElse 0.
//!   JigsawJunction (17-30): sourceX, sourceGroundY, sourceZ, deltaY,
//!     destProjection; junctions recorded on BOTH pieces (CFR 294-295).
//!   SequencedPriorityIterator: decompiled fresh from the mapped jar —
//!     add fast-path (priority == highest => addLast to the current queue),
//!     re-point when priority >= highest, pop FIFO, on drain switch to the
//!     max-priority non-empty queue (the early break at highestPrio-1 is
//!     equivalent to a full max-scan: at most ONE queue holds the key
//!     highestPrio-1 and every key above it was just drained => the break
//!     fires exactly when the true max is highestPrio-1; hash order cannot
//!     change the outcome — full scan used here, deterministic).
//!   ChunkGenerator.getFirstFreeHeight == getBaseHeight (572-574);
//!     NoiseBasedChunkGenerator.iterateNoiseColumn (157-200) samples the RAW
//!     noise column with DensityFunctions.BeardifierMarker (= 0 beard
//!     contribution, matching Df::Beardifier = 0.0 pre-wiring) and stops at
//!     the first NOT_AIR state (Heightmap.Types.WORLD_SURFACE_WG, 142),
//!     returning that y + 1. The REAL sampler lands with increment 2c; here
//!     it is injected via the FirstFreeHeight trait (no RNG ever consumed).
//!
//! NOT in this increment (2c): the real noise-column sampler, structure-level
//! findGenerationPoint wiring, pool_aliases beyond identity, piece_dump oracle
//! capture. Junctions/pieces feed the Beardifier at 2d (increment 1 ready).
//!
//! ZERO behavior change: standalone module, nothing in the tree calls it yet.

use crate::beardifier::InclusiveBox;
use crate::jigsaw::{
    can_attach, rotation_get_random, rotation_get_shuffled, shuffled_jigsaw_blocks, util_shuffled_copy,
    Dir, PoolElement, Projection, ResolvedPool, Rotation, TemplateData, TemplateJigsaw,
};
use crate::jrandom::RandomSource;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Exact free-space algebra — VoxelShape over single-box ArrayVoxelShapes.
//
// Java join semantics on these shapes are CELLS of the merged coordinate
// grid: a cell [u_i, u_{i+1}) is covered by a shape iff it lies fully within
// one of the shape's covered closed boxes (its own coords always split the
// merged grid exactly). Therefore:
//   joinIsNotEmpty(A, B, ONLY_SECOND)  <=>  B \ union(A) has positive measure
//   joinUnoptimized(A, B, ONLY_FIRST)  =   A \ B as an exact point set
// A union of disjoint closed boxes with strict cut conditions reproduces both
// exactly (all coordinates are integers or quarter-integers — f64 exact).
// ---------------------------------------------------------------------------

/// An AABB with exact f64 coordinates (world ints and 0.25 offsets only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DBox {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
}

/// Shapes.create epsilon: any span < 1.0E-7 => EMPTY shape.
const SHAPE_EPS: f64 = 1.0E-7;

impl DBox {
    /// Shapes.create(AABB.of(BoundingBox)) — inclusive int box to the exact
    /// double cell [min, max+1).
    pub fn from_inclusive_box(b: &InclusiveBox) -> DBox {
        DBox {
            x0: b.min_x as f64,
            y0: b.min_y as f64,
            z0: b.min_z as f64,
            x1: b.max_x as f64 + 1.0,
            y1: b.max_y as f64 + 1.0,
            z1: b.max_z as f64 + 1.0,
        }
    }

    pub fn spans_positive(&self) -> bool {
        self.x1 - self.x0 >= SHAPE_EPS && self.y1 - self.y0 >= SHAPE_EPS && self.z1 - self.z0 >= SHAPE_EPS
    }

    /// AABB.deflate(v) = inflate(-v): min += v, max -= v per axis.
    pub fn deflate(&self, v: f64) -> DBox {
        DBox {
            x0: self.x0 + v,
            y0: self.y0 + v,
            z0: self.z0 + v,
            x1: self.x1 - v,
            y1: self.y1 - v,
            z1: self.z1 - v,
        }
    }
}

/// One closed-box subtraction: R \ C as up to 6 disjoint boxes with STRICT
/// cut conditions (touching boxes remove nothing — the half-open cell
/// convention). Every returned piece has positive spans.
fn subtract_one(r: DBox, c: DBox) -> Vec<DBox> {
    // overlap must have positive measure on all axes
    if c.x0 >= r.x1 || c.x1 <= r.x0 || c.y0 >= r.y1 || c.y1 <= r.y0 || c.z0 >= r.z1 || c.z1 <= r.z0 {
        return vec![r];
    }
    let mut out = Vec::with_capacity(6);
    // x slabs (full R y/z)
    if r.x0 < c.x0 {
        out.push(DBox { x1: c.x0, ..r });
    }
    if c.x1 < r.x1 {
        out.push(DBox { x0: c.x1, ..r });
    }
    // y slabs clipped to the x-overlap
    let x = DBox { x0: r.x0.max(c.x0), x1: r.x1.min(c.x1), ..r };
    if x.x1 - x.x0 >= SHAPE_EPS {
        if r.y0 < c.y0 {
            out.push(DBox { y1: c.y0, ..x });
        }
        if c.y1 < r.y1 {
            out.push(DBox { y0: c.y1, ..x });
        }
        // z slabs clipped to the x/y-overlap
        let xy = DBox { y0: r.y0.max(c.y0), y1: r.y1.min(c.y1), ..x };
        if xy.y1 - xy.y0 >= SHAPE_EPS {
            if r.z0 < c.z0 {
                out.push(DBox { z1: c.z0, ..xy });
            }
            if c.z1 < r.z1 {
                out.push(DBox { z0: c.z1, ..xy });
            }
        }
    }
    out
}

/// The free-space shape: a union of disjoint closed boxes (empty vec = no
/// free space — every placement collides, matching Java's EMPTY shape).
#[derive(Debug, Clone, Default)]
pub struct FreeShape(pub Vec<DBox>);

impl FreeShape {
    /// Shapes.create(AABB.of(box)) — single box, or EMPTY when degenerate.
    pub fn from_inclusive_box(b: &InclusiveBox) -> FreeShape {
        let d = DBox::from_inclusive_box(b);
        if d.spans_positive() {
            FreeShape(vec![d])
        } else {
            FreeShape::default()
        }
    }

    pub fn from_dbox(d: DBox) -> FreeShape {
        if d.spans_positive() {
            FreeShape(vec![d])
        } else {
            FreeShape::default()
        }
    }

    /// Shapes.join(create(a), create(b), ONLY_FIRST) = a \ b.
    pub fn join_only_first(a: DBox, b: DBox) -> FreeShape {
        let mut s = FreeShape::from_dbox(a);
        s.subtract(b);
        s
    }

    /// Shapes.joinIsNotEmpty(A, B, ONLY_SECOND): B minus union(self) has a
    /// positive-measure survivor.
    pub fn only_second_nonempty(&self, b: DBox) -> bool {
        if !b.spans_positive() {
            return false; // Shapes.create(B) == EMPTY
        }
        let mut cur = vec![b];
        for c in &self.0 {
            let mut next = Vec::new();
            for r in cur {
                next.extend(subtract_one(r, *c));
            }
            cur = next;
            if cur.is_empty() {
                return false;
            }
        }
        !cur.is_empty()
    }

    /// Shapes.joinUnoptimized(A, B, ONLY_FIRST): self <- self \ B.
    /// Degenerate B is the EMPTY shape — no-op (matches Java: ONLY_FIRST
    /// with an empty second operand keeps A unchanged).
    pub fn subtract(&mut self, c: DBox) {
        if !c.spans_positive() {
            return;
        }
        let mut next = Vec::with_capacity(self.0.len());
        for r in self.0.drain(..) {
            next.extend(subtract_one(r, c));
        }
        self.0 = next;
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// ---------------------------------------------------------------------------
// SequencedPriorityIterator (net.minecraft.util) — exact port.
// ---------------------------------------------------------------------------

/// Exact port of the decompiled SequencedPriorityIterator. The invariant
/// `highest_prio != MIN => queues[highest_prio] is non-empty and is the
/// queue Java's highestPrioQueue references` lets the current queue be
/// resolved through the map (Java's reference is always that same object).
pub struct SequencedPriorityIterator<T> {
    queues: BTreeMap<i32, VecDeque<T>>,
    highest_prio: i32,
}

impl<T> Default for SequencedPriorityIterator<T> {
    fn default() -> Self {
        SequencedPriorityIterator { queues: BTreeMap::new(), highest_prio: i32::MIN }
    }
}

impl<T> SequencedPriorityIterator<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, value: T, priority: i32) {
        if priority == self.highest_prio && self.highest_prio != i32::MIN {
            // Java fast path: highestPrioQueue.addLast(value)
            self.queues.get_mut(&priority).expect("highest queue present").push_back(value);
        } else {
            let deque = self.queues.entry(priority).or_default();
            deque.push_back(value);
            if priority >= self.highest_prio {
                self.highest_prio = priority;
            }
        }
    }

    /// computeNext: pop FIFO from the highest-priority queue; on drain,
    /// switch to the max-priority NON-EMPTY queue (full scan — proven
    /// equivalent to the decompiled early break at highestPrio-1).
    pub fn next(&mut self) -> Option<T> {
        if self.highest_prio == i32::MIN {
            return None;
        }
        let v = self.queues.get_mut(&self.highest_prio).expect("highest queue present").pop_front().expect("non-empty invariant");
        self.switch_to_next_highest();
        Some(v)
    }

    fn switch_to_next_highest(&mut self) {
        let mut best = i32::MIN;
        for (k, q) in &self.queues {
            if *k > best && !q.is_empty() {
                best = *k;
            }
        }
        self.highest_prio = best;
    }
}

// ---------------------------------------------------------------------------
// Assembly model
// ---------------------------------------------------------------------------

/// JigsawJunction (CFR 17-30).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Junction {
    pub source_x: i32,
    pub source_ground_y: i32,
    pub source_z: i32,
    pub delta_y: i32,
    pub dest_projection: Projection,
}

/// PoolElementStructurePiece (CFR 37-55): element + position + groundLevelDelta
/// + rotation + bbox + junctions (assembly-relevant fields only).
#[derive(Debug, Clone)]
pub struct Piece {
    pub element: PoolElement,
    pub position: (i32, i32, i32),
    pub ground_level_delta: i32,
    pub rotation: Rotation,
    pub bounding_box: InclusiveBox,
    pub junctions: Vec<Junction>,
}

impl Piece {
    /// StructurePiece.move + PoolElementStructurePiece.move: bbox mutated IN
    /// PLACE and position offset (CFR StructurePiece.java 427-429).
    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) {
        self.position = (self.position.0 + dx, self.position.1 + dy, self.position.2 + dz);
        self.bounding_box.min_x += dx;
        self.bounding_box.min_y += dy;
        self.bounding_box.min_z += dz;
        self.bounding_box.max_x += dx;
        self.bounding_box.max_y += dy;
        self.bounding_box.max_z += dz;
    }
}

/// A shared mutable shape slot — Java MutableObject<VoxelShape> aliasing.
/// The per-piece slot starts None (lazily the piece bbox); the inherited
/// `free` slot is always Some by construction (top-level init + the lazy
/// init happens before any child can inherit it).
pub type ShapeSlot = Rc<RefCell<Option<FreeShape>>>;

/// PieceState (CFR 308-309): piece index into Placer.pieces + the shape slot
/// the piece was placed with + depth.
#[derive(Debug, Clone)]
pub struct PieceState {
    pub piece_index: usize,
    pub free: ShapeSlot,
    pub depth: i32,
}

/// The WORLD_SURFACE_WG first-free-height sampler (ChunkGenerator
/// .getFirstFreeHeight == getBaseHeight, CFR 572-574). NEVER consumes the
/// structure RNG. The real noise-column sampler is increment 2c.
pub trait FirstFreeHeight {
    fn first_free_height(&mut self, x: i32, z: i32) -> i32;
}

/// Registry surface for pools + templates. `resolve` is the alias-resolved
/// registry lookup (aliases are IDENTITY in 2b — PoolAliasBinding port is
/// the next increment); `max_size` is StructureTemplatePool.getMaxSize with
/// implementor-side caching (missing pool => 0, matching the `.orElse(0)`
/// at the expansion-hack call sites, CFR 247-249).
pub trait PoolSource {
    fn resolve(&self, key: &str) -> Option<ResolvedPool>;
    fn template_of(&mut self, element: &PoolElement) -> Option<TemplateData>;
    fn max_size(&mut self, key: &str) -> i32;
    // loud divergence counters (Java: LOGGER.warn / crash paths)
    fn note_missing_pool(&mut self, key: &str);
    fn note_missing_template(&mut self, location: &str);
    fn note_unsupported(&mut self, kind: &str);
}

/// minecraft:empty — the Pools.EMPTY registry key (holder identity checks,
/// CFR 210/215: `size() == 0 && !is(Pools.EMPTY)`).
fn is_empty_holder(key: &str) -> bool {
    key == "minecraft:empty"
}

/// Placer (CFR 167-183).
pub struct Placer<'a, R: RandomSource, S: FirstFreeHeight, P: PoolSource> {
    pools: &'a mut P,
    max_depth: i32,
    pieces: &'a mut Vec<Piece>,
    rng: &'a mut R,
    sampler: &'a mut S,
    use_expansion_hack: bool,
    placing: SequencedPriorityIterator<PieceState>,
}

impl<'a, R: RandomSource, S: FirstFreeHeight, P: PoolSource> Placer<'a, R, S, P> {
    pub fn new(
        pools: &'a mut P,
        max_depth: i32,
        pieces: &'a mut Vec<Piece>,
        rng: &'a mut R,
        sampler: &'a mut S,
        use_expansion_hack: bool,
    ) -> Self {
        Placer { pools, max_depth, pieces, rng, sampler, use_expansion_hack, placing: SequencedPriorityIterator::new() }
    }

    // -- element views (bbox + jigsaws, exact RNG consumption) --------------

    /// StructurePoolElement.getBoundingBox(manager, offset, rotation).
    /// EmptyPoolElement: bbox is forbidden (filtered upstream — a None here
    /// is a loud divergence). List: union of children. Missing template:
    /// Java's StructureTemplateManager.get().orElseThrow() crashes; we note
    /// loudly and return None (the caller skips / the fallback policy owns
    /// the chunk).
    fn element_bbox(&mut self, e: &PoolElement, offset: (i32, i32, i32), rot: Rotation) -> Option<InclusiveBox> {
        match e {
            PoolElement::Empty => None,
            PoolElement::Single { location, .. } => {
                let t = self.template(location)?;
                Some(t.bounding_box(offset, rot))
            }
            PoolElement::List { elements, .. } => {
                let mut acc: Option<InclusiveBox> = None;
                for c in elements {
                    let b = self.element_bbox(c, offset, rot)?;
                    acc = Some(match acc {
                        None => b,
                        Some(a) => InclusiveBox::encapsulating(&a, &b),
                    });
                }
                acc
            }
            PoolElement::Unsupported { kind } => {
                self.pools.note_unsupported(kind);
                None
            }
        }
    }

    fn template(&mut self, location: &str) -> Option<TemplateData> {
        match self.pools.template_of(&PoolElement::Single { location: location.to_string(), projection: Projection::Rigid }) {
            Some(t) => Some(t),
            None => {
                self.pools.note_missing_template(location);
                None
            }
        }
    }

    /// StructurePoolElement.getShuffledJigsawBlocks — shuffle + stable
    /// selectionPriority-DESC sort (Single/List via jigsaw.rs, which consumes
    /// RNG exactly: util_shuffle of the jigsaw list). Empty: no jigsaws, NO
    /// RNG (EmptyPoolElement returns List.of()). Unsupported: loud note, no
    /// RNG (Java would consume — divergence is counted, fallback owns it).
    fn element_jigsaws(&mut self, e: &PoolElement, offset: (i32, i32, i32), rot: Rotation) -> Vec<TemplateJigsaw> {
        match e {
            PoolElement::Empty => Vec::new(),
            PoolElement::Single { location, .. } => match self.template(location) {
                Some(t) => shuffled_jigsaw_blocks(&t, offset, rot, self.rng),
                None => Vec::new(),
            },
            PoolElement::List { elements, .. } => match elements.first() {
                Some(first) => self.element_jigsaws(first, offset, rot),
                None => {
                    self.pools.note_unsupported("list_pool_element:empty");
                    Vec::new()
                }
            },
            PoolElement::Unsupported { kind } => {
                self.pools.note_unsupported(kind);
                Vec::new()
            }
        }
    }

    // -- Placer.tryPlacingChildren (CFR 185-305) ----------------------------

    pub fn try_placing_children(&mut self, piece_index: usize, free: ShapeSlot, depth: i32) {
        let (element, position, rotation, bounding_box, min_y, ground_level_delta, parent_projection) = {
            let p = &self.pieces[piece_index];
            (
                p.element.clone(),
                p.position,
                p.rotation,
                p.bounding_box,
                p.bounding_box.min_y,
                p.ground_level_delta,
                p.element.projection(),
            )
        };
        let flag = parent_projection == Projection::Rigid;
        let per_piece: ShapeSlot = Rc::new(RefCell::new(None));
        let parent_jigsaws = self.element_jigsaws(&element, position, rotation);
        'parent: for parent_jig in parent_jigsaws {
            let structure_block_info = parent_jig.clone();
            let front_facing = structure_block_info.front; // rotated by jigsaws_at
            let block_pos = structure_block_info.pos;      // parent jigsaw ABS pos
            let block_pos1 = relative(block_pos, front_facing);
            let i = block_pos.1 - min_y;
            let mut i1 = i32::MIN; // lazy firstFreeHeight memo per parent jigsaw
            let pool_key = structure_block_info.pool.clone(); // alias: identity (2b)
            let Some(pool) = self.pools.resolve(&pool_key) else {
                self.pools.note_missing_pool(&pool_key);
                continue;
            };
            if pool.size() == 0 && !is_empty_holder(&pool_key) {
                self.pools.note_missing_pool(&pool_key);
                continue;
            }
            let fallback_key = pool.fallback.clone();
            let Some(fallback) = self.pools.resolve(&fallback_key) else {
                self.pools.note_missing_pool(&fallback_key);
                continue;
            };
            if fallback.size() == 0 && !is_empty_holder(&fallback_key) {
                self.pools.note_missing_pool(&fallback_key);
                continue;
            }
            let is_inside = bounding_box.is_inside(block_pos1.0, block_pos1.1, block_pos1.2);
            let slot: ShapeSlot = if is_inside {
                let mut guard = per_piece.borrow_mut();
                if guard.is_none() {
                    *guard = Some(FreeShape::from_inclusive_box(&bounding_box));
                }
                per_piece.clone()
            } else {
                free.clone()
            };
            // candidate list: pool templates (only when depth != maxDepth)
            // + fallback templates (ALWAYS) — exact RNG order (CFR 229-232).
            let mut candidates: Vec<PoolElement> = Vec::new();
            if depth != self.max_depth {
                candidates.extend(pool.get_shuffled_templates(self.rng));
            }
            candidates.extend(fallback.get_shuffled_templates(self.rng));
            let placement_priority = structure_block_info.placement_priority;
            for cand in candidates {
                if matches!(cand, PoolElement::Empty) {
                    break; // while hasNext && != EmptyPoolElement.INSTANCE
                }
                for rot in rotation_get_shuffled(self.rng) {
                    // candidate jigsaws FIRST (RNG), bbox second (CFR 237-238)
                    let shuffled = self.element_jigsaws(&cand, (0, 0, 0), rot);
                    let Some(bbox1) = self.element_bbox(&cand, (0, 0, 0), rot) else {
                        continue;
                    };
                    // expansion hack (CFR 239-250)
                    let i2 = if self.use_expansion_hack && bbox1.get_yspan() <= 16 {
                        shuffled
                            .iter()
                            .map(|info| {
                                let nb = relative(info.pos, info.front);
                                if !bbox1.is_inside(nb.0, nb.1, nb.2) {
                                    return 0;
                                }
                                let k = info.pool.clone();
                                let a = self.pools.max_size(&k);
                                let b = self.pools.resolve(&k).map(|p| self.pools.max_size(&p.fallback)).unwrap_or(0);
                                a.max(b)
                            })
                            .max()
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    for child_jig in &shuffled {
                        if !can_attach(&parent_jig, child_jig) {
                            continue;
                        }
                        let block_pos2 = child_jig.pos;
                        let block_pos3 = (block_pos1.0 - block_pos2.0, block_pos1.1 - block_pos2.1, block_pos1.2 - block_pos2.2);
                        let Some(bbox2) = self.element_bbox(&cand, block_pos3, rot) else {
                            continue;
                        };
                        let min_y1 = bbox2.min_y;
                        let flag1 = cand.projection() == Projection::Rigid;
                        let y = block_pos2.1;
                        let i3 = i - y + front_facing.step_y();
                        let i4 = if flag && flag1 {
                            min_y + i3
                        } else {
                            if i1 == i32::MIN {
                                i1 = self.sampler.first_free_height(block_pos.0, block_pos.2);
                            }
                            i1 - y
                        };
                        let i5 = i4 - min_y1;
                        let mut bbox3 = bbox2.moved(0, i5, 0);
                        let block_pos4 = (block_pos3.0, block_pos3.1 + i5, block_pos3.2);
                        if i2 > 0 {
                            let max = (i2 + 1).max(bbox3.get_yspan());
                            bbox3.encapsulate_pos(bbox3.min_x, bbox3.min_y + max, bbox3.min_z);
                        }
                        // collision: B' = bbox3.deflate(0.25) vs the slot shape
                        let collides = {
                            let guard = slot.borrow();
                            let shape = guard.as_ref().expect("free slot initialized");
                            shape.only_second_nonempty(DBox::from_inclusive_box(&bbox3).deflate(0.25))
                        };
                        if collides {
                            continue;
                        }
                        {
                            let mut guard = slot.borrow_mut();
                            let shape = guard.as_mut().expect("free slot initialized");
                            shape.subtract(DBox::from_inclusive_box(&bbox3));
                        }
                        let i6 = if flag1 { ground_level_delta - i3 } else { cand.ground_level_delta() };
                        let mut new_piece = Piece {
                            element: cand.clone(),
                            position: block_pos4,
                            ground_level_delta: i6,
                            rotation: rot,
                            bounding_box: bbox3,
                            junctions: Vec::new(),
                        };
                        let i7 = if flag {
                            min_y + i
                        } else if flag1 {
                            i4 + y
                        } else {
                            if i1 == i32::MIN {
                                i1 = self.sampler.first_free_height(block_pos.0, block_pos.2);
                            }
                            i1 + i3 / 2 // Java int division truncation
                        };
                        // junctions on BOTH pieces (CFR 294-295)
                        self.pieces[piece_index].junctions.push(Junction {
                            source_x: block_pos1.0,
                            source_ground_y: i7 - i + ground_level_delta,
                            source_z: block_pos1.2,
                            delta_y: i3,
                            dest_projection: cand.projection(),
                        });
                        new_piece.junctions.push(Junction {
                            source_x: block_pos.0,
                            source_ground_y: i7 - y + i6,
                            source_z: block_pos.2,
                            delta_y: -i3,
                            dest_projection: parent_projection,
                        });
                        let new_index = self.pieces.len();
                        self.pieces.push(new_piece);
                        if depth + 1 > self.max_depth {
                            continue 'parent;
                        }
                        self.placing.add(PieceState { piece_index: new_index, free: slot.clone(), depth: depth + 1 }, placement_priority);
                        continue 'parent;
                    }
                }
            }
        }
    }

    /// addPieces private overload (CFR 124-131): try children of the start
    /// piece, then drain the SequencedPriorityIterator.
    pub fn run(&mut self, start_index: usize, free: ShapeSlot) {
        self.try_placing_children(start_index, free, 0);
        while let Some(state) = self.placing.next() {
            self.try_placing_children(state.piece_index, state.free, state.depth);
        }
    }
}

fn relative(pos: (i32, i32, i32), d: Dir) -> (i32, i32, i32) {
    let (dx, dy, dz) = dir_step(d);
    (pos.0 + dx, pos.1 + dy, pos.2 + dz)
}

fn dir_step(d: Dir) -> (i32, i32, i32) {
    match d {
        Dir::Down => (0, -1, 0),
        Dir::Up => (0, 1, 0),
        Dir::North => (0, 0, -1),
        Dir::South => (0, 0, 1),
        Dir::West => (-1, 0, 0),
        Dir::East => (1, 0, 0),
    }
}

// ---------------------------------------------------------------------------
// JigsawPlacement.addPieces entry (CFR 65-105 + stub lambda 152-161)
// ---------------------------------------------------------------------------

/// Structure-level assembly inputs (village_plains @ 3053459: pool
/// minecraft:village/plains/town_centers, depth 6, WORLD_SURFACE_WG,
/// expansion hack TRUE, maxDistance 80x4064, padding ZERO — data/minecraft/
/// worldgen/structure/village_plains.json).
#[derive(Debug, Clone)]
pub struct AssemblyParams {
    pub start_pool: String,
    pub start_jigsaw_name: Option<String>,
    pub max_depth: i32,
    pub pos: (i32, i32, i32),
    pub use_expansion_hack: bool,
    /// project_start_to_heightmap == WORLD_SURFACE_WG (the only worldgen type)
    pub project_start_to_heightmap: bool,
    /// JigsawStructure.MaxDistance (horizontal, vertical)
    pub max_distance: (i32, i32),
    /// DimensionPadding (bottom, top); (0,0) == DimensionPadding.ZERO
    pub dimension_padding: (i32, i32),
    /// LevelHeightAccessor bounds: min inclusive, max INCLUSIVE
    pub level_min_y: i32,
    pub level_max_y: i32,
}

pub struct AssemblyResult {
    /// Structure.GenerationStub.position = (centerX, i4, centerZ)
    pub stub_position: (i32, i32, i32),
    pub pieces: Vec<Piece>,
}

/// The exact addPieces flow. RNG order: (1) Rotation.getRandom (nextInt(4)),
/// (2) pool.getRandomTemplate (nextInt when size >= 2; weight-expanded), (3)
/// the start-jigsaw-name scan (one getShuffledJigsawBlocks) — nothing else.
pub fn add_pieces<R: RandomSource, S: FirstFreeHeight, P: PoolSource>(
    params: &AssemblyParams,
    pools: &mut P,
    rng: &mut R,
    sampler: &mut S,
) -> Option<AssemblyResult> {
    let rot = rotation_get_random(rng);
    // alias lookup is identity in 2b; Java falls back to the raw holder when
    // the alias-resolved key is absent — identity makes that unreachable for
    // a registered start pool. A missing pool here is a loud divergence.
    let Some(start_pool) = pools.resolve(&params.start_pool) else {
        pools.note_missing_pool(&params.start_pool);
        return None;
    };
    let start = start_pool.get_random_template(rng);
    if matches!(start, PoolElement::Empty) {
        return None;
    }
    let block_pos = match &params.start_jigsaw_name {
        Some(name) => {
            let jigsaws = element_jigsaws_entry(&start, params.pos, rot, pools, rng);
            let mut found = None;
            for j in jigsaws {
                if *name == j.name {
                    found = Some(j.pos);
                    break;
                }
            }
            found? // getRandomNamedJigsaw empty -> LOGGER.error + Optional.empty
        }
        None => params.pos,
    };
    let vec3i = (block_pos.0 - params.pos.0, block_pos.1 - params.pos.1, block_pos.2 - params.pos.2);
    let block_pos1 = (params.pos.0 - vec3i.0, params.pos.1 - vec3i.1, params.pos.2 - vec3i.2);
    let start_bbox = element_bbox_entry(&start, block_pos1, rot, pools)?;
    let mut start_piece = Piece {
        element: start.clone(),
        position: block_pos1,
        ground_level_delta: start.ground_level_delta(),
        rotation: rot,
        bounding_box: start_bbox,
        junctions: Vec::new(),
    };
    let i = (start_bbox.max_x + start_bbox.min_x) / 2; // Java truncating division
    let i1 = (start_bbox.max_z + start_bbox.min_z) / 2;
    let i2 = if !params.project_start_to_heightmap {
        block_pos1.1
    } else {
        params.pos.1 + sampler.first_free_height(i, i1)
    };
    let i3 = start_bbox.min_y + start_piece.ground_level_delta;
    // move MUTATES the bbox object in place (StructurePiece.java 427-429) —
    // every later use sees the POST-MOVE box.
    start_piece.move_by(0, i2 - i3, 0);
    if params.dimension_padding != (0, 0) {
        let lo = params.level_min_y + params.dimension_padding.0;
        let hi = params.level_max_y - params.dimension_padding.1;
        if start_piece.bounding_box.min_y < lo || start_piece.bounding_box.max_y > hi {
            return None; // isStartTooCloseToWorldHeightLimits
        }
    }
    let i4 = i2 + vec3i.1;
    let stub_position = (i, i4, i1);
    let mut pieces = vec![start_piece];
    if params.max_depth > 0 {
        let (h, v) = params.max_distance;
        let (pb, pt) = params.dimension_padding;
        let aabb = DBox {
            x0: (i - h) as f64,
            y0: (i4 - v).max(params.level_min_y + pb) as f64,
            z0: (i1 - h) as f64,
            x1: (i + h + 1) as f64,
            y1: (i4 + v + 1).min(params.level_max_y + 1 - pt) as f64,
            z1: (i1 + h + 1) as f64,
        };
        let mut free = FreeShape::from_dbox(aabb);
        free.subtract(DBox::from_inclusive_box(&pieces[0].bounding_box)); // POST-MOVE
        let free_slot: ShapeSlot = Rc::new(RefCell::new(Some(free)));
        let mut placer = Placer::new(pools, params.max_depth, &mut pieces, rng, sampler, params.use_expansion_hack);
        placer.run(0, free_slot);
    }
    Some(AssemblyResult { stub_position, pieces })
}

fn element_bbox_entry<P: PoolSource>(
    e: &PoolElement,
    offset: (i32, i32, i32),
    rot: Rotation,
    pools: &mut P,
) -> Option<InclusiveBox> {
    match e {
        PoolElement::Empty => None,
        PoolElement::Single { location, .. } => match pools.template_of(e) {
            Some(t) => Some(t.bounding_box(offset, rot)),
            None => {
                pools.note_missing_template(location);
                None
            }
        },
        PoolElement::List { elements, .. } => {
            let mut acc: Option<InclusiveBox> = None;
            for c in elements {
                let b = element_bbox_entry(c, offset, rot, pools)?;
                acc = Some(match acc {
                    None => b,
                    Some(a) => InclusiveBox::encapsulating(&a, &b),
                });
            }
            acc
        }
        PoolElement::Unsupported { kind } => {
            pools.note_unsupported(kind);
            None
        }
    }
}

fn element_jigsaws_entry<R: RandomSource, P: PoolSource>(
    e: &PoolElement,
    offset: (i32, i32, i32),
    rot: Rotation,
    pools: &mut P,
    rng: &mut R,
) -> Vec<TemplateJigsaw> {
    match e {
        PoolElement::Empty => Vec::new(),
        PoolElement::Single { .. } => match pools.template_of(e) {
            Some(t) => shuffled_jigsaw_blocks(&t, offset, rot, rng),
            None => Vec::new(),
        },
        PoolElement::List { elements, .. } => match elements.first() {
            Some(first) => element_jigsaws_entry(first, offset, rot, pools, rng),
            None => {
                pools.note_unsupported("list_pool_element:empty");
                Vec::new()
            }
        },
        PoolElement::Unsupported { kind } => {
            pools.note_unsupported(kind);
            Vec::new()
        }
    }
}

// ---------------------------------------------------------------------------
// Tests — all expectations hand-derived from the CFR oracle semantics.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::jigsaw::{
        util_shuffle, JointType, PoolElement, Projection, Rotation, ROTATION_VALUES, TemplateData, TemplateJigsaw,
    };
    use crate::jrandom::LegacyRandomSource;
    use std::collections::HashMap;

    // -- fixtures -----------------------------------------------------------

    fn jig(pos: (i32, i32, i32), front: Dir, name: &str, pool: &str, target: &str, joint: JointType) -> TemplateJigsaw {
        TemplateJigsaw {
            pos,
            front,
            top: Dir::Up,
            name: name.to_string(),
            pool: pool.to_string(),
            target: target.to_string(),
            placement_priority: 0,
            selection_priority: 0,
            joint,
        }
    }

    fn tpl(size: (i32, i32, i32), jigsaws: Vec<TemplateJigsaw>) -> TemplateData {
        TemplateData { size, jigsaws }
    }

    fn single(location: &str) -> PoolElement {
        PoolElement::Single { location: location.to_string(), projection: Projection::Rigid }
    }

    fn empty_pool() -> ResolvedPool {
        ResolvedPool { fallback: "minecraft:empty".into(), templates: Vec::new(), has_aliases: false }
    }

    struct TestPools {
        pools: HashMap<String, ResolvedPool>,
        templates: HashMap<String, TemplateData>,
        notes: RefCell<Vec<String>>,
    }

    impl TestPools {
        fn new() -> Self {
            let mut pools = HashMap::new();
            pools.insert("minecraft:empty".to_string(), empty_pool());
            TestPools { pools, templates: HashMap::new(), notes: RefCell::new(Vec::new()) }
        }
        fn pool(mut self, key: &str, fallback: &str, templates: Vec<PoolElement>) -> Self {
            self.pools.insert(key.to_string(), ResolvedPool { fallback: fallback.to_string(), templates, has_aliases: false });
            self
        }
        fn tpl(mut self, key: &str, t: TemplateData) -> Self {
            self.templates.insert(key.to_string(), t);
            self
        }
    }

    impl PoolSource for TestPools {
        fn resolve(&self, key: &str) -> Option<ResolvedPool> {
            self.pools.get(key).cloned()
        }
        fn template_of(&mut self, element: &PoolElement) -> Option<TemplateData> {
            match element {
                PoolElement::Single { location, .. } => self.templates.get(location).cloned(),
                _ => None,
            }
        }
        fn max_size(&mut self, key: &str) -> i32 {
            let Some(p) = self.pools.get(key) else { return 0 };
            let mut max = 0;
            for t in &p.templates {
                if matches!(t, PoolElement::Empty) {
                    continue;
                }
                let Some(td) = self.templates.get(match t {
                    PoolElement::Single { location, .. } => location.as_str(),
                    _ => continue,
                }) else { continue };
                max = max.max(td.bounding_box((0, 0, 0), Rotation::None).get_yspan());
            }
            max
        }
        fn note_missing_pool(&mut self, key: &str) {
            self.notes.borrow_mut().push(format!("pool:{key}"));
        }
        fn note_missing_template(&mut self, location: &str) {
            self.notes.borrow_mut().push(format!("tpl:{location}"));
        }
        fn note_unsupported(&mut self, kind: &str) {
            self.notes.borrow_mut().push(format!("unsup:{kind}"));
        }
    }

    struct ConstHeight(i32, RefCell<Vec<(i32, i32)>>);
    impl FirstFreeHeight for ConstHeight {
        fn first_free_height(&mut self, x: i32, z: i32) -> i32 {
            self.1.borrow_mut().push((x, z));
            self.0
        }
    }

    struct NoHeight;
    impl FirstFreeHeight for NoHeight {
        fn first_free_height(&mut self, _: i32, _: i32) -> i32 {
            panic!("heightmap sampled on the rigid-only path");
        }
    }

    fn slot_of(f: FreeShape) -> ShapeSlot {
        Rc::new(RefCell::new(Some(f)))
    }

    fn box3(min: (i32, i32, i32), max: (i32, i32, i32)) -> InclusiveBox {
        InclusiveBox { min_x: min.0, min_y: min.1, min_z: min.2, max_x: max.0, max_y: max.1, max_z: max.2 }
    }

    /// Seed scan: first seed whose first rotation shuffle puts `want` at
    /// index 0 (deterministic fixture instead of magic constants).
    fn seed_with_first_rotation(want: Rotation) -> i64 {
        for s in 0..10_000i64 {
            let mut r = LegacyRandomSource::new(s);
            if rotation_get_shuffled(&mut r)[0] == want {
                return s;
            }
        }
        unreachable!()
    }

    // T1: Shapes.create epsilon + AABB.of + deflate --------------------------
    #[test]
    fn dbox_epsilon_aabb_of_deflate() {
        let b = box3((0, 64, 0), (7, 67, 7));
        let d = DBox::from_inclusive_box(&b);
        assert_eq!(d, DBox { x0: 0.0, y0: 64.0, z0: 0.0, x1: 8.0, y1: 68.0, z1: 8.0 });
        assert_eq!(d.deflate(0.25), DBox { x0: 0.25, y0: 64.25, z0: 0.25, x1: 7.75, y1: 67.75, z1: 7.75 });
        // 1-wide int box => 0.5 span after deflate: still positive
        assert!(d.deflate(0.25).spans_positive());
        // degenerate: zero-thickness box => Shapes.create EMPTY
        let thin = DBox { x0: 5.0, y0: 5.0, z0: 5.0, x1: 5.0, y1: 6.0, z1: 6.0 };
        assert!(!thin.spans_positive());
        assert!(FreeShape::from_dbox(thin).is_empty());
        // touching A=[0,1] vs B=[1,2]: B is not covered by A (half-open cells)
        let a = FreeShape::from_dbox(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 1.0, y1: 1.0, z1: 1.0 });
        assert!(a.only_second_nonempty(DBox { x0: 1.0, y0: 0.0, z0: 0.0, x1: 2.0, y1: 1.0, z1: 1.0 }));
    }

    // T2: subtract_one — the 6-slab split and edge cases ---------------------
    #[test]
    fn subtract_one_split_and_edges() {
        let r = DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 10.0, y1: 10.0, z1: 10.0 };
        let c = DBox { x0: 2.0, y0: 3.0, z0: 4.0, x1: 4.0, y1: 5.0, z1: 6.0 };
        let got = subtract_one(r, c);
        let want = vec![
            DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 2.0, y1: 10.0, z1: 10.0 },   // x-left
            DBox { x0: 4.0, y0: 0.0, z0: 0.0, x1: 10.0, y1: 10.0, z1: 10.0 },  // x-right
            DBox { x0: 2.0, y0: 0.0, z0: 0.0, x1: 4.0, y1: 3.0, z1: 10.0 },    // y-low
            DBox { x0: 2.0, y0: 5.0, z0: 0.0, x1: 4.0, y1: 10.0, z1: 10.0 },   // y-high
            DBox { x0: 2.0, y0: 3.0, z0: 0.0, x1: 4.0, y1: 5.0, z1: 4.0 },     // z-low
            DBox { x0: 2.0, y0: 3.0, z0: 6.0, x1: 4.0, y1: 5.0, z1: 10.0 },    // z-high
        ];
        assert_eq!(got, want);
        // touching: C.x0 == R.x1 removes nothing
        let t = subtract_one(r, DBox { x0: 10.0, y0: 0.0, z0: 0.0, x1: 12.0, y1: 5.0, z1: 5.0 });
        assert_eq!(t, vec![r]);
        // covering: C contains R => empty
        let cov = subtract_one(r, DBox { x0: -1.0, y0: -1.0, z0: -1.0, x1: 11.0, y1: 11.0, z1: 11.0 });
        assert!(cov.is_empty());
        // identical => empty
        assert!(subtract_one(r, r).is_empty());
        // disjoint => unchanged
        let dis = subtract_one(r, DBox { x0: 20.0, y0: 20.0, z0: 20.0, x1: 21.0, y1: 21.0, z1: 21.0 });
        assert_eq!(dis, vec![r]);
    }

    // T3: only_second_nonempty — the collision predicate ---------------------
    #[test]
    fn only_second_nonempty_cases() {
        let a = FreeShape::from_dbox(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 10.0, y1: 10.0, z1: 10.0 });
        // fully inside => no collision
        assert!(!a.only_second_nonempty(DBox { x0: 2.0, y0: 2.0, z0: 2.0, x1: 8.0, y1: 8.0, z1: 8.0 }));
        // exactly equal => no collision (B \ A empty)
        assert!(!a.only_second_nonempty(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 10.0, y1: 10.0, z1: 10.0 }));
        // sticking out 0.25 => collision
        assert!(a.only_second_nonempty(DBox { x0: 9.0, y0: 2.0, z0: 2.0, x1: 10.25, y1: 8.0, z1: 8.0 }));
        // degenerate B => EMPTY shape => never collides
        assert!(!a.only_second_nonempty(DBox { x0: 1.0, y0: 1.0, z0: 1.0, x1: 1.0, y1: 9.0, z1: 9.0 }));
        // empty A (fully consumed) => B \ A = B => collision
        let e = FreeShape::default();
        assert!(e.only_second_nonempty(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 1.0, y1: 1.0, z1: 1.0 }));
        // L-shaped A: two boxes; B in the notch => covered
        let l = FreeShape(vec![
            DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 10.0, y1: 10.0, z1: 4.0 },
            DBox { x0: 0.0, y0: 0.0, z0: 4.0, x1: 4.0, y1: 10.0, z1: 10.0 },
        ]);
        assert!(!l.only_second_nonempty(DBox { x0: 1.0, y0: 1.0, z0: 5.0, x1: 3.0, y1: 3.0, z1: 8.0 }));
        assert!(l.only_second_nonempty(DBox { x0: 6.0, y0: 1.0, z0: 5.0, x1: 8.0, y1: 3.0, z1: 8.0 }));
    }

    // T4: FreeShape accumulation (joinUnoptimized ONLY_FIRST chain) ----------
    #[test]
    fn free_shape_subtract_accumulates() {
        let mut f = FreeShape::from_dbox(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 8.0, y1: 8.0, z1: 8.0 });
        f.subtract(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 8.0, y1: 3.0, z1: 8.0 });
        assert_eq!(f.0, vec![DBox { x0: 0.0, y0: 3.0, z0: 0.0, x1: 8.0, y1: 8.0, z1: 8.0 }]);
        f.subtract(DBox { x0: 0.0, y0: 3.0, z0: 0.0, x1: 4.0, y1: 8.0, z1: 8.0 });
        assert_eq!(f.0, vec![DBox { x0: 4.0, y0: 3.0, z0: 0.0, x1: 8.0, y1: 8.0, z1: 8.0 }]);
        // degenerate subtract = no-op (EMPTY second operand)
        f.subtract(DBox { x0: 1.0, y0: 1.0, z0: 1.0, x1: 1.0, y1: 2.0, z1: 2.0 });
        assert_eq!(f.0.len(), 1);
        // subtract everything => empty => future placements collide
        f.subtract(DBox { x0: 4.0, y0: 3.0, z0: 0.0, x1: 8.0, y1: 8.0, z1: 8.0 });
        assert!(f.is_empty());
        assert!(f.only_second_nonempty(DBox { x0: 0.0, y0: 0.0, z0: 0.0, x1: 1.0, y1: 1.0, z1: 1.0 }));
    }

    // T5: SequencedPriorityIterator — exact decompiled semantics --------------
    #[test]
    fn sequenced_priority_iterator_semantics() {
        // FIFO within a priority, then switch to the next-highest non-empty
        let mut q: SequencedPriorityIterator<&str> = SequencedPriorityIterator::new();
        q.add("a", 1);
        q.add("b", 0);
        q.add("c", 1);
        assert_eq!(q.next(), Some("a"));
        assert_eq!(q.next(), Some("c")); // FIFO within priority 1
        assert_eq!(q.next(), Some("b")); // switch to priority 0
        assert_eq!(q.next(), None);

        // higher priority added mid-iteration jumps the queue (>= re-point)
        let mut q: SequencedPriorityIterator<i32> = SequencedPriorityIterator::new();
        q.add(1, 1);
        q.add(2, 2); // re-point to 2
        q.add(3, 2); // fast path: priority == highest => append to SAME queue
        q.add(0, 0);
        assert_eq!(q.next(), Some(2));
        assert_eq!(q.next(), Some(3));
        assert_eq!(q.next(), Some(1)); // drained 2 -> max non-empty = 1
        assert_eq!(q.next(), Some(0));
        assert_eq!(q.next(), None);

        // add AFTER the top queue drained: re-point happens (prio >= MIN)
        let mut q: SequencedPriorityIterator<i32> = SequencedPriorityIterator::new();
        q.add(1, 1);
        assert_eq!(q.next(), Some(1));
        assert_eq!(q.next(), None);
        q.add(2, 0); // 0 >= MIN => becomes highest
        assert_eq!(q.next(), Some(2));
        q.add(3, -5); // any int >= MIN => picked up
        assert_eq!(q.next(), Some(3));
        assert_eq!(q.next(), None);

        // lower-priority adds while a higher queue is non-empty must WAIT
        let mut q: SequencedPriorityIterator<i32> = SequencedPriorityIterator::new();
        q.add(1, 2);
        q.add(2, 0); // stored, must not jump
        q.add(3, 2); // fast path appends after 1
        assert_eq!(q.next(), Some(1));
        assert_eq!(q.next(), Some(3));
        assert_eq!(q.next(), Some(2));
        assert_eq!(q.next(), None);
    }

    // -- T6..T10 fixture: parent 8x4x8 @ (10,64,10) + child 3x3x3 ------------

    fn parent_piece() -> Piece {
        Piece {
            element: single("test:parent"),
            position: (10, 64, 10),
            ground_level_delta: 1,
            rotation: Rotation::None,
            bounding_box: box3((10, 64, 10), (17, 67, 17)),
            junctions: Vec::new(),
        }
    }

    /// free slot mirroring the REAL top-level shape: maxDistance AABB minus
    /// the (post-move) start-piece box. The parent-box hole is load-bearing:
    /// attachments landing inside the hole (over the start piece) collide.
    fn parent_free_slot() -> ShapeSlot {
        let aabb = DBox { x0: -70.0, y0: -16.0, z0: -70.0, x1: 91.0, y1: 145.0, z1: 91.0 };
        let parent = DBox { x0: 10.0, y0: 64.0, z0: 10.0, x1: 18.0, y1: 68.0, z1: 18.0 };
        slot_of(FreeShape::join_only_first(aabb, parent))
    }

    fn basic_pools() -> TestPools {
        TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:child")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]))
            .tpl("test:child", tpl((3, 3, 3), vec![jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable)]))
    }

    // T6: successful placement — geometry, junctions, RNG stream -------------
    #[test]
    fn try_placing_children_success_geometry_junctions_rng() {
        let seed = seed_with_first_rotation(Rotation::None);
        let mut pools = basic_pools();
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        // exactly one child placed, geometry per hand trace (rot NONE):
        assert_eq!(pieces.len(), 2);
        let child = &pieces[1];
        assert_eq!(child.position, (18, 64, 13));
        assert_eq!(child.bounding_box, box3((18, 64, 13), (20, 66, 13 + 2)));
        assert_eq!(child.rotation, Rotation::None);
        assert_eq!(child.ground_level_delta, 1); // rigid: parent gld - i3 = 1 - 0
        assert!(child.junctions.is_empty() == false);
        assert_eq!(child.junctions[0], Junction {
            source_x: 17,          // parent jigsaw abs pos x
            source_ground_y: 65,   // i7 - y + i6 = (64+1) - 1 + 1
            source_z: 13,
            delta_y: 0,            // -i3
            dest_projection: Projection::Rigid,
        });
        let pj = &pieces[0];
        assert_eq!(pj.junctions.len(), 1);
        assert_eq!(pj.junctions[0], Junction {
            source_x: 18,          // blockPos1 x
            source_ground_y: 65,   // i7 - i + gld = 65 - 1 + 1
            source_z: 13,
            delta_y: 0,
            dest_projection: Projection::Rigid, // child projection
        });
        // RNG stream: ONE rotation shuffle per candidate (the 4 rotations are
        // materialized by a single 3-nextInt shuffle). Parent: 1 candidate ->
        // success at rotation NONE -> candidate loop stops. Child attempt:
        // 1 candidate -> can_attach only succeeds at CW180, whose box lands
        // in the parent-box hole of free -> collision -> 4 rotations scanned.
        let mut r2 = LegacyRandomSource::new(seed);
        rotation_get_shuffled(&mut r2); // parent candidate
        rotation_get_shuffled(&mut r2); // child candidate (drain)
        assert_eq!(rng.next_long(), r2.next_long());
        assert_eq!(rng.next_int_bound(1000), r2.next_int_bound(1000));
    }

    // T7a: child attaching INSIDE the parent bbox must stay within it; one
    // poking out is rejected against the per-piece (parent bbox) shape.
    #[test]
    fn try_placing_children_inside_parent_poke_out_rejects() {
        let seed = seed_with_first_rotation(Rotation::None);
        // jigsaw (6,1,3) East -> blockPos1 (17,65,13) INSIDE (17 <= 17);
        // child 6x3x6 at blockPos3 (17,64,13) spans x 17..22 — pokes out of
        // the parent bbox (maxX 17) even after deflate -> rejected.
        let mut pools = TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:wide")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((6, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]))
            .tpl("test:wide", tpl((6, 3, 6), vec![jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        assert_eq!(pieces.len(), 1);
        assert!(pieces[0].junctions.is_empty());
        let mut r2 = LegacyRandomSource::new(seed);
        rotation_get_shuffled(&mut r2); // 1 candidate
        assert_eq!(rng.next_long(), r2.next_long());
    }

    // T7b: interior child accepted; a second interior child overlapping the
    // first is rejected via the per-piece shape (parent bbox MINUS child1).
    #[test]
    fn try_placing_children_interior_accumulates_per_piece_shape() {
        // seed pinning: jigsaw shuffle of 2 (1 nextInt), then two rotation
        // shuffles each starting at NONE (deterministic consumption).
        let mut seed = None;
        for s in 0..100_000i64 {
            let mut r = LegacyRandomSource::new(s);
            r.next_int_bound(2); // parent jigsaw list shuffle
            if rotation_get_shuffled(&mut r)[0] != Rotation::None {
                continue;
            }
            if rotation_get_shuffled(&mut r)[0] != Rotation::None {
                continue;
            }
            seed = Some(s);
            break;
        }
        let seed = seed.expect("pinned seed");
        let mut pools = TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:child")])
            .tpl("test:parent", tpl((8, 4, 8), vec![
                jig((3, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable),
                jig((3, 2, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable),
            ]))
            .tpl("test:child", tpl((3, 3, 3), vec![jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        // j1's child fits inside the parent bbox -> placed; j2's child
        // (14..16, 65..67, 13..15) overlaps child1 (64..66 y) -> the per-piece
        // shape has child1 subtracted -> collision -> rejected.
        assert_eq!(pieces.len(), 2);
        assert_eq!(pieces[1].position, (14, 64, 13));
        assert_eq!(pieces[1].bounding_box, box3((14, 64, 13), (16, 66, 15)));
        assert_eq!(pieces[1].junctions[0].source_ground_y, 65);
        assert_eq!(pieces[0].junctions.len(), 1);
        // RNG: jigsaw shuffle (1) + one rotation shuffle per parent jigsaw
        // (3+3) + the drain's child attempt (3; its CW180 box lands in the
        // parent hole of free -> rejected).
        let mut r2 = LegacyRandomSource::new(seed);
        r2.next_int_bound(2);
        rotation_get_shuffled(&mut r2);
        rotation_get_shuffled(&mut r2);
        rotation_get_shuffled(&mut r2);
        assert_eq!(rng.next_long(), r2.next_long());
    }

    // T8: depth == maxDepth -> fallback-only candidates; no queue on overflow
    #[test]
    fn depth_boundary_fallback_only_and_no_queue() {
        let seed = seed_with_first_rotation(Rotation::None);
        // maxDepth 0: pool templates SKIPPED (depth == maxDepth), fallback is
        // minecraft:empty (0 templates) -> no candidates -> no RNG at all.
        let mut pools = basic_pools();
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 0, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        assert_eq!(pieces.len(), 1);
        let mut r2 = LegacyRandomSource::new(seed);
        assert_eq!(rng.next_long(), r2.next_long()); // ZERO consumption

        // maxDepth 1: parent places child (queued: 1 > 1 false), the child's
        // own attempt runs at depth == maxDepth -> fallback-only -> stops.
        let mut pools = basic_pools();
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 1, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        assert_eq!(pieces.len(), 2);
        let mut r2 = LegacyRandomSource::new(seed);
        rotation_get_shuffled(&mut r2); // parent only
        assert_eq!(rng.next_long(), r2.next_long());
    }

    // T9: expansion hack — i2 grows bbox3 (encapsulate min_y + max(i2+1,span))
    #[test]
    fn expansion_hack_grows_child_box() {
        let seed = seed_with_first_rotation(Rotation::None);
        // child template: attaching jigsaw at (0,1,0) West (attaches at rot
        // NONE) + a non-attaching jigsaw at (1,1,1) North whose pool feeds
        // i2 (its pos+front is inside the rotated template bbox at rot NONE).
        let mut pools = TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:child")])
            .pool("test:windows", "test:wf", vec![single("test:window")])
            .pool("test:wf", "minecraft:empty", vec![single("test:wfb")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]))
            .tpl("test:child", tpl((3, 3, 3), vec![
                jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable),
                jig((1, 1, 1), Dir::North, "win", "test:windows", "", JointType::Rollable),
            ]))
            .tpl("test:window", tpl((1, 5, 1), vec![]))
            .tpl("test:wfb", tpl((1, 2, 1), vec![]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            // useExpansionHack = TRUE (village_plains.json)
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, true);
            placer.run(0, parent_free_slot());
        }
        // i2 = max(maxSize(test:windows)=5, maxSize(test:wf)=2) = 5 > 0 ->
        // max(6, yspan 3) = 6 -> encapsulate (minX, min_y+6, minZ) -> the
        // placed child box grows to maxY 70 (was 66).
        assert_eq!(pieces.len(), 2);
        assert_eq!(pieces[1].bounding_box.min_y, 64);
        assert_eq!(pieces[1].bounding_box.max_y, 70);

        // rejection variant: a big free room -> the GROWN box collides even
        // though the raw child box would fit.
        let mut pools = TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:child")])
            .pool("test:windows", "test:wf", vec![single("test:window")])
            .pool("test:wf", "minecraft:empty", vec![single("test:wfb")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]))
            .tpl("test:child", tpl((3, 3, 3), vec![
                jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable),
                jig((1, 1, 1), Dir::North, "win", "test:windows", "", JointType::Rollable),
            ]))
            .tpl("test:window", tpl((1, 5, 1), vec![]))
            .tpl("test:wfb", tpl((1, 2, 1), vec![]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        // room just tall enough for the RAW child (maxY 66.75 after deflate)
        // but NOT for the GROWN one (maxY 70.75): free = room minus parent.
        let room = DBox { x0: 10.0, y0: 64.0, z0: 10.0, x1: 21.0, y1: 69.0, z1: 24.0 };
        let parent = DBox { x0: 10.0, y0: 64.0, z0: 10.0, x1: 18.0, y1: 68.0, z1: 18.0 };
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, true);
            placer.run(0, slot_of(FreeShape::join_only_first(room, parent)));
        }
        // the grown box (y up to 70) pokes above the room (y 69) -> collision
        // -> rejected on every attaching path.
        assert_eq!(pieces.len(), 1);
        // RNG: 1 candidate = rotation shuffle (3) + 4 jigsaw-list shuffles of
        // 2 (1 nextInt each, per rotation).
        let mut r2 = LegacyRandomSource::new(seed);
        rotation_get_shuffled(&mut r2);
        for _ in 0..4 {
            let mut two = [0u8, 1u8];
            util_shuffle(&mut two, &mut r2);
        }
        assert_eq!(rng.next_long(), r2.next_long());
    }

    // T10: add_pieces entry — start piece, heightmap, top-level free shape --
    #[test]
    fn add_pieces_entry_full_flow() {
        // seed pinning: rotation pick = NONE, template pick = parent (index
        // 0), parent-attempt shuffle[0] = NONE, child-attempt shuffle[0] =
        // CW180 (the child->grandchild attach rotation).
        let mut seed = None;
        for s in 0..100_000i64 {
            let mut r = LegacyRandomSource::new(s);
            if ROTATION_VALUES[r.next_int_bound(4) as usize] != Rotation::None {
                continue;
            }
            if r.next_int_bound(2) != 0 {
                continue;
            }
            if rotation_get_shuffled(&mut r)[0] != Rotation::None {
                continue;
            }
            if rotation_get_shuffled(&mut r)[0] != Rotation::Clockwise180 {
                continue;
            }
            seed = Some(s);
            break;
        }
        let seed = seed.expect("pinned seed");
        let mut pools = TestPools::new()
            .pool("test:start", "minecraft:empty", vec![single("test:parent"), single("test:child")])
            .pool("test:houses", "minecraft:empty", vec![single("test:child")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]))
            .tpl("test:child", tpl((3, 3, 3), vec![jig((0, 1, 0), Dir::West, "child", "test:houses", "", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut sampler = ConstHeight(70, RefCell::new(Vec::new()));
        let params = AssemblyParams {
            start_pool: "test:start".into(),
            start_jigsaw_name: None,
            max_depth: 2,
            pos: (10, 0, 10),
            use_expansion_hack: false,
            project_start_to_heightmap: true, // WORLD_SURFACE_WG
            max_distance: (80, 80),
            dimension_padding: (0, 0),
            level_min_y: -64,
            level_max_y: 319,
        };
        let res = add_pieces(&params, &mut pools, &mut rng, &mut sampler).expect("assembly");
        // start height: sampler at the bbox center (13, 13) -> i2 = 0 + 70
        assert_eq!(sampler.1.borrow().as_slice(), &[(13, 13)]);
        assert_eq!(res.stub_position, (13, 70, 13));
        assert_eq!(pieces0(&res).position, (10, 69, 10)); // move(0, 70 - (0+1))
        assert_eq!(pieces0(&res).bounding_box, box3((10, 69, 10), (17, 72, 17)));

        assert_eq!(res.pieces.len(), 2, "parent + child; grandchild rejected");
        let child = &res.pieces[1];
        assert_eq!(child.position, (18, 69, 13));
        assert_eq!(child.bounding_box, box3((18, 69, 13), (20, 71, 15)));
        assert_eq!(child.ground_level_delta, 1);
        // junction chain: parent<-child
        assert_eq!(res.pieces[0].junctions[0].source_ground_y, 70);
        assert_eq!(res.pieces[1].junctions.len(), 1);
        assert_eq!(res.pieces[1].junctions[0].source_ground_y, 70);
        // the child's own attempt attaches only at CW180 whose box
        // (15..17, 69..71, 11..13) lands INSIDE the start-piece hole of the
        // top-level free shape -> collision -> grandchild correctly rejected
        // (it would overlap the start piece).

        // RNG stream: rot pick (1) + template pick (1) + parent rotation
        // shuffle (3) + child-attempt rotation shuffle (3) = 8 nextInt; the
        // grandchild attempt at depth == maxDepth consumes nothing.
        let mut r2 = LegacyRandomSource::new(seed);
        r2.next_int_bound(4);
        r2.next_int_bound(2);
        rotation_get_shuffled(&mut r2);
        rotation_get_shuffled(&mut r2);
        assert_eq!(rng.next_long(), r2.next_long());
        assert_eq!(rng.next_int_bound(997), r2.next_int_bound(997));
    }

    fn pieces0(res: &AssemblyResult) -> &Piece {
        &res.pieces[0]
    }

    /// The top-level free shape for the T10 numbers: AABB (clamped y
    /// [-10, 151]) minus the POST-MOVE start box -> exactly 6 slabs.
    #[test]
    fn top_level_free_shape_is_aabb_minus_moved_start() {
        let aabb = DBox { x0: -67.0, y0: -10.0, z0: -67.0, x1: 94.0, y1: 151.0, z1: 94.0 };
        let start = DBox::from_inclusive_box(&box3((10, 69, 10), (17, 72, 17)));
        let f = FreeShape::join_only_first(aabb, start);
        assert_eq!(f.0, vec![
            DBox { x0: -67.0, y0: -10.0, z0: -67.0, x1: 10.0, y1: 151.0, z1: 94.0 },
            DBox { x0: 18.0, y0: -10.0, z0: -67.0, x1: 94.0, y1: 151.0, z1: 94.0 },
            DBox { x0: 10.0, y0: -10.0, z0: -67.0, x1: 18.0, y1: 69.0, z1: 94.0 },
            DBox { x0: 10.0, y0: 73.0, z0: -67.0, x1: 18.0, y1: 151.0, z1: 94.0 },
            DBox { x0: 10.0, y0: 69.0, z0: -67.0, x1: 18.0, y1: 73.0, z1: 10.0 },
            DBox { x0: 10.0, y0: 69.0, z0: 18.0, x1: 18.0, y1: 73.0, z1: 94.0 },
        ]);
    }

    // T11: start_jigsaw_name — mirror position + scan RNG --------------------
    #[test]
    fn add_pieces_start_jigsaw_name_mirrors() {
        let mut seed = None;
        for s in 0..100_000i64 {
            let mut r = LegacyRandomSource::new(s);
            if ROTATION_VALUES[r.next_int_bound(4) as usize] != Rotation::None {
                continue;
            }
            if r.next_int_bound(2) != 0 {
                continue;
            }
            seed = Some(s);
            break;
        }
        let seed = seed.expect("pinned seed");
        // start template 8x4x8 with TWO jigsaws named a/b; shuffle of 2
        // consumes 1 nextInt regardless of order; "b" at (2,0,3) is found.
        let mut pools = TestPools::new()
            .pool("test:start", "minecraft:empty", vec![single("test:two"), single("test:child")])
            .tpl("test:two", tpl((8, 4, 8), vec![
                jig((0, 1, 0), Dir::East, "a", "minecraft:empty", "", JointType::Rollable),
                jig((2, 0, 3), Dir::East, "b", "minecraft:empty", "", JointType::Rollable),
            ]))
            .tpl("test:child", tpl((3, 3, 3), vec![]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut sampler = NoHeight; // heightmap NOT used: start_jigsaw_name
        // matched a jigsaw whose Y feeds blockPos, but projectStartToHeightmap
        // is absent here -> i2 = blockPos1.y directly.
        let params = AssemblyParams {
            start_pool: "test:start".into(),
            start_jigsaw_name: Some("b".into()),
            max_depth: 0,
            pos: (10, 0, 10),
            use_expansion_hack: false,
            project_start_to_heightmap: false,
            max_distance: (80, 80),
            dimension_padding: (0, 0),
            level_min_y: -64,
            level_max_y: 319,
        };
        let res = add_pieces(&params, &mut pools, &mut rng, &mut sampler).expect("assembly");
        // blockPos = (12, 0, 13); vec3i = (2, 0, 3); blockPos1 = (8, 0, 7);
        // i2 = blockPos1.y = 0 (no heightmap); i3 = min_y + gld = 0 + 1 = 1;
        // move(0, -1, 0) -> position (8,-1,7), bbox (8,-1,7, 15,2,14)
        assert_eq!(pieces0(&res).position, (8, -1, 7));
        assert_eq!(pieces0(&res).bounding_box, box3((8, -1, 7), (15, 2, 14)));
        // RNG: rot(1) + template(1) + named-jigsaw scan shuffle of 2 (1) = 3
        let mut r2 = LegacyRandomSource::new(seed);
        r2.next_int_bound(4);
        r2.next_int_bound(2);
        let mut two = [0u8, 1u8];
        util_shuffle(&mut two, &mut r2);
        assert_eq!(rng.next_long(), r2.next_long());
    }

    // T12: isStartTooCloseToWorldHeightLimits --------------------------------
    #[test]
    fn add_pieces_dimension_padding_rejects() {
        let seed = seed_with_first_rotation(Rotation::None);
        let base = |padding: (i32, i32)| AssemblyParams {
            start_pool: "test:single".into(),
            start_jigsaw_name: None,
            max_depth: 0,
            pos: (10, 0, 10),
            use_expansion_hack: false,
            project_start_to_heightmap: true,
            max_distance: (80, 80),
            dimension_padding: padding,
            level_min_y: -64,
            level_max_y: 319,
        };
        let mut pools = TestPools::new()
            .pool("test:single", "minecraft:empty", vec![single("test:parent")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "minecraft:empty", "child", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        // sampler 10 -> start moved to y 9..12; padding (100, 0) -> lower
        // bound -64 + 100 = 36 > 9 -> rejected.
        let mut sampler = ConstHeight(10, RefCell::new(Vec::new()));
        let params = base((100, 0));
        assert!(add_pieces(&params, &mut pools, &mut rng, &mut sampler).is_none());
        // ZERO padding -> the check is skipped entirely -> accepted.
        let mut pools = TestPools::new()
            .pool("test:single", "minecraft:empty", vec![single("test:parent")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "minecraft:empty", "child", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut sampler = ConstHeight(10, RefCell::new(Vec::new()));
        let params = base((0, 0));
        assert!(add_pieces(&params, &mut pools, &mut rng, &mut sampler).is_some());
    }

    // T13: loud notes — missing pool / missing template -----------------------
    #[test]
    fn missing_pool_and_template_are_loud() {
        let seed = seed_with_first_rotation(Rotation::None);
        // parent jigsaw points at a pool that does not resolve -> skipped
        let mut pools = TestPools::new()
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:missing", "child", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        assert_eq!(pieces.len(), 1);
        assert!(pools.notes.borrow().iter().any(|n| n == "pool:test:missing"));

        // pool resolves but its template file is missing -> loud note, skip
        let mut pools = TestPools::new()
            .pool("test:houses", "minecraft:empty", vec![single("test:gone")])
            .tpl("test:parent", tpl((8, 4, 8), vec![jig((7, 1, 3), Dir::East, "attach", "test:houses", "child", JointType::Rollable)]));
        let mut rng = LegacyRandomSource::new(seed);
        let mut pieces = vec![parent_piece()];
        {
            let mut sampler = NoHeight;
            let mut placer = Placer::new(&mut pools, 6, &mut pieces, &mut rng, &mut sampler, false);
            placer.run(0, parent_free_slot());
        }
        assert_eq!(pieces.len(), 1);
        assert!(pools.notes.borrow().iter().any(|n| n == "tpl:test:gone"));
    }
}
