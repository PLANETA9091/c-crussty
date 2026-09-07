package ca.spottedleaf.moonrise.common.misc;

/**
 * Variant-B probe (real .so): loads native/libpaper_native_jni.so into THIS
 * classloader (JNI native-method binding only searches libraries loaded by
 * the defining loader, so the load must happen here) and reports that no
 * invocation counting is available for the closed native.
 */
public final class SmokeProbe {
    private SmokeProbe() {}

    static {
        String so = System.getProperty("amsmoke.so");
        if (so == null || so.isEmpty()) {
            throw new IllegalStateException("variant real-so requires -Damsmoke.so=<libpaper_native_jni.so>");
        }
        System.load(so);
    }

    /** The closed native exposes no counter; -1 means "counting unavailable". */
    public static long invocations() {
        return -1L;
    }

    public static boolean counting() {
        return false;
    }

    public static int lastOpsCapacity() {
        return -1;
    }

    public static int lastKeysCapacity() {
        return -1;
    }
}
