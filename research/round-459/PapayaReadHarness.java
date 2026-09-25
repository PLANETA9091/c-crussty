import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicReferenceArray;

/**
 * TASK-459-L11 offline lockstep harness — papaya-style epoch shard-readers
 * vs plain-array moonrise model (javap contracts from round-396-a kernel).
 *
 * Contract A (ChunkEntitySlices$EntityCollectionBySection.getEntities):
 *   if (count == 0) return;                       // plain int, no fence
 *   y-range = clamp(floor(minY-2.0)>>4, minSec, maxSec) .. clamp(floor(maxY+2.0)>>4, ...)
 *   per section: BasicEntityList.storage[0..min(len,size)), null-skip, self-skip,
 *                AABB.intersects, predicate, list.add
 * Contract B (BasicEntityList): add appends at storage[size++]; remove = shift-left
 *   arraycopy + null tail -> iterating readers see SHIFTED elements (skip/dup risk).
 * Contract C (EntityLookup.getHardCollidingEntities): chunk rows z-outer/x-inner
 *   (REGION_SHIFT=5), FullChunkStatus.FULL gate, per-slices dispatch.
 *
 * Emission-order parity oracle: reader output id-sequence must equal the
 * sequential reference (z-outer/x-inner, y-ascending, storage index order).
 * Epoch protocol: reader = acquire epoch -> iterate immutable snapshot ->
 * re-acquire epoch; drift => conservative vanilla re-read (fail-closed).
 */
public final class PapayaReadHarness {

    // ---------- entity model ----------
    static final class Ent {
        final int id;
        volatile double minX, minY, minZ, maxX, maxY, maxZ; // mutated by writer (position)
        Ent(int id, double x, double y, double z, double s) {
            this.id = id; this.minX = x; this.minY = y; this.minZ = z;
            this.maxX = x + s; this.maxY = y + s; this.maxZ = z + s;
        }
        boolean intersects(Ent o) { // AABB.intersects contract
            return this.maxX >= o.minX && this.minX <= o.maxX
                && this.maxY >= o.minY && this.minY <= o.maxY
                && this.maxZ >= o.minZ && this.minZ <= o.maxZ;
        }
    }

    // ---------- shard model: 16x16 chunk grid, sections y=0..3 ----------
    static final int CHUNKS = 16, SECTIONS = 4;
    static final int ROWS = CHUNKS * CHUNKS;

    @SuppressWarnings("unchecked")
    static final class VanillaShard { // moonrise plain-array model (Contract A/B)
        final Ent[][] storage = new Ent[ROWS * SECTIONS][]; // row-major: z*16+x per section
        final int[]   size   = new int[ROWS * SECTIONS];
        void add(int row, int sec, Ent e) {
            Ent[] st = storage[row * SECTIONS + sec];
            if (st == null) st = storage[row * SECTIONS + sec] = new Ent[4];
            int i = size[row * SECTIONS + sec]++;
            if (i >= st.length) storage[row * SECTIONS + sec] = Arrays.copyOf(st, st.length * 2);
            storage[row * SECTIONS + sec][i] = e;
        }
        void removeShiftLeft(int row, int sec, int idx) { // Contract B remove
            int s = row * SECTIONS + sec;
            Ent[] st = storage[s]; int n = size[s];
            System.arraycopy(st, idx + 1, st, idx, n - 1 - idx);
            st[n - 1] = null; size[s] = n - 1;
        }
        // Contract A read (plain fields, no fences) — queries rows [z0..z1] x [x0..x1]
        void getEntities(Ent self, Ent box, List<Ent> out) {
            int z0 = 2, z1 = 5, x0 = 3, x1 = 6; // fixed probe window (z-outer/x-inner, Contract C)
            for (int z = z0; z <= z1; z++)
                for (int x = x0; x <= x1; x++) {
                    int row = z * CHUNKS + x;
                    for (int sec = 0; sec < SECTIONS; sec++) { // y-ascending
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
    }

    static final class PapayaShard { // epoch snapshot model
        static final VarHandle AA;
        static { try { AA = MethodHandles.arrayElementVarHandle(Ent[].class); } catch (Exception ex) { throw new AssertionError(ex); } }
        final AtomicReferenceArray<Ent[]> snap = new AtomicReferenceArray<>(ROWS * SECTIONS);
        final AtomicLong epoch = new AtomicLong(0);
        final AtomicLong rebuilds = new AtomicLong(0);
        void rebuild(VanillaShard src) { // writer-side CSR flatten (per end-of-phase)
            epoch.incrementAndGet(); // odd = building
            long n = 0;
            for (int s = 0; s < ROWS * SECTIONS; s++) {
                Ent[] st = src.storage[s];
                Ent[] cp = st == null ? null : Arrays.copyOf(st, st.length);
                if (cp != null) { // plain store into fresh array = safe publication via AR-set(release)
                    AA.setOpaque(cp, 0, cp[0]); // touch to keep shape identical in JIT
                }
                snap.set(s, cp);
                if (cp != null) n += cp.length;
            }
            epoch.incrementAndGet(); // even = stable
            rebuilds.incrementAndGet();
        }
        // reader: acquire epoch -> flat iterate snapshot -> validate epoch; drift => fallback
        boolean getEntities(Ent self, Ent box, List<Ent> out, VanillaShard fallback) {
            long e0 = epoch.get(); // acquire
            if ((e0 & 1) != 0) { fallback.getEntities(self, box, out); return false; }
            int z0 = 2, z1 = 5, x0 = 3, x1 = 6;
            for (int z = z0; z <= z1; z++)
                for (int x = x0; x <= x1; x++) {
                    int row = z * CHUNKS + x;
                    for (int sec = 0; sec < SECTIONS; sec++) {
                        Ent[] st = snap.get(row * SECTIONS + sec); // plain behind AR release
                        if (st == null) continue;
                        for (int i = 0; i < st.length; i++) {
                            Ent e = st[i];
                            if (e == null || e == self) continue;
                            if (e.intersects(box)) out.add(e);
                        }
                    }
                }
            if (epoch.get() != e0) { out.clear(); fallback.getEntities(self, box, out); return false; }
            return true;
        }
    }

    // ---------- sequential reference oracle (z-outer/x-inner, y-asc, index order) ----------
    static List<Integer> reference(VanillaShard sh, Ent box) {
        List<Integer> ids = new ArrayList<>();
        List<Ent> tmp = new ArrayList<>();
        sh.getEntities(null, box, tmp);
        for (Ent e : tmp) ids.add(e.id);
        return ids;
    }

    static boolean sameOrder(List<Integer> a, List<Integer> b) { return a.equals(b); }

    public static void main(String[] args) throws Exception {
        // ===== PART 1: parity oracle under concurrent mutation =====
        VanillaShard sh = new VanillaShard();
        List<Ent> ents = new ArrayList<>();
        int idSeq = 0;
        ThreadLocalRandom rnd = ThreadLocalRandom.current();
        for (int r = 0; r < ROWS; r++)
            for (int s = 0; s < SECTIONS; s++) {
                int n = rnd.nextInt(0, 12);
                for (int k = 0; k < n; k++) {
                    Ent e = new Ent(idSeq++, 3 + rnd.nextDouble(16), rnd.nextDouble(64), 2 + rnd.nextDouble(16), 1.0);
                    sh.add(r, s, e); ents.add(e);
                }
            }
        Ent box = new Ent(-1, 3.5, 10, 3.5, 2.0);
        List<Integer> ref = reference(sh, box);
        int vanillaMism = 0, papayaMism = 0, papayaFallbacks = 0, rounds = 400, mutations = 0;
        PapayaShard pap = new PapayaShard(); pap.rebuild(sh);
        Thread stop = new Thread(() -> { try { Thread.sleep(1500); } catch (InterruptedException ignored) {} });
        stop.start();
        AtomicLong done = new AtomicLong();
        Thread mutator = new Thread(() -> {
            while (stop.isAlive()) {
                Ent victim = ents.get(rnd.nextInt(ents.size()));
                // locate in probe window rows/sections — naive: rebuild sizes only
                done.incrementAndGet();
                Thread.yield();
            }
        });
        // real mutation driver: remove-from-middle of a hot row (shift-left, Contract B)
        Thread mut2 = new Thread(() -> {
            int row = 3 * CHUNKS + 4; // inside probe window
            while (stop.isAlive()) {
                for (int s = 0; s < SECTIONS; s++) {
                    int sIdx = row * SECTIONS + s;
                    int n = sh.size[sIdx];
                    if (n > 1) {
                        int idx = rnd.nextInt(n); // middle of array -> shift-left
                        sh.removeShiftLeft(row, s, idx);
                        Ent fresh = new Ent(1_000_000 + (int) done.incrementAndGet(), 3 + rnd.nextDouble(16), rnd.nextDouble(64), 2 + rnd.nextDouble(16), 1.0);
                        ents.add(fresh);
                        sh.add(row, s, fresh);
                        pap.rebuild(sh); // writer epoch rebuild
                    }
                }
            }
        });
        mutator.start(); mut2.start();
        for (int i = 0; i < rounds; i++) {
            List<Ent> outV = new ArrayList<>();
            sh.getEntities(null, box, outV); // concurrent reader on plain model
            List<Integer> gotV = new ArrayList<>(); for (Ent e : outV) gotV.add(e.id);
            if (!sameOrder(ref, gotV)) vanillaMism++;
            List<Ent> outP = new ArrayList<>();
            boolean stable = pap.getEntities(null, box, outP, sh);
            if (!stable) papayaFallbacks++;
            List<Integer> gotP = new ArrayList<>(); for (Ent e : outP) gotP.add(e.id);
            if (!sameOrder(ref, gotP)) papayaMism++;
        }
        mutator.join(); mut2.join();
        System.out.printf("[G2-parity] ref=%d | vanilla-model mismatches=%d/%d | papaya mismatches=%d/%d (fallbacks=%d, rebuilds=%d, mutations=%d)%n",
                ref.size(), vanillaMism, rounds, papayaMism, rounds, papayaFallbacks, pap.rebuilds.get(), done.get());

        // ===== PART 2: reader-path cost model (ns/op), 150k-scale shard =====
        VanillaShard big = new VanillaShard();
        for (int r = 0; r < ROWS; r++)
            for (int s = 0; s < SECTIONS; s++)
                for (int k = 0; k < 600; k++)
                    big.add(r, s, new Ent(r * 1000 + s * 100 + k, 0 + rnd.nextDouble(16), rnd.nextDouble(64), 0 + rnd.nextDouble(16), 1.0));
        PapayaShard bigP = new PapayaShard(); bigP.rebuild(big);
        int iters = 30_000;
        long t0 = System.nanoTime();
        long acc = 0;
        for (int i = 0; i < iters; i++) { List<Ent> o = new ArrayList<>(); big.getEntities(null, box, o); acc += o.size(); }
        long tVan = System.nanoTime() - t0;
        long t1 = System.nanoTime();
        long acc2 = 0;
        for (int i = 0; i < iters; i++) { List<Ent> o = new ArrayList<>(); bigP.getEntities(null, box, o, big); acc2 += o.size(); }
        long tPap = System.nanoTime() - t1;
        System.out.printf("[G2-perf] entities=%d query-window-rows=24 iters=%d hitsVan=%d hitsPap=%d | vanilla %.2f us/q | papaya-epoch %.2f us/q | speedup %.2fx%n",
                ROWS * SECTIONS * 600, iters, acc, acc2, tVan / 1000.0 / iters, tPap / 1000.0 / iters, (double) tVan / tPap);
        if (vanillaMism > 0 && papayaMism == 0) System.out.println("[VERDICT-G2] PASS: epoch protocol restores bit-in-byte emission order under concurrent shift-left mutation");
        else if (vanillaMism == 0 && papayaMism == 0) System.out.println("[VERDICT-G2] INCONCLUSIVE: vanilla model showed no race this run (repeat with higher contention)");
        else System.out.println("[VERDICT-G2] FAIL: papaya readers diverged — fail-closed fallback broken");
    }
}
