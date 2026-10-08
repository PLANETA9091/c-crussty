//! Climate parameter space + RTree biome search (NCF P2.4).
//!
//! Bit-exact port of net.minecraft.world.level.biome.Climate (CFR decompile
//! of the mojang-mapped Purpur 1.21.10 jar, cfr-out 2026-10-05):
//!
//! * `quantize_coord`: `(long)(f * 10000.0f)` — the multiply is F32 and the
//!   cast truncates toward zero (Java (long) float == Rust `as i64`). This is
//!   observable on negative half-way values and on f32 rounding of the
//!   product; the unit tests pin both.
//! * `Parameter.distance(long)`: `l = v - max; l1 = min - v; l > 0 ? l :
//!   max(l1, 0)` — i64 arithmetic (no overflow risk in practice, but the
//!   fitness SUM is accumulated in wrapping i64 exactly like Java long math).
//! * `RTree`: build (stable sort with the 7-key comparator chain starting at
//!   rotating index, bucketize via `pow(6, floor(log6(n - 0.01)))`, cost =
//!   sum |max-min|, span-merged node parameter spaces) and search (initial
//!   best leaf PERSISTS across searches — Java keeps it in a ThreadLocal,
//!   which we replicate as state on the RTree; strict `<` comparisons skip
//!   equal-distance children, so traversal order + the persistent leaf
//!   affect which of several equally-fit leaves is returned — replicated
//!   exactly, including the `node == leaf2` identity shortcut that reuses
//!   the node bound when the subtree search returned no better leaf).
//!
//! No approximations: every comparison is `<=`/`<` exactly as decompiled.
//!
//! clippy::needless_range_loop + type_complexity allowed: the RTree build
//! mirrors Java's indexed loops and HashMap-of-comparators shape verbatim.
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

/// Climate.quantizeCoord — `(long)(coord * 10000.0f)`.
#[inline]
pub fn quantize_coord(coord: f32) -> i64 {
    (coord * 10000.0f32) as i64
}

/// Climate.unquantizeCoord.
#[inline]
pub fn unquantize_coord(coord: i64) -> f32 {
    coord as f32 / 10000.0f32
}

/// Mth.square(long) — Java long math, wraps silently.
#[inline]
fn square_long(v: i64) -> i64 {
    v.wrapping_mul(v)
}

/// Climate.Parameter (quantized i64 interval).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameter {
    pub min: i64,
    pub max: i64,
}

impl Parameter {
    pub fn point(value: f32) -> Self {
        Parameter::span(value, value)
    }

    pub fn span(min: f32, max: f32) -> Self {
        // Java throws on min > max; keep the check for port fidelity.
        assert!(min <= max, "min > max: {min} {max}");
        Parameter { min: quantize_coord(min), max: quantize_coord(max) }
    }

    /// Climate.Parameter#distance(long pointValue).
    #[inline]
    pub fn distance_to_value(&self, point_value: i64) -> i64 {
        let l = point_value.wrapping_sub(self.max);
        let l1 = self.min.wrapping_sub(point_value);
        if l > 0 {
            l
        } else {
            l1.max(0)
        }
    }

    /// Climate.Parameter#distance(Parameter).
    #[inline]
    pub fn distance_to_param(&self, other: &Parameter) -> i64 {
        let l = other.min.wrapping_sub(self.max);
        let l1 = self.min.wrapping_sub(other.max);
        if l > 0 {
            l
        } else {
            l1.max(0)
        }
    }

    /// Climate.Parameter#span(Parameter) — bounding interval.
    pub fn span_param(&self, other: &Parameter) -> Parameter {
        Parameter { min: self.min.min(other.min), max: self.max.max(other.max) }
    }
}

/// Climate.TargetPoint — 6 quantized coords; the implicit 7th (offset) is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetPoint {
    pub temperature: i64,
    pub humidity: i64,
    pub continentalness: i64,
    pub erosion: i64,
    pub depth: i64,
    pub weirdness: i64,
}

impl TargetPoint {
    /// TargetPoint.toParameterArray — 7 values with offset 0.
    pub fn to_parameter_array(&self) -> [i64; 7] {
        [
            self.temperature,
            self.humidity,
            self.continentalness,
            self.erosion,
            self.depth,
            self.weirdness,
            0,
        ]
    }
}

/// Climate.ParameterPoint.
#[derive(Debug, Clone, Copy)]
pub struct ParameterPoint {
    pub temperature: Parameter,
    pub humidity: Parameter,
    pub continentalness: Parameter,
    pub erosion: Parameter,
    pub depth: Parameter,
    pub weirdness: Parameter,
    pub offset: i64,
}

impl ParameterPoint {
    /// parameterSpace(): the 7 spans (offset becomes [offset, offset]).
    pub fn parameter_space(&self) -> [Parameter; 7] {
        [
            self.temperature,
            self.humidity,
            self.continentalness,
            self.erosion,
            self.depth,
            self.weirdness,
            Parameter { min: self.offset, max: self.offset },
        ]
    }

    /// ParameterPoint#fitness(TargetPoint) — wrapping i64 accumulation.
    pub fn fitness(&self, point: &TargetPoint) -> i64 {
        let vals = point.to_parameter_array();
        let space = self.parameter_space();
        let mut acc: i64 = 0;
        for i in 0..7 {
            acc = acc.wrapping_add(square_long(space[i].distance_to_value(vals[i])));
        }
        acc
    }
}

// --------------------------------------------------------------------------
// RTree
// --------------------------------------------------------------------------

const CHILDREN_PER_NODE: usize = 6;
const PARAM_SPACE_SIZE: usize = 7;

/// Node ids: leafs are the input points (index into the point list),
/// subtrees get ids starting at leaf_count. Identity (==) comparisons in
/// SubTree.search use these ids (Java compares object identity).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeId {
    Leaf(usize),
    SubTree(usize),
}

struct SubTreeNode {
    children: Vec<NodeId>,
    parameter_space: [Parameter; 7],
}

/// The built search tree. IMMUTABLE after create: the persistent best-so-far
/// hint (Java `ThreadLocal<Leaf> lastResult`) is owned by the CALLER and
/// passed into search() as `&mut Option<usize>` — this keeps the tree
/// shareable (Arc) across chunk/phase owners while each owner keeps exactly
/// the memo-chain semantics its gates were validated with (fresh per chunk,
/// chained within the chunk).
pub struct RTree {
    leaves: Vec<ParameterPoint>,
    subtrees: Vec<SubTreeNode>,
    root: NodeId,
}

impl RTree {
    /// RTree.create — leaves in INPUT ORDER (order matters: stable sorts and
    /// child iteration depend on it).
    pub fn create(points: Vec<ParameterPoint>) -> Self {
        assert!(!points.is_empty(), "Need at least one value to build the search tree.");
        let leaf_count = points.len();
        let mut tree = RTree { leaves: points, subtrees: Vec::new(), root: NodeId::Leaf(0) };
        let children: Vec<NodeId> = (0..leaf_count).map(NodeId::Leaf).collect();
        tree.root = tree.build(&children);
        tree
    }

    fn parameter_space_of(&self, id: NodeId) -> [Parameter; 7] {
        match id {
            NodeId::Leaf(i) => self.leaves[i].parameter_space(),
            NodeId::SubTree(i) => self.subtrees[i].parameter_space,
        }
    }

    /// RTree.build(paramSpaceSize, children).
    ///
    /// Faithful port, including the in-place mutation of `children` across
    /// axis iterations (stable sorts cascade — a later iteration sorts the
    /// list order the previous iteration left behind) and the FINAL
    /// absolute-variant sort applied to the BUCKET LIST itself (buckets are
    /// compared as nodes by their span-merged parameter spaces), not to the
    /// children inside each bucket.
    fn build(&mut self, children: &[NodeId]) -> NodeId {
        assert!(!children.is_empty(), "Need at least one child to build a node");
        if children.len() == 1 {
            return children[0];
        }
        if children.len() <= CHILDREN_PER_NODE {
            // sort by sum of |(min+max)/2| over the parameter space (STABLE),
            // then a plain SubTree
            let mut sorted: Vec<NodeId> = children.to_vec();
            let keys: Vec<i64> = sorted
                .iter()
                .map(|&c| {
                    let space = self.parameter_space_of(c);
                    let mut l: i64 = 0;
                    for p in &space {
                        l = l.wrapping_add(((p.min.wrapping_add(p.max)) / 2).abs());
                    }
                    l
                })
                .collect();
            let mut order: Vec<usize> = (0..sorted.len()).collect();
            order.sort_by_key(|&i| keys[i]);
            sorted = order.into_iter().map(|i| sorted[i]).collect();
            return self.push_subtree(&sorted);
        }
        // axis selection: `children` is re-sorted IN PLACE per axis (Java
        // List.sort mutates), each iteration starting from the previous
        // iteration's order
        let mut working: Vec<NodeId> = children.to_vec();
        let mut best_cost = i64::MAX;
        let mut best_axis = 0usize;
        let mut best_buckets: Vec<Vec<NodeId>> = Vec::new();
        for axis in 0..PARAM_SPACE_SIZE {
            sort_rotating_in_place(&mut working, axis, false, &self.leaves, &self.subtrees);
            let buckets = bucketize(&working);
            let mut cost: i64 = 0;
            for b in &buckets {
                cost = cost.wrapping_add(subtree_cost(b, &self.leaves, &self.subtrees));
            }
            if best_cost > cost {
                best_cost = cost;
                best_axis = axis;
                best_buckets = buckets;
            }
        }
        // final sort: the BUCKET LIST as nodes (bucketize wraps each bucket in
        // a SubTree, whose space is the span-merge of its children), absolute
        // variant, axis best_axis
        sort_bucket_list(&mut best_buckets, best_axis, &self.leaves, &self.subtrees);
        let child_ids: Vec<NodeId> = best_buckets.iter().map(|b| self.build(b)).collect();
        self.push_subtree(&child_ids)
    }

    fn push_subtree(&mut self, children: &[NodeId]) -> NodeId {
        // buildParameterSpace: span-merge children spaces
        let mut space: [Option<Parameter>; 7] = [None; 7];
        for &c in children {
            let ps = self.parameter_space_of(c);
            for i in 0..7 {
                space[i] = Some(match space[i] {
                    None => ps[i],
                    Some(cur) => cur.span_param(&ps[i]),
                });
            }
        }
        let parameter_space: [Parameter; 7] =
            std::array::from_fn(|i| space[i].expect("7-dim parameter space"));
        let id = NodeId::SubTree(self.subtrees.len());
        self.subtrees.push(SubTreeNode { children: children.to_vec(), parameter_space });
        id
    }

    /// RTree.Node#distance — sum of squared parameter distances (wrapping).
    fn node_distance(&self, id: NodeId, values: &[i64; 7]) -> i64 {
        let space = self.parameter_space_of(id);
        let mut l: i64 = 0;
        for i in 0..7 {
            l = l.wrapping_add(square_long(space[i].distance_to_value(values[i])));
        }
        l
    }

    /// SubTree.search — exact port (see module docs). `searched` leaf is the
    /// persistent best-so-far (Java ThreadLocal), `None` = Long.MAX_VALUE.
    fn search_node(&self, id: NodeId, searched: Option<usize>, values: &[i64; 7]) -> Option<usize> {
        match id {
            NodeId::Leaf(i) => Some(i), // Leaf.search returns `this`
            NodeId::SubTree(si) => {
                let node = &self.subtrees[si];
                let mut best_dist: i64 = match searched {
                    None => i64::MAX,
                    Some(li) => self.node_distance(NodeId::Leaf(li), values),
                };
                let mut best_leaf: Option<usize> = searched;
                for &child in &node.children {
                    let bound = self.node_distance(child, values);
                    if best_dist <= bound {
                        continue;
                    }
                    let got = self.search_node(child, best_leaf, values);
                    // identity shortcut: `node == leaf2 ? l1 : distance(leaf2)`.
                    // `node` is the CHILD here; the recursion returns the child
                    // itself only for Leaf children (Leaf.search returns
                    // `this`) — SubTree children always produce a LEAF id.
                    let l2 = if child == NodeId::Leaf(got.unwrap()) {
                        bound
                    } else {
                        self.node_distance(NodeId::Leaf(got.unwrap()), values)
                    };
                    if best_dist <= l2 {
                        continue;
                    }
                    best_dist = l2;
                    best_leaf = got;
                }
                best_leaf
            }
        }
    }

    /// ParameterList.findValueIndex — search with the CALLER-owned persistent
    /// best-so-far hint (`*memo` = last found leaf, None = cold).
    pub fn search(&self, target: &TargetPoint, memo: &mut Option<usize>) -> usize {
        let values = target.to_parameter_array();
        let got = self.search_node(self.root, *memo, &values);
        let leaf = got.expect("RTree search always returns a leaf");
        *memo = Some(leaf);
        leaf
    }

    pub fn leaf_point(&self, leaf: usize) -> &ParameterPoint {
        &self.leaves[leaf]
    }
}

/// RTree.sort — the rotating 7-key comparator chain: primary key at `axis`,
/// then (axis+k) % 7 for k = 1..6; `absolute` maps each key through |v|.
/// Java's List.sort is STABLE — Rust sort_by is stable too.
fn sort_rotating_in_place(
    children: &mut [NodeId],
    axis: usize,
    absolute: bool,
    leaves: &[ParameterPoint],
    subtrees: &[SubTreeNode],
) {
    let key_of = |c: &NodeId| -> [i64; 7] {
        let space = match c {
            NodeId::Leaf(i) => leaves[*i].parameter_space(),
            NodeId::SubTree(i) => subtrees[*i].parameter_space,
        };
        let mut keys = [0i64; 7];
        for k in 0..7 {
            let idx = (axis + k) % 7;
            let mid = (space[idx].min.wrapping_add(space[idx].max)) / 2;
            keys[k] = if absolute { mid.abs() } else { mid };
        }
        keys
    };
    let keys: Vec<[i64; 7]> = children.iter().map(key_of).collect();
    let mut order: Vec<usize> = (0..children.len()).collect();
    order.sort_by(|&a, &b| keys[a].cmp(&keys[b]));
    let sorted: Vec<NodeId> = order.iter().map(|&i| children[i]).collect();
    children.copy_from_slice(&sorted);
}

/// RTree.sort over a BUCKET list (each bucket compared by the span-merged
/// space its children produce — bucketize's `new SubTree(list1)` semantics).
fn sort_bucket_list(
    buckets: &mut [Vec<NodeId>],
    axis: usize,
    leaves: &[ParameterPoint],
    subtrees: &[SubTreeNode],
) {
    let space_of_bucket = |bucket: &Vec<NodeId>| -> [Parameter; 7] {
        let mut space: [Option<Parameter>; 7] = [None; 7];
        for &c in bucket {
            let ps = match c {
                NodeId::Leaf(i) => leaves[i].parameter_space(),
                NodeId::SubTree(i) => subtrees[i].parameter_space,
            };
            for i in 0..7 {
                space[i] = Some(match space[i] {
                    None => ps[i],
                    Some(cur) => cur.span_param(&ps[i]),
                });
            }
        }
        std::array::from_fn(|i| space[i].expect("7-dim parameter space"))
    };
    let keys: Vec<[i64; 7]> = buckets
        .iter()
        .map(|b| {
            let space = space_of_bucket(b);
            let mut keys = [0i64; 7];
            for k in 0..7 {
                let idx = (axis + k) % 7;
                let mid = (space[idx].min.wrapping_add(space[idx].max)) / 2;
                keys[k] = mid.abs();
            }
            keys
        })
        .collect();
    let mut order: Vec<usize> = (0..buckets.len()).collect();
    order.sort_by(|&a, &b| keys[a].cmp(&keys[b]));
    let sorted: Vec<Vec<NodeId>> = order.iter().map(|&i| buckets[i].clone()).collect();
    for (dst, src) in sorted.into_iter().enumerate() {
        buckets[dst] = src;
    }
}

/// RTree.bucketize — bucket size `pow(6, floor(log6(n - 0.01)))` (floor
/// applies to the LOG RATIO = the exponent, NOT to the power).
fn bucketize(nodes: &[NodeId]) -> Vec<Vec<NodeId>> {
    let n = nodes.len() as f64;
    let i = (6.0f64.powf(((n - 0.01).ln() / 6.0f64.ln()).floor())) as usize;
    let mut buckets: Vec<Vec<NodeId>> = Vec::new();
    let mut current: Vec<NodeId> = Vec::new();
    for &node in nodes {
        current.push(node);
        if current.len() >= i {
            buckets.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        buckets.push(current);
    }
    buckets
}

/// RTree.cost — sum of |max - min| over the span-merged space (wrapping).
fn subtree_cost(
    bucket: &[NodeId],
    leaves: &[ParameterPoint],
    subtrees: &[SubTreeNode],
) -> i64 {
    // span-merge
    let mut space: [Option<Parameter>; 7] = [None; 7];
    for &c in bucket {
        let ps = match c {
            NodeId::Leaf(i) => leaves[i].parameter_space(),
            NodeId::SubTree(i) => subtrees[i].parameter_space,
        };
        for i in 0..7 {
            space[i] = Some(match space[i] {
                None => ps[i],
                Some(cur) => cur.span_param(&ps[i]),
            });
        }
    }
    let mut l: i64 = 0;
    for p in space.into_iter().flatten() {
        l = l.wrapping_add(p.max.wrapping_sub(p.min).abs());
    }
    l
}

// --------------------------------------------------------------------------
// ParameterList
// --------------------------------------------------------------------------

/// Climate.ParameterList<T=biome name>: the built RTree plus the ordered
/// point/name table.
pub struct ParameterList {
    pub tree: RTree,
    pub names: Vec<String>,
}

impl ParameterList {
    pub fn new(points: Vec<(ParameterPoint, String)>) -> Self {
        let names = points.iter().map(|(_, n)| n.clone()).collect();
        let tree = RTree::create(points.into_iter().map(|(p, _)| p).collect());
        ParameterList { tree, names }
    }

    /// findValueIndex — biome name for the target (search hint owned by the
    /// caller: fresh per chunk, chained within the chunk — the exact memo
    /// semantics the bit-gates were validated with).
    pub fn find_value(&self, target: &TargetPoint, memo: &mut Option<usize>) -> &str {
        let leaf = self.tree.search(target, memo);
        &self.names[leaf]
    }

    /// findValueBruteForce — the reference scan (Java findValueBruteForce,
    /// used by tests to prove the tree agrees with the linear minimum).
    pub fn find_value_brute_force(&self, target: &TargetPoint) -> &str {
        let mut best = self.tree.leaf_point(0).fitness(target);
        let mut best_i = 0usize;
        for (i, p) in self.tree.leaves.iter().enumerate().skip(1) {
            let f = p.fitness(target);
            if f < best {
                best = f;
                best_i = i;
            }
        }
        &self.names[best_i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_truncates_toward_zero_in_f32() {
        // (long)(f * 10000.0f): the PRODUCT is f32 — pin the cases where the
        // f32 rounding matters.
        assert_eq!(quantize_coord(0.5f32), 5000);
        assert_eq!(quantize_coord(-0.5f32), -5000);
        assert_eq!(quantize_coord(0.0f32), 0);
        // -0.00004 * 10000 = -0.4 -> trunc toward zero = 0
        assert_eq!(quantize_coord(-0.00004f32), 0);
        assert_eq!(quantize_coord(0.00004f32), 0);
        // f32 rounding: 0.1f * 10000 = 1000.0000... in f32? 0.1f = 0.100000001490116...
        // product in f32 rounds to exactly 1000.0 -> 1000
        assert_eq!(quantize_coord(0.1f32), 1000);
        // Java: (long)(1.0f * 10000.0f) = 10000
        assert_eq!(quantize_coord(1.0f32), 10000);
        assert_eq!(quantize_coord(-1.2f32), -12000);
    }

    #[test]
    fn parameter_distance_semantics() {
        let p = Parameter { min: -100, max: 100 };
        assert_eq!(p.distance_to_value(0), 0);
        assert_eq!(p.distance_to_value(150), 50);
        assert_eq!(p.distance_to_value(-200), 100);
        let span = Parameter { min: -200, max: -100 };
        // intervals [-200,-100] and [-100,100] TOUCH -> distance 0 (Java
        // distance is the strict gap: l > 0 else max(l1, 0))
        assert_eq!(p.distance_to_param(&span), 0);
        assert_eq!(span.distance_to_param(&p), 0);
        let disjoint = Parameter { min: 200, max: 300 };
        assert_eq!(p.distance_to_param(&disjoint), 100);
        assert_eq!(disjoint.distance_to_param(&p), 100);
    }

    #[test]
    fn single_point_tree_agrees_with_brute_force() {
        // n=1 tree: build returns the leaf directly.
        let pt = ParameterPoint {
            temperature: Parameter::point(0.0),
            humidity: Parameter::point(0.0),
            continentalness: Parameter::point(0.0),
            erosion: Parameter::point(0.0),
            depth: Parameter::point(0.0),
            weirdness: Parameter::point(0.0),
            offset: 0,
        };
        let mut list = ParameterList::new(vec![(pt, "minecraft:test".into())]);
        let t = TargetPoint {
            temperature: 100,
            humidity: -100,
            continentalness: 0,
            erosion: 0,
            depth: 0,
            weirdness: 0,
        };
        let mut memo = None;
        assert_eq!(list.find_value(&t, &mut memo), "minecraft:test");
        assert_eq!(list.find_value_brute_force(&t), "minecraft:test");
    }

    #[test]
    fn tree_search_matches_brute_force_on_synthetic_points() {
        // 40 synthetic points spread over the 6-dim space; many queries; the
        // tree result must equal the brute-force minimum (no ties by
        // construction: offset column differs per point).
        let mut points = Vec::new();
        for i in 0..40i64 {
            let f = |k: i64| (i * 137 + k * 911) % 20001 - 10000;
            points.push((
                ParameterPoint {
                    temperature: Parameter { min: f(1), max: f(1) },
                    humidity: Parameter { min: f(2), max: f(2) },
                    continentalness: Parameter { min: f(3), max: f(3) },
                    erosion: Parameter { min: f(4), max: f(4) },
                    depth: Parameter { min: f(5), max: f(5) },
                    weirdness: Parameter { min: f(6), max: f(6) },
                    offset: i * 100,
                },
                format!("biome_{i}"),
            ));
        }
        let list = ParameterList::new(points);
        let mut memo = None;
        for q in 0..200i64 {
            let t = TargetPoint {
                temperature: (q * 73) % 20001 - 10000,
                humidity: (q * 151) % 20001 - 10000,
                continentalness: (q * 977) % 20001 - 10000,
                erosion: (q * 331) % 20001 - 10000,
                depth: (q * 419) % 20001 - 10000,
                weirdness: (q * 571) % 20001 - 10000,
            };
            let via_tree: String = list.find_value(&t, &mut memo).to_string();
            let via_bf: String = list.find_value_brute_force(&t).to_string();
            assert_eq!(via_tree, via_bf, "tree != brute force at query {q}");
        }
    }

    #[test]
    fn bucketize_size_matches_java_formula() {
        // Java: (int)Math.pow(6.0, Math.floor(Math.log(n - 0.01) / Math.log(6.0)))
        let java_bucket = |n: usize| -> usize {
            (6.0f64.powf(((n as f64 - 0.01).ln() / 6.0f64.ln()).floor())) as usize
        };
        assert_eq!(java_bucket(2), 1);  // log(1.99)/log(6) = 0.384 -> floor 0 -> 6^0 = 1
        assert_eq!(java_bucket(6), 1);  // log(5.99)/log(6) = 0.999 -> floor 0 -> 1
        assert_eq!(java_bucket(7), 6);  // log(6.99)/log(6) = 1.085 -> floor 1 -> 6
        assert_eq!(java_bucket(37), 36); // log(36.99)/log(6) = 2.015 -> floor 2 -> 36
        assert_eq!(java_bucket(179), 36); // log(178.99)/log(6) = 2.895 -> floor 2 -> 36
        assert_eq!(java_bucket(217), 216); // log(216.99)/log(6) = 3.0003 -> floor 3 -> 216
        // sanity for the crate path
        assert_eq!(bucketize(&[NodeId::Leaf(0); 7]).iter().map(|b| b.len()).sum::<usize>(), 7);
        assert_eq!(bucketize(&[NodeId::Leaf(0); 179]).iter().map(|b| b.len()).sum::<usize>(), 179);
    }
}
