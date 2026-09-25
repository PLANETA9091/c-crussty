package net.minecraft.world.entity;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.Shapes;

/**
 * TASK-459-56 (ID-P31, закон 11 WILD) — INSIDE-BATCH: discovery тика
 * {@code Entity.checkInsideBlocks} одним bulk-JNI (scaffold).
 *
 * Гейт-паттерн — сиблинг InsideBlockOps.gate (S7-135): retarget
 * {@code Entity.isAffectedByBlocks} @ offset 1 внутри
 * {@code checkInsideBlocks(List, StepBasedCollector)} на
 * {@code InsideBatchOps.batchGate(Entity)Z} (receiver-first, 3B→3B,
 * длина сохранена). Владелец сайта ОДИН (S7-162 supersede-дисциплина
 * entity_compose): armed inside_batch вытесняет inside_cache на сайте.
 *
 * ФОРМА ТИКА (закон 6 — подсистема целиком: gate+discovery+tail):
 *  1) java собирает батч ВСЕХ checkInsideBlocks-кандидатов тика
 *     (entity id, x/y/z, bb-флет 6×double, section-ключи 16³-секций,
 *     пересекаемых swept-боксом; фиксированная ширина MAXSEC=8);
 *  2) ОДИН нативный вызов insideBatchMask (RegisterNatives, arm-order
 *     контракт: класс определён + нативы зарегистрированы + selfTest ДО
 *     BATCH_ARMED=true) → Rust возвращает битмаску секций-кандидатов
 *     per-entity (superset);
 *  3) java строгий ВАНИЛЬНЫЙ хвост visit-обхода по секциям-кандидатам:
 *     порядок/шаги/флаги 1:1 с ванилью (advanceStep + entityInside +
 *     onInsideBlock + FluidState.entityInside), applyAndClear остаётся
 *     ванильным вызывателем.
 *
 * PARITY: маска = superset. False-positive разрешён (лишний кандидат
 * обслуживает ванильный хвост), false-negative ЗАПРЕЩЁН: любая ошибка
 * структуры/переполнение/чужой collector ⇒ all-ones ⇒ чистая ваниль.
 * Dirty-list секций-мутантов (secWrite-бампы, паттерн entity_index.rs)
 * OR-ится в маску между сборкой батча и хвостом — lost-effect невозможен.
 *
 * NCDFE-канон (паттерн d73758a3/5ecd841a, fa9054d9 ARM-AFTER-DEFINE):
 * класс определяется в раннем arm-хуке ДО первого retarget-вызова; констант-
 * пул ref insideBatchMask резолвится только внутри armed-ветки ⇒ при
 * BATCH_ARMED=false NCDFE недостижим (урок leg #2'' 35318755582).
 *
 * Lever dormant: CRUSSTY_INSIDE_BATCH не установлен ⇒ класс не определяется,
 * сайт не компонуется, хук не регистрируется — ваниль бит-в-байт.
 *
 * =========================================================================
 * P34 QUANTUM-REST SIDECAR (TASK-460-40, квант-гейт поверх INSIDE-BATCH):
 * пропуск re-check при K-тиках покоя.
 *
 * Класс «квант-покой» (RESEARCH-459-P34.md, модель src/inside_quantum_gate.rs):
 * подвижные-но-медленные сущности 0 &lt; |Δ| &lt; ε на протяжении K тиков
 * подряд (jitter-in-place, crowd-pressed, gravity-settling). Ванила платит
 * за них ДВЕ полных traversal за тик (main from-to + финальный to-to,
 * вторая даёт только visitedBlocks-дюпы) + LongOpenHashSet dedup + visitor.
 *
 * ВАЙРИНГ (этот тик, v1): после K тихих тиков DWELL каждый REST-тик мост
 * выполняет СВЕЖИЙ swept-обход дрейф-сегмента (xo→current, бокс в текущей
 * позиции, deflate 9.999999747378752E-6) теми же ванильными примитивами,
 * что static-зеркало InsideBlockOps.Recorder (javap-контракт визитора:
 * isAlive-гейт 0-8, budget 16 = 9-16, hitShape/inFluid 80-135, эффекты
 * в visit-порядке 192-315), и возвращает false — ванильное тело
 * checkInsideBlocks пропущено (вторая traversal + dedup не исполняются),
 * applyAndClear делает ванильный вызыватель.
 *
 * VANILLA-СЕМАНТИКА / DIRTY-LIST: серв = СВЕЖИЙ обход живых состояний
 * (level.getBlockState каждый visit) — мутация секции-мутанта НЕ МОЖЕТ
 * дать устаревший результат: блок-лист мутантов обязан пере-чекаться ⇒
 * обеспечен структурно (кэшированных РЕЗУЛЬТАТОВ нет — переигрываются
 * ВЫЗОВЫ по живым состояниям, контракт static-гейта «кэшируются ВЫЗОВЫ,
 * не результаты»); гистерезис: один скачок |Δ| ≥ ε ⇒ деклассификация +
 * 1 обязательный полный ванильный тик (Schmitt-форма, без флаттера на
 * границе ε); backlog-переполнение (дрен-фолт) ⇒ one-shot DISARM —
 * ваниль навсегда (fail-closed). Точный ноль Δ (биты) — владение
 * static-класса: НЕ наш (дизъюнкт, Decision::Static), ваниль.
 *
 * КОНСТАНТЫ ε/K/MAXSTEPS/BACKLOG_CAP — оффлайн-оракул карточки ID-P34
 * (v1-дефолты; re-derive до любого изменения). NCDFE: все новые refs
 * (Unsafe-поле collector'а, ядерные классы обхода) резолвятся в
 * static-init/armed-ветке после define в раннем arm-хуке (rust);
 * QUANTUM_ARMED ставится rust-стороной ТОЛЬКО после quantumSelfTest()==1
 * (arm-order: define → natives → selfTest → arm).
 * =========================================================================
 */
public final class InsideBatchOps {

    private InsideBatchOps() {}

    /** Arm-order контракт: true ТОЛЬКО после define+RegisterNatives+selfTest (rust arm-хук). */
    static volatile boolean BATCH_ARMED = false;

    /** Максимальное число секций-кандидатов на сущность (2×2×2 при дефляции traversal). */
    static final int MAXSEC = 8;

    /** Джавап-дефляция ванильного traversal-бокса (зеркало InsideBlockOps.DEFLATE). */
    static final double DEFLATE = 9.999999747378752E-6d;

    /** Бюджет батча на тик (переполнение ⇒ all-ones для хвоста — ваниль). */
    static final int MAXBATCH = 4096;

    // ThreadLocal плоские буферы батча (ноль аллокаций на тик после прогрева;
    // никакой ооп-массив в hot-пути — урок §153/§155).
    static final ThreadLocal<long[]> TL_EIDS = ThreadLocal.withInitial(() -> new long[MAXBATCH]);
    static final ThreadLocal<double[]> TL_XYZ = ThreadLocal.withInitial(() -> new double[MAXBATCH * 3]);
    static final ThreadLocal<double[]> TL_BB = ThreadLocal.withInitial(() -> new double[MAXBATCH * 6]);
    static final ThreadLocal<int[]> TL_KEYS = ThreadLocal.withInitial(() -> new int[MAXBATCH * MAXSEC]);
    static final ThreadLocal<int[]> TL_NSEC = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_DIRTY = ThreadLocal.withInitial(() -> new int[MAXBATCH]);
    static final ThreadLocal<int[]> TL_OUT = ThreadLocal.withInitial(() -> new int[MAXBATCH]);

    // ------------------------------------------------------------------
    // P34 QUANTUM-REST SIDECAR: плоское состояние классификатора
    // (слот = eid & (QNSLOTS-1), штамп = eid, 0 = пусто; 2^18 слотов
    // покрывают 150k живых — TASK-432-B ёмкостный контракт static-семьи).
    // ------------------------------------------------------------------

    static final int QNSLOTS = 1 << 18;
    /** ε: подпороговое смещение за тик (оракул ID-P34, v1-дефолт). */
    static final double Q_EPSILON = 1.0e-4;
    static final double Q_EPSILON_SQR = Q_EPSILON * Q_EPSILON;
    /** K: тихих тиков до промоции в rest-класс (оракул ID-P34, v1-дефолт). */
    static final int Q_K_TICKS = 20;
    /** Бюджет визитора ванили (javap 9-16) — кап дрена сегментов за тик. */
    static final int Q_MAXSTEPS = 16;
    /** Жёсткий кап бэклога: выше — дрен-фолт ⇒ one-shot disarm. */
    static final int Q_BACKLOG_CAP = Q_MAXSTEPS + Q_K_TICKS;
    /** Ванильный moved-порог (javap: square(0.9999900000002526)) для intersected-флага. */
    static final double Q_MOVED_SQR = 0.9999900000002526d * 0.9999900000002526d;

    static final int QP_VANILLA = 0;
    static final int QP_DWELL = 1;
    static final int QP_REST = 2;
    static final int QP_DISARMED = 3;

    /** Решения stepClassifier: 0 = ваниль (не пропускать), 1 = SERVE (сегмент переигран мостом). */
    static final int QD_VANILLA = 0;
    static final int QD_SERVE = 1;

    static final long[] Q_EID = new long[QNSLOTS];
    static final int[] Q_PHASE = new int[QNSLOTS];
    static final int[] Q_DWELL = new int[QNSLOTS];
    static final int[] Q_HOLD = new int[QNSLOTS];
    static final int[] Q_PENDING = new int[QNSLOTS];

    /** Scratch {phase, dwell, hold, pending} для stepClassifier (ноль аллокаций на тик). */
    static final ThreadLocal<int[]> TL_QSTATE = ThreadLocal.withInitial(() -> new int[4]);

    /** QUANTUM_ARMED ставит rust ТОЛЬКО после define+natives+quantumSelfTest()==1. */
    static volatile boolean QUANTUM_ARMED = false;

    // Unsafe-доступ к приватному полю collector'а (тот же контракт, что
    // InsideBlockOps.col — дубликат паттерна: мост самодостаточен, fail-closed
    // ARMED=false, если поле отсутствует/Unsafe недоступен).
    static final sun.misc.Unsafe Q_UNSAFE;
    static final long Q_COL_OFFSET;
    static {
        sun.misc.Unsafe u = null;
        long off = 0L;
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            u = (sun.misc.Unsafe) uf.get(null);
            java.lang.reflect.Field cf = Entity.class.getDeclaredField("insideEffectCollector");
            off = u.objectFieldOffset(cf);
        } catch (Throwable t) {
            u = null;
        }
        Q_UNSAFE = u;
        Q_COL_OFFSET = off;
    }

    private static InsideBlockEffectApplier.StepBasedCollector qcol(Entity e) {
        if (Q_UNSAFE == null) {
            return null;
        }
        Object o = Q_UNSAFE.getObject(e, Q_COL_OFFSET);
        return o instanceof InsideBlockEffectApplier.StepBasedCollector c ? c : null;
    }

    /**
     * ЧИСТАЯ машина классификатора (одна реализация для hot-пути и selfTest).
     * st = {phase, dwell, hold, pending}; s = |Δ|² тика; zeroDelta = бит-точный
     * ноль (точный ноль — владение static-класса, дизъюнкт).
     *
     * @return QD_VANILLA (полный ванильный re-check) | QD_SERVE (мост уже
     *         переиграл дрейф-сегмент — ванильное тело пропустить).
     */
    static int stepClassifier(int[] st, double s, boolean zeroDelta) {
        int phase = st[0];
        if (phase == QP_DISARMED) {
            return QD_VANILLA;
        }
        if (st[2] > 0) { // гистерезис: обязательный ванильный тик после скачка
            st[2]--;
            st[1] = 0;
            st[0] = QP_VANILLA;
            st[3] = 0;
            return QD_VANILLA;
        }
        if (zeroDelta) { // static-класс (дизъюнкт) — не наш, ваниль
            st[0] = QP_VANILLA;
            st[1] = 0;
            st[3] = 0;
            return QD_VANILLA;
        }
        if (!(s < Q_EPSILON_SQR && s != 0.0d)) { // скачок ≥ε: деклассификация + Schmitt-hold
            st[0] = QP_VANILLA;
            st[1] = 0;
            st[3] = 0;
            st[2] = 1;
            return QD_VANILLA;
        }
        // под-ε микродрейф
        if (phase == QP_REST) {
            st[3] = Math.min(st[3] + 1, Q_BACKLOG_CAP + 1);
            if (st[3] > Q_BACKLOG_CAP) { // дрен-фолт — one-shot disarm (fail-closed)
                st[0] = QP_DISARMED;
                st[3] = 0;
                st[1] = 0;
                return QD_VANILLA;
            }
            st[3] -= Math.min(st[3], Q_MAXSTEPS); // дренированные сегменты переигрываются sweep'ом
            return QD_SERVE;
        }
        st[1]++;
        if (st[1] >= Q_K_TICKS) { // промоция: этот тик переигрывает свой сегмент (steps=1)
            st[0] = QP_REST;
            st[3] = 0;
            return QD_SERVE;
        }
        st[0] = QP_DWELL; // DWELL: ваниль — источник истины
        st[3] = 0;
        return QD_VANILLA;
    }

    /**
     * Ретаргет-точка метода-входа. BATCH_ARMED=false ⇒
     * e.isAffectedByBlocks() — ванильное тело как обычно. Armed-ветка:
     * квант-REST — пропуск re-check (сегмент переигран мостом), иначе ваниль.
     */
    public static boolean batchGate(Entity e) {
        if (!BATCH_ARMED) {
            return e.isAffectedByBlocks();
        }
        if (QUANTUM_ARMED && quantumServe(e)) {
            return false; // re-check пропущен — эффекты уже в collector'е
        }
        return e.isAffectedByBlocks();
    }

    /**
     * P34 квант-REST серв. true ⇒ тик обслужен (свежий swept-обход сегмента
     * xo→current в visit-порядке ванили), ванильное тело можно пропустить.
     * Никогда не бросает семантики наружу: любой сомнительный случай —
     * false (полное ванильное тело).
     */
    static boolean quantumServe(Entity e) {
        if (!e.isAlive() || !e.isAffectedByBlocks()) {
            return false; // ванильный гейт и так закрыт — состояние не трогаем
        }
        InsideBlockEffectApplier.StepBasedCollector col = qcol(e);
        if (col == null) {
            return false; // неожиданный collector — ваниль (fail-closed)
        }
        long eid = e.getId();
        int slot = (int) (eid & (QNSLOTS - 1));
        if (Q_EID[slot] != eid) {
            if (Q_EID[slot] != 0) {
                return false; // слот чужой — НЕ трогаем (ping-pong харденинг S7-136), ваниль
            }
            Q_EID[slot] = eid; // клейм пустого слота
            Q_PHASE[slot] = QP_VANILLA;
            Q_DWELL[slot] = 0;
            Q_HOLD[slot] = 0;
            Q_PENDING[slot] = 0;
        }
        // from = позиция начала тика (javap-контракт: from==xo при движении),
        // Δ = текущая − from — тот же критерий, что у static-детектора гейта.
        double fx = e.xo, fy = e.yo, fz = e.zo;
        double px = e.getX(), py = e.getY(), pz = e.getZ();
        double dx = px - fx, dy = py - fy, dz = pz - fz;
        double s = dx * dx + dy * dy + dz * dz;
        boolean zeroDelta = dx == 0.0d && dy == 0.0d && dz == 0.0d;
        int[] st = TL_QSTATE.get();
        st[0] = Q_PHASE[slot];
        st[1] = Q_DWELL[slot];
        st[2] = Q_HOLD[slot];
        st[3] = Q_PENDING[slot];
        int decision = stepClassifier(st, s, zeroDelta);
        Q_PHASE[slot] = st[0];
        Q_DWELL[slot] = st[1];
        Q_HOLD[slot] = st[2];
        Q_PENDING[slot] = st[3];
        if (decision != QD_SERVE) {
            return false;
        }
        sweepServe(e, e.level(), col, fx, fy, fz, px, py, pz);
        return true;
    }

    /**
     * Свежий swept-обход дрейф-сегмента (from→to, бокс в to, deflate) с
     * немедленным применением эффектов в visit-порядке ванили (зеркало
     * InsideBlockOps.Recorder: isAlive-гейт, budget 16, hitShape/inFluid,
     * intersected = hasMoved || box.intersects). Живые состояния каждый visit —
     * секции-мутанты пере-чекаются обязателно (dirty-list семантика).
     */
    static void sweepServe(Entity e, Level level, InsideBlockEffectApplier.StepBasedCollector col,
                           double fx, double fy, double fz, double px, double py, double pz) {
        Vec3 from = new Vec3(fx, fy, fz);
        Vec3 to = new Vec3(px, py, pz);
        AABB box = e.makeBoundingBox(to).deflate(DEFLATE);
        BlockGetter.forEachBlockIntersectedBetween(from, to, box,
                new QuantumVisitor(e, level, col, from, to, box));
    }

    /** Visit-рекордер квант-сегмента (зеркало контракта визитора 0-16, 80-315). */
    private static final class QuantumVisitor implements BlockGetter.BlockStepVisitor {
        final Entity e;
        final Level level;
        final InsideBlockEffectApplier.StepBasedCollector col;
        final Vec3 from;
        final Vec3 to;
        final AABB box;

        QuantumVisitor(Entity e, Level level, InsideBlockEffectApplier.StepBasedCollector col,
                       Vec3 from, Vec3 to, AABB box) {
            this.e = e;
            this.level = level;
            this.col = col;
            this.from = from;
            this.to = to;
            this.box = box;
        }

        @Override
        public boolean visit(BlockPos bp, int step) {
            if (step >= Q_MAXSTEPS) {
                return false; // зеркало бюджета визитора (9-16)
            }
            if (!e.isAlive()) {
                return false; // зеркало гейта визитора (0-8)
            }
            BlockState st = level.getBlockState(bp);
            if (st.isAir()) {
                return true; // air — без эффектов (контракт debug-ветки)
            }
            // hitShape (контракт визитора 80-121): swept-форма от from к to
            var shape = st.getEntityInsideCollisionShape(level, bp, e);
            boolean hs = shape == Shapes.block()
                    || e.collidedWithShapeMovingFrom(from, to, shape.move(new Vec3(bp)).toAabbs());
            // inFluid (контракт визитора 123-135 — вычисляется всегда)
            boolean infl = e.collidedWithFluid(st.getFluidState(), bp, from, to);
            if (!hs && !infl) {
                return true; // не-effectful (контракт 140-147)
            }
            // intersected = hasMoved || box.intersects; hasMoved — ванильный
            // moved-порог от РЕАЛЬНОГО from/to (для микродрейфа — false)
            boolean intersected = from.distanceToSqr(to) > Q_MOVED_SQR || box.intersects(bp);
            // применяем немедленно — порядок = visit-порядок (контракт 192-315)
            if (hs) {
                col.advanceStep(step, bp);
                st.entityInside(level, bp, e, col, intersected);
                e.onInsideBlock(st);
            }
            if (infl) {
                col.advanceStep(step, bp);
                st.getFluidState().entityInside(level, bp, e, col);
            }
            return true;
        }
    }

    /**
     * In-process selfTest классификатора (вызывает rust ДО noteQuantumArmed;
     * 0 = фейл ⇒ QUANTUM не армится — fail-closed). Инварианты зеркалят
     * 8 cargo-тестов src/inside_quantum_gate.rs.
     */
    static int quantumSelfTest() {
        int fails = 0;
        // band edges: 0 < |Δ| < ε принят, ноль/граница/ваниль-полоса — нет
        {
            int[] st = fresh();
            if (stepClassifier(st, 1.0e-5 * 1.0e-5, false) != QD_VANILLA) fails++; // dwell 1
            if (stepClassifier(fresh(), 0.0d, true) != QD_VANILLA) fails++;        // static
            if (stepClassifier(fresh(), Q_EPSILON_SQR, false) != QD_VANILLA) fails++; // граница
            if (stepClassifier(fresh(), 0.81d, false) != QD_VANILLA) fails++;      // ваниль-полоса
        }
        // dwell K-1 остаётся ванилью; промоция на K-м переигрывает свой сегмент
        {
            int[] st = fresh();
            boolean promo = false;
            for (int i = 0; i < Q_K_TICKS; i++) {
                promo = stepClassifier(st, 2.5e-9, false) == QD_SERVE;
            }
            if (!promo || st[0] != QP_REST || st[3] != 0) fails++;
        }
        // деклассификация + hold: скачок ⇒ ваниль + 1 hold-тик
        {
            int[] st = fresh();
            for (int i = 0; i < Q_K_TICKS; i++) stepClassifier(st, 2.5e-9, false);
            if (stepClassifier(st, 0.25d, false) != QD_VANILLA) fails++;
            if (st[2] != 1) fails++;
            if (stepClassifier(st, 2.5e-9, false) != QD_VANILLA) fails++; // hold-тик
            if (st[2] != 0 || st[1] != 0 || st[0] != QP_VANILLA) fails++;
        }
        // steady REST: дренирует ровно 1 сегмент за тик
        {
            int[] st = fresh();
            for (int i = 0; i < Q_K_TICKS + 2; i++) stepClassifier(st, 2.5e-9, false);
            if (st[0] != QP_REST || st[3] != 0) fails++;
        }
        // переполнение бэклога: one-shot disarm, вечная ваниль
        {
            int[] st = fresh();
            st[0] = QP_REST;
            st[3] = Q_BACKLOG_CAP + 1;
            if (stepClassifier(st, 2.5e-9, false) != QD_VANILLA) fails++;
            if (st[0] != QP_DISARMED) fails++;
            if (stepClassifier(st, 2.5e-9, false) != QD_VANILLA) fails++;
        }
        // констант-контракт с rust-моделью
        if (Q_K_TICKS != 20 || Q_MAXSTEPS != 16 || Q_BACKLOG_CAP != 36) fails++;
        if (Double.doubleToRawLongBits(Q_EPSILON_SQR) != Double.doubleToRawLongBits(1.0e-4 * 1.0e-4)) fails++;
        return fails == 0 ? 1 : 0;
    }

    private static int[] fresh() {
        return new int[]{QP_VANILLA, 0, 0, 0};
    }

    /**
     * Scaff-сборка батча одного тика (контракт вызовов для полного v1).
     * Возвращает число сущностей в батче; ошибки структуры ⇒ 0 (ваниль).
     */
    static int collectBatch(java.util.List<Entity> candidates, Level level) {
        int n = Math.min(candidates.size(), MAXBATCH);
        long[] eids = TL_EIDS.get();
        double[] xyz = TL_XYZ.get();
        double[] bb = TL_BB.get();
        int[] keys = TL_KEYS.get();
        int[] nsec = TL_NSEC.get();
        int[] dirty = TL_DIRTY.get();
        for (int i = 0; i < n; i++) {
            Entity e = candidates.get(i);
            eids[i] = e.getId();
            xyz[i * 3] = e.getX();
            xyz[i * 3 + 1] = e.getY();
            xyz[i * 3 + 2] = e.getZ();
            AABB box = e.makeBoundingBox(new Vec3(e.getX(), e.getY(), e.getZ())).deflate(DEFLATE);
            bb[i * 6] = box.minX;
            bb[i * 6 + 1] = box.minY;
            bb[i * 6 + 2] = box.minZ;
            bb[i * 6 + 3] = box.maxX;
            bb[i * 6 + 4] = box.maxY;
            bb[i * 6 + 5] = box.maxZ;
            nsec[i] = sectionKeys(box, keys, i * MAXSEC);
            dirty[i] = 0; // dirty-list секций-мутантов OR-ится при сборке батча (v1)
        }
        return n;
    }

    /** section-ключи 16³-секций, пересекаемых боксом (упаковка x10|z10|y12, scaffold). */
    private static int sectionKeys(AABB box, int[] keys, int base) {
        int n = 0;
        int x0 = (int) Math.floor(box.minX / 16.0), x1 = (int) Math.floor(box.maxX / 16.0);
        int y0 = (int) Math.floor(box.minY / 16.0), y1 = (int) Math.floor(box.maxY / 16.0);
        int z0 = (int) Math.floor(box.minZ / 16.0), z1 = (int) Math.floor(box.maxZ / 16.0);
        for (int sx = x0; sx <= x1; sx++) {
            for (int sy = y0; sy <= y1; sy++) {
                for (int sz = z0; sz <= z1; sz++) {
                    if (n >= MAXSEC || sy < 0 || sy > 0x3FF) {
                        return Math.min(n, MAXSEC); // переполнение ⇒ caller даёт all-ones
                    }
                    keys[base + n++] = ((sx & 0x3FF) << 20) | ((sz & 0x3FF) << 10) | (sy & 0x3FF);
                }
            }
        }
        return n;
    }

    /**
     * Натив (RegisterNatives rust-стороной, src/inside_batch.rs): ОДИН переход
     * на батч. n сущностей; out[i] = битмаска секций-кандидатов (superset|dirty).
     * rc &lt; 0 ⇒ ERR ⇒ all-ones хвост (ваниль). Объявлен здесь, резолвится
     * только в armed-ветке полного v1 (NCDFE-канон).
     */
    private static native int insideBatchMask(
            int n, int maxsec,
            long[] eids, double[] xyz, double[] bb,
            int[] secKeys, int[] nsec, int[] dirty, int[] out);

    /** Arm-хук моста (вызывает rust после define+RegisterNatives+selfTest). */
    public static void noteBatchArmed() {
        BATCH_ARMED = true;
    }

    /** Arm-хук квант-сайдкара (rust ТОЛЬКО после define+natives+quantumSelfTest()==1). */
    public static void noteQuantumArmed() {
        QUANTUM_ARMED = true;
    }
}
