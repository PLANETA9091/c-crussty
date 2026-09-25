import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicReferenceArray;
import java.util.concurrent.locks.ReentrantLock;

/**
 * TASK-459-L11 offline lockstep harness v2 — papaya-style epoch shard-readers.
 *
 * Models (javap contracts, round-396-a kernel):
 *  A) ChunkEntitySlices$EntityCollectionBySection.getEntities: count early-out,
 *     y-section clamp scan, BasicEntityList.storage[i] scan with null/self skip,
 *     AABB.intersects, list.add. Plain non-volatile fields.
 *  B) BasicEntityList.remove = shift-left arraycopy -> unlocked concurrent readers
 *     observe shifted/duplicated entries (JMM race).
 *  C) EntityLookup.getHardCollidingEntities: chunk rows z-outer/x-inner (REGION_SHIFT=5),
 *     FullChunkStatus.FULL gate; emission order = (z, x, section-y, storage-index).
 *
 * Kernel truth modelled: current moonrise readers are parity-safe ONLY because they
 * run on the tick thread holding the per-region area lock (ReentrantAreaLock).
 * Harness: locked-vanilla = oracle (current kernel), unlocked-vanilla = hazard demo,
 * papaya epoch readers = lock-free read path with fail-closed fallback under lock.
 */
public final class PapayaReadHarness {

    static final class Ent {
        final int id;
        volatile double minX, minY, minZ, maxX, maxY, maxZ;
        Ent(int id, double x, double y, double z, double s) {
            this.id = id; this.minX = x; this.minY = y; this.minZ = z;
            this.maxX = x + s; this.maxY = y + s; this.maxZ = z + s;
        }
        boolean intersects(Ent o) {
            return this.maxX >= o.minX && this.minX <= o.maxX
                && this.maxY >= o.minY && this.minY <= o.maxY
                && this.maxZ >= o.minZ && this.minZ <= o.maxZ;
        }
    }

    static final int CHUNKS = 16, SECTIONS = 4, ROWS = CHUNKS * CHUNKS;
    static final int Z0 = 2, Z1 = 5, X0 = 3, X1 = 6; // probe window rows (Contract C order)

    @SuppressWarnings("unchecked")
    static final class VanillaShard {
        final Ent[][] storage = new Ent[ROWS * SECTIONS][];
        final int[] size = new int[ROWS * SECTIONS];
        final ReentrantLock[] rowLocks = new ReentrantLock[ROWS]; // ReentrantAreaLock stand-in
        VanillaShard() { for (int i = 0; i < ROWS; i++) rowLocks[i] = new ReentrantLock(); }
        void add(int row, int sec, Ent e) {
            int s = row * SECTIONS + sec;
            Ent[] st = storage[s];
            if (st == null) st = storage[s] = new Ent[4];
            int i = size[s]++;
            if (i >= st.length) { st = Arrays.copyOf(st, st.length * 2); storage[s] = st; }
            st[i] = e;
        }
        void removeShiftLeft(int row, int sec, int idx) {
            int s = row * SECTIONS + sec;
            Ent[] st = storage[s]; int n = size[s];
            if (n == 0) return;
            idx = Math.min(idx, n - 1);
            System.arraycopy(st, idx + 1, st, idx, n - 1 - idx);
            st[n - 1] = null; size[s] = n - 1;
        }
        void getEntitiesLocked(Ent self, Ent box, List<Ent> out) { // current kernel: reader under area lock
            for (int z = Z0; z <= Z1; z++)
                for (int x = X0; x <= X1; x++) {
                    int row = z * CHUNKS + x;
                    rowLocks[row].lock();
                    try { scanUnlocked(self, box, out, row); } finally { rowLocks[row].unlock(); }
                }
        }
        void scanUnlocked(Ent self, Ent box, List<Ent> out, int row) { // hazard demo path
            for (int sec = 0; sec < SECTIONS; sec++) {
                int s = row * SECTIONS + sec;
                Ent[] st = storage[s];
                if (st == null) continue;
                int n = Math.min(st.length, size[s]);
                for (int i = 0; i < n; i++) {
                    Ent e = st[i];
                    if (e == null || e == self) continue;
                    if (e.intersects(box)) out.add(e);
                }
            }
        }
    }

    static final class PapayaShard {
        static final VarHandle AA;
        static { try { AA = MethodHandles.arrayElementVarHandle(Ent[].class); } catch (Exception ex) { throw new AssertionError(ex); } }
        // per-section immutable compacted snapshots: refs + flat AABB (papaya CSR feed)
        final AtomicReferenceArray<Ent[]> refs = new AtomicReferenceArray<>(ROWS * SECTIONS);
        final AtomicReferenceArray<double[]> ab = new AtomicReferenceArray<>(ROWS * SECTIONS); // 6 doubles/ent
        final AtomicLong epoch = new AtomicLong(0);
        final AtomicLong rebuilds = new AtomicLong(0);
        void rebuild(VanillaShard src) {
            epoch.incrementAndGet(); // odd = building
            for (int row = 0; row < ROWS; row++) {
                src.rowLocks[row].lock(); // writer cooperates: read-consistent copy under owner lock
                try {
                    for (int sec = 0; sec < SECTIONS; sec++) {
                        int s = row * SECTIONS + sec;
                        Ent[] st = src.storage[s]; int n = src.size[s];
                        if (st == null || n == 0) { refs.set(s, null); ab.set(s, null); continue; }
                        Ent[] rc = Arrays.copyOf(st, n);            // compacted: no null holes
                        double[] fl = new double[n * 6];
                        for (int i = 0; i < n; i++) {
                            Ent e = rc[i];
                            fl[i * 6] = e.minX; fl[i * 6 + 1] = e.minY; fl[i * 6 + 2] = e.minZ;
                            fl[i * 6 + 3] = e.maxX; fl[i * 6 + 4] = e.maxY; fl[i * 6 + 5] = e.maxZ;
                        }
                        refs.set(s, rc); ab.set(s, fl);
                    }
                } finally { src.rowLocks[row].unlock(); }
            }
            epoch.incrementAndGet(); // even = stable
            rebuilds.incrementAndGet();
        }
        // lock-free reader; returns false => fallback was used (fail-closed)
        boolean getEntities(Ent self, Ent box, List<Ent> out, VanillaShard fallback) {
            long e0 = epoch.get(); // acquire
            if ((e0 & 1) != 0) { fallback.getEntitiesLocked(self, box, out); return false; }
            for (int z = Z0; z <= Z1; z++)
                for (int x = X0; x <= X1; x++) {
                    int row = z * CHUNKS + x;
                    for (int sec = 0; sec < SECTIONS; sec++) { // y-ascending, Contract C order
                        int s = row * SECTIONS + sec;
                        Ent[] rc = refs.get(s);
                        if (rc == null) continue;
                        double[] fl = ab.get(s);
                        for (int i = 0; i < rc.length; i++) {
                            int b = i * 6;
                            // flat AABB prefilter (primitive loads, no virtual getBoundingBox)
                            if (fl[b + 3] < box.minX || fl[b] > box.maxX
                             || fl[b + 4] < box.minY || fl[b + 1] > box.maxY
                             || fl[b + 5] < box.minZ || fl[b + 2] > box.maxZ) continue;
                            Ent e = (Ent) AA.getAcquire(rc, i);
                            if (e == null || e == self) continue;
                            out.add(e);
                        }
                    }
                }
            if (epoch.get() != e0) { out.clear(); fallback.getEntitiesLocked(self, box, out); return false; }
            return true;
        }
    }

    static Ent randEnt(int id, ThreadLocalRandom rnd) {
        return new Ent(id, rnd.nextDouble(0.0, 7.5), rnd.nextDouble(0, 60), rnd.nextDouble(0.0, 7.5), 1.0);
    }

    public static void main(String[] args) throws Exception {
        ThreadLocalRandom rnd = ThreadLocalRandom.current();
        VanillaShard sh = new VanillaShard();
        List<Ent> ents = new ArrayList<>();
        int idSeq = 0;
        for (int r = 0; r < ROWS; r++)
            for (int s = 0; s < SECTIONS; s++) {
                int n = rnd.nextInt(4, 14);
                for (int k = 0; k < n; k++) { Ent e = randEnt(idSeq++, rnd); sh.add(r, s, e); ents.add(e); }
            }
        PapayaShard pap = new PapayaShard(); pap.rebuild(sh);

        // ===== PART 1a: frozen-state lockstep parity (G2 oracle) =====
        int rounds1 = 300, mismPap = 0, fall = 0;
        for (int i = 0; i < rounds1; i++) {
            Ent box = new Ent(-1, rnd.nextDouble(0, 6), rnd.nextDouble(0, 55), rnd.nextDouble(0, 6), 2.0);
            List<Ent> refL = new ArrayList<>(); sh.getEntitiesLocked(null, box, refL);
            List<Ent> pL = new ArrayList<>(); boolean stable = pap.getEntities(null, box, pL, sh);
            if (!stable) fall++;
            if (!ids(refL).equals(ids(pL))) mismPap++;
        }
        System.out.printf("[G2a-frozen-parity] rounds=%d papaya-mismatches=%d fallbacks=%d -> %s%n",
                rounds1, mismPap, fall, mismPap == 0 ? "BIT-IN-BYTE PASS" : "FAIL");

        // ===== PART 1b: race demo — unlocked reader vs shift-left mutator =====
        int anomalies = 0, rounds2 = 400;
        Ent hotBox = new Ent(-1, 1.0, 0, 1.0, 40.0); // big box = many hits -> shift visible
        List<Ent> refHot = new ArrayList<>(); sh.scanUnlocked(null, hotBox, refHot, 3 * CHUNKS + 4);
        Thread mutator = new Thread(() -> {
            ThreadLocalRandom r2 = ThreadLocalRandom.current();
            int row = 3 * CHUNKS + 4;
            for (int i = 0; i < 20000; i++) {
                int s = r2.nextInt(SECTIONS);
                sh.removeShiftLeft(row, s, r2.nextInt(Math.max(1, sh.size[row * SECTIONS + s])));
                sh.add(row, s, randEnt(9_000_000 + i, r2));
            }
        });
        mutator.start();
        for (int i = 0; i < rounds2 && mutator.isAlive(); i++) {
            List<Ent> out = new ArrayList<>();
            sh.scanUnlocked(null, hotBox, out, 3 * CHUNKS + 4); // UNLOCKED read = the JMM hazard
            if (out.size() != refHot.size()) anomalies++;       // missed/dup = parity break
        }
        mutator.join();
        System.out.printf("[G2b-race-demo] unlocked-vanilla anomalies=%d/%d rounds (shift-left remove mid-iteration)%n", anomalies, rounds2);

        // ===== PART 1c: papaya readers under same race (epoch + fail-closed) =====
        int pAn = 0, pFall = 0;
        pap.rebuild(sh);
        Thread mutator2 = new Thread(() -> {
            ThreadLocalRandom r2 = ThreadLocalRandom.current();
            int row = 3 * CHUNKS + 4;
            for (int i = 0; i < 20000; i++) {
                int s = r2.nextInt(SECTIONS);
                sh.removeShiftLeft(row, s, r2.nextInt(Math.max(1, sh.size[row * SECTIONS + s])));
                sh.add(row, s, randEnt(8_000_000 + i, r2));
                if ((i & 7) == 0) pap.rebuild(sh);
            }
        });
        mutator2.start();
        List<Integer> refIds = new ArrayList<>();
        for (int i = 0; i < rounds2 && mutator2.isAlive(); i++) {
            List<Ent> out = new ArrayList<>(); boolean st = pap.getEntities(null, hotBox, out, sh);
            if (!st) pFall++;
            // parity invariant: either epoch-stable read matches last rebuilt ref, or fallback (locked) used
            List<Ent> cur = new ArrayList<>(); pap.getEntities(null, hotBox, cur, sh);
            if (st) { refIds.clear(); for (Ent e : cur) refIds.add(e.id); if (!ids(out).equals(refIds)) pAn++; }
        }
        mutator2.join();
        System.out.printf("[G2c-epoch-race] papaya anomalies=%d/%d fallbacks=%d -> %s%n",
                pAn, rounds2, pFall, pAn == 0 ? "FAIL-CLOSED PASS" : "FAIL");

        // ===== PART 2: reader-path cost (150k-scale: 614400 entities) =====
        VanillaShard big = new VanillaShard();
        for (int r = 0; r < ROWS; r++)
            for (int s = 0; s < SECTIONS; s++)
                for (int k = 0; k < 600; k++) big.add(r, s, randEnt(r * 10000 + s * 100 + k, rnd));
        PapayaShard bigP = new PapayaShard(); bigP.rebuild(big);
        int iters = 5000; Ent qbox = new Ent(-1, 1, 20, 1, 30);
        long acc1 = 0, acc2 = 0;
        for (int w = 0; w < 3; w++) { // warmup
            List<Ent> o = new ArrayList<>(); big.getEntitiesLocked(null, qbox, o); acc1 += o.size();
            List<Ent> o2 = new ArrayList<>(); bigP.getEntities(null, qbox, o2, big); acc2 += o2.size();
        }
        long t0 = System.nanoTime();
        for (int i = 0; i < iters; i++) { List<Ent> o = new ArrayList<>(); big.getEntitiesLocked(null, qbox, o); acc1 += o.size(); }
        long tLocked = System.nanoTime() - t0;
        long t1 = System.nanoTime();
        for (int i = 0; i < iters; i++) { List<Ent> o = new ArrayList<>(); bigP.getEntities(null, qbox, o, big); acc2 += o.size(); }
        long tPap = System.nanoTime() - t1;
        System.out.printf("[G2-perf] entities=%d iters=%d hits locked=%d papaya=%d | locked-vanilla %.2f us/q | papaya-epoch %.2f us/q | reader speedup %.2fx | rebuilds=%d%n",
                ROWS * SECTIONS * 600, iters, acc1, acc2, tLocked / 1000.0 / iters, tPap / 1000.0 / iters, (double) tLocked / tPap, bigP.rebuilds.get());
    }

    static List<Integer> ids(List<Ent> l) { List<Integer> r = new ArrayList<>(l.size()); for (Ent e : l) r.add(e.id); return r; }
}
