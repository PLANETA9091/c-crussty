package net.minecraft.world.level.pathfinder;

import it.unimi.dsi.fastutil.ints.Int2ObjectMap;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Mob;
import java.util.concurrent.atomic.LongAdder;
import net.minecraft.world.level.PathNavigationRegion;
// NOTE: the vanilla region class lives in net.minecraft.world.level (javap
// NodeEvaluator.prepare), NOT in pathfinder — wrong-package import would
// bake a wrong descriptor into the bridge (resolution-closure guard).

/**
 * NAV-POOL (TASK-410-A k5, lever cmp405_navplane — STRICT eq, the SAME lever
 * as the navplane read plane: the pool is part of the nav/pathfinding vector).
 *
 * Vanilla per-search node machinery (javap, patched-kernel 1.21.10):
 *  - NodeEvaluator.nodes = Int2ObjectOpenHashMap; prepare() calls
 *    nodes.clear() on EVERY search (O(capacity) table nuke per recompute);
 *  - getNode(x,y,z) = nodes.computeIfAbsent(Node.createHash(x,y,z),
 *    (a,b,c) -> new Node(x,y,z)) — the invokedynamic lambda carries a
 *    3-int capture and is ALLOCATED ON EVERY getNode CALL (hit or miss);
 *  - every unique position visited allocates a fresh Node (~64B) that
 *    becomes garbage at the next prepare() clear;
 *  - map capacity re-grows (rehash) after every clear-then-refill.
 *
 * Pool protocol (Lithium ai.pathing precedent, adapted to final fields):
 *  - Node.x/y/z are FINAL, so objects can only be reused in-place for the
 *    SAME position. The fresh-shape IS the generation tag: prepare()
 *    launders every cached node back to the exact Node.<init> defaults
 *    (heapIdx=-1, closed=false, g=h=f=0, cameFrom=null, walkedDistance=0,
 *    costMalus=0, type=BLOCKED — javap-verbatim) instead of clearing the
 *    map. A node reused for the same position next search is bit-identical
 *    to a vanilla fresh node.
 *  - getNode(x,y,z) = map.get(hash); null OR position mismatch -> new
 *    Node + put (identical result semantics to vanilla computeIfAbsent on
 *    a fresh map; within one search a hash collision with a different
 *    position is impossible: x/z use 15 bits + sign, local searches span
 *    < 128 blocks, so positions inside one search hash distinctly).
 *    NO lambda is built on the hot path -> zero alloc on map hits.
 *  - Path (and its consumers) read only final x/y/z from stored nodes
 *    (javap -c Path: exactly three getfield Node.x/y/z), so laundering
 *    mutable fields of nodes still referenced by a live path is unobservable.
 *  - Memory bound: if the retained map exceeds MAP_CAP the evaluator falls
 *    back to the vanilla clear for that search (overflow counter) — bounded
 *    retention, parity unaffected (vanilla mode is trivially vanilla).
 *
 * Rust side (src/nav_pool.rs): the pool ENGINE (arena with vec reuse +
 * generation epochs instead of clears) plus reference-A*-vs-pooled-A*
 * cargo parity tests over seeded random worlds; ONE bulk JNI per search
 * (navPoolTick) feeds the arena epoch + churn telemetry and emits the
 * EFFECT marker (epoch/reuse line every 128 searches) — law 6: one JNI
 * per search, never per node; the node-traffic inner loop stays pure Java.
 *
 * Empty lever flag: this class is never defined and NodeEvaluator.prepare /
 * NodeEvaluator.getNode bodies are never retargeted (gate in
 * src/region_threads.rs composes the redirects only when
 * CRUSSTY_LEVER_FLAG eq cmp405_navplane) -> vanilla bit-in-byte by
 * construction. Different flag values are rejected too.
 */
public final class NavPoolOps {
    private NavPoolOps() {}

    /** Retention bound per evaluator; beyond it the search goes vanilla. */
    static final int MAP_CAP = 4096;

    /** Churn telemetry (advisory only; races blur stats, never parity). */
    static final LongAdder HITS = new LongAdder();
    static final LongAdder NEWS = new LongAdder();
    static final LongAdder OVERFLOWS = new LongAdder();

    /** ONE bulk-JNI per search: Rust arena epoch + churn stats + marker. */
    public static native void navPoolTick(int mapSize, long hits, long news,
                                          long overflows);

    /**
     * Receiver-prepended entry for the NodeEvaluator.prepare retarget.
     * Vanilla body (javap-verbatim NodeEvaluator.prepare codelen=67):
     * currentContext = new PathfindingContext(region, mob); this.mob = mob;
     * [nodes.clear() REPLACED by the pool laundering]; entityWidth/Height/
     * Depth = Mth.floor(bb + 1.0F). The native tick runs FIRST (search
     * boundary): epoch++ + previous-search stats flush; any failure is
     * swallowed (stats only, parity never depends on the native).
     */
    public static void prepare(NodeEvaluator eval, PathNavigationRegion region,
                               Mob mob) {
        int mapSize = eval.nodes.size();
        long hits = HITS.sumThenReset();
        long news = NEWS.sumThenReset();
        long overflows = OVERFLOWS.sumThenReset();
        try {
            navPoolTick(mapSize, hits, news, overflows);
        } catch (Throwable t) {
            // telemetry only — the pool must stay fail-open vanilla
        }

        eval.currentContext = new PathfindingContext(region, mob);
        eval.mob = mob;
        Int2ObjectMap<Node> nodes = eval.nodes;
        if (nodes.size() > MAP_CAP) {
            nodes.clear();
            OVERFLOWS.increment();
        } else {
            for (Node n : nodes.values()) {
                n.heapIdx = -1;
                n.closed = false;
                n.g = 0.0F;
                n.h = 0.0F;
                n.f = 0.0F;
                n.cameFrom = null;
                n.walkedDistance = 0.0F;
                n.costMalus = 0.0F;
                n.type = PathType.BLOCKED;
            }
        }
        eval.entityWidth = Mth.floor(mob.getBbWidth() + 1.0F);
        eval.entityHeight = Mth.floor(mob.getBbHeight() + 1.0F);
        eval.entityDepth = Mth.floor(mob.getBbWidth() + 1.0F);
    }

    /**
     * Receiver-prepended entry for the NodeEvaluator.getNode(III) retarget.
     * Vanilla: nodes.computeIfAbsent(Node.createHash(x,y,z), -> new Node).
     * Pool: get; null or stale-position (cross-search hash collision) ->
     * new Node + put. Bit-identical results, zero alloc on hits.
     */
    public static Node getNode(NodeEvaluator eval, int x, int y, int z) {
        Int2ObjectMap<Node> nodes = eval.nodes;
        int hash = Node.createHash(x, y, z);
        Node n = nodes.get(hash);
        if (n == null || n.x != x || n.y != y || n.z != z) {
            n = new Node(x, y, z);
            nodes.put(hash, n);
            NEWS.increment();
        } else {
            HITS.increment();
        }
        return n;
    }
}
