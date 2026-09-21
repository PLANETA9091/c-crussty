package net.minecraft.world.entity.item;

/**
 * ITEM-FOOTPRINT bridge (round-397-g-footprint, TASK-397-G — the
 * items/ItemEntity.tick footprint lane).
 *
 * The lever retargets EVERY getfield/putfield of the hot ItemEntity scalar
 * fields {@code age} and {@code pickupDelay} inside class ItemEntity (48
 * sites by the javap census: tick 9, inactiveTick 7, playerTouch 6, static
 * merge 6, setters 8, ctor 2, NBT save/load 4, isMergable 3,
 * getAge/hasPickUpDelay 2, makeFakeItem/setUnlimitedLifetime/... rest) onto
 * the static accessors below — 3B getfield -> 3B invokestatic, 3B putfield ->
 * 3B invokestatic, verifier-identical stack shapes, StackMapTable untouched.
 *
 * Storage is a FLAT stride-4-int arena in OFF-HEAP memory owned by the rust
 * side (single native {@link #ensure} call; the arena is 64-byte aligned, 16
 * bytes per entity: age@+0, pickupDelay@+4, migration stamp@+8, so 4 hot
 * entity pairs share one cache line vs 2-4 entities per line in the
 * scattered heap). Index key is the monotonic per-run Entity id (getId());
 * slots are calloc-zeroed.
 *
 * MIGRATION (parity backstop for entities that predate the retarget): an
 * entity alive before the retransform has its truth in the heap fields while
 * its arena slot is still zero — reading the arena raw would silently reset
 * pickupDelay=32767 (the bench population's never-pickup delay) to 0 and
 * reset despawn timers. Therefore:
 *   - {@code hwm} (high-water mark) = Entity.ENTITY_COUNTER value captured
 *     once at bridge class-init, i.e. after the retarget is live: every
 *     entity with id <= hwm was created BEFORE the retarget and is lazily
 *     migrated on first post-retarget access (fields -> arena, one-shot,
 *     marked by the stamp word); ids > hwm are arena-native from birth and
 *     pay a single predicted compare, nothing else.
 *   - Writes are write-through ({@code e.age = v} after the arena store) so
 *     every out-of-class reader of the vanilla field (e.g. CraftBukkit
 *     mirrors) observes fresh values, and a migration never resurrects a
 *     stale value. The only documented semantic edge is a DIRECT
 *     out-of-class field WRITER (Bukkit mirror setters, not on the server
 *     tick path) racing between migration and the next write-through.
 *
 * Hot-path cost: getId() (inlined field read) + bounds check + Unsafe
 * scalar — NO JNI per access (the JNI-transition refutation of alloc_diet
 * applies at 150k x 6 accesses/tick).
 *
 * Fail-closed to vanilla: if the arena was never allocated (Unsafe
 * unavailable, native ensure failed, OOM) every accessor transparently
 * degenerates to the vanilla field access — vanilla semantics inside the
 * retarget, parity by construction.
 */
public final class ItemFootprintOps {
    private static final sun.misc.Unsafe U = unsafeOrNull();

    /** Arena base address (0 = vanilla fallback, sticky). */
    private static long base = 0L;
    /** Arena capacity in ENTITIES (stride 16 bytes). */
    private static int cap = 0;
    /** Sticky failure: never touch the arena again, pure field semantics. */
    private static boolean failed = false;

    private static final int STRIDE = 16;
    /** Migration stamp: slot holds migrated (field -> arena) values. */
    private static final int MIG = 0x4D475244; // "MGRD"
    /**
     * High-water mark of PRE-retarget entity ids (captured once at class
     * init). Integer.MAX_VALUE = unknown counter (migrate-everything: safe,
     * stamp-gated, one-shot).
     */
    private static int hwm = captureHwm();

    /** Initial capacity: 4M entities = 64MB off-heap, zero growth in bench. */
    private static final int INITIAL_ENTITIES = 1 << 22;

    static {
        if (U == null) {
            failed = true;
        } else {
            try {
                request(INITIAL_ENTITIES);
            } catch (Throwable t) {
                failed = true;
            }
        }
    }

    private static sun.misc.Unsafe unsafeOrNull() {
        try {
            java.lang.reflect.Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            return (sun.misc.Unsafe) f.get(null);
        } catch (Throwable t) {
            return null;
        }
    }

    /**
     * The Entity id counter at the moment the bridge class initializes. The
     * bridge initializes on the FIRST retargeted access — i.e. after the
     * retransform landed — so ids <= hwm are exactly the pre-retarget
     * entities. Reflection (private static final ENTITY_COUNTER, mojang-
     * mapped kernel); on any failure fall open to migrate-everything.
     */
    private static int captureHwm() {
        try {
            java.lang.reflect.Field f =
                    net.minecraft.world.entity.Entity.class.getDeclaredField("ENTITY_COUNTER");
            f.setAccessible(true);
            return ((java.util.concurrent.atomic.AtomicInteger) f.get(null)).get();
        } catch (Throwable t) {
            return Integer.MAX_VALUE;
        }
    }

    /**
     * RUST-SIDE ARENA: allocate (or grow) the flat stride-4-int arena so it
     * holds at least {@code minEntities} slots; returns the 64-byte-aligned
     * base address or 0 on failure. Monotonic: the previous arena is never
     * freed (its contents are copied), so any (base, cap) snapshot observed
     * by a caller stays valid for ids below that snapshot's cap forever.
     */
    private static native long ensure(int minEntities);

    private static synchronized void request(int minEntities) {
        if (failed || cap >= minEntities) {
            return;
        }
        long b = ensure(minEntities);
        if (b == 0L) {
            failed = true;
            return;
        }
        base = b;
        cap = minEntities;
    }

    /**
     * Address of the (age|pickupDelay|stamp) record for {@code e}'s id, or
     * 0 = vanilla fallback. Reads (base, cap) as locals so a concurrent grow
     * can never produce a base/cap pair that is invalid for an id below the
     * observed cap (rust side never frees an arena). Migrates pre-retarget
     * entities on first touch (see hwm/MIG).
     */
    private static long pair(ItemEntity e) {
        if (failed) {
            return 0L;
        }
        int id = e.getId();
        long b = base;
        int c = cap;
        if (id >= c || b == 0L) {
            if (id < 0) {
                return 0L;
            }
            request(id + 1024);
            if (failed) {
                return 0L;
            }
            b = base;
        }
        long a = b + ((long) id << 4);
        if (id <= hwm && U.getInt(a + 8L) != MIG) {
            // One-shot migration: the heap fields are the truth for every
            // entity that predates the retarget; copy both, mark the slot.
            // Same-entity accesses are same-thread (region tick ownership),
            // and even a race would write identical values.
            U.putInt(a, e.age);
            U.putInt(a + 4L, e.pickupDelay);
            U.putInt(a + 8L, MIG);
        }
        return a;
    }

    public static int getAge(ItemEntity e) {
        long a = pair(e);
        return a == 0L ? e.age : U.getInt(a);
    }

    public static void putAge(ItemEntity e, int v) {
        long a = pair(e);
        if (a == 0L) {
            e.age = v;
            return;
        }
        U.putInt(a, v);
        e.age = v;
    }

    public static int getPickupDelay(ItemEntity e) {
        long a = pair(e);
        return a == 0L ? e.pickupDelay : U.getInt(a + 4L);
    }

    public static void putPickupDelay(ItemEntity e, int v) {
        long a = pair(e);
        if (a == 0L) {
            e.pickupDelay = v;
            return;
        }
        U.putInt(a + 4L, v);
        e.pickupDelay = v;
    }

    private ItemFootprintOps() {
    }
}
