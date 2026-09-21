package net.minecraft.world.entity.item;

// ============================================================================
// ItemLifetimeOps (TASK-397-H, MEGA-ROUND-2 — lever items_despawn_heap).
//
// Event-driven despawn schedule for ItemEntity (RESEARCH-H.md, round-2):
// the vanilla per-tick tail gate `if (!isClientSide && age >= despawnRate)
// { ItemDespawnEvent / discard }` (tick() AND inactiveTick()) is replaced by
// a heap-scheduled due lookup. The despawn DUE TICK is computed ONCE per item
// from the exact vanilla algebra:
//
//     vanilla despawns at the first tail where age (which grows by 1 every
//     tick before the tail) reaches despawnRate  =>
//         due = sightingTick + (despawnRate - age0), clamped to >= sighting
//     (age0 = the age FIELD at the sighting tail; overdue items are due
//     immediately, never later than vanilla)
//
// The per-tick hot path for a NOT-due item is: ONE flat long[] read + compare
// (no getfield on the entity, no virtual calls, no age iteration). At the due
// tick the ops write the GROUND TRUTH into the field (`age = despawnRate` via
// the vanilla check) and return false so the UNTOUCHED vanilla tail bytes
// execute the exact vanilla despawn block (ItemDespawnEvent -> cancelled ?
// age=0 : discard). Despawn semantics stay vanilla by construction; the
// bounded writer-drift lateness is documented in docs/TASK-397-H.md.
//
// Heap: flat parallel arrays (long due + int id), NO boxing (kills the
// java_util PriorityQueue lane on this subsystem), lazy tombstone pops
// (stamped validation; /kill, merge-discard and chunk unload never do O(N)
// removals).
//
// Region-thread safety (bank v4 arms region_threads=4): an entity is
// despawn-processed ONLY by the thread that ticks it (per-item due
// validation); the heap is a scheduler index + observability, never a
// cross-thread drain. Non-volatile long reads cannot misfire: any torn or
// garbage due value falls into the due path which validates against the
// exact field before letting the vanilla block run (fail-safe backstop).
//
// PickupDelay is NOT heap-scheduled in v1 (bench scene pins 32767 = dead
// lane; writer surface = playerTake intra-tick putfields + Bukkit
// setPickupDelay). Documented decision in docs/TASK-397-H.md.
//
// Dormant-invisible: the ops class is only DEFINED when the Rust side parsed
// CRUSSTY_LEVER_FLAG == "items_despawn_heap"; the byte splice is strict
// (exactly one tail match per method) and fail-closed to pristine bytes.
// ============================================================================

public final class ItemLifetimeOps {

    private ItemLifetimeOps() {}

    // despawnRate is PRIVATE in Purpur (config-driven via setItem): the ops
    // read it through a cached MethodHandle getter — after C2 inlining this
    // is a bare getfield, no reflection cost on the hot path.
    private static final java.lang.invoke.MethodHandle DESPAWN_RATE;
    static {
        java.lang.invoke.MethodHandle mh = null;
        try {
            java.lang.reflect.Field fr = ItemEntity.class.getDeclaredField("despawnRate");
            fr.setAccessible(true);
            mh = java.lang.invoke.MethodHandles.lookup().unreflectGetter(fr);
        } catch (Throwable t) {
            System.err.println(
                "[net.minecraft.world.entity.ItemLifetimeOps] [S7-H] despawnRate getter unavailable:"
                + " " + t);
        }
        DESPAWN_RATE = mh;
    }

    private static int rate(ItemEntity e) throws Throwable {
        return (int) DESPAWN_RATE.invokeExact(e);
    }

    // ---- lever gate (mirrors the Rust parse; the class only exists when
    // ---- armed, but static-init state is the single audit trail) ---------
    public static final boolean LEVER_ARMED;
    static {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        LEVER_ARMED = "items_despawn_heap".equals(f == null ? "" : f.trim());
        if (LEVER_ARMED) {
            System.err.println(
                "[net.minecraft.world.entity.ItemLifetimeOps] [S7-H] items_despawn_heap ARMED:"
                + " ItemEntity tick/inactiveTick tail -> heap-scheduled despawn gate"
                + " (event-driven lifetime subsystem v1, despawn-only)");
        }
    }

    // ---- schedule tables (indexed by entity id; ids grow monotonically) ---
    private static volatile long[] dueById = unscheduled(4096);
    private static volatile Object[] entityById = new Object[4096];

    /** -1 = "no schedule" sentinel (game time is >= 0, NEVER = MAX). */
    private static long[] unscheduled(int n) {
        long[] a = new long[n];
        java.util.Arrays.fill(a, -1L);
        return a;
    }

    /** despawn never (age == -32768 unlimited lifetime at scheduling). */
    public static final long NEVER = Long.MAX_VALUE;

    // ---- flat min-heap by due tick (no boxing) ----------------------------
    private static long[] heapDue = new long[4096];
    private static int[] heapId = new int[4096];
    private static int heapSize;

    // ---- telemetry (observability only) ------------------------------------
    public static volatile long scheduled;
    public static volatile long rescheduledFromField;
    public static volatile long despawnFired;
    public static volatile long staleHeapPops;

    // ---- hot path ---------------------------------------------------------
    /**
     * Tail gate replacing the vanilla `!isClientSide && age >= despawnRate`
     * head in ItemEntity.tick()/inactiveTick(). Returns TRUE = skip the
     * vanilla despawn block (the untouched vanilla ifne branches to method
     * end), FALSE = the untouched vanilla block must run (ops guarantees
     * age >= despawnRate at that moment).
     *
     * MUST never throw: any internal failure falls back to the exact
     * vanilla truth computed from the fields (fail-open to vanilla).
     */
    public static boolean tailGate(ItemEntity e) {
        // Broken-ops fail-safe: return false and let the SURVIVING vanilla
        // bytes decide (the spliced tail keeps `age >= despawnRate` + the
        // event/discard block — only the level/isClientSide probe was
        // replaced; on a dedicated server isClientSide is always false).
        if (DESPAWN_RATE == null) {
            return false;
        }
        try {
            long now = e.level().getGameTime();
            int id = e.getId();
            long[] dueT = dueById;
            if (id >= dueT.length) {
                grow(id);
                dueT = dueById;
            }
            long due = dueT[id];
            if (due < 0L) { // -1 = unscheduled sentinel
                schedule(e, id, now);
                due = dueById[id];
                if (due > now || due == NEVER) {
                    return true;
                }
                // scheduled already-overdue (degenerate rate config / writer
                // drift): fall through and fire THIS tail, exactly like the
                // vanilla check would have.
            }
            if (due > now) {
                return true; // the ONLY hot path for ~all items/ticks
            }
            if (due == NEVER) {
                return true; // unlimited lifetime (age == -32768 at scheduling)
            }
            // due reached: resolve from the exact vanilla field (ground truth)
            if (e.age >= rate(e)) {
                despawnFired++;
                return false; // vanilla tail fires event/discard exactly
            }
            // life was EXTENDED after scheduling (despawn-event cancelled
            // (age=0), setExtendedLifetime, or writer drift): resync from field
            rescheduledFromField++;
            schedule(e, id, now);
            lazyClean(now);
            return true;
        } catch (Throwable t) {
            // fail-open to the exact vanilla decision (surviving vanilla check)
            return false;
        }
    }

    // ---- scheduling -------------------------------------------------------
    private static void schedule(ItemEntity e, int id, long now) {
        if (e.isRemoved()) {
            return; // dead entity: no schedule (stale heap entry cleaned lazily)
        }
        int rate;
        try {
            rate = rate(e);
        } catch (Throwable t) {
            return; // broken-ops fail-safe: no schedule; vanilla check survives
        }
        int age = e.age;
        final long due;
        if (age == -32768) {
            due = NEVER;
        } else {
            int left = rate - age; // tails left until the vanilla field check fires
            if (left < 0) {
                left = 0; // already overdue at first sighting: due = now
            }
            due = now + left; // EXACT vanilla despawn tick
        }
        if (id >= entityById.length) {
            grow(id);
        }
        entityById[id] = e;
        dueById[id] = due;
        push(due, id);
        scheduled++;
    }

    private static void grow(int id) {
        synchronized (ItemLifetimeOps.class) {
            if (id >= dueById.length) {
                int n = dueById.length;
                while (n <= id) {
                    n <<= 1;
                }
                long[] d = unscheduled(n);
                System.arraycopy(dueById, 0, d, 0, dueById.length);
                Object[] ob = new Object[n];
                System.arraycopy(entityById, 0, ob, 0, entityById.length);
                dueById = d;
                entityById = ob;
            }
        }
    }

    // ---- flat min-heap (long key, int payload; lazy tombstones) -----------
    private static void push(long due, int id) {
        synchronized (ItemLifetimeOps.class) {
            if (heapSize == heapDue.length) {
                if (heapSize > (1 << 26)) {
                    heapSize = 0; // pathological growth guard: keep running
                } else {
                    long[] nd = new long[heapDue.length << 1];
                    System.arraycopy(heapDue, 0, nd, 0, heapSize);
                    int[] ni = new int[heapId.length << 1];
                    System.arraycopy(heapId, 0, ni, 0, heapSize);
                    heapDue = nd;
                    heapId = ni;
                }
            }
            int i = heapSize++;
            heapDue[i] = due;
            heapId[i] = id;
            while (i > 0) {
                int p = (i - 1) >>> 1;
                if (heapDue[p] <= heapDue[i]) {
                    break;
                }
                long td = heapDue[p];
                int ti = heapId[p];
                heapDue[p] = heapDue[i];
                heapId[p] = heapId[i];
                heapDue[i] = td;
                heapId[i] = ti;
                i = p;
            }
        }
    }

    /**
     * Amortized lazy clean: pop only STALE top entries (stamp mismatch or
     * removed entity). Valid due entries of OTHER entities are left alone —
     * their owning region thread resolves them through their own tail gate
     * (cross-thread despawn is forbidden by design).
     */
    private static void lazyClean(long now) {
        synchronized (ItemLifetimeOps.class) {
            while (heapSize > 0 && heapDue[0] <= now) {
                int topId = heapId[0];
                long topDue = heapDue[0];
                boolean stale;
                if (topId >= dueById.length) {
                    stale = true;
                } else {
                    long cur = dueById[topId];
                    if (cur != topDue) {
                        stale = true; // tombstoned by reschedule/kill
                    } else {
                        Object ref = entityById[topId];
                        stale = !(ref instanceof ItemEntity alive)
                                || alive.isRemoved();
                    }
                }
                if (!stale) {
                    break; // valid top: stop (owner thread will consume it)
                }
                staleHeapPops++;
                popTop();
            }
        }
    }

    private static void popTop() {
        heapSize--;
        heapDue[0] = heapDue[heapSize];
        heapId[0] = heapId[heapSize];
        int i = 0;
        while (true) {
            int l = 2 * i + 1;
            if (l >= heapSize) {
                return;
            }
            int r = l + 1;
            int m = (r < heapSize && heapDue[r] < heapDue[l]) ? r : l;
            if (heapDue[i] <= heapDue[m]) {
                return;
            }
            long td = heapDue[m];
            int ti = heapId[m];
            heapDue[m] = heapDue[i];
            heapId[m] = heapId[i];
            heapDue[i] = td;
            heapId[i] = ti;
            i = m;
        }
    }
}
