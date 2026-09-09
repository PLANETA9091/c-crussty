import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.*;

public class BatchOpsProbe {
    public static void main(String[] args) throws Exception {
        Path build = Paths.get(args[0]);
        URL[] urls = { build.toUri().toURL() };
        try (URLClassLoader l = new URLClassLoader(urls, ClassLoader.getPlatformClassLoader())) {
            Class<?> ops = Class.forName("net.minecraft.world.level.levelgen.synth.ImprovedNoiseNativeOps", true, l);
            System.out.println("Ops loaded: " + ops);
            Class<?> batch = Class.forName("net.minecraft.world.level.levelgen.synth.ImprovedNoiseBatchOps", true, l);
            System.out.println("BatchOps loaded: " + batch);
            Object rc = batch.getMethod("selfTestFlush").invoke(null);
            System.out.println("selfTestFlush rc = " + rc + " (expect -101 = bridge absent/degraded, or 131087)");
            System.out.println("flushes = " + batch.getMethod("flushes").invoke(null));
        }
    }
}
