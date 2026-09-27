import java.util.*;

public class QueueBench {
    // Mimic vanilla ExecutionContext.runCommandQueue loop (javap: Deque<CommandQueueEntry> + record fields + EntryAction.execute):
    // ArrayDeque pop + record field reads + virtual dispatch per command entry.
    interface EntryAction { void execute(long ctx, Frame f); }
    record Frame(int depth) {}
    record CmdEntry(Frame frame, EntryAction action) {}

    static class Plain implements EntryAction { public void execute(long ctx, Frame f) { BLACKHOLE += f.depth(); } }
    static class Plain2 implements EntryAction { public void execute(long ctx, Frame f) { BLACKHOLE += f.depth() + 1; } }
    static class Plain3 implements EntryAction { public void execute(long ctx, Frame f) { BLACKHOLE += f.depth() + 2; } }
    static long BLACKHOLE;

    static final EntryAction[] ACTIONS = { new Plain(), new Plain2(), new Plain3() }; // megamorphic call site

    public static void main(String[] a) {
        int N = 807; // DnT commands per tick (upper bound full sweep)
        int REPS = 200_000;
        run("warmup", N, 20_000, true);
        for (int r = 0; r < 5; r++) run("mega-run" + r, N, REPS, false);
        EntryAction only = new Plain();
        for (int r = 0; r < 5; r++) runMono("mono-run" + r, N, REPS, false, only);
        System.out.println("BH " + BLACKHOLE);
    }

    static void run(String tag, int n, int reps, boolean quiet) {
        ArrayDeque<CmdEntry> q = new ArrayDeque<>(256);
        long t0 = System.nanoTime();
        for (int r = 0; r < reps; r++) {
            for (int i = 0; i < n; i++) q.addLast(new CmdEntry(new Frame(i & 7), ACTIONS[i % 3]));
            CmdEntry e;
            while ((e = q.pollFirst()) != null) { e.action.execute(0L, e.frame); }
        }
        long dt = System.nanoTime() - t0;
        double per = (double) dt / ((long) reps * n);
        if (!quiet) System.out.printf("%s: %.1f ns/entry (megamorphic, alloc-per-entry)%n", tag, per);
        BLACKHOLE += (long) per;
    }

    static void runMono(String tag, int n, int reps, boolean quiet, EntryAction only) {
        ArrayDeque<CmdEntry> q = new ArrayDeque<>(256);
        long t0 = System.nanoTime();
        for (int r = 0; r < reps; r++) {
            for (int i = 0; i < n; i++) q.addLast(new CmdEntry(new Frame(i & 7), only));
            CmdEntry e;
            while ((e = q.pollFirst()) != null) { e.action.execute(0L, e.frame); }
        }
        long dt = System.nanoTime() - t0;
        double per = (double) dt / ((long) reps * n);
        if (!quiet) System.out.printf("%s: %.1f ns/entry (monomorphic, alloc-per-entry)%n", tag, per);
        BLACKHOLE += (long) per;
    }
}
