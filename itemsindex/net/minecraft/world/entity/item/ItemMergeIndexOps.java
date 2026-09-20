package net.minecraft.world.entity.item;

import it.unimi.dsi.fastutil.longs.Long2ObjectOpenHashMap;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.locks.ReentrantLock;

import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * ItemMergeIndexOps — TASK-395 MEGA-ROUND agent-A lever `items_index`.
 *
 * Section-resident spatial-hash index of ItemEntity merge candidates.
 * The patched (body-swapped) {@code ItemEntity.mergeWithNeighbours()} calls
 * {@link #candidates} INSTEAD of
 * {@code Level.getEntitiesOfClass(ItemEntity.class, inflatedBox, predicate)}:
 * the AABB section-scan is replaced by a 0.5-grid lookup. Everything else
 * stays VANILLA — the body keeps the self-gate (isMergable), the loop body
 * (neighbour isMergable re-check, paper fixItemsMergingThroughWalls clipDirect
 * exit, tryToMerge, isRemoved exit) as in-class bytes calling the ORIGINAL
 * private methods, and the radius/y-inflate/box math is the vanilla formula
 * recomputed here (spigotConfig.itemMerge + paper onlyMergeItemsHorizontally),
 * both public configuration surface.
 *
 * Index shape: per-Level 0.5-grid fastutil Long2ObjectOpenHashMap of tiny
 * cells, striped across 64 ReentrantLock shards (region_threads>=2 safe).
 * Entries are (ref, stamp); stamps are wall-clock buckets so there is no
 * MinecraftServer dependency. Maintenance is LAZY and query-proportional:
 *   - refresh(self) at every query (an item moves only during its own tick,
 *     before its merge check — the query refreshes the mover),
 *   - compact-on-touch drops removed entries and keeps the latest entry per
 *     entity,
 *   - stale entries are semantics-free: a candidate is returned only if it is
 *     alive, != self, and its CURRENT bounding box intersects the query box —
 *     exactly the filters vanilla applies (section query box test + predicate
 *     x != this; the predicate's isMergable is subsumed by the vanilla loop's
 *     per-candidate isMergable re-check, which vanilla evaluates no earlier).
 * Cell coverage: candidate half-extent is 0.125 (ITEM dimensions 0.25), so a
 * center cell can sit at most one 0.5-cell outside the query box while the
 * boxes still intersect — the scan covers the query box cell range expanded
 * by exactly one cell per axis (superset of the vanilla AABB hit set).
 *
 * Deviations (LEVER.md): candidate order = cell-key/insertion order (vanilla =
 * section-storage order); index sees an entity at its first executed merge
 * query (vanilla stride-gated the same way).
 *
 * Diagnostics: QUERIES/CANDIDATES/SWEEP_DROPS are plain public counters
 * (absorb-side evidence that the lever armed and how often it engaged).
 */
public final class ItemMergeIndexOps {

    private ItemMergeIndexOps() {}

    private static final int SHARDS = 64;
    private static final int SHARD_MASK = SHARDS - 1;
    private static final int COMPACT_AT = 128;
    /** Stamp freshness window in ~4.19s units (~134s) — generous vs the
     * vanilla 40-tick static merge stride at any TPS above ~0.3. */
    private static final long STAMP_WINDOW = 32L;
    /** ITEM entity half-extent (0.25 dims) < one 0.5-cell — one-cell cover. */
    private static final int CELL_EXPAND = 1;

    public static volatile long QUERIES = 0;
    public static volatile long CANDIDATES = 0;
    public static volatile long SWEEP_DROPS = 0;

    private static final class Cell {
        ItemEntity[] refs = new ItemEntity[4];
        long[] stamps = new long[4];
        int n;
    }

    private static final class Shard extends ReentrantLock {
        final Long2ObjectOpenHashMap<Cell> cells = new Long2ObjectOpenHashMap<>(256, 0.6f);
    }

    private static final ConcurrentHashMap<Object, Shard[]> BY_LEVEL = new ConcurrentHashMap<>();

    private static final ThreadLocal<ItemEntity[]> SCRATCH =
            ThreadLocal.withInitial(() -> new ItemEntity[8]);

    private static long stampNow() {
        return System.nanoTime() >>> 22;
    }

    private static long cellKey(double x, double y, double z) {
        // 0.5-block quantization, packed 21 bits per axis (sign-truncated —
        // wrap-around pairs are ~2M blocks apart and can never pass the box
        // intersection test, so collisions degrade perf, never correctness).
        long qx = (long) Math.floor(x * 2.0);
        long qy = (long) Math.floor(y * 2.0);
        long qz = (long) Math.floor(z * 2.0);
        return ((qx & 0x1FFFFFL) << 42) | ((qz & 0x1FFFFFL) << 21) | (qy & 0x1FFFFFL);
    }

    private static long packedKey(int x, int y, int z) {
        return (((long) x & 0x1FFFFFL) << 42)
             | (((long) z & 0x1FFFFFL) << 21)
             | ((long) y & 0x1FFFFFL);
    }

    private static Shard[] shardsFor(Level level) {
        return BY_LEVEL.computeIfAbsent(level, l -> new Shard[SHARDS]);
    }

    private static void grow(Cell c) {
        int cap = c.refs.length;
        int ncap = cap < 8 ? 8 : cap + (cap >> 1);
        ItemEntity[] nr = new ItemEntity[ncap];
        long[] ns = new long[ncap];
        System.arraycopy(c.refs, 0, nr, 0, c.n);
        System.arraycopy(c.stamps, 0, ns, 0, c.n);
        c.refs = nr;
        c.stamps = ns;
    }

    /** In-place compaction under the shard lock: drop removed/aged entries,
     * keep the LATEST entry per entity (linear dedup — cells are small). */
    private static void compact(Cell c, long now) {
        int w = 0;
        for (int i = 0; i < c.n; i++) {
            ItemEntity e = c.refs[i];
            if (e == null || e.isRemoved() || now - c.stamps[i] > STAMP_WINDOW) {
                SWEEP_DROPS++;
                continue;
            }
            boolean superseded = false;
            for (int j = i + 1; j < c.n; j++) {
                if (c.refs[j] == e) { superseded = true; break; }
            }
            if (superseded) {
                SWEEP_DROPS++;
                continue;
            }
            c.refs[w] = e;
            c.stamps[w] = c.stamps[i];
            w++;
        }
        for (int i = w; i < c.n; i++) c.refs[i] = null;
        c.n = w;
    }

    private static void refresh(Shard[] shards, ItemEntity self, long now) {
        long key = cellKey(self.getX(), self.getY(), self.getZ());
        Shard sh = shards[(int) (key & SHARD_MASK)];
        sh.lock();
        try {
            Cell c = sh.cells.get(key);
            if (c == null) {
                c = new Cell();
                sh.cells.put(key, c);
            }
            if (c.n == c.refs.length) grow(c);
            c.refs[c.n] = self;
            c.stamps[c.n] = now;
            c.n++;
            if (c.n >= COMPACT_AT) compact(c, now);
        } finally {
            sh.unlock();
        }
    }

    /**
     * The retargeted merge candidate scan. Called from the body-swapped
     * ItemEntity.mergeWithNeighbours AFTER the vanilla self-gate
     * (isMergable). Computes the vanilla inflated box (spigotConfig.itemMerge
     * radius; paper onlyMergeItemsHorizontally collapses the y-inflate to
     * 0.0), then returns the index candidates: alive, != self, box
     * intersecting. The vanilla loop applies the remaining per-candidate
     * filters (isMergable re-check, wall clip) on the ORIGINAL private code.
     */
    public static List<ItemEntity> candidates(ItemEntity self) {
        QUERIES++;
        Level level = self.level();
        double d = level.spigotConfig.itemMerge;
        AABB box = self.getBoundingBox().inflate(d,
                level.paperConfig().entities.behavior.onlyMergeItemsHorizontally ? 0.0D : d - 0.5D, d);
        return queryIndex(self, level, box);
    }

    private static List<ItemEntity> queryIndex(ItemEntity self, Level level, AABB box) {
        Shard[] shards = shardsFor(level);
        long now = stampNow();
        refresh(shards, self, now);

        ArrayList<ItemEntity> out = new ArrayList<>(4);
        int x0 = floor2(box.minX) - CELL_EXPAND, x1 = floor2(box.maxX) + CELL_EXPAND;
        int y0 = floor2(box.minY) - CELL_EXPAND, y1 = floor2(box.maxY) + CELL_EXPAND;
        int z0 = floor2(box.minZ) - CELL_EXPAND, z1 = floor2(box.maxZ) + CELL_EXPAND;
        ItemEntity[] scratch = SCRATCH.get();
        for (int x = x0; x <= x1; x++) {
            for (int y = y0; y <= y1; y++) {
                for (int z = z0; z <= z1; z++) {
                    long key = packedKey(x, y, z);
                    Shard sh = shards[(int) (key & SHARD_MASK)];
                    int snapN;
                    sh.lock();
                    try {
                        Cell c = sh.cells.get(key);
                        if (c == null) {
                            continue;
                        }
                        compact(c, now);
                        if (c.n == 0) {
                            sh.cells.remove(key);
                            continue;
                        }
                        // private snapshot: reads after unlock are race-free
                        while (scratch.length < c.n) {
                            scratch = new ItemEntity[scratch.length * 2];
                            SCRATCH.set(scratch);
                        }
                        System.arraycopy(c.refs, 0, scratch, 0, c.n);
                        snapN = c.n;
                    } finally {
                        sh.unlock();
                    }
                    for (int i = 0; i < snapN; i++) {
                        ItemEntity cand = scratch[i];
                        if (cand == null || cand.isRemoved()) continue;
                        if (cand == self) continue; // vanilla predicate: x != this
                        if (!cand.getBoundingBox().intersects(box)) continue;
                        // vanilla getEntitiesOfClass returns DISTINCT entities;
                        // a ref can linger in a stale cell (moved between
                        // touches) — linear dedup, candidate sets are tiny.
                        boolean dup = false;
                        for (int j = 0; j < out.size(); j++) {
                            if (out.get(j) == cand) { dup = true; break; }
                        }
                        if (dup) continue;
                        CANDIDATES++;
                        out.add(cand);
                    }
                }
            }
        }
        return out;
    }

    private static int floor2(double v) {
        return (int) Math.floor(v * 2.0);
    }
}
