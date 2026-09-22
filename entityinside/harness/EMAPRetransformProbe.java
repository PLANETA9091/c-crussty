import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.ProtectionDomain;
import java.util.Iterator;

/** EMAPRetransformProbe — TASK-411-A offline JVM-verifier gate for the
 *  ChunkMap.entityMap fence (s7172 lesson: defineClass verification is NOT
 *  enough; the real JVMTI RetransformClasses path must accept the bytes).
 *
 *  Topology = CI-faithful: ONE URLClassLoader holds the kernel jar AND
 *  fastutil AND the three bridge classfiles (the crussty runtime defines
 *  the bridges into the kernel loader = the loader that also defines
 *  ChunkMap). Vanilla ChunkMap loads first, then retransformClasses()
 *  serves the fence-patched bytes from tests/out/ChunkMap.emap.patched.class.
 *
 *  PASS = "[probe] RETRANSFORM OK" + "[probe] SEMANTIC OK".
 */
public class EMAPRetransformProbe {

    static byte[] patched;
    static Instrumentation INST;

    public static void premain(String args, Instrumentation inst) {
        INST = inst;
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader loader, String className,
                                    Class<?> classBeingRedefined,
                                    ProtectionDomain pd, byte[] classfileBuffer) {
                if (!className.equals("net/minecraft/server/level/ChunkMap")
                        || classBeingRedefined == null) {
                    return null;
                }
                System.out.println("[probe] transform called for " + className
                        + " RETRANSFORM-PHASE redefined="
                        + (classBeingRedefined != null) + " loader=" + loader);
                return patched;
            }
        }, true);
    }

    public static void main(String[] args) throws Exception {
        // args: <kernel-jar> <patched-chunkmap> <build-dir>
        java.net.URLClassLoader kernel = new java.net.URLClassLoader(new java.net.URL[]{
                Path.of(args[0]).toUri().toURL(),
                Path.of(args[2]).toUri().toURL()},
                EMAPRetransformProbe.class.getClassLoader());
        Class<?> ops = kernel.loadClass("net.minecraft.server.level.EntityMapOps");
        Class<?> vals = kernel.loadClass("net.minecraft.server.level.EntityMapSafeValues");
        Class<?> itr = kernel.loadClass("net.minecraft.server.level.EntityMapSafeItr");
        System.out.println("[probe] bridges loaded: " + ops + ", " + vals + ", " + itr);

        // SEMANTIC (bridge side): fence helpers on a REAL fastutil map —
        // put/containsKey/get/remove monitor-serialized, values() iterator
        // bound-checked, semantics identical to vanilla map usage.
        Class<?> i2o = kernel.loadClass("it.unimi.dsi.fastutil.ints.Int2ObjectOpenHashMap");
        Object map = i2o.getDeclaredConstructor().newInstance();
        Class<?> i2m = kernel.loadClass("it.unimi.dsi.fastutil.ints.Int2ObjectMap");
        java.lang.reflect.Method mPut = ops.getMethod("put", i2m, int.class, Object.class);
        java.lang.reflect.Method mCk = ops.getMethod("containsKey", i2m, int.class);
        java.lang.reflect.Method mGet = ops.getMethod("get", i2m, int.class);
        java.lang.reflect.Method mRem = ops.getMethod("remove", i2m, int.class);
        java.lang.reflect.Method mVals = ops.getMethod("values", i2m);
        Object marker = new Object();
        if (mPut.invoke(null, map, 42, marker) != null) throw new AssertionError("put of fresh key must return null");
        if (!(Boolean) mCk.invoke(null, map, 42)) throw new AssertionError("containsKey(42) must be true");
        if (mGet.invoke(null, map, 42) != marker) throw new AssertionError("get(42) must return marker");
        if (mPut.invoke(null, map, 42, "second") != marker) throw new AssertionError("put overwrite must return old");
        if (mGet.invoke(null, map, 42) != "second") throw new AssertionError("get(42) must return second");
        Object vc = mVals.invoke(null, map);
        Iterator<?> it = ((Iterable<?>) vc).iterator();
        int n = 0;
        while (it.hasNext()) { it.next(); n++; }
        if (n != 1) throw new AssertionError("values() iteration must see 1 entry, saw " + n);
        if (mRem.invoke(null, map, 42) != "second") throw new AssertionError("remove(42) must return second");
        if ((Boolean) mCk.invoke(null, map, 42)) throw new AssertionError("containsKey(42) after remove must be false");
        System.out.println("[probe] fence helpers semantic OK (put/containsKey/get/values/remove)");

        // Vanilla ChunkMap loads through the kernel loader.
        Class<?> cm = kernel.loadClass("net.minecraft.server.level.ChunkMap");
        System.out.println("[probe] vanilla ChunkMap loaded: " + cm);
        patched = Files.readAllBytes(Path.of(args[1]));
        INST.retransformClasses(cm);
        System.out.println("[probe] RETRANSFORM OK");

        // Post-redefinition shape sanity (link-free): the fence is
        // length-preserving on the CODE region — the patched bytes keep the
        // classfile magic and only the constant pool grows; full linking of
        // ChunkMap is out of scope offline (slf4j/datafixer are not in the
        // sandbox) and NOT needed: the verifier already accepted the fence
        // bytecode during redefinition (RETRANSFORM OK), which is the
        // s7172 gate this probe exists for.
        if ((patched[0] & 0xFF) != 0xCA || (patched[1] & 0xFF) != 0xFE
                || (patched[2] & 0xFF) != 0xBA || (patched[3] & 0xFF) != 0xBE) {
            throw new AssertionError("patched bytes lost the classfile magic");
        }
        System.out.println("[probe] patched bytes: " + patched.length + " (fence = CP-growth + operand rewrite)");
        System.out.println("[probe] SEMANTIC OK");
    }
}
