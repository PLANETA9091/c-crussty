package net.minecraft.world.entity;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;

/**
 * SLICES-LCG-EPOCH (TASK-459-80, WILD C-X1, закон 11 v18.2 — lever cmp459_cx1,
 * STRICT dormant): 64-битная LCG-эпоха на entity-секцию.
 *
 * ИДЕЯ: вместо полных инвалидаций EntitySectionArrays при мутациях срезов
 * (EntitySectionStorage / ClassInstanceMultiMap по 16³-секции) — per-section
 * 64-бит epoch: писатель бампает ОДНИМ add (LCG-шаг, нечётный ⇒ полный период
 * 2^64 по Халлу–Добеллу ⇒ нет ABA-повтора), читатель сравнивает эпоху ОДНИМ cmp
 * до полного пути (CHM-lookup + перестройка среза). Паттерн secWrite-эпох
 * ×458-P3 (P32 SNAP sidecar epoch-одним-int-cmp + P36 флет-массив эпох секций),
 * но на ENTITY-секциях и 64-бит (P36 требовал: «эпоха 64-бит»).
 *
 * SEQLOCK: снапшот валиден только при неизменной эпохе за время копии
 * (double-check до/после build; in-repo прецедент cmp399_shard version-sampling).
 * STALE-SERVE невозможен: дифт ⇒ discard ⇒ читатель идёт полным ванильным путём.
 *
 * STRICT DORMANT: класс НЕ определяется в kernel loader из dormant-состояния
 * (никакого define_class из src/slices_lcg_epoch.rs на этом раунде), трекед
 * блобы не пересобирались; ARMED=false по умолчанию ⇒ все пути = ваниль.
 * Активация только при CRUSSTY_LEVER_FLAG строго "cmp459_cx1" (пустой/чужой
 * флаг = ваниль бит-в-байт). FAIL-CLOSED (G6): любой Throwable → ваниль.
 *
 * GATES (RESEARCH-459-CX1.md §4): G1 ARM marker "cmp459_cx1: selfTest=true
 * BEFORE arm"; G2 lockstep бит-в-байт списков срезов vs прямой обход
 * EntitySectionStorage; G3 NCDFE=0 ($Nested вместе с Ops, канон 35902792520);
 * G4 stale-window=0; G5 population-parity, band 6.0–9.5M; G6 fail-closed DISARM.
 *
 * CAPTURE-МАТЕМ: моб-broadphase полка 4–8% CPU (item-версия 15.66%, items_index);
 * захват 80–95% ходок ⇒ +1.0..2.0пп при полной проводке, +0.5..1.0пп в композиции
 * P32/P36; потолок 4–8пп < 20пп ⇒ композиционная нога (закон 13).
 *
 * ИСТОЧНИКИ: cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf (Fraser, epoch-based
 * reclamation); en.wikipedia.org/wiki/Linear_congruential_generator (Hull–Dobell);
 * en.wikipedia.org/wiki/Seqlock; docs.rs/crossbeam-epoch.
 */
public final class SlicesLcgEpochOps {

    private SlicesLcgEpochOps() {}

    /** Flipped ONLY by rust after define+RegisterNatives+selfTest (dormant: никогда на этом раунде). */
    private static volatile boolean ARMED = false;

    /** Нечётный шаг Кнута: a=1 ⇒ полный период 2^64 (Халл–Добелл), bump = 1 add. */
    public static final long LCG_STEP = 1442695040888963407L;

    /** Sentinel «снапшота нет»: 0 никогда не выдаётся как валидная эпоха (fail-closed). */
    public static final long NO_SNAPSHOT = 0L;

    /** Per-section epoch: key = section-handle (EntitySection / секция-хозяин среза). */
    private static final ConcurrentHashMap<Object, AtomicLong> EPOCHS = new ConcurrentHashMap<>();

    /** STRICT-гейт: только точный флаг cmp459_cx1 (пустой/чужой = ваниль). */
    public static boolean strictGate() {
        String v = System.getenv("CRUSSTY_LEVER_FLAG");
        return v != null && "cmp459_cx1".equals(v.trim());
    }

    /** Rust flip (после selfTest; в dormant-состоянии не вызывается). */
    public static void arm() {
        ARMED = strictGate();
    }

    public static boolean isArmed() {
        return ARMED;
    }

    /**
     * WRITER-путь: bump эпохи секции одним add (нечётный шаг ⇒ нет ABA).
     * Идентичная запись (ref-equal state) НЕ бампает — secWrite-контракт
     * InsideSnapOps: old == newState ⇒ слова и палитра не изменились.
     * Дисарм/чужой флаг ⇒ NO_SNAPSHOT (ваниль, ноль эффектов).
     */
    public static long bumpEpoch(Object section, boolean realChange) {
        if (!ARMED) {
            return NO_SNAPSHOT; // дисарм = ваниль, ноль эффектов
        }
        return bumpRaw(section, realChange);
    }

    /** Ядро бампа без ARMED-гейта (для selfTest строго до arm). */
    private static long bumpRaw(Object section, boolean realChange) {
        try {
            if (section == null || !realChange) {
                return NO_SNAPSHOT;
            }
            return EPOCHS.computeIfAbsent(section, k -> new AtomicLong(LCG_STEP))
                    .addAndGet(LCG_STEP); // один add на steady-state
        } catch (Throwable t) {
            return NO_SNAPSHOT; // G6 fail-closed: ваниль
        }
    }

    /**
     * READER-фаст-гейт: один get + один cmp. true ⇒ снапшот среза жив, полный
     * путь (CHM-walk + перестройка) пропускается; false ⇒ полный ванильный путь.
     */
    public static boolean epochValid(Object section, long snapshotEpoch) {
        if (!ARMED) {
            return false; // дисарм = полный ванильный путь
        }
        return validRaw(section, snapshotEpoch);
    }

    /** Ядро гейта без ARMED-проверки (для selfTest строго до arm). */
    private static boolean validRaw(Object section, long snapshotEpoch) {
        try {
            if (section == null || snapshotEpoch == NO_SNAPSHOT) {
                return false;
            }
            AtomicLong a = EPOCHS.get(section);
            return a != null && a.get() == snapshotEpoch;
        } catch (Throwable t) {
            return false; // G6: miss → ваниль
        }
    }

    /** Seqlock double-check: снапшот валиден только при неизменной эпохе за копию. */
    public static boolean epochStable(Object section, long epochBefore, long epochAfter) {
        return ARMED && epochBefore == epochAfter && epochBefore != NO_SNAPSHOT
                && validRaw(section, epochBefore);
    }

    /** Self-test (строго ДО arm; маркер G1): логика ядра без ARMED-гейта. */
    public static boolean selfTest() {
        try {
            if (LCG_STEP % 2L == 0L) return false;                       // нечётность шага
            Object sec = new Object();
            long before = bumpRaw(sec, true);
            if (before == NO_SNAPSHOT) return false;
            if (bumpRaw(sec, false) != before) return false;              // идентичная запись не бампает
            long after = bumpRaw(sec, true);
            if (after != before + LCG_STEP) return false;                 // ровно один add
            if (!validRaw(sec, before)) return false;                     // таблица держит свежую эпоху
            if (validRaw(sec, before - LCG_STEP)) return false;           // старая эпоха = miss (нет ABA)
            if (validRaw(new Object(), after)) return false;              // чужая секция = miss
            if (epochValid(sec, after)) return false;                     // dormant: публичный гейт закрыт
            if (isArmed()) return false;                                  // dormant: ARMED=false
            return true;
        } catch (Throwable t) {
            return false; // G6
        }
    }
}
