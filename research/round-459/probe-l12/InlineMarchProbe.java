import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.Set;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** DDA-1 root-cause probe #2 (TASK-459-L12): vanilla addCollisionsAlongTravel
 *  (reflection, truth) vs inline-clip transcription variants (naive slab,
 *  Y-first near-vanilla) — measures divergence prevalence per family. */
public final class InlineMarchProbe {
    static Method vanDDA;
    static final class Rec implements BlockGetter.BlockStepVisitor {
        final List<String> rows = new ArrayList<>();
        public boolean visit(BlockPos p, int s) { rows.add(p.asLong() + "#" + s); return true; }
    }
    // ---------- inline clip variants ----------
    /** V1: naive slab method (typical "clean" reimplementation). */
    static java.util.Optional<Vec3> clipNaive(double x0, double y0, double z0,
            double x1, double y1, double z1, Vec3 from, Vec3 to) {
        double dx = to.x - from.x, dy = to.y - from.y, dz = to.z - from.z;
        double tmin = 0.0, tmax = 1.0;
        double[] t = new double[1];
        t[0] = Double.NaN;
        boolean hit = false;
        double tEnter = -1;
        // X slab
        if (Math.abs(dx) < 1.0E-7) {
            if (from.x < x0 || from.x > x1) return java.util.Optional.empty();
        } else {
            double ta = (x0 - from.x) / dx, tb = (x1 - from.x) / dx;
            if (ta > tb) { double tmp = ta; ta = tb; tb = tmp; }
            tEnter = Math.max(tEnter, ta); tmax = Math.min(tmax, tb);
            hit = true;
        }
        double tEnter2 = tEnter;
        if (Math.abs(dy) < 1.0E-7) {
            if (from.y < y0 || from.y > y1) return java.util.Optional.empty();
        } else {
            double ta = (y0 - from.y) / dy, tb = (y1 - from.y) / dy;
            if (ta > tb) { double tmp = ta; ta = tb; tb = tmp; }
            tEnter2 = Math.max(tEnter2, ta); tmax = Math.min(tmax, tb);
            hit = true;
        }
        if (Math.abs(dz) < 1.0E-7) {
            if (from.z < z0 || from.z > z1) return java.util.Optional.empty();
        } else {
            double ta = (z0 - from.z) / dz, tb = (z1 - from.z) / dz;
            if (ta > tb) { double tmp = ta; ta = tb; tb = tmp; }
            tEnter2 = Math.max(tEnter2, ta); tmax = Math.min(tmax, tb);
            hit = true;
        }
        if (!hit || tEnter2 > tmax || tEnter2 < 0.0 || tEnter2 > 1.0) return java.util.Optional.empty();
        return java.util.Optional.of(new Vec3(from.x + tEnter2 * dx, from.y + tEnter2 * dy, from.z + tEnter2 * dz));
    }
    /** V2: near-vanilla formulas but axes processed Y-first with slightly
     *  reordered guard comparisons (a natural transcription choice). */
    static boolean SLIP = true; // V2 slip on; V3 = same port with slip fixed
    static java.util.Optional<Vec3> clipYFirst(double x0, double y0, double z0,
            double x1, double y1, double z1, Vec3 from, Vec3 to) {
        double[] tDown = {1.0};
        double dx = to.x - from.x, dy = to.y - from.y, dz = to.z - from.z;
        Direction d = Direction.DOWN;
        // Y block (first, unlike vanilla which starts with X)
        if (dy > 1.0E-7) {
            d = clipPointY(tDown, d, dy, dz, dx, y0, z1, x1, y1, x0, z0, from.y, from.z, from.x);
        } else if (dy < -1.0E-7) {
            d = clipPointY(tDown, d, dy, dz, dx, y1, z1, x1, y0, x0, z0, from.y, from.z, from.x);
        }
        // X block
        if (dx > 1.0E-7) {
            d = clipPointX(tDown, d, dx, dy, dz, x0, y0, z1, y1, z0, x1, from.x, from.y, from.z);
        } else if (dx < -1.0E-7) {
            d = clipPointX(tDown, d, dx, dy, dz, x1, y0, z1, y0, z0, x0, from.x, from.y, from.z);
        }
        // Z block
        if (dz > 1.0E-7) {
            d = clipPointZ(tDown, d, dz, dx, dy, z0, x0, y1, x1, y0, z1, from.z, from.x, from.y);
        } else if (dz < -1.0E-7) {
            d = clipPointZ(tDown, d, dz, dx, dy, z1, x0, y1, x1, y0, z0, from.z, from.x, from.y);
        }
        if (d == null) return java.util.Optional.empty();
        double t = tDown[0];
        return java.util.Optional.of(new Vec3(from.x + t * dx, from.y + t * dy, from.z + t * dz));
    }
    static Direction clipPointY(double[] tDown, Direction fallback, double dy, double dz, double dx,
            double plane, double z1, double x1, double y1, double x0, double z0,
            double py, double pz, double px) {
        double t = (plane - py) / dy;
        double z = pz + t * dz, x = px + t * dx;
        if (SLIP) { x = x + (t * dx - (py + t * dx)) * 0.0 + (px + t * dx - x); } // shifted calc (transcription slip)
        if (x >= x0 - 1.0E-7 && x <= x1 + 1.0E-7 && z >= z0 - 1.0E-7 && z <= z1 + 1.0E-7) {
            if (t >= 0.0 && t < tDown[0]) { tDown[0] = t; return fallback; }
        }
        return fallback;
    }
    static Direction clipPointX(double[] tDown, Direction fallback, double dx, double dy, double dz,
            double plane, double y0, double z1, double y1, double z0, double x1,
            double px, double py, double pz) {
        double t = (plane - px) / dx;
        double y = py + t * dy, z = pz + t * dz;
        if (y >= y0 - 1.0E-7 && y <= y1 + 1.0E-7 && z >= z0 - 1.0E-7 && z <= z1 + 1.0E-7) {
            if (t >= 0.0 && t < tDown[0]) { tDown[0] = t; return fallback; }
        }
        return fallback;
    }
    static Direction clipPointZ(double[] tDown, Direction fallback, double dz, double dx, double dy,
            double plane, double x0, double y1, double x1, double y0, double z1,
            double pz, double px, double py) {
        double t = (plane - pz) / dz;
        double x = px + t * dx, y = py + t * dy;
        if (x >= x0 - 1.0E-7 && x <= x1 + 1.0E-7 && y >= y0 - 1.0E-7 && y <= y1 + 1.0E-7) {
            if (t >= 0.0 && t < tDown[0]) { tDown[0] = t; return fallback; }
        }
        return fallback;
    }
    // ---------- DDA march (bit-exact formulas from javap, pluggable clip) ----------
    static List<String> march(Vec3 delta, AABB box, int clipMode) {
        List<String> rows = new ArrayList<>();
        Set<Long> visited = new java.util.HashSet<>();
        double sizeX = box.getXsize(), sizeY = box.getYsize(), sizeZ = box.getZsize();
        int sx = delta.x >= 0 ? 1 : -1, sy = delta.y >= 0 ? 1 : -1, sz = delta.z >= 0 ? 1 : -1;
        double ax = Math.abs(delta.x), ay = Math.abs(delta.y), az = Math.abs(delta.z);
        int fX, fY, fZ;
        if (ax <= ay && ax <= az) { fX = -sx; fY = -sz; fZ = sy; }
        else if (ay <= az)        { fX = sz;  fY = -sy; fZ = -sx; }
        else                      { fX = -sy; fY = sx;  fZ = -sz; }
        double cX = Mth.lerp(0.5, box.minX, box.maxX);
        double cY = Mth.lerp(0.5, box.minY, box.maxY);
        double cZ = Mth.lerp(0.5, box.minZ, box.maxZ);
        Vec3 outer = new Vec3(cX + sizeX * 0.5 * (double) fX, cY + sizeY * 0.5 * (double) fY, cZ + sizeZ * 0.5 * (double) fZ);
        Vec3 travel = outer.subtract(delta);
        int bx = Mth.floor(travel.x), by = Mth.floor(travel.y), bz = Mth.floor(travel.z);
        int sX = Mth.sign(delta.x), sY = Mth.sign(delta.y), sZ = Mth.sign(delta.z);
        double stX = sX == 0 ? Double.MAX_VALUE : (double) sX / delta.x;
        double stY = sY == 0 ? Double.MAX_VALUE : (double) sY / delta.y;
        double stZ = sZ == 0 ? Double.MAX_VALUE : (double) sZ / delta.z;
        double tmX = stX * (sX > 0 ? 1.0 - Mth.frac(travel.x) : Mth.frac(travel.x));
        double tmY = stY * (sY > 0 ? 1.0 - Mth.frac(travel.y) : Mth.frac(travel.y));
        double tmZ = stZ * (sZ > 0 ? 1.0 - Mth.frac(travel.z) : Mth.frac(travel.z));
        int steps = 0, guard = 0;
        while (tmX <= 1.0 || tmY <= 1.0 || tmZ <= 1.0) {
            if (tmX < tmY) { if (tmX < tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; } }
            else { if (tmY < tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; } }
            java.util.Optional<Vec3> hit = clipMode == 0
                    ? AABB.clip((double) bx, (double) by, (double) bz, (double) (bx + 1), (double) (by + 1), (double) (bz + 1), travel, outer)
                    : clipMode == 1
                    ? clipNaive(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, travel, outer)
                    : clipYFirst(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, travel, outer);
            if (hit.isEmpty()) { if (++guard > 400) break; continue; }
            steps++;
            Vec3 h = hit.get();
            double clX = Mth.clamp(h.x, (double) ((float) bx + 1.0E-5f), (double) bx + 1.0 - 9.999999747378752E-6);
            double clY = Mth.clamp(h.y, (double) ((float) by + 1.0E-5f), (double) by + 1.0 - 9.999999747378752E-6);
            double clZ = Mth.clamp(h.z, (double) ((float) bz + 1.0E-5f), (double) bz + 1.0 - 9.999999747378752E-6);
            int oX = Mth.floor(clX - sizeX * (double) fX);
            int oY = Mth.floor(clY - sizeY * (double) fY);
            int oZ = Mth.floor(clZ - sizeZ * (double) fZ);
            corners(bx, by, bz, oX, oY, oZ, delta, visited, steps, rows);
            if (++guard > 400) break;
        }
        return rows;
    }
    static void corners(int x0, int y0, int z0, int x1, int y1, int z1,
                        Vec3 delta, Set<Long> visited, int step, List<String> rows) {
        int minX = Math.min(x0, x1), minY = Math.min(y0, y1), minZ = Math.min(z0, z1);
        int maxX = Math.max(x0, x1), maxY = Math.max(y0, y1), maxZ = Math.max(z0, z1);
        int extX = maxX - minX, extY = maxY - minY, extZ = maxZ - minZ;
        int stX = delta.x >= 0 ? 1 : -1, stY = delta.y >= 0 ? 1 : -1, stZ = delta.z >= 0 ? 1 : -1;
        int startX = delta.x >= 0 ? minX : maxX, startY = delta.y >= 0 ? minY : maxY, startZ = delta.z >= 0 ? minZ : maxZ;
        boolean zx = Math.abs(delta.x) < Math.abs(delta.z);
        int stepMid, stepInner, startMid, startInner, extMid, extInner;
        if (zx) { stepMid = delta.z >= 0 ? 1 : -1; startMid = delta.z >= 0 ? minZ : maxZ; extMid = extZ;
                  stepInner = delta.x >= 0 ? 1 : -1; startInner = delta.x >= 0 ? minX : maxX; extInner = extX; }
        else    { stepMid = delta.x >= 0 ? 1 : -1; startMid = delta.x >= 0 ? minX : maxX; extMid = extX;
                  stepInner = delta.z >= 0 ? 1 : -1; startInner = delta.z >= 0 ? minZ : maxZ; extInner = extZ; }
        for (int f = 0; f <= extY; f++) { int cy = startY + stY * f;
            for (int s = 0; s <= extMid; s++) { int cMid = startMid + stepMid * s;
                for (int t = 0; t <= extInner; t++) { int cInner = startInner + stepInner * t;
                    int px, pz; if (zx) { px = cInner; pz = cMid; } else { px = cMid; pz = cInner; }
                    long key = BlockPos.asLong(px, cy, pz);
                    if (visited.add(key)) rows.add(key + "#" + step);
                } } }
    }
    static int[] famCount = new int[16];
    static String[] famName = new String[16];
    static int famN = 0;
    static int famIdx(String n) { for (int i = 0; i < famN; i++) if (famName[i].equals(n)) return i;
        famName[famN] = n; return famN++; }
    static void check(int clipMode, String fam, Vec3 from, Vec3 to, AABB box, long seed) throws Exception {
        Vec3 delta = to.subtract(from);
        Rec rec = new Rec();
        vanDDA.invoke(null, new it.unimi.dsi.fastutil.longs.LongOpenHashSet(), delta, box, rec);
        List<String> van = rec.rows;
        List<String> mine = march(delta, box, clipMode);
        if (!van.equals(mine)) famCount[famIdx(fam)]++;
    }
    public static void main(String[] args) throws Exception {
        vanDDA = BlockGetter.class.getDeclaredMethod("addCollisionsAlongTravel",
                Class.forName("it.unimi.dsi.fastutil.longs.LongSet"),
                Vec3.class, AABB.class, BlockGetter.BlockStepVisitor.class);
        vanDDA.setAccessible(true);
        int N = args.length > 0 ? Integer.parseInt(args[0]) : 350000;
        for (int mode = 1; mode <= 3; mode++) { SLIP = mode != 3;
            famN = 0;
            // F1 exact-corner
            for (int a = 0; a < 40; a++) {
                double x0 = (a % 5) - 2.0, y0 = 64.0 + (a / 5) * 1.0, z0 = -3.0 + (a % 3);
                for (double[] d : new double[][]{{2,-2,2},{3,-3,3},{4,-2,2},{2,-4,4},{6,-2,4},{2,-2,6},{3,3,-3},{2,2,-2}}) {
                    Vec3 f = new Vec3(x0, y0, z0);
                    Vec3 t = new Vec3(x0 + d[0], y0 + d[1], z0 + d[2]);
                    AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
                    check(mode, "F1-corner-exact", f, t, b, 7000 + a);
                }
            }
            // F2 axis-edge
            for (int a = 0; a < 40; a++) {
                double y = 64.0 + (a % 4), z = -3.0 + (a / 4);
                Vec3 f = new Vec3(1.0, y, z);
                Vec3 t = new Vec3(3.0 + (a % 5), y, z);
                AABB b = new AABB(0.7, y, z - 0.3, 1.3, y + 1.8, z + 0.3);
                check(mode, "F2-axis-edge", f, t, b, 8000 + a);
            }
            // F3 tMax ties
            for (int a = 0; a < 40; a++) {
                double x0 = (a % 5) - 2.0 + 0.37, y0 = 64.62, z0 = -3.11;
                double m = 1.0 + (a % 6) * 0.5;
                for (double[] d : new double[][]{{m,0,m},{m,0,-m},{-m,m,m},{m,m,m},{-m,-m,-m},{m,-m,m}}) {
                    Vec3 f = new Vec3(x0, y0, z0);
                    Vec3 t = new Vec3(x0 + d[0], y0 + d[1], z0 + d[2]);
                    AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
                    check(mode, "F3-tmax-tie", f, t, b, 9000 + a);
                }
            }
            // F4 large coords
            for (int a = 0; a < 40; a++) {
                double base = 1.0e5 * (1 + a % 4) * (a % 2 == 0 ? 1 : 3.0);
                if (a % 4 == 3) base = 3.0e7;
                double x0 = base + 0.37, y0 = 64.62, z0 = -base - 3.11;
                Vec3 f = new Vec3(x0, y0, z0);
                Vec3 t = new Vec3(x0 + 2.5, y0 - 1.25, z0 + 4.75);
                AABB b = new AABB(x0 - 0.3, y0, z0 - 0.3, x0 + 0.3, y0 + 1.8, z0 + 0.3);
                check(mode, "F4-large-coords", f, t, b, 10000 + a);
            }
            // F8 random envelope
            Random rnd = new Random(0xC0FFEE);
            for (int i = 0; i < N; i++) {
                double fx = (rnd.nextDouble() - 0.5) * 60.0;
                double fy = 20.0 + rnd.nextDouble() * 40.0;
                double fz = (rnd.nextDouble() - 0.5) * 60.0;
                Vec3 f = new Vec3(fx, fy, fz);
                double mag = rnd.nextDouble() < 0.85 ? rnd.nextDouble() * 3.0 : rnd.nextDouble() * 12.0;
                Vec3 t = f.add(new Vec3((rnd.nextDouble()*2-1)*mag, (rnd.nextDouble()*2-1)*mag*0.5, (rnd.nextDouble()*2-1)*mag));
                double w = 0.3 + rnd.nextDouble() * 2.7, h = 0.3 + rnd.nextDouble() * 2.7;
                AABB b = new AABB(t.x - w/2, t.y, t.z - w/2, t.x + w/2, t.y + h, t.z + w/2);
                check(mode, "F8-random", f, t, b, rnd.nextLong());
            }
            System.out.println("=== inline-clip variant " + (mode == 1 ? "V1-NAIVE-SLAB" : mode == 2 ? "V2-Y-FIRST-SLIP" : "V3-Y-FIRST-CLEAN") + " ===");
            int tot = 0;
            for (int i = 0; i < famN; i++) {
                System.out.println("  " + famName[i] + ": mismatches=" + famCount[i]);
                tot += famCount[i];
            }
            System.out.println("  TOTAL: " + tot + " mismatches (random envelope N=" + N + ")");
        }
    }
}
