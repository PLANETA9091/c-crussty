import java.util.List;
import java.util.ArrayList;
import java.util.Random;
import java.util.function.IntFunction;

/**
 * AG-249 w530 selftest — ESEL iter-3 java publisher (eselPublish/eselArmNow).
 *
 * Mirrors EntityIndexOps.eselPublish / eselArmNow / $FlatView line-for-line
 * (the kernel-linked class cannot be instantiated offline — same harness
 * pattern as EselFastPathSelfTest / EselNcdfeProbeSelfTest) and validates the
 * two rust-ladder obligations (src/esel_bind.rs push path):
 *
 *  - PUBLISH→ARM ORDER: eselArmNow is a NO-OP until the publish latch
 *    (ESEL_VIEW != null) AND the single-resolver are live; the ARM flip is
 *    the last step and fires the G2 marker strictly after it. ARM-before-
 *    publish would NPE eselFast → eselBroken sticky → plane burned, so the
 *    ladder (arm without publish) must stay inert.
 *  - count==1 ⇔ single!=0 VALIDATION, BOTH SIDES, fail-closed:
 *      count==1 with single==0  → rc=0, view untouched;
 *      single!=0 with count!=1  → rc=0, view untouched;
 *    plus the structural guards (null arrays, length mismatch, empty,
 *    >MAX_SLOTS, negative type/count, duplicate keys, single outside
 *    int-representable positive ids) — ANY violation → rc=0 and the
 *    PREVIOUS published view stays live (a rejected publish must never
 *    blank a good view).
 *
 * GATES: G-P1 valid publish; G-P2 ⇔ left side; G-P3 ⇔ right side;
 * G-P4 structural guards; G-P5 rejected publish preserves old view;
 * G-A1 arm-before-publish no-op; G-A2 arm-without-resolver no-op;
 * G-A3 resolver→arm flips + one G2 marker; G-A4 latch persists (resolver
 * installed after publish still arms); G-V1 view semantics (unknown key →
 * count 0 walk-safe; typeSingle resolver contract); G-V2 probe-map
 * termination/validity over randomized registries (load ≤ 0.5).
 *
 * Exit 0 = all invariants hold; nonzero = regression.
 */
public final class EselPublishArmSelfTest {

    static final int MAX_SLOTS = 4096; // MUST match ESEL_MAX_SLOTS / esel_bind::MAX_SLOTS

    // ---- mirror of EntityIndexOps iter-3 state ----
    static final class PubState {
        // published FlatView payload
        int[] keys, rowOf, counts;
        long[] singles;
        int mask;
        boolean viewLive;
        // arm latch + G2 marker
        boolean armed;
        long markers;
        IntFunction<Integer> resolver; // id → entity handle (IntFunction<Entity> offline)
    }

    static boolean oneMap(int[] types, int[] counts, long[] singles, PubState st) {
        final int n = types.length;
        int cap = 4;
        while (cap < n * 2) cap <<= 1;
        int[] k = new int[cap];
        java.util.Arrays.fill(k, -1);
        int[] r = new int[cap];
        int m = cap - 1;
        for (int row = 0; row < n; row++) {
            int h = types[row] ^ (types[row] >>> 16);
            int i = h & m;
            while (k[i] != -1) i = (i + 1) & m;
            k[i] = types[row];
            r[i] = row;
        }
        st.keys = k; st.rowOf = r; st.counts = counts; st.singles = singles; st.mask = m;
        st.viewLive = true;
        return true;
    }

    static int viewRow(PubState st, int key) {
        if (key < 0) return -1;
        int h = key ^ (key >>> 16);
        int i = h & st.mask;
        while (true) {
            int slot = st.keys[i];
            if (slot == key) return st.rowOf[i];
            if (slot == -1) return -1;
            i = (i + 1) & st.mask;
        }
    }

    static int typeCount(PubState st, int key) {
        int row = viewRow(st, key);
        return row < 0 ? 0 : st.counts[row];
    }

    static Integer typeSingle(PubState st, int key) {
        int row = viewRow(st, key);
        if (row < 0 || st.counts[row] != 1) return null;
        long s = st.singles[row];
        IntFunction<Integer> res = st.resolver;
        if (res == null || s <= 0L || s > Integer.MAX_VALUE) return null;
        return res.apply((int) s);
    }

    // ---- mirror of eselPublish (line-for-line validations) ----
    static int eselPublish(int[] slotType, int[] counts, long[] singles, PubState st) {
        try {
            if (slotType == null || counts == null || singles == null) return 0;
            final int n = slotType.length;
            if (n == 0 || counts.length != n || singles.length != n || n > MAX_SLOTS) return 0;
            for (int i = 0; i < n; i++) {
                if (slotType[i] < 0 || counts[i] < 0) return 0;
                boolean one = counts[i] == 1;
                long s = singles[i];
                if (one != (s != 0L)) return 0;          // count==1 ⇔ single!=0
                if (one && (s < 0L || s > Integer.MAX_VALUE)) return 0;
                for (int j = i + 1; j < n; j++) {
                    if (slotType[j] == slotType[i]) return 0; // unique keys
                }
            }
            oneMap(slotType.clone(), counts.clone(), singles.clone(), st);
            return 1;
        } catch (Throwable t) {
            return 0; // fail-closed: view untouched
        }
    }

    // ---- mirror of eselArmNow (publish latch + resolver guard, ARM last) ----
    static void eselArmNow(PubState st) {
        if (!st.viewLive || st.resolver == null) return; // no latch/resolver → never arm
        st.armed = true;
        st.markers++; // G2 marker strictly after the flip
    }

    static int[] types(int... t) { return t; }
    static int[] counts(int... c) { return c; }
    static long[] singles(long... s) { return s; }

    public static void main(String[] args) {
        List<String> fails = new ArrayList<>();

        // G-P1 valid publish → rc=1, view live and answers lookups
        PubState s1 = new PubState();
        int[] t1 = types(33, 11, 22);
        int[] c1 = counts(1, 0, 5);
        long[] g1 = singles(9001, 0, 0);
        if (eselPublish(t1, c1, g1, s1) != 1) fails.add("G-P1 rc");
        if (!s1.viewLive) fails.add("G-P1 live");
        if (typeCount(s1, 33) != 1 || typeCount(s1, 11) != 0 || typeCount(s1, 22) != 5)
            fails.add("G-P1 counts");
        if (typeCount(s1, 999) != 0) fails.add("G-P1 unknown-walk-safe");

        // G-P2 ⇔ LEFT side: count==1 with single==0 → rc=0, view untouched
        PubState s2 = new PubState();
        eselPublish(t1, c1, g1, s2); // good baseline view
        if (eselPublish(types(7), counts(1), singles(0), s2) != 0) fails.add("G-P2 reject");
        if (typeCount(s2, 33) != 1 || typeCount(s2, 7) != 0) fails.add("G-P2 view-blanked");

        // G-P3 ⇔ RIGHT side: single!=0 with count!=1 → rc=0, view untouched
        if (eselPublish(types(8), counts(0), singles(5), s2) != 0) fails.add("G-P3 count0");
        if (eselPublish(types(8), counts(4), singles(5), s2) != 0) fails.add("G-P3 count4");
        if (typeCount(s2, 8) != 0) fails.add("G-P3 view-blanked");

        // G-P4 structural guards (each must rc=0)
        PubState s4 = new PubState();
        if (eselPublish(null, counts(1), singles(1), s4) != 0) fails.add("G-P4 null-type");
        if (eselPublish(types(1), null, singles(1), s4) != 0) fails.add("G-P4 null-counts");
        if (eselPublish(types(1), counts(1), null, s4) != 0) fails.add("G-P4 null-singles");
        if (eselPublish(types(), counts(), singles(), s4) != 0) fails.add("G-P4 empty");
        if (eselPublish(types(1), counts(1, 0), singles(1, 0), s4) != 0) fails.add("G-P4 len-mismatch");
        int[] many = new int[MAX_SLOTS + 1];
        if (eselPublish(many, many, new long[MAX_SLOTS + 1], s4) != 0) fails.add("G-P4 max-slots");
        if (eselPublish(types(-1), counts(0), singles(0), s4) != 0) fails.add("G-P4 neg-type");
        if (eselPublish(types(1), counts(-1), singles(0), s4) != 0) fails.add("G-P4 neg-count");
        if (eselPublish(types(1, 1), counts(1, 0), singles(1, 0), s4) != 0) fails.add("G-P4 dup-key");
        if (eselPublish(types(1), counts(1), singles(Integer.MIN_VALUE), s4) != 0) fails.add("G-P4 neg-single");
        if (eselPublish(types(1), counts(1), singles((long) Integer.MAX_VALUE + 1L), s4) != 0)
            fails.add("G-P4 over-single");
        if (s4.viewLive) fails.add("G-P4 never-live");

        // G-P5 rejected publish preserves the previous GOOD view (no blanking)
        if (typeCount(s2, 33) != 1 || typeCount(s2, 22) != 5) fails.add("G-P5 baseline");
        if (eselPublish(types(1), counts(1), singles(0), s2) != 0) fails.add("G-P5 reject");
        if (typeCount(s2, 33) != 1 || typeCount(s2, 22) != 5) fails.add("G-P5 view-lost");

        // G-A1 ARM before publish → no-op (even with resolver live)
        PubState s6 = new PubState();
        s6.resolver = id -> id;
        eselArmNow(s6);
        if (s6.armed || s6.markers != 0) fails.add("G-A1 arm-no-publish");

        // G-A2 ARM after publish but WITHOUT resolver → no-op (fail-closed)
        PubState s7 = new PubState();
        if (eselPublish(t1, c1, g1, s7) != 1) fails.add("G-A2 publish");
        eselArmNow(s7);
        if (s7.armed || s7.markers != 0) fails.add("G-A2 arm-no-resolver");

        // G-A3 resolver installed → arm flips + exactly one G2 marker
        s7.resolver = id -> id;
        eselArmNow(s7);
        if (!s7.armed) fails.add("G-A3 armed");
        if (s7.markers != 1) fails.add("G-A3 marker");
        // ARM strictly AFTER publish: the view latch must be the publish, not the arm
        if (typeSingle(s7, 33) == null || typeSingle(s7, 33) != 9001) fails.add("G-A3 single");

        // G-A4 latch persists: publish first, resolver later → still arms
        PubState s8 = new PubState();
        if (eselPublish(t1, c1, g1, s8) != 1) fails.add("G-A4 publish");
        eselArmNow(s8);
        if (s8.armed) fails.add("G-A4 early-arm");
        s8.resolver = id -> id;
        eselArmNow(s8);
        if (!s8.armed || s8.markers != 1) fails.add("G-A4 late-arm");

        // G-V1 typeSingle contract: count!=1 rows and unknown keys → null
        if (typeSingle(s7, 22) != null) fails.add("G-V1 multi-null");
        if (typeSingle(s7, 11) != null) fails.add("G-V1 zero-null");
        if (typeSingle(s7, 4242) != null) fails.add("G-V1 unknown-null");

        // G-V2 randomized probe-map validity: every key lands, no aliasing
        Random r = new Random(529249);
        for (int it = 0; it < 3000; it++) {
            int n = 1 + r.nextInt(64);
            java.util.TreeSet<Integer> ks = new java.util.TreeSet<>();
            while (ks.size() < n) ks.add(r.nextInt(1 << 20));
            int[] tt = new int[n];
            int[] cc = new int[n];
            long[] gg = new long[n];
            int i = 0;
            for (int k : ks) {
                tt[i] = k;
                cc[i] = r.nextInt(4); // 0..3 (1 requires single)
                gg[i] = cc[i] == 1 ? 1L + r.nextInt(1_000_000) : 0L;
                i++;
            }
            PubState sv = new PubState();
            if (eselPublish(tt, cc, gg, sv) != 1) { fails.add("G-V2 rc iter=" + it); break; }
            sv.resolver = id -> id; // id→entity identity for the single contract
            int j = 0;
            for (int k : ks) {
                if (typeCount(sv, k) != cc[j]) { fails.add("G-V2 count iter=" + it); }
                if (cc[j] == 1) {
                    Integer got = typeSingle(sv, k);
                    if (got == null || got != (int) gg[j]) fails.add("G-V2 single iter=" + it);
                } else if (typeSingle(sv, k) != null) {
                    fails.add("G-V2 rogue-single iter=" + it);
                }
                j++;
            }
            if (!fails.isEmpty()) break;
        }

        System.out.println("[esel-publish] probe verdict: " + (fails.isEmpty() ? "GREEN" : "RED"));
        for (String f : fails) System.out.println("[esel-publish] FAIL " + f);
        System.out.println("[esel-publish] order=publish(latch)->resolver->arm(marker); "
                + "caps=MAX_SLOTS " + MAX_SLOTS + "; invariant=count==1 <=> single!=0 (both sides rc=0)");
        if (!fails.isEmpty()) System.exit(1);
    }

    private EselPublishArmSelfTest() {}
}
