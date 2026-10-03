package net.minecraft.server.level;

import it.unimi.dsi.fastutil.ints.Int2ObjectMap;
import it.unimi.dsi.fastutil.objects.ObjectCollection;

/**
 * TASK-411-A (k5b): ChunkMap.entityMap full-table race fence.
 *
 * Root cause (docs/TASK411_A_AIOOBE_ROOTCAUSE.md): vanilla entityMap is an
 * unsynchronized fastutil Int2ObjectOpenHashMap mutated from two thread
 * domains in the region-threads era — population-phase puts (ChunkMap
 * addEntity, main thread) vs tick-phase removes/containsKey probes
 * (region workers). Concurrent put/remove desynchronize the fastutil
 * size/table invariant, the table can fill past maxFill WITHOUT rehash,
 * and then: (a) containsKey enters the open-addressing probe loop that
 * only terminates on an empty slot -> the watchdog-stuck infinite probe
 * (containsKey:349 <- ChunkMap.addEntity:953), and (b) the fastutil
 * MapIterator downscan walks key[--pos] past slot 0 -> "Index -1 ...
 * length 131073" (n+1 table at n=131072).
 *
 * THE FIX (fail-dominant fence, vanilla semantics bit-in-byte): every
 * entityMap call site in ChunkMap (containsKey x3, put x1, remove x1,
 * get x4, values x3 = 12 sites, javap census on kernel 1.21.10) is
 * retargeted to the static helpers below. Each helper serializes on the
 * MAP INSTANCE's own monitor ("map") — a per-map sectional lock:
 *  - all mutators (put/remove) + probes (containsKey/get) are mutually
 *    exclusive -> the fastutil size/table invariant can never desync ->
 *    the table can never fill past maxFill -> the infinite probe and the
 *    -1 downscan are impossible BY CONSTRUCTION (the root cause, not the
 *    symptom);
 *  - values() returns a view whose iterators are created INSIDE the map
 *    monitor and are bound-checked/fail-dominant (EntityMapSafeItr): a
 *    hypothetical corrupted-table AIOOBE degrades to an early iteration
 *    end (vanilla weakly-consistent iteration semantics), never a crash;
 *  - uncontended monitor acquire on Java 21 = thin lock (~15ns) — the
 *    population storm does ~300k synchronized ops over ~40s: noise.
 *
 * PARITY: map semantics untouched (same insertion/lookup/iteration order,
 * same check-then-act window in addEntity as vanilla, same weakly
 * consistent iteration); only serialization is added. A retarget failure
 * of ANY strict count leaves ChunkMap completely vanilla (fail-dominant).
 * The read-only site in Entity.resendPossiblyDesyncedEntityData is
 * DELIBERATELY left vanilla (rare debug path, read-only — the vanilla
 * fallback for that site).
 *
 * Delivery pattern of NavPlaneOps/NavPoolOps (TASK-405-A/410-A): compiled
 * offline against the runtime kernel jar (Mojang-mapped), defined into the
 * KERNEL loader at region_threads activation time BEFORE the ChunkMap
 * retransform. Class-file major pinned to 65 (kernel JVM = Java 21).
 * ZERO nested classes (S7-163 discipline): three flat top-level classes —
 * EntityMapOps, EntityMapSafeValues, EntityMapSafeItr.
 */
public final class EntityMapOps {

    private static volatile boolean armedOnce = false;

    private EntityMapOps() {}

    /// PROBE-THEN-PATCH contract (inside_bitmask precedent): the rust side
    /// defines this class into the kernel loader; a successful define is
    /// the ARM probe (a class that fails to define would abort the emap
    /// compose stage, never the retransform).
    public static String armState() {
        return "ARMED";
    }

    /// One-shot EFFECT marker (bench checklist: grep EMAPSAFE in boot log).
    static void armMark() {
        if (!armedOnce) {
            armedOnce = true;
            System.err.println(
                "[EMAPSAFE] entityMap full-table race fence ACTIVE "
                    + "(monitor-serialized mutators + snapshot-safe iterators)");
        }
    }

    // ---- the five fence helpers (receiver-prepended static form) ----
    // Raw Int2ObjectMap params: the retargeted call sites are interface
    // invokes (erased descriptors); raw types keep the erased signatures
    // EXACT (no checked bridge mismatch possible).

    public static boolean containsKey(Int2ObjectMap map, int k) {
        armMark();
        synchronized (map) {
            return map.containsKey(k);
        }
    }

    public static Object put(Int2ObjectMap map, int k, Object v) {
        armMark();
        synchronized (map) {
            return map.put(k, v);
        }
    }

    public static Object remove(Int2ObjectMap map, int k) {
        synchronized (map) {
            return map.remove(k);
        }
    }

    public static Object get(Int2ObjectMap map, int k) {
        synchronized (map) {
            return map.get(k);
        }
    }

    /// Snapshot-safe view: iterators are created under the map monitor
    /// (atomic with respect to every fenced mutator) and are bound-checked
    /// (EntityMapSafeItr) — fail-dominant, never a server crash.
    public static ObjectCollection values(Int2ObjectMap map) {
        ObjectCollection inner;
        synchronized (map) {
            inner = map.values();
        }
        return new EntityMapSafeValues(inner, map);
    }

    // ==================== REFSYNC (TASK-412-A) ====================
    // ReferenceList mutator fence (round-412-a-k5b leg #2 evidence:
    // run 35698807454 — watchdog RUNNABLE forever in
    // Reference2IntOpenHashMap.find:246 <- putIfAbsent:422 <-
    // ReferenceList.add:65 <- ServerEntityLookup.entityStartLoaded:115 <-
    // EntityLookup.addNewEntity <- main-thread BenchPopulationPlugin
    // injection, plus 3278 AIOOBE ("Index -1 ... length 16385" downscan
    // x203 + message-less x3075) in the same population window).
    // The emap entityMap fence (12 ChunkMap sites) closed the k5-diag
    // hang; the race then surfaced on the OTHER unsynchronized fastutil
    // map: ReferenceList.referenceToIndex (Reference2IntOpenHashMap,
    // n+1 = 16385 at n=16384). Same mechanism: main-thread population
    // adds vs region-worker unload removes (entityEndLoaded) interleave
    // on the open-addressing table -> size/table desync -> full table ->
    // infinite probe (hang) / below-slot-0 downscan (AIOOBE -1).
    //
    // THE FIX: every ReferenceList add/remove/contains call site in the
    // kernel (javap census on kernel 1.21.10 = 8 classes / 22 sites:
    // add x11, remove x9, contains x2) is retargeted to the helpers
    // below — monitor serialized on the LIST INSTANCE. All map
    // mutators+probes mutually exclusive => the table can never fill
    // past maxFill => the infinite probe and the -1 downscan are
    // impossible by construction. Read-only array readers
    // (size/getRawDataUnchecked — the raw-array iteration idiom) are
    // DELIBERATELY left vanilla (Entity.resendPossiblyDesyncedEntityData
    // precedent: read-only sites cannot corrupt the map; they inherit
    // the vanilla weakly-consistent iteration semantics).
    //
    // PARITY: same return values, same check-then-act window as vanilla,
    // null receiver still NPEs (inside the helper); only serialization
    // added. Zero lock nesting (helpers acquire exactly the list monitor
    // and call nothing that synchronizes) => deadlock-free by
    // construction. A census violation on ANY class leaves that class
    // completely vanilla (fail-dominant, classfile.rs strict counts).

    /// One-shot EFFECT marker (bench checklist: grep REFSAFE in boot log).
    private static volatile boolean refArmedOnce = false;

    static void refArmMark() {
        if (!refArmedOnce) {
            refArmedOnce = true;
            System.err.println(
                "[REFSAFE] ReferenceList mutator fence ACTIVE "
                    + "(add/remove/contains monitor-serialized on the list instance)");
        }
    }

    // Raw ReferenceList params: the retargeted call sites are invokevirtual
    // (erased descriptors); raw types keep the erased signatures EXACT.

    public static boolean refListAdd(ca.spottedleaf.moonrise.common.list.ReferenceList list, Object e) {
        refArmMark();
        synchronized (list) {
            return list.add(e);
        }
    }

    public static boolean refListRemove(ca.spottedleaf.moonrise.common.list.ReferenceList list, Object e) {
        synchronized (list) {
            return list.remove(e);
        }
    }

    public static boolean refListContains(ca.spottedleaf.moonrise.common.list.ReferenceList list, Object e) {
        synchronized (list) {
            return list.contains(e);
        }
    }
}
