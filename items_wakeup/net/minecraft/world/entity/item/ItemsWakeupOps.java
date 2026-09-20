package net.minecraft.world.entity.item;

import it.unimi.dsi.fastutil.longs.Long2DoubleOpenHashMap;
import it.unimi.dsi.fastutil.longs.LongOpenHashSet;
import java.lang.reflect.Method;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.phys.AABB;

/**
 * TASK-396-I (MEGA-ROUND-1, vector I) — EVENT-DRIVEN WAKEUP-LIST for
 * ItemEntity merge scans (lever_flag "items_wakeup").
 *
 * PROBLEM (baseline bank v4, s7206#3, 115655 samples): items/ItemEntity.tick
 * 31.17% java (TOP-1), broadphase 15.66% (getEntities from item merges),
 * fastutil 8.54% + java_util 7.01% (collections of the same queries). ~70% of
 * the 150k population are item entities.
 *
 * VANILLA FACTS (javap purpur-1.21.10 + 1.21.x source mirror):
 *   - merge scans are NOT per-tick: tick() gates mergeWithNeighbours() by
 *     `tickCount % (crossedBlockBoundary ? 2 : 40) == 0` — vanilla itself
 *     throttles the scan; stationary items rescan every 40 ticks.
 *   - teleport() holds the second (and last) invoke site: scan right after a
 *     teleport (event-driven in vanilla).
 *   - mergeWithNeighbours() = level.getEntitiesOfClass(ItemEntity.class,
 *     AABB.inflate(itemMerge, ...), selector) + iterate + tryToMerge.
 *     One broadphase query + one List alloc PER SCAN PER ITEM: O(cluster).
 *   - isMergable(): isAlive && pickupDelay != 32767 && age != -32768 &&
 *     age < despawnRate && count < maxStackSize. NOTE: pickupDelay > 0 does
 *     NOT block merging — only the 32767 sentinel does.
 *
 * ARCHITECTURE (per-entity event SCHEDULING — not result memoization, not
 * per-section chunk stamps; architecturally distinct from the refuted
 * fluid_dirty-memo S7-153):
 *   The two mergeWithNeighbours call sites are retargeted 1:1 (receiver
 *   prepended, identical stack shape) to the statics below. A scan (the REAL
 *   vanilla body, invoked via reflection delegate — zero logic drift) runs
 *   only on an EVENT:
 *     E1 first sighting of the entity id (spawn / chunk load-in): no stamp;
 *     E2 accumulated movement > 0.25 blocks since the last scan (per-entity
 *        x/y/z stamps, the task-spec threshold);
 *     E3 active flag (one-shot wake), set by merge-driven invalidation;
 *     E4 neighbour wake: after a scan that ACTUALLY merged something
 *        (self removed or stack count changed), every ItemEntity within
 *        itemMerge radius of the scan position is woken. This covers
 *        partial-merge donors whose remainder stack is mutated in place
 *        (ItemStack.shrink) with NO setItem call — vanilla would re-find
 *        them on the next 40-tick rescan.
 *   After every scan the stamp is refreshed and the active bit consumed
 *   (ACTIVE_ONCE semantics). Stationary items therefore pay one O(1) gate
 *   per 40-tick schedule instead of an O(cluster) broadphase query.
 *
 * INVARIANT (pair argument, docs/mega-round/ROUND-396-I.md): if two
 * stationary items are within merge radius, they already merged at the first
 * scan that saw both (the AABB query is symmetric — X finds Y iff Y finds
 * X), or the merge is impossible (stack/wall/age states, which only change
 * through the events above). Repeat scans of stationary pairs find nothing:
 * pure work elimination. Deviations are documented (D1 sub-threshold drift
 * window, D2 playerTouch partial pickup has no setItem site, D3 wall-clip
 * dynamics, D4 event-latency instead of the 40-tick timer).
 *
 * PARITY/FAIL-DOMINANT: the vanilla body is never re-implemented — the
 * original private method is invoked through a setAccessible Method resolved
 * at static init and verified via selfTest() BEFORE the rust side
 * retransforms ItemEntity (no retransform without a working delegate =
 * vanilla path preserved by construction). Any post-retransform delegate
 * failure degrades to a skipped scan (log-once), never to a wrong merge.
 *
 * CONCURRENCY: region_threads=4 ticks entities in parallel; map operations
 * are serialized on a single monitor; the vanilla scan itself runs OUTSIDE
 * the lock under the same threading the bank already uses for vanilla scans.
 *
 * INJECTS-ONLY: this class is defined into the KERNEL loader at runtime by
 * src/items_wakeup.rs (env CRUSSTY_LEVER_FLAG=items_wakeup). Empty flag =
 * never defined, bytes never served = exact vanilla.
 */
public final class ItemsWakeupOps {

    private ItemsWakeupOps() {}

    // ---------- task-spec event threshold: E2 total movement > 0.25 blocks ----------
    static final double MOVE_EPS2 = 0.25 * 0.25;

    private static final Object LOCK = new Object();
    private static final Long2DoubleOpenHashMap LAST_X = new Long2DoubleOpenHashMap();
    private static final Long2DoubleOpenHashMap LAST_Y = new Long2DoubleOpenHashMap();
    private static final Long2DoubleOpenHashMap LAST_Z = new Long2DoubleOpenHashMap();
    private static final LongOpenHashSet ACTIVE = new LongOpenHashSet();

    private static volatile Method VANILLA_MERGE;
    private static volatile String INIT_ERR;
    private static volatile boolean LOGGED_DELEGATE_FAIL;

    static {
        try {
            Method m = ItemEntity.class.getDeclaredMethod("mergeWithNeighbours");
            m.setAccessible(true);
            VANILLA_MERGE = m;
            System.err.println("[crussty-items-wakeup] ItemsWakeupOps static-init OK (delegate resolved)");
        } catch (Throwable t) {
            INIT_ERR = t.toString();
            System.err.println("[crussty-items-wakeup] ItemsWakeupOps static-init FAIL: " + INIT_ERR);
        }
    }

    /**
     * Pre-retransform contract (called by src/items_wakeup.rs through JNI):
     * "OK" only when the reflection delegate to the private vanilla body is
     * usable. Any other value keeps ItemEntity vanilla (fail-closed).
     */
    public static String selfTest() {
        return VANILLA_MERGE != null ? "OK" : ("FAIL:" + INIT_ERR);
    }

    // ---------- wake bookkeeping ----------

    static void wake(long id) {
        synchronized (LOCK) {
            ACTIVE.add(id);
        }
    }

    private static void stamp(long id, double x, double y, double z) {
        synchronized (LOCK) {
            LAST_X.put(id, x);
            LAST_Y.put(id, y);
            LAST_Z.put(id, z);
            ACTIVE.remove(id);
        }
    }

    /**
     * E1/E2/E3 gate for the tick() call site. True = run the vanilla scan.
     */
    private static boolean scanNeeded(ItemEntity self, long id) {
        synchronized (LOCK) {
            if (ACTIVE.contains(id)) {
                return true; // E3 one-shot wake (consumed by stamp())
            }
            if (!LAST_X.containsKey(id)) {
                return true; // E1 first sighting: spawn / chunk load-in
            }
            double dx = self.getX() - LAST_X.get(id);
            double dy = self.getY() - LAST_Y.get(id);
            double dz = self.getZ() - LAST_Z.get(id);
            return dx * dx + dy * dy + dz * dz > MOVE_EPS2; // E2
        }
    }

    // ---------- retargeted call sites ----------

    /**
     * Retarget of BOTH tick()-scheduled scans (the only tick()-class site,
     * offset 471 in the baseline kernel): gate, then the REAL vanilla body.
     */
    public static void mergeWithNeighbours(ItemEntity self) {
        try {
            long id = self.getId();
            if (scanNeeded(self, id)) {
                runVanilla(self, id);
            }
        } catch (Throwable t) {
            // paranoia net (incl. init errors): a skipped scan can only delay
            // a merge to the next event — never fabricate one, never crash.
            logDelegateFail(t);
        }
    }

    /**
     * Retarget of the teleport() call site (offset 29 in the baseline
     * kernel): vanilla semantics = unconditional scan after teleport —
     * kept unconditional (teleports are rare; exact parity on site 2).
     */
    public static void mergeAfterTeleport(ItemEntity self) {
        try {
            runVanilla(self, self.getId());
        } catch (Throwable t) {
            logDelegateFail(t);
        }
    }

    // ---------- shared scan executor ----------

    private static void runVanilla(ItemEntity self, long id) {
        Method m = VANILLA_MERGE;
        if (m == null) {
            // Belt-and-suspenders (rust selfTest() should have blocked the
            // retransform): without a delegate we cannot reach the vanilla
            // body — skip loudly. This never fabricates a merge.
            if (!LOGGED_DELEGATE_FAIL) {
                LOGGED_DELEGATE_FAIL = true;
                System.err.println("[crussty-items-wakeup] delegate missing at call time — scans skipped (fail-safe)");
            }
            return;
        }
        double px = self.getX();
        double py = self.getY();
        double pz = self.getZ();
        int pc = self.getItem().getCount();
        stamp(id, self.getX(), self.getY(), self.getZ()); // refresh stamp + consume ACTIVE_ONCE
        try {
            m.invoke(self);
        } catch (java.lang.reflect.InvocationTargetException ite) {
            Throwable cause = ite.getCause();
            if (cause instanceof RuntimeException) {
                throw (RuntimeException) cause;   // preserve vanilla crash semantics
            }
            if (cause instanceof Error) {
                throw (Error) cause;
            }
            logDelegateFail(ite);
            return;
        } catch (Throwable t) {
            logDelegateFail(t);
            return;
        }
        // E4 neighbour wake: only when the scan actually merged something
        // (self was the discarded donor OR self's stack changed — the
        // recipient/survivor or remainder donor side needs one re-scan).
        if (self.isRemoved() || self.getItem().getCount() != pc) {
            wakeZone(self.level(), px, py, pz, id);
        }
    }

    private static void logDelegateFail(Throwable t) {
        if (!LOGGED_DELEGATE_FAIL) {
            LOGGED_DELEGATE_FAIL = true;
            System.err.println("[crussty-items-wakeup] delegate invoke failed: " + t);
        }
    }

    /**
     * E4: wake every item within the itemMerge radius of the merge position.
     * Superset AABB (3D inflate with the actual config radius) covers the
     * onlyMergeItemsHorizontally variant. Best-effort: a failure here can
     * only delay a merge to the next movement event (never fabricate one).
     */
    private static void wakeZone(net.minecraft.world.level.Level level, double px, double py, double pz, long excludeId) {
        try {
            double r = level.spigotConfig.itemMerge;
            AABB box = new AABB(px - r, py - r, pz - r, px + r, py + r, pz + r);
            for (ItemEntity e : level.getEntitiesOfClass(ItemEntity.class, box)) {
                if (e.getId() != excludeId) {
                    wake(e.getId());
                }
            }
        } catch (Throwable t) {
            logDelegateFail(t);
        }
    }
}
