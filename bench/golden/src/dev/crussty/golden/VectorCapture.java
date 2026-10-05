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
import net.minecraft.world.level.levelgen.blending.Blender;
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
        try {
            rows = writeRandom(vecDir.resolve("random.csv"))
                    + writeNoise(vecDir.resolve("noise.csv"), level, seed)
                    + writeDensity(vecDir.resolve("density.csv"), level);
            interpStats = writeInterp(vecDir.resolve("interp.csv"), level, seed);
            rows += interpStats.rows();
            climateRows = writeClimate(vecDir.resolve("climate.csv"), level, seed);
            rows += climateRows;
            climatePointRows = writeClimatePoints(vecDir.resolve("climate_points.csv"), level);
            rows += climatePointRows;
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
    static final int FIRST_BLOCK_X = 1600;
    static final int FIRST_BLOCK_Z = 1600;
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

    private static InterpStats writeInterp(Path path, ServerLevel level, long seed) throws IOException {
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

        NoiseChunk nc = new NoiseChunk(CELLS_XZ, randomState, FIRST_BLOCK_X, FIRST_BLOCK_Z,
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
            w.write("# chunk=" + (FIRST_BLOCK_X >> 4) + "," + (FIRST_BLOCK_Z >> 4));
            w.newLine();
            w.write("# firstNoise=" + FIRST_BLOCK_X + "," + FIRST_BLOCK_Z);
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
                                int bx = FIRST_BLOCK_X + cx * cellW + inX;
                                nc.updateForX(bx, (double) inX / (double) cellW);
                                for (int inZ = 0; inZ < cellW; inZ++) {
                                    int bz = FIRST_BLOCK_Z + cz * cellW + inZ;
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
}
