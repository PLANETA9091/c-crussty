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
     * Ретаргет-точка метода-входа (scaffold). BATCH_ARMED=false ⇒
     * e.isAffectedByBlocks() — ванильное тело как обычно. Armed-ветка v1
     * (полная реализация после оракула entityinside/harness): сборка батча
     * + insideBatchMask + strict-хвост по кандидатам.
     */
    public static boolean batchGate(Entity e) {
        if (!BATCH_ARMED) {
            return e.isAffectedByBlocks();
        }
        // scaffold: strict-хвост ещё не подключён — чистая ваниль (fail-closed).
        return e.isAffectedByBlocks();
    }

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
