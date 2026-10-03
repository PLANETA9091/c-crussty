package net.minecraft.world.entity;

/**
 * MOB-SWA EPOCH BRIDGE (AG-247 w530, TASK-458-I iter-2) — {@code swarEpoch},
 * the THIRD bulk-JNI in the EPOCH_LOCK window (window order: swarEpoch ->
 * eqEpoch -> senseArena; one bulk transition per subsystem per tick, закон 6).
 * Connects the rust iter-1 kernel (src/mobs_swa.rs @master f4a9e40f: 2×
 * lower_bound binsearch window over the min_x-sorted SoA, portable SWAR
 * 8-wide u64 batch-AABB mask, bit-for-bit scalar fallback, monotone
 * insertion-sort SAP push-tail) to the java broadphase lane
 * ({@code pushCandidates} enumeration = 8.8-10.9% CPU @47k+ population).
 *
 * КОНТРАКТ ОДНОГО ВЫЗОВА (superset-канон InsideBatchOps):
 *  1) java отдаёт фид живых боксов (long[] ids + float[] flat minX,maxX,minZ,
 *     maxZ — y НЕ пруним: вертикальный дрейф не ограничен маржой x/z,
 *     RESEARCH-458-I §1) и набор query-ректов этого тика;
 *  2) ОДИН нативный вызов {@code swarEpoch}: rust применяет фид к персистентной
 *     xmin-сортированной SoA (upsert/snapshot-evict), тянет монотонный SAP
 *     хвост (insertion pass O(n+inv), self-heal full re-sort при деградации),
 *     строит окно [minX - MARGIN - max_span .. maxX + MARGIN) 2×lower_bound и
 *     8-лановую SWAR-маску (mode 1) или бит-в-байт скалярную (mode 0);
 *  3) выдача CSR: outOff[q+1] префикс-смещения в outRow (entity id), outOvf[q]
 *     бит0 = переполнение строки (пер-квери fallback: java ведёт ЭТУ кверию
 *     ванильным перечислением — fail-open superset).
 * PARITY: строка = superset точных x/z-пересечений (inflate MARGIN=8.0 —
 * канон pushCandidates «≫ тик-дрейфа»); false-positive разрешён (ванильный
 * строгий хвост перепроверит), false-negative ЗАПРЕЩЁН: NaN/окно/маска —
 * fail-closed (NaN-лейн не матчится, канон-тест iter-1), ошибки структуры ⇒
 * ERR ⇒ java-тик ваниль.
 *
 * LEVER: STRICT-eq {@code CRUSSTY_LEVER_FLAG == "cmp458_swar"} (один id,
 * никаких союзов по env — swarx-4 урок; пустой/чужой флаг = класс не
 * определяется rust-лестницей, ваниль бит-в-байт).
 *
 * NCDFE-КАНОН (×93-indy, run 35902792520): класс определяется В РАННЕМ
 * arm-хуке (src/swar_bridge.rs activate -> kernel loader) ДО первого пуша
 * данных; в <clinit> НЕТ indy/метод-ссылок и НЕТ ThreadLocal.withInitial —
 * только plain-инициализация (паттерн InsideSnapOps.java:241-254, LongAdder
 * как InsideEpochGate); вложенных классов НЕТ (нечему ломаться в kernel
 * loader). ARM-лестница fail-closed: define -> RegisterNatives -> selfTest
 * -> noteSwaArmed (ARM строго последним); любой дефект = SWAR_BROKEN sticky
 * (ERR_STRUCT) или класс не армится вовсе — ваниль.
 *
 * SELF-CONTAINED (урок S7-148): только java.lang/java.util.concurrent —
 * компилируется без runtime-jar. Pinned-пересборка (toolchain пинненных
 * скриптов этого репо: ecj.jar --release-эквивалент 21 = major 65, nested
 * install FIRST, flat legacy copy, flat==nested gate):
 *   java -jar randomtick/ecj.jar -source 21 -target 21 -nowarn \
 *     -d BUILD entityinside/net/minecraft/world/entity/MobSwaOps.java
 *   cp BUILD/net/minecraft/world/entity/MobSwaOps.class \
 *     entityinside/build/net/minecraft/world/entity/   # nested (include_bytes!)
 *   cp BUILD/net/minecraft/world/entity/MobSwaOps.class \
 *     entityinside/build/MobSwaOps.class               # flat + cmp gate
 *
 * ITER-3 HANDOFF (сайты, не этой ногой): call-site в EPOCH_LOCK-окне
 * (EntityGoalQueryOps.maybeEpoch, сразу ПЕРЕД eqEpoch — eq-цепи строятся из
 * СВЕЖИХ колонок, ID-I01/I02), production-фид из MobPushOps (0-JNI FEED
 * scratch), строгий ванильный хвост по CSR-строкам (level + other≠entity +
 * live AABB.intersects + pushableBy — байт-в-байт), ovf/ERR-лестница
 * vanillaFill.
 */
public final class MobSwaOps {

    private MobSwaOps() {
    }

    // ------------------------------------------------------------------
    // ARM-КОНТРАКТ (rust-лестница: define -> RegisterNatives -> selfTest
    // -> noteSwaArmed; ARM строго последним, InsideBatchOps-дисциплина)
    // ------------------------------------------------------------------

    /** true ТОЛЬКО после define+RegisterNatives+selfTest (rust arm-хук). */
    static volatile boolean SWAR_ARMED = false;

    /** Sticky-дефект: ERR_STRUCT от натива жжёт лейн навсегда (дисциплина
     * MobPushOps ERR_STRUCT-disarm); до ARM не читается лестницей. */
    static volatile boolean SWAR_BROKEN = false;

    // RC-коды (надмножество rust ERR_*; java-вызыватель трактует ЛЮБОЙ rc != 0
    // как «ванильный тик для этой кверии/тика» — fail-closed).
    /** OK: outOff/outRow/outOvf когерентны (ovf-биты — пер-квери fallback). */
    public static final int RC_OK = 0;
    /** Структурная ошибка натива (null/длина) — sticky broken в swarTick. */
    static final int ERR_STRUCT = -1;
    /** Диапазонная ошибка (caps/длины массивов) — только этот тик ваниль. */
    static final int ERR_RANGE = -2;
    /** Мост не вооружён (или sticky broken) — чистая ваниль. */
    public static final int RC_DISARMED = 3;

    // ------------------------------------------------------------------
    // Канон-константы (RESEARCH-458-I §0-§1; зеркала src/mobs_swa.rs)
    // ------------------------------------------------------------------

    /** Push-маржа pushCandidates (box ± MARGIN): «≫ тик-дрейфа (~1-2 блока)». */
    public static final float MARGIN = 8.0f;

    /** Кап фида на один epoch-вызов (47k+ популяция — с запасом). */
    static final int MAXFEED = 65536;
    /** Кап query-ректов на один epoch-вызов (батч всего тика). */
    static final int MAXQ = 1024;
    /** Кап CSR-строк ВЫЗОВА (вызовер выход за кап = ovf-флаг кверии). */
    static final int ROW_TOTAL_CAP = 1 << 20;

    /** Режимы маски: 0 = скалярный fallback, 1 = SWAR 8-wide. */
    public static final int MODE_SCALAR = 0;
    public static final int MODE_SWAR = 1;

    // Статы (LongAdder-паттерн InsideEpochGate — striped, <clinit>-plain).
    static final java.util.concurrent.atomic.LongAdder STAT_CALLS =
            new java.util.concurrent.atomic.LongAdder();
    static final java.util.concurrent.atomic.LongAdder STAT_OVF_QUERIES =
            new java.util.concurrent.atomic.LongAdder();
    static final java.util.concurrent.atomic.LongAdder STAT_ERR =
            new java.util.concurrent.atomic.LongAdder();

    public static long statCalls() {
        return STAT_CALLS.sum();
    }

    public static long statOvfQueries() {
        return STAT_OVF_QUERIES.sum();
    }

    public static long statErr() {
        return STAT_ERR.sum();
    }

    /** Жив ли мост (ARMED и не sticky-broken) — для логов/телеметрии. */
    public static boolean isArmed() {
        return SWAR_ARMED && !SWAR_BROKEN;
    }

    /** Arm-хук моста (вызывает rust после define+RegisterNatives+selfTest). */
    public static void noteSwaArmed() {
        SWAR_ARMED = true;
    }

    // ------------------------------------------------------------------
    // EPOCH-ВХОД (iter-3 call-site: EntityGoalQueryOps.maybeEpoch, ДО eqEpoch)
    // ------------------------------------------------------------------

    /**
     * Один bulk-JNI на тик. rc==0 ⇒ outOff[0..nQ+1]/outRow/outOvf валидны;
     * rc != 0 ⇒ этот тик ваниль (вызыватель ведёт все кверии vanillaFill).
     * Каждая кверия с outOvf[qi]&1 = 0 валидна, с битом — ванильная кверия.
     * Дисциплина лестницы вызывателя (iter-3): swarRow-кандидаты -> строгий
     * ванильный хвост; ovf/ERR -> pushCandidates -> vanillaFill.
     */
    public static int swarTick(int mode, int snapshot, int nFeed, long[] feedIds,
            float[] feedBox, int nQ, float[] qBox, int[] outOff, long[] outRow,
            int[] outOvf) {
        if (!SWAR_ARMED || SWAR_BROKEN) {
            return RC_DISARMED;
        }
        STAT_CALLS.increment();
        final int rc;
        try {
            rc = swarEpoch(mode, snapshot, nFeed, feedIds, feedBox, nQ, qBox,
                    outOff, outRow, outOvf);
        } catch (Throwable t) {
            SWAR_BROKEN = true; // sticky:NoSuchMethodError/linked-type дефект
            STAT_ERR.increment();
            return ERR_STRUCT;
        }
        if (rc == ERR_STRUCT) {
            SWAR_BROKEN = true; // rust сказал «структура» — дизарм навсегда
        }
        if (rc != RC_OK) {
            STAT_ERR.increment();
        }
        return rc;
    }

    // ------------------------------------------------------------------
    // Нативы (RegisterNatives rust-стороной, src/swar_bridge.rs; резолвятся
    // только внутри selfTest/armed-ветки — NCDFE-канон ленивой линковки)
    // ------------------------------------------------------------------

    /**
     * mode 0=scalar | 1=SWAR; snapshot 1 = фид = ПОЛНЫЙ живой сет (evict),
     * 0 = частичный upsert; nFeed строк фида (ids[i] + feedBox[i*4..i*4+4]);
     * nQ кверий (qBox[i*4..i*4+4] = minX,maxX,minZ,maxZ); out: outOff[nQ+1]
     * префикс, outRow CSR entity-id, outOvf[nQ] пер-квери переполнение.
     * rc&lt;0 = ERR (структура/диапазон). Сигнатура = SWAR_EPOCH_SIG rust-стороны
     * БАЙТ-В-БАЙТ (урок 409-E: stray sig = silent sleeping gate).
     */
    private static native int swarEpoch(int mode, int snapshot, int nFeed,
            long[] feedIds, float[] feedBox, int nQ, float[] qBox,
            int[] outOff, long[] outRow, int[] outOvf);

    /** Сброс персистентной SoA моста (selfTest-гигиена; не hot-путь). */
    private static native int swarReset();

    // ------------------------------------------------------------------
    // САМТЕСТ (вызывается rust-лестницей ДО noteSwaArmed; fail-closed)
    // ------------------------------------------------------------------

    /**
     * Самоград end-to-end через СВЯЗКУ: java-оракул против rust-кернела.
     * 1) детерминированный LCG-фид 256 боксов + 32 кверий;
     * 2) mode SWAR vs mode SCALAR — offsets/rows/ovf БИТ-В-БИТ;
     * 3) superset-оракул: каждое точное x/z-пересечение (f32-арифметика как
     *    в кернеле: inflate MARGIN, 4 ordered cmp) обязано быть в строке;
     * 4) гигиена: swarReset() до и после.
     * Любой Throwable/false = lever не публикуется (fail-dominant).
     */
    public static boolean selfTest() {
        try {
            final int n = 256, qn = 32, rowCap = 1 << 16;
            long[] ids = new long[n];
            float[] fb = new float[n * 4];
            float[] qb = new float[qn * 4];
            long s = 0x2545F4914F6CDD1DL;
            float maxSpan = 0.0f;
            for (int i = 0; i < n; i++) {
                ids[i] = i + 1L;
                s = lcg(s);
                final float cx = (float) (s % 1024L) - 512.0f;
                s = lcg(s);
                final float cz = (float) (s % 1024L) - 512.0f;
                s = lcg(s);
                final float span = 0.6f + (float) (s % 15L) * 0.1f; // 0.6..2.0
                if (span > maxSpan) {
                    maxSpan = span;
                }
                fb[i * 4] = cx - span * 0.5f;
                fb[i * 4 + 1] = cx + span * 0.5f;
                s = lcg(s);
                final float zd = 0.6f + (float) (s % 15L) * 0.1f;
                fb[i * 4 + 2] = cz - zd * 0.5f;
                fb[i * 4 + 3] = cz + zd * 0.5f;
            }
            for (int q = 0; q < qn; q++) {
                s = lcg(s);
                final float qx = (float) (s % 1024L) - 512.0f;
                s = lcg(s);
                final float qz = (float) (s % 1024L) - 512.0f;
                final float hw = 1.0f + (float) (q % 7L);
                qb[q * 4] = qx - hw;
                qb[q * 4 + 1] = qx + hw;
                qb[q * 4 + 2] = qz - hw;
                qb[q * 4 + 3] = qz + hw;
            }
            if (swarReset() != RC_OK) {
                return false;
            }
            final int[] off1 = new int[qn + 1], off0 = new int[qn + 1];
            final int[] ovf1 = new int[qn], ovf0 = new int[qn];
            final long[] row1 = new long[rowCap], row0 = new long[rowCap];
            if (swarEpoch(MODE_SWAR, 1, n, ids, fb, qn, qb, off1, row1, ovf1) != RC_OK) {
                return false;
            }
            if (swarEpoch(MODE_SCALAR, 1, n, ids, fb, qn, qb, off0, row0, ovf0) != RC_OK) {
                return false;
            }
            // (2) бит-в-бит: скаляр обязан повторить SWAR-строку chunk-за-chunk
            for (int k = 0; k <= qn; k++) {
                if (off1[k] != off0[k]) {
                    return false;
                }
            }
            for (int k = 0; k < off1[qn]; k++) {
                if (row1[k] != row0[k]) {
                    return false;
                }
            }
            for (int k = 0; k < qn; k++) {
                if (ovf1[k] != ovf0[k]) {
                    return false;
                }
            }
            // (3) superset: точное f32-пересечение (инфлейт MARGIN) обязано
            // попасть в CSR-строку кверии (порядок строк не контрактуется)
            for (int q = 0; q < qn; q++) {
                if (ovf1[q] != 0) {
                    return false; // самтест-масштаб обязан влезать без ovf
                }
                final float qminX = qb[q * 4] - MARGIN, qmaxX = qb[q * 4 + 1] + MARGIN;
                final float qminZ = qb[q * 4 + 2] - MARGIN, qmaxZ = qb[q * 4 + 3] + MARGIN;
                for (int i = 0; i < n; i++) {
                    if (fb[i * 4] <= qmaxX && fb[i * 4 + 1] >= qminX
                            && fb[i * 4 + 2] <= qmaxZ && fb[i * 4 + 3] >= qminZ) {
                        boolean found = false;
                        for (int k = off1[q]; k < off1[q + 1]; k++) {
                            if (row1[k] == ids[i]) {
                                found = true;
                                break;
                            }
                        }
                        if (!found) {
                            return false; // false-negative ЗАПРЕЩЁН
                        }
                    }
                }
            }
            if (swarReset() != RC_OK) {
                return false;
            }
            return true;
        } catch (Throwable t) {
            return false;
        }
    }

    /** xorshift64*-стиль LCG-шаг (plain, без indy — NCDFE-канон <clinit>). */
    private static long lcg(long x) {
        return x * 6364136223846793005L + 1442695040888963407L;
    }
}
