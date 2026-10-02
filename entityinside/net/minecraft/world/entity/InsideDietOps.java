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
}
