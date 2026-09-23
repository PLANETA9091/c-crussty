package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;
import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.Level;

/**
 * SSCAN-DESPAWN (TASK-406-E, vector R4 despawn/spawn/activation scans — lever
 * cmp406_sscan).
 *
 * BULK NEAREST-PLAYER PLANE: единый retarget сайта
 * {@code Mob.checkDespawn → invokevirtual Level.findNearbyPlayer(Entity,D,
 * Predicate)} (javap ground truth purpur-1.21.10: ровно 1 сайт @ offset 55,
 * единственный findNearbyPlayer в Mob.class) на статический мост
 * {@code MobScanOps.findNearbyPlayerGate(Level,Entity,D,Predicate) → Player}
 * (desc = virtual desc с receiver-классом Level, препендированным — contract
 * retarget_virtual_to_static). Vanilla despawn-скан — это O(мобы×игроки)
 * итерация {@code getNearestPlayer(DDDD,Predicate)} на КАЖДОГО моба КАЖДЫЙ
 * тик (150k мобов бенча). РЕШЕНИЕ (rust = единственный источник): ОДИН
 * bulk-JNI {@code sscanEpoch(tick, idTop, players[D], nearest[I])} за тик —
 * rust DOD-проходом по SoA-позициям mobs_soa пишет колонку
 * {@code nearest[denseId] = индекс ближайшего qualifying-игрока | -1} в
 * разделяемый java int[]; решение пер-моба = O(1) чтение колонки
 * (MobPushOps.idBoxOf → плотный id). per-entity JNI отсутствует (закон 6:
 * JNI = один bulk-вызов на тик-батч).
 *
 * ВАНИЛЬНОСТЬ БИТ-В-БАЙТ: мост возвращает ТОГО ЖЕ игрока, которого вернул бы
 * ванильный скан (первый строго-ближайший по порядку {@code level.players()},
 * при равенстве остаётся более ранний — лестница
 * {@code (best == -1.0 || d < best)} и порядок компонент
 * {@code (px-mx)²+(py-my)²+(pz-mz)²} из {@code Entity.distanceToSqr}
 * воспроизведены 1:1 в rust-проходе); предикат применяется ОДИН раз к каждому
 * игроку при снапшоте (зависит только от состояния игрока, не моба);
 * дистанционный фильтр не применяется (checkDespawn передаёт -1.0). ВСЁ тело
 * checkDespawn ПОСЛЕ скана (hard/soft despawn-диапазоны paper, noActionTime,
 * random.nextInt(800), removeWhenFarAway, discard) — НЕТРОНУТЫЙ ванильный
 * байткод, читающий координаты возвращённого игрока.
 *
 * FAIL-CLOSED: ENABLED STRICT-eq "cmp406_sscan" (пустой/чужой флаг — сайт
 * вообще не ретаргетится rust-стороной); не-Mob / не в SoA-плоскости
 * (idBoxOf == null) / id вне последней эпохи → ваниль
 * {@code level.findNearbyPlayer(...)} на этот вызов; пустой qualifying-набор
 * игроков → null (бит-в-байт равно ванильному null); sscanProbe
 * magic-mismatch / sscanEpoch ERR_STRUCT → дизарм навсегда (ваниль каждый
 * тик); ERR_RANGE / эпоха-промах → ваниль на этот тик (эпоха ретраится на
 * следующем). Снапшот публикуется volatile-четвёркой (SNAPSHOT, NEAREST,
 * NEAREST_LEN) c EPOCH_TICK как release-edge — читатели (мейн-тик) видят
 * консистентную тройку.
 */
public final class MobScanOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // STRICT eq (TASK-402-F урок полу-armed гейта): только точный флаг
        // раунда-406 или мультикомпозит-409. База stagtick вооружается
        // СВОИМИ гейтами; этот мост под cmp405_stagtick не вызывается
        // (rust не ставит сайт).
        return f != null && (f.trim().equals("cmp406_sscan")
                // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                || f.trim().equals("cmp409_multi") || f.trim().equals("cmp412_meganav")
                // TASK-414-B: eqsnap-v3 family + leg flag cmp414_cvs (root-cause cv3b-1).
                || f.trim().equals("cmp412_eqsnapv3") || f.trim().equals("cmp414_cvs")
                // TASK-417-C: cvs-носитель ⊕ queryplane.
                || f.trim().equals("cmp417_bq")
                // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
                || f.trim().equals("cmp420_colpush")
                // TASK-421-A: brain-носитель (STRICT OR).
                || f.trim().equals("cmp421_brain")
                // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
                || f.trim().equals("cmp422_brain2")
                // TASK-424-A: GC-ревизия brain3 (STRICT OR).
                || f.trim().equals("cmp423_brain3")
                // TASK-426-A: SoA-feed carrier (STRICT OR).
                || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp428_chunkunion") || f.trim().equals("cmp429_wgen"));
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x5353; // "SS"
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final Logger LOG = Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_sscan.rs, RegisterNatives после define) ----
    private static native int sscanProbe();
    private static native int sscanEpoch(int tick, int idTop, double[] players, int[] nearest);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    /** Снапшот qualifying-игроков (порядок level.players() сохранён). */
    private static volatile Player[] SNAPSHOT = new Player[0];
    /** Колонка nearest[denseId] -> индекс в SNAPSHOT | -1, публикуется с NEAREST_LEN. */
    private static volatile int[] NEAREST = new int[1024];
    /** Сколько валидных элементов записал rust в последний sscanEpoch. */
    private static volatile int NEAREST_LEN = 0;
    /** Пустой qualifying-набор на этот тик: ваниль возвращает null всем. */
    private static volatile boolean SNAPSHOT_EMPTY = false;
    /** Серверный тик последней успешной эпохи (double-checked locking). */
    private static volatile long EPOCH_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    /** One-shot ARM/effect-пруф (виден в server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;

    /** Метка активного флага для ARM-строк (TASK-426-A). */
    private static final String LABEL = label();

    private static String label() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f == null ? "(off)" : f.trim();
    }

    private MobScanOps() {}

    /** Ленивая проверка натива (Throwable -> false, ретрай на следующем тике). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (MobScanOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = sscanProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Диагностика rust-стороны: дизарм ли плейн. */
    public static boolean armed() {
        return ENABLED && !broken && probeOnce();
    }

    /**
     * Замена сайта {@code invokevirtual Level.findNearbyPlayer} в
     * Mob.checkDespawn. Возвращает Player — stack-identical замещение
     * (receiver Level consummирован, аргументы те же). Desc РОВНО
     * (Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;DLjava/util/function/Predicate;)Lnet/minecraft/world/entity/player/Player;
     * — virtual desc с receiver-классом Level, препендированным (валидатор
     * compose).
     */
    public static Player findNearbyPlayerGate(Level level, Entity entity, double distance,
            Predicate<Entity> predicate) {
        if (ENABLED && !broken && predicate != null && entity instanceof Mob mob) {
            int[] box = MobPushOps.idBoxOf(mob);
            if (box != null) {
                maybeEpoch(level, predicate);
                if (broken) {
                    return level.findNearbyPlayer(entity, distance, predicate);
                }
                if (SNAPSHOT_EMPTY) {
                    return null; // ни один игрок не проходит предикат = ванильный null
                }
                int id = box[0];
                int[] col = NEAREST;
                int len = NEAREST_LEN;
                if (id >= 0 && col != null && id < len && id < col.length) {
                    int idx = col[id];
                    if (idx < 0) {
                        return null; // rust: ни один qualifying-игрок = ванильный null
                    }
                    Player[] ps = SNAPSHOT;
                    if (idx < ps.length) {
                        if (!ARM_LOGGED) {
                            ARM_LOGGED = true;
                            LOG.info("[crussty-plugin] cmp406_sscan: despawn-scan EFFECT armed (first gate hit"
                                    + " at tick " + MinecraftServer.getServer().getTickCount()
                                    + ", denseId=" + id + ", playerIdx=" + idx + ")");
                        }
                        return ps[idx]; // ТОТ ЖЕ игрок, что вернул бы ванильный скан
                    }
                }
            }
        }
        // fail-closed: не-Mob / не в плоскости / эпоха-промах / дизарм — ваниль.
        return level.findNearbyPlayer(entity, distance, predicate);
    }

    /**
     * Одна эпоха скана на серверный тик: снапшот qualifying-игроков + ОДИН
     * bulk-JNI по всей SoA-популяции (rust пишет колонку nearest).
     * Double-checked по volatile EPOCH_TICK.
     * rc >= 0 = число записанных валидных элементов (=idTop отсканированных).
     */
    private static void maybeEpoch(Level level, Predicate<Entity> predicate) {
        long t = MinecraftServer.getServer().getTickCount();
        if (EPOCH_TICK == t) {
            return; // горячий путь: один volatile-read
        }
        synchronized (EPOCH_LOCK) {
            if (EPOCH_TICK == t || broken) {
                return;
            }
            // Снапшот qualifying-игроков: ТОТ ЖЕ предикат, ТОТ ЖЕ порядок
            // level.players() — ванильная семантика getNearestPlayer.
            List<Player> qs = new ArrayList<>();
            try {
                for (Player p : level.players()) {
                    if (predicate.test(p)) {
                        qs.add(p);
                    }
                }
            } catch (Throwable th) {
                return; // предикат кинул — этот тик ваниль (эпоха не публикуется)
            }
            if (qs.isEmpty()) {
                SNAPSHOT_EMPTY = true; // все gate-вызовы вернут null = ванильный null
                EPOCH_TICK = t;
                return;
            }
            double[] coords = new double[qs.size() * 3];
            Player[] arr = qs.toArray(new Player[0]);
            for (int i = 0; i < arr.length; i++) {
                Player p = arr[i];
                coords[i * 3] = p.getX();
                coords[i * 3 + 1] = p.getY();
                coords[i * 3 + 2] = p.getZ();
            }
            int idTop = MobPushOps.idCount();
            if (idTop <= 0) {
                // TASK-424-A empty-plane short-circuit: SoA холодная — vanilla
                // без JNI/буферов/лога на этот тик (эпоха помечена).
                SNAPSHOT_EMPTY = true;
                EPOCH_TICK = t;
                return;
            }
            int[] col = NEAREST;
            int cap = Math.max(1024, MobPushOps.idCapacity());
            if (col == null || col.length < cap) {
                col = new int[cap];
            }
            int rc;
            try {
                rc = sscanEpoch((int) t, idTop, coords, col);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                broken = true; // структурный отказ — весь рычаг в ваниль навсегда
                LOG.warning("[crussty-plugin] cmp406_sscan: sscanEpoch ERR_STRUCT — despawn scan disarmed to vanilla");
                return;
            }
            if (rc == ERR_RANGE) {
                return; // параметр-промах: весь тик ваниль, эпоха ретраится на следующем тике
            }
            SNAPSHOT_EMPTY = false;
            SNAPSHOT = arr;         // публикуем снапшот ДО колонки
            NEAREST = col;
            NEAREST_LEN = rc;       // volatile write = publication edge для читателей
            EPOCH_TICK = t;         // release-edge: читатели видят консистентную тройку
            if (!ARM_LOGGED) {
                ARM_LOGGED = true; // TASK-424-A: one-shot на publish
                LOG.info("[crussty-plugin] " + LABEL + ": epoch ok tick=" + t
                        + " mobSlots=" + rc + " players=" + arr.length
                        + " (bulk JNI 1/tick over soa population)");
            }
        }
    }
}
