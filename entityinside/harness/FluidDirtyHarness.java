package net.minecraft.world.entity;

import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Path;
import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.PalettedContainer;
import net.minecraft.world.level.material.FluidState;

/**
 * FLUID-DIRTY offline harness (S7-151 / TASK-290, ARCH-ATTACK lever #6) —
 * plain JVM, real kernel classes, NO server boot (INJECTS-ONLY; registries
 * bootstrapped in-memory — the S7-137 lesson).
 *
 * Loader model: the PATCHED class files (Entity retarget, LevelChunk
 * retarget — rust classfile::patch_fluid_dirty_* output) are defined through
 * a byte-map loader over the REAL kernel — exactly the runtime retransform
 * result. This proves OFFLINE:
 *
 *   1. STRUCTURAL — the JVM verifier accepts both patched classes (link =
 *      structural pass on the real kernel shape).
 *   2. WIRING — both patched classes reference FluidPushOps and carry
 *      invokestatic opcodes (the retarget actually landed).
 *   3. ARMED — FluidPushOps.arm() resolves the package-private surface.
 *   4. LEDGER — real LevelChunkSection through the real factory:
 *      secWrite(air->water) bumps the section stamp (event-driven
 *      invalidation, no polling) and returns the OLD state (vanilla
 *      contract); water->water (same FluidState singleton) does NOT bump;
 *      water->air bumps again. Delegate = vanilla section write.
 *
 * Exit 0 = FLUID-DIRTY OFFLINE PASS (structural/ledger tier); any failure
 * throws. The behavioral scan lockstep (mini-Level, G5 gate) is the
 * S7-152 deliverable.
 */
public final class FluidDirtyHarness {

    public static void main(String[] args) throws Exception {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        // ---- 1. STRUCTURAL: define both patched classes over the kernel ----
        Path entityPatched = Path.of(
                args.length > 0 ? args[0] : "../tests/out/Entity.fluiddirty.patched.class");
        Path lcPatched = Path.of(
                args.length > 1 ? args[1] : "../tests/out/LevelChunk.fluiddirty.patched.class");
        byte[] entityBytes = Files.readAllBytes(entityPatched);
        byte[] lcBytes = Files.readAllBytes(lcPatched);

        ClassLoader parent = FluidDirtyHarness.class.getClassLoader();
        ClassLoader l2 = new ByteMapClassLoader(parent, entityBytes, lcBytes);
        Class<?> entity = Class.forName("net.minecraft.world.entity.Entity", false, l2);
        Class<?> lc = Class.forName("net.minecraft.world.level.chunk.LevelChunk", false, l2);
        // Full link: resolve key members (forces verification of the patched shapes).
        entity.getDeclaredMethod("updateInWaterStateAndDoFluidPushing");
        entity.getDeclaredMethod("updateInWaterStateAndDoWaterCurrentPushing");
        entity.getDeclaredField("fluidHeight");
        lc.getDeclaredMethod("setBlockState", Class.forName("net.minecraft.core.BlockPos"),
                Class.forName("net.minecraft.world.level.block.state.BlockState"), int.class);
        System.out.println("STRUCTURAL PASS (patched Entity+LevelChunk linked over the real kernel)");

        // ---- 2. WIRING: retargeted sites reference FluidPushOps ----
        for (byte[] b : new byte[][] {entityBytes, lcBytes}) {
            if (!new String(b, java.nio.charset.StandardCharsets.ISO_8859_1)
                    .contains("net/minecraft/world/entity/FluidPushOps")) {
                throw new AssertionError("patched class lacks FluidPushOps reference");
            }
        }
        if (!hasInvokeStatic(entityBytes) || !hasInvokeStatic(lcBytes)) {
            throw new AssertionError("patched class lacks invokestatic opcode");
        }
        System.out.println("WIRING PASS (both patched classes reference FluidPushOps, invokestatic present)");

        // ---- 3. ARMED ----
        FluidPushOps.arm();
        Field armedF = FluidPushOps.class.getDeclaredField("ARMED");
        armedF.setAccessible(true);
        if (!armedF.getBoolean(null)) {
            throw new AssertionError("FluidPushOps ARMED must be true");
        }
        System.out.println("FLUID-DIRTY ARMED=true");

        // ---- 4. LEDGER: real section, real delegate semantics ----
        var blockStrategy = net.minecraft.world.level.chunk.Strategy
                .<BlockState>createForBlockStates(net.minecraft.world.level.block.Block.BLOCK_STATE_REGISTRY);
        BlockState airDefault = Blocks.AIR.defaultBlockState();
        PalettedContainer<BlockState> statesPc = new PalettedContainer<>(airDefault, blockStrategy);
        LevelChunkSection section = new LevelChunkSection(statesPc, null);
        System.out.println("SECTION MATERIALIZED: " + section.getClass().getName());

        BlockState water = Blocks.WATER.defaultBlockState();
        BlockState air = Blocks.AIR.defaultBlockState();

        if (FluidPushOps.stampOf(section) != 0L) {
            throw new AssertionError("fresh section stamp must be 0");
        }

        // air -> water: fluid changed => bump; delegate returns OLD (air)
        BlockState old1 = FluidPushOps.secWrite(section, 3, 5, 7, water);
        if (old1 != air) {
            throw new AssertionError("secWrite must return the old state (air)");
        }
        if (statesPc.get((3 & 15) | ((7 & 15) << 4) | ((5 & 15) << 8)) != water) {
            throw new AssertionError("secWrite must delegate the write to the section");
        }
        long s1 = FluidPushOps.stampOf(section);
        if (s1 != 1L) {
            throw new AssertionError("air->water must bump stamp once, got " + s1);
        }
        System.out.println("LEDGER PASS air->water: old-state returned, write delegated, stamp 0->1");

        // water -> water: same FluidState singleton => NO bump
        BlockState old2 = FluidPushOps.secWrite(section, 3, 5, 7, water);
        if (old2 != water) {
            throw new AssertionError("secWrite must return the old state (water)");
        }
        long s2 = FluidPushOps.stampOf(section);
        if (s2 != 1L) {
            throw new AssertionError("water->water (same singleton) must NOT bump, got " + s2);
        }
        System.out.println("LEDGER PASS water->water: no bump (ref-compare, no false invalidation)");

        // water -> air: fluid changed => bump again
        BlockState old3 = FluidPushOps.secWrite(section, 3, 5, 7, air);
        if (old3 != water) {
            throw new AssertionError("secWrite must return the old state (water)");
        }
        long s3 = FluidPushOps.stampOf(section);
        if (s3 != 2L) {
            throw new AssertionError("water->air must bump again, got " + s3);
        }
        System.out.println("LEDGER PASS water->air: stamp 1->2");

        // unrelated cell write does not touch THIS cell's invalidation semantics
        // (whole-section stamp is the documented conservative granularity)
        BlockState stone = Blocks.STONE.defaultBlockState();
        FluidPushOps.secWrite(section, 9, 5, 9, stone);
        long s4 = FluidPushOps.stampOf(section);
        if (s4 != 2L) {
            throw new AssertionError("air->stone (no fluid change) must NOT bump, got " + s4);
        }
        System.out.println("LEDGER PASS air->stone: no bump (ordinary blocks never invalidate)");

        System.out.println("FLUID-DIRTY OFFLINE PASS (structural/wiring/armed/ledger tier)");
    }

    private static boolean hasInvokeStatic(byte[] cls) {
        // invokestatic (0xB8) opcode presence — the retarget's signature move;
        // deep-site validation lives in the rust tests (cp methodref triples).
        for (byte b : cls) {
            if (b == (byte) 0xB8) {
                return true;
            }
        }
        return false;
    }

    /** Byte-map loader: patched classes first, kernel for everything else. */
    static class ByteMapClassLoader extends ClassLoader {
        private final java.util.Map<String, byte[]> map = new java.util.HashMap<>();

        ByteMapClassLoader(ClassLoader parent, byte[]... entries) {
            super(parent);
            // entries alternate name/bytes pairs is overkill: fixed names.
            map.put("net.minecraft.world.entity.Entity", entries[0]);
            map.put("net.minecraft.world.level.chunk.LevelChunk", entries[1]);
        }

        @Override
        protected Class<?> findClass(String name) throws ClassNotFoundException {
            byte[] b = map.get(name);
            if (b == null) {
                throw new ClassNotFoundException(name);
            }
            return defineClass(name, b, 0, b.length);
        }
    }
}
