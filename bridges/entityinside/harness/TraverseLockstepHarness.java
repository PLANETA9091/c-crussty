package harness;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.TraverseOps;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TraverseLockstepHarness (S7-163) — OFFLINE bit-exact lockstep between
 * the vanilla BlockGetter.forEachBlockIntersectedBetween and the flat
 * TraverseOps.forEachFlat replacement (lever #9 FLAT-TRAVERSAL).
 *
 * Methodology: random (from, to, box) scenarios across the REAL
 * operating envelope of Entity.checkInsideBlocks (entity-sized boxes,
 * short travel deltas) PLUS the degenerate families that pin the
 * verbatim-formula edges:
 *   - stationary boundary: delta length around square(1.0E-5f)
 *     (both sides of the Mth.square(1.0E-5f) threshold);
 *   - sign-zero axes: delta components exactly 0.0 (Mth.sign == 0 ->
 *     Double.MAX_VALUE steps, tMax = step*frac edge);
 *   - block-aligned coordinates (floor/frac edges, clip tMax = 0);
 *   - negative/large coordinates (asLong packing, int extents);
 *   - boxes spanning many blocks (multi-cell DDA marches) and
 *     sub-block boxes (corner-only visits);
 *   - clip-empty cells (dda cells the ray misses -> vanilla SKIPs).
 *
 * Visitor policy per scenario (scripted, deterministic seed):
 *   0 = always true, 1 = always false, 2 = true for first K visits
 *   then false (K = scenario-seeded), 3 = pseudo-random booleans.
 * The recorded sequence is (posLong, step) — pos is captured as
 * asLong() AT VISIT TIME because both implementations hand out a
 * shared mutable cursor (vanilla contract).
 *
 * Mismatch on ANY scenario (sequence, return value, or
 * short-circuit point) = FAIL (exit 2).
 *
 * Run: java -cp <kernel>:<libs>:entityinside/build:harness-build \
 *        harness.TraverseLockstepHarness [scenarios]
 */
public final class TraverseLockstepHarness {

    private static final int POLICIES = 4;

    private TraverseLockstepHarness() {}

    /** Visitor recording (posLong, step) pairs with a scripted boolean. */
    private static final class Recorder implements BlockGetter.BlockStepVisitor {
        final List<Long> pos = new ArrayList<>();
        final List<Integer> step = new ArrayList<>();
        int policy;
        final Random rnd;
        int visits;
        int cutoff;

        Recorder(int policy, long seed) {
            this.policy = policy;
            this.rnd = new Random(seed ^ 0x5DEECE66DL);
        }

        void reset(long seed) {
            this.pos.clear();
            this.step.clear();
            this.visits = 0;
            Random r = new Random(seed);
            this.cutoff = 1 + r.nextInt(24);
        }

        @Override
        public boolean visit(BlockPos p, int s) {
            this.pos.add(p.asLong());
            this.step.add(s);
            this.visits++;
            switch (this.policy) {
                case 0:
                    return true;
                case 1:
                    return false;
                case 2:
                    return this.visits <= this.cutoff;
                default:
                    return this.rnd.nextBoolean();
            }
        }
    }

    private static long runVanilla(Vec3 from, Vec3 to, AABB box,
                                   BlockGetter.BlockStepVisitor v) {
        boolean ok = BlockGetter.forEachBlockIntersectedBetween(from, to, box, v);
        return ok ? 1L : 0L;
    }

    private static long runFlat(Vec3 from, Vec3 to, AABB box,
                                BlockGetter.BlockStepVisitor v) {
        boolean ok = TraverseOps.forEachFlat(from, to, box, v);
        return ok ? 1L : 0L;
    }

    private static String cmp(Recorder a, Recorder b) {
        if (a.pos.size() != b.pos.size()) {
            return "visit-count " + a.pos.size() + " vs " + b.pos.size();
        }
        for (int i = 0; i < a.pos.size(); i++) {
            if (a.pos.get(i).longValue() != b.pos.get(i).longValue()
                    || a.step.get(i).intValue() != b.step.get(i).intValue()) {
                return "visit#" + i + ": (" + BlockPos.of(a.pos.get(i)) + ","
                        + a.step.get(i) + ") vs (" + BlockPos.of(b.pos.get(i))
                        + "," + b.step.get(i) + ")";
            }
        }
        return null;
    }

    private static boolean scenario(String tag, Vec3 from, Vec3 to, AABB box,
                                    long seed) {
        Recorder rv = new Recorder(0, seed);
        Recorder rf = new Recorder(0, seed);
        for (int policy = 0; policy < POLICIES; policy++) {
            rv.reset(seed + policy);
            rv.policy = policy;
            long retV = runVanilla(from, to, box, rv);
            rf.reset(seed + policy);
            rf.policy = policy;
            long retF = runFlat(from, to, box, rf);
            if (retV != retF) {
                System.out.println("FAIL[" + tag + "] policy=" + policy
                        + " return " + retV + " vs " + retF
                        + " from=" + from + " to=" + to + " box=" + box);
                return false;
            }
            String diff = cmp(rv, rf);
            if (diff != null) {
                System.out.println("FAIL[" + tag + "] policy=" + policy
                        + " " + diff + " from=" + from + " to=" + to
                        + " box=" + box);
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        int scenarios = args.length > 0 ? Integer.parseInt(args[0]) : 12000;
        Random rnd = new Random(0xC0FFEE);
        int failures = 0;
        int done = 0;

        // ---- degenerate families (deterministic, small) ----
        double[] aligned = {-64, -5, -1, 0, 1, 5, 64, 1234};
        for (double ax : aligned) {
            for (double ay : aligned) {
                Vec3 from = new Vec3(ax, ay, 0.5);
                Vec3 to = new Vec3(ax + 0.9, ay + 0.1, 0.5);
                AABB box = new AABB(ax - 0.3, ay, -0.15, ax + 0.3, ay + 1.8, 0.15);
                if (!scenario("aligned", from, to, box, (long) (ax * 31 + ay))) {
                    failures++;
                }
                done++;
            }
        }
        // stationary boundary: |delta| ~ 1.0E-5 threshold
        double s = 1.0E-5f;
        for (double d : new double[] {0.0, s * 0.5, s * 0.999, s, s * 1.001,
                s * 2.0, Math.sqrt(s), Math.sqrt(s) * 1.0000001}) {
            Vec3 from = new Vec3(10.0, 64.0, -20.0);
            Vec3 to = new Vec3(10.0 + d, 64.0, -20.0);
            AABB box = new AABB(9.7, 64.0, -20.3, 10.3, 65.8, -19.7);
            if (!scenario("stationary-boundary", from, to, box, (long) (d * 1e9))) {
                failures++;
            }
            done++;
        }
        // sign-zero axes (each subset)
        for (int m = 0; m < 8; m++) {
            Vec3 delta = new Vec3((m & 1) == 0 ? 0.0 : 1.5,
                    (m & 2) == 0 ? 0.0 : -0.75, (m & 4) == 0 ? 0.0 : 2.25);
            if (delta.lengthSqr() < MTH_SQUARE_EPS) {
                delta = new Vec3(delta.x == 0 ? 0.0 : delta.x,
                        delta.y == 0 ? 0.0 : delta.y, delta.z == 0 ? 3.0 : delta.z);
            }
            Vec3 from = new Vec3(3.25, 70.5, -11.75);
            Vec3 to = from.add(delta);
            AABB box = new AABB(2.95, 70.5, -12.05, 3.55, 72.3, -11.45);
            if (!scenario("sign-zero", from, to, box, 1000 + m)) {
                failures++;
            }
            done++;
        }
        // multi-block marches in all 27 octant directions
        for (int dx = -1; dx <= 1; dx++) {
            for (int dy = -1; dy <= 1; dy++) {
                for (int dz = -1; dz <= 1; dz++) {
                    if (dx == 0 && dy == 0 && dz == 0) {
                        continue;
                    }
                    Vec3 delta = new Vec3(dx * 5.5, dy * 3.25, dz * 7.75);
                    Vec3 from = new Vec3(1.37, 64.62, -3.11);
                    Vec3 to = from.add(delta);
                    AABB box = new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81);
                    if (!scenario("march", from, to, box, 2000 + dx * 9 + dy * 3 + dz)) {
                        failures++;
                    }
                    done++;
                }
            }
        }

        // ---- random families ----
        for (int i = 0; i < scenarios; i++) {
            double fx = (rnd.nextDouble() - 0.5) * 60.0;
            double fy = 20.0 + rnd.nextDouble() * 40.0;
            double fz = (rnd.nextDouble() - 0.5) * 60.0;
            Vec3 from = new Vec3(fx, fy, fz);
            // travel deltas: mostly short (entity-tick envelope), some long
            double mag = rnd.nextDouble() < 0.85
                    ? rnd.nextDouble() * 3.0
                    : rnd.nextDouble() * 12.0;
            double dx = (rnd.nextDouble() * 2.0 - 1.0) * mag;
            double dy = (rnd.nextDouble() * 2.0 - 1.0) * mag * 0.5;
            double dz = (rnd.nextDouble() * 2.0 - 1.0) * mag;
            Vec3 to = from.add(new Vec3(dx, dy, dz));
            double w = 0.3 + rnd.nextDouble() * 2.7;   // entity-sized boxes
            double h = 0.3 + rnd.nextDouble() * 2.7;
            AABB box = new AABB(to.x - w / 2, to.y, to.z - w / 2,
                    to.x + w / 2, to.y + h, to.z + w / 2);
            if (!scenario("random", from, to, box, rnd.nextLong())) {
                failures++;
                if (failures > 5) {
                    break;
                }
            }
            done++;
        }

        if (failures > 0) {
            System.out.println("TraverseLockstepHarness FAIL: " + failures
                    + " failures over " + done + " scenarios");
            System.exit(2);
        }
        System.out.println("TraverseLockstepHarness PASS: " + done
                + " scenarios x " + POLICIES
                + " visitor policies bit-in-bit (sequence+return)");
        System.out.println("TraverseLockstepHarness OFFLINE PASS");
    }

    // Mth.square(1.0E-5f) as a double constant for the generator only.
    private static final double MTH_SQUARE_EPS = (double) (1.0E-5f * 1.0E-5f);
}
