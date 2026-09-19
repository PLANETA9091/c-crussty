package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil;
import ca.spottedleaf.moonrise.patches.collisions.shape.CollisionVoxelShape;
import it.unimi.dsi.fastutil.floats.FloatArraySet;
import it.unimi.dsi.fastutil.floats.FloatArrays;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.VoxelShape;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/**
 * TRAVEL-DIET v2a COLLIDE-DIET (RECON-21 contract, lever #14).
 *
 * Scalar (allocation-diet) rewrite of the private Entity.collide(Vec3) body
 * plus the private static Entity.calculateStepHeights — verbatim javap mirror
 * of the kernel jar (contract-traveldiet-s7182b/{collide_body,
 * calc_step_heights}.txt, run-s7178-recal). Every double/float operation,
 * its ORDER, and every branch condition is a verbatim copy; dcmpg/dcmpl and
 * fcmpg/fcmpl ladder semantics are preserved bit-exactly (javac natural
 * comparison polarity matches the contract, corners documented inline).
 *
 * Allocation diet per collide call vs vanilla:
 *   - up to 3 AABB temporaries (expandTowards(Vec3)@126, move(DDD)@333,
 *     expandTowards(DDD)@355 + chained @372) -> thread-local scratch slots
 *     fully overwritten before every use (stack-slot replacement, NOT a
 *     cache — same law as ZeroAllocOps double[1]);
 *   - 4 ArrayList temporaries -> thread-local scratch lists, clear() before
 *     every use, aliasing (list20 == list9) preserved per bytecode;
 *   - FloatArraySet in calculateStepHeights -> thread-local scratch.
 * The RESULT objects (Vec3 from performCollisions, float[] from
 * toFloatArray, the final Vec3.add) are products and stay allocations.
 * cutDownwards/cutUpwards are called as-is (vertical-only branch, static
 * vanilla utilities).
 *
 * Scratch state is NEVER read before being fully written and is never
 * carried between calls: no memoization, median-exact vanilla parity by
 * construction (S7-164 precedent).
 *
 * Delivered into the KERNEL loader (same as ZeroAllocOps/InsideDietOps);
 * the Entity.collide body is redirected here by entity_compose
 * (receiver-prepended invokestatic, same descriptor, length-preserving).
 *
 * NO nested classes (S7-163 lesson): scratch is an Object[] in a
 * ThreadLocal, not a holder class.
 */
public final class TravelDietOps {

    private TravelDietOps() {}

    // ---- scratch slots (single classfile law: Object[], no holder class) ----
    // slots 0..3 = AABB scratch (0: box8, 1: box17, 2: box18, 3: spare),
    // 4..7 = ArrayList scratch (4: list6 shapes, 5: list7 aabbs,
    //         6: list9 hard, 7: list19 shapes), 8 = FloatArraySet scratch.
    private static final int SCRATCH_SIZE = 9;
    // withInitial + non-capturing lambda: NO nested class in the classfile
    // (S7-163 law — the anonymous ThreadLocal subclass detonates as
    // NoClassDefFoundError on first entity tick).
    private static final java.lang.ThreadLocal<Object[]> SCRATCH =
        java.lang.ThreadLocal.withInitial(TravelDietOps::freshScratch);

    private static Object[] freshScratch() {
        Object[] s = new Object[SCRATCH_SIZE];
        for (int i = 0; i < 4; ++i) {
            s[i] = new AABB(0.0D, 0.0D, 0.0D, 0.0D, 0.0D, 0.0D);
        }
        for (int i = 4; i < 8; ++i) {
            s[i] = new ArrayList<Object>();
        }
        s[8] = new FloatArraySet();
        return s;
    }

    // ---- Unsafe offsets (SkipStoreOps precedent: resolved once) ----
    private static final sun.misc.Unsafe UNSAFE;
    private static final long OFF_MINX, OFF_MINY, OFF_MINZ, OFF_MAXX, OFF_MAXY, OFF_MAXZ;

    static {
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            OFF_MINX = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minX"));
            OFF_MINY = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minY"));
            OFF_MINZ = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minZ"));
            OFF_MAXX = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxX"));
            OFF_MAXY = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxY"));
            OFF_MAXZ = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxZ"));
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    private static AABB box(final int slot) {
        return (AABB) SCRATCH.get()[slot];
    }

    @SuppressWarnings("unchecked")
    private static List<Object> list(final int slot) {
        List<Object> l = (List<Object>) SCRATCH.get()[slot];
        l.clear();
        return l;
    }

    private static FloatArraySet fset() {
        FloatArraySet s = (FloatArraySet) SCRATCH.get()[8];
        s.clear();
        return s;
    }

    /**
     * Bit-exact mirror of AABB.expandTowards(DDD) writing into dst:
     * per axis: if (d < 0) min += d; else if (d > 0) max += d
     * (javac `<` -> dcmpg+ifge, `>` -> dcmpl+ifle — the exact contract
     * polarity; NaN falls through with no change, as in vanilla).
     */
    private static AABB expandInto(final AABB dst, final AABB src,
                                   final double dx, final double dy, final double dz) {
        double minX = src.minX;
        double minY = src.minY;
        double minZ = src.minZ;
        double maxX = src.maxX;
        double maxY = src.maxY;
        double maxZ = src.maxZ;
        if (dx < 0.0D) {
            minX += dx;
        } else if (dx > 0.0D) {
            maxX += dx;
        }
        if (dy < 0.0D) {
            minY += dy;
        } else if (dy > 0.0D) {
            maxY += dy;
        }
        if (dz < 0.0D) {
            minZ += dz;
        } else if (dz > 0.0D) {
            maxZ += dz;
        }
        sun.misc.Unsafe u = UNSAFE;
        u.putDouble(dst, OFF_MINX, minX);
        u.putDouble(dst, OFF_MINY, minY);
        u.putDouble(dst, OFF_MINZ, minZ);
        u.putDouble(dst, OFF_MAXX, maxX);
        u.putDouble(dst, OFF_MAXY, maxY);
        u.putDouble(dst, OFF_MAXZ, maxZ);
        return dst;
    }

    /** Bit-exact mirror of AABB.move(DDD) writing into dst (pure adds). */
    private static AABB moveInto(final AABB dst, final AABB src,
                                 final double dx, final double dy, final double dz) {
        sun.misc.Unsafe u = UNSAFE;
        u.putDouble(dst, OFF_MINX, src.minX + dx);
        u.putDouble(dst, OFF_MINY, src.minY + dy);
        u.putDouble(dst, OFF_MINZ, src.minZ + dz);
        u.putDouble(dst, OFF_MAXX, src.maxX + dx);
        u.putDouble(dst, OFF_MAXY, src.maxY + dy);
        u.putDouble(dst, OFF_MAXZ, src.maxZ + dz);
        return dst;
    }

    /**
     * Receiver-prepended scalar mirror of the private Entity.collide(Vec3)
     * body — RECON-21 contract, offsets documented per block.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static Vec3 collide(final Entity self, final Vec3 movement) {
        // off 0..52: zero checks (dcmpl ladder; NaN -> false, as vanilla)
        boolean xZero = movement.x == 0.0D;
        boolean yZero = movement.y == 0.0D;
        boolean zZero = movement.z == 0.0D;
        if (xZero && yZero && zZero) {
            return movement; // off 46..56: iand,iand,ifeq -> areturn
        }
        // off 57..79: bb + two fresh lists
        AABB bb = self.getBoundingBox();
        List<VoxelShape> list6 = (List) list(4);
        List<AABB> list7 = (List) list(5);
        // off 81..129: query box: cut branch (vertical) or expandTowards(Vec3)
        AABB box8;
        if (xZero && zZero) {
            // off 93: dcmpg ifge -> y<0 ? cutDownwards : cutUpwards (javac `<` = dcmpg)
            box8 = movement.y < 0.0D
                ? CollisionUtil.cutDownwards(bb, movement.y)
                : CollisionUtil.cutUpwards(bb, movement.y);
        } else {
            box8 = expandInto(box(0), bb, movement.x, movement.y, movement.z);
        }
        // off 131..157: hard entity collisions
        List<AABB> list9 = (List) list(6);
        CollisionUtil.getEntityHardCollisions((Level) self.level(), self, box8, list9, 0, null);
        // off 158..177: block/worldborder collisions
        CollisionUtil.getCollisionsForBlocksOrWorldBorder(
            (Level) self.level(), self, box8, list6, list7, 4, null);
        // off 178..187: list7.addAll(list9)
        list7.addAll(list9);
        // off 188..198: performCollisions
        Vec3 vec10 = CollisionUtil.performCollisions(movement, bb, list6, list7);
        // off 200..258: dcmpl ladders -> diffs (NaN -> true, as vanilla dcmpl)
        boolean xDiff = vec10.x != movement.x;
        boolean yDiff = vec10.y != movement.y;
        boolean zDiff = vec10.z != movement.z;
        // off 260..279: down = yDiff && movement.y dcmpg 0.0 < 0
        boolean down = yDiff && movement.y < 0.0D;
        // off 281..316: !(down || onGround) -> return; !(xDiff || zDiff) -> return
        if (!(down || self.onGround)) {
            return vec10;
        }
        if (!xDiff && !zDiff) {
            return vec10;
        }
        // off 303..318: step = (double) maxUpStep(); dcmpg ifgt else return
        // (javac `step > 0.0` = dcmpg+ifle -> NaN continues, as vanilla)
        double step = (double) self.maxUpStep();
        if (step > 0.0D) {
            // fall through to step-up ladder
        } else {
            return vec10;
        }
        // off 319..341: down ? bb.move(0, vec10.y, 0) : bb
        AABB box17 = down ? moveInto(box(1), bb, 0.0D, vec10.y, 0.0D) : bb;
        // off 343..358: box18 = box17.expandTowards(mx, step, mz)
        AABB box18 = expandInto(box(2), box17, movement.x, step, movement.z);
        // off 360..375: if (!down) box18 = box18.expandTowards(0, -9.999999747378752E-6, 0)
        if (!down) {
            box18 = expandInto(box(2), box18, 0.0D, -9.999999747378752E-6D, 0.0D);
        }
        // off 377..409: list19 fresh; list20 = list9 (alias); second block query
        List<VoxelShape> list19 = (List) list(7);
        List<AABB> list20 = list9;
        CollisionUtil.getCollisionsForBlocksOrWorldBorder(
            (Level) self.level(), self, box18, list19, list20, 4, null);
        // off 410..428: steps = calculateStepHeights(box17, list19, list20, (f)step, (f)vec10.y)
        float[] steps = calculateStepHeights(box17, list19, list20, (float) step, (float) vec10.y);
        // off 430..517: candidate ladder
        for (int i = 0; i < steps.length; ++i) {
            float h = steps[i];
            Vec3 cand = new Vec3(movement.x, (double) h, movement.z);
            Vec3 res = CollisionUtil.performCollisions(cand, box17, list19, list20);
            if (res.horizontalDistanceSqr() > vec10.horizontalDistanceSqr()) {
                // off 495..513: res.add(0, box17.minY - bb.minY, 0) — product Vec3
                return res.add(0.0D, box17.minY - bb.minY, 0.0D);
            }
        }
        // off 520: return vec10
        return vec10;
    }

    /**
     * Verbatim mirror of the private static Entity.calculateStepHeights —
     * RECON-21 contract (calc_step_heights.txt). FloatArraySet is a
     * thread-local scratch (cleared before every use); the returned float[]
     * and FloatArrays.unstableSort are vanilla as-is.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static float[] calculateStepHeights(final AABB box, final List<VoxelShape> shapes,
                                                final List<AABB> aabbs,
                                                final float stepF, final float currY) {
        FloatArraySet set = fset();
        outer:
        for (int i = 0; i < shapes.size(); ++i) {
            final VoxelShape shape = shapes.get(i);
            // moonrise patch: VoxelShape implements CollisionVoxelShape (javap-verified)
            final CollisionVoxelShape cs = (CollisionVoxelShape) shape;
            final double[] rootY = cs.moonrise$rootCoordinatesY();
            final double offY = cs.moonrise$offsetY();
            for (int j = 0; j < rootY.length; ++j) {
                final double d = rootY[j] + offY;
                final float f = (float) (d - box.minY);
                // fcmpl ifle -> if (f > stepF) continue outer (f3 = stepF)
                if (f > stepF) {
                    continue outer;
                }
                // fcmpg iflt -> skip add when f < 0 (javac `>=` = fcmpg: NaN falls through)
                if (f >= 0.0F) {
                    // fcmpl ifne -> add when f != currY (f4 = currY)
                    if (f != currY) {
                        set.add(f);
                    }
                }
            }
        }
        for (int i = 0; i < aabbs.size(); ++i) {
            final AABB a = aabbs.get(i);
            final float fMin = (float) (a.minY - box.minY);
            final float fMax = (float) (a.maxY - box.minY);
            if (fMin >= 0.0F && fMin != currY && !(fMin > stepF)) {
                set.add(fMin);
            }
            if (fMax >= 0.0F && fMax != currY && !(fMax > stepF)) {
                set.add(fMax);
            }
        }
        final float[] out = set.toFloatArray();
        FloatArrays.unstableSort(out);
        return out;
    }
}
