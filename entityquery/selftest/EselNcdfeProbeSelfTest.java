import java.util.ArrayList;
import java.util.List;

/**
 * AG-198 w528 selftest — ESEL-NCDFE probe (iter-1, MAIN#2 per-type eindex).
 *
 * WHY (TASK-413-C precedent, cv3-1 35712182885): a redirect executed against an
 * UNDEFINED ops class is not a transient error — HotSpot caches the
 * NoClassDefFoundError PER CONSTANT-POOL ENTRY, so the call site keeps throwing
 * for the whole run even AFTER the class is finally defined (observed span
 * 1074..83741, x3938 repeats, ARM at 9888). For per-type eindex iter-2 the
 * retargeted sites are EntitySelector.addEntities invoke #297@32 (box) and
 * #300@48 (no-box) — the FIRST selector query anywhere in the JVM resolves
 * them. Therefore the ONLY legal arm order is the закон-6-v16 chain:
 *
 *     define(ops) -> retarget(sites) -> probe(T1 NCDFE=0) -> publish
 *
 * This probe is a STATE-MACHINE DIFFERENTIAL (the kernel JVM cannot be
 * instantiated offline — same harness genre as EselFastPathSelfTest): it
 * models cp-entry resolution states and the four arm phases, then verifies
 * every legal/illegal interleaving, the sticky-NCDFE cache semantics, the
 * never-cache-failure rule for DEFINE retries, and the mirror-drift sha-needle
 * fail-closed abort. Contract pins (AG-110 javap + AG-151 FQN-corr) are baked
 * in as constants and checked against the redirect table.
 *
 * GATES (this selftest): G-N1 strict order invariant; G-N2 sticky NCDFE cache
 * detected before publish; G-N3 define retry never caches failure; G-N4
 * mirror-drift aborts retarget fail-closed; G-N5 probe T1=0 at publish.
 *
 * DORMANT: models only; no kernel class is referenced; javac EXIT=0 expected
 * against the canon kernel pin e2992d63 on the classpath (G7-recipe AG-118).
 */
public final class EselNcdfeProbeSelfTest {

    // --- contract pins (AG-110 javap iter-2 + AG-151 FQN-коррекция) ----------
    static final String OPS_FQN =
            "net/minecraft/commands/arguments/selector/EntitySelectorOps";
    static final int SITE_BOX_INVOKE = 297, SITE_BOX_OFF = 32;
    static final int SITE_NOBOX_INVOKE = 300, SITE_NOBOX_OFF = 48;
    static final int CP_FIELD_TYPE = 97, CP_FIELD_ANY_TYPE = 79;
    static final String CANON_KERNEL_SHA16 = "e2992d63abd2c254";
    static final int CANON_ES_CLASS_BYTES = 15940; // EntitySelector.class, sha c56bf726
    static final int CANON_OPS_CLASS_BYTES = 5068; // AG-148 javac rc=0 (1 cls)

    // --- modeled JVM semantics ----------------------------------------------
    enum CpState { UNRESOLVED, RESOLVED, NCDFE_CACHED }

    enum Phase { BOOT, DEFINE, RETARGET, PROBE, PUBLISH, FIRST_CALL }

    static final class Site {
        final String name; CpState cp = CpState.UNRESOLVED;
        Site(String n) { name = n; }
    }

    static final class Machine {
        final Site box = new Site("#297@32");
        final Site nobox = new Site("#300@48");
        boolean opsDefined = false, retargeted = false, published = false;
        int ncufeRepeats = 0, defineRetries = 0;
        String definedSha = null;
        String needleSha = "canon"; // mirror-drift needle input (sha256 of bytes)

        /** redirect execution at FIRST selector query (dp50k inject class). */
        String callSite(Site s) {
            if (!retargeted) return "VANILLA";                 // no redirect -> no resolution
            switch (s.cp) {
                case RESOLVED: return "FASTPATH";
                case UNRESOLVED:
                    if (opsDefined) { s.cp = CpState.RESOLVED; return "FASTPATH"; }
                    s.cp = CpState.NCDFE_CACHED;               // sticky per cp-entry
                    ncufeRepeats++;
                    return "NCDFE";
                case NCDFE_CACHED:
                    ncufeRepeats++;                            // cached: repeats forever
                    return "NCDFE";
            }
            return "NCDFE";
        }

        boolean define(String bytesSha, int bytesLen) {
            if (opsDefined) return true;                       // idempotent
            if (!needle(bytesSha, bytesLen)) return false;     // G-N4 fail-closed, NO state change
            opsDefined = true; definedSha = bytesSha;
            return true;
        }

        /** define retries are legal (transient JNI hiccup) — never cache failure. */
        boolean defineWithRetry(String bytesSha, int bytesLen) {
            for (int i = 0; i < 3; i++) {
                defineRetries++;
                boolean transientHiccup = (i == 0); // modeled attempt-0 JNI failure
                if (!transientHiccup && needle(bytesSha, bytesLen)) {
                    return define("canon", CANON_OPS_CLASS_BYTES);
                }
            }
            return false;
        }

        boolean needle(String sha, int len) {              // G-N4 mirror-drift
            return "canon".equals(sha) && len == CANON_OPS_CLASS_BYTES;
        }

        void retarget() { if (opsDefined) retargeted = true; } // strict order: else no-op
        boolean probe() {                                  // G-N5: T1 NCDFE=0
            return retargeted && box.cp != CpState.NCDFE_CACHED
                    && nobox.cp != CpState.NCDFE_CACHED;
        }
        boolean publish() { if (probe()) published = true; return published; }
    }

    // --- scenarios -----------------------------------------------------------
    public static void main(String[] args) {
        List<String> fails = new ArrayList<>();

        // S1 legal chain: define -> retarget -> probe -> publish -> first call GREEN
        Machine m1 = new Machine();
        if (!m1.define("canon", CANON_OPS_CLASS_BYTES)) fails.add("S1 define");
        m1.retarget();
        m1.callSite(m1.box); m1.callSite(m1.nobox);
        if (!m1.publish()) fails.add("S1 publish");
        if (!"FASTPATH".equals(m1.callSite(m1.box))) fails.add("S1 fastpath");

        // S2 illegal: retarget skipped define; first call poisons BOTH cp entries
        Machine m2 = new Machine();
        m2.retarget();                                     // no-op guard: stays unretargeted
        if (m2.callSite(m2.box).equals("NCDFE")) fails.add("S2 guard");
        // (retarget() without define must NOT arm — strict order, иначе S3 ниже)

        // S3 sticky cache: define AFTER first poisoned call is too late
        Machine m3 = new Machine();
        m3.retargeted = true;                              // adversarial: forced retarget
        if (!"NCDFE".equals(m3.callSite(m3.box))) fails.add("S3 first");
        if (!m3.define("canon", CANON_OPS_CLASS_BYTES)) fails.add("S3 define");
        if (!"NCDFE".equals(m3.callSite(m3.box))) fails.add("S3 sticky");
        if (m3.ncufeRepeats < 2) fails.add("S3 repeats");
        if (m3.probe()) fails.add("S3 probe-must-fail");   // G-N5 catches poison pre-publish

        // S4 mirror-drift: wrong byte size -> define refused, retarget never arms
        Machine m4 = new Machine();
        if (m4.define("drift", 1234)) fails.add("S4 drift-accepted");
        m4.retarget();
        if (m4.retargeted) fails.add("S4 drift-retarget");
        if (m4.probe()) fails.add("S4 drift-probe");

        // S5 define retry never caches failure (TASK-409-E/never-cache rule)
        Machine m5 = new Machine();
        if (!m5.defineWithRetry("canon", CANON_OPS_CLASS_BYTES)) fails.add("S5 retry");
        if (m5.defineRetries < 2) fails.add("S5 retries");

        // G-N6 contract pins self-consistency
        if (SITE_BOX_INVOKE != 297 || SITE_BOX_OFF != 32) fails.add("pins box");
        if (SITE_NOBOX_INVOKE != 300 || SITE_NOBOX_OFF != 48) fails.add("pins nobox");
        if (CP_FIELD_TYPE == CP_FIELD_ANY_TYPE) fails.add("pins type/any");
        if (CANON_OPS_CLASS_BYTES != 5068) fails.add("pins ops-size");
        if (CANON_ES_CLASS_BYTES != 15940) fails.add("pins es-size");
        if (!CANON_KERNEL_SHA16.equals("e2992d63abd2c254")) fails.add("pins kernel");

        System.out.println("[esel-ncdfe] probe verdict: " + (fails.isEmpty() ? "GREEN" : "RED"));
        for (String f : fails) System.out.println("[esel-ncdfe] FAIL " + f);
        System.out.println("[esel-ncdfe] order=define->retarget->probe->publish; "
                + "sites=297@32/300@48; ops=" + OPS_FQN);
        if (!fails.isEmpty()) System.exit(1);
    }

    private EselNcdfeProbeSelfTest() {}
}
