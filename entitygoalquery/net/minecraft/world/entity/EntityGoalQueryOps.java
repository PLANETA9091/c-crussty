package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

import net.minecraft.server.MinecraftServer;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * EINDEX-Q (TASK-410-C, K3 PIVOT of vector R2 — lever cmp410_eindexq).
 *
 * GOAL-TARGET/SENSING QUERY CLASS: бенч-профиль (research RESEARCH-C-k3.md,
 * collapsed-cpu round409anchora/round409cleg4): цель-поиск мобов —
 * NearestAttackableTargetGoal.findTarget + AvoidEntityGoal.canUse доставляются
 * единственным чоукпойнтом
 * {@code Level.getEntitiesOfClass(Class,AABB,Predicate) ->
 * EntityLookup.getEntities -> ChunkEntitySlices.getEntities} (86-88% среза,
 * ~2.7-3.0% wall). Секционный walk moonrise на КАЖДЫЙ goal-запрос заменяется
 * плоским CHAIN-СКАНОМ РАСТ-снимка.
 *
 * РЕШЕНИЕ (rust = единственный источник данных): ОДИН bulk-JNI
 * {@code eqEpoch(tick, idTop, soa[D], head[I], next[I])} за тик — rust DOD-
 * проходом по SoA-плоскости mobs_soa (x/y/z/hw/hh/flags — ТЕ ЖЕ dense-id, что
 * MobPushOps.byId) строит за ОДИН линейный проход intrusive-chain хеш-таблицу:
 * bucket = hash(floor(x/16), floor(z/16)) из 65536; head[h] = id+1 голова
 * цепочки, next[id] = id+1 следующий (0 = конец); soa[] = замороженные колонки
 * [x,y,z,hw,hh] stride 5 того же прохода. ПОЧЕМУ ЦЕПИ, А НЕ CSR-СМЕЩЕНИЯ:
 * 2-проходный CSR (counts → prefix → fill) требует глобальной консистентности
 * снапшота — seqlock-ретраи под непрерывным писателем (150k upserts/тик)
 * никогда не сходятся; однопроходные цепи самосогласованны при ЛЮБОМ
 * перемешивании писателей: каждый id попадает ровно в одну цепь (хэш ТЕХ ЖЕ
 * замороженных x/z, что в колонках) — snapshot всегда структурно валиден,
 * «рваная» строка даёт призрака на ≤1 тик у коробочной границы (маржа 8 блоков
 * ≫ tick-дрейфа мобов). Java-сторона query = чистые чтения массивов (ноль
 * per-query JNI): диапазон ячеек AABB±MARGIN → цепи → SoA-прун (замороженные
 * колонки vs AABB±MARGIN) → byId[id] → level()/class/bb.intersects (ТОЧНЫЙ
 * ванильный тест на ЖИВОМ bb) → predicate. per-entity JNI отсутствует
 * (закон 6); JNI = 1 bulk/тик + rust-проход по плоской SoA памяти.
 *
 * ВАНИЛЬНОСТЬ (суперсет-контракт): кандидаты = strict-надмножество ванильного
 * перечисления для ОВЕЩЕСТВЛЕННЫХ классов: гейт обслуживает ТОЛЬКО cls с
 * Mob.class.isAssignableFrom(cls) || Player.class.isAssignableFrom(cls) —
 * полный универсе LivingEntity-кандидатов (все ванильные Mob'ы в SoA через
 * push-плоскость: LivingEntity.aiStep → pushEntities → self-upsert каждый
 * тик; игроки всегда из level.players()). ArmorStand/boat/etc — ванильный
 * путь (гейт не обслуживает). ПРЕЦИЗИОННЫЙ фильтр =
 * {@code entity.getBoundingBox().intersects(box)} — тот же тест, что ваниль;
 * SoA-колонки (замороженные позиции, hw>=полуэкстентов) — только prune с
 * МАРЖОЙ MARGIN=8.0 блоков (≥4× максимального tick-перемещения бенч-моба;
 * коробка пруна = box.inflate(MARGIN) — та же, по которой набраны ячейки).
 * Порядок кандидатов = chain/bucket-порядок (документированная дельта класса
 * items_subsys2/mobpush; потребители — getNearestEntity: строго-ближайший,
 * лестница «первый строго-ближе», от порядка не зависит кроме точных
 * double-равенств). СВЕЖЕСТЬ: строки SoA = end-of-last-move (идентично
 * контракту СВЕЖЕСТИ push-плоскости, TASK-401-E); дрейф замороженного
 * снапшота = перемещение моба в окне (epoch..query) текущего тика ≤ 1 тик
 * движения ≪ MARGIN; новый спавн до первого upsert невидим ≤1 тик (тот же
 * принятый дельта-класс); удалённый-до-свеепа моб даёт лишнего кандидата
 * (фильтруется живым bb + isAlive-предикатами цепочки targeting — ваниль
 * возвращает их ровно до removeEntity из lookup'а). Хвост findTarget/canUse
 * ПОСЛЕ списка (getNearestEntity, TargetingConditions, pathfind) —
 * НЕТРОНУТЫЙ ванильный байткод. Profiler-счётчик «getEntities» из ванильного
 * тела getEntitiesOfClass воспроизведён на snapshot-пути (прецедент
 * MobPushOps.pushables).
 *
 * FAIL-CLOSED: ENABLED STRICT-eq "cmp410_eindexq"|"cmp411_k4soa" (пустой/чужой
 * флаг — сайт вообще не ретаргетится rust-стороной); SoA-плоскость не готова
 * (MobPushOps.planeReady()==false: broken/oversized/probe) → ваниль;
 * idTop==0 (плоскость холодная) → ваниль на этот тик; rc==0 при idTop>0
 * (холодные таблицы) → ваниль на этот тик (эпоха ретраится следующим);
 * eqProbe magic-mismatch / eqEpoch ERR_STRUCT → дизарм навсегда (ваниль
 * каждый тик); ERR_RANGE / структурный дрейф снапшота / абсурдный rect →
 * ваниль на этот вызов. Публикация снапшота — volatile-четвёрка (SOA, HEAD,
 * NEXT) c SNAP_ROWS/EPOCH_TICK как release-edge (лестница MobScanOps
 * TASK-406-E). ИМЯ КЛАССА EntityGoalQueryOps: FQCN net.minecraft.world.entity.
 * EntityQueryOps УЖЕ ЗАНЯТ alloc-diet бриджем S7-133 (entityquery/, dormant
 * под чужим флагом) — дубль-определение в тот же loader = LinkageError.
 */
public final class EntityGoalQueryOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // STRICT eq (TASK-402-F урок полу-armed гейта): только точный флаг
        // раунда-410 / раунда-411. База/другие рычаги этот мост не вызывают
        // (rust не ставит сайт).
        // TASK-411-C (k4soa): K4-нога — та же снапшот-механика; отличия:
        // популяция с радиус-гейтом 2.0 (rust pad 2) + новый пакет-приватный
        // pushCandidates (push-лейн из снапшота, вызывается MobPushOps.pushables).
        // TASK-411-C (eqsnap, v2): та же механика; upserts идут в
        // пер-потоковые delta-шарды руста (0 локов), eq_epoch сливает их
        // одним bulk-drain O(dirty) перед chain-build (пост-мортем cl1
        // 35691270899: per-entity WLOCK-мутации = 24.9% CPU → 0.5 TPS).
        return f != null && (f.trim().equals("cmp410_eindexq")
                || f.trim().equals("cmp411_k4soa")
                || f.trim().equals("cmp411_eqsnap")
                // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR;
                // cmp412_meganav-сайты остаются нетронутыми.
                || f.trim().equals("cmp412_eqsnapv3"));
    }

    /** TASK-411-C (k4soa): K4-режим (маркировка EFFECT-строк). */
    private static boolean k4Mode() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp411_k4soa");
    }

    private static final boolean ENABLED = leverEnabled();
    private static final boolean K4 = k4Mode();
    /** Метка флага для EFFECT/диагностических строк (одна из ARM-пар). */
    private static final String FLAG_LABEL;
    static {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        FLAG_LABEL = f != null && f.trim().equals("cmp412_eqsnapv3")
                ? "cmp412_eqsnapv3" // TASK-412-C (eqsnap-v3): точная метка.
                : (f != null && f.trim().equals("cmp411_eqsnap")
                        ? "cmp411_eqsnap" : (K4 ? "cmp411_k4soa" : "cmp410_eindexq"));
    }

    private static final int PROBE_MAGIC = 0x4547; // "EG"
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/entity_query.rs, RegisterNatives после define) ----
    private static native int eqProbe();

    /**
     * ONE bulk JNI per tick: chain-снимок SoA-популяции mobs_soa. rc >= 0 =
     * число СВЯЗАННЫХ строк (аливных с конечными координатами); ERR_RANGE —
     * параметр-промах/холодные таблицы (ваниль на этот тик); ERR_STRUCT —
     * дизарм.
     */
    private static native int eqEpoch(int tick, int idTop, double[] soa,
            int[] head, int[] next);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    /** One-shot ARM/effect-пруфы (видны в server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;
    private static volatile boolean EPOCH_LOGGED = false;

    // ---- снапшот (volatile publication ladder как MobScanOps) ----
    /** Замороженные колонки [x,y,z,hw,hh] × row (stride STRIDE). */
    private static volatile double[] SOA = new double[0];
    /** Головы цепей по bucket'ам: id+1, 0 = пустая цепь. Length CELLS. */
    private static volatile int[] HEAD = new int[0];
    /** Следующий в цепи: id+1, 0 = конец. Indexed by dense id. */
    private static volatile int[] NEXT = new int[0];
    /** Граница валидных dense-id снапшота (= idTop эпохи). */
    private static volatile int SNAP_ROWS = 0;
    /** Серверный тик последней успешной эпохи (double-checked locking). */
    private static volatile long EPOCH_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    // ---- геометрия снапшота: ДОЛЖЕН совпадать с src/entity_query.rs ----
    static final int CELLS = 1 << 16; // 65536 bucket'ов
    static final int CELL_SIZE = 16;  // блоков на ребро ячейки
    static final int STRIDE = 5;      // x,y,z,hw,hh на ряд
    /**
     * Маржа суперсета: прямоугольник ячеек и SoA-прун строятся по
     * box.inflate(MARGIN); живой bb-тест — по НЕизменённой коробке. 8 блоков
     * ≫ максимального tick-дрейфа бенч-мобов (~1-2 блока) и ≥ 1.0-радиуса
     * гейта плоскости — снапшот-прун не может скрыть живого кандидата.
     */
    static final double MARGIN = 8.0D;

    /**
     * TASK-411-C (k4soa): scratch для дедупликации bucket'ов push-прямоугольника
     * (pushCandidates; окно ≤ 64 bucket'ов). Per-thread, zero per-query alloc —
     * pushables зовётся каждым living-энтити каждый тик (150k масштаб),
     * аллокация на запрос недопустима (GC-давление = обратный эффект).
     */
    private static final ThreadLocal<int[]> BUCKET_SCRATCH =
            ThreadLocal.withInitial(() -> new int[64]);

    private EntityGoalQueryOps() {}

    /** Ленивая проверка натива (Throwable -> false, ретрай на следующем тике). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (EntityGoalQueryOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = eqProbe() == PROBE_MAGIC;
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

    /** Совпадает с rust cell_hash: (cx*0x9E3779B1) ^ (cz*0x85EBCA77), ^сам>>>16, маска. */
    private static int cellHash(int cx, int cz) {
        int h = cx * 0x9E3779B1 ^ cz * 0x85EBCA77;
        h ^= h >>> 16;
        return h & (CELLS - 1);
    }

    private static int floorCell(double v) {
        return (int) Math.floor(v / (double) CELL_SIZE);
    }

    /**
     * Замена сайта {@code invokevirtual Level.getEntitiesOfClass} в
     * NearestAttackableTargetGoal.findTarget и AvoidEntityGoal.canUse.
     * Stack-identical: receiver Level consummирован, desc = virtual desc с
     * препендированным receiver-классом Level (валидатор compose).
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static List<Entity> entitiesOfClassGate(Level level, Class<? extends Entity> cls,
            AABB box, Predicate<? super Entity> pred) {
        if (ENABLED && !broken && pred != null && box != null && cls != null
                // суперсет-гвардия универсе кандидатов (см. class doc):
                && (Mob.class.isAssignableFrom(cls) || Player.class.isAssignableFrom(cls))
                && MobPushOps.planeReady()) {
            maybeEpoch();
            if (!broken) {
                List<Entity> out = snapshotQuery(level, cls, box, pred);
                if (out != null) {
                    // Ванильное тело getEntitiesOfClass начинается с
                    // Profiler-счётчика — реплика на snapshot-пути (fallback
                    // ниже инкрементирует СВОЙ счётчик внутри ванильного тела).
                    Profiler.get().incrementCounter("getEntities");
                    if (!ARM_LOGGED) {
                        ARM_LOGGED = true;
                        LOG.info("[crussty-plugin] " + FLAG_LABEL + ": goal-query EFFECT armed"
                                + " (first gate hit at tick "
                                + MinecraftServer.getServer().getTickCount()
                                + ", class=" + cls.getSimpleName() + ")");
                    }
                    return out;
                }
            }
        }
        // fail-closed: не-Mob/Player-класс / плоскость не готова / эпоха-промах /
        // дизарм — ТОЧНО ванильное тело (Level.getEntitiesOfClass final body);
        // raw-cast стирается к тому же invokevirtual-дескриптору.
        return level.getEntitiesOfClass((Class) cls, box, pred);
    }

    /**
     * Одна эпоха снапшота на серверный тик: ОДИН bulk-JNI eqEpoch строит
     * chain-таблицу по всей SoA-популяции. Double-checked по volatile
     * EPOCH_TICK. Гейт-вызовы приходят с region-worker потоков — гонка
     * закрывается лестницей публикации (volatile-четвёрка + EPOCH_TICK
     * release-edge).
     */
    private static void maybeEpoch() {
        long t = MinecraftServer.getServer().getTickCount();
        if (EPOCH_TICK == t) {
            return; // горячий путь: один volatile-read
        }
        synchronized (EPOCH_LOCK) {
            if (EPOCH_TICK == t || broken) {
                return;
            }
            int idTop = MobPushOps.idCount();
            if (idTop <= 0) {
                return; // плоскость холодная — суперсет не доказан, ваниль этот тик
            }
            int cap = Math.max(idTop, MobPushOps.idCapacity());
            double[] soa = SOA;
            if (soa.length < (long) cap * STRIDE) {
                soa = new double[cap * STRIDE];
            }
            int[] head = HEAD;
            if (head.length != CELLS) {
                head = new int[CELLS];
            }
            int[] next = NEXT;
            if (next.length < cap) {
                next = new int[cap];
            }
            int rc;
            try {
                rc = eqEpoch((int) t, idTop, soa, head, next);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                broken = true; // структурный отказ — весь рычаг в ваниль навсегда
                LOG.warning("[crussty-plugin] " + FLAG_LABEL + ": eqEpoch ERR_STRUCT — goal-query disarmed to vanilla");
                return;
            }
            if (rc == ERR_RANGE) {
                return; // параметр-промах: весь тик ваниль, эпоха ретраится следующим
            }
            if (rc == 0) {
                // Холодные таблицы (ни одной живой строки) при idTop>0 —
                // публиковать ПУСТОЙ снапшот НЕЛЬЗЯ (пропуск кандидатов =
                // парити-фантом); ваниль этот тик, эпоха ретраится следующим.
                return;
            }
            SOA = soa;              // публикуем колонки ДО цепей
            HEAD = head;
            NEXT = next;
            SNAP_ROWS = idTop;      // volatile write = publication edge для читателей
            EPOCH_TICK = t;         // release-edge: читатели видят консистентную тройку
            if (!EPOCH_LOGGED) {
                EPOCH_LOGGED = true;
                LOG.info("[crussty-plugin] " + FLAG_LABEL + ": epoch ok tick=" + t
                        + " rows=" + idTop + " linked=" + rc
                        + " (bulk JNI 1/tick over mobs_soa SoA population, chain cells="
                        + CELLS + ")");
            }
        }
    }

    /**
     * Плоский chain-скан снапшота. @return null — структурный дрейф/не
     * поддающийся запрос (ваниль на этот вызов); иначе список кандидатов
     * (суперсет → точные ванильные фильтры внутри).
     */
    private static List<Entity> snapshotQuery(Level level, Class<? extends Entity> cls,
            AABB box, Predicate<? super Entity> pred) {
        final double[] soa = SOA;
        final int[] head = HEAD;
        final int[] next = NEXT;
        final int rows = SNAP_ROWS;
        if (rows <= 0 || soa.length < (long) rows * STRIDE
                || head.length != CELLS || next.length < rows) {
            return null; // структурный дрейф — ваниль на этот вызов
        }
        // Прямоугольник ячеек по РАСШИРЕННОЙ коробке (та же маржа, по которой
        // rust строил прун-контракт) — снапшот-перечисление не может скрыть
        // живого кандидата с tick-дрейфом позиции.
        final double mx0 = box.minX - MARGIN, mx1 = box.maxX + MARGIN;
        final double mz0 = box.minZ - MARGIN, mz1 = box.maxZ + MARGIN;
        final int cx0 = floorCell(mx0);
        final int cx1 = floorCell(mx1);
        final int cz0 = floorCell(mz0);
        final int cz1 = floorCell(mz1);
        if ((long) (cx1 - cx0) * (long) (cz1 - cz0) > 4096L) {
            return null; // абсурдный rect (far-land/portal abuse) — ваниль
        }
        final Entity[] byId = MobPushOps.byIdArr();
        if (byId == null || byId.length == 0) {
            return null;
        }
        final double bx0 = box.minX, by0 = box.minY, bz0 = box.minZ;
        final double bx1 = box.maxX, by1 = box.maxY, bz1 = box.maxZ;
        ArrayList<Entity> out = new ArrayList<>(32);
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                // Две разные ячейки прямоугольника могут дать один bucket —
                // цепь пройдётся дважды; кандидаты-дубликаты безвредны для
                // потребителя (getNearestEntity: строго-ближайший — тот же
                // пик; лестница «первый строго-ближе» не зависит от дубля).
                int h = cellHash(cx, cz);
                for (int link = head[h]; link != 0; link = next[link - 1]) {
                    int id = link - 1;
                    if (id < 0 || id >= rows) {
                        break; // структурный дрейф цепи — ваниль на этот вызов
                    }
                    int b = id * STRIDE;
                    double x = soa[b];
                    double z = soa[b + 2];
                    double hw = soa[b + 3];
                    // SoA-прун по замороженным колонкам против РАСШИРЕННОЙ
                    // коробки (superset живого AABB.intersects: hw >= x/z
                    // полуэкстентов; маржа ≥ tick-дрейфа). y не пруним:
                    // вертикальный дрейф (падение) не ограничен маржой по x/z.
                    if (x - hw >= mx1 || x + hw <= mx0
                            || z - hw >= mz1 || z + hw <= mz0) {
                        continue;
                    }
                    Entity cand = byId[id];
                    if (cand == null || cand.level() != level) {
                        continue;
                    }
                    if (!cls.isInstance(cand)) {
                        continue;
                    }
                    // ТОЧНЫЙ ванильный тест на ЖИВОМ bb — идентичен
                    // EntityLookup.getEntities пер-кандидатному фильтру.
                    if (!cand.getBoundingBox().intersects(box)) {
                        continue;
                    }
                    if (pred.test(cand)) {
                        out.add(cand);
                    }
                }
            }
        }
        // Player-классы НЕ обслуживаются SoA-перечислением (level.players()
        // — authoritative источник; ≤ fake_players+1 объектов): без этого
        // Player.class-запросы теряли бы игроков = нарушение суперсет-
        // контракта. Фильтры те же точные ванильные.
        if (Player.class.isAssignableFrom(cls)) {
            for (Player p : level.players()) {
                if (p == null || p.level() != level || !cls.isInstance(p)) {
                    continue;
                }
                if (!p.getBoundingBox().intersects(box)) {
                    continue;
                }
                if (pred.test(p)) {
                    out.add(p);
                }
            }
        }
        return out;
    }

    /**
     * TASK-411-C (k4soa): push-кандидаты из chain-снапшота (0 per-query JNI).
     * Вызывается MobPushOps.pushables под флагом cmp411_k4soa ПЕРЕД легаси
     * mobQuery. Универс = SoA-популяция (ТОТ ЖЕ контракт round-401, что и
     * mobQuery); предикат = EntitySelector.pushableBy(entity); фильтры:
     * level, other != entity, живой bb.intersects(box).
     *
     * ДЕДУП ПО БУКЕТАМ (не по ячейкам!): две разные ячейки прямоугольника с
     * одним bucket дали бы двойной проход одной цепи = дубликат-кандидата =
     * двойной doPush (для nearest-пика дубликаты безвредны — для push tail
     * НЕТ). Прямоугольник push-запроса мал (box + MARGIN ≈ 2-3 ячейки на
     * ось) — попарная дедупликация ≤ 64 bucket'ов дешева.
     *
     * @return false — не обслужено (снапшот не готов / структурный дрейф /
     *         абсурдный rect; out НЕ тронут или очищен), иначе true и out =
     *         кандидаты без дубликатов.
     */
    static boolean pushCandidates(Level level, Entity entity, AABB box,
            java.util.List<Entity> out) {
        if (broken) {
            return false;
        }
        maybeEpoch();
        final double[] soa = SOA;
        final int[] head = HEAD;
        final int[] next = NEXT;
        final int rows = SNAP_ROWS;
        if (rows <= 0 || soa.length < (long) rows * STRIDE
                || head.length != CELLS || next.length < rows) {
            return false; // структурный дрейф/холодный снапшот — легаси путь
        }
        // Тот же расширенный прямоугольник, что snapshotQuery: маржа покрывает
        // радиус населения (≤ MobPushOps.RADIUS_GATE = 2.0) + tick-дрейф.
        final double mx0 = box.minX - MARGIN, mx1 = box.maxX + MARGIN;
        final double mz0 = box.minZ - MARGIN, mz1 = box.maxZ + MARGIN;
        final int cx0 = floorCell(mx0);
        final int cx1 = floorCell(mx1);
        final int cz0 = floorCell(mz0);
        final int cz1 = floorCell(mz1);
        final long rw = (long) (cx1 - cx0) + 1L, rh = (long) (cz1 - cz0) + 1L;
        if (rw * rh > 64L) {
            return false; // абсурдный/гигантский rect — легаси путь
        }
        final Entity[] byId = MobPushOps.byIdArr();
        if (byId == null || byId.length == 0) {
            return false;
        }
        // Дедуп bucket'ов прямоугольника (scratch переиспользуется — 0 alloc).
        final int cnt = (int) (rw * rh);
        final int[] buckets = BUCKET_SCRATCH.get();
        int n = 0;
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                int h = cellHash(cx, cz);
                boolean dup = false;
                for (int i = 0; i < n; i++) {
                    if (buckets[i] == h) {
                        dup = true;
                        break;
                    }
                }
                if (!dup) {
                    buckets[n++] = h;
                }
            }
        }
        final Predicate<? super Entity> pred = EntitySelector.pushableBy(entity);
        for (int i = 0; i < n; i++) {
            final int h = buckets[i];
            for (int link = head[h]; link != 0; link = next[link - 1]) {
                final int id = link - 1;
                if (id < 0 || id >= rows) {
                    out.clear();
                    return false; // структурный дрейф цепи — легаси путь
                }
                final int b = id * STRIDE;
                final double x = soa[b];
                final double z = soa[b + 2];
                final double hw = soa[b + 3];
                // SoA-прун по замороженным колонкам против РАСШИРЕННОЙ
                // коробки (superset живого AABB.intersects; y не пруним —
                // вертикальный дрейф не ограничен маржой по x/z).
                if (x - hw >= mx1 || x + hw <= mx0
                        || z - hw >= mz1 || z + hw <= mz0) {
                    continue;
                }
                final Entity cand = byId[id];
                if (cand == null || cand == entity || cand.level() != level) {
                    continue;
                }
                // ТОЧНЫЙ ванильный тест на ЖИВОМ bb.
                if (!cand.getBoundingBox().intersects(box)) {
                    continue;
                }
                if (pred.test(cand)) {
                    out.add(cand);
                }
            }
        }
        return true;
    }
}
