//! Runtime wiring for the INSIDE-BATCH lever (TASK-411-B R3 — the
//! inside-discovery lane: RESEARCH-B-k3 profile 11.76/12.77/12.87% scene,
//! item-доминанта 60-62%, target = Entity.checkInsideBlocks).
//!
//! The lane pays the full traversal machinery (guava-iterators, cursor,
//! PalettedContainer.get, visit-set, budget, collector) for every
//! per-movement still query even when ALL visited blocks are air — the
//! vanilla FBIB still-branch visits BlockPos.betweenClosed(box) with
//! step≡0, so the vanilla return is ALWAYS 1 and air-visits perform no
//! observable action. This lever serves those queries from a NEGATIVE
//! FAST-PLANE (thread-confined verdict table keyed by the full query
//! fingerprint) and refreshes the verdicts with ONE bulk JNI per phase
//! per thread (per-entity JNI is forbidden by mandate).
//!
//! Split of responsibilities (fail-closed from BOTH sides):
//!   - Java (InsideRustOps, compiled offline vs the patched kernel jar):
//!     gate STRICT eq (empty = vanilla), ORIG MethodHandle vanilla
//!     replica, fast-plane/gather/flush bookkeeping, distinct-section
//!     resolution (hasOnlyAir / unknown=2), permanent disarm on any
//!     Throwable or negative native rc.
//!   - Rust (this module): bridge define + RegisterNatives + armState
//!     probe (probe-then-patch), the Entity byte-stage composes through
//!     entity_compose (STRICT sites==3), and the native verdict stage
//!     `inside_batch_tick`: structural re-derivation of floors/counts
//!     from the wire geometry (ERR_STRUCT → negative rc → java disarm)
//!     + bit-in-byte section-flag combine into out[].
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp411_insidebat"` (STRICT eq;
//! empty = vanilla bit-in-byte). Off by default — dormant-invisible
//! discipline (inside_cache/fluid_guard precedent).
//!
//! Fail-closed matrix: bridge define failure OR armState() != "ARMED" →
//! BRIDGE_READY never published → compose stage skipped (Entity vanilla);
//! patcher Err (kernel shape mismatch) → stage skipped; native structural
//! violation → negative rc → java DISARMED forever (vanilla replica);
//! unknown section / hull too big / SEC_CAP overflow / skipped flush →
//! stale verdicts → miss → vanilla.

use jvmti_bindings::prelude::*;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideRustOps";
/// Nested thread-state class — a JNI-defined class resolves nested types
/// from the SAME loader (kernel loader), so it must be defined explicitly
/// (inside_cache / InsideBlockOps$Recorder precedent; the lazy
/// ThreadLocal.withInitial(TState::new) resolution would otherwise throw
/// NoClassDefFoundError on the first checkInside call → permanent disarm).
const TSTATE_CLASS: &str = "net/minecraft/world/entity/InsideRustOps$TState";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideRustOps.class");
const TSTATE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideRustOps$TState.class");

const NATIVE_NAME: &str = "insideBatchTick";
const NATIVE_SIG: &str = "(III[I[D[I[I[B)I";

// --- structural error codes (negative rc ⇒ java permanent disarm) -------
pub const ERR_LEN: i32 = -1;
pub const ERR_FLOOR: i32 = -2;
pub const ERR_COUNT: i32 = -3;
pub const ERR_TRIPLE: i32 = -4;
pub const ERR_FLAG: i32 = -5;
/// Per-record section-hull product cap (java guarantees ≤ 64 on the wire).
pub const HULL_SEC_MAX: i64 = 64;

fn lever_mode() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp411_insidebat")
    )
}

/// Gate visibility for the entity_compose stage pipeline.
pub fn enabled_pub() -> bool {
    lever_mode()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the DEFINED ops class (0 = none) — probe via this ref only:
/// a JNI-defined class is not reachable from env.find_class on a native
/// thread (system loader; noise_fill.rs smoke-1 / s7204 evidence).
static OPS_GREF: AtomicUsize = AtomicUsize::new(0);

/// Pollable gate for the entity_compose stage pipeline.
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !lever_mode() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(Ordering::Acquire)
}

/// Register (idempotent; call once from cplugin_init). NO byte hook — the
/// Entity patch composes through entity_compose (stage 1c).
pub fn register() {
    if !lever_mode() {
        eprintln!(
            "[crussty-plugin] inside_batch: dormant (CRUSSTY_LEVER_FLAG must equal cmp411_insidebat)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_batch: bridge owner armed, Entity stage delegated to entity_compose"
    );
}

/// Probe the defined bridge: armState() must return "ARMED".
fn probe_armed() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
        let gref = OPS_GREF.load(Ordering::SeqCst) as jni::jclass;
        if gref.is_null() {
            return false;
        }
        let Some(mid) = env.get_static_method_id(gref, "armState", "()Ljava/lang/String;") else {
            crate::clear_exception(env);
            return false;
        };
        let res = env.call_static_object_method(gref, mid, &[]);
        if res.is_null() {
            crate::clear_exception(env);
            return false;
        }
        let state = env.get_string_utf(res as jni::jstring).unwrap_or_default();
        env.delete_local_ref(res);
        state == "ARMED"
    })
    .unwrap_or(false)
}

/// Background activation: wait for the kernel Entity class, define the
/// bridge into the kernel loader, RegisterNatives the bulk verdict stage,
/// PROBE ARMED, publish BRIDGE_READY. The Entity patch is computed and
/// applied by entity_compose.
pub fn activate() {
    if !lever_mode() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_batch: {ENTITY_CLASS} not loaded within 180s, bridge stays undefined"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_batch: forcing kernel load of {ENTITY_CLASS}"
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] inside_batch: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_batch: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [("ops", OPS_BYTES), ("tstate", TSTATE_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] inside_batch: embedded {name} is class major {major} but JVM supports up to {jvm_major} — rebuild via scripts/build_inside_batch_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] inside_batch: define_class({OPS_CLASS}) failed");
                return false;
            };
            let Some(ts) = env.define_class(TSTATE_CLASS, gref, TSTATE_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] inside_batch: define_class({TSTATE_CLASS}) failed");
                env.delete_local_ref(c);
                return false;
            };
            env.delete_local_ref(ts);
            // RegisterNatives the bulk verdict stage (prepare_manager pattern:
            // per-kernel bridge class global ref is NOT needed — the native
            // receives only primitive arrays). MUST happen before BRIDGE_READY
            // (probe-then-patch: only an ARMED + registered bridge publishes).
            let name = CString::new(NATIVE_NAME).expect("no NUL");
            let sig = CString::new(NATIVE_SIG).expect("no NUL");
            let natives = [jni::JNINativeMethod {
                name: name.as_ptr(),
                signature: sig.as_ptr(),
                fnPtr: inside_batch_tick as *const c_void as *mut c_void,
            }];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] inside_batch: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                return false;
            }
            // Keep a global ref to the ops class for probe_armed().
            let gr = env.new_global_ref(c);
            OPS_GREF.store(gr as usize, Ordering::SeqCst);
            env.delete_local_ref(c);
            eprintln!("[crussty-plugin] inside_batch: defined {OPS_CLASS} (+TState) + registered {NATIVE_NAME}{NATIVE_SIG}");
            true
        });
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] inside_batch: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // PROBE-THEN-PATCH: only an ARMED bridge may be published.
        if !probe_armed() {
            eprintln!(
                "[crussty-plugin] inside_batch: armState() != ARMED (gate/MethodHandle/Unsafe resolution failed), hook stays dormant"
            );
            return;
        }

        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "checkInside", "inside_batch v1");
        eprintln!(
            "[crussty-plugin] inside_batch: bridge defined+armed, BRIDGE_READY (Entity stage composes via entity_compose)"
        );
    });
}

// ---------------------------------------------------------------------------
// Native verdict stage: structural validation + bit-in-byte combine
// ---------------------------------------------------------------------------

/// Section coordinate of a box bound: java `floori(v) >> 4` semantics —
/// (int)v truncates toward zero, then adjust below the integral value
/// (NaN → 0 in both languages; `as` casts saturate identically), then the
/// ARITHMETIC >>4 (matches java on negatives: floor(-1.5)=-2 → -2>>4=-1).
fn fl_sec(v: f64) -> i32 {
    let i = v as i32;
    let f = if v < i as f64 { i - 1 } else { i };
    f >> 4
}

/// THE verdict core (also the batch oracle under cargo test).
///
/// Wire contract (java flushPending): `sec_idx` carries the DISTINCT hull
/// sections in FIRST-OCCURRENCE order of the per-record nested x/y/z
/// enumeration (java's secDedup: a section shared by several records is
/// wired once, at the position where the earliest record's enumeration
/// reached it); `flags` is parallel to `sec_idx` (0 = hasOnlyAir, 1 =
/// non-air blocks-or-fluids, 2 = unknown).
///
/// The core therefore REPLAYS java's enumeration deterministically from the
/// metaI/metaD wire geometry and cross-validates the distinct sequence
/// bit-in-byte:
///
/// For each of the `n` records:
///   1. Independently re-derive the section floors from the metaD box:
///      X/Z floors must match the passed floors EXACTLY (no clamping on
///      those axes); Y floors must equal the geometry floors clamped into
///      [min_sec_y, max_sec_y-1]. Any mismatch = a java-side index bug →
///      ERR_FLOOR (java will disarm permanently).
///   2. The hull product must be ≤ 64 sections (java's HULL_SEC_MAX wire
///      promise) → ERR_COUNT.
///   3. Enumerate the hull nested x/y/z exactly like java's flush, with the
///      same global dedup: on a FIRST occurrence the next wire triple must
///      match this cell EXACTLY (ERR_TRIPLE), its flag must be ≤ 2
///      (ERR_FLAG) and the distinct count must stay ≤ cap (ERR_COUNT).
///   4. Combine flags bit-in-byte: out[i] = 0 iff ALL sections of the
///      record's hull carry flag 0 (hasOnlyAir). Flag 1 (non-air: blocks
///      OR fluids — nonEmptyBlockCount counts both) and 2 (unknown) both
///      yield verdict 1 = vanilla (fluid effects preserved by construction).
///   5. After the replay the distinct count must EQUAL the wire triple
///      count (ERR_COUNT) — no phantom/missing sections tolerated.
///
/// Returns 0 on success (out filled), negative ERR_* otherwise (out
/// untouched — the caller treats any negative as permanent disarm).
pub fn inside_batch_core(
    n: usize,
    min_sec_y: i32,
    max_sec_y: i32,
    meta_i: &[i32],
    meta_d: &[f64],
    sec_idx: &[i32],
    flags: &[i32],
    out: &mut [u8],
) -> i32 {
    if min_sec_y >= max_sec_y {
        return ERR_FLOOR;
    }
    if meta_i.len() < 7 * n || meta_d.len() < 6 * n || out.len() < n {
        return ERR_LEN;
    }
    if sec_idx.len() % 3 != 0 || flags.len() < sec_idx.len() / 3 {
        return ERR_LEN;
    }
    let cap_sections = sec_idx.len() / 3;
    let mut dedup: std::collections::HashMap<(i32, i32, i32), usize> =
        std::collections::HashMap::with_capacity(cap_sections);
    for i in 0..n {
        let mi = 7 * i;
        let md = 6 * i;
        let x0 = meta_i[mi + 1];
        let y0 = meta_i[mi + 2];
        let z0 = meta_i[mi + 3];
        let x1 = meta_i[mi + 4];
        let y1 = meta_i[mi + 5];
        let z1 = meta_i[mi + 6];
        // Independent re-derivation from the wire geometry.
        let bx0 = fl_sec(meta_d[md]);
        let by0 = fl_sec(meta_d[md + 1]);
        let bz0 = fl_sec(meta_d[md + 2]);
        let bx1 = fl_sec(meta_d[md + 3]);
        let by1 = fl_sec(meta_d[md + 4]);
        let bz1 = fl_sec(meta_d[md + 5]);
        if x0 != bx0 || x1 != bx1 || z0 != bz0 || z1 != bz1 {
            return ERR_FLOOR; // X/Z: no clamp — must be exact
        }
        if y0 != by0.max(min_sec_y) || y1 != by1.min(max_sec_y - 1) {
            return ERR_FLOOR; // Y: exact clamp math (max up / min down)
        }
        if y0 > y1 {
            return ERR_FLOOR; // java never wires empty (out-of-world) hulls
        }
        let prod = (x1 - x0 + 1) as i64 * (y1 - y0 + 1) as i64 * (z1 - z0 + 1) as i64;
        if prod > HULL_SEC_MAX {
            return ERR_COUNT;
        }
        let mut all_air = true;
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                for cz in z0..=z1 {
                    let dist = match dedup.get(&(cx, cy, cz)) {
                        Some(&d) => d,
                        None => {
                            let d = dedup.len();
                            if d >= cap_sections {
                                return ERR_COUNT; // more distinct than the wire carries
                            }
                            let t = d * 3;
                            if sec_idx[t] != cx || sec_idx[t + 1] != cy || sec_idx[t + 2] != cz {
                                return ERR_TRIPLE;
                            }
                            let f = flags[d];
                            if f > 2 {
                                return ERR_FLAG;
                            }
                            dedup.insert((cx, cy, cz), d);
                            d
                        }
                    };
                    if flags[dist] != 0 {
                        all_air = false;
                    }
                }
            }
        }
        out[i] = if all_air { 0 } else { 1 };
    }
    if dedup.len() != cap_sections {
        return ERR_COUNT; // wire carries sections no record enumerates
    }
    0
}

/// JNI surface of [`inside_batch_core`] (extern "system", Region-API copies,
/// fail-closed on null/short arrays).
///
/// # Safety
/// Called via RegisterNatives from the armed bridge only.
#[allow(clippy::too_many_arguments)]
pub unsafe extern "system" fn inside_batch_tick(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    min_sec_y: jni::jint,
    max_sec_y: jni::jint,
    meta_i: jni::jintArray,
    meta_d: jni::jdoubleArray,
    sec_idx: jni::jintArray,
    flags: jni::jintArray,
    out: jni::jbyteArray,
) -> jni::jint {
    if !lever_mode() {
        return ERR_LEN;
    }
    if env.is_null()
        || meta_i.is_null()
        || meta_d.is_null()
        || sec_idx.is_null()
        || flags.is_null()
        || out.is_null()
    {
        return ERR_LEN;
    }
    if n < 0 {
        return ERR_LEN;
    }
    let vt = unsafe { &**env };
    let ilen = unsafe { (vt.GetArrayLength)(env, meta_i) };
    let dlen = unsafe { (vt.GetArrayLength)(env, meta_d) };
    let slen = unsafe { (vt.GetArrayLength)(env, sec_idx) };
    let flen = unsafe { (vt.GetArrayLength)(env, flags) };
    let olen = unsafe { (vt.GetArrayLength)(env, out) };
    if ilen < 0 || dlen < 0 || slen < 0 || flen < 0 || olen < 0 {
        return ERR_LEN;
    }
    let nu = n as usize;
    if (ilen as usize) < 7 * nu || (dlen as usize) < 6 * nu || (olen as usize) < nu {
        return ERR_LEN;
    }
    let mut ibuf: Vec<i32> = vec![0; ilen as usize];
    let mut dbuf: Vec<f64> = vec![0.0; dlen as usize];
    let mut sbuf: Vec<i32> = vec![0; slen as usize];
    let mut fbuf: Vec<i32> = vec![0; flen as usize];
    unsafe {
        (vt.GetIntArrayRegion)(env, meta_i, 0, ilen, ibuf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, meta_d, 0, dlen, dbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, sec_idx, 0, slen, sbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, flags, 0, flen, fbuf.as_mut_ptr());
    }
    let mut obuf: Vec<u8> = vec![1; olen as usize];
    let rc = inside_batch_core(
        nu,
        min_sec_y,
        max_sec_y,
        &ibuf,
        &dbuf,
        &sbuf,
        &fbuf,
        &mut obuf,
    );
    if rc == 0 {
        unsafe { (vt.SetByteArrayRegion)(env, out, 0, n, obuf.as_ptr() as *const i8) };
    }
    rc
}

// ---------------------------------------------------------------------------
// Batch oracle tests (cargo test --lib): N entities × mixed blocks/fluids
// vs vanilla math. The vanilla truth: verdict 0 ⟺ EVERY cell of
// BlockPos.betweenClosed(box) is air ⟺ EVERY section of the hull carries
// hasOnlyAir. The oracle recomputes the expectation CELL-WISE from a
// synthetic per-section content map (blocks and fluids both = non-air)
// and requires the core to agree bit-in-byte.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod oracle {
    use super::*;

    /// Deterministic LCG (java-RngOps style, fixed seed) for reproducibility.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0 >> 11
        }
        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    const MIN_SEC_Y: i32 = -4; // world sections [-4, 8) — like a 1.21 world
    const MAX_SEC_Y: i32 = 8;

    /// Synthetic section content: true = all-air, false = contains blocks
    /// OR fluids (the flag domain cannot distinguish them — both non-air).
    struct World {
        air: std::collections::HashSet<(i32, i32, i32)>,
    }

    impl World {
        fn flag(&self, cx: i32, cy: i32, cz: i32) -> i32 {
            if self.air.contains(&(cx, cy, cz)) {
                0
            } else {
                1 // blocks or fluids — non-air either way
            }
        }
        fn contains(&self, cx: i32, cy: i32, cz: i32) -> bool {
            self.air.contains(&(cx, cy, cz))
        }
    }

    fn floori(v: f64) -> i32 {
        let i = v as i32;
        if v < i as f64 {
            i - 1
        } else {
            i
        }
    }

    #[test]
    fn oracle_mixed_blocks_and_fluids_matches_vanilla_cell_math() {
        let mut rng = Lcg(0x411B);
        let n = 256usize;
        let mut world = World {
            air: std::collections::HashSet::new(),
        };
        // Mixed scenery: mostly air, patches of solid blocks and fluid
        // (water/lava) sections — the flag domain treats them identically.
        for cx in -8..=8 {
            for cy in MIN_SEC_Y..MAX_SEC_Y {
                for cz in -8..=8 {
                    let r = rng.below(100);
                    world.air.insert((cx, cy, cz));
                    if r < 15 {
                        world.air.remove(&(cx, cy, cz)); // solid blocks
                    } else if r < 22 {
                        world.air.remove(&(cx, cy, cz)); // fluid-filled
                    }
                }
            }
        }

        let mut meta_i: Vec<i32> = Vec::new();
        let mut meta_d: Vec<f64> = Vec::new();
        let mut boxes: Vec<[f64; 6]> = Vec::new();
        let mut expected: Vec<u8> = Vec::new();
        for _ in 0..n {
            // Entity box: random center + small extents (stationary item/mob
            // scale), snapped inside the synthetic chunk range.
            let cxx = (rng.below(13)) as f64 - 6.5;
            let cyy = (rng.below(10)) as f64 - 5.0;
            let czz = (rng.below(13)) as f64 - 6.5;
            let hw = 0.3 + (rng.below(100)) as f64 / 200.0; // half width
            let hh = 0.3 + (rng.below(100)) as f64 / 200.0; // half height
            let deflated = 1.0e-5; // hull ≈ box here; deflate is cosmetic
            let b = [
                cxx - hw + deflated,
                cyy - hh + deflated,
                czz - hw + deflated,
                cxx + hw - deflated,
                cyy + hh - deflated,
                czz + hw - deflated,
            ];
            // Java-side floors (incl. Y clamp).
            let x0 = floori(b[0]) >> 4;
            let y0 = (floori(b[1]) >> 4).max(MIN_SEC_Y);
            let z0 = floori(b[2]) >> 4;
            let x1 = floori(b[3]) >> 4;
            let y1 = (floori(b[4]) >> 4).min(MAX_SEC_Y - 1);
            let z1 = floori(b[5]) >> 4;
            assert!(y0 <= y1, "hull must not be empty in this scenario");

            // Wire arrays (java enumeration order, running dedup).
            meta_i.extend_from_slice(&[0, x0, y0, z0, x1, y1, z1]);
            meta_d.extend_from_slice(&b);
            boxes.push(b);

            // Vanilla truth, CELL-wise: every cell of betweenClosed(box)
            // must be air for verdict 0. Cells outside the world height are
            // void-air (vanilla getBlockState) — the clamp excludes them.
            let mut all_air = true;
            for cx in x0..=x1 {
                for cy in y0..=y1 {
                    for cz in z0..=z1 {
                        if !world.contains(cx, cy, cz) {
                            all_air = false;
                        }
                    }
                }
            }
            expected.push(if all_air { 0 } else { 1 });
        }

        // Distinct-section resolution (java flush side).
        let mut dedup: std::collections::HashMap<(i32, i32, i32), usize> =
            std::collections::HashMap::new();
        let mut sec_idx: Vec<i32> = Vec::new();
        let mut flags: Vec<i32> = Vec::new();
        for (i, b) in boxes.iter().enumerate() {
            let x0 = floori(b[0]) >> 4;
            let y0 = (floori(b[1]) >> 4).max(MIN_SEC_Y);
            let z0 = floori(b[2]) >> 4;
            let x1 = floori(b[3]) >> 4;
            let y1 = (floori(b[4]) >> 4).min(MAX_SEC_Y - 1);
            let z1 = floori(b[5]) >> 4;
            let mut key_and_id = |c: (i32, i32, i32), sec_idx: &mut Vec<i32>, flags: &mut Vec<i32>| {
                *dedup.entry(c).or_insert_with(|| {
                    sec_idx.extend_from_slice(&[c.0, c.1, c.2]);
                    flags.push(world.flag(c.0, c.1, c.2));
                    flags.len() - 1
                });
                let _ = i;
            };
            for cx in x0..=x1 {
                for cy in y0..=y1 {
                    for cz in z0..=z1 {
                        key_and_id((cx, cy, cz), &mut sec_idx, &mut flags);
                    }
                }
            }
        }

        let mut out = vec![1u8; n];
        let rc = inside_batch_core(n, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &sec_idx, &flags, &mut out);
        assert_eq!(rc, 0, "structural validation must pass on the oracle wire");
        assert_eq!(
            out,
            expected,
            "core verdicts must match vanilla cell-wise math bit-in-byte"
        );
        // The scenario must actually exercise both verdict classes.
        assert!(out.iter().any(|&v| v == 0), "some all-air records expected");
        assert!(out.iter().any(|&v| v == 1), "some non-air records expected");
    }

    #[test]
    fn err_floor_mismatch() {
        // metaD geometry says section 1; metaI claims 0 → ERR_FLOOR.
        let meta_i = [42, 0, 0, 0, 0, 0, 0];
        let meta_d = [
            16.0, 0.0, 0.0, 31.9, 15.9, 15.9, // floors: (1,0,0)-(1,0,0)
        ];
        let mut out = [1u8; 1];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &[1, 0, 0], &[0], &mut out);
        assert_eq!(rc, ERR_FLOOR);
    }

    #[test]
    fn err_y_clamp_mismatch() {
        // Box crossing the top of the world: raw y floors 120>>4=7 .. 129>>4=8;
        // java must clamp y1 DOWN to maxSecY-1 = 7 (hull = section y=7 only).
        let meta_d = [0.0, 120.0, 0.0, 1.0, 129.9, 1.0];
        let mut out = [1u8; 1];
        let good = [42, 0, 7, 0, 0, 7, 0]; // clamped to [-4, 7]
        let rc = inside_batch_core(1, -4, 8, &good, &meta_d, &[0, 7, 0], &[0], &mut out);
        assert_eq!(rc, 0, "correct clamp accepted");
        assert_eq!(out[0], 0, "flagged all-air → verdict 0");
        let bad_unclamped = [42, 0, 7, 0, 0, 8, 0]; // y1 left at raw floor — java bug class
        let rc = inside_batch_core(
            1,
            -4,
            8,
            &bad_unclamped,
            &meta_d,
            &[0, 7, 0, 0, 8, 0],
            &[0, 0],
            &mut out,
        );
        assert_eq!(rc, ERR_FLOOR, "unclamped Y must be rejected");
        let bad_wrong_floor = [42, 0, 6, 0, 0, 7, 0]; // y0 not derived from geometry
        let rc = inside_batch_core(
            1,
            -4,
            8,
            &bad_wrong_floor,
            &meta_d,
            &[0, 6, 0, 0, 7, 0],
            &[0, 0],
            &mut out,
        );
        assert_eq!(rc, ERR_FLOOR, "wrong Y floor must be rejected");
        // Bottom clamp: box [-70.0 .. -55.0] → raw floors -5 .. -4; java clamps
        // y0 UP to minSecY = -4 (hull collapses to section y=-4, void-air below).
        let meta_d2 = [0.0, -70.0, 0.0, 1.0, -55.0, 1.0];
        let good2 = [42, 0, -4, 0, 0, -4, 0];
        let rc = inside_batch_core(1, -4, 8, &good2, &meta_d2, &[0, -4, 0], &[0], &mut out);
        assert_eq!(rc, 0, "bottom clamp accepted");
        assert_eq!(out[0], 0);
    }

    #[test]
    fn err_hull_too_big() {
        // 5×5×5 = 125 sections > 64 — java promises never to wire this; the
        // metaD geometry must agree with the claimed floors (0..4 per axis:
        // floor(79.9) = 79, 79>>4 = 4) so the rejection lands on the count,
        // not on the floor cross-check.
        let meta_i = [42, 0, 0, 0, 4, 4, 4];
        let meta_d = [0.0, 0.0, 0.0, 79.9, 79.9, 79.9];
        let mut out = [1u8; 1];
        let sec = vec![0i32; 3 * 128];
        let flags = vec![0i32; 128];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &sec, &flags, &mut out);
        assert_eq!(rc, ERR_COUNT);
    }

    #[test]
    fn err_triple_mismatch() {
        // Java enumerates x/y/z nested; a swapped triple must be rejected.
        let meta_i = [42, 0, 0, 0, 1, 0, 0]; // 2 sections: (0,0,0),(1,0,0)
        let meta_d = [0.0, 0.0, 0.0, 31.9, 1.0, 1.0];
        let mut out = [1u8; 1];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &[1, 0, 0, 0, 0, 0], &[0, 0], &mut out);
        assert_eq!(rc, ERR_TRIPLE);
    }

    #[test]
    fn err_bad_flag_domain() {
        let meta_i = [42, 0, 0, 0, 0, 0, 0];
        let meta_d = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut out = [1u8; 1];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &[0, 0, 0], &[7], &mut out);
        assert_eq!(rc, ERR_FLAG);
    }

    #[test]
    fn err_short_arrays() {
        let mut out = [1u8; 1];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &[42], &[0.0; 6], &[0, 0, 0], &[0], &mut out);
        assert_eq!(rc, ERR_LEN);
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &[42, 0, 0, 0, 0, 0, 0], &[0.0; 5], &[0, 0, 0], &[0], &mut out);
        assert_eq!(rc, ERR_LEN);
    }

    #[test]
    fn unknown_section_yields_vanilla() {
        // Flag 2 (unloaded chunk) → verdict 1 (vanilla), not 0.
        let meta_i = [42, 0, 0, 0, 0, 0, 0];
        let meta_d = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut out = [1u8; 1];
        let rc = inside_batch_core(1, MIN_SEC_Y, MAX_SEC_Y, &meta_i, &meta_d, &[0, 0, 0], &[2], &mut out);
        assert_eq!(rc, 0);
        assert_eq!(out[0], 1, "unknown section must stay vanilla");
    }
}
