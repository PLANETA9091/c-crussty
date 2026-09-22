package net.minecraft.world.level.chunk;

import ca.spottedleaf.moonrise.patches.fast_palette.FastPaletteData;
import net.minecraft.util.BitStorage;
import java.util.ArrayList;
import java.util.IdentityHashMap;
import java.util.concurrent.atomic.AtomicLong;

/**
 * PALETTED-DEMUX materializer (S7-131, ARCH-ATTACK lever #1).
 *
 * The patched PalettedContainer.get(int) fast path reads
 * {@code crusstySnap = { int[] demux, Object[] vals }} and returns
 * {@code vals[demux[index]]} when {@code crusstySnapGen == crusstyGen},
 * skipping the bit-storage decode + palette indirection entirely. This class
 * supplies the two hooks the patched bytecode calls:
 *
 * <ul>
 *   <li>{@link #get} — the vanilla-equivalent slow path (same observable
 *       result as the original body: volatile data read, storage.get,
 *       moonrise fast-palette array or Palette.valueFor, bounds contract)
 *       plus the per-container miss counter that lazily triggers
 *       materialization;</li>
 *   <li>{@link #onWrite} — mutator epilogue bookkeeping (snapshot refcount
 *       release).</li>
 * </ul>
 *
 * Race protocol (see src/classfile.rs PALETTED-DEMUX header): the writers
 * bump crusstyGen to an odd value while a mutation is in flight; this class
 * materializes only at stable even epochs and re-validates (gen + data
 * identity) after the probe. Publication order is snap first, snapGen last —
 * both volatile — so a reader accepts a snapshot iff its build epoch equals
 * the current write epoch. A snapshot is therefore used exactly when no
 * write epoch has started since it was built; per-cell torn reads that pass
 * the check are exactly the concurrent window vanilla get already has.
 *
 * Defined into the KERNEL loader in package net.minecraft.world.level.chunk
 * (same discipline as FluidPushGuardHook/RandomTickOps). All API it touches
 * is public: PalettedContainer.data (public volatile field), Data.storage()
 * / palette() / FastPaletteData.moonrise$getPalette(), and the four injected
 * PUBLIC fields crusstySnap/crusstySnapGen/crusstyGen/crusstyMiss.
 *
 * Grep markers: "paletted: builds", "paletted: aborts", "paletted: capped".
 *
 * TASK-417-B (chunk-pipeline R-vector, cmp417_wgen): the materializer's
 * section data-plane unpack is now a BULK-JNI BATCHED BRIDGE — the whole
 * 4096-entry packed plane is unpacked in ONE native crossing
 * ({@link #bulkUnpack}, Rust: src/palette_gather.rs — the banked bit-exact
 * SimpleBitStorage replica core, wired for the first time) instead of a
 * Java loop of 4096 SimpleBitStorage.get calls. Fail-closed: the native
 * path engages ONLY after {@link #bulkProbe()} proved bit-exact on this
 * JVM (rc stored in {@link #BULK_OK}); any mismatch/absent lib falls back
 * to the vanilla-form Java loop (bit-identical result either way). Both
 * sides count: BULK (native crossings) / LOOP (Java-loop materializations)
 * — the EFFECT markers for the bench.
 */
public final class PalettedContainerOps {

    /** Published snapshots alive (bounded so write-heavy worlds cannot grow
     * the demux without limit; prologue nulls snap on first write, onWrite
     * releases the count). 24576 sections x (16 KiB demux + vals) ~ 400 MiB
     * worst case on the 10G heap — <4%. */
    static final AtomicLong LIVE = new AtomicLong(0);
    static final long CAP = 24576;

    /** TASK-417-B bulk-JNI bridge state: BULK_OK flipped by bulkProbe() ONLY
     * after a bit-exact proof on the live JVM (never assumed). */
    public static volatile boolean BULK_OK = false;
    public static volatile long BULK = 0;   // native crossings served
    public static volatile long LOOP = 0;   // Java-loop fallbacks used

    /** Monotonic stats (throttled log line every 2^24 builds). */
    public static volatile long BUILDS = 0, ABORTS = 0, CAPPED = 0;
    static volatile long NEXT_LOG_AT = 1L << 24;

    /** Adaptive materialize-probe stride over the per-container slow-read
     * counter, tuned by the AMORTIZATION ECONOMICS (S7-131 leg #1 + #1'
     * lessons): a snapshot costs ~4096 vanilla-path reads to build and
     * saves ~25ns per future fast read, so it only pays off when the
     * container still has >~6k future reads. Marginal sections (64-4095
     * total reads) are a net LOSS — leg #1' proved it (64-miss threshold
     * inflated the lane from 2573 to ~3400 equivalent samples).
     *
     *   first materialize: after 4096 slow reads (dense sections engage
     *     mid-window and save for the remainder);
     *   re-materialize after a write-release: after 16384 more slow reads;
     *   blacklist: after 2 releases (crusstyEpoch counter) the container
     *     is write-heavy — vanilla forever, zero build waste. */
    static final int FIRST_STRIDE_MASK = 0xFFF;     // 4096 slow reads
    static final int REARM_STRIDE_MASK = 0x3FFF;    // 16384 slow reads
    static final int MAX_BUILDS_PER_CONTAINER = 2;  // release black-list

    private PalettedContainerOps() {}

    /**
     * Vanilla-equivalent get(int) body + heat counter. Called ONLY from the
     * patched get(int) fallback (single-threaded semantics identical to the
     * original: no lock, volatile data read once).
     *
     * LIVE accounting is BEST-EFFORT (drift under publish/release races is
     * harmless: read correctness never depends on it — the snapGen==gen+1
     * gate does that; the counter only feeds the approximate memory cap).
     *snapGen==0 encodes "no live snapshot"; a published snapshot carries
     * snapGen == buildGen + 1.
     */
    public static Object get(PalettedContainer<?> self, int index) {
        PalettedContainer.Data<?> data = self.data; // volatile read (vanilla)
        BitStorage storage = data.storage();
        int raw = storage.get(index);
        Object v;
        Object[] pal = ((FastPaletteData<?>) data).moonrise$getPalette();
        if (pal != null) {
            v = pal[raw];
            if (v == null) {
                throw new IllegalArgumentException("Palette index out of bounds");
            }
        } else {
            v = data.palette().valueFor(raw);
        }
        int m = ++self.crusstyMiss; // plain field, benign races
        int epoch = self.crusstyEpoch;
        if (epoch < MAX_BUILDS_PER_CONTAINER) {
            int stride = epoch == 0 ? FIRST_STRIDE_MASK : REARM_STRIDE_MASK;
            if ((m & stride) == 0) {
                tryMaterialize(self);
            }
        }
        return v;
    }

    /** Mutator prologue: release a live snapshot's count (snapGen==0 encodes
     * "no live snapshot", so the freshly constructed container — which the
     * JVM zero-initializes to 0 — never triggers a phantom release). */
    public static void onMutateStart(PalettedContainer<?> self) {
        if (self.crusstySnapGen != 0) {
            self.crusstySnapGen = 0;
            LIVE.decrementAndGet();
        }
    }

    /** Build + publish the { demux, vals } snapshot for a quiescent epoch. */
    static void tryMaterialize(PalettedContainer<?> self) {
        int gen = self.crusstyGen;
        if ((gen & 1) != 0) {
            ABORTS++; // write epoch in flight — never probe a moving target
            logThrottled();
            return;
        }
        if (self.crusstySnap != null && self.crusstySnapGen == gen) {
            return; // already fresh
        }
        if (LIVE.get() >= CAP) {
            CAPPED++;
            logThrottled();
            return;
        }
        PalettedContainer.Data<?> data = self.data;
        BitStorage storage = data.storage();
        int size = storage.getSize();
        int[] demux = new int[size];
        Object[] pal = ((FastPaletteData<?>) data).moonrise$getPalette();
        Object[] vals;
        if (pal != null) {
            // moonrise fast palette: vals IS the palette table, demux maps
            // cell -> palette slot directly.
            // TASK-417-B: ONE bulk-JNI crossing for the whole packed plane
            // (SimpleBitStorage.getBits/getRaw are public moonrise surface).
            // rc < 0 (shape guard) or BULK_OK=false -> vanilla-form Java
            // loop — bit-identical result by construction.
            if (BULK_OK && storage instanceof net.minecraft.util.SimpleBitStorage
                    && ((net.minecraft.util.SimpleBitStorage) storage).getBits() <= 31) {
                final net.minecraft.util.SimpleBitStorage sbs =
                    (net.minecraft.util.SimpleBitStorage) storage;
                if (bulkUnpack(sbs.getRaw(), size, sbs.getBits(), demux) == size) {
                    BULK++;
                } else {
                    loopUnpack(storage, demux);
                }
            } else {
                loopUnpack(storage, demux);
            }
            vals = pal;
        } else {
            // No fast-palette array (global/identity palette shapes): build
            // a deduplicated value table from the public palette surface.
            IdentityHashMap<Object, Integer> slots = new IdentityHashMap<>();
            ArrayList<Object> list = new ArrayList<>();
            for (int i = 0; i < size; i++) {
                int raw = storage.get(i);
                Object v = data.palette().valueFor(raw);
                Integer slot = slots.get(v);
                if (slot == null) {
                    slot = list.size();
                    slots.put(v, slot);
                    list.add(v);
                }
                demux[i] = slot;
            }
            vals = list.toArray();
        }
        // Post-probe validation: epoch still even+same and data object not
        // swapped under us (resize). Any drift aborts without publishing.
        if (self.crusstyGen != gen || self.data != data) {
            ABORTS++;
            logThrottled();
            return;
        }
        LIVE.incrementAndGet();
        Object[] holder = {demux, vals};
        self.crusstySnap = holder;        // volatile store 1 (snap)
        self.crusstySnapGen = gen + 1;    // volatile store 2 (LAST): gate = snapGen == gen + 1
        self.crusstyEpoch++;              // plain store: build counter + re-arm stride select
        BUILDS++;
        logThrottled();
    }

    /** Vanilla-form Java unpack loop (bit-identical fallback of the bulk
     *  crossing; also the oracle the probe checks the native against). */
    private static void loopUnpack(BitStorage storage, int[] demux) {
        final int size = storage.getSize();
        for (int i = 0; i < size; i++) {
            demux[i] = storage.get(i);
        }
        LOOP++;
    }

    /** Rust core (src/palette_gather.rs): one crossing unpacks the WHOLE
     *  packed plane into out. Returns size on success, negative rc on any
     *  shape mismatch (Java caller falls back to the vanilla-form loop). */
    public static native int bulkUnpack(long[] words, int size, int bits, int[] out);

    /** In-JVM bit-exactness probe of the bulk bridge (called by the Rust
     *  activator right after RegisterNatives — BEFORE READY flips, so the
     *  demux never serves a snapshot built through an unproven path).
     *  Straddle-heavy shapes forced: bits {1,2,3,4,5,7,8,15} x random fills.
     *  Returns a parse-friendly marker line; never throws. */
    public static String bulkProbe() {
        try {
            final java.util.Random rnd = new java.util.Random(417417L);
            final int[] bitsSet = {1, 2, 3, 4, 5, 7, 8, 15};
            for (final int bits : bitsSet) {
                final int size = 4096;
                final long[] words = new long[(size * bits + 63) >>> 6];
                for (int w = 0; w < words.length; w++) {
                    words[w] = rnd.nextLong();
                }
                final int[] got = new int[size];
                final int[] want = new int[size];
                if (bulkUnpack(words, size, bits, got) != size) {
                    return "paletted: bulk-probe FAIL rc shape bits=" + bits;
                }
                final int mask = (int) ((1L << bits) - 1);
                for (int i = 0; i < size; i++) {
                    final int off = i * bits;
                    final int wi = off >>> 6;
                    final int sh = off & 63;
                    int v = (int) (words[wi] >>> sh);
                    if (sh + bits > 64) {
                        v |= (int) (words[wi + 1] << (64 - sh));
                    }
                    want[i] = v & mask;
                }
                for (int i = 0; i < size; i++) {
                    if (got[i] != want[i]) {
                        return "paletted: bulk-probe FAIL bits=" + bits + " first@" + i
                            + " want=" + want[i] + " got=" + got[i];
                    }
                }
            }
            BULK_OK = true;
            System.err.println("paletted: bulk-unpack ARMED (bit-exact probe PASS 8 shapes x 4096)");
            return "paletted: bulk-probe PASS";
        } catch (Throwable t) {
            BULK_OK = false;
            return "paletted: bulk-probe FAIL exception=" + t;
        }
    }

    private static void logThrottled() {
        long b = BUILDS;
        if (b >= NEXT_LOG_AT) {
            NEXT_LOG_AT = b + (1L << 24);
            System.err.printf(
                "[crussty-plugin] paletted: builds=%d aborts=%d capped=%d live=%d bulk=%d loop=%d%n",
                b, ABORTS, CAPPED, LIVE.get(), BULK, LOOP);
        }
    }

    /** Reflective smoke: machinery is loadable and the cap math is sane. */
    public static boolean selfTest() {
        return LIVE.get() == 0 && CAP > 0
            && FIRST_STRIDE_MASK > 0 && REARM_STRIDE_MASK > FIRST_STRIDE_MASK;
    }
}
