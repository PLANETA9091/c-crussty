package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import net.minecraft.core.BlockPos;
import net.minecraft.server.MinecraftServer;
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
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TASK-415-C FLUID-BULK (lever cmp415-fluid family, flag STRICT-eq
 * "cmp415_fluid"): subsystem-scale rebuild of the fluid-update plane
 * Entity.updateFluidHeightAndDoFluidPushing — the top untouched lane of the
 * era (16.7% of the cvs1r profile).
 *
 * RECON ROOT-CAUSE (cvs1r cpu-collapsed decomposition, TASK-415-C): of the
 * 16.78% fluid lane, reads dominate — PalettedContainer.get + SimpleBitStorage
 * + VarHandle volatile chains ~10-12%, touchingUnloadedChunk -> hasChunksAt ->
 * ServerChunkCache.getChunkNow concurrent-map volatile reads ~6.6%, the
 * per-call bridge/guard machinery (weak-map cache probes, AABB inflate,
 * bounds math) ~10-15%, and only the wet-cell math (FlowingFluid.getHeight /
 * getFlow ~18%) is irreducible vanilla work. The 150k population re-reads the
 * SAME cells 10-100x per tick (items pile inside 0.5 blocks; every entity of
 * a pile re-sweeps the same cells).
 *
 * DESIGN (the "gather-piggyback" recon direction, honestly amended): a
 * tick-scoped CELL LUT replaces the data plane. Per (worker thread, tick,
 * fluid-generation epoch) each cell's FluidState is read EXACTLY ONCE with
 * the bit-exact vanilla fetch (the FluidPushGuardHook.slow flat-section
 * body, javap-proven since TASK-80) and every later sweep of the same cell
 * on that thread is one array probe. This IS the "gather piggyback": the
 * first entity to touch a cell does the gathering for the whole pile — no
 * separate gather pass exists (a Rust prefill pass was evaluated and
 * REJECTED by economics: it duplicates sweep reads 1:1 onto a single
 * thread's ThreadLocal plane; the lazy probe-dedup gathers for free).
 *
 * INVALIDATION (RECON-43 ec880c2 generation-write-bump contract, NOT the
 * forbidden fluid_dirty memo): the ONE retargeted write site
 * LevelChunk.setBlockState -> LevelChunkSection.setBlockState is delegated
 * through {@link #secWrite}, which bumps the GLOBAL fluid generation
 * {@link #GEN} on a real fluid-state change (FluidState singleton
 * ref-compare). LUT validity = (tick, GEN) pair; a write mid-tick
 * invalidates the whole LUT for the next sweep; a sweep that observed a GEN
 * bump mid-flight is REDONE once, then falls to vanilla — never mixed.
 * Chunk-column presence is cached under the same (tick, GEN) validity with
 * the documented invariant that chunk UNLOADS are processed outside the
 * region entity-tick phases (chunk system runs between ticks), so
 * touchingUnloadedChunk's answer cannot change inside one entity-tick phase
 * for an unchanged chunk map.
 *
 * ONE BULK JNI/TICK (law 6): {@code fluidBulk(tick, io)} is invoked once per
 * server tick (JVM-wide, EPOCH volatile release-edge, the MobScanOps
 * protocol) — RUST owns the tick-epoch authority, aggregates the census
 * (calls/hits/wet/slow deltas), runs the SoA validation pass and the
 * fail-closed disarm policy. Per-entity JNI is absent by construction (the
 * hot path is pure Java array probes).
 *
 * VANILLA SEMANTICS (law 4): the sweep body is the javap transcript of the
 * vanilla method (identical to the deployed FluidPushGuardHook.slow): same
 * deflated-box cell bounds, same float/double height arithmetic, same
 * lastLavaContact writes, same flow accumulation ladder
 * (maxDepth<0.4 ? flow.scale(maxDepth) : flow), same normalize/scale/0.003 /
 * 0.0045 push tail, same fluidHeight.put(tag, maxDepth) and boolean return.
 * With the lever flag unset the whole module never loads and Entity runs
 * vanilla bit-in-byte.
 *
 * FAIL-CLOSED: any Throwable in the fast path -> permanent per-call vanilla
 * body (bit-exact); fluidBulk ERR_STRUCT -> LUT disarmed forever (vanilla);
 * GEN re-check failure -> the sweep is redone once, then vanilla.
 */
public final class FluidBulkOps {

    private FluidBulkOps() {}

    /** STRICT-eq lever gate (round-400 protocol; empty/foreign flag = vanilla). */
    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        if (f == null) return false;
        f = f.trim();
        // TASK-415-C: own lever + carrier composite superset. The composite
        // "cmp415_fluid" is the only arming flag on this branch (the carrier
        // modules gate on their own OR-chains; see src/fluid_bulk.rs).
        return f.equals("cmp415_fluid");
    }

    private static final boolean ENABLED = leverEnabled();

    /** Global fluid generation (RECON-43 write-bump contract). Bumped by
     * {@link #secWrite} on real fluid-state changes; LUT validity key. */
    public static volatile long GEN = 0L;

    // ---------------- census (drained by fluidBulk each tick) ----------------
    public static volatile long C_CALLS = 0, C_HITS = 0, C_WET = 0,
            C_SLOW = 0, C_PRESENCE = 0, C_FILL = 0;

    // ---------------- the ONE bulk JNI per tick (src/fluid_bulk.rs) ----------
    private static native int fluidProbe();
    private static native int fluidBulk(int tick, long[] io);

    static final int PROBE_MAGIC = 0x4642; // "FB"
    static final int ERR_STRUCT = -1;
    static final int ERR_RANGE = -2;

    private static volatile boolean nativeOk;
    /** Rust policy verdict: true while the bulk channel is healthy. */
    private static volatile boolean rustOk;
    private static volatile long STAMPED_TICK = Long.MIN_VALUE;
    private static final Object EPOCH_LOCK = new Object();

    private static boolean probeOnce() {
        if (nativeOk) return true;
        synchronized (FluidBulkOps.class) {
            if (nativeOk) return true;
            try {
                nativeOk = fluidProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** ARM/EFFECT marker (one-shot, lands in server-stdout.log). */
    private static volatile boolean ARM_LOGGED = false;

    /**
     * Tick-epoch handshake: ONCE per server tick JVM-wide, the rust bulk
     * native validates the epoch and drains the census deltas. Every other
     * call this tick is one volatile read. rc<0 keeps the plane alive
     * (rustOk=false degrades the LUT to per-call vanilla, never wrong).
     */
    private static void maybeEpoch() {
        long t = MinecraftServer.getServer().getTickCount();
        if (STAMPED_TICK == t) return; // hot path
        synchronized (EPOCH_LOCK) {
            if (STAMPED_TICK == t) return;
            long[] io = new long[8];
            io[0] = t;
            io[1] = C_CALLS; io[2] = C_HITS; io[3] = C_WET; io[4] = C_SLOW;
            io[5] = C_PRESENCE; io[6] = C_FILL; io[7] = GEN;
            int rc;
            try {
                rc = fluidBulk((int) t, io);
            } catch (Throwable th) {
                rc = ERR_STRUCT;
            }
            if (rc == ERR_STRUCT) {
                rustOk = false;
                System.err.println(
                        "[crussty-plugin] cmp415_fluid: fluidBulk ERR_STRUCT — LUT disarmed to vanilla");
            } else {
                rustOk = rc >= 0;
            }
            if (!ARM_LOGGED && rc >= 0) {
                ARM_LOGGED = true;
                System.err.println(
                        "[crussty-plugin] cmp415_fluid: EFFECT armed (bulk epoch stamped at tick " + t
                                + ", rc=" + rc + ", cells cached this boot: " + io[7] + ")");
            }
            STAMPED_TICK = t;
        }
    }

    // ---------------- thread-confined LUT + presence cache ----------------

    static final int LUT_BITS = 16; // 65536 slots (~1 MiB/thread)
    static final int LUT_MASK = (1 << LUT_BITS) - 1;

    static final int PRES_BITS = 12; // 4096 columns
    static final int PRES_MASK = (1 << PRES_BITS) - 1;

    static final class Plane {
        final long[] key = new long[1 << LUT_BITS];
        final FluidState[] val = new FluidState[1 << LUT_BITS];
        final long[] pkey = new long[1 << PRES_BITS];
        final boolean[] pval = new boolean[1 << PRES_BITS];
        long gen = Long.MIN_VALUE;
        long tick = Long.MIN_VALUE;
    }

    private static final ThreadLocal<Plane> PLANE =
            ThreadLocal.withInitial(Plane::new);

    private static long mix(long k) {
        k *= 0x9E3779B97F4A7C15L;
        k ^= k >>> 29;
        k *= 0xBF58476D1CE4E5B9L;
        k ^= k >>> 32;
        return k;
    }

    /** Pack an (x,y,z) cell into a 64-bit probe key; y has 12 bits,
     * x/z 26 bits each — disjoint fields, no aliasing inside world bounds. */
    private static long pack(int x, int y, int z) {
        return ((x & 0x3FFFFFFL) << 38) | ((z & 0x3FFFFFFL) << 12) | (y & 0xFFFL);
    }

    // ---------------- the whole-body bridge ----------------

    public static boolean updateFluidHeightAndDoFluidPushing(Entity self, TagKey<Fluid> tag, double speed) {
        if (!ENABLED || !rustOk) {
            return slowVanilla(self, tag, speed);
        }
        final Plane p = PLANE.get();
        final long tick = MinecraftServer.getServer().getTickCount();
        if (p.tick != tick || p.gen != GEN) {
            // (tick, GEN) rollover: drop all entries for this thread.
            java.util.Arrays.fill(p.val, null);
            p.tick = tick;
            p.gen = GEN;
            maybeEpoch();
            if (!rustOk) {
                return slowVanilla(self, tag, speed);
            }
        }
        try {
            return sweep(self, tag, speed, p);
        } catch (Throwable th) {
            // Fail-dominant: any inconsistency degrades this plane to the
            // bit-exact vanilla body forever (matches fluid_guard protocol).
            rustOk = false;
            return slowVanilla(self, tag, speed);
        }
    }

    /** Bit-exact vanilla sweep with the LUT data plane. */
    private static boolean sweep(Entity self, TagKey<Fluid> tag, double speed, Plane p) {
        C_CALLS++;
        if (self.touchingUnloadedChunk()) {
            return false;
        }
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushFlag = self.isPushedByFluid();

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0;
        boolean inFluid = false;
        int flowCount = 0;
        long genAtStart = GEN;

        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    long k = pack(x, y, z);
                    int slot = (int) mix(k) & LUT_MASK;
                    FluidState fs = p.val[slot];
                    if (fs == null || p.key[slot] != k) {
                        // vanilla fetch: the only read of this cell on this
                        // thread this (tick, gen) — the flat-section body of
                        // the guard, via Level.getFluidState semantics.
                        fs = fetch(p, level, x, y, z, minSection);
                        C_FILL++;
                        p.key[slot] = k;
                        p.val[slot] = fs;
                    } else {
                        C_HITS++;
                    }
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    C_WET++;
                    mpos.set(x, y, z);
                    if (tag == FluidTags.LAVA) {
                        self.lastLavaContact = mpos.immutable();
                    }
                    // javap-exact arithmetic: (float)y + height in FLOAT, then
                    // widened, minus box.minY in double (bytecode i2f/fadd/f2d).
                    double d0 = (double) ((float) y + fs.getHeight(level, mpos)) - box.minY;
                    if (d0 < 0.0) {
                        continue;
                    }
                    inFluid = true;
                    maxDepth = Math.max(maxDepth, d0);
                    if (!pushFlag) {
                        continue;
                    }
                    flowCount++; // javap @478-482: counted BEFORE getFlow
                    Vec3 flow = fs.getFlow(level, mpos);
                    if (maxDepth < 0.4) {
                        flowAcc = flowAcc.add(flow.scale(maxDepth));
                    } else {
                        flowAcc = flowAcc.add(flow);
                    }
                }
            }
        }
        // GEN re-check: a write that raced the sweep invalidates mixed reads.
        if (genAtStart != GEN) {
            C_SLOW++;
            return sweepRetry(self, tag, speed, p);
        }
        self.fluidHeight.put(tag, maxDepth);
        if (flowAcc == Vec3.ZERO) {
            return inFluid;
        }
        flowAcc = flowAcc.scale(1.0 / flowCount); // javap @572-578
        Vec3 delta = self.getDeltaMovement();
        if (!(self instanceof net.minecraft.world.entity.player.Player)) {
            flowAcc = flowAcc.normalize();
        }
        flowAcc = flowAcc.scale(speed);
        if (Math.abs(delta.x) < 0.003 && Math.abs(delta.z) < 0.003 && flowAcc.length() < 0.0045) {
            flowAcc = flowAcc.normalize().scale(0.0045);
        }
        self.setDeltaMovement(delta.add(flowAcc));
        return true;
    }

    /** One retry after a mid-sweep GEN bump; fresh (gen-stamped) LUT state. */
    private static boolean sweepRetry(Entity self, TagKey<Fluid> tag, double speed, Plane p) {
        java.util.Arrays.fill(p.val, null);
        p.gen = GEN;
        return sweep(self, tag, speed, p);
    }

    /**
     * The vanilla per-cell fetch with the guard's flat-section chunk protocol
     * (load=false after touchingUnloadedChunk guaranteed presence). Cached
     * column presence rides the same (tick, GEN) validity window.
     */
    private static FluidState fetch(Plane p, Level level, int x, int y, int z, int minSection) {
        ChunkSource source = level.getChunkSource();
        int cx = x >> 4, cz = z >> 4;
        // presence key: hash of the column pair (its own table — key universe
        // is independent of the cell LUT, but kept hash-mixed for spread)
        long pk = mix(pack(cx, 0, cz)) | Long.MIN_VALUE;
        int ps = (int) mix(pk) & PRES_MASK;
        ChunkAccess chunk;
        if (p.pkey[ps] == pk && p.pval[ps]) {
            chunk = source.getChunk(cx, cz, ChunkStatus.FULL, false);
            if (chunk == null) {
                // presence said loaded, the map disagrees — vanilla retry.
                chunk = source.getChunk(cx, cz, ChunkStatus.FULL, true);
            }
        } else {
            C_PRESENCE++;
            chunk = source.getChunk(cx, cz, ChunkStatus.FULL, true);
            p.pkey[ps] = pk;
            p.pval[ps] = true;
        }
        LevelChunkSection sec = chunk.getSections()[(y >> 4) - minSection];
        return ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
    }

    /**
     * The faithful vanilla body (FluidPushGuardHook.slow verbatim semantics —
     * the deployed javap transcript of Entity.updateFluidHeightAndDoFluidPushing).
     * Used while the LUT plane is unarmed/broken; never mixed with the LUT.
     */
    private static boolean slowVanilla(Entity self, TagKey<Fluid> tag, double speed) {
        C_SLOW++;
        if (self.touchingUnloadedChunk()) {
            return false;
        }
        Level level = self.level();
        AABB box = self.getBoundingBox().deflate(0.001);
        int minSection = WorldUtil.getMinSection(level);
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        boolean pushFlag = self.isPushedByFluid();

        int cx0 = minX >> 4, cx1 = maxX >> 4, cz0 = minZ >> 4, cz1 = maxZ >> 4;
        int spanX = cx1 - cx0 + 1;
        int offset = -(cx0 + spanX * cz0);
        LevelChunkSection[][] flat = new LevelChunkSection[spanX * (cz1 - cz0 + 1)][];
        ChunkSource source = level.getChunkSource();
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                // load=false: touchingUnloadedChunk() has guaranteed presence.
                flat[cx + spanX * cz + offset] = source.getChunk(cx, cz, ChunkStatus.FULL, false).getSections();
            }
        }

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0;
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
                    if (d0 < 0.0) {
                        continue;
                    }
                    inFluid = true;
                    maxDepth = Math.max(maxDepth, d0);
                    if (!pushFlag) {
                        continue;
                    }
                    flowCount++;
                    Vec3 flow = fs.getFlow(level, mpos);
                    if (maxDepth < 0.4) {
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
        flowAcc = flowAcc.scale(1.0 / flowCount);
        Vec3 delta = self.getDeltaMovement();
        if (!(self instanceof net.minecraft.world.entity.player.Player)) {
            flowAcc = flowAcc.normalize();
        }
        flowAcc = flowAcc.scale(speed);
        if (Math.abs(delta.x) < 0.003 && Math.abs(delta.z) < 0.003 && flowAcc.length() < 0.0045) {
            flowAcc = flowAcc.normalize().scale(0.0045);
        }
        self.setDeltaMovement(delta.add(flowAcc));
        return true;
    }

    // ---------------- the RECON-43 write-bump retarget site ----------------

    /**
     * Delegate of the single LevelChunk.setBlockState ->
     * LevelChunkSection.setBlockState(IIILBlockState)BlockState call site
     * (retarget contract identical to the RECON-43 ledger: returns the OLD
     * state; a real fluid-state change (FluidState singleton ref-compare)
     * bumps the global GEN — the LUT invalidation edge. Plain block writes
     * never touch GEN, so static worlds keep a perfect hit rate.
     */
    public static BlockState secWrite(LevelChunkSection section, int x, int y, int z, BlockState newState) {
        BlockState old = section.setBlockState(x, y, z, newState);
        if (old.getFluidState() != newState.getFluidState()) {
            GEN++;
        }
        return old;
    }

    /** Reflective self-test (driven from src/fluid_bulk.rs after arming):
     * probe/pack/presence algebra sanity inside the live JVM. */
    public static boolean selfTest() {
        Plane p = new Plane();
        long k = pack(123456, 64, -654321);
        if (k == pack(123457, 64, -654321) || k == pack(123456, 65, -654321)) return false;
        int s = (int) mix(k) & LUT_MASK;
        if (s < 0 || s >= p.key.length) return false;
        p.key[s] = k;
        if (p.val[s] != null) return false;
        p.pkey[s & PRES_MASK] = mix(pack(3, 0, 4)) | Long.MIN_VALUE;
        if (p.pkey[s & PRES_MASK] >= 0) return false;
        // GEN visibility + census array shape
        long g0 = GEN;
        GEN = g0 + 1;
        boolean ok = GEN == g0 + 1;
        GEN = g0;
        return ok;
    }
}
