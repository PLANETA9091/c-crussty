import java.lang.reflect.Method;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Bootstrap for the area-map headless smoke test (stays on the app
 * classpath). Builds a child loader whose parent is the PLATFORM loader and
 * which
 *   - defines ca.spottedleaf.moonrise.common.misc.SingleUserAreaMap from the
 *     PATCHED classfile bytes produced by patch_tool.py (byte-identical to
 *     the shipped Rust patcher output),
 *   - serves everything else from two directories:
 *       build/&lt;variant&gt; : shipped SingleUserAreaMapOps{,$Scratch,$1} classes
 *                            + the variant's PaperNativeAreaMap/SmokeProbe
 *       build/harness      : SmokeMain + RecordingMap + NaiveDiff,
 * then runs areamapsmoke.SmokeMain inside that loader.
 *
 * Usage: java -cp build/boot Boot &lt;smokeDir&gt; &lt;variantDirName&gt;
 *   e.g. Boot tests/area_map_smoke misc        (variant A: counting stub)
 *        Boot tests/area_map_smoke misc_real   (variant B: real .so)
 */
public final class Boot {
    private Boot() {}

    public static void main(String[] argv) throws Exception {
        Path base = Paths.get(argv[0]).toAbsolutePath();
        Path patched = base.resolve("build").resolve("patched_SingleUserAreaMap.class");
        String variantDir = argv[1];
        byte[] patchedBytes = Files.readAllBytes(patched);
        System.out.println("boot: defining patched SingleUserAreaMap from " + patched
                + " (" + patchedBytes.length + " bytes)");

        URL[] urls = {
                base.resolve("build").resolve(variantDir).toUri().toURL(),
                base.resolve("build").resolve("harness").toUri().toURL(),
        };
        URLClassLoader loader = new URLClassLoader("area-map-smoke", urls, ClassLoader.getPlatformClassLoader()) {
            @Override
            protected Class<?> findClass(String name) throws ClassNotFoundException {
                if (name.equals("ca.spottedleaf.moonrise.common.misc.SingleUserAreaMap")) {
                    return defineClass(name, patchedBytes, 0, patchedBytes.length);
                }
                return super.findClass(name);
            }
        };

        Class<?> main = loader.loadClass("areamapsmoke.SmokeMain");
        Method m = main.getMethod("main", String[].class);
        m.invoke(null, (Object) new String[0]);
        // SmokeMain calls System.exit; if it ever returns cleanly, exit 0.
    }
}
