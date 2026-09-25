package net.minecraft.world.level;

import java.util.HashSet;
import java.util.Set;

/**
 * SPAWN-BLOCK COHORT BITSET — TASK-459-83 (WILD C-X4, закон 11 тик-459).
 * Lever {@code cmp459_cx4}, STRICT eq. DORMANT stub: волна-1 = контракт +
 * selfTest-твин нативной модели (spawn_cohort_bitset.rs); вайринг natives
 * (RegisterNatives) и вызов из NaturalSpawner.isValidSpawnPostitionForType —
 * волна-2. Класс НЕ входит в tracked-блобы кернела (гейт placebo-канона:
 * lever в SOURCES без пересборки блобов = спящий гейт ×425/roar-2 — здесь
 * блобов нет вовсе, поэтому ложного ARM быть не может).
 *
 * <p>javap-грунд (round-396-a, JDK-21): NaturalSpawner.spawnCategoryForChunk
 * = 1×getRandomPosWithin + гейт y &gt; minY+1 → spawnCategoryForPosition →
 * isValidSpawnPostitionForType: PreCreatureSpawnEvent (offsets 14→40) →
 * canSpawnMobAt (113) → SpawnPlacements.isSpawnPositionOk (124) →
 * SpawnPlacements.checkSpawnRules (142, свет) → noCollision(getSpawnAABB)
 * (180). Когорта = (stateId, light, collision/fluid-класс, placement×category);
 * ивент и mobcap-плоскость когорт-переиспользуемыми НЕ являются.</p>
 *
 * <p>STRICT superset-гейт: реюз только после завершённой полной ваниль-проверки
 * (probe → Run → vanilla → observe); любой strict-флаг (event-abort /
 * unknown-light / unknown-collision) = отказ и заперт навсегда; BROKEN-латч
 * (переполнение/аномалия) → disarm в ваниль до рестарта JVM. При lever off
 * хук не регистрируется → поведение спавна бит-в-байт по построению.</p>
 *
 * <p>NCDFE-канон (волна-2): EARLY-define этого класса в arm-хуке ДО первого
 * spawn-цикла (прецедент MobPushOps.pushables:467, d73758a3-носители);
 * гейт T1: NCDFE=0, иначе DELIVERY-FAIL.</p>
 */
public final class SpawnCohortBitsetOps {

    /** STRICT eq lever (канон cmp459_cx4; пустой/чужой флаг = ваниль). */
    public static final String LEVER = "cmp459_cx4";

    /** Емкость когортной таблицы одного тика (ноль-alloc после старта). */
    public static final int SLOTS = 4096;

    /** Строгие флаги входа: любое отличие от чистой блочной проверки = отказ. */
    public static final int FL_EVENT_ABORT = 1 << 0;   // PreCreatureSpawnEvent — позиционно-зависим
    public static final int FL_UNKNOWN_LIGHT = 1 << 1; // свет не читался/вне 0..15
    public static final int FL_UNKNOWN_COLL = 1 << 2;  // collision/fluid-класс не разрешён

    private static volatile boolean ok = true; // one-shot disarm (волна-2: JNI-аномалии)

    private SpawnCohortBitsetOps() {
    }

    /** armed(): STRICT eq по trim; BROKEN-латч отключает эффективный ARM. */
    public static boolean armed() {
        String v = System.getenv("CRUSSTY_LEVER_FLAG");
        return v != null && v.trim().equals(LEVER) && ok;
    }

    /** Волна-2: reset когорт на границе тика (натив). В dormant — не вызывается. */
    private static native void cohortReset();

    /** Волна-2: probe когорты; bit0=skip, bit1=refused (натив; broken → 0/ваниль). */
    private static native int cohortProbe(long key, int strictFlags);

    /** Волна-2: observe завершённой ваниль-проверки (натив). */
    private static native void cohortObserve(long key);

    /**
     * Ключ когорты: stateId(24b) | light(4b) | coll(4b) | cat(8b); старшие
     * 24b = 0 — бит-в-байт твин pack_key (spawn_cohort_bitset.rs).
     */
    public static long packKey(int stateId, int light, int coll, int cat) {
        return ((stateId & 0x00FF_FFFFL) << 16)
                | ((light & 0x0FL) << 12)
                | ((coll & 0x0FL) << 8)
                | (cat & 0xFFL);
    }

    /**
     * selfTest — офлайн lockstep-оракул (G2, бит-в-байт): java-твин модели
     * против наивной HashSet-реплики на детерминированном LCG-потоке; плюс
     * refusal-семантика строгих флагов и reset-семантика тика. Вызывается
     * в волнах-1/2 из теста носителя; не требует натива и кернел-классов.
     *
     * @return true — все инварианты соблюдены
     */
    public static boolean selfTest() {
        final int n = 4096;
        MiniBitset bs = new MiniBitset(SLOTS);
        Set<Long> reference = new HashSet<>();
        long z = 0x123456789ABCDEFL;
        int refusalsSeen = 0;
        int skipsSeen = 0;
        for (int i = 0; i < n; i++) {
            z = z * 6364136223846793005L + 1442695040888963407L;
            int stateId = (int) ((z >>> 33) % 256);
            z = z * 6364136223846793005L + 1442695040888963407L;
            int light = (int) ((z >>> 33) % 4);
            z = z * 6364136223846793005L + 1442695040888963407L;
            int coll = (int) ((z >>> 33) % 2);
            z = z * 6364136223846793005L + 1442695040888963407L;
            int cat = (int) ((z >>> 33) % 8);
            z = z * 6364136223846793005L + 1442695040888963407L;
            int flags = (int) ((z >>> 33) % 16);
            long key = packKey(stateId, light, coll, cat);
            int strict = flags != 0 ? FL_UNKNOWN_COLL : 0;

            int v = bs.probe(key, strict); // bit0=skip, bit1=refused
            boolean wantSkip = strict == 0 && reference.contains(key);
            if ((v & 1) == 1 != wantSkip) {
                return false; // lockstep бит-в-байт нарушен
            }
            if (strict != 0) {
                if ((v & 2) == 0) {
                    return false; // refusal не помечен
                }
                refusalsSeen++;
            } else if (v == 0 && !reference.contains(key)) {
                reference.add(key); // ваниль-проверка завершена → observe
                bs.observe(key);
            }
            if ((v & 1) == 1) {
                skipsSeen++;
            }
        }
        if (bs.broken || bs.skips != skipsSeen || bs.refusals != refusalsSeen) {
            return false;
        }
        // Reset-семантика: меж-тиковый реюз запрещён.
        bs.resetTick();
        for (long k : reference) {
            if (bs.probe(k, 0) == 1) {
                return false; // после reset ни одна когорта не skip
            }
            bs.observe(k);
        }
        return true;
    }

    /**
     * Java-твин нативной модели (детерминированный, без натива): открытая
     * адресация + bitset checked; семантика 1:1 с spawn_cohort_bitset.rs.
     */
    static final class MiniBitset {
        private final long[] keys = new long[SLOTS];
        private final boolean[] used = new boolean[SLOTS];
        private final boolean[] checked = new boolean[SLOTS];
        int skips;
        int refusals;
        boolean broken;

        MiniBitset(int slotsIgnored) {
            // SLOTS фиксирован константой контракта; аргумент — для читаемости.
        }

        private static int slot(long key) {
            long z = key + 0x9E3779B97F4A7C15L;
            z = (z ^ (z >>> 30)) * 0xBF58476D1CE4E5B9L;
            z = (z ^ (z >>> 27)) * 0x94D049BB133111EBL;
            return (int) ((z ^ (z >>> 31)) & (SLOTS - 1));
        }

        int probe(long key, int strictFlags) {
            if (broken) {
                return 0; // fail-closed: ваниль
            }
            if (strictFlags != 0) {
                refusals++;
                return 2;
            }
            int start = slot(key);
            for (int p = 0; p < SLOTS; p++) {
                int i = (start + p) & (SLOTS - 1);
                if (!used[i]) {
                    used[i] = true;
                    keys[i] = key;
                    return 0;
                }
                if (keys[i] == key) {
                    if (checked[i]) {
                        skips++;
                        return 1;
                    }
                    return 0;
                }
            }
            broken = true; // one-shot BROKEN → ваниль до конца процесса
            return 0;
        }

        void observe(long key) {
            if (broken) {
                return;
            }
            int start = slot(key);
            for (int p = 0; p < SLOTS; p++) {
                int i = (start + p) & (SLOTS - 1);
                if (!used[i]) {
                    return; // незавершённая проверка — checked не поднимается
                }
                if (keys[i] == key) {
                    checked[i] = true;
                    return;
                }
            }
        }

        void resetTick() {
            java.util.Arrays.fill(used, false);
            java.util.Arrays.fill(checked, false);
        }
    }

    /** CLI: java -cp build net.minecraft.world.level.SpawnCohortBitsetOps */
    public static void main(String[] args) {
        System.out.println("armed(off-env)=false → " + !armed());
        System.out.println("selfTestCohort=" + selfTest());
    }
}
