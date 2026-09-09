package ca.spottedleaf.moonrise.common.misc;

import java.util.Arrays;
import java.util.Locale;

/**
 * TASK-20: area-map apply-loop micro-bench (headless, REAL 120ms batches).
 *
 * Paths measured (one JVM, one callback-receiver shape for all of them):
 *
 *   floor       - trivial static call: the per-call floor.
 *   vanilla     - UNPATCHED SingleUserAreaMap.update() from the tracked fixture
 *                 bytes (tests/fixtures/SingleUserAreaMap.class, real Paper
 *                 kernel class): per-cell enumeration + direct virtual
 *                 callback = the pre-TASK-11 baseline.
 *   run_real    - PATCHED bridge SingleUserAreaMapOps.run() end-to-end against
 *                 the REAL libpaper_native_jni.so: same-state -> fast path
 *                 (0 native calls), changed -> 1 native enumeration + apply
 *                 loop.
 *   native_only - direct PaperNativeAreaMap.nativeUpdateOpsBatch without the
 *                 apply loop: native enumeration cost alone; for SAME shape it
 *                 is exactly the JNI floor that the fast path skips.
 *   apply_loop  - the shipped apply loop over pre-filled op/key buffers,
 *                 isolated: 1 long + 1 byte + 1 virtual call per op.
 *   apply_noop  - the same loop with empty callback bodies: loop + decode +
 *                 dispatch floor.
 *
 * Shapes: SAME (0 ops), MOVE1 (1-chunk move, same d, 2*side ops), DISJOINT
 * (far move, 2*side^2 ops). Nominal grid 128/512/1024 means side ~= N, i.e.
 * view distance d = N/2 and side = 2d+1 = 129/513/1025.
 *
 * Methodology (mirrors bench/noise_ab): explicit warmup until ~100M callback
 * invocations (min 10 updates) so C2 compiles even the slowest cells;
 * decade-stepped calibration to a >=100ms batch (target 120ms); median-of-5
 * batches; forward+reverse cell order with min-of-medians; volatile SINK vs
 * DCE; parity BEFORE any timing (counts + signed key sums + element-wise
 * sorted native-vs-naive buffer compare). Parity failure = exit(3), never
 * publish a timing.
 */
public final class ApplyBench {
    private ApplyBench() {}

    static volatile long SINK = 0L;
    /** Opaque per-batch coordinate bit: prevents C2 from folding the 0-op
     *  SAME-shape paths into an empty loop (coords stay same-state at run
     *  time but are unknown at compile time). */
    static volatile long OPAQUE = System.nanoTime();

    static final int[] NOMINAL = {128, 512, 1024};
    static final int[] D = {64, 256, 512};
    static final String[] SHAPES = {"SAME", "MOVE1", "DISJOINT"};
    static final String[] PATHS = {"vanilla", "run_real", "native_only", "apply_loop", "apply_noop"};

    // paths (indices into PATHS plus the size-independent floor)
    static final int P_VANILLA = 0, P_RUN = 1, P_NATIVE = 2, P_APPLY = 3, P_APPLY_NOOP = 4, P_FLOOR = 5;

    // largest buffer need: disjoint move at d=512 -> 2 * 1025^2 ops
    static final int MAXOPS = 2 * 1025 * 1025;

    // ===== callback receivers ================================================

    /** Records every callback into a preallocated ring + signed sum (no allocation). */
    static final class RecMap extends SingleUserAreaMap<Object> {
        final long[] ring;
        int cursor = 0;
        long sum = 0;
        int adds = 0, removes = 0;
        RecMap(long[] ring) { super("p"); this.ring = ring; }
        @Override public void addCallback(Object param, int chunkX, int chunkZ) {
            final long k = ((long) chunkZ << 32) | (chunkX & 0xFFFFFFFFL);
            if (cursor < ring.length) ring[cursor++] = k;
            adds++; sum += k;
        }
        @Override public void removeCallback(Object param, int chunkX, int chunkZ) {
            final long k = ((long) chunkZ << 32) | (chunkX & 0xFFFFFFFFL);
            if (cursor < ring.length) ring[cursor++] = k;
            removes++; sum -= k;
        }
        void resetCounters() { cursor = 0; sum = 0L; adds = 0; removes = 0; }
    }

    /** Empty callback bodies: the loop + decode + dispatch floor. */
    static final class NoopMap extends SingleUserAreaMap<Object> {
        NoopMap() { super("p"); }
        @Override public void addCallback(Object param, int chunkX, int chunkZ) {}
        @Override public void removeCallback(Object param, int chunkX, int chunkZ) {}
    }

    static int floorFn(int v) { return v; }

    // ===== enumeration mirrors ================================================

    /** Naive fill: mirror of bench/areamap fake native (removes Old\\New, then adds New\\Old). */
    static int naiveFill(byte[] ops, long[] keys, int fromX, int fromZ, int oldD, int toX, int toZ, int newD) {
        int n = 0;
        for (int dx = -oldD; dx <= oldD; dx++) {
            for (int dz = -oldD; dz <= oldD; dz++) {
                final int x = fromX + dx, z = fromZ + dz;
                if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD) {
                    if (n < ops.length) { ops[n] = 1; keys[n] = ((long) z << 32) | (x & 0xFFFFFFFFL); n++; }
                }
            }
        }
        for (int dx = -newD; dx <= newD; dx++) {
            for (int dz = -newD; dz <= newD; dz++) {
                final int x = toX + dx, z = toZ + dz;
                if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD) {
                    if (n < ops.length) { ops[n] = 0; keys[n] = ((long) z << 32) | (x & 0xFFFFFFFFL); n++; }
                }
            }
        }
        return n;
    }

    /** Exact copy of the shipped SingleUserAreaMapOps.run() apply loop. */
    static void applyLoop(SingleUserAreaMap<Object> map, byte[] ops, long[] keys, int n) {
        for (int i = 0; i < n; i++) {
            long key = keys[i];
            int x = (int) key;          // chunk_as_long: x in the low 32 bits
            int z = (int) (key >>> 32); // z in the high 32 bits
            if (ops[i] == 0) {
                map.addCallback("p", x, z);    // AreaOp::Add
            } else {
                map.removeCallback("p", x, z); // AreaOp::Remove
            }
        }
    }

    /** apply_loop variant for the noop receiver: one add per op keeps the
     *  loads observable without changing the loop shape. */
    static long applyLoopNoop(NoopMap map, byte[] ops, long[] keys, int n) {
        long acc = 0L;
        for (int i = 0; i < n; i++) {
            long key = keys[i];
            int x = (int) key;
            int z = (int) (key >>> 32);
            if (ops[i] == 0) {
                map.addCallback("p", x, z);
            } else {
                map.removeCallback("p", x, z);
            }
            acc += key;
        }
        return acc;
    }

    // ===== measurement cell ===================================================

    static final class Ctx {
        final String shape;
        final int nominal, d, side, cells;
        final long ops;           // expected callback ops per single update
        final int ax, az, bx, bz; // ping-pong endpoints
        Ctx(String shape, int nominal, int d) {
            this.shape = shape;
            this.nominal = nominal;
            this.d = d;
            this.side = 2 * d + 1;
            this.cells = side * side;
            this.ops = shape.equals("SAME") ? 0L
                     : shape.equals("MOVE1") ? 2L * side
                     : 2L * cells;
            if (shape.equals("DISJOINT")) {
                this.ax = -(d + 2); this.az = -(d + 2);
                this.bx = (d + 2);  this.bz = (d + 2);
            } else if (shape.equals("MOVE1")) {
                this.ax = 0; this.az = 0;
                this.bx = 1; this.bz = 0;
            } else {
                this.ax = 0; this.az = 0;
                this.bx = 0; this.bz = 0;
            }
        }
        boolean hasApply() { return ops > 0L; }
    }

    // ===== shared state =======================================================

    static RecMap recV, recR, recA;
    static NoopMap noop;
    static byte[] preOps; static long[] preKeys;   // prefilled by naiveFill per cell
    static byte[] natOps; static long[] natKeys;   // native-written buffers (timed path + parity)
    static long[] scratchA, scratchB;   // parity sort buffers

    // ===== batch drivers ======================================================

    static void runBatch(int path, Ctx c, long K) {
        // SAME-shape coordinate: same-state at run time (from == to), opaque
        // to the JIT (volatile read once per batch) so the guard chain and the
        // vanilla field writes cannot be folded away.
        final int vx = 4096 + (int) ((OPAQUE >>> 33) & 1L);
        switch (path) {
            case P_FLOOR: {
                long acc = 0L;
                for (long i = 0L; i < K; i++) acc += floorFn(vx);
                SINK += acc;
                break;
            }
            case P_VANILLA: {
                long acc = 0L;
                if (c.shape.equals("SAME")) {
                    for (long i = 0L; i < K; i++) {
                        recV.resetCounters();
                        if (recV.update(vx, c.az, c.d)) acc++;
                        acc += recV.cursor;
                        recV.resetCounters();
                        if (recV.update(vx, c.az, c.d)) acc++;
                        acc += recV.cursor;
                    }
                } else {
                    for (long i = 0L; i < K; i++) {
                        recV.resetCounters();
                        if (recV.update(c.bx, c.bz, c.d)) acc++;
                        acc += recV.cursor;
                        recV.resetCounters();
                        if (recV.update(c.ax, c.az, c.d)) acc++;
                        acc += recV.cursor;
                    }
                }
                SINK += acc;
                break;
            }
            case P_RUN: {
                long acc = 0L;
                if (c.shape.equals("SAME")) {
                    for (long i = 0L; i < K; i++) {
                        SingleUserAreaMapOps.run(recR, vx, c.az, c.d, vx, c.az, c.d, "p");
                        acc += recR.cursor;
                        recR.resetCounters();
                        SingleUserAreaMapOps.run(recR, vx, c.az, c.d, vx, c.az, c.d, "p");
                        acc += recR.cursor;
                        recR.resetCounters();
                    }
                } else {
                    for (long i = 0L; i < K; i++) {
                        SingleUserAreaMapOps.run(recR, c.ax, c.az, c.d, c.bx, c.bz, c.d, "p");
                        acc += recR.cursor;
                        recR.resetCounters();
                        SingleUserAreaMapOps.run(recR, c.bx, c.bz, c.d, c.ax, c.az, c.d, "p");
                        acc += recR.cursor;
                        recR.resetCounters();
                    }
                }
                SINK += acc;
                break;
            }
            case P_NATIVE: {
                long acc = 0L;
                if (c.shape.equals("SAME")) {
                    for (long i = 0L; i < K; i++) {
                        acc += PaperNativeAreaMap.nativeUpdateOpsBatch(vx, c.az, c.d, vx, c.az, c.d, natOps, natKeys);
                        acc += PaperNativeAreaMap.nativeUpdateOpsBatch(vx, c.az, c.d, vx, c.az, c.d, natOps, natKeys);
                    }
                } else {
                    for (long i = 0L; i < K; i++) {
                        acc += PaperNativeAreaMap.nativeUpdateOpsBatch(c.ax, c.az, c.d, c.bx, c.bz, c.d, natOps, natKeys);
                        acc += PaperNativeAreaMap.nativeUpdateOpsBatch(c.bx, c.bz, c.d, c.ax, c.az, c.d, natOps, natKeys);
                    }
                }
                SINK += acc;
                break;
            }
            case P_APPLY: {
                final int n = (int) c.ops;
                long acc = 0L;
                for (long i = 0L; i < K; i++) {
                    applyLoop(recA, preOps, preKeys, n);
                    acc += recA.cursor;
                    recA.resetCounters();
                    applyLoop(recA, preOps, preKeys, n);
                    acc += recA.cursor;
                    recA.resetCounters();
                }
                SINK += acc;
                break;
            }
            case P_APPLY_NOOP: {
                final int n = (int) c.ops;
                long acc = 0L;
                for (long i = 0L; i < K; i++) {
                    acc += applyLoopNoop(noop, preOps, preKeys, n);
                    acc += applyLoopNoop(noop, preOps, preKeys, n);
                }
                SINK += acc;
                break;
            }
            default:
                throw new IllegalArgumentException("bad path " + path);
        }
    }

    static long timeBatch(int path, Ctx c, long K) {
        final long t0 = System.nanoTime();
        runBatch(path, c, K);
        return System.nanoTime() - t0;
    }

    /** Decade-stepped calibration: find K so one batch lands near 120ms. */
    static long calibrate(int path, Ctx c) {
        long K = 1L;
        long dt = timeBatch(path, c, K);
        while (dt < 20_000_000L && K < 300_000_000L) {
            K = Math.min(300_000_000L, K * 10L);
            dt = timeBatch(path, c, K);
        }
        final double scale = 120_000_000.0 / Math.max(1.0, (double) dt);
        return Math.max(1L, Math.min(300_000_000L, (long) Math.min(300_000_000.0, K * scale)));
    }

    /** Warmup: drive the exact batch code until ~100M callback invocations
     *  (or 100k updates for the 0-op shapes) so C2 compiles slow cells too. */
    static void warmup(int path, Ctx c) {
        if (path == P_FLOOR) {
            runBatch(path, c, 200_000L);
            return;
        }
        final long opsPerUpdate = c.ops;
        long updates = opsPerUpdate > 0L ? 100_000_000L / opsPerUpdate : 100_000L;
        updates = Math.max(10L, Math.min(100_000L, updates));
        long K = Math.max(1L, updates / 2L);
        // cap a single warmup call to 30M iterations (fast paths)
        K = Math.min(K, 30_000_000L);
        runBatch(path, c, K);
    }

    /** One calibrated median-of-5 measurement; returns ns per iteration. */
    static double measure(int path, Ctx c) {
        final long K = calibrate(path, c);
        final double[] med = new double[5];
        for (int rep = 0; rep < 5; rep++) {
            med[rep] = (double) timeBatch(path, c, K);
        }
        Arrays.sort(med);
        System.out.printf(Locale.ROOT, "BATCH %s %d %s K=%d median=%.1fms%n",
                c.shape, c.nominal, pathName(path), K, med[2] / 1e6);
        return med[2] / (double) K;
    }

    static String pathName(int path) {
        return path == P_FLOOR ? "floor" : PATHS[path];
    }

    // ===== parity =============================================================

    static void fail(String msg) {
        System.out.println("PARITY FAIL :: " + msg);
        System.out.println("APPLY_BENCH: PARITY FAILED — no timings published");
        System.exit(3);
    }

    static void parity(Ctx c) {
        final long expected = c.ops;

        // vanilla: init via add(), then ONE update A->B
        final RecMap v = recV;
        v.add(c.ax, c.az, c.d);
        v.resetCounters();
        final boolean ret = v.update(c.bx, c.bz, c.d);
        if (!ret) fail(c + ": vanilla update returned false");
        if (v.adds + v.removes != expected)
            fail(c + ": vanilla ops " + (v.adds + v.removes) + " != expected " + expected);

        // patched run(): ONE call A->B (stateless bridge)
        final RecMap r = recR;
        SingleUserAreaMapOps.run(r, c.ax, c.az, c.d, c.bx, c.bz, c.d, "p");
        if (r.adds + r.removes != expected)
            fail(c + ": run ops " + (r.adds + r.removes) + " != expected " + expected);
        if (r.adds != v.adds || r.removes != v.removes || r.sum != v.sum)
            fail(c + ": vanilla vs run mismatch (adds " + v.adds + "/" + r.adds
                    + " removes " + v.removes + "/" + r.removes + " sum " + v.sum + "/" + r.sum + ")");

        // isolated apply loop over naive-filled buffers
        final int n = naiveFill(preOps, preKeys, c.ax, c.az, c.d, c.bx, c.bz, c.d);
        if (n != expected)
            fail(c + ": naiveFill ops " + n + " != expected " + expected);
        final RecMap a = recA;
        applyLoop(a, preOps, preKeys, n);
        if (a.adds != r.adds || a.removes != r.removes || a.sum != r.sum)
            fail(c + ": run vs apply_loop mismatch (adds " + r.adds + "/" + a.adds
                    + " removes " + r.removes + "/" + a.removes + " sum " + r.sum + "/" + a.sum + ")");

        // native buffers must equal naive buffers element-wise (sorted)
        final int nn = PaperNativeAreaMap.nativeUpdateOpsBatch(c.ax, c.az, c.d, c.bx, c.bz, c.d, natOps, natKeys);
        if (nn != expected)
            fail(c + ": native ops " + nn + " != expected " + expected);
        System.arraycopy(natKeys, 0, scratchA, 0, nn);
        System.arraycopy(preKeys, 0, scratchB, 0, nn);
        Arrays.sort(scratchA, 0, nn);
        Arrays.sort(scratchB, 0, nn);
        for (int i = 0; i < nn; i++) {
            if (scratchA[i] != scratchB[i]) fail(c + ": native key @" + i + " != naive key");
            if (natOps[i] > 1) fail(c + ": native op byte @" + i + " invalid");
        }
        // signed-sum sanity across all three producers
        long natSum = 0L;
        for (int i = 0; i < nn; i++) natSum += natOps[i] == 0 ? natKeys[i] : -natKeys[i];
        if (natSum != r.sum) fail(c + ": native sum " + natSum + " != run sum " + r.sum);

        System.out.printf(Locale.ROOT, "PARITY %s %d OK (ops=%d adds=%d removes=%d)%n",
                c.shape, c.nominal, expected, r.adds, r.removes);
    }

    // ===== driver =============================================================

    public static void main(String[] args) {
        final String so = System.getProperty("crussty.native");
        if (so == null) {
            System.out.println("missing -Dcrussty.native — REAL mode required");
            System.exit(2);
        }
        System.load(so);

        System.out.println("APPLY_BENCH TASK-20 (real native: " + so + ")");
        System.out.println("jvm: " + System.getProperty("java.vm.version")
                + " heap: " + (Runtime.getRuntime().maxMemory() >> 20) + "M");

        recV = new RecMap(new long[MAXOPS]);
        recR = new RecMap(new long[MAXOPS]);
        recA = new RecMap(new long[MAXOPS]);
        noop = new NoopMap();
        preOps = new byte[MAXOPS];
        preKeys = new long[MAXOPS];
        natOps = new byte[MAXOPS];
        natKeys = new long[MAXOPS];
        scratchA = new long[MAXOPS];
        scratchB = new long[MAXOPS];

        final Ctx[] ctxs = new Ctx[NOMINAL.length * SHAPES.length];
        int ci = 0;
        for (int i = 0; i < NOMINAL.length; i++) {
            for (final String shape : SHAPES) {
                ctxs[ci++] = new Ctx(shape, NOMINAL[i], D[i]);
            }
        }

        for (final Ctx c : ctxs) parity(c);
        System.out.println("PARITY ALL OK — timing below");

        for (final Ctx c : ctxs) {
            System.out.printf(Locale.ROOT, "DIMS %s %d d=%d side=%d cells=%d ops_per_update=%d%n",
                    c.shape, c.nominal, c.d, c.side, c.cells, c.ops);
        }

        // warmup every cell (forward order)
        for (final Ctx c : ctxs) {
            for (int p = 0; p < PATHS.length; p++) {
                if ((p == P_APPLY || p == P_APPLY_NOOP) && !c.hasApply()) continue;
                warmup(p, c);
            }
        }
        warmup(P_FLOOR, ctxs[0]);

        // forward + reverse passes, min-of-medians per cell
        final double[][] bestIter = new double[PATHS.length + 1][ctxs.length];
        for (final double[] row : bestIter) Arrays.fill(row, Double.POSITIVE_INFINITY);

        for (int pass = 0; pass < 2; pass++) {
            final boolean fwd = pass == 0;
            System.out.println("PASS " + (fwd ? "forward" : "reverse"));
            // floor once per pass (size-independent)
            final int floorIdx = PATHS.length; // store floor under extra slot at ctx 0
            bestIter[floorIdx][0] = Math.min(bestIter[floorIdx][0], measure(P_FLOOR, ctxs[0]));
            for (int i = 0; i < ctxs.length; i++) {
                final int idx = fwd ? i : ctxs.length - 1 - i;
                final Ctx c = ctxs[idx];
                for (int pi = 0; pi < PATHS.length; pi++) {
                    final int p = fwd ? pi : PATHS.length - 1 - pi;
                    if ((p == P_APPLY || p == P_APPLY_NOOP) && !c.hasApply()) continue;
                    bestIter[p][idx] = Math.min(bestIter[p][idx], measure(p, c));
                }
            }
        }

        // final table (min-of-medians)
        System.out.println("TABLE per-update = ns per single update (ping-pong halves averaged); per-op = ns per applied callback op");
        for (int idx = 0; idx < ctxs.length; idx++) {
            final Ctx c = ctxs[idx];
            for (int p = 0; p < PATHS.length; p++) {
                if ((p == P_APPLY || p == P_APPLY_NOOP) && !c.hasApply()) continue;
                final double perIter = bestIter[p][idx];
                final double perUpdate = perIter / 2.0;
                final double perOp = c.ops > 0L ? perIter / (2.0 * c.ops) : 0.0;
                System.out.printf(Locale.ROOT, "TABLE %s %d %s %.1f %.2f %d%n",
                        c.shape, c.nominal, pathName(p), perUpdate, perOp, c.ops);
            }
        }
        final double floorNs = bestIter[PATHS.length][0];
        System.out.printf(Locale.ROOT, "TABLE FLOOR floor %.1f %.2f 0%n", floorNs, floorNs);
        System.out.println("APPLY_BENCH: DONE");
    }

}
