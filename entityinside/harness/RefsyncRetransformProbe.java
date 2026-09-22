import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.ProtectionDomain;

/** RefsyncRetransformProbe — TASK-412-A offline JVM-verifier gate for the
 *  ReferenceList mutator fence (s7172 lesson: defineClass verification is
 *  NOT enough; the real JVMTI RetransformClasses path must accept the bytes).
 *
 *  Topology = CI-faithful: ONE URLClassLoader holds the kernel jar AND the
 *  EntityMapOps bridge (the crussty runtime defines the bridge into the
 *  kernel loader = the loader that also defines the censused kernel classes).
 *  Vanilla kernel classes load first, then retransformClasses() serves the
 *  refsync-patched bytes from tests/out/*.refsync.patched.class.
 *
 *  PASS = "[probe] RETRANSFORM OK" x3 + "[probe] REF SAFE (races done)".
 */
public class RefsyncRetransformProbe {

    static Instrumentation INST;
    static final java.util.Map<String, byte[]> PATCHED = new java.util.HashMap<>();

    public static void premain(String args, Instrumentation inst) {
        INST = inst;
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader loader, String className,
                                    Class<?> classBeingRedefined,
                                    ProtectionDomain pd, byte[] classfileBuffer) {
                byte[] p = PATCHED.get(className);
                if (p == null || classBeingRedefined == null) {
                    return null;
                }
                System.out.println("[probe] transform " + className
                    + " redefined=" + (classBeingRedefined != null)
                    + " " + classfileBuffer.length + " -> " + p.length);
                return p;
            }
        }, true);
    }

    /** Race the fence: 4 ensure-add threads + 4 remove threads on ONE shared
     *  ReferenceList through the monitor-serialized helpers. Without the
     *  fence this desyncs the Reference2IntOpenHashMap table (infinite
     *  probe / AIOOBE -1); with it the list must end structurally
     *  consistent (map size == list count) and no thread may see an
     *  exception. NOTE: "every ensured element present at the end" is NOT
     *  assertable — a remover may remove an element after its last
     *  re-add; the fence guarantees linearizability, not liveness of
     *  elements against concurrent removers. */
    static void hammer(String name, Object list, Object[] remElems,
                       Method mAdd, Method mRem, Method mCk,
                       java.util.concurrent.atomic.AtomicBoolean failed) throws Exception {
        final int OPS = 100_000;
        Thread[] ts = new Thread[8];
        for (int t = 0; t < 4; t++) {
            final int ft = t;
            final Object elem = new Object() { public String toString() { return "E" + ft; } };
            ts[t] = new Thread(() -> {
                try {
                    for (int i = 0; i < OPS; i++) {
                        if (!(Boolean) mCk.invoke(null, list, elem)) {
                            mAdd.invoke(null, list, elem);
                        }
                    }
                } catch (Throwable e) { failed.set(true); throw new RuntimeException(e); }
            }, name + "-add-" + t);
        }
        for (int t = 0; t < 4; t++) {
            final Object elem = remElems[t];
            ts[4 + t] = new Thread(() -> {
                try {
                    for (int i = 0; i < OPS; i++) {
                        mRem.invoke(null, list, elem);
                    }
                } catch (Throwable e) { failed.set(true); throw new RuntimeException(e); }
            }, name + "-rem-" + t);
        }
        for (Thread th : ts) th.start();
        for (Thread th : ts) th.join();
    }

    public static void main(String[] args) throws Exception {
        // args: <kernel-jar> <build-dir> <out-dir>
        java.net.URLClassLoader kernel = new java.net.URLClassLoader(new java.net.URL[]{
                Path.of(args[0]).toUri().toURL(),
                Path.of(args[1]).toUri().toURL(),
                // fastutil resolves at runtime from the server libs dir; the
                // materialized kernel jar does not bundle it.
                Path.of("/home/z/tools/fastutil.jar").toUri().toURL()},
                RefsyncRetransformProbe.class.getClassLoader());
        Class<?> ops = kernel.loadClass("net.minecraft.server.level.EntityMapOps");
        System.out.println("[probe] bridge loaded: " + ops);

        // SEMANTIC (bridge side): refList* helpers on a REAL fastutil-backed
        // ReferenceList from the kernel jar — add/contains/remove semantics
        // identical to vanilla ReferenceList usage.
        Class<?> rlClass = kernel.loadClass("ca.spottedleaf.moonrise.common.list.ReferenceList");
        java.lang.reflect.Constructor<?> ctor = rlClass.getDeclaredConstructor();
        ctor.setAccessible(true);
        Object list = ctor.newInstance();
        Object a = new Object(), b = new Object();
        Method mAdd = ops.getMethod("refListAdd", rlClass, Object.class);
        Method mRem = ops.getMethod("refListRemove", rlClass, Object.class);
        Method mCk = ops.getMethod("refListContains", rlClass, Object.class);
        if (!(Boolean) mAdd.invoke(null, list, a)) throw new AssertionError("add(a) fresh must be true");
        if ((Boolean) mAdd.invoke(null, list, a)) throw new AssertionError("add(a) twice must be false");
        if (!(Boolean) mCk.invoke(null, list, a)) throw new AssertionError("contains(a) must be true");
        if (!(Boolean) mAdd.invoke(null, list, b)) throw new AssertionError("add(b) fresh must be true");
        if (!(Boolean) mRem.invoke(null, list, a)) throw new AssertionError("remove(a) must be true");
        if ((Boolean) mRem.invoke(null, list, a)) throw new AssertionError("remove(a) twice must be false");
        if ((Boolean) mCk.invoke(null, list, a)) throw new AssertionError("contains(a) after remove must be false");
        if (!(Boolean) mCk.invoke(null, list, b)) throw new AssertionError("contains(b) must survive");
        Method mSize = rlClass.getMethod("size");
        if ((Integer) mSize.invoke(list) != 1) throw new AssertionError("size after add a,b remove a must be 1");
        System.out.println("[probe] refsync helpers semantic OK (add/contains/remove parity)");

        // Fence-vs-storm: 4 threads ensure-add(e), 4 threads remove(e), 100k
        // ops each, on ONE list. Success = terminates without AIOOBE/hang,
        // no thread exceptions, and the post-quiescent STRUCTURE agrees
        // (referenceToIndex.size() == count) — the invariant whose violation
        // is the k5b corruption (infinite probe / Index -1 downscan).
        Object[] seeded = new Object[4];
        java.util.concurrent.atomic.AtomicBoolean failed =
                new java.util.concurrent.atomic.AtomicBoolean(false);
        for (int t = 0; t < 4; t++) {
            final int ft = t;
            seeded[t] = new Object() { public String toString() { return "R" + ft; } };
            mAdd.invoke(null, list, seeded[t]);
        }
        long t0 = System.nanoTime();
        hammer("refsync", list, seeded, mAdd, mRem, mCk, failed);
        long ms = (System.nanoTime() - t0) / 1_000_000;
        if (failed.get()) throw new AssertionError("a hammer thread hit an exception (fence leak?)");
        java.lang.reflect.Field fMap = rlClass.getDeclaredField("referenceToIndex");
        fMap.setAccessible(true);
        Object innerMap = fMap.get(list);
        Method mMapSize = innerMap.getClass().getMethod("size");
        int mapSize = (Integer) mMapSize.invoke(innerMap);
        int listCount = (Integer) mSize.invoke(list);
        if (mapSize != listCount)
            throw new AssertionError("STRUCTURAL DESYNC: map size " + mapSize + " != list count " + listCount);
        System.out.println("[probe] REF SAFE (races done in " + ms + " ms, map=" + mapSize + " list=" + listCount + ")");

        // RETRANSFORM (verifier gate): three censused kernel classes through
        // the real JVMTI redefinition path. Patched bytes are the flat dump
        // files of classfile::dump_refsync_patched_for_verifier.
        for (String[] spec : new String[][]{
                {"ca.spottedleaf.moonrise.paper.util.BaseChunkSystemHooks",
                 "ca/spottedleaf/moonrise/paper/util/BaseChunkSystemHooks",
                 "BaseChunkSystemHooks.refsync.patched.class"},
                {"ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk",
                 "ca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk",
                 "NearbyPlayers$TrackedChunk.refsync.patched.class"},
                {"io.papermc.paper.threadedregions.EntityScheduler$EntitySchedulerTickList",
                 "io/papermc/paper/threadedregions/EntityScheduler$EntitySchedulerTickList",
                 "EntityScheduler$EntitySchedulerTickList.refsync.patched.class"},
        }) {
            Class<?> k = kernel.loadClass(spec[0]);
            PATCHED.put(spec[1], Files.readAllBytes(Path.of(args[2], spec[2])));
            INST.retransformClasses(k);
            System.out.println("[probe] RETRANSFORM OK " + spec[0]);
        }
        System.out.println("[probe] SEMANTIC OK");
    }
}
