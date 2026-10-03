package net.minecraft.server.level;

import ca.spottedleaf.moonrise.common.list.ReferenceList;
import ca.spottedleaf.moonrise.patches.chunk_system.entity.ChunkSystemEntity;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemServerLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.server.ServerEntityLookup;
import ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerEntity;
import ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerTrackedEntity;
import net.minecraft.world.entity.Entity;

/**
 * TrackerTickOps (S7-158b, TASK-298 — ARCH-ATTACK lever #7 hardening:
 * region-threaded entity ticking, removal-safe tracker sweep).
 *
 * Live crash 35363758352 (leg #2 soak tail, 15:54:54): vanilla
 * {@code ChunkMap.newTrackerTick} sweeps
 * {@code ServerEntityLookup.trackerEntities.getRawDataUnchecked()} — an
 * UNCHECKED array walk with the length captured once — and calls
 * {@code entity.moonrise$getTrackedEntity()} with no null guard (javap line
 * table: line 1017 = the getTrackedEntity site). Under REGION-THREADS the
 * parallel entity phase removes entities from the world ON WORKER THREADS;
 * the moonrise ReferenceList swap-remove nulls tail slots of the same
 * backing array, and the main thread's next sweep — running with a stale
 * captured length whenever a removal lands inside its loop window — read a
 * nulled slot and died with
 * {@code NullPointerException: ... "entity" is null} (server fatal).
 *
 * This bridge is the vanilla body BYTE-FOR-BYTE (same interfaces, same
 * call order, javap-verified against purpur-1.21.10 kernel e2992d63) plus
 * exactly one difference: nulled slots are SKIPPED instead of fatal — the
 * "removal-safe iteration" option of the preregistered S7-158 plan (the
 * alternative — deferring EntityLookup removals through the FIFO — would
 * change same-tick broadphase visibility; the guard keeps every worker-side
 * timing vanilla). Entries skipped by the guard are entities vanilla would
 * have CRASHED on; in the race-free vanilla world the state is unreachable,
 * so the observable semantics of the race-free server are unchanged.
 *
 * Wiring: retarget of the ONLY {@code newTrackerTick} call site in the
 * kernel ({@code ChunkMap.tick()V}, whose body is exactly that single
 * call) to {@link #newTrackerTick(ChunkMap)} — strict Retargeted{1},
 * fail-closed (rust region_threads v3). Defined into the KERNEL loader in
 * net.minecraft.server.level (same-package access to the package-private
 * ChunkMap$TrackedEntity / ServerEntity members, same-loader protocol).
 *
 * The sweep loop is factored into {@link #sweep(ServerEntityLookup)} so the
 * OFFLINE harness can regression-test the null guard over REAL kernel
 * classes without booting a level.
 */
public final class TrackerTickOps {

    private TrackerTickOps() {}

    /** Retarget of the single {@code ChunkMap.tick()V -> newTrackerTick()} site. */
    public static void newTrackerTick(ChunkMap map) {
        ServerEntityLookup entityLookup =
                (ServerEntityLookup) ((ChunkSystemServerLevel) map.level).moonrise$getEntityLookup();
        sweep(entityLookup);
    }

    /** Vanilla newTrackerTick body (javap-verified) + removal-safe guard. */
    public static void sweep(ServerEntityLookup entityLookup) {
        ReferenceList<Entity> trackerEntities = entityLookup.trackerEntities;
        Entity[] trackerEntitiesRaw = (Entity[]) trackerEntities.getRawDataUnchecked();
        for (int i = 0, len = trackerEntities.size(); i < len; i++) {
            Entity entity = trackerEntitiesRaw[i];
            // S7-158b removal-safe guard: a parallel worker's entity removal
            // swap-removed this slot (ReferenceList nulls the tail). Vanilla
            // NPEs here ("entity" is null, ChunkMap.java:1017) — skip instead.
            if (entity == null) {
                continue;
            }
            ChunkMap.TrackedEntity tracker = entity.moonrise$getTrackedEntity();
            if (tracker == null) {
                continue; // vanilla (bytecode 57: ifnonnull / goto next)
            }
            ((EntityTrackerTrackedEntity) tracker).moonrise$tick(
                    ((ChunkSystemEntity) entity).moonrise$getChunkData().nearbyPlayers);
            if (((EntityTrackerTrackedEntity) tracker).moonrise$hasPlayers()
                    || ((ChunkSystemEntity) entity).moonrise$getChunkStatus()
                            .isOrAfter(FullChunkStatus.ENTITY_TICKING)) {
                tracker.serverEntity.sendChanges();
            }
        }
    }
}
