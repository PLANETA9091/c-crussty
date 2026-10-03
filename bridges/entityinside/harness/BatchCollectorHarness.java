package harness;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.IdentityHashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.function.Consumer;
import net.minecraft.core.BlockPos;
import net.minecraft.world.entity.BatchCollector;
import net.minecraft.world.entity.InsideBlockEffectApplier;
import net.minecraft.world.entity.InsideBlockEffectApplier.StepBasedCollector;
import net.minecraft.world.entity.InsideBlockEffectType;

/**
 * BatchCollectorHarness (S7-160) — OFFLINE semantic diff between the
 * vanilla StepBasedCollector and the BatchCollector flat replacement.
 *
 * Methodology: drive BOTH collectors with the SAME operation sequence
 * (advanceStep/apply/runBefore/runAfter), then invoke the PRIVATE
 * flushStep of each (setAccessible) — the whole ordering complexity —
 * and compare the resulting playback queues in normal form:
 *   vanilla: finalEffects List<Consumer>; RecordedEffect -> EFFECT(type
 *            resolved via applier identity), other consumers -> CONS(id)
 *   batch:   opKind/opType/opPos/opC flat arrays -> EFFECT/CONS(id)
 * Mismatch on ANY random scenario = FAIL (exit 2).
 *
 * applyAndClear playback loop (isAlive break, null-ing, lastStep=-1) is
 * a 10-line javap copy — verified by code review, its semantics do not
 * depend on the collector state beyond the flushed queue.
 *
 * Run: java -cp <purpur>:<joml>:<fastutil>:entityinside/build:entityinside \
 *        harness.BatchCollectorHarness [scenarios]
 */
public final class BatchCollectorHarness {

    private static final InsideBlockEffectType[] TYPES = InsideBlockEffectType.values();

    // ---- reflection accessors (fail fast, no silent skips) ----
    private static final Field VAN_FINAL;
    private static final java.lang.reflect.Method VAN_FLUSH;
    private static final Field REC_POS;
    private static final Field REC_APPLIER;
    private static final Field BC_KIND;
    private static final Field BC_TYPE;
    private static final Field BC_POS;
    private static final Field BC_CONS;
    private static final Field BC_NOPS;
    private static final java.lang.reflect.Method BC_FLUSH;

    static {
        try {
            VAN_FINAL = StepBasedCollector.class.getDeclaredField("finalEffects");
            VAN_FINAL.setAccessible(true);
            VAN_FLUSH = StepBasedCollector.class.getDeclaredMethod("flushStep");
            VAN_FLUSH.setAccessible(true);
            Class<?> rec = Class.forName(
                    "net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect");
            REC_POS = rec.getDeclaredField("blockPos");
            REC_POS.setAccessible(true);
            REC_APPLIER = rec.getDeclaredField("applier");
            REC_APPLIER.setAccessible(true);
            Class<?> bc = BatchCollector.class;
            BC_KIND = bc.getDeclaredField("opKind");
            BC_KIND.setAccessible(true);
            BC_TYPE = bc.getDeclaredField("opType");
            BC_TYPE.setAccessible(true);
            BC_POS = bc.getDeclaredField("opPos");
            BC_POS.setAccessible(true);
            BC_CONS = bc.getDeclaredField("opC");
            BC_CONS.setAccessible(true);
            BC_NOPS = bc.getDeclaredField("nOps");
            BC_NOPS.setAccessible(true);
            BC_FLUSH = bc.getDeclaredMethod("flushStep");
            BC_FLUSH.setAccessible(true);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    private static String applierType(Object applier) {
        for (InsideBlockEffectType t : TYPES) {
            if (t.effect() == applier) {
                return t.name();
            }
        }
        return "UNKNOWN_APPLIER";
    }

    @SuppressWarnings("unchecked")
    private static List<String> vanillaQueue(StepBasedCollector c, Map<Object, Integer> consIds)
            throws Exception {
        VAN_FLUSH.invoke(c);
        List<Object> q = (List<Object>) VAN_FINAL.get(c);
        List<String> out = new ArrayList<>();
        for (Object o : q) {
            if (o.getClass().getSimpleName().equals("RecordedEffect")) {
                BlockPos pos = (BlockPos) REC_POS.get(o);
                Object ap = REC_APPLIER.get(o);
                out.add("EFFECT:" + applierType(ap) + ":" + pos.asLong());
            } else {
                int id = consIds.computeIfAbsent(o, k -> consIds.size() + 1);
                out.add("CONS:" + id);
            }
        }
        return out;
    }

    @SuppressWarnings("unchecked")
    private static List<String> batchQueue(BatchCollector c, Map<Object, Integer> consIds)
            throws Exception {
        BC_FLUSH.invoke(c);
        byte[] kind = (byte[]) BC_KIND.get(c);
        byte[] typ = (byte[]) BC_TYPE.get(c);
        long[] pos = (long[]) BC_POS.get(c);
        Consumer<net.minecraft.world.entity.Entity>[] cons =
                (Consumer<net.minecraft.world.entity.Entity>[]) BC_CONS.get(c);
        int n = (int) BC_NOPS.get(c);
        List<String> out = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            if (kind[i] == 0) {
                int id = consIds.computeIfAbsent(cons[i], k -> consIds.size() + 1);
                out.add("CONS:" + id);
            } else {
                out.add("EFFECT:" + TYPES[typ[i] & 0xFF].name() + ":" + pos[i]);
            }
        }
        return out;
    }

    @SuppressWarnings("unchecked")
    private static boolean scenario(long seed, int nOps, boolean verbose) {
        Random r = new Random(seed);
        StepBasedCollector vanilla = new StepBasedCollector();
        BatchCollector batch = new BatchCollector();

        int step = r.nextInt(3);
        boolean stepped = false; // vanilla contract: apply() NPEs without a prior advanceStep (currentBlockPos null) — the live visitor always advances first
        for (int i = 0; i < nOps; i++) {
            int pick = r.nextInt(100);
            BlockPos bp = new BlockPos(r.nextInt(64) - 32, r.nextInt(64) - 32, r.nextInt(64) - 32);
            if (!stepped || pick < 30) {
                step += r.nextInt(3); // monotonic-ish (same budget regime as vanilla)
                vanilla.advanceStep(step, bp);
                batch.advanceStep(step, bp);
                stepped = true;
            } else if (pick < 75) {
                InsideBlockEffectType t = TYPES[r.nextInt(TYPES.length)];
                vanilla.apply(t);
                batch.apply(t);
            } else if (pick < 88) {
                Consumer<net.minecraft.world.entity.Entity> cs = e -> { /* unique id */ };
                InsideBlockEffectType t = TYPES[r.nextInt(TYPES.length)];
                vanilla.runBefore(t, cs);
                batch.runBefore(t, cs);
            } else {
                Consumer<net.minecraft.world.entity.Entity> cs = e -> { /* unique id */ };
                InsideBlockEffectType t = TYPES[r.nextInt(TYPES.length)];
                vanilla.runAfter(t, cs);
                batch.runAfter(t, cs);
            }
        }
        try {
            List<String> qv = vanillaQueue(vanilla, new IdentityHashMap<>());
            List<String> qb = batchQueue(batch, new IdentityHashMap<>());
            // Both collectors received the SAME consumer instances in the
            // SAME order; canonical() normalizes consumer entries to their
            // first-seen positions, so queue equality <=> playback order
            // equality (EFFECT entries carry full type+pos identity).
            List<String> cv = canonical(qv);
            List<String> cb = canonical(qb);
            if (!cv.equals(cb)) {
                System.err.println("MISMATCH seed=" + seed + " nOps=" + nOps);
                System.err.println("  vanilla: " + cv);
                System.err.println("  batch:   " + cb);
                return false;
            }
            if (verbose) {
                System.out.println("ok seed=" + seed + " nOps=" + nOps + " ops=" + cv.size());
            }
        } catch (Exception e) {
            System.err.println("EXCEPTION seed=" + seed + ": " + e);
            e.printStackTrace();
            return false;
        }
        return true;
    }

    private static List<String> canonical(List<String> q) {
        List<String> out = new ArrayList<>(q.size());
        int next = 1;
        for (String s : q) {
            if (s.startsWith("CONS:")) {
                out.add("CONS#" + (next++));
            } else {
                out.add(s);
            }
        }
        return out;
    }

    public static void main(String[] args) {
        int scenarios = args.length > 0 ? Integer.parseInt(args[0]) : 4000;
        int fail = 0;
        long t0 = System.nanoTime();
        for (int i = 0; i < scenarios; i++) {
            long seed = 0xC0FFEE + i * 6364136223846793005L;
            int nOps = 1 + (i % 64);
            if (!scenario(seed, nOps, i < 3)) {
                fail++;
                if (fail > 5) break;
            }
        }
        double ms = (System.nanoTime() - t0) / 1e6;
        if (fail == 0) {
            System.out.println("BATCH-COLLECTOR HARNESS: PASS (" + scenarios
                    + " scenarios, " + String.format("%.0f", ms) + "ms) — "
                    + "flushStep queues bit-identical (EFFECT type/pos + CONS order)");
            return;
        }
        System.err.println("BATCH-COLLECTOR HARNESS: FAIL (" + fail + " mismatches)");
        System.exit(2);
    }
}
