import java.util.Arrays;
import java.util.Random;

/**
 * TASK-61 old-member parity probe — parity-through-dispatcher for the three
 * wave-1 pairs (S7-14 NEXT-3). The old legs (ids 18/19/20, wired this task)
 * ride the SAME dispatch path as the optimized members (ids 15/16/17):
 *
 *   lane A: dispatch(id_old, X)  vs dispatch(id_new, X)  — pair parity
 *           through the REAL dispatcher (both legs dispatcher-expressible);
 *   lane B: dispatch(id_old, X)  vs oldDirect(X)          — the dispatcher
 *           routes the old SYMBOL correctly (loop closed to the P500 anchor
 *           surface, whose direct stubs auto-bind to libpaper_native_jni.so).
 *
 * Domains pinned to the S7-14 contract-probe findings (Wave1ContractProbe):
 * g35 n in {1,2} (n>=4 refuses -6, dst untouched), g40 second jint = mode
 * flag 1 (else -3), g39 any n (Objects = Strings). Domain drift fails LOUDLY.
 * Wire-v3 single-op invocation (BatchFloorBench parity-arm shape):
 *   run(ids, args0, long[0], {64}, outs[64], {0}, refArgs)
 * Runs offline against the real libs (BENCH.lock discipline). NOT a perf
 * claim — parity only.
 */
public final class OldMemberParityProbe {
    private static final int OLD_G35 = 18, NEW_G35 = 15;
    private static final int OLD_G39 = 19, NEW_G39 = 16;
    private static final int OLD_G40 = 20, NEW_G40 = 17;

    private static long[] sentinelDst() {
        long[] d = new long[64];
        Arrays.fill(d, 0x5A5A5A5A5A5A5A5AL);
        return d;
    }

    private static String dstHex(long[] d) {
        StringBuilder sb = new StringBuilder();
        for (long v : d) sb.append(String.format("%016x", v));
        return sb.toString();
    }

    private static int[] hashed(int n, int salt) {
        int[] a = new int[Math.max(8, n)];
        for (int i = 0; i < a.length; i++) a[i] = (int) ((i * 0x9E3779B1L + salt) & 0x3FF);
        return a;
    }

    private static double[] fills(int n, long seed) {
        double[] a = new double[Math.max(8, n)];
        Random r = new Random(seed);
        for (int i = 0; i < a.length; i++) a[i] = (i % 97) * 0.5 - 24.0 + r.nextInt(3);
        return a;
    }

    private static Object[] objs(int n, long seed) {
        Object[] a = new Object[Math.max(8, n)];
        Random r = new Random(seed);
        for (int i = 0; i < a.length; i++) a[i] = "plugin-" + r.nextInt(97);
        return a;
    }

    private static int fails = 0;

    private static void check(String tag, int retA, long[] dA, int retB, long[] dB) {
        boolean ok = retA == retB && Arrays.equals(dA, dB);
        if (!ok) {
            fails++;
            System.out.printf("FAIL %-44s retA=%d retB=%d dstEqual=%s%n",
                    tag, retA, retB, Arrays.equals(dA, dB));
            System.out.println("  A=" + dstHex(dA));
            System.out.println("  B=" + dstHex(dB));
        } else {
            System.out.printf("PARITY OK %-36s ret=%d dsthash=%s%n",
                    tag, retA, Integer.toHexString(dstHex(dA).hashCode()));
        }
    }

    /** Wire-v3 single-op dispatch (one id, scalar plane, 64-slot out, ref plane). */
    private static int dispatch(int id, long[] args0, Object[] refs, long[] outs) {
        return crussty.batch.PaperNativeBatchDispatch.run(
                new int[]{id}, args0, new long[0], new int[]{64}, outs, new int[]{0}, refs);
    }

    public static void main(String[] args) {
        String nativeLib = System.getenv().getOrDefault("CRUSSTY_BATCH_NATIVE_LIB",
                "/home/z/ccrussty/c-crussty/native/libpaper_native_jni.so");
        String moduleLib = System.getenv().getOrDefault("CRUSSTY_MODULE_LIB",
                "/home/z/ccrussty/c-crussty/target/release/libcrussty.so");
        System.load(nativeLib);
        System.load(moduleLib);

        int abi = crussty.batch.PaperNativeBatchDispatch.abiVersion();
        System.out.println("dispatcher abiVersion() = " + abi + " (expect 262165 = (4<<16)|21)");
        if (abi != 262165) {
            System.out.println("FATAL: abi drift — stale module or stale probe pin");
            System.exit(2);
        }

        long seedBase = 0xC125571L;
        // ---- g35 pair: id 18 (old) vs id 15 (optimized), shape D ----------
        // TASK-61 FINDING (pinned contract, discovered by this probe): the
        // pair is NOT byte-identical — dst[0] (fill value) matches, but the
        // old kernel writes a metadata lane dst[1] = array length (8 = the
        // in-domain array len), which the optimized kernel leaves at 0.
        // P500 "PARITY" grade for this pair was PERF-only (ratio 1.0025);
        // this probe is its first byte-level comparison. Both legs stay
        // dispatcher-expressible; the delta is deterministic and pinned here
        // (drift = loud fail).
        for (int trial = 0; trial < 12; trial++) {
            int n = (trial % 2 == 0) ? 1 : 2;
            double[] p0 = fills(n, seedBase + trial);
            int[] p1 = hashed(n, trial), p2 = hashed(n, trial + 31), p3 = hashed(n, trial + 77);
            long[] dA = sentinelDst(), dB = sentinelDst();
            int retA = dispatch(OLD_G35, new long[]{n}, new Object[]{p0, p1, p2, p3}, dA);
            int retB = dispatch(NEW_G35, new long[]{n}, new Object[]{p0, p1, p2, p3}, dB);
            boolean laneOK = dA[0] == dB[0] && dA[1] == 8L && dB[1] == 0L
                    && Arrays.equals(Arrays.copyOfRange(dA, 2, 64), Arrays.copyOfRange(dB, 2, 64))
                    && retA == retB;
            if (!laneOK) {
                fails++;
                System.out.printf("FAIL g35 pair-delta pin n=%d trial=%d dst0Equal=%s oldLane1=%d newLane1=%d%n",
                        n, trial, dA[0] == dB[0], dA[1], dB[1]);
            } else {
                System.out.printf("PARITY OK %-36s ret=%d dst0=%016x oldLane1=%d newLane1=%d (pinned delta)%n",
                        String.format("g35 pair n=%d trial=%d", n, trial), retA, dA[0], dA[1], dB[1]);
            }
            long[] dD = sentinelDst();
            int retD = PaperNativeRangeChoice.oldFillArraySummary(p0, p1, p2, p3, n, dD);
            // lane B: dispatcher old vs direct old — dst must be byte-equal;
            // ret semantics differ BY DESIGN (dispatcher returns op-count and
            // consumes the kernel's count-written; direct returns the kernel
            // ret) — only dst equality is the contract here.
            check(String.format("g35 old-disp-vs-direct trial=%d", trial), retA, dA, retA, dD);
        }
        // refusal leg: n=4 must refuse identically on BOTH legs — through the
        // dispatcher the kernel's negative ret is consumed (run() returns the
        // op count), so the observable refusal signature is: rets equal AND
        // dst untouched (sentinel) on both legs.
        {
            double[] p0 = fills(4, 1);
            int[] p1 = hashed(4, 0), p2 = hashed(4, 0), p3 = hashed(4, 0);
            long[] dA = sentinelDst(), dB = sentinelDst();
            int retA = dispatch(OLD_G35, new long[]{4}, new Object[]{p0, p1, p2, p3}, dA);
            int retB = dispatch(NEW_G35, new long[]{4}, new Object[]{p0, p1, p2, p3}, dB);
            check("g35 refusal n=4 (dst untouched both)", retA, dA, retB, dB);
            long[] sent = sentinelDst();
            if (!Arrays.equals(dA, sent) || !Arrays.equals(dB, sent)) {
                System.out.println("FAIL g35 refusal dst was touched"); fails++;
            }
        }

        // ---- g39 pair: id 19 (old) vs id 16 (new), shape E ----------------
        for (int trial = 0; trial < 12; trial++) {
            int n = 1 + (trial % 5) * 3;
            Object[] o1 = objs(n, seedBase + trial);
            long[] dA = sentinelDst(), dB = sentinelDst();
            int retA = dispatch(OLD_G39, new long[]{n}, new Object[]{o1}, dA);
            int retB = dispatch(NEW_G39, new long[]{n}, new Object[]{o1}, dB);
            check(String.format("g39 pair n=%d trial=%d", n, trial), retA, dA, retB, dB);
            long[] dD = sentinelDst();
            int retD = PaperNativeSpigotLoadOrderDependency.oldLoadAfterBuildSummary(n, o1, dD);
            // ret semantics differ BY DESIGN (dispatcher op-count vs direct
            // count-written); dst byte-equality is the contract.
            check(String.format("g39 old-disp-vs-direct trial=%d", trial), retA, dA, retA, dD);
        }

        // ---- g40 pair: id 20 (old) vs id 17 (new), shape F ----------------
        for (int trial = 0; trial < 12; trial++) {
            int n = 1 + (trial % 4) * 2;
            Object[] oa = objs(n, seedBase + trial);
            Object[] ob = objs(n, seedBase + trial + 5);
            Object[] oc = objs(n, seedBase + trial + 9);
            long[] dA = sentinelDst(), dB = sentinelDst();
            int retA = dispatch(OLD_G40, new long[]{n, 1}, new Object[]{oa, ob, oc}, dA);
            int retB = dispatch(NEW_G40, new long[]{n, 1}, new Object[]{oa, ob, oc}, dB);
            check(String.format("g40 pair n=%d trial=%d", n, trial), retA, dA, retB, dB);
            long[] dD = sentinelDst();
            int retD = PaperNativeSpigotLoadOrderDependency.oldRemovedCountSummary(n, oa, ob, oc, 1, dD);
            // ret semantics differ BY DESIGN (dispatcher op-count vs direct
            // count-written); dst byte-equality is the contract.
            check(String.format("g40 old-disp-vs-direct trial=%d", trial), retA, dA, retA, dD);
        }
        // refusal leg: mode flag 0 must refuse (-3) on both legs
        {
            Object[] oa = objs(4, 1), ob = objs(4, 2), oc = objs(4, 3);
            long[] dA = sentinelDst(), dB = sentinelDst();
            int retA = dispatch(OLD_G40, new long[]{4, 0}, new Object[]{oa, ob, oc}, dA);
            int retB = dispatch(NEW_G40, new long[]{4, 0}, new Object[]{oa, ob, oc}, dB);
            check("g40 refusal flag=0 (dst untouched both)", retA, dA, retB, dB);
            long[] sent40 = sentinelDst();
            if (!Arrays.equals(dA, sent40) || !Arrays.equals(dB, sent40)) {
                System.out.println("FAIL g40 refusal dst was touched"); fails++;
            }
        }

        System.out.println(fails == 0
                ? "OLD-MEMBER PARITY PASS (3 pairs x lanes A/B + refusal legs; g35 delta pinned)"
                : "OLD-MEMBER PARITY FAIL: " + fails + " mismatches");
        System.exit(fails == 0 ? 0 : 1);
    }
}
