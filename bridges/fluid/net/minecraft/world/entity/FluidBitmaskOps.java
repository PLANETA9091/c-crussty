package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;

/**
 * RECON-43 ARCH-LEVER #16 (TASK-389): section-resident fluid bitmaps replacing
 * the per-entity-per-tick block iteration data plane of
 * Entity.updateFluidHeightAndDoFluidPushing (the 14.6%-of-java-scene lane).
 *
 * Contract: research/gc-recon-2026-09-19/RECON43_FLUIDPUSH_BITMASK_CONTRACT.md
 * (commit ec880c2). Median-exact class (RECON-33 inside_bitmask precedent):
 * a CLEAN verdict implies the vanilla body for THIS tag reduces to the
 * verified pure-negative tail — fluidHeight.put(tag, 0.0) + return false
 * (FluidPushGuardHook.slow: the put is UNCONDITIONAL, maxDepth stays 0.0,
 * no delta change, no lastLavaContact write for WATER; LAVA cannot be CLEAN
 * on a volume whose lava bits are zero — identical reasoning).
 *
 * Invalidation: identity-keyed dirty-stamp ledger FluidPushOps.LEDGER
 * (bumped by the LevelChunk.setBlockState retarget ONLY on real fluid-state
 * changes) + section object identity — a swapped section object can never
 * inherit a stale verdict (fresh objects start at stamp 0 but the identity
 * compare fails first). Positional instability (S7-153 lesson) does not
 * apply: the cache is per-SECTION, not per-entity.
 *
 * Budget: at most BUILDS_PER_TICK section builds per game tick (bounded
 * first-touch wave, ~100µs per build); misses beyond the budget fall through
 * to the banked bit-exact slow path. Thread-safe (region_threads workers):
 * CHM + idempotent deterministic rebuilds.
 */
public final class FluidBitmaskOps {

    /** bits[0..63] = WATER, bits[64..127] = LAVA; slot=(x&15)|((z&15)<<4)|((y&15)<<8). */
    static final class Entry {
        final LevelChunkSection sec;
        final long stamp;
        final long[] bits;
        Entry(LevelChunkSection s, long st, long[] b) { sec = s; stamp = st; bits = b; }
    }

    static final ConcurrentHashMap<Long, Entry> CACHE = new ConcurrentHashMap<>();
    static final int CACHE_CAP = 1 << 15;
    static final int BUILDS_PER_TICK = 64;
    static final AtomicInteger BUDGET = new AtomicInteger();
    static volatile long budgetWindow = -1L;
    /** ~1.05s windows (2^-30 ns). */
    static long window() { return System.nanoTime() >>> 30; }

    private FluidBitmaskOps() {}

    /**
     * @return true  => volume provably holds NO fluid matching tag:
     *                caller performs the bit-exact negative tail
     *                (fluidHeight.put(tag, 0.0); return false).
     *         false => run the banked bit-exact slow path (vanilla).
     */
    public static boolean clean(Entity self, TagKey<Fluid> tag) {
        final boolean water = tag == FluidTags.WATER;
        if (!water && tag != FluidTags.LAVA) {
            return false; // foreign tag: vanilla slow path (hook only sees WATER/LAVA today)
        }
        Level level = self.level();
        if (self.touchingUnloadedChunk()) {
            return false; // slow path owns the vanilla early-return semantics
        }
        AABB box = self.getBoundingBox().deflate(0.001);
        int minSection = WorldUtil.getMinSection(level);
        int minSectionY = minSection >> 4;
        int minX = Mth.floor(box.minX);
        int minY = Math.max(minSection << 4, Mth.floor(box.minY));
        int minZ = Mth.floor(box.minZ);
        int maxX = Mth.ceil(box.maxX) - 1;
        int maxY = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(box.maxY) - 1);
        int maxZ = Mth.ceil(box.maxZ) - 1;
        if (maxX < minX || maxY < minY || maxZ < minZ) {
            return false;
        }
        ChunkSource source = level.getChunkSource();
        for (int cy = minY >> 4, cyEnd = maxY >> 4; cy <= cyEnd; cy++) {
            for (int cz = minZ >> 4, czEnd = maxZ >> 4; cz <= czEnd; cz++) {
                for (int cx = minX >> 4, cxEnd = maxX >> 4; cx <= cxEnd; cx++) {
                    ChunkAccess chunk = source.getChunk(cx, cz, ChunkStatus.FULL, false);
                    if (chunk == null) {
                        return false; // unloaded race: conservative vanilla
                    }
                    LevelChunkSection[] secs = chunk.getSections();
                    int si = cy - minSectionY;
                    if (si < 0 || si >= secs.length) {
                        continue; // outside storage: no fluid possible
                    }
                    LevelChunkSection sec = secs[si];
                    if (sec == null || sec.hasOnlyAir()) {
                        continue;
                    }
                    Entry e = entryFor(cx, cy, cz, sec);
                    if (e == null) {
                        return false; // build budget exhausted: vanilla scan
                    }
                    if (volumeHasTag(e.bits, water, cx, cy, cz, minX, maxX, minY, maxY, minZ, maxZ)) {
                        return false; // matching fluid present: vanilla scan
                    }
                }
            }
        }
        return true; // CLEAN: bit-exact negative tail for THIS tag
    }

    /** Fresh-or-rebuild lookup; null => caller must fall back to vanilla. */
    static Entry entryFor(int cx, int cy, int cz, LevelChunkSection sec) {
        long key = (((long) cx & 0x3FFFFF) << 42) | (((long) cy & 0x3FFFFF) << 21) | ((long) cz & 0x3FFFFF);
        Entry e = CACHE.get(key);
        long st = FluidPushOps.stampOf(sec);
        if (e != null && e.sec == sec && e.stamp == st) {
            return e;
        }
        return rebuild(key, sec, st);
    }

    static Entry rebuild(long key, LevelChunkSection sec, long st) {
        long now = window();
        if (now != budgetWindow) {
            budgetWindow = now;
            BUDGET.set(BUILDS_PER_TICK);
        }
        if (BUDGET.decrementAndGet() < 0 || CACHE.size() >= CACHE_CAP) {
            return null;
        }
        long[] bits = new long[128];
        for (int i = 0; i < 4096; i++) {
            FluidState fs = ((BlockState) sec.states.get(i)).getFluidState();
            if (fs.isEmpty()) {
                continue;
            }
            if (fs.is(FluidTags.WATER)) {
                bits[i >> 6] |= 1L << (i & 63);
            }
            if (fs.is(FluidTags.LAVA)) {
                bits[64 + (i >> 6)] |= 1L << (i & 63);
            }
        }
        Entry fresh = new Entry(sec, st, bits);
        CACHE.put(key, fresh);
        return fresh;
    }

    /** Exact-volume bit test for one (tag, section) pair. */
    static boolean volumeHasTag(long[] bits, boolean water, int cx, int cy, int cz,
                                int minX, int maxX, int minY, int maxY, int minZ, int maxZ) {
        final long[] half = water ? bits : null;
        final int off = water ? 0 : 64;
        final int y0 = Math.max(minY, cy << 4), y1 = Math.min(maxY, (cy << 4) | 15);
        final int z0 = Math.max(minZ, cz << 4), z1 = Math.min(maxZ, (cz << 4) | 15);
        final int x0 = Math.max(minX, cx << 4), x1 = Math.min(maxX, (cx << 4) | 15);
        for (int y = y0; y <= y1; y++) {
            for (int z = z0; z <= z1; z++) {
                for (int x = x0; x <= x1; x++) {
                    int slot = (x & 15) | ((z & 15) << 4) | ((y & 15) << 8);
                    if ((bits[off + (slot >> 6)] & (1L << (slot & 63))) != 0L) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /** Reflective arming self-test (driven from src/fluid_bitmask.rs). */
    public static boolean selfTest() {
        Entry probe = new Entry(null, 7L, new long[128]);
        CACHE.put(0xABCDEF987L, probe);
        boolean ok = CACHE.get(0xABCDEF987L) == probe && probe.bits.length == 128;
        CACHE.remove(0xABCDEF987L);
        return ok && CACHE.isEmpty();
    }
}
