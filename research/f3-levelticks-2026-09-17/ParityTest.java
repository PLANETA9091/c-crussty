package net.minecraft.server.level;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.function.BiConsumer;

import ca.spottedleaf.moonrise.common.util.CoordinateUtils;
import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.Bootstrap;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.chunk.Strategy;
import net.minecraft.world.level.storage.WritableLevelData;
import net.minecraft.world.ticks.LevelTicks;
import net.minecraft.world.ticks.ScheduledTick;
import net.minecraft.world.ticks.TickPriority;
import sun.misc.Unsafe;

/**
 * F3 LEVELTICKS-READS parity bank (TASK-251 / S7-115) — REAL production
 * entries, plain JVM, NO server boot (INJECTS-ONLY: SharedConstants +
 * Bootstrap.bootStrap = static data only; Unsafe.allocateInstance seams —
 * f2 precedent class of init).
 *
 * Flows under test (run21 kernel classes throughout):
 *   REF  = test-side bytecode-exact mirrors of vanilla
 *          LevelTicks.runCollectedTicks (@0-76), ServerLevel.tickBlock
 *          (@0-53), Level.getBlockState (@0-71) — transcribed from
 *          cfdumps/*.f3dump.txt.
 *   NEW  = TickBlockOps.runCollectedTicks / tickBlock / readBlockState
 *          (package-private — this test lives in the same package on
 *          purpose).
 *
 * The moonrise mid-tick CALL is byte-identical in both flows by construction
 * (same invokeinterface on the same receiver field) and is NOT executed: a
 * bare MinecraftServer has no scheduler internals. Every executed scenario
 * keeps counters off multiples of 8; the branch INDEX arithmetic is compared
 * exactly.
 *
 * Scenarios:
 *   S1 drain order + bookkeeping (queue/set/list), non-empty set
 *   S2 vanilla set-removal QUIRK: !toRunThisTickSet.isEmpty() guard
 *   S3 read parity, no drain window: capture fall-through, outside-height
 *      -> VOID_AIR, palette hits, empty-section AIR short-circuit, neg coords
 *   S4 tickBlock counter progression + branch-index arithmetic
 *   S5 capture branch EXECUTED via real CraftBlockState.getHandle dispatch
 *   S6 full NEW drain over twin levels: read-stream identity + THE WIN —
 *      chunk-resolution count via a counting fullChunks table stub
 *   S9 fuzz 40 randomized scenarios (S6 machine, randomized geometry)
 */
public class ParityTest {

    static Unsafe U;

    static long OFF_LEVELDATA, OFF_MINY, OFF_MAXY, OFF_MINSEC_Y;
    static long OFF_TICKED, OFF_SERVER, OFF_CHUNKSOURCE, OFF_RANDOM;
    static long OFF_QUEUE, OFF_SET, OFF_LIST;
    static long OFF_SECTIONS, OFF_MINSECTION, OFF_NONEMPTY, OFF_STATES;
    static long OFF_HA;             // ChunkAccess.levelHeightAccessor
    static long OFF_FULLCHUNKS, OFF_CBS_DATA;

    static final int MIN_Y = -64, MAX_Y = 320, MIN_SEC_Y = -4;
    static final int SECTIONS = (MAX_Y >> 4) - (MIN_Y >> 4); // 24

    // state constants are LAZY (assigned in main AFTER bootStrap) — a static
    // initializer touching Blocks.* would run registries before bootstrap
    static BlockState STONE, DIRT, DEEPSLATE, AIR, VOID_AIR;

    static long chunkLookups;

    // ================================================================ fixtures

    static final class CountingTable
            extends ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable<LevelChunk> {
        @Override
        public LevelChunk get(long key) {
            chunkLookups++;
            return super.get(key);
        }
    }

    static ServerLevel newLevel(HashMap<Long, LevelChunk> chunks) throws Exception {
        ServerLevel level = (ServerLevel) U.allocateInstance(ServerLevel.class);
        Object proxy = java.lang.reflect.Proxy.newProxyInstance(ParityTest.class.getClassLoader(),
                new Class<?>[] { WritableLevelData.class },
                (p, m, a) -> {
                    Class<?> r = m.getReturnType();
                    if (r == boolean.class) return Boolean.FALSE;
                    if (r == int.class) return 0;
                    if (r == long.class) return 0L;
                    return null;
                });
        U.putObject(level, OFF_LEVELDATA, proxy);
        U.putInt(level, OFF_MINY, MIN_Y);
        U.putInt(level, OFF_MAXY, MAX_Y);
        U.putInt(level, OFF_MINSEC_Y, MIN_SEC_Y);
        U.putLong(level, OFF_TICKED, 0L);
        // server stays null: mid-tick branch never executed (see class doc)
        U.putObject(level, OFF_CHUNKSOURCE, newChunkCache(chunks));
        U.putObject(level, OFF_RANDOM, RandomSource.create(42L));
        level.captureTreeGeneration = false;
        level.capturedBlockStates = new HashMap<>();
        return level;
    }

    static ServerChunkCache newChunkCache(HashMap<Long, LevelChunk> chunks) throws Exception {
        ServerChunkCache cache = (ServerChunkCache) U.allocateInstance(ServerChunkCache.class);
        CountingTable table = new CountingTable();
        for (var e : chunks.entrySet()) table.put(e.getKey(), e.getValue());
        U.putObject(cache, OFF_FULLCHUNKS, table);
        return cache;
    }

    /** Real LevelChunk: full sections array; palette-backed slots per
     *  {@code byWorldPos} ("x,y,z" -> state); everything else keeps
     *  nonEmptyBlockCount==0 (the getBlockStateFinal AIR short-circuit). */
    static LevelChunk newChunk(HashMap<String, BlockState> byWorldPos) throws Exception {
        LevelChunk chunk = (LevelChunk) U.allocateInstance(LevelChunk.class);
        U.putInt(chunk, OFF_MINSECTION, MIN_SEC_Y);
        // section resolution goes through ChunkAccess.levelHeightAccessor
        // (getBlockStateFinal -> getSectionIndex -> getMinSectionY -> getMinY):
        // install a REAL LevelHeightAccessor implementation — the same
        // heights as the level fixture (min -64, height 384)
        U.putObject(chunk, OFF_HA, new net.minecraft.world.level.LevelHeightAccessor() {
            @Override public int getHeight() { return MAX_Y - MIN_Y; }
            @Override public int getMinY() { return MIN_Y; }
        });
        LevelChunkSection[] sections = new LevelChunkSection[SECTIONS];
        @SuppressWarnings("unchecked")
        Strategy<BlockState> strat = (Strategy<BlockState>) (Strategy<?>) Strategy.createForBlockStates(BuiltInRegistries.BLOCK);
        for (int s = 0; s < SECTIONS; s++) {
            LevelChunkSection sec = (LevelChunkSection) U.allocateInstance(LevelChunkSection.class);
            PalettedContainer<BlockState> pc = new PalettedContainer<>(AIR, strat);
            U.putObject(sec, OFF_NONEMPTY, (short) 0);
            U.putObject(sec, OFF_STATES, pc);
            sections[s] = sec;
        }
        for (var e : byWorldPos.entrySet()) {
            String[] p = e.getKey().split(",");
            int x = Integer.parseInt(p[0]), y = Integer.parseInt(p[1]), z = Integer.parseInt(p[2]);
            int si = (y >> 4) - MIN_SEC_Y;
            LevelChunkSection sec = sections[si];
            @SuppressWarnings("unchecked")
            PalettedContainer<BlockState> pc = (PalettedContainer<BlockState>) U.getObject(sec, OFF_STATES);
            pc.set(x & 15, y & 15, z & 15, e.getValue());
            U.putObject(sec, OFF_NONEMPTY, (short) 1);
        }
        U.putObject(chunk, OFF_SECTIONS, sections);
        return chunk;
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    static LevelTicks newTicks(java.util.Queue queue, java.util.Set set, java.util.List list) throws Exception {
        LevelTicks ticks = (LevelTicks) U.allocateInstance(LevelTicks.class);
        U.putObject(ticks, OFF_QUEUE, queue);
        U.putObject(ticks, OFF_SET, set);
        U.putObject(ticks, OFF_LIST, list);
        return ticks;
    }

    static ScheduledTick<Block> t(Block block, BlockPos pos, long order) {
        return new ScheduledTick<>(block, pos, order, TickPriority.NORMAL, 0L);
    }

    // ================================================================ REF mirrors

    /** Vanilla LevelTicks.runCollectedTicks @0-76 bytecode-exact. */
    @SuppressWarnings({"unchecked", "rawtypes"})
    static void refDrain(LevelTicks ticks, BiConsumer accept) {
        java.util.Queue queue = (java.util.Queue) U.getObject(ticks, OFF_QUEUE);
        java.util.Set set = (java.util.Set) U.getObject(ticks, OFF_SET);
        java.util.List list = (java.util.List) U.getObject(ticks, OFF_LIST);
        while (!queue.isEmpty()) {
            Object t = queue.poll();
            if (!set.isEmpty()) set.remove(t);
            list.add(t);
            accept.accept(((ScheduledTick) t).pos(), ((ScheduledTick) t).type());
        }
    }

    /** Vanilla Level.getBlockState @0-71 bytecode-exact (no cache hop). */
    static BlockState refRead(Level level, BlockPos pos) {
        if (level.captureTreeGeneration) {
            org.bukkit.craftbukkit.block.CraftBlockState captured = level.capturedBlockStates.get(pos);
            if (captured != null) return captured.getHandle();
        }
        if (level.isOutsideBuildHeight(pos)) return VOID_AIR;
        ChunkAccess chunk = level.getChunk(pos.getX() >> 4, pos.getZ() >> 4, ChunkStatus.FULL, true);
        return chunk.getBlockState(pos);
    }

    /** Vanilla ServerLevel.tickBlock @0-53 bytecode-exact; the mid-tick CALL
     *  is recorded, not executed (bare MinecraftServer has no scheduler). */
    static void refTickBlock(Level level, BlockPos pos, Block block, ArrayList<String> ev) {
        BlockState state = refRead(level, pos);
        ev.add("read@" + pos.asLong() + "=" + System.identityHashCode(state));
        if (state.is(block)) {
            state.tick((ServerLevel) level, pos, level.random);
            ev.add("tick@" + pos.asLong());
        }
        long c = U.getLong(level, OFF_TICKED) + 1L;
        U.putLong(level, OFF_TICKED, c);
        ev.add("cnt=" + c);
        if ((c & 7L) == 0L) ev.add("midtick@" + c); // recorded, not executed
    }

    // ================================================================ asserts

    static void check(boolean ok, String what) {
        if (!ok) throw new AssertionError("FAIL: " + what);
        System.out.println("  ok: " + what);
    }

    static void checkEq(Object a, Object b, String what) {
        if (!java.util.Objects.equals(a, b))
            throw new AssertionError("FAIL: " + what + "\n  ref : " + a + "\n  new : " + b);
        System.out.println("  ok: " + what);
    }

    // ================================================================ shared world

    /** 3 chunks: (0,0) stone/dirt/deepslate, (-1,-1) stone at neg coords,
     *  (31,0) present but empty (far position, AIR). */
    static HashMap<Long, LevelChunk> world2() throws Exception {
        var chunks = new HashMap<Long, LevelChunk>();
        var c00 = new HashMap<String, BlockState>();
        c00.put("5,64,7", STONE);
        c00.put("5,65,7", DIRT);
        c00.put("6,70,8", DEEPSLATE);
        chunks.put(CoordinateUtils.getChunkKey(0, 0), newChunk(c00));
        var cm1 = new HashMap<String, BlockState>();
        cm1.put("-20,100,-20", STONE);
        chunks.put(CoordinateUtils.getChunkKey(-2, -2), newChunk(cm1)); // -20>>4 = -2 (floor)
        var cfar = new HashMap<String, BlockState>();
        chunks.put(CoordinateUtils.getChunkKey(31, 0), newChunk(cfar));
        return chunks;
    }

    // ================================================================ scenarios

    static void runS1() throws Exception {
        System.out.println("S1 drain order/bookkeeping (set non-empty)");
        var ticks = new ScheduledTick<?>[]{
                t(Blocks.STONE, new BlockPos(5, 64, 7), 1),
                t(Blocks.DIRT, new BlockPos(5, 65, 7), 2),
                t(Blocks.STONE, new BlockPos(-20, 100, -20), 3)};
        var qRef = new java.util.ArrayDeque<ScheduledTick<?>>();
        var qNew = new java.util.ArrayDeque<ScheduledTick<?>>();
        var sRef = new HashSet<ScheduledTick<?>>();
        var sNew = new HashSet<ScheduledTick<?>>();
        for (var tk : ticks) {
            qRef.add(tk); qNew.add(tk);
            sRef.add(tk); sNew.add(tk);
        }
        var tRef = newTicks(qRef, sRef, new ArrayList<>());
        var tNew = newTicks(qNew, sNew, new ArrayList<>());

        var evRef = new ArrayList<String>();
        refDrain(tRef, (pos, type) -> evRef.add(((BlockPos) pos).asLong() + ":" + type));
        var evNew = new ArrayList<String>();
        TickBlockOps.runCollectedTicks(tNew, (pos, type) -> evNew.add(pos.asLong() + ":" + type));

        checkEq(evRef, evNew, "S1 event stream order-exact");
        checkEq(U.getObject(tRef, OFF_LIST).toString(), U.getObject(tNew, OFF_LIST).toString(),
                "S1 alreadyRunThisTick final");
        checkEq(U.getObject(tRef, OFF_SET).toString(), U.getObject(tNew, OFF_SET).toString(),
                "S1 toRunThisTickSet final");
        check(((java.util.Queue<?>) U.getObject(tNew, OFF_QUEUE)).isEmpty(), "S1 queue drained");
    }

    static void runS2() throws Exception {
        System.out.println("S2 vanilla quirk: set removal guarded by !isEmpty()");
        var q = new java.util.ArrayDeque<ScheduledTick<?>>();
        q.add(t(Blocks.STONE, new BlockPos(5, 64, 7), 1));
        var tRef = newTicks(new java.util.ArrayDeque<>(q), new HashSet<>(), new ArrayList<>());
        var tNew = newTicks(new java.util.ArrayDeque<>(q), new HashSet<>(), new ArrayList<>());
        var evRef = new ArrayList<String>();
        var evNew = new ArrayList<String>();
        refDrain(tRef, (pos, type) -> evRef.add(((BlockPos) pos).asLong() + ":" + type));
        TickBlockOps.runCollectedTicks(tNew, (pos, type) -> evNew.add(pos.asLong() + ":" + type));
        checkEq(evRef, evNew, "S2 event stream");
        check(((java.util.List<?>) U.getObject(tNew, OFF_LIST)).size() == 1, "S2 list accumulated (removal skipped)");
        check(((java.util.Set<?>) U.getObject(tNew, OFF_SET)).isEmpty(), "S2 set stayed empty");
    }

    static void runS3() throws Exception {
        System.out.println("S3 read parity (cache==null, real getChunk path)");
        ServerLevel lv = newLevel(world2());
        chunkLookups = 0;
        BlockPos[] grid = {
                new BlockPos(5, 64, 7), new BlockPos(5, 65, 7), new BlockPos(5, 66, 7),  // stone/dirt/air
                new BlockPos(6, 70, 8), new BlockPos(6, 71, 8),                          // deepslate/air
                new BlockPos(-20, 100, -20), new BlockPos(-20, 101, -20),                // neg coords stone/air
                new BlockPos(500, 64, 7),                                                // far empty chunk -> AIR
                new BlockPos(5, MAX_Y + 1, 7), new BlockPos(5, MIN_Y - 1, 7)             // outside -> VOID_AIR
        };
        for (BlockPos p : grid) {
            BlockState ref = refRead(lv, p);
            BlockState neu = TickBlockOps.readBlockState(lv, p);
            check(ref == neu, "S3 identity @ " + p.toShortString() + " -> " + nameOf(ref));
        }
        System.out.println("  chunkLookups=" + chunkLookups + " (real ServerChunkCache.fullChunks path)");
    }

    static String nameOf(BlockState s) {
        if (s == STONE) return "STONE";
        if (s == DIRT) return "DIRT";
        if (s == DEEPSLATE) return "DEEPSLATE";
        if (s == VOID_AIR) return "VOID_AIR";
        if (s == AIR) return "AIR";
        return "?";
    }

    static void runS4() throws Exception {
        System.out.println("S4 tickBlock counter + branch arithmetic");
        long[] preseeds = {1L, 2L, 6L, 8L, 4094L}; // (c+1) & 7 != 0 -> mid-tick never fires
        BlockPos[] grid = {new BlockPos(5, 64, 7), new BlockPos(5, 66, 7), new BlockPos(5, MAX_Y + 1, 7)};
        Block[] blocks = {Blocks.STONE, Blocks.DIRT};
        for (long c0 : preseeds) {
            for (BlockPos p : grid) {
                for (Block b : blocks) {
                    ServerLevel lvA = newLevel(world2());
                    ServerLevel lvB = newLevel(world2());
                    U.putLong(lvA, OFF_TICKED, c0);
                    U.putLong(lvB, OFF_TICKED, c0);
                    var ev = new ArrayList<String>();
                    refTickBlock(lvA, p, b, ev);
                    TickBlockOps.tickBlock(lvB, p, b);
                    long cA = U.getLong(lvA, OFF_TICKED), cB = U.getLong(lvB, OFF_TICKED);
                    if (cA != cB)
                        throw new AssertionError("FAIL: counter diverged c0=" + c0 + " " + p.toShortString() + ": " + cA + " vs " + cB);
                    boolean firedRef = ev.contains("midtick@" + cA);
                    boolean firedNew = (cB & 7L) == 0L;
                    if (firedRef != firedNew)
                        throw new AssertionError("FAIL: branch index diverged c0=" + c0);
                    if (firedRef) throw new AssertionError("test invariant broken: mid-tick fired");
                }
            }
        }
        System.out.println("  ok: 30/30 counter+branch exact (call itself byte-identical by construction)");
    }

    static void runS5() throws Exception {
        System.out.println("S5 captureTreeGeneration branch");
        ServerLevel lv = newLevel(world2());
        lv.captureTreeGeneration = true;
        check(TickBlockOps.readBlockState(lv, new BlockPos(5, 64, 7)) == STONE, "S5a empty capture map falls through");
        // real CraftBlockState via allocateInstance (no ctor — world seams not
        // needed; getHandle() is a trivial data-field read, cfdump-verified)
        org.bukkit.craftbukkit.block.CraftBlockState cbs =
                (org.bukkit.craftbukkit.block.CraftBlockState) U.allocateInstance(org.bukkit.craftbukkit.block.CraftBlockState.class);
        U.putObject(cbs, OFF_CBS_DATA, DIRT);
        lv.capturedBlockStates.put(new BlockPos(5, 64, 7), cbs);
        check(TickBlockOps.readBlockState(lv, new BlockPos(5, 64, 7)) == DIRT, "S5b capture hit -> getHandle dispatch");
        check(refRead(lv, new BlockPos(5, 64, 7)) == DIRT, "S5b REF agrees");
    }

    static void runS6() throws Exception {
        System.out.println("S6 integration drain: read identity + resolution count");
        var ticks = new ScheduledTick<?>[]{
                t(Blocks.STONE, new BlockPos(5, 64, 7), 1),
                t(Blocks.DIRT, new BlockPos(5, 65, 7), 2),
                t(Blocks.STONE, new BlockPos(6, 70, 8), 3),
                t(Blocks.STONE, new BlockPos(5, 64, 7), 4),
                t(Blocks.STONE, new BlockPos(-20, 100, -20), 5),
                t(Blocks.DIRT, new BlockPos(-19, 100, -19), 6)};
        var qRef = new java.util.ArrayDeque<ScheduledTick<?>>();
        var qNew = new java.util.ArrayDeque<ScheduledTick<?>>();
        var sRef = new HashSet<ScheduledTick<?>>();
        var sNew = new HashSet<ScheduledTick<?>>();
        for (var tk : ticks) {
            qRef.add(tk); qNew.add(tk);
            sRef.add(tk); sNew.add(tk);
        }
        var tRef = newTicks(qRef, sRef, new ArrayList<>());
        var tNew = newTicks(qNew, sNew, new ArrayList<>());

        chunkLookups = 0;
        ServerLevel lvRef = newLevel(world2());
        var refReads = new ArrayList<String>();
        refDrain(tRef, (pos, type) ->
                refReads.add(System.identityHashCode(refRead(lvRef, (BlockPos) pos)) + "@" + ((BlockPos) pos).asLong()));
        long refLookups = chunkLookups;

        chunkLookups = 0;
        ServerLevel lvNew = newLevel(world2());
        var newReads = new ArrayList<String>();
        TickBlockOps.runCollectedTicks(tNew, (pos, type) ->
                newReads.add(System.identityHashCode(TickBlockOps.readBlockState(lvNew, (BlockPos) pos)) + "@" + pos.asLong()));
        long newLookups = chunkLookups;

        checkEq(refReads, newReads, "S6 read stream identity (order + state objects)");
        if (newLookups >= refLookups)
            throw new AssertionError("FAIL: no batch win: ref=" + refLookups + " new=" + newLookups);
        System.out.println("  ok: chunk resolutions REF=" + refLookups + " NEW=" + newLookups + " (per-pos -> per-section)");
    }

    static void runS9() throws Exception {
        System.out.println("S9 fuzz 40 scenarios");
        var rnd = new java.util.Random(20260917L);
        Block[] blocks = {Blocks.STONE, Blocks.DIRT, Blocks.DEEPSLATE};
        for (int it = 0; it < 40; it++) {
            int nChunks = 1 + rnd.nextInt(3);
            var palette = new ArrayList<int[]>();
            var chunks = new HashMap<Long, LevelChunk>();
            for (int c = 0; c < nChunks; c++) {
                int cx = rnd.nextInt(5) - 2, cz = rnd.nextInt(5) - 2;
                var states = new HashMap<String, BlockState>();
                int nSolid = 1 + rnd.nextInt(4);
                for (int s = 0; s < nSolid; s++) {
                    int x = cx * 16 + rnd.nextInt(16);
                    int z = cz * 16 + rnd.nextInt(16);
                    int y = MIN_Y + rnd.nextInt(SECTIONS * 16);
                    states.put(x + "," + y + "," + z, blocks[rnd.nextInt(blocks.length)].defaultBlockState());
                    palette.add(new int[]{x, y, z});
                }
                chunks.put(CoordinateUtils.getChunkKey(cx, cz), newChunk(states));
            }
            int nTicks = 3 + rnd.nextInt(3); // 3..5
            var q = new java.util.ArrayDeque<ScheduledTick<?>>();
            var set = new HashSet<ScheduledTick<?>>();
            for (int i = 0; i < nTicks; i++) {
                int[] p = palette.get(rnd.nextInt(palette.size()));
                var tk = t(blocks[rnd.nextInt(blocks.length)], new BlockPos(p[0], p[1], p[2]), i + 1);
                q.add(tk);
                set.add(tk);
            }
            ServerLevel lvRef = newLevel(chunks);
            ServerLevel lvNew = newLevel(chunks);
            var tRef = newTicks(new java.util.ArrayDeque<>(q), new HashSet<>(set), new ArrayList<>());
            var tNew = newTicks(new java.util.ArrayDeque<>(q), new HashSet<>(set), new ArrayList<>());
            var refReads = new ArrayList<String>();
            var newReads = new ArrayList<String>();
            refDrain(tRef, (pos, type) ->
                    refReads.add(System.identityHashCode(refRead(lvRef, (BlockPos) pos)) + "@" + ((BlockPos) pos).asLong()));
            TickBlockOps.runCollectedTicks(tNew, (pos, type) ->
                    newReads.add(System.identityHashCode(TickBlockOps.readBlockState(lvNew, (BlockPos) pos)) + "@" + ((BlockPos) pos).asLong()));
            if (!refReads.equals(newReads))
                throw new AssertionError("FAIL fuzz #" + it + ": read streams diverged\n" + refReads + "\n" + newReads);
            long c0 = 8L * rnd.nextInt(100) + 2L; // 2 (mod 8): c+1..c+5 stay off multiples of 8
            U.putLong(lvRef, OFF_TICKED, c0);
            U.putLong(lvNew, OFF_TICKED, c0);
            var ev = new ArrayList<String>();
            for (var tk : q) {
                refTickBlock(lvRef, (BlockPos) tk.pos(), (Block) tk.type(), ev);
                TickBlockOps.tickBlock(lvNew, (BlockPos) tk.pos(), (Block) tk.type());
            }
            if (U.getLong(lvRef, OFF_TICKED) != U.getLong(lvNew, OFF_TICKED))
                throw new AssertionError("FAIL fuzz #" + it + ": counter diverged");
        }
        System.out.println("  ok: 40/40 scenarios identical");
    }

    // ================================================================ main

    public static void main(String[] args) throws Exception {
        Field uf = Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        U = (Unsafe) uf.get(null);

        OFF_LEVELDATA = U.objectFieldOffset(Level.class.getDeclaredField("levelData"));
        OFF_MINY = U.objectFieldOffset(Level.class.getDeclaredField("minY"));
        OFF_MAXY = U.objectFieldOffset(Level.class.getDeclaredField("maxY"));
        OFF_MINSEC_Y = U.objectFieldOffset(Level.class.getDeclaredField("minSectionY"));
        OFF_TICKED = U.objectFieldOffset(ServerLevel.class.getDeclaredField("tickedBlocksOrFluids"));
        OFF_SERVER = U.objectFieldOffset(ServerLevel.class.getDeclaredField("server"));
        OFF_CHUNKSOURCE = U.objectFieldOffset(ServerLevel.class.getDeclaredField("chunkSource"));
        OFF_RANDOM = U.objectFieldOffset(Level.class.getDeclaredField("random"));
        OFF_QUEUE = U.objectFieldOffset(LevelTicks.class.getDeclaredField("toRunThisTick"));
        OFF_SET = U.objectFieldOffset(LevelTicks.class.getDeclaredField("toRunThisTickSet"));
        OFF_LIST = U.objectFieldOffset(LevelTicks.class.getDeclaredField("alreadyRunThisTick"));
        OFF_SECTIONS = U.objectFieldOffset(ChunkAccess.class.getDeclaredField("sections"));
        OFF_MINSECTION = U.objectFieldOffset(ChunkAccess.class.getDeclaredField("minSection"));
        OFF_NONEMPTY = U.objectFieldOffset(LevelChunkSection.class.getDeclaredField("nonEmptyBlockCount"));
        OFF_STATES = U.objectFieldOffset(LevelChunkSection.class.getDeclaredField("states"));
        OFF_HA = U.objectFieldOffset(ChunkAccess.class.getDeclaredField("levelHeightAccessor"));
        OFF_FULLCHUNKS = U.objectFieldOffset(ServerChunkCache.class.getDeclaredField("fullChunks"));
        OFF_CBS_DATA = U.objectFieldOffset(org.bukkit.craftbukkit.block.CraftBlockState.class.getDeclaredField("data"));

        // static-data init only (INJECTS-ONLY, f2 precedent)
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        System.out.println("SharedConstants + Bootstrap.bootStrap OK (registries+codecs, no server)");

        STONE = Blocks.STONE.defaultBlockState();
        DIRT = Blocks.DIRT.defaultBlockState();
        DEEPSLATE = Blocks.DEEPSLATE.defaultBlockState();
        AIR = Blocks.AIR.defaultBlockState();
        VOID_AIR = Blocks.VOID_AIR.defaultBlockState();

        runS1();
        runS2();
        runS3();
        runS4();
        runS5();
        runS6();
        runS9();
        System.out.println("F3 READS PARITY: PASS");
    }
}
