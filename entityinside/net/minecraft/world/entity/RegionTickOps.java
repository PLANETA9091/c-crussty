package net.minecraft.world.entity;

import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.CyclicBarrier;
import java.util.function.Consumer;

import it.unimi.dsi.fastutil.objects.ObjectOpenHashSet;

import ca.spottedleaf.moonrise.common.util.TickThread;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.Level;
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
 *
 * S7-167 STEAL MODE (lever #13, RECON-15/TASK-333): env CRUSSTY_REGION_STEAL.
 * RECON-15 (fresh s7169 wall profile) measured the static-bucket phase 3:
 * main bucket-0 wall 478 samples vs 651 per helper (total work 2432, critical
 * path 651 => mean parallelism 3.74/4 = 93% efficiency) and the main thread
 * PARKED on the DONE barrier 121/900 = 13.4% of its wall — static quadrant
 * partitioning is workload-UNAWARE (bucket imbalance + jitter is suffered,
 * not shared). The steal mode replaces per-slot buckets with ONE shared
 * snapshot array in exact vanilla iteration order + an AtomicInteger chunk
 * cursor (STEAL_CHUNK=512): every participant (main + W-1 helpers) pulls
 * chunks until the queue is exhausted. Per-entity logic is untouched
 * (vanilla consumer bit-for-bit); intra-chunk order = snapshot order;
 * cross-chunk interleaving is the SAME parity class the owner bar already
 * accepts for region ticks (median-exact, documented S7-155 boundary).
 * Expected effect: DONE-park -> ~0 (the main finishes last, not first),
 * critical path 651 -> ~608 samples (total/4), GC/worker jitter absorbed by
 * the queue instead of idling the main thread. Rollback = env (bank v3
 * keeps the static path bit-identical when CRUSSTY_REGION_STEAL is absent).
 */
public final class RegionTickOps {

    private static final int WORKERS = parseWorkers();
    private static final int REGION_CHUNKS = 8;

    /**
     * S7-160 BATCH-COLLECTOR / S7-162 no-ensure: with CRUSSTY_BATCH_COLLECTOR=1
     * the vanilla StepBasedCollector instance inside Entity.insideEffectCollector
     * is replaced AT CONSTRUCTION TIME — the single NEW site in Entity.<init>
     * is retargeted to BatchCollector by the entity_compose compose chain
     * (persistent by construction; the S7-160/161 lazy ensure-swap was RETIRED:
     * run 35391679176 proved all 801 BatchCollector.<init> samples came from
     * the per-tick ensure loop re-constructing for pre-arm entities — the
     * swap never stuck — and the gate itself burned 737 samples).
     *
     * This flag now gates (a) the periodic INSTANCES telemetry print below
     * and (b) nothing else: the tickBucket hot path is vanilla-identical.
     * Gating through this compile-time constant keeps the BatchCollector
     * class resolve lazy: with the gate off the constant-pool entry is never
     * executed and the (possibly undefined) bridge class is never touched.
     */
    private static final boolean BATCH_COLLECTOR = parseBatchCollector();

    /**
     * S7-162 INSTANCES telemetry: BatchCollector construction count, printed
     * every TELEMETRY_INTERVAL forEach invocations (main-thread only —
     * ServerLevel.tick is the sole caller). Answers the S7-161 open question
     * (a): is the ctor called per-spawn (live-scene natural spawn flow) or
     * repeatedly per-tick (the retired ensure loop). Never executed with the
     * gate off (lazy CP resolution — the bridge class stays undefined).
     */
    private static final long TELEMETRY_INTERVAL = 600;
    private static long telemetryTicks = 0;

    private static boolean parseBatchCollector() {
        try {
            String v = System.getenv("CRUSSTY_BATCH_COLLECTOR");
            if (v == null) return false;
            v = v.trim().toLowerCase();
            return v.equals("1") || v.equals("true") || v.equals("on") || v.equals("yes");
        } catch (Throwable t) {
            return false;
        }
    }

    private static final class Mut {
        final boolean add;
        final Entity entity;
        Mut(boolean add, Entity entity) { this.add = add; this.entity = entity; }
    }

    /** FIFO of EntityCallbacks mutations observed during the parallel phase. */
    private static final ConcurrentLinkedQueue<Mut> PENDING = new ConcurrentLinkedQueue<>();
    private static volatile boolean phaseActive = false;

    /**
     * ROUND-397 items_sweep2 gate for ItemsSweepOps.tickMerge: the item-merge
     * suppression is valid ONLY inside the parallel phase (every bucket has
     * been swept this tick and every mergeWithNeighbours caller is a slot
     * worker or the main slot-0 path). Out-of-phase callers fall back to the
     * pristine vanilla body. Package-visible state, public probe.
     */
    public static boolean sweepPhase() {
        return phaseActive;
    }

    /**
     * BU-DEFER (S7-168, STEAL v2 defect-fix — TASK-335): deferred
     * ServerLevel.sendBlockUpdated navigate-phase records from workers.
     * s7176 root-cause: a worker iterating navigatingMobs while the main
     * thread mutates it detonates the fastutil SetIterator NPE ("wrapped is
     * null") — and the main thread then dies on the same set. Workers
     * enqueue [ServerLevel, BlockPos, old, new, flags]; the main thread
     * replays them FIFO (per-worker queue, queues drained in registration
     * order) in phase 4, right after the EntityCallbacks Mut drain.
     * Registration: a worker adds its queue once (CopyOnWriteArrayList +
     * per-worker REGGED flag; lists carry distinct elements so equals()
     * can never collapse two queues).
     */
    private static final java.util.List<java.util.ArrayList<Object[]>> BU_REG =
            new java.util.concurrent.CopyOnWriteArrayList<>();
    private static final ThreadLocal<java.util.ArrayList<Object[]>> BU_TL =
            ThreadLocal.withInitial(java.util.ArrayList::new);
    private static final ThreadLocal<Boolean> BU_REGGED =
            ThreadLocal.withInitial(() -> Boolean.FALSE);

    /** Worker entry (called by BlockUpdateOps.handle for worker threads). */
    public static void deferBlockUpdate(Object level, Object pos, Object oldS,
                                        Object newS, int flags) {
        java.util.ArrayList<Object[]> q = BU_TL.get();
        q.add(new Object[]{level, pos, oldS, newS, flags});
        if (!BU_REGGED.get()) {
            BU_REG.add(q);
            BU_REGGED.set(Boolean.TRUE);
        }
    }

    /** Main-thread phase-4 replay: FIFO across per-worker queues. */
    public static void drainDeferredBlockUpdates() {
        if (BU_REG.isEmpty()) {
            return;
        }
        for (java.util.ArrayList<Object[]> q : BU_REG) {
            while (!q.isEmpty()) {
                Object[] rec = q.remove(0);
                net.minecraft.server.level.BlockUpdateOps.vanilla(
                        (net.minecraft.server.level.ServerLevel) rec[0],
                        (net.minecraft.core.BlockPos) rec[1],
                        (net.minecraft.world.level.block.state.BlockState) rec[2],
                        (net.minecraft.world.level.block.state.BlockState) rec[3],
                        (Integer) rec[4]);
            }
        }
    }

    /**
     * S7-157b (run 35353820223 crash lesson): Paper pumps MAIN-thread
     * mid-tick tasks per entity tick from Level.guardEntityTick ->
     * moonrise$midTickTasks -> ServerChunkCache$MainThreadExecutor.pollTask —
     * that queue is main-thread-only and its pollTask is not thread-safe
     * against concurrent empties (NoSuchElementException race, live crash at
     * 14:10:20). Region workers therefore NEVER pump: the gate below
     * suppresses the call on worker threads; the main thread keeps its exact
     * vanilla pump (bucket 0 + per-tick loops + tickBlockEntities pump).
     * Liveness: mid-tick tasks still drain every tick via
     * MinecraftServer.tickMidTickTasks + the main thread's own
     * guardEntityTick pumps (bucket 0) + the tickBlockEntities pump.
     */
    private static final ThreadLocal<Boolean> WORKER_FLAG =
            ThreadLocal.withInitial(() -> Boolean.FALSE);

    /**
     * Per-phase state, published before GO, read by helpers after GO
     *  (CyclicBarrier arrival = happens-before edge).
     *
     * S7-158c GC-DIET: the per-tick snapshot+partition is now ZERO-ALLOC in
     * the steady state (live leg 35363758352: young GC 118 -> 155 = +31.4%
     * vs gate <= +15% — the v1 design allocated a fresh ArrayList snapshot
     * (150k refs) + W bucket ArrayLists (~150k refs) + a consumer array
     * EVERY tick ≈ MBs/s of churn on top of vanilla). Persistent arrays are
     * grown only on demand and reused; publication to workers keeps the
     * barrier happens-before edge. Consumer array is a constant (single
     * shared consumer for every slot).*/
    private static volatile Entity[][] bucketArr = new Entity[0][];
    private static volatile int[] bucketLen = new int[0];
    private static volatile Consumer<Entity> consumer;
    private static volatile Throwable workerError;

    // S7-172: under MAIN_OFFLOAD (REGION_STEAL="2") an extra helper joins
    // both barriers — main orchestrates instead of ticking slot 0, so the
    // participants are main + w helpers = w+1 (legacy: main-as-slot-0 +
    // w-1 helpers = w). Inlined parse (not a field) because these finals
    // initialize textually BEFORE the STEAL/MAIN_OFFLOAD flags below.
    private static final CyclicBarrier GO =
            new CyclicBarrier(WORKERS + (parseMainOffload() ? 1 : 0));
    private static final CyclicBarrier DONE =
            new CyclicBarrier(WORKERS + (parseMainOffload() ? 1 : 0));

    /**
     * S7-174 v2 pump capture (main-thread-confined): the Level of the first
     * entity seen by this tick's fill pass. Main uses it during phase-3 to
     * pump the main-only mid-tick queue (moonrise$midTickTasks) instead of
     * parking at DONE — restoring the legacy interleave (main pumped
     * per-entity from bucket 0). Null list -> null level -> v1 parking for
     * that tick. Only main reads/writes it (fill and phase-3 are both on
     * the server thread, program order suffices).
     */
    private static Level pumpLevel;

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

    /**
     * S7-167 STEAL MODE (lever #13): chunked shared queue instead of static
     * per-slot buckets. Dormant-invisible: absent env = static path
     * bit-identical to the shipped bank v3 behaviour.
     */
    private static final boolean STEAL = parseSteal();

    /** Entities per stolen chunk: tail imbalance <= STEAL_CHUNK (~0.34% of 150k). */
    private static final int STEAL_CHUNK = 512;

    private static boolean parseSteal() {
        try {
            String v = System.getenv("CRUSSTY_REGION_STEAL");
            if (v == null) return false;
            v = v.trim().toLowerCase();
            return v.equals("1") || v.equals("true") || v.equals("on") || v.equals("yes");
        } catch (Throwable t) {
            return false;
        }
    }

    /**
     * S7-172 P2-OFFLOAD v1 (RECON-37 I=1.01 OFFLOAD-READY, TASK-371):
     * REGION_STEAL="2" = MAIN-OFFLOAD static mode — w helpers (not w-1)
     * tick ALL w buckets; main orchestrates only (snapshot fill -> GO
     * release -> DONE join -> phase-4/4b drains). Motivation: RECON-37
     * threaded wall (s7196 re-roll 35488526730, digest-verified): worker
     * duties 67.4/66.3/68.2% (I=1.01 <= 1.15 = OFFLOAD-READY), main =
     * slot-0 bucket + its serial phases; draining main's bucket to a 4th
     * helper re-targets the critical path from main-total (19.7 units,
     * RECON-36 §2) to the slowest worker (14.9 units at I=1.0 — RECON-36
     * §4 ceiling +25..33% > ДВОЙНОГО БАРА). Parity: bucketOf unchanged
     * (same entity->slot map), per-bucket order = snapshot order, GO/DONE
     * discipline unchanged — the interleave class is identical to the
     * banked region_threads>=2 baseline; the only change is WHICH thread
     * executes slot 0 (slot-0 mid-tick pump suppression = the SAME
     * S7-157b class already accepted for slots 1..w-1; liveness drains
     * via MinecraftServer.tickMidTickTasks + tickBlockEntities pump).
     * "0"/absent = legacy static bit-identical; "1" = STEAL v1. The mode
     * rides the REGION_STEAL input enum because world-bench.yml is at the
     * 25-input GitHub cap (no new input possible). Precedence: with
     * STEAL="1" this flag is ignored (steal branch first).
     */
    private static final boolean MAIN_OFFLOAD = parseMainOffload();

    private static boolean parseMainOffload() {
        try {
            String v = System.getenv("CRUSSTY_REGION_STEAL");
            if (v == null) return false;
            String t = v.trim();
            // "2" = v1 static main-free offload; "3" = S7-174 v2 offload
            // with main pumping mid-tick duties during phase-3.
            return t.equals("2") || t.equals("3");
        } catch (Throwable t) {
            return false;
        }
    }

    /**
     * S7-174 v2: REGION_STEAL="3" = MAIN_OFFLOAD + main pumps the
     * main-only mid-tick queue during phase-3 (pump-restoration lever —
     * the largest sub-lane of the v1 regression, S7-174 recon).
     */
    private static final boolean PUMP_OFFLOAD = parsePumpOffload();

    private static boolean parsePumpOffload() {
        try {
            String v = System.getenv("CRUSSTY_REGION_STEAL");
            if (v == null) return false;
            return v.trim().equals("3");
        } catch (Throwable t) {
            return false;
        }
    }

    /**
     * Shared snapshot for the steal mode: ONE array in exact vanilla
     * iteration order (reused, grown on demand — zero-alloc steady state).
     * Publication to helpers = volatile write + the GO barrier
     * happens-before edge (same guarantee as the static mode).
     */
    private static volatile Entity[] stealArr = new Entity[0];
    private static volatile int stealLen = 0;

    /**
     * Chunk cursor: main resets to 0 BEFORE GO (happens-before edge);
     * participants pull index windows with getAndAdd(STEAL_CHUNK).
     */
    private static final java.util.concurrent.atomic.AtomicInteger STEAL_CURSOR =
            new java.util.concurrent.atomic.AtomicInteger();

    /** Main-thread-only snapshot length scratch (no per-tick alloc). */
    private static int snapLen;

    /** Retarget of the single ServerLevel.tick forEach call site (1:1 stack). */
    public static void forEach(EntityTickList list, Consumer<Entity> consumer) {
        if (BATCH_COLLECTOR && (++telemetryTicks % TELEMETRY_INTERVAL) == 0L) {
            System.err.println(
                    "[crussty-plugin] batch_collector: telemetry tick=" + telemetryTicks
                    + " instances=" + BatchCollector.instances()
                    + " workers=" + WORKERS);
        }
        int w = WORKERS;
        if (w <= 1) {
            list.forEach(consumer); // vanilla bit-identical
            return;
        }
        try {
            if (STEAL) {
                stealTick(list, consumer, w);
            } else {
                parallelTick(list, consumer, w);
            }
        } finally {
            // belt and suspenders: the phase must never outlive this frame
            phaseActive = false;
        }
    }

    /**
     * S7-167 steal-mode tick: ONE shared snapshot array in exact vanilla
     * iteration order + chunk cursor; main and helpers all pull chunks until
     * the queue is exhausted, so nobody idles on DONE while work remains.
     */
    private static void stealTick(EntityTickList list, Consumer<Entity> c, int w) {
        // Phase 1+2 (serial, zero-alloc steady state): vanilla-protocol
        // forEach pass fills the shared array in place (grow-on-overflow,
        // rare); publication = volatile writes + GO barrier happens-before.
        Entity[] arr = stealArr;
        if (arr.length == 0) {
            arr = new Entity[8192];
            stealArr = arr;
        }
        snapLen = 0;
        list.forEach(e -> {
            Entity[] b = stealArr;
            int i = snapLen;
            if (i == b.length) {
                b = java.util.Arrays.copyOf(b, i * 2);
                stealArr = b;
            }
            b[i] = e;
            snapLen = i + 1;
        });
        arr = stealArr;
        final int total = snapLen;
        consumer = c;
        stealLen = total;

        // Phase 3 (parallel): main = one of the pullers, helpers 1..w-1.
        // stealChunks() joins DONE in its own finally — the ONLY join point.
        ensureHelpers(w);
        phaseActive = true;
        STEAL_CURSOR.set(0);
        try {
            GO.await(); // releases helpers (they park here between ticks)
            stealChunks();
        } catch (Throwable t) {
            if (workerError == null) workerError = t;
            phaseActive = false;
        }
        phaseActive = false;

        // Post-join retention hygiene: null stale tail refs beyond snapLen
        // so discarded entities are not kept alive by the reused array
        // (zero-alloc pass, main-only, workers parked after DONE).
        for (int j = total, n2 = arr.length; j < n2; j++) arr[j] = null;

        // Phase 4 (serial): drain deferred EntityCallbacks mutations in FIFO order.
        Mut m;
        while ((m = PENDING.poll()) != null) {
            if (m.add) list.add(m.entity); else list.remove(m.entity);
        }

        // Phase 4b (serial, S7-168): replay deferred sendBlockUpdated
        // navigate-passes (STEAL v2 defect-fix) — main-only, after join.
        drainDeferredBlockUpdates();

        Throwable err = workerError;
        if (err != null) {
            if (err instanceof RuntimeException) throw (RuntimeException) err;
            if (err instanceof Error) throw (Error) err;
            throw new RuntimeException(err);
        }
    }

    /**
     * Pull STEAL_CHUNK-sized windows from the shared cursor until exhausted.
     * Intra-chunk order = snapshot order = vanilla insertion order; the
     * DONE barrier in the finally keeps the join point identical to the
     * static mode (crash semantics: workerError rethrown on main).
     */
    private static void stealChunks() {
        try {
            Consumer<Entity> c = consumer;
            Entity[] b = stealArr;
            int total = stealLen;
            for (int i = STEAL_CURSOR.getAndAdd(STEAL_CHUNK); i < total;
                 i = STEAL_CURSOR.getAndAdd(STEAL_CHUNK)) {
                int end = Math.min(i + STEAL_CHUNK, total);
                for (int j = i; j < end; j++) {
                    c.accept(b[j]); // vanilla per-entity logic, bit-for-bit
                }
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

    private static void parallelTick(EntityTickList list, Consumer<Entity> c, int w) {
        // Phase 1+2 (serial, S7-158c ZERO-ALLOC steady state): ONE vanilla-
        // protocol forEach pass fills persistent per-slot arrays in place
        // (grow-on-overflow, rare); the old design allocated a 150k-ref
        // snapshot list + W bucket lists + a consumer array EVERY tick.
        // Publication = volatile writes + the GO barrier happens-before
        // edge (same guarantees as v1's tasks[]/consumers[] publication).
        int[] len0 = bucketLen;
        Entity[][] arr0 = bucketArr;
        final int[] len;
        final Entity[][] arr;
        if (arr0.length < w) {
            arr = new Entity[w][];
            len = new int[w];
            // S7-158c fix: freshly allocated slot arrays start EMPTY (a bare
            // `new Entity[w][]` leaves null slots — the fill loop dereferences
            // b.length on the first element and would NPE; caught by the
            // RegionThreadsHarness parallel child).
            for (int i = 0; i < w; i++) arr[i] = new Entity[0];
        } else {
            arr = arr0;
            len = len0;
        }
        for (int i = 0; i < w; i++) len[i] = 0;
        pumpLevel = null; // S7-174: reset BEFORE fill; captured at first entity
        list.forEach(e -> {
            if (pumpLevel == null) pumpLevel = e.level(); // S7-174 capture
            int s = bucketOf(e, w);
            Entity[] b = arr[s];
            if (len[s] == b.length) {
                b = java.util.Arrays.copyOf(b, Math.max(16, len[s] * 2));
                arr[s] = b;
            }
            b[len[s]++] = e;
        });
        consumer = c;
        bucketArr = arr;
        bucketLen = len;

        // Phase 3 (parallel): main = slot 0, helpers = 1..w-1.
        // tickBucket(0) joins DONE in its own finally — the ONLY join point
        // (a second awaitDone here would deadlock: helpers loop back to GO
        // after the single DONE release).
        ensureHelpers(w);
        phaseActive = true;
        try {
            GO.await(); // releases helpers (they park here between ticks)
            if (MAIN_OFFLOAD) {
                // S7-172: main orchestrates only — the join moves out of
                // tickBucket(0); helpers own ALL slots (0..w-1). Same
                // join semantics: last DONE arrival releases everyone.
                //
                // S7-174 v2 (REGION_STEAL="3", PUMP_OFFLOAD): instead of
                // parking for the whole phase, main pumps the main-only
                // mid-tick queue (moonrise$midTickTasks) — restoring the
                // legacy interleave in which main pumped per-entity from
                // bucket 0 (S7-157b keep-side). Root-cause of the v1
                // regression (s7197 run 35499022752, -19..-21% dual bar):
                // with main parked at DONE nobody pumps during phase-3,
                // mid-tick work serializes after the phase, per-tick bucket
                // cost inflates (~+30% bucket-samples per tick at -23%
                // ticks/window vs s7196 legacy). Pump cadence (bounded
                // spin + 500ns yield) affects latency only, not queue
                // order — the pollTask queue stays main-thread-exclusive
                // (WORKER_FLAG gate untouched). Loop escape: workerError,
                // phase clear, or all w helpers arrived at DONE (getNumber
                // Waiting() >= WORKERS; helpers always reach DONE via
                // tickBucket's finally, so no missed-wakeup deadlock); a
                // broken barrier yields 0 waiting + workerError from the
                // helpers' finally -> same unwind as v1.
                if (PUMP_OFFLOAD) {
                    Level lvl = pumpLevel;
                    try {
                        while (lvl != null && phaseActive && workerError == null
                                && DONE.getNumberWaiting() < WORKERS) {
                            if (lvl instanceof ChunkSystemLevel p) {
                                p.moonrise$midTickTasks(); // vanilla dispatch
                            }
                            java.util.concurrent.locks.LockSupport
                                    .parkNanos(500L);
                        }
                    } catch (Throwable t3) {
                        if (workerError == null) workerError = t3;
                    }
                }
                try {
                    DONE.await();
                } catch (Throwable t2) {
                    if (workerError == null) workerError = t2;
                }
            } else {
                tickBucket(0);
            }
        } catch (Throwable t) {
            if (workerError == null) workerError = t;
            phaseActive = false;
        }
        phaseActive = false;

        // Post-join retention hygiene: null the stale tail refs beyond each
        // slot's used length so discarded entities are not kept alive by the
        // reused arrays (zero-alloc pass, main-only, workers parked).
        for (int i = 0; i < w; i++) {
            Entity[] b = arr[i];
            for (int j = len[i], n2 = b.length; j < n2; j++) b[j] = null;
        }

        // Phase 4 (serial): drain deferred EntityCallbacks mutations in FIFO order.
        Mut m;
        while ((m = PENDING.poll()) != null) {
            if (m.add) list.add(m.entity); else list.remove(m.entity);
        }

        // Phase 4b (serial, S7-168): replay deferred sendBlockUpdated
        // navigate-passes (STEAL v2 defect-fix) — main-only, after join.
        drainDeferredBlockUpdates();

        Throwable err = workerError;
        if (err != null) {
            if (err instanceof RuntimeException) throw (RuntimeException) err;
            if (err instanceof Error) throw (Error) err;
            throw new RuntimeException(err);
        }
    }

    private static void tickBucket(int slot) {
        try {
            Entity[] bucket = bucketArr[slot];
            Consumer<Entity> c = consumer;
            // ROUND-397 items_sweep2 (TASK-397-E): ONE sort-based sweep-line
            // batch-merge pass per bucket per tick replaces the per-entity
            // mergeWithNeighbours broadphase queries (which the retargeted
            // ItemEntity.tick -> ItemsSweepOps.tickMerge site suppresses for
            // the phase). Dormant-invisible: ItemsSweepOps.SWEEP=false makes
            // this a static-read no-op (the class is always co-defined with
            // this bridge). Failure inside is self-quarantined there.
            ItemsSweepOps.sweepBucket(bucket, bucketLen[slot]);
            for (int i = 0, n = bucketLen[slot]; i < n; i++) {
                c.accept(bucket[i]); // vanilla per-entity logic, bit-for-bit
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
            // S7-172 MAIN_OFFLOAD: helpers cover slots 0..w-1 (helper i
            // ticks slot i-1); legacy: helpers cover 1..w-1, main slot 0.
            int hi = MAIN_OFFLOAD ? w : w - 1;
            for (int i = 1; i <= hi; i++) {
                final int slot = MAIN_OFFLOAD ? i - 1 : i;
                Thread t = new TickThread(() -> {
                    WORKER_FLAG.set(Boolean.TRUE); // S7-157b: never pump mid-tick
                    while (true) {
                        try {
                            GO.await();
                            if (STEAL) {
                                stealChunks(); // S7-167: pull shared chunks
                            } else {
                                tickBucket(slot);
                            }
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
        ensureNavMobsGuarded(entity); // S7-170: swap long before any worker phase races
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
        ensureNavMobsGuarded(entity); // S7-170: idempotent, covers late-first-tick levels
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

    /**
     * S7-157b mid-tick gate — retarget of the ONLY worker-reachable pump
     * site (Level.guardEntityTick: invokevirtual moonrise$midTickTasks, the
     * per-entity call that crashed the live leg through
     * ServerChunkCache$MainThreadExecutor.pollTask). Workers suppress; the
     * main thread reproduces the exact vanilla virtual dispatch (the
     * most-derived override — ServerLevel.moonrise$midTickTasks on the
     * dedicated server, the Level stub elsewhere).
     */
    public static void midTickTasks(Level level) {
        if (WORKER_FLAG.get()) {
            return; // region worker: main-thread queue is off-limits
        }
        if (level instanceof ChunkSystemLevel patched) {
            patched.moonrise$midTickTasks(); // vanilla dispatch, bit-identical
        }
        // non-patched Level: the vanilla Level stub body was `return` — same
    }

    /** Harness/census probe: is the CURRENT thread a region worker? */
    public static boolean isWorker() {
        return WORKER_FLAG.get();
    }

    // ==================================================================
    // S7-170 NAV-MOBS-GUARD (RECON-22, TASK-348) — fastutil navigatingMobs
    // cross-thread structural race, LATENT in bank v3 (region_threads>=2,
    // bu_defer=0: workers reach the VANILLA sendBlockUpdated body which
    // iterates ServerLevel.navigatingMobs unprotected).
    //
    // javap inventory (contract-s7170-serverlevel.txt / -entitycallbacks.txt,
    // kernel jar s7178-recal): the ONLY sites touching the field are
    //   (1) ServerLevel.<init>        : putfield new ObjectOpenHashSet
    //   (2) ServerLevel.sendBlockUpdated : getfield + iterate (Paper wraps the
    //       collection loop in a ConcurrentModificationException catch and
    //       retries — but the fastutil "wrapped is null" NPE is NOT a CME, it
    //       escapes: s7186 class-A crash; s7176/s7180 class-B fatal path)
    //   (3) ServerLevel$EntityCallbacks.onTrackingStart : Set.add(Mob)
    //   (4) ServerLevel$EntityCallbacks.onTrackingEnd    : Set.remove(Mob)
    // All four go through the java.util.Set INTERFACE (invokeinterface), so a
    // single-point substitution of the FIELD VALUE routes every reader and
    // every writer through the guard — zero retargets, zero classfile.rs /
    // cargo changes, one delivered class (RegionTickOps already is one).
    //
    // Guard design (bit-identical iteration order): the wrapper clones the
    // vanilla ObjectOpenHashSet (fastutil clone() copies the internal hash
    // table -> iteration order EXACTLY matches the unguarded vanilla set);
    // mutations serialize on the wrapper monitor; iterator() takes the
    // snapshot under the same monitor and iterates the frozen copy (a
    // consistent view — the vanilla concurrent iteration was undefined
    // behaviour; Paper's CME-retry becomes a dead path, preregistered).
    //
    // Parity boundary (preregistered, RECON-22 §4): recomputePath invocation
    // order is preserved for identical set contents (clone order); the
    // accepted interleave class is the window between the snapshot and the
    // recomputePath loop — the same cross-thread observation class the owner
    // bar already accepts for region ticks (S7-155). recomputePath touches
    // no navigatingMobs state (javap), so the loop cannot corrupt the set.
    //
    // Activation: idempotent, MAIN-thread-only, from the already-delivered
    // onTickingStart/onTickingEnd retargets (fire on the first entity add /
    // remove flow, long before the first worker phase can race; workers never
    // run the swap). WORKERS<=1 compiles to a no-op (javac folds the static
    // final constant) — vanilla passthrough bit-identical.
    // ==================================================================

    private static final sun.misc.Unsafe S7170_UNSAFE;
    private static final long NAVIGATING_MOBS_OFFSET;
    static {
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            S7170_UNSAFE = (sun.misc.Unsafe) uf.get(null);
            java.lang.reflect.Field f = Class.forName("net.minecraft.server.level.ServerLevel")
                    .getDeclaredField("navigatingMobs");
            NAVIGATING_MOBS_OFFSET = S7170_UNSAFE.objectFieldOffset(f);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /**
     * S7-170: swap level.navigatingMobs to the guarded set, once per level.
     * Main-thread-only call sites (onTickingStart/onTickingEnd retargets run
     * main-direct outside a phase, main-drain at the barrier inside one).
     * Fail-closed: a non-ServerLevel level (harness stubs) is skipped.
     */
    public static void ensureNavMobsGuarded(Entity entity) {
        if (WORKERS <= 1) return; // vanilla passthrough, javac-folded when off
        if (!(entity.level() instanceof net.minecraft.server.level.ServerLevel level)) return;
        Object cur = S7170_UNSAFE.getObject(level, NAVIGATING_MOBS_OFFSET);
        if (cur instanceof GuardedNavigatingMobs) return; // fast path
        synchronized (RegionTickOps.class) {
            cur = S7170_UNSAFE.getObject(level, NAVIGATING_MOBS_OFFSET);
            if (cur instanceof GuardedNavigatingMobs) return;
            S7170_UNSAFE.putObject(level, NAVIGATING_MOBS_OFFSET,
                    new GuardedNavigatingMobs(cur));
            System.out.println("[S7-170] nav-mobs guarded: level=" + level.dimension().location()
                    + " seeded=" + ((GuardedNavigatingMobs) S7170_UNSAFE.getObject(level,
                        NAVIGATING_MOBS_OFFSET)).size());
        }
    }

    /**
     * S7-170 guarded view of ServerLevel.navigatingMobs. Every mutation and
     * every iteration serializes on the monitor; iteration order of the
     * snapshot is BIT-IDENTICAL to the vanilla fastutil set (table clone).
     */
    public static final class GuardedNavigatingMobs implements java.util.Set<Mob> {
        private final ObjectOpenHashSet<Mob> inner;

        GuardedNavigatingMobs(Object existing) {
            if (existing instanceof ObjectOpenHashSet<?> ff) {
                @SuppressWarnings("unchecked")
                ObjectOpenHashSet<Mob> src = (ObjectOpenHashSet<Mob>) ff;
                this.inner = src.clone(); // exact table copy -> exact vanilla order
            } else {
                // non-vanilla backing set: rehash copy (order = that set's iteration)
                @SuppressWarnings("unchecked")
                java.util.Collection<Mob> src = (java.util.Collection<Mob>) existing;
                this.inner = new ObjectOpenHashSet<>(src);
            }
        }

        /** Harness probe: elements visible through a frozen snapshot. */
        public int innerSize() { synchronized (this) { return inner.size(); } }

        @Override public boolean add(Mob m) { synchronized (this) { return inner.add(m); } }
        @Override public boolean remove(Object o) { synchronized (this) { return inner.remove(o); } }
        @Override public boolean contains(Object o) { synchronized (this) { return inner.contains(o); } }
        @Override public int size() { synchronized (this) { return inner.size(); } }
        @Override public boolean isEmpty() { synchronized (this) { return inner.isEmpty(); } }
        @Override public void clear() { synchronized (this) { inner.clear(); } }

        @Override public java.util.Iterator<Mob> iterator() {
            final ObjectOpenHashSet<Mob> snap;
            synchronized (this) { snap = inner.clone(); } // frozen, thread-private
            return snap.iterator();
        }

        @Override public Object[] toArray() { synchronized (this) { return inner.toArray(); } }
        @Override public <T> T[] toArray(T[] a) { synchronized (this) { return inner.toArray(a); } }
        @Override public boolean containsAll(java.util.Collection<?> c) {
            synchronized (this) { return inner.containsAll(c); }
        }
        @Override public boolean addAll(java.util.Collection<? extends Mob> c) {
            synchronized (this) { return inner.addAll(c); }
        }
        @Override public boolean removeAll(java.util.Collection<?> c) {
            synchronized (this) { return inner.removeAll(c); }
        }
        @Override public boolean retainAll(java.util.Collection<?> c) {
            synchronized (this) { return inner.retainAll(c); }
        }
        @Override public boolean equals(Object o) { synchronized (this) { return inner.equals(o); } }
        @Override public int hashCode() { synchronized (this) { return inner.hashCode(); } }
        @Override public String toString() { synchronized (this) { return inner.toString(); } }
        @Override public void forEach(java.util.function.Consumer<? super Mob> action) {
            final ObjectOpenHashSet<Mob> snap;
            synchronized (this) { snap = inner.clone(); }
            snap.forEach(action);
        }
        @Override public boolean removeIf(java.util.function.Predicate<? super Mob> filter) {
            synchronized (this) { return inner.removeIf(filter); }
        }
        @Override public java.util.Spliterator<Mob> spliterator() {
            final ObjectOpenHashSet<Mob> snap;
            synchronized (this) { snap = inner.clone(); }
            return snap.spliterator();
        }
    }
}
