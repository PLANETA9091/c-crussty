import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.TraverseOps;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** DDA-1 root-cause probe (TASK-459-L12): vanilla walk = truth vs
 *  TraverseOps.forEachFlat transcription, adversarial degenerate families. */
public final class DdaProbe {
    static final int POLICIES = 4;
    static final class Rec implements BlockGetter.BlockStepVisitor {
        final List<Long> pos = new ArrayList<>();
        final List<Integer> step = new ArrayList<>();
        int policy; final Random rnd; int visits; int cutoff;
        Rec(int policy, long seed) { this.policy = policy; this.rnd = new Random(seed ^ 0x5DEECE66DL); }
        void reset(long seed) { pos.clear(); step.clear(); visits = 0;
            Random r = new Random(seed); cutoff = 1 + r.nextInt(24); }
        public boolean visit(BlockPos p, int s) {
            pos.add(p.asLong()); step.add(s); visits++;
            switch (policy) { case 0: return true; case 1: return false;
                case 2: return visits <= cutoff; default: return rnd.nextBoolean(); }
        }
    }
    static long van(Vec3 f, Vec3 t, AABB b, Rec r) {
        return BlockGetter.forEachBlockIntersectedBetween(f, t, b, r) ? 1 : 0;
    }
    static long flat(Vec3 f, Vec3 t, AABB b, Rec r) {
        return TraverseOps.forEachFlat(f, t, b, r) ? 1 : 0;
    }
    static boolean scenario(Vec3 from, Vec3 to, AABB box, long seed) {
        for (int p = 0; p < POLICIES; p++) {
            Rec a = new Rec(p, seed + p); Rec b = new Rec(p, seed + p);
            a.reset(seed + p); long rv = van(from, to, box, a);
            b.reset(seed + p); long rf = flat(from, to, box, b);
            if (rv != rf) return false;
            if (a.pos.size() != b.pos.size()) return false;
            for (int i = 0; i < a.pos.size(); i++)
                if (a.pos.get(i).longValue() != b.pos.get(i).longValue()
                        || a.step.get(i).intValue() != b.step.get(i).intValue()) return false;
        }
        return true;
    }

    static int[] famCount = new int[16];
    static String[] famName = new String[16];
    static int famN = 0;
    static int famIdx(String n) { for (int i = 0; i < famN; i++) if (famName[i].equals(n)) return i;
        famName[famN] = n; return famN++; }
    static void record(String fam) { int i = famIdx(fam); famCount[i] += 1; }
    static void run(String fam, Vec3 f, Vec3 t, AABB b, long seed) { if (!scenario(f, t, b, seed)) record(fam); }

    public static void main(String[] args) {
        // ---- F1: exact-corner rays: 3-axis integer crossings at common t ----
        for (int a = 0; a < 40; a++) {
            double x0 = (a % 5) - 2.0, y0 = 64.0 + (a / 5) * 1.0, z0 = -3.0 + (a % 3);
            Vec3 f = new Vec3(x0, y0, z0);
            double[][] deltas = {{2,-2,2},{3,-3,3},{4,-2,2},{2,-4,4},{6,-2,4},{2,-2,6},{3,3,-3},{2,2,-2}};
            for (double[] d : deltas) {
                Vec3 t = new Vec3(x0 + d[0], y0 + d[1], z0 + d[2]);
                AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
                run("F1-corner-exact", f, t, b, 7000 + a);
            }
        }
        // ---- F2: axis-aligned travel ON integer plane (edge ray whole way) ----
        for (int a = 0; a < 40; a++) {
            double y = 64.0 + (a % 4), z = -3.0 + (a / 4);
            Vec3 f = new Vec3(1.0, y, z);
            Vec3 t = new Vec3(1.0 + 2 + (a % 5), y, z);
            AABB b = new AABB(0.7, y, z - 0.3, 1.3, y + 1.8, z + 0.3);
            run("F2-axis-edge", f, t, b, 8000 + a);
        }
        // ---- F3: tMax ties |dx|==|dz| and full diagonal ----
        for (int a = 0; a < 40; a++) {
            double x0 = (a % 5) - 2.0 + 0.37, y0 = 64.62, z0 = -3.11;
            Vec3 f = new Vec3(x0, y0, z0);
            double m = 1.0 + (a % 6) * 0.5;
            double[][] deltas = {{m,0,m},{m,0,-m},{-m,m,m},{m,m,m},{-m,-m,-m},{m,-m,m}};
            for (double[] d : deltas) {
                Vec3 t = new Vec3(x0 + d[0], y0 + d[1], z0 + d[2]);
                AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
                run("F3-tmax-tie", f, t, b, 9000 + a);
            }
        }
        // ---- F4: large coords (float-eps clamp regime, near/beyond 2^24) ----
        for (int a = 0; a < 40; a++) {
            double base = 1.0e5 * (1 + a % 4) * (a % 2 == 0 ? 1 : 3.0);
            if (a % 4 == 3) base = 3.0e7;
            double x0 = base + 0.37, y0 = 64.62, z0 = -base - 3.11;
            Vec3 f = new Vec3(x0, y0, z0);
            Vec3 t = new Vec3(x0 + 2.5, y0 - 1.25, z0 + 4.75);
            AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
            run("F4-large-coords", f, t, b, 10000 + a);
        }
        // ---- F5: stationary boundary flanks ----
        double s = 1.0E-5f;
        for (double d : new double[]{0.0, s*0.5, s*0.999, s, s*1.001, s*2.0, Math.sqrt(s), Math.sqrt(s)*1.0000001, s*0.99999999, s*1.00000001}) {
            Vec3 f = new Vec3(10.0, 64.0, -20.0);
            Vec3 t = new Vec3(10.0 + d, 64.0, -20.0);
            AABB b = new AABB(9.7, 64.0, -20.3, 10.3, 65.8, -19.7);
            run("F5-stationary", f, t, b, (long)(d * 1e9));
        }
        // ---- F6: sign-zero subsets on integer crossings ----
        for (int m = 0; m < 8; m++) {
            Vec3 delta = new Vec3((m&1)==0?0.0:2.0, (m&2)==0?0.0:-2.0, (m&4)==0?0.0:2.0);
            if (delta.lengthSqr() < s*s) delta = new Vec3(delta.x==0?0.0:delta.x, delta.y==0?0.0:delta.y, delta.z==0?3.0:delta.z);
            Vec3 f = new Vec3(3.0, 70.0, -11.0);
            Vec3 t = f.add(delta);
            AABB b = new AABB(2.7, 70.0, -11.3, 3.3, 71.8, -10.7);
            run("F6-sign-zero-integer", f, t, b, 11000 + m);
        }
        // ---- F7: octant marches from aligned + fractional starts ----
        for (int dx = -1; dx <= 1; dx++) for (int dy = -1; dy <= 1; dy++) for (int dz = -1; dz <= 1; dz++) {
            if (dx==0 && dy==0 && dz==0) continue;
            for (double[] st : new double[][]{{0.0,0.0,0.0},{0.37,0.62,0.11},{0.5,0.5,0.5}}) {
                Vec3 f = new Vec3(1.0+st[0], 64.0+st[1], -3.0+st[2]);
                Vec3 t = new Vec3(f.x + dx*5.5, f.y + dy*3.25, f.z + dz*7.75);
                AABB b = new AABB(0.7, 64.0, -3.3, 1.3, 65.8, -2.7);
                run("F7-march-aligned", f, t, b, 12000 + dx*9 + dy*3 + dz);
            }
        }
        // ---- F8: random envelope (DDA-1 bulk) ----
        int N8 = args.length > 0 ? Integer.parseInt(args[0]) : 200000;
        Random rnd = new Random(0xC0FFEE);
        for (int i = 0; i < N8; i++) {
            double fx = (rnd.nextDouble() - 0.5) * 60.0;
            double fy = 20.0 + rnd.nextDouble() * 40.0;
            double fz = (rnd.nextDouble() - 0.5) * 60.0;
            Vec3 f = new Vec3(fx, fy, fz);
            double mag = rnd.nextDouble() < 0.85 ? rnd.nextDouble() * 3.0 : rnd.nextDouble() * 12.0;
            Vec3 t = f.add(new Vec3((rnd.nextDouble()*2-1)*mag, (rnd.nextDouble()*2-1)*mag*0.5, (rnd.nextDouble()*2-1)*mag));
            double w = 0.3 + rnd.nextDouble() * 2.7, h = 0.3 + rnd.nextDouble() * 2.7;
            AABB b = new AABB(t.x - w/2, t.y, t.z - w/2, t.x + w/2, t.y + h, t.z + w/2);
            if (!scenario(f, t, b, rnd.nextLong())) { record("F8-random");
                if (famCount[famIdx("F8-random")] > 5) break; }
        }
        int fail = 0;
        for (int i = 0; i < famN; i++) {
            System.out.println(famName[i] + ": mismatches=" + famCount[i]);
            fail += famCount[i];
        }
        System.out.println("TOTAL mismatches across families: " + fail);
        System.out.println(fail == 0 ? "DDA-PROBE ALL-PASS" : "DDA-PROBE FAIL");
        System.exit(fail == 0 ? 0 : 2);
    }
}
