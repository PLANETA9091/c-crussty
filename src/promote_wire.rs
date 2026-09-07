//! TASK-53 — live-armed self-test for the promotion binding
//! (docs/PROVEN_WINS_SYNC.md §4.2, docs/KERNEL_POLICY.md §Lifecycle).
//!
//! When `CRUSSTY_KERNEL_PROMOTE` is armed, the registration chokepoint binds
//! each promotion pair's ORIGINAL bridge method (`old*ContextSummary`) to its
//! paired P500-WIN symbol (`new*ContextSummary`). This module then replays
//! deterministic FIXTURE vectors — captured offline from the OLD
//! implementations by `bench/p500/parity/FlatCacheParity.java` (3648-input
//! byte-exact parity gate, fixtures verified identical across JVM runs) —
//! through the REAL injected bridge AFTER the rebind. Because the offline
//! expectations come from the old implementations, a post-rebind match proves
//! the swap is semantics-transparent END-TO-END (live JVM, real closed .so,
//! re-bound surface). Each fixture is additionally called through the paired
//! win-name bridge method: both names must agree byte-exactly.
//!
//! Dormant (env unset) this module does nothing at all — zero calls, zero
//! logs, mirroring the other armed surfaces.

use crate::kernel_policy;
use jvmti_bindings::prelude::*;

const BRIDGE: &str = "PaperNativeNoiseChunkFlatCacheContext";
const SIG: &str = "(II[J)I";
const ARR_LEN: usize = 8;

/// One deterministic fixture: inputs + the OLD-implementation expectation
/// (return value + full dst contents) captured by the offline parity driver.
struct Fixture {
    /// true = *TrueContextSummary* pair, false = *FalseContextSummary* pair.
    true_pair: bool,
    a0: i32,
    a1: i32,
    src: [i64; ARR_LEN],
    res: i32,
    dst: [i64; ARR_LEN],
}

/// Verbatim from bench/p500/parity/results/FLATCACHE_PARITY_RAW.tsv
/// (FIXTURE rows; cross-JVM determinism verified by a double run).
const FIXTURES: &[Fixture] = &[
    Fixture { true_pair: true, a0: 16, a1: 31, src: [7, 31, 3, 15, 63, 1, 9, 21], res: 3, dst: [16, 16, 4638144666238189568, 15, 63, 1, 9, 21] },
    Fixture { true_pair: true, a0: 0, a1: 7, src: [1, 2, 3, 4, 5, 6, 7, 8], res: 3, dst: [0, 0, 0, 4, 5, 6, 7, 8] },
    Fixture { true_pair: true, a0: 64, a1: 63, src: [-5, 12, 0, -1, 4096, -4096, 77, 123], res: 3, dst: [64, 64, 4656581277212737536, -1, 4096, -4096, 77, 123] },
    Fixture { true_pair: true, a0: 256, a1: 1, src: [1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 3, dst: [256, 256, 4674701228838486016, 42, 42, 0, 0, 1] },
    Fixture { true_pair: false, a0: 16, a1: 31, src: [7, 31, 3, 15, 63, 1, 9, 21], res: 3, dst: [16, 16, 0, 15, 63, 1, 9, 21] },
    Fixture { true_pair: false, a0: 0, a1: 7, src: [1, 2, 3, 4, 5, 6, 7, 8], res: 3, dst: [0, 0, 0, 4, 5, 6, 7, 8] },
    Fixture { true_pair: false, a0: 64, a1: 63, src: [-5, 12, 0, -1, 4096, -4096, 77, 123], res: 3, dst: [64, 64, 0, -1, 4096, -4096, 77, 123] },
    Fixture { true_pair: false, a0: 256, a1: 1, src: [1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 3, dst: [256, 256, 0, 42, 42, 0, 0, 1] },
];

/// Run the self-test if (and only if) the promotion binding is armed.
/// Called from lib.rs after the surface injection + live proof.
pub fn selftest_if_armed(env: &JniEnv) {
    if !kernel_policy::promotion_armed() {
        return;
    }
    eprintln!(
        "[crussty-plugin] kernel_promote: armed ({} pairs) — running live self-test through the real bridge",
        kernel_policy::PROMOTE_PAIRS.len()
    );
    match run(env) {
        Ok(bridge_checks) => eprintln!(
            "[crussty-plugin] kernel_promote: SELF-TEST PASS (fixtures {}/{} byte-exact vs offline old-impl expectations, bridge parity {}/{} from-name == to-name post-rebind)",
            FIXTURES.len(),
            FIXTURES.len(),
            bridge_checks,
            FIXTURES.len()
        ),
        Err(e) => eprintln!("[crussty-plugin] kernel_promote: SELF-TEST FAIL — {e}"),
    }
}

fn run(env: &JniEnv) -> Result<usize, String> {
    let Some(cls) = env.find_class(BRIDGE) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        return Err(format!("find_class({BRIDGE}) failed (surface not injected?)"));
    };

    // Resolve all four bridge methods up front (fail fast, log once).
    let mut from_mids = Vec::with_capacity(2);
    let mut to_mids = Vec::with_capacity(2);
    for (true_pair, from_name, to_name) in [
        (true, "oldTrueContextSummary", "newTrueContextSummary"),
        (false, "oldFalseContextSummary", "newFalseContextSummary"),
    ] {
        let Some(from_mid) = env.get_static_method_id(cls, from_name, SIG) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            env.delete_local_ref(cls);
            return Err(format!("{from_name} unresolved"));
        };
        let Some(to_mid) = env.get_static_method_id(cls, to_name, SIG) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            env.delete_local_ref(cls);
            return Err(format!("{to_name} unresolved"));
        };
        from_mids.push((true_pair, from_mid));
        to_mids.push((true_pair, to_mid));
    }

    let call = |mid: jni::jmethodID, arr: jni::jlongArray, f: &Fixture| -> Result<i32, String> {
        let r = env.call_static_int_method(
            cls,
            mid,
            &[
                jni::jvalue { i: f.a0 },
                jni::jvalue { i: f.a1 },
                jni::jvalue { l: arr },
            ],
        );
        let exc = cplug_sdk::jni_util::clear_exception(env);
        if exc {
            return Err(format!("bridge call threw (a0={} a1={})", f.a0, f.a1));
        }
        Ok(r)
    };

    let mut bridge_checks = 0usize;
    let mut failures = 0usize;

    for (idx, f) in FIXTURES.iter().enumerate() {
        let from_mid = from_mids
            .iter()
            .find(|(tp, _)| *tp == f.true_pair)
            .map(|(_, m)| *m)
            .ok_or("from method id missing")?;
        let to_mid = to_mids
            .iter()
            .find(|(tp, _)| *tp == f.true_pair)
            .map(|(_, m)| *m)
            .ok_or("to method id missing")?;

        // Arm 1: the ORIGINAL (re-bound) bridge name must reproduce the
        // offline OLD-implementation expectation — the semantics-transparent
        // swap proof, end to end.
        let Some(arr) = env.new_long_array(ARR_LEN as jni::jsize) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            env.delete_local_ref(cls);
            return Err("new_long_array failed".into());
        };
        env.set_long_array_region(arr, 0, ARR_LEN as jni::jsize, &f.src);
        let r_from = match call(from_mid, arr, f) {
            Ok(r) => r,
            Err(e) => {
                env.delete_local_ref(arr);
                failures += 1;
                eprintln!("[crussty-plugin] kernel_promote: fixture #{idx}: {e}");
                continue;
            }
        };
        let mut got = [0i64; ARR_LEN];
        env.get_long_array_region(arr, 0, ARR_LEN as jni::jsize, &mut got);
        env.delete_local_ref(arr);

        if r_from != f.res || got != f.dst {
            failures += 1;
            eprintln!(
                "[crussty-plugin] kernel_promote: fixture #{idx} MISMATCH (via re-bound name): res {r_from} (want {}), dst {got:?} (want {:?})",
                f.res, f.dst
            );
            continue;
        }

        // Arm 2: the win-name bridge method must agree byte-exactly with the
        // re-bound original name (both names resolve + execute consistently).
        let Some(arr2) = env.new_long_array(ARR_LEN as jni::jsize) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            env.delete_local_ref(cls);
            return Err("new_long_array failed".into());
        };
        env.set_long_array_region(arr2, 0, ARR_LEN as jni::jsize, &f.src);
        let r_to = match call(to_mid, arr2, f) {
            Ok(r) => r,
            Err(e) => {
                env.delete_local_ref(arr2);
                failures += 1;
                eprintln!("[crussty-plugin] kernel_promote: fixture #{idx}: {e}");
                continue;
            }
        };
        let mut got2 = [0i64; ARR_LEN];
        env.get_long_array_region(arr2, 0, ARR_LEN as jni::jsize, &mut got2);
        env.delete_local_ref(arr2);

        if r_to != r_from || got2 != got {
            failures += 1;
            eprintln!(
                "[crussty-plugin] kernel_promote: fixture #{idx} BRIDGE-PARITY FAIL: from-res {r_from} vs to-res {r_to}"
            );
            continue;
        }
        bridge_checks += 1;
    }

    env.delete_local_ref(cls);
    if failures > 0 {
        return Err(format!("{failures} fixture failure(s)"));
    }
    Ok(bridge_checks)
}
