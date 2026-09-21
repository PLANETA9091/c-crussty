package net.minecraft.world.entity;

import java.util.List;
import java.util.concurrent.atomic.AtomicLong;

import net.minecraft.world.entity.ai.control.JumpControl;
import net.minecraft.world.entity.ai.control.LookControl;
import net.minecraft.world.entity.ai.control.MoveControl;
import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.ai.navigation.PathNavigation;
import net.minecraft.world.entity.ai.sensing.Sensing;
import net.minecraft.world.entity.player.Player;

/**
 * AiWakeupOps — EVENT-DRIVEN AI WAKEUP (TASK-401-G, lever cmp401_wakeup).
 *
 * Расширение прецедента TASK-399-I (cmp399_wakeup, 4 goal-сайта): 9
 * LDC(W)-anchor'енных call-site'ов Mob.serverAiStep ретаргечены на
 * receiver-prepended statics (форма retarget_ldcw_virtual_to_static):
 *   2x GoalSelector.tickRunningGoals(Z) -> tickRunning(Mob,GoalSelector,Z)
 *   2x GoalSelector.tick()              -> tickSel(Mob,GoalSelector)
 *   1x Sensing.tick()                   -> tickSensing(Mob,Sensing)
 *   1x PathNavigation.tick()            -> tickNav(Mob,PathNavigation)
 *   1x MoveControl.tick()               -> tickMove(Mob,MoveControl)
 *   1x LookControl.tick()               -> tickLook(Mob,LookControl)
 *   1x JumpControl.tick()               -> tickJump(Mob,JumpControl)
 * customServerAiStep ("mob tick", Brain-мобы) и noActionTime (despawn) НЕ
 * входят в скоуп и остаются ванильными.
 *
 * Стационарные мобы (нет running MOVE/JUMP-цели, нет running target-goal, нет
 * path, нет target, нет recent damage) пропускают сенсорику/навигацию/контролы
 * и goal-циклы, пока не сработает событие-пробудитель, читаемое O(1) из
 * ВАНИЛЬНЫХ полей (hurt / setTarget / nav-busy), детерминированный таймер
 * id-хэш round-robin <= PERIOD тиков (редкие RandomStroll-роллы сохраняются
 * как в ваниле), ИЛИ появление игрока ближе PLAYER_RANGE блоков — poll
 * кэшируется (не чаще 1/тик/моба), чтобы видимое поведение (head-track,
 * aggro-реакция) оставалось ванильным рядом с игроками.
 *
 * Паритет:
 *  - активные/аггро мобы тикают ванильно 100% времени (hasCriticalTask);
 *  - спящий моб не меняет своё состояние вне опрошенных событий: goals уже
 *    заморожены (goal-циклы пропущены), navigation.tick() при isDone() —
 *    no-op, MoveControl при operation=WAIT — no-op, LookControl/JumpControl
 *    без драйвера-цели — no-op; sensing-кэш обновляется на первом тике
 *    пробуждения ДО target-роллов (sensing предшествует targeting в
 *    serverAiStep);
 *  - hurtTime/invulnerableTime декрементятся вне serverAiStep
 *    (LivingEntity.baseTick) — wake на первом тике;
 *  - выход из сна всегда через vanilla-цикл (ни один goal не стартует во сне).
 *
 * Fail-closed: класс определяется в loader Mob'а ТОЛЬКО когда lever armed и
 * Mob уже загружен (src/wakeup.rs, прецедент brainhook.rs); если define или
 * какой-либо из 9 ретаргетов не случился — ретаргеченных call-site'ов не
 * существует, ванильный путь по построению. Телеметрия — сэмплирование
 * 1/64-тик для id<4096 в tickSensing (каждый моб проходит ровно 1 раз/тик) +
 * shutdown-строка в stdout; hot-path без атомиков.
 */
public final class AiWakeupOps {
    private AiWakeupOps() {}

    /** Верхняя граница сна: детерминированный таймер (id-hash round-robin). */
    private static final int PERIOD = 40;
    /** Чистых тиков подряд до решения о сне. */
    private static final int CAND_TICKS = 4;
    /** Игрок ближе этого радиуса не даёт мобу спать (видимый паритет). */
    private static final double PLAYER_RANGE_SQR = 256.0;
    /** Сэмплирование телеметрии: каждый 64-й тик. */
    private static final int SAMPLE_MASK = 63;

    private static final AtomicLong SAMPLED_AWAKE = new AtomicLong();
    private static final AtomicLong SAMPLED_ASLEEP = new AtomicLong();

    static {
        try {
            Runtime.getRuntime().addShutdownHook(new Thread(() -> {
                System.out.println("[crussty-plugin] cmp401_wakeup: telemetry sampled_awake="
                    + SAMPLED_AWAKE.get() + " sampled_asleep=" + SAMPLED_ASLEEP.get()
                    + " (mob-ticks, id<4096, every 64th tick)");
            }, "crussty-wakeup-telemetry"));
        } catch (Throwable t) {
            // телеметрия не обязательна — тишаем
        }
    }

    // state[id*4+0]: 0 = awake, 1 = asleep (ai-подсистемы пропущены)
    // state[id*4+1]: tickCount на момент засыпания
    // state[id*4+2]: подряд идущих чистых тиков (candidacy streak)
    // state[id*4+3]: tickCount последнего proximity-poll (0 = не пилили)
    private static int[] state = new int[4096 * 4];

    private static int[] grow(int id) {
        int cap = state.length / 4;
        while (cap <= id) {
            cap *= 2;
        }
        int[] next = new int[cap * 4];
        System.arraycopy(state, 0, next, 0, state.length);
        state = next;
        return next;
    }

    /** Ретаргет-приёмник GoalSelector.tick()V (сайты targetSelector/goalSelector). */
    public static void tickSel(Mob mob, GoalSelector sel) {
        if (gate(mob)) {
            return;
        }
        sel.tick();
    }

    /** Ретаргет-приёмник GoalSelector.tickRunningGoals(Z)V (обе секции). */
    public static void tickRunning(Mob mob, GoalSelector sel, boolean stopAll) {
        if (gate(mob)) {
            return;
        }
        sel.tickRunningGoals(stopAll);
    }

    /** Ретаргет-приёмник Sensing.tick()V — первый сайт serverAiStep. */
    public static void tickSensing(Mob mob, Sensing sensing) {
        final boolean skip = gate(mob);
        if ((mob.tickCount & SAMPLE_MASK) == 0) {
            final int id = mob.getId();
            if (id >= 0 && id < 4096) {
                if (skip) {
                    SAMPLED_ASLEEP.getAndIncrement();
                } else {
                    SAMPLED_AWAKE.getAndIncrement();
                }
            }
        }
        if (skip) {
            return;
        }
        sensing.tick();
    }

    /** Ретаргет-приёмник PathNavigation.tick()V. */
    public static void tickNav(Mob mob, PathNavigation nav) {
        if (gate(mob)) {
            return;
        }
        nav.tick();
    }

    /** Ретаргет-приёмник MoveControl.tick()V. */
    public static void tickMove(Mob mob, MoveControl ctl) {
        if (gate(mob)) {
            return;
        }
        ctl.tick();
    }

    /** Ретаргет-приёмник LookControl.tick()V. */
    public static void tickLook(Mob mob, LookControl ctl) {
        if (gate(mob)) {
            return;
        }
        ctl.tick();
    }

    /** Ретаргет-приёмник JumpControl.tick()V (только ldc "jump"-сайт). */
    public static void tickJump(Mob mob, JumpControl ctl) {
        if (gate(mob)) {
            return;
        }
        ctl.tick();
    }

    /** true = любой не-спектатор-игрок в PLAYER_RANGE (видимый паритет). */
    private static boolean playerNear(Mob mob) {
        final List<? extends Player> ps = mob.level().players();
        for (int i = 0; i < ps.size(); i++) {
            final Player p = ps.get(i);
            if (!p.isSpectator() && p.distanceToSqr(mob) < PLAYER_RANGE_SQR) {
                return true;
            }
        }
        return false;
    }

    /**
     * true = пропустить ai-подсистему этого тика (моб спит).
     * Все ветки — plain field reads + int-массив, O(1), без аллокаций.
     */
    private static boolean gate(Mob mob) {
        final int id = mob.getId();
        final int base = id * 4;
        int[] st = state;
        if (base + 3 >= st.length) {
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
                st[base + 3] = 0;
                return false;
            }
            // proximity-poll не чаще 1/тик/моба (единственный O(players) poll).
            if (st[base + 3] != tick) {
                st[base + 3] = tick;
                if (playerNear(mob)) {
                    st[base] = 0;
                    st[base + 2] = 0;
                    return false;
                }
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
        st[base + 3] = 0;
        return false; // этот тик ещё ванильный (все подсистемы no-op)
    }

    /**
     * true = моб активен: running goal с MOVE/JUMP (движение/плавание) или
     * ЛЮБОЙ running target-goal (агрессия). LOOK-only running goals сну не
     * мешают — их видимый эффект (head-track) защищён proximity-wake.
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
