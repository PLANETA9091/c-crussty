package net.minecraft.server.level;

import ca.spottedleaf.moonrise.common.PlatformHooks;
import ca.spottedleaf.moonrise.common.list.ShortList;
import ca.spottedleaf.moonrise.common.util.SimpleThreadUnsafeRandom;
import ca.spottedleaf.moonrise.common.util.WorldUtil;
import ca.spottedleaf.moonrise.patches.block_counting.BlockCountingChunkSection;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.material.FluidState;
import sun.misc.Unsafe;

/**
 * F1 BATCH-RNG helper (family-agg pack member F1, TASK-247 / S7-111).
 *
 * Replaces the body of ServerLevel.optimiseRandomTick(LevelChunk, int) with an
 * inlined-LCG pick loop. Bit-exact parity contract:
 *
 *  - SimpleThreadUnsafeRandom.advanceSeed(): value = (value * 25214903917 + 11) & (2^48-1)
 *  - SimpleThreadUnsafeRandom.nextInt():     (int)(advanceSeed() >>> 16)
 *    (verified by cfdump on the materialized kernel, build 2025-12-11:
 *     MULTIPLIER=25214903917, ADDEND=11, MASK=2^48-1, nextInt = seed>>>16 as int)
 *
 *  - The pick loop keeps the 48-bit seed in a LOCAL (register-resident) long.
 *    Sync with the object happens ONLY at body-call boundaries via direct
 *    Unsafe get/put on the private field `value`:
 *      put  before each body call (object must serve the body the same stream),
 *      get  after each body call  (body consumed an unknown number of draws),
 *      put  once at the end       (stream position must survive for the rest
 *                                  of the tick — other simpleRandom consumers).
 *    We deliberately BYPASS setSeed(long): it calls gaussianSource.reset(),
 *    which would discard a cached gaussian spare the original flow keeps —
 *    direct field write leaves the gaussian cache untouched, matching the
 *    original (setSeed is never called mid-loop by the original code).
 *
 *  - Per rejected pick (idx >= ticking.size()) the fast path performs ZERO
 *    virtual dispatch and ZERO object-field traffic: 1 lmul + 1 ladd + 1 land
 *    + shift/cast, all on a local. This is the F1 replacement target sized by
 *    TASK-233 at 1.6-1.9% MSPT (the nextInt+advanceSeed frame cost).
 *
 * Transcribed instruction-by-instruction from
 * research/rng-recon-2026-09-16/optimiseRandomTick.javap (Purpur 1.21.10,
 * mojang-mapped build 2025-12-11):
 *   flag   = !PlatformHooks.get().configFixMC224294()          (@20-36)
 *   baseX  = chunkPos.x << 4;  baseZ = chunkPos.z << 4          (@44-60)
 *   per section: yBase = (si + minSection) << 4                 (@76-83)
 *   skip if !section.isRandomlyTickingBlocks()                  (@98-106)
 *   ticking = ((BlockCountingChunkSection)section).moonrise$getTickingBlockList()
 *   per attempt: size = ticking.size(); idx = nextInt() & 4095  (@127-143)
 *   skip if idx >= size                                          (@145-152)
 *   packed = ticking.getRaw(idx) & 0xFFFF                        (@155-166)
 *   state  = (BlockState) states.get(packed)                     (@168-178)
 *   pos    = new BlockPos((packed & 15) | baseX,
 *                         ((packed >>> 8) & 15) | yBase,
 *                         ((packed >>> 4) & 15) | baseZ)         (@180-215)
 *   state.randomTick(level, pos, random)                         (@218-230)
 *   if (flag) { fs = state.getFluidState(); if (fs.isRandomlyTicking())
 *               fs.randomTick(level, pos, random); }             (@231-263)
 *
 * `states` is PUBLIC final on LevelChunkSection (flags 0x0011, cfdump-verified),
 * so no package-private access is needed anywhere in this helper; the private
 * `simpleRandom` field of ServerLevel is passed IN by the patched body (the
 * body lives inside ServerLevel and may read its own field).
 */
public final class RandomTickOps {

    private RandomTickOps() {}

    private static final Unsafe UNSAFE;
    private static final long VALUE_OFFSET;

    static {
        try {
            java.lang.reflect.Field uf = Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (Unsafe) uf.get(null);
            java.lang.reflect.Field vf =
                SimpleThreadUnsafeRandom.class.getDeclaredField("value");
            vf.setAccessible(true);
            VALUE_OFFSET = UNSAFE.objectFieldOffset(vf);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    // Bit-exact LCG constants (cfdump of SimpleThreadUnsafeRandom, see header).
    private static final long MULTIPLIER = 25214903917L;
    private static final long ADDEND = 11L;
    private static final long MASK = (1L << 48) - 1L;

    public static void run(final ServerLevel level, final LevelChunk chunk, final int ticks,
                           final SimpleThreadUnsafeRandom random) {
        final LevelChunkSection[] sections = chunk.getSections();
        final int minSection = WorldUtil.getMinSection(level);
        final boolean doFluids = !PlatformHooks.get().configFixMC224294();
        final ChunkPos cpos = chunk.getPos();
        final int baseX = cpos.x << 4;
        final int baseZ = cpos.z << 4;

        // Local authoritative seed — register-resident across picks.
        long seed = UNSAFE.getLong(random, VALUE_OFFSET);

        for (int si = 0; si < sections.length; si++) {
            final int yBase = (si + minSection) << 4;
            final LevelChunkSection section = sections[si];
            if (!section.isRandomlyTickingBlocks()) continue;
            final PalettedContainer states = section.states;
            final ShortList ticking =
                ((BlockCountingChunkSection) section).moonrise$getTickingBlockList();

            for (int t = 0; t < ticks; t++) {
                final int size = ticking.size();
                // inlined nextInt() & 4095 — bit-exact advanceSeed + shift
                seed = (seed * MULTIPLIER + ADDEND) & MASK;
                final int idx = (int) (seed >>> 16) & 4095;
                if (idx >= size) continue; // fast path: no dispatch, no field traffic

                // hit: sync the object to our stream position, let the body
                // consume, then pull the advanced position back.
                UNSAFE.putLong(random, VALUE_OFFSET, seed);
                final int packed = ticking.getRaw(idx) & 0xFFFF;
                final BlockState state = (BlockState) states.get(packed);
                final BlockPos pos = new BlockPos(
                    (packed & 15) | baseX,
                    ((packed >>> 8) & 15) | yBase,
                    ((packed >>> 4) & 15) | baseZ);
                state.randomTick(level, pos, random);
                if (doFluids) {
                    final FluidState fs = state.getFluidState();
                    if (fs.isRandomlyTicking()) {
                        fs.randomTick(level, pos, random);
                    }
                }
                seed = UNSAFE.getLong(random, VALUE_OFFSET);
            }
        }

        // Preserve the stream position for the rest of the tick (the object
        // must reflect ALL consumed picks, including rejected ones).
        UNSAFE.putLong(random, VALUE_OFFSET, seed);
    }
}
