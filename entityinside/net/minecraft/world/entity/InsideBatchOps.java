package net.minecraft.world.entity;

import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TASK-459-56 (ID-P31, закон 11 WILD) — INSIDE-BATCH: discovery тика
 * {@code Entity.checkInsideBlocks} одним bulk-JNI. TASK-463-65a
 * (chkclimb-13, STRICT-TAIL v1 — LEDGER-25 чеклист C1-C8): armed-ветка
 * living — SoA-сборка + T=512 бакеты + ОДИН bulk-JNI на бакет
 * (296 переходов/тик при pop 150k vs 150,000 per-entity и vs 375,000
 * strict-superset-натаива, ×506.8/×1266.9 экономика §3 LEDGER-25).
 *
 * Гейт-паттерн — сиблинг InsideBlockOps.gate (S7-135): retarget
 * {@code Entity.isAffectedByBlocks} @ offset 1 внутри
 * {@code checkInsideBlocks(List, StepBasedCollector)} на
 * {@code InsideBatchOps.batchGate(Entity)Z} (receiver-first, 3B→3B,
 * длина сохранена). Владелец сайта ОДИН (S7-162 supersede-дисциплина
 * entity_compose): armed inside_batch вытесняет inside_cache на сайте.
 *
 * STRICT-TAIL ДИСЦИПЛИНА (LEDGER-25 §2.4): «строгость» живёт в JAVA-хвосте,
 * НЕ в Rust-маске. Rust-маска остаётся дешёвой геометрией (superset,
 * 5.42 ns/entity, mean 2.5 секций); strict-superset В НАТИВЕ отвергнут
 * (×506.8 JNI-налога = смерть ноги). Форма тика:
 *  1) java собирает батч кандидатов тика в TL-буферы (SoA-флет: eid, xyz,
 *     bb-флет 6×f64 через makeBoundingBox().deflate(DEFLATE), section-ключи
 *     16³-секций пересекаемых swept-боксом, ширина MAXSEC=8, nsec, dirty);
 *  2) каждый T=512-бакет сбрасывается ОДНИМ нативным вызовом
 *     insideBatchMask(n, maxsec, base, ...) → per-entity битмаска
 *     секций-кандидатов (superset | dirty; overflow/nsec==0 ⇒ all-ones);
 *  3) strict java-хвост visit-обхода по секциям-кандидатам
 *     (getBlockState→isAir→getEntityInsideCollisionShape→…→advanceStep,
 *     applyAndClear ванильного вызывателя) — ORACLE-GATED: v1 маски = телеметрия
 *     TL_OUT (ноль потребителей в live-пути ⇒ парити = ваниль by construction),
 *     live-подключение хвоста = v2 после lockstep-оракула канона
 *     InsideBitmaskLockstepHarness (10⁶×3 бит-в-байт). Открытие ×463:
 *     receiver-only сайт batchGate физически не достигает List&lt;Movement&gt;/
 *     StepBasedCollector кадра checkInsideBlocks — live-хвост требует второго
 *     сайта ретаргета (внутри checkInsideBlocks(Vec3,Vec3,...)) или
 *     реконструкции Movement из xo/yo/zo; обе ветки — v2.
 *
 * PARITY: маска = superset. False-positive разрешён (лишний кандидат
 * обслуживает ванильный хвост), false-negative ЗАПРЕЩЁН: неизвестная
 * структура/переполнение/nsec==0 ⇒ all-ones ⇒ чистая ваниль. ×463-фикс FN:
 * sectionKeys при переполнении (&gt;MAXSEC секций, напр. entity bb шире 2×16³)
 * теперь сигнализирует -1 ⇒ nsec=0 ⇒ all-ones (раньше было МОЛЧАЛИВОЕ
 * усечение до 8 ключей — FN-дыра против «FN=0 структурно» C6); y-секции вне
 * [0,0x3FF] дропаются безопасно (Level.getBlockState вне высоты ⇒ void_air
 * ⇒ isAir ⇒ ноль визитов). i64-CSR (SectionPos.asLong) закрывает x/z-wrap
 * за ±8192 блоков — v2 (контракт «ядро маски не меняется»).
 *
 * SELFTEST (C5b, ×463-дыра закрыта): arm-order контракт
 * «define→RegisterNatives→selfTest→BATCH_ARMED» — rust вызывает
 * {@link #selfTest()} ДО {@link #noteBatchArmed()}; selfTest!=1 ⇒ BATCH_ARMED
 * не публикуется (fail-closed, громкий лог). selfTest = (а) AIOOBE-проба
 * границы i=MAXBATCH−1 на всех 8 TL-буферах (факт ×462-64), (б) count-инвариант
 * superset на оракул-сценах (RESEARCH-459-P31 §3): маска ⊇ геометрически
 * ожидаемых секций-кандидатов на фиксированном сиде, (в) overflow-сцена ⇒
 * all-ones, (г) nsec==0 ⇒ all-ones.
 *
 * NCDFE-канон (паттерн d73758a3/5ecd841a, fa9054d9 ARM-AFTER-DEFINE):
 * класс определяется в раннем arm-хуке ДО первого retarget-вызова; констант-
 * пул ref insideBatchMask резолвится только внутри armed-ветки/selfTest ⇒
 * при BATCH_ARMED=false NCDFE недостижим (урок leg #2'' 35318755582).
 * 8-й ThreadLocal (TL_CUR) добавляет 8-ю lambda в &lt;clinit&gt; — та же
 * JDK-бутстрапленная семья ThreadLocal.withInitial, что и канонные 7.
 *
 * G6 one-shot disarm: ЛЮБОЙ Throwable из armed-ветки ⇒ BATCH_ARMED=false
 * навсегда (ваниль до рестарта) + THREW-телеметрия (гейт G6: threw=0).
 *
 * Lever dormant: CRUSSTY_INSIDE_BATCH не установлен И lever_flag не
 * cmp456_chunkmono_p31snap ⇒ класс не определяется, сайт не компонуется,
 * хук не регистрируется — ваниль бит-в-байт.
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

    /** Бакет bulk-JNI (C3, LEDGER-25): T=512 праймери, 74 вызова/воркер/тик при
     * pop 150k, leftover 124, batched 99.67%, волна 4096=8×512; T обязан делить 4096. */
    static final int T = 512;

    /** Телеметрия CI-ноги (javap-видимые volatile-счётчики; не участвуют в парити). */
    static volatile long FLUSHED = 0L;
    static volatile long MASKED = 0L;
    static volatile long ERRS = 0L;
    static volatile long THREW = 0L;

    // ThreadLocal плоские буферы батча (ноль аллокаций на тик после прогрева;
    // никакой ооп-массив в hot-пути — урок §153/§155).
    // C1-контракт ×462-поправка: 7 буферов = 496 KB/поток (ровно 4096×124 B),
    // 8-й TL_CUR = int[1] (роллинг-индекс записи, +4 B).
    static final ThreadLocal<long[]> TL_EIDS = ThreadLocal.withInitial(() -> new long[MAXBATCH]);
    static final ThreadLocal<double[]> TL_XYZ = ThreadLocal.withInitial(() -> new double[MAXBATCH * 3]);
    static final ThreadLocal<double[]> TL_BB = ThreadLocal.withInitial(() -> new double[MAXBATCH * 6]);
    static final ThreadLocal<int[]> TL_KEYS = ThreadLocal.withInitial(() -> new int[MAXBATCH * MAXSEC]);
    static final ThreadLocal<int[]> TL_NSEC = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_DIRTY = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_OUT = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_CUR = ThreadLocal.withInitial(() -> new int[1]);

    /**
     * Ретаргет-точка метода-входа. BATCH_ARMED=false ⇒ e.isAffectedByBlocks()
     * — ванильное тело как обычно. Armed-ветка v1 (STRICT-TAIL collection
     * plane): SoA-append + T=512 бакет-флаш (ОДИН bulk-JNI на бакет) +
     * one-shot disarm на Throwable; возврат ВСЕГДА ванильный
     * e.isAffectedByBlocks() — strict-хвост визитов = v2 (oracle-gated,
     * маски TL_OUT в v1 не потребляются live-путём ⇒ парити = ваниль).
     */
    public static boolean batchGate(Entity e) {
        if (!BATCH_ARMED) {
            return e.isAffectedByBlocks();
        }
        try {
            collectIntoBatch(e);
        } catch (Throwable t) {
            BATCH_ARMED = false; // G6 one-shot disarm
            THREW++;
            return e.isAffectedByBlocks();
        }
        return e.isAffectedByBlocks();
    }

    /**
     * SoA-append одной сущности в TL-буферы (контракт STRICT-TAIL v1).
     * Каждый T=512-й элемент сбрасывает бакет ОДНИМ bulk-JNI; MAXBATCH —
     * роллинг-окно (без сброса семантики: superset per-entity независим).
     */
    static void collectIntoBatch(Entity e) {
        int[] cur = TL_CUR.get();
        int i = cur[0];
        long[] eids = TL_EIDS.get();
        double[] xyz = TL_XYZ.get();
        double[] bb = TL_BB.get();
        int[] keys = TL_KEYS.get();
        int[] nsec = TL_NSEC.get();
        int[] dirty = TL_DIRTY.get();
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
        int nk = sectionKeys(box, keys, i * MAXSEC);
        if (nk < 0) {
            nsec[i] = 0; // overflow (>MAXSEC секций) ⇒ fail-open all-ones (×463 FN-фикс)
        } else {
            nsec[i] = nk;
        }
        dirty[i] = 0; // dirty-list секций-мутантов OR-ится до хвоста (v2; в v1 хвоста нет ⇒ потерь нет)
        i++;
        if (i % T == 0) {
            flushBucket(i - T);
        }
        cur[0] = (i >= MAXBATCH) ? 0 : i;
    }

    /**
     * ОДИН bulk-JNI на T=512-бакет (C3/C4: 296 переходов/тик при pop 150k,
     * critical-only на rust-стороне — ноль Region-copy 64KB×74).
     * rc&lt;0 ⇒ ERR-телеметрия (маски бакета валидны только при rc==0;
     * в v1 nothing consumes them — strict-хвост v2). Частичный бакет живёт до
     * заполнения (телеметрия v1 миксует тики — парити не касается); v2
     * обязана сбрасывать хвостовой бакет на границе тика (tick-boundary hook).
     */
    static void flushBucket(int base) {
        int rc = insideBatchMask(T, MAXSEC, base, TL_EIDS.get(), TL_XYZ.get(), TL_BB.get(),
                TL_KEYS.get(), TL_NSEC.get(), TL_DIRTY.get(), TL_OUT.get());
        FLUSHED++;
        if (rc == 0) {
            MASKED += T;
        } else {
            ERRS++;
        }
    }

    /**
     * section-ключи 16³-секций, пересекаемых боксом (упаковка x10|z10|y12).
     * ×463: переполнение (&gt;MAXSEC секций) и пустой/вырожденный бокс
     * сигнализируются -1 (caller даёт nsec=0 ⇒ all-ones) вместо молчаливого
     * усечения — FN=0 структурно (C6). y вне [0,0x3FF] дроп безопасен:
     * вне высоты Level.getBlockState ⇒ void_air ⇒ isAir ⇒ ноль визитов.
     */
    private static int sectionKeys(AABB box, int[] keys, int base) {
        int x0 = (int) Math.floor(box.minX / 16.0), x1 = (int) Math.floor(box.maxX / 16.0);
        int y0 = (int) Math.floor(box.minY / 16.0), y1 = (int) Math.floor(box.maxY / 16.0);
        int z0 = (int) Math.floor(box.minZ / 16.0), z1 = (int) Math.floor(box.maxZ / 16.0);
        long span = (long) (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1);
        if (span <= 0 || span > MAXSEC) {
            return -1; // вырожденный/переполненный бокс ⇒ fail-open (никаких усечений)
        }
        int n = 0;
        for (int sx = x0; sx <= x1; sx++) {
            for (int sy = y0; sy <= y1; sy++) {
                if (sy < 0 || sy > 0x3FF) {
                    continue; // вне высоты мира: void_air, визитов нет — бит не нужен
                }
                for (int sz = z0; sz <= z1; sz++) {
                    keys[base + n++] = ((sx & 0x3FF) << 20) | ((sz & 0x3FF) << 10) | (sy & 0x3FF);
                }
            }
        }
        return n;
    }

    /**
     * Натив (RegisterNatives rust-стороной, src/inside_batch.rs): ОДИН переход
     * на T=512-бакет. out[base+i] = битмаска секций-кандидатов (superset|dirty)
     * для сущностей [base, base+n); rc &lt; 0 ⇒ ERR (java-реплика: all-ones =
     * чистая ваниль). ×463: сигнатура расширена base-офсетом (без копий бакета
     * — C4-дисциплина «ноль 64KB×74»); критические регионы &lt;5 µs (страж
     * на rust-стороне). Объявлен здесь, резолвится только в armed-ветке/
     * selfTest после RegisterNatives (NCDFE-канон).
     */
    private static native int insideBatchMask(
            int n, int maxsec, int base,
            long[] eids, double[] xyz, double[] bb,
            int[] secKeys, int[] nsec, int[] dirty, int[] out);

    /**
     * C5b selfTest (×463-дыра «0 самотестов» закрыта): вызывается rust-хуком
     * МЕЖДУ RegisterNatives и noteBatchArmed; 1 = PASS, 0 = FAIL (BATCH_ARMED
     * не публикуется). Состав (LEDGER-25 C5b):
     *  (а) AIOOBE-проба границы i=MAXBATCH−1 на всех 8 TL-буферах;
     *  (б) T-контракт: MAXBATCH % T == 0 (волна 4096=8×512);
     *  (в) count-инвариант superset: маска ⊇ геометрически ожидаемых секций
     *      на 256 LCG-сценах (фикс-сид 0x1D51DEBB) + boundary-офсеты;
     *  (г) overflow-сцена (&gt;MAXSEC) ⇒ nsec=0 ⇒ all-ones;
     *  (д) nsec==0 ⇒ all-ones (fail-open).
     */
    public static int selfTest() {
        try {
            // (а) AIOOBE-проба границы: запись+чтение последнего слота каждого буфера.
            long[] eids = TL_EIDS.get();
            double[] xyz = TL_XYZ.get();
            double[] bb = TL_BB.get();
            int[] keys = TL_KEYS.get();
            int[] nsec = TL_NSEC.get();
            int[] dirty = TL_DIRTY.get();
            int[] out = TL_OUT.get();
            int[] cur = TL_CUR.get();
            eids[MAXBATCH - 1] = -1L;
            xyz[MAXBATCH * 3 - 1] = -1.0d;
            bb[MAXBATCH * 6 - 1] = -1.0d;
            keys[MAXBATCH * MAXSEC - 1] = -1;
            nsec[MAXBATCH - 1] = -1;
            dirty[MAXBATCH - 1] = -1;
            out[MAXBATCH - 1] = -1;
            cur[0] = MAXBATCH - 1;
            if (eids[MAXBATCH - 1] != -1L || xyz[MAXBATCH * 3 - 1] != -1.0d
                    || bb[MAXBATCH * 6 - 1] != -1.0d || keys[MAXBATCH * MAXSEC - 1] != -1
                    || nsec[MAXBATCH - 1] != -1 || dirty[MAXBATCH - 1] != -1
                    || out[MAXBATCH - 1] != -1 || cur[0] != MAXBATCH - 1) {
                return 0;
            }
            cur[0] = 0;
            // (б) T-контракт.
            if (MAXBATCH % T != 0 || T <= 0 || T > MAXBATCH) {
                return 0;
            }
            // (в)-(д) оракул-сцены через реальный bulk-JNI (натив уже зарегистрирован).
            long seed = 0x1D51DEBBL;
            for (int s = 0; s < 256; s++) {
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                // x/z только в [+8,+4104]: sx,sz ∈ [0,256] — ключи БЕЗ wrap (±8192-граница
                // wrap- hazard задокументирован как v2 i64-CSR; оракул v1 честен in-contract).
                double cx = 8.0 + ((int) (seed >>> 33) % 4096);
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                double cy = 32.0 + ((int) (seed >>> 33) % 200);
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                double cz = 8.0 + ((int) (seed >>> 33) % 4096);
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                // s<253: полуширина ≤6.0 ⇒ span секций ≤ 2/ось ≤ 8 (без overflow);
                // s≥253: форс-overflow (span 3×3×3=27 &gt; MAXSEC) ⇒ (г) all-ones.
                double hw = (s < 253) ? 0.3 + ((int) (seed >>> 33) % 570) / 100.0 : 20.0;
                seed = seed * 6364136223846793005L + 1442695040888963407L;
                double hh = (s < 253) ? 0.3 + ((int) (seed >>> 33) % 570) / 100.0 : 1.0;
                // boundary-офсеты: сцены 250-252 прижаты к 16-границе минус eps.
                double bx0 = cx - hw, bx1 = cx + hw, by0 = cy - hh, by1 = cy + hh, bz0 = cz - hw, bz1 = cz + hw;
                if (s >= 250 && s < 253) {
                    double g = Math.floor(cx / 16.0) * 16.0;
                    bx0 = g - 9.999999747378752E-6;
                    bx1 = g + 9.999999747378752E-6 * 2.0;
                }
                bb[0] = bx0; bb[1] = by0; bb[2] = bz0;
                bb[3] = bx1; bb[4] = by1; bb[5] = bz1;
                int nk = sectionKeys(new AABB(bx0, by0, bz0, bx1, by1, bz1), keys, 0);
                if (nk < 0) {
                    nsec[0] = 0; // overflow ⇒ fail-open
                } else {
                    nsec[0] = nk;
                }
                dirty[0] = 0;
                int rc = insideBatchMask(1, MAXSEC, 0, eids, xyz, bb, keys, nsec, dirty, out);
                if (rc != 0) {
                    return 0;
                }
                int mask = out[0];
                if (nk < 0) {
                    if (mask != -1) {
                        return 0; // (г) overflow ⇒ all-ones
                    }
                    continue;
                }
                if (nk == 0) {
                    if (mask != -1) {
                        return 0; // (д) nsec==0 ⇒ all-ones
                    }
                    continue;
                }
                // (в) count-инвариант: каждая геометрически ожидаемая секция обязана иметь бит.
                double ex0 = bx0 - DEFLATE, ey0 = by0 - DEFLATE, ez0 = bz0 - DEFLATE;
                double ex1 = bx1 + DEFLATE, ey1 = by1 + DEFLATE, ez1 = bz1 + DEFLATE;
                int expected = 0;
                for (int k = 0; k < nk; k++) {
                    int key = keys[k];
                    double sx = ((key >> 20) & 0x3FF) * 16.0;
                    double sy = (key & 0x3FF) * 16.0;
                    double sz = ((key >> 10) & 0x3FF) * 16.0;
                    boolean hit = ex0 < sx + 16.0 && ex1 > sx
                            && ey0 < sy + 16.0 && ey1 > sy
                            && ez0 < sz + 16.0 && ez1 > sz;
                    if (hit) {
                        expected |= (1 << k);
                    }
                }
                if ((mask & expected) != expected || Integer.bitCount(mask) < Integer.bitCount(expected)) {
                    return 0;
                }
            }
            return 1;
        } catch (Throwable t) {
            return 0;
        }
    }

    /** Arm-хук моста (вызывает rust после define+RegisterNatives+selfTest==1). */
    public static void noteBatchArmed() {
        BATCH_ARMED = true;
    }
}
