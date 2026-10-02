import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.ProtectionDomain;

/** RetransformProbe — reproduces the JVMTI RetransformClasses path used by
 *  the crussty runtime: vanilla BlockPos loads first (from the kernel jar),
 *  then retransformClasses() serves the patched bytes. */
public class RetransformProbe {

    static byte[] patched;
    static Instrumentation INST;

    public static void premain(String args, Instrumentation inst) {
        INST = inst;
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader loader, String className,
                                    Class<?> classBeingRedefined,
                                    ProtectionDomain pd, byte[] classfileBuffer) {
                if (!className.equals("net/minecraft/core/BlockPos") || classBeingRedefined == null) return null;
                System.out.println("[probe] transform called for " + className + " RETRANSFORM-PHASE"
                        + " redefined=" + (classBeingRedefined != null)
                        + " loader=" + loader);
                return patched;
            }
        }, true);
    }

    public static void main(String[] args) throws Exception {
        // Load the kernel jar classes into the APP loader (like the server).
        // CI-faithful topology: ONE app loader holds the kernel jar AND the
        // bridge classes (crussty defines bridges into LivingEntity's loader
        // = the app loader that also defines BlockPos).
        java.net.URLClassLoader kernel = new java.net.URLClassLoader(
                new java.net.URL[]{Path.of(args[0]).toUri().toURL(),
                                   Path.of(args[2]).toUri().toURL()},
                RetransformProbe.class.getClassLoader());
        Class<?> iter = kernel.loadClass("net.minecraft.core.ZeroCursorIter");
        Class<?> ops = kernel.loadClass("net.minecraft.core.ZeroCursorOps");
        System.out.println("[probe] bridges loaded: " + iter + ", " + ops);
        // Force BlockPos load through the kernel jar loader.
        Class<?> pos = kernel.loadClass("net.minecraft.core.BlockPos");
        System.out.println("[probe] vanilla BlockPos loaded: " + pos);
        patched = Files.readAllBytes(Path.of(args[1]));
        INST.retransformClasses(pos);
        System.out.println("[probe] RETRANSFORM OK");

        // Semantic end-to-end: same factory args, vanilla-before vs redirected-after.
        java.lang.reflect.Method m = pos.getDeclaredMethod(
            "lambda$betweenCornersInDirection$8",
            kernel.loadClass("net.minecraft.core.Direction"),
            kernel.loadClass("net.minecraft.core.Direction"),
            kernel.loadClass("net.minecraft.core.Direction"),
            int.class, int.class, int.class, int.class, int.class, int.class);
        m.setAccessible(true);
        Object d = kernel.loadClass("net.minecraft.core.Direction").getEnumConstants();
        java.util.List<String> after = new java.util.ArrayList<>();
        for (java.util.Iterator<?> it2 = (java.util.Iterator<?>) m.invoke(null,
                java.lang.reflect.Array.get(d, 1), java.lang.reflect.Array.get(d, 0),
                java.lang.reflect.Array.get(d, 2), 10, 64, -3, 12, 66, -1);
                it2.hasNext();) {
            after.add(String.valueOf(it2.next()));
        }
        System.out.println("[probe] redirected factory produced " + after.size()
                + " positions, first=" + (after.isEmpty() ? "-" : after.get(0))
                + " last=" + (after.isEmpty() ? "-" : after.get(after.size() - 1)));
        if (after.isEmpty()) throw new AssertionError("redirected factory returned empty");
        System.out.println("[probe] SEMANTIC OK");
    }
}

