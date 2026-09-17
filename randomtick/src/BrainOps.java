package net.minecraft.world.entity.ai;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.WeakHashMap;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.ai.behavior.Behavior;
import net.minecraft.world.entity.ai.behavior.BehaviorControl;
import net.minecraft.world.entity.schedule.Activity;

/**
 * F2 BRAIN-ITERATORS helper (family-agg pack member F2, TASK-249 / S7-113).
 *
 * Replaces the body of Brain.startEachNonRunningBehavior(ServerLevel, LivingEntity)
 * with a flat-snapshot lens. Median-exact parity contract (bytecode transcribed
 * instruction-by-instruction from the materialized kernel, mojang-mapped Purpur
 * 1.21.10 build 2025-12-11, run21 artifact — Brain.cfdump.txt @870):
 *
 *   startEachNonRunningBehavior: stack=5 locals=12 len=178
 *     @0-4    gameTime = level.getGameTime()               // read ONCE per call
 *     @5-19   outer it = availableBehaviorsByPriority.values().iterator()
 *     @21-41  inner = (Map) it.next()                      // per priority
 *     @43-55  innerIt = inner.entrySet().iterator()
 *     @57-89  activity = (Activity) entry.getKey()         // per activity entry
 *     @91-102 if (!activeActivities.contains(activity)) skip whole group
 *     @105-120 setIt = ((Set) entry.getValue()).iterator()
 *     @122-142 b = (BehaviorControl) setIt.next()          // per behavior
 *     @144-154 if (b.getStatus() != Behavior.Status.STOPPED) continue
 *     @157-167 b.tryStart(level, entity, gameTime)         // result POPped
 *
 * REPLACED COST (task168 sizing, 1.6-1.7% CPU lane / F2 core 0.9-1.5% MSPT):
 * the triple-nested iterator machinery (3 iterator allocations + hasNext/next
 * virtual chains + LinkedHash* traversal) per mob per tick. The LIVE semantics
 * are preserved 1:1 — activeActivities membership and behavior status are read
 * live per slot exactly where vanilla reads them; tryStart/status calls are
 * untouched polymorphic dispatch (task168: VERIFIED NOT replaceable).
 *
 * SNAPSHOT LENS:
 *   - availableBehaviorsByPriority is setup-stable: the ONLY mutation sites in
 *     the whole class (cfdump scan of every getfield of this field) are
 *       a) <init> @22: putfield Maps.newTreeMap()  — OUTER = TreeMap
 *          (iteration ASCENDS by priority Integer; entrySet == values order),
 *       b) addActivityAndRemoveMemoriesWhenStopped @693: computeIfAbsent
 *          (outer, supplier Maps.newLinkedHashMap — INNER = LinkedHashMap,
 *          activity insertion order) + computeIfAbsent (inner, supplier
 *          Sets.newLinkedHashSet — behavior insertion order) + Set.add
 *          — GROWTH only,
 *       c) removeAllBehaviors @714: Map.clear().
 *     No vanilla method ever removes a single entry, replaces an entry value,
 *     or re-inserts a key (no remove/put on this map anywhere).
 *   - A flat snapshot {acts[], behs[], groupStart[]} is cached per brain map
 *     (WEAK identity key) and rebuilt when any cheap structural probe fails.
 *   - FINGERPRINT (exact vs the surface above), all O(1) reads, ZERO iterator
 *     allocation on the hot path:
 *       1. outer.size() == cached key count
 *       2. outer.get(cachedKey_i) == cachedInnerMap_i      (identity probe —
 *          catches clear+re-add, which recreates inner maps)
 *       3. innerMap_i.size() == cached                     (catches new
 *          activity entry via computeIfAbsent)
 *       4. innerMap_i.get(cachedActKey_ij) == cachedSet_ij (identity probe)
 *       5. cachedSet_ij.size() == cached                   (catches Set.add)
 *     Any reachable mutation (b) or (c) breaks at least one probe: (b) always
 *     grows a size; (c) zeroes outer.size(). Hence a passing fingerprint
 *     implies an unchanged traversal order — the lens is median-exact.
 *   - groupStart[i] marks a slot that begins a vanilla (priority, activity)
 *     ENTRY; activeActivities.contains is evaluated exactly there (vanilla
 *     @91-102 evaluates it once per entry, before the behavior loop). Same
 *     activity under two priorities = two groups = two live checks, matching
 *     vanilla. Empty entries contribute no slots: vanilla then executes
 *     contains() (@91-102) before finding the set empty — a pure read with no
 *     observable effect (activity keys are non-null by computeIfAbsent
 *     construction), so eliding it is semantics-preserving.
 *
 * CACHE KEY (the AbstractMap.equals TRAP): a WeakHashMap keyed directly by the
 * brain's map would be WRONG — LinkedHashMap inherits deep equals/hashCode
 * (AbstractMap.hashCode = sum of entry hashes, order-insensitive), so two
 * structurally equal maps of DIFFERENT brains could collide and cross-serve
 * snapshots (order divergence). Keys are wrapped in an identity IdKey
 * (identityHashCode + ==). Cache values die with their brain map (weak refs) —
 * MineShield-3 mobs spawn/despawn constantly, no leak.
 *
 * RESIDUAL (documented, out of vanilla mutation surface): a hypothetical
 * LinkedHashMap.remove(k)+put(k,v) (single-entry re-insert to tail, same refs
 * and sizes) between two calls of the same brain would evade the fingerprint
 * (sizes equal, identity probes equal, order changed). NO vanilla code path
 * does this (surface proof above); noted for completeness.
 *
 * CONCURRENCY: snapshot objects are immutable; the synchronized WeakHashMap is
 * race-safe; fingerprint check-then-build races produce distinct valid
 * snapshots (benign last-writer-wins). Vanilla in the same window would risk
 * ConcurrentModificationException; the lens is strictly more tolerant.
 *
 * NEXT TICK (byte hook, patch_update/patch_optimise_random_tick pattern):
 * replace the method body with the 14-byte straight-line sequence
 *   aload_0; getfield availableBehaviorsByPriority:Ljava/util/Map;
 *   aload_0; getfield activeActivities:Ljava/util/Set;
 *   aload_1; aload_2;
 *   invokestatic BrainOps.startEachNonRunning:
 *     (Ljava/util/Map;Ljava/util/Set;Lnet/minecraft/server/level/ServerLevel;
 *      Lnet/minecraft/world/entity/LivingEntity;)V
 *   return
 * max_stack=5, max_locals=3, no branches => empty StackMapTable. The getfields
 * execute inside Brain.class itself — verifier-legal on its own private fields
 * (0x0012), so the helper needs NO Unsafe/reflection at all.
 */
public final class BrainOps {

    private BrainOps() {}

    // ------------------------------------------------------------------ key

    /** Identity wrapper — see class javadoc (AbstractMap.equals trap). */
    private static final class IdKey {
        final Object ref;

        IdKey(Object ref) { this.ref = ref; }

        @Override
        public int hashCode() { return System.identityHashCode(ref); }

        @Override
        public boolean equals(Object o) {
            return o instanceof IdKey && ((IdKey) o).ref == this.ref;
        }
    }

    private static final WeakHashMap<IdKey, Snapshot> CACHE = new WeakHashMap<>();

    // -------------------------------------------------------------- snapshot

    private static final class Snapshot {
        final Object source;            // identity anchor (the outer map)
        final Integer[] keys;           // outer keys (priorities), traversal order
        final Object[] innerMaps;       // inner map refs, per key
        final int[] innerSizes;
        final Object[][] actKeys;       // per key: Activity refs (inner map keys)
        final Object[][] sets;          // per key: behavior Set refs
        final int[][] setSizes;
        // flat lens (hot path):
        final Object[] acts;            // Activity per slot, exact vanilla order
        final BehaviorControl<LivingEntity>[] behs;
        final boolean[] groupStart;     // true at each (priority, activity) entry start

        @SuppressWarnings("unchecked")
        Snapshot(Object source, Integer[] keys, Object[] innerMaps, int[] innerSizes,
                 Object[][] actKeys, Object[][] sets, int[][] setSizes,
                 Object[] acts, List<?> rawBehs, boolean[] groupStart) {
            this.source = source;
            this.keys = keys;
            this.innerMaps = innerMaps;
            this.innerSizes = innerSizes;
            this.actKeys = actKeys;
            this.sets = sets;
            this.setSizes = setSizes;
            this.acts = acts;
            this.behs = (BehaviorControl<LivingEntity>[]) rawBehs.toArray(new BehaviorControl[0]);
            this.groupStart = groupStart;
        }
    }

    // ------------------------------------------------------------- hot entry

    /**
     * Production entry — called by the patched body of
     * Brain.startEachNonRunningBehavior. Mirrors vanilla @0-177 exactly
     * (see class javadoc for the offset-by-offset contract).
     */
    public static void startEachNonRunning(Map<?, ?> byPriority, Set<?> activeActivities,
                                           ServerLevel level, LivingEntity entity) {
        final long gameTime = level.getGameTime();          // vanilla @0-4: read ONCE
        final Snapshot s = snapshot(byPriority);
        final Object[] acts = s.acts;
        final BehaviorControl<LivingEntity>[] behs = s.behs;
        final boolean[] groupStart = s.groupStart;
        boolean groupActive = false;
        for (int i = 0; i < behs.length; i++) {
            if (groupStart[i]) {                            // vanilla @91-102: once per entry
                groupActive = activeActivities.contains(acts[i]);
            }
            if (!groupActive) {
                continue;
            }
            final BehaviorControl<LivingEntity> b = behs[i];
            if (b.getStatus() == Behavior.Status.STOPPED) { // vanilla @144-154: live read
                b.tryStart(level, entity, gameTime);        // vanilla @157-167: result popped
            }
        }
    }

    // ------------------------------------------------------------ cache path

    private static Snapshot snapshot(Map<?, ?> outer) {
        final IdKey key = new IdKey(outer);
        final Snapshot s = CACHE.get(key);
        if (s != null && matches(s, outer)) {
            return s;
        }
        final Snapshot fresh = build(outer);
        CACHE.put(key, fresh);
        return fresh;
    }

    /**
     * Exact-vs-surface fingerprint: five O(1) probe families, zero iterator
     * allocation. See class javadoc for the proof that any vanilla mutation
     * breaks at least one probe.
     */
    private static boolean matches(Snapshot s, Map<?, ?> outer) {
        if (s.source != outer || outer.size() != s.keys.length) {
            return false;                                   // probes 1, (c) clear
        }
        for (int i = 0; i < s.keys.length; i++) {
            final Object inner = outer.get(s.keys[i]);      // probe 2: identity
            if (inner != s.innerMaps[i]) {
                return false;
            }
            if (((Map<?, ?>) inner).size() != s.innerSizes[i]) {
                return false;                               // probe 3: new activity
            }
            final Object[] aks = s.actKeys[i];
            final Object[] sts = s.sets[i];
            final int[] szs = s.setSizes[i];
            for (int j = 0; j < sts.length; j++) {
                if (((Map<?, ?>) inner).get(aks[j]) != sts[j]) {
                    return false;                           // probe 4: identity
                }
                if (((Set<?>) sts[j]).size() != szs[j]) {
                    return false;                           // probe 5: Set.add
                }
            }
        }
        return true;
    }

    /**
     * Rebuild: traverses the CURRENT structure in exact vanilla order
     * (outer TreeMap: entrySet == values order, ascending priority — vanilla
     * iterates values() @5-19; inner LinkedHashMap + LinkedHashSet: insertion
     * order, same iterators vanilla drives). groupStart marks vanilla entry
     * boundaries.
     */
    private static Snapshot build(Map<?, ?> outer) {
        final ArrayList<Integer> keys = new ArrayList<>();
        final ArrayList<Object> inners = new ArrayList<>();
        final ArrayList<Object[]> actKeysPerInner = new ArrayList<>();
        final ArrayList<Object[]> setsPerInner = new ArrayList<>();
        final ArrayList<Object> actsA = new ArrayList<>();
        final ArrayList<BehaviorControl<LivingEntity>> behsA = new ArrayList<>();
        final ArrayList<Boolean> groupStartA = new ArrayList<>();

        for (Map.Entry<?, ?> e : outer.entrySet()) {
            final Integer key = (Integer) e.getKey();
            final Map<?, ?> inner = (Map<?, ?>) e.getValue();
            final ArrayList<Object> ak = new ArrayList<>();
            final ArrayList<Object> st = new ArrayList<>();
            keys.add(key);
            inners.add(inner);
            for (Map.Entry<?, ?> en : inner.entrySet()) {
                final Object actKey = en.getKey();          // vanilla @79-89
                final Set<?> bs = (Set<?>) en.getValue();
                boolean first = true;
                for (Object b : bs) {                       // vanilla @105-142, insertion order
                    actsA.add(actKey);
                    behsA.add((BehaviorControl<LivingEntity>) (BehaviorControl<?>) b);
                    groupStartA.add(first ? Boolean.TRUE : Boolean.FALSE);
                    first = false;
                }
                ak.add(actKey);
                st.add(bs);
            }
            actKeysPerInner.add(ak.toArray());
            setsPerInner.add(st.toArray());
        }

        final int nInner = keys.size();
        final int[] innerSizes = new int[nInner];
        for (int i = 0; i < nInner; i++) {
            innerSizes[i] = ((Map<?, ?>) inners.get(i)).size();
        }
        final int[][] setSizes = new int[nInner][];
        for (int i = 0; i < nInner; i++) {
            final Object[] sts = setsPerInner.get(i);
            setSizes[i] = new int[sts.length];
            for (int j = 0; j < sts.length; j++) {
                setSizes[i][j] = ((Set<?>) sts[j]).size();
            }
        }
        final Object[] acts = actsA.toArray();
        final boolean[] gs = new boolean[groupStartA.size()];
        for (int i = 0; i < gs.length; i++) {
            gs[i] = groupStartA.get(i).booleanValue();
        }

        return new Snapshot(outer,
                keys.toArray(new Integer[0]),
                inners.toArray(), innerSizes,
                actKeysPerInner.toArray(new Object[0][]),
                setsPerInner.toArray(new Object[0][]), setSizes,
                acts, behsA, gs);
    }
}
