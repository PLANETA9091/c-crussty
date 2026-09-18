package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.CyclicBarrier;
import java.util.function.Consumer;

import ca.spottedleaf.moonrise.common.util.TickThread;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.entity.EntityTickList;

/**
 * RegionTickOps (S7-156, TASK-295 — ARCH-ATTACK lever #7: region-threaded
 * entity ticking, the last xN-class lane after single-lever exhaustion).
 *
 * Feasibility gate (S7-155): GREEN — hot per-tick path is shared-RNG clean
 * (per-entity Entity.random only), dominant interactions are spatially local
 * (push ~1 block, merge ~0.5, collide = movement AABB vs 8x8-chunk regions),
 * moonrise EntityLookup is already concurrent, and the entity loop is a
 * SINGLE forEach segment in ServerLevel.tick — the exact retarget point.
 *
 * Design (preregistered S7-155 §5):
 *  - The per-entity logic stays BYTE-FOR-BYTE vanilla: the vanilla consumer
 *    lambda created by ServerLevel.tick is received and executed as-is.
 *    Only the LOOP CONTAINER changes: instead of one global
 *    EntityTickList.forEach pass, entities are snapshotted (vanilla
 *    iteration, single thread), partitioned into W spatial buckets, and the
 *    buckets are ticked in parallel by W-1 persistent TickThread helpers +
 *    the main thread (slot 0). Intra-bucket order = snapshot order = vanilla
 *    insertion order.
 *  - EntityTickList mutations are event-deferred during the parallel phase:
 *    the only add site (ServerLevel$EntityCallbacks.onTickingStart) and the
 *    only remove site (onTickingEnd) are retargeted here; when a phase is
 *    active they enqueue (FIFO) and are drained at the barrier in exact
 *    callback order. Entities added mid-phase therefore begin ticking next
 *    tick — the same observable behaviour as the vanilla iterator snapshot
 *    (IteratorSafeOrderedReferenceSet never visits mid-iteration adds).
 *    Removals are deferred harmlessly: the vanilla consumer's own
 *    isRemoved() gate governs ticking, and the set drain happens at join.
 *  - Parity boundary (documented, owner bar = median-exact parity): reads of
 *    entities in other buckets observe pre-tick or mid-tick positions
 *    (interleaving), reads within a bucket are sequential vanilla; per-tick
 *    RNG is per-entity (S7-155 census), the rare shared Level.random paths
 *    (Zombie.hurtServer etc, 0.19% profile) race benignly on Xoroshiro
 *    (lost updates only, no corruption/exceptions).
 *  - Fail-closed: WORKERS <= 1 -> plain vanilla list.forEach (bit-identical
 *    path). No fallbacks beyond that: exceptions in worker buckets are
 *    stashed and rethrown on the main thread after the join barrier
 *    (server dies, vanilla crash semantics preserved).
 *
 * Dormant-invisible: env CRUSSTY_REGION_THREADS (parsed once; 1/absent =
 * vanilla passthrough). Region span: 8 chunks (quadrant buckets at W=4,
 * x-stripes otherwise).
 */
public final class RegionTickOps {

    private static final int WORKERS = parseWorkers();
    private static final int REGION_CHUNKS = 8;

    private static final class Mut {
        final boolean add;
        final Entity entity;
        Mut(boolean add, Entity entity) { this.add = add; this.entity = entity; }
    }

    /** FIFO of EntityCallbacks mutations observed during the parallel phase. */
    private static final ConcurrentLinkedQueue<Mut> PENDING = new ConcurrentLinkedQueue<>();
    private static volatile boolean phaseActive = false;

    /** Per-phase state, published before GO, read by helpers after GO
     *  (CyclicBarrier arrival = happens-before edge). */
    private static volatile List<Entity>[] tasks = new List[0];
    private static volatile Consumer<Entity>[] consumers = new Consumer[0];
    private static volatile Throwable workerError;

    private static final CyclicBarrier GO = new CyclicBarrier(WORKERS);
    private static final CyclicBarrier DONE = new CyclicBarrier(WORKERS);

    private static volatile boolean helpersStarted = false;

    private RegionTickOps() {}

    private static int parseWorkers() {
        try {
            String v = System.getenv("CRUSSTY_REGION_THREADS");
            if (v == null) return 1;
            int w = Integer.parseInt(v.trim());
            if (w <= 1) return 1;
            return Math.min(w, 16);
        } catch (Throwable t) {
            return 1;
        }
    }

    /** Retarget of the single ServerLevel.tick forEach call site (1:1 stack). */
    public static void forEach(EntityTickList list, Consumer<Entity> consumer) {
        int w = WORKERS;
        if (w <= 1) {
            list.forEach(consumer); // vanilla bit-identical
            return;
        }
        try {
            parallelTick(list, consumer, w);
        } finally {
            // belt and suspenders: the phase must never outlive this frame
            phaseActive = false;
        }
    }

    @SuppressWarnings("unchecked")
    private static void parallelTick(EntityTickList list, Consumer<Entity> consumer, int w) {
        // Phase 1 (serial): snapshot via the VANILLA iteration protocol.
        List<Entity> all = new ArrayList<>();
        list.forEach(all::add);

        // Phase 2 (serial): spatial partition.
        List<Entity>[] buckets = new List[w];
        for (int i = 0; i < w; i++) {
            buckets[i] = new ArrayList<>(Math.max(16, all.size() / w + 16));
        }
        for (int i = 0, n = all.size(); i < n; i++) {
            Entity e = all.get(i);
            buckets[bucketOf(e, w)].add(e);
        }
        Consumer<Entity>[] cs = new Consumer[w];
        for (int i = 0; i < w; i++) cs[i] = consumer;
        tasks = buckets;
        consumers = cs;
        workerError = null;

        // Phase 3 (parallel): main = slot 0, helpers = 1..w-1.
        // tickBucket(0) joins DONE in its own finally — the ONLY join point
        // (a second awaitDone here would deadlock: helpers loop back to GO
        // after the single DONE release).
        ensureHelpers(w);
        phaseActive = true;
        try {
            GO.await(); // releases helpers (they park here between ticks)
            tickBucket(0);
        } catch (Throwable t) {
            if (workerError == null) workerError = t;
            phaseActive = false;
        }
        phaseActive = false;

        // Phase 4 (serial): drain deferred EntityCallbacks mutations in FIFO order.
        Mut m;
        while ((m = PENDING.poll()) != null) {
            if (m.add) list.add(m.entity); else list.remove(m.entity);
        }

        Throwable err = workerError;
        if (err != null) {
            if (err instanceof RuntimeException) throw (RuntimeException) err;
            if (err instanceof Error) throw (Error) err;
            throw new RuntimeException(err);
        }
    }

    private static void tickBucket(int slot) {
        try {
            List<Entity> bucket = tasks[slot];
            Consumer<Entity> consumer = consumers[slot];
            for (int i = 0, n = bucket.size(); i < n; i++) {
                consumer.accept(bucket.get(i)); // vanilla per-entity logic, bit-for-bit
            }
        } catch (Throwable t) {
            if (workerError == null) workerError = t; // crash surfaces on main at join
        } finally {
            try {
                DONE.await();
            } catch (Throwable t) {
                if (workerError == null) workerError = t;
            }
        }
    }

    private static void ensureHelpers(int w) {
        if (helpersStarted) return;
        synchronized (RegionTickOps.class) {
            if (helpersStarted) return;
            for (int i = 1; i < w; i++) {
                final int slot = i;
                Thread t = new TickThread(() -> {
                    while (true) {
                        try {
                            GO.await();
                            tickBucket(slot);
                        } catch (Throwable tt) {
                            // broken barrier / tick failure: surface via workerError
                            // (main rethrows at join); park briefly, never hot-spin
                            if (workerError == null) workerError = tt;
                            try {
                                Thread.sleep(50);
                            } catch (InterruptedException ie) {
                                return;
                            }
                        }
                    }
                }, "crussty-region-worker-" + slot);
                t.setDaemon(true);
                t.start();
            }
            helpersStarted = true;
        }
    }

    /**
     * Retarget of the ONLY EntityTickList.add call site
     * (ServerLevel$EntityCallbacks.onTickingStart) — deferred during a phase.
     */
    public static void onTickingStart(EntityTickList list, Entity entity) {
        if (phaseActive) {
            PENDING.add(new Mut(true, entity));
        } else {
            list.add(entity);
        }
    }

    /**
     * Retarget of the ONLY EntityTickList.remove call site
     * (ServerLevel$EntityCallbacks.onTickingEnd) — deferred during a phase.
     */
    public static void onTickingEnd(EntityTickList list, Entity entity) {
        if (phaseActive) {
            PENDING.add(new Mut(false, entity));
        } else {
            list.remove(entity);
        }
    }

    /** Spatial bucketing: 8-chunk regions; quadrants at W=4, x-stripes else. */
    static int bucketOf(Entity entity, int w) {
        ChunkPos cp = entity.chunkPosition();
        int rx = Math.floorDiv(cp.x, REGION_CHUNKS);
        int rz = Math.floorDiv(cp.z, REGION_CHUNKS);
        if (w == 4) return ((rx & 1) << 1) | (rz & 1);
        if (w == 2) return rx & 1;
        return Math.floorMod(rx * 668265261 + rz * 374761393, w);
    }

    /** Census probe for the OFFLINE harness (must equal env at boot). */
    public static int workers() {
        return WORKERS;
    }
}
