package net.minecraft.world.entity;

import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;

/**
 * MOB-AI-WINDOW (TASK-406-D, vector R3 mob_ai_step — lever cmp406_aibatch).
 *
 * GOLDEN-PHASE AI-WINDOW PLANE: единый retarget сайта
 * {@code LivingEntity.aiStep → invokevirtual serverAiStep()V} (javap ground
 * truth purpur-1.21.10: ровно 1 сайт @ offset 277, внутри guard
 * {@code isEffectiveAi() && !level().isClientSide()}) на статический мост
 * {@code MobAiOps.serverAiStepGate(LivingEntity)V} (desc = virtual desc с
 * receiver-классом LivingEntity, препендированным — contract
 * retarget_virtual_to_static; dleg2 compose-reject урок: (LEntity;)V
 * отклоняется валидатором stack-shape). Мост:
 *
 *   - не-Mob (Player/ServerPlayer/ArmorStand) — ваниль {@code serverAiStep()}
 *     КАЖДЫЙ тик (видимая семантика игроков не трогается);
 *   - Mob вне окна — ВЕСЬ AI-плейн (sensing/targeting/goals/navigation/brain/
 *     move-look-jump controls) пропускается на этот тик (rate-инвариант
 *     mob-stagger сегмента, легализован upstream: Paper ActivationRange
 *     inactiveTick / Pufferfish DAB / Airplane DEAR tiered 1/2, 1/4);
 *   - Mob в окне — ванильный виртуальный {@code serverAiStep()}.
 *
 * ОКНО (rust = единственный источник решения): rust-плоскость mobs_soa держит
 * SoA-мобов; один раз за серверный тик (double-checked по volatile epoch)
 * мост делает ОДИН bulk-JNI {@code aiEpoch(tick, n, idTop, window)} — rust
 * DOD-проходом пишет коло́нку окна {@code window[denseId] = {0,1}} для ВСЕЙ
 * живой популяции (flags bit0=alive) в разделяемый java-массив; решение
 * пер-моба = O(1) чтение коло́нки по плотному id (MobPushOps.idBoxOf).
 * per-entity JNI отсутствует: JNI = ОДИН bulk-вызов на тик-батч (закон 6
 * RUST-FIRST). Правило окна: {@code floorMod(GOLDEN32(id) + tick, N) == 0},
 * N = CRUSSTY_AI_N | CRUSSTY_LEVER_ARG (clamp [2..64], default 4 — Airplane
 * DEAR tier N=4).
 *
 * FAIL-CLOSED: ENABLED STRICT-eq "cmp406_aibatch" (пустой/чужой флаг — сайт
 * вообще не ретаргетится rust-стороной); не-Mob/не в SoA-плоскости → ваниль
 * на этот вызов; aiProbe magic-mismatch / aiEpoch ERR_STRUCT → AI-плейн
 * дизарм навсегда (ваниль каждый тик); ERR_RANGE/эпоха-промах → ваниль на
 * этот тик (окно обновится на следующем). WINDOW-зеркало публикуется
 * volatile-парой (array, len): читатель под guard'ом id < len && id <
 * array.length — гонки grow безопасны (happens-before через volatile write
 * WINDOW_LEN после заполнения).
 */
public final class MobAiOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // STRICT eq (TASK-402-F урок полу-armed гейта): только точный флаг
        // раунда-406 или мультикомпозит-409. База stagtick вооружается
        // СВОИМИ гейтами; этот мост под cmp405_stagtick не вызывается
        // (rust не ставит сайт).
        return f != null && (f.trim().equals("cmp406_aibatch")
                // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                || f.trim().equals("cmp409_multi") || f.trim().equals("cmp412_meganav")
                // TASK-414-B: eqsnap-v3 family + leg flag cmp414_cvs (root-cause cv3b-1).
                || f.trim().equals("cmp412_eqsnapv3") || f.trim().equals("cmp414_cvs")
                // TASK-417-C: cvs-носитель ⊕ queryplane.
                || f.trim().equals("cmp417_bq")
                // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
                || f.trim().equals("cmp420_colpush"));
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x4149; // "AI"
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final Logger LOG = Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_ai.rs, RegisterNatives после define) ----
    private static native int aiProbe();
    private static native int aiEpoch(int tick, int n, int idTop, int[] window);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    /** Коло́нка окна (denseId -> 0 skip / 1 run), публикуется volatile. */
    private static volatile int[] WINDOW = new int[1024];
    /** Сколько валидных элементов записал rust в последний aiEpoch. */
    private static volatile int WINDOW_LEN = 0;
    /** Серверный тик последней успешной эпохи (double-checked locking). */
    private static volatile long EPOCH_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    /** One-shot ARM/effect-пруф (виден в server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;

    private MobAiOps() {}

    /** Ленивая проверка натива (Throwable -> false, ретрай на следующем тике). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (MobAiOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = aiProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Диагностика rust-стороны: дизарм ли плейн. */
    public static boolean armed() {
        return ENABLED && !broken && probeOnce();
    }

    /**
     * Замена сайта {@code invokevirtual serverAiStep()V} в LivingEntity.aiStep.
     * Возвращает void — stack-identical замещение (receiver consummирован).
     * Desc РОВНО (Lnet/minecraft/world/entity/LivingEntity;)V — virtual desc
     * ()V с receiver-классом, препендированным (валидатор compose).
     */
    public static void serverAiStepGate(LivingEntity le) {
        if (le instanceof Mob mob && skipAi(mob)) {
            return; // вне окна: AI-плейн этого моба пропущен (тело aiStep/пуш/коллизии — ваниль)
        }
        le.serverAiStep(); // ваниль (виртуально: Mob/Player/ArmorStand override)
    }

    /** Оконное решение: true — пропускать AI-плейн этому мобу на этот тик. */
    private static boolean skipAi(Mob mob) {
        if (broken) {
            return false;
        }
        int[] box = MobPushOps.idBoxOf(mob);
        if (box == null) {
            return false; // моб не в SoA-плоскости (новый/не-композитный путь) — ваниль
        }
        int id = box[0];
        long t = MinecraftServer.getServer().getTickCount();
        maybeEpoch(t);
        if (broken) {
            return false;
        }
        int[] w = WINDOW;
        int len = WINDOW_LEN;
        if (id < 0 || id >= len || id >= w.length) {
            return false; // вне последней эпохи — ваниль (fail-closed)
        }
        boolean skip = w[id] == 0;
        if (skip && !ARM_LOGGED) {
            ARM_LOGGED = true;
            LOG.info("[crussty-plugin] cmp406_aibatch: mob-ai window EFFECT armed (first skip at tick "
                    + t + ", denseId=" + id + ", n=" + windowN() + ")");
        }
        return skip;
    }

    /** N окна (та же лестница, что у rust: AI_N | LEVER_ARG, clamp [2..64], default 4). */
    static int windowN() {
        String s = System.getenv("CRUSSTY_AI_N");
        if (s == null || s.isBlank()) {
            s = System.getenv("CRUSSTY_LEVER_ARG");
        }
        if (s != null && !s.isBlank()) {
            try {
                int v = Integer.parseInt(s.trim());
                if (v >= 2) {
                    return Math.min(v, 64);
                }
            } catch (Throwable ignore) {
                // fall through to default
            }
        }
        return 4;
    }

    /**
     * Одна эпоха окна на серверный тик: ОДИН bulk-JNI по всей SoA-популяции
     * (rust пишет коло́нку окна). Double-checked по volatile EPOCH_TICK.
     * rc >= 0 = число записанных валидных элементов (=idTop отсканированных).
     */
    private static void maybeEpoch(long t) {
        if (EPOCH_TICK == t) {
            return; // горячий путь: один volatile-read
        }
        synchronized (EPOCH_LOCK) {
            if (EPOCH_TICK == t || broken) {
                return;
            }
            int n = windowN();
            int idTop = MobPushOps.idCount();
            int[] w = WINDOW;
            int cap = Math.max(1024, MobPushOps.idCapacity());
            if (w.length < cap) {
                w = new int[cap];
            }
            int rc;
            try {
                rc = aiEpoch((int) t, n, idTop, w);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                broken = true; // структурный отказ — весь рычаг в ваниль навсегда
                LOG.warning("[crussty-plugin] cmp406_aibatch: aiEpoch ERR_STRUCT — mob-ai window disarmed to vanilla");
                return;
            }
            if (rc == ERR_RANGE) {
                return; // параметр-промах: весь тик ваниль, окно ретраится на следующем тике
            }
            WINDOW = w;             // публикуем массив ДО длины
            WINDOW_LEN = rc;        // volatile write = publication edge для читателей
            EPOCH_TICK = t;
            if (!ARM_LOGGED) {
                LOG.info("[crussty-plugin] cmp406_aibatch: epoch ok tick=" + t
                        + " windowLen=" + rc + " n=" + n
                        + " (bulk JNI 1/tick over soa population)");
            }
        }
    }
}
