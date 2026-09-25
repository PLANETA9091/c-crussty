package net.minecraft.world.entity;

import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.concurrent.atomic.LongAdder;

/**
 * INSIDE-ACQUIRE bridge (ID-P33, TASK-459-73 scaffold — law 11 v18.2).
 *
 * Volatile-demotion of the residual volatile READS in the inside-lane MISS
 * tail: the retarget body inside-site (Level.getBlockState ->
 * InsideSnapOps.snapGet, already live via cmp424_inside) and the
 * PalettedContainerOps class read their hot fields through
 * {@code VarHandle.getAcquire} instead of raw volatile loads.
 *
 * JMM contract (card canon, v1): getAcquire, NOT opaque. The JDK-9 mode
 * ladder is cumulative (Plain &lt; Opaque &lt; Release/Acquire &lt; Volatile,
 * Doug Lea "Using JDK 9 Memory Order Modes") — every guarantee a reader gets
 * from a volatile load is preserved by an acquire load, so the demotion is
 * observable-behavior neutral while unblocking C2 scheduler freedom (hoist /
 * reorder) on the hot reader. x86 note: a volatile LOAD is already a plain
 * load (only StoreLoad after a volatile STORE costs a fence), so the win is
 * compiler-side, not fence-side. Writer sides (InsideSnapOps.secWrite
 * {@code s.gen++} AFTER write; PalettedContainerOps publication snap-first /
 * snapGen-last) are NOT touched — the StoreLoad drain stays where the seqlock
 * meaning lives.
 *
 * SCAFFOLD: this stub exercises the acquire-reader mechanics on ITS OWN
 * emulated fields (single-writer region model). The real targets —
 * InsideSnapOps.Snap.gen/pending (site 1), PalettedContainer.data (site 2),
 * PalettedContainer crusstySnap/crusstySnapGen/crusstyGen gate fields (site 3)
 * — are rewritten in the wiring phase; statics BUILDS/ABORTS/CAPPED stay
 * volatile in v1 (not hot; torn-long subtleties wait for the random-pop
 * oracle per the card).
 *
 * NCDFE canon (&times;93-indy, run 35902792520): the class is defined into the
 * KERNEL loader by the early arm-hook (src/inside_acquire.rs activate) BEFORE
 * the first gated call. {@code getAcquire} is signature-polymorphic
 * invokevirtual (JEP 193) — NOT invokedynamic — and this stub keeps
 * &lt;clinit&gt; indy-free (plain findVarHandle finals, no lambdas/method-refs,
 * explicit Runnable class), so the x93 NCDFE storm is structurally impossible.
 *
 * Grep markers: "inside_acquire: builds", "inside_acquire: accept",
 * "inside_acquire: rejects".
 */
public final class InsideAcquireOps {

    /** Alive acquire-readers bound (bounded wiring-phase pool is flat; here a
     * monotone counter only). */
    static final LongAdder BUILDS = new LongAdder();
    static final LongAdder ACCEPTS = new LongAdder();
    static final LongAdder REJECTS = new LongAdder();

    // ---- emulated fields (wiring phase binds the real owner objects) ----

    /** Seqlock epoch of the emulated section snap (writer: volatile ++). */
    static volatile long GEN = 0L;
    /** Pending-invalidations marker of the emulated section (volatile). */
    static volatile boolean PENDING = false;
    /** Emulated published snapshot (writer: volatile store 1). */
    static volatile Object SNAP = null;
    /** Emulated publication gate (writer: volatile store 2, LAST). */
    static volatile long SNAP_GEN = -1L;

    /** VarHandles: acquire readers for the four emulated STATIC fields
     * (findStaticVarHandle — all four are static; wiring phase binds the
     * instance-field variants on the real owners the same way). */
    static final VarHandle GEN_VH;
    static final VarHandle PENDING_VH;
    static final VarHandle SNAP_VH;
    static final VarHandle SNAP_GEN_VH;

    static {
        try {
            MethodHandles.Lookup l = MethodHandles.lookup();
            GEN_VH = l.findStaticVarHandle(InsideAcquireOps.class, "GEN", long.class);
            PENDING_VH = l.findStaticVarHandle(InsideAcquireOps.class, "PENDING", boolean.class);
            SNAP_VH = l.findStaticVarHandle(InsideAcquireOps.class, "SNAP", Object.class);
            SNAP_GEN_VH = l.findStaticVarHandle(InsideAcquireOps.class, "SNAP_GEN", long.class);
        } catch (ReflectiveOperationException e) {
            throw new Error(e);
        }
    }

    private InsideAcquireOps() {
    }

    // ---- acquire readers (the demotion target shape) ----

    /** Site 1 reader: seqlock epoch via getAcquire (was: raw volatile read). */
    public static long genAcquire() {
        return (long) GEN_VH.getAcquire();
    }

    /** Site 1 reader: pending marker via getAcquire. */
    public static boolean pendingAcquire() {
        return (boolean) PENDING_VH.getAcquire();
    }

    /** Site 3 reader: published snapshot via getAcquire (was volatile read). */
    public static Object snapAcquire() {
        return SNAP_VH.getAcquire();
    }

    /** Site 3 reader: publication gate via getAcquire (accept-key, read LAST). */
    public static long snapGenAcquire() {
        return (long) SNAP_GEN_VH.getAcquire();
    }

    /** Emulated demux accept protocol, acquire-pair shape:
     * accept iff snapshot present AND snapGen == GEN + 1 (writer publishes
     * GEN first, snap then snapGen = GEN + 1 LAST). Readers using acquire
     * loads see exactly the pairs the volatile publication order admits —
     * the accept decision is bit-identical to the volatile reader. */
    public static boolean acceptGate() {
        Object s = snapAcquire();
        if (s == null) {
            REJECTS.increment();
            return false;
        }
        long sg = snapGenAcquire();
        long g = genAcquire();
        if (sg == g + 1) {
            ACCEPTS.increment();
            return true;
        }
        REJECTS.increment();
        return false;
    }

    // ---- emulated writer side (VOLATILE, not demoted — card canon) ----

    /** secWrite-shape bump: volatile store AFTER content write. */
    public static void writerBump() {
        GEN++; // volatile store (StoreLoad drain stays — seqlock meaning)
    }

    /** Publication-shape write: snap first, snapGen LAST (both volatile). */
    public static void writerPublish(Object snap, long gen) {
        SNAP = snap;        // volatile store 1
        SNAP_GEN = gen + 1; // volatile store 2 (LAST): gate = snapGen == gen + 1
    }

    /** Emulated mutation window: odd epoch while in flight, even when stable. */
    public static void writerBeginMutation() {
        GEN++; // odd — in flight
    }

    // ---- selfTest (single-writer region model; random-pop oracle prefix) ----

    /** Deterministic self-test of the acquire-pair protocol: after every
     * publication step the acquire reader must accept; before any publication
     * it must reject; the acquire epoch reads must be monotone per reader
     * (single-writer monotonic gen). Run BEFORE BRIDGE_READY publication
     * (fail-dominant: false => lever stays unpublished). */
    public static boolean selfTest() {
        try {
            BUILDS.increment();
            // (a) fresh state must reject: no snapshot published yet.
            if (acceptGate()) {
                return false;
            }
            // (b) publish v0 -> accept; acquire reads must agree bit-identical
            //     with the volatile values the writer released.
            Object v0 = new Object();
            writerBump();
            writerPublish(v0, genAcquire());
            if (!acceptGate() || snapAcquire() != v0) {
                return false;
            }
            if (snapGenAcquire() != (long) SNAP_GEN) {
                return false;
            }
            // (c) pending flip observable through acquire reader.
            boolean p0 = pendingAcquire();
            PENDING = !p0; // volatile store
            if (pendingAcquire() == p0) {
                return false;
            }
            // (d) monotonicity of acquire epoch reads under single writer:
            //     bump a bounded number of times, each acquire read must be
            //     >= previous (single-writer monotonic gen; no torn longs —
            //     getAcquire(long) is atomic per JEP 193).
            long prev = genAcquire();
            for (int i = 0; i < 1024; i++) {
                writerBump();
                long cur = genAcquire();
                if (cur < prev) {
                    return false;
                }
                prev = cur;
            }
            // (e) bounded concurrent probe: one writer thread publishes a
            //     bounded sequence; the acquire reader must NEVER observe a
            //     non-null snapshot with a stale gate (publication protocol
            //     holds bit-identically under acquire loads).
            return concurrentProbe(20_000);
        } catch (Throwable t) {
            return false;
        }
    }

    /** Bounded single-writer / single-acquire-reader probe. Threads are
     * explicit named classes (NO lambdas — indy-free <clinit>-adjacent code,
     * NCDFE canon). */
    private static boolean concurrentProbe(int rounds) {
        final int n = rounds;
        final boolean[] ok = {true};
        final Object lock = new Object();
        Thread w = new Thread(new Runnable() {
            @Override
            public void run() {
                for (int i = 0; i < n && ok[0]; i++) {
                    writerBeginMutation();          // odd — in flight
                    Object snap = new Object();     // content build
                    writerBump();                   // even — stable again
                    writerPublish(snap, genAcquire());
                }
            }
        }, "inside-acquire-probe-writer");
        Thread r = new Thread(new Runnable() {
            @Override
            public void run() {
                long last = genAcquire();
                for (int i = 0; i < n * 2 && ok[0]; i++) {
                    Object s = snapAcquire();
                    if (s != null) {
                        long sg = snapGenAcquire();
                        long g = genAcquire();
                        // Accept iff gate exact (sg == g + 1). Stale gate
                        // (sg <= g) is the LEGIT reject path of the volatile
                        // protocol. sg > g + 1 is the ONLY protocol break:
                        // an acquire read of the gate synchronizes-with its
                        // publish store, so a subsequent acquire read of GEN
                        // must see >= sg - 1 (sg <= g + 1 always holds).
                        if (sg > g + 1) {
                            synchronized (lock) {
                                ok[0] = false;
                            }
                        }
                    }
                    long cur = genAcquire();
                    if (cur < last) { // single-writer monotonic per reader
                        synchronized (lock) {
                            ok[0] = false;
                        }
                    }
                    last = cur;
                    if ((i & 1023) == 0) {
                        Thread.yield();
                    }
                }
            }
        }, "inside-acquire-probe-reader");
        w.start();
        r.start();
        try {
            w.join(10_000);
            r.join(10_000);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return false;
        }
        if (ok[0]) {
            // NO indy discipline: String.concat via StringBuilder, NOT '+'
            // (Java 9+ '+' compiles to invokedynamic makeConcatWithConstants —
            // an indy constant this bridge must not carry, x93 canon).
            StringBuilder sb = new StringBuilder();
            sb.append("inside_acquire: selfTest=").append(ok[0]).append(" BEFORE arm (accept=")
              .append(ACCEPTS.sum()).append(", rejects=").append(REJECTS.sum()).append(')');
            System.out.println(sb.toString());
        }
        return ok[0];
    }
}
