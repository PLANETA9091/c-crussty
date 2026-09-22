import net.minecraft.world.entity.QueryPlaneOps;

/**
 * TASK-416-A recipe 3 pre-gate: run QueryPlaneOps.selfTest() LOCALLY (soot)
 * against the FRESH blob + real kernel jar, before any dispatch.
 * Expect: true for {cmp412_b2p1, cmp415_mcomp, cmp416_mcomp, cmp417_mcomp}, false otherwise.
 */
public final class QueryPlaneSelfTestMain {
    public static void main(String[] args) {
        String lever = System.getenv("CRUSSTY_LEVER_FLAG");
        String expect = System.getenv("EXPECTED");
        boolean got;
        try {
            got = QueryPlaneOps.selfTest();
        } catch (Throwable t) {
            System.out.println("selfTest THREW: " + t);
            got = false;
        }
        System.out.println("RESULT lever=" + lever + " selfTest=" + got
                + (expect != null ? " expected=" + expect : ""));
        if (expect != null && Boolean.parseBoolean(expect) != got) {
            System.out.println("GATE-FAIL");
            System.exit(1);
        }
        System.out.println("GATE-OK");
    }
}
