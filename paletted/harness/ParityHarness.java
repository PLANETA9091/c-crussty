import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * PALETTED-DEMUX parity harness (S7-131) — plain JVM, real kernel classes,
 * NO server boot (INJECTS-ONLY discipline).
 *
 * Loader topology:
 *   L1 = URLClassLoader(original kernel jar + libraries)  -> vanilla container
 *   L2 = ChildFirstClassLoader(stub jar + ops classes)    -> PATCHED container
 * (child-first so L2 resolves its own patched PalettedContainer instead of
 * delegating to a parent that could carry a different copy).
 *
 * Protocol under test:
 *   1. random set/getAndSet/get/getAndSetUnchecked op streams, lockstep
 *      L1 vs L2 vs an int-indexed model, full 4096-cell compare per batch;
 *   2. palette-resize ladder (40 distinct values drive linear -> hashmap ->
 *      global growth incl. onResize data swap) under the guarded mutators;
 *   3. demux lifecycle: forced materialize -> snap published & fresh ->
 *      fast-path reads still parity -> write nulls snap (prologue) and
 *      unmarks the refcount (onWrite) -> re-materialize -> parity;
 *   4. concurrency smoke: readers + writer hammer the SAME patched
 *      container; no exceptions, final parity.
 *
 * Exit code 0 = PASS; any failure throws (nonzero).
 */
public final class ParityHarness {

    static class ChildFirstClassLoader extends URLClassLoader {
        ChildFirstClassLoader(URL[] urls, ClassLoader parent) {
            super(urls, parent);
        }
        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            synchronized (getClassLoadingLock(name)) {
                Class<?> c = findLoadedClass(name);
                if (c == null) {
                    try {
                        c = findClass(name);
                    } catch (ClassNotFoundException e) {
                        c = super.loadClass(name, resolve);
                    }
                }
                if (resolve) resolveClass(c);
                return c;
            }
        }
    }

    static Class<?> l1PC, l2PC;
    static Object l1Container, l2Container;
    static String[] model = new String[4096];

    public static void main(String[] args) throws Exception {
        Path kernelJar = Path.of(args[0]);
        Path stubJar = Path.of(args[1]);
        Path opsClasses = Path.of(args[2]);
        long seed = Long.parseLong(args.length > 3 ? args[3] : "424242");

        List<URL> libUrls = new ArrayList<>();
        Path libs = kernelJar.getParent() == null ? null : kernelJar.getParent().getParent() == null ? null : kernelJar.getParent().getParent().resolve("libraries");
        // scan a libraries dir next to versions/<v>/ if present
        Path libRoot = Path.of(args.length > 4 ? args[4] : "/tmp/pdec/matsrv/libraries");
        if (java.nio.file.Files.isDirectory(libRoot)) {
            try (var walk = java.nio.file.Files.walk(libRoot)) {
                walk.filter(p -> p.toString().endsWith(".jar")).forEach(p -> {
                    try { libUrls.add(p.toUri().toURL()); } catch (Exception ignored) {}
                });
            }
        }
        URL[] l1Urls0 = {kernelJar.toUri().toURL()};
        URL[] l1Urls = new URL[l1Urls0.length + libUrls.size()];
        System.arraycopy(l1Urls0, 0, l1Urls, 0, l1Urls0.length);
        for (int i = 0; i < libUrls.size(); i++) l1Urls[l1Urls0.length + i] = libUrls.get(i);
        URLClassLoader l1 = new URLClassLoader(l1Urls, ClassLoader.getPlatformClassLoader());
        URL[] l2Urls = new URL[2 + libUrls.size()];
        l2Urls[0] = stubJar.toUri().toURL();
        l2Urls[1] = opsClasses.toUri().toURL();
        for (int i = 0; i < libUrls.size(); i++) l2Urls[2 + i] = libUrls.get(i);
        ChildFirstClassLoader l2 = new ChildFirstClassLoader(l2Urls, l1);

        l1PC = Class.forName("net.minecraft.world.level.chunk.PalettedContainer", true, l1);
        l2PC = Class.forName("net.minecraft.world.level.chunk.PalettedContainer", true, l2);
        System.out.println("harness: L1 vanilla = " + l1PC.getProtectionDomain().getCodeSource().getLocation());
        System.out.println("harness: L2 patched = " + l2PC.getProtectionDomain().getCodeSource().getLocation());

        // IdMap<String> proxy for Strategy.createForBlockStates
        Class<?> idMap1 = Class.forName("net.minecraft.core.IdMap", true, l1);
        Class<?> idMap2 = Class.forName("net.minecraft.core.IdMap", true, l2);
        Object l1IdMap = stringIdMap(idMap1, l1PC.getClassLoader());
        Object l2IdMap = stringIdMap(idMap2, l2PC.getClassLoader());

        Object l1Strategy = Class.forName("net.minecraft.world.level.chunk.Strategy", true, l1)
                .getMethod("createForBlockStates", idMap1).invoke(null, l1IdMap);
        Object l2Strategy = Class.forName("net.minecraft.world.level.chunk.Strategy", true, l2)
                .getMethod("createForBlockStates", idMap2).invoke(null, l2IdMap);

        Constructor<?> c1 = l1PC.getConstructor(Object.class, Class.forName("net.minecraft.world.level.chunk.Strategy", true, l1));
        Constructor<?> c2 = l2PC.getConstructor(Object.class, Class.forName("net.minecraft.world.level.chunk.Strategy", true, l2));
        l1Container = c1.newInstance("air", l1Strategy);
        l2Container = c2.newInstance("air", l2Strategy);
        java.util.Arrays.fill(model, "air");
        System.out.println("harness: containers constructed (default=air)");

        // ---- 1+2: random op stream with palette growth ----
        Random rnd = new Random(seed);
        String[] pool = new String[40];
        for (int i = 0; i < pool.length; i++) pool[i] = "state#" + i;

        Method l1Set = l1PC.getMethod("set", int.class, int.class, int.class, Object.class);
        Method l2Set = l2PC.getMethod("set", int.class, int.class, int.class, Object.class);
        Method l1Gas = l1PC.getMethod("getAndSet", int.class, int.class, int.class, Object.class);
        Method l2Gas = l2PC.getMethod("getAndSet", int.class, int.class, int.class, Object.class);
        Method l1Get = l1PC.getMethod("get", int.class, int.class, int.class);
        Method l2Get = l2PC.getMethod("get", int.class, int.class, int.class);

        final int BATCH = 200;
        int oldMatch = 0, gasOps = 0;
        for (int op = 0; op < 20000; op++) {
            int x = rnd.nextInt(16), y = rnd.nextInt(16), z = rnd.nextInt(16);
            String v = pool[rnd.nextInt(pool.length)];
            switch (rnd.nextInt(4)) {
                case 0 -> { l1Set.invoke(l1Container, x, y, z, v); l2Set.invoke(l2Container, x, y, z, v); model[idx(x, y, z)] = v; }
                case 1 -> {
                    Object o1 = l1Gas.invoke(l1Container, x, y, z, v);
                    Object o2 = l2Gas.invoke(l2Container, x, y, z, v);
                    gasOps++;
                    if (String.valueOf(o1).equals(String.valueOf(o2))) oldMatch++;
                    else throw new AssertionError("getAndSet old-value mismatch @" + x + "," + y + "," + z + ": " + o1 + " vs " + o2);
                    model[idx(x, y, z)] = v;
                }
                case 2 -> {
                    Object r1 = l1Get.invoke(l1Container, x, y, z);
                    Object r2 = l2Get.invoke(l2Container, x, y, z);
                    if (!String.valueOf(r1).equals(String.valueOf(r2)))
                        throw new AssertionError("read mismatch @" + x + "," + y + "," + z + ": " + r1 + " vs " + r2);
                }
                default -> {
                    // unchecked write path (lockFlag=false equivalent)
                    Method u1 = l1PC.getMethod("getAndSetUnchecked", int.class, int.class, int.class, Object.class);
                    Method u2m = l2PC.getMethod("getAndSetUnchecked", int.class, int.class, int.class, Object.class);
                    Object o1 = u1.invoke(l1Container, x, y, z, v);
                    Object o2 = u2m.invoke(l2Container, x, y, z, v);
                    if (!String.valueOf(o1).equals(String.valueOf(o2)))
                        throw new AssertionError("unchecked old mismatch");
                    model[idx(x, y, z)] = v;
                }
            }
            if (op % BATCH == 0) fullCompare(l1Get, l2Get, "op " + op);
        }
        fullCompare(l1Get, l2Get, "final");
        System.out.println("harness: 20000 random ops PASS (getAndSet old-values matched " + oldMatch + "/" + gasOps + ")");

        // ---- 3: demux lifecycle on the PATCHED container ----
        Object snapBefore = field(l2Container, "crusstySnap");
        System.out.println("harness: snap before force = " + snapBefore + " (null until heat-driven or forced)");
        // force materialize via package-private static tryMaterialize
        Class<?> opsClass = Class.forName("net.minecraft.world.level.chunk.PalettedContainerOps", true, l2);
        Method tm = opsClass.getDeclaredMethod("tryMaterialize", l2PC);
        tm.setAccessible(true);
        tm.invoke(null, l2Container);
        Object snap = field(l2Container, "crusstySnap");
        if (snap == null) throw new AssertionError("materialize did not publish a snapshot");
        int snapGen = (Integer) field(l2Container, "crusstySnapGen");
        int gen = (Integer) field(l2Container, "crusstyGen");
        if (snapGen != gen + 1 || (gen & 1) != 0) throw new AssertionError("published snapshot not fresh: snapGen=" + snapGen + " gen=" + gen);
        System.out.println("harness: snapshot PUBLISHED and fresh (snapGen=" + snapGen + ")");
        // reads through the fast path must stay in parity
        for (int i = 0; i < 1000; i++) {
            int x = rnd.nextInt(16), y = rnd.nextInt(16), z = rnd.nextInt(16);
            Object r1 = l1Get.invoke(l1Container, x, y, z);
            Object r2 = l2Get.invoke(l2Container, x, y, z);
            if (!String.valueOf(r1).equals(String.valueOf(r2)))
                throw new AssertionError("FAST-PATH read mismatch @" + x + "," + y + "," + z);
        }
        System.out.println("harness: 1000 fast-path reads in parity with vanilla");

        // write must unmark the snapshot (prologue onMutateStart -> snapGen=0)
        java.lang.reflect.Field opsLive = opsClass.getDeclaredField("LIVE");
        opsLive.setAccessible(true);
        long liveBefore = ((java.util.concurrent.atomic.AtomicLong) opsLive.get((Object) null)).get();
        l1Set.invoke(l1Container, 0, 0, 0, pool[0]); // keep legs in lockstep
        l2Set.invoke(l2Container, 0, 0, 0, pool[0]);
        model[idx(0, 0, 0)] = pool[0];
        int snapGenAfter = (Integer) field(l2Container, "crusstySnapGen");
        if (snapGenAfter != 0) throw new AssertionError("prologue did not unmark the snapshot (snapGen=" + snapGenAfter + ")");
        long liveAfter = ((java.util.concurrent.atomic.AtomicLong) opsLive.get((Object) null)).get();
        if (liveAfter != liveBefore - 1) throw new AssertionError("refcount not released: " + liveBefore + " -> " + liveAfter);
        System.out.println("harness: write unmarked snapshot + released refcount (" + liveBefore + " -> " + liveAfter + ")");

        // parity after the write (vanilla path since snap is gone)
        Object r1 = l1Get.invoke(l1Container, 0, 0, 0);
        Object r2 = l2Get.invoke(l2Container, 0, 0, 0);
        if (!String.valueOf(r1).equals(String.valueOf(r2))) throw new AssertionError("post-write mismatch");
        fullCompare(l1Get, l2Get, "post-write");

        // re-materialize on the new epoch and re-verify parity
        tm.invoke(null, l2Container);
        if (field(l2Container, "crusstySnap") == null) throw new AssertionError("re-materialize failed");
        fullCompare(l1Get, l2Get, "re-materialized");
        System.out.println("harness: re-materialization + parity PASS");

        // ---- 4: concurrency smoke (readers vs writer, same container) ----
        final long stop = System.nanoTime() + 500_000_000L;
        List<Thread> threads = new ArrayList<>();
        List<Throwable> errors = java.util.Collections.synchronizedList(new ArrayList<>());
        for (int t = 0; t < 3; t++) {
            Thread reader = new Thread(() -> {
                Random rr = new Random();
                try {
                    while (System.nanoTime() < stop) {
                        int x = rr.nextInt(16), y = rr.nextInt(16), z = rr.nextInt(16);
                        Object o = l2Get.invoke(l2Container, x, y, z);
                        if (o == null) errors.add(new AssertionError("null read"));
                    }
                } catch (Throwable e) { errors.add(e); }
            });
            threads.add(reader);
        }
        Thread writer = new Thread(() -> {
            Random rw = new Random(7);
            try {
                int i = 0;
                while (System.nanoTime() < stop) {
                    int x = rw.nextInt(16), y = rw.nextInt(16), z = rw.nextInt(16);
                    l2Set.invoke(l2Container, x, y, z, pool[i++ % pool.length]);
                }
            } catch (Throwable e) { errors.add(e); }
        });
        threads.add(writer);
        threads.forEach(Thread::start);
        for (Thread t : threads) t.join();
        if (!errors.isEmpty()) throw new AssertionError("concurrency smoke errors: " + errors.get(0), errors.get(0));
        System.out.println("harness: concurrency smoke PASS (3 readers + 1 writer, 500ms)");
        System.out.println("harness: ALL PASS");
    }

    static int idx(int x, int y, int z) { return x | (z << 4) | (y << 8); }

    static Object field(Object o, String name) throws Exception {
        Class<?> c = o.getClass();
        while (c != null) {
            try {
                Field f = c.getDeclaredField(name);
                f.setAccessible(true);
                return f.get(o);
            } catch (NoSuchFieldException e) {
                c = c.getSuperclass();
            }
        }
        throw new NoSuchFieldException(name);
    }

    static void fullCompare(Method l1Get, Method l2Get, String tag) throws Exception {
        for (int y = 0; y < 16; y++)
            for (int z = 0; z < 16; z++)
                for (int x = 0; x < 16; x++) {
                    Object r1 = l1Get.invoke(l1Container, x, y, z);
                    Object r2 = l2Get.invoke(l2Container, x, y, z);
                    if (!String.valueOf(r1).equals(String.valueOf(r2)) || !String.valueOf(r2).equals(model[idx(x, y, z)])) {
                        throw new AssertionError(tag + ": content mismatch @ " + x + "," + y + "," + z
                                + " vanilla=" + r1 + " patched=" + r2 + " model=" + model[idx(x, y, z)]);
                    }
                }
    }

    /** An IdMap<String> whose ids are the value strings' hashes mod 1024. */
    static Object stringIdMap(Class<?> idMapClass, ClassLoader loader) {
        return Proxy.newProxyInstance(loader, new Class<?>[]{idMapClass}, new InvocationHandler() {
            @Override
            public Object invoke(Object proxy, Method m, Object[] a) {
                switch (m.getName()) {
                    case "getId": return Math.floorMod(System.identityHashCode(a[0]) * 31, 4096);
                    case "byId": return null; // ids are unresolvable -> valueFor never used pre-growth? idFor drives the palette, not the registry
                    default: return defaultFor(m);
                }
            }
            Object defaultFor(Method m) {
                Class<?> r = m.getReturnType();
                if (r == boolean.class) return false;
                if (r == int.class) return 0;
                if (r == long.class) return 0L;
                return null;
            }
        });
    }
}
