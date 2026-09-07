/** TASK-48 bench stub — verbatim copy of bench/p500/java/PaperNativeDensityAp2MinMaxFill.java
 *  (generated there by bench/p500/gen_p500_bench.py; DO NOT EDIT either side — keep in sync).
 *  Declares the Crussty CE exports as static natives so the JVM binds them
 *  to libpaper_native_jni.so by exact JNI symbol name (P500 real-mode style:
 *  System.load(closed lib) BEFORE System.load(libcrussty.so)).
 *  Shape A' (III[J)I — the batch dispatcher's wave-1 kernel pair (ids 12/13). */
public final class PaperNativeDensityAp2MinMaxFill {
    public static native int newSummary(int a0, int a1, int a2, long[] a3);
    public static native int oldSummary(int a0, int a1, int a2, long[] a3);
}
