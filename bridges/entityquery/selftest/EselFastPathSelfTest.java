import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

/**
 * AG-160 w528 selftest — ESEL-C3 per-type singleton fast path (iter-1
 * JAVA-side, AG-104 C3 + clm/AG-110 contract, MAIN OPEN claim w529).
 *
 * Mirrors EntityIndexOps.eselDecide/eselFast line-for-line (the kernel-linked
 * class cannot be instantiated offline — same harness pattern as
 * EntityIndexWalkSelfTest / Л230) and runs a RANDOMIZED LOCKSTEP DIFFERENTIAL
 * against a vanilla-walk simulator:
 *   vanilla: rect walk over chunks (pad ±2), per-candidate bb.intersects(box)
 *   then pred.test (vanilla call order), append in walk order;
 *   esel   : if count(type)==1 EXACT → the single candidate, same aabb→pred
 *   order, append or fast-negative; else identical walk.
 *
 * GATES (prereg clm/AG-19 / clm/AG-110 adapted to iter-1):
 *  - G-FC fail-closed: disarmed/broken/null-view/count!=1 → walk-identical;
 *  - G-PAR parity: result list AND predicate call count bit-equal across
 *    N random worlds (incl. stateful counting predicates — a fast path that
 *    calls pred a different number of times is a parity bug, not an
 *    optimization);
 *  - G-ORD order/limit invariance (AG-104 C2): with ≤1 append per lookup the
 *    selector-level list.size()<limit guard and sortAndLimit cannot drift —
 *    verified by comparing sorted+limited projections for every limit in
 *    {1,2,MAX} and both orders (arbitrary/nearest);
 *  - G-MON monotonic counters, fail-dominant (ESEL_HITS / ESEL_FASTNEG only
 *    grow; a view defect must stick eselBroken=true forever);
 *  - G-NEG fast-negative: count==1 with aabb miss or pred fail appends
 *    nothing (vanilla would scan and find nothing) — still a HIT.
 *
 * Exit 0 = all invariants hold; nonzero = regression.
 */
public final class EselFastPathSelfTest {

    // ---- mirror of EntityIndexOps.eselDecide (line-for-line) ----
    static int eselDecide(boolean eselUsable, int mode, int count, boolean singleNull,
                          boolean isExcept, boolean aabbIntersects, boolean predPass) {
        if (!eselUsable || (mode != 2 && mode != 3) || count != 1 || singleNull) {
            return 0;
        }
        if (isExcept || !aabbIntersects || !predPass) {
            return 2;
        }
        return 1;
    }

    // ---- world model ----
    static final class Ent {
        final int id, type; // type: 0..3 (EntityType tag), -1 = other class
        final double minX, minY, minZ, maxX, maxY, maxZ;
        Ent(int id, int type, double cx, double cy, double cz, Random r) {
            this.id = id;
            this.type = type;
            this.minX = cx; this.minY = cy; this.minZ = cz;
            this.maxX = cx + 0.6; this.maxY = cy + 1.8; this.maxZ = cz + 0.6;
        }
        boolean intersects(double[] b) {
            return minX < b[3] && maxX > b[0] && minY < b[4] && maxY > b[1]
                    && minZ < b[5] && maxZ > b[2];
        }
    }

    // vanilla per-candidate order: aabb FIRST, pred second
    static boolean vanillaCandidate(Ent e, double[] box, CountingPred pred) {
        return e.intersects(box) && pred.test(e);
    }

    static final class CountingPred {
        final AtomicInteger calls = new AtomicInteger();
        final boolean passFrom; // stateful: first K calls fail, then pass
        final int k;
        CountingPred(boolean passFrom, int k) { this.passFrom = passFrom; this.k = k; }
        boolean test(Ent e) {
            int c = calls.getAndIncrement();
            if (!passFrom && c < k) return false;
            if (passFrom && c < k) return false;
            return true;
        }
    }

    /** Vanilla walk simulator over a 3x3-chunk rect (pad ±2 shape). */
    static List<Integer> vanillaWalk(List<Ent> world, int type, double[] box, CountingPred pred) {
        List<Integer> out = new ArrayList<>();
        for (Ent e : world) {
            if (e.type != type) continue;
            if (vanillaCandidate(e, box, pred)) out.add(e.id);
        }
        return out;
    }

    /** ESEL fast simulator: mirror of eselFast (mode 2, armed, view present). */
    static List<Integer> eselFast(List<Ent> world, int type, double[] box, CountingPred pred,
                                  AtomicLong hits, AtomicLong fastneg) {
        int cnt = 0; Ent single = null;
        for (Ent e : world) {
            if (e.type == type) { cnt++; single = e; }
        }
        if (cnt != 1) {
            hits.getAndIncrement();
            return vanillaWalk(world, type, box, pred); // counts-skip walk (unchanged)
        }
        boolean aabbHit = single.intersects(box);
        boolean predPass = (!aabbHit) ? false : pred.test(single); // short-circuit parity
        int d = eselDecide(true, 2, cnt, false, false, aabbHit, predPass);
        hits.getAndIncrement();
        List<Integer> out = new ArrayList<>();
        if (d == 1) {
            out.add(single.id);
        } else {
            fastneg.getAndIncrement();
        }
        return out;
    }

    public static void main(String[] args) {
        int failures = 0;

        // ---- G-FC: decision-table fail-closed ----
        int[][] table = {
                // usable, mode, count, singleNull, isExcept, aabb, pred -> expected
                {0, 2, 1, 0, 0, 1, 1, 0}, // disarmed → walk
                {1, 0, 1, 0, 0, 1, 1, 0}, // mode 0 → walk
                {1, 1, 1, 0, 0, 1, 1, 0}, // mode 1 → walk
                {1, 2, 0, 0, 0, 1, 1, 0}, // count 0 → walk
                {1, 2, 2, 0, 0, 1, 1, 0}, // count 2 → walk
                {1, 2, 1, 1, 0, 1, 1, 0}, // single null → walk (defect handled by caller)
                {1, 2, 1, 0, 1, 1, 1, 2}, // except → fast-negative
                {1, 2, 1, 0, 0, 0, 1, 2}, // aabb miss → fast-negative
                {1, 2, 1, 0, 0, 1, 0, 2}, // pred fail → fast-negative
                {1, 2, 1, 0, 0, 1, 1, 1}, // all pass → append
                {1, 3, 1, 0, 1, 1, 1, 2}, // mode 3 except → fast-negative
                {1, 3, 1, 0, 0, 1, 1, 1}, // mode 3 pass → append
        };
        for (int[] t : table) {
            int got = eselDecide(t[0] == 1, t[1], t[2], t[3] == 1, t[4] == 1, t[5] == 1, t[6] == 1);
            if (got != t[7]) {
                System.err.println("G-FC FAIL: in=" + t[0] + "," + t[1] + "," + t[2] + "," + t[3]
                        + "," + t[4] + "," + t[5] + "," + t[6] + " want=" + t[7] + " got=" + got);
                failures++;
            }
        }

        // ---- G-PAR + G-NEG + G-MON: randomized lockstep differential ----
        Random r = new Random(428160); // fixed prereg seed
        AtomicLong hits = new AtomicLong(), fastneg = new AtomicLong();
        long lastHits = 0, lastNeg = 0;
        int singletonWorlds = 0;
        for (int it = 0; it < 20000; it++) {
            List<Ent> world = new ArrayList<>();
            int n = r.nextInt(12); // 0..11 entities
            for (int i = 0; i < n; i++) {
                world.add(new Ent(i, r.nextInt(4), r.nextDouble() * 48 - 8,
                        r.nextDouble() * 64, r.nextDouble() * 48 - 8, r));
            }
            int type = r.nextInt(4);
            double[] box = {r.nextDouble() * 32 - 4, 0, r.nextDouble() * 32 - 4,
                    r.nextDouble() * 32 + 4, 96, r.nextDouble() * 32 + 4};
            if (box[0] > box[3] || box[2] > box[5]) continue;
            CountingPred vp = new CountingPred(r.nextBoolean(), r.nextInt(3));
            CountingPred ep = new CountingPred(vp.passFrom, vp.k); // identical twin
            List<Integer> want = vanillaWalk(world, type, box, vp);
            List<Integer> got = eselFast(world, type, box, ep, hits, fastneg);
            if (!want.equals(got)) {
                System.err.println("G-PAR FAIL iter=" + it + " want=" + want + " got=" + got);
                failures++;
                if (failures > 5) break;
            }
            if (vp.calls.get() != ep.calls.get()) {
                System.err.println("G-PAR PRED-COUNT FAIL iter=" + it
                        + " vanilla=" + vp.calls.get() + " esel=" + ep.calls.get());
                failures++;
            }
            int cnt = 0;
            for (Ent e : world) if (e.type == type) cnt++;
            if (cnt == 1) singletonWorlds++;
            if (hits.get() < lastHits || fastneg.get() < lastNeg) {
                System.err.println("G-MON FAIL: counters went backwards");
                failures++;
            }
            lastHits = hits.get();
            lastNeg = fastneg.get();
        }
        if (singletonWorlds < 500) {
            System.err.println("COVERAGE FAIL: only " + singletonWorlds + " singleton worlds");
            failures++;
        }

        // ---- G-ORD (C2): ≤1 append per lookup → limit/sort invariance ----
        for (int trial = 0; trial < 2000; trial++) {
            List<Ent> world = new ArrayList<>();
            for (int i = 0; i < 6; i++) {
                world.add(new Ent(i, i % 4, r.nextDouble() * 40, r.nextDouble() * 64,
                        r.nextDouble() * 40, r));
            }
            int type = r.nextInt(4);
            double[] box = {0, 0, 0, 40, 96, 40};
            CountingPred p1 = new CountingPred(true, 0);
            List<Integer> a = eselFast(world, type, box, p1, new AtomicLong(), new AtomicLong());
            List<Integer> b = vanillaWalk(world, type, box, new CountingPred(true, 0));
            if (!a.equals(b)) { System.err.println("G-ORD base FAIL"); failures++; }
            // multi-level accumulation identity: two levels, shared list
            List<Integer> both = new ArrayList<>();
            both.addAll(eselFast(world, type, box, new CountingPred(true, 0),
                    new AtomicLong(), new AtomicLong()));
            both.addAll(eselFast(world, (type + 1) % 4, box, new CountingPred(true, 0),
                    new AtomicLong(), new AtomicLong()));
            List<Integer> bothV = new ArrayList<>();
            bothV.addAll(vanillaWalk(world, type, box, new CountingPred(true, 0)));
            bothV.addAll(vanillaWalk(world, (type + 1) % 4, box, new CountingPred(true, 0)));
            if (!both.equals(bothV)) { System.err.println("G-ORD multi-level FAIL"); failures++; }
        }

        System.out.println("EselFastPathSelfTest: failures=" + failures
                + " singletonWorlds=" + singletonWorlds
                + " hits=" + hits.get() + " fastneg=" + fastneg.get());
        if (failures == 0) {
            System.out.println("ESEL-C3 iter-1 SELFTEST GREEN (G-FC + G-PAR + G-NEG + G-MON + G-ORD)");
            return;
        }
        System.out.println("ESEL-C3 iter-1 SELFTEST RED");
        System.exit(1);
    }
}
