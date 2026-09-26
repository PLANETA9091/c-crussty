package net.minecraft.world.entity;

import net.minecraft.world.entity.monster.Monster;

/**
 * C21 LIGHT-ENGINE PLANE — census bridge (round-466, lever {@code cmp466_light}).
 *
 * VECTOR (закон 8 chunk/worldgen ось): свет-плоскость 150k-бенча. Профильная
 * ценз navmath1 cpu-collapsed (116,469 сэмплов, 300s, 150k pop): свет-READ
 * лейн = {@code Monster.updateNoActionTime → getLightLevelDependentMagicValue
 * → LevelLightEngine.getRawBrightness → StarLightInterface.getRawBrightness}
 * = 1632 сэмпла (1.40% all-CPU), плюс isSunBurnTick 150 (0.13%), canUse 37,
 * getPathfindingCostFromLightLevels 13 → вся read-плоскость ~1.57%; свет-COMPUTE
 * (StarLight задачи на light-threads PrioritisedQueueExecutorThread) = 84
 * сэмпла (0.07%). Потолок плоскости: 100% capture = ~+1.6пп CPU → ×12.7 ниже
 * бара +20 — REFUTED_CENS-класс (закон 13b), диспатч этой ноги = ценз-артефакт
 * (NOT-A-BENCH телеметрия, прецедент C20 F3-LevelTicks) + live capture-матем
 * для компо-клана (mob-family).
 *
 * БИТ-В-БАЙТ ВАНИЛЬНОСТЬ (javap ground truth patched-kernel.jar
 * net/minecraft/world/entity/monster/Monster.updateNoActionTime, 23 байта):
 * <pre>
 *   0: aload_0; 1: invokevirtual getLightLevelDependentMagicValue:()F
 *   4: fstore_1; 5: fload_1; 6: ldc 0.5f; 8: fcmpl; 9: ifle 22
 *   12: aload_0; 13: dup; 14: getfield LivingEntity.noActionTime:I
 *   17: iconst_2; 18: iadd; 19: putfield; 22: return
 * </pre>
 * Т.е. ваниль = {@code f = getLLDMV(); if (f > 0.5F) noActionTime += 2;}.
 * Этот мост вызывает ТОТ ЖЕ публичный {@code getLightLevelDependentMagicValue()}
 * (НЕ патченный этим левером — redirect переписывает ТОЛЬКО тело
 * updateNoActionTime, dispatch через invokevirtual на Entity.resolve →
 * идентичный ванильному) и повторяет сравнение/инкремент 1:1. Счётчики —
 * добавка ТОЛЬКО armed-ноги (вне lever vanilla-байты Monster вообще не
 * патчатся: STRICT-eq rust-гейт не ставит hook).
 *
 * FAIL-CLOSED: rust-сторона (light_plane.rs) конструирует patch ТОЛЬКО при
 * {@code CRUSSTY_LEVER_FLAG == "cmp466_light"}; пустой/чужой флаг = ни hook,
 * ни retransform, ни bridge — Monster остаётся ванильным байт-в-байт.
 */
public final class LightOps {

    /** Lever id (constant-pool marker для check_blobs_sync javap-гейта). */
    public static final String LEVER_ID = "cmp466_light";

    /** Census tag: каждое stderr-линие бьётся этим маркером (stdout-ценз). */
    public static final String CENS_TAG = "c21-light-cens";

    /** Flush interval: каждые 2^24 вызовов (≈5.6s при 3M calls/s → ~54 линии/300s). */
    private static final long FLUSH_MASK = 0xFFFFFFL;

    private static long calls;
    private static long hits;
    private static long lastNs;
    private static long lastCalls;
    private static long lastHits;

    private LightOps() {}

    /**
     * Ценз-обёртка ванильного {@code Monster.updateNoActionTime()V}
     * (whole-body redirect из light_plane.rs; receiver = dispatch-ный Monster).
     */
    public static void muaNoActionTime(Monster m) {
        calls++;
        if ((calls & FLUSH_MASK) == 0L) {
            flush();
        }
        float f = m.getLightLevelDependentMagicValue();
        if (f > 0.5F) {
            hits++;
            m.noActionTime += 2;
        }
    }

    private static void flush() {
        long now = System.nanoTime();
        if (lastNs == 0L) {
            lastNs = now;
            lastCalls = calls;
            lastHits = hits;
            return;
        }
        long dt = (now - lastNs) / 1_000_000L;
        long pc = calls - lastCalls;
        long ph = hits - lastHits;
        System.err.println("[" + CENS_TAG + "] lever=" + LEVER_ID
            + " calls=" + calls
            + " hits=" + hits
            + " period_ms=" + dt
            + " calls_per_s=" + (dt > 0 ? (pc * 1000L) / dt : -1L)
            + " hits_per_s=" + (dt > 0 ? (ph * 1000L) / dt : -1L));
        lastNs = now;
        lastCalls = calls;
        lastHits = hits;
    }
}
