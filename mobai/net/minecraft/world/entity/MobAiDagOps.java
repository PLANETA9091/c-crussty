package net.minecraft.world.entity;

/**
 * C-X2 MOB-AI DAG SNAPSHOT — java-стаб контракта (TASK-459-81, round-459-cx2).
 *
 * Идея (RESEARCH-459-CX2.md): DAG зависимостей goals→sensors→brain строится
 * ОДИН раз за N тиков (snapshot-фаза, N = {@link #REBUILD_PERIOD} = 4, окно
 * согласовано с cmp406_aibatch golden-window); тики между — плоский проход по
 * замороженному топологическому порядку (frozen order, столбец
 * {@link #snapOrder}); мутации топологии (add/removeGoal, behavior add,
 * activity switch) бампают эпоху инвалидации — следующий window-тик
 * перестраивает DAG.
 *
 * Разделение фаз (Orkin, GOAP, GDC 2006: план строится редко — исполняется
 * часто; GameAIPro ch.6: плоская память узлов + стабильный обход):
 *   BUILD  (1 раз за N тиков): rust-сторона (ai_dag_snapshot.rs) строит
 *          Kahn-топосорт по ключу (tier, insertion-index) — insertion-порядок
 *          == ванильный порядок обхода множеств (паритет порядка, G2).
 *   EXEC   (каждый тик): плоский проход по столбцу snapOrder[denseId] —
 *          O(1) java-чтение на моба, ноль итератор-аллокаций
 *          (замена: GoalSelector.tick ×2 + tickRunningGoals ×2 сайтов в
 *          Mob.serverAiStep()V и Brain.startEachNonRunningBehavior — тройная
 *          вложенная итерация TreeMap/Map/Set, 3 iterator-alloc на моба/тик).
 *   EPOCH  (мутации): {@link #invalidate(int)} бампает INVALID_EPOCH; если
 *          java-сторая видит дрейф (javaEpoch != rust build_epoch) — ваниль
 *          весь тик (FAIL-CLOSED).
 *
 * Срезы-цели (профиль chkmono457-14): nav/ai остаток 2.75-3.2пп
 * (nav_ai 14.16%→3.31% после cmp406-окна) + brain 1-1.5пп
 * (Brain.tick 0.54% @559 сэмплов, tickEachRunningBehavior 0.10% @101).
 *
 * STRICT DORMANT (закон ×406/×421): lever = env CRUSSTY_LEVER_FLAG
 * STRICT-eq "cmp459_cx2"; пустой/чужой флаг = ваниль бит-в-байт. Этот стаб —
 * контракт для javap-верификации и будущей ARM-фазы: пока армирования нет,
 * {@link #gateEnabled()} возвращает false и все пути = ваниль.
 *
 * Порядок классов: вложенные классы резолвятся лениво через defining loader
 * (урок brainhook.rs: inner-first define при ARM-фазе).
 */
public final class MobAiDagOps {

    /** Период snapshot-фазы (тики); согласован с cmp406 golden-window N=4. */
    public static final int REBUILD_PERIOD = 4;

    /** Tier-ранги DAG = порядок ванильного прохода serverAiStep. */
    public static final int TIER_SENSOR = 0;
    public static final int TIER_GOAL = 1;
    public static final int TIER_BRAIN = 2;

    /** Маркер ARM-фазы (grep-гейт G1): "dormant" → "armed-pending-wiring". */
    public static final String STATUS = "dormant";

    /** Глобальная эпоха ПОСТРОЕНИЯ снапшота (rust-сторона инкрементит). */
    public static volatile int buildEpoch;

    /** Глобальная эпоха ИНВАЛИДАЦИИ (растёт на любой мутации топологии). */
    public static volatile int invalidEpoch;

    /**
     * Замороженный плоский порядок (shared int[]-столбец, bulk-JNI закон:
     * одна транзиция на батч — rust пишет столбец целиком под
     * GetPrimitiveArrayCritical, java читает O(1); образец mobs_ai window[]).
     * Индексация: snapOrder[k] = denseId в порядке топосорта; длина = topoLen.
     * До ARM-фазы — пустой (ваниль, ноль эффектов).
     */
    public static int[] snapOrder = new int[0];

    /** Число живых узлов в замороженном порядке (0 = ваниль). */
    public static int topoLen;

    /** Полный ли топосорт: false = цикл/мёртвый поставщик → ваниль весь тик. */
    public static boolean topoComplete = true;

    private MobAiDagOps() {
    }

    /**
     * STRICT-гейт вайринга: пока false — все сайты Mob.serverAiStep()V идут
     * ванилью (byte-indistinguishable). Стаб-константа: ARM-фаза перепишет
     * на чтение lever-флага; source-of-truth гейта = rust lever_enabled().
     */
    public static boolean gateEnabled() {
        return false;
    }

    /** EPOCH-ИНВАЛИДАЦИЯ: мутация топологии (goal/behavior слот denseId). */
    public static void invalidate(int denseId) {
        invalidEpoch++;
    }

    /** Чтение замороженного порядка: позиция узла denseId в topo, или -1. */
    public static int orderOf(int denseId) {
        final int[] order = snapOrder;
        final int n = topoLen;
        for (int k = 0; k < n; k++) {
            if (order[k] == denseId) {
                return k;
            }
        }
        return -1;
    }
}
