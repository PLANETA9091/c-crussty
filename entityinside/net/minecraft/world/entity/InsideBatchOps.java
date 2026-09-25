package net.minecraft.world.entity;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TASK-459-56 (ID-P31, закон 11 WILD) — INSIDE-BATCH: discovery тика
 * {@code Entity.checkInsideBlocks} одним bulk-JNI (scaffold).
 *
 * Гейт-паттерн — сиблинг InsideBlockOps.gate (S7-135): retarget
 * {@code Entity.isAffectedByBlocks} @ offset 1 внутри
 * {@code checkInsideBlocks(List, StepBasedCollector)} на
 * {@code InsideBatchOps.batchGate(Entity)Z} (receiver-first, 3B→3B,
 * длина сохранена). Владелец сайта ОДИН (S7-162 supersede-дисциплина
 * entity_compose): armed inside_batch вытесняет inside_cache на сайте.
 *
 * ФОРМА ТИКА (закон 6 — подсистема целиком: gate+discovery+tail):
 *  1) java собирает батч ВСЕХ checkInsideBlocks-кандидатов тика
 *     (entity id, x/y/z, bb-флет 6×double, section-ключи 16³-секций,
 *     пересекаемых swept-боксом; фиксированная ширина MAXSEC=8);
 *  2) ОДИН нативный вызов insideBatchMask (RegisterNatives, arm-order
 *     контракт: класс определён + нативы зарегистрированы + selfTest ДО
 *     BATCH_ARMED=true) → Rust возвращает битмаску секций-кандидатов
 *     per-entity (superset);
 *  3) java строгий ВАНИЛЬНЫЙ хвост visit-обхода по секциям-кандидатам:
 *     порядок/шаги/флаги 1:1 с ванилью (advanceStep + entityInside +
 *     onInsideBlock + FluidState.entityInside), applyAndClear остаётся
 *     ванильным вызывателем.
 *
 * PARITY: маска = superset. False-positive разрешён (лишний кандидат
 * обслуживает ванильный хвост), false-negative ЗАПРЕЩЁН: любая ошибка
 * структуры/переполнение/чужой collector ⇒ all-ones ⇒ чистая ваниль.
 * Dirty-list секций-мутантов (secWrite-бампы, паттерн entity_index.rs)
 * OR-ится в маску между сборкой батча и хвостом — lost-effect невозможен.
 *
 * NCDFE-канон (паттерн d73758a3/5ecd841a, fa9054d9 ARM-AFTER-DEFINE):
 * класс определяется в раннем arm-хуке ДО первого retarget-вызова; констант-
 * пул ref insideBatchMask резолвится только внутри armed-ветки ⇒ при
 * BATCH_ARMED=false NCDFE недостижим (урок leg #2'' 35318755582).
 *
 * Lever dormant: CRUSSTY_INSIDE_BATCH не установлен ⇒ класс не определяется,
 * сайт не компонуется, хук не регистрируется — ваниль бит-в-байт.
 */
public final class InsideBatchOps {

    private InsideBatchOps() {}

    /** Arm-order контракт: true ТОЛЬКО после define+RegisterNatives+selfTest (rust arm-хук). */
    static volatile boolean BATCH_ARMED = false;

    /** Максимальное число секций-кандидатов на сущность (2×2×2 при дефляции traversal). */
    static final int MAXSEC = 8;

    /** Джавап-дефляция ванильного traversal-бокса (зеркало InsideBlockOps.DEFLATE). */
    static final double DEFLATE = 9.999999747378752E-6d;

    /** Бюджет батча на тик (переполнение ⇒ all-ones для хвоста — ваниль). */
    static final int MAXBATCH = 4096;

    // ThreadLocal плоские буферы батча (ноль аллокаций на тик после прогрева;
    // никакой ооп-массив в hot-пути — урок §153/§155).
    static final ThreadLocal<long[]> TL_EIDS = ThreadLocal.withInitial(() -> new long[MAXBATCH]);
    static final ThreadLocal<double[]> TL_XYZ = ThreadLocal.withInitial(() -> new double[MAXBATCH * 3]);
    static final ThreadLocal<double[]> TL_BB = ThreadLocal.withInitial(() -> new double[MAXBATCH * 6]);
    static final ThreadLocal<int[]> TL_KEYS = ThreadLocal.withInitial(() -> new int[MAXBATCH * MAXSEC]);
    static final ThreadLocal<int[]> TL_NSEC = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_DIRTY = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_OUT = ThreadLocal.withInitial(() -> new int[MAXBATCH]);

    /**
     * Ретаргет-точка метода-входа (TASK-460-03 climb wiring, S7-162 site
     * owner = InsideBatchOps, supersede inside_cache на сайте
     * isAffectedByBlocks@offset-1). BATCH_ARMED=false ⇒
     * e.isAffectedByBlocks() — ванильное тело как обычно.
     *
     * Armed-ветка v1 (климб-дисциплина закон 16 — STRICT pruned tail
     * oracle-gated, RESEARCH-459-P31 §3: 10k+ lockstep-сцен до STRICT-on):
     *  - СТАТИЧНАЯ сущность ⇒ сервис через живой inside_cache мемо-гейт
     *    InsideBlockOps.gate(e) — бит-в-бит то же поведение, что при
     *    владении сайтом inside_cache (носитель cmp457_paldelta): HIT —
     *    зеркало+replay, miss/динамика — ванильный гвард изнутри gate.
     *  - ИНАЧЕ ⇒ ванильный гвард (полный vanilla visit-хвост как обычно).
     *
     * Композишн-контракт (rust entity_compose STAGE 1): сцена
     * компонуется ТОЛЬКО при готовых ОБОИХ мостах (InsideBatchOps определён
     * + selfTest=1 + натив зарегистрирован, И InsideBlockOps определён) —
     * ссылка InsideBlockOps.gate из armed-ветки NCDFE-недостижима
     * (урок leg #2'' 35318755582, canon d73758a3/5ecd841a).
     */
    public static boolean batchGate(Entity e) {
        if (!BATCH_ARMED) {
            return e.isAffectedByBlocks();
        }
        return InsideBlockOps.gate(e);
    }

    /**
     * Самтест моста (вызывается rust-стороной ПОСЛЕ define+RegisterNatives,
     * ДО noteBatchArmed — arm-order контракт). Возвращает 1 = PASS / 0 = FAIL.
     * Чистая java-арифметика + ОДИН нативный round-trip insideBatchMask на
     * синтетическом батче (бит-инвариант §3: пересекающая секция получает
     * бит, далёкая — нет; ERR-код ⇒ 0). Никаких ссылок на kernel-классы
     * сверх package-сиблингов — define/selfTest не могут NCDFE.
     */
    static int selfTest() {
        try {
            // 1) упаковка/распаковка ключа — roundtrip (x10|z10|y12).
            int k = key(5, 4, 9);
            if (ux(k) != 5 || uy(k) != 4 || uz(k) != 9) return 0;
            // 2) геометрия sectionKeys: бокс вокруг (8,70,8) ловит секцию (0,4,0).
            int[] keys = new int[MAXSEC];
            AABB near = new AABB(7, 69, 7, 9, 71, 9).deflate(-DEFLATE);
            int n1 = sectionKeys(near, keys, 0);
            boolean hitNear = false;
            for (int i = 0; i < n1; i++) if (keys[i] == key(0, 4, 0)) hitNear = true;
            if (n1 <= 0 || !hitNear) return 0;
            // 3) native round-trip (n=3): e1 — ключ СВОЕЙ секции (0,4,0) ⇒ бит;
            //    e2 — ЧУЖОЙ далёкий ключ (секция (0,4,0) против бокса у 488) ⇒ бит НЕ
            //    ставится (superset точечный); e3 — далёкий ключ + dirty=1 ⇒ бит
            //    форсится OR-ом dirty-листа.
            int n = 3;
            long[] eids = {41L, 42L, 43L};
            double[] xyz = {8, 70, 8, 488, 70, 488, 488, 70, 488};
            double[] bb = {7, 69, 7, 9, 71, 9, 487, 69, 487, 489, 71, 489,
                           487, 69, 487, 489, 71, 489};
            int[] sec = {key(0, 4, 0), key(0, 4, 0), key(30, 4, 30)};
            int[] nsec = {1, 1, 1};
            int[] dirty = {0, 0, 1};
            int[] out = {-1, -1, -1};
            int rc = insideBatchMask(n, MAXSEC, eids, xyz, bb, sec, nsec, dirty, out);
            if (rc != 0 || out[0] < 0 || out[1] < 0 || out[2] < 0) return 0;
            if ((out[0] & 1) == 0) return 0; // near-section bit must be set
            if ((out[1] & 1) != 0) return 0; // far-section bit must NOT be set
            if ((out[2] & 1) == 0) return 0; // dirty forces the bit (OR superset)
            return 1;
        } catch (Throwable t) {
            return 0;
        }
    }

    /** Ключ секции (x10|z10|y12) — java-зеркало pack_section (scaffold). */
    private static int key(int sx, int sy, int sz) {
        return ((sx & 0x3FF) << 20) | ((sz & 0x3FF) << 10) | (sy & 0x3FF);
    }
    private static int ux(int k) { return (k >> 20) & 0x3FF; }
    private static int uy(int k) { return k & 0x3FF; }
    private static int uz(int k) { return (k >> 10) & 0x3FF; }

    /**
     * Scaff-сборка батча одного тика (контракт вызовов для полного v1).
     * Возвращает число сущностей в батче; ошибки структуры ⇒ 0 (ваниль).
     */
    static int collectBatch(java.util.List<Entity> candidates, Level level) {
        int n = Math.min(candidates.size(), MAXBATCH);
        long[] eids = TL_EIDS.get();
        double[] xyz = TL_XYZ.get();
        double[] bb = TL_BB.get();
        int[] keys = TL_KEYS.get();
        int[] nsec = TL_NSEC.get();
        int[] dirty = TL_DIRTY.get();
        for (int i = 0; i < n; i++) {
            Entity e = candidates.get(i);
            eids[i] = e.getId();
            xyz[i * 3] = e.getX();
            xyz[i * 3 + 1] = e.getY();
            xyz[i * 3 + 2] = e.getZ();
            AABB box = e.makeBoundingBox(new Vec3(e.getX(), e.getY(), e.getZ())).deflate(DEFLATE);
            bb[i * 6] = box.minX;
            bb[i * 6 + 1] = box.minY;
            bb[i * 6 + 2] = box.minZ;
            bb[i * 6 + 3] = box.maxX;
            bb[i * 6 + 4] = box.maxY;
            bb[i * 6 + 5] = box.maxZ;
            nsec[i] = sectionKeys(box, keys, i * MAXSEC);
            dirty[i] = 0; // dirty-list секций-мутантов OR-ится при сборке батча (v1)
        }
        return n;
    }

    /** section-ключи 16³-секций, пересекаемых боксом (упаковка x10|z10|y12, scaffold). */
    private static int sectionKeys(AABB box, int[] keys, int base) {
        int n = 0;
        int x0 = (int) Math.floor(box.minX / 16.0), x1 = (int) Math.floor(box.maxX / 16.0);
        int y0 = (int) Math.floor(box.minY / 16.0), y1 = (int) Math.floor(box.maxY / 16.0);
        int z0 = (int) Math.floor(box.minZ / 16.0), z1 = (int) Math.floor(box.maxZ / 16.0);
        for (int sx = x0; sx <= x1; sx++) {
            for (int sy = y0; sy <= y1; sy++) {
                for (int sz = z0; sz <= z1; sz++) {
                    if (n >= MAXSEC || sy < 0 || sy > 0x3FF) {
                        return Math.min(n, MAXSEC); // переполнение ⇒ caller даёт all-ones
                    }
                    keys[base + n++] = ((sx & 0x3FF) << 20) | ((sz & 0x3FF) << 10) | (sy & 0x3FF);
                }
            }
        }
        return n;
    }

    /**
     * Натив (RegisterNatives rust-стороной, src/inside_batch.rs): ОДИН переход
     * на батч. n сущностей; out[i] = битмаска секций-кандидатов (superset|dirty).
     * rc &lt; 0 ⇒ ERR ⇒ all-ones хвост (ваниль). Объявлен здесь, резолвится
     * только в armed-ветке полного v1 (NCDFE-канон).
     */
    private static native int insideBatchMask(
            int n, int maxsec,
            long[] eids, double[] xyz, double[] bb,
            int[] secKeys, int[] nsec, int[] dirty, int[] out);

    /** Arm-хук моста (вызывает rust после define+RegisterNatives+selfTest). */
    public static void noteBatchArmed() {
        BATCH_ARMED = true;
    }
}
