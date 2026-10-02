package ca.spottedleaf.moonrise.common.misc;

import java.util.concurrent.atomic.AtomicLong;

/**
 * Counting STUB for the closed native bridge -- smoke-test-only, NEVER shipped.
 *
 * Mirrors the documented native contract of the real
 * {@code PaperNativeAreaMap.nativeUpdateOpsBatch} (see src/area_map.rs
 * bridge_selftest / naive_set_difference):
 *   - emits adds = new square \ old square with op byte 0,
 *   - emits removes = old square \ new square with op byte != 0 (1),
 *   - key packing: x in the low 32 bits, z in the high 32 bits,
 *   - returns the number of emitted ops,
 * and additionally counts every invocation so the smoke test can prove the
 * same-state fast path SKIPS the native call (delta == 0) and every changed
 * update calls it EXACTLY once (delta == 1).
 */
final class PaperNativeAreaMap {
    private PaperNativeAreaMap() {}

    static final AtomicLong INVOCATIONS = new AtomicLong();

    static volatile int lastFromX, lastFromZ, lastOldD, lastToX, lastToZ, lastNewD;
    static volatile int lastOpsCapacity = -1, lastKeysCapacity = -1;

    static int nativeUpdateOpsBatch(int fromX, int fromZ, int oldD,
                                    int toX, int toZ, int newD,
                                    byte[] ops, long[] keys) {
        INVOCATIONS.incrementAndGet();
        lastFromX = fromX; lastFromZ = fromZ; lastOldD = oldD;
        lastToX = toX; lastToZ = toZ; lastNewD = newD;
        lastOpsCapacity = ops.length;
        lastKeysCapacity = keys.length;

        int n = 0;
        // adds = new \ old  (op byte 0)
        for (int x = toX - newD; x <= toX + newD; x++) {
            for (int z = toZ - newD; z <= toZ + newD; z++) {
                if (x < fromX - oldD || x > fromX + oldD || z < fromZ - oldD || z > fromZ + oldD) {
                    if (n >= ops.length || n >= keys.length) {
                        throw new IllegalStateException("stub buffer too small: n=" + n);
                    }
                    ops[n] = 0;
                    keys[n] = key(x, z);
                    n++;
                }
            }
        }
        // removes = old \ new  (op byte 1)
        for (int x = fromX - oldD; x <= fromX + oldD; x++) {
            for (int z = fromZ - oldD; z <= fromZ + oldD; z++) {
                if (x < toX - newD || x > toX + newD || z < toZ - newD || z > toZ + newD) {
                    if (n >= ops.length || n >= keys.length) {
                        throw new IllegalStateException("stub buffer too small: n=" + n);
                    }
                    ops[n] = 1;
                    keys[n] = key(x, z);
                    n++;
                }
            }
        }
        return n;
    }

    static long key(int x, int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }
}
