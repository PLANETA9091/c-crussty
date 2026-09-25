package net.minecraft.world.entity;

/**
 * NAV-DEADBAND bridge (TASK-459-71, ID-P46 — закон 11 тик-459; lever
 * cmp459_p46 STRICT eq). v1 = PathNavigation.doStuckDetection only.
 *
 * Collect-проход идёт ПОРЯДКОМ ИТЕРАЦИИ navigatingMobs, плоские массивы
 * delay-счётчиков и node-дистанций -> ОДИН bulk-JNI navStuckBatch ->
 * apply-проход stop()/resetStuckTimeout()/запись полей в том же порядке
 * (parity). Канон: NavPlaneOps/decideJava/ERR-ladder (nav_plane.rs,
 * TASK-405-A), MoveOps/navmath_flat.rs (TASK-459-69 P44).
 *
 * SCAFFOLD-КОММИТ: это СТАБ. Методы handle/collect/apply приходят в v1
 * wiring-коммите вместе с build-скриптом (паттерн build_collidebatch_ops.sh,
 * find-гейт «ровно один classfile» — NCDFE-канон S7-163) и вписыванием
 * вызова в PathNavigation.tick через classfile.rs. До этого класс нигде
 * не компилируется и не определяется — ваниль бит-в-байт по построению.
 * NCDFE-канон: НОЛЬ nested/lambdas, EARLY-define в kernel loader, selfTest
 * до ARM (уроки x452/entity_query cv3-1).
 *
 * javap-контракт (RESEARCH-459-P46.md §1.3, patched-kernel.jar 2025-12-11):
 *   (a) stuck-check, gate tick - lastStuckCheck > 100 (if_icmple):
 *       f  = getSpeed() > 1.0f ? getSpeed() : getSpeed()*getSpeed()  // fcmpl
 *       f1 = f * 100.0f * 0.25f
 *       d  = pos.distanceToSqr(lastStuckCheckPos) = (dx*dx+dy*dy)+dz*dz
 *       d < (double)(f1*f1) -> isStuck=true; stop(); else isStuck=false
 *       lastStuckCheck = tick; lastStuckCheckPos = pos
 *   (b) timeout-check (каждый тик при path!=null && !isDone, БЕЗ гейта):
 *       node == timeoutCachedNode -> timeoutTimer += g - lastTimeoutCheck
 *       else -> cachedNode=node; d=pos.distanceTo(atBottomCenterOf(node));
 *               timeoutLimit = speed>0.0f ? d/(double)speed*20.0d : 0.0d
 *       timeoutLimit > 0.0d && timeoutTimer > timeoutLimit*3.0d -> timeoutPath()
 *       lastTimeoutCheck = g
 *   Vec3.atBottomCenterOf(Vec3i) = (x+0.5d, y, z+0.5d) — Y БЕЗ +0.5.
 */
public final class NavDeadbandOps {
    private NavDeadbandOps() {}

    /** out-флаги решения слота (apply-диспетчер v1; бит-маска). */
    public static final int OUT_NONE = 0;         // deadband-ветвь, состояние без изменений
    public static final int OUT_STUCK_STOP = 1;   // isStuck=true + stop(); lastStuckCheck/Pos := tick/pos
    public static final int OUT_STUCK_CLEAR = 2;  // isStuck=false; lastStuckCheck/Pos := tick/pos
    public static final int OUT_TIMEOUT_STOP = 4; // timeoutPath(): resetStuckTimeout + stop()

    /** STUCK_CHECK_INTERVAL / STUCK_THRESHOLD_DISTANCE_FACTOR (javap-инлайны ядра). */
    public static final int STUCK_CHECK_INTERVAL = 100;
    public static final float STUCK_THRESHOLD_DISTANCE_FACTOR = 0.25F;

    /** fcmpl JVM-семантика (NaN -> -1), НЕ Float.compare (NaN там = max). */
    private static int fcmpl(float a, float b) {
        return (a > b) ? 1 : (a == b ? 0 : -1);
    }

    /** One-shot disarm latch: любой rc<0/throwable -> decideJava навсегда. */
    static volatile boolean batchOk = true;

    /**
     * ОДИН bulk-JNI на всё множество навигирующих мобов тика. Входы (входы
     * карточки ID-P46: delay-счётчики + node-дистанции), порядок = порядок
     * итерации navigatingMobs:
     *   ticks[n]         i32  PathNavigation.tick
     *   lastStuck[n]     i32  PathNavigation.lastStuckCheck
     *   timers[n*4]      i64  timeoutTimer, lastTimeoutCheck, gameTime, pad0
     *   meta[n*2]        i32  speedBits(f32), flags(bit0 pathActive)
     *   node[n*6]        i32  nodeXYZ, timeoutCachedNodeXYZ
     *   pos[n*6]         f64  tempMobPosXYZ, lastStuckCheckPosXYZ
     *   limit[n]         f64  timeoutLimit (текущее поле)
     * Выходы (apply-проход, порядок collect):
     *   outFlags[n]      i8   OUT_* бит-маска
     *   outTimer[n*2]    i64  новый timeoutTimer, новый lastTimeoutCheck
     *   outLimit[n]      f64  новый timeoutLimit
     * rc: 0 ok; ERR_STRUCT -1 (corrupt/disarm); ERR_RANGE -2 (java-фолбэк на батч).
     */
    public static native int navStuckBatch(int n, int[] ticks, int[] lastStuck,
                                           long[] timers, int[] meta, int[] node,
                                           double[] pos, double[] limit,
                                           byte[] outFlags, long[] outTimer,
                                           double[] outLimit);

    /** java-реплика решателя: та же математика бит-в-бит (fcmpl/dcmpl
     *  NaN-семантика как в JVM); вызывается на ERR/throwable для всего
     *  батча (catch -> disarm, канон NavPlaneOps — без CME-retry). */
    static int decideJava(int n, int[] ticks, int[] lastStuck, long[] timers,
                          int[] meta, int[] node, double[] pos, double[] limit,
                          byte[] outFlags, long[] outTimer, double[] outLimit) {
        for (int i = 0; i < n; i++) {
            int out = OUT_NONE;
            if (ticks[i] - lastStuck[i] > STUCK_CHECK_INTERVAL) {
                float sp = Float.intBitsToFloat(meta[i * 2]);
                float f = (fcmpl(sp, 1.0F) < 0) ? sp * sp : sp; // fcmpl,iflt: NaN -> ветвь sp*sp
                float f1 = f * 100.0F * STUCK_THRESHOLD_DISTANCE_FACTOR;
                double dx = pos[i * 6]     - pos[i * 6 + 3];
                double dy = pos[i * 6 + 1] - pos[i * 6 + 4];
                double dz = pos[i * 6 + 2] - pos[i * 6 + 5];
                double d = (dx * dx + dy * dy) + dz * dz; // лево-ассоц javap
                if (d < (double) (f1 * f1)) {             // dcmpg: NaN -> else
                    out |= OUT_STUCK_STOP;
                } else {
                    out |= OUT_STUCK_CLEAR;
                }
            }
            boolean pathActive = (meta[i * 2 + 1] & 1) != 0;
            if (pathActive) {
                long g = timers[i * 4 + 2];
                boolean sameNode = node[i * 6] == node[i * 6 + 3]
                        && node[i * 6 + 1] == node[i * 6 + 4]
                        && node[i * 6 + 2] == node[i * 6 + 5];
                long timer = timers[i * 4];
                double newLimit = limit[i];
                if (sameNode) {
                    timer += g - timers[i * 4 + 1]; // lsub,ladd
                } else {
                    double ndx = pos[i * 6]     - ((double) node[i * 6 + 3] + 0.5D);
                    double ndy = pos[i * 6 + 1] - (double) node[i * 6 + 4];
                    double ndz = pos[i * 6 + 2] - ((double) node[i * 6 + 5] + 0.5D);
                    double d = Math.sqrt((ndx * ndx + ndy * ndy) + ndz * ndz);
                    float sp = Float.intBitsToFloat(meta[i * 2]);
                    newLimit = (fcmpl(sp, 0.0F) <= 0) ? 0.0D
                            : (d / (double) sp) * 20.0D;      // fcmpl,ifle; ddiv,dmul
                }
                // javap 227/244 dcmpl: NaN -> -1 -> ifle -> skip.
                if (Double.compare(newLimit, 0.0D) > 0
                        && (double) timer > newLimit * 3.0D) {
                    out |= OUT_TIMEOUT_STOP;
                }
                outTimer[i * 2] = timer;
                outTimer[i * 2 + 1] = g;                  // пишется ВСЕГДА при active
                outLimit[i] = newLimit;
            } else {
                outTimer[i * 2] = timers[i * 4];
                outTimer[i * 2 + 1] = timers[i * 4 + 1];
                outLimit[i] = limit[i];
            }
            outFlags[i] = (byte) out;
        }
        return 0;
    }

    /** selfTest до ARM (NCDFE-канон): decideJava-ядро на краевых входах. */
    public static boolean selfTest() {
        int[] ticks = {1101, 1100};
        int[] lastStuck = {1000, 1000};
        long[] timers = {0, 0, 1000, 0, 0, 0, 1000, 0};
        int[] meta = {Float.floatToIntBits(1.0F), 0, Float.floatToIntBits(1.0F), 0};
        int[] node = new int[12];
        double[] pos = {0, 64, 0, 0, 64, 0, 0, 64, 0, 0, 64, 0};
        double[] limit = {0, 0};
        byte[] outFlags = new byte[2];
        long[] outTimer = new long[4];
        double[] outLimit = new double[2];
        int rc = decideJava(2, ticks, lastStuck, timers, meta, node, pos, limit,
                outFlags, outTimer, outLimit);
        return rc == 0 && (outFlags[0] & OUT_STUCK_STOP) != 0
                && (outFlags[1] & (OUT_STUCK_STOP | OUT_STUCK_CLEAR)) == 0;
    }
}
