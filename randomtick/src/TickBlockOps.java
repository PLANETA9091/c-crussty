package net.minecraft.server.level;

import ca.spottedleaf.moonrise.patches.chunk_system.server.ChunkSystemMinecraftServer;
import net.minecraft.core.BlockPos;
import net.minecraft.server.MinecraftServer;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.ticks.LevelTicks;
import net.minecraft.world.ticks.ScheduledTick;
import org.bukkit.craftbukkit.block.CraftBlockState;
import sun.misc.Unsafe;

import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.List;
import java.util.Queue;
import java.util.Set;
import java.util.function.BiConsumer;

/**
 * F3 LEVELTICKS-READS helper (family-agg pack member F3, TASK-251/S7-115;
 * protocol: docs/FAMILY_AGG_PREREGISTRATION.md §5 — F3-reads slice, task167:
 * batch-by-section resolve saves the getChunk/hash part of the block-state
 * reads slice, ~0.3-0.5% of the tick; everything else per-query irreducible).
 *
 * TWO byte hooks land on these entry points (next increment, patch_update /
 * patch_optimise_random_tick / patch_brain_start_each machine):
 *
 *   1. LevelTicks.runCollectedTicks — body swap to
 *      invokestatic TickBlockOps.runCollectedTicks(LevelTicks, BiConsumer).
 *      The helper REPLICATES the vanilla drain bytecode-exactly
 *      (LevelTicks.javap @0-76, run21 kernel):
 *        while (!toRunThisTick.isEmpty()) {
 *            t = (ScheduledTick) toRunThisTick.poll();
 *            if (!toRunThisTickSet.isEmpty()) toRunThisTickSet.remove(t);
 *            alreadyRunThisTick.add(t);
 *            biConsumer.accept(t.pos(), t.type());
 *        }
 *      including the vanilla QUIRK that the set-removal is guarded by
 *      !toRunThisTickSet.isEmpty(). The private fields are read through
 *      Unsafe at cached offsets (they are final — the refs cannot change
 *      mid-drain, one read is bytecode-equivalent to per-iteration getfield).
 *      The helper additionally opens a per-drain SECTION CACHE window
 *      (ThreadLocal, set before the loop, removed in finally) that
 *      tickBlock consults — see below.
 *
 *   2. ServerLevel.tickBlock — body swap to
 *      invokestatic TickBlockOps.tickBlock(ServerLevel, BlockPos, Block).
 *      The helper replicates the vanilla body (tickBlock.javap @0-53,
 *      run21 kernel) with the read routed through getBlockState-lens:
 *        captureTreeGeneration branch (@0-29, verbatim, checked FIRST),
 *        isOutsideBuildHeight -> VOID_AIR (@30-44, verbatim),
 *        getChunk(x>>4, z>>4, FULL, true) — REPLACED by a per-drain
 *        section->chunk cache: the FIRST tick at a section pays the exact
 *        vanilla getChunk call, subsequent ticks at the SAME section skip
 *        it entirely (the batch win). ChunkAccess.getBlockState(pos) and
 *        everything below stays a real call on the resolved chunk.
 *        Outside a drain window (cache == null) the read is the EXACT
 *        vanilla path — identical semantics, merely unbatched; tickBlock
 *        is only ever invoked from the drain lambda, so the null-cache
 *        branch exists for constructor-time parity safety, not as an
 *        alternate production path.
 *      Then state.is(block) -> state.tick(level, pos, level.random)
 *      (@6-24, Level.random is public final — direct read), the
 *      tickedBlocksOrFluids counter increment (@24-43, private long via
 *      Unsafe get/put), and the mid-tick yield gate (@44-53): when
 *      (counter & 7) == 0 the SAME interface call
 *      ChunkSystemMinecraftServer.moonrise$executeMidTickTasks() fires on
 *      the private server field (Unsafe read) — byte-identical callee, so
 *      the mid-tick semantics are carried over unchanged.
 *
 * NOTHING LANDS before the single aggregate A/B (§5 protocol): dormant
 * until hooked + armed, verdict at the pack A/B against the banked pair
 * 76.01/76.98; parity contract banked in
 * research/f3-levelticks-2026-09-17/parity_output.txt (REAL production
 * entries, plain JVM, INJECTS-ONLY — no boot).
 */
public final class TickBlockOps {

    private TickBlockOps() {}

    // ------------------------------------------------------------- offsets

    private static final Unsafe U;
    private static final long OFF_MIN_Y;        // Level.minY (private int)
    private static final long OFF_MAX_Y;        // Level.maxY (private int)
    private static final long OFF_TICKED;       // ServerLevel.tickedBlocksOrFluids (private long)
    private static final long OFF_SERVER;       // ServerLevel.server (private final MinecraftServer)
    private static final long OFF_QUEUE;        // LevelTicks.toRunThisTick (private final Queue)
    private static final long OFF_SET;          // LevelTicks.toRunThisTickSet (private final Set)
    private static final long OFF_LIST;         // LevelTicks.alreadyRunThisTick (private final List)

    static {
        try {
            Field f = Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            U = (Unsafe) f.get(null);
            OFF_MIN_Y = U.objectFieldOffset(Level.class.getDeclaredField("minY"));
            OFF_MAX_Y = U.objectFieldOffset(Level.class.getDeclaredField("maxY"));
            OFF_TICKED = U.objectFieldOffset(ServerLevel.class.getDeclaredField("tickedBlocksOrFluids"));
            OFF_SERVER = U.objectFieldOffset(ServerLevel.class.getDeclaredField("server"));
            OFF_QUEUE = U.objectFieldOffset(LevelTicks.class.getDeclaredField("toRunThisTick"));
            OFF_SET = U.objectFieldOffset(LevelTicks.class.getDeclaredField("toRunThisTickSet"));
            OFF_LIST = U.objectFieldOffset(LevelTicks.class.getDeclaredField("alreadyRunThisTick"));
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    // --------------------------------------------------- drain section cache

    /**
     * Per-drain section->chunk cache. Opened by {@link #runCollectedTicks}
     * for the duration of the drain, consulted by {@link #tickBlock}.
     * Key = (chunkX << 32) | (chunkZ & 0xFFFFFFFFL) — CoordinateUtils layout.
     */
    private static final ThreadLocal<HashMap<Long, ChunkAccess>> DRAIN_CACHE = new ThreadLocal<>();

    private static long sectionKey(int cx, int cz) {
        return ((long) (cx & 0xFFFFFFFFL) << 32) | (cz & 0xFFFFFFFFL);
    }

    // ------------------------------------------------------- drain (hook 1)

    /**
     * Bytecode-exact replacement of LevelTicks.runCollectedTicks
     * (LevelTicks.javap @0-76, run21 kernel) plus the section-cache window.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static void runCollectedTicks(LevelTicks<?> ticks, BiConsumer<BlockPos, Object> accept) {
        final Queue<ScheduledTick<?>> queue =
                (Queue) U.getObject(ticks, OFF_QUEUE);
        final Set<ScheduledTick<?>> set =
                (Set) U.getObject(ticks, OFF_SET);
        final List<ScheduledTick<?>> list =
                (List) U.getObject(ticks, OFF_LIST);
        DRAIN_CACHE.set(new HashMap<>());
        try {
            while (!queue.isEmpty()) {                          // vanilla @0-9
                ScheduledTick<?> t = (ScheduledTick<?>) queue.poll();   // @12-24
                if (!set.isEmpty()) {                           // @25-34 (vanilla quirk: guarded)
                    set.remove(t);                              // @37-47
                }
                list.add(t);                                    // @48-58
                accept.accept((BlockPos) t.pos(), t.type());    // @59-68
            }
        } finally {
            DRAIN_CACHE.remove();
        }
    }

    // ------------------------------------------------- tickBlock (hook 2)

    /**
     * Bytecode-exact replacement of ServerLevel.tickBlock
     * (tickBlock.javap @0-53, run21 kernel) with the section-cached read.
     */
    public static void tickBlock(ServerLevel level, BlockPos pos, Block block) {
        final BlockState state = readBlockState(level, pos);    // vanilla @0-5 (getBlockState)
        if (state.is(block)) {                                  // vanilla @6-11
            state.tick(level, pos, level.random);               // vanilla @14-23 (random: public final)
        }
        long counter = U.getLong(level, OFF_TICKED) + 1L;       // vanilla @24-34
        U.putLong(level, OFF_TICKED, counter);
        if ((counter & 7L) == 0L) {                             // vanilla @35-41 (inverted guard)
            MinecraftServer server = (MinecraftServer) U.getObject(level, OFF_SERVER);
            ((ChunkSystemMinecraftServer) server).moonrise$executeMidTickTasks();   // vanilla @44-53
        }
    }

    /**
     * Level.getBlockState verbatim (Level.f3dump @0-71, run21 kernel) with
     * the getChunk hop routed through the per-drain section cache.
     */
    static BlockState readBlockState(Level level, BlockPos pos) {
        if (level.captureTreeGeneration) {                      // vanilla @0-4 (public field)
            CraftBlockState captured = level.capturedBlockStates.get(pos);   // @7-17
            if (captured != null) {                             // @21
                return captured.getHandle();                    // @25-29
            }
        }
        if (level.isOutsideBuildHeight(pos)) {                  // vanilla @30-35
            return Blocks.VOID_AIR.defaultBlockState();         // @38-44
        }
        final int cx = pos.getX() >> 4;                         // @45-58
        final int cz = pos.getZ() >> 4;
        final HashMap<Long, ChunkAccess> cache = DRAIN_CACHE.get();
        final ChunkAccess chunk;
        if (cache == null) {
            chunk = level.getChunk(cx, cz, ChunkStatus.FULL, true);   // vanilla @62 (exact call)
        } else {
            final Long key = sectionKey(cx, cz);
            final ChunkAccess cached = cache.get(key);
            if (cached != null) {
                chunk = cached;                                 // THE BATCH WIN: no getChunk
            } else {
                chunk = level.getChunk(cx, cz, ChunkStatus.FULL, true);   // once per section
                cache.put(key, chunk);
            }
        }
        return chunk.getBlockState(pos);                        // vanilla @66-71 (real call)
    }
}
