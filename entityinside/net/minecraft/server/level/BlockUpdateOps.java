package net.minecraft.server.level;

import it.unimi.dsi.fastutil.objects.ObjectArrayList;
import net.minecraft.Util;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.navigation.PathNavigation;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.shapes.BooleanOp;
import net.minecraft.world.phys.shapes.Shapes;

/**
 * BU-DEFER (S7-168, STEAL v2 defect-fix — TASK-335).
 *
 * s7176 crash root-cause (run 35452002378): region worker tics an entity
 * (RegionTickOps.stealChunks -> Zombie.travel -> checkFallDamage ->
 * FarmBlock.turnToDirt -> Level.setBlockAndUpdate) which reaches
 * ServerLevel.sendBlockUpdated; the vanilla body iterates
 * ServerLevel.navigatingMobs (fastutil ObjectOpenHashSet) under the
 * isUpdatingNavigations latch that in vanilla only the Server thread can
 * hold. A worker iterating while the main thread mutates the same set
 * detonates ObjectOpenHashSet$SetIterator NPE ("wrapped is null") — and the
 * main thread then dies on the same set (ReportedException "Exception while
 * updating neighbours") -> graceful shutdown mid-soak.
 *
 * Fix contract (javap-verbatim, ServerLevel.class codelen=235):
 *  - WORKER thread: execute the order-safe prefix inline (chunkSource.
 *    blockChanged + pathTypesByPosCache.invalidate + shapes-delta gate) and
 *    DEFER only the navigatingMobs pass: the main thread replays it in the
 *    exact enqueue order in RegionTickOps phase 4 (after the DONE barrier,
 *    main-only) — same FIFO replay discipline as the EntityCallbacks Mut
 *    queue. blockChanged/invalidate are already main-or-worker safe in
 *    Paper (per-chunk changed queue + PathTypeCache concurrent map).
 *  - MAIN thread: reproduce the vanilla body BIT-FOR-BIT (the redirect
 *    replaces the original body, so the bridge IS the vanilla path here;
 *    the 160..171 dead block in the original is unreachable and the
 *    else-branch it represents is Level.sendBlockUpdated — absent from
 *    Level.class, i.e. the replay never recurses).
 *
 * Delivery: defined into the KERNEL loader by region_threads activation
 * (same package as ServerLevel, so the package-private navigatingMobs /
 * isUpdatingNavigations fields resolve at runtime; the private
 * pathTypesByPosCache goes through sun.misc.Unsafe, same pattern as
 * ZeroAllocOps for Entity private fields).
 *
 * Parity: the main-thread path is javap-verbatim; the worker deferral only
 * shifts the navigatingMobs recomputePath replay to the end-of-phase point
 * (the accepted interleave class of STEAL, S7-155).
 */
public final class BlockUpdateOps {
    private BlockUpdateOps() {}

    // Unsafe accessor for the PRIVATE ServerLevel.pathTypesByPosCache.
    private static final sun.misc.Unsafe UNSAFE;
    private static final long PATH_TYPES_OFFSET;

    static {
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            java.lang.reflect.Field f = ServerLevel.class.getDeclaredField("pathTypesByPosCache");
            PATH_TYPES_OFFSET = UNSAFE.objectFieldOffset(f);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /** Receiver-prepended entry the classfile.rs retarget emits. */
    public static void handle(ServerLevel level, BlockPos pos, BlockState oldS,
                              BlockState newS, int flags) {
        if (net.minecraft.world.entity.RegionTickOps.isWorker()) {
            // Order-safe inline prefix, then defer the navigatingMobs pass.
            level.getChunkSource().blockChanged(pos);
            pathTypes(level).invalidate(pos);
            io.papermc.paper.configuration.WorldConfiguration.Misc misc =
                    level.paperConfig().misc;
            if (misc.updatePathfindingOnBlockUpdate
                    && Shapes.joinIsNotEmpty(
                            oldS.getCollisionShape(level, pos),
                            newS.getCollisionShape(level, pos),
                            BooleanOp.NOT_SAME)) {
                net.minecraft.world.entity.RegionTickOps.deferBlockUpdate(
                        level, pos, oldS, newS, flags);
            }
            return;
        }
        vanilla(level, pos, oldS, newS, flags);
    }

    /** javap-verbatim vanilla body (main thread / drain replay). */
    public static void vanilla(ServerLevel level, BlockPos pos, BlockState oldS,
                               BlockState newS, int flags) {
        if (level.isUpdatingNavigations) {
            Util.logAndPauseIfInIde(
                    "Detected use of sendBlockUpdated while updating navigations",
                    new IllegalStateException(
                            "Thread is already updating navigations"));
        }
        level.getChunkSource().blockChanged(pos);
        pathTypes(level).invalidate(pos);
        io.papermc.paper.configuration.WorldConfiguration.Misc misc =
                level.paperConfig().misc;
        if (!misc.updatePathfindingOnBlockUpdate) {
            return;
        }
        if (!Shapes.joinIsNotEmpty(
                oldS.getCollisionShape(level, pos),
                newS.getCollisionShape(level, pos),
                BooleanOp.NOT_SAME)) {
            return;
        }
        level.isUpdatingNavigations = true;
        try {
            ObjectArrayList<PathNavigation> list = new ObjectArrayList<>();
            for (Object o : level.navigatingMobs) {
                Mob mob = (Mob) o;
                PathNavigation nav = mob.getNavigation();
                if (!nav.shouldRecomputePath(pos)) {
                    continue;
                }
                list.add(nav);
            }
            for (int i = 0; i < list.size(); i++) {
                list.get(i).recomputePath();
            }
        } finally {
            level.isUpdatingNavigations = false;
        }
    }

    private static net.minecraft.world.level.pathfinder.PathTypeCache pathTypes(
            ServerLevel level) {
        return (net.minecraft.world.level.pathfinder.PathTypeCache)
                UNSAFE.getObject(level, PATH_TYPES_OFFSET);
    }
}
