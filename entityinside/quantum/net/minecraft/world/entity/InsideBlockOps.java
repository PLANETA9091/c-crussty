package net.minecraft.world.entity.quantum;

/**
 * P34 INSIDE-QUANTUM-GATE — SCAFFOLD stub (TASK-459-74, идея ID-P34
 * round-458p-ideas; law-11: scaffold-тик БЕЗ in-process семантики —
 * STRICT-off до оффлайн-оракула 10k-сцен, vanilla bytes в этом тике
 * НЕ трогаются).
 *
 * Развитие inside_cache static-gate (net.minecraft.world.entity.InsideBlockOps,
 * S7-135): статичный гейт обслуживает ТОЛЬКО бит-неподвижные сущности
 * (deltaMovement==0 && позиция бит-равна кэшу). Этот класс добавляет
 * «квант-покой» — класс подвижных-но-медленных:
 *
 *   0 &lt; |Δ| &lt; ε на протяжении K тиков подряд  →  сервинг discovery
 *   из флет-слотов (та же плоская примитивная машинерия InsideBlockOps)
 *   с ИНКРЕМЕНТАЛЬНЫМ replay ванильных effect-вызовов (entityInside /
 *   onInsideBlock / FluidState.entityInside) по накопленному микродрейфу
 *   (anchor → текущая позиция, бит-в-байт контракт static-гейта).
 *
 * JAVAP GROUND TRUTH (canon InsideBlockOps.java, S7-135/TASK-432-B):
 *   moved-порог ванили: distanceToSqr(from,to) &gt; square(0.9999900000002526)
 *     ≈ (0.99999)² — «квант»-полоса ε=1.0e-4 лежит на 4 порядка ниже;
 *   budget визитора 16 (MAXSTEPS) = кап дрена replay-бэклога за тик;
 *   слот = eid &amp; (NSLOTS-1), штамп = eid (0 = пусто), NSLOTS = 2^18
 *     (TASK-432-B: 150000 живых сущностей полностью помещаются в мемо);
 *   deflate-бокс 9.999999747378752E-6 (javap #2462 inner checkInsideBlocks).
 *
 * REPLAY CADENCE (in-tick, exactly-once): во время DWELL ваниль — источник
 * истины (полный discovery, anchor живёт за ней, бэклог пуст — без
 * двойного применения). Тик промоции переигрывает СВОЙ микросегмент
 * (steps=1). Каждый REST-тик добавляет свой сегмент в бэклог и серв
 * дренирует до MAXSTEPS сегментов за тик (старшие первыми) — сегмент
 * каждого тика обрабатывается ровно один раз, в свой тик. Бэклог выше
 * жёсткого капа MAXSTEPS+K_TICKS = дрен-фолт (wiring fault) → one-shot
 * disarm (fail-closed, ваниль навсегда).
 *
 * ORACLE-ГЕЙТ (карточка ID-P34): ε-критерий МЕНЯЕТ ТАЙМИНГ stateful-эффектов
 * (freeze/fire cadence зависит от per-tick последовательности вызовов).
 * Константы ε/K — константы ОФФЛАЙН-ОРАКУЛА; включение класса — только
 * после 10k-сцен (lockstep бит-в-байт); фоллбэк — чистая ваниль. Здесь
 * значения ε=1.0e-4, K=20 — v1-дефолты для оракула, НЕ для арминга.
 *
 * NCDFE-КАНОН: в v1 класс НИКОГДА не define_class'ится в kernel loader
 * (dormant: гейт CRUSSTY_INSIDE_QUANTUM off + оракула нет) — нет блоба,
 * нет include_bytes!, нет ретаргета ⇒ некому поймать NoClassDefFoundError.
 * При будущем арминге — define в раннем arm-хуке (канон inside_cache.rs:
 * kernel loader тих, boot-storm пройден, major-версия ≤ JVM), до первого
 * разрешительного вызова сплетённого сайта.
 *
 * JDK-only: НИКАКИХ импортов ядра — чистая референс-модель классификатора
 * (зеркало src/inside_quantum_gate.rs) + selfTest; kernel-facing JNI /
 * флет-слот wiring — шаг следующей ноги (прецедент P45: javac-rebuild
 * отдельным шагом).
 *
 * SELFTEST-КАНОН: {@link #main(String[])} исполняет инварианты и
 * System.exit(1) на первом нарушении — провал громкий, не «серый».
 */
public final class InsideBlockOps {

    private InsideBlockOps() {}

    // -- Oracle constants (v1 defaults; re-derive before ANY arming) ------
    /** ε: подпороговое смещение за тик, блоки (строгая полоса, ср. Rust). */
    public static final double EPSILON = 1.0e-4;
    /** K: длительность dwell в тиках до продвижения в rest-класс. */
    public static final int K_TICKS = 20;
    /** Кап дрена replay-бэклога за тик = бюджет визитора ванили. */
    public static final int MAXSTEPS = 16;
    /** Жёсткий кап бэклога: выше — дрен-фолт → one-shot disarm. */
    public static final int BACKLOG_CAP = MAXSTEPS + K_TICKS;

    // -- Phases ------------------------------------------------------------
    public static final int PHASE_VANILLA = 0;
    public static final int PHASE_DWELL = 1;
    public static final int PHASE_REST = 2;
    public static final int PHASE_DISARMED = 3;

    /** DORMANT: арминг класса запрещён до оффлайн-оракула (карточка P34). */
    public static final boolean ARMED = false;

    private static final double EPSILON_SQR = EPSILON * EPSILON;

    /** Решение на тик для Entity.checkInsideBlocks. */
    public enum Decision {
        VANILLA, STATIC_GATE, SERVE
    }

    /** Плоское состояние классификатора одной сущности. */
    public static final class QuantumSlot {
        public long eid;
        public int phase = PHASE_VANILLA;
        public int dwell;
        public int vanillaHold;
        public int pending;
        public double anchorX, anchorY, anchorZ;
    }

    /**
     * Полоса «квант-покоя»: 0 &lt; |Δ|² &lt; ε² (бит-строго; точный ноль —
     * владение static-гейта inside_cache, не этого класса).
     */
    public static boolean subEpsilon(double dx, double dy, double dz) {
        double s = dx * dx + dy * dy + dz * dz;
        return s < EPSILON_SQR && s != 0.0d;
    }

    /**
     * Шаг классификатора на один тик (зеркало inside_quantum_gate::tick).
     *
     * @return упакованное решение: lo-половина — ordinal Decision;
     *         для SERVE hi-половина — число дренированных сегментов.
     */
    public static long tick(QuantumSlot s, double dx, double dy, double dz,
                            double px, double py, double pz) {
        if (s.phase == PHASE_DISARMED) {
            return Decision.VANILLA.ordinal();
        }
        if (s.vanillaHold > 0) { // гистерезис: обязательный ванильный тик
            s.vanillaHold--;
            s.dwell = 0;
            s.pending = 0;
            s.phase = PHASE_VANILLA;
            s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
            return Decision.VANILLA.ordinal();
        }
        if (dx == 0.0d && dy == 0.0d && dz == 0.0d) { // static-gate владение
            s.dwell = 0;
            s.pending = 0;
            s.phase = PHASE_VANILLA;
            s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
            return Decision.STATIC_GATE.ordinal();
        }
        if (!subEpsilon(dx, dy, dz)) { // вне полосы: деклассификация
            s.phase = PHASE_VANILLA;
            s.dwell = 0;
            s.pending = 0;
            s.vanillaHold = 1;
            s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
            return Decision.VANILLA.ordinal();
        }
        // Подпороговый микро-шаг.
        s.dwell++;
        if (s.phase == PHASE_REST) {
            // Сегмент этого тика в бэклог; серв дренирует до MAXSTEPS
            // (старшие первыми); штатный steady-state = 1 за тик.
            s.pending++;
            if (s.pending > BACKLOG_CAP) { // дрен-фолт → one-shot disarm
                s.phase = PHASE_DISARMED;
                s.pending = 0;
                s.dwell = 0;
                return Decision.VANILLA.ordinal();
            }
            int steps = Math.min(s.pending, MAXSTEPS);
            s.pending -= steps;
            s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
            return Decision.SERVE.ordinal() | ((long) steps << 32);
        }
        if (s.dwell >= K_TICKS) {
            // Промоция: K ванильных тиков оплачены; СВОЙ сегмент этого тика
            // переигрывается in-tick (steps=1), начинается steady REST.
            s.phase = PHASE_REST;
            s.pending = 0;
            s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
            return Decision.SERVE.ordinal() | (1L << 32);
        }
        // DWELL: ваниль — источник истины, anchor за ней, бэклог пуст.
        s.phase = PHASE_DWELL;
        s.pending = 0;
        s.anchorX = px; s.anchorY = py; s.anchorZ = pz;
        return Decision.VANILLA.ordinal();
    }

    /** Шагов replay из упакованного решения SERVE (hi-половина). */
    public static int serveSteps(long packedDecision) {
        return (int) (packedDecision >>> 32);
    }

    /** ordinal из упакованного решения (lo-половина). */
    public static int decisionOf(long packedDecision) {
        return (int) packedDecision;
    }

    // -- SelfTest -----------------------------------------------------------

    private static void require(boolean ok, String what) {
        if (!ok) {
            System.err.println("[P34-selftest] FAIL: " + what);
            System.exit(1);
        }
    }

    /** Инварианты классификатора; exit(1) на нарушении (громкий провал). */
    public static boolean selfTest() {
        // Полоса: 0 < |Δ| < ε принимается, ноль/граница/ваниль-полоса — нет.
        require(subEpsilon(1.0e-5, 0.0, 0.0), "band: micro accepted");
        require(!subEpsilon(0.0, 0.0, 0.0), "band: exact zero rejected");
        require(!subEpsilon(EPSILON, 0.0, 0.0), "band: boundary rejected");
        require(!subEpsilon(0.9, 0.0, 0.0), "band: vanilla moved-band rejected");

        // Dwell K-1 — без продвижения (ваниль, бэклог пуст); K-й тик —
        // промоция, переигрывает свой сегмент (steps=1).
        QuantumSlot s = new QuantumSlot();
        for (int i = 0; i < K_TICKS - 1; i++) {
            require(decisionOf(tick(s, 5.0e-5, 0, 0, 1.0 + i * 5.0e-5, 2, 3))
                            == Decision.VANILLA.ordinal(),
                    "dwell " + i + ": vanilla");
        }
        require(s.phase == PHASE_DWELL && s.dwell == K_TICKS - 1 && s.pending == 0,
                "pre-promotion state");
        long promoted = tick(s, 5.0e-5, 0, 0, 1.0 + 19.0 * 5.0e-5, 2, 3);
        require(decisionOf(promoted) == Decision.SERVE.ordinal()
                && serveSteps(promoted) == 1, "promotion serves own segment");
        require(s.phase == PHASE_REST && s.pending == 0, "post-promotion state");

        // Гистерезис: один скачок ≥ε → ваниль + hold, dwell с нуля.
        require(decisionOf(tick(s, 0.5, 0, 0, 9.0, 2, 3))
                == Decision.VANILLA.ordinal(), "jump: vanilla");
        require(s.vanillaHold == 1, "jump: hold armed");
        tick(s, 5.0e-5, 0, 0, 9.0, 2, 3);
        require(s.phase == PHASE_VANILLA && s.dwell == 0 && s.vanillaHold == 0,
                "hold consumed, dwell restarted");

        // Точный ноль → static-гейт (классы не пересекаются).
        QuantumSlot t = new QuantumSlot();
        require(decisionOf(tick(t, 0.0, 0.0, 0.0, 1.0, 2, 3))
                == Decision.STATIC_GATE.ordinal(), "zero delta -> static gate");

        // Steady-state REST: один in-tick сегмент за тик.
        QuantumSlot r = new QuantumSlot();
        for (int i = 0; i <= K_TICKS; i++) {
            tick(r, 5.0e-5, 0, 0, 1.0 + i * 5.0e-5, 2, 3);
        }
        long steady = tick(r, 5.0e-5, 0, 0, 1.0 + 20.0 * 5.0e-5, 2, 3);
        require(decisionOf(steady) == Decision.SERVE.ordinal()
                && serveSteps(steady) == 1, "steady-state serves 1 step/tick");
        require(r.anchorX == 1.0 + 20.0 * 5.0e-5, "anchor re-pinned bit-exact");

        // Кап дрена на границе: бэклог ровно MAXSTEPS — дренируется целиком,
        // свежий сегмент остаётся на следующий тик.
        QuantumSlot c = new QuantumSlot();
        c.phase = PHASE_REST;
        c.pending = MAXSTEPS;
        long capped = tick(c, 5.0e-5, 0, 0, 4.0, 2, 3);
        require(decisionOf(capped) == Decision.SERVE.ordinal()
                && serveSteps(capped) == MAXSTEPS && c.pending == 1,
                "drain cap boundary");

        // Жёсткий кап (дрен-фолт) → one-shot disarm, навсегда ваниль.
        QuantumSlot o = new QuantumSlot();
        o.phase = PHASE_REST;
        o.pending = BACKLOG_CAP + 1;
        require(decisionOf(tick(o, 5.0e-5, 0, 0, 4.0, 2, 3))
                == Decision.VANILLA.ordinal(), "hard cap: vanilla");
        require(o.phase == PHASE_DISARMED, "hard cap: disarmed");
        require(decisionOf(tick(o, 5.0e-5, 0, 0, 4.0, 2, 3))
                == Decision.VANILLA.ordinal() && o.phase == PHASE_DISARMED,
                "disarm is one-shot permanent");

        return true;
    }

    public static void main(String[] args) {
        require(!ARMED, "dormant canon: ARMED must stay false in scaffold");
        selfTest();
        System.out.println("[P34-selftest] OK — quantum-rest classifier "
                + "invariants hold (eps=" + EPSILON + ", K=" + K_TICKS
                + ", maxsteps=" + MAXSTEPS + ", backlogCap=" + BACKLOG_CAP
                + ", dormant=" + !ARMED + ")");
    }
}
