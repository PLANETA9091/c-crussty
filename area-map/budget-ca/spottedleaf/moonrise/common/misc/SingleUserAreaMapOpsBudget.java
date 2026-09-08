package ca.spottedleaf.moonrise.common.misc;

/**
 * BUDGETED-scratch variant of the area-map apply bridge (TASK-64 variant C,
 * {@code docs/AREAMAP_DENSE_APPLY_DESIGN.md} §11).
 *
 * <h2>Why this file exists</h2>
 *
 * The kernel's byte hook rewrites {@code SingleUserAreaMap.update()} into
 * {@code invokestatic SingleUserAreaMapOps.run(...)}, so the runtime class
 * MUST be named {@code SingleUserAreaMapOps}. This source therefore declares
 * a package-private class of that name in a DIFFERENTLY named file, so both
 * scratch policies compile side-by-side without javac duplicate-class
 * conflicts:
 *
 * <ul>
 *   <li>{@code area-map/ca/.../SingleUserAreaMapOps.java} — legacy policy
 *       (grow-only ThreadLocal scratch up to the full worst-case cap
 *       {@code (2·oldD+1)² + (2·newD+1)²}); compiled by
 *       {@code scripts/build_area_map.sh} into {@code area-map/build/}.</li>
 *   <li>this file — budgeted policy; compiled by
 *       {@code scripts/build_area_map_budget.sh} into
 *       {@code area-map/build-budget/}.</li>
 * </ul>
 *
 * The plugin's {@code CRUSSTY_AREAMAP_BUDGET} gate (default off) picks which
 * compiled set is {@code define_class}'d into the map loader at activation
 * time. Gate off = the legacy bytes are defined verbatim; there is no
 * runtime flag and no behavioral delta on the default path.
 *
 * <h2>Budgeted policy (measured basis: STEP-0 probe, design §11.1)</h2>
 *
 * The closed {@code nativeUpdateOpsBatch} validates {@code len >= n} (not
 * {@code len >= cap}) and reports the exact required size as {@code -n0}
 * when the offered buffers are too short; the reject is side-effect-free
 * (sentinel-verified {@code writes_past_n=0}). That license lets the bridge
 * offer a budget-sized scratch instead of the full cap-sized one:
 *
 * <ul>
 *   <li>initial / floor capacity: {@value #INITIAL_CAP} entries (same as
 *       legacy: covers d=8 for both squares, past any prod first use);</li>
 *   <li>{@code n < 0} → exactly one retry with {@code len = -n} (the native's
 *       built-in size oracle); a second failure, a non-positive or
 *       above-cap demand is treated as a native contract violation and
 *       fails safe (no callbacks, same observable shape as the legacy
 *       {@code n < 0} error return);</li>
 *   <li>after every productive call the budget grows monotonically to
 *       {@code min(cap, 2·n)} when larger than the current arrays — the
 *       §11.2 falsifier-1 mitigation (a sawtooth workload cannot retry-storm:
 *       each shape increase costs at most one side-effect-free reject) and
 *       the falsifier-3 mitigation (realloc happens only here and in the
 *       retry, never on a steady-state call); arrays never shrink, so the
 *       footprint is bounded by {@code min(cap, 2·max-n-seen)} — strictly
 *       below the legacy {@code >= cap} at all times;</li>
 *   <li>hardening (H-12 follow-up): a returned {@code n} beyond the offered
 *       array length fails safe (no callbacks) instead of indexing out of
 *       bounds.</li>
 * </ul>
 *
 * Semantics are untouched: the native enumerates exactly the same ops it
 * always did; buffer length is not semantics (TASK-30 oracle re-run on this
 * scratch policy is the parity gate — {@code bench/areamap/run_budget_ab.sh}).
 *
 * <p>Compile with {@code --release 8} (major 52) against RuntimeStubs.java so
 * the class loads from the map loader via DefineClass. Field names
 * {@code ops}/{@code keys} in {@link Scratch} and the {@code SCRATCH}
 * ThreadLocal are load-bearing beyond this class: the TASK-30 oracle probes
 * the offered capacity through them by reflection.</p>
 */
final class SingleUserAreaMapOps {
    private SingleUserAreaMapOps() {}

    /** Scratch floor and initial capacity: covers d=8 for BOTH squares
     *  (2*17^2 = 578), i.e. well past any real first use; never shrinks. */
    private static final int INITIAL_CAP = 2 * (2 * 8 + 1) * (2 * 8 + 1);

    private static final class Scratch {
        // Field initializers are load-bearing (see the legacy class): run()
        // reads s.ops.length directly, and a null Scratch.ops would NPE on
        // the first patched update() of every thread. The build script's
        // newarray guard enforces this shape.
        byte[] ops = new byte[INITIAL_CAP];
        long[] keys = new long[INITIAL_CAP];
    }

    private static final ThreadLocal<Scratch> SCRATCH = new ThreadLocal<Scratch>() {
        @Override
        protected Scratch initialValue() {
            return new Scratch();
        }
    };

    /** Upper bound on the number of difference ops for rect sizes oldD/newD. */
    static int maxOps(int oldD, int newD) {
        long oldSide = 2L * oldD + 1L;
        long newSide = 2L * newD + 1L;
        long cap = oldSide * oldSide + newSide * newSide;
        return cap > Integer.MAX_VALUE ? Integer.MAX_VALUE : (int) cap;
    }

    /**
     * Replace {@code update}'s enumeration: emit the difference natively and
     * apply callbacks from the produced op list. Signature must match the
     * invokestatic emitted by the SingleUserAreaMap.update byte hook (it is
     * byte-identical to the legacy bridge's — same descriptor, same guards).
     */
    static void run(
        SingleUserAreaMap map, int fromX, int fromZ, int oldD,
        int toX, int toZ, int newD, Object param
    ) {
        if (fromX == Integer.MIN_VALUE) {
            return; // never initialized: native enumerates nothing, no callbacks
        }
        // Same-state fast path: identical squares -> the difference is empty
        // by definition. Skipping it saves the full JNI transition + buffer
        // touch on the most frequent update shape. Field writes are NOT
        // skipped: the patched update() body already wrote toX/toZ/newD
        // before invoking run().
        if (fromX == toX && fromZ == toZ && oldD == newD) {
            return;
        }
        final Scratch s = SCRATCH.get();
        // cap is the true worst-case diff bound; used ONLY as the retry
        // sanity ceiling and the growth clamp, never as an allocation target
        // (that is the whole point of the budgeted policy).
        final int cap = maxOps(oldD, newD);
        byte[] ops = s.ops;
        long[] keys = s.keys;
        int n = PaperNativeAreaMap.nativeUpdateOpsBatch(fromX, fromZ, oldD, toX, toZ, newD, ops, keys);
        if (n < 0) {
            // Budgeted policy: the closed native reports the exact required
            // size as -n0 (STEP-0 probe: reject is side-effect-free). One
            // retry with a right-sized buffer. -Integer.MIN_VALUE overflows
            // back to itself (still negative) and is caught by req <= 0.
            final int req = -n;
            if (req <= 0 || req > cap) {
                return; // native contract violation: fail safe, emit nothing
            }
            if (s.ops.length < req) {
                s.ops = new byte[req];
                s.keys = new long[req];
            }
            ops = s.ops;
            keys = s.keys;
            n = PaperNativeAreaMap.nativeUpdateOpsBatch(fromX, fromZ, oldD, toX, toZ, newD, ops, keys);
            if (n < 0) {
                return; // retry failed: native is buggy/hostile — fail safe
            }
        }
        // H-12 follow-up: never trust the closed native's count past the
        // offered capacity — a hostile/buggy n would otherwise OOB-index
        // ops/keys inside the patched kernel hot path. Fail safe instead.
        if (n > ops.length) {
            return;
        }
        for (int i = 0; i < n; i++) {
            long key = keys[i];
            int x = (int) key;          // chunk_as_long: x in the low 32 bits
            int z = (int) (key >>> 32); // z in the high 32 bits
            if (ops[i] == 0) {
                map.addCallback(param, x, z);    // AreaOp::Add
            } else {
                map.removeCallback(param, x, z); // AreaOp::Remove
            }
        }
        // Post-call adaptive growth (data already consumed): monotone,
        // doubling, clamped by the true bound. Never shrinks; realloc only
        // here or in the retry — a steady-state call does not allocate
        // (design §11.2 falsifier 3), and each workload-shape increase costs
        // at most one side-effect-free reject before the budget adapts
        // (falsifier 1).
        long want = 2L * n;
        if (want > cap) {
            want = cap;
        }
        if (want > s.ops.length) {
            s.ops = new byte[(int) want];
            s.keys = new long[(int) want];
        }
    }
}
