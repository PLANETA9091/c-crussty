import java.io.BufferedReader;
import java.io.FileReader;
import java.lang.instrument.ClassFileTransformer;
import java.security.ProtectionDomain;
import java.util.ArrayList;
import java.util.List;

/**
 * TASK-97 (S7-40): parallel &lt;clinit&gt; prewarm spike — R2-framework first brick.
 * v2 design (after v1 measured ok=0: library classes live on paperclip's CHILD
 * URLClassLoader in the default direct-jar topology, invisible to the system
 * loader): a ClassFileTransformer captures the ACTUAL loader of the first
 * target-family class load, then spawns MIN_PRIORITY daemon workers doing
 * Class.forName(name, true, capturedLoader) — loader-agnostic, fires exactly
 * when the registry/bootstrap window starts touching these families.
 *
 * -javaagent:prewarm_agent.jar=/path/to/tierS.lst -> arm B (active)
 * -javaagent:prewarm_agent.jar=                   -> arm A (dormant no-op)
 * Identical flags/classpath/agent-count in both arms: paired delta isolates
 * the prewarm variable (baseline-drift law: within-session ABBA only).
 * Default OFF. Never touches server files, configs, or gameplay values.
 */
public class PrewarmAgent {

    static volatile boolean spawned = false;

    public static void premain(String options, java.lang.instrument.Instrumentation inst) {
        final String listPath = (options == null || options.trim().isEmpty()) ? null : options.trim();
        if (listPath == null) {
            System.err.println("[prewarm] no list -> dormant no-op (arm A)");
            return;
        }
        final List<String> classes = new ArrayList<>();
        final List<String> prefixes = new ArrayList<>();
        try (BufferedReader br = new BufferedReader(new FileReader(listPath))) {
            String line;
            while ((line = br.readLine()) != null) {
                line = line.trim();
                if (line.isEmpty() || line.startsWith("#")) continue;
                classes.add(line);
                int dot = line.lastIndexOf('.');
                if (dot > 0) prefixes.add(line.substring(0, dot + 1));
            }
        } catch (Throwable t) {
            System.err.println("[prewarm] list unreadable -> dormant: " + t);
            return;
        }
        if (classes.isEmpty()) {
            System.err.println("[prewarm] empty list -> dormant");
            return;
        }
        final long t0 = System.nanoTime();
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader loader, String className, Class<?> beingDefined,
                                    ProtectionDomain pd, byte[] cb) {
                if (!spawned && className != null && loader != null) {
                    String cn = className.replace('/', '.');
                    for (String p : prefixes) {
                        if (cn.startsWith(p)) {
                            ClassLoader cl = loader;
                            spawned = true;
                            spawn(cl, classes, t0);
                            break;
                        }
                    }
                }
                return null; // observation only, zero class modification
            }
        });
        System.err.println("[prewarm] ARM-B armed: " + classes.size()
                + " classes, trigger on first target-family load");
    }

    static void spawn(final ClassLoader loader, final List<String> classes, final long t0) {
        final int workers = Math.max(1, Integer.getInteger("prewarm.workers", 2));
        for (int w = 0; w < workers; w++) {
            final int wid = w;
            Thread th = new Thread(() -> {
                int ok = 0, fail = 0;
                for (int i = wid; i < classes.size(); i += workers) {
                    try {
                        Class.forName(classes.get(i), true, loader);
                        ok++;
                    } catch (Throwable skip) {
                        fail++; // tier-S contamination attempt recorded, never fatal
                    }
                }
                long ms = (System.nanoTime() - t0) / 1_000_000L;
                System.err.println("[prewarm] worker" + wid + " loader=" + loader + " ok="
                        + ok + " fail=" + fail + " in " + ms + "ms");
            }, "crussty-prewarm-" + w);
            th.setDaemon(true);
            th.setPriority(Thread.MIN_PRIORITY + 1); // yield to main-thread construction when contended
            th.start();
        }
        System.err.println("[prewarm] ARM-B active via loader-capture: " + classes.size()
                + " classes, " + workers + " workers");
    }
}
