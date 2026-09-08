package collision;

import java.nio.file.*;
import java.util.Arrays;

/**
 * TASK-104 self-test (S7-46; pattern per TASK-90 SelfTest 5a8577b): proves each woven
 * probe target is VALID BYTECODE by loading transformed bytes through the JVM verifier
 * and running them; asserts exact per-probe counter deltas. Catches the nested-class
 * binary-name class of bugs (TASK-90 proven) for all 6 probes Q1-Q4/M1-M2.
 */
public final class CollisionSelfTest {

    public static void main(String[] argv) throws Exception {
        // Entity carries THREE probes: collide->0, move->3, setPos->5.
        Class<?> entity = woven("net/minecraft/world/entity/Entity", "net.minecraft.world.entity.Entity");
        Object e = entity.getDeclaredConstructor().newInstance();
        Object vec = forName("net.minecraft.world.phys.Vec3").getDeclaredConstructor().newInstance();
        Object mt = forName("net.minecraft.world.entity.MoverType").getDeclaredConstructor().newInstance();

        long b0 = CollisionCensusAgent.StaticCounter.COUNTERS.get(0);
        long b3 = CollisionCensusAgent.StaticCounter.COUNTERS.get(3);
        long b5 = CollisionCensusAgent.StaticCounter.COUNTERS.get(5);
        java.lang.reflect.Method collide = entity.getMethod("collide", forName("net.minecraft.world.phys.Vec3"));
        java.lang.reflect.Method move = entity.getMethod("move", forName("net.minecraft.world.entity.MoverType"),
            forName("net.minecraft.world.phys.Vec3"));
        java.lang.reflect.Method setPos = entity.getMethod("setPos", double.class, double.class, double.class);
        for (int i = 0; i < 3; i++) collide.invoke(e, vec);
        for (int i = 0; i < 4; i++) move.invoke(e, mt, vec);
        for (int i = 0; i < 2; i++) setPos.invoke(e, 1.0, 2.0, 3.0);
        chk(0, b0, 3, "Q1 Entity.collide");
        chk(3, b3, 4, "Q4 Entity.move");
        chk(5, b5, 2, "M2 Entity.setPos");

        // BlockCollisions.computeNext -> counter 1 (protected: declared-method reflection)
        Class<?> bc = woven("net/minecraft/world/level/BlockCollisions", "net.minecraft.world.level.BlockCollisions");
        Object bci = bc.getDeclaredConstructor().newInstance();
        java.lang.reflect.Method cn = bc.getDeclaredMethod("computeNext");
        cn.setAccessible(true);
        long b1 = CollisionCensusAgent.StaticCounter.COUNTERS.get(1);
        for (int i = 0; i < 5; i++) cn.invoke(bci);
        chk(1, b1, 5, "Q2 BlockCollisions.computeNext");

        // Shapes.collide (static) -> counter 2
        Class<?> sh = woven("net/minecraft/world/phys/shapes/Shapes", "net.minecraft.world.phys.shapes.Shapes");
        java.lang.reflect.Method sc = sh.getMethod("collide", forName("net.minecraft.core.Direction$Axis"),
            forName("net.minecraft.world.phys.AABB"), Class.forName("java.lang.Iterable"), double.class);
        Object axis = forName("net.minecraft.core.Direction$Axis").getDeclaredConstructor().newInstance();
        Object aabb = forName("net.minecraft.world.phys.AABB").getDeclaredConstructor().newInstance();
        long b2 = CollisionCensusAgent.StaticCounter.COUNTERS.get(2);
        for (int i = 0; i < 6; i++) sc.invoke(null, axis, aabb, java.util.List.of(), 1.5);
        chk(2, b2, 6, "Q3 Shapes.collide");

        // Level.setBlock -> counter 4
        Class<?> lv = woven("net/minecraft/world/level/Level", "net.minecraft.world.level.Level");
        Object lvi = lv.getDeclaredConstructor().newInstance();
        java.lang.reflect.Method sb = lv.getMethod("setBlock", forName("net.minecraft.core.BlockPos"),
            forName("net.minecraft.world.level.block.state.BlockState"), int.class);
        Object bp = forName("net.minecraft.core.BlockPos").getDeclaredConstructor().newInstance();
        Object bs = forName("net.minecraft.world.level.block.state.BlockState").getDeclaredConstructor().newInstance();
        long b4 = CollisionCensusAgent.StaticCounter.COUNTERS.get(4);
        for (int i = 0; i < 4; i++) sb.invoke(lvi, bp, bs, 3);
        chk(4, b4, 4, "M1 Level.setBlock");

        System.out.println("[collision-selftest] PASS — all 6 probes fire with exact deltas, verifier-clean");
    }

    static Class<?> forName(String n) throws Exception { return Class.forName(n); }

    static void chk(int ord, long before, long expect, String label) {
        long after = CollisionCensusAgent.StaticCounter.COUNTERS.get(ord);
        if (after - before != expect)
            throw new AssertionError(label + ": counter delta " + (after - before) + " != " + expect);
        System.out.println("[collision-selftest] " + label + " delta==" + expect + " OK");
    }

    /** Read compiled fake -> weave exactly as the transformer would -> assert change -> define. */
    static Class<?> woven(String internalName, String binaryName) throws Exception {
        Path src = Paths.get("build/selftest", internalName + ".class");
        byte[] raw = Files.readAllBytes(src);
        byte[] bytes = CollisionCensusAgent.weave(raw, internalName);
        if (Arrays.equals(raw, bytes)) throw new AssertionError("weave produced no change for " + internalName);
        Files.createDirectories(Paths.get("build/woven"));
        Files.write(Paths.get("build/woven", binaryName + ".woven.class"), bytes);
        return new Loader().define(binaryName, bytes);
    }

    static final class Loader extends ClassLoader {
        Class<?> define(String name, byte[] bytes) {
            return defineClass(name, bytes, 0, bytes.length);
        }
    }

    private CollisionSelfTest() {}
}
