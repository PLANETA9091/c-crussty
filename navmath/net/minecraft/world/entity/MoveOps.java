package net.minecraft.world.entity;

import net.minecraft.util.Mth;

/**
 * NAVMATH-FLAT bridge (TASK-459-69, ID-P44 — закон 11 тик-459; lever
 * cmp459_navmath STRICT eq). v1 = MoveControl.tick only, 1 ретаргет.
 *
 * Collect-проход идёт ПОРЯДКОМ ИТЕРАЦИИ СЕТА мобов, плоские массивы
 * (operationType, posDelta, rot, speedMod) -> ОДИН bulk-JNI navMoveBatch ->
 * apply-проход setYRot/setSpeed/setZza/jump в том же порядке (parity).
 * Канон: NavPlaneOps/decideJava/ERR-ladder (nav_plane.rs, TASK-405-A),
 * mobs_soa SoA-дисциплина.
 *
 * SCAFFOLD-КОММИТ: это СТАБ. Методы handle/collect/apply приходят в v1
 * wiring-коммите вместе с build-скриптом (паттерн build_collidebatch_ops.sh,
 * find-гейт «ровно один classfile» — NCDFE-канон S7-163) и ретаргетом
 * MoveControl.tick через classfile.rs. До этого класс нигде не компилируется
 * и не определяется — ваниль бит-в-байт по построению.
 *
 * javap-контракт (RESEARCH-459-P44.md §1, patched-kernel.jar 2025-12-11):
 *   dx = wantedX-mobX; dz = wantedZ-mobZ; dy = wantedY-mobY
 *   d3 = (dx*dx+dy*dy)+dz*dz; d3 < 2.500000277905201E-7 -> setZza(0)
 *   f9 = (float)(Mth.atan2(dz,dx)*180.0/3.1415927410125732)-90.0f
 *   setYRot(rotlerp(getYRot(), f9, 90.0f))
 *   setSpeed((float)(speedModifier * getAttributeValue(MOVEMENT_SPEED)))
 *   jump := (dy>maxUpStep && dx*dx+dz*dz < max(1.0f,bbWidth))
 *        || (!shapeEmpty && mobY < shapeMaxY && !DOOR && !FENCE)
 *   operation := WAIT (до математики); jump -> operation := JUMPING
 */
public final class MoveOps {
    private MoveOps() {}

    /** out-опкоды решений ядра (apply-диспетчер v1). */
    public static final int OUT_STOP = 0;        // setZza(0.0F); op уже WAIT
    public static final int OUT_MOVE = 1;        // setYRot(rot); setSpeed(spd); op WAIT
    public static final int OUT_MOVE_JUMP = 2;   // OUT_MOVE + jump(); op := JUMPING
    public static final int OUT_VANILLA = 3;     // STRAFE/JUMPING -> vanilla inline (widen-2)

    /** One-shot disarm latch: любой rc<0/throwable -> decideJava навсегда. */
    static volatile boolean batchOk = true;

    /**
     * Один раз до первого батча (static-init): копия СОБСТВЕННЫХ Mth-таблиц
     * живой JVM -> паритет atan2 ПО ПОСТРОЕНИЮ (RESEARCH §4: rust-libm даёт
     * 10/514 ulp-расхождений с HotSpot — потому таблицы не генерируются в
     * ядре). rc<0 -> batchOk=false.
     * native-сигнатура: 0 ok / ERR_STRUCT -1 / ERR_RANGE -2.
     */
    private static native int navMoveInit(double[] asinTab, double[] cosTab);

    /**
     * ОДИН bulk-JNI на всё множество мобов тика. Входы:
     *   ops[n]        i32  Operation ordinal (WAIT 0/MOVE_TO 1/STRAFE 2/JUMPING 3)
     *   meta[n*4]     i32  rotBits(f32), maxUpStepBits(f32), bbWidthBits(f32),
     *                      shapeFlags(bit0 empty, bit1 DOOR, bit2 FENCE)
     *   pos[n*6]      f64  wantedX/Y/Z, mobX/Y/Z
     *   speed[n*2]    f64  speedModifier, movementSpeedAttr
     *   shapeMaxY[n]  f64  precollect: shape.max(Y) + (double)blockPos.getY()
     * Выходы (apply-проход, порядок collect):
     *   outOp[n]      i8   OUT_* (см. выше)
     *   outRot[n]     i32  yRot биты (f32)
     *   outSpd[n]     i32  speed биты (f32)
     * rc: 0 ok; ERR_STRUCT -1 (corrupt/disarm); ERR_RANGE -2 (java-фолбэк на батч).
     */
    public static native int navMoveBatch(int n, int[] ops, int[] meta, double[] pos,
                                          double[] speed, double[] shapeMaxY,
                                          byte[] outOp, int[] outRot, int[] outSpd);

    /** java-реплика решателя: та же математика через публичные Mth (бит-в-бит;
     *  отличается только исполнитель). Вызывается на ERR/throwable для всего
     *  батча (v1: без CME-retry — catch -> disarm, канон NavPlaneOps). */
    static int decideJava(int n, int[] ops, int[] meta, double[] pos,
                          double[] speed, double[] shapeMaxY,
                          byte[] outOp, int[] outRot, int[] outSpd) {
        for (int i = 0; i < n; i++) {
            int op = ops[i];
            if (op == 1) {
                double dx = pos[i*6]   - pos[i*6+3];
                double dz = pos[i*6+2] - pos[i*6+5];
                double dy = pos[i*6+1] - pos[i*6+4];
                double d3 = (dx*dx + dy*dy) + dz*dz;
                if (d3 < 2.500000277905201E-7) { outOp[i] = OUT_STOP; continue; }
                float f9 = (float)(Mth.atan2(dz, dx) * 180.0 / 3.1415927410125732) - 90.0f;
                float rot = rotlerp(Float.intBitsToFloat(meta[i*4]), f9, 90.0f);
                float spd = (float)(speed[i*2] * speed[i*2+1]);
                boolean bigStep = dy > (double)Float.intBitsToFloat(meta[i*4+1]);
                boolean narrow = (dx*dx + dz*dz) < (double)Math.max(1.0f, Float.intBitsToFloat(meta[i*4+2]));
                boolean shapeEmpty = (meta[i*4+3] & 1) != 0;
                boolean door = (meta[i*4+3] & 2) != 0;
                boolean fence = (meta[i*4+3] & 4) != 0;
                boolean jump = (bigStep && narrow) || (!shapeEmpty && pos[i*6+4] < shapeMaxY[i] && !door && !fence);
                outOp[i] = (byte)(jump ? OUT_MOVE_JUMP : OUT_MOVE);
                outRot[i] = Float.floatToRawIntBits(rot);
                outSpd[i] = Float.floatToRawIntBits(spd);
            } else if (op == 0) {
                outOp[i] = OUT_STOP;
            } else {
                outOp[i] = OUT_VANILLA; // STRAFE/JUMPING: vanilla inline (widen-2)
            }
        }
        return 0;
    }

    /** MoveControl.rotlerp javap 0..34 (fcmpl/fcmpg NaN-семантика JVM). */
    private static float rotlerp(float a, float b, float maxDelta) {
        float d = wrapDegrees(b - a);
        if (d > maxDelta) d = maxDelta;
        if (d < -maxDelta) d = -maxDelta;
        float e = a + d;
        if (e < 0.0f) e += 360.0f; else if (e > 360.0f) e -= 360.0f;
        return e;
    }

    /** Mth.wrapDegrees(float) javap 0..30 (frem = truncated mod). */
    private static float wrapDegrees(float v) {
        float f = v % 360.0f;
        if (f >= 180.0f) f -= 360.0f;
        if (f < -180.0f) f += 360.0f;
        return f;
    }
}
