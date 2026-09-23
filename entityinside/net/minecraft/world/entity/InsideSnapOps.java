package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.patches.fast_palette.FastPaletteData;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.LongAdder;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.LevelHeightAccessor;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.block.state.BlockState;

/**
 * INSIDE-SNAP (TASK-424-B, round-423 vector B — lever cmp424_inside, закон 6 v17):
 * per-section BlockState[4096] snapshots for the inside-blocks discovery lane.
 *
 * PROFILE-B (round-421-b-inside, ARMED carrier): the lane's ADDRESSABLE slice is
 * the volatile per-position state read chain:
 *   PalettedContainer.get 0.487 + SimpleBitStorage.get 0.126 + readPalette 0.099
 *   + LevelChunk.getBlockStateFinal 0.498 (+ part of LevelChunk.getBlockState
 *   0.593 / Level.getBlockState self) ≈ 1.3-1.6% total CPU.
 * The traversal orchestration (betweenCorners/DDA/LongOpenHashSet) is OUT OF SCOPE
 * (ghost ≠ snapshot; forbidden flat_traversal #9 territory); the fluid sub-lane
 * and the collector are other levers' lanes.
 *
 * ARCHITECTURE (law 6: ONE bulk-JNI per collect; per-entity/per-site JNI = design error):
 *   1. SNAPSHOTS: ConcurrentHashMap<LevelChunkSection, Snap>; Snap = { volatile long
 *      gen (invalidation epoch), volatile long builtAtGen, volatile BlockState[]
 *      states (full mode), volatile BlockState single (bpe==0 mode) }.
 *      HIT serve = states[(y&15)<<8|(z&15)<<4|(x&15)] — the SAME object vanilla
 *      readPalette returns (FastPaletteData.moonrise$getPalette()[idx]).
 *   2. INVALIDATION (event-driven): the SINGLE LevelChunkSection.setBlockState
 *      (IIILBlockState)BlockState call site inside LevelChunk.setBlockState is
 *      retargeted to secWrite(...) (FluidPushOps.secWrite pattern): vanilla write
 *      delegate + ref-compare (old != newState) -> bump snap.gen (volatile).
 *      old == newState implies packed words AND palette unchanged => snapshot still
 *      exact => no bump (writes with identical state cost nothing).
 *   3. GATE: the SINGLE Level.getBlockState call site inside
 *      Entity.lambda$checkInsideBlocks$2 is retargeted to snapGet(Level,BlockPos)
 *      (receiver-first 3B->3B). Fresh hit = flat array read; ANY miss/failure falls
 *      through to level.getBlockState(pos) — the vanilla body itself (bit-exact
 *      continuation, no reimplementation).
 *   4. COLLECT (ONE bulk JNI per collect, incremental, steady-state ≈ 0 collects):
 *      pending sections (first-touch registration or secWrite invalidation) are
 *      gathered into flat buffers: meta[k*3]={bpe,wordsOff,wordsLen}, words flat
 *      copy, native snapCollect decodes 4096 palette indices per section
 *      BIT-EXACTLY (kernel SimpleBitStorage MODULO layout: word = i/valuesPerLong,
 *      shift = (i%valuesPerLong)*bits — javap @4789ca7 magic/mulBits; NO straddle
 *      case in this kernel), java scatters BlockState[4096] from the captured
 *      moonrise palette array and publishes with a gen re-verify (seqlock: any
 *      in-flight write bumps gen and the build is discarded for the next collect).
 *   5. MEMORY: CAP sections total (register refuses beyond), FULL_CAP full-mode
 *      (4096-ref) arrays; single-mode snaps are ~100 B. 64 sections per collect
 *      (bounded burst, FluidBitmaskOps BUILDS_PER_TICK discipline).
 *
 * PARITY (median-exact):
 *   - HIT: identical BlockState object (same palette array slot as vanilla
 *     readPalette), same call order effects (the visitor body is untouched — only
 *     the getBlockState call is retargeted).
 *   - MISS: level.getBlockState(pos) — the vanilla method itself.
 *   - Documented delta (unreachable in practice, same class as fluid_dirty): the
 *     vanilla captureTreeGeneration branch (worldgen structure capture) is not
 *     replicated in the fast path — entities do not tick during capture.
 *
 * FAIL-DOMINANT: any Throwable in the gate -> vanilla; native ERR_STRUCT ->
 * permanent ARMED=false disarm (never a wrong serve); collection is best-effort —
 * sections the collector cannot serve just keep missing to vanilla.
 *
 * TASK-432-B DEEPENING (lever cmp432_inside2, STRICT-OR cmp430_inside):
 *   6. SERVE FASTPATH: per-thread, per-TICK warm lanes {level,cx,cz -> chunk,
 *      sections, sec -> snap} — the per-visit chunk-map lookup and the
 *      per-visit SNAPS CHM.get collapse to reference compares for the common
 *      section-local visit bursts (lambda gate AND the inside_cache gate's
 *      verify/replay loops routed here). Tick-stamped: a lane never outlives
 *      the tick that warmed it (no stale-chunk serve across unload/reload).
 *   7. STALE-MISS CONTINUATION: when a snap exists but is stale/pending, the
 *      miss continues EXACTLY like LevelChunk.getBlockStateFinal's non-air
 *      branch from the already-resolved section (sec.states.get(packed)) —
 *      bit-exact vanilla continuation WITHOUT the redundant second chunk-map
 *      lookup the old fallthrough level.getBlockState(pos) paid.
 *   8. GATE FUSION HOOK: arm() notifies InsideBlockOps (inside_cache gate)
 *      so the memo-verify/replay block reads route through this plane too
 *      (one snapshot store, ONE bulk-JNI collect, two consumers).
 *
 * TASK-435-B DEEPENING-4 (lever cmp435_inside3, STRICT-OR cmp432_inside2):
 *   9. SERVE-V3 LEAN FASTPATH (research/gc-recon-2026-09-19/ROUND435_B_RESEARCH.md):
 *      with the plane LIVE the serve() self-time is the NEW top-6 leaf (~1.9%
 *      on the round-434b legs; call volume is driven by the inside_cache
 *      HIT-verify of ~107k static items — same-chunk/same-section bursts).
 *      Per-visit V3 hit path replaces the 8-lane scan + 2-itable
 *      getSectionIndex chain + hasOnlyAir call + per-HIT LongAdder with:
 *      a) L0 DIRECT LANE: lanes[LANES] slot checked first (5 compares);
 *         8-slot round-robin stays as fallback with copy-on-match into L0
 *         (chunk switches cost one copy, bursts cost one compare);
 *      b) INLINE SECTION-INDEX: lane.minSecY cached at claim —
 *         si = (y>>4) - minSecY, javap-equivalent to the kernel chain
 *         (SectionPos.blockToSectionCoord(I) = i>>4 ishr-only;
 *         getSectionIndex(y) = (y>>4) - getMinSectionY()); RUNTIME PARITY
 *         PROBE per claim (3 sampled y against the live chunk) with itable
 *         fallback on any mismatch (fail-open, median-exact);
 *      c) AIR-MASK: per-claim long mask of null/hasOnlyAir sections
 *         (hasOnlyAir = nonEmptyBlockCount==0, plain short read) — the
 *         per-visit call collapses to one bit test (mask only when
 *         secs.length <= 64; conservative direct checks otherwise);
 *      d) SAMPLED PER-THREAD HITS: STAT_HITS LongAdder (striped cells, 4
 *         region workers) on EVERY hit was cache-line contention on the
 *         hottest path — V3 counts per-thread in lanes[0].hitSub (plain
 *         int, zero sharing) and adds to the LongAdder every 64 (exact
 *         within 63/thread; hits() is diagnostic, absorb greps the
 *         "first gate HIT served" marker which is unchanged).
 *      V2 stays byte-for-byte (serve() dispatches V3 ? serveV3 : serveV2):
 *      cmp432_inside2/cmp430_inside legs are the untouched control.
 *      PARITY: same BlockState objects, same null/miss branches, same
 *      seqlock and stale-miss continuation; rust flips V3 (static v3())
 *      ONLY under lever cmp435_inside3 BEFORE selfTest/arm (fail-closed
 *      resolution: v3() unresolvable => dormant).
 *
 * INJECTS-ONLY: class defined into the kernel loader by src/inside_snap.rs
 * (register_natives for snapCollect; ARMED flipped by rust after selfTest).
 */
public final class InsideSnapOps {

    private InsideSnapOps() {}

    // ------------------------------------------------------------------
    // GATE + ARM
    // ------------------------------------------------------------------

    /** Flipped by rust ONLY after define+RegisterNatives+selfTest (colpush arm-order: arm is the LAST step). */
    private static volatile boolean ARMED = false;

    public static void arm() {
        ARMED = true;
        try {
            // TASK-432-B gate fusion: the inside_cache gate (InsideBlockOps,
            // defined only when CRUSSTY_INSIDE_CACHE=1) switches its
            // verify/replay block reads to this plane. Absent bridge (input
            // off) => NCDFE caught => fusion simply off, snap plane unaffected.
            InsideBlockOps.noteSnapArmed();
        } catch (Throwable t) {
            // Bridge may not be defined YET (define-order race between the two
            // activation workers): one bounded daemon retry. Permanent absence
            // (input off) stays fail-open — the snapshot plane does not depend
            // on the cache gate.
            Thread r = new Thread(() -> {
                try {
                    Thread.sleep(5_000L);
                    InsideBlockOps.noteSnapArmed();
                } catch (Throwable ignored) {
                }
            }, "inside-snap-fusion-retry");
            r.setDaemon(true);
            r.start();
        }
        LOG.info("inside_snap: ARMED (snapshot gate live; single-site getBlockState retarget in Entity.lambda$checkInsideBlocks$2; inside_cache gate fusion notified)");
    }

    public static boolean armed() {
        return ARMED;
    }

    /**
     * TASK-435-B V3 arm (called by rust ONLY under lever cmp435_inside3,
     * BEFORE selfTest/arm — fail-closed: if rust cannot resolve this static
     * the whole plane stays dormant rather than silently running V2).
     */
    private static volatile boolean V3 = false;

    public static void v3() {
        V3 = true;
    }

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    /** Effect marker (first real HIT serve) — grep-able stdout proof. */
    private static volatile boolean FIRST_HIT_LOGGED = false;

    // ------------------------------------------------------------------
    // SNAPSHOTS
    // ------------------------------------------------------------------

    /** Max tracked sections (identity-keyed). Beyond: gate misses to vanilla. */
    static final int CAP = 1 << 15;
    /** Max full-mode (BlockState[4096]) snapshots: 8192 * 32KB refs ≈ 262MB worst. */
    static final int FULL_CAP = 1 << 13;
    /** Max sections gathered per collect (bounded burst). */
    static final int MAX_PER_COLLECT = 64;

    static final class Snap {
        volatile long gen;            // invalidation epoch (secWrite bumps on real change)
        volatile long builtAtGen;     // epoch the published content was built at
        volatile BlockState[] states; // full mode (null in single mode)
        volatile BlockState single;   // single mode: one-object serve (bpe==0)
        volatile boolean pending;     // needs (re)build
        volatile int fails;           // consecutive publish failures (spin guard)
    }

    static final ConcurrentHashMap<LevelChunkSection, Snap> SNAPS = new ConcurrentHashMap<>();
    static final AtomicInteger FULL_COUNT = new AtomicInteger();
    /** TASK-432-B: per-HIT counter — LongAdder (striped) kills the CAS
     *  contention the 4 region workers paid on every served position. */
    static final LongAdder STAT_HITS = new LongAdder();
    static final AtomicLong STAT_MISSES = new AtomicLong();
    static final AtomicLong STAT_COLLECTS = new AtomicLong();
    static final AtomicLong STAT_SECTIONS = new AtomicLong();
    static final AtomicLong STAT_INVALIDATIONS = new AtomicLong();

    public static long hits() { return STAT_HITS.sum(); }
    public static long misses() { return STAT_MISSES.get(); }
    public static long collects() { return STAT_COLLECTS.get(); }
    public static long sections() { return SNAPS.size(); }
    public static long invalidations() { return STAT_INVALIDATIONS.get(); }

    // ------------------------------------------------------------------
    // GATE (the retargeted site) — receiver-first 3B->3B
    // ------------------------------------------------------------------

    public static BlockState snapGet(Level level, BlockPos pos) {
        if (ARMED) {
            try {
                BlockState hit = serve(level, pos);
                if (hit != null) {
                    if (!V3) {
                        // V3 counts per-thread sampled (see lanes[0].hitSub);
                        // V2 keeps the legacy per-hit LongAdder bump.
                        STAT_HITS.increment();
                    }
                    if (!FIRST_HIT_LOGGED) {
                        FIRST_HIT_LOGGED = true;
                        LOG.info("inside_snap: first gate HIT served (palette[" + ((pos.getY() & 15) << 8 | (pos.getZ() & 15) << 4 | (pos.getX() & 15)) + "] from fresh section snapshot)");
                    }
                    return hit;
                }
            } catch (Throwable t) {
                // fail-dominant: vanilla continuation below
            }
        }
        return level.getBlockState(pos); // bit-exact vanilla continuation (miss path)
    }

    // ------------------------------------------------------------------
    // TASK-432-B SERVE FASTPATH — per-thread, per-tick warm lanes
    // ------------------------------------------------------------------

    /** Claimable lanes (round-robin). TASK-435-B: slot LANES is the L0 direct lane. */
    static final int LANES = 8;

    static final class Lane {
        long tick;                 // game-time stamp of the warm pass
        Level level;               // identity guard (multi-world cx/cz clash)
        int cx, cz;
        ChunkAccess ch;
        LevelChunkSection[] secs;
        LevelChunkSection sec;     // L2: sec-ref -> snap (sec->snap is immutable)
        Snap snap;
        // TASK-435-B V3 extras (never read by V2): inline section-index base
        // + runtime-probed ok-flag + per-claim air-mask + per-thread hit
        // counter (sampled into STAT_HITS; lives in slot 0, never copied).
        int minSecY;
        boolean inlineOk;
        boolean maskOk;
        long airMask;
        int hitSub;
    }

    // NO-INDY CLINIT (round-3 NCDFE root-cause, canon ×93-indy): the previous
    // `ThreadLocal.withInitial(InsideSnapOps::newLanes)` was a method-ref INDY
    // whose bootstrap ran during <clinit>; linking `InsideSnapOps::newLanes`
    // resolves the descriptor type `[Lnet/minecraft/world/entity/InsideSnapOps$Lane;`
    // — on the kernel-loader-defined bridge that resolution went through the
    // kernel loader, and with no InsideSnapOps$Lane blob defined there the
    // bootstrap threw NoClassDefFoundError INSIDE <clinit> (run 35902792520:
    // ExceptionInInitializerError @ InsideSnapOps.java:220 -> the class was
    // permanently erroneous -> NCDFE 473412 storm -> fail-closed empty world).
    // Fix = plain `new ThreadLocal<>()` (java.base, no nested class, no indy):
    // nothing Lane-typed is touched until the first per-thread lanes() use on
    // a live server thread (post-Bootstrap), and $Lane is now also DEFINED
    // into the kernel loader by the plugin (see src/inside_snap.rs LANE_CLASS).
    static final ThreadLocal<Lane[]> LANE_TL = new ThreadLocal<>();

    /** Per-thread lanes, created on first USE (never in <clinit>). */
    private static Lane[] lanes() {
        Lane[] a = LANE_TL.get();
        if (a == null) {
            a = newLanes();
            LANE_TL.set(a);
        }
        return a;
    }

    private static Lane[] newLanes() {
        // TASK-435-B: LANES claimable + 1 L0 direct slot (index LANES).
        Lane[] a = new Lane[LANES + 1];
        for (int i = 0; i < LANES + 1; i++) {
            a[i] = new Lane();
        }
        return a;
    }

    /** Shared round-robin for the claim-on-miss slot (misses are the slow path). */
    static final AtomicInteger LANE_CURSOR = new AtomicInteger();

    /** Fresh-hit serve; null => miss (caller falls through to vanilla).
     *  TASK-435-B: V3 dispatch — V2 is the byte-for-byte control path. */
    static BlockState serve(Level level, BlockPos pos) {
        return V3 ? serveV3(level, pos) : serveV2(level, pos);
    }

    /**
     * TASK-435-B V3 lean fastpath (cmp435_inside3). Bit-exact contract of
     * serveV2: identical BlockState objects, identical null/miss branches,
     * identical seqlock + stale-miss continuation; only the per-visit
     * CONSTANT overhead is restructured (research ROUND435_B_RESEARCH.md).
     */
    static BlockState serveV3(Level level, BlockPos pos) {
        final int x = pos.getX(), y = pos.getY(), z = pos.getZ();
        final int cx = x >> 4, cz = z >> 4;
        final long nowTick = level.getGameTime();
        final Lane[] lanes = lanes();
        final Lane l0 = lanes[LANES]; // L0 direct slot — never claimed
        ChunkAccess ch = l0.ch;
        LevelChunkSection[] secs = l0.secs;
        if (!(l0.tick == nowTick && l0.level == level && l0.cx == cx && l0.cz == cz)) {
            Lane hit = null;
            for (int i = 0; i < LANES; i++) {
                Lane l = lanes[i];
                if (l.tick == nowTick && l.level == level && l.cx == cx && l.cz == cz) {
                    hit = l;
                    break;
                }
            }
            if (hit != null) {
                copyLane(hit, l0); // copy-on-match: bursts pay 5 compares again
            } else {
                ch = level.getChunkSource().getChunk(cx, cz, ChunkStatus.FULL, false);
                if (ch == null) {
                    return null; // not loaded: vanilla owns the load semantics
                }
                secs = ch.getSections();
                Lane l = lanes[(LANE_CURSOR.getAndIncrement() & (LANES - 1))];
                l.tick = nowTick;
                l.level = level;
                l.cx = cx;
                l.cz = cz;
                l.ch = ch;
                l.secs = secs;
                l.sec = null;
                l.snap = null;
                // V3 claim extras. Inline-si parity probe (fail-open): the
                // kernel chain is javap-proven ((y>>4) - getMinSectionY()),
                // the probe re-proves it per-chunk against the LIVE receiver.
                final LevelHeightAccessor ha = (LevelHeightAccessor) ch;
                final int minSecY = ha.getMinSectionY();
                l.minSecY = minSecY;
                l.inlineOk = ha.getSectionIndex(0) == (0 >> 4) - minSecY
                        && ha.getSectionIndex(17) == (17 >> 4) - minSecY
                        && ha.getSectionIndex(-17) == ((-17) >> 4) - minSecY;
                l.maskOk = secs.length <= 64;
                l.airMask = l.maskOk ? buildAirMask(secs) : 0L;
                copyLane(l, l0); // L0 mirrors the fresh claim
                ch = l0.ch;
                secs = l0.secs;
            }
        }
        final int si = l0.inlineOk ? ((y >> 4) - l0.minSecY)
                : ((LevelHeightAccessor) ch).getSectionIndex(y);
        if (si < 0 || si >= secs.length) {
            return null; // outside storage: vanilla (VOID_AIR branch)
        }
        if (l0.maskOk) {
            if (((l0.airMask >> si) & 1L) != 0L) {
                return null; // sec == null || hasOnlyAir(): vanilla AIR branch
            }
        } else {
            LevelChunkSection pre = secs[si];
            if (pre == null || pre.hasOnlyAir()) {
                return null;
            }
        }
        final LevelChunkSection sec = secs[si];
        Snap s;
        if (l0.sec == sec && l0.snap != null) {
            s = l0.snap; // sec->snap mapping is immutable (SNAPS never evicts)
        } else {
            s = SNAPS.get(sec);
            if (s == null) {
                register(sec);
                maybeCollect();
                return null;
            }
            l0.sec = sec;
            l0.snap = s;
        }
        final int packed = ((y & 15) << 8) | ((z & 15) << 4) | (x & 15);
        BlockState[] a = s.states;
        if (a != null && s.builtAtGen == s.gen) {
            return hitV3(lanes, a[packed]);
        }
        BlockState sg = s.single;
        if (sg != null && s.builtAtGen == s.gen) {
            return hitV3(lanes, sg);
        }
        if (s.pending) {
            maybeCollect();
        }
        // STALE-MISS: bit-exact continuation of LevelChunk.getBlockStateFinal's
        // non-air branch from the section resolved above (V2 semantics).
        return sec.states.get(packed);
    }

    /** Per-thread sampled hit accounting (V3): exact within 63/thread. */
    private static BlockState hitV3(Lane[] lanes, BlockState st) {
        if (st != null) {
            Lane c = lanes[0]; // slot 0 doubles as the per-thread stats holder
            if (++c.hitSub == 64) {
                c.hitSub = 0;
                STAT_HITS.add(64);
            }
        }
        return st;
    }

    /** Per-claim air-mask: bit si set <=> secs[si] == null || hasOnlyAir(). */
    private static long buildAirMask(LevelChunkSection[] secs) {
        long mask = 0L;
        for (int i = 0; i < secs.length && i < 64; i++) {
            LevelChunkSection sc = secs[i];
            if (sc == null || sc.hasOnlyAir()) {
                mask |= (1L << i);
            }
        }
        return mask;
    }

    private static void copyLane(Lane src, Lane dst) {
        dst.tick = src.tick;
        dst.level = src.level;
        dst.cx = src.cx;
        dst.cz = src.cz;
        dst.ch = src.ch;
        dst.secs = src.secs;
        dst.sec = src.sec;
        dst.snap = src.snap;
        dst.minSecY = src.minSecY;
        dst.inlineOk = src.inlineOk;
        dst.maskOk = src.maskOk;
        dst.airMask = src.airMask;
        // dst.hitSub NOT copied: slot 0 is the per-thread stats holder
    }

    static BlockState serveV2(Level level, BlockPos pos) {
        int x = pos.getX(), y = pos.getY(), z = pos.getZ();
        int cx = x >> 4, cz = z >> 4;
        long nowTick = level.getGameTime();
        Lane[] lanes = lanes();
        Lane lane = null;
        for (int i = 0; i < LANES; i++) {
            Lane l = lanes[i];
            if (l.tick == nowTick && l.level == level && l.cx == cx && l.cz == cz) {
                lane = l;
                break;
            }
        }
        ChunkAccess ch;
        LevelChunkSection[] secs;
        if (lane != null) {
            ch = lane.ch;
            secs = lane.secs;
        } else {
            ch = level.getChunkSource().getChunk(cx, cz, ChunkStatus.FULL, false);
            if (ch == null) {
                return null; // not loaded: vanilla owns the load semantics
            }
            secs = ch.getSections();
            Lane l = lanes[(LANE_CURSOR.getAndIncrement() & (LANES - 1))];
            l.tick = nowTick;
            l.level = level;
            l.cx = cx;
            l.cz = cz;
            l.ch = ch;
            l.secs = secs;
            l.sec = null;
            l.snap = null;
            lane = l;
        }
        int si = ((LevelHeightAccessor) ch).getSectionIndex(y);
        if (si < 0 || si >= secs.length) {
            return null; // outside storage: vanilla (VOID_AIR branch)
        }
        LevelChunkSection sec = secs[si];
        if (sec == null || sec.hasOnlyAir()) {
            return null; // vanilla AIR branch; no snapshot needed
        }
        Snap s;
        if (lane.sec == sec && lane.snap != null) {
            s = lane.snap; // sec->snap mapping is immutable (SNAPS never evicts)
        } else {
            s = SNAPS.get(sec);
            if (s == null) {
                register(sec);
                maybeCollect();
                return null;
            }
            lane.sec = sec;
            lane.snap = s;
        }
        int packed = ((y & 15) << 8) | ((z & 15) << 4) | (x & 15);
        BlockState[] a = s.states;
        if (a != null && s.builtAtGen == s.gen) {
            return a[packed];
        }
        BlockState sg = s.single;
        if (sg != null && s.builtAtGen == s.gen) {
            return sg;
        }
        if (s.pending) {
            maybeCollect();
        }
        // STALE-MISS: bit-exact continuation of LevelChunk.getBlockStateFinal's
        // non-air branch from the section resolved above (same packing javap
        // @3f6f6e6: ((y&15)<<8)|((z&15)<<4)|(x&15)) — kills the redundant second
        // chunk-map lookup the old level.getBlockState(pos) fallthrough paid.
        return sec.states.get(packed);
    }

    static void register(LevelChunkSection sec) {
        if (SNAPS.size() >= CAP) {
            return; // cap: this section keeps missing to vanilla
        }
        Snap fresh = new Snap();
        fresh.pending = true;
        if (SNAPS.putIfAbsent(sec, fresh) == null) {
            PENDING.incrementAndGet();
        }
    }

    // ------------------------------------------------------------------
    // INVALIDATION (the retargeted write site) — secWrite pattern
    // ------------------------------------------------------------------

    public static BlockState secWrite(LevelChunkSection section, int x, int y, int z, BlockState newState) {
        BlockState old = section.setBlockState(x, y, z, newState); // vanilla write delegate
        if (old != newState) { // ref-compare: identical state => words+palette unchanged
            Snap s = SNAPS.get(section);
            if (s != null) {
                s.gen++;          // volatile bump AFTER the write (seqlock discipline)
                s.pending = true;
                s.fails = 0;      // real write happened: a retry is meaningful again
                STAT_INVALIDATIONS.incrementAndGet();
                PENDING.incrementAndGet();
            }
        }
        return old;
    }

    // ------------------------------------------------------------------
    // COLLECT — ONE bulk JNI per collect (event-driven, ≈0 in steady state)
    // ------------------------------------------------------------------

    /** Dirty-trigger heuristic: >0 => a sweep is worthwhile. */
    static final AtomicInteger PENDING = new AtomicInteger();
    static final AtomicBoolean COLLECTING = new AtomicBoolean();

    static void maybeCollect() {
        if (!ARMED || PENDING.get() <= 0) {
            return;
        }
        if (!COLLECTING.compareAndSet(false, true)) {
            return; // one collector at a time (region-threads workers)
        }
        try {
            collect();
        } catch (Throwable t) {
            LOG.warning("inside_snap: collect failed (" + t + ") — snapshots keep missing to vanilla");
        } finally {
            COLLECTING.set(false);
        }
    }

    // grow-once flat buffers (G3 discipline: no per-collect allocation on the hot path)
    static int[] META = new int[3 * MAX_PER_COLLECT];
    static long[] WBUF = new long[64 * 1024];
    static int[] OUT = new int[4096 * MAX_PER_COLLECT];
    static LevelChunkSection[] SECB = new LevelChunkSection[MAX_PER_COLLECT];
    static Object[] PALB = new Object[MAX_PER_COLLECT]; // BlockState[] erasure
    static Snap[] SNAPB = new Snap[MAX_PER_COLLECT];
    static long[] GENB = new long[MAX_PER_COLLECT];

    static final int ERR_STRUCT = -1;
    static final int ERR_RANGE = -2;

    private static void grow(int wordsNeeded, int sectionsNeeded) {
        if (wordsNeeded > WBUF.length) WBUF = new long[Math.max(wordsNeeded, WBUF.length * 2)];
        if (sectionsNeeded * 3 > META.length) {
            META = new int[Math.max(sectionsNeeded * 3, META.length * 2)];
            OUT = new int[4096 * sectionsNeeded];
            SECB = new LevelChunkSection[sectionsNeeded];
            PALB = new Object[sectionsNeeded];
            SNAPB = new Snap[sectionsNeeded];
            GENB = new long[sectionsNeeded];
        }
    }

    @SuppressWarnings("unchecked")
    static void collect() {
        PENDING.getAndSet(0); // sweep accounts its own leftovers below
        int n = 0;
        int wordsLen = 0;
        boolean overflow = false;
        for (Map.Entry<LevelChunkSection, Snap> e : SNAPS.entrySet()) {
            if (n >= MAX_PER_COLLECT) {
                overflow = true; // more pending entries may remain: re-trigger
                break;
            }
            LevelChunkSection sec = e.getKey();
            Snap s = e.getValue();
            if (!s.pending) {
                continue;
            }
            PalettedContainer.Data<BlockState> d = sec.states.data; // ONE volatile read
            // CCE-433 fix: kernel palette array is ERASED (allocated as Object[]) on this
            // build — a call-site cast to BlockState[] threw CCE x18k/leg and the whole
            // stage-1c plane stayed dead (fail-closed). Double-cast through Object erases
            // the array checkcast; ELEMENTS are cast individually below (they ARE
            // BlockState instances — only the ARRAY type is erased).
            Object[] pal = (Object[]) (Object) d.moonrise$getPalette();
            if (pal == null) {
                continue; // fast palette not built: keep missing to vanilla (readPaletteSlow territory)
            }
            int bpe = d.storage().getBits();
            long[] raw = d.storage().getRaw();
            int need = (bpe == 0) ? 0 : ((4096 + 64 / bpe - 1) / (64 / bpe));
            if (bpe != 0 && raw.length < need) {
                continue; // kernel invariant mismatch: skip (conservative)
            }
            grow(wordsLen + need, n + 1);
            META[n * 3] = bpe;
            META[n * 3 + 1] = wordsLen;
            META[n * 3 + 2] = need;
            if (need > 0) {
                System.arraycopy(raw, 0, WBUF, wordsLen, need); // torn-copy safe: seqlock re-verify below
            }
            SECB[n] = sec;
            PALB[n] = pal;
            SNAPB[n] = s;
            GENB[n] = s.gen;
            wordsLen += need;
            n++;
        }
        if (n == 0) {
            if (overflow) {
                PENDING.incrementAndGet();
            }
            return;
        }
        STAT_COLLECTS.incrementAndGet();
        int rc = snapCollect(n, META, WBUF, OUT);
        if (rc == ERR_STRUCT) {
            // fail-closed: disarm permanently (never a wrong serve)
            ARMED = false;
            LOG.warning("inside_snap: snapCollect ERR_STRUCT — DISARMED (vanilla getBlockState restored)");
            return;
        }
        if (rc != n) {
            // transient (range/grow): leave everything pending for the next sweep
            PENDING.incrementAndGet();
            return;
        }
        int leftover = 0;
        for (int k = 0; k < n; k++) {
            Snap s = SNAPB[k];
            if (s == null) continue;
            long g = GENB[k];
            int bpe = META[k * 3];
            if (s.gen != g) {
                s.pending = true; // invalidated in flight: rebuild next sweep
                leftover++;
                continue;
            }
            if (bpe == 0) {
                BlockState v = (BlockState) ((Object[]) PALB[k])[0]; // CCE-433: element-level cast
                if (v == null) {
                    if (++s.fails >= 3) { s.pending = false; } else { s.pending = true; leftover++; }
                    continue;
                }
                s.single = v;      // publish content, then stamp (reader order: content -> gens)
                if (s.states != null) { // full -> single downgrade: release a full slot
                    s.states = null;
                    FULL_COUNT.decrementAndGet();
                }
                s.builtAtGen = g;
            } else {
                Object palO = PALB[k];
                Object[] pal = (Object[]) palO; // CCE-433: erased array, element casts below
                BlockState[] arr = new BlockState[4096];
                boolean bad = false;
                int base = k * 4096;
                for (int i = 0; i < 4096; i++) {
                    int idx = OUT[base + i];
                    if (idx < 0 || idx >= pal.length) { bad = true; break; }
                    BlockState v = (BlockState) pal[idx]; // CCE-433: element-level cast
                    if (v == null) { bad = true; break; }
                    arr[i] = v;
                }
                if (bad) {
                    // torn read (in-flight write) => retry; a STRUCTURALLY bad
                    // section (palette invariant violation) must NOT spin the
                    // collector: after 3 failed sweeps give up permanently
                    // (vanilla miss forever) until a real write resets fails.
                    if (++s.fails >= 3) {
                        s.pending = false;
                    } else {
                        s.pending = true;
                        leftover++;
                    }
                    continue;
                }
                if (FULL_COUNT.get() >= FULL_CAP && s.states == null) {
                    // memory cap: keep vanilla for brand-new full-mode sections
                    s.pending = false;
                    continue;
                }
                if (s.states == null) {
                    FULL_COUNT.incrementAndGet();
                }
                s.states = arr;
                s.builtAtGen = g;
            }
            s.pending = false;
            s.fails = 0; // clean publish: reset the spin guard
            if (s.gen != g) { // in-flight invalidation AFTER publish: disarm stamp
                s.builtAtGen = -1L;
                s.pending = true;
                leftover++;
            }
        }
        STAT_SECTIONS.addAndGet(n - leftover);
        if (leftover > 0 || overflow) {
            PENDING.addAndGet(leftover + (overflow ? 1 : 0));
        }
    }

    // ------------------------------------------------------------------
    // NATIVE (rust core: bit-exact kernel-layout bulk decode)
    // ------------------------------------------------------------------

    private static native int snapCollect(int nsec, int[] meta, long[] words, int[] out);

    private static native int snapProbe();

    // ------------------------------------------------------------------
    // SELFTEST (called by rust on the KEPT define_class ref BEFORE arm)
    // ------------------------------------------------------------------

    /** Rust probe magic "BSNP". */
    static final int PROBE_MAGIC = 0x42534E50;

    public static boolean selfTest() {
        if (snapProbe() != PROBE_MAGIC) {
            return false;
        }
        // bpe=4 synthetic section: 4096 entries, vpl=16, 256 words.
        // entry i carries value (i * 7) & 15 — packed with the kernel MODULO
        // layout (word w holds entries [w*16,(w+1)*16) at shifts 0,4,..,60).
        int[] meta = {4, 0, 256};
        long[] words = new long[256];
        for (int w = 0; w < 256; w++) {
            long acc = 0;
            for (int j = 0; j < 16; j++) {
                int i = w * 16 + j;
                acc |= (long) ((i * 7) & 15) << (j * 4);
            }
            words[w] = acc;
        }
        int[] out = new int[4096];
        if (snapCollect(1, meta, words, out) != 1) {
            return false;
        }
        for (int i = 0; i < 4096; i++) {
            if (out[i] != ((i * 7) & 15)) {
                return false;
            }
        }
        // bpe=15: vpl=4, 1024 words, maximal packing (60 bits used per word).
        int[] meta15 = {15, 0, 1024};
        long[] words15 = new long[1024];
        for (int w = 0; w < 1024; w++) {
            long acc = 0;
            for (int j = 0; j < 4; j++) {
                int i = w * 4 + j;
                acc |= (long) ((i * 11 + 3) & 32767) << (j * 15);
            }
            words15[w] = acc;
        }
        int[] out15 = new int[4096];
        if (snapCollect(1, meta15, words15, out15) != 1) {
            return false;
        }
        for (int i = 0; i < 4096; i++) {
            if (out15[i] != ((i * 11 + 3) & 32767)) {
                return false;
            }
        }
        return true;
    }
}
