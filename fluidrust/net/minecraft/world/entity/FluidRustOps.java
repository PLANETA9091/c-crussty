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
 * TASK-410-B (lever cmp410_fluidsec): fluid→Rust SECTIONAL batch plane —
 * the law-(6) redesign of the TASK-405-B bridge after bleg1 RED
 * (fluid lane 16.72%→71.48% from PER-ENTITY JNI transitions + hot-loop
 * array traffic). The retarget itself is unchanged (the same TWO kernel
 * wrapper call-sites of Entity.updateFluidHeightAndDoFluidPushing — WATER
 * updateInWaterStateAndDoWaterCurrentPushing offset 39 + LAVA
 * updateInWaterStateAndDoFluidPushing offset 41, the only two in the
 * kernel — receiver-prepended (Entity,TagKey,double)Z strict 1+1, served
 * by the SINGLE entity_compose Entity pipeline from pristine bytes).
 *
 * THE K2 CALL-BOUNDARY REDESIGN (one JNI per TICK, not per entity):
 *
 *   1. GATHER (java, ZERO JNI): every wrapper call runs the vanilla cell
 *      scan exactly like the vanilla body (flat section coverage re-fetched
 *      EVERY call, per-call FluidState identity verdict table, d0 float
 *      arithmetic vanilla-side). Inline, bit-exact, exactly as vanilla:
 *      lastLavaContact, fluidHeight.put(tag, maxDepth) (running max of d0 —
 *      plain java max during the scan) and the RETURN value (positive ⇒
 *      true, because vanilla returns inFluid when flowAcc never left the
 *      Vec3.ZERO reference and inFluid is true for every recorded cell;
 *      ZERO-identity ⇒ return inFluid). Positive calls (cells that reach
 *      getFlow) additionally append their per-cell flow INPUTS (17 floats +
 *      flags per cell, same javap contract as before) and ONE record
 *      (entity ref, newEntity flag, player flag, delta, speed, cell range)
 *      into the thread-local TICK QUEUE. No JNI, no delta mutation here.
 *
 *   2. FLUSH (ONE bulk JNI per tick per thread): the first wrapper call
 *      whose level game-time differs from the queue stamp flushes the whole
 *      pending queue. tickTime() bumps gameTime BEFORE the entity phase and
 *      it is constant within one level's entity phase (javap ServerLevel.tick
 *      offset 236 vs 532), so the flush lands once per level phase. A
 *      8192-record valve bounds the queue mid-phase (a mid-phase flush is
 *      still exact: the apply is ordered and the next record re-captures the
 *      applied delta with newEntity=1).
 *
 *   3. RUST (one pass over ALL entities): per record the exact vanilla
 *      accumulation (running maxDepth 0.4 branch, ZERO-identity touched,
 *      flowCount = cellCount) and the exact vanilla tail (scale(1.0/count),
 *      Player normalize exemption, scale(speed), 0.003/0.0045 floor check,
 *      delta.add). The WATER→LAVA delta threading is preserved BIT-IN-BIT:
 *      a record with newEntity=0 continues the previous record of the SAME
 *      entity, so the lava tail sees (deltaW + accW) exactly like vanilla's
 *      second getDeltaMovement(). Outputs are ONE packed array copied out
 *      ONCE (Get*ArrayRegion, no Critical in any loop).
 *
 *   4. APPLY (java, ordered): per record with touched≠0 — the only vanilla
 *      call that is deferred to the flush point — setDeltaMovement(outDelta).
 *      Everything else (return value, fluidHeight, lastLavaContact) already
 *      happened inline at the exact vanilla call site, so the gather-time
 *      entity state is exactly vanilla's; the sole semantic difference of
 *      the whole lever is that the push add lands at the flush point
 *      (start of the entity's next pass) instead of mid-baseTick. Verdicts
 *      (push vector + height) are bit-in-bit vanilla per the oracle tests.
 *
 *   5. FAIL-CLOSED: any gather problem (unloaded neighbor column, native
 *      probe failure, any Throwable in the fast plane) → faithful inline
 *      java replica of the vanilla body (slow — vanilla FluidState.getFlow
 *      API, fresh flat array per call, NO queue). A flush native error ⇒
 *      this one batch is discarded (pushes deferred this tick are lost),
 *      broken=true disarms permanently to the inline slow replica — never a
 *      half-applied batch. No world state crosses calls (law 5): the queue
 *      carries only entity refs + recorded inputs and drains every tick.
 *
 * CONCURRENCY: region-parallel ticking — the queue is thread-confined
 * (records are created and drained by the same ticking thread); natives are
 * stateless; nativeOk/broken are volatile.
 *
 * Fail-closed: ENABLED (env CRUSSTY_LEVER_FLAG == "cmp410_fluidsec", STRICT
 * eq) && nativeOk && !broken. Empty/foreign flag ⇒ the rust side never
 * serves the patch (entity_compose stage dormant) and this class is never
 * invoked (vanilla bit-in-bit).
 */
public final class FluidRustOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp410_fluidsec");
    }

    private static final boolean ENABLED = leverEnabled();

    private static final int PROBE_MAGIC = 0x464C; // "FL"

    /** Return codes: 0 ok; -1 ERR_STRUCT (permanent disarm); -2 ERR_RANGE. */
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    /** Valve: flush mid-phase when the pending queue reaches this many records. */
    private static final int MAX_RECORDS = 8192;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/fluid_rust.rs, RegisterNatives after define) ----
    private static native int fluidProbe();
    /**
     * ONE call per tick per thread. n records; per record (i):
     * metaI[4i..4i+4) = [newEntity(0/1), player(0/1), cellStart, cellCount];
     * metaD[4i..4i+4) = [deltaX, deltaY, deltaZ, speed];
     * cells at [cellStart, cellStart+cellCount): d0s (d0), fls (17 floats:
     * [ownHeight, nH0..3, belowH0..3, stepX0..3, stepZ0..3]), flags (bit i:
     * bit0+4d affects, bit4+4d blocksMotion, bit8+4d belowAffects,
     * bit12+4d solidFace, bit16+4d solidFaceAbove, bit20 falling).
     * out[4i..4i+4) = [outX, outY, outZ, touched(0/1)] — outDelta already
     * includes the exact vanilla tail AND the water→lava delta threading.
     * Returns 0 ok; ERR_STRUCT; ERR_RANGE.
     */
    private static native int fluidBatchTick(int n, int[] metaI, double[] metaD,
            double[] d0s, float[] fls, int[] flags, double[] out);

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
    // Thread-confined tick queue + scratch (S7-132 discipline: grow-only,
    // no steady-state allocation; one ticking thread owns one queue from
    // creation to flush).
    // ------------------------------------------------------------------
    private static final class Q {
        long stamp = Long.MIN_VALUE; // NONE until the first record of a phase
        int count;
        Entity[] entities = new Entity[64];
        int[] metaI = new int[4 * 64];
        double[] metaD = new double[4 * 64];
        double[] d0s = new double[256];
        float[] fls = new float[17 * 256];
        int[] flags = new int[256];
        int cellCount;
        double[] out = new double[4 * 64];
        // per-call gather scratch (rewritten every call):
        Object[] vKeys = new Object[32];
        byte[] vVals = new byte[32];
        LevelChunkSection[][] flat = new LevelChunkSection[8][];
        double[] sd0 = new double[64];
        float[] sfl = new float[17 * 64];
        int[] sflag = new int[64];
        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
    }

    private static final ThreadLocal<Q> Q_TL = ThreadLocal.withInitial(Q::new);

    private static volatile boolean EFFECT_LOGGED;

    private static void flushQueue(Q q) {
        int n = q.count;
        q.stamp = Long.MIN_VALUE;
        if (n == 0) {
            return;
        }
        int rc;
        try {
            double[] out = q.out;
            if (out.length < 4 * n) {
                out = new double[Math.max(4 * n, out.length * 2)];
                q.out = out;
            }
            rc = fluidBatchTick(n, q.metaI, q.metaD, q.d0s, q.fls, q.flags, out);
        } catch (Throwable t) {
            rc = ERR_STRUCT;
        }
        if (rc != 0) {
            // Fail-closed disarm: discard THIS batch (deferred pushes lost for
            // these records) and fall back to the exact inline vanilla replica
            // for every future call. Never half-applied, never thrown into the
            // tick.
            broken = true;
            LOG.warning("fluid_rust: flush failed rc=" + rc + " n=" + n
                    + " — batch discarded, disarmed to inline vanilla replica");
            q.count = 0;
            q.cellCount = 0;
            java.util.Arrays.fill(q.entities, 0, n, null);
            return;
        }
        boolean effect = false;
        for (int i = 0; i < n; i++) {
            double touched = q.out[4 * i + 3];
            Entity e = q.entities[i];
            q.entities[i] = null;
            if (touched == 0.0D || e == null) {
                continue; // vanilla never calls setDeltaMovement when acc == Vec3.ZERO
            }
            effect = true;
            e.setDeltaMovement(new Vec3(q.out[4 * i], q.out[4 * i + 1], q.out[4 * i + 2]));
        }
        if (!EFFECT_LOGGED && effect) {
            EFFECT_LOGGED = true;
            LOG.info("fluid_rust: EFFECT armed — first batch applied n=" + n
                    + " (bulk 1 JNI/tick, rust verdicts bit-in-bit, TASK-410-B)");
        }
        q.count = 0;
        q.cellCount = 0;
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
            Q q = Q_TL.get();
            long now = self.level().getGameTime();
            if (q.stamp != Long.MIN_VALUE && now != q.stamp) {
                flushQueue(q);
            }
            Gather g = gather(self, tag, speed, q, now);
            if (g == null) {
                return slow(self, tag, speed); // uncovered column / gather problem
            }
            // Vanilla tail, javap 548..560 order: put(tag, maxDepth) BEFORE the
            // ZERO-identity check; maxDepth is the running max of d0 over the
            // scan (plain java, identical bits). Positive ⇒ cells>0 ⇒ vanilla
            // returns true (inFluid is true for every recorded cell; the
            // ZERO-identity return of inFluid is true too).
            self.fluidHeight.put(tag, g.maxDepth);
            if (g.cells > 0) {
                return true;
            }
            return g.inFluid;
        } catch (Throwable t) {
            if (t instanceof OutOfMemoryError || t instanceof StackOverflowError) {
                throw t;
            }
            return slow(self, tag, speed); // fail-dominant: inline vanilla semantics
        }
    }

    private static final class Gather {
        double maxDepth;
        boolean inFluid;
        int cells;
    }

    // ------------------------------------------------------------------
    // Gather: the vanilla scan (bit-exact inputs) + record append. Zero JNI.
    // Returns null when the call must degrade to the slow replica.
    // ------------------------------------------------------------------
    private static Gather gather(Entity self, TagKey<Fluid> tag, double speed, Q q, long now) {
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

        // Coverage expanded ±1 cell in x/z (neighbor reads of edge cells);
        // sections re-fetched EVERY call (fresh world reads, law 5).
        int fx0 = (minX - 1) >> 4, fx1 = (maxX + 1) >> 4;
        int fz0 = (minZ - 1) >> 4, fz1 = (maxZ + 1) >> 4;
        int fspanX = fx1 - fx0 + 1;
        int cols = fspanX * (fz1 - fz0 + 1);
        LevelChunkSection[][] flat = q.flat;
        if (flat.length < cols) {
            flat = new LevelChunkSection[Math.max(cols, flat.length * 2)][];
            q.flat = flat;
        }
        ChunkSource source = level.getChunkSource();
        for (int cz = fz0; cz <= fz1; cz++) {
            for (int cx = fx0; cx <= fx1; cx++) {
                // load=false — matching the vanilla box fetch; an unloaded
                // neighbor column degrades to the slow replica.
                ChunkAccess ch = source.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (ch == null) {
                    return null;
                }
                flat[cx - fx0 + fspanX * (cz - fz0)] = ch.getSections();
            }
        }
        int maxSecY = (WorldUtil.getMaxSection(level) << 4) | 15;
        BlockPos.MutableBlockPos mpos = q.mpos;
        Object[] vKeys = q.vKeys;
        byte[] vVals = q.vVals;
        java.util.Arrays.fill(vKeys, null);

        boolean lavaTag = tag == FluidTags.LAVA;

        Gather r = new Gather();
        int cells = 0;
        double[] sd0 = q.sd0;
        float[] sfl = q.sfl;
        int[] sflag = q.sflag;

        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) - fx0 + fspanX * ((z >> 4) - fz0)][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    byte v = verdictOf(fs, tag, vKeys, vVals);
                    if (v != 2) {
                        continue; // EMPTY or OTHER — vanilla isEmpty || !is(tag) skip
                    }
                    mpos.set(x, y, z);
                    if (lavaTag) {
                        self.lastLavaContact = mpos.immutable(); // vanilla: BEFORE the d0 filter
                    }
                    float ownH = fs.getHeight(level, mpos); // javap @431
                    double d0 = (double) ((float) y + ownH) - box.minY;
                    if (d0 < 0.0D) {
                        continue;
                    }
                    r.inFluid = true;
                    r.maxDepth = Math.max(r.maxDepth, d0);
                    if (!pushedBy) {
                        continue;
                    }
                    if (cells == sd0.length) {
                        int cap = Math.max(cells + 1, sd0.length * 2);
                        double[] nd0 = new double[cap];
                        float[] nfl = new float[17 * cap];
                        int[] nflag = new int[cap];
                        System.arraycopy(sd0, 0, nd0, 0, cells);
                        System.arraycopy(sfl, 0, nfl, 0, 17 * cells);
                        System.arraycopy(sflag, 0, nflag, 0, cells);
                        q.sd0 = sd0 = nd0;
                        q.sfl = sfl = nfl;
                        q.sflag = sflag = nflag;
                    }
                    sd0[cells] = d0;
                    // ---- per-cell flow inputs (getFlow javap contract) ----
                    Fluid ownType = fs.getType();
                    boolean falling = fs.getValue(FlowingFluid.FALLING);
                    int flBase = cells * 17;
                    sfl[flBase] = ownH;
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
                        sfl[flBase + 1 + di] = nH;
                        sfl[flBase + 5 + di] = belowH;
                        sfl[flBase + 9 + di] = (float) dir.getStepX();
                        sfl[flBase + 13 + di] = (float) dir.getStepZ();
                        di++;
                    }
                    sflag[cells] = flag;
                    cells++;
                }
            }
        }

        if (cells > 0) {
            // ---- atomic record append into the tick queue (still zero JNI) ----
            if (q.count >= MAX_RECORDS) {
                flushQueue(q); // valve; the next record re-captures the applied delta
            }
            int n = q.count;
            if (q.entities.length <= n) {
                int cap = Math.max(n + 1, q.entities.length * 2);
                q.entities = java.util.Arrays.copyOf(q.entities, cap);
                q.metaI = java.util.Arrays.copyOf(q.metaI, 4 * cap);
                q.metaD = java.util.Arrays.copyOf(q.metaD, 4 * cap);
                q.out = new double[Math.max(q.out.length, 4 * cap)];
            }
            int total = q.cellCount + cells;
            if (q.d0s.length < total) {
                int cap = Math.max(total, q.d0s.length * 2);
                q.d0s = java.util.Arrays.copyOf(q.d0s, cap);
                q.fls = java.util.Arrays.copyOf(q.fls, 17 * cap);
                q.flags = java.util.Arrays.copyOf(q.flags, cap);
            }
            boolean newEntity = n == 0 || q.entities[n - 1] != self;
            Vec3 delta = self.getDeltaMovement();
            System.arraycopy(sd0, 0, q.d0s, q.cellCount, cells);
            System.arraycopy(sfl, 0, q.fls, 17 * q.cellCount, 17 * cells);
            System.arraycopy(sflag, 0, q.flags, q.cellCount, cells);
            q.entities[n] = self;
            q.metaI[4 * n] = newEntity ? 1 : 0;
            q.metaI[4 * n + 1] = self instanceof net.minecraft.world.entity.player.Player ? 1 : 0;
            q.metaI[4 * n + 2] = q.cellCount;
            q.metaI[4 * n + 3] = cells;
            q.metaD[4 * n] = delta.x;
            q.metaD[4 * n + 1] = delta.y;
            q.metaD[4 * n + 2] = delta.z;
            q.metaD[4 * n + 3] = speed;
            q.cellCount = total;
            q.count = n + 1;
            if (q.stamp == Long.MIN_VALUE) {
                q.stamp = now;
            }
            r.cells = cells;
        }
        return r;
    }

    /** Per-call FluidState identity verdict table (open-addressed, 2^5 slots;
     *  identityHashCode collisions → linear probe). 1=EMPTY 2=TAGGED 3=OTHER.
     *  Live ONLY for the duration of one call (law 5). */
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
    // SLOW replica: faithful vanilla body (javap 13225..13560) — the
    // fail-closed inline path (gather problems, post-disarm calls). No
    // queue, per-call fresh flat array, vanilla FluidState.getFlow API.
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
