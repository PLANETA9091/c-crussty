import net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise;
import net.minecraft.world.level.biome.PaperNativeClimateRTree;

import java.util.Arrays;
import java.util.Locale;

/**
 * P501 HANDLE-CHAIN PROBE — TASK-157 (agent-7625532f, 2026-09-10).
 *
 * Per TASK-156: the 7 CRASHed handle-consumer groups (FQ ClimateRTree
 * checksum/free/search, FQ PerlinNoise free/getValue) crashed only because
 * per-(fqcn,sig) group JVMs cannot pass a build→consumer handle. This probe
 * runs ONE JVM per FAMILY and chains build→sentinel-check→consume→free
 * in-process, using synthetic arguments only.
 *
 * PRE-REGISTERED verdict tree (CLAIMS TASK-157, before any data):
 *   H-PASS     build returns a non-sentinel handle → consumers measured
 *              on a REAL handle (first handle-lane numbers);
 *   H-SENTINEL build returns 0/-1-class sentinel or throws on synthetic
 *              inputs → consumers SKIPPED, handle lane CLOSED for synthetic
 *              probing (owner-ask for production context);
 *   H-CRASH    consumer aborts the JVM even with its own handle → documented
 *              as-is; family JVM isolation bounds the damage.
 *
 * Timing discipline = canonical Bench.java shape: batched do-while with
 * 8 calls/iteration against a ~120 ms budget, WARM=2 discarded batches,
 * ROUNDS=5 measured batches, median; single-call medians for destructive
 * ops (free) over an untimed pool. FIRST MEASUREMENTS — no baseline, no
 * wiring, no drift claims. INJECTS-ONLY: 0 boots, 0 product changes.
 *
 * Usage: java -Dp501.libs=... -cp classes HandleChainProbe perlin|rtree
 */
public final class HandleChainProbe {
    public static volatile long SINK;
    static final long BATCH_NS = 120_000_000;
    static final long WARM_NS = 120_000_000;
    static final int ROUNDS = 5;
    static final int WARM = 2;

    public static void main(String[] argv) {
        for (String p : System.getProperty("p501.libs", "").split(java.io.File.pathSeparator))
            if (!p.isBlank()) System.load(p);
        String family = argv[0];
        switch (family) {
            case "perlin" -> perlin();
            case "rtree" -> rtree();
            default -> throw new IllegalArgumentException("family " + family);
        }
        System.out.printf(Locale.ROOT, "SINK\t%s\t%d%n", family, SINK);
        System.out.flush();
    }

    // ------------------------------------------------------------- perlin --
    static void perlin() {
        byte[] b0 = new byte[16], b1 = new byte[16];
        for (int i = 0; i < 16; i++) { b0[i] = (byte) (i * 31 + 7); b1[i] = (byte) (i * 17 + 3); }
        double[] o0 = d8(1.0), o1 = d8(0.5), o2 = d8(0.25), o3 = d8(0.125);

        // build: sentinel check first (single call)
        long h;
        try {
            h = PaperNativePerlinNoise.nativeBuildHandle(b0, b1, o0, o1, o2, o3, 0.5, 1.0);
        } catch (Throwable t) {
            row("perlin", "build", -1, "BUILD-EXC " + t.getClass().getSimpleName());
            skipped("perlin");
            return;
        }
        if (h == 0L) {
            row("perlin", "build", -1, "BUILD-SENTINEL h=0");
            skipped("perlin");
            return;
        }
        // hot loops over the SAME handle
        batchRow("perlin", "getValue", () -> SINK += PaperNativePerlinNoise.nativeGetValue(h, 0.5, 64.5, 0.5, 0.25, 0.75, false));
        batchRow("perlin", "getValueNoYScale", () -> SINK += (long) PaperNativePerlinNoise.nativeGetValueNoYScale(h, 0.5, 64.5, 0.5));

        // build/free cost: pool of 64, args VARY per index (anti-memoization),
        // each build timed once (median of 64 single calls), each handle freed
        // EXACTLY once (timed) — double-free impossible by construction
        long[] pool = new long[64];
        try {
            double[] bs = new double[pool.length];
            for (int i = 0; i < pool.length; i++) {
                b0[0] = (byte) (i * 7 + 1);
                long t0 = System.nanoTime();
                pool[i] = PaperNativePerlinNoise.nativeBuildHandle(b0, b1, o0, o1, o2, o3, 0.5, 1.0);
                bs[i] = System.nanoTime() - t0;
                SINK += pool[i];
            }
            row("perlin", "buildCost", medianOf(bs), "OK median-of-64 single builds (args vary per index)");
            double[] fs = new double[pool.length];
            for (int i = 0; i < pool.length; i++) {
                long t0 = System.nanoTime();
                PaperNativePerlinNoise.nativeFreeHandle(pool[i]);
                fs[i] = System.nanoTime() - t0;
            }
            row("perlin", "freeCost", medianOf(fs), "OK median-of-64 single frees (each handle once)");
            PaperNativePerlinNoise.nativeFreeHandle(h);
        } catch (Throwable t) {
            row("perlin", "buildCost", -1, "EXC " + t.getClass().getSimpleName());
        }
    }

    // -------------------------------------------------------------- rtree --
    static void rtree() {
        int n = 256;
        long[] keys = new long[n], vals = new long[n];
        for (int i = 0; i < n; i++) {
            keys[i] = (0x9E3779B97F4A7C15L * i) ^ (i * 0xC2B2AE3D27D4EB4FL);
            vals[i] = (i * 0x9E3779B1L) & 0x3FF;
        }
        long h;
        try {
            h = PaperNativeClimateRTree.nativeBuildTreeHandle(keys, vals);
        } catch (Throwable t) {
            row("rtree", "build", -1, "BUILD-EXC " + t.getClass().getSimpleName());
            skipped("rtree");
            return;
        }
        if (h == 0L) {
            row("rtree", "build", -1, "BUILD-SENTINEL h=0");
            skipped("rtree");
            return;
        }
        row("rtree", "build", single(() -> PaperNativeClimateRTree.nativeBuildTreeHandle(keys, vals)), "BUILD-OK h!=0");
        SINK += h;

        batchRow("rtree", "checksum", () -> SINK += PaperNativeClimateRTree.nativeChecksumTreeHandle(h));
        batchRow("rtree", "searchCurrentOnePacked",
                () -> SINK += PaperNativeClimateRTree.nativeSearchCurrentOnePacked(h, 0x101L, 0x202L, 0x303L, 0x404L, 0x505L, 0x606L, 0x707L, 8));
        batchRow("rtree", "searchBoundedOnePacked",
                () -> SINK += PaperNativeClimateRTree.nativeSearchBoundedOnePacked(h, 0x101L, 0x202L, 0x303L, 0x404L, 0x505L, 0x606L, 0x707L, 8));
        long[] q = new long[32]; int[] qi = new int[32]; long[] dst = new long[64];
        for (int i = 0; i < 32; i++) { q[i] = 0x9E3779B97F4A7C15L * i; qi[i] = i; }
        batchRow("rtree", "searchBoundedBatchPacked", () -> SINK += PaperNativeClimateRTree.nativeSearchBoundedBatchPacked(h, q, 32, qi, dst));

        // build/free cost: pool of 64, vals[0] VARY per index (anti-memoization),
        // each build timed once, each handle freed EXACTLY once (timed)
        long[] pool = new long[64];
        try {
            double[] bs = new double[pool.length];
            for (int i = 0; i < pool.length; i++) {
                vals[0] = (i * 0x9E3779B1L) & 0x3FF;
                long t0 = System.nanoTime();
                pool[i] = PaperNativeClimateRTree.nativeBuildTreeHandle(keys, vals);
                bs[i] = System.nanoTime() - t0;
                SINK += pool[i];
            }
            row("rtree", "buildCost", medianOf(bs), "OK median-of-64 single builds (vals vary per index)");
            double[] fs = new double[pool.length];
            for (int i = 0; i < pool.length; i++) {
                long t0 = System.nanoTime();
                PaperNativeClimateRTree.nativeFreeTreeHandle(pool[i]);
                fs[i] = System.nanoTime() - t0;
            }
            row("rtree", "freeCost", medianOf(fs), "OK median-of-64 single frees (each handle once)");
            PaperNativeClimateRTree.nativeFreeTreeHandle(h);
        } catch (Throwable t) {
            row("rtree", "buildCost", -1, "EXC " + t.getClass().getSimpleName());
        }
    }

    static void skipped(String family) {
        row(family, "consumers", -1, "SKIPPED-SENTINEL (build failed — handle lane closed for synthetic probing)");
    }

    // ------------------------------------------------- timing (canonical) --
    interface Op { long call(); }

    static void batchRow(String family, String op, Runnable r) {
        try {
            // warm
            for (int w = 0; w < WARM; w++) runBatch(r, WARM_NS);
            double[] s = new double[ROUNDS];
            for (int r5 = 0; r5 < ROUNDS; r5++) s[r5] = runBatch(r, BATCH_NS);
            Arrays.sort(s);
            row(family, op, s[s.length / 2], "OK batched (same handle)");
        } catch (Throwable t) {
            row(family, op, -1, "ERR " + t.getClass().getSimpleName());
        }
    }

    static double runBatch(Runnable r, long budgetNs) {
        long calls = 0, t0 = System.nanoTime(), el;
        do {
            for (int k = 0; k < 8; k++) r.run();
            calls += 8;
            el = System.nanoTime() - t0;
        } while (el < budgetNs);
        return (double) el / calls;
    }

    static double single(Op op) {
        try {
            long t0 = System.nanoTime();
            long v = op.call();
            long el = System.nanoTime() - t0;
            SINK += v;
            return el;
        } catch (Throwable t) {
            return -1;
        }
    }

    static double medianOf(double[] s) {
        double[] c = s.clone();
        Arrays.sort(c);
        return c[c.length / 2];
    }

    static void row(String family, String op, double ns, String status) {
        System.out.printf(Locale.ROOT, "RESULT\t%s\t%s\t%.1f\t%s%n", family, op, ns, status);
        System.out.flush();
        System.err.println("  " + family + "/" + op + " -> " + status
                + (ns < 0 ? "" : String.format(Locale.ROOT, " %.1f ns/op", ns)));
    }

    static double[] d8(double base) {
        double[] a = new double[8];
        for (int i = 0; i < a.length; i++) a[i] = base / (i + 1);
        return a;
    }
}
