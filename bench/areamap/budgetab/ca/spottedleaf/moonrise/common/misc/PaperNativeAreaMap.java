package ca.spottedleaf.moonrise.common.misc;

/**
 * BUDGET-RIG fake for the closed {@code nativeUpdateOpsBatch} (TASK-64
 * variant C rig only — the shared smoke/oracle fake in bench/areamap/ca is
 * intentionally untouched, so every pre-existing rig keeps its exact
 * semantics).
 *
 * Emulates the CLOSED native's buffer contract as MEASURED by the STEP-0
 * probe (docs/AREAMAP_DENSE_APPLY_DESIGN.md §11.1, committed 2026-09-08):
 *
 * <ul>
 *   <li>accepts offered {@code len >= n0} (n0 = full naive diff size) and
 *       returns n0 — the boundary is exactly {@code len >= n0};</li>
 *   <li>rejects {@code len < n0} with EXACTLY {@code -n0} (the built-in size
 *       oracle), side-effect-free: NOTHING is written on reject (sentinel
 *       probe measured {@code writes_past_n=0});</li>
 *   <li>no hidden len-dependent semantics between n0 and cap: the emitted
 *       multiset depends only on the rect arguments;</li>
 *   <li>emission order = the shared fake's canonical order (removes
 *       old∖new first, then adds new∖old, row-major) — order is not part of
 *       the TASK-30 multiset canon, fixed here only for determinism.</li>
 * </ul>
 *
 * NOT the closed lib: the REAL-mode oracle run in this same rig (realdecl
 * against libpaper_native_jni.so) is the binding semantic gate (§11.2
 * falsifier 2). This fake exists so the budgeted bridge's retry logic is
 * testable headlessly and deterministically.
 */
final class PaperNativeAreaMap {
    private PaperNativeAreaMap() {}

    static int nativeUpdateOpsBatch(
        int fromX, int fromZ, int oldD, int toX, int toZ, int newD,
        byte[] ops, long[] keys
    ) {
        // pass 1: exact diff size n0 (the native's measured behavior:
        // it validates len >= n BEFORE writing anything — §11.1)
        int n0 = 0;
        for (int dx = -oldD; dx <= oldD; dx++) {
            for (int dz = -oldD; dz <= oldD; dz++) {
                final int x = fromX + dx, z = fromZ + dz;
                if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD) n0++;
            }
        }
        for (int dx = -newD; dx <= newD; dx++) {
            for (int dz = -newD; dz <= newD; dz++) {
                final int x = toX + dx, z = toZ + dz;
                if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD) n0++;
            }
        }
        if (ops.length < n0 || keys.length < n0) {
            return -n0; // side-effect-free reject with the exact size oracle
        }
        // pass 2: emit in the shared fake's canonical order
        int n = 0;
        for (int dx = -oldD; dx <= oldD; dx++) {
            for (int dz = -oldD; dz <= oldD; dz++) {
                final int x = fromX + dx, z = fromZ + dz;
                if (Math.abs(x - toX) > newD || Math.abs(z - toZ) > newD) {
                    ops[n] = 1; keys[n] = pack(x, z); n++;
                }
            }
        }
        for (int dx = -newD; dx <= newD; dx++) {
            for (int dz = -newD; dz <= newD; dz++) {
                final int x = toX + dx, z = toZ + dz;
                if (Math.abs(x - fromX) > oldD || Math.abs(z - fromZ) > oldD) {
                    ops[n] = 0; keys[n] = pack(x, z); n++;
                }
            }
        }
        return n;
    }

    static long pack(int x, int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }
}
