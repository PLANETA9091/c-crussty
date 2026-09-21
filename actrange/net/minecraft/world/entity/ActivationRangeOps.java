package net.minecraft.world.entity;

import io.papermc.paper.entity.activation.ActivationType;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.logging.Logger;
import net.minecraft.world.TickRateManager;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.Level;

/**
 * ACTRANGE (TASK-400-H, vector H — activation-range / DAB port, lever cmp399_dab).
 *
 * Port of the upstream activation-range architecture family onto the measured
 * 150k-entity scene, fused from two upstream designs:
 *
 *  1. Paper "Entity Activation Range 2.0" (io.papermc.paper.entity.activation.
 *     ActivationRange): per-type taxonomy via Entity.activationType (7 types)
 *     + Entity.defaultActivationState (immunity list) + per-type ranges
 *     (spigot.yml defaults: animals 32 / monsters 32 / raiders 64 / misc 16 /
 *     water 16 / villagers 32 / flying-monsters 32).
 *
 *  2. Pufferfish DAB — "Dynamic Activation of Brain" (pufferfish-gg/Pufferfish,
 *     minecraft-patches/features/0009-Dynamic-Activation-of-Brain.patch,
 *     Paul Sauve/Airplane 2021): per-entity-type activation bounds around
 *     players; entities far away get their BRAINS switched off
 *     ("Optimizes entity brains when they're far away from the player").
 *
 * SEMANTICS (documented superiority deviation — see LEVER-H.md):
 *   inside ANY player's per-type box  -> exact vanilla tick (byte-for-byte path);
 *   outside ALL boxes                 -> HARD FREEZE = vanilla 1.20.3+ `/tick
 *      freeze` semantics (TickRateManager.isEntityFrozen): the per-entity
 *      consumer skips checkDespawn + guardEntityTick entirely, so no tick, no
 *      inactiveTick, no tickCount++ — but the entity STAYS in the world
 *      (entityTickList / sections / getEntities / fixtures all see it).
 *      This is STRICTER than upstream DAB (which staggers far entities at
 *      freq = dist^2 >> mod) and stricter than Paper-EAR (whose inactiveTick
 *      still runs GoalSelector canUse evaluation every 20 ticks — the
 *      residual nav_ai lane measured in round-anchor1, 998 collapsed stacks).
 *
 * WHY THE SINGLE HOOK: TickRateManager.isEntityFrozen has exactly ONE call
 * site in the kernel — the head of ServerLevel.lambda$tick$4 (the per-entity
 * consumer of the main entity tick loop, the very consumer RegionTickOps
 * invokes under region_threads>=2). Redirecting that one site freezes through
 * BOTH the vanilla loop and the region-threaded loop. The other two
 * retargets (activateEntities -> no-op, checkIfActive -> const true) remove
 * the Paper bookkeeping that is dead under hard-freeze semantics:
 *   - activateEntities: per-player AABB section scans per level tick +
 *     activatedTick writes (nothing reads activatedTick anymore);
 *   - checkIfActive: entities reaching tickNonPassenger are by definition
 *     non-frozen, they MUST full-tick (a const true prevents the no-op
 *     activateEntities from accidentally demoting actives to inactiveTick).
 *
 * PARITY GUARDS:
 *   - global /tick freeze (trm.isEntityFrozen) is consulted FIRST — vanilla
 *     global freeze semantics preserved bit-for-bit;
 *   - players, Paper's defaultActivationState immunity list, ItemEntity
 *     (cmp399_* arms the J-subsystem which ticks items outside this path;
 *     despawn/merge lanes stay anchor-parity) and player-passenger carriers
 *     are never frozen;
 *   - empty players() list -> nothing frozen (bench-3 no-player parity);
 *   - any Throwable -> vanilla verdict (fail-open to vanilla behavior).
 *
 * ZERO-ALLOC hot path: scalar Chebyshev box test (dx,dy,dz <= range) against
 * the live players() list (4 fake players in the bench); no iterators, no
 * boxing, no AABB allocations per entity per tick; the 30s stats heartbeat is
 * an inline CAS-guarded logger — no timer threads, no extra classes (the
 * bridge must stay a SINGLE classfile for the kernel-loader define).
 *
 * NOT FOR PRODUCTION. Bench lever under CRUSSTY_LEVER_FLAG=cmp399_dab
 * (rust side: src/actrange.rs — defines this class into the kernel loader,
 * applies the three ServerLevel retargets, retransforms, verifies).
 */
public final class ActivationRangeOps {

    private static final Logger LOG = Logger.getLogger("crussty-plugin");
    private static final long STATS_INTERVAL_NANOS = 30_000_000_000L;
    static final String MARK = "[crussty-plugin] cmp399_dab:";

    // ---- per-type activation bounds (Pufferfish DAB-style per-type boxes;
    // ---- values = spigot.yml entity-activation-range defaults, verbatim) --
    static volatile int animal = 32;
    static volatile int monster = 32;
    static volatile int villager = 32;
    static volatile int raider = 64;
    static volatile int water = 16;
    static volatile int flying = 32;
    static volatile int misc = 16;

    // ---- observability (30s stats line in server-stdout.log = ARM evidence)
    static final AtomicLong FROZEN = new AtomicLong();
    static final AtomicLong ACTIVE = new AtomicLong();
    static final AtomicLong LAST_STATS = new AtomicLong(System.nanoTime());
    static volatile String firstFreezeType = null;

    private ActivationRangeOps() {
    }

    /** Per-type range lookup (identity compares; no enum-switch synthetics). */
    static int rangeFor(ActivationType t) {
        if (t == ActivationType.ANIMAL) {
            return animal;
        }
        if (t == ActivationType.MONSTER) {
            return monster;
        }
        if (t == ActivationType.VILLAGER) {
            return villager;
        }
        if (t == ActivationType.RAIDER) {
            return raider;
        }
        if (t == ActivationType.WATER) {
            return water;
        }
        if (t == ActivationType.FLYING_MONSTER) {
            return flying;
        }
        return misc;
    }

    /**
     * Retarget #1 (ServerLevel.lambda$tick$4, the per-entity tick-loop
     * consumer): vanilla-freeze parity first, then the per-type activation
     * bounds. TRUE = frozen (skip checkDespawn + tick entirely).
     */
    public static boolean isEntityFrozen(TickRateManager trm, Entity entity) {
        try {
            if (trm.isEntityFrozen(entity)) {
                FROZEN.incrementAndGet();
                return true; // vanilla /tick freeze — byte-for-byte parity
            }
            // Immunities: Paper's defaultActivationState list (players,
            // projectiles, TNT, minecarts, boats, ...), items (J-subsystem
            // owns the item lane under cmp399_*), player-carried vehicles.
            if (entity.defaultActivationState
                    || entity instanceof ItemEntity
                    || entity.countPlayerPassengers() > 0) {
                return false;
            }
            int range = rangeFor(entity.activationType);
            if (range <= 0) {
                return false; // 0 = vanilla behavior (spigot EAR semantics)
            }
            Level level = entity.level();
            List<? extends Player> players = level.players();
            if (players.isEmpty()) {
                return false; // no players -> DAB has no anchor -> vanilla
            }
            double ex = entity.getX(), ey = entity.getY(), ez = entity.getZ();
            for (int i = 0; i < players.size(); i++) {
                Player p = players.get(i);
                if (p.isSpectator()) {
                    continue;
                }
                double dx = p.getX() - ex;
                if (dx < 0.0D) dx = -dx;
                if (dx > range) continue;
                double dy = p.getY() - ey;
                if (dy < 0.0D) dy = -dy;
                if (dy > range) continue;
                double dz = p.getZ() - ez;
                if (dz < 0.0D) dz = -dz;
                if (dz > range) continue;
                ACTIVE.incrementAndGet();
                return false; // inside this player's per-type box -> full tick
            }
            long f = FROZEN.incrementAndGet();
            if (f == 1L) {
                firstFreezeType = entity.getClass().getSimpleName();
                LOG.info(MARK + " first freeze verdict: " + firstFreezeType
                        + " (outside all per-type boxes; /tick-freeze semantics)");
            }
            maybeStats();
            return true;
        } catch (Throwable t) {
            try {
                return trm.isEntityFrozen(entity); // fail-open = vanilla
            } catch (Throwable suppressed) {
                return false;
            }
        }
    }

    /** CAS-guarded 30s heartbeat: at most one log line per window. */
    private static void maybeStats() {
        long now = System.nanoTime();
        long last = LAST_STATS.get();
        if (now - last >= STATS_INTERVAL_NANOS && LAST_STATS.compareAndSet(last, now)) {
            LOG.info(MARK + " stats(30s): frozen=" + FROZEN.get()
                    + " active=" + ACTIVE.get());
        }
    }

    /**
     * Retarget #2 (ServerLevel.tick()V): Paper's per-player pre-activation
     * scan (activateEntities) is dead weight under hard-freeze semantics —
     * the per-entity verdict happens in the tick loop itself.
     */
    public static void activateEntities(Level level) {
        // no-op (documented in LEVER-H.md)
    }

    /**
     * Retarget #3 (ServerLevel.tickNonPassenger): everything that reaches
     * tickNonPassenger is non-frozen by construction and MUST full-tick.
     */
    public static boolean checkIfActive(Entity entity) {
        return true;
    }

    /** Reflective self-test (rust side calls post-define, pre-retransform). */
    public static boolean selfTest() {
        try {
            boolean ok = animal == 32 && monster == 32 && villager == 32
                    && raider == 64 && water == 16 && flying == 32 && misc == 16
                    && ActivationType.ANIMAL != null && ActivationType.MISC != null;
            LOG.info(MARK + " selfTest: preset dab-hard-1 ranges ok=" + ok
                    + " (animal=32 monster=32 villager=32 raider=64 water=16"
                    + " flying=32 misc=16; immune=defaultActivationState+items;"
                    + " freeze=/tick-freeze semantics)");
            return ok;
        } catch (Throwable t) {
            LOG.warning(MARK + " selfTest failed: " + t);
            return false;
        }
    }
}
