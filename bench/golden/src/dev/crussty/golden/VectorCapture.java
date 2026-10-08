package dev.crussty.golden;

// ============================================================================
// VectorCapture — golden random/noise/density vectors from the REAL Paper
// 1.21.10 classes (NCF P0.3 tail — the P1 gate's Java half).
//
// Command: /goldenvec <label>   (console/RCON only, runs synchronously on
// the main thread — no chunk generation, no scheduling needed)
//
// What it writes (under the same output root as the chunk dumps):
//   <out>/<label>/vectors/random.csv
//     family,src,seed_lo,seed_hi,op,arg,index,value
//       family=stafford   : RandomSupport.mixStafford13(arg=seed)
//       family=upgrade128 : src=upgrade128_lo|upgrade128_hi,
//                           RandomSupport.upgradeSeedTo128bit(arg=seed)
//       family=random     : src=legacy|xoroshiro, the 8-op draw pattern
//                           (nextInt/nextIntBound(1000)/nextIntBound(97)/
//                            nextLong/nextBoolean/nextFloat/nextDouble/
//                            nextGaussian) x 40 rounds
//       family=pos        : src=legacy_at|xoroshiro_at|legacy_fromhash|
//                           xoroshiro_fromhash — factories built DIRECTLY
//                           from known seeds (no parent-draw ambiguity);
//                           arg = "x|y|z" for at(), the name for fromhash
//     value encoding: signed decimal for ints/longs/booleans,
//     Float.toHexString / Double.toHexString for floats (lossless round-trip).
//
//   <out>/<label>/vectors/noise.csv
//     impl,seed,params,x,y,z,v0,v1,v2,v3
//       improved, improved_deriv — ImprovedNoise on XoroshiroRandomSource(seed)
//       perlin, perlin_fixed_y   — PerlinNoise.create (new factory path)
//       normal, normal_legacy    — NormalNoise.create / createLegacyNetherBiome
//       blended                  — BlendedNoise legacy construction
//       rs_noise                 — RandomState.getOrCreateNoise(key) — the
//                                  SAME instances the live router uses; this
//                                  exercises the full fromHashOf(key) wiring
//     params: k=v pairs joined by ';', lists joined by '|', doubles as hex.
//
//   <out>/<label>/vectors/density.csv
//     field,x,y,z,value — all 15 NoiseRouter fields through
//     DensityFunction.SinglePointContext on the LIVE world's RandomState
//     (ServerChunkCache.randomState()). This is the unbound scalar path:
//     markers pass through, Blender.empty is identity — exactly the shape
//     the Rust scalar evaluator (P1.5) implements.
//
//   <out>/<label>/vectors/interp.csv   (session 4, P2.3-tail)
//     # NCF interp vectors v1 / worldSeed / chunk=100,100 / firstNoise=1600,1600 /
//     # cellWidth cellHeight cellCountXZ cellCountY cellNoiseMinY / interpCount
//     interp_index,block_x,block_y,block_z,Double.toHexString(value)
//     Per-block density values from the REAL NoiseChunk interpolation machinery
//     of a live NoiseChunk built exactly like NoiseBasedChunkGenerator.doFill
//     (cfr-out lines 283-344): NoiseChunk(4, randomState, 1600, 1600, ...) then
//     initializeForFirstCellX -> advanceCellX(cx) -> selectCellYZ(cy, cz) ->
//     updateForY -> updateForX -> updateForZ per block, value = every
//     NoiseInterpolator's compute(nc) in interpolator-list order, swapSlices()
//     after each cell-X column (doFill line 340 — REQUIRED, see below),
//     stopInterpolation() at the end. Drive order: cx asc, cz asc, cy desc,
//     inY desc, inX asc, inZ asc, interp index 0..n-1. Rust replays row order.
//
//   <out>/<label>/vectors/climate.csv  (session 4, P2.4)
//     # NCF climate vectors v1 / worldSeed
//     q_x,q_y,q_z,t,h,c,e,d,w,biome
//     t..w = the 6 quantized longs of Climate.TargetPoint
//     (temperature/humidity/continentalness/erosion/depth/weirdness);
//     biome = registered name of getNoiseBiome(qx,qy,qz,sampler).
//     Region A: qx in {396..410 step 2}, qz same, qy in {-8,0,..,48} (innermost)
//     = 512 rows. Region B AFTER A: qx in {-30,-28,-26}, qz same, qy {0,16,32}
//     = 27 rows. Row order: qx outer -> qz -> qy innermost (the RTree
//     ThreadLocal last-result cache persists row to row; Rust must replay in
//     the same order).
//
//   <out>/<label>/vectors/climate_points.csv (session 4, P2.4)
//     # NCF climate points v1 — live minecraft:overworld parameter list
//     t_min,t_max,h_min,h_max,c_min,c_max,e_min,e_max,d_min,d_max,w_min,w_max,offset,biome
//     The LIVE MultiNoiseBiomeSource's Climate.ParameterList (reflection on the
//     private `parameters` Either field, see below) -> values() in order.
//     ~180 rows; the exact count is logged at capture time.
//
// Purity law: identical to GoldenDumperPlugin — vanilla boot only, NO
// CRUSSTY agent. Determinism: no wall clock, no world-state dependence
// beyond the world seed; grids are fixed constants.
//
// API verification status (2026-10-05 + session-4 additions, javap on the
// mojang-mapped jar /home/z/server/versions/1.21.10/purpur-1.21.10.jar):
//   ServerChunkCache.randomState()                              public
//   ServerChunkCache.getGenerator() -> ChunkGenerator           public
//   ChunkGenerator.getBiomeSource() -> BiomeSource              public
//   NoiseRouter record accessors (barrierNoise()..veinGap())    public
//   RandomState.getOrCreateNoise(ResourceKey<NoiseParameters>)  public
//   RandomState.sampler() -> Climate$Sampler                    public (record)
//   Climate$Sampler.sample(int,int,int) -> Climate$TargetPoint  public (record)
//   Climate$TargetPoint.temperature()/humidity()/continentalness()/
//     erosion()/depth()/weirdness()                             public -> long
//   BiomeSource.getNoiseBiome(int,int,int,Climate$Sampler)      public abstract,
//     implemented public in MultiNoiseBiomeSource -> Holder<Biome>
//   MultiNoiseBiomeSource field `parameters`:
//     private final Either<Climate$ParameterList<Holder<Biome>>,
//                          Holder<MultiNoiseBiomeSourceParameterList>>
//     (the overworld source uses the PRESET arm: right(); left() is the direct
//     createFromList arm. Both handled.)
//   MultiNoiseBiomeSourceParameterList.parameters()             public
//     -> Climate$ParameterList<Holder<Biome>>
//   Climate$ParameterList.values()                              public
//     -> List<Pair<Climate$ParameterPoint, T>> (order preserved)
//   Climate$ParameterPoint.temperature()/.../weirdness()        public
//     -> Climate$Parameter; offset() -> long
//   Climate$Parameter.min()/max()                               public -> long
//   ServerLevel.registryAccess() -> RegistryAccess              public (Level)
//   REGISTRY NOTE (session-4 runtime fix): RegistryAccess.lookupOrThrow has
//   THREE overloads in 1.21.10 and the javac-resolved call site failed to
//   LINK at runtime (NoSuchMethodError on the erased descriptor). The
//   capture therefore avoids registry lookups entirely:
//   - NoiseGeneratorSettings -> NoiseBasedChunkGenerator.generatorSettings()
//     (public accessor on the LIVE generator, javap-verified),
//   - biome/block names -> Holder.getRegisteredName() (public default)
//     and BuiltInRegistries.BLOCK.getKey(Object) (DefaultedRegistry).
//   NoiseGeneratorSettings.noiseSettings()                      public (record)
//   NoiseSettings.clampToHeightAccessor(LevelHeightAccessor)    public (record)
//   NoiseSettings.getCellWidth()/getCellHeight()/minY()/height() public
//   NoiseChunk(int,RandomState,int,int,NoiseSettings,
//     DensityFunctions$BeardifierOrMarker,NoiseGeneratorSettings,
//     Aquifer$FluidPicker,Blender)                              PUBLIC ctor
//   NoiseChunk.initializeForFirstCellX()/advanceCellX(int)/
//     selectCellYZ(int y,int z)/updateForY(int,double)/updateForX(int,double)/
//     updateForZ(int,double)/stopInterpolation()/swapSlices()   public
//   NoiseChunk.blockX()/blockY()/blockZ()                       public
//   Mth.floorDiv(int,int)                                       public
//   DensityFunctions.BeardifierMarker — PROTECTED nested class (InnerClasses
//     attribute), so its type is not nameable from the plugin; the INSTANCE
//     field is public static final -> fetched via reflection and cast to the
//     PUBLIC nested interface DensityFunctions.BeardifierOrMarker (the ctor's
//     declared parameter type).
//   Aquifer$FluidPicker: functional interface, method
//     computeFluid(int x,int y,int z) -> Aquifer$FluidStatus    public
//     (NOT apply() — verified; the lambda takes three ints)
//   Aquifer$FluidStatus(int fluidLevel, BlockState fluidType)   public record ctor
//     — NOTE fluidType is a BLOCK state; from a Fluid use
//     FluidState.createLegacyBlock() (public), e.g.
//     Fluids.WATER.defaultFluidState().createLegacyBlock()
//   Fluids.WATER : FlowingFluid ; Fluid.defaultFluidState()     public -> FluidState
//   Blender.empty()                                             public
//     (Blender is net.minecraft.world.level.levelgen.blending.Blender)
//   com.mojang.datafixers.util.Pair.getFirst()/getSecond()      public
//   com.mojang.datafixers.util.Either.left()/right()            public -> Optional
//   Holder.value() / Holder.getRegisteredName()                 public
//   Registry.getKey(T) -> ResourceLocation                      public abstract
//
// Reflection surface (setAccessible(true), verified 2026-10-05 against the
// mapped jar; the CLASSES are public but these MEMBERS are not:
//   1. NoiseChunk.interpolators :
//        final java.util.List<NoiseChunk$NoiseInterpolator> interpolators
//      (package-private final field) — read as List<?>, element order = the
//      order NoiseInterpolators were appended during the ctor's router
//      mapAll walk (deterministic for a fixed datapack).
//   2. NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext):
//      public method on a public inner class, reached via reflection per the
//      harness contract (Method cached once, in interpolator order). With
//      context == the NoiseChunk and fillingCell==false it returns the
//      trilinear-lerped `value` — exactly what doFill's block loop consumes.
//   3. MultiNoiseBiomeSource.parameters (private final Either, see above):
//      left() = direct Climate$ParameterList, right() = Holder<...ParameterList>
//      -> .value() -> MultiNoiseBiomeSourceParameterList.parameters() (public).
//
// Deviation note (session 4): the driving loop includes nc.swapSlices() after
// each cell-X column — NoiseBasedChunkGenerator.doFill line 340 in cfr-out —
// which the task pseudo-code omitted. Without it, columns cx>0 would
// interpolate against stale x-slices and would NOT match the real generator.
// ============================================================================

import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

import com.mojang.datafixers.util.Pair;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.core.Holder;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.resources.ResourceKey;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.Mth;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.biome.Biome;
import net.minecraft.world.level.biome.Climate;
import net.minecraft.world.level.biome.MultiNoiseBiomeSource;
import net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList;
import net.minecraft.world.level.levelgen.Aquifer;
import net.minecraft.world.level.levelgen.DensityFunction;
import net.minecraft.world.level.levelgen.DensityFunctions;
import net.minecraft.world.level.levelgen.LegacyRandomSource;
import net.minecraft.world.level.levelgen.NoiseChunk;
import net.minecraft.world.level.levelgen.NoiseGeneratorSettings;
import net.minecraft.world.level.levelgen.NoiseRouter;
import net.minecraft.world.level.levelgen.NoiseSettings;
import net.minecraft.world.level.levelgen.RandomState;
import net.minecraft.world.level.levelgen.RandomSupport;
import net.minecraft.world.level.levelgen.XoroshiroRandomSource;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.levelgen.blending.Blender;
import net.minecraft.world.level.levelgen.structure.BoundingBox;
import net.minecraft.world.level.levelgen.structure.PoolElementStructurePiece;
import net.minecraft.world.level.levelgen.structure.Structure;
import net.minecraft.world.level.levelgen.structure.StructurePiece;
import net.minecraft.world.level.levelgen.structure.StructureStart;
import net.minecraft.world.level.levelgen.structure.pools.JigsawJunction;
import net.minecraft.world.level.levelgen.synth.BlendedNoise;
import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.NormalNoise;
import net.minecraft.world.level.levelgen.synth.PerlinNoise;
import net.minecraft.world.level.material.Fluids;

import org.bukkit.command.CommandSender;
import org.bukkit.craftbukkit.CraftWorld;
import org.bukkit.plugin.java.JavaPlugin;

public final class VectorCapture {

    private VectorCapture() {}

    /** Draw-pattern rounds per random source (8 ops each). */
    static final int RANDOM_ROUNDS = 40;
    static final long[] RANDOM_SEEDS = {
        3053459L, 90210L, 424242L, 8675309L, 133700L, 1L, -1L, Long.MIN_VALUE, Long.MAX_VALUE,
    };
    static final long[] STAFFORD_SEEDS = {
        0L, 1L, 3053459L, 90210L, 424242L, 8675309L, 133700L, -1L,
        Long.MAX_VALUE, Long.MIN_VALUE, 0xDEADBEEFCAFEBABEL,
    };

    public static void run(JavaPlugin plugin, String label, CommandSender ack) {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        long seed = level.getSeed();

        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            Path pluginsDir = plugin.getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) {
                ack.sendMessage("goldenvec: cannot resolve server dir — set -Dgoldendump.out=<dir>");
                return;
            }
            root = serverDir.resolve("golden");
        }
        Path vecDir = root.resolve(label).resolve("vectors");
        try {
            Files.createDirectories(vecDir);
        } catch (IOException e) {
            ack.sendMessage("goldenvec: cannot create " + vecDir + ": " + e);
            return;
        }

        long t0 = System.nanoTime();
        int rows;
        InterpStats interpStats;
        int climateRows;
        int climatePointRows;
        int aquiferRows;
        try {
            rows = writeRandom(vecDir.resolve("random.csv"))
                    + writeNoise(vecDir.resolve("noise.csv"), level, seed)
                    + writeDensity(vecDir.resolve("density.csv"), level);
            interpStats = writeInterp(vecDir.resolve("interp.csv"), level, seed, FIRST_BLOCK_X, FIRST_BLOCK_Z);
            rows += interpStats.rows();
            climateRows = writeClimate(vecDir.resolve("climate.csv"), level, seed);
            rows += climateRows;
            climatePointRows = writeClimatePoints(vecDir.resolve("climate_points.csv"), level);
            rows += climatePointRows;
            aquiferRows = writeAquifer(vecDir.resolve("aquifer.csv"),
                    vecDir.resolve("aquifer_meta.txt"), level, seed, FIRST_BLOCK_X, FIRST_BLOCK_Z);
            rows += aquiferRows;
        } catch (Throwable t) {
            plugin.getLogger().warning("GOLDEN VECTOR FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("goldenvec: FAILED: " + t);
            return;
        }
        long ms = (System.nanoTime() - t0) / 1_000_000L;
        plugin.getLogger().info("GOLDEN VECTOR COMPLETE rows=" + rows + " ms=" + ms + " dir=" + vecDir);
        plugin.getLogger().info("GOLDEN VECTOR interp: count=" + interpStats.interpCount()
                + " rows=" + interpStats.rows() + " nonFinite=" + interpStats.nonFinite()
                + " cell=" + interpStats.cellW() + "x" + interpStats.cellH()
                + " cellCountY=" + interpStats.cellCountY() + " cellNoiseMinY=" + interpStats.minCellY());
        plugin.getLogger().info("GOLDEN VECTOR climate: sampled=" + climateRows
                + " parameterPoints=" + climatePointRows);
        plugin.getLogger().info("GOLDEN VECTOR aquifer: rows=" + aquiferRows);
        ack.sendMessage("goldenvec: complete rows=" + rows + " dir=" + vecDir);
    }

    /** interp.csv capture summary (logged, not part of the file contract). */
    private record InterpStats(int rows, int interpCount, long nonFinite,
                               int cellW, int cellH, int cellCountY, int minCellY) {}

    // ------------------------------------------------------------------
    // CSV plumbing: 8 columns, family,src,seed_lo,seed_hi,op,arg,index,value
    // ------------------------------------------------------------------

    private static void row(BufferedWriter w, String family, String src, long seedLo, long seedHi,
                            String op, String arg, int index, String value) throws IOException {
        w.write(family);
        w.write(',');
        w.write(src);
        w.write(',');
        w.write(Long.toString(seedLo));
        w.write(',');
        w.write(Long.toString(seedHi));
        w.write(',');
        w.write(op);
        w.write(',');
        w.write(arg);
        w.write(',');
        w.write(Integer.toString(index));
        w.write(',');
        w.write(value);
        w.newLine();
    }

    private static void header(BufferedWriter w, String what) throws IOException {
        w.write("# NCF golden vectors v1 (P0.3 tail) — " + what);
        w.newLine();
    }

    private static BufferedWriter newWriter(Path p) throws IOException {
        return Files.newBufferedWriter(p, StandardCharsets.UTF_8);
    }

    private static String hx(double v) {
        return Double.toHexString(v);
    }

    // ------------------------------------------------------------------
    // random.csv
    // ------------------------------------------------------------------

    private static int writeRandom(Path path) throws IOException {
        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            header(w, "random lineage vectors");

            for (long s : STAFFORD_SEEDS) {
                row(w, "stafford", "-", s, 0, "mix", Long.toString(s), 0,
                        Long.toString(RandomSupport.mixStafford13(s)));
                RandomSupport.Seed128bit up = RandomSupport.upgradeSeedTo128bit(s);
                row(w, "upgrade128", "upgrade128_lo", s, 0, "lo", Long.toString(s), 0, Long.toString(up.seedLo()));
                row(w, "upgrade128", "upgrade128_hi", s, 0, "hi", Long.toString(s), 0, Long.toString(up.seedHi()));
                rows += 3;
            }

            String[] srcs = {"legacy", "xoroshiro"};
            for (String src : srcs) {
                for (long s : RANDOM_SEEDS) {
                    RandomSource rs = src.equals("legacy")
                            ? new LegacyRandomSource(s)
                            : new XoroshiroRandomSource(s);
                    int idx = 0;
                    for (int i = 0; i < RANDOM_ROUNDS; i++) {
                        row(w, "random", src, s, 0, "nextInt", "0", idx++, Integer.toString(rs.nextInt()));
                        row(w, "random", src, s, 0, "nextIntBound", "1000", idx++, Integer.toString(rs.nextInt(1000)));
                        row(w, "random", src, s, 0, "nextIntBound", "97", idx++, Integer.toString(rs.nextInt(97)));
                        row(w, "random", src, s, 0, "nextLong", "0", idx++, Long.toString(rs.nextLong()));
                        row(w, "random", src, s, 0, "nextBoolean", "0", idx++, Boolean.toString(rs.nextBoolean()));
                        row(w, "random", src, s, 0, "nextFloat", "0", idx++, Float.toHexString(rs.nextFloat()));
                        row(w, "random", src, s, 0, "nextDouble", "0", idx++, Double.toHexString(rs.nextDouble()));
                        row(w, "random", src, s, 0, "nextGaussian", "0", idx++, Double.toHexString(rs.nextGaussian()));
                        rows += 8;
                    }
                }
            }

            // positional factories: constructed DIRECTLY from known seeds
            final long facLo = 305419896L;    // 0x12345678
            final long facHi = -669520596L;   // 0xD8198E6C (any fixed constant)
            final long legacyFacSeed = 777_777_777L;
            int[][] positions = {
                {0, 0, 0}, {1, 64, -1}, {100, 64, 100}, {-500, 32, 500}, {2147483000, -64, -2147483000},
            };
            String[] names = {"minecraft:terrain", "minecraft:shift", "crussty_test"};

            LegacyRandomSource.LegacyPositionalRandomFactory legacyFac =
                    new LegacyRandomSource.LegacyPositionalRandomFactory(legacyFacSeed);
            XoroshiroRandomSource.XoroshiroPositionalRandomFactory xoroFac =
                    new XoroshiroRandomSource.XoroshiroPositionalRandomFactory(facLo, facHi);

            for (int[] p : positions) {
                String posArg = p[0] + "|" + p[1] + "|" + p[2];
                RandomSource a = legacyFac.at(p[0], p[1], p[2]);
                row(w, "pos", "legacy_at", legacyFacSeed, 0, "nextLong", posArg, 0, Long.toString(a.nextLong()));
                row(w, "pos", "legacy_at", legacyFacSeed, 0, "nextIntBound", posArg, 0, Integer.toString(a.nextInt(97)));
                row(w, "pos", "legacy_at", legacyFacSeed, 0, "nextGaussian", posArg, 0, Double.toHexString(a.nextGaussian()));
                row(w, "pos", "legacy_at", legacyFacSeed, 0, "nextDouble", posArg, 0, Double.toHexString(a.nextDouble()));
                RandomSource b = xoroFac.at(p[0], p[1], p[2]);
                row(w, "pos", "xoroshiro_at", facLo, facHi, "nextLong", posArg, 0, Long.toString(b.nextLong()));
                row(w, "pos", "xoroshiro_at", facLo, facHi, "nextIntBound", posArg, 0, Integer.toString(b.nextInt(97)));
                row(w, "pos", "xoroshiro_at", facLo, facHi, "nextGaussian", posArg, 0, Double.toHexString(b.nextGaussian()));
                row(w, "pos", "xoroshiro_at", facLo, facHi, "nextDouble", posArg, 0, Double.toHexString(b.nextDouble()));
                rows += 8;
            }
            for (String name : names) {
                RandomSource a = legacyFac.fromHashOf(name);
                row(w, "pos", "legacy_fromhash", legacyFacSeed, 0, "nextLong", name, 0, Long.toString(a.nextLong()));
                row(w, "pos", "legacy_fromhash", legacyFacSeed, 0, "nextIntBound", name, 0, Integer.toString(a.nextInt(97)));
                row(w, "pos", "legacy_fromhash", legacyFacSeed, 0, "nextGaussian", name, 0, Double.toHexString(a.nextGaussian()));
                RandomSource b = xoroFac.fromHashOf(name);
                row(w, "pos", "xoroshiro_fromhash", facLo, facHi, "nextLong", name, 0, Long.toString(b.nextLong()));
                row(w, "pos", "xoroshiro_fromhash", facLo, facHi, "nextIntBound", name, 0, Integer.toString(b.nextInt(97)));
                row(w, "pos", "xoroshiro_fromhash", facLo, facHi, "nextGaussian", name, 0, Double.toHexString(b.nextGaussian()));
                rows += 6;
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // noise.csv
    // ------------------------------------------------------------------

    private static int writeNoise(Path path, ServerLevel level, long worldSeed) throws IOException {
        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            header(w, "noise kernel vectors (params are hex floats)");

            // --- ImprovedNoise (direct ctor on XoroshiroRandomSource(seed))
            for (long s : new long[]{3053459L, 90210L}) {
                ImprovedNoise n = new ImprovedNoise(new XoroshiroRandomSource(s));
                for (int i = 0; i < 24; i++) {
                    double x = i * 1.5 + 0.25;
                    double y = i * 0.75 - 3.5;
                    double z = -i * 2.25;
                    w.write("noise,improved," + s + ",0,-,"
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(n.noise(x, y, z)) + ",,,");
                    w.newLine();
                    rows++;
                }
                for (int i = 0; i < 12; i++) {
                    double x = i * 3.25 - 7.0;
                    double y = -i * 1.125;
                    double z = i * 0.5 + 11.0;
                    double[] d = new double[3];
                    double v = n.noiseWithDerivative(x, y, z, d);
                    w.write("noise,improved_deriv," + s + ",0,-,"
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(v) + "," + hx(d[0]) + "," + hx(d[1]) + "," + hx(d[2]));
                    w.newLine();
                    rows++;
                }
            }

            // --- PerlinNoise (new factory path) with two param sets
            double[][] ampSets = {
                {1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 0.0},   // includes a zero amplitude
                {1.0, 2.0, 2.0, 4.0, 1.0},
            };
            int[] foSets = {-3, -4};
            for (int set = 0; set < ampSets.length; set++) {
                long s = 424242L + set;
                StringBuilder amps = new StringBuilder();
                for (int i = 0; i < ampSets[set].length; i++) {
                    if (i > 0) amps.append('|');
                    amps.append(hx(ampSets[set][i]));
                }
                String params = "fo=" + foSets[set] + ";amps=" + amps;
                PerlinNoise pn = PerlinNoise.create(new XoroshiroRandomSource(s),
                        foSets[set], new DoubleArrayList(ampSets[set]));
                for (int i = 0; i < 24; i++) {
                    double x = i * 2.75;
                    double y = -i * 0.625 + 4.0;
                    double z = i * 1.125 - 2.0;
                    w.write("noise,perlin," + s + ",0," + params + ","
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(pn.getValue(x, y, z)) + ",,,");
                    w.newLine();
                    rows++;
                }
                for (int i = 0; i < 8; i++) {
                    double x = i * 1.75;
                    double y = i * 0.5 - 1.0;
                    double z = -i * 2.5;
                    double yScale = 1.0;
                    double yMax = 2.0;
                    String p2 = params + ";yscale=" + hx(yScale) + ";ymax=" + hx(yMax);
                    w.write("noise,perlin_fixed_y," + s + ",0," + p2 + ","
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(pn.getValue(x, y, z, yScale, yMax, false)) + ",,,");
                    w.newLine();
                    rows++;
                }
            }

            // --- NormalNoise (new factory + legacy nether)
            {
                long s = 8675309L;
                NormalNoise nn = NormalNoise.create(new XoroshiroRandomSource(s), -7,
                        1.0, 1.0, 1.0);
                String params = "fo=-7;amps=" + hx(1.0) + "|" + hx(1.0) + "|" + hx(1.0);
                for (int i = 0; i < 24; i++) {
                    double x = i * 0.875 + 1.5;
                    double y = -i * 0.3125;
                    double z = i * 2.0 - 9.0;
                    w.write("noise,normal," + s + ",0," + params + ","
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(nn.getValue(x, y, z)) + ",,,");
                    w.newLine();
                    rows++;
                }
                NormalNoise ln = NormalNoise.createLegacyNetherBiome(new XoroshiroRandomSource(s),
                        new NormalNoise.NoiseParameters(-7, new DoubleArrayList(new double[]{1.0, 1.0})));
                String lparams = "fo=-7;amps=" + hx(1.0) + "|" + hx(1.0);
                for (int i = 0; i < 12; i++) {
                    double x = i * 1.25;
                    double y = i * 0.4375 - 2.0;
                    double z = -i * 0.875;
                    w.write("noise,normal_legacy," + s + ",0," + lparams + ","
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(ln.getValue(x, y, z)) + ",,,");
                    w.newLine();
                    rows++;
                }
            }

            // --- BlendedNoise (legacy octaves construction)
            double[][] bParams = {
                {1.0, 1.0, 1.0, 2.0, 1.0},     // xz_scale, y_scale, xz_factor, y_factor, smear
                {1.0, 1.0, 80.0, 160.0, 1.0},  // overworld-ish factors
            };
            for (int set = 0; set < bParams.length; set++) {
                long s = 133700L + set;
                BlendedNoise bn = new BlendedNoise(new XoroshiroRandomSource(s),
                        bParams[set][0], bParams[set][1], bParams[set][2], bParams[set][3], bParams[set][4]);
                String params = "xz=" + hx(bParams[set][0]) + ";ys=" + hx(bParams[set][1])
                        + ";xzf=" + hx(bParams[set][2]) + ";yf=" + hx(bParams[set][3])
                        + ";smear=" + hx(bParams[set][4]);
                for (int i = 0; i < 16; i++) {
                    int x = i * 137 + 1600;
                    int y = i * 29 - 100;
                    int z = -i * 173 + 200;
                    // NOTE: x/y/z columns carry the INTEGER block coords here
                    w.write("noise,blended," + s + ",0," + params + ","
                            + x + "," + y + "," + z + ","
                            + hx(bn.compute(new DensityFunction.SinglePointContext(x, y, z))) + ",,,");
                    w.newLine();
                    rows++;
                }
            }

            // --- rs_noise: the LIVE RandomState's own instances (full wiring path)
            RandomState rs = level.getChunkSource().randomState();
            String[] keys = {
                "minecraft:jagged", "minecraft:spaghetti_3d_1", "minecraft:temperature",
                "minecraft:vegetation", "minecraft:continentalness", "minecraft:erosion",
                "minecraft:ridge", "minecraft:cave_cheese",
            };
            for (String key : keys) {
                ResourceKey<NormalNoise.NoiseParameters> rk = ResourceKey.create(
                        Registries.NOISE, ResourceLocation.parse(key));
                NormalNoise nn = rs.getOrCreateNoise(rk);
                for (int i = 0; i < 12; i++) {
                    double x = i * 1.375 + 0.5;
                    double y = -i * 0.8125 + 2.0;
                    double z = i * 0.25 - 1.5;
                    w.write("noise,rs_noise," + worldSeed + ",0," + key + ","
                            + hx(x) + "," + hx(y) + "," + hx(z) + ","
                            + hx(nn.getValue(x, y, z)) + ",,,");
                    w.newLine();
                    rows++;
                }
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // density.csv — all 15 router fields on the LIVE RandomState
    // ------------------------------------------------------------------

    private static final int[] XS = {1600, 1648, 1696, 1744, 1792, 1840};
    private static final int[] YS = {-64, -32, 0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 319};

    private static int writeDensity(Path path, ServerLevel level) throws IOException {
        RandomState rs = level.getChunkSource().randomState();
        NoiseRouter router = rs.router();
        java.util.LinkedHashMap<String, DensityFunction> dfs = new java.util.LinkedHashMap<>();
        dfs.put("barrier", router.barrierNoise());
        dfs.put("fluid_level_floodedness", router.fluidLevelFloodednessNoise());
        dfs.put("fluid_level_spread", router.fluidLevelSpreadNoise());
        dfs.put("lava", router.lavaNoise());
        dfs.put("temperature", router.temperature());
        dfs.put("vegetation", router.vegetation());
        dfs.put("continents", router.continents());
        dfs.put("erosion", router.erosion());
        dfs.put("depth", router.depth());
        dfs.put("ridges", router.ridges());
        dfs.put("preliminary_surface_level", router.preliminarySurfaceLevel());
        dfs.put("final_density", router.finalDensity());
        dfs.put("vein_toggle", router.veinToggle());
        dfs.put("vein_ridged", router.veinRidged());
        dfs.put("vein_gap", router.veinGap());

        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            header(w, "NoiseRouter scalar vectors (SinglePointContext compute on the live RandomState)");
            w.write("# worldSeed=" + level.getSeed());
            w.newLine();
            for (var e : dfs.entrySet()) {
                DensityFunction df = e.getValue();
                for (int x : XS) {
                    for (int y : YS) {
                        for (int z : XS) {
                            DensityFunction.SinglePointContext ctx =
                                    new DensityFunction.SinglePointContext(x, y, z);
                            double v = df.compute(ctx);
                            w.write(e.getKey() + "," + x + "," + y + "," + z + "," + hx(v));
                            w.newLine();
                            rows++;
                        }
                    }
                }
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // interp.csv — NoiseChunk cell interpolation vectors (P2.3-tail).
    // Drives a live NoiseChunk through the EXACT doFill call sequence
    // (cfr-out NoiseBasedChunkGenerator.java lines 283-344) and records
    // every NoiseInterpolator's interpolated value per block.
    // ------------------------------------------------------------------

    /** Chunk 100 min block coords — TASK-63 canon region, matches density.csv grid. */
    /** Canon: chunk (100,100). Override for targeted captures:
     *  -Dgoldendump.aquiferX=1632 -Dgoldendump.aquiferZ=1712 (task 5 bisect). */
    static final int FIRST_BLOCK_X = (int) java.util.Objects.requireNonNull(Long.getLong("goldendump.aquiferX", 1600L)).longValue();
    static final int FIRST_BLOCK_Z = (int) java.util.Objects.requireNonNull(Long.getLong("goldendump.aquiferZ", 1600L)).longValue();
    /** 16 / cellWidth(4) — one chunk column of cells. */
    static final int CELLS_XZ = 4;

    /**
     * DensityFunctions.BeardifierMarker is a PROTECTED nested class (its type
     * cannot be named from the plugin), but its INSTANCE field is public —
     * fetched reflectively and cast to the public BeardifierOrMarker interface
     * (the NoiseChunk ctor's declared parameter type).
     */
    private static DensityFunctions.BeardifierOrMarker beardifierMarker() {
        try {
            Class<?> marker = Class.forName(
                    "net.minecraft.world.level.levelgen.DensityFunctions$BeardifierMarker");
            return (DensityFunctions.BeardifierOrMarker) marker.getField("INSTANCE").get(null);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cannot fetch DensityFunctions.BeardifierMarker.INSTANCE", e);
        }
    }

    private static InterpStats writeInterp(Path path, ServerLevel level, long seed,
                                           int firstBlockX, int firstBlockZ) throws IOException {
        RandomState randomState = level.getChunkSource().randomState();

        // NoiseGeneratorSettings: the LIVE generator's own holder (stable
        // public accessor — avoids the RegistryAccess.lookupOrThrow overload
        // set entirely; javap: NoiseBasedChunkGenerator.generatorSettings()
        // -> Holder<NoiseGeneratorSettings>, NoiseBasedChunkGenerator.java).
        NoiseGeneratorSettings settings =
                ((net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator)
                        level.getChunkSource().getGenerator())
                .generatorSettings()
                .value();
        NoiseSettings noiseSettings = settings.noiseSettings().clampToHeightAccessor(level);

        // Fluid picker: functional interface computeFluid(int,int,int) -> FluidStatus
        // (NOT apply). Value is irrelevant: this capture never evaluates aquifer
        // substance (no getInterpolatedState/blockStateRule calls).
        // FluidStatus.fluidType is a BLOCK state — createLegacyBlock() converts.
        Aquifer.FluidPicker fluidPicker = (x, y, z) -> new Aquifer.FluidStatus(-54,
                Fluids.WATER.defaultFluidState().createLegacyBlock());

        NoiseChunk nc = new NoiseChunk(CELLS_XZ, randomState, firstBlockX, firstBlockZ,
                noiseSettings, beardifierMarker(), settings,
                fluidPicker, Blender.empty());

        // Reflection: package-private field NoiseChunk.interpolators -> List<?>
        java.util.List<?> interps;
        try {
            java.lang.reflect.Field f = NoiseChunk.class.getDeclaredField("interpolators");
            f.setAccessible(true);
            interps = (java.util.List<?>) f.get(nc);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cannot read NoiseChunk.interpolators via reflection", e);
        }
        int interpCount = interps.size();
        if (interpCount < 1) {
            throw new IllegalStateException("interpCount=" + interpCount
                    + " — no NoiseInterpolators in the live NoiseChunk (router wiring changed?)");
        }
        java.lang.reflect.Method[] compute = new java.lang.reflect.Method[interpCount];
        for (int i = 0; i < interpCount; i++) {
            for (java.lang.reflect.Method m : interps.get(i).getClass().getDeclaredMethods()) {
                if (m.getName().equals("compute")
                        && m.getParameterCount() == 1
                        && m.getParameterTypes()[0] == DensityFunction.FunctionContext.class) {
                    m.setAccessible(true);
                    compute[i] = m;
                    break;
                }
            }
            if (compute[i] == null) {
                throw new IllegalStateException("interpolator " + i + " ("
                        + interps.get(i).getClass().getName()
                        + "): compute(DensityFunction$FunctionContext) not found");
            }
        }

        int cellW = noiseSettings.getCellWidth();
        int cellH = noiseSettings.getCellHeight();
        int cellCountY = Mth.floorDiv(noiseSettings.height(), cellH);
        int minCellY = Mth.floorDiv(noiseSettings.minY(), cellH);

        int rows = 0;
        long nonFinite = 0;
        try (BufferedWriter w = newWriter(path)) {
            w.write("# NCF interp vectors v1");
            w.newLine();
            w.write("# worldSeed=" + seed);
            w.newLine();
            w.write("# chunk=" + (firstBlockX >> 4) + "," + (firstBlockZ >> 4));
            w.newLine();
            w.write("# firstNoise=" + firstBlockX + "," + firstBlockZ);
            w.newLine();
            w.write("# cellWidth=" + cellW + " cellHeight=" + cellH + " cellCountXZ=" + CELLS_XZ
                    + " cellCountY=" + cellCountY + " cellNoiseMinY=" + minCellY);
            w.newLine();
            w.write("# interpCount=" + interpCount);
            w.newLine();

            // doFill replica. NOTE swapSlices() after each cx column (doFill line 340).
            nc.initializeForFirstCellX();
            for (int cx = 0; cx < CELLS_XZ; cx++) {
                nc.advanceCellX(cx);
                for (int cz = 0; cz < CELLS_XZ; cz++) {
                    for (int cy = cellCountY - 1; cy >= 0; cy--) {
                        nc.selectCellYZ(cy, cz);
                        for (int inY = cellH - 1; inY >= 0; inY--) {
                            int by = (minCellY + cy) * cellH + inY;
                            nc.updateForY(by, (double) inY / (double) cellH);
                            for (int inX = 0; inX < cellW; inX++) {
                                int bx = firstBlockX + cx * cellW + inX;
                                nc.updateForX(bx, (double) inX / (double) cellW);
                                for (int inZ = 0; inZ < cellW; inZ++) {
                                    int bz = firstBlockZ + cz * cellW + inZ;
                                    nc.updateForZ(bz, (double) inZ / (double) cellW);
                                    // sanity: the chunk context must track the drive loop
                                    if (nc.blockX() != bx || nc.blockY() != by || nc.blockZ() != bz) {
                                        throw new IllegalStateException("NoiseChunk context drift: expected "
                                                + bx + "," + by + "," + bz + " got "
                                                + nc.blockX() + "," + nc.blockY() + "," + nc.blockZ());
                                    }
                                    for (int i = 0; i < interpCount; i++) {
                                        double v;
                                        try {
                                            v = (Double) compute[i].invoke(interps.get(i), nc);
                                        } catch (ReflectiveOperationException e) {
                                            throw new IllegalStateException("interpolator " + i
                                                    + " compute failed at " + bx + "," + by + "," + bz, e);
                                        }
                                        if (!Double.isFinite(v)) {
                                            nonFinite++;
                                        }
                                        w.write(i + "," + bx + "," + by + "," + bz + "," + hx(v));
                                        w.newLine();
                                        rows++;
                                    }
                                }
                            }
                        }
                    }
                }
                nc.swapSlices();
            }
            nc.stopInterpolation();
        }
        return new InterpStats(rows, interpCount, nonFinite, cellW, cellH, cellCountY, minCellY);
    }

    // ------------------------------------------------------------------
    // climate.csv — climate sampler targets + live biome result (P2.4).
    // ------------------------------------------------------------------

    /** Region A xz quart coords (canonical overworld region, chunk ~100). */
    private static final int[] CLIMATE_A_XZ = {396, 398, 400, 402, 404, 406, 408, 410};
    private static final int[] CLIMATE_A_Y = {-8, 0, 8, 16, 24, 32, 40, 48};
    /** Region B (negative coords) — comes AFTER region A. */
    private static final int[] CLIMATE_B_XZ = {-30, -28, -26};
    private static final int[] CLIMATE_B_Y = {0, 16, 32};

    private static int writeClimate(Path path, ServerLevel level, long seed) throws IOException {
        RandomState randomState = level.getChunkSource().randomState();
        Climate.Sampler sampler = randomState.sampler();
        MultiNoiseBiomeSource mnbs = multiNoiseSource(level);

        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            w.write("# NCF climate vectors v1");
            w.newLine();
            w.write("# worldSeed=" + seed);
            w.newLine();
            // Region A then Region B; qx outer -> qz -> qy innermost (row order
            // is CONTRACT: the RTree ThreadLocal last-result state persists
            // row to row, the Rust replay must visit rows in this order).
            rows += climateRegion(w, sampler, mnbs, CLIMATE_A_XZ, CLIMATE_A_XZ, CLIMATE_A_Y);
            rows += climateRegion(w, sampler, mnbs, CLIMATE_B_XZ, CLIMATE_B_XZ, CLIMATE_B_Y);
        }
        return rows;
    }

    private static int climateRegion(BufferedWriter w, Climate.Sampler sampler,
                                     MultiNoiseBiomeSource mnbs,
                                     int[] xs, int[] zs, int[] ys) throws IOException {
        int rows = 0;
        for (int qx : xs) {
            for (int qz : zs) {
                for (int qy : ys) {
                    Climate.TargetPoint tp = sampler.sample(qx, qy, qz);
                    Holder<Biome> hb = mnbs.getNoiseBiome(qx, qy, qz, sampler);
                    w.write(qx + "," + qy + "," + qz
                            + "," + tp.temperature() + "," + tp.humidity()
                            + "," + tp.continentalness() + "," + tp.erosion()
                            + "," + tp.depth() + "," + tp.weirdness()
                            + "," + registeredName(hb));
                    w.newLine();
                    rows++;
                }
            }
        }
        return rows;
    }

    private static MultiNoiseBiomeSource multiNoiseSource(ServerLevel level) {
        net.minecraft.world.level.chunk.ChunkGenerator gen = level.getChunkSource().getGenerator();
        net.minecraft.world.level.biome.BiomeSource bs = gen.getBiomeSource();
        if (!(bs instanceof MultiNoiseBiomeSource mnbs)) {
            throw new IllegalStateException("world biome source is not MultiNoiseBiomeSource: "
                    + bs.getClass().getName());
        }
        return mnbs;
    }

    private static String registeredName(Holder<Biome> h) {
        // Holder.getRegisteredName() is a public default returning the
        // "namespace:path" string — stable linkage, no registry needed.
        return h.getRegisteredName();
    }

    // ------------------------------------------------------------------
    // climate_points.csv — the LIVE overworld parameter list (P2.4).
    // ------------------------------------------------------------------

    private static int writeClimatePoints(Path path, ServerLevel level) throws IOException {
        Climate.ParameterList<?> pl = liveParameterList(multiNoiseSource(level));

        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            w.write("# NCF climate points v1 — live minecraft:overworld parameter list");
            w.newLine();
            java.util.List<?> values = (java.util.List<?>) pl.values();
            for (Object o : values) {
                @SuppressWarnings("unchecked")
                Pair<Climate.ParameterPoint, Holder<Biome>> pair =
                        (Pair<Climate.ParameterPoint, Holder<Biome>>) o;
                Climate.ParameterPoint pp = pair.getFirst();
                Climate.Parameter t = pp.temperature(), h = pp.humidity(),
                        c = pp.continentalness(), e = pp.erosion(),
                        d = pp.depth(), we = pp.weirdness();
                w.write(t.min() + "," + t.max()
                        + "," + h.min() + "," + h.max()
                        + "," + c.min() + "," + c.max()
                        + "," + e.min() + "," + e.max()
                        + "," + d.min() + "," + d.max()
                        + "," + we.min() + "," + we.max()
                        + "," + pp.offset()
                        + "," + registeredName(pair.getSecond()));
                w.newLine();
                rows++;
            }
        }
        return rows;
    }

    /**
     * Reflection path to the live source's Climate.ParameterList (documented in
     * the header): private final field `parameters` holds an
     * Either&lt;ParameterList, Holder&lt;MultiNoiseBiomeSourceParameterList&gt;&gt;.
     * The overworld source uses the preset arm (right); direct lists use left.
     */
    private static Climate.ParameterList<?> liveParameterList(MultiNoiseBiomeSource mnbs) {
        try {
            java.lang.reflect.Field f = MultiNoiseBiomeSource.class.getDeclaredField("parameters");
            f.setAccessible(true);
            Object val = f.get(mnbs);
            if (val instanceof com.mojang.datafixers.util.Either<?, ?> either) {
                if (either.left().isPresent()) {
                    return (Climate.ParameterList<?>) either.left().get();
                }
                if (either.right().isPresent()) {
                    Object holder = either.right().get();
                    Object v = ((Holder<?>) holder).value();
                    if (v instanceof MultiNoiseBiomeSourceParameterList preset) {
                        return preset.parameters();
                    }
                    throw new IllegalStateException("preset arm holds unexpected type: "
                            + v.getClass().getName());
                }
            }
            throw new IllegalStateException("MultiNoiseBiomeSource.parameters is "
                    + (val == null ? "null" : val.getClass().getName()));
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cannot read MultiNoiseBiomeSource.parameters", e);
        }
    }

    // ------------------------------------------------------------------
    // aquifer.csv + aquifer_meta.txt (task 5 bisect rig)
    // ------------------------------------------------------------------

    /**
     * Drives the REAL NoiseChunk doFill loop for chunk (1600>>4, 1600>>4) with
     * the REAL global fluid picker and captures, per block:
     *   block_x,block_y,block_z,substance(hex),decision,sched
     * where substance = the CacheAllInCell values slot the blockStateRule's
     * aquifer rule consumes (NoiseChunk.cellCaches[0].values at the forIndex
     * slot), decision = the aquifer rule result (BlockState registry name or
     * "null"), sched = shouldScheduleFluidUpdate after the call.
     * aquifer_meta.txt: grid bounds + aquiferLocationCache + aquiferCache
     * (fluid level/type per slot) + skipSamplingAboveY.
     */
    private static int writeAquifer(Path csv, Path meta, ServerLevel level, long seed,
                                    int firstBlockX, int firstBlockZ) throws Exception {
        RandomState randomState = level.getChunkSource().randomState();
        NoiseGeneratorSettings settings =
                ((net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator)
                        level.getChunkSource().getGenerator())
                .generatorSettings().value();
        NoiseSettings noiseSettings = settings.noiseSettings().clampToHeightAccessor(level);

        // NoiseBasedChunkGenerator.createFluidPicker replica (lines 92-101):
        // y < min(-54, seaLevel) -> FluidStatus(-54, LAVA) else FluidStatus(seaLevel, defaultFluid).
        net.minecraft.world.level.block.state.BlockState lavaState =
                Fluids.LAVA.defaultFluidState().createLegacyBlock();
        net.minecraft.world.level.block.state.BlockState waterState =
                Fluids.WATER.defaultFluidState().createLegacyBlock();
        int seaLevel = settings.seaLevel();
        java.util.function.IntFunction<Aquifer.FluidStatus> picker = (y) -> y < Math.min(-54, seaLevel)
                ? new Aquifer.FluidStatus(-54, lavaState)
                : new Aquifer.FluidStatus(seaLevel, waterState);
        Aquifer.FluidPicker realPicker = (x, y, z) -> picker.apply(y);

        NoiseChunk nc = new NoiseChunk(CELLS_XZ, randomState, firstBlockX, firstBlockZ,
                noiseSettings, beardifierMarker(), settings, realPicker, Blender.empty());

        // the aquifer (public accessor, NoiseChunk line 324)
        Aquifer aquifer = nc.aquifer();
        if (!(aquifer instanceof Aquifer.NoiseBasedAquifer)) {
            throw new IllegalStateException("aquifers disabled on the live settings?");
        }
        java.lang.reflect.Method computeSubstance;
        java.lang.reflect.Method schedFlag;
        try {
            computeSubstance = aquifer.getClass().getDeclaredMethod("computeSubstance",
                    DensityFunction.FunctionContext.class, double.class);
            computeSubstance.setAccessible(true);
            schedFlag = aquifer.getClass().getDeclaredMethod("shouldScheduleFluidUpdate");
            schedFlag.setAccessible(true);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("aquifer computeSubstance/shouldScheduleFluidUpdate not found", e);
        }

        // cellCaches[0].values — the substance composite (cacheAllInCell(add(final, beardifier)))
        java.lang.reflect.Field cellCachesF;
        java.lang.reflect.Field valuesF;
        java.lang.reflect.Field inCellXF;
        java.lang.reflect.Field inCellYF;
        java.lang.reflect.Field inCellZF;
        try {
            cellCachesF = NoiseChunk.class.getDeclaredField("cellCaches");
            cellCachesF.setAccessible(true);
            Class<?> cac = Class.forName("net.minecraft.world.level.levelgen.NoiseChunk$CacheAllInCell");
            valuesF = cac.getDeclaredField("values");
            valuesF.setAccessible(true);
            inCellXF = NoiseChunk.class.getDeclaredField("inCellX");
            inCellXF.setAccessible(true);
            inCellYF = NoiseChunk.class.getDeclaredField("inCellY");
            inCellYF.setAccessible(true);
            inCellZF = NoiseChunk.class.getDeclaredField("inCellZ");
            inCellZF.setAccessible(true);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cellCaches/inCell reflection failed", e);
        }
        java.util.List<?> cellCaches;
        try {
            cellCaches = (java.util.List<?>) cellCachesF.get(nc);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cellCaches read failed", e);
        }
        if (cellCaches.isEmpty()) {
            throw new IllegalStateException("no CacheAllInCell in the live NoiseChunk");
        }
        Object substanceCache;
        try {
            substanceCache = cellCaches.get(0);
        } catch (Exception e) {
            throw new IllegalStateException("cellCaches[0] read failed", e);
        }
        double[] substanceValues;
        try {
            substanceValues = (double[]) valuesF.get(substanceCache);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("CacheAllInCell.values read failed", e);
        }

        int cellW = noiseSettings.getCellWidth();
        int cellH = noiseSettings.getCellHeight();
        int cellCountY = Mth.floorDiv(noiseSettings.height(), cellH);
        int minCellY = Mth.floorDiv(noiseSettings.minY(), cellH);

        int rows = 0;
        try (BufferedWriter w = newWriter(csv)) {
            w.write("# NCF aquifer vectors v1 / worldSeed=" + seed
                    + " / chunk=" + (firstBlockX >> 4) + "," + (firstBlockZ >> 4));
            w.newLine();
            w.write("# block_x,block_y,block_z,substance(hex),decision,sched");
            w.newLine();
            nc.initializeForFirstCellX();
            for (int cx = 0; cx < CELLS_XZ; cx++) {
                nc.advanceCellX(cx);
                for (int cz = 0; cz < CELLS_XZ; cz++) {
                    for (int cy = cellCountY - 1; cy >= 0; cy--) {
                        nc.selectCellYZ(cy, cz);
                        for (int inY = cellH - 1; inY >= 0; inY--) {
                            int by = (minCellY + cy) * cellH + inY;
                            nc.updateForY(by, (double) inY / (double) cellH);
                            for (int inX = 0; inX < cellW; inX++) {
                                int bx = firstBlockX + cx * cellW + inX;
                                nc.updateForX(bx, (double) inX / (double) cellW);
                                for (int inZ = 0; inZ < cellW; inZ++) {
                                    int bz = firstBlockZ + cz * cellW + inZ;
                                    nc.updateForZ(bz, (double) inZ / (double) cellW);
                                    int icx = inCellXF.getInt(nc);
                                    int icy = inCellYF.getInt(nc);
                                    int icz = inCellZF.getInt(nc);
                                    int slot = ((cellH - 1 - icy) * cellW + icx) * cellW + icz;
                                    double substance = substanceValues[slot];
                                    Object res;
                                    try {
                                        res = computeSubstance.invoke(aquifer, nc, substance);
                                    } catch (ReflectiveOperationException e) {
                                        throw new IllegalStateException("computeSubstance failed at "
                                                + bx + "," + by + "," + bz, e);
                                    }
                                    boolean sched = (Boolean) schedFlag.invoke(aquifer);
                                    String decision;
                                    if (res == null) {
                                        decision = "null";
                                    } else {
                                        net.minecraft.world.level.block.state.BlockState bs =
                                                (net.minecraft.world.level.block.state.BlockState) res;
                                        decision = BuiltInRegistries.BLOCK.getKey(bs.getBlock()).toString();
                                        if (!bs.getProperties().isEmpty()) {
                                            StringBuilder sb = new StringBuilder();
                                            for (net.minecraft.world.level.block.state.properties.Property<?> pr : bs.getProperties()) {
                                                if (sb.length() > 0) sb.append(',');
                                                sb.append(pr.getName()).append('=').append(bs.getValue(pr));
                                            }
                                            decision += "[" + sb + "]";
                                        }
                                    }
                                    w.write(bx + "," + by + "," + bz + "," + hx(substance) + ","
                                            + decision + "," + (sched ? "1" : "0"));
                                    w.newLine();
                                    rows++;
                                }
                            }
                        }
                    }
                }
                nc.swapSlices();
            }
            nc.stopInterpolation();
        }

        // aquifer_meta.txt — caches after the full drive
        try (BufferedWriter w = newWriter(meta)) {
            w.write("# aquifer meta / seed=" + seed);
            w.newLine();
            for (String fname : new String[]{"skipSamplingAboveY", "minGridX", "minGridY", "minGridZ",
                    "gridSizeX", "gridSizeZ"}) {
                java.lang.reflect.Field f = aquifer.getClass().getDeclaredField(fname);
                f.setAccessible(true);
                w.write(fname + "=" + f.getInt(aquifer));
                w.newLine();
            }
            java.lang.reflect.Field locF = aquifer.getClass().getDeclaredField("aquiferLocationCache");
            locF.setAccessible(true);
            long[] locs = (long[]) locF.get(aquifer);
            w.write("locations.len=" + locs.length);
            w.newLine();
            for (int i = 0; i < locs.length; i++) {
                w.write("loc " + i + " " + locs[i]);
                w.newLine();
            }
            java.lang.reflect.Field statusF = aquifer.getClass().getDeclaredField("aquiferCache");
            statusF.setAccessible(true);
            Object[] statuses = (Object[]) statusF.get(aquifer);
            java.lang.reflect.Field levelF = statuses.getClass().getComponentType()
                    .getDeclaredField("fluidLevel");
            levelF.setAccessible(true);
            java.lang.reflect.Field typeF = statuses.getClass().getComponentType()
                    .getDeclaredField("fluidType");
            typeF.setAccessible(true);
            for (int i = 0; i < statuses.length; i++) {
                Object st = statuses[i];
                if (st == null) {
                    w.write("fluid " + i + " -");
                    w.newLine();
                } else {
                    int fl = levelF.getInt(st);
                    net.minecraft.world.level.block.state.BlockState ft =
                            (net.minecraft.world.level.block.state.BlockState) typeF.get(st);
                    String name = BuiltInRegistries.BLOCK.getKey(ft.getBlock()).toString();
                    w.write("fluid " + i + " " + fl + " " + name);
                    w.newLine();
                }
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // Session 6 bisect rig: capture the SurfaceRules.Context walk
    // ------------------------------------------------------------------

    /** Replicates SurfaceSystem.buildSurface's column walk on the REAL
     * generated chunk and records per visited block the exact Context state
     * the surface rule sees (reflection into package-private members). */
    public static void captureSurface(JavaPlugin plugin, int cx, int cz, CommandSender ack) throws Exception {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            Path pluginsDir = plugin.getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) throw new IllegalStateException("cannot resolve server dir");
            root = serverDir.resolve("golden");
        }
        Files.createDirectories(root);
        Path out = root.resolve("surface_trace_c" + cx + "_" + cz + ".tsv");

        var chunkAccess = level.getChunk(cx, cz);
        RandomState randomState = level.getChunkSource().randomState();
        var generator = (net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator)
            level.getChunkSource().getGenerator();
        java.lang.reflect.Method mCreateNoise = net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.class
            .getDeclaredMethod("createNoiseChunk", net.minecraft.world.level.chunk.ChunkAccess.class,
                net.minecraft.world.level.StructureManager.class, Blender.class,
                net.minecraft.world.level.levelgen.RandomState.class);
        mCreateNoise.setAccessible(true);
        var noiseChunk = chunkAccess.getOrCreateNoiseChunk(
            c -> {
                try {
                    return (net.minecraft.world.level.levelgen.NoiseChunk) mCreateNoise.invoke(generator, c,
                        level.structureManager(), Blender.empty(), randomState);
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            });

        var surfaceSystem = randomState.surfaceSystem();
        var settingsField = net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.class.getDeclaredField("settings");
        settingsField.setAccessible(true);
        Object settingsRaw = settingsField.get(generator);
        net.minecraft.world.level.levelgen.NoiseGeneratorSettings settings;
        if (settingsRaw instanceof java.util.function.Supplier<?> sup) {
            settings = (net.minecraft.world.level.levelgen.NoiseGeneratorSettings) sup.get();
        } else if (settingsRaw instanceof net.minecraft.core.Holder<?> holder) {
            settings = (net.minecraft.world.level.levelgen.NoiseGeneratorSettings) holder.value();
        } else {
            settings = (net.minecraft.world.level.levelgen.NoiseGeneratorSettings) settingsRaw;
        }
        net.minecraft.world.level.levelgen.SurfaceRules.RuleSource ruleSource = settings.surfaceRule();
        var defaultBlock = settings.defaultBlock();

        var biomeRegistry = level.registryAccess().lookupOrThrow(net.minecraft.core.registries.Registries.BIOME);
        var biomeManager = level.getBiomeManager();
        var genCtx = new net.minecraft.world.level.levelgen.WorldGenerationContext(generator, level, level);

        Class<?> ctxClass = Class.forName("net.minecraft.world.level.levelgen.SurfaceRules$Context");
        java.lang.reflect.Constructor<?> ctxCtor = ctxClass.getDeclaredConstructor(
            net.minecraft.world.level.levelgen.SurfaceSystem.class,
            net.minecraft.world.level.levelgen.RandomState.class,
            net.minecraft.world.level.chunk.ChunkAccess.class,
            net.minecraft.world.level.levelgen.NoiseChunk.class,
            java.util.function.Function.class,
            net.minecraft.core.Registry.class,
            net.minecraft.world.level.levelgen.WorldGenerationContext.class);
        ctxCtor.setAccessible(true);
        Object surfaceCtx = ctxCtor.newInstance(surfaceSystem, randomState, chunkAccess, noiseChunk,
            (java.util.function.Function<net.minecraft.core.BlockPos, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>)
                pos -> biomeManager.getBiome(pos),
            biomeRegistry, genCtx);
        net.minecraft.world.level.levelgen.SurfaceRules.SurfaceRule surfaceRule =
            ruleSource.apply((net.minecraft.world.level.levelgen.SurfaceRules.Context) surfaceCtx);
        java.lang.reflect.Method tryApply = surfaceRule.getClass().getMethod("tryApply", int.class, int.class, int.class);
        tryApply.setAccessible(true);

        java.lang.reflect.Field fBlockX = surfaceCtx.getClass().getDeclaredField("blockX");
        java.lang.reflect.Field fBlockZ = surfaceCtx.getClass().getDeclaredField("blockZ");
        java.lang.reflect.Field fSurfaceDepth = surfaceCtx.getClass().getDeclaredField("surfaceDepth");
        java.lang.reflect.Field fWaterHeight = surfaceCtx.getClass().getDeclaredField("waterHeight");
        java.lang.reflect.Field fStoneBelow = surfaceCtx.getClass().getDeclaredField("stoneDepthBelow");
        java.lang.reflect.Field fStoneAbove = surfaceCtx.getClass().getDeclaredField("stoneDepthAbove");
        java.lang.reflect.Field fBiome = surfaceCtx.getClass().getDeclaredField("biome");
        for (java.lang.reflect.Field f : new java.lang.reflect.Field[] {fBlockX, fBlockZ, fSurfaceDepth, fWaterHeight, fStoneBelow, fStoneAbove, fBiome}) {
            f.setAccessible(true);
        }
        java.lang.reflect.Method mGetMin = ctxClass.getDeclaredMethod("getMinSurfaceLevel");
        mGetMin.setAccessible(true);
        java.lang.reflect.Method mGetSecondary = ctxClass.getDeclaredMethod("getSurfaceSecondary");
        mGetSecondary.setAccessible(true);
        java.lang.reflect.Method mUpdateXZ = ctxClass.getDeclaredMethod("updateXZ", int.class, int.class);
        mUpdateXZ.setAccessible(true);
        java.lang.reflect.Method mUpdateY = ctxClass.getDeclaredMethod("updateY", int.class, int.class, int.class, int.class, int.class, int.class);
        mUpdateY.setAccessible(true);

        int minBlockX = cx << 4;
        int minBlockZ = cz << 4;
        int minY = chunkAccess.getMinY();
        var sb = new StringBuilder();
        sb.append("x\tz\ty\tbiome\tsurfaceDepth\tminSurfaceLevel\tsecondary\twaterHeight\tstoneAbove\tstoneBelow\treplacement\n");
        int n = 0;
        for (int i = 0; i < 16; ++i) {
            for (int i1 = 0; i1 < 16; ++i1) {
                int i2 = minBlockX + i;
                int i3 = minBlockZ + i1;
                int i4 = chunkAccess.getHeight(net.minecraft.world.level.levelgen.Heightmap.Types.WORLD_SURFACE_WG, i, i1) + 1;
                mUpdateXZ.invoke(surfaceCtx, i2, i3);
                int i5 = chunkAccess.getHeight(net.minecraft.world.level.levelgen.Heightmap.Types.WORLD_SURFACE_WG, i, i1) + 1;
                int i6 = 0;
                int i7 = Integer.MIN_VALUE;
                int i8 = Integer.MAX_VALUE;
                for (int i9 = i5; i9 >= minY; --i9) {
                    var block = chunkAccess.getBlockState(new net.minecraft.core.BlockPos(i2, i9, i3));
                    if (block.isAir()) { i6 = 0; i7 = Integer.MIN_VALUE; continue; }
                    if (!block.getFluidState().isEmpty()) {
                        if (i7 != Integer.MIN_VALUE) continue;
                        i7 = i9 + 1;
                        continue;
                    }
                    if (i8 >= i9) {
                        i8 = -32512;
                        for (int i10 = i9 - 1; i10 >= minY - 1; --i10) {
                            var below = chunkAccess.getBlockState(new net.minecraft.core.BlockPos(i2, i10, i3));
                            if (isStone(below)) continue;
                            i8 = i10 + 1;
                            break;
                        }
                    }
                    mUpdateY.invoke(surfaceCtx, ++i6, i9 - i8 + 1, i7, i2, i9, i3);
                    Object biomeVal = fBiome.get(surfaceCtx);
                    net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> biomeHolder = null;
                    if (biomeVal instanceof java.util.function.Supplier<?> sup) {
                        biomeHolder = (net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>) sup.get();
                    } else if (biomeVal instanceof net.minecraft.core.Holder<?> h) {
                        biomeHolder = (net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>) h;
                    }
                    String biomeName = biomeHolder == null ? "?"
                        : biomeHolder.unwrapKey().map(k -> k.location().toString()).orElse("?");
                    Object replacement = block.equals(defaultBlock)
                        ? tryApply.invoke(surfaceRule, i2, i9, i3) : null;
                    String replName = replacement == null ? "-"
                        : ((net.minecraft.world.level.block.state.BlockState) replacement).getBlock().toString()
                            .replaceAll("[\\[\\]{}=,]", "");
                    sb.append(i2).append('\t').append(i3).append('\t').append(i9)
                      .append('\t').append(biomeName)
                      .append('\t').append(fSurfaceDepth.getInt(surfaceCtx))
                      .append('\t').append(mGetMin.invoke(surfaceCtx))
                      .append('\t').append(mGetSecondary.invoke(surfaceCtx))
                      .append('\t').append(fWaterHeight.getInt(surfaceCtx))
                      .append('\t').append(fStoneAbove.getInt(surfaceCtx))
                      .append('\t').append(fStoneBelow.getInt(surfaceCtx))
                      .append('\t').append(replName)
                      .append('\n');
                    ++n;
                    if (replacement != null) {
                        chunkAccess.setBlockState(new net.minecraft.core.BlockPos(i2, i9, i3),
                            (net.minecraft.world.level.block.state.BlockState) replacement);
                    }
                }
            }
        }
        Files.writeString(out, sb.toString());
        plugin.getLogger().info("GOLDEN SURFACE CAPTURE COMPLETE n=" + n + " dir=" + out);
        ack.sendMessage("goldensurface: complete n=" + n + " dir=" + out);
    }

    private static boolean isStone(net.minecraft.world.level.block.state.BlockState s) {
        return !s.isAir() && s.getFluidState().isEmpty();
    }

    // ------------------------------------------------------------------
    // T38-B bisect rig: /goldendensity <blockX> <blockZ> <y0> <y1> <label>
    // Dumps the FULL vector family at an ARBITRARY chunk column (negative
    // coords included — the gate-P2 blob region was never covered by the
    // canon /goldenvec corpus, whose density grid is 1600..1840 only):
    //   interp.csv    — the 16 NoiseInterpolator per-block values (same
    //                   format/headers as /goldenvec, `# firstNoise=` carries
    //                   the coords; veccheck parses them since T38-B)
    //   aquifer.csv + aquifer_meta.txt — substance/decision per block
    //                   (same format as /goldenvec; aquacheck reads the
    //                   chunk from the rows)
    //   density.csv   — field,x,y,z,value rows (same format as /goldenvec):
    //                   (a) the 15 router fields on the cell-corner lattice
    //                   (x/z = base..base+16 step 4, y = full height step 8),
    //                   (b) the 6 aquifer-relevant fields at EVERY block in
    //                   the y band [y0..y1] (the substance/aquifer inputs
    //                   that the corner lattice cannot cover).
    // No JVM flags: the coords arrive as command arguments.
    // ------------------------------------------------------------------

    public static void captureDensity(JavaPlugin plugin, int bx, int bz, int y0, int y1,
                                      String label, CommandSender ack) {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        long seed = level.getSeed();

        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            Path pluginsDir = plugin.getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) {
                ack.sendMessage("goldendensity: cannot resolve server dir — set -Dgoldendump.out=<dir>");
                return;
            }
            root = serverDir.resolve("golden");
        }
        Path vecDir = root.resolve(label).resolve("vectors");
        try {
            Files.createDirectories(vecDir);
        } catch (IOException e) {
            ack.sendMessage("goldendensity: cannot create " + vecDir + ": " + e);
            return;
        }

        long t0 = System.nanoTime();
        try {
            // T38-B rig fix (addendum 23): writeInterp/writeAquifer drive the
            // per-CHUNK cell machinery (NoiseChunk cellStart/inCell indices +
            // CacheAllInCell 4x8x4=128 slots) — their base MUST be the chunk
            // corner, otherwise inCellX/Y/Z go out of range
            // (ArrayIndexOutOfBoundsException 128/128 — reproduced locally for
            // misaligned bases 253/-246/284; the old blob base -160 was
            // chunk-aligned by luck). Snap the machine dumps to the chunk
            // corner; writeDensityBlob keeps the caller's (possibly
            // misaligned) block coords — pure df.compute is coord-free.
            int machineBaseX = bx & ~15;
            int machineBaseZ = bz & ~15;
            InterpStats interpStats = writeInterp(vecDir.resolve("interp.csv"), level, seed, machineBaseX, machineBaseZ);
            int aquaRows = writeAquifer(vecDir.resolve("aquifer.csv"),
                    vecDir.resolve("aquifer_meta.txt"), level, seed, machineBaseX, machineBaseZ);
            int densityRows = writeDensityBlob(vecDir.resolve("density.csv"), level, bx, bz, y0, y1);
            long ms = (System.nanoTime() - t0) / 1_000_000L;
            plugin.getLogger().info("GOLDEN DENSITY COMPLETE rows=" + (interpStats.rows() + aquaRows + densityRows)
                    + " ms=" + ms + " dir=" + vecDir + " chunk=" + (bx >> 4) + "," + (bz >> 4)
                    + " band=" + y0 + ".." + y1);
            ack.sendMessage("goldendensity: complete dir=" + vecDir);
        } catch (Throwable t) {
            plugin.getLogger().warning("GOLDEN DENSITY FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("goldendensity: FAILED: " + t);
        }
    }

    /** The 15 router fields (same set/order as writeDensity). */
    private static java.util.LinkedHashMap<String, DensityFunction> routerFields(ServerLevel level) {
        RandomState rs = level.getChunkSource().randomState();
        NoiseRouter router = rs.router();
        java.util.LinkedHashMap<String, DensityFunction> dfs = new java.util.LinkedHashMap<>();
        dfs.put("barrier", router.barrierNoise());
        dfs.put("fluid_level_floodedness", router.fluidLevelFloodednessNoise());
        dfs.put("fluid_level_spread", router.fluidLevelSpreadNoise());
        dfs.put("lava", router.lavaNoise());
        dfs.put("temperature", router.temperature());
        dfs.put("vegetation", router.vegetation());
        dfs.put("continents", router.continents());
        dfs.put("erosion", router.erosion());
        dfs.put("depth", router.depth());
        dfs.put("ridges", router.ridges());
        dfs.put("preliminary_surface_level", router.preliminarySurfaceLevel());
        dfs.put("final_density", router.finalDensity());
        dfs.put("vein_toggle", router.veinToggle());
        dfs.put("vein_ridged", router.veinRidged());
        dfs.put("vein_gap", router.veinGap());
        return dfs;
    }

    private static int writeDensityBlob(Path path, ServerLevel level, int bx, int bz, int y0, int y1) throws IOException {
        NoiseGeneratorSettings settings =
                ((net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator)
                        level.getChunkSource().getGenerator())
                .generatorSettings().value();
        NoiseSettings noiseSettings = settings.noiseSettings().clampToHeightAccessor(level);
        int cellH = noiseSettings.getCellHeight();
        int minCellY = Mth.floorDiv(noiseSettings.minY(), cellH);
        int cellCountY = Mth.floorDiv(noiseSettings.height(), cellH);

        java.util.LinkedHashMap<String, DensityFunction> dfs = routerFields(level);
        // the aquifer-relevant subset sampled at EVERY block in the band
        // (+ final_density since P5.3 inc6: the coarse cell-corner lattice
        // does NOT cover per-block divergence inside a cell — the gate-p2
        // resid blobs diverge at blocks the coarse grid never samples)
        String[] bandFields = {"barrier", "fluid_level_floodedness", "fluid_level_spread",
                "lava", "erosion", "depth", "final_density"};

        int rows = 0;
        try (BufferedWriter w = newWriter(path)) {
            header(w, "NoiseRouter scalar vectors — blob region (T38-B) / base="
                    + bx + "," + bz + " band=" + y0 + ".." + y1);
            w.write("# worldSeed=" + level.getSeed());
            w.newLine();
            // (a) cell-corner lattice, full height — the interpolation inputs
            for (var e : dfs.entrySet()) {
                DensityFunction df = e.getValue();
                for (int x = bx; x <= bx + 16; x += 4) {
                    for (int cy = 0; cy <= cellCountY; cy++) {
                        int y = (minCellY + cy) * cellH;
                        for (int z = bz; z <= bz + 16; z += 4) {
                            double v = df.compute(new DensityFunction.SinglePointContext(x, y, z));
                            w.write(e.getKey() + "," + x + "," + y + "," + z + "," + hx(v));
                            w.newLine();
                            rows++;
                        }
                    }
                }
            }
            // (b) per-block band sweep — the aquifer's own inputs
            for (String fname : bandFields) {
                DensityFunction df = dfs.get(fname);
                for (int y = y0; y <= y1; y++) {
                    for (int x = bx; x < bx + 16; x++) {
                        for (int z = bz; z < bz + 16; z++) {
                            double v = df.compute(new DensityFunction.SinglePointContext(x, y, z));
                            w.write(fname + "," + x + "," + y + "," + z + "," + hx(v));
                            w.newLine();
                            rows++;
                        }
                    }
                }
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // T38-B resid bisect: /aquafields <blockX> <blockY> <blockZ>
    //
    // Dumps the MACHINE-WRAPPED aquifer field values — the NoiseBasedAquifer's
    // private DensityFunction fields are noiseRouter1 = noiseRouter.mapAll(wrap)
    // versions (NoiseChunk.FlatCache@y0 / NoiseChunk.Cache2D instances), NOT the
    // raw randomState router the density.csv capture evaluates. Plus the REAL
    // computeFluid status and the deep-dark verdict at the position, using the
    // SAME wrapped fields computeSurfaceLevel sees. This is the status-layer
    // oracle that closes the "all 15 fields bit-exact yet the status differs"
    // contradiction (worklog addenda 25/26).
    // ------------------------------------------------------------------
    public static void aquaFields(JavaPlugin plugin, int x, int y, int z, CommandSender ack) {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        try {
            NoiseGeneratorSettings settings =
                    ((net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator)
                            level.getChunkSource().getGenerator())
                    .generatorSettings().value();
            NoiseSettings noiseSettings = settings.noiseSettings().clampToHeightAccessor(level);

            net.minecraft.world.level.block.state.BlockState lavaState =
                    Fluids.LAVA.defaultFluidState().createLegacyBlock();
            net.minecraft.world.level.block.state.BlockState waterState =
                    Fluids.WATER.defaultFluidState().createLegacyBlock();
            int seaLevel = settings.seaLevel();
            Aquifer.FluidPicker realPicker = (px, py, pz) -> py < Math.min(-54, seaLevel)
                    ? new Aquifer.FluidStatus(-54, lavaState)
                    : new Aquifer.FluidStatus(seaLevel, waterState);

            RandomState randomState = level.getChunkSource().randomState();
            int machineBaseX = x & ~15;
            int machineBaseZ = z & ~15;
            NoiseChunk nc = new NoiseChunk(CELLS_XZ, randomState, machineBaseX, machineBaseZ,
                    noiseSettings, beardifierMarker(), settings, realPicker, Blender.empty());

            Aquifer aquifer = nc.aquifer();
            if (!(aquifer instanceof Aquifer.NoiseBasedAquifer)) {
                ack.sendMessage("aquafields: aquifers disabled on the live settings?");
                return;
            }
            Class<?> c = aquifer.getClass();
            java.util.LinkedHashMap<String, DensityFunction> wrapped = new java.util.LinkedHashMap<>();
            for (String n : new String[]{"barrierNoise", "fluidLevelFloodednessNoise",
                    "fluidLevelSpreadNoise", "lavaNoise", "erosion", "depth"}) {
                java.lang.reflect.Field fl = c.getDeclaredField(n);
                fl.setAccessible(true);
                wrapped.put(n, (DensityFunction) fl.get(aquifer));
            }
            DensityFunction.FunctionContext ctx = new DensityFunction.SinglePointContext(x, y, z);
            StringBuilder line = new StringBuilder("aquafields @(" + x + "," + y + "," + z + ")");
            for (var e : wrapped.entrySet()) {
                double v = e.getValue().compute(ctx);
                line.append(" ").append(e.getKey()).append("=").append(Double.toHexString(v));
            }
            boolean deepDark = net.minecraft.world.level.biome.OverworldBiomeBuilder
                    .isDeepDarkRegion(wrapped.get("erosion"), wrapped.get("depth"), ctx);
            line.append(" isDeepDark=").append(deepDark);
            // the real computeFluid (private) — the status the aquifer caches
            java.lang.reflect.Method cf = c.getDeclaredMethod("computeFluid",
                    int.class, int.class, int.class);
            cf.setAccessible(true);
            Object status = cf.invoke(aquifer, x, y, z);
            int fluidLevel = (Integer) status.getClass().getDeclaredMethod("fluidLevel").invoke(status);
            Object fluidType = status.getClass().getDeclaredMethod("fluidType").invoke(status);
            line.append(" computeFluid=").append(fluidLevel).append("/").append(fluidType);
            // prelim surface levels for the 13-offset scan around the column
            // SURFACE_SAMPLING_OFFSETS_IN_CHUNKS = (0,0),(-2,-1),(-1,-1),(0,-1),
            // (1,-1),(-3,0),(-2,0),(-1,0),(1,0),(-2,1),(-1,1),(0,1),(1,1)
            int[] offX = {0, -2, -1, 0, 1, -3, -2, -1, 1, -2, -1, 0, 1};
            int[] offZ = {0, -1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1};
            java.lang.reflect.Method psl = NoiseChunk.class.getDeclaredMethod(
                    "preliminarySurfaceLevel", int.class, int.class);
            psl.setAccessible(true);
            StringBuilder cols = new StringBuilder(" prelim[");
            for (int i = 0; i < offX.length; i++) {
                int px = x + offX[i] * 16;
                int pz = z + offZ[i] * 16;
                cols.append(offX[i]).append(",").append(offZ[i]).append("=")
                        .append(psl.invoke(nc, px, pz)).append(" ");
            }
            cols.append("]");
            line.append(cols);
            String out = line.toString();
            plugin.getLogger().info(out);
            ack.sendMessage(out);
        } catch (Throwable t) {
            plugin.getLogger().warning("AQUAFIELDS FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("aquafields: FAILED: " + t);
        }
    }

    /**
     * NCF P5.3 increment 2d oracle — /goldenpieces <chunkX> <chunkZ>:
     * force-generate the chunk to STRUCTURE_STARTS and dump every valid
     * StructureStart's pieces (bounding box, ground level delta, rotation,
     * element, position, junctions) as JSON — the bit-exact oracle for the
     * Rust piece engine (piece_dump). JigsawJunction fields map 1:1 to the
     * Rust Junction (source_x, source_ground_y, source_z, delta_y,
     * dest_projection); BoundingBox min/max are INCLUSIVE like the Rust
     * InclusiveBox; piece order = StructurePiecesBuilder.build() order
     * (the placer's push order — same as the Rust Vec<Piece>).
     */
    public static void capturePieces(JavaPlugin plugin, int cx, int cz, CommandSender ack) {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            Path pluginsDir = plugin.getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) {
                ack.sendMessage("goldenpieces: cannot resolve server dir — set -Dgoldendump.out=<dir>");
                return;
            }
            root = serverDir.resolve("golden");
        }
        Path out = root.resolve("pieces_" + cx + "_" + cz + ".json");
        try {
            ChunkAccess chunk = level.getChunkSource().getChunk(cx, cz, ChunkStatus.STRUCTURE_STARTS, true);
            java.util.Map<Structure, StructureStart> starts = chunk.getAllStarts();
            java.util.IdentityHashMap<Structure, String> names = new java.util.IdentityHashMap<>();
            level.registryAccess().lookupOrThrow(Registries.STRUCTURE).listElements()
                    .forEach(h -> h.unwrapKey().ifPresent(k ->
                            names.put(h.value(), String.valueOf(k.location()))));
            java.util.List<java.util.Map.Entry<Structure, StructureStart>> entries =
                    new java.util.ArrayList<>(starts.entrySet());
            entries.sort(java.util.Comparator.comparing(
                    e -> names.getOrDefault(e.getKey(), "?")));
            StringBuilder sb = new StringBuilder();
            sb.append("{\n \"chunk\": [").append(cx).append(", ").append(cz).append("],\n");
            sb.append(" \"seed\": ").append(level.getSeed()).append(",\n");
            sb.append(" \"starts\": [\n");
            boolean firstStart = true;
            for (java.util.Map.Entry<Structure, StructureStart> en : entries) {
                StructureStart start = en.getValue();
                if (start == null || !start.isValid()) continue;
                if (!firstStart) sb.append(",\n");
                firstStart = false;
                sb.append("  {\"structure\": \"").append(names.getOrDefault(en.getKey(), "?"))
                  .append("\", \"chunkPos\": [").append(start.getChunkPos().x)
                  .append(", ").append(start.getChunkPos().z).append("],\n");
                sb.append("   \"pieces\": [\n");
                boolean firstPiece = true;
                for (StructurePiece piece : start.getPieces()) {
                    if (!firstPiece) sb.append(",\n");
                    firstPiece = false;
                    BoundingBox b = piece.getBoundingBox();
                    sb.append("    {\"box\": [").append(b.minX()).append(", ").append(b.minY())
                      .append(", ").append(b.minZ()).append(", ").append(b.maxX())
                      .append(", ").append(b.maxY()).append(", ").append(b.maxZ()).append("]");
                    sb.append(", \"rotation\": \"").append(piece.getRotation().name()).append("\"");
                    sb.append(", \"type\": \"").append(piece.getClass().getSimpleName()).append("\"");
                    if (piece instanceof PoolElementStructurePiece pe) {
                        sb.append(", \"gld\": ").append(pe.getGroundLevelDelta());
                        sb.append(", \"element\": \"").append(pe.getElement().toString().replace("\"", "'")).append("\"");
                        sb.append(", \"position\": [").append(pe.getPosition().getX())
                          .append(", ").append(pe.getPosition().getY())
                          .append(", ").append(pe.getPosition().getZ()).append("]");
                        sb.append(", \"junctions\": [");
                        boolean firstJ = true;
                        for (JigsawJunction j : pe.getJunctions()) {
                            if (!firstJ) sb.append(", ");
                            firstJ = false;
                            sb.append("[").append(j.getSourceX()).append(", ")
                              .append(j.getSourceGroundY()).append(", ")
                              .append(j.getSourceZ()).append(", ")
                              .append(j.getDeltaY()).append(", \"")
                              .append(j.getDestProjection().getName()).append("\"]");
                        }
                        sb.append("]");
                    }
                    sb.append("}");
                }
                sb.append("\n   ]}");
            }
            sb.append("\n ]}\n");
            Files.createDirectories(out.getParent());
            Files.writeString(out, sb.toString());
            String msg = "goldenpieces: wrote " + out + " (" + sb.length() + " bytes)";
            plugin.getLogger().info(msg);
            ack.sendMessage(msg);
        } catch (Throwable t) {
            plugin.getLogger().warning("GOLDENPIECES FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("goldenpieces: FAILED: " + t);
        }
    }

    /**
     * P5.3 inc6 diagnostics: /goldenrefs <chunkX> <chunkZ> — dump the chunk's
     * STRUCTURE_REFERENCES map (structure -> origin chunk set) AND the exact
     * Beardifier input list (StructureManager.startsForStructure with the
     * terrainAdaptation != NONE filter), each start's bounding box and the
     * pieces that pass isCloseToChunk(12). This settles reference-set
     * semantics empirically (the createReferences box-touch scan).
     */
    public static void captureRefs(JavaPlugin plugin, int cx, int cz, CommandSender ack) {
        ServerLevel level = ((CraftWorld) plugin.getServer().getWorlds().get(0)).getHandle();
        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            Path pluginsDir = plugin.getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) {
                ack.sendMessage("goldenrefs: cannot resolve server dir — set -Dgoldendump.out=<dir>");
                return;
            }
            root = serverDir.resolve("golden");
        }
        Path out = root.resolve("refs_" + cx + "_" + cz + ".json");
        try {
            // force STRUCTURE_REFERENCES so the createReferences scan ran
            ChunkAccess chunk = level.getChunkSource().getChunk(cx, cz, ChunkStatus.STRUCTURE_REFERENCES, true);
            java.util.IdentityHashMap<Structure, String> names = new java.util.IdentityHashMap<>();
            level.registryAccess().lookupOrThrow(Registries.STRUCTURE).listElements()
                    .forEach(h -> h.unwrapKey().ifPresent(k ->
                            names.put(h.value(), String.valueOf(k.location()))));
            StringBuilder sb = new StringBuilder();
            sb.append("{\n \"chunk\": [").append(cx).append(", ").append(cz).append("],\n");
            sb.append(" \"seed\": ").append(level.getSeed()).append(",\n");
            // 1. raw references map
            sb.append(" \"references\": [\n");
            var refs = chunk.getAllReferences();
            boolean firstRef = true;
            for (var e : refs.entrySet()) {
                Structure st = e.getKey();
                if (!firstRef) sb.append(",\n");
                firstRef = false;
                sb.append("  {\"structure\": \"").append(names.getOrDefault(st, "?")).append("\", \"origins\": [");
                boolean firstLong = true;
                for (java.util.Iterator<Long> it = e.getValue().iterator(); it.hasNext(); ) {
                    long l = it.next();
                    net.minecraft.world.level.ChunkPos p = new net.minecraft.world.level.ChunkPos(l);
                    if (!firstLong) sb.append(", ");
                    sb.append("[").append(p.x).append(", ").append(p.z).append("]");
                    firstLong = false;
                }
                sb.append("]}");
            }
            sb.append("\n ],\n");
            // 2. the EXACT Beardifier input: startsForStructure with the
            // terrainAdaptation != NONE predicate
            net.minecraft.world.level.StructureManager sm = level.structureManager();
            var beardStarts = sm.startsForStructure(
                    new net.minecraft.world.level.ChunkPos(cx, cz),
                    st -> st.terrainAdaptation() != net.minecraft.world.level.levelgen.structure.TerrainAdjustment.NONE);
            sb.append(" \"beardifier_starts\": [\n");
            boolean firstStart = true;
            for (var start : beardStarts) {
                if (!firstStart) sb.append(",\n");
                firstStart = false;
                sb.append("  {\"structure\": \"").append(names.getOrDefault(start.getStructure(), "?"))
                        .append("\", \"chunkPos\": [").append(start.getChunkPos().x).append(", ").append(start.getChunkPos().z)
                        .append("], \"adjustment\": \"").append(start.getStructure().terrainAdaptation())
                        .append("\", \"box\": [").append(start.getBoundingBox().minX()).append(", ").append(start.getBoundingBox().minY())
                        .append(", ").append(start.getBoundingBox().minZ()).append(", ").append(start.getBoundingBox().maxX())
                        .append(", ").append(start.getBoundingBox().maxY()).append(", ").append(start.getBoundingBox().maxZ())
                        .append("], \"pieces\": [\n");
                boolean firstPiece = true;
                for (var piece : start.getPieces()) {
                    if (!piece.isCloseToChunk(new net.minecraft.world.level.ChunkPos(cx, cz), 12)) continue;
                    if (!firstPiece) sb.append(",\n");
                    firstPiece = false;
                    var b = piece.getBoundingBox();
                    sb.append("   [").append(b.minX()).append(", ").append(b.minY()).append(", ").append(b.minZ())
                            .append(", ").append(b.maxX()).append(", ").append(b.maxY()).append(", ").append(b.maxZ()).append("]");
                }
                sb.append("\n  ]}");
            }
            sb.append("\n ]}\n");
            Files.createDirectories(out.getParent());
            Files.writeString(out, sb.toString());
            String msg = "goldenrefs: wrote " + out + " (" + sb.length() + " bytes)";
            plugin.getLogger().info(msg);
            ack.sendMessage(msg);
        } catch (Throwable t) {
            plugin.getLogger().warning("GOLDENREFS FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("goldenrefs: FAILED: " + t);
        }
    }
}
