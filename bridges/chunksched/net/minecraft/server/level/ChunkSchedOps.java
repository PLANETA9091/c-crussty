package net.minecraft.server.level;

import ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable;
import net.minecraft.world.level.chunk.LevelChunk;
import ca.spottedleaf.moonrise.common.PlatformHooks;
import ca.spottedleaf.moonrise.common.util.CoordinateUtils;
import ca.spottedleaf.moonrise.common.util.TickThread;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemServerLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler;
import ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * CHUNK6-SCHED — ServerChunkCache scheduling mono-plane (TASK-456-C,
 * lever cmp456_chunkmono, law 8 chunk/worldgen axis; RESEARCH-456-C.md:
 * scheduling slice 4.6-5.2% wall = getChunkNow 0.6-0.8 self +
 * ConcurrentLong2ReferenceChainedHashTable.getNode 1.2 shared + JIT lambda
 * 0.8-1.0 + off-main scheduling tails).
 *
 * Body-redirects (redirect_method_body_to_static, BLOCKUPD/NAVPLANE
 * precedent), BOTH-or-none (composite fail-dominant):
 * <ul>
 *   <li>{@code ServerChunkCache.getChunkNow(int,int)} -> {@link #getNow}</li>
 *   <li>{@code ServerChunkCache.moonrise$setFullChunk(int,int,LevelChunk)}
 *       -> {@link #onSetFullChunk}</li>
 * </ul>
 *
 * Data plane:
 * <ul>
 *   <li><b>Java L1 shadow</b> — direct-mapped 16384-slot (key, chunk) table
 *       written ONLY from onSetFullChunk (the single fullChunks mutation
 *       point, javap-verified: put/remove nowhere else in the kernel).
 *       Ordering makes stale-positives impossible: shadow put AFTER table
 *       put (window = vanilla-equivalent slow fallback), shadow del BEFORE
 *       table del (window = pre-removal table state, vanilla-consistent).
 *       Reads are acquire, writes release (cross-thread visibility with the
 *       vanilla table's own happens-before).</li>
 *   <li><b>Rust L2 key-mirror</b> — chunk-granular JNI events (mirrorEvent,
 *       the parse/send precedent; ZERO per-entity JNI — law 6), drift
 *       counted rust-side; drift > 0 = BROKEN latch = fail-open to the
 *       bit-exact vanilla body below.</li>
 * </ul>
 *
 * Parity contract (law 4):
 * <ul>
 *   <li>empty lever flag -> hooks never registered, bytecode byte-identical
 *       vanilla (dormant-invisible);</li>
 *   <li>{@link #getNow} fallback = the javap-verbatim vanilla body (fullChunks
 *       get + hasCurrentlyLoadingChunk branch + TickThread branch + holder
 *       getCurrentlyLoadingChunk path);</li>
 *   <li>fast-path hit returns the object the table itself would return (the
 *       shadow is fed by the only mutation point, release/acquire ordered);
 *       fast-path miss NEVER invents a chunk — it falls through;</li>
 *   <li>any Throwable on the armed path -> BROKEN latch (fail-open, vanilla
 *       forever);</li>
 *   <li>moonrise$setFullChunk semantics replicated exactly (put on non-null,
 *       remove on null) BEFORE the shadow/mirror side channel.</li>
 * </ul>
 *
 * Delivery discipline: ZERO nested classes (kernel-loader define, colpush
 * NCDFE lesson); natives registered by rust (mirrorEvent, schedProbe); ARMED
 * flipped by rust arm() AFTER the retransform is live (inside_snap arm-order
 * lesson: no [ARMED .. patched) window).
 */
public final class ChunkSchedOps {

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    /** Flipped by rust arm() LAST — after both retargets are retransformed live. */
    static final AtomicBoolean ARMED = new AtomicBoolean(false);
    /** Rust flip (pre-selfTest): fast-path consulted at all. */
    static volatile boolean FAST = false;
    /** Fail-open latch: any Throwable or rust drift>0 disarms the fast-path forever. */
    static final AtomicBoolean BROKEN = new AtomicBoolean(false);

    static final AtomicLong STAT_HITS = new AtomicLong();
    static final AtomicLong STAT_FALLBACKS = new AtomicLong();
    static final AtomicLong STAT_EVENTS = new AtomicLong();
    static volatile boolean FIRST_HIT_LOGGED = false;

    static final long PROBE_MAGIC_EXPECT = 0x456C456CL;

    // ------------------------------------------------------------------
    // L1 SHADOW (direct-mapped; zero alloc; zero JNI on the hot path)
    // ------------------------------------------------------------------

    static final int SHIFT = 14;
    static final int SIZE = 1 << SHIFT;
    static final int MASK = SIZE - 1;
    static final long[] K = new long[SIZE];
    static final LevelChunk[] V = new LevelChunk[SIZE];
    static final VarHandle KH = MethodHandles.arrayElementVarHandle(long[].class);
    static final VarHandle VH = MethodHandles.arrayElementVarHandle(LevelChunk[].class);

    static int slot(long key) {
        int h = (int) (key ^ (key >>> 25));
        h *= 0x9E3779B9;
        return h & MASK;
    }

    static void shadowPut(long key, LevelChunk chunk) {
        int s = slot(key);
        VH.setRelease(V, s, chunk);
        KH.setRelease(K, s, key);
    }

    static void shadowDel(long key) {
        int s = slot(key);
        KH.setRelease(K, s, 0L);
        VH.setRelease(V, s, null);
    }

    static LevelChunk fastProbe(long key) {
        int s = slot(key);
        long k = (long) KH.getAcquire(K, s);
        if (k != key) {
            return null;
        }
        return (LevelChunk) VH.getAcquire(V, s);
    }

    // ------------------------------------------------------------------
    // RETARGETED SITE #2 — ServerChunkCache.moonrise$setFullChunk
    // (vanilla semantics FIRST, then the side channel — airtight ordering)
    // ------------------------------------------------------------------

    public static void onSetFullChunk(ServerChunkCache scs, int x, int z, LevelChunk chunk) {
        ConcurrentLong2ReferenceChainedHashTable<LevelChunk> t = scs.fullChunks;
        long key = CoordinateUtils.getChunkKey(x, z);
        if (chunk != null) {
            t.put(key, chunk);
        } else {
            t.remove(key);
        }
        if (FAST && !BROKEN.get()) {
            try {
                if (chunk != null) {
                    shadowPut(key, chunk);
                } else {
                    shadowDel(key);
                }
                if (mirrorEvent(key, chunk != null)) {
                    noteDrift(1L);
                }
                STAT_EVENTS.incrementAndGet();
            } catch (Throwable th) {
                BROKEN.set(true);
                LOG.info("cmp456_chunkmono: chunk-sched side-channel Throwable -> fail-open latch (vanilla table authoritative)");
            }
        }
    }

    // ------------------------------------------------------------------
    // RETARGETED SITE #1 — ServerChunkCache.getChunkNow
    // (javap-verbatim vanilla body as the fallback; fast-path above it)
    // ------------------------------------------------------------------

    public static LevelChunk getNow(ServerChunkCache scs, int x, int z) {
        if (FAST && !BROKEN.get() && ARMED.get()) {
            try {
                LevelChunk f = fastProbe(CoordinateUtils.getChunkKey(x, z));
                if (f != null && !PlatformHooks.get().hasCurrentlyLoadingChunk()) {
                    STAT_HITS.incrementAndGet();
                    if (!FIRST_HIT_LOGGED) {
                        FIRST_HIT_LOGGED = true;
                        LOG.info("cmp456_chunkmono: chunk-sched EFFECT armed (first getChunkNow fast-path hit; shadow serves the fullChunks probe)");
                    }
                    return f;
                }
                STAT_FALLBACKS.incrementAndGet();
            } catch (Throwable th) {
                BROKEN.set(true);
            }
        }
        return vanillaNow(scs, x, z);
    }

    /** javap-verbatim vanilla ServerChunkCache.getChunkNow body. */
    static LevelChunk vanillaNow(ServerChunkCache scs, int x, int z) {
        LevelChunk c = (LevelChunk) scs.fullChunks.get(CoordinateUtils.getChunkKey(x, z));
        if (!PlatformHooks.get().hasCurrentlyLoadingChunk()) {
            return c;
        }
        if (c == null || !TickThread.isTickThread()) {
            return c;
        }
        NewChunkHolder holder = ((ChunkSystemServerLevel) scs.getLevel())
                .moonrise$getChunkTaskScheduler().chunkHolderManager.getChunkHolder(x, z);
        if (holder == null) {
            return c;
        }
        return PlatformHooks.get().getCurrentlyLoadingChunk(holder.vanillaChunkHolder);
    }

    // ------------------------------------------------------------------
    // NATIVES (rust L2 key-mirror; chunk-granular events, law-6 legal)
    // ------------------------------------------------------------------

    /** Rust L2: u64 key set add/del + drift counting (dup-add / miss-del).
     *  Returns true when drift was detected (java fail-opens the fast-path). */
    private static native boolean mirrorEvent(long key, boolean add);

    /** Rust probe (RegisterNatives verification; returns 0x456C456C). */
    private static native long schedProbe();

    /** Rust -> java drift sync: >0 disarms the fast-path (fail-open). */
    static void noteDrift(long drift) {
        if (drift > 0) {
            BROKEN.set(true);
            LOG.info("cmp456_chunkmono: rust mirror drift=" + drift + " -> fail-open latch (fast-path off, vanilla table authoritative)");
        }
    }

    public static long hits() { return STAT_HITS.get(); }
    public static long fallbacks() { return STAT_FALLBACKS.get(); }
    public static long events() { return STAT_EVENTS.get(); }
    public static boolean broken() { return BROKEN.get(); }
    public static boolean armed() { return ARMED.get(); }

    public static void arm() {
        FAST = true;
        ARMED.set(true);
        LOG.info("cmp456_chunkmono: ARMED chunk-sched (getChunkNow + moonrise$setFullChunk retargets live; shadow " + SIZE
                + " slots; rust key-mirror live; fail-open drift latch)");
    }

    public static void disarm() {
        FAST = false;
        ARMED.set(false);
        LOG.info("cmp456_chunkmono: DISARMED (fail-closed; vanilla bodies live)");
    }

    /** Structural self-test (rust calls BEFORE arm; fail-closed on Throwable). */
    public static boolean selfTest() {
        try {
            if (schedProbe() != PROBE_MAGIC_EXPECT) {
                return false;
            }
            // shadow round-trip (synthetic keys, real table mechanics)
            long k0 = CoordinateUtils.getChunkKey(12345, -6789);
            long k1 = CoordinateUtils.getChunkKey(-1, 0);
            shadowDel(k0);
            if (fastProbe(k0) != null) {
                return false;
            }
            if (fastProbe(k1) != null) {
                return false;
            }
            mirrorEvent(k0, true);
            mirrorEvent(k0, false);
            return true;
        } catch (Throwable t) {
            return false;
        }
    }

    private ChunkSchedOps() {
    }
}
