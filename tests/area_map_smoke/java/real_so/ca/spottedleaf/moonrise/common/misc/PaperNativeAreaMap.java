package ca.spottedleaf.moonrise.common.misc;

/**
 * Variant-B native class (real .so): same shape as the compile-time stub in
 * the shipped RuntimeStubs.java, but this one is actually used at runtime --
 * the JVM binds {@code nativeUpdateOpsBatch} to
 * {@code Java_ca_spottedleaf_moonrise_common_misc_PaperNativeAreaMap_nativeUpdateOpsBatch}
 * exported by native/libpaper_native_jni.so (loaded by SmokeProbe, same
 * classloader, so JNI library scoping resolves it).
 */
final class PaperNativeAreaMap {
    private PaperNativeAreaMap() {}

    static native int nativeUpdateOpsBatch(
        int fromX, int fromZ, int oldD, int toX, int toZ, int newD,
        byte[] ops, long[] keys
    );
}
