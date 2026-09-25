package net.minecraft.world.entity;

import it.unimi.dsi.fastutil.longs.LongSet;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * INSIDE-DIET lever #12 v1 (TASK-332): glue-diet inside-blocks sweep.
 *
 * Vanilla {@code Entity.checkInsideBlocks(Vec3,Vec3,StepBasedCollector,LongSet,int)}
 * allocates, PER CALL: makeBoundingBox + deflate (2 AABB), one AtomicInteger,
 * one 10-capture lambda visitor (body-redirect target capture shape ≈ 72B).
 * This bridge replaces that glue: ONE box object (vanilla makeBoundingBox +
 * hand-inlined deflate — bit-identical fields), ONE visitor object (carries
 * the vanilla AtomicInteger counter as a plain int field) and hands the walk
 * to the REAL vanilla static BlockGetter.forEachBlockIntersectedBetween —
 * so the walk itself (fast-path, corner phase, DDA phase, visit ordering,
 * stepIdx values) is bit-exact BY CONSTRUCTION, not by transcription.
 *
 * Offline proof (v1): a full walk+DDA transcription attempt diverged on
 * 21/350k randomized lockstep scenarios and was REJECTED; v2 (walk
 * transcription with an offline lockstep proof) is a separate lever.
 *
 * KNOWN DELIBERATE DEVIATION (documented, non-observable on production/CI):
 * the visitor drops the debug-subscriber branches (debugBlockIntersection);
 * they are passive observers gated by
 * hasAnySubscriberFor(ENTITY_BLOCK_INTERSECTIONS) — false on servers without
 * the debug subscriber. Crash path keeps the ReportedException wrapping.
 */
public final class InsideDietOps {

    private InsideDietOps() {}

    /** Bit-exact replica of the vanilla 5-arg Entity.checkInsideBlocks body
     *  with glue-free per-call allocations. Receiver prepended
     *  (instance->static body redirect). */
    public static int checkInsideBlocks(Entity e, Vec3 from, Vec3 to,
                                        InsideBlockEffectApplier.StepBasedCollector collector,
                                        LongSet visited, int budget) {
        // vanilla: AABB box = makeBoundingBox(to).deflate(9.999999747378752E-6)
        // AABB.deflate(d) = new AABB(minX+d, minY+d, minZ+d, maxX-d, maxY-d, maxZ-d)
        AABB full = e.makeBoundingBox(to);
        AABB box = new AABB(full.minX + 9.999999747378752E-6,
                            full.minY + 9.999999747378752E-6,
                            full.minZ + 9.999999747378752E-6,
                            full.maxX - 9.999999747378752E-6,
                            full.maxY - 9.999999747378752E-6,
                            full.maxZ - 9.999999747378752E-6);
        // vanilla: movedFar = from.distanceToSqr(to) > Mth.square(0.9999900000002526d)
        boolean movedFar = from.distanceToSqr(to) > Mth.square(0.9999900000002526);
        // vanilla: counter = new AtomicInteger() — now a plain visitor field.
        InsideDietVisitor v = new InsideDietVisitor(e, budget, from, to, visited,
                                                    movedFar, box, collector);
        // vanilla walk — bit-exact by construction (v1 scope decision).
        BlockGetter.forEachBlockIntersectedBetween(from, to, box, v);
        return v.counter + 1;
    }

    // =====================================================================
    // ID-P35 DDA-v2 HYBRID SHADOW (TASK-459-75) — SCAFFOLD, DORMANT.
    //
    // The vanilla walk above stays the SOLE executor (source of truth). The
    // v2 shadow is a sampled (1/200, deterministic ordinal) PARALLEL
    // verification of the DDA walk transcription (src/inside_dda_v2.rs
    // offline model); ANY divergence trips the one-shot disarm latch below
    // (batchOk pattern, nav_plane canon) and the shadow never runs again —
    // vanilla continues bit-for-bit.
    //
    // NCDFE-canon: P35_LEVER is false, NOTHING in the active bytecode calls
    // any member of this section, the class is NOT rebuilt (include_bytes!
    // in inside_diet.rs serves the v1 bytecode) and no retarget composes —
    // vanilla by construction. The future JNI surface (ddaV2Verify) is
    // registered RegisterNatives-style ONLY on the just-defined class AFTER
    // define (define-BEFORE-arm; NCDFE storm guard, zero_alloc leg#1 lesson).
    // javac-rebuild of this class = step of the next leg.
    // =====================================================================

    /** Dormant lever flag: shadow verification is OFF until the offline
     *  1M-scene gate (disarm-fires = 0) is met and the leg explicitly arms. */
    static final boolean P35_LEVER = false;

    /** One-shot disarm latch (batchOk pattern): first divergence/ERR sets it
     *  false for the rest of the process — no re-arm path. */
    static volatile boolean ddav2Ok = true;

    /** Deterministic 1/200 sampler ordinal (lockstep-reproducible, no RNG). */
    static long ddav2CallOrdinal = 0L;

    /** Sampler period: verify every 200th call, legs-only (CPU bound). */
    static final int DDAV2_SAMPLE_EVERY = 200;

    /** Disarm firings + verdict counters (DATA-PLAN for the leg gate). */
    static volatile long ddav2Divergences = 0L;
    static volatile long ddav2Verified = 0L;
    static volatile long ddav2Matched = 0L;

    /** @return true iff THIS call must run the shadow verification. */
    static boolean ddaV2Sample() {
        return P35_LEVER && ddav2Ok && (ddav2CallOrdinal++ % DDAV2_SAMPLE_EVERY == 0);
    }

    /** One-shot disarm (batchOk pattern): mirror of
     *  inside_dda_v2::disarm in src/inside_dda_v2.rs. */
    static void ddaV2Disarm(String reason) {
        if (ddav2Ok) {
            ddav2Ok = false;
            ddav2Divergences++;
            System.err.println(
                "[crussty-plugin] inside_dda_v2: DISARM (" + reason
                + ") — shadow verifier off, vanilla walk remains the executor");
        }
    }
}
