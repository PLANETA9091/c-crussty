//! Runtime wiring for the `area_map` hook.
//!
//! Two halfs, both verified end-to-end:
//!
//! 1. Hook: once the kernel's `SingleUserAreaMap` is loaded we define the
//!    `SingleUserAreaMapOps` helper classes (compiled against the kernel's
//!    class shapes, see `area-map/`) into the SAME loader as the map, then
//!    `READY` flips and the class is retransformed. The byte hook fires on
//!    the retransform (or on a late original load) and swaps `update()`'s
//!    body for a branch-minimal `invokestatic SingleUserAreaMapOps.run(...)`.
//!
//!    The helper classes must live in the map's loader, NOT the bootstrap:
//!    their bytecode references `SingleUserAreaMap` directly, and a
//!    bootstrap-defined copy would both fail to resolve the kernel class and
//!    shadow it for the kernel's own (parent-first) loader.
//!
//! 2. Self-test: after activation, drive the REAL bridge
//!    (`PaperNativeAreaMap.nativeUpdateOpsBatch` through JNI) over random
//!    rects and compare every produced (op, x, z) against the naive
//!    set-difference (adds = new∖old, removes = old∖new). This exercises
//!    bridge registration + JNI marshalling + native enumeration + the
//!    op/key encodings the helper's apply loop decodes.

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;


pub const MAP_CLASS: &str = classfile::MAP_CLASS;
const OPS_NAME: &str = classfile::OPS_CLASS;
const SCRATCH_NAME: &str = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$Scratch";
const OPS1_NAME: &str = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$1";
const NATIVE_CLASS: &str = "ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap";

const OPS1_BYTES: &[u8] = include_bytes!(
    "../area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$1.class"
);
const SCRATCH_BYTES: &[u8] = include_bytes!(
    "../area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$Scratch.class"
);
const OPS_BYTES: &[u8] = include_bytes!(
    "../area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class"
);

// TASK-64 variant C: budgeted-scratch policy, same three class names, built
// from area-map/budget-ca/.../SingleUserAreaMapOpsBudget.java by
// scripts/build_area_map_budget.sh (see that source for the policy and the
// measured closed-native contract it relies on).
const BUDGET_OPS1_BYTES: &[u8] = include_bytes!(
    "../area-map/build-budget/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$1.class"
);
const BUDGET_SCRATCH_BYTES: &[u8] = include_bytes!(
    "../area-map/build-budget/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps$Scratch.class"
);
const BUDGET_OPS_BYTES: &[u8] = include_bytes!(
    "../area-map/build-budget/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class"
);

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// Which helper byte set is defined into the map loader. Class NAMES are
/// identical in both sets (the patched kernel invokestatic resolves
/// `SingleUserAreaMapOps.run` either way) — only the BYTES differ.
type HelperSet = [(&'static str, &'static [u8]); 3];
const LEGACY_SET: HelperSet = [
    (SCRATCH_NAME, SCRATCH_BYTES),
    (OPS1_NAME, OPS1_BYTES),
    (OPS_NAME, OPS_BYTES),
];
const BUDGET_SET: HelperSet = [
    (SCRATCH_NAME, BUDGET_SCRATCH_BYTES),
    (OPS1_NAME, BUDGET_OPS1_BYTES),
    (OPS_NAME, BUDGET_OPS_BYTES),
];

/// Rollout mode for the budgeted scratch policy (TASK-64 variant C,
/// design §11.2/§11.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetMode {
    Off,
    On,
}

/// Env var consulted (once) for the budget rollout gate.
const BUDGET_ENV: &str = "CRUSSTY_AREAMAP_BUDGET";

/// Fail-safe parse (the `batch_api::parse_rollout` rule): only exact `on`
/// (trimmed, any case) widens; unset/empty/`off`/`1`/garbage → Off.
fn parse_budget(raw: Option<&str>) -> BudgetMode {
    match raw.map(|v| v.trim().to_ascii_lowercase()) {
        Some(v) if v == "on" => BudgetMode::On,
        _ => BudgetMode::Off,
    }
}

/// The process-wide budget mode, read from `CRUSSTY_AREAMAP_BUDGET` exactly
/// once (OnceLock, the `batch_api::rollout_mode` pattern). Emits the
/// grep-able boot marker once per process. Kill-switch / rollback = flip the
/// env + server restart; no rebuild, no `.so` swap. Gate off = the legacy
/// bytes are defined verbatim (no runtime flag, no behavioral delta).
pub fn budget_mode() -> BudgetMode {
    static MODE: OnceLock<BudgetMode> = OnceLock::new();
    *MODE.get_or_init(|| {
        let raw = std::env::var(BUDGET_ENV).ok();
        let m = parse_budget(raw.as_deref());
        let (name, note) = match m {
            BudgetMode::Off => (
                "off",
                "legacy grow-to-cap scratch (default; fail-safe)",
            ),
            BudgetMode::On => (
                "on",
                "budgeted scratch + -n0 retry (native-contract self-test gates arming)",
            ),
        };
        let shown = raw.as_deref().unwrap_or("unset");
        eprintln!(
            "[crussty-plugin] area_map: budget gate {BUDGET_ENV}={shown} -> mode={name} ({note})"
        );
        m
    })
}

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    cplug_sdk::hooks::register_bytes(MAP_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match classfile::patch_update(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] area_map: patched {name} update() ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] area_map: patch failed: {e}");
                None
            }
        }
    });
}

/// Background activation: wait for the kernel map class, define the helper
/// classes into its loader, then flip READY and retransform so the hook
/// applies the patch.
pub fn activate() {
    std::thread::spawn(|| {
        // Poll via JVMTI GetLoadedClasses (SDK) — unlike raw JNI find_class
        // (system loader only) this sees classes defined in the kernel's own
        // classloader, where Moonrise loads SingleUserAreaMap. Moonrise loads
        // it lazily (first area-map use), so on an idle world we also force
        // the load through the kernel loader via Class.forName after a grace
        // period; the hook then applies on the retransform below.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(MAP_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!("[crussty-plugin] area_map: {MAP_CLASS} not loaded within 180s, hook stays dormant");
                return;
            }
            // Retry the force-load until it works: the single-shot attempt
            // raced the kernel boot (org.bukkit.Bukkit is only resolvable
            // once the plugin system is up) and a missed attempt left the
            // hook dormant for the whole run on a fast boot.
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                if forced_attempts < 12 || forced_attempts % 12 == 0 {
                    forced_attempts += 1;
                    eprintln!("[crussty-plugin] area_map: forcing kernel load of {MAP_CLASS} (attempt {forced_attempts})");
                }
                force_load_kernel_class();
            }
            // TASK-22/C1 negative backoff: while the class name has never
            // been sighted through the ClassFileLoadHook feed, find_class
            // answers from the feed without any JVMTI scan, so a relaxed 10s
            // cadence costs nothing; once sighted, keep the 2s cadence for
            // activation latency. The 180s deadline handling above is
            // unchanged.
            let sighted = cplug_sdk::classes::is_sighted(MAP_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // TASK-64 variant C: pick which helper byte set gets defined. Gate
        // off (default) = legacy bytes, zero delta. Gate on = the closed
        // native's len>=n / -n0 contract is probed IN THIS JVM first (the
        // arming assumption is per-deployed-.so, not per-build) and a failure
        // falls back to the legacy bytes — a hostile or older native can
        // never break the server through this path (define_class for the
        // same name is one-shot, so the decision MUST precede definition).
        let (budget_armed, set): (bool, &HelperSet) = match budget_mode() {
            BudgetMode::Off => {
                eprintln!(
                    "[crussty-plugin] area_map: budget path OFF (gate off; legacy grow-to-cap scratch)"
                );
                (false, &LEGACY_SET)
            }
            BudgetMode::On => match cplug_sdk::jni_util::with_attached(budget_contract_selftest)
            {
                Some(Some(true)) => {
                    (true, &BUDGET_SET)
                }
                Some(Some(false)) => {
                    eprintln!(
                        "[crussty-plugin] area_map: budget self-test FAILED -> legacy scratch retained"
                    );
                    (false, &LEGACY_SET)
                }
                // outer None = no env; Some(None) = self-test infra failure
                _ => {
                    eprintln!(
                        "[crussty-plugin] area_map: budget self-test skipped (no env or infra) -> legacy scratch retained"
                    );
                    (false, &LEGACY_SET)
                }
            },
        };

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(MAP_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return false;
            };
            let mut ok = true;
            for (name, bytes) in set.iter().copied() {
                match env.define_class(name, loader, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] area_map: defined {name} in map loader");
                    }
                    None => {
                        crate::clear_exception(env);
                        eprintln!("[crussty-plugin] area_map: define_class({name}) failed");
                        ok = false;
                    }
                }
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] area_map: helper definition aborted (no env or loader)");
            return;
        }

        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MAP_CLASS);
        eprintln!("[crussty-plugin] area_map: hook armed, retransform rc={rc}");
        if budget_armed {
            eprintln!(
                "[crussty-plugin] area_map: budget path ARMED (SingleUserAreaMapOps = budgeted scratch, -n0 retry)"
            );
        }
        // TASK-22/C1 acceptance: exactly one line per hook with the final
        // scans-avoided count (the poller has exited the loop by now, so
        // this is the final number for this hook's polling window).
        eprintln!(
            "[crussty-plugin] area_map: sighting feed: {} full class-heap scans avoided",
            cplug_sdk::classes::scans_avoided(MAP_CLASS)
        );

        // Semantic self-test through the real bridge + native.
        if cplug_sdk::jni_util::with_attached(bridge_selftest).is_none() {
            eprintln!("[crussty-plugin] area_map: self-test skipped (no env)");
        }
    });
}

/// Drive nativeUpdateOpsBatch through the bridge and check every produced
/// (op, x, z) against the naive set difference. All positions are chosen so
/// the tests stay deterministic (fixed LCG, small radii).
fn bridge_selftest(env: &JniEnv) -> Option<()> {
    let cls = env.find_class(NATIVE_CLASS)?;
    let mid = env.get_static_method_id(
        cls,
        "nativeUpdateOpsBatch",
        "(IIIIII[B[J)I",
    )?;
    let cap = 2 * (2 * 6 + 1) * (2 * 6 + 1); // max for d=6 in both squares

    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    let mut checked = 0usize;
    let mut failures = 0usize;
    for _ in 0..64 {
        let from_x = ((next() % 21) as i32) - 10;
        let from_z = ((next() % 21) as i32) - 10;
        let to_x = ((next() % 21) as i32) - 10;
        let to_z = ((next() % 21) as i32) - 10;
        let old_d = (next() % 7) as i32;
        let new_d = (next() % 7) as i32;

        let ops_arr = env.new_byte_array(cap)?;
        let Some(keys_arr) = env.new_long_array(cap) else {
            env.delete_local_ref(ops_arr);
            return None;
        };
        let n = env.call_static_int_method(
            cls,
            mid,
            &[
                jni::jvalue { i: from_x },
                jni::jvalue { i: from_z },
                jni::jvalue { i: old_d },
                jni::jvalue { i: to_x },
                jni::jvalue { i: to_z },
                jni::jvalue { i: new_d },
                jni::jvalue { l: ops_arr },
                jni::jvalue { l: keys_arr },
            ],
        );
        let had_exc = crate::clear_exception(env);

        let mut ops = vec![0i8; cap as usize];
        let mut keys = vec![0i64; cap as usize];
        env.get_byte_array_region(ops_arr, 0, cap, &mut ops);
        env.get_long_array_region(keys_arr, 0, cap, &mut keys);
        env.delete_local_ref(ops_arr);
        env.delete_local_ref(keys_arr);
        if had_exc {
            eprintln!("[crussty-plugin] area_map: self-test rect ({from_x},{from_z},d{old_d})->({to_x},{to_z},d{new_d}) threw");
            failures += 1;
            continue;
        }

        // H-12 (hardening audit): never trust the closed native's count past
        // the buffer capacity — a hostile/buggy n would OOB-index ops/keys.
        let n = (n.max(0) as usize).min(cap as usize);
        let (expected_adds, expected_removes) = naive_set_difference(from_x, from_z, old_d, to_x, to_z, new_d);
        let mut actual_adds = HashSet::new();
        let mut actual_removes = HashSet::new();
        let mut dup = false;
        for i in 0..n {
            let key = keys[i];
            let x = key as i32;
            let z = (key >> 32) as i32;
            let cell = (x, z);
            if ops[i] == 0 {
                dup |= !actual_adds.insert(cell);
            } else {
                dup |= !actual_removes.insert(cell);
            }
        }
        if dup || n != expected_adds.len() + expected_removes.len()
            || actual_adds != expected_adds
            || actual_removes != expected_removes
        {
            failures += 1;
            if failures <= 3 {
                eprintln!(
                    "[crussty-plugin] area_map: SELF-TEST FAIL ({from_x},{from_z},d{old_d})->({to_x},{to_z},d{new_d}): n={n} expected {}+{} (dup={dup})",
                    expected_adds.len(),
                    expected_removes.len()
                );
            }
        }
        checked += 1;
    }
    env.delete_local_ref(cls);
    if failures == 0 {
        eprintln!("[crussty-plugin] area_map: self-test OK ({checked} rects, native == naive set difference)");
    } else {
        eprintln!("[crussty-plugin] area_map: self-test FAILED {failures}/{checked}");
    }
    Some(())
}

/// TASK-64 variant C arming gate: probe the closed native's buffer contract
/// IN THIS JVM before the budgeted helper bytes may be defined (the contract
/// is a property of the deployed .so, not of this build — design §11.1/§11.2
/// falsifier 2). Per deterministic rect, verifies:
///   (a) control  — cap-sized buffers reproduce the naive set difference
///       (the legacy shape, unchanged);
///   (b) size oracle — len = n0-1 is rejected with EXACTLY -n0 (the built-in
///       size oracle the budgeted retry relies on; STEP-0 probe);
///   (c) boundary — len = n0 is accepted and reproduces the naive set
///       difference (the len >= n contract at its exact boundary).
/// Any violation → `Some(false)` → the caller defines the legacy bytes
/// instead (fail-safe; define_class for the same name is one-shot, so this
/// decision must precede definition). `None` = JNI infrastructure failure
/// (also fails safe at the caller).
fn budget_contract_selftest(env: &JniEnv) -> Option<bool> {
    let cls = env.find_class(NATIVE_CLASS)?;
    let mid = env.get_static_method_id(cls, "nativeUpdateOpsBatch", "(IIIIII[B[J)I")?;

    // Deterministic shapes: minimal, cardinal 1-chunk, diagonal 1-chunk,
    // asymmetric resize, prod-max d=33 (diagonal), disjoint jump (n0 lands
    // exactly on the budgeted INITIAL_CAP floor boundary).
    let rects: [Rect; 6] = [
        (0, 0, 2, 1, 0, 2),
        (0, 0, 6, 1, 0, 6),
        (0, 0, 9, 1, 1, 9),
        (5, 7, 3, 6, 7, 9),
        (0, 0, 33, 1, 1, 33),
        (10, 10, 8, -5, -5, 8),
    ];
    let probe = NativeProbe { env, cls, mid };

    let mut checked = 0usize;
    let mut failures = 0usize;
    for &r in rects.iter() {
        let (fx, fz, fd, tx, tz, td) = r;
        let exp = naive_set_difference(fx, fz, fd, tx, tz, td);
        let n0 = exp.0.len() + exp.1.len(); // > 0 for every rect here
        let cap = max_ops_bound(fd, td);

        // (a) control at cap
        if !probe.probe_parity(r, cap, &exp, &mut checked, &mut failures) {
            env.delete_local_ref(cls);
            return None;
        }

        // (b) rejection oracle at n0-1: raw return must be exactly -n0
        let (n, had_exc) = probe.call_raw(r, (n0 - 1) as i32)?;
        if !had_exc && n == -(n0 as i32) {
            checked += 1;
        } else {
            failures += 1;
            eprintln!(
                "[crussty-plugin] area_map: budget self-test: ({fx},{fz},d{fd})->({tx},{tz},d{td}) len={} expected -{n0} got n={n} exc={had_exc}",
                n0 - 1
            );
        }

        // (c) boundary acceptance at n0
        if !probe.probe_parity(r, n0 as i32, &exp, &mut checked, &mut failures) {
            env.delete_local_ref(cls);
            return None;
        }
    }
    env.delete_local_ref(cls);
    if failures == 0 {
        eprintln!(
            "[crussty-plugin] area_map: budget self-test OK ({checked} probes, len>=n contract + -n0 size oracle verified)"
        );
        Some(true)
    } else {
        eprintln!(
            "[crussty-plugin] area_map: budget self-test FAILED ({failures} bad of {} probes)",
            checked + failures
        );
        Some(false)
    }
}

/// Deterministic rect: (fromX, fromZ, oldD, toX, toZ, newD).
type Rect = (i32, i32, i32, i32, i32, i32);

/// JNI call surface for the contract probes (bundled so the probe helpers
/// stay under the clippy arg limit — the surface is fixed per self-test).
struct NativeProbe<'a> {
    env: &'a JniEnv,
    cls: jni::jclass,
    mid: jni::jmethodID,
}

impl NativeProbe<'_> {
    /// One native call at the given buffer length; returns the raw
    /// (n, exception flag) without interpreting either (the negative oracle
    /// IS the signal). `None` = JNI infrastructure failure.
    fn call_raw(&self, r: Rect, len: i32) -> Option<(i32, bool)> {
        let (fx, fz, fd, tx, tz, td) = r;
        let ops_arr = self.env.new_byte_array(len)?;
        let Some(keys_arr) = self.env.new_long_array(len) else {
            self.env.delete_local_ref(ops_arr);
            return None;
        };
        let n = self.env.call_static_int_method(
            self.cls,
            self.mid,
            &[
                jni::jvalue { i: fx },
                jni::jvalue { i: fz },
                jni::jvalue { i: fd },
                jni::jvalue { i: tx },
                jni::jvalue { i: tz },
                jni::jvalue { i: td },
                jni::jvalue { l: ops_arr },
                jni::jvalue { l: keys_arr },
            ],
        );
        let had_exc = crate::clear_exception(self.env);
        self.env.delete_local_ref(ops_arr);
        self.env.delete_local_ref(keys_arr);
        Some((n, had_exc))
    }

    /// One native call at the given buffer length; the produced (op, cell)
    /// multiset must equal the naive set difference. Counts one probe.
    /// `false` (vs the verdict in `failures`) = JNI infrastructure failure.
    fn probe_parity(
        &self,
        r: Rect,
        len: i32,
        exp: &RectDiff,
        checked: &mut usize,
        failures: &mut usize,
    ) -> bool {
        let (fx, fz, fd, tx, tz, td) = r;
        let (exp_adds, exp_removes) = exp;
        let ops_arr = match self.env.new_byte_array(len) {
            Some(a) => a,
            None => return false,
        };
        let Some(keys_arr) = self.env.new_long_array(len) else {
            self.env.delete_local_ref(ops_arr);
            return false;
        };
        let n = self.env.call_static_int_method(
            self.cls,
            self.mid,
            &[
                jni::jvalue { i: fx },
                jni::jvalue { i: fz },
                jni::jvalue { i: fd },
                jni::jvalue { i: tx },
                jni::jvalue { i: tz },
                jni::jvalue { i: td },
                jni::jvalue { l: ops_arr },
                jni::jvalue { l: keys_arr },
            ],
        );
        let had_exc = crate::clear_exception(self.env);
        let mut ops = vec![0i8; len.max(0) as usize];
        let mut keys = vec![0i64; len.max(0) as usize];
        if n > 0 && n <= len {
            self.env.get_byte_array_region(ops_arr, 0, n, &mut ops[..n as usize]);
            self.env.get_long_array_region(keys_arr, 0, n, &mut keys[..n as usize]);
        }
        self.env.delete_local_ref(ops_arr);
        self.env.delete_local_ref(keys_arr);

        let exp_n = exp_adds.len() + exp_removes.len();
        let ok = if had_exc || n != exp_n as i32 {
            false
        } else {
            let mut adds: HashSet<(i32, i32)> = HashSet::new();
            let mut removes: HashSet<(i32, i32)> = HashSet::new();
            let mut dup = false;
            for i in 0..n as usize {
                let key = keys[i];
                let cell = (key as i32, (key >> 32) as i32);
                if ops[i] == 0 {
                    dup |= !adds.insert(cell);
                } else {
                    dup |= !removes.insert(cell);
                }
            }
            !dup && adds == *exp_adds && removes == *exp_removes
        };
        if ok {
            *checked += 1;
        } else {
            *failures += 1;
            eprintln!(
                "[crussty-plugin] area_map: budget self-test: ({fx},{fz},d{fd})->({tx},{tz},d{td}) len={len} n={n} exc={had_exc} parity broken"
            );
        }
        true
    }
}

/// True worst-case diff bound (the budgeted policy's growth clamp and retry
/// sanity ceiling): (2·oldD+1)² + (2·newD+1)², int-clamped like the Java
/// `maxOps`.
fn max_ops_bound(old_d: i32, new_d: i32) -> i32 {
    let old_side = 2i64 * old_d as i64 + 1;
    let new_side = 2i64 * new_d as i64 + 1;
    let cap = old_side * old_side + new_side * new_side;
    if cap > i32::MAX as i64 { i32::MAX } else { cap as i32 }
}

/// Naive reference: adds = new square ∖ old square, removes = old ∖ new.
type RectDiff = (HashSet<(i32, i32)>, HashSet<(i32, i32)>);
fn naive_set_difference(
    from_x: i32,
    from_z: i32,
    old_d: i32,
    to_x: i32,
    to_z: i32,
    new_d: i32,
) -> RectDiff {
    let mut old_set = HashSet::new();
    for x in from_x - old_d..=from_x + old_d {
        for z in from_z - old_d..=from_z + old_d {
            old_set.insert((x, z));
        }
    }
    let mut new_set = HashSet::new();
    for x in to_x - new_d..=to_x + new_d {
        for z in to_z - new_d..=to_z + new_d {
            new_set.insert((x, z));
        }
    }
    let adds: HashSet<(i32, i32)> = new_set.difference(&old_set).copied().collect();
    let removes: HashSet<(i32, i32)> = old_set.difference(&new_set).copied().collect();
    (adds, removes)
}

/// Force the kernel to load `MAP_CLASS` (lazy Moonrise class) through its own
/// classloader. JNI `FindClass` from a native thread only resolves against the
/// system loader, and `Class.forName(String, boolean, ClassLoader)` is the only
/// load trigger that lets us name that loader explicitly. Runs its static
/// initializer too (initialize=true), which is a good canary that the patched
/// class actually links in the kernel — errors surface as exceptions here, not
/// later as a fatal `VerifyError` at first use.
fn force_load_kernel_class() {
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        let Some(seed) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
            eprintln!("[crussty-plugin] area_map: force load: Bukkit not found");
            return None::<()>;
        };
        // getClassLoader is declared on java.lang.Class; GetMethodID on an
        // interface (Bukkit) does not inherit Object instance methods, so
        // resolve the mid from the Class class directly.
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] area_map: force load: no Class class");
            return None::<()>;
        };
        let Some(loader) = env
            .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
            .and_then(|mid| {
                let l = env.call_object_method(seed.as_jclass(), mid, &[]);
                (l as usize != 0).then_some(l)
            })
        else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] area_map: force load: no kernel loader");
            return None::<()>;
        };
        let forname = env.get_static_method_id(
            class_cls,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        );
        let Some(forname) = forname else {
            crate::clear_exception(env);
            eprintln!("[crussty-plugin] area_map: force load: no forName mid");
            return None::<()>;
        };
        let dot = MAP_CLASS.replace('/', ".");
        let name = env.new_string(&dot)?;
        let loaded = env.call_static_object_method(
            class_cls,
            forname,
            &[
                jni::jvalue { l: name },
                jni::jvalue { z: 1 /* true */ },
                jni::jvalue { l: loader },
            ],
        );
        let had_exc = crate::clear_exception(env);
        if loaded.is_null() {
            eprintln!("[crussty-plugin] area_map: Class.forName({MAP_CLASS}) failed (exc={had_exc})");
        } else {
            eprintln!("[crussty-plugin] area_map: Class.forName({MAP_CLASS}) succeeded");
        }
        env.delete_local_ref(loaded);
        env.delete_local_ref(name);
        env.delete_local_ref(class_cls);
        env.delete_local_ref(loader);
        let _ = had_exc;
        Some(())
    });
}

#[cfg(test)]
mod budget_tests {
    use super::*;

    /// Fail-safe parse (design §11.4 row 3): only exact `on` (trimmed, any
    /// case) widens; unset/empty/off/1/garbage land on the conservative Off.
    #[test]
    fn parse_budget_failsafe() {
        assert_eq!(parse_budget(Some("on")), BudgetMode::On);
        assert_eq!(parse_budget(Some(" ON ")), BudgetMode::On);
        assert_eq!(parse_budget(Some("On")), BudgetMode::On);
        assert_eq!(parse_budget(None), BudgetMode::Off);
        assert_eq!(parse_budget(Some("")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("off")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("1")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("0")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("auto")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("garbage")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("onx")), BudgetMode::Off);
        assert_eq!(parse_budget(Some("\u{0}on")), BudgetMode::Off);
    }

    /// The gate is only meaningful if the two byte sets are genuinely
    /// different artifacts (protects against a stale build-budget that was
    /// accidentally built from the legacy source).
    #[test]
    fn budget_bytes_differ_from_legacy() {
        for (name, legacy, budget) in [
            (OPS_NAME, OPS_BYTES, BUDGET_OPS_BYTES),
            (SCRATCH_NAME, SCRATCH_BYTES, BUDGET_SCRATCH_BYTES),
            (OPS1_NAME, OPS1_BYTES, BUDGET_OPS1_BYTES),
        ] {
            assert!(!legacy.is_empty(), "{name} legacy bytes empty");
            assert!(!budget.is_empty(), "{name} budget bytes empty");
            assert_ne!(legacy, budget, "{name}: budget set must not be the legacy bytes");
        }
    }

    /// int-clamped worst-case bound agrees with the Java maxOps on the
    /// self-test shapes and the prod shapes.
    #[test]
    fn max_ops_bound_matches_java_maxops() {
        // 2*(2d+1)^2 for oldD == newD == d
        for d in [2, 6, 8, 9, 33, 63, 255, 511] {
            let expected: i32 = 2 * (2 * d + 1) * (2 * d + 1);
            assert_eq!(max_ops_bound(d, d), expected);
        }
        // asymmetric: (2*3+1)^2 + (2*9+1)^2 = 49 + 361 = 410
        assert_eq!(max_ops_bound(3, 9), 410);
        // below the clamp: d=5000 -> 2*(10001)^2 = 200040002
        assert_eq!(max_ops_bound(5000, 5000), 200_040_002);
        // int clamp triggers (2*32769^2 = 2147614722 > i32::MAX), no i64
        // overflow on the way there
        assert_eq!(max_ops_bound(16384, 16384), i32::MAX);
    }
}
