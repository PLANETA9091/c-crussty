package dev.crussty.areamapprobe;

import ca.spottedleaf.moonrise.common.misc.SingleUserAreaMap;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * TASK-68 call-level probe: drives the PATCHED
 * {@code SingleUserAreaMap.update(int,int,int)} through the public entry in
 * the LIVE server JVM and checks the recorded callback delta against the
 * naive set difference. Where {@code bridge_selftest} (src/area_map.rs) calls
 * {@code PaperNativeAreaMap.nativeUpdateOpsBatch} DIRECTLY (native-only), this
 * probe exercises the full production chain: patched body -> invokestatic
 * {@code SingleUserAreaMapOps.run} (legacy OR budget arm — whichever bytes the
 * module defined) -> Scratch ThreadLocal -> native enumeration -> callback
 * dispatch into a recording map.
 *
 * Seeding uses the public {@code add(fx,fz,od)} (NOT reflection): javap of the
 * kernel fixture proves {@code add} performs the identical field writes plus
 * the full-square initial adds, so every rect additionally verifies the
 * original apply path (2d+1)^2 adds — free coverage, zero setAccessible.
 *
 * Compiled with {@code --release 8} (major 52) against the REAL kernel fixture
 * (tests/fixtures/SingleUserAreaMap.class) by scripts/build_area_map_probe.sh;
 * both classfiles (this + nested RecMap) are embedded into the module via
 * include_bytes! and defined into the MAP's loader by src/area_map.rs (same
 * runtime-package rules as the ops helpers; unique names, one-shot per JVM).
 *
 * Fail-safe contract: diagnostics-only. Every row catches Throwable, records
 * a failure bit + detail line, and probe() always returns — a probe failure
 * can never abort the boot or alter server behavior. Deterministic: fixed
 * xorshift64 seed, no clock, no randomness, no server state touched (all maps
 * are private RecMap instances on the probe thread).
 */
public final class AreaMapProbe {

    // ---- failure bits (returned from probe(); 0 == all green) ------------
    public static final int B_NOT_SET = 1 << 0;   // NOT_SET guard row
    public static final int B_IAE_FRESH = 1 << 1; // negative-d on fresh map
    public static final int B_IAE_SEEDED = 1 << 2; // negative-d after seeding
    public static final int B_RETRY = 1 << 3;     // budget -n0 / legacy grow row (n0=650 > 578)
    public static final int B_ESCALATE = 1 << 4;  // escalation row (n0=1458)
    public static final int B_GROW = 1 << 5;      // ring grow + same-state after grow
    public static final int B_SAMESTATE = 1 << 6; // 64x same-state, zero delta
    public static final int B_SPOTS = 1 << 7;     // 7 hand-verified spots
    public static final int B_SWEEP = 1 << 8;     // 64 seeded rects (bridge_selftest twin)

    /** Total driven rects (for the marker line; rows + 64 same-state + 7 spots + 64 sweep). */
    public static final int TOTAL_RECTS = 141;

    private static int bits = 0;
    private static final List<String> DETAILS = new ArrayList<String>();

    /** Scratch.ops.length captured right after the retry row (arm discriminator: 650 budget / 1156 legacy). */
    public static int scratchLenAfterRetry = -1;

    private static final Object TOKEN = new Object();

    private AreaMapProbe() {}

    /** Recording map: appends every callback as (key, add?) into grow-only arrays. */
    public static final class RecMap extends SingleUserAreaMap<Object> {
        public long[] keys = new long[1024];
        public boolean[] adds = new boolean[1024];
        public int cnt = 0;

        public RecMap(Object token) {
            super(token);
        }

        private void rec(boolean add, int x, int z) {
            if (cnt == keys.length) {
                keys = Arrays.copyOf(keys, keys.length * 2);
                adds = Arrays.copyOf(adds, adds.length * 2);
            }
            keys[cnt] = key(x, z);
            adds[cnt] = add;
            cnt++;
        }

        @Override
        protected void addCallback(Object param, int chunkX, int chunkZ) {
            if (param != getParameter()) {
                throw new AssertionError("addCallback: param identity != map parameter");
            }
            rec(true, chunkX, chunkZ);
        }

        @Override
        protected void removeCallback(Object param, int chunkX, int chunkZ) {
            if (param != getParameter()) {
                throw new AssertionError("removeCallback: param identity != map parameter");
            }
            rec(false, chunkX, chunkZ);
        }
    }

    // ---- key/naive-diff helpers (mirror tests/area_map_smoke NaiveDiff) --

    private static long key(int x, int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }

    /** Sorted cell keys of the (2d+1)^2 square at (cx,cz). */
    private static long[] square(int cx, int cz, int d) {
        int side = 2 * d + 1;
        long[] out = new long[side * side];
        int i = 0;
        for (int z = cz - d; z <= cz + d; z++) {
            for (int x = cx - d; x <= cx + d; x++) {
                out[i++] = key(x, z);
            }
        }
        Arrays.sort(out);
        return out;
    }

    /** Sorted set difference a \ b (two-pointer over sorted arrays). */
    private static long[] setMinus(long[] a, long[] b) {
        long[] out = new long[a.length];
        int n = 0;
        int i = 0, j = 0;
        while (i < a.length) {
            while (j < b.length && b[j] < a[i]) {
                j++;
            }
            if (j < b.length && b[j] == a[i]) {
                j++;
            } else {
                out[n++] = a[i];
            }
            i++;
        }
        return Arrays.copyOf(out, n);
    }

    private static boolean eq(long[] a, long[] b) {
        if (a.length != b.length) {
            return false;
        }
        for (int i = 0; i < a.length; i++) {
            if (a[i] != b[i]) {
                return false;
            }
        }
        return true;
    }

    private static synchronized void fail(int bit, String msg) {
        bits |= bit;
        if (DETAILS.size() < 3) {
            DETAILS.add(msg);
        }
    }

    /**
     * One fresh-map rect: add(fx,fz,od) seeds state and must fire exactly
     * (2od+1)^2 adds; update(tx,tz,nd) must return true, write the getters,
     * and produce a callback delta == naive set difference.
     */
    private static boolean runRect(int fx, int fz, int od, int tx, int tz, int nd,
                                   int bit, String tag) {
        try {
            RecMap m = new RecMap(TOKEN);
            m.add(fx, fz, od);
            int before = m.cnt;
            boolean okInit = before == (2 * od + 1) * (2 * od + 1);
            boolean r = m.update(tx, tz, nd);
            boolean okGet = m.getLastChunkX() == tx && m.getLastChunkZ() == tz
                    && m.getLastDistance() == nd;
            long[] adds = new long[m.cnt - before];
            long[] rems = new long[m.cnt - before];
            int na = 0, nr = 0;
            for (int i = before; i < m.cnt; i++) {
                if (m.adds[i]) {
                    adds[na++] = m.keys[i];
                } else {
                    rems[nr++] = m.keys[i];
                }
            }
            Arrays.sort(adds, 0, na);
            Arrays.sort(rems, 0, nr);
            long[] ea = setMinus(square(tx, tz, nd), square(fx, fz, od));
            long[] er = setMinus(square(fx, fz, od), square(tx, tz, nd));
            boolean ok = okInit && r && okGet
                    && na == ea.length && nr == er.length
                    && eq(Arrays.copyOf(adds, na), ea)
                    && eq(Arrays.copyOf(rems, nr), er);
            if (!ok) {
                fail(bit, tag + ": init=" + okInit + " ret=" + r + " get=" + okGet
                        + " adds " + na + "/" + ea.length + " rems " + nr + "/" + er.length);
            }
            return ok;
        } catch (Throwable t) {
            fail(bit, tag + ": threw " + t);
            return false;
        }
    }

    /** xorshift64 — same seed/stream as bridge_selftest (src/area_map.rs) and SmokeMain CHK-3. */
    private static long seed = 0x9E3779B97F4A7C15L;

    private static long next() {
        seed ^= seed << 13;
        seed ^= seed >>> 7;
        seed ^= seed << 17;
        return seed;
    }

    /** Reflection read of the ops Scratch ThreadLocal for THIS thread (arm discriminator). */
    private static int scratchOpsLen() {
        try {
            Class<?> ops = Class.forName("ca.spottedleaf.moonrise.common.misc.SingleUserAreaMapOps");
            Field fScratch = ops.getDeclaredField("SCRATCH");
            fScratch.setAccessible(true);
            Object tl = fScratch.get(null);
            Method get = tl.getClass().getMethod("get");
            Object scratch = get.invoke(tl);
            Field fOps = scratch.getClass().getDeclaredField("ops");
            fOps.setAccessible(true);
            return ((byte[]) fOps.get(scratch)).length;
        } catch (Throwable t) {
            return -1;
        }
    }

    /**
     * Full matrix. Returns 0 on all-green, else the OR of failure bits;
     * {@link #detail()} carries the first failure descriptions and
     * {@link #scratchLenAfterRetry} the arm discriminator captured after the
     * retry row.
     */
    public static synchronized int probe() {
        bits = 0;
        DETAILS.clear();
        scratchLenAfterRetry = -1;

        // Row 1: NOT_SET guard — fresh map update() returns false, fields
        // untouched, no callbacks, run() never reached (no Scratch touch).
        try {
            RecMap m = new RecMap(TOKEN);
            boolean r = m.update(2, 3, 4);
            boolean ok = !r && m.cnt == 0
                    && m.getLastChunkX() == Integer.MIN_VALUE
                    && m.getLastChunkZ() == Integer.MIN_VALUE
                    && m.getLastDistance() == Integer.MIN_VALUE;
            if (!ok) {
                fail(B_NOT_SET, "NOT_SET: ret=" + r + " cnt=" + m.cnt
                        + " gx=" + m.getLastChunkX());
            }
        } catch (Throwable t) {
            fail(B_NOT_SET, "NOT_SET threw " + t);
        }

        // Row 2: negative-d IAE on a fresh map (guard A precedes NOT_SET).
        try {
            RecMap m = new RecMap(TOKEN);
            try {
                m.update(0, 0, -1);
                fail(B_IAE_FRESH, "IAE-fresh: no exception");
            } catch (IllegalArgumentException e) {
                if (!String.valueOf(e.getMessage()).contains("-1")) {
                    fail(B_IAE_FRESH, "IAE-fresh: msg=" + e.getMessage());
                }
            }
        } catch (Throwable t) {
            fail(B_IAE_FRESH, "IAE-fresh outer " + t);
        }

        // Row 3: negative-d IAE after seeding — fields untouched, no callbacks.
        try {
            RecMap m = new RecMap(TOKEN);
            m.add(5, 5, 5);
            int before = m.cnt;
            try {
                m.update(6, 6, -1);
                fail(B_IAE_SEEDED, "IAE-seeded: no exception");
            } catch (IllegalArgumentException e) {
                boolean ok = String.valueOf(e.getMessage()).contains("-1")
                        && m.getLastChunkX() == 5 && m.getLastChunkZ() == 5
                        && m.getLastDistance() == 5 && m.cnt == before;
                if (!ok) {
                    fail(B_IAE_SEEDED, "IAE-seeded: msg=" + e.getMessage()
                            + " cnt " + m.cnt + "/" + before);
                }
            }
        } catch (Throwable t) {
            fail(B_IAE_SEEDED, "IAE-seeded outer " + t);
        }

        // Row 4: FIRST changed-state update on this thread => Scratch is at
        // INITIAL_CAP=578; n0 = 17^2+19^2 = 650 > 578, so the budget arm takes
        // the -n0 retry and the legacy arm grows 578->1156. Full parity check.
        runRect(0, 0, 8, 20, 20, 9, B_RETRY, "retry");
        scratchLenAfterRetry = scratchOpsLen();

        // Row 5: escalation — n0 = 27^2+27^2 = 1458 (budget retries to exact
        // req; legacy doubles 1156->2312).
        runRect(0, 0, 13, 40, 40, 13, B_ESCALATE, "escalate");

        // Row 6: ring grow (CHK-1 mirror) + same-state after grow.
        try {
            RecMap m = new RecMap(TOKEN);
            m.add(12, 9, 8);
            int before = m.cnt;
            boolean r = m.update(12, 9, 9);
            int delta = m.cnt - before;
            int before2 = m.cnt;
            boolean r2 = m.update(12, 9, 9);
            boolean ok = r && delta == 72 && r2 && m.cnt == before2
                    && m.getLastChunkX() == 12 && m.getLastChunkZ() == 9
                    && m.getLastDistance() == 9;
            if (!ok) {
                fail(B_GROW, "grow: r=" + r + " delta=" + delta + " r2=" + r2
                        + " d2=" + (m.cnt - before2));
            }
        } catch (Throwable t) {
            fail(B_GROW, "grow threw " + t);
        }

        // Row 7: same-state fast path x64 — every call returns true with zero
        // delta (the patched body still writes the fields; run() early-outs).
        try {
            RecMap m = new RecMap(TOKEN);
            m.add(7, -3, 4);
            int bad = 0;
            for (int i = 0; i < 64; i++) {
                int before = m.cnt;
                boolean r = m.update(7, -3, 4);
                if (!r || m.cnt != before) {
                    bad++;
                }
            }
            if (bad != 0) {
                fail(B_SAMESTATE, "same-state: " + bad + "/64 bad");
            }
        } catch (Throwable t) {
            fail(B_SAMESTATE, "same-state threw " + t);
        }

        // Row 8: hand-verified spots (SmokeMain CHK-3 set; the "(0,0,1)->(0,0,2)"
        // delta is 16 adds — the rig's "24 adds" comment is wrong, not the code).
        int spotBad = 0;
        spotBad += runRect(5, 5, 3, 5, 5, 3, B_SPOTS, "spot0") ? 0 : 1;
        spotBad += runRect(0, 0, 0, 0, 0, 0, B_SPOTS, "spot1") ? 0 : 1;
        spotBad += runRect(0, 0, 0, 1, 0, 0, B_SPOTS, "spot2") ? 0 : 1;
        spotBad += runRect(0, 0, 1, 1, 1, 1, B_SPOTS, "spot3") ? 0 : 1;
        spotBad += runRect(0, 0, 2, 0, 0, 1, B_SPOTS, "spot4") ? 0 : 1;
        spotBad += runRect(0, 0, 1, 0, 0, 2, B_SPOTS, "spot5") ? 0 : 1;
        spotBad += runRect(-3, -3, 6, 3, 3, 6, B_SPOTS, "spot6") ? 0 : 1;
        if (spotBad != 0) {
            fail(B_SPOTS, "spots: " + spotBad + "/7 bad");
        }

        // Row 9: 64 seeded rects — the exact xorshift64 stream (seed, draw
        // order fx,fz,tx,tz,od,nd) of bridge_selftest: the direct-native
        // selftest already proved this stream against the naive oracle at the
        // native layer; here the SAME stream must survive the full patched
        // call chain.
        int sweepBad = 0;
        for (int i = 0; i < 64; i++) {
            int fx = (int) (Long.remainderUnsigned(next(), 21)) - 10;
            int fz = (int) (Long.remainderUnsigned(next(), 21)) - 10;
            int tx = (int) (Long.remainderUnsigned(next(), 21)) - 10;
            int tz = (int) (Long.remainderUnsigned(next(), 21)) - 10;
            int od = (int) (Long.remainderUnsigned(next(), 7));
            int nd = (int) (Long.remainderUnsigned(next(), 7));
            if (!runRect(fx, fz, od, tx, tz, nd, B_SWEEP, "sweep" + i)) {
                sweepBad++;
            }
        }
        if (sweepBad != 0) {
            fail(B_SWEEP, "sweep: " + sweepBad + "/64 bad");
        }

        return bits;
    }

    /** First (up to 3) failure descriptions, "OK" when none. */
    public static synchronized String detail() {
        if (DETAILS.isEmpty()) {
            return "OK";
        }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < DETAILS.size(); i++) {
            if (i > 0) {
                sb.append(" | ");
            }
            sb.append(DETAILS.get(i));
        }
        return sb.toString();
    }
}
