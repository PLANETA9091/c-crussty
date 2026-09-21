package net.minecraft.world.entity;

import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.WrappedGoal;

/**
 * AiWakeupOps — EVENT-DRIVEN AI WAKEUP-LIST (TASK-399-I, lever cmp399_wakeup).
 *
 * Стационарные мобы (нет running-goal с MOVE/JUMP-флагами, нет running
 * target-goal, нет path, нет target, нет recent damage) ВЫПАДАЮТ из goal-тикa:
 * 4 call-site'а Mob.serverAiStep ретаргечены (receiver-prepended statics,
 * форма retarget_ldc_virtual_to_static/RECON-13d) на tickSel/tickRunning ниже.
 * Возврат в AI — по событиям-пробудителям, читаемым O(1) из ВАНИЛЬНЫХ полей
 * (hurt/aggro/path — dirty-check без новых патч-точек), либо по
 * детерминированному таймеру пробуждения id-хэш round-robin <= PERIOD тиков,
 * чтобы редкие случайные стимулы (RandomStroll) сохраняли регулярные
 * canUse-роллы как в ваниле.
 *
 * Паритет (договор TASK-399-I):
 *  - активные/аггро мобы (running MOVE/JUMP или любой TARGET) тикают ванильно
 *    100% времени — hasCriticalTask блокирует сон;
 *  - спящий моб пропускает ТОЛЬКО GoalSelector.tick()/tickRunningGoals —
 *    sensing/navigation/customServerAiStep (Brain-мобы)/controls/noActionTime
 *    (despawn) остаются ванильными;
 *  - hurt -> hurtTime/invulnerableTime != 0 -> wake на первом тике
 *    (hurtTime декрементится в LivingEntity.baseTick, вне serverAiStep);
 *  - setTarget (в т.ч. кросс-моб alert) -> getTarget() != null -> wake;
 *  - внешний moveTo -> !getNavigation().isDone() -> wake;
 *  - сну предшествуют CAND_TICKS подряд чистых тиков; выход из сна всегда
 *    через тот же candidacy-фазу (ни один goal не стартует во сне: goal-циклы
 *    пропущены, стартовать ему неоткуда).
 *
 * Структура: плоский int[] = 3 слота на entity-id (sleep-флаг / sleep-stamp /
 * cand-стрик), grow по требованию. БЕЗ блокировок: int[]-элементы атомарны и
 * не рвутся (JLS 17.6), каждый моб тикается ровно одним region-воркером,
 * так что ячейка id имеет единственного писателя; отложенная видимость читов
 * стоит максимум один лишний ванильный тик (fail-safe направление).
 *
 * Fail-closed: класс определяется в loader Mob'а ТОЛЬКО когда lever armed и
 * Mob уже загружен (src/wakeup.rs, прецедент brainhook.rs/BrainOps); если
 * define/ретаргет не случился, ретаргеченных call-site'ов не существует —
 * ванильный путь по построению. Ноль аллокаций на hot-path (итераторы
 * getAvailableGoals — только в hasCriticalTask, 1/CAND_TICKS на моба).
 */
public final class AiWakeupOps {
    private AiWakeupOps() {}

    /** Верхняя граница сна: детерминированный таймер (id-hash round-robin). */
    private static final int PERIOD = 40;
    /** Чистых тиков подряд до решения о сне. */
    private static final int CAND_TICKS = 4;

    // state[id*3+0]: 0 = awake (тики ванильно), 1 = asleep (goal-циклы пропущены)
    // state[id*3+1]: tickCount на момент засыпания (штамп сна)
    // state[id*3+2]: подряд идущих "чистых" тиков (candidacy streak)
    private static int[] state = new int[4096 * 3];

    private static int[] grow(int id) {
        int cap = state.length / 3;
        while (cap <= id) {
            cap *= 2;
        }
        int[] next = new int[cap * 3];
        System.arraycopy(state, 0, next, 0, state.length);
        state = next;
        return next;
    }

    /** Ретаргет-приёмник GoalSelector.tick()V: Mob.serverAiStep pc 161/183. */
    public static void tickSel(Mob mob, GoalSelector sel) {
        if (gate(mob)) {
            return;
        }
        sel.tick();
    }

    /** Ретаргет-приёмник GoalSelector.tickRunningGoals(Z)V: pc 113/136. */
    public static void tickRunning(Mob mob, GoalSelector sel, boolean stopAll) {
        if (gate(mob)) {
            return;
        }
        sel.tickRunningGoals(stopAll);
    }

    /**
     * true = пропустить goal-цикл этого тика (моб спит).
     * Все ветки — plain field reads + int-массив, O(1), без аллокаций.
     */
    private static boolean gate(Mob mob) {
        final int id = mob.getId();
        final int base = id * 3;
        int[] st = state;
        if (base + 2 >= st.length) {
            st = grow(id);
        }
        final int tick = mob.tickCount;
        if (st[base] != 0) {
            // ASLEEP: поллинг событий-пробудителей по ванильному состоянию.
            if (mob.hurtTime != 0
                    || mob.invulnerableTime != 0
                    || mob.getTarget() != null
                    || !mob.getNavigation().isDone()
                    || (tick + id) % PERIOD == 0) {
                st[base] = 0;
                st[base + 2] = 0;
                return false;
            }
            return true;
        }
        // AWAKE: амортизированная кандидатность.
        if (mob.hurtTime != 0
                || mob.invulnerableTime != 0
                || mob.getTarget() != null
                || !mob.getNavigation().isDone()) {
            st[base + 2] = 0;
            return false;
        }
        final int c = st[base + 2] + 1;
        if (c < CAND_TICKS) {
            st[base + 2] = c;
            return false;
        }
        st[base + 2] = 0;
        if (hasCriticalTask(mob)) {
            return false; // активный/аггро моб — ваниль навсегда
        }
        st[base] = 1;
        st[base + 1] = tick;
        return false; // этот тик ещё ванильный (goal-цикл всё равно no-op)
    }

    /**
     * true = моб активен: running goal с MOVE/JUMP (движение/плавание:
     * Wander, Panic, FloatGoal, EatGrass...) или ЛЮБОЙ running target-goal
     * (TARGET: агрессия/аггро). LOOK-only running goals сну не мешают.
     */
    private static boolean hasCriticalTask(Mob mob) {
        for (WrappedGoal g : mob.targetSelector.getAvailableGoals()) {
            if (g.isRunning()) {
                return true;
            }
        }
        for (WrappedGoal g : mob.goalSelector.getAvailableGoals()) {
            if (g.isRunning()
                    && (g.getFlags().hasElement(Goal.Flag.MOVE)
                        || g.getFlags().hasElement(Goal.Flag.JUMP))) {
                return true;
            }
        }
        return false;
    }
}
