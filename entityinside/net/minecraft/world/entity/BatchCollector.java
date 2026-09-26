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
 * CONSTRUCTION PROTOCOL (S7-162, supersedes the retired SWAP PROTOCOL):
 * BatchCollector instances are created by the RETARGETED CTOR — the single
 * `new StepBasedCollector; dup; invokespecial <init>` site in
 * Entity.<init>(EntityType, Level) is rewritten to BatchCollector by the
 * entity_compose compose chain (classfile::patch_entity_collector_ctor;
 * RngOps precedent class). Persistent by construction: the field is
 * written once via the vanilla putfield, before `this` escapes, so no
 * JIT constant-folding can resurrect the old instance.
 *
 * RETIRED (S7-160/161 evidence, run 35391679176): the per-tick
 * BatchCollector.ensure swap from RegionTickOps.tickBucket. All 801
 * BatchCollector.<init> CPU samples in that run came from ensure
 * re-constructing for the same pre-arm entities every tick (the Unsafe
 * swap into the final field never stuck), and the per-entity gate itself
 * burned 737 samples — the whole loop is gone; the tickBucket hot path
 * is vanilla-identical again.
 *
 * TELEMETRY (S7-162): INSTANCES counts every construction (incremented
 * in the ctor); RegionTickOps prints it every 600 forEach invocations
 * ("batch_collector: telemetry tick=N instances=M workers=W") — it
 * answers whether ctors come from live-scene spawn flow or from a
 * hidden per-tick loop.
 *
 * P48-V2 (round-465, LEDGER-45 GO-candidate) — two zero-risk hot-path
 * micro-skips on the SAME blob, NO new lever flag: the class is only
 * defined when the carrier arm path (batch_collector=1 under the
 * composite cmp456_chunkmono_p31snap, STRICT-OR) defines it, so the
 * skis ride the existing arm; no separate toggle, no env guard (an
 * off-by-default guard would placebo the branch-level A/B leg; the
 * branch IS the toggle):
 *   (a) flushStep empty-skip — `if (!dirty) return;` before the ORDER
 *       walk. Vanilla-parity proof (javap -c StepBasedCollector,
 *       patched-kernel 1.21.10): flushStep = 136B/59 instr walking
 *       APPLY_ORDER x 3 maps; on an all-empty state every operation is
 *       observably inert — makeEnumMap-prepopulated before/after lists:
 *       addAll(empty) returns false WITHOUT growth (ArrayList.addAll
 *       no-ops on len 0, only transient Object[0] garbage), clear() on
 *       empty removes nothing (removeRange(0,0) no-op, modCount
 *       untouched), effectsInStep.remove(absent) -> null -> no add;
 *       finalEffects and all three maps end bit-identical. The early-out
 *       is that walk's fixed point, minus the garbage.
 *   (b) advanceStep short-circuit — on a step transition, skip the
 *       flushStep() INVOKE (not the bookkeeping) when nothing is
 *       pending. lastStep=step stays UNCONDITIONAL (bit-identical
 *       vanilla advanceStep = 23B/9 instr: putfield currentBlockPos;
 *       lastStep!=step -> putfield lastStep; invokevirtual flushStep):
 *       dropping the lastStep write would let a same-step duplicate
 *       advanceStep flush mid-step and break the EnumMap.put last-wins
 *       contract (duplicate effect entries in the queue). With dirty
 *       tracked, `if (dirty) flushStep()` == unconditional flushStep()
 *       on empty, exactly.
 *   dirty invariant: set by the three record entry points (apply /
 *   runBefore / runAfter — the ONLY writers of stepHas/before/after),
 *   cleared by flushStep (its walk consumes ALL step state). Therefore
 *   dirty==false <=> stepHas all false ^ before/after all empty, and
 *   the flag can never desynchronize from the observable step state.
 *   The op QUEUE (nOps) is intentionally NOT part of dirty: it is
 *   playback state, not step state (applyAndClear plays it regardless).
 *   javap delta (this class, measured): flushStep 162B -> 175B (+guard),
 *   advanceStep 26B -> 33B (+guard; putfield/putfield prefix byte-faithful
 *   to the vanilla 23B body), apply 23B -> 28B, runBefore/runAfter
 *   15B -> 20B; applyAndClear 141B, appendConsumer 47B, appendEffect 58B,
 *   grow 101B untouched; vanilla-replica methods stay byte-faithful
 *   in ORDER and grouping. BatchCollectorHarness (semantic diff vs
 *   vanilla flushStep queues, randomized advanceStep/apply/runBefore/
 *   runAfter scenarios) is the behavioral gate.
 */
public final class BatchCollector extends InsideBlockEffectApplier.StepBasedCollector {

    private static final InsideBlockEffectType[] ORDER = InsideBlockEffectType.values();
    private static final int NT = ORDER.length;

    // ---- telemetry (static, no Unsafe, no reflection) ----
    private static final java.util.concurrent.atomic.AtomicLong INSTANCES =
            new java.util.concurrent.atomic.AtomicLong();

    /** S7-162 telemetry: total BatchCollector constructions since class init. */
    public static long instances() {
        return INSTANCES.get();
    }

    // ---- step state (last-wins per type == EnumMap.put contract) ----
    private final boolean[] stepHas = new boolean[NT];
    private final long[] stepPos = new long[NT];
    private long currentPacked; // javap #92 currentBlockPos (packed bits)
    private int lastStep = -1;  // javap #90

    // P48-v2: pending-work flag over stepHas/before/after (see header).
    private boolean dirty;

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
        INSTANCES.incrementAndGet(); // S7-162 telemetry
        for (int i = 0; i < NT; i++) {
            this.before[i] = new ArrayList<>();
            this.after[i] = new ArrayList<>();
        }
    }

    @Override
    public void advanceStep(int step, BlockPos pos) {
        this.currentPacked = pos.asLong();
        if (this.lastStep != step) {
            this.lastStep = step; // unconditional: vanilla 23B body, last-wins contract
            if (this.dirty) {     // P48-v2 (b): skip the invoke on an empty step
                this.flushStep();
            }
        }
    }

    @Override
    public void apply(InsideBlockEffectType type) {
        int o = type.ordinal();
        this.stepPos[o] = this.currentPacked;
        this.stepHas[o] = true;
        this.dirty = true;
    }

    @Override
    public void runBefore(InsideBlockEffectType type, Consumer<Entity> c) {
        this.before[type.ordinal()].add(c);
        this.dirty = true;
    }

    @Override
    public void runAfter(InsideBlockEffectType type, Consumer<Entity> c) {
        this.after[type.ordinal()].add(c);
        this.dirty = true;
    }

    /** Flat replica of the vanilla flushStep (same ORDER, same grouping). */
    private void flushStep() {
        if (!this.dirty) { // P48-v2 (a): vanilla empty walk = bit-identical fixed point
            return;
        }
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
        this.dirty = false; // the walk consumed ALL step state
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
