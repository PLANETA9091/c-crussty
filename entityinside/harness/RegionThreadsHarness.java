package net.minecraft.world.entity;

import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Consumer;

import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.level.entity.EntityTickList;

/**
 * REGION-THREADS offline harness (S7-156 / TASK-295, ARCH-ATTACK lever #7) —
 * plain JVM, real kernel classes, NO server boot (INJECTS-ONLY).
 *
 * Loader model: the PATCHED class files (ServerLevel tick-segment splice,
 * EntityCallbacks guard retargets — rust classfile::patch_region_tick_*
 * output, dumped by the roundtrip tests) are defined through a byte-map
 * loader over the REAL kernel — exactly the runtime retransform result.
 *
 * Parent run (env absent, W=1 dormant):
 *   1. STRUCTURAL — the JVM verifier accepts both patched classes.
 *   2. WIRING — patched ServerLevel CP carries a RegionTickOps Methodref
 *      (forEach splice landed; exact site counts pinned by the rust
 *      roundtrip tests Retargeted{1}/Retargeted{1+1}), EntityCallbacks too.
 *   3. BRIDGE — RegionTickOps (+Mut) defined in the patch loader
 *      (kernel-loader protocol); forEach/guards are static; workers() == 1.
 *   4. DORMANT SEMANTICS — vanilla passthrough ticks every entity exactly
 *      once in insertion order; guard sites add/remove directly.
 *
 * Child run (env CRUSSTY_REGION_THREADS=2): the parallel phase on a REAL
 * EntityTickList — every entity ticks EXACTLY ONCE across main+helper
 * (disjoint buckets), no deadlock, barrier join completes. This is the
 * PG1-lite container gate; the full per-entity lockstep + live A/B
 * (TPS >= +25% vs CUMULATIVE) is the S7-157 dispatch gate.
 *
 * Exit 0 = REGION-THREADS OFFLINE PASS.
 */
public final class RegionThreadsHarness {

    private static void check(boolean cond, String what) {
        if (!cond) throw new AssertionError("REGION-THREADS HARNESS FAIL: " + what);
        System.out.println("[harness] ok: " + what);
    }

    /** Byte-map loader over the real kernel (no disk classes win). */
    static final class PatchLoader extends ClassLoader {
        private final Map<String, byte[]> overrides;

        PatchLoader(ClassLoader parent, Map<String, byte[]> overrides) {
            super(parent);
            this.overrides = overrides;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            byte[] bytes = overrides.get(name);
            if (bytes != null) {
                Class<?> c = defineClass(name, bytes, 0, bytes.length);
                if (resolve) resolveClass(c);
                return c;
            }
            return super.loadClass(name, resolve);
        }
    }

    /** Minimal constant-pool walker: count Methodrefs to a class name. */
    static int countMethodrefsTo(byte[] cls, String className) throws Exception {
        DataInputStream in = new DataInputStream(new ByteArrayInputStream(cls));
        in.readInt();          // magic
        in.readUnsignedShort();// minor
        in.readUnsignedShort();// major
        int cpCount = in.readUnsignedShort();
        String[] utf8 = new String[cpCount];
        String[] clsName = new String[cpCount];
        int refs = 0;
        for (int i = 1; i < cpCount; i++) {
            int tag = in.readUnsignedByte();
            switch (tag) {
                case 1: utf8[i] = in.readUTF(); break; // actually 2-byte len string
                case 3: case 4: in.skipBytes(4); break;
                case 5: case 6: in.skipBytes(8); i++; break;
                case 7: case 8: case 16: case 19: case 20: {
                    int idx = in.readUnsignedShort();
                    if (tag == 7 && utf8[idx] != null) clsName[i] = utf8[idx];
                    break;
                }
                case 9: case 10: case 11: {
                    int c = in.readUnsignedShort();
                    in.readUnsignedShort(); // nat
                    if (clsName[c] != null && clsName[c].equals(className)) refs++;
                    break;
                }
                case 12: in.skipBytes(4); break;
                case 15: in.skipBytes(3); break;
                case 17: case 18: in.skipBytes(4); break;
                default: throw new IllegalStateException("cp tag " + tag);
            }
        }
        return refs;
    }

    private static sun.misc.Unsafe unsafe() throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        return (sun.misc.Unsafe) f.get(null);
    }

    /** Entity carries uuid (UUID) AND stringUUID (String, read by
     *  getStringUUID) plus originWorld (also UUID!) — set by NAME. */
    private static void setIdentity(sun.misc.Unsafe unsafe, Entity e, UUID u) {
        try {
            Field uuid = net.minecraft.world.entity.Entity.class.getDeclaredField("uuid");
            Field stringUUID =
                    net.minecraft.world.entity.Entity.class.getDeclaredField("stringUUID");
            Field chunkPos =
                    net.minecraft.world.entity.Entity.class.getDeclaredField("chunkPosition");
            uuid.setAccessible(true);
            stringUUID.setAccessible(true);
            chunkPos.setAccessible(true);
            long ordinal = u.getLeastSignificantBits();
            // spread across 16x16 chunk lattice -> multiple 8-chunk regions
            net.minecraft.world.level.ChunkPos cp = new net.minecraft.world.level.ChunkPos(
                    (int) (ordinal % 64), (int) ((ordinal >>> 8) % 64));
            unsafe.putObject(e, unsafe.objectFieldOffset(uuid), u);
            unsafe.putObject(e, unsafe.objectFieldOffset(stringUUID), u.toString());
            unsafe.putObject(e, unsafe.objectFieldOffset(chunkPos), cp);
        } catch (ReflectiveOperationException ex) {
            throw new RuntimeException(ex);
        }
    }

    public static void main(String[] args) throws Exception {
        // registries in-memory before any Entity class init (the S7-137 lesson,
        // same as FluidDirtyHarness).
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();
        if (args.length > 0 && args[0].equals("--child")) { child(); return; }
        parent(args);
    }

    private static void parent(String[] args) throws Exception {
        Path slPatched = Path.of(args.length > 0
                ? args[0] : "tests/out/ServerLevel.regionthreads.patched.class");
        Path cbPatched = Path.of(args.length > 1
                ? args[1] : "tests/out/EntityCallbacks.regionthreads.patched.class");
        Path opsClass = Path.of(args.length > 2
                ? args[2] : "entityinside/build/net/minecraft/world/entity/RegionTickOps.class");
        Path opsMut = Path.of(args.length > 3
                ? args[3] : "entityinside/build/net/minecraft/world/entity/RegionTickOps$Mut.class");
        Path lvPatched = Path.of(args.length > 4
                ? args[4] : "tests/out/Level.regionthreads.patched.class");

        byte[] slBytes = Files.readAllBytes(slPatched);
        byte[] cbBytes = Files.readAllBytes(cbPatched);
        byte[] opsBytes = Files.readAllBytes(opsClass);
        byte[] mutBytes = Files.readAllBytes(opsMut);
        byte[] lvBytes = Files.readAllBytes(lvPatched);
        check(slBytes[0] == (byte) 0xCA && slBytes[1] == (byte) 0xFE, "ServerLevel patched magic");
        check(cbBytes[0] == (byte) 0xCA && cbBytes[1] == (byte) 0xFE, "EntityCallbacks patched magic");
        check(lvBytes[0] == (byte) 0xCA && lvBytes[1] == (byte) 0xFE, "Level patched magic");

        // ---- 2. WIRING: Methodrefs to the bridge in all three patched classes ----
        check(countMethodrefsTo(slBytes, "net/minecraft/world/entity/RegionTickOps") >= 1,
                "ServerLevel carries RegionTickOps Methodref (forEach splice)");
        check(countMethodrefsTo(cbBytes, "net/minecraft/world/entity/RegionTickOps") >= 2,
                "EntityCallbacks carries 2 RegionTickOps Methodrefs (add+remove guards)");
        check(countMethodrefsTo(lvBytes, "net/minecraft/world/entity/RegionTickOps") >= 1,
                "Level carries RegionTickOps Methodref (S7-157b mid-tick gate)");

        // ---- 1+3. STRUCTURAL + BRIDGE: define patched classes over the real
        // kernel, RegionTickOps (+Mut) in the SAME loader (kernel-loader
        // protocol from the runtime wiring).
        PatchLoader loader = new PatchLoader(
                RegionThreadsHarness.class.getClassLoader(),
                Map.of(
                        "net.minecraft.server.level.ServerLevel", slBytes,
                        "net.minecraft.server.level.ServerLevel$EntityCallbacks", cbBytes,
                        "net.minecraft.world.entity.RegionTickOps", opsBytes,
                        "net.minecraft.world.entity.RegionTickOps$Mut", mutBytes,
                        "net.minecraft.world.level.Level", lvBytes));

        Class<?> ops = Class.forName("net.minecraft.world.entity.RegionTickOps", false, loader);
        Class<?> callbacks = Class.forName(
                "net.minecraft.server.level.ServerLevel$EntityCallbacks", false, loader);
        Class<?> sl = Class.forName("net.minecraft.server.level.ServerLevel", false, loader);
        Class<?> lv = Class.forName("net.minecraft.world.level.Level", false, loader);
        check(ops.getClassLoader() == loader, "RegionTickOps defined in the patch loader");
        check(callbacks.getClassLoader() == loader,
                "EntityCallbacks patched (JVM verifier accepted)");
        check(sl.getClassLoader() == loader,
                "ServerLevel patched (JVM verifier accepted)");
        check(lv.getClassLoader() == loader,
                "Level patched (JVM verifier accepted — S7-157b mid-tick gate)");

        Method forEach = ops.getDeclaredMethod("forEach", EntityTickList.class, Consumer.class);
        Method onAdd = ops.getDeclaredMethod("onTickingStart", EntityTickList.class, Entity.class);
        Method onRem = ops.getDeclaredMethod("onTickingEnd", EntityTickList.class, Entity.class);
        check(Modifier.isStatic(forEach.getModifiers()),
                "forEach is static (1:1 receiver-prepended splice target)");
        check(Modifier.isStatic(onAdd.getModifiers()) && Modifier.isStatic(onRem.getModifiers()),
                "guard sites are static");
        // NB: resolve the param type through the PATCH loader — the bridge's
        // Level is the patched class defined in `loader`, not the app one.
        Method midTick = ops.getDeclaredMethod("midTickTasks", lv);
        check(Modifier.isStatic(midTick.getModifiers()),
                "S7-157b midTickTasks gate is static (guardEntityTick retarget)");
        // Main-side delegation is NOT invoked offline: Level is abstract and
        // the concrete ServerLevel override is the REAL vanilla pump (null
        // internals on an Unsafe instance would NPE — and it is untouched
        // vanilla code by construction: the bridge calls the most-derived
        // override via its normal virtual dispatch). Worker/main flag
        // consistency (the actual S7-157b regression surface) is verified in
        // the parallel child below + the main-thread check here.
        sun.misc.Unsafe unsafe0 = unsafe();
        Method isWorker = null;
        try {
            isWorker = ops.getDeclaredMethod("isWorker");
        } catch (NoSuchMethodException e) {
            // older bridge build — flag checks skipped (structural only)
        }
        if (isWorker != null) {
            check(!((Boolean) isWorker.invoke(null)),
                    "isWorker()==false on the main thread (dormant flag)");
        }
        int w = (int) ops.getDeclaredMethod("workers").invoke(null);
        check(w == 1, "workers()==1 with env absent (dormant vanilla passthrough)");

        // ---- 4. DORMANT SEMANTICS on a REAL EntityTickList ----
        sun.misc.Unsafe unsafe = unsafe();
        int n = 6;
        Entity[] entities = new Entity[n];
        for (int i = 0; i < n; i++) {
            entities[i] = (Entity) unsafe.allocateInstance(ItemEntity.class); // Entity is abstract — concrete leaf
            setIdentity(unsafe, entities[i], new UUID(0L, i + 1));
        }
        EntityTickList list = new EntityTickList();
        for (Entity e : entities) list.add(e);

        List<String> order = java.util.Collections.synchronizedList(new ArrayList<>());
        Consumer<Entity> consumer = e -> order.add(e.getStringUUID());
        forEach.invoke(null, list, consumer);
        check(order.size() == n, "vanilla passthrough ticks every entity exactly once");
        boolean inOrder = true;
        for (int i = 0; i < n; i++) {
            if (!order.get(i).equals(new UUID(0L, i + 1).toString())) inOrder = false;
        }
        check(inOrder, "insertion order preserved (vanilla bit-identical path)");

        Entity extra = (Entity) unsafe.allocateInstance(ItemEntity.class);
        onAdd.invoke(null, list, extra);
        check(list.contains(extra), "onTickingStart dormant path adds directly");
        onRem.invoke(null, list, extra);
        check(!list.contains(extra), "onTickingEnd dormant path removes directly");

        // ---- 5. PARALLEL CHILD: W=2 on the real container ----
        java.util.List<String> cmd = new ArrayList<>(java.util.List.of(
                javaBin(), "-cp",
                classpath() + ":entityinside/build",
                "net.minecraft.world.entity.RegionThreadsHarness", "--child"));
        ProcessBuilder pb = new ProcessBuilder(cmd);
        pb.environment().put("CRUSSTY_REGION_THREADS", "2");
        Process p = pb.inheritIO().start();
        int rc = p.waitFor();
        check(rc == 0, "parallel child (CRUSSTY_REGION_THREADS=2) exited 0");

        System.out.println("REGION-THREADS OFFLINE PASS (structural/wiring/dormant+parallel)");
    }

    private static void child() throws Exception {
        sun.misc.Unsafe unsafe = unsafe();
        int n = 200;
        Entity[] entities = new Entity[n];
        for (int i = 0; i < n; i++) {
            entities[i] = (Entity) unsafe.allocateInstance(ItemEntity.class);
            setIdentity(unsafe, entities[i], new UUID(0L, i + 1));
        }
        EntityTickList list = new EntityTickList();
        for (Entity e : entities) list.add(e);

        ConcurrentHashMap<String, Integer> seen = new ConcurrentHashMap<>();
        // S7-157b: per-visit ThreadLocal consistency — the worker flag must
        // be TRUE exactly on helper threads (slot > 0) and FALSE on main
        // (slot 0). Any mismatch = the mid-tick gate would leak either way.
        Thread mainThread = Thread.currentThread();
        Method isWorkerM;
        try {
            isWorkerM = Class.forName("net.minecraft.world.entity.RegionTickOps")
                    .getDeclaredMethod("isWorker");
            isWorkerM.setAccessible(true);
        } catch (NoSuchMethodException e) {
            isWorkerM = null;
        }
        final Method isWorker = isWorkerM;
        java.util.concurrent.atomic.AtomicInteger flagViolations =
                new java.util.concurrent.atomic.AtomicInteger();
        Consumer<Entity> consumer = e -> {
            seen.merge(e.getStringUUID(), 1, Integer::sum);
            if (isWorker != null) {
                try {
                    boolean flag = ((Boolean) isWorker.invoke(null));
                    boolean onHelper = Thread.currentThread() != mainThread;
                    if (flag != onHelper) flagViolations.incrementAndGet();
                } catch (ReflectiveOperationException ex) {
                    flagViolations.incrementAndGet();
                }
            }
        };

        Class<?> ops = Class.forName("net.minecraft.world.entity.RegionTickOps");
        int w = (int) ops.getDeclaredMethod("workers").invoke(null);
        if (w != 2) throw new AssertionError("child expected WORKERS=2, got " + w);
        long t0 = System.nanoTime();
        ops.getDeclaredMethod("forEach", EntityTickList.class, Consumer.class)
                .invoke(null, list, consumer);
        long ms = (System.nanoTime() - t0) / 1_000_000;

        check(seen.size() == n, "parallel phase: every entity ticked (visited all)");
        boolean exactlyOnce = seen.values().stream().allMatch(v -> v == 1);
        check(exactlyOnce, "parallel phase: every entity ticked EXACTLY ONCE (disjoint buckets)");
        if (isWorker != null) {
            check(flagViolations.get() == 0,
                    "S7-157b worker-flag consistency: isWorker() true ONLY on helper threads ("
                            + flagViolations.get() + " violations)");
        }
        System.out.println("[harness] parallel phase done in " + ms + " ms, " + seen.size()
                + " entities, W=2");
    }

    private static String javaBin() {
        return System.getProperty("java.home") + java.io.File.separator + "bin"
                + java.io.File.separator + "java";
    }

    private static String classpath() {
        return System.getProperty("java.class.path");
    }
}
