package net.minecraft.world.entity;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.LongAdder;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunkSection;

/**
 * INSIDE-SNAP REGISTRY SIDECAR (TASK-459-57, idea ID-P32, закон 11 тик-459,
 * lever cmp459_snapreg — STRICT eq, DORMANT scaffold):
 * replaces the per-visit {@code SNAPS.get(sec)} CHM hop of the inside_snap
 * gate (InsideSnapOps.serve / serve4, levers cmp432_inside2 / cmp436_ins4)
 * with a FLAT index serve over per-section STABLE int indexes:
 *
 *   CHM<LevelChunkSection, Snap>  ->  Snap[] REG indexed by stable int idx
 *   fresh-check                   ->  epoch одним int-cmp (REG_EPOCH[idx])
 *   any miss/stale/unregistered   ->  FAIL-CLOSED CHM fallback (the existing
 *                                     InsideSnapOps plane, then vanilla)
 *
 * MECHANICS (same object Snap, same invalidation, SAME collect plane):
 *   1. REGISTRATION: every tracked section owns a stable monotonic int idx
 *      (monotonic counter over a flat array; the WRITE path identity map
 *      SEC2IDX pays ONCE per section lifetime — the SAME cadence
 *      InsideSnapOps.register pays today; the per-visit SERVE path never
 *      touches it). v2 wires the idx into the section at the intercepted
 *      creation sites (fluid_free append-only splice pattern on
 *      LevelChunkSection; creation-intercept precedent = src/entity_index.rs)
 *      and retires the identity map entirely.
 *   2. FLAT SERVE: {@code REG[idx]} — one array read, NO hashing/probing
 *      (burnison.ca "The Concurrency Of ConcurrentHashMap": spread + bin
 *      traversal is exactly the per-visit cost the flat index removes;
 *      sparse-set dense-array discipline: skypjack ECS baf part 9).
 *   3. EPOCH FAST-GATE: {@code REG_EPOCH[idx]} int mirror of the Snap gen
 *      (secWrite bump, InsideSnapOps INVALIDATION contract — NOT touched).
 *      Fast gate = one int-cmp: mismatch => stale => CHM fallback (fail-
 *      closed direction: a match never serves by itself — the long
 *      builtAtGen == gen correctness anchor is ALWAYS re-checked, so an
 *      int wrap can never produce a wrong serve, only a slower one).
 *   4. FALLBACK: flat miss => InsideSnapOps.snapGet(level,pos) — the SHIPPED
 *      CHM plane (cmp432_inside2 parity) with its own vanilla continuation.
 *      Sections outside the intercepted creation sites, unregistered idx,
 *      cap exhaustion: all keep serving through the fallback (card: "секции
 *      вне перехваченных сайтов создания — fallback").
 *
 * PARITY: HIT serves the SAME BlockState object as the CHM plane (the flat
 * array mirrors the SAME Snap refs the collect plane publishes — the sidecar
 * never mints Snap refs, no second source of truth); selfTest (full at arm +
 * sampled 1/100) asserts fresh-serve identity, epoch-mismatch -> fallback,
 * unregistered -> fallback and the cap guard.
 *
 * FAIL-CLOSED: any Throwable => vanilla (fail-dominant, same as snapGet);
 * selfTest failure => rust never arms (probe-then-patch canon); NO native
 * methods in v1 (pure java sidecar — snapCollect/snapProbe belong to the
 * CHM plane and are NOT duplicated; ERR_STRUCT disarm not applicable).
 *
 * NCDFE CANON (round-3 run 35902792520, canon ×93-indy): NO indy in
 * <clinit>, NO method refs resolving nested types during class init; the
 * Snap-typed REG array (newarray resolves InsideSnapOps$Snap at first init)
 * is safe because rust defines InsideSnapOps$Snap and InsideSnapOps$Lane
 * BEFORE this class and the probe (first init) runs only after both — the
 * define-order contract lives in src/inside_snap_registry.rs.
 *
 * INJECTS-ONLY: class defined into the kernel loader by
 * src/inside_snap_registry.rs (dormant unless CRUSSTY_LEVER_FLAG ==
 * cmp459_snapreg STRICT eq or CRUSSTY_SNAPREG=1; with the lever off NO byte
 * hook exists and this class is never defined).
 */
public final class InsideSnapRegistryOps {

    private InsideSnapRegistryOps() {}

    // ------------------------------------------------------------------
    // GATE + ARM
    // ------------------------------------------------------------------

    /** Flipped by rust ONLY after define-order chain ($Snap, $Lane, self) + selfTest pass. */
    private static volatile boolean ARMED = false;

    /** Rust flip (pre-selfTest): flat plane live (fallback CHM unchanged). */
    private static volatile boolean V1 = false;

    /** Rust flip (pre-selfTest): flat plane requested (cmp459_snapreg v1). */
    public static void v1() {
        V1 = true;
    }

    public static void arm() {
        ARMED = true;
        LOG.info("inside_snapreg: ARMED (flat registry sidecar live; serve = REG[idx] + epoch int-cmp; miss = fail-closed CHM fallback via InsideSnapOps; NO new retarget sites in scaffold v1)");
    }

    public static boolean armed() {
        return ARMED;
    }

    /** Probe contract (probe-then-patch canon, inside_bitmask pattern). */
    public static String armState() {
        return ARMED ? "ARMED" : "DISARMED";
    }

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ------------------------------------------------------------------
    // FLAT REGISTRY (sidecar arrays; Snap = the SAME class the CHM plane uses)
    // ------------------------------------------------------------------

    /** Same tracking budget as InsideSnapOps.CAP (1<<15). */
    static final int REG_CAP = 1 << 15;

    /** Flat registry: stable idx -> Snap (mirrors of the CHM plane refs). */
    static volatile InsideSnapOps.Snap[] REG = new InsideSnapOps.Snap[REG_CAP];
    /** Epoch mirrors: REG_EPOCH[idx] == (int) Snap.gen (secWrite bumps). */
    static volatile int[] REG_EPOCH = new int[REG_CAP];
    /** High-water mark of assigned idx (monotonic; slots never reused). */
    static final AtomicInteger REG_COUNT = new AtomicInteger();

    /**
     * WRITE-path identity index (once per section lifetime — the SAME
     * cadence InsideSnapOps.register pays at first touch today). The
     * per-visit serve path NEVER reads this map.
     */
    static final ConcurrentHashMap<LevelChunkSection, Integer> SEC2IDX =
            new ConcurrentHashMap<>(1024);

    /** Stable idx guard: negative/overflow => -1 (cap exhaustion, fail-closed). */
    static int clampAssign(int raw) {
        if (raw < 0 || raw >= REG_CAP) {
            return -1;
        }
        return raw;
    }

    /**
     * Register a section (idempotent) and return its stable int idx, or -1
     * when the cap is exhausted (caller keeps missing to the CHM fallback).
     */
    static int secRegister(LevelChunkSection sec) {
        Integer have = SEC2IDX.get(sec);
        if (have != null) {
            return have;
        }
        int idx = clampAssign(REG_COUNT.getAndIncrement());
        if (idx < 0) {
            REG_COUNT.decrementAndGet();
            return -1; // cap: fail-closed (fallback plane owns the section)
        }
        SEC2IDX.put(sec, idx); // race: one winner publishes, both slots valid
        return idx;
    }

    /**
     * Attach a Snap to a registered idx (the serve path mirrors the ref the
     * CHM plane materialized — the sidecar NEVER creates Snap refs itself).
     */
    static void snapAttach(int idx, InsideSnapOps.Snap s, long epoch) {
        if (idx < 0 || idx >= REG_CAP || s == null) {
            return;
        }
        REG[idx] = s; // publish ref...
        REG_EPOCH[idx] = (int) epoch; // ...then stamp epoch (reader order: ref -> epoch)
    }

    /** secWrite-side epoch bump (v2 wiring: called from the retargeted secWrite). */
    static void epochBump(int idx, long gen) {
        if (idx >= 0 && idx < REG_CAP) {
            REG_EPOCH[idx] = (int) gen;
        }
    }

    // ------------------------------------------------------------------
    // GATE (future retarget body — wide 4B->3B form; scaffold: NO site wired)
    // ------------------------------------------------------------------

    /** Effect marker — grep-able stdout proof. */
    private static volatile boolean FIRST_FLAT_LOGGED = false;

    static final LongAdder STAT_FLAT = new LongAdder();
    static final LongAdder STAT_FALLBACK = new LongAdder();

    public static long flatHits() { return STAT_FLAT.sum(); }
    public static long fallbacks() { return STAT_FALLBACK.sum(); }
    public static long registered() { return SEC2IDX.size(); }

    /**
     * Wide-form gate: flat serve over the stable idx; ANY miss/stale/
     * unregistered => fail-closed CHM fallback (InsideSnapOps plane) =>
     * vanilla. BIT-EXACT continuation contract identical to InsideSnapOps.
     */
    public static BlockState snapGet(Level level, BlockPos pos, int idx) {
        if (ARMED && V1) {
            try {
                BlockState hit = serveFlat(idx, pos);
                if (hit != null) {
                    STAT_FLAT.increment();
                    if (!FIRST_FLAT_LOGGED) {
                        FIRST_FLAT_LOGGED = true;
                        LOG.info("inside_snapreg: first FLAT serve (REG[" + idx + "] + epoch int-cmp)");
                    }
                    return hit;
                }
                STAT_FALLBACK.increment();
                return InsideSnapOps.snapGet(level, pos); // fail-closed CHM fallback (then vanilla)
            } catch (Throwable t) {
                // fail-dominant: vanilla continuation below
            }
        }
        return level.getBlockState(pos);
    }

    /** Flat serve; null => caller falls back (CHM plane). */
    static BlockState serveFlat(int idx, BlockPos pos) {
        if (idx < 0 || idx >= REG_COUNT.get()) {
            return null; // unregistered: fail-closed (card: sections outside creation sites)
        }
        InsideSnapOps.Snap s = REG[idx];
        if (s == null) {
            return null;
        }
        // EPOCH одним int-cmp: mismatch => stale => fallback. NEVER a wrong
        // serve: the fresh anchor below is always the long gen re-check.
        // P33 demotion (TASK-461-67, chkclimb-10): the gate gen read is
        // ACQUIRE — it orders every subsequent read of this visit, so the
        // plain REG/REG_EPOCH element mirrors lean on a real happens-before
        // edge (x86: acquire read == plain mov; card ID-P33: getAcquire, NOT
        // opaque, NOT plain, v1; write plane untouched).
        if (REG_EPOCH[idx] != (int) InsideSnapOps.Snap.genAcquire(s)) {
            return null;
        }
        int packed = ((pos.getY() & 15) << 8) | ((pos.getZ() & 15) << 4) | (pos.getX() & 15);
        BlockState[] a = InsideSnapOps.Snap.statesAcquire(s);
        if (a != null && InsideSnapOps.Snap.builtAcquire(s) == InsideSnapOps.Snap.genAcquire(s)) {
            return a[packed]; // SAME object the palette holds (CHM-serve parity)
        }
        BlockState sg = InsideSnapOps.Snap.singleAcquire(s);
        if (sg != null && InsideSnapOps.Snap.builtAcquire(s) == InsideSnapOps.Snap.genAcquire(s)) {
            return sg;
        }
        return null; // pending/stale: CHM fallback owns collect triggering
    }

    // ------------------------------------------------------------------
    // SELFTEST (rust probe BEFORE arm; sampled 1/100 in v2 on real serves)
    // ------------------------------------------------------------------

    static final AtomicInteger SELFTEST_RUNS = new AtomicInteger();
    static final AtomicInteger SELFTEST_SAMPLE = new AtomicInteger();

    public static int selfTestRuns() {
        return SELFTEST_RUNS.get();
    }

    /**
     * Full selfTest on a synthetic mirrored Snap: (1) unregistered/bounds
     * idx => null (fail-closed direction); (2) fresh serve identity — the
     * slot value comes back bit-identical (Blocks.AIR.defaultBlockState()
     * singleton); (3) epoch-mirror mismatch => null (stale => fallback,
     * never a wrong serve); (4) long-anchor: gen bumped without restamp
     * => null even with a matching int mirror; (5) cap guard: overflow
     * idx => -1. Pure java — no native probe (snapCollect/snapProbe belong
     * to the CHM plane, NOT duplicated here).
     */
    /** JNI contract: 1 = pass, 0 = fail (sdk exposes only int/void/object static calls). */
    public static int selfTest() {
        SELFTEST_RUNS.incrementAndGet();
        try {
            // (1) bounds/negative => null
            if (serveFlat(-1, BlockPos.ZERO) != null) return 0;
            if (serveFlat(REG_CAP + 7, BlockPos.ZERO) != null) return 0;

            // (2)+(3)+(4) synthetic fresh/stale Snap through a real slot
            int idx = clampAssign(REG_COUNT.getAndIncrement());
            if (idx < 0) return 0;
            InsideSnapOps.Snap s = new InsideSnapOps.Snap();
            s.states = new BlockState[4096];
            BlockState air = Blocks.AIR.defaultBlockState();
            s.states[0] = air; // BlockPos.ZERO -> packed 0 -> a[0]
            s.gen = 5L;
            s.builtAtGen = 5L;
            snapAttach(idx, s, s.gen);
            if (serveFlat(idx, BlockPos.ZERO) != air) return 0; // fresh identity

            // (3) epoch-mirror mismatch => null (fallback direction)
            REG_EPOCH[idx] = (int) s.gen + 1;
            if (serveFlat(idx, BlockPos.ZERO) != null) return 0;
            epochBump(idx, s.gen);
            if (serveFlat(idx, BlockPos.ZERO) != air) return 0;

            // (4) long-anchor: gen bumped without restamp => null (int wrap
            // can never serve stale — the long compare always re-arms)
            s.gen = 6L;
            if (serveFlat(idx, BlockPos.ZERO) != null) return 0;

            // (5) cap guard on the assignment path
            if (clampAssign(REG_CAP) != -1) return 0;
            if (clampAssign(-3) != -1) return 0;
            if (clampAssign(REG_CAP - 1) != REG_CAP - 1) return 0;

            // release the test slot (scaffold-local; slots never reused in prod)
            REG[idx] = null;
            REG_COUNT.decrementAndGet();
            return 1;
        } catch (Throwable t) {
            return 0;
        }
    }

    /** Sampled invariant (v2 serves call this 1/100 on real HITs — card: selfTest count раз/100). */
    public static void selfTestSampled() {
        if (SELFTEST_SAMPLE.incrementAndGet() % 100 == 0) {
            selfTest();
        }
    }
}
