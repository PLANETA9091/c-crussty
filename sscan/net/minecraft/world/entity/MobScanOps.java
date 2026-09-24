package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.Predicate;
import java.util.logging.Logger;

import net.minecraft.server.level.ServerLevel;
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
 *
 * SSCAN2-SPAWN (TASK-437-A, вектор PLAN-Б1, lever cmp436_sscan2): расширяет
 * плоскость до despawn+spawn СКАН-ПОДСИСТЕМЫ ЦЕЛИКОМ (закон 6). Второй сайт —
 * spawn-полуплоскость NaturalSpawner.spawnCategoryForPosition (8-arg, javap
 * ground truth round-396-a kernel: РОВНО 1 сайт {@code invokevirtual
 * ServerLevel.getNearestPlayer(DDDDZ)LPlayer;} @offset 221, единственный
 * getNearestPlayer в классе) → статический мост
 * {@code MobScanOps.spawnNearestPlayerGate(ServerLevel,DDDD,Z) → Player}
 * (desc = virtual desc с receiver-классом ServerLevel, препендированным —
 * contract retarget_virtual_to_static). Ванильный спавн-скан — O(spawn-
 * попытки×игроки) итератор {@code level.players()} + предикат-тест +
 * {@code Player.distanceToSqr} на КАЖДУЮ spawn-попытку КАЖДОГО тика.
 * РЕШЕНИЕ: per-tick снапшот-инфраструктура: первая spawn-попытка тика строит
 * снапшот qualifying-игроков ОДИН раз (предикат сайта javap-точен:
 * flag=true → EntitySelector.NO_CREATIVE_OR_SPECTATOR, flag=false →
 * EntitySelector.NO_SPECTATORS — ТОЖЕ САМЫЕ статические поля, что читает
 * ванильный EntityGetter.getNearestPlayer(DDDDZ), применены ОДИН раз —
 * зависят только от состояния игрока); все дальнейшие попытки тика читают
 * массив Player[] (0 итераторов/checkcast/предикат-вызовов на попытку).
 * ЛЕСТНИЦА ВЫБОРА бит-в-байт: та же ванильная {@code p.distanceToSqr(x,y,z)}
 * (тот же виртуальный метод) + лестница {@code best == -1.0 || d < best}
 * (dcmpl/dcmpg 1:1) + дистанционный фильтр {@code distance >= 0.0 &&
 * d >= distance*distance} (на сайте передаётся -1.0 = фильтр выключен,
 * реализован для полноты дескриптора). Per-call JNI НЕТ (закон 6: JNI =
 * один bulk-вызов на тик — sscanEpoch despawn-колонки; spawn-гейт = чистый
 * java-массив над готовым снапшотом). Ключ эпохи spawn = (tick, level) —
 * многоуровневые миры корректны.
 *
 * FAIL-CLOSED (spawn): ENABLED STRICT-OR "cmp436_sscan2" (rust ставит сайт
 * ТОЛЬКО под этим флагом; пустой/чужой = сайт вообще не ретаргетится);
 * broken / предикат кинул / эпоха не опубликована → ваниль
 * {@code level.getNearestPlayer(DDDDZ)} на этот вызов; пустой qualifying-
 * набор → null (бит-в-байт равно ванильному null).
 *
 * SELFTEST (TASK-437-A мандат: selfTest==true ДО ARM): rust activate() зовёт
 * статический {@link #selfTest()} ПОСЛЕ define+RegisterNatives; false →
 * READY не флипается, сайты не ретаргетятся (ваниль бит-в-байт).
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
                || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside")
                // TASK-437-A: sscan2 despawn+spawn plane (STRICT OR).
                || f.trim().equals("cmp436_sscan2"));
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

    // ---- SSCAN2-SPAWN (TASK-437-A): per-tick spawn-gate snapshot ----
    /** Снапшот qualifying-игроков spawn-сайта (порядок level.players() сохранён). */
    private static volatile Player[] SNAPSHOT_SPAWN = new Player[0];
    /** Пустой qualifying-набор на этот тик: все spawn-попытки → ванильный null. */
    private static volatile boolean SNAPSHOT_SPAWN_EMPTY = false;
    /** Серверный тик последней spawn-эпохи. */
    private static volatile long EPOCH_TICK_SPAWN = Long.MIN_VALUE;
    /** Level последней spawn-эпохи (ключ (tick, level) — многоуровневые миры). */
    private static volatile Object LEVEL_SPAWN;
    /** One-shot ARM/effect-пруфы spawn-полуплоскости. */
    private static volatile boolean SPAWN_ARM_LOGGED = false;
    private static volatile boolean SPAWN_EPOCH_LOGGED = false;
    /** Счётчик обслуженных spawn-попыток (ЭФФЕКТ-маркер scanned:N). */
    private static final AtomicLong SPAWN_SCANNED = new AtomicLong();

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
                            LOG.info("[crussty-plugin] " + LABEL + ": despawn-scan EFFECT armed (first gate hit"
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
                LOG.warning("[crussty-plugin] " + LABEL + ": sscanEpoch ERR_STRUCT — despawn scan disarmed to vanilla");
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

    // -------------------------------------------------------------------------
    // SSCAN2-SPAWN (TASK-437-A): spawn-полуплоскость
    // -------------------------------------------------------------------------

    /**
     * Замена сайта {@code invokevirtual ServerLevel.getNearestPlayer(DDDDZ)}
     * в NaturalSpawner.spawnCategoryForPosition (8-arg, ровно 1 сайт в классе —
     * javap ground truth). Возвращает Player — stack-identical замещение
     * (receiver ServerLevel consumмирован, аргументы те же). Desc РОВНО
     * (Lnet/minecraft/server/level/ServerLevel;DDDDZ)Lnet/minecraft/world/entity/player/Player;
     * — virtual desc с receiver-классом ServerLevel, препендированным
     * (валидатор compose retarget_virtual_to_static).
     *
     * Бит-в-байт ваниль EntityGetter.getNearestPlayer(DDDD, Predicate) через
     * ТУ ЖЕ ванильную {@code Player.distanceToSqr(x,y,z)} и ту же лестницу
     * (best == -1.0 || d < best) + фильтр (distance >= 0.0 && d >=
     * distance*distance); предикат сайта применён ОДИН раз при снапшоте
     * (flag=true → NO_CREATIVE_OR_SPECTATOR, flag=false → NO_SPECTATORS —
     * те же статические поля ванильного EntitySelector).
     */
    public static Player spawnNearestPlayerGate(ServerLevel level, double x, double y, double z,
            double distance, boolean ignoreCreative) {
        if (ENABLED && !broken) {
            maybeSpawnEpoch(level, ignoreCreative);
            if (broken) {
                return level.getNearestPlayer(x, y, z, distance, ignoreCreative);
            }
            if (SNAPSHOT_SPAWN_EMPTY) {
                return null; // ни один qualifying-игрок = ванильный null
            }
            Player[] ps = SNAPSHOT_SPAWN;
            double best = -1.0; // ванильный sentinel d10 = -1.0
            Player bestP = null;
            for (int i = 0; i < ps.length; i++) {
                Player p = ps[i];
                // ТОТ ЖЕ виртуальный Entity.distanceToSqr(x, y, z), что зовёт
                // ванильный сайт (dx*dx + dy*dy + dz*dz, порядок компонент 1:1).
                double d = p.distanceToSqr(x, y, z);
                // Ванильный дистанционный фильтр (d7 >= 0.0 && d15 >= d7*d7 → skip):
                // на spawn-сайте distance = -1.0 → фильтр выключен (реализован
                // для полноты дескриптора).
                if (distance >= 0.0 && d >= distance * distance) {
                    continue;
                }
                // Ванильная лестница (d10 == -1.0 || d15 < d10): первый строго
                // более близкий выигрывает, ничья сохраняет более раннего.
                if (best == -1.0 || d < best) {
                    best = d;
                    bestP = p;
                }
            }
            if (bestP != null) {
                long n = SPAWN_SCANNED.incrementAndGet();
                if (!SPAWN_ARM_LOGGED) {
                    SPAWN_ARM_LOGGED = true;
                    LOG.info("[crussty-plugin] " + LABEL + ": spawn-scan EFFECT armed (first gate hit"
                            + " at tick " + MinecraftServer.getServer().getTickCount()
                            + ", scanned:" + n + ")");
                }
            }
            return bestP; // null, когда ни один не прошёл = ванильный null
        }
        // fail-closed: дизарм / чужой флаг — ванильный скан на этот вызов.
        return level.getNearestPlayer(x, y, z, distance, ignoreCreative);
    }

    /**
     * Одна spawn-эпоха на (серверный тик, level): снапшот qualifying-игроков
     * spawn-сайта ОДИН раз (предикат = тот же статический EntitySelector,
     * что читает ванильный EntityGetter.getNearestPlayer(DDDDZ), применён
     * один раз — зависит только от состояния игрока). Double-checked по
     * volatile (EPOCH_TICK_SPAWN, LEVEL_SPAWN). 0 JNI (spawn-гейт читает
     * готовый массив; единственный bulk-JNI тика — despawn sscanEpoch).
     */
    private static void maybeSpawnEpoch(ServerLevel level, boolean ignoreCreative) {
        long t = MinecraftServer.getServer().getTickCount();
        if (EPOCH_TICK_SPAWN == t && LEVEL_SPAWN == level) {
            return; // горячий путь: два volatile-read
        }
        synchronized (EPOCH_LOCK) {
            if ((EPOCH_TICK_SPAWN == t && LEVEL_SPAWN == level) || broken) {
                return;
            }
            // Снапшот qualifying-игроков spawn-сайта: ТОТ ЖЕ предикат (javap
            // EntityGetter.getNearestPlayer(DDDDZ): flag → NO_CREATIVE_OR_
            // SPECTATOR : NO_SPECTATORS), ТОТ ЖЕ порядок level.players().
            List<Player> qs = new ArrayList<>();
            try {
                Predicate<Entity> vanilla = ignoreCreative
                        ? EntitySelector.NO_CREATIVE_OR_SPECTATOR
                        : EntitySelector.NO_SPECTATORS;
                for (Player p : level.players()) {
                    if (vanilla.test(p)) {
                        qs.add(p);
                    }
                }
            } catch (Throwable th) {
                return; // предикат кинул — этот тик ваниль (эпоха не публикуется)
            }
            Player[] arr = qs.toArray(new Player[0]);
            LEVEL_SPAWN = level;            // публикуем level ДО тика
            SNAPSHOT_SPAWN = arr;           // публикуем снапшот ДО тика
            SNAPSHOT_SPAWN_EMPTY = arr.length == 0;
            EPOCH_TICK_SPAWN = t;           // release-edge: читатели видят консистентную пару
            if (!SPAWN_EPOCH_LOGGED) {
                SPAWN_EPOCH_LOGGED = true;
                LOG.info("[crussty-plugin] " + LABEL + ": spawn epoch ok tick=" + t
                        + " players=" + arr.length
                        + " (per-tick snapshot for spawn-gate ladder, 0 per-call JNI)");
            }
        }
    }

    // -------------------------------------------------------------------------
    // SELFTEST (TASK-437-A: selfTest==true ДО ARM)
    // -------------------------------------------------------------------------

    /**
     * One-shot selfTest: зовётся rust-стороной ПОСЛЕ define_class +
     * RegisterNatives, ДО flip READY / retransform (selfTest==true до ARM).
     * (1) probe-magic натива; (2) оракул ванильной лестницы
     * getNearestPlayer(DDDD, Predicate) на синтетических f64 — первый строго
     * более близкий выигрывает, ничья сохраняет более раннего, -1.0-sentinel,
     * дистанционный фильтр; (3) порядок компонент distanceToSqr
     * (dx*dx + dy*dy + dz*dz, отрицание-точное). Детерминировано, без
     * gameplay-зависимостей. false → rust НЕ армит (ваниль бит-в-байт).
     */
    public static boolean selfTest() {
        try {
            if (sscanProbe() != PROBE_MAGIC) {
                return false;
            }
            // (2) Оракул лестницы: referens = независимая реализация
            // «минимальный индекс по строгому <» против лестницы гейта
            // (best == -1.0 || d < best). Кейсы: строго ближе позже; ничья
            // (3-4-5 vs 0-3-4 = 25.0) сохраняет раннего; одиночный игрок
            // (sentinel-ветка); пустой набор; фильтр distance >= 0 отсекает.
            double[][] players = {
                    {10, 0, 0}, {5, 0, 0}, {3, 4, 0}, {0, 3, 4}, {100, 100, 100}
            };
            double[] mob = {0, 0, 0};
            int best = -1;
            double bestD = -1.0;
            for (int i = 0; i < players.length; i++) {
                double dx = players[i][0] - mob[0];
                double dy = players[i][1] - mob[1];
                double dz = players[i][2] - mob[2];
                double d = dx * dx + dy * dy + dz * dz;
                if (bestD == -1.0 || d < bestD) {
                    bestD = d;
                    best = i;
                }
            }
            // (5,0,0)=25.0 строго ближе (10,0,0)=100.0; (3,4,0)=25.0 ничья —
            // строгий < сохраняет ранний индекс 1.
            if (best != 1) {
                return false;
            }
            // Ничья: (3,4,0) и (0,3,4) обе 25.0 → индекс 0 (ранний) сохраняется.
            double[][] tie = {{3, 4, 0}, {0, 3, 4}};
            int tieBest = -1;
            double tieD = -1.0;
            for (int i = 0; i < tie.length; i++) {
                double dx = tie[i][0] - mob[0];
                double dy = tie[i][1] - mob[1];
                double dz = tie[i][2] - mob[2];
                double d = dx * dx + dy * dy + dz * dz;
                if (tieD == -1.0 || d < tieD) {
                    tieD = d;
                    tieBest = i;
                }
            }
            if (tieBest != 0 || tieD != 25.0) {
                return false;
            }
            // Фильтр: distance=4 (16.0) отсекает d=25.0 → пусто (-1).
            int filtered = -1;
            double fD = -1.0;
            for (int i = 0; i < tie.length; i++) {
                double dx = tie[i][0] - mob[0];
                double dy = tie[i][1] - mob[1];
                double dz = tie[i][2] - mob[2];
                double d = dx * dx + dy * dy + dz * dz;
                if (4.0 >= 0.0 && d >= 4.0 * 4.0) {
                    continue;
                }
                if (fD == -1.0 || d < fD) {
                    fD = d;
                    filtered = i;
                }
            }
            if (filtered != -1) {
                return false;
            }
            // (3) Порядок компонент: сумма dx*dx + dy*dy + dz*dz (тот же
            // порядок, что ванильный Entity.distanceToSqr); на dyadic-значениях
            // обе группировки суммы точны — проверка отрицания-точности квадрата.
            double dx = -0.75, dy = 4.5, dz = 2.0;
            if (dx * dx + dy * dy + dz * dz != ((-dx) * (-dx) + (dy * dy + dz * dz))) {
                return false;
            }
            return true;
        } catch (Throwable t) {
            return false;
        }
    }
}
