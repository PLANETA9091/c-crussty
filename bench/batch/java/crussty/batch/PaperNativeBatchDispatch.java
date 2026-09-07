
/** Batch-floor bench stub — declares the Crussty batch bridge class exactly as
 *  the runtime defines it at inject time (batch_api.rs BATCH_CLASS/RUN_SIG).
 *  In the bench (no engine) the JVM auto-resolves the native symbols from the
 *  module cdylib once it is System.load'ed: run()/abiVersion() are exported by
 *  libcrussty.so (RUN_SYMBOL/ABI_SYMBOL); kernels resolve inside the
 *  dispatcher through its own dlopen of libpaper_native_jni.so
 *  (CRUSSTY_BATCH_NATIVE_LIB, batch_api::self_init). */
package crussty.batch;

public final class PaperNativeBatchDispatch {
    private PaperNativeBatchDispatch() {}

    public static native int run(int[] kernelIds, long[] args0, long[] args1,
                                 int[] argCounts, long[] outs, int[] outOffsets);

    public static native int abiVersion();
}
