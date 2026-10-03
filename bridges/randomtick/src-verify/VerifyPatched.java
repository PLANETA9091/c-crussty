import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * F1 runtime verifier gate (S7-112): link-time bytecode verification of the
 * Rust-patched ServerLevel on a REAL HotSpot, no server boot (INJECTS-ONLY).
 *
 * The Rust byte-level tests prove the SHAPE of the swap (11-byte body, CP
 * operand resolution, empty StackMapTable); only the JVM's type-checking
 * verifier can prove the bytes are LEGAL for class major 65. resolveClass()
 * forces linking (verification + preparation) without initialization, so no
 * <clinit> side effects and no world — the same gate the kernel's own
 * ServerLevel passes on every boot.
 *
 * Loader topology: child-first for net.minecraft.server.level.ServerLevel
 * (defined from the PATCHED bytes), everything else parent-first from the
 * kernel jar — one coherent ServerLevel, no dual-class hazards for the
 * verifier's assignability checks.
 *
 * Usage: java VerifyPatched <kernel.jar> <ServerLevel.patched.class>
 * Exit 0 + "VERIFY-OK" = verifier accepted; any VerifyError/ClassFormatError
 * = the patch is illegal and the hook would have failed closed at runtime.
 */
public class VerifyPatched {

    /** Child-first only for the patched target; parent (kernel jar) for all else. */
    static final class PatchedLoader extends URLClassLoader {
        private final String targetName;
        private final byte[] patched;

        PatchedLoader(URL jar, String targetName, byte[] patched) {
            super(new URL[] { jar }, PatchedLoader.class.getClassLoader());
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
                            c = findClass(name); // kernel jar (child)
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
            System.err.println("usage: VerifyPatched <kernel.jar> <ServerLevel.patched.class>");
            System.exit(2);
        }
        URL jar = Path.of(args[0]).toUri().toURL();
        byte[] patched = Files.readAllBytes(Path.of(args[1]));
        String target = "net.minecraft.server.level.ServerLevel";

        Class<?> c = new PatchedLoader(jar, target, patched).loadClass(target, true);
        System.out.println("VERIFY-OK linked=" + c.getName()
                + " bytes=" + patched.length
                + " major=" + ((patched[6] & 0xFF) << 8 | (patched[7] & 0xFF)));
    }
}
