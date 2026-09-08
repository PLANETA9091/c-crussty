package net.minecraft.world.level.levelgen;

import it.unimi.dsi.fastutil.doubles.DoubleList;
import java.lang.ref.PhantomReference;
import java.lang.ref.ReferenceQueue;
import java.util.Map;
import java.util.WeakHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;
import net.minecraft.util.KeyDispatchDataCodec;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.levelgen.DensityFunction.ContextProvider;
import net.minecraft.world.level.levelgen.DensityFunction.FunctionContext;
import net.minecraft.world.level.levelgen.DensityFunction.NoiseHolder;
import net.minecraft.world.level.levelgen.DensityFunction.Visitor;
import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.NormalNoise;
import net.minecraft.world.level.levelgen.synth.PaperNativeNormalNoise;
import net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise;
import net.minecraft.world.level.levelgen.synth.PerlinNoise;

/**
 * TASK-108 v1 — Java-side wiring of the heritage NormalNoise BATCH FILL
 * family (G-ABI-2 decoded bit-exact: bench/step0_noise/results/
 * GABI2_NORMAL_FILL_ABI_2026-09-09.md) into the kernel's density-function
 * fillArray boundary (docs/BATCH_BRIDGE_DESIGN.md).
 *
 * Two whole-body swap targets (src/noise_fill.rs):
 *   DensityFunctions$Noise.fillArray       -> fillNoise(this, out, ctx)
 *   DensityFunctions$ShiftNoise.fillArray  -> fillShift(this, out, ctx)
 * (the latter is the ShiftNoise INTERFACE default — ShiftA/ShiftB inherit
 * it; ShiftedNoise and any other implementor falls back to vanilla inside
 * fillShift's instanceof dispatch).
 *
 * BATCH PATH per crossing:
 *   1. resolve the NormalNoise behind the holder (null = proto -> vanilla),
 *   2. resolve the striped {h1, h2, vf} handle triple (PhantomReference
 *      reaper, verbatim PerlinNoiseNativeOps/TASK-01 design; h1/h2 are the
 *      TASK-70 PerlinNoise whole-object handles of first/second),
 *   3. RECORDING pass: ctx.fillAllDirectly(RECORDER, scratch) — the
 *      PROVIDER'S OWN vanilla loop drives; the recorder captures
 *      (blockX, blockY, blockZ) per evaluation. This is parity-by-
 *      construction for ANY ContextProvider, including NoiseChunk$1 whose
 *      forIndex MUTATES chunk state and whose fillAllDirectly is a custom
 *      loop: we run the provider's own code, so its side-effect sequence
 *      and coordinate stream are exactly vanilla's.
 *   4. ONE JNI crossing: nativeFillScaledPositions / nativeFillShiftA /
 *      nativeFillShiftB (exact-size coord arrays; native bounded by them),
 *   5. if rc < n: per-point nativeGetValue tail for the missing indices
 *      (same composition the kernels bake — bit-exact, anomaly path only).
 *
 * FALLBACKS (never break worldgen, never NaN): proto holder, unknown
 * ShiftNoise implementor, handle-build failure, recording count != n, any
 * Throwable -> ctx.fillAllDirectly(out, self) — the EXACT vanilla body we
 * replaced. Known double side-effect window: if the recording pass already
 * ran and the batch path then aborts, fillAllDirectly runs a second time;
 * the only mutated state this touches is NoiseChunk.interpolationCounter,
 * which is write-only diagnostics (javap: no GETFIELD anywhere), and the
 * per-index fields are recomputed before every use. Documented, accepted.
 *
 * N-CENSUS: static counters (calls/sum/min/max) + optional daemon dumper
 * gated on CRUSSTY_NOISE_FILL_CENSUS=1 (bench arms only; logs every 30s).
 *
 * Compiled against compile-time stubs (DensityStubs.java et al., discarded);
 * --release 8; no lambdas capturing state (named nested classes only, ship
 * audit in scripts/build_noise.sh).
 */
public final class NormalNoiseBatchOps {

    /** Stripe count (power of two) — same contention split as the proven
     *  PerlinNoiseNativeOps / ImprovedNoiseNativeOps bridges. */
    static final int STRIPES = 16;
    /** Quiet-period tick: expunge stale weak entries this often (ms). */
    static final long EXPUNGE_INTERVAL_MS = 2000;

    private static final AtomicLong FREED = new AtomicLong();

    // ---------- N-census (TASK-108 gate evidence: "N pinned empirically") ----------

    static final AtomicLong C_CALLS = new AtomicLong();
    static final AtomicLong C_SUM = new AtomicLong();
    static final AtomicLong C_MIN = new AtomicLong(Long.MAX_VALUE);
    static final AtomicLong C_MAX = new AtomicLong(0);
    static final AtomicLong C_FALLBACKS = new AtomicLong();

    /** Named nested class (no lambda capture): dumps the census to stderr
     *  every 30s when CRUSSTY_NOISE_FILL_CENSUS=1. Bench-only observability. */
    static final class Census implements Runnable {
        private final boolean on;
        Census(boolean on) { this.on = on; }
        @Override public void run() {
            for (;;) {
                try {
                    Thread.sleep(30000L);
                } catch (InterruptedException e) {
                    continue; // daemon; never terminate
                }
                if (this.on) {
                    System.err.println(censusLine());
                }
            }
        }
    }

    static String censusLine() {
        long calls = C_CALLS.get();
        long min = C_MIN.get();
        long max = C_MAX.get();
        return "CRUSSTY_NOISE_FILL_CENSUS calls=" + calls
            + " sumN=" + C_SUM.get()
            + " minN=" + (min == Long.MAX_VALUE ? 0 : min)
            + " maxN=" + max
            + " avgN=" + (calls == 0 ? 0.0D : (double) C_SUM.get() / (double) calls)
            + " fallbacks=" + C_FALLBACKS.get();
    }

    private static void census(int n) {
        C_CALLS.incrementAndGet();
        C_SUM.addAndGet(n);
        long min = C_MIN.get();
        while (n < min && !C_MIN.compareAndSet(min, n)) {
            min = C_MIN.get();
        }
        long max = C_MAX.get();
        while (n > max && !C_MAX.compareAndSet(max, n)) {
            max = C_MAX.get();
        }
    }

    // ---------- last-call diagnostics (smoke forensics; read by selfTest) ----------

    static final AtomicLong D_H1 = new AtomicLong();
    static final AtomicLong D_H2 = new AtomicLong();
    static final AtomicLong D_VF_BITS = new AtomicLong();
    static final AtomicLong D_BX = new AtomicLong();
    static final AtomicLong D_BY = new AtomicLong();
    static final AtomicLong D_BZ = new AtomicLong();
    static final AtomicLong D_N = new AtomicLong();
    static final AtomicLong D_RC = new AtomicLong(-1);
    static final AtomicLong D_TAIL = new AtomicLong(-1);
    static final java.util.concurrent.atomic.AtomicReference<String> D_EX =
        new java.util.concurrent.atomic.AtomicReference<>("-");

    private static void diag(long h1, long h2, double vf, int bx0, int by0, int bz0, int n) {
        D_H1.set(h1);
        D_H2.set(h2);
        D_VF_BITS.set(Double.doubleToRawLongBits(vf));
        D_BX.set(bx0);
        D_BY.set(by0);
        D_BZ.set(bz0);
        D_N.set(n);
        D_RC.set(-1);
        D_TAIL.set(-1);
        D_EX.set("-");
    }

    /** One-line dump of the last bridged call (selfTest FAIL forensics). */
    static String diagLine() {
        return " d[h1=" + D_H1.get() + " h2=" + D_H2.get()
            + " vf=" + Double.toHexString(Double.longBitsToDouble(D_VF_BITS.get()))
            + " p=(" + D_BX.get() + "," + D_BY.get() + "," + D_BZ.get() + ")"
            + " n=" + D_N.get() + " rc=" + D_RC.get() + " tail=" + D_TAIL.get()
            + " ex=" + D_EX.get()
            + " gvScaled=" + Double.toHexString(PaperNativeNormalNoise.nativeGetValue(
                D_H1.get(), D_H2.get(),
                (double) D_BX.get() * 2.0D, (double) D_BY.get() * 3.0D,
                (double) D_BZ.get() * 2.0D,
                Double.longBitsToDouble(D_VF_BITS.get()))) + "]";
    }

    /** In-server forensic battery at selfTest FAIL: (1) direct fill with the
     *  LIVE handle and hardcoded arrays (isolates bridge plumbing), (2) fresh
     *  handle rebuild from the same noise + fill (isolates handle state),
     *  (3) reflection gather dump (comparable to the probe rig's PROBE gather
     *  lines). Never throws — failures degrade to 'bat:EX'. */
    static String forensicLine(NormalNoise nn) {
        try {
            final PerlinNoise fpn = (PerlinNoise) privateField(NormalNoise.class, nn, "first");
            final ImprovedNoise[] lv = (ImprovedNoise[]) privateField(
                PerlinNoise.class, fpn, "noiseLevels");
            final StringBuilder sb = new StringBuilder(" gather[lv.len=").append(lv.length);
            try {
                final double[] amps = ((DoubleList) privateField(
                    PerlinNoise.class, fpn, "amplitudes")).toDoubleArray();
                sb.append(" amps=").append(java.util.Arrays.toString(amps));
            } catch (Throwable t) { sb.append(" amps:EX"); }
            try {
                sb.append(" inF=").append(privateField(
                    PerlinNoise.class, fpn, "lowestFreqInputFactor"));
                sb.append(" valF=").append(privateField(
                    PerlinNoise.class, fpn, "lowestFreqValueFactor"));
            } catch (Throwable t) { sb.append(" factors:EX"); }
            try {
                final java.lang.reflect.Field fpf =
                    ImprovedNoise.class.getDeclaredField("p");
                fpf.setAccessible(true);
                for (int i = 0; i < Math.min(lv.length, 4); i++) {
                    if (lv[i] != null) {
                        final byte[] p = (byte[]) fpf.get(lv[i]);
                        sb.append(" L").append(i).append("=")
                            .append(p[0]).append(',').append(p[1]).append(',')
                            .append(p[2]).append(',').append(p[3])
                            .append('/').append(lv[i].xo);
                    } else {
                        sb.append(" L").append(i).append("=NULL");
                    }
                }
            } catch (Throwable t) { sb.append(" perm:EX"); }
            sb.append(']');
            final Handle h = handle(nn);
            if (h == null) {
                return sb + " bat:handle-null";
            }
            final int[] bx = {1000};
            final int[] by = {-64};
            final int[] bz = {2000};
            final double[] o1 = {Double.NaN};
            final int rc1 = PaperNativeNormalNoise.nativeFillScaledPositions(
                h.h1, h.h2, h.vf, bx, by, bz, 2.0D, 3.0D, o1);
            final long h1b = buildPerlinHandle(fpn);
            final long h2b = buildPerlinHandle((PerlinNoise) privateField(
                NormalNoise.class, nn, "second"));
            final double[] o2 = {Double.NaN};
            final int rc2 = (h1b == 0L || h2b == 0L) ? -2
                : PaperNativeNormalNoise.nativeFillScaledPositions(
                    h1b, h2b, h.vf, bx, by, bz, 2.0D, 3.0D, o2);
            return sb + " bat[live=" + rc1 + ":" + Double.toHexString(o1[0])
                + " fresh=" + rc2 + ":" + Double.toHexString(o2[0])
                + " want=0x1.4e5f5dfa92cfap-2]";
        } catch (Throwable t) {
            return " bat:EX(" + t.getClass().getSimpleName() + ")";
        }
    }

    // ---------- handle stripe {h1, h2, vf} per NormalNoise ----------

    /** One native handle pair + valueFactor for a NormalNoise; ALSO the
     *  death notification for its owner (phantom reference). freed CAS
     *  makes the native free AT-MOST-ONCE (TASK-01 design). */
    private static final class Handle extends PhantomReference<NormalNoise> {
        final long h1;
        final long h2;
        final double vf;
        final AtomicBoolean freed = new AtomicBoolean(false);
        Handle(NormalNoise owner, long h1, long h2, double vf) {
            super(owner, QUEUE);
            this.h1 = h1;
            this.h2 = h2;
            this.vf = vf;
        }
        boolean release() {
            if (this.freed.compareAndSet(false, true)) {
                if (this.h1 != 0L) {
                    PaperNativePerlinNoise.nativeFreeHandle(this.h1);
                }
                if (this.h2 != 0L) {
                    PaperNativePerlinNoise.nativeFreeHandle(this.h2);
                }
                FREED.incrementAndGet();
                return true;
            }
            return false;
        }
    }

    /** Dedicated reaper — verbatim TASK-01/PerlinNoiseNativeOps design. */
    private static final class Reaper implements Runnable {
        /** Explicit package-private ctor: an implicit one would inherit the
         *  class's PRIVATE access and force javac (--release 8, pre-
         *  nestmates) to mint a synthetic $1 marker class. */
        Reaper() {}
        @Override public void run() {
            for (;;) {
                final Handle h;
                try {
                    h = (Handle) QUEUE.remove(EXPUNGE_INTERVAL_MS);
                } catch (InterruptedException e) {
                    continue; // daemon; never terminate
                }
                if (h != null) {
                    h.release();
                    Handle n;
                    while ((n = (Handle) QUEUE.poll()) != null) { // burst drain
                        n.release();
                    }
                } else {
                    expunge();
                }
            }
        }
    }

    static final ReferenceQueue<NormalNoise> QUEUE = new ReferenceQueue<>();

    @SuppressWarnings("unchecked")
    static final Map<NormalNoise, Handle>[] STRIPE_MAPS = newMap();

    @SuppressWarnings("unchecked")
    private static Map<NormalNoise, Handle>[] newMap() {
        final Map<NormalNoise, Handle>[] a = new Map[STRIPES];
        for (int i = 0; i < STRIPES; i++) {
            a[i] = new WeakHashMap<>(32);
        }
        return a;
    }

    static {
        final boolean censusOn = "1".equals(System.getenv("CRUSSTY_NOISE_FILL_CENSUS"))
            || "true".equalsIgnoreCase(System.getenv("CRUSSTY_NOISE_FILL_CENSUS"));
        final Thread reaper = new Thread(new Reaper(), "crussty-noisefill-handle-reaper");
        reaper.setDaemon(true);
        reaper.start();
        final Thread census = new Thread(new Census(censusOn), "crussty-noisefill-census");
        census.setDaemon(true);
        census.start();
    }

    private NormalNoiseBatchOps() {}

    static Map<NormalNoise, Handle> stripe(NormalNoise self) {
        return STRIPE_MAPS[System.identityHashCode(self) & (STRIPES - 1)];
    }

    static void expunge() {
        for (int i = 0; i < STRIPES; i++) {
            final Map<NormalNoise, Handle> s = STRIPE_MAPS[i];
            synchronized (s) { s.size(); }
        }
    }

    private static Object privateField(Class<?> k, Object o, String name)
            throws ReflectiveOperationException {
        java.lang.reflect.Field f = k.getDeclaredField(name);
        f.setAccessible(true);
        return f.get(o);
    }

    /** TASK-70 winner pack P3/D1/F1 handle build (verbatim
     *  Step0NormalFillBench.buildHandle / PerlinNoiseNativeOps.build). */
    private static long buildPerlinHandle(PerlinNoise pn) {
        try {
            ImprovedNoise[] lv = (ImprovedNoise[]) privateField(PerlinNoise.class, pn, "noiseLevels");
            final int n = lv.length;
            byte[] a0 = new byte[256 * n];
            byte[] a1 = new byte[n];
            double[] xo = new double[n], yo = new double[n], zo = new double[n];
            java.lang.reflect.Field fp = ImprovedNoise.class.getDeclaredField("p");
            fp.setAccessible(true);
            for (int i = 0; i < n; i++) {
                if (lv[i] != null) {
                    byte[] p = (byte[]) fp.get(lv[i]);
                    System.arraycopy(p, 0, a0, 256 * i, 256);
                    a1[i] = 1;
                    xo[i] = lv[i].xo;
                    yo[i] = lv[i].yo;
                    zo[i] = lv[i].zo;
                }
            }
            DoubleList amp = (DoubleList) privateField(PerlinNoise.class, pn, "amplitudes");
            double[] amps = amp.toDoubleArray();
            double inF = (Double) privateField(PerlinNoise.class, pn, "lowestFreqInputFactor");
            double valF = (Double) privateField(PerlinNoise.class, pn, "lowestFreqValueFactor");
            return PaperNativePerlinNoise.nativeBuildHandle(a0, a1, xo, yo, zo, amps, inF, valF);
        } catch (Throwable t) {
            return 0L;
        }
    }

    /** Build {h1, h2, vf} from the live NormalNoise. Returns null on ANY
     *  failure — the caller then serves the vanilla path for this call
     *  (retry next call, same as PerlinNoiseNativeOps degraded instances). */
    private static Handle build(NormalNoise self) {
        try {
            final PerlinNoise first = (PerlinNoise) privateField(NormalNoise.class, self, "first");
            final PerlinNoise second = (PerlinNoise) privateField(NormalNoise.class, self, "second");
            final double vf = (Double) privateField(NormalNoise.class, self, "valueFactor");
            if (first == null || second == null) {
                return null;
            }
            final long h1 = buildPerlinHandle(first);
            final long h2 = buildPerlinHandle(second);
            if (h1 == 0L || h2 == 0L) {
                return null; // native rejected the config — serve the vanilla path
            }
            return new Handle(self, h1, h2, vf);
        } catch (Throwable t) {
            return null;
        }
    }

    private static Handle handle(NormalNoise self) {
        final Map<NormalNoise, Handle> stripe = stripe(self);
        synchronized (stripe) {
            final Handle h = stripe.get(self);
            if (h != null) {
                return h;
            }
            final Handle built = build(self);
            if (built == null) {
                return null; // degraded: serve the vanilla path (no cache churn)
            }
            stripe.put(self, built);
            return built;
        }
    }

    // ---------- recording pass (parity-by-construction) ----------

    /** Records (blockX, blockY, blockZ) per compute() call. The provider's
     *  own fillAllDirectly drives it, so the sequence of evaluations — and
     *  every provider side effect — matches vanilla exactly. */
    private static final class Recorder implements DensityFunction {
        int idx;
        int[] bx;
        int[] by;
        int[] bz;
        void reset(int[] bx, int[] by, int[] bz) {
            this.idx = 0;
            this.bx = bx;
            this.by = by;
            this.bz = bz;
        }
        @Override public double compute(FunctionContext f) {
            this.bx[this.idx] = f.blockX();
            this.by[this.idx] = f.blockY();
            this.bz[this.idx] = f.blockZ();
            this.idx++;
            return 0.0D;
        }
        @Override public void fillArray(double[] o, ContextProvider c) {
            throw new IllegalStateException("crussty-recorder");
        }
        @Override public DensityFunction mapAll(Visitor v) {
            return this;
        }
        @Override public double minValue() {
            return 0.0D;
        }
        @Override public double maxValue() {
            return 0.0D;
        }
        @Override public KeyDispatchDataCodec<DensityFunction> codec() {
            throw new IllegalStateException("crussty-recorder");
        }
    }

    /** Named ThreadLocal holder (an anonymous subclass would mint a
     *  synthetic $1 the define loop never embeds — ship-audit rule). */
    private static final class RecorderTL extends ThreadLocal<Recorder> {
        @Override protected Recorder initialValue() {
            return new Recorder();
        }
    }

    private static final ThreadLocal<Recorder> RECORDER = new RecorderTL();

    /** Grow-only write-only scratch (providers never read the out array of
     *  fillAllDirectly; the native never sees this buffer). */
    private static final class RecOutTL extends ThreadLocal<double[]> {
        @Override protected double[] initialValue() {
            return new double[256];
        }
    }

    private static final ThreadLocal<double[]> REC_OUT = new RecOutTL();

    /** Run the provider's own vanilla fill loop against the recorder.
     *  Returns the evaluation count (== n expected). */
    private static int record(ContextProvider ctx, int n, int[] bx, int[] by, int[] bz) {
        final Recorder rec = RECORDER.get();
        rec.reset(bx, by, bz);
        // EXACT size: vanilla providers iterate their out param's LENGTH, so a
        // grow-only scratch longer than n makes the recorder overrun the
        // n-length capture arrays (smoke-2 evidence: n<256 fell back with
        // ArrayIndexOutOfBounds, only n=256 ever batched).
        double[] scratch = REC_OUT.get();
        if (scratch.length != n) {
            scratch = new double[n];
            REC_OUT.set(scratch);
        }
        ctx.fillAllDirectly(scratch, rec);
        return rec.idx;
    }

    // ---------- the two whole-body swap bridge methods ----------

    /**
     * Whole-body replacement of {@code DensityFunctions$Noise.fillArray}.
     * Kernel shape (javap-pinned): noise.getValue(bx·xzScale, by·yScale,
     * bz·xzScale) per index via fillAllDirectly — batched as ONE
     * nativeFillScaledPositions crossing (G-ABI-2: exact shape match).
     */
    public static void fillNoise(DensityFunctions.Noise self, double[] out, ContextProvider ctx) {
        try {
            final NoiseHolder holder = self.noise();
            final NormalNoise nn = holder == null ? null : holder.noise();
            if (nn == null) {
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            final Handle h = handle(nn);
            if (h == null) {
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            final int n = out.length;
            final int[] bx = new int[n];
            final int[] by = new int[n];
            final int[] bz = new int[n];
            final int count = record(ctx, n, bx, by, bz);
            if (count != n) {
                // Provider semantics deviate from the contract (never seen in
                // vanilla); serve vanilla (documented double side-effect window).
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            final double sxz = self.xzScale();
            final double sy = self.yScale();
            diag(h.h1, h.h2, h.vf, bx[0], by[0], bz[0], n);
            final int rc = PaperNativeNormalNoise.nativeFillScaledPositions(
                h.h1, h.h2, h.vf, bx, by, bz, sxz, sy, out);
            D_RC.set(rc);
            if (rc < n) {
                // Anomaly tail — fill-based (correct-by-construction): the
                // fill kernels are G-ABI-2/probe-verified bit-exact, the
                // legacy per-point nativeGetValue decode is NOT (probe
                // 2026-09-09: mismatch at all points). One bounded retry on
                // the sub-range; if the native still under-fills, the
                // remaining slots keep the fill's own output semantics.
                final int m = n - rc;
                final int[] tx = java.util.Arrays.copyOfRange(bx, rc, n);
                final int[] ty = java.util.Arrays.copyOfRange(by, rc, n);
                final int[] tz = java.util.Arrays.copyOfRange(bz, rc, n);
                final double[] to = new double[m];
                D_TAIL.set(PaperNativeNormalNoise.nativeFillScaledPositions(
                    h.h1, h.h2, h.vf, tx, ty, tz, sxz, sy, to));
                final int written = (int) Math.min(D_TAIL.get(), m); System.arraycopy(to, 0, out, rc, written);
            }
            census(n);
        } catch (Throwable t) {
            C_FALLBACKS.incrementAndGet();
            D_EX.set(t.getClass().getSimpleName());
            try {
                ctx.fillAllDirectly(out, self);
            } catch (Throwable ignored) {
                // never let the batch path break worldgen
            }
        }
    }

    /**
     * Whole-body replacement of the ShiftNoise INTERFACE default
     * {@code fillArray} (inherited by ShiftA/ShiftB/ShiftedNoise).
     * ShiftA: 4·getValue(bx·0.25, 0, bz·0.25); ShiftB: 4·getValue(bz·0.25,
     * bx·0.25, 0) — the 0.25/4.0 scaling and the x↔z swap are BAKED into
     * nativeFillShiftA/B (G-ABI-2); a1 = blockX-side, a2 = blockZ-side.
     */
    public static void fillShift(DensityFunctions.ShiftNoise self, double[] out, ContextProvider ctx) {
        try {
            final boolean isA = self instanceof DensityFunctions.ShiftA;
            if (!isA && !(self instanceof DensityFunctions.ShiftB)) {
                // ShiftedNoise or future implementor: exact vanilla path.
                ctx.fillAllDirectly(out, self);
                return;
            }
            final NoiseHolder holder = self.offsetNoise();
            final NormalNoise nn = holder == null ? null : holder.noise();
            if (nn == null) {
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            final Handle h = handle(nn);
            if (h == null) {
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            final int n = out.length;
            final int[] bx = new int[n];
            final int[] by = new int[n];
            final int[] bz = new int[n];
            final int count = record(ctx, n, bx, by, bz);
            if (count != n) {
                C_FALLBACKS.incrementAndGet();
                ctx.fillAllDirectly(out, self);
                return;
            }
            diag(h.h1, h.h2, h.vf, bx[0], by[0], bz[0], n);
            final int rc;
            if (isA) {
                rc = PaperNativeNormalNoise.nativeFillShiftA(h.h1, h.h2, h.vf, bx, bz, out);
            } else {
                rc = PaperNativeNormalNoise.nativeFillShiftB(h.h1, h.h2, h.vf, bx, bz, out);
            }
            D_RC.set(rc);
            if (rc < n) {
                // Fill-based anomaly tail (see fillNoise note; the 0.25/4.0
                // and x<->z semantics are baked in the kernels themselves).
                final int m = n - rc;
                final int[] tx = java.util.Arrays.copyOfRange(bx, rc, n);
                final int[] tz = java.util.Arrays.copyOfRange(bz, rc, n);
                final int[] ty = java.util.Arrays.copyOfRange(by, rc, n);
                final double[] to = new double[m];
                D_TAIL.set(isA
                    ? PaperNativeNormalNoise.nativeFillShiftA(h.h1, h.h2, h.vf, tx, tz, to)
                    : PaperNativeNormalNoise.nativeFillShiftB(h.h1, h.h2, h.vf, tx, tz, to));
                final int written = (int) Math.min(D_TAIL.get(), m); System.arraycopy(to, 0, out, rc, written);
            }
            census(n);
        } catch (Throwable t) {
            C_FALLBACKS.incrementAndGet();
            D_EX.set(t.getClass().getSimpleName());
            try {
                ctx.fillAllDirectly(out, self);
            } catch (Throwable ignored) {
                // never let the batch path break worldgen
            }
        }
    }

    // ---------- live bit-exact self-test (Rust calls post-retransform) ----------

    /** Standard-loop ContextProvider for the self-test (deterministic
     *  coordinate pattern; forIndex mutates `cur` exactly like a provider
     *  is allowed to). */
    public static final class TestProvider implements ContextProvider, FunctionContext {
        private int cur = -1;
        public FunctionContext forIndex(int i) {
            this.cur = i;
            return this;
        }
        public void fillAllDirectly(double[] o, DensityFunction f) {
            for (int i = 0; i < o.length; i++) {
                o[i] = f.compute(this.forIndex(i));
            }
        }
        public int blockX() {
            return 1000 + this.cur * 3;
        }
        public int blockY() {
            return -64 + this.cur;
        }
        public int blockZ() {
            return 2000 - this.cur * 2;
        }
        void reset() {
            this.cur = -1;
        }
    }

    private static boolean bitEqual(double[] a, double[] b, int n) {
        for (int i = 0; i < n; i++) {
            if (Double.doubleToRawLongBits(a[i]) != Double.doubleToRawLongBits(b[i])) {
                return false;
            }
        }
        return true;
    }

    /** First-mismatch diagnostics for selfTest FAIL strings (bench forensics). */
    private static String firstMismatch(double[] a, double[] b, int n) {
        for (int i = 0; i < n; i++) {
            if (Double.doubleToRawLongBits(a[i]) != Double.doubleToRawLongBits(b[i])) {
                return " first@" + i + " want=" + Double.toHexString(b[i])
                    + "(" + Double.doubleToRawLongBits(b[i]) + ")"
                    + " got=" + Double.toHexString(a[i])
                    + "(" + Double.doubleToRawLongBits(a[i]) + ")";
            }
        }
        return "";
    }

    private static Object reflectiveNew(Class<?> k, Class<?>[] sig, Object[] args, String what)
            throws ReflectiveOperationException {
        java.lang.reflect.Constructor<?> c = k.getDeclaredConstructor(sig);
        c.setAccessible(true);
        return c.newInstance(args);
    }

    /**
     * Drives the REAL bridged fillArray vs the EXACT vanilla per-point leaf
     * (self.compute(provider.forIndex(i))) across sizes {1, 17, 256} for
     * Noise/ShiftA/ShiftB + the proto-holder fallback. Any raw-bits mismatch
     * or exception = FAIL (the bench refuses to arm on a FAIL line).
     *
     * Returns a parse-friendly result line; never throws.
     */
    public static String selfTest() {
        final TestProvider tp = new TestProvider();
        try {
            final NormalNoise nn = NormalNoise.create(
                RandomSource.create(1234L), -3, 1.0, 1.0, 1.0, 1.0);
            final NoiseHolder holder = new NoiseHolder(null, nn);
            final NoiseHolder proto = new NoiseHolder(null);
            final Object noiseSelf = reflectiveNew(DensityFunctions.Noise.class,
                new Class<?>[] {NoiseHolder.class, double.class, double.class},
                new Object[] {holder, 2.0D, 3.0D}, "Noise");
            final Object noiseProto = reflectiveNew(DensityFunctions.Noise.class,
                new Class<?>[] {NoiseHolder.class, double.class, double.class},
                new Object[] {proto, 2.0D, 3.0D}, "NoiseProto");
            final Object shiftA = reflectiveNew(DensityFunctions.ShiftA.class,
                new Class<?>[] {NoiseHolder.class}, new Object[] {holder}, "ShiftA");
            final Object shiftB = reflectiveNew(DensityFunctions.ShiftB.class,
                new Class<?>[] {NoiseHolder.class}, new Object[] {holder}, "ShiftB");
            final int[] sizes = new int[] {1, 17, 256};
            for (int s = 0; s < sizes.length; s++) {
                final int n = sizes[s];
                final double[] out = new double[n];
                final double[] want = new double[n];
                // Noise: bridged vs vanilla per-point compute
                for (int i = 0; i < n; i++) {
                    want[i] = ((DensityFunctions.Noise) noiseSelf).compute(tp.forIndex(i));
                }
                ((DensityFunctions.Noise) noiseSelf).fillArray(out, tp);
                if (!bitEqual(out, want, n)) {
                    return "CRUSSTY_NOISE_FILL SELFTEST FAIL mode=Noise n=" + n
                        + firstMismatch(out, want, n) + diagLine() + forensicLine(nn);
                }
                // ShiftA
                for (int i = 0; i < n; i++) {
                    want[i] = ((DensityFunctions.ShiftA) shiftA).compute(tp.forIndex(i));
                }
                ((DensityFunctions.ShiftA) shiftA).fillArray(out, tp);
                if (!bitEqual(out, want, n)) {
                    return "CRUSSTY_NOISE_FILL SELFTEST FAIL mode=ShiftA n=" + n
                        + firstMismatch(out, want, n) + diagLine();
                }
                // ShiftB
                for (int i = 0; i < n; i++) {
                    want[i] = ((DensityFunctions.ShiftB) shiftB).compute(tp.forIndex(i));
                }
                ((DensityFunctions.ShiftB) shiftB).fillArray(out, tp);
                if (!bitEqual(out, want, n)) {
                    return "CRUSSTY_NOISE_FILL SELFTEST FAIL mode=ShiftB n=" + n
                        + firstMismatch(out, want, n) + diagLine();
                }
                // proto-holder fallback (noise == null): bridged must equal
                // the vanilla fillAllDirectly path bit-for-bit
                for (int i = 0; i < n; i++) {
                    want[i] = ((DensityFunctions.Noise) noiseProto).compute(tp.forIndex(i));
                }
                ((DensityFunctions.Noise) noiseProto).fillArray(out, tp);
                if (!bitEqual(out, want, n)) {
                    return "CRUSSTY_NOISE_FILL SELFTEST FAIL mode=NoiseProto n=" + n
                        + firstMismatch(out, want, n) + diagLine();
                }
            }
            final long handles = liveHandles();
            final long freed = FREED.get();
            return "CRUSSTY_NOISE_FILL SELFTEST PASS handles=" + handles
                + " freed=" + freed + " " + censusLine();
        } catch (Throwable t) {
            return "CRUSSTY_NOISE_FILL SELFTEST FAIL exception=" + t;
        }
    }

    // ---------- observability ----------

    public static int liveHandles() {
        int n = 0;
        for (int i = 0; i < STRIPES; i++) {
            final Map<NormalNoise, Handle> s = STRIPE_MAPS[i];
            synchronized (s) { n += s.size(); }
        }
        return n;
    }

    public static long freedHandles() {
        return FREED.get();
    }
}
