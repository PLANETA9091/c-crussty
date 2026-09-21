package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import ca.spottedleaf.moonrise.patches.block_counting.BlockCountingChunkSection;
import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil;
import ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.LazyEntityCollisionContext;
import ca.spottedleaf.moonrise.patches.collisions.block.CollisionBlockState;
import ca.spottedleaf.moonrise.patches.collisions.shape.CollisionVoxelShape;
import it.unimi.dsi.fastutil.floats.FloatArraySet;
import it.unimi.dsi.fastutil.floats.FloatArrays;
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
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.VoxelShape;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;
import java.util.function.BiPredicate;

/**
 * COLLIDE-SWEEP v1 (TASK-400-F, vector collidesweep) — swept/batched
 * block-collision data plane for the movement lane (collide 8.68% java
 * @150k x region_threads=4).
 *
 * Upstream lineage (RESEARCH-F.md):
 *   - Lithium PR #84 (merged 2020-08-03): chunk sections counting oversized
 *     blocks -> fewer blocks checked around a moving entity (the
 *     section-level pre-gate this port walks verbatim as
 *     BlockCountingChunkSection.moonrise$hasSpecialCollidingBlocks);
 *   - Lithium PR #83: shape cache in collision code (in this kernel:
 *     CollisionBlockState.moonrise$getConstantContextCollisionShape — the
 *     cached per-state shape this port reads once per block and dedupes the
 *     WORK of: one cached unmoved single-AABB representation, offset-tested,
 *     materialized (AABB.move allocation) ONLY on an intersect hit);
 *   - Lithium EntityMixin.collide -> ChunkAwareBlockCollisionSweeperVoxelShape
 *     (chunk-aware sweep on the same patch site: Entity.collide(Vec3));
 *   - Paper 322970283426 "Implement JellySquid's Entity Collision optimisations
 *     patch", aff206ef815b "Optimize WorldBorder collision checks and air",
 *     51bc97bd9037 "Optimize Small Entity Movement";
 *   - deployed upstream of this repo: Moonrise CollisionUtil (the
 *     getCollisionsForBlocksOrWorldBorder body this class mirrors bit-exact).
 *
 * SWEEP DELTA vs the Moonrise scan (the only behavioral-neutral change):
 *   the single-AABB path vanilla order is
 *       moved = aabb.move(x,y,z);           // ALLOCATES for every candidate
 *       hit   = voxelShapeIntersect(box, moved);
 *       if (hit) boxes.add(moved); ...
 *   this port computes the six moved bounds (the SAME six double adds
 *   AABB.move performs), runs the SAME dcmpg/dcmpl epsilon ladder of
 *   CollisionUtil.voxelShapeIntersect (disassembly-mirrored polarity), and
 *   allocates the moved AABB ONLY on the hit path (where vanilla allocates
 *   it too). At ~150k movers the miss path dominates, so the per-block
 *   candidate allocation (hundreds of thousands .. millions per tick)
 *   disappears. voxel-path (stairs/slabs/fences): shape.move +
 *   voxelShapeIntersectNoEmpty stay vanilla verbatim.
 *
 * COLLIDE PORT: bit-exact scalar mirror of the private Entity.collide(Vec3)
 * body (TravelDietOps v2a contract lineage: thread-local scratch AABB
 * slots + 4 scratch ArrayLists + FloatArraySet, cleared before every use,
 * never read before fully written, never carried between calls; products —
 * Vec3/float[] — stay allocations), with the block/worldborder scans
 * served by THIS class's swept pipeline. calculateStepHeights is the
 * verbatim mirror of the private static Entity.calculateStepHeights
 * (FloatArraySet scratch, product float[] + unstableSort vanilla as-is).
 *
 * PARITY CONTRACT (empty flag = vanilla by construction; armed flag =
 * identical visitation order, identical filters/count50 ring logic,
 * identical double operation order, identical list contents):
 *   chunkZ -> chunkX -> sectionY -> y -> z -> x; ring filter
 *   (1 face: hasLargeCollisionShape, 2 faces: MOVING_PISTON, 3: skip);
 *   flags 1/2/4/8 (load-chunks / unloaded-box / worldborder / check-only).
 *
 * NO nested classfiles (S7-163 law): scratch is an Object[] in a
 * ThreadLocal created via a non-capturing method reference to a private
 * static; no anonymous classes; no capturing lambdas.
 */
public final class CollideSweepOps {

    private CollideSweepOps() {}

    // ---- scratch slots (single classfile law: Object[], no holder class) ----
    // 0..2 = AABB scratch (0: expandTowards box, 1: step box17, 2: step box18),
    // 3..6 = ArrayList scratch (3: shapesVoxel, 4: boxesAABB, 5: hard, 6: step shapes),
    // 7 = FloatArraySet scratch.
    private static final int SCRATCH_SIZE = 8;

    private static final java.lang.ThreadLocal<Object[]> SCRATCH =
        java.lang.ThreadLocal.withInitial(CollideSweepOps::freshScratch);

    private static Object[] freshScratch() {
        Object[] s = new Object[SCRATCH_SIZE];
        for (int i = 0; i < 3; ++i) {
            s[i] = new AABB(0.0D, 0.0D, 0.0D, 0.0D, 0.0D, 0.0D);
        }
        for (int i = 3; i < 7; ++i) {
            s[i] = new ArrayList<Object>();
        }
        s[7] = new FloatArraySet();
        return s;
    }

    private static AABB box(final int slot) {
        return (AABB) SCRATCH.get()[slot];
    }

    @SuppressWarnings("unchecked")
    private static List<Object> list(final int slot) {
        List<Object> l = (List<Object>) SCRATCH.get()[slot];
        l.clear();
        return l;
    }

    private static FloatArraySet fset() {
        FloatArraySet s = (FloatArraySet) SCRATCH.get()[7];
        s.clear();
        return s;
    }

    // ---- Unsafe offsets (SkipStoreOps/TravelDietOps precedent) ----
    private static final sun.misc.Unsafe UNSAFE;
    private static final long OFF_MINX, OFF_MINY, OFF_MINZ, OFF_MAXX, OFF_MAXY, OFF_MAXZ;

    static {
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            OFF_MINX = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minX"));
            OFF_MINY = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minY"));
            OFF_MINZ = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("minZ"));
            OFF_MAXX = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxX"));
            OFF_MAXY = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxY"));
            OFF_MAXZ = UNSAFE.objectFieldOffset(AABB.class.getDeclaredField("maxZ"));
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /**
     * Bit-exact mirror of AABB.expandTowards(DDD) writing into dst
     * (TravelDietOps contract: per axis d<0 -> min+=d; d>0 -> max+=d;
     * javac polarity dcmpg/ifge + dcmpl/ifle; NaN falls through).
     */
    private static AABB expandInto(final AABB dst, final AABB src,
                                   final double dx, final double dy, final double dz) {
        double minX = src.minX;
        double minY = src.minY;
        double minZ = src.minZ;
        double maxX = src.maxX;
        double maxY = src.maxY;
        double maxZ = src.maxZ;
        if (dx < 0.0D) {
            minX += dx;
        } else if (dx > 0.0D) {
            maxX += dx;
        }
        if (dy < 0.0D) {
            minY += dy;
        } else if (dy > 0.0D) {
            maxY += dy;
        }
        if (dz < 0.0D) {
            minZ += dz;
        } else if (dz > 0.0D) {
            maxZ += dz;
        }
        sun.misc.Unsafe u = UNSAFE;
        u.putDouble(dst, OFF_MINX, minX);
        u.putDouble(dst, OFF_MINY, minY);
        u.putDouble(dst, OFF_MINZ, minZ);
        u.putDouble(dst, OFF_MAXX, maxX);
        u.putDouble(dst, OFF_MAXY, maxY);
        u.putDouble(dst, OFF_MAXZ, maxZ);
        return dst;
    }

    /** Bit-exact mirror of AABB.move(DDD) writing into dst (pure adds). */
    private static AABB moveInto(final AABB dst, final AABB src,
                                 final double dx, final double dy, final double dz) {
        sun.misc.Unsafe u = UNSAFE;
        u.putDouble(dst, OFF_MINX, src.minX + dx);
        u.putDouble(dst, OFF_MINY, src.minY + dy);
        u.putDouble(dst, OFF_MINZ, src.minZ + dz);
        u.putDouble(dst, OFF_MAXX, src.maxX + dx);
        u.putDouble(dst, OFF_MAXY, src.maxY + dy);
        u.putDouble(dst, OFF_MAXZ, src.maxZ + dz);
        return dst;
    }

    /**
     * SWEEP CORE: bit-exact decomposition of
     * {@code CollisionUtil.voxelShapeIntersect(box, aabb.move(x, y, z))}.
     *
     * The six moved bounds are the SAME six double adds AABB.move performs
     * (field + delta, no reassociation); the comparisons reproduce the
     * disassembled ladder of CollisionUtil.voxelShapeIntersect(AABB, AABB)
     * exactly:
     *   (box.min - moved.max) dcmpg -1.0E-7 ; ifge false  -> require < -1.0E-7
     *   (box.max - moved.min) dcmpl  1.0E-7 ; ifle false  -> require >  1.0E-7
     * (NaN: dcmpg -> +1 => ifge taken => false; dcmpl -> -1 => ifle taken =>
     * false; Java {@code <}/{@code >} on NaN is false — same polarity).
     * Allocation-free on the miss path.
     */
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

    /**
     * Receiver-prepended body redirect target of the private
     * Entity.collide(Vec3) — scalar mirror of the kernel body
     * (javap offsets documented inline, TravelDietOps v2a lineage) with the
     * block/worldborder scans served by the swept pipeline below.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static Vec3 collide(final Entity self, final Vec3 movement) {
        // off 0..52: zero checks (dcmpl ladder; NaN -> false, as vanilla)
        boolean xZero = movement.x == 0.0D;
        boolean yZero = movement.y == 0.0D;
        boolean zZero = movement.z == 0.0D;
        if (xZero && yZero && zZero) {
            return movement; // off 46..56
        }
        // off 57..79: bb + shapesVoxel/boxesAABB scratch lists
        AABB bb = self.getBoundingBox();
        List<VoxelShape> list6 = (List) list(3);
        List<AABB> list7 = (List) list(4);
        // off 81..129: query box: cut branch (vertical-only) or expandTowards
        AABB box8;
        if (xZero && zZero) {
            box8 = movement.y < 0.0D
                ? CollisionUtil.cutDownwards(bb, movement.y)
                : CollisionUtil.cutUpwards(bb, movement.y);
        } else {
            box8 = expandInto(box(0), bb, movement.x, movement.y, movement.z);
        }
        // off 131..157: hard entity collisions
        List<AABB> list9 = (List) list(5);
        CollisionUtil.getEntityHardCollisions((Level) self.level(), self, box8, list9, 0, null);
        // off 158..177: swept block/worldborder scan
        blockCollisions((Level) self.level(), self, box8, list6, list7, 4, null);
        // off 178..187: list7.addAll(list9)
        list7.addAll(list9);
        // off 188..198: per-axis resolution (vanilla static, bit-exact)
        Vec3 vec10 = CollisionUtil.performCollisions(movement, bb, list6, list7);
        // off 200..258: dcmpl ladders -> diffs (NaN -> true, as vanilla dcmpl)
        boolean xDiff = vec10.x != movement.x;
        boolean yDiff = vec10.y != movement.y;
        boolean zDiff = vec10.z != movement.z;
        // off 260..279: down = yDiff && movement.y dcmpg 0.0 < 0
        boolean down = yDiff && movement.y < 0.0D;
        // off 281..316: !(down || onGround) -> return; !(xDiff || zDiff) -> return
        if (!(down || self.onGround)) {
            return vec10;
        }
        if (!xDiff && !zDiff) {
            return vec10;
        }
        // off 303..318: step = (double) maxUpStep(); dcmpg ifgt (NaN continues)
        double step = (double) self.maxUpStep();
        if (step > 0.0D) {
            // fall through to step-up ladder
        } else {
            return vec10;
        }
        // off 319..341: down ? bb.move(0, vec10.y, 0) : bb
        AABB box17 = down ? moveInto(box(1), bb, 0.0D, vec10.y, 0.0D) : bb;
        // off 343..358: box18 = box17.expandTowards(mx, step, mz)
        AABB box18 = expandInto(box(2), box17, movement.x, step, movement.z);
        // off 360..375: if (!down) box18 = box18.expandTowards(0, -9.999999747378752E-6, 0)
        if (!down) {
            box18 = expandInto(box(2), box18, 0.0D, -9.999999747378752E-6D, 0.0D);
        }
        // off 377..409: list19 fresh; list20 = list9 (alias); second sweep
        List<VoxelShape> list19 = (List) list(6);
        List<AABB> list20 = list9;
        blockCollisions((Level) self.level(), self, box18, list19, list20, 4, null);
        // off 410..428: steps = calculateStepHeights(box17, list19, list20, (f)step, (f)vec10.y)
        float[] steps = calculateStepHeights(box17, list19, list20, (float) step, (float) vec10.y);
        // off 430..517: candidate ladder
        for (int i = 0; i < steps.length; ++i) {
            float h = steps[i];
            Vec3 cand = new Vec3(movement.x, (double) h, movement.z);
            Vec3 res = CollisionUtil.performCollisions(cand, box17, list19, list20);
            if (res.horizontalDistanceSqr() > vec10.horizontalDistanceSqr()) {
                // off 495..513: res.add(0, box17.minY - bb.minY, 0) — product Vec3
                return res.add(0.0D, box17.minY - bb.minY, 0.0D);
            }
        }
        // off 520: return vec10
        return vec10;
    }

    /**
     * Verbatim mirror of the private static Entity.calculateStepHeights
     * (TravelDietOps contract, calc ladder: fcmpl/fcmpg polarity, root
     * coordinates + offsetY via CollisionVoxelShape, product float[] +
     * FloatArrays.unstableSort vanilla as-is).
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static float[] calculateStepHeights(final AABB box, final List<VoxelShape> shapes,
                                                final List<AABB> aabbs,
                                                final float stepF, final float currY) {
        FloatArraySet set = fset();
        outer:
        for (int i = 0; i < shapes.size(); ++i) {
            final VoxelShape shape = shapes.get(i);
            final CollisionVoxelShape cs = (CollisionVoxelShape) shape;
            final double[] rootY = cs.moonrise$rootCoordinatesY();
            final double offY = cs.moonrise$offsetY();
            for (int j = 0; j < rootY.length; ++j) {
                final double d = rootY[j] + offY;
                final float f = (float) (d - box.minY);
                if (f > stepF) {
                    continue outer;
                }
                if (f >= 0.0F) {
                    if (f != currY) {
                        set.add(f);
                    }
                }
            }
        }
        for (int i = 0; i < aabbs.size(); ++i) {
            final AABB a = aabbs.get(i);
            final float fMin = (float) (a.minY - box.minY);
            final float fMax = (float) (a.maxY - box.minY);
            if (fMin >= 0.0F && fMin != currY && !(fMin > stepF)) {
                set.add(fMin);
            }
            if (fMax >= 0.0F && fMax != currY && !(fMax > stepF)) {
                set.add(fMax);
            }
        }
        final float[] out = set.toFloatArray();
        FloatArrays.unstableSort(out);
        return out;
    }

    /**
     * Body redirect target of the static
     * CollisionUtil.getCollisionsForBlocksOrWorldBorder — the swept/batched
     * block scan (bit-exact port, identical visitation order and filters;
     * the ONLY delta is the allocation-free single-AABB miss path).
     *
     * Descriptor contract (redirect_static_method_body_to_static):
     * (Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;
     *   Lnet/minecraft/world/phys/AABB;Ljava/util/List;Ljava/util/List;I
     *   Ljava/util/function/BiPredicate;)Z
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static boolean blockCollisions(final Level level, final Entity entity, final AABB box,
                                          final List<VoxelShape> shapesVoxel,
                                          final List<AABB> boxesAABB, final int flags,
                                          final BiPredicate<BlockState, BlockPos> filter) {
        // off 0..13: checkOnly = (flags & 8) != 0
        final boolean checkOnly = (flags & 8) != 0;
        // off 15..16: collided = false
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
        // off 197..204: mutable pos (vanilla ctor; S7-133 splice is upstream of this body)
        final BlockPos.MutableBlockPos pos = new BlockPos.MutableBlockPos();
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

        // off 289..1066: chunkZ -> chunkX -> sectionY -> y -> z -> x (order fixed)
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
                    for (int ly = ly0; ly <= ly1; ++ly) {
                        final int y = ly | (cy << 4);
                        for (int lz = lz0; lz <= lz1; ++lz) {
                            final int z = lz | (cz << 4);
                            for (int lx = lx0; lx <= lx1; ++lx) {
                                final int x = lx | (cx << 4);
                                // off 658..734: outer-ring gate (Lithium PR #84 semantics)
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
                                // off 737..747: palette index = x | z<<4 | y<<8
                                final BlockState state =
                                    (BlockState) states.get(lx | (lz << 4) | (ly << 8));
                                // off 749..759: cached empty gate (Lithium PR #83 lineage)
                                if (((CollisionBlockState) state).moonrise$emptyContextCollisionShape()) {
                                    continue;
                                }
                                // off 762..769: cached constant shape (may be null)
                                VoxelShape shape =
                                    ((CollisionBlockState) state).moonrise$getConstantContextCollisionShape();
                                // off 771..804: ring filters
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
                                // off 819..856: shape resolution (entity path minecart-gated)
                                if (useEntityCollisionShape) {
                                    shape = context.getCollisionShape(state, level, pos);
                                } else if (shape == null) {
                                    shape = state.getCollisionShape(level, pos, context);
                                }
                                final AABB single =
                                    ((CollisionVoxelShape) shape).moonrise$getSingleAABBRepresentation();
                                if (single != null) {
                                    // off 867..941: single-AABB path — SWEEP: offset-intersect
                                    // first (bit-exact, no allocation), materialize on hit only.
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
                                    // off 945..1025: voxel path — vanilla verbatim
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
                }
            }
        }
        // off 1064..1066
        return collided;
    }
}
