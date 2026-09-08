package net.minecraft.world.level.levelgen.synth;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.util.RandomSource;

/**
 * G-BODY (TASK-71) driver — two separate JVM runs (TASK-58 A/B discipline,
 * no mid-run retransform noise in the timing arm):
 *
 *   mode "java":    agent NOT attached — pristine JVM, P500 arm J only.
 *   mode "patched": agent attached — baseline outputs computed on the
 *                   pristine body, retransform triggered, bit-parity
 *                   checked (same deterministic coordinate stream, same
 *                   JVM), handle-build cost reported, then P500 arms
 *                   P (patched dispatch) and N (direct native, handle
 *                   pre-warmed — isolates the dispatch overhead).
 *
 * P500 hygiene: 120 ms batches, median-of-5, fwd+rev, min-of-medians,
 * SINK, shared coordinate pool (seed 777 — identical to TASK-67/69/70 for
 * cross-report comparability).
 */
public final class Step0BodyBench {
    private Step0BodyBench() {}

    private static double SINK = 0.0;

    private interface Op { long unit(int i); }

    public static void main(String[] args) throws Exception {
        final String mode = args.length > 1 ? args[1] : "java";
        System.load(args[0]);
        System.out.println("lib loaded: " + args[0] + " mode=" + mode);

        final PerlinNoise cont = PerlinNoise.create(
            RandomSource.create(777L), -3,
            DoubleArrayList.wrap(new double[]{1, 1, 1, 1, 1, 1, 1, 1}));

        final int POOL = 8192;
        final double[] xs = new double[POOL], ys = new double[POOL], zs = new double[POOL];
        java.util.Random r = new java.util.Random(777L);
        for (int i = 0; i < POOL; i++) {
            xs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
            ys[i] = -64.0 + r.nextDouble() * 384.0;
            zs[i] = (r.nextDouble() * 2.0 - 1.0) * 1.0e5;
        }

        StringBuilder csv = new StringBuilder("mode,key,value\n");
        csv.append(mode).append(",mode,1\n");

        if (mode.equals("patched")) {
            // ---- baseline on pristine body (agent captured, not yet applied) ----
            final int N = 20000;
            final double[] A = new double[N];
            java.util.Random ra = new java.util.Random(4242L);
            for (int i = 0; i < N; i++) {
                int j = ra.nextInt(POOL);
                A[i] = cont.getValue(xs[j], ys[j], zs[j], 0.0, 0.0, false);
            }
            // ---- trigger the whole-body swap ----
            bodyagent.BodyAgent.retransformNow(PerlinNoise.class);
            if (!bodyagent.BodyAgent.applied()) {
                System.out.println("VERDICT: G-BODY FAIL (retransform did not apply)");
                return;
            }
            System.out.println("[body] retransform applied; pristine=" + bodyagent.BodyAgent.pristineLen() + " bytes captured");
            // ---- parity: same deterministic stream through the patched body ----
            int mism = 0;
            java.util.Random rb = new java.util.Random(4242L);
            for (int i = 0; i < N; i++) {
                int j = rb.nextInt(POOL);
                double b = cont.getValue(xs[j], ys[j], zs[j], 0.0, 0.0, false);
                if (Double.doubleToRawLongBits(A[i]) != Double.doubleToRawLongBits(b)) mism++;
            }
            System.out.println("PARITY pre-vs-post retransform: " + mism + "/" + N + " mismatches");
            final boolean mapHasCont = BodyDispatch.handleOf(cont) != 0L;
            System.out.println("[canary] patched body really executed (cont handle built during parity): " + mapHasCont);
            csv.append(mode).append(",parity,").append(mism).append('\n');
            if (mism != 0 || !mapHasCont) {
                System.out.println("--BEGIN_CSV--\n" + csv + "--END_CSV--");
                System.out.println("VERDICT: G-BODY FAIL (" + (mism != 0 ? "parity" : "patch-not-executed") + ")");
                return;
            }

            // ---- handle build cost (one-time, per instance) ----
            PerlinNoise fresh = PerlinNoise.create(
                RandomSource.create(31337L), -3,
                DoubleArrayList.wrap(new double[]{1, 1, 1, 1, 1, 1, 1, 1}));
            double w = BodyDispatch.getValue(fresh, 0.5, 0.5, 0.5, 0.0, 0.0, false);
            SINK += w;
            long buildNs = BodyDispatch.lastBuildNs();
            System.out.printf("handle build (1 instance, lazy first call): %d ns%n", buildNs);
            csv.append(mode).append(",handle_build_ns,").append(buildNs).append('\n');
            // repeat for spread
            for (int k = 0; k < 3; k++) {
                PerlinNoise f2 = PerlinNoise.create(
                    RandomSource.create(31337L + k), -3,
                    DoubleArrayList.wrap(new double[]{1, 1, 1, 1, 1, 1, 1, 1}));
                SINK += BodyDispatch.getValue(f2, 0.5, 0.5, 0.5, 0.0, 0.0, false);
                csv.append(mode).append(",handle_build_ns").append(k).append(',').append(BodyDispatch.lastBuildNs()).append('\n');
                System.out.printf("  build rep%d: %d ns%n", k, BodyDispatch.lastBuildNs());
            }

            // ---- P500 arms: P (patched dispatch) and N (direct native) ----
            final long h = BodyDispatch.handleOf(cont);
            if (h == 0L) throw new IllegalStateException("cont handle missing after parity phase");
            java.util.List<String> tn = new java.util.ArrayList<>();
            java.util.List<Op> ops = new java.util.ArrayList<>();
            tn.add("P_patched_getValue6"); ops.add((i) -> { SINK += cont.getValue(xs[i], ys[i], zs[i], 0.0, 0.0, false); return 1; });
            tn.add("N_direct_native");     ops.add((i) -> { SINK += PaperNativePerlinNoise.nativeGetValue(h, xs[i], ys[i], zs[i], 0.0, 0.0, false); return 1; });
            timing(tn, ops, POOL, csv, mode);
        } else {
            if (bodyagent.AgentProbe.agentLoaded()) {
                System.out.println("WARN: agent unexpectedly present in java arm");
            }
            java.util.List<String> tn = new java.util.ArrayList<>();
            java.util.List<Op> ops = new java.util.ArrayList<>();
            tn.add("J_pristine_getValue6"); ops.add((i) -> { SINK += cont.getValue(xs[i], ys[i], zs[i], 0.0, 0.0, false); return 1; });
            timing(tn, ops, POOL, csv, mode);
        }

        System.out.println("--BEGIN_CSV--");
        System.out.print(csv);
        System.out.println("--END_CSV--");
        System.out.println("sink=" + SINK);
    }

    private static void timing(java.util.List<String> names, java.util.List<Op> ops,
                               int pool, StringBuilder csv, String mode) {
        final long TARGET_NS = 120_000_000L;
        int n = names.size();
        double[][] fwd = new double[n][], rev = new double[n][];
        for (int a = 0; a < n; a++) fwd[a] = arm(names.get(a), ops.get(a), pool, TARGET_NS);
        for (int a = n - 1; a >= 0; a--) rev[a] = arm("[rev]" + names.get(a), ops.get(a), pool, TARGET_NS);
        System.out.println("== G-BODY timing (ns/call, min-of-medians) ==");
        for (int a = 0; a < n; a++) {
            double best = Math.min(median(fwd[a]), median(rev[a]));
            System.out.printf("%-24s %10.1f ns/call%n", names.get(a), best);
            csv.append(mode).append(',').append(names.get(a)).append(',').append(String.format("%.1f", best)).append('\n');
        }
    }

    private static double[] arm(String name, Op op, int pool, long targetNs) {
        double[] meds = new double[5];
        long wEnd = System.nanoTime() + 2_000_000_000L;
        long i = 0;
        while (System.nanoTime() < wEnd) { i = (i + op.unit((int) (i % pool))) % pool; }
        for (int rep = 0; rep < 5; rep++) {
            long opsDone = 0, iters = 0;
            long t0 = System.nanoTime();
            while (System.nanoTime() - t0 < targetNs) {
                opsDone += op.unit((int) (iters % pool));
                iters++;
            }
            long dt = System.nanoTime() - t0;
            meds[rep] = (double) dt / (double) opsDone;
            System.out.printf("  %-24s rep%d: %d ops in %.1f ms -> %.1f ns/op%n",
                name, rep, opsDone, dt / 1e6, meds[rep]);
        }
        return meds;
    }

    private static double median(double[] a) {
        double[] c = a.clone();
        java.util.Arrays.sort(c);
        return c[c.length / 2];
    }
}
