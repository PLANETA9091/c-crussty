package net.minecraft.world.level;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.ticks.TickPriority;

/**
 * SCHED-DEFER (C30, ROUND-475 — rt4/bc1 thread-confinement SITE-A, spec
 * docs/RT4_CONFINEMENT_SPEC.md §4): canalization of the per-Level scheduled-tick
 * carrier (LevelTicks.allContainers / LevelChunkTicks.allTicks fastutil) against
 * concurrent worker schedules.
 *
 * Root-cause (S62 run 36288008877, DnT-dp2; totemA 16385 / Trek 16386 same class):
 * phase-3 region workers tick entities (RegionTickOps.tickBucket) whose block
 * paths (FallingBlockEntity.fall -> LiquidBlock.neighborChanged) reach
 * ScheduledTickAccess.scheduleTick -> LevelTicks.schedule -> LevelChunkTicks.add
 * (ObjectOpenCustomHashSet.rehash) while the MAIN thread mutates the SAME
 * per-Level container (TickBlockOps.runCollectedTicks -> ServerLevel.tickBlock ->
 * schedule). fastutil open-addressing under concurrent mutation loses the
 * size/table sync -> Index -1 AIOOBE on the next main-thread rehash/add ->
 * ReportedException crash x3 worlds (S25 gates fixed boot, not bench).
 *
 * Fix contract (javap-verbatim of the ScheduledTickAccess defaults, codelen
 * 23/21/23/21): in THIS kernel the four scheduleTick overloads live ONLY as
 * interface defaults on net.minecraft.world.level.ScheduledTickAccess — every
 * caller funnels through them, so a 4-site body redirect covers the whole
 * schedule surface (ServerLevel declares no overrides; scanned patched-kernel).
 *
 *  - MAIN thread: vanilla body BIT-FOR-BIT
 *    (getBlockTicks().schedule(createTick(pos, type, delay, priority))).
 *  - WORKER thread + ServerLevel receiver: DEFER the record to the per-worker
 *    TL queue (RegionTickOps.deferSchedule); the main thread replays it FIFO in
 *    phase-4c (right after the phase-4b BU drain, before workerError rethrow) —
 *    the carrier mutates MAIN-ONLY again; zero-delay schedules replayed before
 *    the same tick's block-tick drain keep vanilla timing (interleave class
 *    S7-155, shift <= 1 tick accepted by spec §3).
 *  - WORKER + any other receiver (worldgen carriers are gen-thread confined):
 *    vanilla body (no shared-carrier exposure).
 *  - FAIL-SAFE (one-shot disarm: replay Throwable or queue overflow):
 *    every path degrades to the DIRECT vanilla body — the vanilla race is
 *    preferred over any bridge panic; marker "[crussty-plugin] [S24-confinement] sched DISARM".
 */
public final class BlockScheduleOps {
    private BlockScheduleOps() {}

    /** Strict site 1/4: scheduleTick(BlockPos, Block, int, TickPriority). */
    public static void scheduleBlock(ScheduledTickAccess self, BlockPos pos,
                                     Block block, int delay, TickPriority priority) {
        if (net.minecraft.world.entity.RegionTickOps.isWorker()
                && self instanceof net.minecraft.server.level.ServerLevel
                && !net.minecraft.world.entity.RegionTickOps.scheduleDisarmed()) {
            net.minecraft.world.entity.RegionTickOps.deferSchedule(
                    self, pos, block, delay, priority, true);
            return;
        }
        block(self, pos, block, delay, priority);
    }

    /** Strict site 2/4: scheduleTick(BlockPos, Fluid, int, TickPriority). */
    public static void scheduleFluid(ScheduledTickAccess self, BlockPos pos,
                                     Fluid fluid, int delay, TickPriority priority) {
        if (net.minecraft.world.entity.RegionTickOps.isWorker()
                && self instanceof net.minecraft.server.level.ServerLevel
                && !net.minecraft.world.entity.RegionTickOps.scheduleDisarmed()) {
            net.minecraft.world.entity.RegionTickOps.deferSchedule(
                    self, pos, fluid, delay, priority, false);
            return;
        }
        fluid(self, pos, fluid, delay, priority);
    }

    /** Strict site 3/4: scheduleTick(BlockPos, Block, int) — default forwards
     *  via createTick(pos, block, delay) exactly as the vanilla default body. */
    public static void scheduleBlockNoPriority(ScheduledTickAccess self, BlockPos pos,
                                               Block block, int delay) {
        if (net.minecraft.world.entity.RegionTickOps.isWorker()
                && self instanceof net.minecraft.server.level.ServerLevel
                && !net.minecraft.world.entity.RegionTickOps.scheduleDisarmed()) {
            net.minecraft.world.entity.RegionTickOps.deferSchedule(
                    self, pos, block, delay, null, true);
            return;
        }
        noPriority(self, pos, block, delay);
    }

    /** Strict site 4/4: scheduleTick(BlockPos, Fluid, int). */
    public static void scheduleFluidNoPriority(ScheduledTickAccess self, BlockPos pos,
                                               Fluid fluid, int delay) {
        if (net.minecraft.world.entity.RegionTickOps.isWorker()
                && self instanceof net.minecraft.server.level.ServerLevel
                && !net.minecraft.world.entity.RegionTickOps.scheduleDisarmed()) {
            net.minecraft.world.entity.RegionTickOps.deferSchedule(
                    self, pos, fluid, delay, null, false);
            return;
        }
        noPriority(self, pos, fluid, delay);
    }

    // ==================================================================
    // javap-verbatim vanilla bodies (main thread / phase-4c replay).
    // ==================================================================

    /** scheduleTick(BlockPos, Block, int, TickPriority) default body. */
    public static void block(ScheduledTickAccess self, BlockPos pos,
                             Block block, int delay, TickPriority priority) {
        self.getBlockTicks().schedule(self.createTick(pos, block, delay, priority));
    }

    /** scheduleTick(BlockPos, Fluid, int, TickPriority) default body. */
    public static void fluid(ScheduledTickAccess self, BlockPos pos,
                             Fluid fluid, int delay, TickPriority priority) {
        self.getFluidTicks().schedule(self.createTick(pos, fluid, delay, priority));
    }

    /** scheduleTick(BlockPos, Block, int) default body. */
    public static void blockNoPriority(ScheduledTickAccess self, BlockPos pos,
                                       Block block, int delay) {
        self.getBlockTicks().schedule(self.createTick(pos, block, delay));
    }

    /** scheduleTick(BlockPos, Fluid, int) default body. */
    public static void fluidNoPriority(ScheduledTickAccess self, BlockPos pos,
                                       Fluid fluid, int delay) {
        self.getFluidTicks().schedule(self.createTick(pos, fluid, delay));
    }

    /** Shared no-priority body (both flavors funnel through createTick(T;I)). */
    private static void noPriority(ScheduledTickAccess self, BlockPos pos,
                                   Object type, int delay) {
        if (type instanceof Block) {
            blockNoPriority(self, pos, (Block) type, delay);
        } else if (type instanceof Fluid) {
            fluidNoPriority(self, pos, (Fluid) type, delay);
        } else {
            // Unreachable via the four retargeted defaults; kept total for
            // fail-safety (vanilla default would ClassCastException identically
            // on an unknown type — here we simply drop the no-priority record).
        }
    }
}
