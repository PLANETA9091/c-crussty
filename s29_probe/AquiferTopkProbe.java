// S29/ROUND-468 - aquifer candidate-selection bit-exact probe (v2).
// Method: C2ME #558 top-k equivalence probe + C2ME #249 world-diff spirit
// (byte-identical decision-stream checksums between vanilla order and the
// cached/replay order).
//
// Ground truth = javap of Aquifer$NoiseBasedAquifer in patched-kernel.jar
// (Purpur 1.21.10, 2025-12-11 build): 12-cell scan (offX {0,1}, offY {-1..1},
// offZ {0,1}), packed-long cell centers (nextInt(10)/9/10 offsets), TOP-4
// mutually-exclusive `>=` cascade (bytecode 387..502), per-NoiseChunk full-grid
// aquiferLocationCache/aquiferCache.
//
// Variants compared, same iteration order, same `>=` tie semantics:
//  V0 VANILLA: per-block 12-cell scan, unpack BlockPos from packed long.
//  V1 CANDIDATE-CACHE (C2ME #558 mechanics): 12 candidates cached per grid
//     cell (refill on cell change), hot path computes top-2 only, deep path
//     replays full TOP-4 from stored candidate distances.
//
// Lanes (per block, both variants emit the SAME value sequence):
//  lane A = full TOP-4 stream (d1..d4 + xyz of positions 1..4)
//  lane B = hot TOP-2 stream (d1..d2 + xyz of positions 1..2)
// Verdict: laneA(V0)==laneA(V1) AND laneB(V0)==laneB(V1) => PROBE-PASS.
public class AquiferTopkProbe {
    static final int CELL_Y = 12;
    static final int CAND = 12;

    static long seedState;
    static int rnd(int bound) {
        seedState = seedState * 6364136223846793005L + 1442695040888963407L;
        return (int) (((seedState >>> 33) & 0x7FFFFFFF) % bound);
    }

    // ---- deterministic cell-center store (stand-in for aquiferLocationCache)
    static final int WORLD = 1 << 16;
    static long[] cellPos = new long[WORLD];
    static long[] slotKey = new long[WORLD];
    static long cellKey(int cx, int cy, int cz) { return (cx & 0x3FFFFFL) << 42 | (long)((cy & 0x3FFFL) << 28) | (cz & 0xFFFFFL); }
    static int cellSlot(long k) { return (int) (mix(k) & (WORLD - 1)); }
    static long mix(long x) { x ^= x >>> 33; x *= 0xff51afd7ed558ccdL; x ^= x >>> 33; return x; }
    static { java.util.Arrays.fill(slotKey, Long.MIN_VALUE); java.util.Arrays.fill(cellPos, Long.MAX_VALUE); }
    static long location(int cx, int cy, int cz) {
        long k = cellKey(cx, cy, cz); int s = cellSlot(k);
        if (slotKey[s] == k) return cellPos[s];
        slotKey[s] = k;
        seedState = mix(k) ^ 0x9E3779B97F4A7C15L;
        int px = (cx << 4) + rnd(10);
        int py = cy * CELL_Y + rnd(9);
        int pz = (cz << 4) + rnd(10);
        long p = ((long)(px & 0x3FFFFFF) << 38) | ((long)(py & 0xFFF) << 12) | (long)(pz & 0x3FFFFFF);
        cellPos[s] = p;
        return p;
    }
    static int pX(long p) { return (int) (p << 26 >> 38); }
    static int pY(long p) { return (int) ((p & 0xFFF0000) >> 16); }
    static int pZ(long p) { return (int) ((p << 38) >> 38); }

    static int gridX(int x) { return (x - 5) >> 4; }
    static int gridY(int y) { return Math.floorDiv(y + 1, CELL_Y); }
    static int gridZ(int z) { return (z - 5) >> 4; }

    // bijective stand-in cell index (probe coordinate window: cx,cz in [-4096,4095], cy in [-64,63])
    static int cellIdx(int cx, int cy, int cz) {
        return ((cx + 4096) << 19) | ((cy + 64) << 12) | (cz + 2048);
    }
    static int idxCx(int idx) { return (idx >>> 19) - 4096; }
    static int idxCy(int idx) { return ((idx >>> 12) & 0x7F) - 64; }
    static int idxCz(int idx) { return (idx & 0xFFF) - 2048; }

    // FNV-1a-style decision-stream lanes: [variant 0..1][lane A/B]
    static long[] accA = new long[2], accB = new long[2];
    static void emitA(int v, int x) { accA[v] = accA[v] * 1099511628211L ^ (x & 0xFFFFFFFFL); }
    static void emitB(int v, int x) { accB[v] = accB[v] * 1099511628211L ^ (x & 0xFFFFFFFFL); }

    static long unpacksVanilla = 0, unpacksCached = 0, readsVanilla = 0, refills = 0, replays = 0;

    // ---- V0 VANILLA: per-block 12-cell scan over packed longs + TOP-4 cascade
    static void vanillaBlock(int x, int y, int z, boolean deep) {
        int gx = gridX(x), gy = gridY(y), gz = gridZ(z);
        int d1 = Integer.MAX_VALUE, d2 = Integer.MAX_VALUE, d3 = Integer.MAX_VALUE, d4 = Integer.MAX_VALUE;
        long p1 = 0, p2 = 0, p3 = 0, p4 = 0;
        for (int offY = -1; offY <= 1; ++offY)
            for (int offZ = 0; offZ <= 1; ++offZ)
                for (int offX = 0; offX <= 1; ++offX) {
                    long pos = location(gx + offX, gy + offY, gz + offZ); readsVanilla++;
                    unpacksVanilla += 3;
                    int dx = pX(pos) - x, dy = pY(pos) - y, dz = pZ(pos) - z;
                    int dist = dx * dx + dy * dy + dz * dz;
                    if (d1 >= dist) { p4 = p3; p3 = p2; p2 = p1; p1 = pos; d4 = d3; d3 = d2; d2 = d1; d1 = dist; }
                    else if (d2 >= dist) { p4 = p3; p3 = p2; p2 = pos; d4 = d3; d3 = d2; d2 = dist; }
                    else if (d3 >= dist) { p4 = p3; p3 = pos; d4 = d3; d3 = dist; }
                    else if (d4 >= dist) { p4 = pos; d4 = dist; }
                }
        if (deep) {
            emitA(0, d1); emitA(0, d2); emitA(0, d3); emitA(0, d4);
            emitA(0, pX(p1)); emitA(0, pY(p1)); emitA(0, pZ(p1));
            emitA(0, pX(p2)); emitA(0, pY(p2)); emitA(0, pZ(p2));
            emitA(0, pX(p3)); emitA(0, pY(p3)); emitA(0, pZ(p3));
            emitA(0, pX(p4)); emitA(0, pY(p4)); emitA(0, pZ(p4));
        }
        emitB(0, d1); emitB(0, d2);
        emitB(0, pX(p1)); emitB(0, pY(p1)); emitB(0, pZ(p1));
        emitB(0, pX(p2)); emitB(0, pY(p2)); emitB(0, pZ(p2));
    }

    // ---- V1 CANDIDATE-CACHE: 12 candidates per grid cell; hot top-2; replay top-4
    static int lastGx = Integer.MIN_VALUE, lastGy = 0, lastGz = 0;
    static int[] candX = new int[CAND], candY = new int[CAND], candZ = new int[CAND];
    static int[] candIdx = new int[CAND], candDist = new int[CAND];
    static long[] candPos = new long[CAND];
    static void refillCandidates(int gx, int gy, int gz) {
        refills++;
        int c = 0;
        for (int offY = -1; offY <= 1; ++offY)
            for (int offZ = 0; offZ <= 1; ++offZ)
                for (int offX = 0; offX <= 1; ++offX) {
                    int cx = gx + offX, cy = gy + offY, cz = gz + offZ;
                    long pos = location(cx, cy, cz);
                    candX[c] = pX(pos); candY[c] = pY(pos); candZ[c] = pZ(pos);
                    candPos[c] = pos; candIdx[c] = cellIdx(cx, cy, cz);
                    unpacksCached += 3;
                    c++;
                }
    }
    static void cachedBlock(int x, int y, int z, boolean deep) {
        int gx = gridX(x), gy = gridY(y), gz = gridZ(z);
        if (gx != lastGx || gy != lastGy || gz != lastGz) { refillCandidates(gx, gy, gz); lastGx = gx; lastGy = gy; lastGz = gz; }
        int d1 = Integer.MAX_VALUE, d2 = Integer.MAX_VALUE;
        int i1 = 0, i2 = 0;
        for (int i = 0; i < CAND; i++) {
            int dx = candX[i] - x, dy = candY[i] - y, dz = candZ[i] - z;
            int dist = dx * dx + dy * dy + dz * dz;
            candDist[i] = dist;
            if (d2 >= dist) { i2 = i; d2 = dist; }
            if (d1 >= dist) { i2 = i1; d2 = d1; i1 = i; d1 = dist; }
        }
        // hot lane: emit from the cached candidate table (no re-unpack)
        emitB(1, d1); emitB(1, d2);
        emitB(1, candX[i1]); emitB(1, candY[i1]); emitB(1, candZ[i1]);
        emitB(1, candX[i2]); emitB(1, candY[i2]); emitB(1, candZ[i2]);
        if (!deep) return;
        // deep replay: full TOP-4 from stored distances, original order/`>=` semantics
        replays++;
        int D1 = Integer.MAX_VALUE, D2 = Integer.MAX_VALUE, D3 = Integer.MAX_VALUE, D4 = Integer.MAX_VALUE;
        int I1 = 0, I2 = 0, I3 = 0, I4 = 0;
        for (int i = 0; i < CAND; i++) {
            int dist = candDist[i];
            if (D1 >= dist) { I4 = I3; I3 = I2; I2 = I1; I1 = i; D4 = D3; D3 = D2; D2 = D1; D1 = dist; }
            else if (D2 >= dist) { I4 = I3; I3 = I2; I2 = i; D4 = D3; D3 = D2; D2 = dist; }
            else if (D3 >= dist) { I4 = I3; I3 = i; D4 = D3; D3 = dist; }
            else if (D4 >= dist) { I4 = i; D4 = dist; }
        }
        emitA(1, D1); emitA(1, D2); emitA(1, D3); emitA(1, D4);
        int[] sel = {I1, I2, I3, I4};
        for (int k = 0; k < 4; k++) {
            emitA(1, candX[sel[k]]); emitA(1, candY[sel[k]]); emitA(1, candZ[sel[k]]);
        }
    }

    public static void main(String[] args) {
        long t0 = System.nanoTime();
        long cases = 0, deepCases = 0;
        int chunks = Integer.parseInt(args.length > 0 ? args[0] : "64");
        long seed0 = Long.parseLong(args.length > 1 ? args[1] : "42");
        for (int chunk = 0; chunk < chunks; chunk++) {
            seedState = seed0 + chunk * 1000003L;
            int baseX = (rnd(2000) - 1000) << 4, baseZ = (rnd(2000) - 1000) << 4;
            lastGx = Integer.MIN_VALUE;
            for (int x = baseX; x < baseX + 16; x++)
                for (int z = baseZ; z < baseZ + 16; z++)
                    for (int y = -60; y < 200; y++) {
                        boolean deep = ((x + y + z) & 1) == 0; // 50% deep in v2 probe
                        vanillaBlock(x, y, z, deep);
                        cachedBlock(x, y, z, deep);
                        cases++;
                        if (deep) deepCases++;
                    }
        }
        String a0 = String.format("%016x", accA[0]), a1 = String.format("%016x", accA[1]);
        String b0 = String.format("%016x", accB[0]), b1 = String.format("%016x", accB[1]);
        System.out.printf("cases=%d deep_replay_cases=%d%n", cases, deepCases);
        System.out.printf("laneA(top4) vanilla=%s cached=%s%n", a0, a1);
        System.out.printf("laneB(top2) vanilla=%s cached=%s%n", b0, b1);
        System.out.printf("unpacks vanilla=%d cached=%d (saved %.2f%%)%n", unpacksVanilla, unpacksCached, 100.0 * (unpacksVanilla - unpacksCached) / unpacksVanilla);
        System.out.printf("location reads: vanilla=%d cached=%d (saved %.3f%%)%n", readsVanilla, refills * CAND, 100.0 * (readsVanilla - refills * CAND) / readsVanilla);
        System.out.printf("cell refills=%d replays=%d%n", refills, replays);
        System.out.printf("elapsed_ms=%d%n", (System.nanoTime() - t0) / 1_000_000);
        boolean pass = a0.equals(a1) && b0.equals(b1);
        System.out.println(pass ? "PROBE-PASS: both decision streams bit-identical" : "PROBE-FAIL: streams differ");
    }
}
