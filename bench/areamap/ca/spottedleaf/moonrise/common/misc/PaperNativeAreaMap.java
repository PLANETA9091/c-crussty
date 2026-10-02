package ca.spottedleaf.moonrise.common.misc;

/**
 * SMOKE-LOCAL fake for the real PaperNativeAreaMap native surface (same
 * FQCN; a static JAVA method satisfies the bridge's invokestatic — no JNI
 * involved). Emits the naive set difference (New \ Old = adds, Old \ New =
 * removes) packed as (z<<32 | x&0xFFFFFFFFL) with op 0=add / 1=remove, and
 * counts invocations so the fast-path / MIN_VALUE guards are observable.
 */
final class PaperNativeAreaMap {
    private PaperNativeAreaMap() {}

    static final java.util.concurrent.atomic.AtomicLong CALLS =
            new java.util.concurrent.atomic.AtomicLong();

    static void resetCalls() { CALLS.set(0L); }
    static long calls() { return CALLS.get(); }

    static int nativeUpdateOpsBatch(
        int fromX, int fromZ, int oldD, int toX, int toZ, int newD,
        byte[] ops, long[] keys
    ) {
        CALLS.incrementAndGet();
        int n = 0;
        // removes: Old \ New
        for (int dx = -oldD; dx <= oldD; dx++) {
            for (int dz = -oldD; dz <= oldD; dz++) {
                final int x = fromX + dx, z = fromZ + dz;
                if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD) {
                    if (n < ops.length) { ops[n] = 1; keys[n] = pack(x, z); n++; }
                }
            }
        }
        // adds: New \ Old
        for (int dx = -newD; dx <= newD; dx++) {
            for (int dz = -newD; dz <= newD; dz++) {
                final int x = toX + dx, z = toZ + dz;
                if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD) {
                    if (n < ops.length) { ops[n] = 0; keys[n] = pack(x, z); n++; }
                }
            }
        }
        return n;
    }

    static long pack(int x, int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }
}
