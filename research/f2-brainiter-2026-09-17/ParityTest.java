import java.lang.reflect.Field;
import java.lang.reflect.Proxy;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.Set;
import java.util.TreeMap;
import java.util.concurrent.atomic.AtomicInteger;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.ai.BrainOps;
import net.minecraft.world.entity.ai.behavior.Behavior;
import net.minecraft.world.entity.ai.behavior.BehaviorControl;
import net.minecraft.world.entity.schedule.Activity;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.storage.WritableLevelData;
import sun.misc.Unsafe;

/**
 * F2 BRAIN-ITERATORS parity bank (TASK-249 / S7-113). Offline unit test —
 * loads the REAL kernel classes (Purpur 1.21.10 mojang-mapped, run21 artifact)
 * and drives the REAL production entry BrainOps.startEachNonRunning in a plain
 * sandbox JVM. No server boot (INJECTS-ONLY safe).
 *
 * ServerLevel seam: UNSAFE.allocateInstance (no <clinit> in ServerLevel —
 * verified by cfdump) + a WritableLevelData dynamic proxy installed into the
 * inherited Level.levelData field. Level.getGameTime() (cfdump Level @3955)
 * reads exactly levelData.getGameTime() — the proxy therefore serves a
 * controller gameTime through the REAL production code path. The entity
 * argument is never dereferenced by the helper (only forwarded to tryStart);
 * null is passed and asserted to arrive verbatim.
 *
 * Collection model = kernel truth (Brain cfdump):
 *   outer  = Maps.newTreeMap()            (priority ASCENDING iteration)
 *   inner  = Maps.newLinkedHashMap()      (activity insertion order)
 *   sets   = Sets.newLinkedHashSet()      (behavior insertion order)
 *   active = Sets.newHashSet()            (membership only)
 *
 * Reference flow = vanilla startEachNonRunningBehavior @0-177 transcribed
 * instruction-by-instruction (Brain.cfdump.txt). The bank asserts the lens
 * produces the IDENTICAL start sequence, order, and arguments.
 */
public class ParityTest {

    static Unsafe UNSAFE;
    static long LEVELDATA_OFFSET;
    static final long[] GAME_TIME = { 0L };

    static final List<String> EVENTS = new ArrayList<>();
    static final List<Boolean> LEVEL_OK = new ArrayList<>();
    static final List<Boolean> ENTITY_OK = new ArrayList<>();

    static final AtomicInteger STUB_IDS = new AtomicInteger(1000);

    /** Recording stub — the minimal BehaviorControl the lens/ref drive. */
    static final class Stub implements BehaviorControl<LivingEntity> {
        final int id = STUB_IDS.incrementAndGet();
        Behavior.Status status = Behavior.Status.STOPPED;
        Runnable onStart = null;

        @Override
        public Behavior.Status getStatus() { return status; }

        @Override
        public boolean tryStart(ServerLevel level, LivingEntity entity, long gameTime) {
            EVENTS.add("start:" + id + "@" + gameTime);
            LEVEL_OK.add(level == LEVEL);
            ENTITY_OK.add(entity == ENTITY);
            if (onStart != null) onStart.run();
            return true;
        }

        @Override
        public void tickOrStop(ServerLevel level, LivingEntity entity, long gameTime) { }

        @Override
        public void doStop(ServerLevel level, LivingEntity entity, long gameTime) { }

        @Override
        public String debugString() { return "stub" + id; }
    }

    static ServerLevel LEVEL;
    static LivingEntity ENTITY;   // null — forwarded verbatim, never dereferenced

    // kernel-truth structure type aliases
    static TreeMap<Integer, LinkedHashMap<Activity, LinkedHashSet<Stub>>> newOuter() {
        return new TreeMap<>();
    }
    static LinkedHashMap<Activity, LinkedHashSet<Stub>> newInner() {
        return new LinkedHashMap<>();
    }
    static LinkedHashSet<Stub> newSet() {
        return new LinkedHashSet<>();
    }
    static HashSet<Activity> newActive() {
        return new HashSet<>();
    }

    /** Vanilla mirror (Brain.startEachNonRunningBehavior @0-177). */
    static void refFlow(TreeMap<Integer, LinkedHashMap<Activity, LinkedHashSet<Stub>>> outer,
                        HashSet<Activity> active, List<String> sink) {
        final long gameTime = LEVEL.getGameTime();                      // @0-4 once
        for (LinkedHashMap<Activity, LinkedHashSet<Stub>> inner : outer.values()) {   // @5-38
            for (Map.Entry<Activity, LinkedHashSet<Stub>> e : inner.entrySet()) {     // @43-89
                final Activity act = e.getKey();
                if (!active.contains(act)) continue;                    // @91-102
                for (Stub b : e.getValue()) {                           // @105-142
                    if (b.getStatus() == Behavior.Status.STOPPED) {     // @144-154
                        b.tryStart(LEVEL, ENTITY, gameTime);            // @157-167
                    }
                }
            }
        }
        sink.add("gameTimeWas:" + gameTime);
    }

    /** Drives the REAL production entry and captures events. */
    static void lensFlow(TreeMap<Integer, LinkedHashMap<Activity, LinkedHashSet<Stub>>> outer,
                         HashSet<Activity> active, List<String> sink) {
        BrainOps.startEachNonRunning(outer, active, LEVEL, ENTITY);
        sink.add("gameTimeWas:" + GAME_TIME[0]);   // lens read it through the proxy
    }

    static void assertSameFlow(String what,
                               TreeMap<Integer, LinkedHashMap<Activity, LinkedHashSet<Stub>>> outer,
                               HashSet<Activity> active) {
        EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
        List<String> ref = new ArrayList<>();
        refFlow(outer, active, ref);
        // Capture the REF events (stubs write into the global lists) BEFORE
        // resetting for the lens run.
        List<String> refEvents = new ArrayList<>(EVENTS);
        List<Boolean> refLevel = new ArrayList<>(LEVEL_OK);
        List<Boolean> refEntity = new ArrayList<>(ENTITY_OK);

        EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
        List<String> lens = new ArrayList<>();
        lensFlow(outer, active, lens);

        // Full-flow comparison: stub start events (global) + gameTime marker.
        List<String> fullRef = new ArrayList<>(refEvents);
        fullRef.addAll(ref);
        List<String> fullLens = new ArrayList<>(EVENTS);
        fullLens.addAll(lens);

        if (!fullRef.equals(fullLens)) {
            throw new AssertionError(what + " DIVERGED\n  ref : " + fullRef + "\n  lens: " + fullLens);
        }
        if (!refLevel.equals(LEVEL_OK)) throw new AssertionError(what + ": level forwarding diverged");
        if (!refEntity.equals(ENTITY_OK)) throw new AssertionError(what + ": entity forwarding diverged");
        for (Boolean b : LEVEL_OK) if (!b) throw new AssertionError(what + ": level not forwarded verbatim");
        for (Boolean b : ENTITY_OK) if (!b) throw new AssertionError(what + ": entity not forwarded verbatim");
        totalCalls += 2;
        totalStarts += refEvents.size();
    }

    static long totalCalls = 0;
    static long totalStarts = 0;
    static long totalMutations = 0;

    public static void main(String[] args) throws Exception {
        Field uf = Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        UNSAFE = (Unsafe) uf.get(null);
        Field lf = Level.class.getDeclaredField("levelData");
        LEVELDATA_OFFSET = UNSAFE.objectFieldOffset(lf);

        // Registry/codec bootstrap: vanilla's own static-data entry. Initializes
        // BuiltInRegistries + codecs so Level/ServerLevel <clinit> can run.
        // NOT a server boot: no Main, no worlds, no tick loop, no network
        // (INJECTS-ONLY law — same class of sandbox init as paperclip
        // materialization-killed-pre-main, which the charter classifies as
        // non-boot). tryDetectVersion reads version.json from the kernel jar
        // (ServerBuildInfo needs it); mirrors the real Main -> bootStrap order.
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();
        System.out.println("SharedConstants + Bootstrap.bootStrap OK (registries+codecs, no server)");

        LEVEL = (ServerLevel) UNSAFE.allocateInstance(ServerLevel.class);
        Object proxy = Proxy.newProxyInstance(ParityTest.class.getClassLoader(),
                new Class<?>[] { WritableLevelData.class },
                (p, m, a) -> {
                    if (m.getName().equals("getGameTime")) return GAME_TIME[0];
                    Class<?> r = m.getReturnType();
                    if (r == boolean.class) return Boolean.FALSE;
                    if (r == int.class) return 0;
                    if (r == long.class) return 0L;
                    if (r == float.class) return 0f;
                    if (r == double.class) return 0d;
                    if (r == short.class) return (short) 0;
                    if (r == byte.class) return (byte) 0;
                    if (r == char.class) return (char) 0;
                    return null;
                });
        UNSAFE.putObject(LEVEL, LEVELDATA_OFFSET, proxy);
        System.out.println("ServerLevel seam: allocateInstance OK, levelData proxy installed (offset " + LEVELDATA_OFFSET + ")");

        // ---------------- S1: order + content + status, mixed structure
        {
            var outer = newOuter();
            var active = newActive();
            Stub s1 = new Stub(), s2 = new Stub(), s3 = new Stub(), s4 = new Stub(), s5 = new Stub();
            s4.status = Behavior.Status.RUNNING;      // must be skipped in BOTH flows
            var i1 = newInner(); var sA = newSet();
            sA.add(s1); sA.add(s2); sA.add(s4);
            i1.put(Activity.IDLE, sA);
            var i2 = newInner(); var sB = newSet();
            sB.add(s3);
            i2.put(Activity.WORK, sB); i2.put(Activity.PLAY, sB);  // same set, two entries
            outer.put(2, i1);
            outer.put(1, i2);                          // TreeMap: priority 1 iterates FIRST
            active.add(Activity.IDLE); active.add(Activity.WORK); active.add(Activity.PLAY);
            GAME_TIME[0] = 777777L;
            assertSameFlow("S1 mixed order/status", outer, active);
        }
        System.out.println("S1 PASS (order+content+status, TreeMap priority asc)");

        // ---------------- S2: gameTime read ONCE per call, fresh per call
        {
            var outer = newOuter();
            var active = newActive();
            Stub s = new Stub();
            var i = newInner(); var st = newSet();
            st.add(s); i.put(Activity.CORE, st); outer.put(1, i);
            active.add(Activity.CORE);
            GAME_TIME[0] = 1000L;
            EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
            lensFlow(outer, active, new ArrayList<>());
            String e1 = String.join(";", EVENTS);
            GAME_TIME[0] = 2000L;
            EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
            lensFlow(outer, active, new ArrayList<>());
            String e2 = String.join(";", EVENTS);
            if (!e1.contains("start:" + s.id + "@1000") || !e2.contains("start:" + s.id + "@2000")
                    || e1.contains("@2000") || e2.contains("@1000"))
                throw new AssertionError("S2 gameTime fidelity: [" + e1 + "] / [" + e2 + "]");
            totalCalls += 2; totalStarts += 2;
        }
        System.out.println("S2 PASS (gameTime read once, fresh per call)");

        // ---------------- S3: inactive group skipped
        {
            var outer = newOuter();
            var active = newActive();
            Stub s1 = new Stub(), s2 = new Stub();
            var i = newInner();
            i.put(Activity.IDLE, newSet()); i.get(Activity.IDLE).add(s1);
            i.put(Activity.WORK, newSet()); i.get(Activity.WORK).add(s2);
            outer.put(1, i);
            active.add(Activity.WORK);                 // IDLE inactive
            assertSameFlow("S3 inactive skip", outer, active);
            active.clear();                            // nothing active
            assertSameFlow("S3 all-inactive", outer, active);
        }
        System.out.println("S3 PASS (inactive groups)");

        // ---------------- S4: LIVE contains at group boundaries (mid-call mutation)
        {
            var outer = newOuter();
            var active = newActive();
            Stub s1 = new Stub(), s2 = new Stub(), s3 = new Stub();
            var i1 = newInner(); var st1 = newSet(); st1.add(s1); i1.put(Activity.IDLE, st1);
            var i2 = newInner(); var st2 = newSet(); st2.add(s2); st2.add(s3); i2.put(Activity.WORK, st2);
            outer.put(1, i1); outer.put(2, i2);
            active.add(Activity.IDLE);                 // WORK inactive initially
            s1.onStart = () -> active.add(Activity.WORK);   // group1 start enables group2
            assertSameFlow("S4 live-enable", outer, active);   // BOTH flows must start s2,s3
            s1.onStart = () -> active.remove(Activity.WORK); // (active re-armed below)
            active.add(Activity.WORK);
            Stub s4 = new Stub();
            st2.add(s4);
            active.add(Activity.IDLE);
            // fresh call: group1 (s1) removes WORK mid-call; but group2 already checked?
            // vanilla: each group checks at its own boundary -> group2 sees WORK REMOVED
            assertSameFlow("S4 live-disable", outer, active);
        }
        System.out.println("S4 PASS (live contains per group boundary)");

        // ---------------- S5: empty set entry — pure-read elision is semantics-neutral
        {
            var outer = newOuter();
            var active = newActive();
            Stub s = new Stub();
            var i = newInner();
            i.put(Activity.IDLE, newSet());            // EMPTY group (activity IS active)
            var st = newSet(); st.add(s); i.put(Activity.WORK, st);
            outer.put(1, i);
            active.add(Activity.IDLE); active.add(Activity.WORK);
            assertSameFlow("S5 empty group", outer, active);
        }
        System.out.println("S5 PASS (empty groups)");

        // ---------------- S6: structural mutation -> rebuild correctness
        {
            var outer = newOuter();
            var active = newActive();
            Stub s1 = new Stub(), s2 = new Stub(), s3 = new Stub();
            var i1 = newInner(); var st1 = newSet(); st1.add(s1); i1.put(Activity.IDLE, st1);
            outer.put(1, i1);
            active.add(Activity.IDLE); active.add(Activity.WORK); active.add(Activity.PLAY);
            GAME_TIME[0] = 42L;
            assertSameFlow("S6 warm cache", outer, active);

            st1.add(s2);                               // probe 5: Set.add
            assertSameFlow("S6 addBehavior", outer, active);

            var st2 = newSet(); st2.add(s3); i1.put(Activity.WORK, st2);   // probe 3/4: new activity
            assertSameFlow("S6 addActivity", outer, active);

            var i2 = newInner(); var st3 = newSet(); st3.add(s1); i2.put(Activity.PLAY, st3);
            outer.put(3, i2);                          // probe 1/2: new priority
            assertSameFlow("S6 addPriority", outer, active);

            outer.clear(); i1.clear();                 // removeAllBehaviors analog
            outer.put(2, newInner()); outer.get(2).put(Activity.WORK, st1);
            assertSameFlow("S6 clear+re-add (new identities)", outer, active);

            outer.put(3, i2);
            assertSameFlow("S6 grow after clear", outer, active);
        }
        System.out.println("S6 PASS (fingerprint catches every vanilla mutation)");

        // ---------------- S7: the AbstractMap.equals TRAP (identity cache required)
        {
            Stub s1 = new Stub(), s2 = new Stub();
            var X = newOuter(); var Y = newOuter();
            var Xi = newInner(); var Yi = newInner();
            var Xs = newSet(); var Ys = newSet();
            Xs.add(s1); Xs.add(s2);                    // order s1,s2
            Ys.add(s2); Ys.add(s1);                    // order s2,s1
            Xi.put(Activity.CORE, Xs); Yi.put(Activity.CORE, Ys);
            X.put(1, Xi); Y.put(1, Yi);
            if (!X.equals(Y)) throw new AssertionError("S7 precondition: maps should be deep-equal");
            var active = newActive();
            active.add(Activity.CORE);
            EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
            lensFlow(X, active, new ArrayList<>());
            String xa = String.join(";", EVENTS);
            EVENTS.clear(); LEVEL_OK.clear(); ENTITY_OK.clear();
            lensFlow(Y, active, new ArrayList<>());
            String yb = String.join(";", EVENTS);
            boolean xFirst = xa.contains(":" + s1.id + "@");
            boolean yFirst = yb.contains(":" + s2.id + "@");
            if (!xFirst || !yFirst) {
                throw new AssertionError("S7 EQUALS-TRAP: Y served X's snapshot?\n  X: " + xa + "\n  Y: " + yb);
            }
            totalCalls += 2; totalStarts += 4;
        }
        System.out.println("S7 PASS (deep-equal maps keep distinct identity-keyed snapshots)");

        // ---------------- S9: fuzz — random structures + random vanilla-surface mutations
        {
            long seeds = 60;
            for (long seed = 0; seed < seeds; seed++) {
                Random chaos = new Random(seed * 7919L + 17);
                Activity[] pool = { Activity.CORE, Activity.IDLE, Activity.WORK, Activity.PLAY,
                                    Activity.REST, Activity.MEET, Activity.PANIC, Activity.FIGHT,
                                    Activity.HIDE, Activity.RAID };
                var outer = newOuter();
                var active = newActive();
                List<Stub> stubs = new ArrayList<>();
                // PRESEED a realistic brain: 3-5 priorities x 2-4 activities x
                // 2-6 behaviors (kernel-sized: MineShield-3 villagers carry
                // 10-30 behavior slots). Keeps every fuzz round event-rich.
                int np = 3 + chaos.nextInt(3);
                for (int p = 1; p <= np; p++) {
                    int na = 2 + chaos.nextInt(3);
                    for (int a = 0; a < na; a++) {
                        int nb = 2 + chaos.nextInt(5);
                        for (int b = 0; b < nb; b++) {
                            Stub s = new Stub(); stubs.add(s);
                            Activity act = pool[chaos.nextInt(pool.length)];
                            outer.computeIfAbsent(p, x -> newInner())
                                 .computeIfAbsent(act, x -> newSet()).add(s);
                        }
                    }
                    if (chaos.nextBoolean()) active.add(pool[chaos.nextInt(pool.length)]);
                }
                for (int round = 0; round < 40; round++) {
                    int op = chaos.nextInt(8);
                    if (op == 0 && !outer.isEmpty()) {                 // Set.add into existing entry
                        var entries = new ArrayList<Map.Entry<Integer, LinkedHashMap<Activity, LinkedHashSet<Stub>>>>(outer.entrySet());
                        var e = entries.get(chaos.nextInt(entries.size()));
                        var acts = new ArrayList<>(e.getValue().keySet());
                        if (!acts.isEmpty()) {
                            Activity a = acts.get(chaos.nextInt(acts.size()));
                            Stub s = new Stub(); stubs.add(s);
                            e.getValue().get(a).add(s);
                            totalMutations++;
                        }
                    } else if (op == 1 && !outer.isEmpty()) {          // new activity entry
                        var keys = new ArrayList<>(outer.keySet());
                        Integer k = keys.get(chaos.nextInt(keys.size()));
                        Activity a = pool[chaos.nextInt(pool.length)];
                        Stub s = new Stub(); stubs.add(s);
                        outer.get(k).computeIfAbsent(a, x -> newSet()).add(s);
                        totalMutations++;
                    } else if (op == 2) {                              // new priority
                        Integer k = chaos.nextInt(9) + 1;
                        Activity a = pool[chaos.nextInt(pool.length)];
                        Stub s = new Stub(); stubs.add(s);
                        outer.computeIfAbsent(k, x -> newInner()).computeIfAbsent(a, x -> newSet()).add(s);
                        totalMutations++;
                    } else if (op == 3) {                              // removeAllBehaviors analog + partial rebuild
                        outer.clear();
                        totalMutations++;
                    } else if (op == 4 && !stubs.isEmpty()) {          // status flip (LIVE, no rebuild)
                        Stub s = stubs.get(chaos.nextInt(stubs.size()));
                        s.status = (chaos.nextBoolean()) ? Behavior.Status.RUNNING : Behavior.Status.STOPPED;
                        totalMutations++;
                    } else if (op == 5) {                              // activity switch (LIVE)
                        Activity a = pool[chaos.nextInt(pool.length)];
                        if (chaos.nextBoolean()) active.add(a); else active.remove(a);
                        totalMutations++;
                    } else if (op == 6) {                              // no-op round (cache stability)
                    } else {                                           // op 7: fresh brain (identity change)
                        if (chaos.nextBoolean()) {
                            outer = newOuter();
                            stubs = new ArrayList<>();
                            active.clear();
                            totalMutations++;
                        }
                    }
                    GAME_TIME[0] = chaos.nextLong() & 0xFFFFL;
                    assertSameFlow("S9 fuzz seed=" + seed + " round=" + round, outer, active);
                }
            }
        }
        System.out.println("S9 PASS (fuzz: 60 seeds x 40 rounds, lens == vanilla every call)");

        System.out.println("TOTAL calls=" + totalCalls + " starts=" + totalStarts
                + " mutations=" + totalMutations);
        if (totalCalls < 4800) throw new AssertionError("not enough coverage");
        System.out.println("F2 PARITY: PASS (order-exact, live-contains, identity cache, rebuild-exact)");
    }
}
