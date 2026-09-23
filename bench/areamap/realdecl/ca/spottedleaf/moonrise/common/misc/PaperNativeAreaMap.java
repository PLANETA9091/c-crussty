package ca.spottedleaf.moonrise.common.misc;

/** REAL-mode declaration: binds to Java_net_..._PaperNativeAreaMap_*
 *  exported by libpaper_native_jni.so (System.load'ed by the driver). */
final class PaperNativeAreaMap {
    private PaperNativeAreaMap() {}
    static native int nativeUpdateOpsBatch(
        int fromX, int fromZ, int oldD, int toX, int toZ, int newD,
        byte[] ops, long[] keys
    );
}
