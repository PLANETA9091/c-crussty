package net.minecraft.world.level.levelgen;

import java.util.Arrays;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import net.minecraft.util.Mth;

/**
 * TASK-108 v3 (PROGRESS-6, agent-7625532f): density tree ARRAY-FORM
 * interpreter — the design is docs/BATCH_BRIDGE_DESIGN.md §7.3-7.5.
 *
 * Contract: {@link #eval} fills {@code out[0..n)} with the same values the
 * vanilla per-point evaluation would produce, but re-ordered so that every
 * batchable leaf is served by ONE native batch crossing (v1 machinery:
 * {@link NormalNoiseBatchOps#fillNoise} / {@link #fillShift}), every
 * combinator is a plain Java array loop, and every unknown node falls back to
 * the LITERAL vanilla sweep ({@code provider.fillAllDirectly(out, node)}) —
 * bit-exactness by construction for the fallback tier.
 *
 * Bit-exactness ledger (why each tier is safe):
 * - Noise/ShiftA/ShiftB leaves: v1 kernels, live smoke-7 selfTest PASS
 *   (PROGRESS-4) + G-ABI-2 standalone probe (PROGRESS-2).
 * - Constant: IEEE fill.
 * - YClampedGradient: same static call Mth.clampedMap((double)blockY,
 *   fromY, toY, fromValue, toValue) as kernel compute (javap 1.21.10).
 * - PureTransformer (Mapped/Clamp/MulOrAdd): the kernel's OWN public
 *   transform(double) applied elementwise — bit-exact by construction.
 * - Ap2: vanilla compute is tableswitch { dadd, dmul, Math.min, Math.max }
 *   (javap 1.21.10) — mirrored exactly.
 * - HolderHolder: pure delegation.
 * - tier 3 fallback: the provider's own fillAllDirectly = the vanilla
 *   default fillArray body, protocol identical (NoiseChunk$1 semantics).
 *
 * Ordering note: unlike vanilla (one interleaved sweep), this interpreter
 * drives one provider sweep per node. Every sweep is internally
 * vanilla-identical (same forIndex sequence), and NoiseChunk point-state
 * (arrayIndex, interpolationCounter) is only used by caches as a
 * point-identity token, never as a magnitude — so re-sweeps cannot change
 * values. The pre-registered v3 gate (bit-exact selftest on the REAL router,
 * in-server) is the arbiter; this class never ships past a FAIL.
 *
 * Census: every eval bumps a per-node-type counter (censusDump()); the
 * in-server selftest prints it so the fallback share per router is visible
 * (design §7.5 strategy).
 */
public final class DensityArrayInterpreter {

    private static final ConcurrentHashMap<String, AtomicLong> CENSUS = new ConcurrentHashMap<>();

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

    private DensityArrayInterpreter() {}

    /** Resets the node census (selftest scaffolding). */
    public static void censusReset() {
        CENSUS.clear();
    }

    /** Sorted "key=count" census dump for selftest/bench logs. */
    public static String censusDump() {
        final StringBuilder sb = new StringBuilder("CENSUS");
        CENSUS.entrySet().stream()
                .sorted((a, b) -> a.getKey().compareTo(b.getKey()))
                .forEach(e -> sb.append(' ').append(e.getKey()).append('=').append(e.getValue().get()));
        return sb.toString();
    }

    private static void bump(final String key) {
        CENSUS.computeIfAbsent(key, k -> new AtomicLong()).incrementAndGet();
    }

    /**
     * Whole-body replacement target for
     * {@code NoiseChunk$NoiseInterpolator.fillArray(double[], ContextProvider)}
     * (v3 Rust wiring lands next tick). Vanilla branch contract preserved:
     * fillingCell → the interpolation-phase arithmetic fill
     * ({@code provider.fillAllDirectly(out, self)}); else array-form eval of
     * the wrapped tree. Any interpreter failure falls back to the vanilla
     * sweep — the batch path must never break worldgen.
     *
     * NOTE: the owning NoiseChunk (synthetic {@code this$0}) is read
     * reflectively (see INTERP_THIS0); this class is injected into the same
     * runtime package/classloader as the kernel (v1 OPS_EMBEDS precedent).
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
            } catch (final Throwable ignored) {
                // never let the interpreter break worldgen
            }
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
        if (node instanceof DensityFunctions.Noise leaf) {
            NormalNoiseBatchOps.fillNoise(leaf, out, provider);
            bump("t1.Noise");
            return;
        }
        // ShiftNoise interface: ShiftA/ShiftB records; ShiftedNoise is NOT a
        // ShiftNoise (direct DensityFunction record) and therefore lands in
        // the tier-3 fallback below, per design §7.5.
        if (node instanceof DensityFunctions.ShiftNoise leaf) {
            NormalNoiseBatchOps.fillShift(leaf, out, provider);
            bump("t1.ShiftNoise." + leaf.getClass().getSimpleName());
            return;
        }

        // ---- tier 1: constants ----
        if (node instanceof DensityFunctions.Constant c) {
            Arrays.fill(out, c.value());
            bump("t1.Constant");
            return;
        }
        if (node instanceof DensityFunctions.YClampedGradient g) {
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
        if (node instanceof DensityFunctions.PureTransformer pt) {
            // transform(double) is elementwise — in-place eval is safe.
            eval(pt.input(), out, provider);
            for (int i = 0; i < n; i++) {
                out[i] = pt.transform(out[i]);
            }
            bump("t2.PureTransformer." + node.getClass().getSimpleName());
            return;
        }
        if (node instanceof DensityFunctions.Ap2 ap) {
            final double[] a = new double[n];
            final double[] b = new double[n];
            eval(ap.argument1(), a, provider);
            eval(ap.argument2(), b, provider);
            switch (ap.type()) {
                case ADD -> {
                    for (int i = 0; i < n; i++) {
                        out[i] = a[i] + b[i];
                    }
                }
                case MUL -> {
                    for (int i = 0; i < n; i++) {
                        out[i] = a[i] * b[i];
                    }
                }
                case MIN -> {
                    for (int i = 0; i < n; i++) {
                        out[i] = Math.min(a[i], b[i]);
                    }
                }
                case MAX -> {
                    for (int i = 0; i < n; i++) {
                        out[i] = Math.max(a[i], b[i]);
                    }
                }
            }
            bump("t2.Ap2." + ap.type());
            return;
        }
        if (node instanceof DensityFunctions.HolderHolder hh) {
            eval(hh.function().value(), out, provider);
            bump("t2.HolderHolder");
            return;
        }

        // ---- tier 3: per-point fallback = LITERALLY the vanilla sweep ----
        provider.fillAllDirectly(out, node);
        bump("t3.FALLBACK." + node.getClass().getSimpleName());
    }
}
