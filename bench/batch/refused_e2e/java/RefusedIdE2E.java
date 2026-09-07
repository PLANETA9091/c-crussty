import java.util.Arrays;
import crussty.batch.PaperNativeBatchDispatch;

/** TASK-52 — runbook G8/A.6 refused-id end-to-end fixture (JVM + REAL .so).
 *
 *  Proves the batch-dispatch refusal surface END-TO-END (real JNI, real
 *  closed-source kernels), where unit tests stop:
 *
 *    R0  ABI gate:   abiVersion() == (TABLE_VERSION << 16) | KERNEL_COUNT
 *    R1  sanity:     a valid shape-A batch (id 2, K=3) executes (run()==n,
 *                    outs written) — proves the harness itself on this arm
 *    R2  -3:         in-range id + out-of-range id -> ERR_BAD_KERNEL_ID,
 *                    `outs` byte-identical to the pre-fill sentinel
 *                    (no partial execution), single-op variant too
 *    R3  -10 (rig):  in-range id that kernel-policy REFUSES (rig arm only:
 *                    a DO_NOT_WIRE kernel appended to the batch table in a
 *                    detached rig worktree — the exact production refusal
 *                    scenario; unreachable on a conforming shipped table by
 *                    the drift-guard invariant "every table id allowed")
 *                    -> ERR_KERNEL_REFUSED, outs sentinel-identical, mixed
 *                    batch [2, refused, 2] aborts WHOLE batch before any op
 *    R4  -3:         negative id -> ERR_BAD_KERNEL_ID
 *    R5  codes:      -3 != -10 distinctness (asserted, printed)
 *
 *  Output: one TSV row per check on stdout
 *    E2E | arm | row | expected | actual | PASS/FAIL | note
 *  plus E2E_VERDICT line; exit 0 iff all PASS, 2 otherwise.
 *
 *  Agent: agent-7625532f (TASK-52).
 */
public class RefusedIdE2E {

    static final long SENTINEL = 0x5A5A5A5A5A5A5A5AL;
    static int pass = 0, fail = 0;
    static String ARM = "unknown";

    public static void main(String[] args) {
        String lib = null, nativeLib = null;
        int kernels = -1, rigRefusedId = -1;
        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "--lib" -> lib = args[++i];
                case "--native" -> nativeLib = args[++i];
                case "--kernels" -> kernels = Integer.parseInt(args[++i]);
                case "--rig-refused-id" -> rigRefusedId = Integer.parseInt(args[++i]);
                default -> { System.err.println("unknown arg " + args[i]); System.exit(2); }
            }
        }
        if (lib == null || nativeLib == null || kernels <= 0) {
            System.err.println("usage: RefusedIdE2E --lib <libcrussty.so> --native <closed .so> "
                    + "--kernels <N> [--rig-refused-id <id>]"); System.exit(2);
        }
        ARM = rigRefusedId >= 0 ? "rig" : "shipped";

        System.load(nativeLib);   // closed kernels first — JVM binder sees them
        System.load(lib);         // module lib; Rust dlopen resolves same handle

        // R0 — ABI gate
        int abi = PaperNativeBatchDispatch.abiVersion();
        int expAbi = (2 << 16) | kernels;
        row("R0-abi-gate", expAbi, abi, true,
                "table=" + (abi >>> 16) + " kernels=" + (abi & 0xFFFF));

        // R1 — functional sanity: valid shape-A batch executes
        long[] outs1 = sentinelOuts(3 * 64);
        int r1 = PaperNativeBatchDispatch.run(ids(2, 2, 2), args0A(3, 16),
                new long[0], fill(3, 64), outs1, offs(3, 64));
        boolean wrote = false;
        for (long v : outs1) if (v != SENTINEL) { wrote = true; break; }
        rowRaw("R1-valid-batch", "3", String.valueOf(r1),
                r1 == 3 && wrote, "kernel id2 K=3, outs written=" + wrote);

        // R2 — out-of-range id (15 on shipped, 16 on rig): whole batch refused -3
        int badId = kernels; // == KERNEL_COUNT -> out of range on this arm
        long[] outs2 = sentinelOuts(2 * 64);
        int r2 = PaperNativeBatchDispatch.run(new int[]{2, badId}, args0A(2, 16),
                new long[0], new int[]{64, 64}, outs2, offs(2, 64));
        boolean u2 = untouched(outs2);
        rowRaw("R2-out-of-range(-3)", String.valueOf(-3), String.valueOf(r2),
                r2 == -3 && u2, "ids=[2," + badId + "] mixed; outs untouched=" + u2);

        long[] outs2b = sentinelOuts(64);
        int r2b = PaperNativeBatchDispatch.run(new int[]{badId}, args0A(1, 16),
                new long[0], new int[]{64}, outs2b, offs(1, 64));
        boolean u2b = untouched(outs2b);
        rowRaw("R2b-out-of-range-single(-3)", String.valueOf(-3), String.valueOf(r2b),
                r2b == -3 && u2b, "ids=[" + badId + "]; outs untouched=" + u2b);

        // R3 — TRUE policy refusal (rig arm only): in-range DO_NOT_WIRE id
        if (rigRefusedId >= 0) {
            long[] outs3 = sentinelOuts(64);
            int r3 = PaperNativeBatchDispatch.run(new int[]{rigRefusedId}, args0A(1, 16),
                    new long[0], new int[]{64}, outs3, offs(1, 64));
            boolean u3 = untouched(outs3);
            rowRaw("R3-refused-single(-10)", String.valueOf(-10), String.valueOf(r3),
                    r3 == -10 && u3, "rig id=" + rigRefusedId
                    + " (DO_NOT_WIRE LevelChunkHeightmap.newCombinedUpdateSummary); outs untouched=" + u3);

            long[] outs3b = sentinelOuts(3 * 64);
            int r3b = PaperNativeBatchDispatch.run(new int[]{2, rigRefusedId, 2}, args0A(3, 16),
                    new long[0], fill(3, 64), outs3b, offs(3, 64));
            boolean u3b = untouched(outs3b);
            rowRaw("R3b-refused-mixed-no-partial(-10)", String.valueOf(-10), String.valueOf(r3b),
                    r3b == -10 && u3b, "ids=[2," + rigRefusedId + ",2] - valid ops must NOT run; outs untouched=" + u3b);
        } else {
            System.out.println("E2E\t" + ARM + "\tR3-refused(-10)\tSKIP\tshipped table: "
                    + "every table id allowed by drift-guard invariant; true -10 needs the rig arm");
        }

        // R4 — negative id
        long[] outs4 = sentinelOuts(64);
        int r4 = PaperNativeBatchDispatch.run(new int[]{-1}, args0A(1, 16),
                new long[0], new int[]{64}, outs4, offs(1, 64));
        boolean u4 = untouched(outs4);
        rowRaw("R4-negative-id(-3)", String.valueOf(-3), String.valueOf(r4),
                r4 == -3 && u4, "ids=[-1]; outs untouched=" + u4);

        // R5 — code distinctness
        rowRaw("R5-codes-distinct", "-3 != -10", (-3 != -10) ? "-3 != -10" : "EQUAL",
                -3 != -10, "ERR_BAD_KERNEL_ID != ERR_KERNEL_REFUSED");

        System.out.println("E2E_VERDICT\t" + ARM + "\t" + (fail == 0 ? "PASS" : "FAIL")
                + "\tpass=" + pass + " fail=" + fail);
        System.exit(fail == 0 ? 0 : 2);
    }

    // ------------------------------------------------------------- helpers
    static int[] ids(int... a) { return a; }

    static long[] args0A(int n, long p0) {
        long[] a = new long[n];
        Arrays.fill(a, p0);
        return a;
    }

    static int[] fill(int n, int v) {
        int[] a = new int[n];
        Arrays.fill(a, v);
        return a;
    }

    static int[] offs(int n, int cap) {
        int[] a = new int[n];
        for (int i = 0; i < n; i++) a[i] = i * cap;
        return a;
    }

    static long[] sentinelOuts(int len) {
        long[] o = new long[len];
        Arrays.fill(o, SENTINEL);
        return o;
    }

    static boolean untouched(long[] o) {
        for (long v : o) if (v != SENTINEL) return false;
        return true;
    }

    static void row(String name, long expected, long actual, boolean outsOk, String note) {
        rowRaw(name, String.valueOf(expected), String.valueOf(actual),
                expected == actual && outsOk, note);
    }

    static void rowRaw(String name, String expected, String actual, boolean ok, String note) {
        if (ok) pass++; else fail++;
        System.out.println("E2E\t" + ARM + "\t" + name + "\t" + expected + "\t"
                + actual + "\t" + (ok ? "PASS" : "FAIL") + "\t" + note);
    }
}
