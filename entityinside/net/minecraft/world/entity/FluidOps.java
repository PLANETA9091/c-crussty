package net.minecraft.world.entity;

import it.unimi.dsi.fastutil.objects.Object2DoubleMap;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.phys.AABB;
import sun.misc.Unsafe;

/**
 * FLUID-FREE-SECTION bridge (S7-143 re-implementation of the lost S7-139;
 * design: research/fluid-free-2026-09-18/DESIGN.md, S7-138).
 *
 * Compiled against the PURE kernel (scripts/build_fluid_ops.sh — javac
 * kernel+fastutil, no patched-classpath stubs): the injected fields
 * (crusstyFf/crusstyFfGen on LevelChunkSection, crusstyGen on
 * PalettedContainer) are reached EXCLUSIVELY via Unsafe field offsets
 * resolved lazily — no class-level dependency on patched shapes, so the
 * bridge can be defined BEFORE any Entity retarget (LinkageError race
 * closed by the inside_chain wait_bridge_ready protocol).
 *
 * HIT-semantic parity (DESIGN §3): when every section under the deflated
 * AABB is fluid-free, vanilla's triple loop finds nothing and its tail runs
 * fluidHeight.put(tag, 0.0) + return false (the LAVA fire branch is
 * unreachable when no fluids exist). The bridge performs EXACTLY that:
 * fluidHeight.put(tag, 0.0); return false; — wrapper side effects
 * (wasTouchingWater=false, fluidHeight.clear() in the lava wrapper head)
 * execute by themselves in the caller's body.
 *
 * Verdict byte semantics (S7-143 lesson): 0=unknown, 1=fluid-free,
 * 2=has-fluids. Zero-init is NEVER served as a verdict; unknown triggers
 * the lazy 4096-cell scan. Cache is validated against the demux MUTATION
 * epoch (PalettedContainer.crusstyGen, ±2 per mutation: prologue+epilogue),
 * NOT crusstySnapGen. Publish order: ffGen FIRST, then ff (single-writer
 * per section is licensed by the kernel's moonrise region-lock; a reader
 * seeing a torn pair either re-reads on the next tick or takes a false
 * miss — fail-dominant).
 *
 * Fail-closed: any Unsafe resolution failure => ARMED stays false =>
 * fgate always delegates to the vanilla method (miss); 50 lazy retries
 * then give up permanently (the gate turns into the vanilla call).
 */
public final class FluidOps {

    private static final sun.misc.Unsafe UNSAFE;
    static {
        sun.misc.Unsafe u;
        try {
            java.lang.reflect.Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            u = (Unsafe) f.get(null);
        } catch (Throwable t) {
            u = null;
        }
        UNSAFE = u;
    }

    private static volatile boolean ARMED = false;
    private static volatile int TRIES = 0;
    private static long OFF_FF = -1;
    private static long OFF_FFGEN = -1;
    private static long OFF_GEN = -1;

    private static final int VERDICT_UNKNOWN = 0;
    private static final int VERDICT_FREE = 1;
    private static final int VERDICT_HAS = 2;

    private FluidOps() {}

    /** Resolve the injected-field offsets (public API for the harness). */
    public static void arm() {
        resolve();
    }

    /** Harness/diagnostics visibility. */
    public static boolean armed() {
        return ARMED;
    }

    private static boolean resolve() {
        if (ARMED) {
            return true;
        }
        if (UNSAFE == null || TRIES >= 50) {
            return false;
        }
        TRIES++;
        try {
            Class<?> sec = LevelChunkSection.class;
            OFF_FF = UNSAFE.objectFieldOffset(sec.getField("crusstyFf"));
            OFF_FFGEN = UNSAFE.objectFieldOffset(sec.getField("crusstyFfGen"));
            Class<?> pc = PalettedContainer.class;
            java.lang.reflect.Field gen;
            try {
                gen = pc.getField("crusstyGen");
            } catch (NoSuchFieldException e) {
                gen = pc.getDeclaredField("crusstyGen");
                gen.setAccessible(true);
            }
            OFF_GEN = UNSAFE.objectFieldOffset(gen);
            ARMED = true;
            return true;
        } catch (Throwable t) {
            return false;
        }
    }

    /**
     * The retargeted gate (both wrapper call-sites point here).
     * Descriptor: (LEntity;LTagKey;D)Z — receiver-first, 3B invokestatic.
     */
    public static boolean fgate(Entity e, TagKey<Fluid> tag, double speed) {
        if (!ARMED && !resolve()) {
            return e.updateFluidHeightAndDoFluidPushing(tag, speed);
        }
        try {
            if (allFluidFree(e)) {
                // Vanilla HIT tail: no non-empty fluid cell was found, so the
                // loop body never ran and the tail wrote 0.0 height + false.
                e.fluidHeight.put(tag, 0.0);
                return false;
            }
        } catch (Throwable t) {
            // Fail-dominant: any bridge fault falls back to vanilla semantics.
            return e.updateFluidHeightAndDoFluidPushing(tag, speed);
        }
        return e.updateFluidHeightAndDoFluidPushing(tag, speed);
    }

    /**
     * True iff every section covering the deflated AABB is proven
     * fluid-free. Unloaded chunk => false (vanilla returns false on
     * touchingUnloadedChunk without a fluidHeight write; letting the miss
     * fall through to vanilla preserves that exactly).
     */
    public static boolean allFluidFree(Entity e) {
        AABB box = e.getBoundingBox().deflate(0.001);
        Level lvl = e.level();
        int cx0 = Mth.floor(box.minX) >> 4;
        int cx1 = Mth.floor(box.maxX) >> 4;
        int cz0 = Mth.floor(box.minZ) >> 4;
        int cz1 = Mth.floor(box.maxZ) >> 4;
        int sy0 = Mth.floor(box.minY) >> 4;
        int sy1 = Mth.floor(box.maxY) >> 4;
        int minSection = lvl.getMinSectionY();
        int sectionsCount = lvl.getSectionsCount();
        for (int cx = cx0; cx <= cx1; cx++) {
            for (int cz = cz0; cz <= cz1; cz++) {
                ChunkAccess ch = lvl.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (ch == null) {
                    return false;
                }
                LevelChunkSection[] secs = ch.getSections();
                int y0 = Math.max(sy0, minSection);
                int y1 = Math.min(sy1, minSection + sectionsCount - 1);
                for (int sy = y0; sy <= y1; sy++) {
                    int idx = sy - minSection;
                    if (idx < 0 || idx >= secs.length) {
                        continue;
                    }
                    LevelChunkSection s = secs[idx];
                    if (s == null || s.hasOnlyAir()) {
                        continue;
                    }
                    if (!sectionFluidFree(s)) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    /**
     * Section verdict with the (ffGen, ff) lazy-scan cache. No fluid-state
     * writes happen here; only the verdict byte/epoch move.
     */
    public static boolean sectionFluidFree(LevelChunkSection s) {
        byte ff = getByte(s, OFF_FF);
        int ffGen = getInt(s, OFF_FFGEN);
        Object states = s.states;
        int cur = getInt(states, OFF_GEN);
        if (ffGen == cur) {
            if (ff == VERDICT_FREE) {
                return true;
            }
            if (ff == VERDICT_HAS) {
                return false;
            }
        }
        // Lazy scan: 4096 cells through the container's own get (demux fast
        // path serves the same values; unknown state = vanilla read).
        PalettedContainer<?> pc = (PalettedContainer<?>) states;
        for (int i = 0; i < 4096; i++) {
            Object state = pc.get(i);
            if (state instanceof net.minecraft.world.level.block.state.BlockState bs
                && !bs.getFluidState().isEmpty()) {
                // Publish (ffGen, ff) ON THE SECTION — write order: epoch
                // first, verdict last. The container epoch itself is NEVER
                // written by the bridge.
                putInt(s, OFF_FFGEN, cur);
                putByte(s, OFF_FF, (byte) VERDICT_HAS);
                return false;
            }
        }
        putInt(s, OFF_FFGEN, cur);
        putByte(s, OFF_FF, (byte) VERDICT_FREE);
        return true;
    }

    private static byte getByte(Object o, long off) {
        return UNSAFE.getByte(o, off);
    }

    private static int getInt(Object o, long off) {
        return UNSAFE.getInt(o, off);
    }

    private static void putByte(Object o, long off, byte v) {
        UNSAFE.putByte(o, off, v);
    }

    private static void putInt(Object o, long off, int v) {
        UNSAFE.putInt(o, off, v);
    }

    /** fluidHeight map view (harness diagnostics). */
    public static Object2DoubleMap<TagKey<Fluid>> fluidHeight(Entity e) {
        return e.fluidHeight;
    }
}
