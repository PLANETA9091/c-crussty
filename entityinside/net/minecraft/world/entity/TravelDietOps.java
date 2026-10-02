package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil;
import ca.spottedleaf.moonrise.patches.collisions.shape.CollisionVoxelShape;
import it.unimi.dsi.fastutil.floats.FloatArraySet;
import it.unimi.dsi.fastutil.floats.FloatArrays;
import net.minecraft.core.Holder;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.ai.attributes.Attribute;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.material.Fluid;
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

    /**
     * TRAVEL-DIET v2b TRAVEL-MATH (RECON-21 contract section 4, lever #14).
     *
     * Verbatim scalar mirror of the protected static Entity.getInputVector
     * (Vec3,float,float) — contract-traveldiet-s7182b Entity.txt @5827,
     * offsets 0-108. The scale() temporary (always) and the normalize()
     * temporary (input lengthSqr > 1.0) are carried as scalar doubles;
     * the returned Vec3 is a PRODUCT and stays an allocation.
     *
     * Byte-exact per javap:
     *   - off 5-16: dcmpg+ifge — return ZERO iff d < 1.0E-7 (NaN falls
     *     through, dcmpg(NaN) = +1);
     *   - off 17-30: dcmpl+ifle — normalize iff d > 1.0 (NaN -> raw input,
     *     dcmpl(NaN) = -1);
     *   - normalize inline (Vec3.txt @188): sqrt(x*x + y*y + z*z), ZERO
     *     fields iff len < 9.999999747378752E-6 (dcmpg+ifge), else x/len;
     *   - off 31-36: scale((double) friction) = x*f, y*f, z*f;
     *   - off 58-105: new Vec3(vx*cos - vz*sin, vy, vz*cos + vx*sin),
     *     dsub/dadd order verbatim, sin/cos of yaw * 0.017453292F.
     * No memoization, no state — median-exact vanilla parity by
     * construction.
     */
    public static Vec3 getInputVector(final Vec3 movementInput, final float friction, final float yaw) {
        // off 0-4: d = movementInput.lengthSqr() (vanilla method call kept)
        final double d = movementInput.lengthSqr();
        // off 5-16: dcmpg ifge -> ZERO only when d < 1.0E-7
        if (d < 1.0E-7D) {
            return Vec3.ZERO;
        }
        final double vx;
        final double vy;
        final double vz;
        if (d > 1.0D) {
            // off 17-30: dcmpl ifle -> normalize branch only when d > 1.0
            // Scalar mirror of Vec3.normalize (Vec3.txt @188, offsets 0-72):
            final double len = Math.sqrt(
                movementInput.x * movementInput.x
                + movementInput.y * movementInput.y
                + movementInput.z * movementInput.z);
            if (len < 9.999999747378752E-6D) {
                // normalize returns Vec3.ZERO -> its fields feed scale()
                vx = 0.0D;
                vy = 0.0D;
                vz = 0.0D;
            } else {
                vx = movementInput.x / len;
                vy = movementInput.y / len;
                vz = movementInput.z / len;
            }
        } else {
            vx = movementInput.x;
            vy = movementInput.y;
            vz = movementInput.z;
        }
        // off 31-36: vec6 = vec5.scale((double) friction) — scalar
        final double sx = vx * (double) friction;
        final double sy = vy * (double) friction;
        final double sz = vz * (double) friction;
        // off 38-56: sin/cos of yaw * 0.017453292F
        final float f6 = Mth.sin(yaw * 0.017453292F);
        final float f7 = Mth.cos(yaw * 0.017453292F);
        // off 58-108: product Vec3 (stays an allocation); dsub/dadd order
        // verbatim: x*cos - z*sin ; y ; z*cos + x*sin
        return new Vec3(
            sx * (double) f7 - sz * (double) f6,
            sy,
            sz * (double) f7 + sx * (double) f6);
    }

    /**
     * TRAVEL-DIET v2b (RECON-21 contract section 4): receiver-prepended
     * verbatim mirror of the private LivingEntity.travelInFluid(Vec3) —
     * contract-traveldiet-s7182b LivingEntity.txt @7977, offsets 0-408.
     *
     * Killed TEMPORARIES (s7177 alloc anchor, water path):
     *   - off 174-194: new Vec3(dm.x, 0.2, dm.z) on
     *     horizontalCollision && onClimbable — consumed by the multiply
     *     below, never escapes -> scalars;
     *   - off 196-207: Vec3.multiply(DDD) slowDown/0.8/slowDown — consumed
     *     by getFluidFallingAdjustedMovement -> scalar input to the
     *     inlined gFFAM ladder below;
     *   - the inlined getFluidFallingAdjustedMovement (public on
     *     LivingEntity, offsets 0-98) materializes exactly ONE product
     *     Vec3 in BOTH branches — vanilla's alias branch (gravity == 0 or
     *     sprinting) re-stores the multiply temporary, the mirror
     *     materializes it instead: alloc-count NEVER worse, -1 Vec3 in
     *     the non-alias branch.
     * The LAVA path (off 228-315) keeps VANILLA Vec3.multiply/scale/
     * getFluidFallingAdjustedMovement calls — their results are STORED via
     * setDeltaMovement (products, not temporaries). move()/moveRelative()
     * stay vanilla (their internals are redirected by their own stages).
     *
     * Float/double op order and every compare polarity are verbatim:
     *   - off 0-17: falling = dm.y <= 0.0 (dcmpg+ifgt, NaN -> false);
     *   - off 85-89: fcmpl+ifle — eff > 0.0F (NaN skips the ladder);
     *   - off 92-119: f += (0.54600006F - f) * eff; g += (g - speed) * eff
     *     (BYTECODE order: fsub pops f8 minus getSpeed() — dump is the
     *     contract, not the decompiled source);
     *   - off 254-259: dcmpg+ifgt — lava multiply branch iff fluidHeight
     *     <= jumpThreshold (NaN -> scale branch);
     *   - off 318-322: dcmpl+ifeq — gravity != 0.0 (NaN executes);
     *   - off 358-385: isFree(x, y + 0.6 - getY() + y0, z) — dadd/dsub/
     *     dadd order verbatim.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static void travelInFluid(final LivingEntity self, final Vec3 travelVector) {
        // off 0-17: boolean flag = this.getDeltaMovement().y <= 0.0
        final boolean falling = self.getDeltaMovement().y <= 0.0D;
        // off 18-22: double d0 = this.getY()
        final double y0 = self.getY();
        // off 23-27: double gravity = this.getEffectiveGravity()
        final double gravity = self.getEffectiveGravity();
        if (self.isInWater()) {
            // ---- WATER PATH, off 29-225 ----
            // off 36-53: f7 = isSprinting ? 0.9F : getWaterSlowDown()
            float slowDown;
            if (self.isSprinting()) {
                slowDown = 0.9F;
            } else {
                slowDown = self.getWaterSlowDown();
            }
            // off 55-58: f8 = 0.02F
            float speed = 0.02F;
            // off 60-68: f9 = (float) getAttributeValue(WATER_MOVEMENT_EFFICIENCY)
            float eff = (float) self.getAttributeValue(
                (Holder<Attribute>) (Holder) Attributes.WATER_MOVEMENT_EFFICIENCY);
            // off 70-83: if (!onGround()) f9 *= 0.5F
            if (!self.onGround()) {
                eff *= 0.5F;
            }
            // off 85-119: fcmpl ifle -> ladder iff eff > 0.0F
            if (eff > 0.0F) {
                // off 92-104: f7 = f7 + (0.54600006F - f7) * f9
                slowDown = slowDown + (0.54600006F - slowDown) * eff;
                // off 106-119: f8 = f8 + (f8 - getSpeed()) * f9 — BYTECODE
                // order (fload 8 first, fsub): dump is the contract
                speed = speed + (speed - self.getSpeed()) * eff;
            }
            // off 121-134: DOLPHINS_GRACE -> f7 = 0.96F
            if (self.hasEffect(MobEffects.DOLPHINS_GRACE)) {
                slowDown = 0.96F;
            }
            // off 136-142: moveRelative(f8, travelVector) — vanilla body
            // (its getInputVector temps die under the static redirect)
            self.moveRelative(speed, travelVector);
            // off 143-151: move(SELF, getDeltaMovement()) — vanilla body
            self.move(MoverType.SELF, self.getDeltaMovement());
            // off 154-158: vec10 = getDeltaMovement()
            final double mx = self.getDeltaMovement().x;
            final double my0 = self.getDeltaMovement().y;
            final double mz = self.getDeltaMovement().z;
            // off 160-194: if (horizontalCollision && onClimbable())
            // vec10 = new Vec3(vec10.x, 0.2, vec10.z) — TEMPORARY killed
            final double my;
            if (self.horizontalCollision && self.onClimbable()) {
                my = 0.2D;
            } else {
                my = my0;
            }
            // off 196-207: vec10 = vec10.multiply((double)f7, 0.8, (double)f7)
            // — TEMPORARY killed (scalars)
            final double bx = mx * (double) slowDown;
            final double by = my * 0.800000011920929D;
            final double bz = mz * (double) slowDown;
            // off 210-222: setDeltaMovement(getFluidFallingAdjustedMovement(
            //   gravity, falling, vec10)) — INLINED scalar ladder:
            // gFFAM offsets 0-98: alias branch (gravity == 0 | sprinting)
            // materializes the input (vanilla re-stored the temp — alloc
            // count identical); compute branch = ONE product Vec3.
            if (gravity != 0.0D && !self.isSprinting()) {
                final double d5;
                if (falling
                    && Math.abs(by - 0.005D) >= 0.003D
                    && Math.abs(by - gravity / 16.0D) < 0.003D) {
                    d5 = -0.003D;
                } else {
                    d5 = by - gravity / 16.0D;
                }
                self.setDeltaMovement(new Vec3(bx, d5, bz));
            } else {
                self.setDeltaMovement(new Vec3(bx, by, bz));
            }
        } else {
            // ---- LAVA PATH, off 228-315: products stored — vanilla calls ----
            // off 228-233: moveRelative(0.02F, travelVector)
            self.moveRelative(0.02F, travelVector);
            // off 236-244: move(SELF, getDeltaMovement())
            self.move(MoverType.SELF, self.getDeltaMovement());
            // off 247-259: dcmpg ifgt -> scale branch iff fluidHeight > threshold
            if (self.getFluidHeight((TagKey<Fluid>) (TagKey) FluidTags.LAVA)
                <= self.getFluidJumpThreshold()) {
                // off 262-279: PRODUCT multiply(0.5, 0.8, 0.5) stored — vanilla
                self.setDeltaMovement(self.getDeltaMovement().multiply(0.5D, 0.800000011920929D, 0.5D));
                // off 282-298: gFFAM over the STORED dm — vanilla (alias/new)
                self.setDeltaMovement(
                    self.getFluidFallingAdjustedMovement(gravity, falling, self.getDeltaMovement()));
            } else {
                // off 304-315: PRODUCT scale(0.5) stored — vanilla
                self.setDeltaMovement(self.getDeltaMovement().scale(0.5D));
            }
        }
        // off 318-342: if (gravity != 0.0) setDeltaMovement(dm.add(0, -gravity/4, 0))
        // — PRODUCT add(DDD) stored — vanilla
        if (gravity != 0.0D) {
            self.setDeltaMovement(
                self.getDeltaMovement().add(0.0D, -gravity / 4.0D, 0.0D));
        }
        // off 345-349: vec7 = getDeltaMovement()
        final Vec3 dm7 = self.getDeltaMovement();
        // off 351-388: if (horizontalCollision && isFree(x, y + 0.6 - getY() + y0, z))
        if (self.horizontalCollision
            && self.isFree(
                dm7.x,
                dm7.y + 0.6000000238418579D - self.getY() + y0,
                dm7.z)) {
            // off 391-405: setDeltaMovement(x, 0.30000001192092896, z) — void variant
            self.setDeltaMovement(dm7.x, 0.30000001192092896D, dm7.z);
        }
    }
}
