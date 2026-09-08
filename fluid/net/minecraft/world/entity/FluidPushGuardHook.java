package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import com.google.common.collect.MapMaker;
import net.minecraft.core.BlockPos;
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
 * TASK-80 whole-body bridge for Entity.updateFluidHeightAndDoFluidPushing.
 *
 * Shape: the patched Entity method body is a single invokestatic into
 * {@link #updateFluidHeightAndDoFluidPushing} (the cplug_sdk asm ReplaceBody
 * spec passes this/slot0, tag/slot1, speed/slot2).
 *
 * Fast path: negative-only same-state guard (docs/GUARD_WAVE_FLUID_PUSH_DESIGN
 * §2). An entry caches ONLY the "no matching-tag fluid cell in the box"
 * outcome; hits are validated by identity re-reading the box cells (FluidState
 * instances are canonical singletons, so any world change surfaces as a
 * different reference). Everything else takes the slow path.
 *
 * Slow path: faithful reimplementation of the vanilla body (javap transcript
 * in the design doc §1) — the whole-body patch cannot call the original, and
 * every callee here is public API. Defining this class into package
 * net.minecraft.world.entity in the KERNEL loader grants the same access
 * rights Entity has (protected fluidHeight).
 */
public final class FluidPushGuardHook {

    /** Weak identity keys (MapMaker.weakKeys uses ==): entries die with the
     * entity, hold no strong refs, and are concurrent-safe under regionized
     * ticking. Values hold only singletons + primitives. Key type is Object:
     * identity comparison needs no Entity typing (and no entity casts). */
    private static final java.util.concurrent.ConcurrentMap<Object, GuardEntry> CACHE =
            new MapMaker().weakKeys().concurrencyLevel(2).makeMap();

    private FluidPushGuardHook() {}

    /** Same-state guard entry. Only pure-negative outcomes are stored. */
    static final class GuardEntry {
        final Level level;
        final TagKey<Fluid> tag;
        final long mx, my, mz, Mx, My, Mz; // raw getBoundingBox() double bits
        final int minX, minY, minZ, maxX, maxY, maxZ; // deflated cell bounds
        final int cx0, cx1, cz0, cz1, spanX, offset; // flat fetch geometry
        final FluidState[] cells;  // FluidState refs in scan order
        GuardEntry(Level level, TagKey<Fluid> tag, AABB box,
                   int minX, int minY, int minZ, int maxX, int maxY, int maxZ,
                   int cx0, int cx1, int cz0, int cz1, int spanX, int offset,
                   FluidState[] cells) {
            this.level = level;
            this.tag = tag;
            this.mx = Double.doubleToLongBits(box.minX);
            this.my = Double.doubleToLongBits(box.minY);
            this.mz = Double.doubleToLongBits(box.minZ);
            this.Mx = Double.doubleToLongBits(box.maxX);
            this.My = Double.doubleToLongBits(box.maxY);
            this.Mz = Double.doubleToLongBits(box.maxZ);
            this.minX = minX; this.minY = minY; this.minZ = minZ;
            this.maxX = maxX; this.maxY = maxY; this.maxZ = maxZ;
            this.cx0 = cx0; this.cx1 = cx1; this.cz0 = cz0; this.cz1 = cz1;
            this.spanX = spanX; this.offset = offset;
            this.cells = cells;
        }
    }

    public static boolean updateFluidHeightAndDoFluidPushing(Entity self, TagKey<Fluid> tag, double speed) {
        GuardEntry e = CACHE.get(self);
        if (e != null && e.tag == tag && e.level == self.level() && boxBitsMatch(self, e)) {
            try {
                if (cellsUnchanged(self, e)) {
                    // Vanilla pure-negative tail: fluidHeight.put(tag, 0.0);
                    // flowAcc == Vec3.ZERO (identity) -> return inFluid=false.
                    self.fluidHeight.put(tag, 0.0);
                    return false;
                }
            } catch (Throwable t) {
                // Any fetch problem (chunk unloaded mid-check, section bounds,
                // etc.): slow path re-derives everything with load=true,
                // exactly like vanilla.
            }
        }
        return slow(self, tag, speed);
    }

    private static boolean boxBitsMatch(Entity self, GuardEntry e) {
        AABB b = self.getBoundingBox();
        return Double.doubleToLongBits(b.minX) == e.mx
            && Double.doubleToLongBits(b.minY) == e.my
            && Double.doubleToLongBits(b.minZ) == e.mz
            && Double.doubleToLongBits(b.maxX) == e.Mx
            && Double.doubleToLongBits(b.maxY) == e.My
            && Double.doubleToLongBits(b.maxZ) == e.Mz;
    }

    /** Identity re-read of every box cell. Needs no invalidation hooks:
     * FluidState singletons change iff the world changed. Chunk fetch uses
     * load=false — any unloaded column -> miss -> slow path. */
    private static boolean cellsUnchanged(Entity self, GuardEntry e) {
        ChunkSource source = e.level.getChunkSource();
        LevelChunkSection[][] flat = new LevelChunkSection[e.spanX * (e.cz1 - e.cz0 + 1)][];
        for (int cz = e.cz0; cz <= e.cz1; cz++) {
            for (int cx = e.cx0; cx <= e.cx1; cx++) {
                ChunkAccess chunk = source.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (chunk == null) return false;
                flat[cx + e.spanX * cz + e.offset] = chunk.getSections();
            }
        }
        int i = 0;
        int minSection = WorldUtil.getMinSection(e.level);
        for (int x = e.minX; x <= e.maxX; x++) {
            for (int y = e.minY; y <= e.maxY; y++) {
                for (int z = e.minZ; z <= e.maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) + e.spanX * (z >> 4) + e.offset][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    if (fs != e.cells[i++]) return false;
                }
            }
        }
        return true;
    }

    /** Faithful vanilla-body reimplementation (design doc §1). Caches the
     * pure-negative outcome; every other outcome is uncached by design
     * (getFlow reads the cell neighborhood — negative-only rule). */
    private static boolean slow(Entity self, TagKey<Fluid> tag, double speed) {
        if (self.touchingUnloadedChunk()) {
            CACHE.remove(self);
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
                // load=false: the original body passes iconst_0 (javap @260) —
                // touchingUnloadedChunk() has already guaranteed presence.
                flat[cx + spanX * cz + offset] = source.getChunk(cx, cz, ChunkStatus.FULL, false).getSections();
            }
        }

        Vec3 flowAcc = Vec3.ZERO;
        double maxDepth = 0.0;
        boolean inFluid = false;
        int flowCount = 0;
        boolean cacheable = true;
        FluidState[] snapshot = new FluidState[Math.max(0,
                (maxX - minX + 1) * Math.max(0, maxY - minY + 1) * Math.max(0, maxZ - minZ + 1))];
        int snap = 0;
        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        for (int x = minX; x <= maxX; x++) {
            for (int y = minY; y <= maxY; y++) {
                for (int z = minZ; z <= maxZ; z++) {
                    LevelChunkSection sec = flat[(x >> 4) + spanX * (z >> 4) + offset][(y >> 4) - minSection];
                    FluidState fs = ((BlockState) sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8))).getFluidState();
                    snapshot[snap++] = fs;
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    cacheable = false; // fluid present -> negative-only rule
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
        self.fluidHeight.put(tag, maxDepth);
        if (cacheable && CACHE.size() < (1 << 16)) {
            CACHE.put(self, new GuardEntry(level, tag, box, minX, minY, minZ, maxX, maxY, maxZ,
                    cx0, cx1, cz0, cz1, spanX, offset, java.util.Arrays.copyOf(snapshot, snap)));
        }
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

    /** Reflective self-test (driven from src/fluid_guard.rs after arming):
     * verifies the guard cache machinery is operational in this JVM. */
    public static boolean selfTest() {
        if (CACHE == null) return false;
        Object probe = new Object();
        AABB dummyBox = new AABB(0.0D, 0.0D, 0.0D, 1.0D, 1.0D, 1.0D);
        GuardEntry dummy = new GuardEntry(null, FluidTags.WATER, dummyBox,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, new FluidState[0]);
        CACHE.put(probe, dummy);
        boolean ok = CACHE.get(probe) == dummy;
        CACHE.remove(probe);
        return ok && CACHE.isEmpty();
    }
}
