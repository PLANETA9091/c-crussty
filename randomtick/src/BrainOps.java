package net.minecraft.world.entity.ai;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.WeakHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.logging.Logger;

import net.minecraft.server.MinecraftServer;
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

    // ------------------------------------------------------- tick2 (sense D)

    /**
     * SENSE tick2 lane (TASK-442-D, vector cmp438_sense family ∨ composite
     * cmp439_sense_scan): the running-behavior tick pass. Vanilla
     * {@code Brain.tickEachRunningBehavior} re-walks the ENTIRE
     * availableBehaviorsByPriority map THROUGH
     * {@code getRunningBehaviors()} — a fresh ObjectArrayList allocation plus
     * the triple-nested iterator machinery (the same walk F2 killed for the
     * start pass) EVERY tick per brain mob, just to collect RUNNING statuses,
     * then calls {@code tickOrStop} per element.
     *
     * REPLACEMENT (decision-exact): the cached flat snapshot (same lens, same
     * fingerprint, same order) + a PER-BRAIN reusable mask:
     *   pass 1: mask[i] = (behs[i].getStatus() == RUNNING)   == the
     *           getRunningBehaviors walk (statuses read LIVE at list-build
     *           time — the vanilla list is snapshotted BEFORE any tickOrStop
     *           runs, so a behavior stopped mid-loop by an earlier doTick is
     *           STILL ticked later by vanilla; the mask replicates exactly
     *           that, reading statuses live in pass 1 only);
     *   pass 2: for i: if (mask[i]) behs[i].tickOrStop(level, entity, gameTime)
     *           in flat order == vanilla list order.
     * gameTime is read ONCE per call (vanilla @0-4). ZERO allocation steady
     * state (mask lives inside the per-brain Snapshot, sized at build time;
     * a brain is ticked by at most one region thread per tick — the mask is
     * thread-confined per tick; worst-case race on brain migration writes
     * the same boolean[] deterministically).
     *
     * FAIL-CLOSED: this method is INERT until the rust side body-swaps
     * Brain.tickEachRunningBehavior (brainhook tick2 gate — family
     * cmp438_sense ∨ cmp439_sense_scan, selfTestTickEach==true BEFORE arm).
     * Vanilla stopAll keeps vanilla getRunningBehaviors (rare path).
     */
    public static void tickEachRunning(Map<?, ?> byPriority, ServerLevel level,
                                       LivingEntity entity) {
        final long gameTime = level.getGameTime();          // vanilla @0-4: read ONCE
        final Snapshot s = snapshot(byPriority);
        final BehaviorControl<LivingEntity>[] behs = s.behs;
        final boolean[] mask = s.runningMask;
        int n = 0;
        for (int i = 0; i < behs.length; i++) {             // == getRunningBehaviors walk
            final boolean r = behs[i].getStatus() == Behavior.Status.RUNNING;
            mask[i] = r;
            if (r) {
                n++;
            }
        }
        if (n == 0) {
            return;
        }
        for (int i = 0; i < behs.length; i++) {             // == vanilla list order
            if (mask[i]) {
                behs[i].tickOrStop(level, entity, gameTime);
                if (!TICK2_EFFECT_LOGGED.get()) {
                    logTick2Effect(behs.length);
                }
            }
        }
    }

    /** STRICT family mirror of the rust brainhook tick2 gate (TASK-442-D). */
    static final String TICK2_FLAGS = "cmp438_sense|cmp439_sense_scan|cmp451_senseins|cmp452_mega|cmp457_paldelta|cmp458_swar";

    private static final Logger LOG = Logger.getLogger("crussty-plugin");
    private static final AtomicBoolean TICK2_EFFECT_LOGGED = new AtomicBoolean(false);

    /** One-shot EFFECT-маркер расширенного сайта (verdикты только по нему). */
    private static void logTick2Effect(int behaviors) {
        if (TICK2_EFFECT_LOGGED.getAndSet(true)) {
            return;
        }
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        LOG.info("[crussty-plugin] " + (f == null ? "(off)" : f.trim())
                + ": sense tick2 EFFECT armed (first flat tickEachRunning hit tick "
                + MinecraftServer.getServer().getTickCount()
                + ", behaviorSlots=" + behaviors + ")");
    }

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
        /** Per-brain reusable RUNNING mask (tick2 lane), sized once at build. */
        final boolean[] runningMask;

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
            this.runningMask = new boolean[this.behs.length];
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

    // ------------------------------------------------- tick2 decision oracle

    /**
     * Ванильная реплика тика running-веток (javap Brain.tickEachRunningBehavior
     * + getRunningBehaviors): собранный-СНАЧАЛА список RUNNING (статусы читаются
     * на этапе списка) затем tickOrStop по элементам. Эталон оракула tick2.
     */
    static int[] naiveTickEach(int[] statusAtBuild, int[][] scripted) {
        // collect list (vanilla getRunningBehaviors: statuses at build time)
        int n = 0;
        for (int s : statusAtBuild) {
            if (s == 1) {
                n++;
            }
        }
        final int[] list = new int[n];
        int j = 0;
        for (int i = 0; i < statusAtBuild.length; i++) {
            if (statusAtBuild[i] == 1) {
                list[j++] = i;
            }
        }
        // tick each (scripted[i] = status flips performed by tickOrStop of i)
        final int[] seen = new int[list.length];
        for (int k = 0; k < list.length; k++) {
            seen[k] = list[k];
            if (scripted[list[k]] != null) {
                for (int f : scripted[list[k]]) {
                    statusAtBuild[f] = 0; // mutation visible ONLY to later collects
                }
            }
        }
        return seen;
    }

    /**
     * Ускоренное ядро tick2 (транскрипция tickEachRunning): pass 1 собирает
     * маску живыми статусами, pass 2 зовёт tickOrStop по маске. Statuses here
     * are MUTATED by the same scripted flips — decisions must match naive.
     */
    static int[] coreTickEach(int[] statusLive, int[][] scripted) {
        final int n = statusLive.length;
        final boolean[] mask = new boolean[n];
        int running = 0;
        for (int i = 0; i < n; i++) {
            mask[i] = statusLive[i] == 1;
            if (mask[i]) {
                running++;
            }
        }
        final int[] seen = new int[running];
        int j = 0;
        for (int i = 0; i < n; i++) {
            if (mask[i]) {
                seen[j++] = i;
                if (scripted[i] != null) {
                    for (int f : scripted[i]) {
                        statusLive[f] = 0;
                    }
                }
            }
        }
        return seen;
    }

    /**
     * selfTestTickEach — exhaustive оракул pass-а tick2: n ≤ 4, все паттерны
     * статусов на build + все скрипты остановок (каждый элемент списка может
     * остановить любой ПОЗДНИЙ элемент — mid-loop stop class), core == naive.
     * Вызывается rust-стороной ДО arm (TASK-437-A pattern); false → tick2
     * не применяется (baseline F2 остаётся).
     */
    public static boolean selfTestTickEach() {
        final int[] dists = {0, 1}; // status alphabet: STOPPED/RUNNING
        for (int n = 0; n <= 4; n++) {
            final int combos = 1 << n;
            for (int pm = 0; pm < combos; pm++) {
                // exhaustive mutation scripts: for every subset S of running
                // slots, slot S kills every later slot (the only reachable
                // mutation class — doTick/stop semantics of vanilla Behavior)
                final int scripts = 1 << n;
                for (int sm = 0; sm < scripts; sm++) {
                    int[] build = new int[n];
                    for (int i = 0; i < n; i++) {
                        build[i] = ((pm >> i) & 1) != 0 ? dists[1] : dists[0];
                    }
                    int[][] scripted = new int[n][];
                    for (int i = 0; i < n; i++) {
                        if (((sm >> i) & 1) != 0) {
                            // script: i stops every LATER slot (mask already built)
                            int cnt = 0;
                            for (int t = i + 1; t < n; t++) {
                                cnt++;
                            }
                            final int[] flips = new int[cnt];
                            int w = 0;
                            for (int t = i + 1; t < n; t++) {
                                flips[w++] = t;
                            }
                            scripted[i] = flips;
                        }
                    }
                    final int[] a = naiveTickEach(build.clone(), scripted);
                    final int[] b = coreTickEach(build.clone(), scripted);
                    if (!java.util.Arrays.equals(a, b)) {
                        return false;
                    }
                }
            }
        }
        return true;
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
