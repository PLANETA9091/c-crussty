package net.minecraft.world.entity;

/**
 * HILBERT STORAGE ORDER — java-зеркало (STUB) плана пре-сортировки слотов
 * внутри 16³-секции (ID-H07, TASK-459-77; Strictly dormant scaffold).
 *
 * Контракт паритета (карточка ID-H07): порядок ВЫДАЧИ остаётся ванильным —
 * реордерится только физический порядок ХРАНЕНИЯ (reorder storage, не
 * выдачу); выдача per-call = то же мульти-множество, те же счётчики.
 *
 * Зеркало повторяет src/hilbert_storage_order.rs бит-в-байт:
 *   key(x,y,z) = hilbert2D(x,z, bits=4) << 4 | (y & 0xF);
 *   план = СТАБИЛЬНАЯ сортировка слот-индексов по ключу;
 *   гарды = биекция + сохранение мульти-множества ключей.
 * Будущая lockstep-цеза (закон 16): план rust vs план java обязан совпасть
 * индекс-в-индекс на детерминированных сэмплах ДО любого define-шага
 * (NCDFE-канон; в <clinit>/статиках ниже нет indy/indy-method-ref —
 * plain-паттерн InsideSnapOps.java:241-254).
 *
 * STRICT DORMANT: класс НИГДЕ не ссылается на кернел-типы кроме пакета,
 * не содержит side-effects, не определяется в kernel loader (define-шаг —
 * wiring-фаза, отдельный коммит). Vanilla-free: голый javac --release 21.
 */
public final class HilbertStorageOps {

    /** Lever-гейт подсистемы (зеркало CRUSSTY_HILBERT_STORAGE). */
    public static final String LEVER_ENV = "CRUSSTY_HILBERT_STORAGE";

    /** Секция 16³: битов на ось. */
    public static final int SECTION_BITS = 4;

    private HilbertStorageOps() {
    }

    /** Lever (зеркало enabled(); 1/true/on/yes). */
    public static boolean enabled() {
        String v = System.getenv(LEVER_ENV);
        if (v == null) {
            return false;
        }
        v = v.trim().toLowerCase(java.util.Locale.ROOT);
        return v.equals("1") || v.equals("true") || v.equals("on") || v.equals("yes");
    }

    /** Поворот/отражение квадранта (канон rot() из d2xy-реализации). */
    private static void rot(final int n, int[] xz, final int rx, final int rz) {
        if (rz == 0) {
            if (rx == 1) {
                xz[0] = n - 1 - xz[0];
                xz[1] = n - 1 - xz[1];
            }
            final int t = xz[0];
            xz[0] = xz[1];
            xz[1] = t;
        }
    }

    /** Координаты сетки {@code 1<<bits} → Hilbert-индекс d (биекция). */
    public static long hilbertXZ2D(int x, int z, final int bits) {
        if (bits < 1 || bits > 15) {
            throw new IllegalArgumentException("bits out of scaffold range: " + bits);
        }
        final int n = 1 << bits;
        final int[] xz = new int[] { x, z };
        long d = 0L;
        int s = n >> 1;
        while (s > 0) {
            final int rx = (xz[0] & s) != 0 ? 1 : 0;
            final int rz = (xz[1] & s) != 0 ? 1 : 0;
            d += (long) s * (long) s * (long) ((3 * rx) ^ rz);
            rot(n, xz, rx, rz);
            s >>= 1;
        }
        return d;
    }

    /** Полный ключ слота секции: Hilbert(x,z) мажор, y минор. */
    public static long sectionSlotKey(final int x, final int y, final int z) {
        return (hilbertXZ2D(x, z, SECTION_BITS) << 4) | (long) (y & 0xF);
    }

    /**
     * План секции: {@code order[r] = слот-индекс} на ранге r скана.
     * СТАБИЛЬНАЯ сортировка (двойники-координаты сохраняют порядок —
     * чистый реордер хранения). Возвращает null при отказе гарда
     * (fail-closed: без плана = ваниль).
     */
    public static int[] buildSectionPlan(final long[] keys) {
        final int n = keys.length;
        final int[] order = new int[n];
        for (int i = 0; i < n; i++) {
            order[i] = i;
        }
        // Стабильная сортировка вставками по long-ключу (n ≤ пары сотен
        // слотов секции; без боксинга и без лямбд — plain-паттерн).
        for (int i = 1; i < n; i++) {
            final int cur = order[i];
            final long k = keys[cur];
            int j = i - 1;
            while (j >= 0 && keys[order[j]] > k) {
                order[j + 1] = order[j];
                j--;
            }
            order[j + 1] = cur;
        }
        return isBijection(order, n) ? order : null;
    }

    /** Биекция-страж: границы + без дублей. */
    public static boolean isBijection(final int[] order, final int len) {
        if (order == null || order.length != len) {
            return false;
        }
        final boolean[] seen = new boolean[len];
        for (final int i : order) {
            if (i < 0 || i >= len || seen[i]) {
                return false;
            }
            seen[i] = true;
        }
        return true;
    }

    /**
     * Мульти-множество ключей переукладки == исходное (паритет per-call
     * выдачи: same entities, same counts). Сортированные копии равны.
     */
    public static boolean multisetPreserved(final long[] keys, final int[] order) {
        if (!isBijection(order, keys.length)) {
            return false;
        }
        final long[] a = keys.clone();
        final long[] b = new long[keys.length];
        for (int r = 0; r < order.length; r++) {
            b[r] = keys[order[r]];
        }
        java.util.Arrays.sort(a);
        java.util.Arrays.sort(b);
        return java.util.Arrays.equals(a, b);
    }

    /**
     * SelfTest: детерминированный LCG-сэмпл секции (тот же seed, что в
     * rust-тесте 0x455F48307713_5EED) → план обязан быть биекцией,
     * сохранять мульти-множество и быть отсортированным по ключу.
     * Порядок порядка == порядок rust-плана (будущая бит-в-байт цеза).
     */
    public static boolean selfTest() {
        long st = 0x455F48307713_5EEDL;
        final int n = 120;
        final long[] keys = new long[n];
        for (int i = 0; i < n; i++) {
            st = st * 6364136223846793005L + 1442695040888963407L;
            final int x = (int) ((st >>> 33) & 0xF);
            final int y = (int) ((st >>> 37) & 0xF);
            final int z = (int) ((st >>> 41) & 0xF);
            keys[i] = sectionSlotKey(x, y, z);
        }
        final int[] plan = buildSectionPlan(keys);
        if (plan == null || !isBijection(plan, n) || !multisetPreserved(keys, plan)) {
            return false;
        }
        for (int r = 1; r < n; r++) {
            if (keys[plan[r - 1]] > keys[plan[r]]) {
                return false;
            }
        }
        // y-минор: одинаковые (x,z) → строго возрастающий ряд ключей по y.
        for (int x = 0; x < 16; x++) {
            for (int z = 0; z < 16; z++) {
                if (sectionSlotKey(x, 0, z) >= sectionSlotKey(x, 1, z)) {
                    return false;
                }
            }
        }
        return true;
    }
}
