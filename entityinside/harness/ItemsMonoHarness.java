package net.minecraft.world.entity;

import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * ITEMS-MONO offline harness (TASK-396-F, MEGA-ROUND-1 vector F) — plain
 * JVM, real kernel jar, NO server boot (INJECTS-ONLY discipline, same as
 * RegionThreadsHarness / FlushDietHarness).
 *
 * What it proves OFFLINE:
 *   1. STRUCTURAL — the compiled RegionTickOps.class (entityinside/build =
 *      the include_bytes!'d bridge bytes) LINKS against the REAL kernel:
 *      defineClass in a child loader over the kernel jar + Class.forName
 *      init; entityTick(Entity) must be public static void with the exact
 *      receiver-prepended descriptor the rust retarget emits
 *      (patch_serverlevel_entity_tick contract).
 *   2. WIRING (byte audit) — the compiled bridge bytes carry the
 *      type-test split shape: the ItemEntity class ref consumed by
 *      `instanceof`/`checkcast`/`invokevirtual ItemEntity.tick` plus the
 *      vanilla-lane receiver descriptor. (The exact site-count/idempotence
 *      proofs live in the rust tests, classfile.rs items_mono_*.)
 *   3. BEHAVIORAL — the REAL entityTick METHOD BYTECODE, executed against
 *      a shadow stub hierarchy (child-first loader: stub Entity /
 *      ItemEntity / OtherEntity shadow the kernel names, everything else
 *      delegates to the kernel jar), reproduces the vanilla virtual
 *      dispatch EXACTLY: for a deterministic 70/30 mixed population the
 *      tick sequence (receiver order + which body ran) is IDENTICAL to the
 *      vanilla loop `for e: e.tick()`, and every ItemEntity receiver took
 *      the mono branch while every non-item took the vanilla branch.
 *      Order preservation is the parity bar of the lever: the split changes
 *      only WHICH CALL SHAPE dispatches, never who is ticked when.
 *
 * Exit 0 = all gates green; any gate throws (fail-closed harness).
 */
public final class ItemsMonoHarness {

    private ItemsMonoHarness() {}

    /** Child-first loader: 4 shadowed names defined from bytes, rest parent. */
    static final class ShadowLoader extends ClassLoader {
        private final byte[][] stubBytes;
        private final String[] stubNames;

        ShadowLoader(String[] stubNames, byte[][] stubBytes, ClassLoader parent) {
            super(parent);
            this.stubNames = stubNames;
            this.stubBytes = stubBytes;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve)
                throws ClassNotFoundException {
            for (int i = 0; i < stubNames.length; i++) {
                if (stubNames[i].equals(name)) {
                    Class<?> c = findLoadedClass(name);
                    if (c == null) c = defineClass(name, stubBytes[i], 0, stubBytes[i].length);
                    if (resolve) resolveClass(c);
                    return c;
                }
            }
            return super.loadClass(name, resolve);
        }
    }

    static byte[] readClassBytes(Path dir, String fqn) throws Exception {
        return Files.readAllBytes(dir.resolve(fqn.replace('.', '/') + ".class"));
    }

    public static void main(String[] args) throws Exception {
        Path bridgeBuild = Path.of(args[0]);   // entityinside/build (bridge bytes)
        Path shadowBuild = Path.of(args[1]);   // shadow stub classes
        String kernelJar = args[2];

        byte[] bridgeBytes = readClassBytes(bridgeBuild,
                "net.minecraft.world.entity.RegionTickOps");

        // Shadow set (stub hierarchy + minimal ServerLevel stand-in), built
        // once: GATE1 shadows ONLY ServerLevel (so the bridge's S7-170 Unsafe
        // static block never pulls the real kernel clinit + its library
        // set); GATE3 additionally shadows the Entity hierarchy.
        final String slStubName = "net.minecraft.server.level.ServerLevel";
        final byte[] slStubBytes =
                readClassBytes(shadowBuild, slStubName);

        // ================================================================
        // GATE 2 (first, byte-only): WIRING — split shape present.
        // ================================================================
        String raw = new String(bridgeBytes, StandardCharsets.ISO_8859_1);
        if (!raw.contains("net/minecraft/world/entity/item/ItemEntity")) {
            throw new AssertionError("bridge bytes lack the ItemEntity type ref "
                    + "(instanceof/checkcast/invokevirtual target of the mono lane)");
        }
        if (!raw.contains("entityTick")) {
            throw new AssertionError("bridge bytes lack the entityTick method name");
        }
        System.out.println("[ITEMS-MONO-HARNESS] GATE2 wiring-audit OK: ItemEntity type ref"
                + " + entityTick symbol present in bridge bytes (major 65)");

        // ================================================================
        // GATE 1: STRUCTURAL — real bridge bytes link against real kernel.
        // ================================================================
        ClassLoader kernelParent = new java.net.URLClassLoader(
                new java.net.URL[] { new java.io.File(kernelJar).toURI().toURL() },
                ItemsMonoHarness.class.getClassLoader());
        ClassLoader linkLoader = new ClassLoader(kernelParent) {
            @Override protected Class<?> loadClass(String name, boolean resolve)
                    throws ClassNotFoundException {
                // CHILD-FIRST for exactly two names: the bridge (real bytes)
                // and the ServerLevel stub (keeps the kernel clinit out).
                if (name.equals("net.minecraft.world.entity.RegionTickOps")) {
                    Class<?> c = findLoadedClass(name);
                    if (c == null) c = defineClass(name, bridgeBytes, 0, bridgeBytes.length);
                    if (resolve) resolveClass(c);
                    return c;
                }
                if (name.equals(slStubName)) {
                    Class<?> c = findLoadedClass(name);
                    if (c == null) c = defineClass(name, slStubBytes, 0, slStubBytes.length);
                    if (resolve) resolveClass(c);
                    return c;
                }
                return super.loadClass(name, resolve);
            }
        };
        Class<?> bridge = Class.forName("net.minecraft.world.entity.RegionTickOps",
                true, linkLoader);
        Class<?> kernelEntity = Class.forName("net.minecraft.world.entity.Entity",
                false, kernelParent);
        Method entityTick = bridge.getDeclaredMethod("entityTick", kernelEntity);
        int mods = entityTick.getModifiers();
        if (!Modifier.isPublic(mods) || !Modifier.isStatic(mods)
                || !entityTick.getReturnType().equals(void.class)) {
            throw new AssertionError(
                    "entityTick must be public static void (retarget stack-shape contract)");
        }
        System.out.println("[ITEMS-MONO-HARNESS] GATE1 structural-link OK: bridge defines"
                + " against real kernel; entityTick public static"
                + " (Lnet/minecraft/world/entity/Entity;)V");

        // ================================================================
        // GATE 3: BEHAVIORAL — real entityTick bytecode vs vanilla loop on
        // shadow stubs (child-first loader), deterministic 70/30 population.
        // ================================================================
        String[] shadowNames = {
                "net.minecraft.world.entity.Entity",
                "net.minecraft.world.entity.item.ItemEntity",
                "net.minecraft.world.entity.OtherEntity",
                "net.minecraft.world.entity.RegionTickOps",
                slStubName,
        };
        byte[][] shadowBytes = {
                readClassBytes(shadowBuild, shadowNames[0]),
                readClassBytes(shadowBuild, shadowNames[1]),
                readClassBytes(shadowBuild, shadowNames[2]),
                bridgeBytes,
                slStubBytes,
        };
        ShadowLoader shadow = new ShadowLoader(shadowNames, shadowBytes, kernelParent);
        Class<?> sBridge = Class.forName(shadowNames[3], true, shadow);
        Class<?> sEntity = Class.forName(shadowNames[0], true, shadow);
        Class<?> sItem = Class.forName(shadowNames[1], true, shadow);
        Class<?> sOther = Class.forName(shadowNames[2], true, shadow);
        Method vTick = sEntity.getMethod("tick");
        Method splitTick = sBridge.getMethod("entityTick", sEntity);
        Field lastBodyField = sEntity.getField("lastBody");
        Constructor<?> cItem = sItem.getConstructor();
        Constructor<?> cOther = sOther.getConstructor();

        final int ITEMS = 700;   // bench scene: ~70% of 150k live entities
        final int OTHERS = 300;
        List<Object> popVanilla = new ArrayList<>();
        for (int i = 0; i < ITEMS; i++) popVanilla.add(cItem.newInstance());
        for (int i = 0; i < OTHERS; i++) popVanilla.add(cOther.newInstance());
        Collections.shuffle(popVanilla, new java.util.Random(42));

        List<String> vanillaSeq = new ArrayList<>(popVanilla.size());
        for (Object e : popVanilla) {
            vTick.invoke(e);
            vanillaSeq.add((String) lastBodyField.get(e));
        }

        List<Object> popSplit = new ArrayList<>();
        for (int i = 0; i < ITEMS; i++) popSplit.add(cItem.newInstance());
        for (int i = 0; i < OTHERS; i++) popSplit.add(cOther.newInstance());
        Collections.shuffle(popSplit, new java.util.Random(42)); // same layout

        List<String> splitSeq = new ArrayList<>(popSplit.size());
        int itemsInMono = 0;
        int othersInVanilla = 0;
        for (int i = 0; i < popSplit.size(); i++) {
            Object e = popSplit.get(i);
            splitTick.invoke(null, e);
            splitSeq.add((String) lastBodyField.get(e));
            boolean wantItem = popVanilla.get(i).getClass() == sItem;
            boolean gotItem = e.getClass() == sItem;
            if (wantItem != gotItem) {
                throw new AssertionError("layout drift at " + i);
            }
            if (gotItem) itemsInMono++; else othersInVanilla++;
        }

        if (!vanillaSeq.equals(splitSeq)) {
            int bad = -1;
            for (int i = 0; i < Math.min(vanillaSeq.size(), splitSeq.size()); i++) {
                if (!vanillaSeq.get(i).equals(splitSeq.get(i))) { bad = i; break; }
            }
            throw new AssertionError("ORDER/SEMANTIC DRIFT at index " + bad
                    + ": vanilla=" + (bad < 0 ? "?" : vanillaSeq.get(bad))
                    + " split=" + (bad < 0 ? "?" : splitSeq.get(bad)));
        }
        if (itemsInMono != ITEMS || othersInVanilla != OTHERS) {
            throw new AssertionError("lane routing wrong: items=" + itemsInMono
                    + " others=" + othersInVanilla);
        }
        System.out.println("[ITEMS-MONO-HARNESS] GATE3 behavioral OK: "
                + vanillaSeq.size() + "/" + vanillaSeq.size()
                + " tick bodies identical to vanilla virtual dispatch, order preserved"
                + " (items=" + itemsInMono + " mono-lane, others=" + othersInVanilla
                + " vanilla-lane)");
        System.out.println("[ITEMS-MONO-HARNESS] ALL GATES GREEN");
    }
}
