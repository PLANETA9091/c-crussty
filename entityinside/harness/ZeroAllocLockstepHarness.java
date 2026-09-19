package net.minecraft.world.level;

import net.minecraft.world.entity.EntityDimensions;
import net.minecraft.core.Direction;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

import java.io.IOException;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * ZERO-ALLOC-INSIDE lockstep oracle (S7-164, lever #10).
 *
 * OFFLINE bit-exact comparison between the scalar ZeroAllocOps tails and
 * the REAL vanilla kernel classes (AABB / EntityDimensions / Vec3), plus a
 * HotSpot verification of the fully redirected Entity.class produced by
 * the classfile body-redirect (defineClass = full bytecode verification:
 * typed loads, max_stack/max_locals, attribute lengths, stack shapes).
 *
 * Scenarios:
 *   1. entityBoxScalars  vs EntityDimensions.makeBoundingBox(Vec3) —
 *      FLOAT half-width division + FLOAT height (javap verbatim), 50k
 *      random sizes/positions.
 *   2. collidedAlongVectorScalars vs AABB.collidedAlongVector(Vec3,List)
 *      — the full loop: getCenter/add/inflate/contains/clip ladder, 300k
 *      random + degenerate scenarios (zero deltas, empty lists, disjoint,
 *      touching, containing boxes).
 * Any mismatch = FAIL (exit 2), printed with the exact scenario.
 *
 * INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
 */
public final class ZeroAllocLockstepHarness {
    public static void main(String[] args) throws Exception {
        // ---- Step 0: define the redirected Entity (full verifier pass) ----
        Path patched = Path.of("entityinside/build/Entity_patched.class");
        if (!Files.exists(patched)) {
            System.err.println("FAIL: entityinside/build/Entity_patched.class missing — run cargo test with CRUSSTY_EMIT_PATCHED_ENTITY");
            System.exit(2);
        }
        byte[] entityBytes = Files.readAllBytes(patched);
        ClassLoader parent = ZeroAllocLockstepHarness.class.getClassLoader();
        ClassLoader entityLoader = new ClassLoader(parent) {
            @Override
            protected Class<?> findClass(String name) throws ClassNotFoundException {
                if (name.equals("net.minecraft.world.entity.Entity")) {
                    return defineClass(name, entityBytes, 0, entityBytes.length);
                }
                return super.loadClass(name, true);
            }
        };
        Class<?> patchedEntity = Class.forName("net.minecraft.world.entity.Entity", false, entityLoader);
        System.out.println("lockstep: redirected Entity verified by HotSpot (major-loadable, " + entityBytes.length + " bytes)");

        // Sanity: the three redirected bodies must still exist as public
        // methods with their vanilla names (the verifier already guarantees
        // the dispatch shape). Method-object lookup without triggering any
        // registry bootstrap (no parameter-class initialization here).
        java.util.Set<String> present = new java.util.HashSet<>();
        for (Method m : patchedEntity.getMethods()) {
            present.add(m.getName());
        }
        for (String expected : new String[] {
            "collidedWithFluid", "collidedWithShapeMovingFrom", "updateFluidHeightAndDoFluidPushing"
        }) {
            if (!present.contains(expected)) {
                System.err.println("FAIL: redirected method missing: " + expected);
                System.exit(2);
            }
        }
        System.out.println("lockstep: redirected methods present (collidedWithFluid, collidedWithShapeMovingFrom, updateFluidHeightAndDoFluidPushing)");

        if (args.length > 0 && args[0].equals("repro")) {
            reproMode(Long.parseLong(args[1]));
            return;
        }

        Random rng = new Random(42);
        long mismatches = 0;

        // ---- 1. entityBoxScalars vs vanilla EntityDimensions.makeBoundingBox ----
        int boxScenarios = 50_000;
        for (int i = 0; i < boxScenarios; i++) {
            float w = (float) (rng.nextDouble() * 4.0) + 0.05f;
            float h = (float) (rng.nextDouble() * 4.0) + 0.05f;
            Vec3 from = new Vec3((rng.nextDouble() - 0.5) * 64.0,
                rng.nextDouble() * 320.0,
                (rng.nextDouble() - 0.5) * 64.0);
            double[] ours = ZeroAllocOps.entityBoxScalars(w, h, from);
            AABB vanilla = EntityDimensions.scalable(w, h).makeBoundingBox(from);
            if (!sameBox(ours, vanilla)) {
                mismatches++;
                System.err.printf("MAKEBOX MISMATCH i=%d w=%s h=%s from=%s ours=%s vanilla=%s%n",
                    i, w, h, from, java.util.Arrays.toString(ours), vanilla);
                if (mismatches > 10) System.exit(2);
            }
        }
        System.out.println("lockstep: makeBox scenarios=" + boxScenarios + " done");

        // ---- 2. collidedAlongVectorScalars vs vanilla AABB.collidedAlongVector ----
        int avScenarios = 300_000;
        for (int i = 0; i < avScenarios; i++) {
            double xmin = (rng.nextDouble() - 0.5) * 16.0;
            double ymin = (rng.nextDouble() - 0.5) * 16.0;
            double zmin = (rng.nextDouble() - 0.5) * 16.0;
            double sx = 0.05 + rng.nextDouble() * 2.0;
            double sy = 0.05 + rng.nextDouble() * 2.0;
            double sz = 0.05 + rng.nextDouble() * 2.0;
            double xmax = xmin + sx, ymax = ymin + sy, zmax = zmin + sz;

            double dx, dy, dz;
            int mode = rng.nextInt(20);
            if (mode == 0) { dx = 0; dy = 0; dz = 0; }                 // stationary
            else if (mode == 1) { dx = 1e-9; dy = 0; dz = 0; }         // sub-eps delta
            else if (mode == 2) { dx = 0; dy = rng.nextDouble() * 2 - 1; dz = 0; }
            else {
                dx = (rng.nextDouble() - 0.5) * 4.0;
                dy = (rng.nextDouble() - 0.5) * 4.0;
                dz = (rng.nextDouble() - 0.5) * 4.0;
            }

            int boxCount = rng.nextInt(7); // 0..6
            List<AABB> boxes = new ArrayList<>(boxCount);
            for (int b = 0; b < boxCount; b++) {
                double bx, by, bz, bw, bh, bd;
                int kind = rng.nextInt(4);
                if (kind == 0) {
                    // box straddling the entity box (likely contains/clip hit)
                    bx = xmin + rng.nextDouble() * sx - 0.5;
                    by = ymin + rng.nextDouble() * sy - 0.5;
                    bz = zmin + rng.nextDouble() * sz - 0.5;
                    bw = 0.3 + rng.nextDouble();
                    bh = 0.3 + rng.nextDouble();
                    bd = 0.3 + rng.nextDouble();
                } else if (kind == 1) {
                    // degenerate/thin box
                    bx = (rng.nextDouble() - 0.5) * 16.0;
                    by = (rng.nextDouble() - 0.5) * 16.0;
                    bz = (rng.nextDouble() - 0.5) * 16.0;
                    bw = rng.nextDouble() * 0.01;
                    bh = rng.nextDouble() * 0.01;
                    bd = rng.nextDouble() * 0.01;
                } else if (kind == 2) {
                    // far away box
                    bx = 100.0 + rng.nextDouble() * 100.0;
                    by = 100.0 + rng.nextDouble() * 100.0;
                    bz = 100.0 + rng.nextDouble() * 100.0;
                    bw = 1.0;
                    bh = 1.0;
                    bd = 1.0;
                } else {
                    bx = (rng.nextDouble() - 0.5) * 16.0;
                    by = (rng.nextDouble() - 0.5) * 16.0;
                    bz = (rng.nextDouble() - 0.5) * 16.0;
                    bw = rng.nextDouble() * 2.0;
                    bh = rng.nextDouble() * 2.0;
                    bd = rng.nextDouble() * 2.0;
                }
                boxes.add(new AABB(bx, by, bz, bx + bw, by + bh, bz + bd));
            }

            AABB entityBox = new AABB(xmin, ymin, zmin, xmax, ymax, zmax);
            boolean vanilla = entityBox.collidedAlongVector(new Vec3(dx, dy, dz), boxes);
            boolean ours = ZeroAllocOps.collidedAlongVectorScalars(
                xmin, ymin, zmin, xmax, ymax, zmax, dx, dy, dz, boxes);
            if (vanilla != ours) {
                mismatches++;
                System.err.printf("COLLIDED-ALONG MISMATCH i=%d box=[%s..%s] delta=(%s,%s,%s) vanilla=%s ours=%s%n",
                    i, entityBox, entityBox, dx, dy, dz, vanilla, ours);
                if (mismatches > 10) System.exit(2);
            }
        }
        System.out.println("lockstep: collidedAlongVector scenarios=" + avScenarios + " done");

        if (mismatches != 0) {
            System.err.println("ZEROALLOC LOCKSTEP FAIL mismatches=" + mismatches);
            System.exit(2);
        }
        System.out.println("ZEROALLOC LOCKSTEP PASS scenarios=" + (boxScenarios + avScenarios));
    }


    /** Deterministic scenario generator shared with the main loop. */
    private static double[] genScenario(Random rng) {
        double xmin = (rng.nextDouble() - 0.5) * 16.0;
        double ymin = (rng.nextDouble() - 0.5) * 16.0;
        double zmin = (rng.nextDouble() - 0.5) * 16.0;
        double sx = 0.05 + rng.nextDouble() * 2.0;
        double sy = 0.05 + rng.nextDouble() * 2.0;
        double sz = 0.05 + rng.nextDouble() * 2.0;
        return new double[] {xmin, ymin, zmin, xmin + sx, ymin + sy, zmin + sz};
    }

    private static void reproMode(long target) throws Exception {
        Random rng = new Random(42);
        // warm-up: the main loop consumed makeBox randomness first
        // (50k scenarios x 5 nextDouble) before the collidedAlongVector loop.
        for (int i = 0; i < 50_000; i++) {
            rng.nextDouble(); rng.nextDouble(); rng.nextDouble(); rng.nextDouble(); rng.nextDouble();
        }
        for (long i = 0; i <= target; i++) {
            double[] eb = genScenario(rng);
            double xmin = eb[0], ymin = eb[1], zmin = eb[2], xmax = eb[3], ymax = eb[4], zmax = eb[5];
            double dx, dy, dz;
            int mode = rng.nextInt(20);
            if (mode == 0) { dx = 0; dy = 0; dz = 0; }
            else if (mode == 1) { dx = 1e-9; dy = 0; dz = 0; }
            else if (mode == 2) { dx = 0; dy = rng.nextDouble() * 2 - 1; dz = 0; }
            else {
                dx = (rng.nextDouble() - 0.5) * 4.0;
                dy = (rng.nextDouble() - 0.5) * 4.0;
                dz = (rng.nextDouble() - 0.5) * 4.0;
            }
            int boxCount = rng.nextInt(7);
            List<AABB> boxes = new ArrayList<>(boxCount);
            for (int b = 0; b < boxCount; b++) {
                int kind = rng.nextInt(4);
                double bx, by, bz, bw, bh, bd;
                if (kind == 0) {
                    bx = xmin + rng.nextDouble() * (xmax - xmin) - 0.5;
                    by = ymin + rng.nextDouble() * (ymax - ymin) - 0.5;
                    bz = zmin + rng.nextDouble() * (zmax - zmin) - 0.5;
                    bw = 0.3 + rng.nextDouble(); bh = 0.3 + rng.nextDouble(); bd = 0.3 + rng.nextDouble();
                } else if (kind == 1) {
                    bx = (rng.nextDouble() - 0.5) * 16.0; by = (rng.nextDouble() - 0.5) * 16.0; bz = (rng.nextDouble() - 0.5) * 16.0;
                    bw = rng.nextDouble() * 0.01; bh = rng.nextDouble() * 0.01; bd = rng.nextDouble() * 0.01;
                } else if (kind == 2) {
                    bx = 100.0 + rng.nextDouble() * 100.0; by = 100.0 + rng.nextDouble() * 100.0; bz = 100.0 + rng.nextDouble() * 100.0;
                    bw = 1.0; bh = 1.0; bd = 1.0;
                } else {
                    bx = (rng.nextDouble() - 0.5) * 16.0; by = (rng.nextDouble() - 0.5) * 16.0; bz = (rng.nextDouble() - 0.5) * 16.0;
                    bw = rng.nextDouble() * 2.0; bh = rng.nextDouble() * 2.0; bd = rng.nextDouble() * 2.0;
                }
                boxes.add(new AABB(bx, by, bz, bx + bw, by + bh, bz + bd));
            }
            if (i != target) continue;

            System.out.println("REPRO i=" + i + " entity=[" + xmin + "," + ymin + "," + zmin + " -> " + xmax + "," + ymax + "," + zmax + "] delta=(" + dx + "," + dy + "," + dz + ") boxes=" + boxes.size());
            AABB entityBox = new AABB(xmin, ymin, zmin, xmax, ymax, zmax);
            System.out.println("vanilla=" + entityBox.collidedAlongVector(new Vec3(dx, dy, dz), boxes));
            System.out.println("ours=" + ZeroAllocOps.collidedAlongVectorScalars(xmin, ymin, zmin, xmax, ymax, zmax, dx, dy, dz, boxes));
            // step-by-step trace per box
            double cx = net.minecraft.util.Mth.lerp(0.5, xmin, xmax);
            double cy = net.minecraft.util.Mth.lerp(0.5, ymin, ymax);
            double cz = net.minecraft.util.Mth.lerp(0.5, zmin, zmax);
            double fx = cx + dx, fy = cy + dy, fz = cz + dz;
            System.out.println("center=(" + cx + "," + cy + "," + cz + ") far=(" + fx + "," + fy + "," + fz + ")");
            double sx = (xmax - xmin) * 0.5 - 1.0E-7;
            double sy = (ymax - ymin) * 0.5 - 1.0E-7;
            double sz = (zmax - zmin) * 0.5 - 1.0E-7;
            for (int b = 0; b < boxes.size(); b++) {
                AABB box = boxes.get(b);
                AABB inflatedVan = box.inflate(sx, sy, sz);
                double ixmin = box.minX - sx, iymin = box.minY - sy, izmin = box.minZ - sz;
                double ixmax = box.maxX + sx, iymax = box.maxY + sy, izmax = box.maxZ + sz;
                System.out.println(" box" + b + " inflatedVanilla=" + inflatedVan);
                System.out.println("   ours=[(" + ixmin + "," + iymin + "," + izmin + ")->(" + ixmax + "," + iymax + "," + izmax + ")]");
                boolean cvFar = inflatedVan.contains(new Vec3(fx, fy, fz));
                boolean cvCen = inflatedVan.contains(new Vec3(cx, cy, cz));
                System.out.println("   vanillaContains far=" + cvFar + " center=" + cvCen);
                java.util.Optional<Vec3> clipV = inflatedVan.clip(new Vec3(cx, cy, cz), new Vec3(fx, fy, fz));
                System.out.println("   vanillaClip=" + clipV);
                // our ladder trace (inlined copy with per-axis prints)
                double[] bounds = {1.0};
                Direction dir = null;
                if (dx > 1.0E-7) dir = traceClip(bounds, dir, dx, dy, dz, ixmin, iymin, iymax, izmin, izmax, Direction.WEST, cx, cy, cz, "X+");
                else if (dx < -1.0E-7) dir = traceClip(bounds, dir, dx, dy, dz, ixmax, iymin, iymax, izmin, izmax, Direction.EAST, cx, cy, cz, "X-");
                if (dy > 1.0E-7) dir = traceClip(bounds, dir, dy, dz, dx, iymin, izmin, izmax, ixmin, ixmax, Direction.DOWN, cy, cz, cx, "Y+");
                else if (dy < -1.0E-7) dir = traceClip(bounds, dir, dy, dz, dx, iymax, izmin, izmax, ixmin, ixmax, Direction.UP, cy, cz, cx, "Y-");
                if (dz > 1.0E-7) dir = traceClip(bounds, dir, dz, dx, dy, izmin, ixmin, ixmax, iymin, iymax, Direction.NORTH, cz, cx, cy, "Z+");
                else if (dz < -1.0E-7) dir = traceClip(bounds, dir, dz, dx, dy, izmax, ixmin, ixmax, iymin, iymax, Direction.SOUTH, cz, cx, cy, "Z-");
                System.out.println("   ourClip dir=" + dir + " bestT=" + bounds[0]);
            }
        }
    }

    private static Direction traceClip(double[] bounds, Direction dir,
        double d, double e, double f,
        double g, double h, double i2, double j, double k,
        Direction face, double m, double n, double o, String axis) {
        double t = (g - m) / d;
        double u = n + t * e;
        double v = o + t * f;
        boolean r1 = 0.0 >= t, r2 = t >= bounds[0], r3 = (h - 1.0E-7 >= u), r4 = (u >= i2 + 1.0E-7), r5 = (j - 1.0E-7 >= v), r6 = (v >= k + 1.0E-7);
        System.out.printf("   [%s] plane=%s t=%s u=%s v=%s checks: t<=0=%s t>=b=%s uLow=%s uHigh=%s vLow=%s vHigh=%s%n",
            axis, g, t, u, v, r1, r2, r3, r4, r5, r6);
        if (r1) return dir;
        if (r2) return dir;
        if (r3) return dir;
        if (r4) return dir;
        if (r5) return dir;
        if (r6) return dir;
        bounds[0] = t;
        return face;
    }
    private static boolean sameBox(double[] ours, AABB vanilla) {
        if (ours == null || ours.length != 6) return false;
        return Double.doubleToLongBits(ours[0]) == Double.doubleToLongBits(vanilla.minX)
            && Double.doubleToLongBits(ours[1]) == Double.doubleToLongBits(vanilla.minY)
            && Double.doubleToLongBits(ours[2]) == Double.doubleToLongBits(vanilla.minZ)
            && Double.doubleToLongBits(ours[3]) == Double.doubleToLongBits(vanilla.maxX)
            && Double.doubleToLongBits(ours[4]) == Double.doubleToLongBits(vanilla.maxY)
            && Double.doubleToLongBits(ours[5]) == Double.doubleToLongBits(vanilla.maxZ);
    }
}
