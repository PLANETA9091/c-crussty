package net.minecraft.world.level.levelgen.synth;

/**
 * G4 demonstrator batching helper (docs/G4_SITE_PATCH_DESIGN.md §5.1) — the
 * same-descriptor retarget target for the bridge call the improved_noise
 * byte hook emits inside {@code ImprovedNoise.noise(DDDDD)D}.
 *
 * <p>Contract (read with the design doc): the improved_noise patch replaces
 * the kernel method body with {@code invokestatic ImprovedNoiseNativeOps.noise}
 * — a fixed-shape call site in bytes we already produce. When the batch
 * rollout gate chain (G1 CRUSSTY_BATCH → G2 kernel-policy mask) arms the
 * site, the Rust side retargets that operand to THIS class's
 * {@code noise} — SAME descriptor
 * {@code (Lnet/minecraft/.../ImprovedNoise;[BDDDDDDDD)D}, so the
 * verifier-visible stack shape is unchanged (Variant R: zero branch fixups,
 * zero StackMapTable deltas).
 *
 * <p>HONEST FRAMING (G4 §1/§5.1): this is a gate-chain demonstrator, NOT a
 * performance claim. Noise paths are body-dominated (11.5 µs body vs a
 * ~115→35-90 ns transition; BATCH_ADOPTION_MATRIX rows 15/16 = LOW) and NO
 * noise batch kernel exists in the closed lib — so the flush leg exercises
 * the dispatcher machinery with a ZERO-OP {@code run()} through the real
 * bridge (ABI gate + entry validation + early-return) and the sampling
 * itself ALWAYS replays the individual, bit-exact
 * {@code ImprovedNoiseNativeOps.noise} call. No gameplay value can change:
 * the samples are computed by the exact same native path as before, one
 * extra Java frame + a ThreadLocal increment per call (nanoseconds vs the
 * 11.5 µs body).
 *
 * <p>Degradation ladder (B.2.2, G4 §4.5): a negative {@code run()} return,
 * an {@code abiVersion()} mismatch, or ANY Throwable from the flush leg sets
 * a per-site flag degrading the site to single-call (plain delegation) for
 * the remainder of the boot — no retry storms, no exception ever escapes
 * into worldgen.
 *
 * <p>Compiled by scripts/build_noise.sh with {@code --release 8} and defined
 * INTO THE KERNEL LOADER by src/improved_noise.rs (4th embedded class, same
 * class-major guard as the other bridges). Dormant unless the retarget
 * lands: the retargeted operand is the ONLY reference to this class, so
 * with the gate off it is never initialized.
 */
public final class ImprovedNoiseBatchOps {

    /**
     * B.3 default auto-threshold T for this site — the compile-time mirror
     * of batch_api.rs DEFAULT_SITE_T = 16 (clamped 8..64). This site is not
     * the g42 kernel, so no per-kernel override applies. Drift is caught by
     * the arm marker (T= comes from the Rust side) — keep the two in sync.
     */
    static final int T = 16;

    /**
     * The abiVersion() word this build expects — the compile-time mirror of
     * batch_api.rs ABI_WORD = (TABLE_VERSION<<16)|KERNEL_COUNT = 196626, v3
     * (wire v3: refArgs plane + 18 kernels, S7-14). A mismatch degrades the
     * site at the first flush (B.2.2 bucket).
     */
    static final int EXPECTED_ABI = 196626;

    /** Per-site degradation flag (B.2.2): once set, single-call for the boot. */
    private static volatile boolean degraded = false;

    /** Zero-op dispatch planes (immutable, shared — run() reads lengths only
     *  for an empty op list). EMPTY_REFS is the wire-v3 reference plane:
     *  a zero-op batch owns 0 ref slots, and length-0 (not null) is the
     *  contract for old-style batches. */
    private static final int[] EMPTY_IDS = new int[0];
    private static final long[] EMPTY_LONGS = new long[0];
    private static final int[] EMPTY_INTS = new int[0];
    private static final Object[] EMPTY_REFS = new Object[0];

    /** Pending-op counter per thread (B.2.1: accumulation lives at the
     *  site, per-thread — the design's ThreadLocal shape). withInitial (Java
     *  8 API) keeps the class file count at exactly 4: an anonymous subclass
     *  would mint a synthetic ImprovedNoiseBatchOps$1.class that the runtime
     *  define loop (4 named embeds) would never define — NoClassDefFoundError
     *  on first use. A lambda compiles to an invokedynamic call site, not a
     *  class file. */
    private static final ThreadLocal<int[]> PENDING = ThreadLocal.withInitial(() -> new int[1]);

    /** Flush counter — e2e/verify observability only (never logged hot). */
    private static volatile long flushes = 0L;

    /** Self-test diagnostic: the last flush outcome (0 = clean). */
    private static volatile int lastFlushStatus = 0;

    private ImprovedNoiseBatchOps() {}

    /**
     * Retarget target — SAME descriptor as
     * {@code ImprovedNoiseNativeOps.noise}. Accumulates the pending count,
     * flush-checks at T, and ALWAYS answers with the individual, bit-exact
     * bridge call (the demonstrator's ground state — see the class doc).
     */
    public static double noise(
        ImprovedNoise self, byte[] p, double xo, double yo, double zo,
        double x, double y, double z, double yScale, double yMax
    ) {
        if (!degraded) {
            final int[] pend = PENDING.get();
            if (++pend[0] >= T) {
                pend[0] = 0;
                if (!flush()) {
                    degraded = true; // single-call for the remainder of the boot
                }
            }
        }
        return ImprovedNoiseNativeOps.noise(self, p, xo, yo, zo, x, y, z, yScale, yMax);
    }

    /**
     * The flush leg (G4 §4.4, demonstrator form): one real dispatcher
     * round-trip through the live bridge — abiVersion() gate, then a
     * zero-op run() (executes no kernel: kernelIds.length == 0 hits the
     * dispatcher's validated early-return). Returns false iff the site must
     * degrade (B.2.2). Swallows EVERY Throwable: the batch bridge can be
     * absent (batch init failed is non-fatal by design) and a worldgen
     * thread must never see a foreign exception.
     */
    private static boolean flush() {
        try {
            if (crussty.batch.PaperNativeBatchDispatch.abiVersion() != EXPECTED_ABI) {
                lastFlushStatus = -1;
                return false;
            }
            final int ret = crussty.batch.PaperNativeBatchDispatch.run(
                EMPTY_IDS, EMPTY_LONGS, EMPTY_LONGS, EMPTY_INTS, EMPTY_LONGS, EMPTY_INTS,
                EMPTY_REFS);
            flushes++;
            lastFlushStatus = ret;
            return ret == 0;
        } catch (Throwable t) {
            lastFlushStatus = -2;
            return false;
        }
    }

    /**
     * Self-test hook (design §6 risk 6: the e2e self-test pattern must
     * extend to the batching helper before any on-mode boot). Drives the
     * flush machinery once and returns EXPECTED_ABI on success, a negative
     * diagnostic on failure. Not on any hot path; also used to prove the
     * helper is live in the kernel loader after define.
     */
    public static int selfTestFlush() {
        if (degraded) {
            return -100;
        }
        return flush() ? EXPECTED_ABI : -101;
    }

    // ---------- observability (e2e / verify; not on the hot path) ----------

    /** Flushes performed so far (monotonic). */
    public static long flushes() {
        return flushes;
    }

    /** Last flush status: 0 clean, run()'s return value, or a diagnostic. */
    public static int lastFlushStatus() {
        return lastFlushStatus;
    }

    /** Whether the site has degraded to single-call for this boot (B.2.2). */
    public static boolean isDegraded() {
        return degraded;
    }
}
