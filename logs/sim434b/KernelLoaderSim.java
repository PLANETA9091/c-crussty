import java.nio.file.*;

/**
 * TASK-434-B round-3 NCDFE local reproduction + fix validation.
 *
 * Simulates the kernel-loader bridge-define contract: InsideSnapOps* classes
 * are defined from RAW BYTES into an isolated loader (no parent delegation for
 * net.minecraft.world.entity.InsideSnapOps*), everything else delegates to the
 * parent (paper-api classpath).
 *
 * Scenario A (BROKEN = run 35902792520): $Lane NOT defined -> Class.forName(init=true)
 *   must throw ExceptionInInitializerError caused by NCDFE [Lnet/.../InsideSnapOps$Lane;
 * Scenario B (FIXED): all three classes defined -> clinit must succeed.
 */
public class KernelLoaderSim {
    static final String OPS = "net.minecraft.world.entity.InsideSnapOps";
    static final String SNAP = "net.minecraft.world.entity.InsideSnapOps$Snap";
    static final String LANE = "net.minecraft.world.entity.InsideSnapOps$Lane";

    static class KernelLoader extends ClassLoader {
        final Path buildNested; // entityinside/build/net/minecraft/world/entity
        KernelLoader(ClassLoader parent, Path buildNested) {
            super(parent);
            this.buildNested = buildNested;
        }
        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            synchronized (getClassLoadingLock(name)) {
                Class<?> c = findLoadedClass(name);
                if (c == null) {
                    if (name.startsWith("net.minecraft.world.entity.InsideSnapOps")) {
                        Path p = buildNested.resolve(name.replace('.', '/') + ".class");
                        if (!Files.exists(p)) {
                            // mirrors the real kernel loader: a never-defined
                            // nested class is a plain CNFE(name), no decoration
                            throw new ClassNotFoundException(name);
                        }
                        byte[] b;
                        try {
                            b = Files.readAllBytes(p);
                        } catch (java.io.IOException e) {
                            throw new ClassNotFoundException(name, e);
                        }
                        c = defineClass(name, b, 0, b.length);
                    } else {
                        c = getParent().loadClass(name);
                    }
                }
                if (resolve) resolveClass(c);
                return c;
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Path oldBuild = Paths.get(args[0]); // 08547cb blob (withInitial indy), $Lane absent
        Path newBuild = Paths.get(args[1]); // fixed blob, $Lane present
        ClassLoader parent = KernelLoaderSim.class.getClassLoader();

        // ---- Scenario A: round-3 BROKEN bytes ($Lane absent from build dir =
        // never defined in the kernel loader; clinit carries the withInitial indy) ----
        KernelLoader broken = new KernelLoader(parent, oldBuild);
        try {
            Class.forName(OPS, true, broken);
            System.out.println("A: UNEXPECTED SUCCESS (root-cause hypothesis REFUTED)");
            System.exit(2);
        } catch (Throwable t) {
            // accept EITHER decoration of the same mechanism: NCDFE naming the
            // array descriptor (real kernel loader) or NCDFE->CNFE($Lane) chain
            // (java.lang.ClassLoader sim). Both = clinit poison via unresolvable
            // $Lane from the defining loader.
            StringBuilder chain = new StringBuilder();
            boolean ncufe = false;
            for (Throwable c = t; c != null; c = c.getCause()) {
                chain.append(c.getClass().getName()).append(": ").append(c.getMessage()).append(" | ");
                if (c instanceof NoClassDefFoundError) ncufe = true;
            }
            String msg = chain.toString();
            boolean hit = ncufe && msg.contains("InsideSnapOps$Lane")
                    && (t instanceof ExceptionInInitializerError || t instanceof NoClassDefFoundError);
            System.out.println("A: " + t.getClass().getSimpleName() + " / chain " + msg);
            System.out.println(hit ? "A: REPRODUCED run-35902792520 clinit-poison (NCDFE via unresolvable $Lane) => root-cause CONFIRMED"
                                   : "A: DIFFERENT failure => root-cause INCOMPLETE");
            if (!hit) System.exit(3);
        }

        // ---- Scenario B: FIXED state (all three defined, de-indy'd clinit) ----
        KernelLoader fixed = new KernelLoader(parent, newBuild);
        try {
            Class<?> ops = Class.forName(OPS, true, fixed);
            System.out.println("B: " + OPS + " initialized OK (clinit green)");
            // selfTest reaches its natives only in-engine; locally assert the
            // method is resolvable and Lane is loadable from the same loader:
            ops.getDeclaredMethod("selfTest");
            Class<?> lane = Class.forName(LANE, true, fixed);
            System.out.println("B: selfTest method resolved; $Lane loadable: " + lane.getName());
            System.out.println("B: LOCAL SELFTEST-RESOLUTION GATE GREEN");
        } catch (Throwable t) {
            Throwable cause = t;
            while (cause.getCause() != null) cause = cause.getCause();
            System.out.println("B: FAILED " + t.getClass().getSimpleName() + " / root " + cause);
            System.exit(4);
        }
    }
}
