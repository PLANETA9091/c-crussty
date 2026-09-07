package ca.spottedleaf.moonrise.common.misc;

/**
 * Variant-A probe (counting stub native): exposes the invocation counter and
 * the scratch-capacity observations of the fake native to the harness package
 * without leaking the fake native class itself.
 */
public final class SmokeProbe {
    private SmokeProbe() {}

    public static long invocations() {
        return PaperNativeAreaMap.INVOCATIONS.get();
    }

    public static boolean counting() {
        return true;
    }

    /** ops[].length the stub saw on its last invocation (>= maxOps if the
     *  grow-only scratch logic is correct). */
    public static int lastOpsCapacity() {
        return PaperNativeAreaMap.lastOpsCapacity;
    }

    /** keys[].length the stub saw on its last invocation (== ops capacity). */
    public static int lastKeysCapacity() {
        return PaperNativeAreaMap.lastKeysCapacity;
    }
}
