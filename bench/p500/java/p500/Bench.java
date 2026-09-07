package p500;

import java.io.*;
import java.nio.file.*;
import java.util.*;

/**
 * P500 bench driver — one group per JVM: args <gid>. TSV on stdout.
 *
 * Fairness protocol (learned the hard way, see results/):
 *  - args are REBUILT (fresh arrays) before every method measurement: some
 *    kernels mutate their inputs (e.g. *RemoveIfSummary), so sharing arrays
 *    across methods would hand the first-measured kernel more work;
 *  - every method is measured twice — a forward pass and a reverse pass —
 *    and the LOWER of the two medians is reported (min-of-two kills the
 *    residual order bias: forward favours early methods, reverse favours
 *    late ones);
 *  - per-method strategy ladder: if a kernel rejects the default shape
 *    (n = N), it is retried with n = 16 / 1 / null-objects before ERR;
 *  - time-bounded batches (~120ms), median of 5, DCE-proof sink.
 */
public final class Bench {
    public static volatile long SINK;
    public static int N = Integer.getInteger("p500.n", 256);

    static String TMP;
    /** Shared temp dir with a few files — used for kernels taking a path argument. */
    public static synchronized String tmpDir() {
        if (TMP == null) {
            try {
                Path d = Files.createTempDirectory("p500-scan");
                for (int k = 0; k < 3; k++) Files.writeString(d.resolve("file" + k + ".jar"), "x");
                Files.createDirectories(d.resolve("sub"));
                Files.writeString(d.resolve("sub").resolve("inner.jar"), "y");
                TMP = d.toString();
            } catch (IOException e) { TMP = "."; }
        }
        return TMP;
    }

    public static void main(String[] argv) throws Exception {
        for (String p : System.getProperty("p500.libs", "").split(File.pathSeparator))
            if (!p.isBlank()) System.load(p);
        int gid = Integer.parseInt(argv[0]);
        Group g;
        try {
            g = (Group) Class.forName("G" + gid).getDeclaredConstructor().newInstance();
        } catch (ClassNotFoundException e) {
            System.out.println("SKIP\t" + gid + "\t?\t?\tno group class");
            return;
        }
        System.err.println("group " + gid + ": " + g.fqcn() + g.sig() + " -> " + Arrays.toString(g.methods()));
        benchGroup(gid, g);
        System.out.println("SINK\t" + SINK);
        System.out.flush();
    }

    // ---------- measurement ----------

    static final int ROUNDS = 5;   // measured batches per pass
    static final int WARM = 2;     // discarded batches per pass
    static final long BATCH_NS = 120_000;

    static void benchGroup(int gid, Group g) {
        String[] names = g.methods();
        int n = names.length;
        int[] strategy = new int[n];       // per-method surviving arg strategy
        boolean[] usable = new boolean[n]; // method survived at least one strategy
        double[] single = new double[n];   // one timed call, for the SLOW lane

        for (int i = 0; i < n; i++) {
            for (int s = 0; s < 4 && !usable[i]; s++) {
                try {
                    g.setup(s);
                    long t0 = System.nanoTime();
                    SINK += g.call(i);
                    single[i] = System.nanoTime() - t0;
                    strategy[i] = s;
                    usable[i] = true;
                } catch (UnsatisfiedLinkError e) {
                    System.err.println("FATAL unbound: " + names[i] + ": " + e);
                    System.exit(3);
                } catch (Throwable t) {
                    System.err.println("  probe " + names[i] + " strategy " + s + ": "
                            + t.getClass().getSimpleName());
                }
            }
            if (!usable[i]) report(gid, g, names[i], Double.NaN, Double.NaN, Double.NaN, "ERR all-strategies");
        }

        double[] medF = onePass(g, names, usable, strategy, single, false);
        double[] medR = onePass(g, names, usable, strategy, single, true);

        for (int i = 0; i < n; i++) {
            if (!usable[i]) continue;
            if (single[i] > 250_000_000L) {
                // SLOW lane: 3 calls, mean (fresh args each call)
                g.setup(strategy[i]);
                long acc = 0;
                for (int k = 0; k < 3; k++) {
                    long a = System.nanoTime(); SINK += g.call(i); acc += System.nanoTime() - a;
                    g.setup(strategy[i]);
                }
                report(gid, g, names[i], acc / 3.0, acc / 3.0, acc / 3.0, "SLOW s" + strategy[i]);
            } else {
                double med = Math.min(medF[i], medR[i]);
                report(gid, g, names[i], med, med, med, "OK s" + strategy[i]);
            }
        }
    }

    /** One full pass over the group. Forward = alphabetical order, reverse = the opposite. */
    static double[] onePass(Group g, String[] names, boolean[] usable, int[] strategy,
                            double[] single, boolean reverse) {
        double[] med = new double[names.length];
        for (int x = 0; x < names.length; x++) {
            int i = reverse ? names.length - 1 - x : x;
            if (!usable[i]) continue;
            g.setup(strategy[i]);               // fresh args for EVERY method
            if (single[i] > 250_000_000L) { med[i] = single[i]; continue; }
            try {
                double[] s = new double[ROUNDS];
                for (int w = 0; w < WARM; w++) runBatch(g, i, 40_000);
                for (int r = 0; r < ROUNDS; r++) s[r] = runBatch(g, i, BATCH_NS);
                Arrays.sort(s);
                med[i] = s[s.length / 2];
            } catch (Throwable t) {
                System.err.println("  batch " + names[i] + " (" + (reverse ? "rev" : "fwd")
                        + "): " + t.getClass().getSimpleName());
                med[i] = Double.NaN;
            }
        }
        return med;
    }

    static double runBatch(Group g, int idx, long budgetNs) {
        long calls = 0, t0 = System.nanoTime(), el;
        do {
            for (int k = 0; k < 8; k++) SINK += g.call(idx);
            calls += 8;
            el = System.nanoTime() - t0;
        } while (el < budgetNs);
        return (double) el / calls;
    }

    static void report(int gid, Group g, String method, double med, double min, double max, String status) {
        String kind = method.startsWith("old") ? "old" : "alt";
        System.out.printf(Locale.ROOT, "RESULT\t%d\t%s\t%s\t%s\t%s\t%.1f\t%.1f\t%.1f\t%s%n",
                gid, g.fqcn(), g.sig(), method, kind, med, min, max, status);
        System.out.flush();
        System.err.println("  " + method + " -> " + status + " "
                + (Double.isNaN(med) ? "-" : String.format(Locale.ROOT, "%.1f ns/op", med)));
    }
}
