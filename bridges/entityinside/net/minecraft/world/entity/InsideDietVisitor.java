package net.minecraft.world.entity;

import it.unimi.dsi.fastutil.longs.LongSet;
import net.minecraft.CrashReport;
import net.minecraft.CrashReportCategory;
import net.minecraft.ReportedException;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.Shapes;
import net.minecraft.world.phys.shapes.VoxelShape;

/**
 * INSIDE-DIET lever #12 v1 (TASK-332): per-call visitor for the inside-blocks
 * walk — ONE small object replacing the vanilla 10-capture lambda + AtomicInteger.
 *
 * visit() is a faithful transcription of the vanilla
 * {@code Entity.lambda$checkInsideBlocks$2} bytecode (purpur-1.21.10 kernel,
 * byte-identical to the CI booted kernel): same gates, same state lookups,
 * same effect application order, same visit booleans, same crash wrapping.
 * The ONLY deliberate deviation: the debug-subscriber branches
 * (debugBlockIntersection calls) are dropped — passive observers gated by
 * hasAnySubscriberFor(ENTITY_BLOCK_INTERSECTIONS), false on production/CI.
 */
public final class InsideDietVisitor implements BlockGetter.BlockStepVisitor {

    /** Vanilla AtomicInteger counter — last passed walkStep (0 initial). */
    public int counter;

    private final Entity e;
    private final int budget;
    private final Vec3 from;
    private final Vec3 to;
    private final LongSet visited;
    private final boolean movedFar;
    private final AABB box;
    private final InsideBlockEffectApplier.StepBasedCollector collector;

    public InsideDietVisitor(Entity e, int budget, Vec3 from, Vec3 to,
                             LongSet visited, boolean movedFar, AABB box,
                             InsideBlockEffectApplier.StepBasedCollector collector) {
        this.e = e;
        this.budget = budget;
        this.from = from;
        this.to = to;
        this.visited = visited;
        this.movedFar = movedFar;
        this.box = box;
        this.collector = collector;
    }

    @Override
    public boolean visit(BlockPos pos, int walkStep) {
        if (!this.e.isAlive()) {
            return false;
        }
        if (walkStep < this.budget) {
            return false;
        }
        this.counter = walkStep;
        Level level = this.e.level();
        BlockState state = level.getBlockState(pos);
        if (state.isAir()) {
            // vanilla debug branch dropped (see class doc)
            return true;
        }
        VoxelShape shape = state.getEntityInsideCollisionShape(level, pos, this.e);
        boolean movedThrough;
        if (shape != Shapes.block()) {
            movedThrough = this.e.collidedWithShapeMovingFrom(this.from, this.to,
                    shape.move(new Vec3(pos)).toAabbs());
        } else {
            movedThrough = true;
        }
        boolean inFluid = this.e.collidedWithFluid(state.getFluidState(), pos,
                this.from, this.to);
        if (movedThrough || inFluid) {
            if (this.visited.add(pos.asLong())) {
                boolean collidedInBox = movedThrough && (!this.movedFar || this.box.intersects(pos));
                try {
                    this.collector.advanceStep(walkStep, pos);
                    state.entityInside(level, pos, this.e, this.collector, collidedInBox);
                    this.e.onInsideBlock(state);
                } catch (Throwable t) {
                    CrashReport report = CrashReport.forThrowable(t, "Colliding entity with block");
                    CrashReportCategory blockCat = report.addCategory("Block being collided with");
                    CrashReportCategory.populateBlockDetails(blockCat, level, pos.immutable(), state);
                    CrashReportCategory entityCat = report.addCategory("Entity being checked for collision");
                    this.e.fillCrashReportCategory(entityCat);
                    throw new ReportedException(report);
                }
                // vanilla bytecode 287-315: fluid step advance INSIDE the
                // visited.add==true branch (add==false goes straight to
                // return true — skipping the fluid advance too)
                if (inFluid) {
                    this.collector.advanceStep(walkStep, pos);
                    state.getFluidState().entityInside(level, pos, this.e, this.collector);
                }
            }
        }
        // vanilla debug branch dropped (see class doc)
        return true;
    }
}
