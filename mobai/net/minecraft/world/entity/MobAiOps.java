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
 * пер-моба = O(1) чтение плотного стампа по ванильному entity id
 * (TASK-427-A2 GATE-BITMAP: VAN_WIN[mob.getId()]; per-mob
 * MobPushOps.idBoxOf/CHM.get с hot path УБИТ — профиль round-mf427l2 top-3;
 * членство в окне публикуется эпохой ОДИН bulk-проход за тик).
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
                || f.trim().equals("cmp420_colpush")
                // TASK-421-A: brain-носитель (STRICT OR).
                || f.trim().equals("cmp421_brain")
                // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
                || f.trim().equals("cmp422_brain2")
                // TASK-424-A: GC-ревизия brain3 (STRICT OR).
                || f.trim().equals("cmp423_brain3")
                // TASK-426-A: SoA-feed carrier (STRICT OR).
                || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp432_inside2") || f.trim().equals("cmp436_ins4") || f.trim().equals("cmp451_senseins") || f.trim().equals("cmp458_swar") || f.trim().equals("cmp457_paldelta") || f.trim().equals("cmp457_eqsnap2") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3") || f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap") || f.trim().equals("cmp466_c98ai") || f.trim().equals("cmp468_s18fluid") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp453_diet") || f.trim().equals("cmp456_poi"));
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

    // ------------------------------------------------------------------
    // TASK-427-A2 GATE-BITMAP: плотные стампы по ВАНИЛЬНОМУ entity id,
    // публикуемые эпохой ОДИН bulk-проход за тик (закон 6, mandate п.3).
    // Профиль round-mf427l2: skipAi -> MobPushOps.idBoxOf -> CHM.get =
    // per-mob hash lookup для КАЖДОГО тикнутого моба каждый тик (inclusive
    // top-3 стека). Эпоха (уже под EPOCH_LOCK, O(idTop) java-проход по
    // WINDOW-колонке + byId) публикует VAN_WIN[vanillaId] = {0,1,2};
    // skipAi читает O(1) массив по mob.getId() — НИКАКИХ per-mob map
    // lookups. Семантика бит-в-байт прежней лестницы: не-стампнут (0) = не
    // в плоскости/вне окна → ваниль (fail-closed, был box==null / id>=len);
    // STAMP_RUN (1) = в плоскости, окно говорит RUN (был w[id]==1);
    // STAMP_SKIP (2) = в плоскости, окно говорит SKIP (был w[id]==0).
    // Видимость/гонки: тот же документированный контракт ghost ≤1 тик, что
    // у WINDOW-колонки (in-place мутируется эпохой под concurrently
    // читателями; транзиентный 0 = ваниль на одно решение — fail-safe
    // направление; растущий массив публикуется volatile-заменой).
    // ------------------------------------------------------------------
    private static final int STAMP_RUN = 1;
    private static final int STAMP_SKIP = 2;
    private static volatile int[] VAN_WIN = new int[4096];
    /** Ванильные id, застампнутанные ТЕКУЩЕЙ эпохой (чистятся следующей). */
    private static int[] touch = new int[4096];
    private static int touchN = 0;

    /** One-shot ARM/effect-пруф (виден в server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;

    /** TASK-426-A: one-shot DATA-PLAN-пруф (mobSlots>0 + windowLen>0 — fed-состояние
     * SoA-популяции, канон TASK-425(0); без него вердикт = плацебо-класс). */
    private static volatile boolean DATA_PLAN_LOGGED = false;

    /** Метка активного флага для ARM/DATA-PLAN-строк (TASK-426-A). */
    private static final String LABEL = label();

    private static String label() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f == null ? "(off)" : f.trim();
    }

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
        long t = MinecraftServer.getServer().getTickCount();
        maybeEpoch(t);
        if (broken) {
            return false;
        }
        // TASK-427-A2 GATE-BITMAP: O(1) dense read по ванильному entity id —
        // per-mob idBoxOf (ConcurrentHashMap.get) убит с hot path; членство
        // в окне публикуется эпохой ОДИН bulk-проход за тик.
        int vid = mob.getId();
        int[] vw = VAN_WIN;
        if (vid < 0 || vid >= vw.length) {
            return false; // не застампнут этой эпохой — не в плоскости → ваниль (fail-closed)
        }
        boolean skip = vw[vid] == STAMP_SKIP;
        if (skip && !ARM_LOGGED) {
            ARM_LOGGED = true;
            LOG.info("[crussty-plugin] cmp406_aibatch: mob-ai window EFFECT armed (first skip at tick "
                    + t + ", vanillaId=" + vid + ", n=" + windowN() + ")");
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
            if (idTop <= 0) {
                // TASK-424-A empty-plane short-circuit: SoA холодная — vanilla
                // БЕЗ JNI/буферов/лога на этот тик (эпоха помечена —
                // холостых эпох и спама «epoch ok» больше нет); WINDOW_LEN=0
                // → читатели fail-closed (id >= len → vanilla).
                WINDOW_LEN = 0;
                publishStamps(0, WINDOW); // TASK-427-A2: стампы прошлой эпохи сняты
                EPOCH_TICK = t;
                return;
            }
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
            // TASK-427-A2 GATE-BITMAP: ОДИН bulk java-проход O(idTop) за тик —
            // WINDOW-колонка (rust, только что записанная этим потоком) + byId
            // → плотные стампы по ванильному id. Throwable здесь = disarm
            // (fail-closed консервативно: никакой половинной публикации).
            try {
                publishStamps(rc, w);
            } catch (Throwable th) {
                broken = true;
                LOG.warning("[crussty-plugin] cmp406_aibatch: gate-bitmap publish threw " + th
                        + " — mob-ai window disarmed to vanilla");
                return;
            }
            EPOCH_TICK = t;
            if (!ARM_LOGGED) {
                ARM_LOGGED = true; // TASK-424-A: one-shot на publish (не в skip)
                LOG.info("[crussty-plugin] " + LABEL + ": epoch ok tick=" + t
                        + " windowLen=" + rc + " n=" + n
                        + " (bulk JNI 1/tick over soa population)");
            }
            if (!DATA_PLAN_LOGGED && rc > 0) {
                DATA_PLAN_LOGGED = true; // TASK-426-A: канонический DATA-PLAN-гейт
                LOG.info("[crussty-plugin] mobfeed DATA-PLAN: mobSlots=" + MobPushOps.idCount()
                        + " windowLen=" + rc + " tick=" + t
                        + " — SoA family FED (mobSlots>0 windowLen>0, flag=" + LABEL + ")");
            }
        }
    }

    /**
     * TASK-427-A2 GATE-BITMAP publisher: снять стампы прошлой эпохи (точный
     * O(prev-epoch) проход по touch-списку), застампнуть текущую — по
     * WINDOW-колонке + byId (dense-id юниверс MobPushOps). Вызывается ТОЛЬКО
     * под EPOCH_LOCK из maybeEpoch (один писатель). VAN_WIN публикуется
     * volatile-заменой при grow; ин-плейс стампы видны читателям с тем же
     * ghost ≤1-тик контрактом, что и WINDOW-колонка.
     */
    private static void publishStamps(int rc, int[] w) {
        for (int i = 0; i < touchN; i++) {
            int vid = touch[i];
            if (vid >= 0 && vid < VAN_WIN.length) {
                VAN_WIN[vid] = 0;
            }
        }
        touchN = 0;
        int[] vw = VAN_WIN;
        Entity[] byId = MobPushOps.byIdArr();
        int m = Math.min(Math.min(rc, w.length), byId.length);
        for (int d = 0; d < m; d++) {
            Entity e = byId[d];
            if (e == null) {
                continue; // свипнутый слот — стампа нет → ваниль
            }
            int vid = e.getId();
            if (vid < 0) {
                continue;
            }
            if (vid >= vw.length) {
                vw = java.util.Arrays.copyOf(vw, Math.max(vw.length * 2, vid + 1024));
                VAN_WIN = vw; // volatile-публикация выросшего массива
            }
            vw[vid] = (w[d] == 0) ? STAMP_SKIP : STAMP_RUN;
            if (touchN == touch.length) {
                touch = java.util.Arrays.copyOf(touch, touch.length * 2);
            }
            touch[touchN++] = vid;
        }
    }
}
