package net.minecraft.world.entity;

import java.lang.reflect.Field;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.Bootstrap;
import net.minecraft.SharedConstants;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;

/**
 * FLUID-FREE-SECTION offline harness (S7-143 re-implementation of the lost
 * S7-139, ARCH-ATTACK lever #5) — plain JVM, real kernel classes, NO server
 * boot (INJECTS-ONLY; registries are bootstrapped in-memory — the S7-137
 * lesson).
 *
 * Loader model: the PATCHED class files (demux PalettedContainer,
 * section-ff LevelChunkSection, FluidOps bridge) are placed AHEAD of the
 * kernel on the classpath — a single loader space, exactly the runtime
 * retransform result. This proves OFFLINE:
 *   1. STRUCTURAL — the JVM verifier accepts the appended section fields
 *      and the demux container (link = structural pass on the real kernel).
 *   2. ARMED — FluidOps.arm() resolves the three injected fields via Unsafe
 *      offsets (ARMED=true) — the runtime pairing (section cache <-> demux
 *      counter) is what the bridge expects.
 *   3. BEHAVIORAL — a REAL section from the REAL factory: fresh-air scans
 *      fluid-free (stable free-HIT), a vanilla setBlockState(WATER) bumps
 *      the demux counter and forces a live verdict flip (event-driven
 *      invalidation, no polling), the restore flips back, and scattered
 *      water stays a persistent miss — median-exact parity by construction.
 *
 * Exit 0 = FLUID-FREE OFFLINE PASS; any failure throws.
 */
public final class FluidFreeHarness {

    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        // Injected surface (present only in the patched shadowed classes).
        LevelChunkSection.class.getDeclaredField("crusstyFf");
        LevelChunkSection.class.getDeclaredField("crusstyFfGen");
        PalettedContainer.class.getDeclaredField("crusstyGen");
        Class.forName("net.minecraft.world.level.chunk.PalettedContainerOps")
                .getDeclaredMethod("get", PalettedContainer.class, int.class);
        System.out.println("STRUCTURAL+INJECTED SURFACE PASS (patched classes shadow the kernel in ONE loader space)");

        // Build a REAL section through the REAL factory.
        // Vanilla-faithful container assembly without a Level: block-state
        // strategy over Block.BLOCK_STATE_REGISTRY, biome strategy over the
        // holder-id map, plains as the default biome holder (mirrors
        // PalettedContainerFactory.create's own bytecode).
        var blockStrategy = net.minecraft.world.level.chunk.Strategy
                .<BlockState>createForBlockStates(net.minecraft.world.level.block.Block.BLOCK_STATE_REGISTRY);
        BlockState airDefault = net.minecraft.world.level.block.Blocks.AIR.defaultBlockState();
        PalettedContainer<BlockState> statesPc = new PalettedContainer<>(airDefault, blockStrategy);
        // The biome container is irrelevant to the fluid lane (the bridge
        // scans `states` only) and the biome registry is datapack-only in
        // this kernel — pass null and never touch it.
        LevelChunkSection section = new LevelChunkSection(statesPc, null);
        System.out.println("SECTION MATERIALIZED: " + section.getClass().getName());

        // ARMED: Unsafe offsets resolve against the patched classes.
        FluidOps.arm();
        Field armedF = FluidOps.class.getDeclaredField("ARMED");
        armedF.setAccessible(true);
        if (!armedF.getBoolean(null)) {
            throw new AssertionError("FluidOps ARMED must be true");
        }
        System.out.println("FLUID-FREE ARMED=true");

        // Behavioral: fresh-air section = free-HIT, stable.
        boolean v1 = FluidOps.sectionFluidFree(section);
        boolean v2 = FluidOps.sectionFluidFree(section);
        if (!v1 || !v2) {
            throw new AssertionError("fresh-air section must be fluid-free and stable, got " + v1 + "/" + v2);
        }
        byte ff1 = readByte(LevelChunkSection.class, section, "crusstyFf");
        if (ff1 != 1) {
            throw new AssertionError("free verdict byte must be 1, got " + ff1);
        }
        int gen0 = readGen(section);
        int ffGen0 = readInt(LevelChunkSection.class, section, "crusstyFfGen");
        if (gen0 != 0 || ffGen0 != 0) {
            throw new AssertionError("fresh-air epoch pair must be 0/0, got gen=" + gen0 + " ffGen=" + ffGen0);
        }
        System.out.println("FREE-HIT: ff=" + ff1 + " ffGen=" + ffGen0 + " gen=" + gen0);

        // EVENT-DRIVEN INVALIDATION: a vanilla mutation (setBlockState WATER)
        // bumps the demux mutation counter (prologue+epilogue = +2) and the
        // next verdict flips to has-fluids WITHOUT any polling.
        BlockState water = Blocks.WATER.defaultBlockState();
        section.setBlockState(3, 5, 7, water);
        int gen2 = readGen(section);
        if (gen2 != 2) {
            throw new AssertionError("water mutation must bump demux gen 0->2, got " + gen2);
        }
        boolean v3 = FluidOps.sectionFluidFree(section);
        if (v3) {
            throw new AssertionError("water section must NOT be fluid-free");
        }
        byte ff2 = readByte(LevelChunkSection.class, section, "crusstyFf");
        int ffGen2 = readInt(LevelChunkSection.class, section, "crusstyFfGen");
        if (ff2 != 2 || ffGen2 != 2) {
            throw new AssertionError("has-fluid verdict must be (2, gen=2), got (" + ff2 + ", " + ffGen2 + ")");
        }
        System.out.println("EVENT: gen " + gen0 + "->" + gen2 + " ff=" + ff2 + " ffGen=" + ffGen2);

        // RESTORE: draining the cell flips the verdict back (no polling).
        section.setBlockState(3, 5, 7, airDefault);
        int gen4 = readGen(section);
        if (gen4 != 4) {
            throw new AssertionError("restore mutation must bump demux gen 2->4, got " + gen4);
        }
        boolean v4 = FluidOps.sectionFluidFree(section);
        if (!v4) {
            throw new AssertionError("restored section must be fluid-free again");
        }
        byte ff3 = readByte(LevelChunkSection.class, section, "crusstyFf");
        int ffGen4 = readInt(LevelChunkSection.class, section, "crusstyFfGen");
        if (ff3 != 1 || ffGen4 != 4) {
            throw new AssertionError("restored verdict must be (1, gen=4), got (" + ff3 + ", " + ffGen4 + ")");
        }
        System.out.println("RESTORE: gen=" + gen4 + " ffGen=" + ffGen4 + " ff=" + ff3 + " verdict=" + v4);

        // SCATTERED-WATER: a fresh section with one mid-section water cell
        // must stay a persistent miss (the scan finds the cell; the verdict
        // cache then serves has-fluids without re-scanning).
        PalettedContainer<BlockState> scatteredPc = new PalettedContainer<>(airDefault, blockStrategy);
        LevelChunkSection scattered = new LevelChunkSection(scatteredPc, null);
        scattered.setBlockState(9, 9, 9, water);
        boolean s1 = FluidOps.sectionFluidFree(scattered);
        boolean s2 = FluidOps.sectionFluidFree(scattered);
        if (s1 || s2) {
            throw new AssertionError("scattered-water section must be a persistent miss, got " + s1 + "/" + s2);
        }
        byte sff = readByte(LevelChunkSection.class, scattered, "crusstyFf");
        if (sff != 2) {
            throw new AssertionError("scattered-water verdict byte must be 2, got " + sff);
        }
        System.out.println("SCATTERED-WATER PARITY PASS: miss stable");

        System.out.println("FLUID-FREE OFFLINE PASS");
        System.exit(0);
    }

    private static byte readByte(Class<?> cls, Object obj, String name) throws Exception {
        Field f = cls.getDeclaredField(name);
        f.setAccessible(true);
        return f.getByte(obj);
    }

    private static int readInt(Class<?> cls0, Object obj, String name) throws Exception {
        java.lang.reflect.Field f = cls0.getDeclaredField(name);
        f.setAccessible(true);
        return f.getInt(obj);
    }

    private static int readGen(LevelChunkSection section) throws Exception {
        Field states = LevelChunkSection.class.getField("states");
        Object pc = states.get(section);
        Field gen = PalettedContainer.class.getDeclaredField("crusstyGen");
        gen.setAccessible(true);
        return gen.getInt(pc);
    }
}
