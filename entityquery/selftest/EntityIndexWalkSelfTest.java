import java.util.HashSet;
import java.util.Set;

/**
 * S59 selftest — EntityIndexOps.query counts-index arithmetic (TASK-405-C,
 * AIOOBE -32 fix, run 36264754336 / LAB_LEDGER Л230).
 *
 * The kernel-linked query() body cannot be instantiated offline (needs
 * EntityLookup/ChunkEntitySlices), so this harness mirrors the EXACT walk
 * arithmetic of the fixed source (the javap gate pins the blob: ishl/ior gz
 * decomposition, rowBase, Throwable belt) and checks it against the RUST
 * contract of eidxFlushQuery (src/entity_index.rs:708-711):
 * dst[(cz-minCZ)*w + (cx-minCX)] for the rect [minCX..maxCX]x[minCZ..maxCZ],
 * row-major, w=maxCX-minCX+1.
 *
 * Adversary zones (Л230): rz in {-2,-1,0,+1,+2} (crash at rz>=+1, silent
 * wrong-row miss at rz<=-1), same for rx; rect sizes 1..64; world-z 512+
 * farms = the original crash shape (1x1 rect at chunk (0,32)).
 *
 * Exit 0 = all invariants hold; nonzero = fix regression.
 */
public final class EntityIndexWalkSelfTest {

    /** The FIXED walk index: mirrors EntityIndexOps.query line-for-line. */
    static int newIdx(int minCX, int minCZ, int w, int rx, int rz, int z, int x) {
        int gz = (rz << 5) | z;                     // S59 fix line 1
        int rowBase = (gz - minCZ) * w - minCX;     // S59 fix line 2
        int cx = (rx << 5) | x;
        return rowBase + cx;
    }

    /** The BUGGY pre-S59 walk index (region-local z row) — repro oracle. */
    static int oldIdx(int minCX, int minCZ, int w, int rx, int rz, int z, int x) {
        int rowBase = (z - minCZ) * w - minCX;
        int cx = (rx << 5) | x;
        return rowBase + cx;
    }

    /** Rust layout: linear slot of chunk (cx,cz) in the counts rect. */
    static int rustIdx(int minCX, int minCZ, int w, int cx, int cz) {
        return (cz - minCZ) * w + (cx - minCX);
    }

    public static void main(String[] args) {
        int fails = 0;

        // --- 1. The exact crash repro (Л230: rz=1, w=1, first row -> -32,
        //        length 256 = default Buf.counts int[256]) -------------------
        // DetectorRail shape: 1x1 rect at chunk (0,32), i.e. world z=512+.
        int o = oldIdx(0, 32, 1, 0, 1, 0, 0);
        int n = newIdx(0, 32, 1, 0, 1, 0, 0);
        if (o != -32) { System.out.println("FAIL repro: old idx " + o + " != -32"); fails++; }
        else System.out.println("OK repro-crash: old formula gives exactly -32 (AIOOBE @len 256)");
        if (n != 0)   { System.out.println("FAIL repro: new idx " + n + " != 0"); fails++; }
        else System.out.println("OK repro-fixed: new formula gives 0 (= rust dst[0])");

        // --- 2. Silent-miss zone (rz <= -1): old reads a wrong row +32*w ----
        // 1x1 rect at chunk (0,-1) (minus-region): old idx = 32 (in a default
        // 256 buffer -> silent foreign-row read, superset violation).
        o = oldIdx(0, -1, 1, 0, -1, 31, 0); // cz=-1 -> rz=-1, local z=31
        n = newIdx(0, -1, 1, 0, -1, 31, 0);
        if (o != 32) { System.out.println("FAIL miss: old idx " + o + " != 32"); fails++; }
        else System.out.println("OK silent-miss: old formula reads row +32 (foreign row, no crash)");
        if (n != 0)  { System.out.println("FAIL miss: new idx " + n + " != 0"); fails++; }
        else System.out.println("OK silent-miss-fixed: new formula reads rust dst[0]");

        // --- 3. Full-walk parity sweep: every rect the walk can serve -------
        int[] regs = {-2, -1, 0, 1, 2};
        int[] sizes = {1, 2, 3, 17, 32, 33, 64};
        int rects = 0;
        for (int rzA : regs) for (int rxA : regs) {
            for (int w : sizes) for (int h : sizes) {
                // anchor corner chunk offsets inside the anchor region
                int[][] corners = {{0, 0}, {31, 31}, {-1, 0}, {0, -1}, {31, 32}};
                for (int[] c : corners) {
                    int minCX = (rxA << 5) + c[0];
                    int minCZ = (rzA << 5) + c[1];
                    int maxCX = minCX + w - 1, maxCZ = minCZ + h - 1;
                    int minRX = minCX >> 5, minRZ = minCZ >> 5;
                    int maxRX = maxCX >> 5, maxRZ = maxCZ >> 5;
                    Set<Integer> seen = new HashSet<>();
                    boolean ok = true;
                    for (int rz = minRZ; rz <= maxRZ; rz++) {
                        int zStart = rz == minRZ ? (minCZ & 31) : 0;
                        int zEnd = rz == maxRZ ? (maxCZ & 31) : 31;
                        for (int rx = minRX; rx <= maxRX; rx++) {
                            int xStart = rx == minRX ? (minCX & 31) : 0;
                            int xEnd = rx == maxRX ? (maxCX & 31) : 31;
                            for (int z = zStart; z <= zEnd; z++) {
                                for (int x = xStart; x <= xEnd; x++) {
                                    int idx = newIdx(minCX, minCZ, w, rx, rz, z, x);
                                    int cz = (rz << 5) | z, cx = (rx << 5) | x;
                                    int want = rustIdx(minCX, minCZ, w, cx, cz);
                                    if (idx != want || idx < 0 || idx >= w * h) {
                                        System.out.printf(
                                            "FAIL sweep rect[minCX=%d,minCZ=%d,w=%d,h=%d] rz=%d z=%d rx=%d x=%d: idx=%d want=%d%n",
                                            minCX, minCZ, w, h, rz, z, rx, x, idx, want);
                                        ok = false;
                                    }
                                    seen.add(idx);
                                }
                            }
                        }
                    }
                    // bijectivity: the walk touches every rust slot exactly once
                    if (seen.size() != w * h) {
                        System.out.printf("FAIL bijection rect[minCX=%d,minCZ=%d,w=%d,h=%d]: %d/%d slots%n",
                            minCX, minCZ, w, h, seen.size(), w * h);
                        ok = false;
                    }
                    if (!ok) fails++;
                    rects++;
                }
            }
        }
        System.out.println("OK sweep: " + rects + " adversarial rects, gz-walk == rust dst[(cz-minCZ)*w+(cx-minCX)], bijective, in-bounds");
        System.out.println(fails == 0 ? "SELFTEST PASS (0 fails)" : "SELFTEST FAIL (" + fails + ")");
        if (fails != 0) System.exit(1);
    }
}
