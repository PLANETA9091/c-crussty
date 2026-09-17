import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * F2 runtime verifier gate (S7-114): link-time bytecode verification of the
 * Rust-patched Brain on a REAL HotSpot, no server boot (INJECTS-ONLY).
 *
 * Mirrors VerifyPatched (F1, S7-112) but reproduces the RUNTIME loader
 * topology from src/brainhook.rs: the patched Brain AND the lens helper trio
 * (BrainOps$IdKey, BrainOps$Snapshot, BrainOps) are child-defined into ONE
 * loader — the exact seam the hook wires (nested classes defined first).
 * Everything else is parent-first from the kernel jar, so Brain's links to
 * LivingEntity/ServerLevel/BehaviorControl and BrainOps's links resolve to
 * the same kernel classes the verifier's assignability checks need — one
 * coherent class space, no dual-class hazards.
 *
 * resolve=true forces linking (verification + preparation) without
 * initialization: no <clinit> side effects, no world, no entities — the same
 * gate Brain passes on every boot.
 *
 * Usage: java VerifyBrain <kernel.jar> <Brain.patched.class> <ops-dir>
 * Exit 0 + "VERIFY-OK" = verifier accepted; any VerifyError/ClassFormatError
 * = the patch is illegal and the hook would have failed closed at runtime.
 */
public class VerifyBrain {

    /** Child-first for the patched Brain + helper trio; kernel jar for all else. */
    static final class PatchedLoader extends URLClassLoader {
        private final Map<String, byte[]> child;

        PatchedLoader(URL jar, Map<String, byte[]> child) {
            super(new URL[] { jar }, PatchedLoader.class.getClassLoader());
            this.child = child;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            synchronized (getClassLoadingLock(name)) {
                Class<?> c = findLoadedClass(name);
                if (c == null) {
                    byte[] bytes = child.get(name);
                    if (bytes != null) {
                        c = defineClass(name, bytes, 0, bytes.length);
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
        if (args.length != 3) {
            System.err.println("usage: VerifyBrain <kernel.jar> <Brain.patched.class> <ops-dir>");
            System.exit(2);
        }
        URL jar = Path.of(args[0]).toUri().toURL();
        byte[] brain = Files.readAllBytes(Path.of(args[1]));
        Path ops = Path.of(args[2]);

        Map<String, byte[]> child = new HashMap<>();
        child.put("net.minecraft.world.entity.ai.Brain", brain);
        // Nested classes FIRST — the runtime define order (brainhook.rs).
        child.put("net.minecraft.world.entity.ai.BrainOps$IdKey",
                Files.readAllBytes(ops.resolve("BrainOps$IdKey.class")));
        child.put("net.minecraft.world.entity.ai.BrainOps$Snapshot",
                Files.readAllBytes(ops.resolve("BrainOps$Snapshot.class")));
        byte[] opsBytes = Files.readAllBytes(ops.resolve("BrainOps.class"));
        child.put("net.minecraft.world.entity.ai.BrainOps", opsBytes);

        Class<?> c = new PatchedLoader(jar, child)
                .loadClass("net.minecraft.world.entity.ai.Brain", true);
        System.out.println("VERIFY-OK linked=" + c.getName()
                + " brain=" + brain.length + "B ops=" + opsBytes.length + "B"
                + " major=" + ((brain[6] & 0xFF) << 8 | (brain[7] & 0xFF)));
    }
}
