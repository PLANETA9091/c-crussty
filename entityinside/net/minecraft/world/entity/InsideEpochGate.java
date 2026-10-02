package net.minecraft.world.entity;

/**
 * INSIDE-EPOCH GATE (ID-P36, TASK-459-58 scaffold) — дешёвый пре-гейт перед
 * полным путём inside-snapshot serve (комбо к P32/inside_cache).
 *
 * МЕХАНИКА (карточка ID-P36, round-458p-ideas):
 *   per-section счётчик мутаций УЖЕ БАМПИТСЯ в InsideSnapOps.secWrite
 *   (volatile long Snap.gen++, bump после write — seqlock-дисциплина live,
 *   InsideSnapOps.java:505-518). P36 НЕ дублирует bump: этот мост держит
 *   ФЛЕТ-МАССИВ ЭПОХ (slot -> last-seen gen) и отдаёт решение
 *   "слот ещё тёплый?" ОДНИМ long-сравнением ДО lane-claim/CHM-обхода
 *   serve/serve4. Несовпадение (miss) -> полный путь -> его miss -> ваниль.
 *
 * КОНТРАКТ PARITY (бит-в-байт):
 *   - miss -> ваниль (единственный эффект = исполнение существующего пути);
 *   - hit-инвариант самтестом при арме: slot-serve обязан вернуть ТОТ ЖЕ ref,
 *     что полный serve (тот же объект, что ванильный readPalette); N>=3
 *     позиций на прогретую секцию, расхождение -> lever не публикуется
 *     (fail-dominant, см. src/inside_epoch_gate.rs);
 *   - эпоха 64-бит monotonic per-section: ABA/wrap недостижимы (2^63 реальных
 *     ref-неравных записей в одну секцию); слот ref-привязан к секции
 *     (SNAPS не эвиктит — sec->snap immutable).
 *
 * NCDFE-КАНОН (×93-indy, run 35902792520): класс определяется В РАННЕМ
 * ARM-ХУКЕ (src/inside_epoch_gate.rs activate -> kernel loader) ДО первого
 * gated-вызова; в <clinit> НЕТ indy/метод-ссылок и ничего Lane-типизированного
 * (plain-инициализация, паттерн InsideSnapOps.java:241-254).
 *
 * SCAFFOLD-ДИСЦИПЛИНА: файл СОЗНАТЕЛЬНО vanilla-free (только java.lang /
 * java.util.concurrent) — компилируется голым `javac --release 21` без
 * runtime-jar (SELF-CONTAINED-урок S7-148: никаких compile-dep вне себя).
 * Vanilla-сигнатуры (Level/BlockPos/BlockState fastGet) приземляются в
 * wiring-фазе; до неё мост не определяется ни в какой loader (lever dormant:
 * CRUSSTY_INSIDE_EPOCH_GATE off = байт-в-байт ваниль, класс вообще не грузится).
 *
 * НОМЕР КЛАССА: net.minecraft.world.entity — тот же пакет, что Entity/
 * InsideSnapOps (define_class в kernel loader, delivery-паттерн InsideBlockOps).
 */
public final class InsideEpochGate {

    private InsideEpochGate() {
    }

    // ------------------------------------------------------------------
    // ФЛЕТ-МАССИВ ЭПОХ (per-section slots)
    // ------------------------------------------------------------------

    /** Ёмкость слот-таблицы — калибрована с InsideSnapOps.CAP (1<<15). */
    static final int CAP = 1 << 15;

    /**
     * Флет-массив последних увиденных эпох. Индекс = слот секции
     * (identity-привязка слота к секции — wiring-фаза ставит ref-ключ в
     * параллельный реестр; SNAPS не эвиктит, так что слот жив, пока жива
     * запись SNAPS). Чтения volatile-free ДОЗВОЛЕНЫ только для решения
     * "тепло/холодно" (ложный cold = лишний полный путь, безопасно);
     * решение "горячий serve" валидируется повторным чтением Snap.gen
     * (seqlock-пара чтений) — wiring-фаза.
     */
    static final long[] EPOCHS = new long[CAP];

    /** Холодный слот (0 = "нет записи"; живой gen начинается с 1). */
    static final long SLOT_COLD = 0L;

    // ------------------------------------------------------------------
    // СТАТЫ (LongAdder-паттерн InsideSnapOps — striped, без CAS-горячих)
    // ------------------------------------------------------------------

    static final java.util.concurrent.atomic.LongAdder STAT_FAST_HITS =
            new java.util.concurrent.atomic.LongAdder();
    static final java.util.concurrent.atomic.LongAdder STAT_FAST_MISSES =
            new java.util.concurrent.atomic.LongAdder();
    static final java.util.concurrent.atomic.LongAdder STAT_SLOT_BINDS =
            new java.util.concurrent.atomic.LongAdder();
    /** Нарушений hit-инварианта (должно остаться 0; иначе lever не публикуется). */
    static final java.util.concurrent.atomic.LongAdder STAT_INVARIANT_FAILS =
            new java.util.concurrent.atomic.LongAdder();

    public static long fastHits() {
        return STAT_FAST_HITS.sum();
    }

    public static long fastMisses() {
        return STAT_FAST_MISSES.sum();
    }

    public static long slotBinds() {
        return STAT_SLOT_BINDS.sum();
    }

    public static long invariantFails() {
        return STAT_INVARIANT_FAILS.sum();
    }

    // ------------------------------------------------------------------
    // МЕХАНИКА ЭПОХ (scaffold-ядро; vanilla-обвязка в wiring-фазе)
    // ------------------------------------------------------------------

    /**
     * Привязать слот к эпохе секции (после первого тёплого serve).
     * slot обязан быть в [0, CAP); gen — живой Snap.gen секции.
     * Возвращает true, если слот был холодным (свежая привязка).
     */
    public static boolean bind(int slot, long gen) {
        checkSlot(slot);
        boolean wasCold = EPOCHS[slot] == SLOT_COLD;
        EPOCHS[slot] = gen;
        STAT_SLOT_BINDS.increment();
        return wasCold;
    }

    /** Последняя записанная эпоха слота (SLOT_COLD = не привязан). */
    public static long epochRead(int slot) {
        checkSlot(slot);
        return EPOCHS[slot];
    }

    /**
     * FAST-GATE РЕШЕНИЕ: тепёл ли слот против текущей эпохи секции.
     * true  -> секция не мутировала соСЛЕДНЕЙ привязки -> можно servить из
     *          слота, пропустив lane-claim/CHM-обход (P36-эффект);
     * false -> полный путь serve/serve4 (его miss -> ваниль).
     * ОДНО long-сравнение — весь смысл карточки (int-cmp до полного пути).
     */
    public static boolean epochMatch(int slot, long sectionGen) {
        checkSlot(slot);
        if (sectionGen == SLOT_COLD || EPOCHS[slot] != sectionGen) {
            STAT_FAST_MISSES.increment();
            return false;
        }
        STAT_FAST_HITS.increment();
        return true;
    }

    /** Сброс слота (pending/rebuild-путь wiring-фазы). */
    public static void unbind(int slot) {
        checkSlot(slot);
        EPOCHS[slot] = SLOT_COLD;
    }

    private static void checkSlot(int slot) {
        if (slot < 0 || slot >= CAP) {
            // вне CAP гейт молчит (ваниль/полный путь) — cap-семантика SNAPS
            throw new IndexOutOfBoundsException("epoch slot " + slot);
        }
    }

    // ------------------------------------------------------------------
    // HIT-ИНВАРИАНТ САМТЕСТ (вызывается arm-хуком ДО публикации BRIDGE_READY)
    // ------------------------------------------------------------------

    /**
     * Самоград: bind/epochMatch/unbind-механика обязана вести себя как
     * seqlock-слот. Любой fail инкрементирует STAT_INVARIANT_FAILS и
     * возвращает false — arm-хук НЕ публикует lever (fail-dominant).
     * Scaffold-срез проверяет механику слотов; ref-equal сравнение
     * slot-serve vs полный serve — в wiring-фазе (нужны vanilla-типы).
     */
    public static boolean selfTest() {
        boolean ok = true;
        final int probe = 0x7A36;
        try {
            // 1. холодный слот: match против gen=0 запрещён (0 = cold-маркер)
            unbind(probe);
            if (epochMatch(probe, SLOT_COLD)) {
                STAT_INVARIANT_FAILS.increment();
                ok = false;
            }
            // 2. bind(gen) -> match(gen) тёплый; match(gen+1) холодный
            bind(probe, 1L);
            if (!epochMatch(probe, 1L)) {
                STAT_INVARIANT_FAILS.increment();
                ok = false;
            }
            if (epochMatch(probe, 2L)) {
                STAT_INVARIANT_FAILS.increment();
                ok = false;
            }
            // 3. 64-бит monotonic: старший разряд не портит сравнение
            final long high = Long.MAX_VALUE - 3L;
            bind(probe, high);
            if (!epochMatch(probe, high) || epochMatch(probe, high + 1L)) {
                STAT_INVARIANT_FAILS.increment();
                ok = false;
            }
            // 4. unbind возвращает слот в cold
            unbind(probe);
            if (epochRead(probe) != SLOT_COLD || epochMatch(probe, high)) {
                STAT_INVARIANT_FAILS.increment();
                ok = false;
            }
        } catch (Throwable t) {
            STAT_INVARIANT_FAILS.increment();
            ok = false;
        }
        return ok;
    }
}
