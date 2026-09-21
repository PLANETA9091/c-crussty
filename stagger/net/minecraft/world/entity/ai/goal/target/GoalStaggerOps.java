package net.minecraft.world.entity.ai.goal.target;

import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.entity.player.Player;

/**
 * cmp401_stagger (TASK-401-I) — GOAL-CANUSE STAGGER lane (MULTI-RATE).
 *
 * Ванильный `GoalSelector.tick()V` update-фаза (javap offset 117-268)
 * поллит `WrappedGoal.canUse()` для КАЖДОЙ незапущенной цели КАЖДЫЙ ТИК.
 * Дорогие поллы — реальные сканы: `NearestAttackableTargetGoal.findTarget`
 * → `Level.getEntitiesOfClass` (энумерация всех сущностей в 32³-боксе:
 * Zombie$1→Turtle, Drowned$1→Axolotl), RemoveBlockGoal.findNearestBlock,
 * и т.д. Ванилла ограничивает их только randomInterval=10/20.
 *
 * MULTI-RATE ВАНИЛЬ-ИНВАРИАНТ («видимое — чаще, справочное — реже»):
 *  - TargetGoal кроме Player-NATG (HurtByTargetGoal, revenge, custom)
 *    → ванилла каждый тик (аггро-пути видимы);
 *  - NearestAttackableTargetGoal с targetType Player/ServerPlayer
 *    → ванилла каждый тик (aggro latency = видимое поведение; скан
 *      сам дёшев — getNearestPlayer по списку игроков);
 *  - прочие поллы (non-player NATG getEntitiesOfClass, stroll,
 *    remove-block, float, look...) → гейт 1/N по фазе
 *    (srvTick + golden(identityHashCode(wrappedGoal))) % N == 0;
 *    miss → false (точно то же, что canUse()==false в ванилле),
 *    hit → ванильный wg.canUse().
 *
 * Запущенные цели НЕ затрагиваются: cleanup-фаза (canContinueToUse) и
 * tickRunningGoals идут мимо этого сайта (site ровно 1 в tick()V,
 * offset 166). Goals, требующие every-tick апдейта после старта
 * (requiresUpdateEveryTick), стартуют как обычно — апдейт-каденс
 * задается tickRunningGoals, не canUse.
 *
 * Тик-источник: MinecraftServer.getServer().getTickCount() — статик,
 * сервер кэшируется в static final (бридж определяется ПОСЛЕ бута).
 *
 * Fail-closed: ARMED=false → каждый полл = ванильный wg.canUse().
 * RNG-замечание: пропущенный полл не потребляет nextInt — случайные
 * последовательности расходятся статистически (распределение то же —
 * ванилла сама рандомна).
 *
 * TASK-403-A N-scan: N выводится из CRUSSTY_LEVER_FLAG (stagn2→2,
 * stagn8→8, stagn16→16); иначе legacy-режим (CRUSSTY_STAGGER_N →
 * LEVER_ARG, дефолт 4).
 */
public final class GoalStaggerOps {

    private GoalStaggerOps() {}

    static final int N;
    static final net.minecraft.server.MinecraftServer SRV;
    static final boolean ARMED;

    static {
        int n = readN();
        net.minecraft.server.MinecraftServer srv = null;
        try {
            srv = net.minecraft.server.MinecraftServer.getServer();
        } catch (Throwable t) {
            srv = null;
        }
        N = n;
        SRV = srv;
        ARMED = N >= 2 && SRV != null;
    }

    private static int readN() {
        // TASK-403-A N-scan: флаг определяет окно детерминированно на ногу
        // (lever_arg=1 по банку; env-оверрайды не применяются к новым флагам).
        String flag = null;
        try {
            flag = System.getenv("CRUSSTY_LEVER_FLAG");
        } catch (Throwable ignored) {
            // env-мусор → legacy-режим
        }
        if ("cmp403_stagcomp2".equals(flag)) {
            return 2; // TASK-403 upper: stagcomp ⊕ stagn2 единый флаг
        }
        if ("cmp403_stagn2".equals(flag)) {
            return 2;
        }
        if ("cmp403_stagn8".equals(flag)) {
            return 8;
        }
        if ("cmp403_stagn16".equals(flag)) {
            return 16;
        }
        String raw = null;
        try {
            raw = System.getenv("CRUSSTY_STAGGER_N");
            if (raw == null || raw.isBlank()) {
                raw = System.getenv("CRUSSTY_LEVER_ARG");
            }
            if (raw != null) {
                int v = Integer.parseInt(raw.trim());
                if (v >= 2) {
                    return Math.min(v, 64);
                }
            }
        } catch (Throwable ignored) {
            // env-мусор → дефолт
        }
        return 4;
    }

    /**
     * Retarget-точка: единственный invokevirtual WrappedGoal.canUse()Z
     * внутри GoalSelector.tick()V (receiver-first форма сохранена).
     */
    public static boolean canUseGate(WrappedGoal wg) {
        if (!ARMED) {
            return wg.canUse();
        }
        Goal g = wg.getGoal();
        if (g == null) {
            return wg.canUse();
        }
        if (g instanceof TargetGoal) {
            if (!(g instanceof NearestAttackableTargetGoal)) {
                return wg.canUse(); // HurtBy и пр. — видимые, каждый тик
            }
            @SuppressWarnings("rawtypes")
            NearestAttackableTargetGoal natg = (NearestAttackableTargetGoal) g;
            Class<?> tt = natg.targetType;
            if (tt == Player.class || tt == net.minecraft.server.level.ServerPlayer.class) {
                return wg.canUse(); // player-aggro — видимый, каждый тик
            }
            // non-player NATG (Turtle/Axolotl...): дорогой справочный скан — 1/N
        }
        int phase = System.identityHashCode(wg) * 0x9E3779B9;
        if (Math.floorMod((long) SRV.getTickCount() + phase, N) != 0) {
            return false; // пропуск полла == ванильный canUse()==false
        }
        return wg.canUse();
    }
}
