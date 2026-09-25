package net.minecraft.server.level;

import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicIntegerArray;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicLongArray;
import java.util.concurrent.atomic.AtomicReferenceArray;

/**
 * ID-H05 papaya-lockfree shard-readers — Java sidecar stub (round-459-h05).
 *
 * <p>STRICT DORMANT: {@link #ARMED} is {@code false} and every entry point
 * is fail-closed — callers MUST fall back to the vanilla locked broadphase
 * path ({@code EntityLookup}/SWMR read path) whenever any method returns
 * {@code null}/{@code false}. This class performs NO behavioral change on
 * the live bank until a future wiring task flips the ARM marker (boot
 * marker {@code papaya_shard_reads ARMED}; gate G1: fallbacks &lt; 1%).</p>
 *
 * <p><b>NCDFE-канон (T1 gate: NCDFE = 0)</b> — mandatory contract:</p>
 * <ul>
 *   <li>EARLY-define: this class MUST be {@code define_class}'d from the
 *       bootstrap loader in the early arm hook, BEFORE the first broadphase
 *       query (precedent: EntityGoalQueryOps @ MobPushOps.pushables:467;
 *       commits d73758a3 chunkmono / 5ecd841a poi / 9d71b461 eqsnap2).</li>
 *   <li>{@code <clinit>} touches ONLY JDK types (j.u.c atomics + arrays) —
 *       never any {@code net.minecraft.*} class — so an early define can
 *       never trigger {@link NoClassDefFoundError}. Mirror-drift needle:
 *       field {@link #PAPAYA_SHARD_MARK} must be visible to the javap
 *       flat==nested check in every run.</li>
 *   <li>Bytecode pin: {@code --release 8} (major 52), kernel JVM = Java 21;
 *       hard-fail above major 65 (repo-documented canon).</li>
 * </ul>
 *
 * <p><b>Protocol</b> (papaya-style epoch readers, RESEARCH-459-H05.md):
 * shard = 32x32-chunk region; each slot holds an immutable epoch-stamped
 * snapshot of EntitySection slice ids. Reader: {@link #pin()} (acquire load
 * + reader-counter increment, NO lock) &rarr; {@link #readShard} (acquire
 * load of snapshot + epoch check). Writer: {@link #publish} (fill-then-
 * publish, release semantics via volatile array store) + retire old
 * snapshot; reclamation only when readers are quiesced. Drift (snapshot
 * epoch newer than pinned epoch) &rarr; {@code null} &rarr; caller falls
 * back to vanilla (fail-closed, no partial results — parity contract:
 * emission order z,x,y-asc, storage-idx, bit-in-byte vs vanilla).</p>
 */
public final class PapayaShardReadOps {

    /** javap mirror-drift needle (NCDFE-канон). */
    public static final String PAPAYA_SHARD_MARK = "papaya_shard_reads/ID-H05/round-459-h05";

    /** 32x32-chunk regions (ChunkSlicesRegion precedent, RESEARCH-459-L11). */
    public static final int SHARDS = 32;

    /** STRICT DORMANT: never flipped by this scaffold. */
    public static volatile boolean ARMED = false;

    private static final AtomicLong EPOCH = new AtomicLong(0L);
    private static final AtomicInteger READERS = new AtomicInteger(0);
    /** Per-shard published snapshot (opaque slice-id payload, JDK-only). */
    private static final AtomicReferenceArray<long[]> SNAP = new AtomicReferenceArray<>(SHARDS);
    /** Epoch stamp of each published snapshot. */
    private static final AtomicLongArray SNAP_EPOCH = new AtomicLongArray(SHARDS);
    /** Per-shard in-flight readers (pinned before the snapshot load). */
    private static final AtomicIntegerArray SLOT_READERS = new AtomicIntegerArray(SHARDS);

    // Effect counters (gate G1 markers; all zero while dormant).
    private static final AtomicLong STAT_PINS = new AtomicLong(0L);
    private static final AtomicLong STAT_STABLE_READS = new AtomicLong(0L);
    private static final AtomicLong STAT_FALLBACKS = new AtomicLong(0L);
    private static final AtomicLong STAT_PUBLISHES = new AtomicLong(0L);
    private static final AtomicLong STAT_RECLAIMS = new AtomicLong(0L);

    private PapayaShardReadOps() {
    }

    /** Pinned epoch token (papaya Guard/pin analogue). */
    public static final class Pin {
        public final long epoch;

        Pin(long epoch) {
            this.epoch = epoch;
        }
    }

    /** Lock-free pin: 1 acquire load + 1 fetch-add; never blocks. */
    public static Pin pin() {
        READERS.incrementAndGet();
        STAT_PINS.incrementAndGet();
        return new Pin(EPOCH.get());
    }

    public static void unpin() {
        READERS.decrementAndGet();
    }

    /**
     * Lock-free reader. Returns the shard snapshot payload when the epoch
     * matches the pin (stable read), or {@code null} when the shard has no
     * snapshot or drift is detected — caller MUST use the vanilla locked
     * path then (fail-closed). Payload is a flat array of opaque Entity-
     * Section slice ids; consumers must treat it as immutable.
     */
    public static long[] readShard(final int shard, final Pin pin) {
        if (!ARMED || shard < 0 || shard >= SHARDS) {
            STAT_FALLBACKS.incrementAndGet();
            return null;
        }
        SLOT_READERS.incrementAndGet(shard);
        try {
            final long[] snap = SNAP.get(shard);
            if (snap == null) {
                STAT_FALLBACKS.incrementAndGet();
                return null;
            }
            if (SNAP_EPOCH.get(shard) > pin.epoch) {
                STAT_FALLBACKS.incrementAndGet();
                return null;
            }
            STAT_STABLE_READS.incrementAndGet();
            return snap;
        } finally {
            SLOT_READERS.decrementAndGet(shard);
        }
    }

    /**
     * Writer: fill-then-publish (relativistic order: content complete before
     * the volatile reference store). Retires the previous snapshot; the old
     * payload stays valid for readers that already loaded it (anchored).
     * Reclamation here is bookkeeping only — Java GC owns the memory.
     */
    public static void publish(final int shard, final long[] sections) {
        if (shard < 0 || shard >= SHARDS || sections == null) {
            throw new IllegalArgumentException("papaya_shard publish: bad shard/payload");
        }
        final long newEpoch = EPOCH.incrementAndGet();
        SNAP_EPOCH.set(shard, newEpoch);
        SNAP.set(shard, sections); // volatile store = publication edge
        STAT_PUBLISHES.incrementAndGet();
        STAT_RECLAIMS.incrementAndGet(); // prior payload dropped -> GC
    }

    public static long statPins() {
        return STAT_PINS.get();
    }

    public static long statStableReads() {
        return STAT_STABLE_READS.get();
    }

    public static long statFallbacks() {
        return STAT_FALLBACKS.get();
    }

    public static long statPublishes() {
        return STAT_PUBLISHES.get();
    }

    /**
     * Dormant self-test (gate G6: selfTest == true, threw == 0). JDK-only,
     * no kernel classes — safe to run from the early arm hook.
     */
    public static boolean selfTest() {
        try {
            final boolean saved = ARMED;
            ARMED = true;
            // DORMANT invariant: ARMED defaults false -> read falls back.
            ARMED = false;
            if (readShard(0, pin()) != null) {
                return false;
            }
            ARMED = true;
            publish(1, new long[] {4L, 5L, 6L});
            final Pin p = pin();
            final long[] snap = readShard(1, p);
            if (snap == null || snap.length != 3 || snap[2] != 6L) {
                return false;
            }
            // Drift: newer publish invalidates the stale pin -> null.
            publish(1, new long[] {7L});
            if (readShard(1, p) != null) {
                return false;
            }
            ARMED = saved;
            return true;
        } catch (final Throwable t) {
            return false;
        }
    }
}
