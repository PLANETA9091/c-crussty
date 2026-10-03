import net.minecraft.world.phys.AABB;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * TRAVEL-DIET v2a offline lockstep harness (RECON-21, lever #14) — plain
 * JVM, real kernel classes, NO server boot (INJECTS-ONLY discipline).
 *
 * What it proves OFFLINE, bit-exactly:
 *   1. STRUCTURAL — TravelDietOps defines + links against the REAL kernel
 *      jar (CollisionUtil/Level/VoxelShape references resolve; a bad splice
 *      dies here, never on a live boot).
 *   2. EXPAND/MOVE MIRRORS — TravelDietOps.expandInto/moveInto (private
 *      static, reflection) vs the REAL AABB.expandTowards(DDD)/move(DDD)
 *      across 500,000 randomized cases PLUS the corner set (+-0.0, NaN,
 *      +-Inf, 1e-300, 1e300, the exact step-up epsilon -9.999999747378752E-6):
 *      all six fields compared by Double.doubleToRawLongBits.
 *   3. STEP-HEIGHT LOCKSTEP — TravelDietOps.calculateStepHeights (private
 *      static, reflection) vs the REAL private static
 *      Entity.calculateStepHeights across 50,000 randomized scenarios
 *      (random shape lists from Shapes.box + random AABB lists + random
 *      step/curr floats): float[] compared by Float.floatToRawIntBits in
 *      order (the ladder is order-sensitive by vanilla contract).
 *
 * Residue honestly NOT covered offline: the collide body's Level-touching
 * sections (getEntityHardCollisions / getCollisionsForBlocksOrWorldBorder /
 * performCollisions) — identical CALLS with identical arguments in identical
 * order per the javap contract; live parity is enforced by the CI leg
 * fixture gates (median-exact scene).
 *
 * Exit code 0 = PASS; any failure throws (nonzero).
 */
public final class TravelDietLockstepHarness {

    public static void main(String[] args) throws Exception {
        // ---- 1. STRUCTURAL: load the bridge against the real kernel jar
        Class<?> td = Class.forName("net.minecraft.world.entity.TravelDietOps");
        Class.forName("ca.spottedleaf.moonrise.patches.collisions.CollisionUtil");
        Class.forName("ca.spottedleaf.moonrise.patches.collisions.shape.CollisionVoxelShape");
        System.out.println("[1] structural: TravelDietOps + CollisionUtil + CollisionVoxelShape link OK");

        // ---- 2. EXPAND/MOVE mirrors vs REAL AABB
        Method expandInto = td.getDeclaredMethod("expandInto", AABB.class, AABB.class, double.class, double.class, double.class);
        expandInto.setAccessible(true);
        Method moveInto = td.getDeclaredMethod("moveInto", AABB.class, AABB.class, double.class, double.class, double.class);
        moveInto.setAccessible(true);

        Class<?> aabbC = AABB.class;
        Method vExpand = aabbC.getMethod("expandTowards", double.class, double.class, double.class);
        Method vMove = aabbC.getMethod("move", double.class, double.class, double.class);

        Random rnd = new Random(0x7E1DC0DEL);
        double[] corners = {
            0.0D, -0.0D, Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY,
            1.0e-300D, 1.0e300D, -9.999999747378752e-6D, 1.0e-7D, -1.0e-7D,
        };

        long cases = 0;
        for (int i = 0; i < 500_000; ++i) {
            AABB src = randBox(rnd);
            double dx, dy, dz;
            if (i < corners.length * corners.length * 5 && i % 5 == 0) {
                // corner sweeps: mix the exact boundary values
                dx = corners[rnd.nextInt(corners.length)];
                dy = corners[rnd.nextInt(corners.length)];
                dz = corners[rnd.nextInt(corners.length)];
            } else if (rnd.nextInt(8) == 0) {
                // axis-only deltas (the dominant collide shape)
                dx = pickDelta(rnd); dy = 0.0D; dz = 0.0D;
            } else {
                dx = pickDelta(rnd); dy = pickDelta(rnd); dz = pickDelta(rnd);
            }

            AABB want = (AABB) vExpand.invoke(src, dx, dy, dz);
            AABB got = (AABB) expandInto.invoke(null, freshBox(), src, dx, dy, dz);
            assertBoxBits("expandTowards", i, want, got);
            cases++;

            want = (AABB) vMove.invoke(src, dx, dy, dz);
            got = (AABB) moveInto.invoke(null, freshBox(), src, dx, dy, dz);
            assertBoxBits("move", i, want, got);
            cases++;
        }
        System.out.println("[2] expand/move mirrors: " + cases + " randomized cases bit-exact PASS");

        // ---- 3. calculateStepHeights lockstep vs REAL private static
        net.minecraft.SharedConstants.tryDetectVersion(); // purpur ServerBuildInfo needs the game version
        net.minecraft.server.Bootstrap.bootStrap(); // Entity.<clinit> needs registries
        Class<?> entityC = Class.forName("net.minecraft.world.entity.Entity");
        Method realStep = null;
        for (Method m : entityC.getDeclaredMethods()) {
            if (m.getName().equals("calculateStepHeights")
                    && m.getParameterCount() == 5
                    && m.getReturnType() == float[].class) {
                realStep = m;
                break;
            }
        }
        if (realStep == null) throw new IllegalStateException("real calculateStepHeights not found");
        realStep.setAccessible(true);
        Method tdStep = null;
        for (Method m : td.getDeclaredMethods()) {
            if (m.getName().equals("calculateStepHeights")) {
                tdStep = m;
                break;
            }
        }
        tdStep.setAccessible(true);

        int stepCases = 50_000;
        for (int i = 0; i < stepCases; ++i) {
            AABB box = randBox(rnd);
            int nShapes = rnd.nextInt(5);
            List<Object> shapes = new ArrayList<>();
            for (int s = 0; s < nShapes; ++s) {
                shapes.add(net.minecraft.world.phys.shapes.Shapes.box(
                    rnd.nextDouble() * 4.0 - 2.0, rnd.nextDouble() * 4.0, rnd.nextDouble() * 4.0 - 2.0,
                    rnd.nextDouble() * 4.0 + 2.0, rnd.nextDouble() * 4.0 + 4.0, rnd.nextDouble() * 4.0 + 2.0));
            }
            int nBoxes = rnd.nextInt(5);
            List<Object> aabbs = new ArrayList<>();
            for (int b = 0; b < nBoxes; ++b) {
                aabbs.add(randBox(rnd));
            }
            float stepF = randFloat(rnd);
            float currY = randFloat(rnd);

            float[] want = (float[]) realStep.invoke(null, box, shapes, aabbs, stepF, currY);
            float[] got = (float[]) tdStep.invoke(null, box, shapes, aabbs, stepF, currY);
            if (want.length != got.length) {
                throw new AssertionError("step ladder length mismatch @case " + i
                    + ": real=" + want.length + " mirror=" + got.length);
            }
            for (int k = 0; k < want.length; ++k) {
                if (Float.floatToRawIntBits(want[k]) != Float.floatToRawIntBits(got[k])) {
                    throw new AssertionError("step ladder bit mismatch @case " + i + " idx " + k
                        + ": real=" + want[k] + " (" + Float.floatToRawIntBits(want[k])
                        + ") mirror=" + got[k] + " (" + Float.floatToRawIntBits(got[k]) + ")");
                }
            }
        }
        System.out.println("[3] calculateStepHeights lockstep: " + stepCases + " scenarios bit-exact PASS");

        System.out.println("TRAVEL-DIET LOCKSTEP: ALL PASS");
    }

    private static double pickDelta(Random rnd) {
        // movement-shaped deltas: mostly small, occasionally huge/zero
        int r = rnd.nextInt(16);
        if (r == 0) return 0.0D;
        if (r == 1) return rnd.nextGaussian() * 100.0D;
        return rnd.nextGaussian() * 0.6D;
    }

    private static float randFloat(Random rnd) {
        int r = rnd.nextInt(8);
        if (r == 0) return 0.0F;
        if (r == 1) return -0.0F;
        if (r == 2) return Float.NaN;
        return (float) ((rnd.nextDouble() - 0.25) * 4.0);
    }

    private static AABB randBox(Random rnd) {
        double x0 = rnd.nextDouble() * 2000.0 - 1000.0;
        double y0 = rnd.nextDouble() * 320.0;
        double z0 = rnd.nextDouble() * 2000.0 - 1000.0;
        double w = rnd.nextDouble() * 3.0;
        double h = rnd.nextDouble() * 3.0;
        return new AABB(x0, y0, z0, x0 + w, y0 + h, z0 + w);
    }

    private static AABB freshBox() {
        return new AABB(0, 0, 0, 0, 0, 0);
    }

    private static void assertBoxBits(String what, int i, AABB want, AABB got) {
        check(what, i, "minX", want.minX, got.minX);
        check(what, i, "minY", want.minY, got.minY);
        check(what, i, "minZ", want.minZ, got.minZ);
        check(what, i, "maxX", want.maxX, got.maxX);
        check(what, i, "maxY", want.maxY, got.maxY);
        check(what, i, "maxZ", want.maxZ, got.maxZ);
    }

    private static void check(String what, int i, String f, double w, double g) {
        if (Double.doubleToRawLongBits(w) != Double.doubleToRawLongBits(g)) {
            throw new AssertionError(what + " bit mismatch @case " + i + " field " + f
                + ": real=" + w + " (" + Double.doubleToRawLongBits(w)
                + ") mirror=" + g + " (" + Double.doubleToRawLongBits(g) + ")");
        }
    }

    private TravelDietLockstepHarness() {}
}
