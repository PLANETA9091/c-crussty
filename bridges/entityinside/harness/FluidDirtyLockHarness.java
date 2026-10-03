package net.minecraft.world.entity;

import java.lang.reflect.Field;
import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.server.Bootstrap;
import net.minecraft.tags.TagKey;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.LiquidBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.chunk.UpgradeData;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.lighting.LevelLightEngine;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.storage.WritableLevelData;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.ticks.LevelChunkTicks;
import sun.misc.Unsafe;

/**
 * FLUID-DIRTY behavioral lockstep harness (S7-152, TASK-291 — tier 2,
 * §S7-150-G5): bit-exact differential of the REAL vanilla scan
 * (Entity.updateFluidHeightAndDoFluidPushing — kernel bytecode, untouched)
 * against the LEVER (FluidPushOps.scan) on identical fixtures.
 *
 * Fixture strategy ("scan-contract fixture"): the vanilla Level ctor is
 * bukkit-hostile offline (checkcast ServerLevel + SpigotWorldConfig +
 * CraftWorld inside the ctor — Level disasm 1222-1297), so the Level and
 * the Entities are allocated via Unsafe.allocateInstance (ctors skipped)
 * and EXACTLY the fields the javap scan-contract reads are set
 * (research/fluid-dirty-2026-09-18/step0_*.txt):
 *   Entity: level, bb, id, deltaMovement (+posLock non-null — paper
 *           synchronized setter), fluidHeight map, lastLavaContact,
 *           firstTick=false; isPushedByFluid() = constant true (base impl);
 *           entityData/syncher NOT on the scan path.
 *   Level:  minY/maxY/minSectionY/maxSectionY/sectionsCount (finals —
 *           Unsafe writes), isClientSide=false, captureTreeGeneration=false
 *           (zeroed), levelData = interface proxy; the 20 abstract members
 *           + getChunkSource() are implemented by the subclass.
 *   Chunks: REAL LevelChunk via the REAL direct ctor over REAL
 *           LevelChunkSections (FluidFreeHarness fixture lineage). World
 *           filling goes through the REAL LevelChunkSection.setBlockState
 *           (the same delegate secWrite calls — bypassing only the
 *           LevelChunk bukkit periphery).
 * Everything the scan calls is real kernel code: touchingUnloadedChunk →
 * hasChunksAt → moonrise$areChunksLoaded → chunkSource.hasChunk, the triple
 * loop, PalettedContainer.get, FluidState.getHeight/getFlow (neighbor reads
 * → level.getFluidState → real chunk sections).
 *
 * VERDICT = bit-exact on: return value, fluidHeight raw bits (both tags),
 * deltaMovement raw bits, lastLavaContact coords. Plus memo tier checks:
 * HIT on repeated identical calls, MISS + fresh-world parity after a
 * mutation storm (source→flowing, flowing→air, water→stone, stone→water,
 * air→stone no-bump), unloaded-guard parity, WATER/LAVA scale parity
 * (0.014 / 0.007 / 0.0023333…).
 *
 * Exit 0 = FLUID-DIRTY LOCKSTEP PASS (G5 core); any failure throws.
 */
public final class FluidDirtyLockHarness {

    private static final Unsafe U;
    static {
        try {
            Field f = Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            U = (Unsafe) f.get(null);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    private static final int CH_RANGE = 8;  // чанки [0..CH_RANGE) x [0..CH_RANGE) загружены
    private static final double[] SCALES = {0.014d, 0.007d, 0.0023333333333333335d};

    // ---------------- Level stub ----------------

    static final class MiniLevel extends Level {
        MiniChunkSource src;

        MiniLevel() {
            // never runs (Unsafe.allocateInstance in main)
            super(null, null, null, null, false, false, 0L, 0, null, null, null, null, null);
        }

        @Override
        public ChunkSource getChunkSource() {
            return src;
        }

        @Override
        public net.minecraft.server.level.ServerLevel getMinecraftWorld() {
            return null; // bukkit-периферия, scan path не зовёт
        }

        @Override
        public long nextSubTickCount() {
            return 0L;
        }

        @Override
        public net.minecraft.world.level.storage.LevelData getLevelData() {
            return null; // scan path не зовёт
        }

        @Override
        public net.minecraft.world.DifficultyInstance getCurrentDifficultyAt(net.minecraft.core.BlockPos pos) {
            return null; // scan path не зовёт
        }

        @Override
        public net.minecraft.server.MinecraftServer getServer() {
            return null; // scan path не зовёт
        }

        @Override
        public net.minecraft.util.RandomSource getRandom() {
            return net.minecraft.util.RandomSource.create();
        }

        @Override
        public void playSound(net.minecraft.world.entity.Entity source, BlockPos pos,
                net.minecraft.sounds.SoundEvent sound, net.minecraft.sounds.SoundSource src2,
                float vol, float pitch) {
            // no-op (не на скан-пути)
        }

        @Override
        public void addParticle(net.minecraft.core.particles.ParticleOptions particle,
                double x, double y, double z, double dx, double dy, double dz) {
            // no-op (не на скан-пути)
        }

        @Override
        public void levelEvent(net.minecraft.world.entity.Entity source, int event,
                BlockPos pos, int data) {
            // no-op (не на скан-пути)
        }

        @Override
        public void gameEvent(net.minecraft.core.Holder<net.minecraft.world.level.gameevent.GameEvent> event,
                net.minecraft.world.phys.Vec3 pos,
                net.minecraft.world.level.gameevent.GameEvent.Context context) {
            // no-op (не на скан-пути)
        }

        @Override
        public java.util.List<? extends net.minecraft.world.entity.player.Player> players() {
            return java.util.List.of(); // scan path не зовёт
        }

        @Override
        public int getHeight(net.minecraft.world.level.levelgen.Heightmap.Types type, int x, int z) {
            return 0; // scan path не зовёт
        }

        @Override
        public int getSkyDarken() {
            return 0;
        }

        @Override
        public net.minecraft.world.level.biome.BiomeManager getBiomeManager() {
            return null; // scan path не зовёт
        }

        @Override
        public boolean isClientSide() {
            return false;
        }

        @Override
        public int getSeaLevel() {
            return 63;
        }

        @Override
        public net.minecraft.world.level.dimension.DimensionType dimensionType() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.core.RegistryAccess registryAccess() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.flag.FeatureFlagSet enabledFeatures() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public float getShade(net.minecraft.core.Direction direction, boolean shade) {
            return 1.0f; // scan path не зовёт
        }

        @Override
        public net.minecraft.world.level.border.WorldBorder getWorldBorder() {
            return new net.minecraft.world.level.border.WorldBorder(); // не на скан-пути
        }

        @Override
        public net.minecraft.world.ticks.LevelTickAccess<Fluid> getFluidTicks() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.ticks.LevelTickAccess<Block> getBlockTicks() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem> getTypeKey() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> getUncachedNoiseBiome(int x, int y, int z) {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public void sendBlockUpdated(BlockPos pos, BlockState oldState, BlockState newState, int flags) {
            // no-op (не на скан-пути)
        }

        @Override
        public void playSeededSound(net.minecraft.world.entity.Entity source, double x, double y, double z,
                net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound, net.minecraft.sounds.SoundSource src2,
                float vol, float pitch, long seed) {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public void playSeededSound(net.minecraft.world.entity.Entity except, net.minecraft.world.entity.Entity entity,
                net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound, net.minecraft.sounds.SoundSource src2,
                float vol, float pitch, long seed) {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public void explode(net.minecraft.world.entity.Entity source, net.minecraft.world.damagesource.DamageSource ds,
                net.minecraft.world.level.ExplosionDamageCalculator calc, double x, double y, double z, float radius,
                boolean fire, Level.ExplosionInteraction interaction, net.minecraft.core.particles.ParticleOptions small,
                net.minecraft.core.particles.ParticleOptions large, net.minecraft.util.random.WeightedList<net.minecraft.core.particles.ExplosionParticleInfo> particles,
                net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> sound) {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public String gatherChunkSourceStats() {
            return "mini";
        }

        @Override
        public void setRespawnData(net.minecraft.world.level.storage.LevelData.RespawnData data) {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.level.storage.LevelData.RespawnData getRespawnData() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.entity.Entity getEntity(int id) {
            return null;
        }

        @Override
        public java.util.Collection<net.minecraft.world.entity.boss.EnderDragonPart> dragonParts() {
            return java.util.List.of();
        }

        @Override
        public net.minecraft.world.TickRateManager tickRateManager() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.level.saveddata.maps.MapItemSavedData getMapData(net.minecraft.world.level.saveddata.maps.MapId id) {
            return null;
        }

        @Override
        public void destroyBlockProgress(int breakerId, BlockPos pos, int progress) {
        }

        @Override
        public net.minecraft.world.scores.Scoreboard getScoreboard() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.item.crafting.RecipeAccess recipeAccess() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.level.entity.LevelEntityGetter<net.minecraft.world.entity.Entity> getEntities() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.item.alchemy.PotionBrewing potionBrewing() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public net.minecraft.world.level.block.entity.FuelValues fuelValues() {
            throw new UnsupportedOperationException("scan path never calls this");
        }
    }

    // ---------------- ChunkSource stub ----------------

    static final class MiniChunkSource extends ChunkSource {
        final LevelChunk[] chunks = new LevelChunk[CH_RANGE * CH_RANGE];

        LevelChunk at(int cx, int cz) {
            if (cx < 0 || cz < 0 || cx >= CH_RANGE || cz >= CH_RANGE) {
                return null;
            }
            return chunks[cz * CH_RANGE + cx];
        }

        @Override
        public ChunkAccess getChunk(int cx, int cz, ChunkStatus status, boolean create) {
            return at(cx, cz);
        }

        @Override
        public void tick(java.util.function.BooleanSupplier hasTime, boolean tickChunks) {
        }

        @Override
        public String gatherStats() {
            return "mini";
        }

        @Override
        public int getLoadedChunksCount() {
            return chunks.length;
        }

        @Override
        public LevelLightEngine getLightEngine() {
            return null; // scan path never calls this
        }

        @Override
        public LevelChunk getChunkNow(int cx, int cz) {
            return at(cx, cz);
        }

        @Override
        public boolean hasChunk(int cx, int cz) {
            return at(cx, cz) != null;
        }

        @Override
        public net.minecraft.world.level.chunk.LightChunk getChunkForLighting(int cx, int cz) {
            return at(cx, cz);
        }

        @Override
        public net.minecraft.world.level.BlockGetter getLevel() {
            throw new UnsupportedOperationException("scan path never calls this");
        }

        @Override
        public void close() {
        }
    }

    // ---------------- World assembly ----------------

    // инициализируются В main ПОСЛЕ Bootstrap (урок S7-137: регистры in-memory;
    // статические BlockState-инициализаторы до bootStrap падают)
    private static BlockState AIR;
    private static BlockState WATER;
    private static BlockState LAVA;
    private static BlockState STONE;

    /** Новая секция: все ячейки = AIR (реальный PalettedContainer, реальная стратегия). */
    private static LevelChunkSection freshSection() {
        var blockStrategy = net.minecraft.world.level.chunk.Strategy
                .<BlockState>createForBlockStates(Block.BLOCK_STATE_REGISTRY);
        PalettedContainer<BlockState> statesPc = new PalettedContainer<>(AIR, blockStrategy);
        return new LevelChunkSection(statesPc, null);
    }

    /** LevelChunk через allocateInstance (ctor кастует level к ServerLevel — офлайн нельзя). */
    private static LevelChunk chunkAt(MiniLevel level, int cx, int cz, LevelChunkSection[] sections) {
        try {
            LevelChunk c = (LevelChunk) U.allocateInstance(LevelChunk.class);
            setObj(c, ChunkAccess.class, "chunkPos", new net.minecraft.world.level.ChunkPos(cx, cz));
            setObj(c, ChunkAccess.class, "sections", sections);
            setObj(c, ChunkAccess.class, "levelHeightAccessor", level);
            setObj(c, LevelChunk.class, "level", level);
            return c;
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    private static MiniLevel buildWorld(WritableLevelData data) throws Exception {
        MiniLevel lvl = (MiniLevel) U.allocateInstance(MiniLevel.class);
        setInt(lvl, Level.class, "minY", -64);
        setInt(lvl, Level.class, "maxY", 319);
        setInt(lvl, Level.class, "minSectionY", -4);
        setInt(lvl, Level.class, "maxSectionY", 19);
        setInt(lvl, Level.class, "sectionsCount", 24);
        setBool(lvl, Level.class, "isClientSide", false);
        setBool(lvl, Level.class, "captureTreeGeneration", false);
        setObj(lvl, Level.class, "levelData", data);
        lvl.src = new MiniChunkSource();
        for (int cz = 0; cz < CH_RANGE; cz++) {
            for (int cx = 0; cx < CH_RANGE; cx++) {
                LevelChunkSection[] secs = new LevelChunkSection[24];
                for (int i = 0; i < 24; i++) {
                    secs[i] = freshSection();
                }
                lvl.src.chunks[cz * CH_RANGE + cx] = chunkAt(lvl, cx, cz, secs);
            }
        }
        return lvl;
    }

    /** Заполнение ячейки через РЕАЛЬНЫЙ section-write (без LevelChunk-периферии). */
    private static void put(MiniLevel lvl, BlockPos pos, BlockState state) {
        LevelChunk c = lvl.src.at(pos.getX() >> 4, pos.getZ() >> 4);
        if (c == null) {
            throw new AssertionError("put outside loaded range: " + pos);
        }
        c.getSections()[(pos.getY() >> 4) + 4].setBlockState(
                pos.getX() & 15, pos.getY() & 15, pos.getZ() & 15, state);
    }

    // ---------------- Entity fixture ----------------

    static final class Twin {
        final ItemEntity e;

        Twin(MiniLevel lvl, double x, double y, double z, Vec3 dm, int id) throws Exception {
            e = (ItemEntity) U.allocateInstance(ItemEntity.class);
            setObj(e, Entity.class, "level", lvl);
            double w = 0.25, h = 0.25;
            AABB box = new AABB(x - w / 2, y, z - w / 2, x + w / 2, y + h, z + w / 2);
            setObj(e, Entity.class, "bb", box);
            setInt(e, Entity.class, "id", id);
            setObj(e, Entity.class, "deltaMovement", dm);
            setObj(e, Entity.class, "posLock", new Object());
            setBool(e, Entity.class, "firstTick", false);
            it.unimi.dsi.fastutil.objects.Object2DoubleOpenHashMap<TagKey<Fluid>> map =
                    new it.unimi.dsi.fastutil.objects.Object2DoubleOpenHashMap<>();
            map.defaultReturnValue(0.0d);
            setObj(e, Entity.class, "fluidHeight", map);
        }
    }

    private static long off(Class<?> cls, String name) throws Exception {
        Field f = cls.getDeclaredField(name);
        f.setAccessible(true);
        return U.objectFieldOffset(f);
    }

    private static void setObj(Object o, Class<?> cls, String name, Object v) throws Exception {
        U.putObject(o, off(cls, name), v);
    }

    private static void setInt(Object o, Class<?> cls, String name, int v) throws Exception {
        U.putInt(o, off(cls, name), v);
    }

    private static void setBool(Object o, Class<?> cls, String name, boolean v) throws Exception {
        U.putBoolean(o, off(cls, name), v);
    }

    // ---------------- Comparison ----------------

    private static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> WATER_TAG =
            net.minecraft.tags.FluidTags.WATER;
    private static final net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid> LAVA_TAG =
            net.minecraft.tags.FluidTags.LAVA;

    static final class Snapshot {
        boolean ret;
        long fhBits;
        long dmX, dmY, dmZ;
        BlockPos lavaPos;

        static Snapshot of(ItemEntity e, boolean ret) {
            Snapshot s = new Snapshot();
            s.ret = ret;
            s.fhBits = Double.doubleToRawLongBits(e.fluidHeight.getDouble(WATER_TAG)) * 31
                    + Double.doubleToRawLongBits(e.fluidHeight.getDouble(LAVA_TAG)) * 17
                    + e.fluidHeight.size();
            Vec3 dm = e.getDeltaMovement();
            s.dmX = Double.doubleToRawLongBits(dm.x);
            s.dmY = Double.doubleToRawLongBits(dm.y);
            s.dmZ = Double.doubleToRawLongBits(dm.z);
            s.lavaPos = e.lastLavaContact;
            return s;
        }

        boolean sameAs(Snapshot o) {
            return ret == o.ret && fhBits == o.fhBits && dmX == o.dmX && dmY == o.dmY
                    && dmZ == o.dmZ
                    && ((lavaPos == null && o.lavaPos == null)
                        || (lavaPos != null && o.lavaPos != null
                            && lavaPos.getX() == o.lavaPos.getX()
                            && lavaPos.getY() == o.lavaPos.getY()
                            && lavaPos.getZ() == o.lavaPos.getZ()));
        }

        String dump() {
            return "ret=" + ret + " fhBits=" + fhBits + " dm=(" + dmX + "," + dmY + "," + dmZ + ")"
                    + " lava=" + (lavaPos == null ? "null" : lavaPos.getX() + "," + lavaPos.getY() + "," + lavaPos.getZ());
        }
    }

    /** Сравнение пары близнецов: vanilla scan (kernel bytecode) vs lever scan. */
    private static void assertParity(Twin van, Twin lev,
            TagKey<Fluid> tag, double scale, String label) throws Exception {
        // vanilla: прямое ванильное тело (kernel bytecode, не ретаргечено в харнессе)
        boolean rv = van.e.updateFluidHeightAndDoFluidPushing(tag, scale);
        Snapshot sv = Snapshot.of(van.e, rv);
        // lever: мост (HIT или MISS+capture — оба пути обязаны дать бит-в-бит то же)
        boolean rl = FluidPushOps.scan(lev.e, tag, scale);
        Snapshot sl = Snapshot.of(lev.e, rl);
        if (!sv.sameAs(sl)) {
            throw new AssertionError("PARITY FAIL [" + label + "]\n  vanilla: " + sv.dump()
                    + "\n  lever:   " + sl.dump());
        }
    }

    // ---------------- Main ----------------

    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        AIR = Blocks.AIR.defaultBlockState();
        WATER = Blocks.WATER.defaultBlockState();
        LAVA = Blocks.LAVA.defaultBlockState();
        STONE = Blocks.STONE.defaultBlockState();
        FluidPushOps.arm();
        if (!FluidPushOps.armed()) {
            throw new AssertionError("FluidPushOps must be armed");
        }
        System.out.println("LOCKSTEP BOOTSTRAP OK (registries bootstrapped, bridge armed)");

        int eid = 1_000_000;
        int cases = 0;
        WritableLevelData dataStub = dataStub();

        // ---- Сцена A: источник воды на y=64, каменный пол y=63, чанк (5,3) ----
        MiniLevel lvlA = buildWorld(dataStub);
        for (BlockPos p : new BlockPos[]{new BlockPos(70, 64, 55), new BlockPos(71, 64, 55),
                new BlockPos(70, 64, 56)}) {
            put(lvlA, p, WATER);
        }
        for (int x = 68; x <= 73; x++) {
            for (int z = 53; z <= 58; z++) {
                put(lvlA, new BlockPos(x, 63, z), STONE);
            }
        }

        double[][] poses = {
                {70.5, 64.2, 55.5},     // внутри воды
                {70.5, 64.95, 55.5},    // на кромке воды (высота источника 0.888…)
                {69.0, 63.5, 54.5},     // над сухим камнем рядом
                {79.999, 64.5, 55.5},   // граница чанков x (5|6)
                {70.5, 64.5, 47.999},   // граница чанков z (3|2)
                {64.25, 63.125, 51.75}, // дробные, вне воды
        };
        Vec3[] dms = {
                Vec3.ZERO,
                new Vec3(0.001, 0.0, 0.001), // ветка 0.003/0.0045
                new Vec3(0.01, 0.5, -0.01),
                new Vec3(-0.05, -1.0, 0.05),
        };
        for (double[] p : poses) {
            for (Vec3 dm : dms) {
                Twin van = new Twin(lvlA, p[0], p[1], p[2], dm, eid++);
                Twin lev = new Twin(lvlA, p[0], p[1], p[2], dm, eid++);
                assertParity(van, lev, WATER_TAG, 0.014d, "A" + cases + "-water");
                assertParity(van, lev, LAVA_TAG, 0.007d, "A" + cases + "-lava");
                // HIT-проверка: повторные идентичные вызовы моста — HIT и тот же результат
                long h0 = FluidPushOps.hits();
                boolean r1 = FluidPushOps.scan(lev.e, WATER_TAG, 0.014d);
                long h1 = FluidPushOps.hits();
                boolean r2 = FluidPushOps.scan(lev.e, WATER_TAG, 0.014d);
                long h2 = FluidPushOps.hits();
                if (h1 != h0 + 1 || h2 != h1 + 1) {
                    throw new AssertionError("HIT bookkeeping broken: " + h0 + "->" + h1 + "->" + h2);
                }
                if (r1 != r2) {
                    throw new AssertionError("HIT result diverged: " + r1 + " vs " + r2);
                }
                cases++;
            }
        }
        System.out.println("SCENE A PASS (water source, " + cases + " позиций, обе теги)");

        // ---- Сцена B: лава + вода столбами, граница секций ----
        MiniLevel lvlB = buildWorld(dataStub);
        put(lvlB, new BlockPos(40, 31, 40), LAVA);
        put(lvlB, new BlockPos(40, 30, 40), LAVA);
        put(lvlB, new BlockPos(41, 32, 41), WATER);
        put(lvlB, new BlockPos(41, 31, 41), WATER);
        put(lvlB, new BlockPos(41, 30, 41), WATER);
        for (int x = 38; x <= 43; x++) {
            for (int z = 38; z <= 43; z++) {
                put(lvlB, new BlockPos(x, 29, z), STONE);
            }
        }
        double[][] posesB = {
                {40.5, 30.5, 40.5},     // в лаве
                {41.5, 31.5, 41.5},     // в воде под лавой-соседом
                {40.5, 31.95, 40.5},    // кромка лавы
                {47.999, 30.5, 47.999}, // граница чанков (2|3)
                {40.5, 47.999, 40.5},   // граница секций y
        };
        for (double[] p : posesB) {
            for (Vec3 dm : dms) {
                Twin van = new Twin(lvlB, p[0], p[1], p[2], dm, eid++);
                Twin lev = new Twin(lvlB, p[0], p[1], p[2], dm, eid++);
                assertParity(van, lev, LAVA_TAG, 0.007d, "B" + cases + "-lava");
                assertParity(van, lev, WATER_TAG, 0.014d, "B" + cases + "-water");
                cases++;
            }
        }
        System.out.println("SCENE B PASS (lava+water столб, lastLavaContact parity включительно)");

        // ---- Сцена C: flowing water (уровни 1..3) + шторм мутаций ----
        MiniLevel lvlC = buildWorld(dataStub);
        put(lvlC, new BlockPos(20, 70, 20), STONE);
        put(lvlC, new BlockPos(20, 71, 20), WATER); // источник
        for (int lvlProp = 1; lvlProp <= 3; lvlProp++) {
            BlockState flowing = WATER;
            try {
                flowing = WATER.setValue(LiquidBlock.LEVEL, lvlProp);
            } catch (Throwable t) {
                // свойство недоступно — остаёмся источником (богатство сцены, не парити-риск)
            }
            put(lvlC, new BlockPos(20 + lvlProp, 71, 20), flowing);
        }
        for (int x = 18; x <= 26; x++) {
            put(lvlC, new BlockPos(x, 69, 18), STONE);
        }
        Vec3 dmT = new Vec3(0.001, 0.0, 0.001);
        Twin vanC = new Twin(lvlC, 20.5, 70.2, 20.5, dmT, eid++);
        Twin levC = new Twin(lvlC, 20.5, 70.2, 20.5, dmT, eid++);
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C0-capture");
        assertParity(vanC, levC, LAVA_TAG, 0.0023333333333333335d, "C0-lava");

        // ШТОРМ: каждая мутация через делегат обязана инвалидировать мост (MISS) и
        // следующий вызов моста обязан совпасть со свежей ванилью на НОВОМ мире.
        LevelChunkSection sec = lvlC.src.at(1, 1).getSections()[(71 >> 4) + 4];
        long misses0 = FluidPushOps.misses();
        BlockState old1 = FluidPushOps.secWrite(sec, 4, 71 & 15, 4,
                WATER.setValue(LiquidBlock.LEVEL, 8)); // источник -> слабое течение
        BlockState now1 = sec.states.get(4 | (4 << 4) | ((71 & 15) << 8));
        if (now1.getFluidState() == WATER.getFluidState()) {
            throw new AssertionError("storm: water source -> flowing must change FluidState");
        }
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C-storm1 (source->flowing)");
        if (FluidPushOps.misses() == misses0) {
            throw new AssertionError("storm: mutation must force a MISS (event-driven invalidation)");
        }
        FluidPushOps.secWrite(sec, 5, 71 & 15, 4, AIR); // течение -> воздух
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C-storm2 (flowing->air)");
        FluidPushOps.secWrite(sec, 6, 71 & 15, 4, STONE); // вода -> камень
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C-storm3 (water->stone)");
        FluidPushOps.secWrite(sec, 6, 71 & 15, 4, WATER); // камень -> вода
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C-storm4 (stone->water)");
        long stBefore = FluidPushOps.stampOf(sec);
        FluidPushOps.secWrite(sec, 3, 71 & 15, 4, STONE); // воздух -> камень (нет fluid-дельты)
        if (FluidPushOps.stampOf(sec) != stBefore) {
            throw new AssertionError("storm: air->stone must NOT bump (ref-compare)");
        }
        assertParity(vanC, levC, WATER_TAG, 0.014d, "C-storm5 (no-bump still parity)");
        System.out.println("SCENE C PASS (flowing + шторм: каждая мутация = MISS + свежая ваниль бит-в-бит)");

        // ---- Сцена D: unloaded-guard паритет ----
        MiniLevel lvlD = buildWorld(dataStub);
        double edgeX = (double) (CH_RANGE * 16) - 0.1; // inflate(1) вылезает за [0..CH_RANGE)
        Twin vanD = new Twin(lvlD, edgeX, 100.0, 100.0, Vec3.ZERO, eid++);
        Twin levD = new Twin(lvlD, edgeX, 100.0, 100.0, Vec3.ZERO, eid++);
        boolean rv = vanD.e.updateFluidHeightAndDoFluidPushing(WATER_TAG, 0.014d);
        boolean rl = FluidPushOps.scan(levD.e, WATER_TAG, 0.014d);
        if (rv || rl) {
            throw new AssertionError("unloaded guard must return false on both paths: " + rv + "/" + rl);
        }
        Snapshot svd = Snapshot.of(vanD.e, rv);
        Snapshot sld = Snapshot.of(levD.e, rl);
        if (!svd.sameAs(sld)) {
            throw new AssertionError("unloaded guard parity fail: " + svd.dump() + " vs " + sld.dump());
        }
        System.out.println("SCENE D PASS (unloaded-guard: оба пути false без записи fluidHeight)");

        System.out.println("FLUID-DIRTY LOCKSTEP PASS (G5 core): " + cases + " позиционных кейсов, "
                + "hit=" + FluidPushOps.hits() + " miss=" + FluidPushOps.misses()
                + " vanilla=" + FluidPushOps.vanillas());
    }

    private static WritableLevelData dataStub() {
        return (WritableLevelData) java.lang.reflect.Proxy.newProxyInstance(
                WritableLevelData.class.getClassLoader(),
                new Class<?>[] {WritableLevelData.class},
                (proxy, method, margs) -> {
                    Class<?> rt = method.getReturnType();
                    if (rt == boolean.class) return false;
                    if (rt == int.class) return 0;
                    if (rt == long.class) return 0L;
                    if (rt == float.class) return 0f;
                    if (rt == double.class) return 0d;
                    return null;
                });
    }
}
