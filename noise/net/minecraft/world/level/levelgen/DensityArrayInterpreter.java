package net.minecraft.world.level.levelgen;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import net.minecraft.util.Mth;

/**
 * TASK-108 v3 (PROGRESS-6/7, agent-7625532f): density tree ARRAY-FORM
 * interpreter — docs/BATCH_BRIDGE_DESIGN.md §7.3-7.5.
 *
 * Contract: {@link #eval} fills {@code out[0..n)} with the same values the
 * vanilla per-point evaluation would produce, re-ordered so batchable leaves
 * are served by ONE native batch crossing (v1 machinery:
 * {@link NormalNoiseBatchOps#fillNoise} / {@code fillShift}), combinators are
 * plain Java array loops, and unknown nodes fall back to the LITERAL vanilla
 * sweep ({@code provider.fillAllDirectly(out, node)}) — bit-exactness by
 * construction for the fallback tier.
 *
 * Bit-exactness ledger (per tier):
 * - Noise/ShiftA/ShiftB leaves: v1 kernels, live smoke-7 selfTest PASS
 *   (PROGRESS-4) + G-ABI-2 standalone probe (PROGRESS-2).
 * - Constant: IEEE fill.
 * - YClampedGradient: the SAME Mth.clampedMap call as kernel compute
 *   (javap 1.21.10: i2d x3 + clampedMap).
 * - PureTransformer (Mapped/Clamp/MulOrAdd): the kernel's OWN public
 *   transform(double) applied elementwise — bit-exact by construction.
 * - Ap2: vanilla compute is tableswitch { dadd, dmul, Math.min, Math.max }
 *   (javap 1.21.10) — mirrored with identity-compare on the enum (an enum
 *   switch would mint a synthetic $SwitchMap class the ship audit forbids).
 * - HolderHolder: pure delegation.
 * - tier 3 fallback: the provider's own fillAllDirectly = the vanilla
 *   default fillArray body, protocol identical (NoiseChunk$1 semantics).
 *
 * Ordering safety: unlike vanilla (one interleaved sweep), this interpreter
 * drives one provider sweep per node. Every sweep is internally
 * vanilla-identical (same forIndex sequence), and NoiseChunk point-state
 * (arrayIndex, interpolationCounter) is only used by caches as a
 * point-identity token, never as a magnitude — so re-sweeps cannot change
 * values. The in-server SHADOW selftest (below) on the REAL router is the
 * pre-registered v3 gate; a FAIL keeps the A/B forbidden.
 *
 * SHADOW MODE (env CRUSSTY_V3_SHADOW_N, default 0 = off): the first N
 * bridged column fills additionally compute the EXACT vanilla fill
 * ({@code provider.fillAllDirectly(oracle, wrapped)}) and raw-bits compare
 * it against the interpreter output, then print one
 * {@code CRUSSTY_V3_SHADOW PASS|FAIL} line with the node census. The oracle
 * sweep re-advances NoiseChunk state (diagnostic-only; the mode never runs
 * in production or A/B boots).
 *
 * Java 8 discipline (scripts/build_noise.sh pins --release 8, major 52, and
 * the ship audit forbids synthetic classes): no pattern-matching instanceof,
 * no arrow switches, no capturing lambdas, no anonymous classes, no streams.
 */
public final class DensityArrayInterpreter {

    private static final ConcurrentHashMap CENSUS = new ConcurrentHashMap();

    /**
     * Synthetic back-reference NoiseChunk$NoiseInterpolator.this$0 — javac
     * hides synthetic members at compile time even from binary jars, so the
     * read is reflective (cached; resolved once, same-loader injection makes
     * it trivially accessible). Null = never resolved (would only happen on a
     * foreign classloader), in which case the fillingCell branch is skipped
     * and the slice-fill path serves the call.
     */
    private static final java.lang.reflect.Field INTERP_THIS0;
    static {
        java.lang.reflect.Field f = null;
        try {
            f = NoiseChunk.NoiseInterpolator.class.getDeclaredField("this$0");
            f.setAccessible(true);
        } catch (final Throwable ignored) {
            // fail-closed: slice-fill path below still serves vanilla values
        }
        INTERP_THIS0 = f;
    }

    // ---- shadow selftest state (env-gated, default OFF) ----
    private static final int SHADOW_N = parseShadowN();
    private static final AtomicLong SHADOW_LEFT = new AtomicLong(SHADOW_N);
    private static final AtomicLong SHADOW_CHECKS = new AtomicLong(0);
    private static final AtomicLong SHADOW_FAILS = new AtomicLong(0);
    private static volatile boolean SHADOW_REPORTED = false;

    private static int parseShadowN() {
        try {
            final String v = System.getenv("CRUSSTY_V3_SHADOW_N");
            if (v != null) {
                return Math.max(0, Integer.parseInt(v.trim()));
            }
        } catch (final Throwable ignored) {
            // fall through: shadow stays off
        }
        return 0;
    }

    private DensityArrayInterpreter() {
    }

    /** Resets the node census (probe scaffolding). */
    public static void censusReset() {
        CENSUS.clear();
    }

    /** Sorted "key=count" census dump for selftest/bench logs. */
    public static String censusDump() {
        final StringBuilder sb = new StringBuilder("CENSUS");
        final List keys = new ArrayList(CENSUS.keySet());
        Collections.sort(keys);
        for (int i = 0; i < keys.size(); i++) {
            final String k = (String) keys.get(i);
            final AtomicLong v = (AtomicLong) CENSUS.get(k);
            sb.append(' ').append(k).append('=').append(v == null ? 0L : v.get());
        }
        return sb.toString();
    }

    private static void bump(final String key) {
        AtomicLong v = (AtomicLong) CENSUS.get(key);
        if (v == null) {
            v = new AtomicLong(0);
            final AtomicLong prev = (AtomicLong) CENSUS.putIfAbsent(key, v);
            if (prev != null) {
                v = prev;
            }
        }
        v.incrementAndGet();
    }

    /**
     * Whole-body replacement target for
     * {@code NoiseChunk$NoiseInterpolator.fillArray(double[], ContextProvider)}
     * (wired via NormalNoiseBatchOps.interpFillArray forwarder — the Rust
     * bridge emits invokestatic NormalNoiseBatchOps, bridge_owner law).
     * Vanilla branch contract preserved: fillingCell → the interpolation-phase
     * arithmetic fill ({@code provider.fillAllDirectly(out, self)}); else
     * array-form eval of the wrapped tree. Any interpreter failure falls back
     * to the vanilla sweep — the batch path must never break worldgen.
     */
    public static void interpFillArray(final NoiseChunk.NoiseInterpolator self, final double[] out,
                                       final DensityFunction.ContextProvider provider) {
        try {
            if (INTERP_THIS0 != null) {
                final NoiseChunk chunk = (NoiseChunk) INTERP_THIS0.get(self);
                if (chunk.fillingCell) {
                    provider.fillAllDirectly(out, self);
                    return;
                }
            }
        } catch (final Throwable ignored) {
            // this$0/fillingCell unreadable (never under same-loader
            // injection) — proceed as slice fill; the eval below is still
            // provider-driven and self-contained.
        }
        try {
            eval(self.wrapped(), out, provider);
        } catch (final Throwable t) {
            try {
                provider.fillAllDirectly(out, self.wrapped());
                bump("t3.RECOVERY." + t.getClass().getSimpleName());
            } catch (final Throwable ignored) {
                // never let the interpreter break worldgen
            }
            return;
        }
        if (SHADOW_N > 0) {
            shadowVerify(self, out, provider);
        }
    }

    /**
     * Array-form evaluation of one node into all {@code out.length} slots.
     * Provider-driven: every leaf/fallback sweep goes through the caller's
     * ContextProvider, so provider protocol semantics (NoiseChunk$1 quart
     * state, TestProvider in probes) are preserved unchanged.
     */
    public static void eval(final DensityFunction node, final double[] out,
                            final DensityFunction.ContextProvider provider) {
        final int n = out.length;
        if (n == 0) {
            return;
        }

        // ---- tier 1: noise leaves (v1-proven batch kernels) ----
        if (node instanceof DensityFunctions.Noise) {
            NormalNoiseBatchOps.fillNoise((DensityFunctions.Noise) node, out, provider);
            bump("t1.Noise");
            return;
        }
        // ShiftNoise interface: ShiftA/ShiftB records; ShiftedNoise is NOT a
        // ShiftNoise (direct DensityFunction record) and therefore lands in
        // the tier-3 fallback below, per design §7.5.
        if (node instanceof DensityFunctions.ShiftNoise) {
            NormalNoiseBatchOps.fillShift((DensityFunctions.ShiftNoise) node, out, provider);
            bump("t1.ShiftNoise." + node.getClass().getSimpleName());
            return;
        }

        // ---- tier 1: constants ----
        if (node instanceof DensityFunctions.Constant) {
            Arrays.fill(out, ((DensityFunctions.Constant) node).value());
            bump("t1.Constant");
            return;
        }
        if (node instanceof DensityFunctions.YClampedGradient) {
            final DensityFunctions.YClampedGradient g = (DensityFunctions.YClampedGradient) node;
            final int fromY = g.fromY();
            final int toY = g.toY();
            final double fromV = g.fromValue();
            final double toV = g.toValue();
            for (int i = 0; i < n; i++) {
                out[i] = Mth.clampedMap((double) provider.forIndex(i).blockY(),
                        (double) fromY, (double) toY, fromV, toV);
            }
            bump("t1.YClampedGradient");
            return;
        }

        // ---- tier 2: kernel-own-transform nodes (bit-exact by construction) ----
        if (node instanceof DensityFunctions.PureTransformer) {
            // transform(double) is elementwise — in-place eval is safe.
            final DensityFunctions.PureTransformer pt = (DensityFunctions.PureTransformer) node;
            eval(pt.input(), out, provider);
            for (int i = 0; i < n; i++) {
                out[i] = pt.transform(out[i]);
            }
            bump("t2.PureTransformer." + node.getClass().getSimpleName());
            return;
        }
        if (node instanceof DensityFunctions.Ap2) {
            final DensityFunctions.Ap2 ap = (DensityFunctions.Ap2) node;
            final double[] a = new double[n];
            final double[] b = new double[n];
            eval(ap.argument1(), a, provider);
            eval(ap.argument2(), b, provider);
            // Identity compares, NOT an enum switch: an enum switch would
            // mint a synthetic $SwitchMap class the ship audit forbids.
            final DensityFunctions.TwoArgumentSimpleFunction.Type t = ap.type();
            if (t == DensityFunctions.TwoArgumentSimpleFunction.Type.ADD) {
                for (int i = 0; i < n; i++) {
                    out[i] = a[i] + b[i];
                }
            } else if (t == DensityFunctions.TwoArgumentSimpleFunction.Type.MUL) {
                for (int i = 0; i < n; i++) {
                    out[i] = a[i] * b[i];
                }
            } else if (t == DensityFunctions.TwoArgumentSimpleFunction.Type.MIN) {
                for (int i = 0; i < n; i++) {
                    out[i] = Math.min(a[i], b[i]);
                }
            } else {
                for (int i = 0; i < n; i++) {
                    out[i] = Math.max(a[i], b[i]);
                }
            }
            bump("t2.Ap2." + String.valueOf(t));
            return;
        }
        if (node instanceof DensityFunctions.HolderHolder) {
            eval(((DensityFunctions.HolderHolder) node).function().value(), out, provider);
            bump("t2.HolderHolder");
            return;
        }

        // ---- tier 3: per-point fallback = LITERALLY the vanilla sweep ----
        provider.fillAllDirectly(out, node);
        bump("t3.FALLBACK." + node.getClass().getSimpleName());
    }

    /**
     * In-server bit-exactness gate on the REAL router: the bridged column
     * fill is compared raw-bits against the EXACT vanilla fill (the default
     * fillArray body = provider.fillAllDirectly(out, wrapped)). Diagnostic
     * only — re-advances NoiseChunk state, so it must never run in
     * production/A/B boots (env CRUSSTY_V3_SHADOW_N gates it off).
     */
    private static void shadowVerify(final NoiseChunk.NoiseInterpolator self, final double[] out,
                                     final DensityFunction.ContextProvider provider) {
        if (SHADOW_LEFT.getAndDecrement() <= 0) {
            return;
        }
        try {
            final double[] oracle = new double[out.length];
            final DensityFunction wrapped = self.wrapped();
            provider.fillAllDirectly(oracle, wrapped);
            int bad = -1;
            for (int i = 0; i < out.length; i++) {
                if (Double.doubleToRawLongBits(out[i]) != Double.doubleToRawLongBits(oracle[i])) {
                    bad = i;
                    break;
                }
            }
            SHADOW_CHECKS.incrementAndGet();
            if (bad >= 0) {
                SHADOW_FAILS.incrementAndGet();
                System.out.println("[crussty] V3_SHADOW MISMATCH n=" + out.length
                        + " first@" + bad
                        + " want=" + Double.toHexString(oracle[bad])
                        + " got=" + Double.toHexString(out[bad]));
            }
            if (!SHADOW_REPORTED && SHADOW_LEFT.get() <= 0 && SHADOW_CHECKS.get() > 0) {
                SHADOW_REPORTED = true;
                System.out.println("[crussty] CRUSSTY_V3_SHADOW "
                        + (SHADOW_FAILS.get() == 0 ? "PASS" : "FAIL(" + SHADOW_FAILS.get() + ")")
                        + " checks=" + SHADOW_CHECKS.get()
                        + " " + censusDump());
            }
        } catch (final Throwable t) {
            SHADOW_FAILS.incrementAndGet();
            if (!SHADOW_REPORTED) {
                SHADOW_REPORTED = true;
                System.out.println("[crussty] CRUSSTY_V3_SHADOW FAIL(exception="
                        + t.getClass().getSimpleName() + ") " + censusDump());
            }
        }
    }
}
