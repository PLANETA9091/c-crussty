package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.level.material.FlowingFluid;
import net.minecraft.world.level.material.Fluids;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TASK-405-B (cmp405_fluidrust): fluid→Rust bulk push plane.
 *
 * Whole-site replacement for the TWO kernel call-sites of
 * Entity.updateFluidHeightAndDoFluidPushing (the WATER wrapper
 * updateInWaterStateAndDoWaterCurrentPushing offset 39 and the LAVA wrapper
 * updateInWaterStateAndDoFluidPushing offset 41 — the only two call sites in
 * the kernel, javap census). The retarget is receiver-prepended
 * (Entity,TagKey,double)Z, length-preserving, computed rust-side from pristine
 * bytes (src/classfile.rs patch_fluid_rust) and served by the SINGLE
 * entity_compose Entity pipeline.
 *
 * ARCHITECTURE (honest vertical slice: the PUSH half of the fluid lane —
 * updateFluidHeightAndDoFluidPushing + FlowingFluid.getFlow; fluid spread
 * stays vanilla):
 *
 *   - NEGATIVE-ONLY FAST PLANE (0 JNI): per-call gather on ThreadLocal
 *     reused flat LevelChunkSection coverage (expanded ±1 cell/±1 column for
 *     neighbor reads; sections re-fetched EVERY call — no cross-call world
 *     state, law 5) with a per-call FluidState identity verdict table
 *     (EMPTY/TAGGED/OTHER — kills the per-cell isEmpty+is(tag) registry
 *     lookups). d0 float arithmetic runs java-side exactly like vanilla
 *     ((float)y + getHeight -> f2d - minY). A pure-negative call (or
 *     !isPushedByFluid) reproduces the vanilla tail bit-in-bit:
 *     fluidHeight.put(tag, maxDepth); flowAcc never left Vec3.ZERO
 *     (reference identity) -> return inFluid.
 *
 *   - POSITIVE BULK (ONE JNI per call, only cells that reach getFlow):
 *     java gathers per-cell flow INPUTS in scan order (x→y→z; per direction
 *     in Direction.Plane.HORIZONTAL order): affectsFlow (public replica:
 *     isEmpty || type.isSame), neighbor ownHeight, blocksMotion, below
 *     verdict+height, stepX/stepZ, and for FALLING cells the solidFace/
 *     solidFaceAbove verdicts (public replica of the protected
 *     FlowingFluid.isSolidFace incl. IceBlock/UP clauses). ONE
 *     fluidPushBatch call computes rust-side the exact FlowingFluid.getFlow
 *     vectorization + the push accumulation (running maxDepth 0.4 branch,
 *     Vec3 scale/add/normalize op order, ZERO identity semantics via the
 *     touched flag) and returns (accX,accY,accZ,touched,flowCount). The java
 *     tail then applies fluidHeight.put(tag,maxDepth), the 1.0/flowCount
 *     scale, the Player normalize exemption, the 0.003/0.0045000000000000005
 *     floor and setDeltaMovement — vanilla bytecode order preserved.
 *
 *   - SLOW REPLICA (fail-closed): any Throwable, any uncovered/unloaded
 *     neighbor column, any native error ≠ 0 falls back to a faithful
 *     java replica of the vanilla body (per-call fresh flat array, vanilla
 *     getFlow via the kernel API — FluidState.getFlow(level,pos)). No guard
 *     cache, no memoization of any world state (law 5). lastLavaContact is
 *     set during BOTH gather and slow for every tagged cell in identical
 *     scan order (vanilla sets it BEFORE the d0 filter) — a fallback after a
 *     partial gather re-derives the same final value.
 *
 * CONCURRENCY: region-parallel ticking — ALL per-call state is ThreadLocal;
 * natives are stateless array readers; `broken`/`nativeOk` are volatile.
 *
 * FAIL-CLOSED: ENABLED (env CRUSSTY_LEVER_FLAG == "cmp405_fluidrust", STRICT
 * eq) && nativeOk (probe magic) && !broken. Empty/foreign lever flag → the
 * rust side never serves the patch (entity_compose stage dormant) and this
 * class is never invoked (vanilla bit-in-bit).
 */
public final class FluidRustOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp405_fluidrust");
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x464C; // "FL"

    /** Return codes: 0 ok; -1 ERR_STRUCT (permanent disarm); -2 ERR_RANGE (per-call vanilla). */
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/fluid_rust.rs, RegisterNatives after define) ----
    private static native int fluidProbe();
    /**
     * Bulk push computation. d0s: one d0 per flow-eligible cell (scan order).
     * fls: 17 floats per cell: [ownHeight, nH0..3, belowH0..3, stepX0..3, stepZ0..3].
     * flags: one int per cell, bit i (i=0..3 direction order):
     *   bit0+4d affects, bit4+4d blocksMotion, bit8+4d belowAffects,
     *   bit12+4d solidFace, bit16+4d solidFaceAbove, bit20 falling.
     * out: 5 doubles = [accX, accY, accZ, touched, flowCount].
     * Returns 0 ok; ERR_STRUCT; ERR_RANGE.
     */
    private static native int fluidPushBatch(int n, double[] d0s, float[] fls, int[] flags, double[] out);

    private static volatile boolean nativeOk;
    private static volatile boolean broken;

    private FluidRustOps() {}

    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (FluidRustOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = fluidProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Gate for rust-side diagnostics. */
    public static boolean armed() {
        return ENABLED && !broken && probeOnce();
    }

    // ------------------------------------------------------------------
    // Thread-confined scratch (S7-132 discipline: grow-only, no steady-state
    // allocation; region ticking is one-thread-per-region and never re-enters
    // the same thread).
    // ------------------------------------------------------------------
    private static final ThreadLocal<LevelChunkSection[][]> FLAT_TL =
            ThreadLocal.withInitial(() -> new LevelChunkSection[8][]);
    private static final ThreadLocal<Object[]> VKEY_TL =
            ThreadLocal.withInitial(() -> new Object[32]);
    private static final ThreadLocal<byte[]> VVAL_TL =
            ThreadLocal.withInitial(() -> new byte[32]);
    private static final ThreadLocal<double[]> D0_TL =
            ThreadLocal.withInitial(() -> new double[8]);
    private static final ThreadLocal<float[]> FL_TL =
            ThreadLocal.withInitial(() -> new float[17 * 8]);
    private static final ThreadLocal<int[]> FLAG_TL =
            ThreadLocal.withInitial(() -> new int[8]);
    private static final ThreadLocal<double[]> OUT_TL =
            ThreadLocal.withInitial(() -> new double[5]);
    private static final ThreadLocal<BlockPos.MutableBlockPos> MPOS_TL =
            ThreadLocal.withInitial(BlockPos.MutableBlockPos::new);

    private static LevelChunkSection[][] tlFlat(int cols) {
        LevelChunkSection[][] f = FLAT_TL.get();
        if (f.length < cols) {
            f = new LevelChunkSection[Math.max(cols, f.length * 2)][];
            FLAT_TL.set(f);
        }
        return f;
    }

    private static double[] tlD0(int cells) {
        double[] a = D0_TL.get();
        if (a.length < cells) {
            a = new double[Math.max(cells, a.length * 2)];
            D0_TL.set(a);
        }
        return a;
    }

    private static float[] tlFl(int cells) {
        float[] a = FL_TL.get();
        if (a.length < cells) {
            a = new float[Math.max(17 * cells, a.length * 2)];
            FL_TL.set(a);
        }
        return a;
    }

    private static int[] tlFlags(int cells) {
        int[] a = FLAG_TL.get();
        if (a.length < cells) {
            a = new int[Math.max(cells, a.length * 2)];
            FLAG_TL.set(a);
        }
        return a;
    }

    // ------------------------------------------------------------------
    // Per-call FluidState identity verdict table (open-addressed, 2^5 slots;
    // identityHashCode collisions → linear probe). 1=EMPTY 2=TAGGED 3=OTHER.
    // Live ONLY for the duration of one call (law 5: no cross-tick state).
    // ------------------------------------------------------------------
    private static byte verdictOf(FluidState fs, TagKey<Fluid> tag, Object[] keys, byte[] vals) {
        int slot = System.identityHashCode(fs) & 31;
        for (int probe = 0; probe < 32; probe++) {
            int s = (slot + probe) & 31;
            Object k = keys[s];
            if (k == null) {
                keys[s] = fs;
                byte v = (byte) (fs.isEmpty() ? 1 : (fs.is(tag) ? 2 : 3));
                vals[s] = v;
                return v;
            }
            if (k == fs) {
                return vals[s];
            }
        }
        return (byte) (fs.isEmpty() ? 1 : (fs.is(tag) ? 2 : 3));
    }

    // ------------------------------------------------------------------
    // Entry (the retargeted body of both kernel wrapper call-sites).
    // ------------------------------------------------------------------
    public static boolean updateFluidHeightAndDoFluidPushing(Entity self, TagKey<Fluid> tag, double speed) {
        if (!ENABLED || broken) {
            return slow(self, tag, speed);
        }
        try {
            if (self.touchingUnloadedChunk()) {
                return false;
            }
            if (!probeOnce()) {
                return slow(self, tag, speed);
            }
            Object[] vKeys = VKEY_TL.get();
            byte[] vVals = VVAL_TL.get();
            java.util.Arrays.fill(vKeys, null);

            FastResult r = fast(self, tag, vKeys, vVals);
            if (r == null) {
                return slow(self, tag, speed); // uncovered column / gather problem — per-call vanilla
            }
            if (r.touched) {
                // Vanilla tail, bytecode order (javap 548..677): put -> identity
                // check -> scale(1.0/count) -> Player normalize exemption ->
                // scale(speed) -> floor check -> setDeltaMovement.
                self.fluidHeight.put(tag, r.maxDepth);
                Vec3 acc = new Vec3(r.accX, r.accY, r.accZ);
                acc = acc.scale(1.0D / r.flowCount);
                Vec3 delta = self.getDeltaMovement();
                if (!(self instanceof net.minecraft.world.entity.player.Player)) {
                    acc = acc.normalize();
                }
                acc = acc.scale(speed);
                if (Math.abs(delta.x) < 0.003D && Math.abs(delta.z) < 0.003D
                        && acc.length() < 0.0045000000000000005D) {
                    acc = acc.normalize().scale(0.0045000000000000005D);
                }
                self.setDeltaMovement(delta.add(acc));
                return true;
            }
            // acc never left the Vec3.ZERO reference: vanilla returns inFluid
            // with NO further mutation. put(maxDepth) happens before the check.
            self.fluidHeight.put(tag, r.maxDepth);
            return r.inFluid;
        } catch (Throwable t) {
            if (t instanceof OutOfMemoryError || t instanceof StackOverflowError) {
                throw t;
            }
            return slow(self, tag, speed); // fail-dominant: per-call vanilla semantics
        }
    }

    private static final class FastResult {
        double maxDepth;
        boolean inFluid;
        boolean touched;
        double accX, accY, accZ;
        int flowCount;
    }

    // ------------------------------------------------------------------
    // Fast plane: gather + (positive) single bulk JNI. Returns null when the
    // call must degrade to the slow replica.
    // ------------------------------------------------------------------
    private static FastResult fast(Entity self, TagKey<Fluid> tag, Object[] vKeys, byte[] vVals) {
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001D);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushedBy = self.isPushedByFluid();

        // Flat coverage expanded ±1 cell in x/z (neighbor reads of tagged
        // cells); sections re-fetched EVERY call (fresh world reads).
        int fx0 = (minX - 1) >> 4, fx1 = (maxX + 1) >> 4;
        int fz0 = (minZ - 1) >> 4, fz1 = (maxZ + 1) >> 4;
        int fspanX = fx1 - fx0 + 1;
        int cols = fspanX * (fz1 - fz0 + 1);
        LevelChunkSection[][] flat = tlFlat(cols);
        ChunkSource source = level.getChunkSource();
        for (int cz = fz0; cz <= fz1; cz++) {
            for (int cx = fx0; cx <= fx1; cx++) {
                // load=false — matching the vanilla box fetch; an unloaded
                // neighbor column degrades to the slow replica (vanilla's
                // getFlow level-reads would load it — slow covers that).
                ChunkAccess ch = source.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (ch == null) {
                    return null;
                }
                flat[cx - fx0 + fspanX * (cz - fz0)] = ch.getSections();
            }
        }
        int maxSecY = (WorldUtil.getMaxSection(level) << 4) | 15;
        BlockPos.MutableBlockPos mpos = MPOS_TL.get();

        FastResult r = new FastResult();
        boolean lavaTag = tag == FluidTags.LAVA;

        int cells = 0;
        double[] d0s = tlD0(8);
        float[] fls = tlFl(8);
        int[] flags = tlFlags(8);

        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) - fx0 + fspanX * ((z >> 4) - fz0)][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    byte v = verdictOf(fs, tag, vKeys, vVals);
                    if (v != 2) {
                        continue; // EMPTY or OTHER — vanilla `isEmpty() || !is(tag)` skip
                    }
                    mpos.set(x, y, z);
                    if (lavaTag) {
                        self.lastLavaContact = mpos.immutable(); // vanilla: BEFORE the d0 filter
                    }
                    float ownH = fs.getHeight(level, mpos); // FluidState.getHeight API (javap @431)
                    double d0 = (double) ((float) y + ownH) - box.minY;
                    if (d0 < 0.0D) {
                        continue;
                    }
                    r.inFluid = true;
                    r.maxDepth = Math.max(r.maxDepth, d0);
                    if (!pushedBy) {
                        continue;
                    }
                    if (cells == d0s.length) {
                        d0s = tlD0(cells + 1);
                        fls = tlFl(cells + 1);
                        flags = tlFlags(cells + 1);
                    }
                    d0s[cells] = d0;
                    // ---- per-cell flow inputs (getFlow javap contract) ----
                    Fluid ownType = fs.getType();
                    boolean falling = fs.getValue(FlowingFluid.FALLING);
                    int flBase = cells * 17;
                    fls[flBase] = ownH;
                    int flag = falling ? (1 << 20) : 0;
                    int di = 0;
                    for (Direction dir : Direction.Plane.HORIZONTAL) {
                        int nx = x + dir.getStepX();
                        int nz = z + dir.getStepZ();
                        FluidState n = fluidStateAt(flat, fspanX, fx0, fz0, minSection, maxSecY, nx, y, nz);
                        boolean affects = n.isEmpty() || n.getType().isSame(ownType);
                        float nH = affects ? n.getOwnHeight() : 0.0F;
                        boolean blocksMotion = false;
                        boolean belowAffects = false;
                        float belowH = 0.0F;
                        if (affects && nH == 0.0F) {
                            BlockState nbs = blockStateAt(flat, fspanX, fx0, fz0, minSection, maxSecY, nx, y, nz);
                            blocksMotion = nbs.blocksMotion();
                            if (!blocksMotion) {
                                FluidState b = fluidStateAt(flat, fspanX, fx0, fz0, minSection, maxSecY, nx, y - 1, nz);
                                belowAffects = b.isEmpty() || b.getType().isSame(ownType);
                                if (belowAffects) {
                                    belowH = b.getOwnHeight();
                                }
                            }
                        }
                        boolean solid = false;
                        boolean solidAbove = false;
                        if (falling) {
                            solid = isSolidFaceReplica(flat, fspanX, fx0, fz0, minSection, maxSecY, level, nx, y, nz, ownType, dir);
                            solidAbove = isSolidFaceReplica(flat, fspanX, fx0, fz0, minSection, maxSecY, level, nx, y + 1, nz, ownType, dir);
                        }
                        if (affects) {
                            flag |= 1 << di;
                        }
                        if (blocksMotion) {
                            flag |= 1 << (4 + di);
                        }
                        if (belowAffects) {
                            flag |= 1 << (8 + di);
                        }
                        if (solid) {
                            flag |= 1 << (12 + di);
                        }
                        if (solidAbove) {
                            flag |= 1 << (16 + di);
                        }
                        fls[flBase + 1 + di] = nH;
                        fls[flBase + 5 + di] = belowH;
                        fls[flBase + 9 + di] = (float) dir.getStepX();
                        fls[flBase + 13 + di] = (float) dir.getStepZ();
                        di++;
                    }
                    flags[cells] = flag;
                    cells++;
                }
            }
        }

        if (cells > 0) {
            double[] out = OUT_TL.get();
            int rc = fluidPushBatch(cells, d0s, fls, flags, out);
            if (rc == ERR_STRUCT) {
                broken = true;
                return null;
            }
            if (rc != 0) {
                return null; // ERR_RANGE — per-call vanilla
            }
            r.accX = out[0];
            r.accY = out[1];
            r.accZ = out[2];
            r.touched = out[3] != 0.0D;
            r.flowCount = (int) out[4];
        }
        return r;
    }

    /** FluidState at (x,y,z) from the flat coverage — exact
     *  LevelChunk.getFluidState(III) semantics (javap 526..): out-of-range
     *  section index or hasOnlyAir section → EMPTY. */
    private static FluidState fluidStateAt(LevelChunkSection[][] flat, int fspanX, int fx0, int fz0,
            int minSection, int maxSecY, int x, int y, int z) {
        if (y < (minSection << 4) || y > maxSecY) {
            return Fluids.EMPTY.defaultFluidState();
        }
        LevelChunkSection sec = flat[(x >> 4) - fx0 + fspanX * ((z >> 4) - fz0)][(y >> 4) - minSection];
        if (sec == null || sec.hasOnlyAir()) {
            return Fluids.EMPTY.defaultFluidState();
        }
        return ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
    }

    private static BlockState blockStateAt(LevelChunkSection[][] flat, int fspanX, int fx0, int fz0,
            int minSection, int maxSecY, int x, int y, int z) {
        if (y < (minSection << 4) || y > maxSecY) {
            return net.minecraft.world.level.block.Blocks.AIR.defaultBlockState();
        }
        LevelChunkSection sec = flat[(x >> 4) - fx0 + fspanX * ((z >> 4) - fz0)][(y >> 4) - minSection];
        if (sec == null || sec.hasOnlyAir()) {
            return net.minecraft.world.level.block.Blocks.AIR.defaultBlockState();
        }
        return (BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8));
    }

    /** Faithful public-API replica of the protected FlowingFluid.isSolidFace
     *  (javap 317..): NOT fsType.isSame(own) && dir != UP &&
     *  !(block instanceof IceBlock) && bs.isFaceSturdy(level, pos, dir). */
    private static boolean isSolidFaceReplica(LevelChunkSection[][] flat, int fspanX, int fx0, int fz0,
            int minSection, int maxSecY, Level level, int x, int y, int z, Fluid ownType, Direction dir) {
        BlockState bs = blockStateAt(flat, fspanX, fx0, fz0, minSection, maxSecY, x, y, z);
        FluidState fs = fluidStateAt(flat, fspanX, fx0, fz0, minSection, maxSecY, x, y, z);
        if (fs.getType().isSame(ownType)) {
            return false;
        }
        if (dir == Direction.UP) {
            return false;
        }
        if (bs.getBlock() instanceof net.minecraft.world.level.block.IceBlock) {
            return false;
        }
        return bs.isFaceSturdy(level, BlockPos.containing(x, y, z), dir);
    }

    // ------------------------------------------------------------------
    // SLOW replica: faithful vanilla body (javap 13225..13560) minus the
    // guard cache — used when the fast plane is unavailable. Per-call fresh
    // flat array (vanilla anewarray shape), vanilla FluidState.getFlow API.
    // ------------------------------------------------------------------
    private static boolean slow(Entity self, TagKey<Fluid> tag, double speed) {
        if (self.touchingUnloadedChunk()) {
            return false;
        }
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001D);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushedBy = self.isPushedByFluid();

        int cx0 = minX >> 4, cx1 = maxX >> 4, cz0 = minZ >> 4, cz1 = maxZ >> 4;
        int spanX = cx1 - cx0 + 1;
        int offset = -(cx0 + spanX * cz0);
        LevelChunkSection[][] flat = new LevelChunkSection[spanX * (cz1 - cz0 + 1)][];
        ChunkSource source = level.getChunkSource();
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                flat[cx + spanX * cz + offset] = source.getChunk(cx, cz, ChunkStatus.FULL, false).getSections();
            }
        }

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0D;
        boolean inFluid = false;
        int flowCount = 0;
        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) + spanX * (z >> 4) + offset][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    mpos.set(x, y, z);
                    if (tag == FluidTags.LAVA) {
                        self.lastLavaContact = mpos.immutable();
                    }
                    double d0 = (double) ((float) y + fs.getHeight(level, mpos)) - box.minY;
                    if (d0 < 0.0D) {
                        continue;
                    }
                    inFluid = true;
                    maxDepth = Math.max(maxDepth, d0);
                    if (!pushedBy) {
                        continue;
                    }
                    flowCount++;
                    Vec3 flow = fs.getFlow(level, mpos);
                    if (maxDepth < 0.4D) {
                        flowAcc = flowAcc.add(flow.scale(maxDepth));
                    } else {
                        flowAcc = flowAcc.add(flow);
                    }
                }
            }
        }
        self.fluidHeight.put(tag, maxDepth);
        if (flowAcc == Vec3.ZERO) {
            return inFluid;
        }
        flowAcc = flowAcc.scale(1.0D / flowCount);
        Vec3 delta = self.getDeltaMovement();
        if (!(self instanceof net.minecraft.world.entity.player.Player)) {
            flowAcc = flowAcc.normalize();
        }
        flowAcc = flowAcc.scale(speed);
        if (Math.abs(delta.x) < 0.003D && Math.abs(delta.z) < 0.003D && flowAcc.length() < 0.0045000000000000005D) {
            flowAcc = flowAcc.normalize().scale(0.0045000000000000005D);
        }
        self.setDeltaMovement(delta.add(flowAcc));
        return true;
    }
}
