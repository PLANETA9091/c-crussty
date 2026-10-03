package net.minecraft.server.level;

import ca.spottedleaf.moonrise.patches.chunk_system.server.ChunkSystemMinecraftServer;
import it.unimi.dsi.fastutil.longs.Long2LongMap;
import it.unimi.dsi.fastutil.longs.Long2LongMaps;
import it.unimi.dsi.fastutil.longs.Long2ObjectMap;
import it.unimi.dsi.fastutil.objects.ObjectIterator;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.profiling.ProfilerFiller;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.ticks.LevelChunkTicks;
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
import java.util.function.LongPredicate;

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
    private static final long OFF_ALL_CONT;     // LevelTicks.allContainers (private final Long2ObjectMap)
    private static final long OFF_NEXT_TICK;    // LevelTicks.nextTickForContainer (private final Long2LongMap)
    private static final long OFF_TO_TICK;      // LevelTicks.containersToTick (private final Queue)
    private static final long OFF_TICK_CHECK;   // LevelTicks.tickCheck (private final LongPredicate)

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
            OFF_ALL_CONT = U.objectFieldOffset(LevelTicks.class.getDeclaredField("allContainers"));
            OFF_NEXT_TICK = U.objectFieldOffset(LevelTicks.class.getDeclaredField("nextTickForContainer"));
            OFF_TO_TICK = U.objectFieldOffset(LevelTicks.class.getDeclaredField("containersToTick"));
            OFF_TICK_CHECK = U.objectFieldOffset(LevelTicks.class.getDeclaredField("tickCheck"));
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

    // ------------------------------------------------- collect (hook 3, F3-queue)

    /**
     * Bytecode-exact replacement of LevelTicks.collectTicks (javap @0-34, run21
     * kernel) — the FUSED sort+counter+drain+reschedule pipeline:
     *
     *   sortContainersToTick        (vanilla @0-165, walk replicated exactly)
     *   incrementCounter            (vanilla @5-24, same call on the passed filler)
     *   drainContainers             (vanilla @0-105, gate machinery inlined)
     *   rescheduleLeftoverContainers (vanilla @0-40, walk replicated exactly)
     *
     * WIN MECHANICS (preregistered ≤0.5% queue slice, "primitive open-addressing
     * drain" line of the F3 family): canScheduleMoreTicks — a virtual call +
     * Queue.size() at 3 sites per drained tick in vanilla — becomes a direct
     * field compare on the Unsafe-fetched toRunThisTick; scheduleForThisTick —
     * a virtual dispatch per scheduled tick — becomes a direct add; the four
     * private fields are fetched ONCE per pipeline instead of per getfield.
     * Everything else is the vanilla loop structure byte-for-byte, including:
     * the sort-walk quirks (remove only on missing container / null peek,
     * setValue reschedule on late peek, tickCheck-false leaves the entry
     * in place), the frozen-innerHead INTRA_TICK_DRAIN_ORDER check (computed
     * ONCE per container from the main-queue head — vanilla never refreshes it
     * inside the inner loop), and the requeue-vs-reschedule branch order.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static void collectTicks(LevelTicks<?> ticks, long gameTime, int maxTicks,
                                    ProfilerFiller profiler) {
        final Long2ObjectMap<LevelChunkTicks<?>> allContainers =
                (Long2ObjectMap) U.getObject(ticks, OFF_ALL_CONT);
        final Long2LongMap nextTickForContainer =
                (Long2LongMap) U.getObject(ticks, OFF_NEXT_TICK);
        final Queue<LevelChunkTicks<?>> containersToTick =
                (Queue) U.getObject(ticks, OFF_TO_TICK);
        final LongPredicate tickCheck = (LongPredicate) U.getObject(ticks, OFF_TICK_CHECK);
        final Queue<ScheduledTick<?>> toRunThisTick = (Queue) U.getObject(ticks, OFF_QUEUE);

        // ---- sortContainersToTick (vanilla @0-165) ----
        final ObjectIterator<Long2LongMap.Entry> it = Long2LongMaps.fastIterator(nextTickForContainer);
        while (it.hasNext()) {                                   // @8-14
            final Long2LongMap.Entry e = it.next();              // @17-23
            final long key = e.getLongKey();                     // @28-35
            final long at = e.getLongValue();                    // @37-44
            if (at > gameTime) {                                 // @46-50 ifgt 162
                continue;
            }
            final LevelChunkTicks<?> container =
                    (LevelChunkTicks<?>) allContainers.get(key); // @53-64
            if (container == null) {                             // @69-80
                it.remove();
                continue;
            }
            final ScheduledTick<?> peek = container.peek();      // @83-88
            if (peek == null) {                                  // @90-101
                it.remove();
                continue;
            }
            if (peek.triggerTick() > gameTime) {                 // @104-113
                e.setValue(peek.triggerTick());                  // @114-125 (reschedule, keep entry)
                continue;
            }
            if (tickCheck.test(key)) {                           // @130-141 (false: entry STAYS)
                it.remove();                                     // @144-150
                containersToTick.add(container);                 // @150-156
            }
        }

        // ---- collectTicks counter (vanilla @5-24) ----
        profiler.incrementCounter("containersToTick", containersToTick.size());

        // ---- drainContainers (vanilla @0-105, gates inlined) ----
        while (toRunThisTick.size() < maxTicks) {                // @0-5 canScheduleMoreTicks
            final LevelChunkTicks<?> container = containersToTick.poll();   // @8-20
            if (container == null) {                             // @21-23 ifnull 105
                break;
            }
            toRunThisTick.add(container.poll());                 // @26-38 (poll + scheduleForThisTick inlined)
            drainFromCurrentContainer(containersToTick, container,
                    gameTime, maxTicks, toRunThisTick);          // @39-50
            final ScheduledTick<?> peek = container.peek();      // @51-56
            if (peek != null) {                                  // @58-60 ifnull 102
                if (peek.triggerTick() <= gameTime                       // @63-70
                        && toRunThisTick.size() < maxTicks) {            // @73-78 canScheduleMoreTicks
                    containersToTick.add(container);             // @81-92 (requeue)
                } else {
                    nextTickForContainer.put(ChunkPos.asLong(peek.pos()), peek.triggerTick());   // @96-99 updateContainerScheduling
                }
            }
        }

        // ---- rescheduleLeftoverContainers (vanilla @0-40) ----
        for (final LevelChunkTicks<?> c : containersToTick) {    // @0-28
            final ScheduledTick<?> peek = c.peek();              // @31-34 (NPE on null — same as vanilla)
            nextTickForContainer.put(ChunkPos.asLong(peek.pos()), peek.triggerTick());
        }
    }

    /**
     * Vanilla drainFromCurrentContainer (@0-103) byte-for-byte. The
     * INTRA_TICK_DRAIN_ORDER check compares against the FROZEN innerHead
     * (the next container's head tick, captured ONCE before the loop) —
     * vanilla never refreshes it, the mirror must not either.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    private static void drainFromCurrentContainer(final Queue containersToTick,
            final LevelChunkTicks<?> container, final long gameTime, final int maxTicks,
            final Queue toRunThisTick) {
        if (toRunThisTick.size() >= maxTicks) {                  // @0-6
            return;
        }
        final LevelChunkTicks<?> nextContainer = (LevelChunkTicks<?>) containersToTick.peek();   // @9-18
        final ScheduledTick<?> innerHead = (nextContainer != null)       // @20-34 (frozen)
                ? nextContainer.peek() : null;
        while (toRunThisTick.size() < maxTicks) {                // @36-42
            final ScheduledTick<?> next = container.peek();      // @45-49
            if (next == null) {                                  // @51-53
                return;
            }
            if (next.triggerTick() > gameTime) {                 // @56-63
                return;
            }
            if (innerHead != null                                // @66-86
                    && ScheduledTick.INTRA_TICK_DRAIN_ORDER.compare(next, innerHead) > 0) {
                return;
            }
            container.poll();                                    // @89-93
            toRunThisTick.add(next);                             // @94-97 (scheduleForThisTick)
        }
    }
}
