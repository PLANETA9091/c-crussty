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
        final String opts = (options == null) ? null : options.trim();
        if (opts == null || opts.isEmpty()) {
            System.err.println("[prewarm] no options -> dormant no-op (arm A)");
            return;
        }
        if (opts.equalsIgnoreCase("b1")) {
            armB1(inst); // TASK-99: DataFixer build offload (TASK-98 audit B1 OFFLOADABLE-EARLY)
            return;
        }
        final String listPath = opts;
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

    /**
     * TASK-99 B1 mode: trigger = first net.minecraft.* load (SharedConstants at
     * Main:3); worker polls until version constants are set, then forces
     * DataFixers.&lt;clinit&gt; (pure function of dataVersion int — TASK-98 audit:
     * 406-class purity scan, zero registry/file/IO deps) on the worker thread
     * while the main thread runs B2 registry bootstrap. Main joins the already
     * built DATA_FIXER at Main:623 (LevelStorageSource.createDefault).
     */
    static void armB1(java.lang.instrument.Instrumentation inst) {
        final long t0 = System.nanoTime();
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader loader, String className, Class<?> beingDefined,
                                    ProtectionDomain pd, byte[] cb) {
                if (!spawned && className != null && loader != null
                        && className.startsWith("net/minecraft/")) {
                    final ClassLoader cl = loader;
                    spawned = true;
                    Thread w = new Thread(() -> {
                        // wait for SharedConstants.tryDetectVersion() on main (idempotent static)
                        boolean ready = false;
                        for (int i = 0; i < 6000; i++) {
                            try {
                                if (Class.forName("net.minecraft.SharedConstants", false, cl)
                                        .getMethod("getCurrentVersion").invoke(null) != null) {
                                    ready = true;
                                    break;
                                }
                            } catch (ClassNotFoundException nf) {
                                // not loaded yet — retry
                            } catch (java.lang.reflect.InvocationTargetException ite) {
                                // getCurrentVersion() throws IllegalStateException("Game version not set")
                                // until Main:3 tryDetectVersion() lands — that IS the not-ready signal
                                if (!(ite.getCause() instanceof IllegalStateException)) {
                                    System.err.println("[prewarm] b1: poll aborted by " + ite.getCause());
                                    break;
                                }
                            } catch (Throwable t2) {
                                System.err.println("[prewarm] b1: poll aborted by " + t2);
                                break; // unexpected reflection issue — give up quietly
                            }
                            try { Thread.sleep(10); } catch (InterruptedException ie) { return; }
                        }
                        if (!ready) {
                            long ms = (System.nanoTime() - t0) / 1_000_000L;
                            System.err.println("[prewarm] b1: version never set after " + ms + "ms -> no-op");
                            return;
                        }
                        try {
                            Class.forName("net.minecraft.util.datafix.DataFixers", true, cl);
                            long ms = (System.nanoTime() - t0) / 1_000_000L;
                            System.err.println("[prewarm] b1: DataFixers built on worker in " + ms + "ms");
                        } catch (Throwable t3) {
                            System.err.println("[prewarm] b1: DataFixers init failed (non-fatal): " + t3);
                        }
                    }, "crussty-prewarm-b1");
                    w.setDaemon(true);
                    w.setPriority(Thread.MIN_PRIORITY + 1);
                    w.start();
                    System.err.println("[prewarm] B1 armed via loader-capture on " + className);
                }
                return null; // observation only
            }
        });
        System.err.println("[prewarm] ARM-B1 armed: DataFixer build offload pending first net.minecraft load");
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
