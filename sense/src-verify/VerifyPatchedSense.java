import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * SENSE-PLANE runtime verifier gate (TASK-438-A2): link-time bytecode
 * verification of the Rust-patched ServerEntityGetter on a REAL HotSpot, no
 * server boot (INJECTS-ONLY) — the randomtick/verify_patched.sh pattern
 * applied to the interface default-method body swap (cmp438_sense).
 *
 * The Rust byte-level tests prove the SHAPE of the swap (14-byte body, CP
 * operand resolution, empty StackMapTable); only the JVM's type-checking
 * verifier can prove the bytes are LEGAL for class major 65 — including the
 * invokestatic SenseOps.nearestEntityGate resolution (SenseOps loads from
 * sense/build — the same blob include_bytes! embeds at runtime).
 *
 * Loader topology: child-first for net.minecraft.server.level.ServerEntityGetter
 * (defined from the PATCHED bytes), everything else from the kernel jar +
 * sense/build (child), then the system loader (JDK) — one coherent getter
 * interface, no dual-class hazards for the verifier's assignability checks.
 *
 * Usage: java VerifyPatchedSense <kernel.jar> <ServerEntityGetter.patched.class>
 * Exit 0 + "VERIFY-OK" = verifier accepted; any VerifyError/ClassFormatError
 * = the patch is illegal and the hook would have failed closed at runtime.
 */
public class VerifyPatchedSense {

    /** Child-first for the patched target; child (kernel+sense) for all else. */
    static final class PatchedLoader extends URLClassLoader {
        private final String targetName;
        private final byte[] patched;

        PatchedLoader(URL[] cp, String targetName, byte[] patched) {
            super(cp, PatchedLoader.class.getClassLoader());
            this.targetName = targetName;
            this.patched = patched;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            synchronized (getClassLoadingLock(name)) {
                Class<?> c = findLoadedClass(name);
                if (c == null) {
                    if (name.equals(targetName)) {
                        c = defineClass(name, patched, 0, patched.length);
                    } else {
                        try {
                            c = findClass(name); // kernel jar + sense/build (child)
                        } catch (ClassNotFoundException e) {
                            c = super.loadClass(name, false); // system (JDK classes)
                        }
                    }
                }
                if (resolve) {
                    resolveClass(c); // link = VERIFY now (the actual gate)
                }
                return c;
            }
        }
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 2) {
            System.err.println("usage: VerifyPatchedSense <kernel.jar> <ServerEntityGetter.patched.class>");
            System.exit(2);
        }
        URL kernel = Path.of(args[0]).toUri().toURL();
        URL sense = Path.of("sense/build").toUri().toURL();
        byte[] patched = Files.readAllBytes(Path.of(args[1]));
        String target = "net.minecraft.server.level.ServerEntityGetter";

        Class<?> c = new PatchedLoader(new URL[] { kernel, sense }, target, patched)
                .loadClass(target, true);
        System.out.println("SENSE VERIFY-OK linked=" + c.getName()
                + " bytes=" + patched.length
                + " major=" + ((patched[6] & 0xFF) << 8 | (patched[7] & 0xFF)));
    }
}
