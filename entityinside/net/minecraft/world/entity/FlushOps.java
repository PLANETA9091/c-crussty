package net.minecraft.world.entity;

import java.util.Collection;
import java.util.List;

/**
 * FLUSH-DIET bridge (S7-137, ARCH-ATTACK lever #4 — the
 * InsideBlockEffectApplier$StepBasedCollector.flushStep lane).
 *
 * STEP-0 javap contract (purpur-1.21.10 kernel, byte-identical to the CI
 * booted kernel): vanilla {@code flushStep()} copies the per-type before /
 * after effect lists into {@code finalEffects} via
 * {@code List.addAll(Collection)}. {@code ArrayList.addAll} resolves its
 * argument through {@code c.toArray()} BEFORE the emptiness check, so every
 * empty-list call still allocates a throwaway {@code new Object[0]} through
 * {@code Arrays.copyOf}. The census run 35275967738 (X150K, 52552 alloc
 * samples) pinned the whole lane to exactly this path:
 *
 *   336 samples (4.6% of the true 25.6GB/60s churn):
 *   StepBasedCollector.advanceStep -> flushStep -> ArrayList.addAll
 *     -> ArrayList.toArray -> Arrays.copyOf -> Object[]
 *
 * The per-entity collector runs advanceStep on every block step of the
 * inside-blocks traversal (2 call sites in the visitor) and flushStep
 * performs two addAll sites per step for every InsideBlockEffectType in
 * APPLY_ORDER — for the overwhelming majority of entities both lists are
 * empty, so the vanilla loop is a pure garbage generator at ~150k
 * entities/tick.
 *
 * This bridge is the retarget target for BOTH flushStep addAll sites
 * (invokeinterface List.addAll -> invokestatic fladd + 2 nop,
 * length-preserving). Semantics are byte-for-byte vanilla: an empty source
 * adds nothing and returns false (exactly what ArrayList.addAll returns
 * after its wasted toArray), a non-empty source is delegated to the same
 * List.addAll — element order, list identity and observable effects are
 * untouched (median-exact parity by construction).
 */
public final class FlushOps {
    private FlushOps() {}

    /**
     * Vanila-equivalent {@code dst.addAll(src)} with the emptiness gate
     * hoisted in front of the {@code toArray} resolution. Descriptor is the
     * erased form of {@code List.addAll(Collection)Z}:
     * (Ljava/util/List;Ljava/util/Collection;)Z. Raw types are deliberate —
     * the wildcard capture of {@code List<?>.addAll(Collection<?>) } cannot
     * express the receiver-passthrough, and the erased descriptor is what
     * the verifier sees.
     */
    @SuppressWarnings({"unchecked", "rawtypes"})
    public static boolean fladd(List dst, Collection src) {
        return src.isEmpty() ? false : dst.addAll(src);
    }
}
