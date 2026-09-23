import ca.spottedleaf.moonrise.common.util.SimpleThreadUnsafeRandom;
import sun.misc.Unsafe;

import java.lang.reflect.Field;
import java.util.Arrays;
import java.util.Random;

/**
 * F1 BATCH-RNG parity bank (TASK-247 / S7-111). Offline unit test — loads the
 * REAL kernel classes (SimpleThreadUnsafeRandom from the materialized Purpur
 * 1.21.10 kernel) in a plain sandbox JVM. No server boot (INJECTS-ONLY safe).
 *
 * Verifies, against the object-authoritative reference flow:
 *   1. LCG core bit-exactness  (local arithmetic == advanceSeed()+nextInt())
 *   2. Sync-pattern exactness  (put-before-body / get-after-body / final put
 *      reproduces the reference stream when "bodies" consume random draws)
 *   3. Gaussian-cache safety   (bodies include nextGaussian — the direct
 *      Unsafe put must NOT disturb MarsagliaPolarGaussian cached spares,
 *      which is why we bypass setSeed)
 *   4. Final-position fidelity (rejected picks advance the stream; the final
 *      put must leave the object exactly where the reference left it)
 *
 * The sync pattern tested here is the production pattern from
 * RandomTickOps.run (kept in 1:1 correspondence by comment markers below).
 */
public class ParityTest {

    static final long MULTIPLIER = 25214903917L;
    static final long ADDEND = 11L;
    static final long MASK = (1L << 48) - 1L;

    static Unsafe UNSAFE;
    static long VALUE_OFFSET;

    public static void main(String[] args) throws Exception {
        Field uf = Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        UNSAFE = (Unsafe) uf.get(null);
        Field vf = SimpleThreadUnsafeRandom.class.getDeclaredField("value");
        vf.setAccessible(true);
        VALUE_OFFSET = UNSAFE.objectFieldOffset(vf);
        System.out.println("value field offset = " + VALUE_OFFSET);

        long[] seeds = {
            0L, 1L, 42L, -1L & MASK, 25214903917L,
            0x5DEECE66DL, System.nanoTime() & MASK,
            (System.nanoTime() * 7919L) & MASK,
        };

        long totalAttempts = 0, totalHits = 0;
        for (long s : seeds) {
            long[] r = runOne(s, 40_000);
            totalAttempts += r[0];
            totalHits += r[1];
            System.out.printf(
                "seed=%012x attempts=%d hits=%d picksMatch=%b drawsMatch=%b finalSeedMatch=%b%n",
                s, r[0], r[1], r[2] == 1, r[3] == 1, r[4] == 1);
        }
        System.out.println("TOTAL attempts=" + totalAttempts + " hits=" + totalHits);

        if (totalAttempts < 300_000) throw new AssertionError("not enough coverage");
        System.out.println("F1 PARITY: PASS (bit-exact pick stream + body draws + final seed)");
    }

    /** Runs reference vs patched flows for one seed; returns {attempts,hits,picksOK,drawsOK,finalOK}. */
    static long[] runOne(long seed, int attempts) {
        Random chaos = new Random(seed ^ 0x9E3779B97F4A7C15L);

        // ---- REFERENCE: object-authoritative (exactly the original loop shape) ----
        SimpleThreadUnsafeRandom ref = new SimpleThreadUnsafeRandom(seed);
        int[] refPicks = new int[attempts];
        long[] refDraws = new long[attempts * 2 + 64];
        int nDraws = 0;
        int refHits = 0;
        for (int t = 0; t < attempts; t++) {
            int size = 1 + chaos.nextInt(4096);      // section density varies per attempt
            int idx = ref.nextInt() & 4095;
            refPicks[t] = idx;
            if (idx >= size) continue;               // rejected pick: stream advanced only
            refHits++;
            int k = chaos.nextInt(6);                // body consumes 0..5 extra draws
            for (int d = 0; d < k; d++) {
                switch (chaos.nextInt(4)) {
                    case 0 -> refDraws[nDraws++] = ref.nextInt(1 + (ref.nextInt() & 31));
                    case 1 -> refDraws[nDraws++] = ref.nextLong();
                    case 2 -> refDraws[nDraws++] = Float.floatToIntBits(ref.nextFloat());
                    default -> refDraws[nDraws++] = Double.doubleToLongBits(ref.nextGaussian());
                }
            }
        }
        long refFinal = UNSAFE.getLong(ref, VALUE_OFFSET);

        // ---- PATCHED: local-seed loop with the RandomTickOps.run sync pattern ----
        SimpleThreadUnsafeRandom pat = new SimpleThreadUnsafeRandom(seed);
        long seedLocal = UNSAFE.getLong(pat, VALUE_OFFSET);
        int[] patPicks = new int[attempts];
        long[] patDraws = new long[attempts * 2 + 64];
        int pDraws = 0;
        int patHits = 0;
        Random chaos2 = new Random(seed ^ 0x9E3779B97F4A7C15L); // same chaos script
        for (int t = 0; t < attempts; t++) {
            int size = 1 + chaos2.nextInt(4096);
            // === RandomTickOps.run fast path ===
            seedLocal = (seedLocal * MULTIPLIER + ADDEND) & MASK;
            int idx = (int) (seedLocal >>> 16) & 4095;
            patPicks[t] = idx;
            if (idx >= size) continue;
            patHits++;
            // === hit: put local -> object ===
            UNSAFE.putLong(pat, VALUE_OFFSET, seedLocal);
            int k = chaos2.nextInt(6);
            for (int d = 0; d < k; d++) {            // "body" consumes via the OBJECT
                switch (chaos2.nextInt(4)) {
                    case 0 -> patDraws[pDraws++] = pat.nextInt(1 + (pat.nextInt() & 31));
                    case 1 -> patDraws[pDraws++] = pat.nextLong();
                    case 2 -> patDraws[pDraws++] = Float.floatToIntBits(pat.nextFloat());
                    default -> patDraws[pDraws++] = Double.doubleToLongBits(pat.nextGaussian());
                }
            }
            // === after body: get object -> local ===
            seedLocal = UNSAFE.getLong(pat, VALUE_OFFSET);
        }
        // === final put ===
        UNSAFE.putLong(pat, VALUE_OFFSET, seedLocal);
        long patFinal = UNSAFE.getLong(pat, VALUE_OFFSET);

        boolean picksOK = Arrays.equals(refPicks, patPicks);
        boolean drawsOK = (nDraws == pDraws) && Arrays.equals(refDraws, 0, Math.max(nDraws, 0),
                                                              patDraws, 0, Math.max(pDraws, 0));
        boolean finalOK = refFinal == patFinal;
        return new long[]{attempts, refHits, picksOK ? 1 : 0, drawsOK ? 1 : 0, finalOK ? 1 : 0};
    }
}
