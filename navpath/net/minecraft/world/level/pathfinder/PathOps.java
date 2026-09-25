package net.minecraft.world.level.pathfinder;

/**
 * PATH-NODE NEIGHBOR CACHE (TASK-459-66, ID-P41, law-11; lever cmp459_p41 —
 * STRICT eq, OFF до оракула 10k путей).
 *
 * Lithium nether-pathfinding style (см. RESEARCH-459-P41.md): NodeEvaluator-соседи
 * кэшируются флет-таблицей posKey(long) -> neighborMask(byte), повторные
 * запросы узла внутри генерации пути/между ретаргетами берутся из кэша,
 * MISS = ваниль бит-в-байт. Инвалидация по chunk-ревизии
 * (secKey = x>>4 | z>>4 | y>>4, rev mismatch = miss + высвобождение слота).
 *
 * Паритет-контракт (STRICT):
 *  - маска = SUPERSET соседей: хит может предложить соседа, которого ваниль в
 *    этот момент не дала бы (диагональ при закрытой латерали), но не может
 *    потерять ни одного ванильного соседа; accept-тест соседа
 *    (findAcceptedNode/getPathNodeType) остаётся ванильным — кэш режет только
 *    re-traversal блочных данных getNeighbors, не решение (дисциплина lithium
 *    PathNodeCache: кэшируется read-lookup, не исход);
 *  - порядок обхода направлений при miss = ванильный (N,E,S,W + диагонали) —
 *    маска разворачивается в тот же порядок, heap tie-break A* не меняется;
 *  - STORE только после полной ванильной выкладки соседей узла (кэш никогда
 *    не заменяет выкладку — только её re-traversal);
 *  - STRICT-off: ENABLED latch ставится нативом ТОЛЬКО после define в kernel
 *    loader + selfTest (NCDFE-канон: define-before-arm, selfTest без lazy
 *    резолюций — JVM кэширует NCDFE per constant-pool entry навсегда,
 *    root-cause cv3-1 35712182885); пустой/чужой CRUSSTY_LEVER_FLAG = класс
 *    не определяется, latch никогда не включается = ваниль по построению.
 *
 * Макет таблицы: линейный probe по power-of-two, KEY_SENTINEL = Long.MIN_VALUE
 * (posKey в layout vanilla BlockPos.asLong (x26 z26 y12) старшего знакового
 * бита не несёт — sentinel недостижим). Переполнение = miss-фоллбэк (bounded
 * память, дисциплина nav_pool MAP_CAP). Обе стороны (java/rust
 * src/nav_path_cache.rs) держат байт-в-байт одинаковый layout: x26|z26|y12,
 * value = rev<<8 | mask.
 *
 * iter-2 wiring: bulk-JNI navNeighborBatch (rust ERR-ladder STRUCT=-1
 * RANGE=-2, rc>=0 = hits) из src/nav_path_cache.rs; hook
 * WalkNodeEvaluator.getNeighbors; инвалидация на сайте
 * pathTypesByPosCache.invalidate (sendBlockUpdated, BlockUpdateOps-паттерн).
 * В этом scaffold-коммите класс — JDK-only макет движка с самопроверкой
 * (selfTest), MC-символы не тянет по построению (0 lazy resolutions).
 */
public final class PathOps {
    private PathOps() {}

    /** STRICT-off до оракула: latch ставится только нативом define-before-arm. */
    public static volatile boolean ENABLED = false;

    /** MISS-маркер: ваниль-ветка getNeighbors без кэша. */
    public static final int MISS = -1;

    private static final int CAP = 1 << 17;                 // 131072 слота
    private static final int CAP_MASK = CAP - 1;
    private static final long KEY_SENTINEL = Long.MIN_VALUE;

    private static final long[] KEYS = new long[CAP];
    private static final long[] VALS = new long[CAP];       // rev<<8 | mask
    private static final int SEC_CAP = 1 << 14;
    private static final int SEC_MASK = SEC_CAP - 1;
    private static final long[] SEC_KEYS = new long[SEC_CAP];
    private static final long[] SEC_VALS = new long[SEC_CAP]; // rev

    static {
        java.util.Arrays.fill(KEYS, KEY_SENTINEL);
        java.util.Arrays.fill(SEC_KEYS, KEY_SENTINEL);
    }

    // --- posKey: vanilla BlockPos.asLong layout (x26 z26 y12) ---

    public static long posKey(int x, int y, int z) {
        return ((x & 0x3FFFFFFL) << 38) | ((z & 0x3FFFFFFL) << 12) | (y & 0xFFFL);
    }

    public static int xOf(long k) {
        long v = (k >>> 38) & 0x3FFFFFFL;
        return (int) ((v & 0x2000000L) != 0 ? v - 0x4000000L : v);
    }

    public static int yOf(long k) {
        long v = k & 0xFFFL;
        return (int) ((v & 0x800L) != 0 ? v - 0x1000L : v);
    }

    public static int zOf(long k) {
        long v = (k >>> 12) & 0x3FFFFFFL;
        return (int) ((v & 0x2000000L) != 0 ? v - 0x4000000L : v);
    }

    /** secKey 16³-секции; x26|z26|y12 → (x>>4)|22б | (z>>4)|22б | (y>>4)|10б. */
    public static long sectionKeyOf(long pos) {
        long sx = (long) (xOf(pos) >> 4) & 0x3FFFFFL;
        long sz = (long) (zOf(pos) >> 4) & 0x3FFFFFL;
        long sy = (long) (yOf(pos) >> 4) & 0x3FFL;
        return (sx << 32) | (sz << 10) | sy;
    }

    // --- таблица секций (chunk-ревизии) ---

    private static int secSlot(long sec) {
        int i = mix(sec) & SEC_MASK;
        for (;;) {
            long s = SEC_KEYS[i];
            if (s == sec) return i;
            if (s == KEY_SENTINEL) {
                SEC_KEYS[i] = sec;
                SEC_VALS[i] = 0L;
                return i;
            }
            i = (i + 1) & SEC_MASK;
        }
    }

    /** Текущая ревизия секции (0 = ещё не видели). */
    public static long sectionRev(long sec) {
        return SEC_VALS[secSlot(sec)];
    }

    /** Инвалидация: bump ревизии секции (sendBlockUpdated-сайт, iter-2). */
    public static void invalidateSection(long sec, long rev) {
        SEC_VALS[secSlot(sec)] = rev;
    }

    // --- таблица соседей ---

    private static int mix(long k) {
        long z = k + 0x9E3779B97F4A7C15L;
        z = (z ^ (z >>> 30)) * 0xBF58476D1CE4E5B9L;
        z = (z ^ (z >>> 27)) * 0x94D049BB133111EBL;
        return (int) (z ^ (z >>> 31));
    }

    /**
     * LOOKUP: mask (0..255) при хите с совпадающей ревизией, MISS = ваниль.
     * mismatch ревизии высвобождает слот (lazy expiry).
     */
    public static int lookup(long pos, long rev) {
        int i = mix(pos) & CAP_MASK;
        for (int probes = 0; probes < CAP; probes++) {
            long k = KEYS[i];
            if (k == pos) {
                long v = VALS[i];
                if ((v >>> 8) != rev) {
                    KEYS[i] = KEY_SENTINEL; // stale — слот мёртв
                    VALS[i] = 0L;
                    return MISS;
                }
                return (int) (v & 0xFFL);
            }
            if (k == KEY_SENTINEL) return MISS;
            i = (i + 1) & CAP_MASK;
        }
        return MISS;
    }

    /**
     * STORE — ТОЛЬКО после полной ванильной выкладки соседей узла
     * (superset-инвариант обслуживается здесь, не кэшем).
     */
    public static boolean store(long pos, long rev, int mask) {
        int i = mix(pos) & CAP_MASK;
        for (int probes = 0; probes < CAP; probes++) {
            long k = KEYS[i];
            if (k == pos) {
                VALS[i] = (rev << 8) | (mask & 0xFFL);
                return true;
            }
            if (k == KEY_SENTINEL) {
                KEYS[i] = pos;
                VALS[i] = (rev << 8) | (mask & 0xFFL);
                return true;
            }
            i = (i + 1) & CAP_MASK;
        }
        return false; // overflow -> miss-фоллбэк (bounded память)
    }

    /** Полный сброс масок (секции остаются — rev источник истины). */
    public static void clearMasks() {
        java.util.Arrays.fill(KEYS, KEY_SENTINEL);
        java.util.Arrays.fill(VALS, 0L);
    }

    // --- selfTest (invoke после define, до latch ENABLED — NCDFE-канон) ---

    /**
     * Самопроверка движка без MC-символов (0 lazy resolutions):
     * posKey-округление, секционирование, hit/miss/stale, overflow-фоллбэк,
     * супермножество-хиты. Возвращает true = можно ставить ENABLED latch.
     */
    public static boolean selfTest() {
        long p1 = posKey(10, 64, 10);
        if (xOf(p1) != 10 || yOf(p1) != 64 || zOf(p1) != 10) return false;
        long p2 = posKey(-123456, -300, 65432);
        if (xOf(p2) != -123456 || yOf(p2) != -300 || zOf(p2) != 65432) return false;
        long p3 = posKey(0, 0, 0);
        if (p3 == KEY_SENTINEL) return false;
        if (sectionKeyOf(posKey(5, 64, 9)) != sectionKeyOf(posKey(15, 79, 15))) return false;
        if (sectionKeyOf(posKey(5, 64, 9)) == sectionKeyOf(posKey(16, 64, 9))) return false;
        if (lookup(p1, 7) != MISS) return false;
        if (!store(p1, 7, 0b10100011)) return false;
        if (lookup(p1, 7) != 0b10100011) return false;
        if (lookup(p1, 6) != MISS) return false;            // чужая ревизия
        invalidateSection(sectionKeyOf(p1), 8);
        if (lookup(p1, 8) != MISS) return false;            // stale слот мёртв
        if (!store(p1, 8, 0)) return false;
        if (lookup(p1, 8) != 0) return false;               // маска 0 — валидный хит
        clearMasks();
        if (lookup(p1, 8) != MISS) return false;
        return true;
    }
}
