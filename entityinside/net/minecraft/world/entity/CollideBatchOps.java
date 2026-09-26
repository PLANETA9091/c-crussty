package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import ca.spottedleaf.moonrise.patches.block_counting.BlockCountingChunkSection;
import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil;
import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.LazyEntityCollisionContext;
import ca.spottedleaf.moonrise.patches.collisions.block.CollisionBlockState;
import ca.spottedleaf.moonrise.patches.collisions.shape.CollisionVoxelShape;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.border.WorldBorder;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.shapes.VoxelShape;

import java.util.List;
import java.util.function.BiPredicate;

/**
 * COLLIDE BATCH-MERGE / SWEEP-LINE (TASK-401-B, vector collide-batch, lever
 * cmp401_collide) — per-tick, per-worker SECTION PLAN cache for the block
 * collision scan (collide 8.68-9.45% java @150k x region_threads=4).
 *
 * UPSTREAM LINEAGE (RESEARCH-B.md):
 *   - Lithium PR #84: per-section counting of oversized blocks -> the
 *     section-level pre-gate walked verbatim here as
 *     BlockCountingChunkSection.moonrise$hasSpecialCollidingBlocks;
 *   - Lithium PR #83: per-state shape caches -> here
 *     CollisionBlockState.moonrise$emptyContextCollisionShape /
 *     moonrise$getConstantContextCollisionShape, read ONCE per section plan
 *     build instead of per-entity per-block;
 *   - Lithium ChunkAwareBlockCollisionSweeper: chunk/section-aware iteration
 *     (still PER-ENTITY — the sweep-line delta of this lever is that the
 *     section plan is SHARED across every entity query of the same section
 *     within one worker tick: the query stream batch-merges into one build);
 *   - Paper 322970283426 / aff206ef815b / 51bc97bd9037: the moonrise
 *     CollisionUtil lineage this class mirrors for its vanilla fragment.
 *
 * MECHANISM: the first (THRESH-1) queries of a section within a worker tick
 * run the verbatim moonrise scan fragment; on the THRESH query the section
 * is swept once (4096 palette cells) into a flat plan: collidable cells
 * only, sorted by packed (lz, lx, ly), constant shapes pre-resolved —
 * single-AABB entries store WORLD-ABSOLUTE bounds (six doubles at build
 * time: the exact adds vanilla's AABB.move does), constant voxel entries
 * store the shape, context-dependent entries (fluids/pistons) stay DYNAMIC
 * and resolve per query exactly like vanilla. Later queries of the same
 * tick walk the plan: covered columns via binary search, pure double math,
 * zero paletted reads per entity. Plan epoch = level.getGameTime(); a
 * stale plan is never walked.
 *
 * PARITY CONTRACT:
 *   - the vanilla fragment below is the round-400-F javap-mirrored port of
 *     CollisionUtil.getCollisionsForBlocksOrWorldBorder (order chunkZ ->
 *     chunkX -> sectionY -> y -> z -> x, ring filters, flags 1/2/4/8,
 *     worldborder-first) — the sub-threshold path runs it verbatim;
 *   - the plan path yields the SAME SET of boxes/shapes for any query.
 *     Consumers are order-free: CollisionUtil.performCollisions composes
 *     the per-axis clamps as running min/max (commutative/associative),
 *     boolean consumers (checkOnly/noCollision/collided) are order-free;
 *     axis order X->Y->Z and the hit-path allocations (new AABB /
 *     shape.move) are the vanilla ones;
 *   - TASK-466-C17 SOLID path (OFF_SOLID): a section whose palette is a
 *     singleton (PalettedContainer.bitsPerEntry()==0, vanilla ZeroBitStorage)
 *     and whose single state has a constant single AABB of exactly [0,1]^3
 *     (and is not special) yields the SAME window box set as the vanilla
 *     fragment by construction — see walkSolid; uniform non-collidable
 *     sections (empty context shape) yield an empty plan, the same set the
 *     sweep would produce; the per-tick epoch gate bounds staleness exactly
 *     like the shipped plan path;
 *   - DYNAMIC entries re-resolve per query through the vanilla context
 *     chain (state.getCollisionShape / LazyEntityCollisionContext), so
 *     fluid shapes never go stale even inside a tick;
 *   - worldborder, unloaded chunks (flags&2), hasOnlyAir, section bounds,
 *     BiPredicate filter call sites — vanilla verbatim;
 *   - empty flag = this class is never defined (rust-side gate) — vanilla
 *     bit-for-bit. Any structural surprise falls back to the vanilla
 *     fragment (fail-closed).
 *
 * NO nested classfiles (S7-163): worker state is one ThreadLocal<Object[]>
 * of flat parallel arrays; plans live in shared per-worker entry pools.
 */
public final class CollideBatchOps {

    private CollideBatchOps() {}

    // ------------------------------------------------------------------
    // Tunables (fixed; no env reads — flag gate lives on the rust side)
    // ------------------------------------------------------------------
    /** queries/section/tick before the plan materializes */
    private static final int THRESH = 24;
    /** sections with more entries than this never get a plan (solid terrain) */
    private static final int DENSE_LIMIT = 1400;
    /** plan table slots per worker (power of two) */
    private static final int TABLE = 1024;
    /** absolute entry pool cap per worker (bounds pool = x6 doubles) */
    private static final int POOL_CAP = 1 << 18;

    // entry flag bits
    private static final int F_LARGE = 1;   // state.hasLargeCollisionShape()
    private static final int F_PISTON = 2;  // state.getBlock() == MOVING_PISTON
    private static final int KIND_MASK = 12;
    private static final int KIND_SINGLE = 0;   // constant shape, single AABB
    private static final int KIND_VOXEL = 4;    // constant shape, real voxel
    private static final int KIND_DYNAMIC = 8;  // context-dependent shape

    // plan offset sentinels
    private static final int OFF_NONE = -1;       // no plan yet (counting)
    private static final int OFF_VANILLA = -2;    // dense/failed: vanilla only this tick
    private static final int OFF_SOLID = -3;      // TASK-466-C17: uniform full-cube section

    // TASK-466-C17 UNIT full cube: walkSolid intersects/move parity anchor — the
    // vanilla single-AABB path for a full-cube state is exactly
    // single.move(x,y,z) with single == [0,1]^3 (moonrise
    // $getSingleAABBRepresentation), so sharing one immutable instance keeps
    // the same doubles bit-for-bit (0.0 + x == x, 1.0 + x == x + 1.0 exactly).
    private static final AABB UNIT_CUBE =
        new AABB(0.0D, 0.0D, 0.0D, 1.0D, 1.0D, 1.0D);

    // TASK-466-C17 path diag (fail-safe: DIAG is an env-gated JIT constant —
    // dead-stripped when CRUSSTY_LEVER_ARG lacks "collidediag"; counters are
    // plain diagnostics, lost updates across workers are acceptable).
    private static final boolean DIAG;
    private static final java.util.concurrent.atomic.AtomicLongArray DIAG_C;
    private static volatile long diagLastTick;

    static {
        boolean d = false;
        String la = null;
        try {
            la = System.getenv("CRUSSTY_LEVER_ARG");
        } catch (final Throwable t) {
            la = null;
        }
        if (la != null && la.indexOf("collidediag") >= 0) {
            d = true;
        }
        DIAG = d;
        DIAG_C = d ? new java.util.concurrent.atomic.AtomicLongArray(8) : null;
    }

    // diag slots: 0 q_subthreshold, 1 q_offvanilla, 2 q_walk, 3 q_solid,
    // 4 builds, 5 solidPlans, 6 emptyPlans, 7 vanillaFragmentCells
    private static void diagAdd(final int slot, final long v) {
        if (DIAG) {
            DIAG_C.addAndGet(slot, v);
        }
    }

    private static void diagMaybePrint(final long now) {
        if (DIAG && now != diagLastTick && now % 600L == 0L) {
            diagLastTick = now;
            System.out.println("[c17diag] t=" + now
                + " subthr=" + DIAG_C.get(0)
                + " offvan=" + DIAG_C.get(1)
                + " walk=" + DIAG_C.get(2)
                + " solid=" + DIAG_C.get(3)
                + " builds=" + DIAG_C.get(4)
                + " solidPlans=" + DIAG_C.get(5)
                + " emptyPlans=" + DIAG_C.get(6));
        }
    }

    // worker state slots (single classfile law: Object[], no holder class)
    // 0 long[] hkey, 1 int[] vCx, 2 int[] vCz, 3 int[] vCy, 4 int[] vLid,
    // 5 long[] epoch, 6 int[] qcount, 7 int[] planOff, 8 int[] planLen,
    // 9 byte[] special, 10 int[] cellsPool, 11 int[] flagsPool,
    // 12 double[] boundsPool, 13 Object[] statesPool, 14 Object[] shapesPool,
    // 15 int[] misc {usedSlots, poolLen}
    private static final int WS_SIZE = 16;

    private static final java.lang.ThreadLocal<Object[]> WS =
        java.lang.ThreadLocal.withInitial(CollideBatchOps::freshWs);

    private static Object[] freshWs() {
        Object[] ws = new Object[WS_SIZE];
        ws[0] = new long[TABLE];
        ws[1] = new int[TABLE];
        ws[2] = new int[TABLE];
        ws[3] = new int[TABLE];
        ws[4] = new int[TABLE];
        ws[5] = new long[TABLE];
        ws[6] = new int[TABLE];
        final int[] planOff = new int[TABLE];
        java.util.Arrays.fill(planOff, OFF_NONE);
        ws[7] = planOff;
        ws[8] = new int[TABLE];
        ws[9] = new byte[TABLE];
        ws[10] = new int[4096];
        ws[11] = new int[4096];
        ws[12] = new double[4096 * 6];
        ws[13] = new Object[4096];
        ws[14] = new Object[4096];
        ws[15] = new int[2];
        return ws;
    }

    @SuppressWarnings("unchecked")
    private static <T> T arr(final Object[] ws, final int slot) {
        return (T) ws[slot];
    }

    private static void resetTable(final Object[] ws) {
        java.util.Arrays.fill((long[]) ws[0], 0L);
        java.util.Arrays.fill((int[]) ws[6], 0);
        java.util.Arrays.fill((int[]) ws[7], OFF_NONE);
        final int[] misc = arr(ws, 15);
        misc[0] = 0;
        misc[1] = 0;
    }

    // ------------------------------------------------------------------
    // Entry point — body redirect target of the static
    // CollisionUtil.getCollisionsForBlocksOrWorldBorder (identical erased
    // descriptor, redirect_static_method_body_to_static).
    // ------------------------------------------------------------------
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static boolean blockCollisions(final Level level, final Entity entity, final AABB box,
                                          final List shapesVoxel,
                                          final List boxesAABB, final int flags,
                                          final BiPredicate<BlockState, BlockPos> filter) {
        // off 0..13: checkOnly = (flags & 8) != 0
        final boolean checkOnly = (flags & 8) != 0;
        boolean collided = false;
        // off 18..78: world border branch (border-first, Paper aff206ef order)
        if ((flags & 4) != 0) {
            final WorldBorder wb = level.getWorldBorder();
            if (CollisionUtil.isCollidingWithBorder(wb, box)
                    && entity != null
                    && wb.isInsideCloseToBorder(entity, box)) {
                if (checkOnly) {
                    return true;
                }
                collided = true;
                shapesVoxel.add(wb.getCollisionShape());
            }
        }
        // off 80..99: scan bounds (floor +-1e-7, then +-1 block ring)
        final int minSection = WorldUtil.getMinSection(level);
        final int minScanX = Mth.floor(box.minX - 1.0E-7D) - 1;
        final int maxScanX = Mth.floor(box.maxX + 1.0E-7D) + 1;
        final int minScanY = Math.max((minSection << 4) - 1, Mth.floor(box.minY - 1.0E-7D) - 1);
        final int maxScanY = Math.min((WorldUtil.getMaxSection(level) << 4) + 16,
            Mth.floor(box.maxY + 1.0E-7D) + 1);
        final int minScanZ = Mth.floor(box.minZ - 1.0E-7D) - 1;
        final int maxScanZ = Mth.floor(box.maxZ + 1.0E-7D) + 1;
        // off 206..221: lazy entity context + entity-shape gate (minecart-only)
        final LazyEntityCollisionContext context = new LazyEntityCollisionContext(entity);
        final boolean useEntityCollisionShape =
            LazyEntityCollisionContext.useEntityCollisionShape(level, entity);
        // off 223..232: degenerate y range -> early out
        if (minScanY > maxScanY) {
            return collided;
        }
        // off 233..267: section bounds
        final int cx0 = minScanX >> 4;
        final int cx1 = maxScanX >> 4;
        final int cy0 = minScanY >> 4;
        final int cy1 = maxScanY >> 4;
        final int cz0 = minScanZ >> 4;
        final int cz1 = maxScanZ >> 4;
        // off 269..281: load-chunks flag
        final boolean loadChunks = (flags & 1) != 0;
        // off 283..287: chunk source
        final ChunkSource chunkSource = level.getChunkSource();

        // SWEEP-LINE STATE (per worker thread)
        final Object[] ws = WS.get();
        final long[] hkey = arr(ws, 0);
        final int[] vCx = arr(ws, 1);
        final int[] vCz = arr(ws, 2);
        final int[] vCy = arr(ws, 3);
        final int[] vLid = arr(ws, 4);
        final long[] epoch = arr(ws, 5);
        final int[] qcount = arr(ws, 6);
        final int[] planOff = arr(ws, 7);
        final long now = level.getGameTime();
        final int lid = System.identityHashCode(level);
        diagMaybePrint(now);

        // off 289..1066: chunkZ -> chunkX -> sectionY (order fixed)
        for (int cz = cz0; cz <= cz1; ++cz) {
            for (int cx = cx0; cx <= cx1; ++cx) {
                final ChunkAccess chunk = chunkSource.getChunk(cx, cz, ChunkStatus.FULL, loadChunks);
                if (chunk == null) {
                    if ((flags & 2) != 0) {
                        if (checkOnly) {
                            return true;
                        }
                        collided = true;
                        boxesAABB.add(CollisionUtil.getBoxForChunk(cx, cz));
                    }
                    continue;
                }
                final LevelChunkSection[] sections = chunk.getSections();
                for (int cy = cy0; cy <= cy1; ++cy) {
                    final int si = cy - minSection;
                    if (si < 0 || si >= sections.length) {
                        continue;
                    }
                    final LevelChunkSection section = sections[si];
                    if (section.hasOnlyAir()) {
                        continue;
                    }

                    // ---------- sweep-line: locate / count / serve ----------
                    final long key = mixKey(lid, cx, cy, cz);
                    int slot = findSlot(hkey, key);
                    if (slot < 0) {
                        slot = insertSlot(ws, key);
                        if (slot < 0) {
                            // post-reset miss: vanilla fragment, no counting
                            collided |= scanSectionVanilla(level, section, cx, cy, cz, box,
                                shapesVoxel, boxesAABB, checkOnly, filter, minScanX, maxScanX,
                                minScanY, maxScanY, minScanZ, maxScanZ, cx0, cx1, cy0, cy1,
                                cz0, cz1, context, useEntityCollisionShape);
                            if (checkOnly && collided) {
                                return true;
                            }
                            continue;
                        }
                        vCx[slot] = cx;
                        vCz[slot] = cz;
                        vCy[slot] = cy;
                        vLid[slot] = lid;
                        epoch[slot] = now;
                        qcount[slot] = 0;
                        planOff[slot] = OFF_NONE;
                    } else if (vCx[slot] != cx || vCz[slot] != cz || vCy[slot] != cy
                            || vLid[slot] != lid) {
                        // hash collision with a different section — replace
                        planOff[slot] = OFF_NONE;
                        vCx[slot] = cx;
                        vCz[slot] = cz;
                        vCy[slot] = cy;
                        vLid[slot] = lid;
                    }
                    if (epoch[slot] != now) {
                        // new tick for this section: counting restarts, the
                        // stale plan is never walked again (freshness gate)
                        epoch[slot] = now;
                        qcount[slot] = 0;
                        if (planOff[slot] != OFF_VANILLA) {
                            planOff[slot] = OFF_NONE;
                        }
                    }
                    final int po = planOff[slot];
                    if (po == OFF_VANILLA) {
                        diagAdd(1, 1L);
                        collided |= scanSectionVanilla(level, section, cx, cy, cz, box,
                            shapesVoxel, boxesAABB, checkOnly, filter, minScanX, maxScanX,
                            minScanY, maxScanY, minScanZ, maxScanZ, cx0, cx1, cy0, cy1,
                            cz0, cz1, context, useEntityCollisionShape);
                        if (checkOnly && collided) {
                            return true;
                        }
                        continue;
                    }
                    if (po == OFF_SOLID) {
                        // TASK-466-C17: uniform full-cube section — the query is
                        // answered from window geometry alone (see walkSolid).
                        diagAdd(3, 1L);
                        collided |= walkSolid(section, cx, cy, cz, box, boxesAABB,
                            checkOnly, filter, minScanX, maxScanX, minScanY, maxScanY,
                            minScanZ, maxScanZ, cx0, cx1, cy0, cy1, cz0, cz1);
                        if (checkOnly && collided) {
                            return true;
                        }
                        continue;
                    }
                    if (po >= 0) {
                        diagAdd(2, 1L);
                        collided |= walkPlan(ws, slot, level, cx, cy, cz, box, shapesVoxel,
                            boxesAABB, checkOnly, filter, minScanX, maxScanX, minScanY, maxScanY,
                            minScanZ, maxScanZ, cx0, cx1, cy0, cy1, cz0, cz1, context,
                            useEntityCollisionShape);
                        if (checkOnly && collided) {
                            return true;
                        }
                        continue;
                    }
                    // below threshold: verbatim vanilla fragment + count
                    diagAdd(0, 1L);
                    collided |= scanSectionVanilla(level, section, cx, cy, cz, box,
                        shapesVoxel, boxesAABB, checkOnly, filter, minScanX, maxScanX,
                        minScanY, maxScanY, minScanZ, maxScanZ, cx0, cx1, cy0, cy1,
                        cz0, cz1, context, useEntityCollisionShape);
                    if (checkOnly && collided) {
                        return true;
                    }
                    qcount[slot]++;
                    if (qcount[slot] >= THRESH) {
                        diagAdd(4, 1L);
                        buildPlan(ws, slot, section, cx, cy, cz);
                    }
                }
            }
        }
        // off 1064..1066
        return collided;
    }

    // ------------------------------------------------------------------
    // Key / slot machinery (open addressing, linear probe)
    // ------------------------------------------------------------------
    private static long mix64(long z) {
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9L;
        z = (z ^ (z >> 27)) * 0x94d049bb133111ebL;
        return z ^ (z >> 31);
    }

    private static long mixKey(final int lid, final int cx, final int cy, final int cz) {
        final long packed = (cx & 0x3FFFFFL) | ((cz & 0x3FFFFFL) << 22) | ((cy & 0xFFFL) << 44);
        final long h = mix64(packed ^ (lid * 0x9E3779B97F4A7C15L));
        return h == 0L ? 0x9E3779B97F4A7C15L : h;
    }

    private static int findSlot(final long[] keys, final long k) {
        final int mask = TABLE - 1;
        int s = (int) (mix64(k) & mask);
        for (int i = 0; i < 64; ++i) {
            final long cur = keys[s];
            if (cur == k) {
                return s;
            }
            if (cur == 0L) {
                return -1;
            }
            s = (s + 1) & mask;
        }
        return -1;
    }

    private static int insertSlot(final Object[] ws, final long k) {
        final long[] keys = arr(ws, 0);
        final int[] misc = arr(ws, 15);
        if (misc[0] >= (TABLE * 3) / 4) {
            resetTable(ws);
        }
        final int mask = TABLE - 1;
        int s = (int) (mix64(k) & mask);
        for (int i = 0; i < TABLE; ++i) {
            if (keys[s] == 0L) {
                keys[s] = k;
                misc[0]++;
                return s;
            }
            s = (s + 1) & mask;
        }
        resetTable(ws);
        return -1;
    }

    // ------------------------------------------------------------------
    // PLAN BUILD: one sweep of the 16^3 section (the batch-merge pass)
    // ------------------------------------------------------------------
    private static void buildPlan(final Object[] ws, final int slot, final LevelChunkSection section,
                                  final int cx, final int cy, final int cz) {
        final int[] misc = arr(ws, 15);
        final int start = misc[1];
        final int[] planOff = arr(ws, 7);
        final int[] planLen = arr(ws, 8);
        final byte[] specialArr = arr(ws, 9);
        // pre-grow so the sweep cannot overflow mid-plan
        final int[] cellsPre = arr(ws, 10);
        if (cellsPre.length - start < DENSE_LIMIT + 1) {
            if (cellsPre.length >= POOL_CAP || !growPools(ws)) {
                planOff[slot] = OFF_VANILLA;
                return;
            }
        }
        final int[] cells = arr(ws, 10);
        final int[] eflags = arr(ws, 11);
        final double[] bounds = arr(ws, 12);
        final Object[] statesPool = arr(ws, 13);
        final Object[] shapesPool = arr(ws, 14);
        final PalettedContainer<BlockState> states = section.states;
        final boolean special =
            ((BlockCountingChunkSection) section).moonrise$hasSpecialCollidingBlocks();
        // TASK-466-C17 singleton-palette pre-gate: bitsPerEntry()==0 means a
        // ZeroBitStorage (vanilla: bits()=0) — every palette lookup resolves to
        // palette[0], so ALL 4096 cells hold exactly one state. Proven uniform
        // without any sweep. Parity envelope identical to the shipped
        // KIND_SINGLE plan path: constant shapes are pre-resolved at build time
        // and consumers are order-free; full-cube singles are context-
        // independent (LazyEntityCollisionContext only reshapes non-cube
        // states), the envelope cmp401_collide shipped with.
        if (states.bitsPerEntry() == 0 && !special) {
            final BlockState u = states.get(0);
            if (((CollisionBlockState) u).moonrise$emptyContextCollisionShape()) {
                // uniform non-collidable section (ocean/lava/leaves-none): an
                // empty plan answers every query with the same (empty) set the
                // sweep would produce, without sweeping 4096 cells.
                planOff[slot] = start;
                planLen[slot] = 0;
                specialArr[slot] = (byte) 0;
                diagAdd(6, 1L);
                return;
            }
            final VoxelShape uConst =
                ((CollisionBlockState) u).moonrise$getConstantContextCollisionShape();
            if (uConst != null) {
                final AABB uSingle =
                    ((CollisionVoxelShape) uConst).moonrise$getSingleAABBRepresentation();
                if (uSingle != null
                        && uSingle.minX == 0.0D && uSingle.minY == 0.0D && uSingle.minZ == 0.0D
                        && uSingle.maxX == 1.0D && uSingle.maxY == 1.0D && uSingle.maxZ == 1.0D) {
                    // uniform FULL-CUBE section (stone/deepslate/bedrock bulk):
                    // no plan entries, walkSolid answers from window geometry.
                    planOff[slot] = OFF_SOLID;
                    planLen[slot] = 0;
                    specialArr[slot] = (byte) 0;
                    diagAdd(5, 1L);
                    return;
                }
            }
            // uniform but not eligible (partial/contextual shape): fall through
            // to the verbatim sweep — dense bail unchanged.
        }
        final int bx = cx << 4;
        final int by = cy << 4;
        final int bz = cz << 4;
        final int cap = Math.min(cells.length - start, DENSE_LIMIT);
        int n = 0;
        boolean dense = false;
        outer:
        for (int lz = 0; lz < 16; ++lz) {
            for (int lx = 0; lx < 16; ++lx) {
                for (int ly = 0; ly < 16; ++ly) {
                    final BlockState state = states.get(lx | (lz << 4) | (ly << 8));
                    // cached empty gate (Lithium PR #83 lineage)
                    if (((CollisionBlockState) state).moonrise$emptyContextCollisionShape()) {
                        continue;
                    }
                    if (n >= cap) {
                        dense = true;
                        break outer;
                    }
                    int fl = 0;
                    if (state.hasLargeCollisionShape()) {
                        fl |= F_LARGE;
                    }
                    if (state.getBlock() == Blocks.MOVING_PISTON) {
                        fl |= F_PISTON;
                    }
                    final VoxelShape constant =
                        ((CollisionBlockState) state).moonrise$getConstantContextCollisionShape();
                    if (constant != null) {
                        final AABB single =
                            ((CollisionVoxelShape) constant).moonrise$getSingleAABBRepresentation();
                        if (single != null) {
                            // world-absolute bounds: the SAME adds vanilla's
                            // AABB.move(x,y,z) performs, precomputed once
                            final int p = start + n;
                            final int b6 = p * 6;
                            bounds[b6] = single.minX + (bx | lx);
                            bounds[b6 + 1] = single.minY + (by | ly);
                            bounds[b6 + 2] = single.minZ + (bz | lz);
                            bounds[b6 + 3] = single.maxX + (bx | lx);
                            bounds[b6 + 4] = single.maxY + (by | ly);
                            bounds[b6 + 5] = single.maxZ + (bz | lz);
                            statesPool[p] = state;
                            shapesPool[p] = null;
                            cells[p] = (lz << 8) | (lx << 4) | ly;
                            eflags[p] = fl | KIND_SINGLE;
                            n++;
                        } else {
                            if (constant.isEmpty()) {
                                continue;
                            }
                            final int p = start + n;
                            statesPool[p] = state;
                            shapesPool[p] = constant;
                            cells[p] = (lz << 8) | (lx << 4) | ly;
                            eflags[p] = fl | KIND_VOXEL;
                            n++;
                        }
                    } else {
                        // context-dependent (fluids/pistons): resolve per query
                        final int p = start + n;
                        statesPool[p] = state;
                        shapesPool[p] = null;
                        cells[p] = (lz << 8) | (lx << 4) | ly;
                        eflags[p] = fl | KIND_DYNAMIC;
                        n++;
                    }
                }
            }
        }
        if (dense) {
            planOff[slot] = OFF_VANILLA;
            return;
        }
        planOff[slot] = start;
        planLen[slot] = n;
        specialArr[slot] = (byte) (special ? 1 : 0);
        misc[1] = start + n;
    }

    /** Double all entry pools (positions of existing plans are preserved). */
    private static boolean growPools(final Object[] ws) {
        final int[] cells = arr(ws, 10);
        final int newSize = cells.length * 2;
        if (newSize > POOL_CAP) {
            return false;
        }
        ws[10] = java.util.Arrays.copyOf(cells, newSize);
        ws[11] = java.util.Arrays.copyOf((int[]) ws[11], newSize);
        final double[] bounds = arr(ws, 12);
        ws[12] = java.util.Arrays.copyOf(bounds, newSize * 6);
        ws[13] = java.util.Arrays.copyOf((Object[]) ws[13], newSize);
        ws[14] = java.util.Arrays.copyOf((Object[]) ws[14], newSize);
        return true;
    }

    // ------------------------------------------------------------------
    // PLAN WALK: same box set as the vanilla fragment (order-free consumers)
    // ------------------------------------------------------------------
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static boolean walkPlan(final Object[] ws, final int slot, final Level level,
                                    final int cx, final int cy, final int cz, final AABB box,
                                    final List shapesVoxel, final List boxesAABB,
                                    final boolean checkOnly,
                                    final BiPredicate<BlockState, BlockPos> filter,
                                    final int minScanX, final int maxScanX,
                                    final int minScanY, final int maxScanY,
                                    final int minScanZ, final int maxScanZ,
                                    final int cx0, final int cx1, final int cy0, final int cy1,
                                    final int cz0, final int cz1,
                                    final LazyEntityCollisionContext context,
                                    final boolean useEntityCollisionShape) {
        final int[] walkPlanOff = arr(ws, 7);
        final int[] walkPlanLen = arr(ws, 8);
        final byte[] walkSpecial = arr(ws, 9);
        final int off = walkPlanOff[slot];
        final int len = walkPlanLen[slot];
        final boolean special = walkSpecial[slot] != 0;
        final int[] cells = arr(ws, 10);
        final int[] eflags = arr(ws, 11);
        final double[] bounds = arr(ws, 12);
        final Object[] statesPool = arr(ws, 13);
        final Object[] shapesPool = arr(ws, 14);
        final boolean noSpecial = !special;
        // local window (identical formulas to the vanilla fragment)
        final int lx0 = (cx == cx0) ? ((minScanX & 15) + (noSpecial ? 1 : 0)) : 0;
        final int lx1 = (cx == cx1) ? ((maxScanX & 15) - (noSpecial ? 1 : 0)) : 15;
        final int lz0 = (cz == cz0) ? ((minScanZ & 15) + (noSpecial ? 1 : 0)) : 0;
        final int lz1 = (cz == cz1) ? ((maxScanZ & 15) - (noSpecial ? 1 : 0)) : 15;
        final int ly0 = (cy == cy0) ? ((minScanY & 15) + (noSpecial ? 1 : 0)) : 0;
        final int ly1 = (cy == cy1) ? ((maxScanY & 15) - (noSpecial ? 1 : 0)) : 15;
        if (lx0 > lx1 || lz0 > lz1 || ly0 > ly1) {
            return false;
        }
        BlockPos.MutableBlockPos pos = null;
        boolean collided = false;
        final int bx = cx << 4;
        final int by = cy << 4;
        final int bz = cz << 4;
        final int end = off + len;
        for (int lz = lz0; lz <= lz1; ++lz) {
            for (int lx = lx0; lx <= lx1; ++lx) {
                // covered column via binary search over the packed cells
                final int colLo = (lz << 8) | (lx << 4);
                final int colHi = colLo | 15;
                int e = lowerBound(cells, off, end, colLo);
                while (e < end && cells[e] <= colHi) {
                    final int ly = cells[e] & 15;
                    if (ly < ly0) {
                        e++;
                        continue;
                    }
                    if (ly > ly1) {
                        break;
                    }
                    final int x = bx | ((cells[e] >>> 4) & 15);
                    final int y = by | ly;
                    final int z = bz | ((cells[e] >>> 8) & 15);
                    final int fl = eflags[e];
                    if (special) {
                        final int ring = ((x == minScanX || x == maxScanX) ? 1 : 0)
                            + ((y == minScanY || y == maxScanY) ? 1 : 0)
                            + ((z == minScanZ || z == maxScanZ) ? 1 : 0);
                        if (ring == 3) {
                            e++;
                            continue;
                        }
                        if (ring == 1) {
                            if ((fl & F_LARGE) == 0) {
                                e++;
                                continue;
                            }
                        } else if (ring == 2) {
                            if ((fl & F_PISTON) == 0) {
                                e++;
                                continue;
                            }
                        }
                    }
                    final int kind = fl & KIND_MASK;
                    if (kind == KIND_SINGLE) {
                        // world-absolute bounds, pure double math
                        final int b6 = e * 6;
                        if ((box.minX - bounds[b6 + 3]) < -1.0E-7D
                                && (box.maxX - bounds[b6]) > 1.0E-7D
                                && (box.minY - bounds[b6 + 4]) < -1.0E-7D
                                && (box.maxY - bounds[b6 + 1]) > 1.0E-7D
                                && (box.minZ - bounds[b6 + 5]) < -1.0E-7D
                                && (box.maxZ - bounds[b6 + 2]) > 1.0E-7D) {
                            if (filter != null) {
                                if (pos == null) {
                                    pos = new BlockPos.MutableBlockPos();
                                }
                                if (!filter.test((BlockState) statesPool[e], pos.set(x, y, z))) {
                                    e++;
                                    continue;
                                }
                            }
                            if (checkOnly) {
                                return true;
                            }
                            collided = true;
                            boxesAABB.add(new AABB(bounds[b6], bounds[b6 + 1], bounds[b6 + 2],
                                bounds[b6 + 3], bounds[b6 + 4], bounds[b6 + 5]));
                        }
                    } else if (kind == KIND_VOXEL) {
                        final VoxelShape shape = (VoxelShape) shapesPool[e];
                        final VoxelShape moved = shape.move(x, y, z);
                        if (!CollisionUtil.voxelShapeIntersectNoEmpty(moved, box)) {
                            e++;
                            continue;
                        }
                        if (filter != null) {
                            if (pos == null) {
                                pos = new BlockPos.MutableBlockPos();
                            }
                            if (!filter.test((BlockState) statesPool[e], pos.set(x, y, z))) {
                                e++;
                                continue;
                            }
                        }
                        if (checkOnly) {
                            return true;
                        }
                        collided = true;
                        shapesVoxel.add(moved);
                    } else {
                        // DYNAMIC: resolve per query exactly like vanilla
                        if (pos == null) {
                            pos = new BlockPos.MutableBlockPos();
                        }
                        pos.set(x, y, z);
                        final BlockState state = (BlockState) statesPool[e];
                        VoxelShape shape = useEntityCollisionShape
                            ? context.getCollisionShape(state, level, pos)
                            : state.getCollisionShape(level, pos, context);
                        final AABB single =
                            ((CollisionVoxelShape) shape).moonrise$getSingleAABBRepresentation();
                        if (single != null) {
                            if (!intersectsMoved(box, single, x, y, z)) {
                                e++;
                                continue;
                            }
                            if (filter != null && !filter.test(state, pos)) {
                                e++;
                                continue;
                            }
                            if (checkOnly) {
                                return true;
                            }
                            collided = true;
                            boxesAABB.add(single.move(x, y, z));
                        } else {
                            if (shape.isEmpty()) {
                                e++;
                                continue;
                            }
                            final VoxelShape moved = shape.move(x, y, z);
                            if (!CollisionUtil.voxelShapeIntersectNoEmpty(moved, box)) {
                                e++;
                                continue;
                            }
                            if (filter != null && !filter.test(state, pos)) {
                                e++;
                                continue;
                            }
                            if (checkOnly) {
                                return true;
                            }
                            collided = true;
                            shapesVoxel.add(moved);
                        }
                    }
                    e++;
                }
            }
        }
        return collided;
    }

    // ------------------------------------------------------------------
    // TASK-466-C17 SOLID WALK: uniform full-cube section (singleton palette).
    // The section holds exactly ONE state whose constant single AABB is exactly
    // [0,1]^3, and it is NOT special (no oversized/piston states => the vanilla
    // ring filters never apply => window bounds use the noSpecial +1/-1 shifts,
    // identical formulas to the vanilla fragment). For any query the vanilla
    // fragment yields exactly: every window cell whose unit cube (moved by the
    // cell coords) intersects the box, filtered by the same filter chain, with
    // new AABB(x, y, z, x+1, y+1, z+1) boxes (bit-identical to single.move for
    // [0,1]^3). This walk reproduces that set without any palette reads.
    // ------------------------------------------------------------------
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static boolean walkSolid(final LevelChunkSection section,
                                     final int cx, final int cy, final int cz, final AABB box,
                                     final List boxesAABB, final boolean checkOnly,
                                     final BiPredicate<BlockState, BlockPos> filter,
                                     final int minScanX, final int maxScanX,
                                     final int minScanY, final int maxScanY,
                                     final int minScanZ, final int maxScanZ,
                                     final int cx0, final int cx1, final int cy0, final int cy1,
                                     final int cz0, final int cz1) {
        // noSpecial window shifts (solid sections are never special by gate)
        final int lx0 = (cx == cx0) ? ((minScanX & 15) + 1) : 0;
        final int lx1 = (cx == cx1) ? ((maxScanX & 15) - 1) : 15;
        final int lz0 = (cz == cz0) ? ((minScanZ & 15) + 1) : 0;
        final int lz1 = (cz == cz1) ? ((maxScanZ & 15) - 1) : 15;
        final int ly0 = (cy == cy0) ? ((minScanY & 15) + 1) : 0;
        final int ly1 = (cy == cy1) ? ((maxScanY & 15) - 1) : 15;
        if (lx0 > lx1 || lz0 > lz1 || ly0 > ly1) {
            return false;
        }
        // O(1) re-read of the uniform state (singleton palette: any index maps
        // to palette[0]; the per-tick epoch gate bounds staleness exactly like
        // the shipped plan path).
        final BlockState solid = section.states.get(0);
        final BlockPos.MutableBlockPos pos = (filter == null) ? null : new BlockPos.MutableBlockPos();
        boolean collided = false;
        final int bx = cx << 4;
        final int by = cy << 4;
        final int bz = cz << 4;
        for (int ly = ly0; ly <= ly1; ++ly) {
            final int y = by | ly;
            for (int lz = lz0; lz <= lz1; ++lz) {
                final int z = bz | lz;
                for (int lx = lx0; lx <= lx1; ++lx) {
                    final int x = bx | lx;
                    if (!intersectsMoved(box, UNIT_CUBE, x, y, z)) {
                        continue;
                    }
                    if (filter != null && !filter.test(solid, pos.set(x, y, z))) {
                        continue;
                    }
                    if (checkOnly) {
                        return true;
                    }
                    collided = true;
                    // bit-identical to AABB.move of the [0,1]^3 single
                    boxesAABB.add(new AABB((double) x, (double) y, (double) z,
                        (double) x + 1.0D, (double) y + 1.0D, (double) z + 1.0D));
                }
            }
        }
        return collided;
    }

    /** Bit-exact decomposition of voxelShapeIntersect(box, aabb.move(x,y,z)). */
    private static boolean intersectsMoved(final AABB box, final AABB shape,
                                           final double x, final double y, final double z) {
        final double sMinX = shape.minX + x;
        final double sMinY = shape.minY + y;
        final double sMinZ = shape.minZ + z;
        final double sMaxX = shape.maxX + x;
        final double sMaxY = shape.maxY + y;
        final double sMaxZ = shape.maxZ + z;
        return (box.minX - sMaxX) < -1.0E-7D
            && (box.maxX - sMinX) > 1.0E-7D
            && (box.minY - sMaxY) < -1.0E-7D
            && (box.maxY - sMinY) > 1.0E-7D
            && (box.minZ - sMaxZ) < -1.0E-7D
            && (box.maxZ - sMinZ) > 1.0E-7D;
    }

    private static int lowerBound(final int[] a, final int from, final int to, final int key) {
        int lo = from;
        int hi = to;
        while (lo < hi) {
            final int mid = (lo + hi) >>> 1;
            if (a[mid] < key) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }

    // ------------------------------------------------------------------
    // VANILLA FRAGMENT: verbatim per-section port of the moonrise scan
    // (round-400-F javap-mirrored; sub-threshold + fail-closed path)
    // ------------------------------------------------------------------
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static boolean scanSectionVanilla(final Level level, final LevelChunkSection section,
                                              final int cx, final int cy, final int cz,
                                              final AABB box, final List shapesVoxel,
                                              final List boxesAABB, final boolean checkOnly,
                                              final BiPredicate<BlockState, BlockPos> filter,
                                              final int minScanX, final int maxScanX,
                                              final int minScanY, final int maxScanY,
                                              final int minScanZ, final int maxScanZ,
                                              final int cx0, final int cx1,
                                              final int cy0, final int cy1,
                                              final int cz0, final int cz1,
                                              final LazyEntityCollisionContext context,
                                              final boolean useEntityCollisionShape) {
        boolean collided = false;
        final boolean special =
            ((BlockCountingChunkSection) section).moonrise$hasSpecialCollidingBlocks();
        final boolean noSpecial = !special;
        final PalettedContainer<BlockState> states = section.states;
        final int lx0 = (cx == cx0) ? ((minScanX & 15) + (noSpecial ? 1 : 0)) : 0;
        final int lx1 = (cx == cx1) ? ((maxScanX & 15) - (noSpecial ? 1 : 0)) : 15;
        final int lz0 = (cz == cz0) ? ((minScanZ & 15) + (noSpecial ? 1 : 0)) : 0;
        final int lz1 = (cz == cz1) ? ((maxScanZ & 15) - (noSpecial ? 1 : 0)) : 15;
        final int ly0 = (cy == cy0) ? ((minScanY & 15) + (noSpecial ? 1 : 0)) : 0;
        final int ly1 = (cy == cy1) ? ((maxScanY & 15) - (noSpecial ? 1 : 0)) : 15;
        if (lx0 > lx1 || lz0 > lz1 || ly0 > ly1) {
            return false;
        }
        final BlockPos.MutableBlockPos pos = new BlockPos.MutableBlockPos();
        for (int ly = ly0; ly <= ly1; ++ly) {
            final int y = ly | (cy << 4);
            for (int lz = lz0; lz <= lz1; ++lz) {
                final int z = lz | (cz << 4);
                for (int lx = lx0; lx <= lx1; ++lx) {
                    final int x = lx | (cx << 4);
                    int ring;
                    if (special) {
                        ring = ((x == minScanX || x == maxScanX) ? 1 : 0)
                             + ((y == minScanY || y == maxScanY) ? 1 : 0)
                             + ((z == minScanZ || z == maxScanZ) ? 1 : 0);
                    } else {
                        ring = 0;
                    }
                    if (ring == 3) {
                        continue;
                    }
                    final BlockState state = states.get(lx | (lz << 4) | (ly << 8));
                    if (((CollisionBlockState) state).moonrise$emptyContextCollisionShape()) {
                        continue;
                    }
                    VoxelShape shape =
                        ((CollisionBlockState) state).moonrise$getConstantContextCollisionShape();
                    if (ring == 1) {
                        if (!state.hasLargeCollisionShape()) {
                            continue;
                        }
                    } else if (ring == 2) {
                        if (state.getBlock() != Blocks.MOVING_PISTON) {
                            continue;
                        }
                    }
                    pos.set(x, y, z);
                    if (useEntityCollisionShape) {
                        shape = context.getCollisionShape(state, level, pos);
                    } else if (shape == null) {
                        shape = state.getCollisionShape(level, pos, context);
                    }
                    final AABB single =
                        ((CollisionVoxelShape) shape).moonrise$getSingleAABBRepresentation();
                    if (single != null) {
                        if (!intersectsMoved(box, single, x, y, z)) {
                            continue;
                        }
                        if (filter != null && !filter.test(state, pos)) {
                            continue;
                        }
                        if (checkOnly) {
                            return true;
                        }
                        collided = true;
                        boxesAABB.add(single.move(x, y, z));
                    } else {
                        if (shape.isEmpty()) {
                            continue;
                        }
                        final VoxelShape moved = shape.move(x, y, z);
                        if (!CollisionUtil.voxelShapeIntersectNoEmpty(moved, box)) {
                            continue;
                        }
                        if (filter != null && !filter.test(state, pos)) {
                            continue;
                        }
                        if (checkOnly) {
                            return true;
                        }
                        collided = true;
                        shapesVoxel.add(moved);
                    }
                }
            }
        }
        return collided;
    }
}
