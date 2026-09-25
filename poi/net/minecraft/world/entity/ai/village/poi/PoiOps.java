package net.minecraft.world.entity.ai.village.poi;

import java.util.logging.Logger;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;

/**
 * POI-PLANE (TASK-456-B, vector R6 "POI" закон-6 подсистема — lever
 * cmp456_poi).
 *
 * ПОДСИСТЕМА: block-state-change POI-плоскость + tick-оркестрация flush.
 * Vanilla-контракт (javap patched-kernel 1.21.10):
 *  - сайт 1: {@code Level.notifyAndUpdatePhysics → invokevirtual
 *    Level.updatePOIOnBlockStateChange(BlockPos,BS,BS)V} — РОВНО 1 сайт,
 *    virtual-dispatch на ServerLevel-override (тело: PoiTypes.forState ×2 +
 *    Objects.equals early-out + редкий exists/remove/add + debugSynchronizers);
 *  - сайт 2: {@code ChunkMap.tick(BooleanSupplier) → invokevirtual
 *    PoiManager.tick(BooleanSupplier)} — РОВНО 1 сайт (фаза "poi",
 *    villageDistanceTracker.propagateUpdates()).
 *
 * RUST = ЕДИНСТВЕННЫЙ ИСТОЧНИК ДАННЫХ ПОДСИСТЕМЫ: маска POI-состояний
 * (stateId → isPoi, построенная ОДИН раз из PoiTypes.hasPoi по
 * Block.BLOCK_STATE_REGISTRY) байндится в rust-сторону (poiBindMask) — DOD
 * данные POI-плоскости живут в Rust; батч POI-событий (levelHash, posLo,
 * posHi, oldId, newId) уходит в Rust-зеркало PoiStore ОДНИМ bulk-JNI на тик
 * (poiEpoch, flush на сайте 2; пустой тик = 0 JNI). per-block JNI
 * отсутствует (закон 6: один bulk-вызов на подсистему на тик).
 *
 * ВАНИЛЬНОСТЬ БИТ-В-БАЙТ: fast-path возвращает early-out ТОЛЬКО когда ОБА
 * состояния не-POI — ваниль в этом случае: forState×2 → оба Optional.empty →
 * Objects.equals==true → никакой семантики (нет exists/remove/add, нет
 * debugSynchronizers). Slow-path = ПОЛНЫЙ ванильный virtual-вызов
 * level.updatePOIOnBlockStateChange (тело НЕ тронуто); батч-запись = чистая
 * телеметрия в сторону Rust (не влияет на решение). Сайт 2 = flush + ваниль
 * pm.tick. Пустой/чужой флаг = ни хука, ни ретаргета — ваниль бит-в-байт.
 *
 * FAIL-CLOSED: не-Level receiver / getId<0 / MASK==null → ванильный вызов;
 * poiProbe mismatch / poiBindMask / poiEpoch ERR_STRUCT → broken=true
 * навсегда (ваниль каждый вызов/тик); ERR_RANGE → ваниль этот тик.
 */
public final class PoiOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals("cmp456_poi")
                // TASK-456-B: полный носитель эры (STRICT OR) — все сертифи-
                // цированные плоскости мастера живут на каждом гейте.
                || f.trim().equals("cmp409_multi") || f.trim().equals("cmp412_meganav")
                || f.trim().equals("cmp412_eqsnapv3") || f.trim().equals("cmp414_cvs")
                || f.trim().equals("cmp417_bq") || f.trim().equals("cmp420_colpush")
                || f.trim().equals("cmp421_brain") || f.trim().equals("cmp422_brain2")
                || f.trim().equals("cmp423_brain3") || f.trim().equals("cmp424_mobfeed")
                || f.trim().equals("cmp430_inside") || f.trim().equals("cmp432_inside2")
                || f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3")
                || f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5")
                || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp452_mega")
                || f.trim().equals("cmp453_diet") || f.trim().equals("cmp436_ins4")
                || f.trim().equals("cmp451_senseins") || f.trim().equals("cmp438_sense"));
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x5049; // "PI"
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final Logger LOG = Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/poi_plane.rs, RegisterNatives после define) ----
    private static native int poiProbe();
    private static native int poiBindMask(long[] mask);
    private static native int poiEpoch(int tick, int count, int[] batch, int[] out);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    /** Маска POI-состояний (stateId → bit), байндится в Rust (poiBindMask). */
    private static volatile long[] MASK;
    private static volatile boolean MASK_BOUND;

    /** Батч POI-событий: 5 int на событие (levelHash, posLo, posHi, oldId, newId). */
    private static int[] EV = new int[5 * 64];
    private static int EV_COUNT = 0; // число СОБЫТИЙ (не int'ов)

    /** Серверный тик последней успешной эпохи (flush-once-per-tick). */
    private static volatile long EPOCH_TICK = Long.MIN_VALUE;

    /** One-shot ARM/effect-пруф (виден в server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;
    private static volatile boolean EPOCH_LOGGED = false;

    private static final String LABEL = label();

    private static String label() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f == null ? "(off)" : f.trim();
    }

    private PoiOps() {}

    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (PoiOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = poiProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /**
     * Ленивая сборка маски POI-состояний (ОДИН раз; ~26k состояний ×
     * containsKey) + байнд в Rust. failures → broken=true (ваниль навсегда).
     */
    private static void ensureMask() {
        if (MASK != null || broken) {
            return;
        }
        synchronized (PoiOps.class) {
            if (MASK != null || broken) {
                return;
            }
            try {
                if (!probeOnce()) {
                    broken = true;
                    LOG.warning("[crussty-plugin] cmp456_poi: poiProbe mismatch — POI plane disarmed to vanilla");
                    return;
                }
                int size = Block.BLOCK_STATE_REGISTRY.size();
                long[] m = new long[(size >>> 6) + 1];
                for (int i = 0; i < size; i++) {
                    if (PoiTypes.hasPoi(Block.BLOCK_STATE_REGISTRY.byId(i))) {
                        m[i >>> 6] |= 1L << (i & 63);
                    }
                }
                int rc = poiBindMask(m);
                if (rc != 0) {
                    broken = true;
                    LOG.warning("[crussty-plugin] cmp456_poi: poiBindMask rc=" + rc + " — POI plane disarmed to vanilla");
                    return;
                }
                MASK = m;
                MASK_BOUND = true;
            } catch (Throwable th) {
                broken = true;
                LOG.warning("[crussty-plugin] cmp456_poi: mask build failed — POI plane disarmed to vanilla");
            }
        }
    }

    /**
     * Структурный selfTest (Rust вызывает ДО ARM; selfTest==true до ARM).
     * Первый вызов также строит маску + байндит в Rust (boot, off-tick).
     */
    public static boolean selfTest() {
        if (!ENABLED || broken) {
            return false;
        }
        ensureMask();
        return !broken && MASK_BOUND && MASK != null;
    }

    /** Диагностика rust-стороны: дизарм ли плейн. */
    public static boolean armed() {
        return ENABLED && !broken && probeOnce();
    }

    private static boolean isPoi(long[] m, int id) {
        return id >= 0 && id < (m.length << 6) && ((m[id >>> 6] >>> (id & 63)) & 1L) != 0;
    }

    /** Батч-запись события (телеметрия в Rust-зеркало; не влияет на решение). */
    private static void appendEvent(Level level, BlockPos pos, int oid, int nid) {
        if (broken) {
            return;
        }
        try {
            long packed = BlockPos.asLong(pos.getX(), pos.getY(), pos.getZ());
            int lo = (int) packed;
            int hi = (int) (packed >>> 32);
            int lh = System.identityHashCode(level);
            int slot;
            synchronized (PoiOps.class) {
                if (EV_COUNT * 5 + 5 > EV.length) {
                    int[] bigger = new int[EV.length * 2];
                    System.arraycopy(EV, 0, bigger, 0, EV.length);
                    EV = bigger;
                }
                slot = EV_COUNT;
                EV_COUNT++;
                EV[slot * 5] = lh;
                EV[slot * 5 + 1] = lo;
                EV[slot * 5 + 2] = hi;
                EV[slot * 5 + 3] = oid;
                EV[slot * 5 + 4] = nid;
            }
            if (!ARM_LOGGED) {
                ARM_LOGGED = true;
                LOG.info("[crussty-plugin] " + LABEL + ": updatePOI EFFECT armed (first POI-relevant block change"
                        + " at tick " + net.minecraft.server.MinecraftServer.getServer().getTickCount()
                        + ", oldId=" + oid + ", newId=" + nid + ", batched for rust mirror)");
            }
        } catch (Throwable th) {
            broken = true; // структурный отказ телеметрии — плейн в ваниль
        }
    }

    /**
     * Замена сайта 1: {@code invokevirtual Level.updatePOIOnBlockStateChange}
     * в Level.notifyAndUpdatePhysics. Desc РОВНО виртуальный desc с
     * receiver-классом Level, препендированным (валидатор compose). Fast-path
     * (оба не-POI) = ванильный no-op бит-в-байт; иначе — ванильный virtual
     * вызов (тело ServerLevel.updatePOIOnBlockStateChange НЕ тронуто).
     */
    public static void updatePoiGate(Level level, BlockPos pos, BlockState oldState, BlockState newState) {
        if (ENABLED && !broken) {
            long[] m = MASK;
            if (m == null && !MASK_BOUND) {
                ensureMask();
                m = MASK;
            }
            if (m != null) {
                int oid = Block.BLOCK_STATE_REGISTRY.getId(oldState);
                int nid = Block.BLOCK_STATE_REGISTRY.getId(newState);
                if (!isPoi(m, oid) && !isPoi(m, nid)) {
                    return; // ваниль: оба Optional.empty → Objects.equals==true → no-op
                }
                appendEvent(level, pos, oid, nid);
            }
        }
        level.updatePOIOnBlockStateChange(pos, oldState, newState);
    }

    /**
     * Замена сайта 2: {@code invokevirtual PoiManager.tick(BooleanSupplier)}
     * в ChunkMap.tick. Flush эпохи = ОДИН bulk-JNI (poiEpoch) батча событий
     * в Rust-зеркало PoiStore (once per tick; пустой тик = 0 JNI), затем
     * ПОЛНЫЙ ванильный вызов pm.tick(hasTimeLeft).
     */
    public static void poiTickGate(PoiManager pm, java.util.function.BooleanSupplier hasTimeLeft) {
        if (ENABLED && !broken) {
            maybeEpoch();
        }
        pm.tick(hasTimeLeft);
    }

    /** Одна эпоха flush на серверный тик (double-checked по EPOCH_TICK). */
    private static void maybeEpoch() {
        long t = net.minecraft.server.MinecraftServer.getServer().getTickCount();
        if (EPOCH_TICK == t) {
            return; // горячий путь: один volatile-read
        }
        synchronized (PoiOps.class) {
            if (EPOCH_TICK == t || broken) {
                return;
            }
            int count;
            int[] snapshot;
            synchronized (PoiOps.class) {
                count = EV_COUNT;
                if (count <= 0) {
                    EPOCH_TICK = t; // пустой тик: 0 JNI
                    return;
                }
                snapshot = new int[count * 5];
                System.arraycopy(EV, 0, snapshot, 0, count * 5);
                EV_COUNT = 0;
            }
            int[] out = new int[1];
            int rc;
            try {
                rc = poiEpoch((int) t, count, snapshot, out);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                broken = true;
                LOG.warning("[crussty-plugin] cmp456_poi: poiEpoch ERR_STRUCT — POI plane disarmed to vanilla");
                return;
            }
            if (rc == ERR_RANGE) {
                return; // параметр-промах: события будут повторно упакованы на следующем тике
            }
            EPOCH_TICK = t;
            if (!EPOCH_LOGGED) {
                EPOCH_LOGGED = true;
                LOG.info("[crussty-plugin] " + LABEL + ": epoch ok tick=" + t
                        + " events=" + rc + " mirrorTotal=" + out[0]
                        + " (bulk JNI 1/tick into rust PoiStore)");
            }
        }
    }
}
