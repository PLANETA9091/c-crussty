package agent;

import java.nio.file.*;
import java.util.Arrays;

/**
 * TASK-90 self-test (agent-7625532f): proves the woven class is VALID BYTECODE
 * by loading the transformed bytes through the JVM verifier and running them.
 * Strongest server-free check: transform -> defineClass -> invoke -> counter delta.
 */
public final class SelfTest {
    public static void main(String[] argv) throws Exception {
        // 1. Load the FAKE BlockEntity bytes (squats the probe-target internal name).
        byte[] raw = Files.readAllBytes(
            Paths.get("build/selftest/net/minecraft/world/level/block/entity/BlockEntity.class"));

        // 2. Weave exactly as the transformer would (same code path).
        byte[] woven = DirtyCensusAgent.weave(raw, "net/minecraft/world/level/block/entity/BlockEntity");
        if (Arrays.equals(raw, woven)) throw new AssertionError("weave produced no change");
        System.out.println("[selftest] woven bytes: " + raw.length + " -> " + woven.length);
        Files.createDirectories(Paths.get("build/woven"));
        Files.write(Paths.get("build/woven/BlockEntity.woven.class"), woven);

        // 3. Define + initialize through a fresh loader (verifier runs HERE).
        FakeLoader cl = new FakeLoader();
        Class<?> fake = cl.define();
        Object inst = fake.getDeclaredConstructor().newInstance();

        // 4. Baseline counter, then invoke setChanged 5 times -> counter C0 must advance by exactly 5.
        long before = DirtyCensusAgent.StaticCounter.COUNTERS.get(0);
        java.lang.reflect.Method setChanged = fake.getMethod("setChanged");
        for (int i = 0; i < 5; i++) setChanged.invoke(inst);
        long after = DirtyCensusAgent.StaticCounter.COUNTERS.get(0);
        if (after - before != 5) throw new AssertionError("counter delta " + (after - before) + " != 5");
        System.out.println("[selftest] PASS — entry probe fires, counter delta exactly 5, verifier-clean");
    }
    static final class FakeLoader extends ClassLoader {
        Class<?> define() throws Exception {
            byte[] woven = Files.readAllBytes(Paths.get("build/woven/BlockEntity.woven.class"));
            return defineClass("net.minecraft.world.level.block.entity.BlockEntity",
                woven, 0, woven.length);
        }
    }
    private SelfTest() {}
}
