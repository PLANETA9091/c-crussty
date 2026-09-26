package net.minecraft.server;

import java.util.Collection;
import java.util.Iterator;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.LongAdder;
import net.minecraft.commands.functions.CommandFunction;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.util.profiling.ProfilerFiller;

/**
 * C100 DATAPACK-STRESS CENSUS bridge (TASK-466-C100, mega-goal 19c
 * datapack-resilience; lever {@code cmp466_dpstress} — STRICT eq,
 * NOT-A-BENCH observation-only telemetry). Native counterpart:
 * src/dp_stress.rs; body swap via
 * classfile::patch_sfmanager_dpstress (redirect_method_body_to_static).
 *
 * ONE BODY (javap purpur-1.21.10 ground truth, round-396-a
 * patched-kernel.jar, 2025-12-11 build):
 *
 *   1. {@code private void executeTagFunctions(Collection, ResourceLocation)}
 *      — the single choke point of EVERY datapack function execution burst
 *      per tick: ServerFunctionManager.tick() calls it once per tick with the
 *      #minecraft:tick tag (and once on postReload with the #minecraft:load
 *      tag). Vanilla body: Profiler.get().push(Supplier) -> iterator loop of
 *      execute(fn, getGameLoopSender()) -> fresh Profiler.get().pop().
 *
 * The bridge mirrors the vanilla body BIT-FOR-BIT (same iteration order, same
 * public call surface: execute + getGameLoopSender are public, the profiler
 * is the vanilla singleton) and adds static LongAdder counters:
 *   - CNT_CALLS — executeTagFunctions invocations (per-tag bursts),
 *   - CNT_FNS   — tag-level functions executed (iterating the collection),
 *   - CNT_NS    — wall time of the bursts (nanoTime pair per call).
 * Telemetry lands on stderr every 1200 counted calls
 * ({@code [c466-dpstress] ...}) + one INIT line on the first call
 * (tag identity, tag size, library size) + a shutdown-hook FINAL line —
 * all captured by the bench server-stdout.log artifact. Overhead ~= 2
 * nanoTime + 3 LongAdder increments per call (1-2 calls/tick) ~= noise;
 * the verdict expectation for a census leg is norm ~= 0 against the
 * stress-world vanilla base (census carrier-only, no TPS claims).
 *
 * DOCUMENTED DELTA (the only one): vanilla pushes the profiler section with
 * a Supplier lambda (lazy toString); the bridge pushes with the eager String
 * overload — one small String build per call even when the profiler is a
 * no-op dummy. Census-only lever; the empty/foreign lever flag never defines
 * this class (vanilla bit-in-byte, dormant-invisible).
 *
 * NO nested classes (single .class blob — flat==nested gate), NO natives,
 * NO Unsafe: the vanilla surface used here (execute, getGameLoopSender,
 * getFunctionNames) is fully public. The shutdown hook is registered lazily
 * on the first census call, never in <clinit> (NCDFE canon).
 *
 * NOTHING LANDS until the rust side defines this class into the kernel
 * loader and retransforms ServerFunctionManager with
 * CRUSSTY_LEVER_FLAG=cmp466_dpstress (strict eq).
 */
public final class DpStressOps {

    private DpStressOps() {}

    // ------------------------------------------------------------- census

    static final LongAdder CNT_CALLS = new LongAdder(); // executeTagFunctions calls
    static final LongAdder CNT_FNS = new LongAdder();   // tag-level functions run
    static final LongAdder CNT_NS = new LongAdder();    // burst wall time, ns

    private static final long DUMP_MASK = 1200 - 1;     // ~1 line / 1200 calls
    private static final AtomicBoolean INIT = new AtomicBoolean(false);
    private static volatile boolean finalHookDone = false;
    private static volatile String initTag = "?";
    private static volatile long initTagFns = -1;
    private static volatile long initLibFns = -1;

    // --------------------------------------------------------------- hook

    public static void execTag(ServerFunctionManager mgr, Collection functions,
                               ResourceLocation tag) {
        ProfilerFiller profiler = Profiler.get();
        profiler.push(tag.toString()); // vanilla: push(Supplier); documented eager-String delta
        long t0 = System.nanoTime();
        long fns = 0;
        for (Iterator it = functions.iterator(); it.hasNext(); ) {
            CommandFunction fn = (CommandFunction) it.next();
            mgr.execute(fn, mgr.getGameLoopSender());
            fns++;
        }
        long dt = System.nanoTime() - t0;
        Profiler.get().pop();
        // census (post-pop: telemetry must never alter the vanilla window)
        if (INIT.compareAndSet(false, true)) {
            initSnapshot(mgr, tag, functions.size());
        }
        CNT_CALLS.increment();
        CNT_FNS.add(fns);
        CNT_NS.add(dt);
        maybeDump();
    }

    // ------------------------------------------------------------ helpers

    private static void initSnapshot(ServerFunctionManager mgr, ResourceLocation tag,
                                     int tagFns) {
        try {
            initTag = String.valueOf(tag);
            initTagFns = tagFns;
            long lib = 0;
            for (Iterator it = mgr.getFunctionNames().iterator(); it.hasNext(); ) {
                it.next();
                lib++;
            }
            initLibFns = lib;
            System.err.println("[c466-dpstress] INIT tag=" + initTag
                + " tag_fns=" + initTagFns + " library_fns=" + initLibFns);
            // final-line hook: graceful stop (SIGTERM) prints the terminal
            // census; SIGKILL runs still carry the volume-window lines above.
            Runtime.getRuntime().addShutdownHook(new Thread(() -> {
                if (finalHookDone) {
                    return;
                }
                finalHookDone = true;
                try {
                    System.err.println(dumpLine("[c466-dpstress]-FINAL"));
                } catch (Throwable ignored) {
                    // telemetry must never break shutdown
                }
            }, "crussty-dpstress-final"));
        } catch (Throwable ignored) {
            // telemetry must never break the tick
        }
    }

    private static void maybeDump() {
        long calls = CNT_CALLS.sum();
        if ((calls & DUMP_MASK) != 0L) {
            return;
        }
        try {
            System.err.println(dumpLine("[c466-dpstress]"));
        } catch (Throwable ignored) {
            // telemetry must never break the tick
        }
    }

    private static String dumpLine(String marker) {
        long calls = CNT_CALLS.sum();
        long fns = CNT_FNS.sum();
        long ns = CNT_NS.sum();
        return marker
            + " init_tag=" + initTag
            + " tag_fns=" + initTagFns
            + " library_fns=" + initLibFns
            + " calls=" + calls
            + " fns=" + fns
            + " ns=" + ns
            + (calls > 0 ? " fns_per_call=" + (fns / calls) : "")
            + (calls > 0 ? " ns_per_call=" + (ns / calls) : "");
    }

    /**
     * Structural oracle (chunk_sched canon): true only when the bridge class
     * links and the census counters round-trip. The rust activator refuses
     * to arm (no retransform) unless this returns true BEFORE the body swap
     * exists (probe-then-patch).
     */
    public static boolean selfTest() {
        try {
            CNT_CALLS.increment();
            boolean ok = CNT_CALLS.sum() >= 1L;
            CNT_CALLS.decrement();
            return ok && CNT_CALLS.sum() == 0L;
        } catch (Throwable t) {
            return false;
        }
    }

    /** Grep-able arm-state (roar-2 lesson: ARM={} silent dormancy forbidden). */
    public static String armState() {
        return "CENSUS_WIRED"; // rust flips the plane with the retransform
    }
}
