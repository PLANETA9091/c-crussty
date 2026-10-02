package ca.spottedleaf.moonrise.common.misc;

/**
 * TASK-64 variant C — A/B bench driver: end-to-end per-call cost of the
 * area-map apply bridge ({@code SingleUserAreaMapOps.run} = scratch mgmt +
 * closed native + apply loop + callbacks) for the LEGACY vs BUDGETED scratch
 * policy, same REAL native, same deterministic streams (design
 * docs/AREAMAP_DENSE_APPLY_DESIGN.md §11.4 row 4).
 *
 * The two arms are two classpaths: identical bench classes, different
 * SingleUserAreaMapOps bytes (area-map/build vs area-map/build-budget),
 * assembled by run_budget_ab.sh. Arm is recorded via -Dcrussty.arm.
 *
 * Streams (same shapes as the TASK-30 oracle): S1-MOVE = 8-direction bounded
 * ring walk at fixed d; S3-MIX = move every call + bounded ±1 resize on the
 * 4th call of each direction cycle. Never same-state (the fast path would
 * skip the measured work).
 *
 * Per-call correctness: emitted op count must equal the O(1) geometric
 * symmetric-difference size. Cross-arm multiset parity: the driver script
 * compares the order-insensitive additive (sum) and XOR folds — identical
 * native + identical streams must produce identical multisets in both arms
 * (the TASK-30 oracle remains the semantic canon).
 *
 * Methodology mirrors the sibling benches: 64 untimed warmup calls (the
 * budgeted scratch fills through side-effect-free rejects here), then 5
 * timed windows, median reported. Timing wraps ONLY the run() call.
 *
 * Output: one TSV line:
 * BUDGETAB arm=.. d=.. shape=.. calls=.. median_ns_per_call=.. total_ops=..
 * fold_sum=.. fold_xor=.. parity=OK|FAIL — exit 0 iff per-call parity held.
 */
public final class BudgetAbBench {
    private BudgetAbBench() {}

    static final int[][] DIR8 = {
        {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1}
    };

    /** Counting map: op counts + order-insensitive multiset folds. */
    static final class CountingMap extends SingleUserAreaMap<Object> {
        long ops = 0;
        long sumFold = 0;
        long xorFold = 0;

        void reset() { ops = 0; sumFold = 0; xorFold = 0; }

        @Override protected void addCallback(final Object param, final int chunkX, final int chunkZ) {
            ops++;
            final long h = mix64(pack(chunkX, chunkZ));
            sumFold += h;
            xorFold ^= h;
        }

        @Override protected void removeCallback(final Object param, final int chunkX, final int chunkZ) {
            ops++;
            // decorrelate the op classes so Add(x,z) and Remove(x,z) fold differently
            final long h = mix64(pack(chunkX, chunkZ) + 0x9E3779B97F4A7C15L);
            sumFold -= h;
            xorFold ^= h;
        }
    }

    static long pack(final int x, final int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }

    /** splitmix64 finalizer — cheap 64-bit avalanche for the folds. */
    static long mix64(long x) {
        x ^= x >>> 33; x *= 0xff51afd7ed558ccdL;
        x ^= x >>> 33; x *= 0xc4ceb9fe1a85ec53L;
        x ^= x >>> 33;
        return x;
    }

    /**
     * args: <d> <move|mix> <calls>
     */
    public static void main(final String[] args) {
        final int d = Integer.parseInt(args[0]);
        final String shape = args[1];
        final int calls = Integer.parseInt(args[2]);
        final String arm = System.getProperty("crussty.arm", "unknown");
        final String nativePath = System.getProperty("crussty.native");
        if (nativePath == null) {
            System.err.println("BUDGETAB requires -Dcrussty.native=<libpaper_native_jni.so>");
            System.exit(2);
        }
        System.load(nativePath);

        final CountingMap m = new CountingMap();
        int cx = 0, cz = 0, r = d;

        long warmOps = 0;
        for (int i = 0; i < 64; i++) {
            final int fx = cx, fz = cz, fd = r;
            cx += DIR8[i % 8][0]; cz += DIR8[i % 8][1];
            if ("mix".equals(shape)) {
                if (i % 8 == 0) r += 1; else if (i % 8 == 4) r -= 1;
            }
            m.reset();
            SingleUserAreaMapOps.run(m, fx, fz, fd, cx, cz, r, null);
            warmOps += m.ops;
        }

        final double[] per = new double[5];
        long totalOps = 0;
        long gSum = 0, gXor = 0;
        boolean parity = true;
        long idx = 64;
        for (int w = 0; w < 5; w++) {
            long t = 0L;
            for (int i = 0; i < calls; i++, idx++) {
                final int fx = cx, fz = cz, fd = r;
                cx += DIR8[(int) (idx % 8)][0]; cz += DIR8[(int) (idx % 8)][1];
                if ("mix".equals(shape)) {
                    if (idx % 8 == 0) r += 1; else if (idx % 8 == 4) r -= 1;
                }
                m.reset();
                final long t0 = System.nanoTime();
                SingleUserAreaMapOps.run(m, fx, fz, fd, cx, cz, r, null);
                t += System.nanoTime() - t0;
                if (m.ops != geomDiff2(fx, fz, fd, cx, cz, r)) parity = false;
                totalOps += m.ops;
                gSum += m.sumFold;
                gXor ^= m.xorFold;
            }
            per[w] = t / (double) calls;
        }
        java.util.Arrays.sort(per);

        System.out.println(String.format(
            "BUDGETAB arm=%s d=%d shape=%s calls=%d median_ns_per_call=%.1f total_ops=%d warmup_ops=%d fold_sum=%d fold_xor=%d parity=%s",
            arm, d, shape, calls, per[2], totalOps, warmOps, gSum, gXor, parity ? "OK" : "FAIL"));
        System.out.flush();
        System.exit(parity ? 0 : 1);
    }

    /** O(1) exact symmetric-difference size of two axis-aligned squares. */
    static long geomDiff2(final int fx, final int fz, final int fd,
                          final int tx, final int tz, final int td) {
        final long ox = Math.max(0L, Math.min((long) fx + fd, (long) tx + td)
                                     - Math.max((long) fx - fd, (long) tx - td) + 1L);
        final long oz = Math.max(0L, Math.min((long) fz + fd, (long) tz + td)
                                     - Math.max((long) fz - fd, (long) tz - td) + 1L);
        final long a = 2L * fd + 1L, b = 2L * td + 1L;
        return a * a + b * b - 2L * ox * oz;
    }
}
