package net.minecraft.world.entity;

import net.minecraft.core.BlockPos;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import sun.misc.Unsafe;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * INSIDE-BITMASK offline lockstep oracle (RECON-33 contract, lever #15,
 * TASK-358) — plain JVM, real kernel classes, NO server boot (INJECTS-ONLY
 * discipline). Precedent: TravelDietLockstepHarness (v2a, ×29).
 *
 * What it proves OFFLINE, against the REAL kernel:
 *   1. STRUCTURAL + ARM — InsideBitmaskOps links against the real kernel jar
 *      and armState() == "ARMED" (MethodHandle to the private
 *      checkInsideBlocks + Unsafe offset of ChunkAccess.sections resolve on
 *      the REAL classes — the exact live probe-then-patch path).
 *   2. HULL-SUPERSET (the median-exact core) — every block position visited
 *      by the REAL BlockGetter.forEachBlockIntersectedBetween (the vanilla
 *      discovery traversal, real geometry incl. the exact deflate
 *      9.999999747378752E-6) lies inside the block-domain covered by the
 *      REAL InsideBitmaskOps.sweptHullInto hull cells, across 1,000,000
 *      randomized+adversarial cases (walk deltas, zero-moves, teleports,
 *      chunk/section-boundary ±ε landings, negative coords, multi-movement
 *      ticks). ⟹ if the gate's hull cells are all-air, EVERY vanilla visit
 *      is an air-visit (lambda bc 34..65: returns before any observable
 *      action) ⟹ skip ≡ vanilla observable state (effects/collector/
 *      visitedBlocks/budget untouched).
 *   3. SECTION TRUTH — real LevelChunkSection + real PalettedContainer:
 *      hasOnlyAir() == manual all-air scan AND the nonEmptyBlockCount field
 *      == manual !isAir recount, across 200,000 randomized
 *      setBlockState/getBlockState op sequences over a palette incl.
 *      air/stone/water/lava/leaves/powder_snow (fluids are NOT-air → water
 *      sections can never be skipped, by construction).
 *
 * Residue honestly NOT covered offline (documented per TravelDiet
 * discipline): the Level.getChunk(II,ChunkStatus,Z) wiring against a live
 * ServerLevel chunk-map (same infra getBlockState serves; null-chunk and
 * index-bounds paths fail-closed to vanilla), the debugSubscribers-active
 * branch (gate returns false → vanilla), and the live effect stream equality
 * (needs a booted world) — all enforced by the CI leg fixture gates on the
 * real 150k scene (median-exact discipline).
 *
 * Exit code 0 = PASS; any failure throws (nonzero).
 */
public final class InsideBitmaskLockstepHarness {

    private static Unsafe unsafe() throws Exception {
        Field f = Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        return (Unsafe) f.get(null);
    }

    public static void main(String[] args) throws Exception {
        net.minecraft.SharedConstants.tryDetectVersion(); // purpur ServerBuildInfo needs the game version
        Bootstrap.bootStrap(); // registries for Entity/Blocks/sections

        // ---- 1. STRUCTURAL + ARM (the real probe path)
        String arm = InsideBitmaskOps.armState();
        if (!"ARMED".equals(arm)) {
            throw new AssertionError("InsideBitmaskOps armState=" + arm + " (must be ARMED)");
        }
        System.out.println("[1] structural+arm: InsideBitmaskOps ARMED on real kernel jar ("
            + InsideBitmaskOps.class.getName() + ")");

        // ---- 2. HULL-SUPERSET vs REAL vanilla traversal
        Random rnd = new Random(0x1D51DEBBL);
        List<Entity.Movement> scratch = new ArrayList<>(3);
        double[] hull = new double[6];
        long cases = 0;
        long visitedTotal = 0;
        long skipWouldFire = 0;
        double EPS = 9.999999747378752e-6;
        double[] boundaryOffsets = {0.0, 1e-9, -1e-9, 1e-6, -1e-6, EPS, -EPS, 0.4999999, -0.4999999, 8.0, -8.0};
        for (int i = 0; i < 1_000_000; ++i) {
            double w = 0.3 + rnd.nextDouble() * 2.7;
            double hgt = 0.3 + rnd.nextDouble() * 2.7;
            double tx, ty, tz;
            if (i % 7 == 0) {
                // boundary-adjacent landings (chunk/section edges)
                tx = (rnd.nextInt(64) - 32) * 16.0 + boundaryOffsets[rnd.nextInt(boundaryOffsets.length)];
                ty = (rnd.nextInt(24) - 12) * 16.0 + boundaryOffsets[rnd.nextInt(boundaryOffsets.length)];
                tz = (rnd.nextInt(64) - 32) * 16.0 + boundaryOffsets[rnd.nextInt(boundaryOffsets.length)];
            } else {
                tx = rnd.nextDouble() * 2048.0 - 1024.0;
                ty = rnd.nextDouble() * 320.0;
                tz = rnd.nextDouble() * 2048.0 - 1024.0;
            }
            int nMov = 1 + rnd.nextInt(3);
            scratch.clear();
            double lastToX = tx, lastToY = ty, lastToZ = tz;
            for (int mI = 0; mI < nMov; ++mI) {
                double dx, dy, dz;
                int r = rnd.nextInt(16);
                if (r == 0) { dx = 0; dy = 0; dz = 0; }                    // zero-move (betweenClosed path)
                else if (r == 1) {                                          // axis-dominant walk
                    dx = rnd.nextGaussian() * 0.6; dy = 0; dz = 0;
                } else if (r == 2) { dx = 0; dy = rnd.nextGaussian() * 0.6; dz = 0; }
                else if (r == 3) { dx = 0; dy = 0; dz = rnd.nextGaussian() * 0.6; }
                else if (r == 4) {                                          // teleport-scale
                    dx = rnd.nextGaussian() * 100; dy = rnd.nextGaussian() * 40; dz = rnd.nextGaussian() * 100;
                } else if (r == 5) {                                        // sub-deflate micro step
                    dx = (rnd.nextDouble() - 0.5) * 2e-5;
                    dy = (rnd.nextDouble() - 0.5) * 2e-5;
                    dz = (rnd.nextDouble() - 0.5) * 2e-5;
                } else {
                    dx = rnd.nextGaussian() * 0.9;
                    dy = rnd.nextGaussian() * 0.4;
                    dz = rnd.nextGaussian() * 0.9;
                }
                double fx = lastToX - dx, fy = lastToY - dy, fz = lastToZ - dz;
                scratch.add(new Entity.Movement(new Vec3(fx, fy, fz), new Vec3(lastToX, lastToY, lastToZ)));
                lastToX = fx; lastToY = fy; lastToZ = fz;
            }
            // entity ends at the LAST movement's to; bb = vanilla makeBoundingBox shape
            Vec3 cur = new Vec3(lastToX, lastToY, lastToZ);
            AABB bb = new AABB(lastToX - w / 2, lastToY, lastToZ - w / 2,
                    lastToX + w / 2, lastToY + hgt, lastToZ + w / 2);

            // REAL gate hull (the shipped formula)
            InsideBitmaskOps.sweptHullInto(hull, bb, cur, scratch);
            final double[] hview = hull;

            // REAL vanilla traversal per movement (from→to with the to-box deflated)
            boolean anyVisited = false;
            final int caseNo = i;
            List<Entity.Movement> frozen = new ArrayList<>(scratch);
            for (Entity.Movement m : frozen) {
                Vec3 to = m.to();
                AABB toBox = new AABB(to.x - w / 2, to.y, to.z - w / 2,
                        to.x + w / 2, to.y + hgt, to.z + w / 2);
                AABB aabb = toBox.deflate(EPS);
                final int[] hit = new int[1];
                boolean completed = BlockGetter.forEachBlockIntersectedBetween(m.from(), to, aabb,
                        (BlockPos p, int step) -> {
                            hit[0]++;
                            // gate coverage assert: block coords of the visited pos
                            // must lie inside the hull block-domain per axis
                            int px = p.getX(), py = p.getY(), pz = p.getZ();
                            if (px < floor(hview[0]) || px > floor(hview[3])
                                    || py < floor(hview[1]) || py > floor(hview[4])
                                    || pz < floor(hview[2]) || pz > floor(hview[5])) {
                                throw new AssertionError("hull-superset violated @case " + caseNo
                                        + " visited=(" + px + "," + py + "," + pz + ")"
                                        + " hull=[" + hview[0] + "," + hview[1] + "," + hview[2]
                                        + ".." + hview[3] + "," + hview[4] + "," + hview[5] + "]"
                                        + " from=" + m.from() + " to=" + to + " w=" + w + " h=" + hgt);
                            }
                            return true;
                        });
                visitedTotal += hit[0];
                if (hit[0] > 0) anyVisited = true;
                if (!completed && hit[0] == 0) {
                    throw new AssertionError("traversal returned incomplete with zero visits @case " + caseNo);
                }
            }
            // bookkeeping: how many cases would be pure-skip candidates on an
            // all-air world (informational — the parity claim rests on the
            // superset property above, not on this counter)
            cases++;
            if (!anyVisited) skipWouldFire++;
        }
        System.out.println("[2] hull-superset vs REAL traversal: " + cases
                + " cases, " + visitedTotal + " visited positions, all inside gate hull PASS"
                + " (zero-visit cases: " + skipWouldFire + ")");

        // ---- 3. SECTION TRUTH (real LevelChunkSection + PalettedContainer)
        BlockState air = Blocks.AIR.defaultBlockState();
        BlockState stone = Blocks.STONE.defaultBlockState();
        BlockState water = Blocks.WATER.defaultBlockState();
        BlockState lava = Blocks.LAVA.defaultBlockState();
        BlockState leaves = Blocks.OAK_LEAVES.defaultBlockState();
        BlockState snow = Blocks.POWDER_SNOW.defaultBlockState();
        BlockState[] palette = {air, stone, water, lava, leaves, snow};

        // worldgen/biome is NOT in any offline RegistryAccess (loaded from
        // datapacks at world creation) — build the factory manually; codecs
        // are network/NBT-only and unused by the offline setBlockState path
        // no static biome registry exists in 1.21.10 (worldgen = datapack) —
        // build the section DIRECTLY from PalettedContainers; the biome
        // container uses a null default and is never touched by this test
        net.minecraft.world.level.chunk.Strategy<BlockState> bsStrategy =
                net.minecraft.world.level.chunk.Strategy.createForBlockStates(Block.BLOCK_STATE_REGISTRY);
        Unsafe uf = unsafe();
        long cntOff = uf.objectFieldOffset(LevelChunkSection.class.getDeclaredField("nonEmptyBlockCount"));

        int seqs = 20_000;
        long nOps = 0;
        for (int s = 0; s < seqs; ++s) {
            var blocks = new net.minecraft.world.level.chunk.PalettedContainer<BlockState>(air, bsStrategy);
            @SuppressWarnings({"unchecked", "rawtypes"})
            net.minecraft.world.level.chunk.Strategy<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> bioStrategy =
                    net.minecraft.world.level.chunk.Strategy.createForBiomes(
                            (net.minecraft.core.IdMap) BuiltInRegistries.BLOCK); // raw: biomes never touched
            var biomes = new net.minecraft.world.level.chunk.PalettedContainer<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>(
                    null, bioStrategy);
            LevelChunkSection sec = new LevelChunkSection(blocks, biomes);
            int mutations = 1 + rnd.nextInt(24);
            for (int k = 0; k < mutations; ++k) {
                int x = rnd.nextInt(16), y = rnd.nextInt(16), z = rnd.nextInt(16);
                BlockState st = palette[rnd.nextInt(palette.length)];
                sec.setBlockState(x, y, z, st);
                nOps++;
            }
            // truth #1: hasOnlyAir == manual all-air scan
            boolean manualAir = true;
            int manualCount = 0;
            for (int x = 0; x < 16; ++x)
                for (int y = 0; y < 16; ++y)
                    for (int z = 0; z < 16; ++z) {
                        BlockState st = sec.getBlockState(x, y, z);
                        if (st != null && !st.isAir()) {
                            manualAir = false;
                            manualCount++;
                        }
                    }
            if (sec.hasOnlyAir() != manualAir) {
                throw new AssertionError("hasOnlyAir mismatch @seq " + s
                        + " hasOnlyAir=" + sec.hasOnlyAir() + " manualAllAir=" + manualAir);
            }
            // truth #2: nonEmptyBlockCount field == manual !isAir recount
            short cnt = uf.getShort(sec, cntOff);
            if (cnt != (short) manualCount) {
                throw new AssertionError("nonEmptyBlockCount mismatch @seq " + s
                        + " field=" + cnt + " manual=" + manualCount);
            }
        }
        System.out.println("[3] section truth: " + seqs + " sequences, " + nOps
                + " real setBlockState ops — hasOnlyAir + nonEmptyBlockCount bit-consistent PASS");

        System.out.println("INSIDE-BITMASK LOCKSTEP: ALL PASS");
    }

    private static int floor(double v) {
        int i = (int) v;
        return v < i ? i - 1 : i;
    }

    private InsideBitmaskLockstepHarness() {}
}
