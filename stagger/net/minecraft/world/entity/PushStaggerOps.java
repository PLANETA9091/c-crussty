package net.minecraft.world.entity;

import net.minecraft.world.phys.AABB;

import java.util.Collections;
import java.util.List;

/**
 * cmp401_stagger (TASK-401-I) — PUSH-SCAN STAGGER lane.
 *
 * Ванильный `LivingEntity.pushEntities()V` КАЖДЫЙ ТИК для каждой
 * LivingEntity выполняет un-gated broadphase скан соседей
 * `level().getPushableEntities(this, box)` (javap purpur-1.21.10,
 * offset 66-78; вызов из aiStep offset 850). При пустом списке ванилла
 * сразу возвращается — вся цена = скан + аллокация List. Это «моб-скан
 * соседей» из RECON-44 (getEntities 9.45% java).
 *
 * Механика (retarget 3B→3B receiver-prepended, строгий сайт=1):
 *   invokevirtual Level.getPushableEntities(Entity,AABB)List  →
 *   invokestatic PushStaggerOps.pushables(Level,Entity,AABB)List
 *
 * Гейт: (srvTick + golden32(eid)) % N == 0 — равномерный бюджет 1/N
 * сущностей за тик, каждая сущность проверяется каждый N-й тик
 * (golden-ratio 0x9E3779B9 по entity-id; тик = общий серверный
 * tickCount). На miss возвращает immutable пустой список — ванильное
 * тело по `list.isEmpty()` немедленно выходит (список не мутируется
 * до этой проверки — javap 79-85), эквивалентно isPushable()==false
 * раннему return. На hit — ванильный вызов без изменений.
 *
 * Инвариант ванильности (preregistration): запущенный push-разлип
 * задерживается ≤N-1 тик; cramming-онсет ≤N-1 тик позже (ванильный
 * ущерб сам nextInt(4)-рандомизирован). В сценах без overlap —
 * разницы ноль.
 *
 * Fail-closed: ARMED=false (N<2 или getServer()==null или env-мусор) →
 * прямой ванильный вызов. Бридж определён в kernel loader ТОЛЬКО когда
 * lever_flag == "cmp401_stagger" (rust-гейт, строго eq).
 */
public final class PushStaggerOps {

    private PushStaggerOps() {}

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
     * Retarget-точка (receiver Level prepended первым параметром).
     */
    public static List pushables(net.minecraft.world.level.Level level, Entity e, AABB box) {
        if (!ARMED) {
            return level.getPushableEntities(e, box);
        }
        long eid = e.getId();
        int phase = (int) (((eid & 0xFFFFFFFFL) * 0x9E3779B9L) >>> 32);
        if (Math.floorMod((long) SRV.getTickCount() + phase, N) != 0) {
            return Collections.emptyList(); // ванильное тело выйдет по isEmpty()
        }
        return level.getPushableEntities(e, box);
    }
}
