package net.minecraft.world.level.biome;

/**
 * TASK-47-w7 bench stub — batch-table id 11 (full internal-name form of
 * PaperNativeClimateRTree.nativeBuildTreeHandle, jni_table.rs:92). Declares
 * the export as a static native so the JVM binds it to
 * libpaper_native_jni.so by exact JNI symbol name (P500 real-mode style).
 * Mirrors bench/p500/java/PaperNativeClimateRTree.java (default-package
 * form, batch-table id 10). DO NOT EDIT — regenerate per jni_table.rs.
 */
public final class PaperNativeClimateRTree {
    public static native long nativeBuildTreeHandle(long[] a0, long[] a1);
    public static native void nativeFreeTreeHandle(long a0);
    public static native long nativeChecksumTreeHandle(long a0);
}
