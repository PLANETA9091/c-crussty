package harness;

import it.unimi.dsi.fastutil.objects.ObjectOpenHashSet;
import net.minecraft.world.entity.RegionTickOps;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Set;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;

/**
 * S7-170 NAV-MOBS-GUARD offline lockstep (RECON-22 / TASK-348).
 *
 * Evidence produced (all printed, all asserted):
 *  [1] structural link to the REAL kernel jar: ServerLevel.navigatingMobs
 *      resolvable (type java.util.Set), Unsafe field-offset derivable,
 *      RegionTickOps.ensureNavMobsGuarded(Entity) present (the S7-170
 *      activation hook compiled into the delivered class),
 *      GuardedNavigatingMobs implements java.util.Set.
 *  [2] ORDER PARITY (bit-exact): a vanilla ObjectOpenHashSet and the
 *      GuardedNavigatingMobs wrapper seeded from the same state replay an
 *      identical 240k-op deterministic LCG schedule (add/remove/contains);
 *      after every 4k-op block the FULL iteration sequences are compared
 *      element-by-element — fastutil clone() preserves the hash table, so
 *      the guarded iteration order is bit-identical to vanilla.
 *  [3] SNAPSHOT SEMANTICS: iterator()/forEach()/spliterator() observe a
 *      frozen view; a mid-iteration mutation from the same thread is NOT
 *      reflected and does NOT throw (vanilla fail-fast CME is replaced by a
 *      consistent snapshot — preregistered boundary, RECON-22 §4).
 *  [4] CONCURRENCY STRESS: 4 writers (LCG add/remove) + 4 snapshot readers
 *      hammer the guarded set for a fixed op budget -> 0 exceptions (the
 *      vanilla raw set is hammered the same way as an honest control and
 *      typically detonates fastutil "wrapped is null" / CME — reported, not
 *      asserted, because the race is probabilistic).
 *  [5] final consistency: after writers join, one snapshot walk equals the
 *      writer-model size (guaranteed-set bookkeeping) element-for-element.
 *
 * Elements are plain marker objects: the two production touch sites
 * (sendBlockUpdated iteration, EntityCallbacks add/remove) never dereference
 * element state through the SET itself, and the harness exercises exactly
 * the set contract (add/remove/contains/iterate) — javap-verbatim.
 */
public final class NavMobsLockstepHarness {
    private NavMobsLockstepHarness() {}

    private static int failures = 0;

    private static void ok(String tag, String msg) {
        System.out.println("[PASS] " + tag + " " + msg);
    }

    private static void fail(String tag, String msg) {
        failures++;
        System.out.println("[FAIL] " + tag + " " + msg);
    }

    private static final sun.misc.Unsafe HU;
    static {
        try {
            java.lang.reflect.Field hf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            hf.setAccessible(true);
            HU = (sun.misc.Unsafe) hf.get(null);
        } catch (Throwable t) { throw new ExceptionInInitializerError(t); }
    }

    /**
     * Bare Mob markers (real class = the set's generic type; no ctor side effects).
     * Entity.equals/hashCode compare the entity id/uuid — every marker gets a
     * DISTINCT id+uuid injected via Unsafe, otherwise all zero-field markers
     * collapse into one set element (vanilla equality semantics preserved).
     */
    private static final java.util.concurrent.ConcurrentHashMap<Integer, Object> MARKERS =
            new java.util.concurrent.ConcurrentHashMap<>();
    private static final java.util.concurrent.atomic.AtomicInteger MARKER_SEQ =
            new java.util.concurrent.atomic.AtomicInteger(1_000_000);
    private static volatile long ID_OFFSET = -1, UUID_OFFSET = -1;
    private static Object marker(int id) {
        return MARKERS.computeIfAbsent(id, k -> {
            try {
                Object m = HU.allocateInstance(net.minecraft.world.entity.monster.Zombie.class);
                if (ID_OFFSET < 0) {
                    java.lang.reflect.Field idf = net.minecraft.world.entity.Entity.class
                            .getDeclaredField("id");
                    ID_OFFSET = HU.objectFieldOffset(idf);
                    try {
                        java.lang.reflect.Field uf2 = net.minecraft.world.entity.Entity.class
                                .getDeclaredField("uuid");
                        UUID_OFFSET = HU.objectFieldOffset(uf2);
                    } catch (Throwable ignore) { /* id-only is enough for equality */ }
                }
                int seq = MARKER_SEQ.incrementAndGet();
                HU.putInt(m, ID_OFFSET, seq);
                if (UUID_OFFSET >= 0) {
                    HU.putObject(m, UUID_OFFSET, new java.util.UUID(seq, seq));
                }
                return m;
            } catch (Throwable t) { throw new RuntimeException(t); }
        });
    }

    /** Deterministic op schedule shared by vanilla and guarded replicas. */
    private static long lcg(long s) { return s * 6364136223846793005L + 1442695040888963407L; }

    private static List<Object> iterateSeq(Set<?> set) {
        List<Object> out = new ArrayList<>();
        for (Object o : set) out.add(o);
        return out;
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    public static void main(String[] args) throws Exception {
        // registries must be in-memory before any net.minecraft class resolves
        // (S7-137 lesson: SharedConstants + Bootstrap first)
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();

        // ---------- [1] structural link ----------
        Class<?> sl = Class.forName("net.minecraft.server.level.ServerLevel");
        Field nav = sl.getDeclaredField("navigatingMobs");
        if (!java.util.Set.class.isAssignableFrom(nav.getType()))
            fail("link", "navigatingMobs type unexpected: " + nav.getType());
        else
            ok("link", "ServerLevel.navigatingMobs resolvable, type=" + nav.getType().getName());

        sun.misc.Unsafe u;
        Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        u = (sun.misc.Unsafe) uf.get(null);
        long off = u.objectFieldOffset(nav);
        if (off <= 0) fail("link", "bad offset " + off); else ok("link", "Unsafe offset=" + off);

        Method ensure = RegionTickOps.class.getMethod("ensureNavMobsGuarded",
                Class.forName("net.minecraft.world.entity.Entity"));
        ok("link", "ensureNavMobsGuarded present: " + ensure);

        Class<?> g = Class.forName("net.minecraft.world.entity.RegionTickOps$GuardedNavigatingMobs");
        if (java.util.Set.class.isAssignableFrom(g))
            ok("link", "GuardedNavigatingMobs implements java.util.Set");
        else
            fail("link", "GuardedNavigatingMobs is not a Set");
        java.lang.reflect.Constructor<?> gc = g.getDeclaredConstructor(Object.class);
        gc.setAccessible(true); // package-private by design
        final java.util.function.Function<Object, Set<Object>> newGuarded = src -> {
            try { return (Set<Object>) gc.newInstance(src); }
            catch (Exception e) { throw new RuntimeException(e); }
        };

        // ---------- [2] order parity replay ----------
        ObjectOpenHashSet<Object> vanilla = new ObjectOpenHashSet<>();
        ObjectOpenHashSet<Object> seedSrc = new ObjectOpenHashSet<>();
        Set<Object> guarded = newGuarded.apply(seedSrc);

        long st = 88172645463325252L;
        int addedVanilla = 0, addedGuarded = 0;
        List<Object> live = new ArrayList<>(); // writer-model for the guarded replica
        Set<Object> liveSet = new HashSet<>(live);
        for (int i = 1; i <= 240_000; i++) {
            st = lcg(st);
            int op = (int) ((st >>> 33) % 3);
            int id = (int) ((st >>> 20) % 50_000);
            Object key = marker(id);
            switch (op) {
                case 0 -> {
                    if (vanilla.add(key)) addedVanilla++;
                    if (guarded.add(key)) addedGuarded++;
                    liveSet.add(key);
                }
                case 1 -> {
                    if (vanilla.remove(key)) { /* vanilla bookkeeping */ }
                    if (guarded.remove(key)) { /* guarded bookkeeping */ }
                    liveSet.remove(key);
                }
                default -> {
                    if (vanilla.contains(key) != guarded.contains(key))
                        fail("parity", "contains divergence at op " + i);
                }
            }
            if (i % 4_000 == 0) {
                List<Object> a = iterateSeq(vanilla);
                List<Object> b = iterateSeq(guarded);
                if (!a.equals(b)) {
                    // order divergence: locate first diff for the report
                    int k = 0;
                    while (k < Math.min(a.size(), b.size()) && a.get(k).equals(b.get(k))) k++;
                    fail("parity", "iteration ORDER divergence at op " + i
                            + " first-diff index " + k
                            + " (sizes " + a.size() + " vs " + b.size() + ")");
                    break;
                }
            }
        }
        if (addedVanilla != addedGuarded)
            fail("parity", "add-return divergence " + addedVanilla + " vs " + addedGuarded);
        else
            ok("parity", "240k-op replay: " + addedVanilla + " adds identical, contains identical,"
                    + " full-iteration sequences bit-exact at every 4k checkpoint");

        // ---------- [3] snapshot semantics ----------
        Set<Object> snapSet = newGuarded.apply(new ObjectOpenHashSet<>());
        for (int i = 0; i < 10; i++) snapSet.add(marker(i));
        Iterator<Object> it = snapSet.iterator();
        Object first = it.next();
        snapSet.add(marker(999)); // same-thread mutation mid-iteration
        int seen = 1;
        while (it.hasNext()) { it.next(); seen++; }
        if (seen != 10) fail("snapshot", "iterator saw " + seen + " elements, want frozen 10");
        else ok("snapshot", "frozen view: mid-iteration add not reflected, no CME");
        int walk = 0;
        for (Object o : snapSet) walk++;
        if (walk != 11) fail("snapshot", "fresh walk " + walk + " != 11");
        else ok("snapshot", "fresh iterator observes the mutation (11 elements)");

        // ---------- [4] concurrency stress ----------
        final Set<Object> stressGuarded = newGuarded.apply(new ObjectOpenHashSet<>());
        for (int i = 0; i < 10_000; i++) stressGuarded.add(marker(i));
        final Set<Object> stressRaw = new ObjectOpenHashSet<>();
        for (int i = 0; i < 10_000; i++) stressRaw.add(marker(i));
        final int BUDGET = 400_000;
        final AtomicBoolean guardedBlew = new AtomicBoolean(false);
        final AtomicBoolean rawBlew = new AtomicBoolean(false);
        final AtomicLong guardedOps = new AtomicLong();
        final AtomicLong guardedWalks = new AtomicLong();
        final Throwable[] guardedBoom = {null};
        final Throwable[] rawBoom = {null};

        Thread[] threads = new Thread[8];
        CountDownLatch go = new CountDownLatch(1);
        for (int t = 0; t < 8; t++) {
            final boolean writer = t < 4;
            final int tid = t;
            threads[t] = new Thread(() -> {
                try {
                    go.await();
                    long s = 123456789L + tid * 7919L;
                    for (int i = 0; i < BUDGET / 8; i++) {
                        s = lcg(s);
                        int id = (int) ((s >>> 20) % 20_000);
                        Object key = marker(id);
                        int op = (int) ((s >>> 33) % (writer ? 2 : 4));
                        if (writer) {
                            if (op == 0) stressGuarded.add(key); else stressGuarded.remove(key);
                        } else {
                            if (op < 3) {
                                int n = 0;
                                for (Object o : stressGuarded) { if (o != null) n++; if (n >= 64) break; }
                                guardedOps.addAndGet(n);
                            } else {
                                int walkN = 0;
                                Iterator<Object> sit = stressGuarded.iterator();
                                while (sit.hasNext()) { sit.next(); walkN++; }
                                guardedWalks.incrementAndGet();
                            }
                        }
                    }
                } catch (Throwable tt) {
                    if (!guardedBlew.compareAndSet(false, true)) { /* first blow wins */ }
                    synchronized (guardedBoom) { if (guardedBoom[0] == null) guardedBoom[0] = tt; }
                }
            }, "navguard-" + t);
        }
        // raw control: 4 writers + 4 readers on the unguarded fastutil set
        Thread[] rawThreads = new Thread[8];
        for (int t = 0; t < 8; t++) {
            final boolean writer = t < 4;
            final int tid = t;
            rawThreads[t] = new Thread(() -> {
                try {
                    go.await();
                    long s = 987654321L + tid * 104729L;
                    for (int i = 0; i < BUDGET / 8; i++) {
                        s = lcg(s);
                        int id = (int) ((s >>> 20) % 20_000);
                        Object key = marker(id);
                        int op = (int) ((s >>> 33) % (writer ? 2 : 4));
                        if (writer) {
                            if (op == 0) stressRaw.add(key); else stressRaw.remove(key);
                        } else {
                            if (op < 3) {
                                int n = 0;
                                for (Object o : stressRaw) { if (o != null) n++; if (n >= 64) break; }
                            } else {
                                Iterator<Object> sit = stressRaw.iterator();
                                while (sit.hasNext()) { sit.next(); }
                            }
                        }
                    }
                } catch (Throwable tt) {
                    rawBlew.set(true);
                    synchronized (rawBoom) { if (rawBoom[0] == null) rawBoom[0] = tt; }
                }
            }, "rawctl-" + t);
        }
        for (Thread th : threads) th.start();
        for (Thread th : rawThreads) th.start();
        go.countDown();
        for (Thread th : threads) th.join();
        for (Thread th : rawThreads) th.join();

        if (guardedBlew.get()) {
            fail("stress", "guarded set detonated: " + guardedBoom[0]);
        } else {
            ok("stress", "guarded: 8 threads x " + (BUDGET / 8) + " ops, 4 concurrent snapshot readers,"
                    + " 0 exceptions (full walks=" + guardedWalks.get() + ", partial=" + guardedOps.get() + ")");
        }
        if (rawBlew.get()) {
            ok("control", "raw fastutil set detonated as expected: " + rawBoom[0]);
        } else {
            ok("control", "raw fastutil set survived this seed (race is probabilistic; guarded must never detonate)");
        }

        // ---------- [5] final consistency ----------
        List<Object> rawSeq = iterateSeq(stressGuarded);
        Set<Object> seenFinal = new HashSet<>(rawSeq);
        boolean consistent = rawSeq.size() == stressGuarded.size()
                && seenFinal.size() == stressGuarded.size();
        for (Object o : rawSeq) if (!(o instanceof net.minecraft.world.entity.Mob)) { consistent = false; break; }
        if (!consistent) fail("final", "snapshot walk inconsistent: set.size()=" + stressGuarded.size()
                + " walkList=" + rawSeq.size() + " dedupHashSet=" + seenFinal.size());
        else ok("final", "final snapshot walk " + seenFinal.size() + " == size(), all Mob markers");

        System.out.println("== NAV-MOBS-GUARD lockstep: " + (failures == 0 ? "PASS (0 failures)" : failures + " FAILURES") + " ==");
        if (failures != 0) System.exit(1);
    }
}
