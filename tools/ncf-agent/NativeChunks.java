package dev.crussty.ncf;

/**
 * NCF P3.4 (step B) — the ONE JNI call per chunk surface.
 *
 * Loaded by the CRUSSTY JVMTI agent module after -agentpath attach; the
 * library is libchunk_factory_jni.so built from chunk-factory (cfg(jni)).
 * Contract: one call per chunk; a non-null result is gzip-compressed NBT
 * (NbtIo.writeCompressed form) of the FULL chunk; null = NO_DATA -> the
 * Moonrise hook (NativeChunksIO) falls through to Java generation (I8).
 * The export is policy-gated: DO_NOT_ENABLE in the kernel registry until
 * the P2 zero-diff gate closes (I9).
 */
public final class NativeChunks {
    private NativeChunks() {}

    /** @return gzip NBT of the chunk, or null = Java generates (I8). */
    public static native byte[] generateChunk(int chunkX, int chunkZ);

    public static boolean available() {
        try {
            System.loadLibrary("chunk_factory_jni");
            return true;
        } catch (Throwable t) {
            return false;
        }
    }
}
