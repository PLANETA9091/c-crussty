package net.minecraft.world.entity.ai.control;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.tags.BlockTags;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.navigation.PathNavigation;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.pathfinder.NodeEvaluator;
import net.minecraft.world.level.pathfinder.PathType;
import net.minecraft.world.phys.shapes.VoxelShape;

/**
 * MOVE-PLANE (TASK-463-69a, lever cmp463_move — STRICT eq): P44 navmath
 * bridge for the mob movement decision math.
 *
 * Lane facts (LEDGER-37, javap x463): the whole per-mob trig load of the
 * nav_ai lane lives in MoveControl.tick (295 instructions, offsets 0..585):
 * 0-2 Mth calls per mob per tick — STRAFE branch = sin@93 + cos@108 (+sqrt@50
 * which stays vanilla Mth), MOVE_TO branch = atan2@290; WAIT/JUMPING = 0.
 * This bridge retargets the tick body (classfile redirect ->
 * MovePlaneOps.handle) and replaces EXACTLY those two math points with the
 * fast Mth replicas below; every other instruction of the vanilla body is
 * replicated bit-in-bit (same op order, same constants, K10/K11 verbatim).
 *
 * MATH CONTRACT (javap-verbatim, 12/12 constant groups, oracle 10^5 GREEN):
 *  - SIN[65536] fill lambda: (float)Math.sin((double)i * PI64 * 2.0 / 65536.0)
 *    — bit-in-bit GREEN 65536/65536 vs jar reflection dump (navoracle).
 *  - fastSin: SIN[(int)(f * 10430.378f) & 65535] (f2i saturates; iand).
 *  - fastCos: SIN[(int)(f * 10430.378f + 16384.0f) & 65535] — the cos offset
 *    16384.0f is added in f32 BEFORE the f2i (K3).
 *  - fastInvSqrt: magic 6910469410427058090L, lshr 1, lsub; Newton
 *    0.5d/1.5d with the vanilla association d1 * (1.5 - (d0 * d1) * d1).
 *  - FRAC_BIAS = Double.longBitsToDouble(4805340802404319232L) = 2^44
 *    (live-verified: bits -> 17592186044416.0; LEDGER-37 K8).
 *  - fastAtan2 = full javap transcription of Mth.atan2 (oracle: 10^5/10^5
 *    bit-in-bit): NaN->NaN; neg flags y<0 then x<0; swap flag = param1 >
 *    param2 (dcmpl@65 + ifle@66 -> y>x, NOT x>y — the x>y inversion gave
 *    99.989% divergence in the x463 model); after swap slot0 = min(|y|,|x|),
 *    slot2 = max; slot2*d9 BEFORE slot0*d9; i13 = (int)rawLowBits(d11)
 *    (bare l2i, no shift) with d11 = FRAC_BIAS + slot0 — floor(u*256) in
 *    [0..256]; d20 = slot0*COS - slot2*(d11-FRAC_BIAS); cubic correction
 *    ((6.0 + d20*d20) * d20) * 0.16666666666666666; correction order STRICT:
 *    swap -> PI/2, then x<0 -> PI, then y<0 -> minus LAST (else 11/10^5).
 *  - ASIN_TAB[257]/COS_TAB[257]: Math.asin(i/256.0) on the JDK = fdlibm
 *    family — bit-in-bit equal to the jar dumps (the 9/257 +/-1 ULP gap is
 *    glibc-vs-fdlibm and applies to RUST generation only, never to this
 *    static init; navoracle CosDump/MD5 evidence).
 *
 * Two DIFFERENT pi values coexist (K4 f64 3.141592653589793d in the table
 * fill + atan2 PI-corrections; K11 f32 3.1415927410125732d in the tick
 * yRot degrees conversion) — unifying them is FORBIDDEN (1-ULP yRot bug).
 *
 * Fail-closed: this class is defined EARLY (BRIDGE_DEFINED block,
 * entity_query.rs) only under the cmp463_move carrier; the rust side
 * registers the moveDecide native BEFORE the MoveControl retransform is
 * published (law 6 v16: define -> register natives -> probe -> publish).
 * The native is the batch trig kernel for the lockstep/parity gate and the
 * future bulk lane; the per-tick hot path here is pure Java (zero JNI —
 * law 6: NO per-entity JNI). Empty/foreign lever flag: class never defined,
 * tick body never retargeted -> vanilla bit-for-byte by construction.
 */
public final class MovePlaneOps {
    private MovePlaneOps() {}

    // ------------------------------------------------------------------
    // Tables (Mth-канон; field names == vanilla Mth for javap-diff parity).
    // ------------------------------------------------------------------
    private static final float[] SIN = new float[65536];
    private static final double[] ASIN_TAB = new double[257];
    private static final double[] COS_TAB = new double[257];
    private static final double FRAC_BIAS = Double.longBitsToDouble(4805340802404319232L);

    static {
        // K2/K4/K5: vanilla fill lambda (bit-in-bit GREEN vs jar dump).
        for (int i = 0; i < 65536; ++i) {
            SIN[i] = (float)Math.sin((double)i * Math.PI * 2.0 / 65536.0);
        }
        // K9: LUT 257, divisor 256.0d; COS stored before ASIN (javap order).
        for (int j = 0; j < 257; ++j) {
            double d0 = (double)j / 256.0;
            double d1 = Math.asin(d0);
            COS_TAB[j] = Math.cos(d1);
            ASIN_TAB[j] = d1;
        }
    }

    // K6/K7: magic + Newton, association verbatim (d1 * (1.5 - (d0*d1)*d1)).
    public static double fastInvSqrt(double x) {
        double d0 = 0.5 * x;
        long i = 6910469410427058090L - (Double.doubleToRawLongBits(x) >> 1);
        double d1 = Double.longBitsToDouble(i);
        return d1 * (1.5 - d0 * d1 * d1);
    }

    // K1/K2: 10430.378f multiplier, 65535 iand, f2i saturation via (int) cast.
    public static float fastSin(float f) {
        return SIN[(int)(f * 10430.378f) & 65535];
    }

    // K3: cos offset 16384.0f in f32 BEFORE f2i (K3 verbatim).
    public static float fastCos(float f) {
        return SIN[(int)(f * 10430.378f + 16384.0f) & 65535];
    }

    /**
     * Full javap transcription of Mth.atan2 (see class doc for the traps).
     * Param order == Mth.atan2: first arg is y, second is x.
     */
    public static double fastAtan2(double y, double x) {
        double z = x * x + y * y;
        if (Double.isNaN(z)) {
            return Double.NaN;
        }
        boolean negY = y < 0.0;
        if (negY) {
            y = -y;
        }
        boolean negX = x < 0.0;
        if (negX) {
            x = -x;
        }
        boolean swap = y > x;
        if (swap) {
            double tmp = x;
            x = y;
            y = tmp;
        }
        double d9 = fastInvSqrt(z);
        x = x * d9;
        y = y * d9;
        double d11 = FRAC_BIAS + y;
        int i13 = (int)Double.doubleToRawLongBits(d11);
        double d14 = ASIN_TAB[i13];
        double d16 = COS_TAB[i13];
        double d18 = d11 - FRAC_BIAS;
        double d20 = y * d16 - x * d18;
        double d22 = (6.0 + d20 * d20) * d20 * 0.16666666666666666;
        double d24 = d14 + d22;
        if (swap) {
            d24 = 1.5707963267948966 - d24;
        }
        if (negX) {
            d24 = 3.141592653589793 - d24;
        }
        if (negY) {
            d24 = -d24;
        }
        return d24;
    }

    // ------------------------------------------------------------------
    // Retarget entry: MoveControl.tick body, bit-in-bit, with the 2 fast
    // math points. Same package as MoveControl -> protected fields/methods
    // (mob/wantedX/speedModifier/strafeForwards/strafeRight/operation,
    // rotlerp) are package-accessible; the PRIVATE isWalkable is replicated
    // statically below (javap 412-452 verbatim).
    // ------------------------------------------------------------------
    public static void handle(MoveControl mc) {
        if (mc.operation == MoveControl.Operation.STRAFE) {
            // STRAFE branch (javap 10..195).
            float f = (float)mc.mob.getAttributeValue(Attributes.MOVEMENT_SPEED);
            float f1 = (float)mc.speedModifier * f;
            float fwd = mc.strafeForwards;
            float right = mc.strafeRight;
            float f5 = Mth.sqrt(fwd * fwd + right * right);
            if (f5 < 1.0F) {
                f5 = 1.0F;
            }
            f5 = f1 / f5;
            fwd = fwd * f5;
            right = right * f5;
            // FAST POINT 1+2 (javap 83..111: getYRot * 0.017453292f -> sin/cos).
            float f6 = fastSin(mc.mob.getYRot() * 0.017453292F);
            float f7 = fastCos(mc.mob.getYRot() * 0.017453292F);
            float f8 = fwd * f7 - right * f6;
            float f9 = right * f7 + fwd * f6;
            if (!isWalkable(mc, f8, f9)) {
                mc.strafeForwards = 1.0F;
                mc.strafeRight = 0.0F;
            }
            mc.mob.setSpeed(f1);
            mc.mob.setZza(mc.strafeForwards);
            mc.mob.setXxa(mc.strafeRight);
            mc.operation = MoveControl.Operation.WAIT;
        } else if (mc.operation == MoveControl.Operation.MOVE_TO) {
            // MOVE_TO branch (javap 198..501).
            mc.operation = MoveControl.Operation.WAIT;
            double d1 = mc.wantedX - mc.mob.getX();
            double d3 = mc.wantedZ - mc.mob.getZ();
            double d5 = mc.wantedY - mc.mob.getY();
            double d7 = d1 * d1 + d5 * d5 + d3 * d3;
            if (d7 < 2.500000277905201E-7) {
                mc.mob.setZza(0.0F);
                return;
            }
            // FAST POINT 3 (javap 288..305): atan2(dz,dx) * 180.0 / f32-PI,
            // d2f, then - 90.0f. K11 f32-pi 3.1415927410125732 verbatim.
            float f9 = (float)(fastAtan2(d3, d1) * 180.0 / 3.1415927410125732) - 90.0F;
            mc.mob.setYRot(mc.rotlerp(mc.mob.getYRot(), f9, 90.0F));
            mc.mob.setSpeed((float)(mc.speedModifier * mc.mob.getAttributeValue(Attributes.MOVEMENT_SPEED)));
            BlockPos blockpos = mc.mob.blockPosition();
            Level level = mc.mob.level();
            BlockState blockstate = level.getBlockState(blockpos);
            VoxelShape voxelshape = blockstate.getCollisionShape(level, blockpos);
            if (d5 > (double)mc.mob.maxUpStep()
                    && d1 * d1 + d3 * d3 < (double)Math.max(1.0F, mc.mob.getBbWidth())) {
                mc.mob.getJumpControl().jump();
                mc.operation = MoveControl.Operation.JUMPING;
            } else if (!voxelshape.isEmpty()
                    && mc.mob.getY() < voxelshape.max(Direction.Axis.Y) + (double)blockpos.getY()
                    && !blockstate.is(BlockTags.DOORS)
                    && !blockstate.is(BlockTags.FENCES)) {
                mc.mob.getJumpControl().jump();
                mc.operation = MoveControl.Operation.JUMPING;
            }
        } else if (mc.operation == MoveControl.Operation.JUMPING) {
            // JUMPING branch (javap 504..574).
            mc.mob.setSpeed((float)(mc.speedModifier * mc.mob.getAttributeValue(Attributes.MOVEMENT_SPEED)));
            if (!mc.mob.onGround() && (!mc.mob.isInLiquid() || !mc.mob.isAffectedByFluids())) {
                // stay JUMPING until grounded/fluid-processed (javap 544..564)
            } else {
                mc.operation = MoveControl.Operation.WAIT;
            }
        } else {
            // default (WAIT/anything else): javap 577..582.
            mc.mob.setZza(0.0F);
        }
    }

    /** Private MoveControl.isWalkable replicated verbatim (javap 412..452). */
    private static boolean isWalkable(MoveControl mc, float fx, float fz) {
        PathNavigation navigation = mc.mob.getNavigation();
        if (navigation != null) {
            NodeEvaluator nodeevaluator = navigation.getNodeEvaluator();
            if (nodeevaluator != null) {
                if (nodeevaluator.getPathType(
                        mc.mob,
                        BlockPos.containing(
                                mc.mob.getX() + (double)fx,
                                (double)mc.mob.getBlockY(),
                                mc.mob.getZ() + (double)fz
                        )
                ) != PathType.WALKABLE) {
                    return false;
                }
            }
        }
        return true;
    }

    // ------------------------------------------------------------------
    // Batch trig kernel (Rust native; the G2-lockstep + bulk-lane entry).
    // op 0 (STRAFE): argA[i] = f64 carrying the f32 angle bits; out[i*2] =
    //   f32 bits of fastSin, out[i*2+1] = f32 bits of fastCos.
    // op 1 (MOVE_TO): argA[i] = y, argB[i] = x; out[i*2] = low 32 bits and
    //   out[i*2+1] = high 32 bits of the f64 fastAtan2 bit pattern.
    // rc 0 = ok; negative = ERR (one-shot batchOk latch -> decideTrigJava).
    // ------------------------------------------------------------------
    public static native int moveDecide(int n, int[] ops, double[] argA,
                                        double[] argB, int[] out);

    private static volatile boolean batchOk = true;

    /** Batch driver: returns false iff the native disarmed (java replica ran). */
    public static boolean moveDecideBatch(int n, int[] ops, double[] argA,
                                          double[] argB, int[] out) {
        if (batchOk) {
            try {
                int rc = moveDecide(n, ops, argA, argB, out);
                if (rc == 0) {
                    return true;
                }
                batchOk = false;
            } catch (Throwable t) {
                batchOk = false;
            }
        }
        decideTrigJava(n, ops, argA, argB, out);
        return false;
    }

    /** Bit-exact java replica of the native kernel (fail-closed path). */
    public static void decideTrigJava(int n, int[] ops, double[] argA,
                                      double[] argB, int[] out) {
        for (int i = 0; i < n; i++) {
            if (ops[i] == 0) {
                float a = Float.intBitsToFloat((int)argA[i]);
                out[i * 2] = Float.floatToIntBits(fastSin(a));
                out[i * 2 + 1] = Float.floatToIntBits(fastCos(a));
            } else {
                long bits = Double.doubleToRawLongBits(fastAtan2(argA[i], argB[i]));
                out[i * 2] = (int)bits;
                out[i * 2 + 1] = (int)(bits >> 32);
            }
        }
    }

    // ------------------------------------------------------------------
    // selfTest: the in-JVM oracle gate (real Mth on the classpath is the
    // ground truth). Grid + random sweep, bit-in-bit on f32/f64 patterns.
    // ------------------------------------------------------------------
    public static boolean selfTest() {
        // landmarks of the transcription traps
        if (Double.doubleToRawLongBits(fastAtan2(0.0, 1.0)) != 0L) {
            return false;
        }
        if (Double.doubleToRawLongBits(fastAtan2(1.0, 0.0))
                != Double.doubleToRawLongBits(1.5707963267948966)) {
            return false;
        }
        long seed = 42L;
        java.util.Random r = new java.util.Random(seed);
        int badSin = 0;
        int badCos = 0;
        int badAtan = 0;
        for (int k = 0; k < 100000; k++) {
            float ang = (float)((r.nextLong() & 0xFFFFFFL)) / 4096.0f - 512.0f;
            if (Float.floatToIntBits(fastSin(ang)) != Float.floatToIntBits(Mth.sin(ang))) {
                badSin++;
            }
            if (Float.floatToIntBits(fastCos(ang)) != Float.floatToIntBits(Mth.cos(ang))) {
                badCos++;
            }
            double y = (double)(r.nextLong() % 1000000L) / 1024.0;
            double x = (double)(r.nextLong() % 1000000L) / 1024.0;
            if (k % 17 == 0) {
                y = -y;
            }
            if (k % 23 == 0) {
                x = -x;
            }
            if (k % 97 == 0) {
                y = 0.0;
            }
            if (k % 101 == 0) {
                x = 0.0;
            }
            if (Double.doubleToRawLongBits(fastAtan2(y, x))
                    != Double.doubleToRawLongBits(Mth.atan2(y, x))) {
                if (badAtan < 3) {
                    System.out.println("  move_plane selfTest DIVERGE y=" + y + " x=" + x);
                }
                badAtan++;
            }
        }
        System.out.println("move_plane selfTest sin=" + (100000 - badSin) + "/100000 cos="
                + (100000 - badCos) + "/100000 atan2=" + (100000 - badAtan) + "/100000"
                + " FRAC_BIAS=" + FRAC_BIAS
                + " bits=" + Double.doubleToRawLongBits(FRAC_BIAS));
        boolean ok = badSin == 0 && badCos == 0 && badAtan == 0;
        if (ok) {
            // literal marker (check_blobs_sync javap/raw-cp gate)
            System.out.println("move_plane selfTest PASS");
        }
        return ok;
    }

    // ARM marker: greppable lever proof in server stdout.
    static final String ARM_MARKER = "move_plane ARMED cmp463_move";
}
