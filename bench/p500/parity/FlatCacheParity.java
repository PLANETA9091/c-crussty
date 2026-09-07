import java.util.Arrays;
import java.util.Random;

/**
 * TASK-53 — semantic parity gate for the PaperNativeNoiseChunkFlatCacheContext
 * old/new pairs (PROVEN_WINS_SYNC §4.2 promotion lifecycle, step 1).
 *
 * For every probe input, runs the OLD kernel and the NEW kernel on identical
 * fresh copies of the input array and byte-compares (return value, full dst
 * array contents). A mismatch forbids promotion (different semantics = the
 * swap is NOT transparent). On PASS, emits deterministic FIXTURE vectors
 * (inputs -> expected result + expected dst) that the plugin's live
 * self-test replays through the real bridge after the promote-rebind.
 *
 * Calls go through the canonical default-package P500 stub
 * (PaperNativeNoiseChunkFlatCacheContext), whose static natives bind by exact
 * JNI symbol name to the REAL closed-source libpaper_native_jni.so loaded via
 * -Dp500.libs (same contract as the canonical P500 harness). No mocks.
 *
 * Output lines (stdout, TSV):
 *   PARITY\t<pair>\tinputs=N\tmismatches=M\tverdict=PASS|MISMATCH
 *   FIXTURE\t<pair>\ta0\ta1\tlen\tresult\tv0,v1,..
 *   TIMING\t<kernel>\tmedian_ns_per_call (informational, non-authoritative)
 *   SINK\t<acc>            (DCE-proof: folded results + dst samples)
 */
public final class FlatCacheParity {

    private static long SINK = 0;

    public static void main(String[] args) {
        String libs = System.getProperty("p500.libs", "");
        if (libs.isEmpty()) {
            System.err.println("FATAL: -Dp500.libs not set");
            System.exit(2);
        }
        for (String p : libs.split(java.io.File.pathSeparator)) {
            if (!p.isBlank()) System.load(p);
        }
        System.err.println("libs loaded: " + libs);

        int badTrue = runPair("oldTrueContextSummary", "newTrueContextSummary");
        int badFalse = runPair("oldFalseContextSummary", "newFalseContextSummary");

        emitFixtures();

        timing("oldTrueContextSummary");
        timing("newTrueContextSummary");
        timing("oldFalseContextSummary");
        timing("newFalseContextSummary");

        System.out.println("SINK\t" + SINK);
        System.err.println("parity mismatches: true=" + badTrue + " false=" + badFalse);
        if (badTrue != 0 || badFalse != 0) System.exit(3);
    }

    private interface Sum {
        int apply(int a0, int a1, long[] dst);
    }

    private static Sum oldKernel(String tag) {
        return tag.equals("true")
                ? PaperNativeNoiseChunkFlatCacheContext::oldTrueContextSummary
                : PaperNativeNoiseChunkFlatCacheContext::oldFalseContextSummary;
    }

    private static Sum newKernel(String tag) {
        return tag.equals("true")
                ? PaperNativeNoiseChunkFlatCacheContext::newTrueContextSummary
                : PaperNativeNoiseChunkFlatCacheContext::newFalseContextSummary;
    }

    /** Runs one old/new pair through edges + randomized inputs. Returns mismatch count. */
    private static int runPair(String oldName, String newName) {
        String tag = oldName.contains("True") ? "true" : "false";
        Sum oldK = oldKernel(tag);
        Sum newK = newKernel(tag);

        int inputs = 0, mismatches = 0;
        long seed = 0xC0FFEEFCA5L ^ oldName.hashCode();
        Random rnd = new Random(seed);

        // --- edge inputs -------------------------------------------------
        int[] a0s = {0, 1, 16, 64, 256, 1024};
        int[] a1s = {0, 1, 7, 31, 63, 255};
        int[] lens = {0, 1, 2, 7, 64, 256};
        long[] specials = {0L, -1L, Long.MIN_VALUE, Long.MAX_VALUE, 1L << 40, -(1L << 40)};
        for (int a0 : a0s) {
            for (int a1 : a1s) {
                for (int len : lens) {
                    for (int v = 0; v < 3; v++) {
                        long[] src = new long[len];
                        switch (v) {
                            case 0 -> { /* zeros */ }
                            case 1 -> Arrays.fill(src, -1L);
                            default -> {
                                for (int i = 0; i < len; i++) src[i] = specials[rnd.nextInt(specials.length)];
                            }
                        }
                        long[][] out = new long[2][];
                        int[] res = new int[2];
                        res[0] = run(oldK, a0, a1, src, out, 0);
                        res[1] = run(newK, a0, a1, src, out, 1);
                        inputs++;
                        if (!same(out[0], out[1], res[0], res[1])) {
                            mismatches++;
                            if (mismatches <= 5) {
                                System.err.printf("MISMATCH %s a0=%d a1=%d len=%d v=%d res %d/%d%n",
                                        tag, a0, a1, len, v, res[0], res[1]);
                            }
                        }
                        SINK = SINK * 31 + res[0] + (out[0].length > 0 ? out[0][0] : 0);
                    }
                }
            }
        }

        // --- randomized inputs -------------------------------------------
        int[] rLens = {0, 1, 7, 16, 64, 128, 256};
        for (int i = 0; i < 3000; i++) {
            int a0 = rnd.nextInt(513);
            int a1 = rnd.nextInt(513);
            long[] src = new long[rLens[rnd.nextInt(rLens.length)]];
            for (int j = 0; j < src.length; j++) {
                int mode = rnd.nextInt(3);
                src[j] = mode == 0 ? rnd.nextInt(256)
                        : mode == 1 ? -rnd.nextInt(1 << 20)
                        : rnd.nextLong() >> (rnd.nextInt(48));
            }
            long[][] out = new long[2][];
            int[] res = new int[2];
            res[0] = run(oldK, a0, a1, src, out, 0);
            res[1] = run(newK, a0, a1, src, out, 1);
            inputs++;
            if (!same(out[0], out[1], res[0], res[1])) {
                mismatches++;
                if (mismatches <= 5) {
                    System.err.printf("MISMATCH %s rand a0=%d a1=%d len=%d res %d/%d%n",
                            tag, a0, a1, src.length, res[0], res[1]);
                }
            }
            SINK = SINK * 31 + res[1] + (out[1].length > 1 ? out[1][1] : 0);

            // cross-call-state detector: NEW called twice consecutively on
            // fresh copies must be self-consistent (same output both times).
            if (i % 97 == 0) {
                long[] c1 = src.clone(), c2 = src.clone();
                int rA = newK.apply(a0, a1, c1);
                int rB = newK.apply(a0, a1, c2);
                if (rA != rB || !Arrays.equals(c1, c2)) {
                    mismatches++;
                    System.err.printf("STATE-INCONSISTENCY %s rand a0=%d a1=%d len=%d%n",
                            tag, a0, a1, src.length);
                }
            }
        }

        String verdict = mismatches == 0 ? "PASS" : "MISMATCH";
        System.out.println("PARITY\t" + tag + "\tinputs=" + inputs
                + "\tmismatches=" + mismatches + "\tverdict=" + verdict);
        return mismatches;
    }

    private static int run(Sum k, int a0, int a1, long[] src, long[][] out, int slot) {
        long[] copy = src.clone();
        int r = k.apply(a0, a1, copy);
        out[slot] = copy;
        return r;
    }

    private static boolean same(long[] a, long[] b, int ra, int rb) {
        return ra == rb && Arrays.equals(a, b);
    }

    // --- deterministic fixtures for the plugin live self-test -------------
    // 4 vectors per pair: small deterministic arrays; the plugin replays
    // these through the REAL bridge after the promote-rebind and asserts
    // (result, dst) byte-equality against the OLD-impl expectations below.

    private static void emitFixtures() {
        fixture("true", 16, 31, new long[]{7, 31, 3, 15, 63, 1, 9, 21});
        fixture("true", 0, 7, new long[]{1, 2, 3, 4, 5, 6, 7, 8});
        fixture("true", 64, 63, new long[]{-5, 12, 0, -1, 4096, -4096, 77, 123});
        fixture("true", 256, 1, new long[]{1000000, -1000000, 42, 42, 42, 0, 0, 1});
        fixture("false", 16, 31, new long[]{7, 31, 3, 15, 63, 1, 9, 21});
        fixture("false", 0, 7, new long[]{1, 2, 3, 4, 5, 6, 7, 8});
        fixture("false", 64, 63, new long[]{-5, 12, 0, -1, 4096, -4096, 77, 123});
        fixture("false", 256, 1, new long[]{1000000, -1000000, 42, 42, 42, 0, 0, 1});
    }

    private static void fixture(String tag, int a0, int a1, long[] src) {
        Sum oldK = oldKernel(tag);
        long[] dst = src.clone();
        int res = oldK.apply(a0, a1, dst);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < dst.length; i++) {
            if (i > 0) sb.append(',');
            sb.append(dst[i]);
        }
        System.out.println("FIXTURE\t" + tag + "\t" + a0 + "\t" + a1 + "\t" + dst.length
                + "\t" + res + "\t" + sb);
        SINK = SINK * 31 + res;
    }

    // --- informational timing (non-authoritative; canonical = P500) -------
    private static void timing(String name) {
        Sum k = switch (name) {
            case "oldTrueContextSummary" -> PaperNativeNoiseChunkFlatCacheContext::oldTrueContextSummary;
            case "newTrueContextSummary" -> PaperNativeNoiseChunkFlatCacheContext::newTrueContextSummary;
            case "oldFalseContextSummary" -> PaperNativeNoiseChunkFlatCacheContext::oldFalseContextSummary;
            default -> PaperNativeNoiseChunkFlatCacheContext::newFalseContextSummary;
        };
        long[] dst = new long[64];
        Arrays.fill(dst, 31L);
        for (int w = 0; w < 2; w++) k.apply(16, 31, dst); // warmup, discarded
        int batch = 2000;
        long best = Long.MAX_VALUE;
        for (int r = 0; r < 5; r++) {
            long t0 = System.nanoTime();
            for (int i = 0; i < batch; i++) {
                SINK += k.apply(16, 31, dst);
            }
            long dt = System.nanoTime() - t0;
            best = Math.min(best, dt / batch);
        }
        System.out.println("TIMING\t" + name + "\t" + best);
    }
}
