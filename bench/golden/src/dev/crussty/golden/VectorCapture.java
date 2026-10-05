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
// Purity law: identical to GoldenDumperPlugin — vanilla boot only, NO
// CRUSSTY agent. Determinism: no wall clock, no world-state dependence
// beyond the world seed; grids are fixed constants.
//
// API verification status (2026-10-05, javap on the mojang-mapped jar):
//   ServerChunkCache.randomState()                              public
//   NoiseRouter record accessors (barrierNoise()..veinGap())    public
//   RandomState.getOrCreateNoise(ResourceKey<NoiseParameters>)  public
//   DensityFunction.SinglePointContext(int,int,int)             public record ctor
//   RandomSupport.mixStafford13 / upgradeSeedTo128bit           public
//   LegacyPositionalRandomFactory(long) public ctor             public
//   XoroshiroPositionalRandomFactory(long,long) public ctor     public
// ============================================================================

import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

import it.unimi.dsi.fastutil.doubles.DoubleArrayList;
import net.minecraft.core.registries.Registries;
import net.minecraft.resources.ResourceKey;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.levelgen.DensityFunction;
import net.minecraft.world.level.levelgen.LegacyRandomSource;
import net.minecraft.world.level.levelgen.NoiseRouter;
import net.minecraft.world.level.levelgen.RandomState;
import net.minecraft.world.level.levelgen.RandomSupport;
import net.minecraft.world.level.levelgen.XoroshiroRandomSource;
import net.minecraft.world.level.levelgen.synth.BlendedNoise;
import net.minecraft.world.level.levelgen.synth.ImprovedNoise;
import net.minecraft.world.level.levelgen.synth.NormalNoise;
import net.minecraft.world.level.levelgen.synth.PerlinNoise;

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
        try {
            rows = writeRandom(vecDir.resolve("random.csv"))
                    + writeNoise(vecDir.resolve("noise.csv"), level, seed)
                    + writeDensity(vecDir.resolve("density.csv"), level);
        } catch (Throwable t) {
            plugin.getLogger().warning("GOLDEN VECTOR FAILED: " + t);
            t.printStackTrace();
            ack.sendMessage("goldenvec: FAILED: " + t);
            return;
        }
        long ms = (System.nanoTime() - t0) / 1_000_000L;
        plugin.getLogger().info("GOLDEN VECTOR COMPLETE rows=" + rows + " ms=" + ms + " dir=" + vecDir);
        ack.sendMessage("goldenvec: complete rows=" + rows + " dir=" + vecDir);
    }

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
}
