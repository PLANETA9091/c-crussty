import java.util.Arrays;

/**
 * TASK-13 wrapper for the P500 harness — waits for the CRUSSTY module to
 * finish its registration-time surface injection before the bench resolves
 * any bridge class.
 *
 * Why this exists: the kernel-policy remap (CRUSSTY_KERNEL_PREF=old,
 * src/kernel_policy.rs::registration_fallback) is a REGISTRATION-time
 * binding — it happens inside lib.rs::define_and_register when the CRUSSTY
 * runtime agent defines its bridge classes into the BOOTSTRAP loader and
 * RegisterNatives them. Class loading delegates parent-first, so whichever
 * copy of a bridge class exists first wins: if the bench resolves a bridge
 * stub before the injection loop finishes, the app-loader copy
 * (bench/p500/java/*.java stubs, plain dlsym binding) is used and the remap
 * never gets a chance to apply.
 *
 * Enabled with -Dp500.kprefwait=true (agent-attached runs). The wrapper polls
 * until the injected copy of every remap-candidate bridge class is visible
 * to the BOOTSTRAP loader and hands over to p500.Bench.
 *
 * POISONING NOTE (round-1 lesson): the probe MUST NOT use plain
 * Class.forName(name) — parent-first delegation would DEFINE and permanently
 * bind the app-loader classpath stub for any probe whose bootstrap copy is
 * not injected yet, after which the injected copy can never win. The probe
 * therefore uses Class.forName(name, false, null) which consults the
 * bootstrap loader ONLY: it neither defines nor binds anything on a miss.
 *
 * After p500.Bench returns, System.exit(0) is mandatory: the runtime keeps
 * non-daemon threads alive and the JVM would otherwise hang until the
 * external timeout kills it.
 */
public final class KernelPrefDriver {
    private KernelPrefDriver() {}

    /** The four DO_NOT_WIRE bridge classes (src/kernel_policy.rs). */
    private static final String[] PROBES = {
        "PaperNativeLevelChunkHeightmap",
        "PaperNativeMarkerCache",
        "PaperNativePalettedReencodeScratch",
        "PaperNativeProtoChunkHeightmap",
    };

    public static void main(String[] argv) throws Exception {
        long t0 = System.currentTimeMillis();
        if (Boolean.getBoolean("p500.kprefwait")) {
            long deadline = t0 + 45_000;
            while (System.currentTimeMillis() < deadline && !allBootstrapDefined()) {
                Thread.sleep(250);
            }
            // grace period so the injection loop is fully past the probe set
            Thread.sleep(1_000);
            System.err.println("[kpref-driver] wait done after "
                + (System.currentTimeMillis() - t0) + " ms (kprefwait=true): " + loaderReport());
        }
        try {
            p500.Bench.main(argv);
        } finally {
            System.out.flush();
            System.err.flush();
            // runtime keeps non-daemon threads: terminate explicitly
            System.exit(0);
        }
    }

    /** Bootstrap-loader-only probe: never defines, never binds, never poisons. */
    private static boolean allBootstrapDefined() {
        for (String n : PROBES) {
            try {
                Class.forName(n, false, null);
            } catch (ClassNotFoundException e) {
                return false;
            }
        }
        return true;
    }

    private static String loaderReport() {
        StringBuilder sb = new StringBuilder();
        ClassLoader app = KernelPrefDriver.class.getClassLoader();
        for (String n : PROBES) {
            boolean boot = false, appCopy = false;
            try {
                Class.forName(n, false, null);
                boot = true;
            } catch (ClassNotFoundException ignored) { /* not injected */ }
            try {
                Class.forName(n, false, app);
                appCopy = true;
            } catch (ClassNotFoundException ignored) { /* no stub */ }
            if (boot) {
                // whichever copies exist, parent-first delegation resolves the bootstrap one
                sb.append(n).append("->bootstrap(injected) ");
            } else if (appCopy) {
                sb.append(n).append("->app(stub-only) ");
            } else {
                sb.append(n).append("->MISSING ");
            }
        }
        return sb.toString().trim();
    }
}
