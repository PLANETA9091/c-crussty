package net.minecraft.world.level.levelgen.synth;

import java.util.WeakHashMap;

/**
 * G-BODY (TASK-71) — dispatch helper for the whole-body swap of
 * PerlinNoise.getValue(DDDDDDZ)D. Lazy per-instance native handle built by
 * the TASK-70-decoded ABI (P3/D1/F1: slot-indexed p-concat + presence mask
 * + per-slot coords/amplitudes + input/value factors), one
 * nativeGetValue(JDDDDDZ)D crossing per call.
 *
 * PROTOTYPE lifecycle note: the handle is cached in a WeakHashMap keyed by
 * the owner (value long[] holds no reference to the key, so entries are
 * collectable) but the NATIVE handle is NOT freed on collection in this
 * rig — production uses the TASK-01 phantom-reaper lifecycle (live-verified
 * in bench/lifecycle). The rig's noise object lives for the whole run, so
 * this leaks at most one handle per rig JVM and is honest for prototype
 * purposes.
 */
public final class BodyDispatch {
    private BodyDispatch() {}

    private static final WeakHashMap<PerlinNoise, long[]> HANDLES = new WeakHashMap<>();
    private static volatile long LAST_BUILD_NS = -1L;

    public static double getValue(PerlinNoise pn, double x, double y, double z,
                                  double y0, double y1, boolean flag) {
        long[] h = HANDLES.get(pn);
        if (h == null) {
            long t0 = System.nanoTime();
            long built = build(pn);
            LAST_BUILD_NS = System.nanoTime() - t0;
            h = new long[]{built};
            HANDLES.put(pn, h);
            System.out.println("[dispatch] built handle for " + System.identityHashCode(pn)
                + " mapSize=" + HANDLES.size() + " buildNs=" + LAST_BUILD_NS);
        }
        return PaperNativePerlinNoise.nativeGetValue(h[0], x, y, z, y0, y1, flag);
    }

    /** Direct access for the prototype's dispatch-overhead arm. */
    public static long handleOf(PerlinNoise pn) {
        long[] h = HANDLES.get(pn);
        return h == null ? 0L : h[0];
    }

    public static long lastBuildNs() { return LAST_BUILD_NS; }

    static long build(PerlinNoise pn) {
        try {
            java.lang.reflect.Field fN = PerlinNoise.class.getDeclaredField("noiseLevels");
            fN.setAccessible(true);
            ImprovedNoise[] lv = (ImprovedNoise[]) fN.get(pn);
            int n = lv.length;
            byte[] a0 = new byte[256 * n];
            byte[] a1 = new byte[n];
            double[] xo = new double[n], yo = new double[n], zo = new double[n];
            java.lang.reflect.Field fp = ImprovedNoise.class.getDeclaredField("p");
            fp.setAccessible(true);
            for (int i = 0; i < n; i++) {
                if (lv[i] != null) {
                    byte[] p = (byte[]) fp.get(lv[i]);
                    System.arraycopy(p, 0, a0, 256 * i, 256);
                    a1[i] = 1;
                    xo[i] = lv[i].xo;
                    yo[i] = lv[i].yo;
                    zo[i] = lv[i].zo;
                }
            }
            java.lang.reflect.Field fA = PerlinNoise.class.getDeclaredField("amplitudes");
            fA.setAccessible(true);
            double[] amp = ((it.unimi.dsi.fastutil.doubles.DoubleList) fA.get(pn)).toDoubleArray();
            double inF = (Double) getF(PerlinNoise.class, pn, "lowestFreqInputFactor");
            double valF = (Double) getF(PerlinNoise.class, pn, "lowestFreqValueFactor");
            long h = PaperNativePerlinNoise.nativeBuildHandle(a0, a1, xo, yo, zo, amp, inF, valF);
            if (h == 0L) throw new IllegalStateException("nativeBuildHandle returned 0");
            return h;
        } catch (RuntimeException e) {
            throw e;
        } catch (Exception e) {
            throw new IllegalStateException("handle build failed", e);
        }
    }

    private static Object getF(Class<?> k, Object o, String f) throws Exception {
        java.lang.reflect.Field rf = k.getDeclaredField(f);
        rf.setAccessible(true);
        return rf.get(o);
    }
}
