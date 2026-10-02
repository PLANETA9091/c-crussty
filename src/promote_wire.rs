//! TASK-53/54 — live-armed self-test for the promotion binding
//! (docs/PROVEN_WINS_SYNC.md §4.2, docs/KERNEL_POLICY.md §Lifecycle).
//!
//! When `CRUSSTY_KERNEL_PROMOTE` is armed, the registration chokepoint binds
//! each promotion pair's ORIGINAL bridge method (`from_kernel`) to its paired
//! P500-WIN symbol (`to_kernel`). This module then replays deterministic
//! FIXTURE vectors — captured offline from the OLD implementations by
//! `bench/p500/parity/FlatCacheParity.java` (TASK-53) and
//! `bench/p500/parity/WinPairParity.java` (TASK-54) after byte-exact
//! old≡new parity gates (3648 inputs/pair wave 1; 1600/3036/3072 wave 2),
//! cross-JVM determinism verified by double runs — through the REAL injected
//! bridge AFTER the rebind. Because the offline expectations come from the
//! OLD implementations, a post-rebind match proves the swap is
//! semantics-transparent END-TO-END (live JVM, real closed .so, re-bound
//! surface). Each fixture is additionally called through the paired win-name
//! bridge method: both names must agree byte-exactly.
//!
//! Argument shapes across the five promoted pairs (sig resolved from
//! `jni_table` at runtime, drift-guarded to be identical on both sides):
//!   `(II[J)I`  FlatCacheContext pair (ints[1] + long[])
//!   `(IIII[J)I` NoiseInterpolatorSlice pair (ints[4] + long[])
//!   `(I[J)I`   PalettedReencodeScratch pair (ints[1] + long[])
//!   `([BI[J)I` ImprovedNoiseInline pair (byte[] + ints[1] + long[])
//!
//! Dormant (env unset) this module does nothing at all — zero calls, zero
//! logs, mirroring the other armed surfaces.

use crate::kernel_policy;
use jvmti_bindings::prelude::*;

/// Argument shape of one fixture, derived from which slots are populated.
struct Fixture {
    /// Promotion pair selector: (bridge class, from_kernel).
    pair: (&'static str, &'static str),
    /// Leading int args (only the first `n_ints` are passed).
    n_ints: usize,
    ints: [i32; 4],
    /// Optional leading byte[] argument (`with_bytes = true`).
    with_bytes: bool,
    bytes: &'static [i8],
    /// long[] input (also the dst array the kernel writes into).
    longs: &'static [i64],
    /// OLD-implementation expectation: return value + full dst contents.
    res: i32,
    dst: &'static [i64],
}

/// Verbatim from bench/p500/parity/results/FLATCACHE_PARITY_RAW.tsv (TASK-53,
/// FIXTURE rows) and bench/p500/parity/results/WINPAIR_PARITY_RAW.tsv
/// (TASK-54, FIXTURE2 rows); cross-JVM determinism verified by double runs.
const FIXTURES: &[Fixture] = &[
    // --- TASK-53: PaperNativeNoiseChunkFlatCacheContext (II[J)I ------------
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldTrueContextSummary"), n_ints: 2, ints: [16, 31, 0, 0], with_bytes: false, bytes: &[], longs: &[7, 31, 3, 15, 63, 1, 9, 21], res: 3, dst: &[16, 16, 4638144666238189568, 15, 63, 1, 9, 21] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldTrueContextSummary"), n_ints: 2, ints: [0, 7, 0, 0], with_bytes: false, bytes: &[], longs: &[1, 2, 3, 4, 5, 6, 7, 8], res: 3, dst: &[0, 0, 0, 4, 5, 6, 7, 8] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldTrueContextSummary"), n_ints: 2, ints: [64, 63, 0, 0], with_bytes: false, bytes: &[], longs: &[-5, 12, 0, -1, 4096, -4096, 77, 123], res: 3, dst: &[64, 64, 4656581277212737536, -1, 4096, -4096, 77, 123] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldTrueContextSummary"), n_ints: 2, ints: [256, 1, 0, 0], with_bytes: false, bytes: &[], longs: &[1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 3, dst: &[256, 256, 4674701228838486016, 42, 42, 0, 0, 1] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldFalseContextSummary"), n_ints: 2, ints: [16, 31, 0, 0], with_bytes: false, bytes: &[], longs: &[7, 31, 3, 15, 63, 1, 9, 21], res: 3, dst: &[16, 16, 0, 15, 63, 1, 9, 21] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldFalseContextSummary"), n_ints: 2, ints: [0, 7, 0, 0], with_bytes: false, bytes: &[], longs: &[1, 2, 3, 4, 5, 6, 7, 8], res: 3, dst: &[0, 0, 0, 4, 5, 6, 7, 8] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldFalseContextSummary"), n_ints: 2, ints: [64, 63, 0, 0], with_bytes: false, bytes: &[], longs: &[-5, 12, 0, -1, 4096, -4096, 77, 123], res: 3, dst: &[64, 64, 0, -1, 4096, -4096, 77, 123] },
    Fixture { pair: ("PaperNativeNoiseChunkFlatCacheContext", "oldFalseContextSummary"), n_ints: 2, ints: [256, 1, 0, 0], with_bytes: false, bytes: &[], longs: &[1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 3, dst: &[256, 256, 0, 42, 42, 0, 0, 1] },
    // --- TASK-54: PaperNativeNoiseInterpolatorSlice (IIII[J)I --------------
    Fixture { pair: ("PaperNativeNoiseInterpolatorSlice", "oldJaggedSummary"), n_ints: 4, ints: [16, 31, 3, 15], with_bytes: false, bytes: &[], longs: &[7, 31, 3, 15, 63, 1, 9, 21], res: 3, dst: &[16, 22320, 4833452728359734272, 15, 63, 1, 9, 21] },
    Fixture { pair: ("PaperNativeNoiseInterpolatorSlice", "oldJaggedSummary"), n_ints: 4, ints: [0, 7, 3, 15], with_bytes: false, bytes: &[], longs: &[1, 2, 3, 4, 5, 6, 7, 8], res: 3, dst: &[0, 0, 0, 4, 5, 6, 7, 8] },
    Fixture { pair: ("PaperNativeNoiseInterpolatorSlice", "oldJaggedSummary"), n_ints: 4, ints: [64, 63, 31, 1], with_bytes: false, bytes: &[], longs: &[-5, 12, 0, -1, 4096, -4096, 77, 123], res: 3, dst: &[64, 124992, 4849462149413434624, -1, 4096, -4096, 77, 123] },
    Fixture { pair: ("PaperNativeNoiseInterpolatorSlice", "oldJaggedSummary"), n_ints: 4, ints: [256, 1, 15, 63], with_bytes: false, bytes: &[], longs: &[1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 3, dst: &[256, 241920, 4764322032387096576, 42, 42, 0, 0, 1] },
    // --- TASK-54: PaperNativeImprovedNoiseInline ([BI[J)I ------------------
    Fixture { pair: ("PaperNativeImprovedNoiseInline", "oldPMethodSummary"), n_ints: 1, ints: [16, 0, 0, 0], with_bytes: true, bytes: &[0, -17, -34, -51, -68, -85, -102, -119, 120, 103, 86, 69, 52, 35, 18, 1], longs: &[7, 31, 3, 15, 63, 1, 9, 21], res: -3, dst: &[7, 31, 3, 15, 63, 1, 9, 21] },
    Fixture { pair: ("PaperNativeImprovedNoiseInline", "oldPMethodSummary"), n_ints: 1, ints: [0, 0, 0, 0], with_bytes: true, bytes: &[0, -17, -34, -51, -68, -85, -102, -119, 120, 103, 86, 69, 52, 35, 18, 1], longs: &[1, 2, 3, 4, 5, 6, 7, 8], res: -3, dst: &[1, 2, 3, 4, 5, 6, 7, 8] },
    Fixture { pair: ("PaperNativeImprovedNoiseInline", "oldPMethodSummary"), n_ints: 1, ints: [8, 0, 0, 0], with_bytes: true, bytes: &[0, 37, 74, 111, -108, -71, -34, 3], longs: &[-5, 12, 0, -1, 4096, -4096, 77, 123], res: -3, dst: &[-5, 12, 0, -1, 4096, -4096, 77, 123] },
    Fixture { pair: ("PaperNativeImprovedNoiseInline", "oldPMethodSummary"), n_ints: 1, ints: [4, 0, 0, 0], with_bytes: true, bytes: &[0, -17, -34, -51], longs: &[1000000, -1000000, 42, 42, 42, 0, 0, 1], res: -3, dst: &[1000000, -1000000, 42, 42, 42, 0, 0, 1] },
    // --- TASK-54: PaperNativePalettedReencodeScratch (I[J)I ----------------
    Fixture { pair: ("PaperNativePalettedReencodeScratch", "oldNewArraySummary"), n_ints: 1, ints: [16, 0, 0, 0], with_bytes: false, bytes: &[], longs: &[7, 31, 3, 15, 63, 1, 9, 21], res: 4, dst: &[16, 4299696267192942464, 3014211390965140080, -6648798010941522952, 63, 1, 9, 21] },
    Fixture { pair: ("PaperNativePalettedReencodeScratch", "oldNewArraySummary"), n_ints: 1, ints: [0, 0, 0, 0], with_bytes: false, bytes: &[], longs: &[1, 2, 3, 4, 5, 6, 7, 8], res: 4, dst: &[0, 0, 0, 0, 5, 6, 7, 8] },
    Fixture { pair: ("PaperNativePalettedReencodeScratch", "oldNewArraySummary"), n_ints: 1, ints: [64, 0, 0, 0], with_bytes: false, bytes: &[], longs: &[-5, 12, 0, -1, 4096, -4096, 77, 123], res: 4, dst: &[64, -1247959004937781760, -3884900653599734408, -6648798010941522952, 4096, -4096, 77, 123] },
    Fixture { pair: ("PaperNativePalettedReencodeScratch", "oldNewArraySummary"), n_ints: 1, ints: [256, 0, 0, 0], with_bytes: false, bytes: &[], longs: &[1000000, -1000000, 42, 42, 42, 0, 0, 1], res: 4, dst: &[256, -4991836019751127040, 607142164520263220, -6648798010941522952, 42, 0, 0, 1] },
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
        Ok((fixtures_ok, bridge_ok)) => eprintln!(
            "[crussty-plugin] kernel_promote: SELF-TEST PASS (fixtures {fixtures_ok}/{} byte-exact vs offline old-impl expectations, bridge parity {bridge_ok}/{} from-name == to-name post-rebind)",
            FIXTURES.len(),
            FIXTURES.len()
        ),
        Err(e) => eprintln!("[crussty-plugin] kernel_promote: SELF-TEST FAIL — {e}"),
    }
}

/// One resolved bridge pair: jclass + both method ids + signature.
struct ResolvedPair {
    class: &'static str,
    from_kernel: &'static str,
    cls: jni::jclass,
    from_mid: jni::jmethodID,
    to_mid: jni::jmethodID,
    sig: String,
}

fn run(env: &JniEnv) -> Result<(usize, usize), String> {
    // Resolve every promoted pair's bridge methods up front (fail fast).
    let mut resolved: Vec<ResolvedPair> = Vec::with_capacity(kernel_policy::PROMOTE_PAIRS.len());
    for p in kernel_policy::PROMOTE_PAIRS {
        let entry = crate::jni_table::MAIN_JNI_TABLE
            .iter()
            .find(|e| e.class == p.class && e.method == p.from_kernel)
            .ok_or_else(|| format!("{}:{} missing from jni_table", p.class, p.from_kernel))?;
        let Some(cls) = env.find_class(p.class) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            return Err(format!("find_class({}) failed (surface not injected?)", p.class));
        };
        let from_mid = env
            .get_static_method_id(cls, p.from_kernel, entry.sig)
            .ok_or_else(|| format!("{}:{} unresolved", p.class, p.from_kernel))?;
        let to_mid = env
            .get_static_method_id(cls, p.to_kernel, entry.sig)
            .ok_or_else(|| format!("{}:{} unresolved", p.class, p.to_kernel))?;
        resolved.push(ResolvedPair {
            class: p.class,
            from_kernel: p.from_kernel,
            cls,
            from_mid,
            to_mid,
            sig: entry.sig.to_string(),
        });
    }

    let mut fixtures_ok = 0usize;
    let mut bridge_ok = 0usize;
    let mut failures = 0usize;

    for (idx, f) in FIXTURES.iter().enumerate() {
        let rp = resolved
            .iter()
            .find(|r| r.class == f.pair.0 && r.from_kernel == f.pair.1)
            .ok_or_else(|| format!("fixture #{idx}: pair {}:{} unresolved", f.pair.0, f.pair.1))?;

        // Build the argument list per the pair's signature shape.
        let (args, long_arr, byte_arr) = build_args(env, rp, f)?;
        let r_from = env.call_static_int_method(rp.cls, rp.from_mid, &args);
        let exc = cplug_sdk::jni_util::clear_exception(env);

        let mut got = vec![0i64; f.dst.len()];
        env.get_long_array_region(long_arr, 0, f.dst.len() as jni::jsize, &mut got);
        if let Some(b) = byte_arr {
            env.delete_local_ref(b);
        }
        env.delete_local_ref(long_arr);

        if exc {
            failures += 1;
            eprintln!("[crussty-plugin] kernel_promote: fixture #{idx} threw");
            continue;
        }
        if r_from != f.res || got != f.dst {
            failures += 1;
            eprintln!(
                "[crussty-plugin] kernel_promote: fixture #{idx} MISMATCH (via re-bound name): res {r_from} (want {}), dst {got:?} (want {:?})",
                f.res, f.dst
            );
            continue;
        }
        fixtures_ok += 1;

        // Bridge parity: the win-name method must agree byte-exactly.
        let (args2, long_arr2, byte_arr2) = build_args(env, rp, f)?;
        let r_to = env.call_static_int_method(rp.cls, rp.to_mid, &args2);
        let exc2 = cplug_sdk::jni_util::clear_exception(env);
        let mut got2 = vec![0i64; f.dst.len()];
        env.get_long_array_region(long_arr2, 0, f.dst.len() as jni::jsize, &mut got2);
        if let Some(b) = byte_arr2 {
            env.delete_local_ref(b);
        }
        env.delete_local_ref(long_arr2);

        if exc2 || r_to != r_from || got2 != got {
            failures += 1;
            eprintln!("[crussty-plugin] kernel_promote: fixture #{idx} BRIDGE-PARITY FAIL (from {r_from} vs to {r_to})");
            continue;
        }
        bridge_ok += 1;
    }

    for rp in &resolved {
        env.delete_local_ref(rp.cls);
    }
    if failures > 0 {
        return Err(format!("{failures} fixture failure(s)"));
    }
    Ok((fixtures_ok, bridge_ok))
}

/// Build jvalue args + fresh arrays for one fixture call under the pair's
/// signature shape (drift-guarded: sig comes from jni_table, and the
/// promote_pairs test pins both sides to the same sig).
fn build_args(
    env: &JniEnv,
    rp: &ResolvedPair,
    f: &Fixture,
) -> Result<(Vec<jni::jvalue>, jni::jlongArray, Option<jni::jbyteArray>), String> {
    let Some(long_arr) = env.new_long_array(f.longs.len() as jni::jsize) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        return Err("new_long_array failed".into());
    };
    env.set_long_array_region(long_arr, 0, f.longs.len() as jni::jsize, f.longs);

    // Distinguish shapes by the resolved signature, drift-guarded against
    // the fixture's own shape tag (jni_table sig changed without the
    // fixture data being updated -> loud failure, not a wrong-arity call).
    let shape_bytes = rp.sig == "([BI[J)I";
    if shape_bytes != f.with_bytes {
        return Err(format!(
            "{}:{} shape drift: sig {} vs fixture with_bytes={}",
            rp.class, rp.from_kernel, rp.sig, f.with_bytes
        ));
    }

    let mut args: Vec<jni::jvalue> = Vec::with_capacity(6);
    let mut byte_arr: Option<jni::jbyteArray> = None;

    if shape_bytes {
        let Some(ba) = env.new_byte_array(f.bytes.len() as jni::jsize) else {
            let _ = cplug_sdk::jni_util::clear_exception(env);
            env.delete_local_ref(long_arr);
            return Err("new_byte_array failed".into());
        };
        env.set_byte_array_region(ba, 0, f.bytes.len() as jni::jsize, f.bytes);
        args.push(jni::jvalue { l: ba });
        args.push(jni::jvalue { i: f.ints[0] });
        args.push(jni::jvalue { l: long_arr });
        byte_arr = Some(ba);
    } else {
        for i in 0..f.n_ints {
            args.push(jni::jvalue { i: f.ints[i] });
        }
        args.push(jni::jvalue { l: long_arr });
    }
    Ok((args, long_arr, byte_arr))
}
