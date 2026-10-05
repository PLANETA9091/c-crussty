/*
 * Decompiled with CFR 0.152.
 * 
 * Could not load the following classes:
 *  com.mojang.datafixers.util.Pair
 *  net.minecraft.SharedConstants
 *  net.minecraft.core.Holder
 *  net.minecraft.core.HolderLookup$Provider
 *  net.minecraft.core.HolderLookup$RegistryLookup
 *  net.minecraft.core.Registry
 *  net.minecraft.core.RegistryAccess
 *  net.minecraft.core.RegistrySetBuilder
 *  net.minecraft.core.registries.BuiltInRegistries
 *  net.minecraft.core.registries.Registries
 *  net.minecraft.data.worldgen.NoiseData
 *  net.minecraft.data.worldgen.TerrainProvider
 *  net.minecraft.resources.ResourceKey
 *  net.minecraft.util.BoundedFloatFunction
 *  net.minecraft.util.CubicSpline
 *  net.minecraft.util.CubicSpline$Multipoint
 *  net.minecraft.util.VisibleForDebug
 *  net.minecraft.world.level.biome.Biome
 *  net.minecraft.world.level.biome.Biomes
 *  net.minecraft.world.level.biome.Climate
 *  net.minecraft.world.level.biome.Climate$Parameter
 *  net.minecraft.world.level.biome.Climate$ParameterPoint
 *  net.minecraft.world.level.levelgen.DensityFunction
 *  net.minecraft.world.level.levelgen.DensityFunction$FunctionContext
 *  net.minecraft.world.level.levelgen.DensityFunctions$Spline$Coordinate
 *  net.minecraft.world.level.levelgen.NoiseRouterData
 */
package net.minecraft.world.level.biome;

import com.mojang.datafixers.util.Pair;
import java.util.List;
import java.util.function.Consumer;
import net.minecraft.SharedConstants;
import net.minecraft.core.Holder;
import net.minecraft.core.HolderLookup;
import net.minecraft.core.Registry;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.RegistrySetBuilder;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.data.worldgen.NoiseData;
import net.minecraft.data.worldgen.TerrainProvider;
import net.minecraft.resources.ResourceKey;
import net.minecraft.util.BoundedFloatFunction;
import net.minecraft.util.CubicSpline;
import net.minecraft.util.VisibleForDebug;
import net.minecraft.world.level.biome.Biome;
import net.minecraft.world.level.biome.Biomes;
import net.minecraft.world.level.biome.Climate;
import net.minecraft.world.level.levelgen.DensityFunction;
import net.minecraft.world.level.levelgen.DensityFunctions;
import net.minecraft.world.level.levelgen.NoiseRouterData;

public final class OverworldBiomeBuilder {
    private static final float VALLEY_SIZE = 0.05f;
    private static final float LOW_START = 0.26666668f;
    public static final float HIGH_START = 0.4f;
    private static final float HIGH_END = 0.93333334f;
    private static final float PEAK_SIZE = 0.1f;
    public static final float PEAK_START = 0.56666666f;
    private static final float PEAK_END = 0.7666667f;
    public static final float NEAR_INLAND_START = -0.11f;
    public static final float MID_INLAND_START = 0.03f;
    public static final float FAR_INLAND_START = 0.3f;
    public static final float EROSION_INDEX_1_START = -0.78f;
    public static final float EROSION_INDEX_2_START = -0.375f;
    private static final float EROSION_DEEP_DARK_DRYNESS_THRESHOLD = -0.225f;
    private static final float DEPTH_DEEP_DARK_DRYNESS_THRESHOLD = 0.9f;
    private final Climate.Parameter FULL_RANGE = Climate.Parameter.span((float)-1.0f, (float)1.0f);
    private final Climate.Parameter[] temperatures = new Climate.Parameter[]{Climate.Parameter.span((float)-1.0f, (float)-0.45f), Climate.Parameter.span((float)-0.45f, (float)-0.15f), Climate.Parameter.span((float)-0.15f, (float)0.2f), Climate.Parameter.span((float)0.2f, (float)0.55f), Climate.Parameter.span((float)0.55f, (float)1.0f)};
    private final Climate.Parameter[] humidities = new Climate.Parameter[]{Climate.Parameter.span((float)-1.0f, (float)-0.35f), Climate.Parameter.span((float)-0.35f, (float)-0.1f), Climate.Parameter.span((float)-0.1f, (float)0.1f), Climate.Parameter.span((float)0.1f, (float)0.3f), Climate.Parameter.span((float)0.3f, (float)1.0f)};
    private final Climate.Parameter[] erosions = new Climate.Parameter[]{Climate.Parameter.span((float)-1.0f, (float)-0.78f), Climate.Parameter.span((float)-0.78f, (float)-0.375f), Climate.Parameter.span((float)-0.375f, (float)-0.2225f), Climate.Parameter.span((float)-0.2225f, (float)0.05f), Climate.Parameter.span((float)0.05f, (float)0.45f), Climate.Parameter.span((float)0.45f, (float)0.55f), Climate.Parameter.span((float)0.55f, (float)1.0f)};
    private final Climate.Parameter FROZEN_RANGE = this.temperatures[0];
    private final Climate.Parameter UNFROZEN_RANGE = Climate.Parameter.span((Climate.Parameter)this.temperatures[1], (Climate.Parameter)this.temperatures[4]);
    private final Climate.Parameter mushroomFieldsContinentalness = Climate.Parameter.span((float)-1.2f, (float)-1.05f);
    private final Climate.Parameter deepOceanContinentalness = Climate.Parameter.span((float)-1.05f, (float)-0.455f);
    private final Climate.Parameter oceanContinentalness = Climate.Parameter.span((float)-0.455f, (float)-0.19f);
    private final Climate.Parameter coastContinentalness = Climate.Parameter.span((float)-0.19f, (float)-0.11f);
    private final Climate.Parameter inlandContinentalness = Climate.Parameter.span((float)-0.11f, (float)0.55f);
    private final Climate.Parameter nearInlandContinentalness = Climate.Parameter.span((float)-0.11f, (float)0.03f);
    private final Climate.Parameter midInlandContinentalness = Climate.Parameter.span((float)0.03f, (float)0.3f);
    private final Climate.Parameter farInlandContinentalness = Climate.Parameter.span((float)0.3f, (float)1.0f);
    private final ResourceKey<Biome>[][] OCEANS = new ResourceKey[][]{{Biomes.DEEP_FROZEN_OCEAN, Biomes.DEEP_COLD_OCEAN, Biomes.DEEP_OCEAN, Biomes.DEEP_LUKEWARM_OCEAN, Biomes.WARM_OCEAN}, {Biomes.FROZEN_OCEAN, Biomes.COLD_OCEAN, Biomes.OCEAN, Biomes.LUKEWARM_OCEAN, Biomes.WARM_OCEAN}};
    private final ResourceKey<Biome>[][] MIDDLE_BIOMES = new ResourceKey[][]{{Biomes.SNOWY_PLAINS, Biomes.SNOWY_PLAINS, Biomes.SNOWY_PLAINS, Biomes.SNOWY_TAIGA, Biomes.TAIGA}, {Biomes.PLAINS, Biomes.PLAINS, Biomes.FOREST, Biomes.TAIGA, Biomes.OLD_GROWTH_SPRUCE_TAIGA}, {Biomes.FLOWER_FOREST, Biomes.PLAINS, Biomes.FOREST, Biomes.BIRCH_FOREST, Biomes.DARK_FOREST}, {Biomes.SAVANNA, Biomes.SAVANNA, Biomes.FOREST, Biomes.JUNGLE, Biomes.JUNGLE}, {Biomes.DESERT, Biomes.DESERT, Biomes.DESERT, Biomes.DESERT, Biomes.DESERT}};
    private final ResourceKey<Biome>[][] MIDDLE_BIOMES_VARIANT = new ResourceKey[][]{{Biomes.ICE_SPIKES, null, Biomes.SNOWY_TAIGA, null, null}, {null, null, null, null, Biomes.OLD_GROWTH_PINE_TAIGA}, {Biomes.SUNFLOWER_PLAINS, null, null, Biomes.OLD_GROWTH_BIRCH_FOREST, null}, {null, null, Biomes.PLAINS, Biomes.SPARSE_JUNGLE, Biomes.BAMBOO_JUNGLE}, {null, null, null, null, null}};
    private final ResourceKey<Biome>[][] PLATEAU_BIOMES = new ResourceKey[][]{{Biomes.SNOWY_PLAINS, Biomes.SNOWY_PLAINS, Biomes.SNOWY_PLAINS, Biomes.SNOWY_TAIGA, Biomes.SNOWY_TAIGA}, {Biomes.MEADOW, Biomes.MEADOW, Biomes.FOREST, Biomes.TAIGA, Biomes.OLD_GROWTH_SPRUCE_TAIGA}, {Biomes.MEADOW, Biomes.MEADOW, Biomes.MEADOW, Biomes.MEADOW, Biomes.PALE_GARDEN}, {Biomes.SAVANNA_PLATEAU, Biomes.SAVANNA_PLATEAU, Biomes.FOREST, Biomes.FOREST, Biomes.JUNGLE}, {Biomes.BADLANDS, Biomes.BADLANDS, Biomes.BADLANDS, Biomes.WOODED_BADLANDS, Biomes.WOODED_BADLANDS}};
    private final ResourceKey<Biome>[][] PLATEAU_BIOMES_VARIANT = new ResourceKey[][]{{Biomes.ICE_SPIKES, null, null, null, null}, {Biomes.CHERRY_GROVE, null, Biomes.MEADOW, Biomes.MEADOW, Biomes.OLD_GROWTH_PINE_TAIGA}, {Biomes.CHERRY_GROVE, Biomes.CHERRY_GROVE, Biomes.FOREST, Biomes.BIRCH_FOREST, null}, {null, null, null, null, null}, {Biomes.ERODED_BADLANDS, Biomes.ERODED_BADLANDS, null, null, null}};
    private final ResourceKey<Biome>[][] SHATTERED_BIOMES = new ResourceKey[][]{{Biomes.WINDSWEPT_GRAVELLY_HILLS, Biomes.WINDSWEPT_GRAVELLY_HILLS, Biomes.WINDSWEPT_HILLS, Biomes.WINDSWEPT_FOREST, Biomes.WINDSWEPT_FOREST}, {Biomes.WINDSWEPT_GRAVELLY_HILLS, Biomes.WINDSWEPT_GRAVELLY_HILLS, Biomes.WINDSWEPT_HILLS, Biomes.WINDSWEPT_FOREST, Biomes.WINDSWEPT_FOREST}, {Biomes.WINDSWEPT_HILLS, Biomes.WINDSWEPT_HILLS, Biomes.WINDSWEPT_HILLS, Biomes.WINDSWEPT_FOREST, Biomes.WINDSWEPT_FOREST}, {null, null, null, null, null}, {null, null, null, null, null}};

    public List<Climate.ParameterPoint> spawnTarget() {
        Climate.Parameter parameter = Climate.Parameter.point((float)0.0f);
        float f = 0.16f;
        return List.of(new Climate.ParameterPoint(this.FULL_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.inlandContinentalness, (Climate.Parameter)this.FULL_RANGE), this.FULL_RANGE, parameter, Climate.Parameter.span((float)-1.0f, (float)-0.16f), 0L), new Climate.ParameterPoint(this.FULL_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.inlandContinentalness, (Climate.Parameter)this.FULL_RANGE), this.FULL_RANGE, parameter, Climate.Parameter.span((float)0.16f, (float)1.0f), 0L));
    }

    protected void addBiomes(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> key) {
        if (SharedConstants.debugGenerateSquareTerrainWithoutNoise) {
            this.addDebugBiomes(key);
        } else {
            this.addOffCoastBiomes(key);
            this.addInlandBiomes(key);
            this.addUndergroundBiomes(key);
        }
    }

    /*
     * WARNING - void declaration
     */
    private void addDebugBiomes(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> key) {
        CubicSpline cubicSpline;
        HolderLookup.Provider provider = new RegistrySetBuilder().add(Registries.DENSITY_FUNCTION, NoiseRouterData::bootstrap).add(Registries.NOISE, NoiseData::bootstrap).build((RegistryAccess)RegistryAccess.fromRegistryOfRegistries((Registry)BuiltInRegistries.REGISTRY));
        HolderLookup.RegistryLookup holderGetter = provider.lookupOrThrow(Registries.DENSITY_FUNCTION);
        DensityFunctions.Spline.Coordinate coordinate = new DensityFunctions.Spline.Coordinate((Holder)holderGetter.getOrThrow(NoiseRouterData.CONTINENTS));
        DensityFunctions.Spline.Coordinate coordinate1 = new DensityFunctions.Spline.Coordinate((Holder)holderGetter.getOrThrow(NoiseRouterData.EROSION));
        DensityFunctions.Spline.Coordinate coordinate2 = new DensityFunctions.Spline.Coordinate((Holder)holderGetter.getOrThrow(NoiseRouterData.RIDGES_FOLDED));
        key.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)Climate.Parameter.point((float)0.0f), (Climate.Parameter)this.FULL_RANGE, (float)0.01f), (Object)Biomes.PLAINS));
        CubicSpline object = TerrainProvider.buildErosionOffsetSpline((BoundedFloatFunction)coordinate1, (BoundedFloatFunction)coordinate2, (float)-0.15f, (float)0.0f, (float)0.0f, (float)0.1f, (float)0.0f, (float)-0.03f, (boolean)false, (boolean)false, (BoundedFloatFunction)BoundedFloatFunction.IDENTITY);
        if (object instanceof CubicSpline.Multipoint) {
            CubicSpline.Multipoint multipoint = (CubicSpline.Multipoint)object;
            ResourceKey resourceKey = Biomes.DESERT;
            float[] fArray = multipoint.locations();
            int n = fArray.length;
            for (int i = 0; i < n; ++i) {
                void var8_9;
                float f = fArray[i];
                key.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)Climate.Parameter.point((float)f), (Climate.Parameter)Climate.Parameter.point((float)0.0f), (Climate.Parameter)this.FULL_RANGE, (float)0.0f), (Object)var8_9));
                ResourceKey resourceKey2 = var8_9 == Biomes.DESERT ? Biomes.BADLANDS : Biomes.DESERT;
            }
        }
        if ((cubicSpline = TerrainProvider.overworldOffset((BoundedFloatFunction)coordinate, (BoundedFloatFunction)coordinate1, (BoundedFloatFunction)coordinate2, (boolean)false)) instanceof CubicSpline.Multipoint) {
            CubicSpline.Multipoint multipoint1 = (CubicSpline.Multipoint)cubicSpline;
            for (float f : multipoint1.locations()) {
                key.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)Climate.Parameter.point((float)f), (Climate.Parameter)this.FULL_RANGE, (Climate.Parameter)Climate.Parameter.point((float)0.0f), (Climate.Parameter)this.FULL_RANGE, (float)0.0f), (Object)Biomes.SNOWY_TAIGA));
            }
        }
    }

    private void addOffCoastBiomes(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer) {
        this.addSurfaceBiome(consumer, this.FULL_RANGE, this.FULL_RANGE, this.mushroomFieldsContinentalness, this.FULL_RANGE, this.FULL_RANGE, 0.0f, (ResourceKey<Biome>)Biomes.MUSHROOM_FIELDS);
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            this.addSurfaceBiome(consumer, parameter, this.FULL_RANGE, this.deepOceanContinentalness, this.FULL_RANGE, this.FULL_RANGE, 0.0f, this.OCEANS[0][i]);
            this.addSurfaceBiome(consumer, parameter, this.FULL_RANGE, this.oceanContinentalness, this.FULL_RANGE, this.FULL_RANGE, 0.0f, this.OCEANS[1][i]);
        }
    }

    private void addInlandBiomes(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer) {
        this.addMidSlice(consumer, Climate.Parameter.span((float)-1.0f, (float)-0.93333334f));
        this.addHighSlice(consumer, Climate.Parameter.span((float)-0.93333334f, (float)-0.7666667f));
        this.addPeaks(consumer, Climate.Parameter.span((float)-0.7666667f, (float)-0.56666666f));
        this.addHighSlice(consumer, Climate.Parameter.span((float)-0.56666666f, (float)-0.4f));
        this.addMidSlice(consumer, Climate.Parameter.span((float)-0.4f, (float)-0.26666668f));
        this.addLowSlice(consumer, Climate.Parameter.span((float)-0.26666668f, (float)-0.05f));
        this.addValleys(consumer, Climate.Parameter.span((float)-0.05f, (float)0.05f));
        this.addLowSlice(consumer, Climate.Parameter.span((float)0.05f, (float)0.26666668f));
        this.addMidSlice(consumer, Climate.Parameter.span((float)0.26666668f, (float)0.4f));
        this.addHighSlice(consumer, Climate.Parameter.span((float)0.4f, (float)0.56666666f));
        this.addPeaks(consumer, Climate.Parameter.span((float)0.56666666f, (float)0.7666667f));
        this.addHighSlice(consumer, Climate.Parameter.span((float)0.7666667f, (float)0.93333334f));
        this.addMidSlice(consumer, Climate.Parameter.span((float)0.93333334f, (float)1.0f));
    }

    private void addPeaks(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter param) {
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            for (int i1 = 0; i1 < this.humidities.length; ++i1) {
                Climate.Parameter parameter1 = this.humidities[i1];
                ResourceKey<Biome> resourceKey = this.pickMiddleBiome(i, i1, param);
                ResourceKey<Biome> resourceKey1 = this.pickMiddleBiomeOrBadlandsIfHot(i, i1, param);
                ResourceKey<Biome> resourceKey2 = this.pickMiddleBiomeOrBadlandsIfHotOrSlopeIfCold(i, i1, param);
                ResourceKey<Biome> resourceKey3 = this.pickPlateauBiome(i, i1, param);
                ResourceKey<Biome> resourceKey4 = this.pickShatteredBiome(i, i1, param);
                ResourceKey<Biome> resourceKey5 = this.maybePickWindsweptSavannaBiome(i, i1, param, resourceKey4);
                ResourceKey<Biome> resourceKey6 = this.pickPeakBiome(i, i1, param);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[0], param, 0.0f, resourceKey6);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), this.erosions[1], param, 0.0f, resourceKey2);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[1], param, 0.0f, resourceKey6);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[3]), param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[2], param, 0.0f, resourceKey3);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.midInlandContinentalness, this.erosions[3], param, 0.0f, resourceKey1);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.farInlandContinentalness, this.erosions[3], param, 0.0f, resourceKey3);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[4], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey5);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey4);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, resourceKey);
            }
        }
    }

    private void addHighSlice(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter param) {
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            for (int i1 = 0; i1 < this.humidities.length; ++i1) {
                Climate.Parameter parameter1 = this.humidities[i1];
                ResourceKey<Biome> resourceKey = this.pickMiddleBiome(i, i1, param);
                ResourceKey<Biome> resourceKey1 = this.pickMiddleBiomeOrBadlandsIfHot(i, i1, param);
                ResourceKey<Biome> resourceKey2 = this.pickMiddleBiomeOrBadlandsIfHotOrSlopeIfCold(i, i1, param);
                ResourceKey<Biome> resourceKey3 = this.pickPlateauBiome(i, i1, param);
                ResourceKey<Biome> resourceKey4 = this.pickShatteredBiome(i, i1, param);
                ResourceKey<Biome> resourceKey5 = this.maybePickWindsweptSavannaBiome(i, i1, param, resourceKey);
                ResourceKey<Biome> resourceKey6 = this.pickSlopeBiome(i, i1, param);
                ResourceKey<Biome> resourceKey7 = this.pickPeakBiome(i, i1, param);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, this.erosions[0], param, 0.0f, resourceKey6);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[0], param, 0.0f, resourceKey7);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, this.erosions[1], param, 0.0f, resourceKey2);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[1], param, 0.0f, resourceKey6);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[3]), param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[2], param, 0.0f, resourceKey3);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.midInlandContinentalness, this.erosions[3], param, 0.0f, resourceKey1);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.farInlandContinentalness, this.erosions[3], param, 0.0f, resourceKey3);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[4], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey5);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey4);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, resourceKey);
            }
        }
    }

    private void addMidSlice(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter param) {
        this.addSurfaceBiome(consumer, this.FULL_RANGE, this.FULL_RANGE, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[2]), param, 0.0f, (ResourceKey<Biome>)Biomes.STONY_SHORE);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[1], (Climate.Parameter)this.temperatures[2]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.SWAMP);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[3], (Climate.Parameter)this.temperatures[4]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.MANGROVE_SWAMP);
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            for (int i1 = 0; i1 < this.humidities.length; ++i1) {
                Climate.Parameter parameter1 = this.humidities[i1];
                ResourceKey<Biome> resourceKey = this.pickMiddleBiome(i, i1, param);
                ResourceKey<Biome> resourceKey1 = this.pickMiddleBiomeOrBadlandsIfHot(i, i1, param);
                ResourceKey<Biome> resourceKey2 = this.pickMiddleBiomeOrBadlandsIfHotOrSlopeIfCold(i, i1, param);
                ResourceKey<Biome> resourceKey3 = this.pickShatteredBiome(i, i1, param);
                ResourceKey<Biome> resourceKey4 = this.pickPlateauBiome(i, i1, param);
                ResourceKey<Biome> resourceKey5 = this.pickBeachBiome(i, i1);
                ResourceKey<Biome> resourceKey6 = this.maybePickWindsweptSavannaBiome(i, i1, param, resourceKey);
                ResourceKey<Biome> resourceKey7 = this.pickShatteredCoastBiome(i, i1, param);
                ResourceKey<Biome> resourceKey8 = this.pickSlopeBiome(i, i1, param);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[0], param, 0.0f, resourceKey8);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.midInlandContinentalness), this.erosions[1], param, 0.0f, resourceKey2);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.farInlandContinentalness, this.erosions[1], param, 0.0f, i == 0 ? resourceKey8 : resourceKey4);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, this.erosions[2], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.midInlandContinentalness, this.erosions[2], param, 0.0f, resourceKey1);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.farInlandContinentalness, this.erosions[2], param, 0.0f, resourceKey4);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.nearInlandContinentalness), this.erosions[3], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[3], param, 0.0f, resourceKey1);
                if (param.max() < 0L) {
                    this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[4], param, 0.0f, resourceKey5);
                    this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[4], param, 0.0f, resourceKey);
                } else {
                    this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[4], param, 0.0f, resourceKey);
                }
                this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[5], param, 0.0f, resourceKey7);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, this.erosions[5], param, 0.0f, resourceKey6);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey3);
                if (param.max() < 0L) {
                    this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[6], param, 0.0f, resourceKey5);
                } else {
                    this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[6], param, 0.0f, resourceKey);
                }
                if (i != 0) continue;
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, resourceKey);
            }
        }
    }

    private void addLowSlice(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter param) {
        this.addSurfaceBiome(consumer, this.FULL_RANGE, this.FULL_RANGE, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[2]), param, 0.0f, (ResourceKey<Biome>)Biomes.STONY_SHORE);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[1], (Climate.Parameter)this.temperatures[2]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.SWAMP);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[3], (Climate.Parameter)this.temperatures[4]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.MANGROVE_SWAMP);
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            for (int i1 = 0; i1 < this.humidities.length; ++i1) {
                Climate.Parameter parameter1 = this.humidities[i1];
                ResourceKey<Biome> resourceKey = this.pickMiddleBiome(i, i1, param);
                ResourceKey<Biome> resourceKey1 = this.pickMiddleBiomeOrBadlandsIfHot(i, i1, param);
                ResourceKey<Biome> resourceKey2 = this.pickMiddleBiomeOrBadlandsIfHotOrSlopeIfCold(i, i1, param);
                ResourceKey<Biome> resourceKey3 = this.pickBeachBiome(i, i1);
                ResourceKey<Biome> resourceKey4 = this.maybePickWindsweptSavannaBiome(i, i1, param, resourceKey);
                ResourceKey<Biome> resourceKey5 = this.pickShatteredCoastBiome(i, i1, param);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, resourceKey1);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, resourceKey2);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[3]), param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[3]), param, 0.0f, resourceKey1);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[3], (Climate.Parameter)this.erosions[4]), param, 0.0f, resourceKey3);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[4], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[5], param, 0.0f, resourceKey5);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.nearInlandContinentalness, this.erosions[5], param, 0.0f, resourceKey4);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[5], param, 0.0f, resourceKey);
                this.addSurfaceBiome(consumer, parameter, parameter1, this.coastContinentalness, this.erosions[6], param, 0.0f, resourceKey3);
                if (i != 0) continue;
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.nearInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, resourceKey);
            }
        }
    }

    private void addValleys(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter param) {
        this.addSurfaceBiome(consumer, this.FROZEN_RANGE, this.FULL_RANGE, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, (ResourceKey<Biome>)(param.max() < 0L ? Biomes.STONY_SHORE : Biomes.FROZEN_RIVER));
        this.addSurfaceBiome(consumer, this.UNFROZEN_RANGE, this.FULL_RANGE, this.coastContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, (ResourceKey<Biome>)(param.max() < 0L ? Biomes.STONY_SHORE : Biomes.RIVER));
        this.addSurfaceBiome(consumer, this.FROZEN_RANGE, this.FULL_RANGE, this.nearInlandContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, (ResourceKey<Biome>)Biomes.FROZEN_RIVER);
        this.addSurfaceBiome(consumer, this.UNFROZEN_RANGE, this.FULL_RANGE, this.nearInlandContinentalness, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, (ResourceKey<Biome>)Biomes.RIVER);
        this.addSurfaceBiome(consumer, this.FROZEN_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[5]), param, 0.0f, (ResourceKey<Biome>)Biomes.FROZEN_RIVER);
        this.addSurfaceBiome(consumer, this.UNFROZEN_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.coastContinentalness, (Climate.Parameter)this.farInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[2], (Climate.Parameter)this.erosions[5]), param, 0.0f, (ResourceKey<Biome>)Biomes.RIVER);
        this.addSurfaceBiome(consumer, this.FROZEN_RANGE, this.FULL_RANGE, this.coastContinentalness, this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.FROZEN_RIVER);
        this.addSurfaceBiome(consumer, this.UNFROZEN_RANGE, this.FULL_RANGE, this.coastContinentalness, this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.RIVER);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[1], (Climate.Parameter)this.temperatures[2]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.inlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.SWAMP);
        this.addSurfaceBiome(consumer, Climate.Parameter.span((Climate.Parameter)this.temperatures[3], (Climate.Parameter)this.temperatures[4]), this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.inlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.MANGROVE_SWAMP);
        this.addSurfaceBiome(consumer, this.FROZEN_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.inlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), this.erosions[6], param, 0.0f, (ResourceKey<Biome>)Biomes.FROZEN_RIVER);
        for (int i = 0; i < this.temperatures.length; ++i) {
            Climate.Parameter parameter = this.temperatures[i];
            for (int i1 = 0; i1 < this.humidities.length; ++i1) {
                Climate.Parameter parameter1 = this.humidities[i1];
                ResourceKey<Biome> resourceKey = this.pickMiddleBiomeOrBadlandsIfHot(i, i1, param);
                this.addSurfaceBiome(consumer, parameter, parameter1, Climate.Parameter.span((Climate.Parameter)this.midInlandContinentalness, (Climate.Parameter)this.farInlandContinentalness), Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), param, 0.0f, resourceKey);
            }
        }
    }

    private void addUndergroundBiomes(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consume) {
        this.addUndergroundBiome(consume, this.FULL_RANGE, this.FULL_RANGE, Climate.Parameter.span((float)0.8f, (float)1.0f), this.FULL_RANGE, this.FULL_RANGE, 0.0f, (ResourceKey<Biome>)Biomes.DRIPSTONE_CAVES);
        this.addUndergroundBiome(consume, this.FULL_RANGE, Climate.Parameter.span((float)0.7f, (float)1.0f), this.FULL_RANGE, this.FULL_RANGE, this.FULL_RANGE, 0.0f, (ResourceKey<Biome>)Biomes.LUSH_CAVES);
        this.addBottomBiome(consume, this.FULL_RANGE, this.FULL_RANGE, this.FULL_RANGE, Climate.Parameter.span((Climate.Parameter)this.erosions[0], (Climate.Parameter)this.erosions[1]), this.FULL_RANGE, 0.0f, (ResourceKey<Biome>)Biomes.DEEP_DARK);
    }

    private ResourceKey<Biome> pickMiddleBiome(int temperature, int humidity, Climate.Parameter param) {
        if (param.max() < 0L) {
            return this.MIDDLE_BIOMES[temperature][humidity];
        }
        ResourceKey<Biome> resourceKey = this.MIDDLE_BIOMES_VARIANT[temperature][humidity];
        return resourceKey == null ? this.MIDDLE_BIOMES[temperature][humidity] : resourceKey;
    }

    private ResourceKey<Biome> pickMiddleBiomeOrBadlandsIfHot(int temperature, int humidity, Climate.Parameter param) {
        return temperature == 4 ? this.pickBadlandsBiome(humidity, param) : this.pickMiddleBiome(temperature, humidity, param);
    }

    private ResourceKey<Biome> pickMiddleBiomeOrBadlandsIfHotOrSlopeIfCold(int temperature, int humidity, Climate.Parameter param) {
        return temperature == 0 ? this.pickSlopeBiome(temperature, humidity, param) : this.pickMiddleBiomeOrBadlandsIfHot(temperature, humidity, param);
    }

    private ResourceKey<Biome> maybePickWindsweptSavannaBiome(int temperature, int humidity, Climate.Parameter param, ResourceKey<Biome> key) {
        return temperature > 1 && humidity < 4 && param.max() >= 0L ? Biomes.WINDSWEPT_SAVANNA : key;
    }

    private ResourceKey<Biome> pickShatteredCoastBiome(int temperature, int humidity, Climate.Parameter param) {
        ResourceKey<Biome> resourceKey = param.max() >= 0L ? this.pickMiddleBiome(temperature, humidity, param) : this.pickBeachBiome(temperature, humidity);
        return this.maybePickWindsweptSavannaBiome(temperature, humidity, param, resourceKey);
    }

    private ResourceKey<Biome> pickBeachBiome(int temperature, int humidity) {
        if (temperature == 0) {
            return Biomes.SNOWY_BEACH;
        }
        return temperature == 4 ? Biomes.DESERT : Biomes.BEACH;
    }

    private ResourceKey<Biome> pickBadlandsBiome(int humidity, Climate.Parameter param) {
        if (humidity < 2) {
            return param.max() < 0L ? Biomes.BADLANDS : Biomes.ERODED_BADLANDS;
        }
        return humidity < 3 ? Biomes.BADLANDS : Biomes.WOODED_BADLANDS;
    }

    private ResourceKey<Biome> pickPlateauBiome(int temperature, int humidity, Climate.Parameter param) {
        ResourceKey<Biome> resourceKey;
        if (param.max() >= 0L && (resourceKey = this.PLATEAU_BIOMES_VARIANT[temperature][humidity]) != null) {
            return resourceKey;
        }
        return this.PLATEAU_BIOMES[temperature][humidity];
    }

    private ResourceKey<Biome> pickPeakBiome(int temperature, int humidity, Climate.Parameter param) {
        if (temperature <= 2) {
            return param.max() < 0L ? Biomes.JAGGED_PEAKS : Biomes.FROZEN_PEAKS;
        }
        return temperature == 3 ? Biomes.STONY_PEAKS : this.pickBadlandsBiome(humidity, param);
    }

    private ResourceKey<Biome> pickSlopeBiome(int temperature, int humidity, Climate.Parameter param) {
        if (temperature >= 3) {
            return this.pickPlateauBiome(temperature, humidity, param);
        }
        return humidity <= 1 ? Biomes.SNOWY_SLOPES : Biomes.GROVE;
    }

    private ResourceKey<Biome> pickShatteredBiome(int temperature, int humidity, Climate.Parameter param) {
        ResourceKey<Biome> resourceKey = this.SHATTERED_BIOMES[temperature][humidity];
        return resourceKey == null ? this.pickMiddleBiome(temperature, humidity, param) : resourceKey;
    }

    private void addSurfaceBiome(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter temperature, Climate.Parameter humidity, Climate.Parameter continentalness, Climate.Parameter erosion, Climate.Parameter depth, float weirdness, ResourceKey<Biome> key) {
        consumer.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)temperature, (Climate.Parameter)humidity, (Climate.Parameter)continentalness, (Climate.Parameter)erosion, (Climate.Parameter)Climate.Parameter.point((float)0.0f), (Climate.Parameter)depth, (float)weirdness), key));
        consumer.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)temperature, (Climate.Parameter)humidity, (Climate.Parameter)continentalness, (Climate.Parameter)erosion, (Climate.Parameter)Climate.Parameter.point((float)1.0f), (Climate.Parameter)depth, (float)weirdness), key));
    }

    private void addUndergroundBiome(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter temperature, Climate.Parameter humidity, Climate.Parameter continentalness, Climate.Parameter erosion, Climate.Parameter depth, float weirdness, ResourceKey<Biome> key) {
        consumer.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)temperature, (Climate.Parameter)humidity, (Climate.Parameter)continentalness, (Climate.Parameter)erosion, (Climate.Parameter)Climate.Parameter.span((float)0.2f, (float)0.9f), (Climate.Parameter)depth, (float)weirdness), key));
    }

    private void addBottomBiome(Consumer<Pair<Climate.ParameterPoint, ResourceKey<Biome>>> consumer, Climate.Parameter temperature, Climate.Parameter humidity, Climate.Parameter continentalness, Climate.Parameter erosion, Climate.Parameter depth, float weirdness, ResourceKey<Biome> key) {
        consumer.accept((Pair<Climate.ParameterPoint, ResourceKey<Biome>>)Pair.of((Object)Climate.parameters((Climate.Parameter)temperature, (Climate.Parameter)humidity, (Climate.Parameter)continentalness, (Climate.Parameter)erosion, (Climate.Parameter)Climate.Parameter.point((float)1.1f), (Climate.Parameter)depth, (float)weirdness), key));
    }

    public static boolean isDeepDarkRegion(DensityFunction erosionFunction, DensityFunction depthFunction, DensityFunction.FunctionContext functionContext) {
        return erosionFunction.compute(functionContext) < (double)-0.225f && depthFunction.compute(functionContext) > (double)0.9f;
    }

    public static String getDebugStringForPeaksAndValleys(double peaksAndValleysData) {
        if (peaksAndValleysData < (double)NoiseRouterData.peaksAndValleys((float)0.05f)) {
            return "Valley";
        }
        if (peaksAndValleysData < (double)NoiseRouterData.peaksAndValleys((float)0.26666668f)) {
            return "Low";
        }
        if (peaksAndValleysData < (double)NoiseRouterData.peaksAndValleys((float)0.4f)) {
            return "Mid";
        }
        return peaksAndValleysData < (double)NoiseRouterData.peaksAndValleys((float)0.56666666f) ? "High" : "Peak";
    }

    public String getDebugStringForContinentalness(double continentalness) {
        double d = Climate.quantizeCoord((float)((float)continentalness));
        if (d < (double)this.mushroomFieldsContinentalness.max()) {
            return "Mushroom fields";
        }
        if (d < (double)this.deepOceanContinentalness.max()) {
            return "Deep ocean";
        }
        if (d < (double)this.oceanContinentalness.max()) {
            return "Ocean";
        }
        if (d < (double)this.coastContinentalness.max()) {
            return "Coast";
        }
        if (d < (double)this.nearInlandContinentalness.max()) {
            return "Near inland";
        }
        return d < (double)this.midInlandContinentalness.max() ? "Mid inland" : "Far inland";
    }

    public String getDebugStringForErosion(double erosion) {
        return OverworldBiomeBuilder.getDebugStringForNoiseValue(erosion, this.erosions);
    }

    public String getDebugStringForTemperature(double temperature) {
        return OverworldBiomeBuilder.getDebugStringForNoiseValue(temperature, this.temperatures);
    }

    public String getDebugStringForHumidity(double humidity) {
        return OverworldBiomeBuilder.getDebugStringForNoiseValue(humidity, this.humidities);
    }

    private static String getDebugStringForNoiseValue(double depth, Climate.Parameter[] values) {
        double d = Climate.quantizeCoord((float)((float)depth));
        for (int i = 0; i < values.length; ++i) {
            if (!(d < (double)values[i].max())) continue;
            return "" + i;
        }
        return "?";
    }

    @VisibleForDebug
    public Climate.Parameter[] getTemperatureThresholds() {
        return this.temperatures;
    }

    @VisibleForDebug
    public Climate.Parameter[] getHumidityThresholds() {
        return this.humidities;
    }

    @VisibleForDebug
    public Climate.Parameter[] getErosionThresholds() {
        return this.erosions;
    }

    @VisibleForDebug
    public Climate.Parameter[] getContinentalnessThresholds() {
        return new Climate.Parameter[]{this.mushroomFieldsContinentalness, this.deepOceanContinentalness, this.oceanContinentalness, this.coastContinentalness, this.nearInlandContinentalness, this.midInlandContinentalness, this.farInlandContinentalness};
    }

    @VisibleForDebug
    public Climate.Parameter[] getPeaksAndValleysThresholds() {
        return new Climate.Parameter[]{Climate.Parameter.span((float)-2.0f, (float)NoiseRouterData.peaksAndValleys((float)0.05f)), Climate.Parameter.span((float)NoiseRouterData.peaksAndValleys((float)0.05f), (float)NoiseRouterData.peaksAndValleys((float)0.26666668f)), Climate.Parameter.span((float)NoiseRouterData.peaksAndValleys((float)0.26666668f), (float)NoiseRouterData.peaksAndValleys((float)0.4f)), Climate.Parameter.span((float)NoiseRouterData.peaksAndValleys((float)0.4f), (float)NoiseRouterData.peaksAndValleys((float)0.56666666f)), Climate.Parameter.span((float)NoiseRouterData.peaksAndValleys((float)0.56666666f), (float)2.0f)};
    }

    @VisibleForDebug
    public Climate.Parameter[] getWeirdnessThresholds() {
        return new Climate.Parameter[]{Climate.Parameter.span((float)-2.0f, (float)0.0f), Climate.Parameter.span((float)0.0f, (float)2.0f)};
    }
}
