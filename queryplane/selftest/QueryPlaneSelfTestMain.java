package net.minecraft.world.entity;

import java.lang.reflect.Field;

/**
 * TASK-417-C local gate: QueryPlaneSelfTestMain 5/5 (app-loader path).
 *
 * Validates the EXACT static the rust side calls after define_class
 * (find_class fix: selfTest on the LOCAL ref — initialization happens on
 * that call). On the runner the same contract is proven by the
 * "ARMED queryplane awake (selfTest==true" boot marker; locally we drive
 * the app-loader path of the COMMITTED blob (queryplane/build/...).
 *
 * Run: CRUSSTY_LEVER_FLAG=cmp417_bq java -cp <kernel.jar>:queryplane/build:queryplane/selftest \
 *        net.minecraft.world.entity.QueryPlaneSelfTestMain   (exit 0 = 5/5 PASS)
 */
public final class QueryPlaneSelfTestMain {

    private static int n;
    private static String fail;

    private static void check(String name, boolean ok) {
        n++;
        System.out.println("check " + n + "/5 " + name + ": " + (ok ? "PASS" : "FAIL"));
        if (!ok && fail == null) {
            fail = name;
        }
    }

    private static Object staticField(String name) throws Exception {
        Field f = QueryPlaneOps.class.getDeclaredField(name);
        f.setAccessible(true);
        return f.get(null);
    }

    public static void main(String[] args) throws Exception {
        String flag = System.getenv("CRUSSTY_LEVER_FLAG");
        check("lever flag is cmp417_bq", "cmp417_bq".equals(flag == null ? null : flag.trim()));

        // <clinit> runs HERE (first static touch — the exact JVMS 5.5
        // initialization the rust-side local-ref call relies on).
        check("selfTest() == true (init on first static call)", QueryPlaneOps.selfTest());
        check("selfTest() idempotent", QueryPlaneOps.selfTest());

        Object hardAdds = staticField("HARD_ADDS");
        check("HARD_ADDS == 0 (probe cold)",
                hardAdds instanceof java.util.concurrent.atomic.AtomicLong
                        && ((java.util.concurrent.atomic.AtomicLong) hardAdds).get() == 0L);

        boolean latchesClean = Boolean.FALSE.equals(staticField("playerPlaneBroken"))
                && Boolean.FALSE.equals(staticField("hardPlaneBroken"))
                && Boolean.TRUE.equals(staticField("ENABLED"));
        check("ENABLED=true, no disarm latches", latchesClean);

        if (fail != null) {
            System.out.println("QueryPlaneSelfTestMain: FAILED at: " + fail);
            System.exit(1);
        }
        System.out.println("QueryPlaneSelfTestMain: 5/5 PASS");
    }
}
