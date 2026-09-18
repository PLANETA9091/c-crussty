package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import net.minecraft.core.BlockPos;

/**
 * BATCH-COLLECTOR (S7-160, ARCH-ATTACK lever #8) — zero-map / flat-storage
 * replacement for the vanilla InsideBlockEffectApplier.StepBasedCollector
 * that lives in Entity.insideEffectCollector.
 *
 * RECON-3 (leg #5 35381522360, CUMULATIVE v2): the entity-phase residual
 * (39.1% of the phase = 21% CPU) is dominated by the checkInsideBlocks /
 * inside-effects pipeline. Its collector half burns CPU on EMPTY
 * EnumMap operations: vanilla flushStep() walks APPLY_ORDER x 3 maps
 * (before.get + addAll + clear, effectsInStep.remove, after.get + addAll +
 * clear) on EVERY step transition of EVERY ticking entity — ~60 map ops
 * per entity per tick (~9M map ops/tick at 150k entities; 1654+489 CPU
 * samples = 1.65% of the run in flushStep+advanceStep alone), and every
 * vanilla apply() allocates a RecordedEffect + BlockPos.immutable().
 *
 * ARCHITECTURE (owner menu: O(n)->O(1) flat indexing + zero-alloc
 * batching; NOT a cache — position-independent, works for moving
 * entities, the exact class of lanes that defeated the three cache
 * levers FLUID-FREE/PALETTED-DEMUX/FLUID-DIRTY):
 *  - step slots: boolean[5] + long[5] (packed BlockPos) — last-wins per
 *    type, identical to the vanilla EnumMap.put contract;
 *  - before/after consumer lists: ArrayList[5] built ONCE, reused via
 *    clear() (the vanilla EnumMap does the same through makeEnumMap);
 *  - final op queue: flat parallel arrays (byte kind, byte type,
 *    long packedPos, Consumer) with grow-on-demand and reuse — replaces
 *    List<Consumer> finalEffects + RecordedEffect records;
 *  - playback (applyAndClear) reproduces the vanilla loop BIT-FOR-BIT:
 *    flushStep(); for each op in append order: if (!entity.isAlive())
 *    break; consumer.accept(entity) or ORDER[type].effect().affect(
 *    entity, BlockPos.of(packed)); then clear + lastStep=-1.
 *
 * PARITY CONTRACT (javap-verified against the pristine kernel):
 *  - advanceStep(int,BlockPos): currentPos=pos; if (lastStep!=step)
 *    {lastStep=step; flushStep();} — exact vanilla body. The position is
 *    captured as long bits at advanceStep time; vanilla keeps a reference
 *    and copies via immutable() at apply() time. Inside a single visit
 *    frame the caller passes the same BlockPos to advanceStep and
 *    entityInside (the betweenClosed iterator mutates it only AFTER the
 *    visit returns), so both observe identical bits — documented, zero
 *    observable difference.
 *  - apply(type): EnumMap.put last-wins == stepHas/stepPos overwrite.
 *  - flushStep(): for t in APPLY_ORDER (= values() order, vanilla static
 *    init): before[t] all -> queue; effect[t] if present -> queue; after
 *    [t] all -> queue; per-type lists cleared. The vanilla
 *    RecordedEffect.accept body is `applier.affect(entity, blockPos)`
 *    (javap) — replayed verbatim as ORDER[o].effect().affect(entity,
 *    BlockPos.of(packed)).
 *  - applyAndClear: flushStep() first; isAlive gate breaks the loop;
 *    finalEffects.clear() == nOps=0 (+null-ing consumed refs, retention
 *    hygiene, no semantic effect); lastStep=-1.
 *  - The parent StepBasedCollector constructor still runs (its EnumMaps
 *    become dead weight ~200B/entity, allocated once per entity — the
 *    vanilla allocation; per-TICK cost is zero and the private parent
 *    methods flushStep/recorded are never invoked: every public entry
 *    point is overridden).
 *
 * SWAP PROTOCOL: BatchCollector.ensure(Entity) — called from
 * RegionTickOps.tickBucket BEFORE the entity's vanilla consumer runs
 * (RegionTickOps is the single entry point of every entity tick under
 * region-threads). The swap is a one-time per-entity Unsafe
 * putObjectVolatile of the insideEffectCollector field; the entity is
 * ticked by exactly one worker per tick and the swap happens before any
 * collector use in that tick, so every getfield in
 * applyEffectsFromBlocks observes exactly one instance per episode.
 * Fail-closed: ARMED=false (Unsafe/field resolution failure) -> ensure
 * is a no-op and the vanilla collector stays.
 *
 * ARMED marker: "[crussty-plugin] batch_collector: ARMED first-swap" on
 * the first swap; cumulative swap count via swaps() (absorb probe).
 */
public final class BatchCollector extends InsideBlockEffectApplier.StepBasedCollector {

    private static final InsideBlockEffectType[] ORDER = InsideBlockEffectType.values();
    private static final int NT = ORDER.length;

    // ---- swap machinery (static, fail-closed) ----
    private static final sun.misc.Unsafe UNSAFE;
    private static final long COL_OFFSET;
    private static final boolean ARMED;
    private static final java.util.concurrent.atomic.AtomicLong SWAPS =
            new java.util.concurrent.atomic.AtomicLong();

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
        UNSAFE = u;
        COL_OFFSET = off;
        ARMED = u != null;
    }

    /** One-time per-entity swap; called from RegionTickOps.tickBucket. */
    public static void ensure(Entity e) {
        if (!ARMED) {
            return;
        }
        Object o = UNSAFE.getObject(e, COL_OFFSET);
        if (o instanceof BatchCollector
                || !(o instanceof InsideBlockEffectApplier.StepBasedCollector)) {
            return;
        }
        UNSAFE.putObjectVolatile(e, COL_OFFSET, new BatchCollector());
        long n = SWAPS.incrementAndGet();
        if (n == 1) {
            System.err.println(
                    "[crussty-plugin] batch_collector: ARMED first-swap (eid=" + e.getId() + ")");
        }
    }

    /** Absorb probe: total swaps performed. */
    public static long swaps() {
        return SWAPS.get();
    }

    // ---- step state (last-wins per type == EnumMap.put contract) ----
    private final boolean[] stepHas = new boolean[NT];
    private final long[] stepPos = new long[NT];
    private long currentPacked; // javap #92 currentBlockPos (packed bits)
    private int lastStep = -1;  // javap #90

    // ---- before/after consumer lists (built once, reused via clear()) ----
    @SuppressWarnings("unchecked")
    private final ArrayList<Consumer<Entity>>[] before = new ArrayList[NT];
    @SuppressWarnings("unchecked")
    private final ArrayList<Consumer<Entity>>[] after = new ArrayList[NT];

    // ---- final op queue (flat parallel arrays, grow-on-demand, reused) ----
    private byte[] opKind = new byte[64]; // 0 = consumer, 1 = step effect
    @SuppressWarnings("unchecked")
    private Consumer<Entity>[] opC = new Consumer[64];
    private byte[] opType = new byte[64];
    private long[] opPos = new long[64];
    private int nOps;

    public BatchCollector() {
        for (int i = 0; i < NT; i++) {
            this.before[i] = new ArrayList<>();
            this.after[i] = new ArrayList<>();
        }
    }

    @Override
    public void advanceStep(int step, BlockPos pos) {
        this.currentPacked = pos.asLong();
        if (this.lastStep != step) {
            this.lastStep = step;
            this.flushStep();
        }
    }

    @Override
    public void apply(InsideBlockEffectType type) {
        int o = type.ordinal();
        this.stepPos[o] = this.currentPacked;
        this.stepHas[o] = true;
    }

    @Override
    public void runBefore(InsideBlockEffectType type, Consumer<Entity> c) {
        this.before[type.ordinal()].add(c);
    }

    @Override
    public void runAfter(InsideBlockEffectType type, Consumer<Entity> c) {
        this.after[type.ordinal()].add(c);
    }

    /** Flat replica of the vanilla flushStep (same ORDER, same grouping). */
    private void flushStep() {
        for (int o = 0; o < NT; o++) {
            List<Consumer<Entity>> b = this.before[o];
            if (!b.isEmpty()) {
                for (int i = 0, n = b.size(); i < n; i++) {
                    this.appendConsumer(b.get(i));
                }
                b.clear();
            }
            if (this.stepHas[o]) {
                this.appendEffect(o, this.stepPos[o]);
                this.stepHas[o] = false;
            }
            List<Consumer<Entity>> a = this.after[o];
            if (!a.isEmpty()) {
                for (int i = 0, n = a.size(); i < n; i++) {
                    this.appendConsumer(a.get(i));
                }
                a.clear();
            }
        }
    }

    private void appendConsumer(Consumer<Entity> c) {
        if (this.nOps == this.opKind.length) {
            this.grow();
        }
        this.opKind[this.nOps] = 0;
        this.opC[this.nOps] = c;
        this.nOps++;
    }

    private void appendEffect(int typeOrd, long packed) {
        if (this.nOps == this.opKind.length) {
            this.grow();
        }
        this.opKind[this.nOps] = 1;
        this.opType[this.nOps] = (byte) typeOrd;
        this.opPos[this.nOps] = packed;
        this.nOps++;
    }

    private void grow() {
        int n = this.opKind.length;
        int nn = n * 2;
        byte[] k = new byte[nn];
        System.arraycopy(this.opKind, 0, k, 0, n);
        this.opKind = k;
        @SuppressWarnings("unchecked")
        Consumer<Entity>[] c = new Consumer[nn];
        System.arraycopy(this.opC, 0, c, 0, n);
        this.opC = c;
        byte[] t = new byte[nn];
        System.arraycopy(this.opType, 0, t, 0, n);
        this.opType = t;
        long[] p = new long[nn];
        System.arraycopy(this.opPos, 0, p, 0, n);
        this.opPos = p;
    }

    /** Vanilla contract: flushStep(); playback with isAlive break; clear. */
    @Override
    public void applyAndClear(Entity entity) {
        this.flushStep();
        int n = this.nOps;
        int i = 0;
        for (; i < n; i++) {
            if (!entity.isAlive()) {
                break;
            }
            if (this.opKind[i] == 0) {
                Consumer<Entity> c = this.opC[i];
                this.opC[i] = null;
                c.accept(entity);
            } else {
                InsideBlockEffectType t = ORDER[this.opType[i] & 0xFF];
                t.effect().affect(entity, BlockPos.of(this.opPos[i]));
            }
        }
        for (; i < n; i++) { // retention hygiene for a broken loop
            if (this.opKind[i] == 0) {
                this.opC[i] = null;
            }
        }
        this.nOps = 0;
        this.lastStep = -1;
    }
}
