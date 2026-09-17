import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * F3 runtime verifier gate (S7-116): link-time bytecode verification of the
 * Rust-patched LevelTicks + ServerLevel on a REAL HotSpot, no server boot
 * (INJECTS-ONLY).
 *
 * Mirrors VerifyBrain (F2, S7-114) but reproduces the RUNTIME loader
 * topology from src/tickhook.rs: the patched LevelTicks (runCollectedTicks
 * swap), the COMPOSED ServerLevel (F1 optimiseRandomTick + F3 tickBlock —
 * the exact image the hook chain produces), and the lens helper TickBlockOps
 * are child-defined into ONE loader. Everything else is parent-first from
 * the kernel jar, so all kernel links (BlockPos/Block/moonrise
 * ChunkSystemMinecraftServer/CraftBlockState) resolve to the same kernel
 * classes the verifier's assignability checks need — one coherent class
 * space, no dual-class hazards.
 *
 * resolve=true forces linking (verification + preparation) without
 * initialization: no <clinit> side effects (Blocks/bootstrap are NOT
 * touched), no world, no ticks — the same gate all three classes pass on
 * every boot.
 *
 * Usage: java VerifyF3 <kernel.jar> <LevelTicks.patched.class>
 *                      <ServerLevel.F1F3.class> <TickBlockOps.class>
 * Exit 0 + "VERIFY-OK" = verifier accepted; any VerifyError/ClassFormatError
 * = the patch is illegal and the hook would have failed closed at runtime.
 */
public class VerifyF3 {

    /** Child-first for the two patched targets + helper; kernel jar for all else. */
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
        if (args.length != 4) {
            System.err.println("usage: VerifyF3 <kernel.jar> <LevelTicks.patched> <ServerLevel.F1F3> <TickBlockOps>");
            System.exit(2);
        }
        URL jar = Path.of(args[0]).toUri().toURL();
        byte[] levelTicks = Files.readAllBytes(Path.of(args[1]));
        byte[] serverLevel = Files.readAllBytes(Path.of(args[2]));
        byte[] ops = Files.readAllBytes(Path.of(args[3]));

        Map<String, byte[]> child = new HashMap<>();
        child.put("net.minecraft.world.ticks.LevelTicks", levelTicks);
        child.put("net.minecraft.server.level.ServerLevel", serverLevel);
        child.put("net.minecraft.server.level.TickBlockOps", ops);

        PatchedLoader l = new PatchedLoader(jar, child);
        // Helper first: its kernel links (LevelTicks/ServerLevel/moonrise)
        // resolve parent-first while it is being verified — mirrors the
        // define_class order in tickhook.rs (helper defined BEFORE READY).
        Class<?> o = l.loadClass("net.minecraft.server.level.TickBlockOps", true);
        Class<?> t = l.loadClass("net.minecraft.world.ticks.LevelTicks", true);
        Class<?> s = l.loadClass("net.minecraft.server.level.ServerLevel", true);
        System.out.println("VERIFY-OK linked=" + o.getName() + "," + t.getName() + "," + s.getName()
                + " levelTicks=" + levelTicks.length + "B serverLevel(F1F3)=" + serverLevel.length
                + "B ops=" + ops.length + "B"
                + " major=" + ((levelTicks[6] & 0xFF) << 8 | (levelTicks[7] & 0xFF)));
    }
}
