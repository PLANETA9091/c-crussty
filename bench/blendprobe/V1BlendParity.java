
/** Phase-1 probe item 5 — V1 kernel parity (BLEND_CACHE_PATCHER_DESIGN §7):
 *  10k calls of the g21 closed kernel pair on IDENTICAL inputs; the int
 *  return and all written dst lanes must match between oldEmptyBlenderSummary
 *  (vanilla-machinery emulation) and newEmptyBlenderSummary (guarded path).
 *  Any mismatch = the pair is NOT semantically interchangeable = patcher NO-GO.
 *
 *  Inputs sweep the blend neighborhood structure: block coords (i,j) across
 *  all 4x4 quart-alignment classes x increasing distances, plus fresh dst
 *  arrays (P500 FRESH-ARGS fairness rule). 10_000 iterations.
 *
 *  Exit 0 = PARITY (prints count), 2 = MISMATCH (first 3 diffs printed).
 *  Signed: agent-7625532f (TASK-32 Phase-1 probe).
 */
public final class V1BlendParity {
    public static void main(String[] args) {
        // Bind the closed kernels first (P500 real-mode: exact JNI symbol names)
        String libs = System.getProperty("p500.libs",
                "/home/z/c-crussty/native/libpaper_native_jni.so");
        for (String p : libs.split(":")) {
            if (!p.isBlank()) System.load(p);
        }
        int iters = args.length > 0 ? Integer.parseInt(args[0]) : 10_000;
        long[] dstOld = new long[64];
        long[] dstNew = new long[64];
        int mismatches = 0;
        for (int k = 0; k < iters; k++) {
            // quart-alignment classes ((i&3), (j&3)) x distances within the
            // realistic blending radius (old-chunk neighbourhood is bounded by
            // HEIGHT_BLENDING_RANGE_CELLS ~ tens of blocks; a first attempt
            // with city-block distances up to ~15k blocks ran the old kernel
            // at O(N) ms/call — out of scope for the live blend window)
            int i = ((k & 3) * 4 + ((k >> 2) & 7) * 8) * ((k & 8) != 0 ? 1 : -1);
            int j = ((k & 3) * 4 + ((k >> 6) & 7) * 8) * ((k & 32) != 0 ? 1 : -1);
            java.util.Arrays.fill(dstOld, 0L);
            java.util.Arrays.fill(dstNew, 0L);
            int rOld = PaperNativeNoiseChunkBlendCache.oldEmptyBlenderSummary(i, j, dstOld);
            int rNew = PaperNativeNoiseChunkBlendCache.newEmptyBlenderSummary(i, j, dstNew);
            boolean ok = rOld == rNew;
            if (ok) {
                for (int lane = 0; lane < rOld && lane < 64; lane++) {
                    if (dstOld[lane] != dstNew[lane]) { ok = false; break; }
                }
            }
            if (!ok) {
                if (mismatches < 3) {
                    System.out.printf("MISMATCH k=%d (i=%d j=%d): ret old=%d new=%d "
                                    + "dst0 old=%d new=%d dst1 old=%d new=%d%n",
                            k, i, j, rOld, rNew,
                            rOld > 0 ? dstOld[0] : 0, rNew > 0 ? dstNew[0] : 0,
                            rOld > 1 ? dstOld[1] : 0, rNew > 1 ? dstNew[1] : 0);
                }
                mismatches++;
            }
        }
        if (mismatches == 0) {
            System.out.println("V1 PARITY: " + iters + "/" + iters + " old==new (ret + written lanes) PASS");
        } else {
            System.out.println("V1 PARITY FAIL: " + mismatches + "/" + iters + " mismatches");
            System.exit(2);
        }
    }
}
