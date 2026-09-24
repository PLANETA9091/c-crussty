//! Runtime wiring for the ITEMS-BATCH rest-plane lever (TASK-446-B, vector
//! cmp446_items; see RESEARCH-B-446-ITEMS.md + itemsbatch/.../ItemBatchOps.java).
//!
//! ARCHITECTURE (law-6 whole-subsystem): the kernel `ItemEntity.tick()V` body
//! is whole-body replaced (item_merge.rs ReplaceBody precedent) by a single
//! invokestatic into `ItemBatchOps.tick(ItemEntity)` — a bridge defined into
//! the kernel loader (same package as Entity). Per tick per region thread the
//! bridge drains the PREVIOUS tick's snapshot batch with ONE bulk native call
//! `planeDecide([DI[I[J)I` (impl below) — decisions are 1-tick-stale
//! classification FULL/REST over stride-12 doubles
//! `{id,x,y,z,vx,vy,vz,age,pickupDelay,tickCount,flags,0}`:
//!   - REST -> vanilla `e.inactiveTick()` (counters + full despawn flow) +
//!     vanilla merge cadence (tickCount % 40 == 0 -> private
//!     mergeWithNeighbours via MethodHandle — vanilla scan/radii/walls).
//!   - FULL -> byte-parity replica of the vanilla tick body (purpur-1.21.10
//!     offsets 0..588, javap-verified in RESEARCH-B-446-ITEMS.md).
//! The heavy per-tick world-voxel scans (fluid 32% / inside-blocks 28% /
//! collision 25% of the 29.5% items lane) therefore run ONLY on full-path
//! items — settled items cost one vanilla inactiveTick + one snapshot write.
//!
//! RUST PLANE: sharded `Mutex<HashMap<id, (rest_seq, last_epoch)>>` (64
//! shards, one uncontended lock per item sighting). Decision per entry:
//!   - eligibility = items_subsys2 planeResting replica: onGround &&
//!     pickupDelay in {0, 32767} && !inWater && !inLava && !portal &&
//!     !removed && horizontalDistanceSqr <= 9.999999747378752E-6 (the EXACT
//!     vanilla move-gate constant, ItemEntity.tick offsets 226..270).
//!   - ineligible -> FULL + rest_seq reset (any move/push/fluid re-converges
//!     the next tick through the snapshot's velocity/flags).
//!   - eligible -> rest_seq += 1; FULL on the landing tick (rest_seq == 1)
//!     and on the faithful recheck (rest_seq % 32 == 0) — the SAME accepted
//!     items_subsys2 REST_PLANE deviation (fluid/fire/inside onset latency
//!     <= 32 ticks); REST otherwise.
//! Ids are monotonic and never reused (vanilla Entity entity-id counter), so
//! stale entries are swept lazily by last_epoch (GC every 1024 epochs, keep
//! window 4096 epochs) — an item re-observed after a sweep just re-lands
//! (one FULL tick), never a wrong classification.
//!
//! GATE: env `CRUSSTY_LEVER_FLAG == "cmp446_items"` (STRICT eq; the java
//! ENABLED gate is baked at compile into the same string — double gate).
//! Empty/foreign flag = no hook registered, nothing defined or retransformed,
//! the module is byte-indistinguishable from the pre-TASK-446 plugin.
//!
//! FAIL-CLOSED: selfTest (java, drives probe + planeDecide with synthetic
//! snapshots) must pass BEFORE the hook arms (READY flips after selfTest);
//! MH resolve failure / native probe mismatch / planeDecide rc < 0 /
//! retransform failure -> the bridge executes the vanilla-replica path
//! forever (or nothing ever arms). Unknown ids classify FULL (fail-open
//! direction — a full tick is vanilla).

use jvmti_bindings::jni;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

const ITEM_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_NAME: &str = "net/minecraft/world/entity/ItemBatchOps";
const METHOD_NAME: &str = "tick";
const METHOD_DESC: &str = "()V";

const OPS_BYTES: &[u8] =
    include_bytes!("../itemsbatch/build/net/minecraft/world/entity/ItemBatchOps.class");

/// Whole-body replacement spec: the kernel method's only argument slot (this)
/// passes straight through to the static bridge.
const BRIDGE_DESC: &str = "(Lnet/minecraft/world/entity/item/ItemEntity;)V";

/// env gate per the round-396 lever protocol (STRICT eq — no composite family:
/// the items rest-plane is a standalone subsystem leg this round).
fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok("cmp446_items"))
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel ItemEntity classloader, captured at activation.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of the class (byte hook
/// while READY=false), else via no-op retransform / resource stream.
static ORIG_BYTES: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
    std::sync::OnceLock::new();
/// Patched bytecode cache (TASK-26/C5 pattern): Arc<[u8]> + major parsed once.
#[derive(Clone)]
struct PatchCache {
    bytes: std::sync::Arc<[u8]>,
    major: u16,
}
static PATCH_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>> =
    std::sync::OnceLock::new();
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn orig_lock() -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock() -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE.get_or_init(|| std::sync::Mutex::new(None))
}

// ---------------------------------------------------------------------------
// Rust rest-plane state
// ---------------------------------------------------------------------------

const PROBE_MAGIC: i32 = 0x1D46;
const SHARDS: usize = 64;
const FULL: i32 = 0;
const REST: i32 = 1;
/// Faithful recheck period (vanilla-equivalent full tick every 32nd resting
/// tick; items_subsys2 REST_PLANE-identical accepted deviation).
const RECHECK: i64 = 32;
/// Vanilla move-gate constant (ItemEntity.tick offsets 226..270):
/// horizontalDistanceSqr() < 9.999999747378752E-6.
const REST_HDSQR: f64 = 9.999_999_747_378_752E-6;

/// Snapshot stride (doubles): id,x,y,z,vx,vy,vz,age,pickupDelay,tickCount,flags,0
const STRIDE: usize = 12;
const F_ON_GROUND: i32 = 1;
const F_IN_WATER: i32 = 2;
const F_IN_LAVA: i32 = 4;
const F_PORTAL: i32 = 8;
const F_REMOVED: i32 = 16;

type ItemState = (i64, i64); // (rest_seq, last_epoch)

fn shards() -> &'static [Mutex<HashMap<i32, ItemState>>; SHARDS] {
    static S: OnceLock<[Mutex<HashMap<i32, ItemState>>; SHARDS]> = OnceLock::new();
    S.get_or_init(|| std::array::from_fn(|_| Mutex::new(HashMap::new())))
}

/// Monotonic decision-epoch counter (bumped once per planeDecide call).
static EPOCH: AtomicI64 = AtomicI64::new(0);

#[inline]
fn shard_of(id: i32) -> usize {
    // Same mix as the java-side decision hash (ItemBatchOps.mix).
    let h = id.wrapping_mul(-1_640_531_527_i32); // 0x9E3779B9 as i32
    ((h ^ (h >> 16)) as usize) & (SHARDS - 1)
}

/// Eligibility predicate (items_subsys2 planeResting replica). `pd` is the
/// snapshot pickupDelay: eligible iff {0, 32767} (the vanilla decrement gate
/// would not fire).
#[inline]
fn eligible(flags: i32, hdsqr: f64, pd: i64) -> bool {
    (flags & F_ON_GROUND) != 0
        && (flags & (F_IN_WATER | F_IN_LAVA | F_PORTAL | F_REMOVED)) == 0
        && (pd == 0 || pd == 32767)
        && hdsqr <= REST_HDSQR
}

/// Batch decision core over the raw snapshot slice (pure, unit-testable).
/// Writes out[i] in {FULL, REST}; bumps stats[0]=full, stats[1]=rest,
/// stats[2]=recheck-fulls; returns 0 or a negative structural code.
fn decide_batch(snap: &[f64], out: &mut [i32], stats: &mut [i64]) -> i32 {
    let n = out.len();
    if snap.len() < n * STRIDE {
        return -1;
    }
    let epoch = EPOCH.fetch_add(1, Ordering::Relaxed) + 1;
    let mut full = 0i64;
    let mut rest = 0i64;
    let mut rechecks = 0i64;
    for i in 0..n {
        let b = i * STRIDE;
        let id = snap[b] as i64 as i32;
        let flags = snap[b + 10] as i64 as i32;
        let vx = snap[b + 4];
        let vz = snap[b + 6];
        let pd = snap[b + 8] as i64;
        let action = {
            let shard = &shards()[shard_of(id)];
            let mut g = match shard.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            match g.get_mut(&id) {
                Some(st) => {
                    if eligible(flags, vx * vx + vz * vz, pd) {
                        st.0 += 1;
                        st.1 = epoch;
                        if st.0 == 1 {
                            FULL
                        } else if st.0 % RECHECK == 0 {
                            rechecks += 1;
                            FULL
                        } else {
                            REST
                        }
                    } else {
                        st.0 = 0;
                        st.1 = epoch;
                        FULL
                    }
                }
                None => {
                    // Fresh sighting: the landing tick (eligible, seq 1) and
                    // the ineligible sight both classify FULL — never a REST
                    // without a full-tick landing first.
                    let seq = if eligible(flags, vx * vx + vz * vz, pd) { 1 } else { 0 };
                    g.insert(id, (seq, epoch));
                    FULL
                }
            }
        };
        match action {
            REST => rest += 1,
            _ => full += 1,
        }
        out[i] = action;
    }
    stats[0] = full;
    stats[1] = rest;
    stats[2] = rechecks;
    // Lazy stale sweep: every 1024 epochs, drop entries unseen for 4096
    // epochs (~1024 ticks at 4 region threads — far beyond any live item's
    // observation gap; a swept item just re-lands with one FULL tick).
    if epoch % 1024 == 0 {
        for s in shards().iter() {
            let mut g = match s.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            g.retain(|_, (_, last)| epoch - *last <= 4096);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// RegisterNatives surface (java: ItemBatchOps.planeProbe / planeDecide)
// ---------------------------------------------------------------------------

/// # Safety
/// Called by the JVM through RegisterNatives; env/clazz are live JNI pointers
/// of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn items_batch_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    PROBE_MAGIC
}

/// ONE bulk call per tick per region thread: classify the previous snapshot
/// batch. rc 0 = ok, <0 = structural break (java side disarms permanently).
///
/// # Safety
/// См. items_batch_probe.
#[no_mangle]
pub unsafe extern "system" fn items_batch_decide(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    snap: jni::jdoubleArray,
    n: jni::jint,
    out: jni::jintArray,
    stats: jni::jlongArray,
) -> jni::jint {
    if env.is_null() || snap.is_null() || out.is_null() || stats.is_null() || n < 0 {
        return -1;
    }
    let vt = unsafe { &**env };
    let snap_len = unsafe { (vt.GetArrayLength)(env, snap) };
    let out_len = unsafe { (vt.GetArrayLength)(env, out) };
    let stat_len = unsafe { (vt.GetArrayLength)(env, stats) };
    if n as i64 > out_len as i64
        || (n as i64) * (STRIDE as i64) > snap_len as i64
        || stat_len < 3
    {
        return -1;
    }
    if n == 0 {
        return 0;
    }

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, snap, std::ptr::null_mut()) };
    if pinned.is_null() {
        return -1;
    }
    let pinned_out = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned_out.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, snap, pinned, 0) };
        return -1;
    }
    // SAFETY: lengths verified above; no JNI calls inside the critical pair.
    let rc = {
        let src = unsafe { std::slice::from_raw_parts(pinned as *const jni::jdouble, (n as usize) * STRIDE) };
        let dst = unsafe { std::slice::from_raw_parts_mut(pinned_out as *mut jni::jint, n as usize) };
        let mut st = [0i64; 3];
        let rc = decide_batch(src, dst, &mut st);
        unsafe {
            (vt.ReleasePrimitiveArrayCritical)(env, out, pinned_out, 0);
            (vt.ReleasePrimitiveArrayCritical)(env, snap, pinned, 0);
        }
        let sv = [
            st[0] as jni::jlong,
            st[1] as jni::jlong,
            st[2] as jni::jlong,
        ];
        unsafe { (vt.SetLongArrayRegion)(env, stats, 0, 3, sv.as_ptr()) };
        rc
    };
    rc
}

// ---------------------------------------------------------------------------
// Wiring (item_merge.rs shape: byte hook -> quiet activation -> patch -> arm)
// ---------------------------------------------------------------------------

/// Register the byte hook (idempotent; call once from cplugin_init).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_batch: dormant (lever_flag != cmp446_items, vanilla ItemEntity.tick)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(ITEM_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] items_batch: pristine sighting {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut orig = orig_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = patch_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_batch: hook serve {} bytes (major {})",
                cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                cached.as_ref().map(|c| c.major).unwrap_or(0)
            );
        }
        cached.map(|c| c.bytes.to_vec())
    });
}

/// Background activation: wait for ItemEntity, define ItemBatchOps into the
/// kernel loader, register the plane natives, run the java selfTest (BEFORE
/// arming — fail-closed), compute the whole-body patch, flip READY,
/// retransform, ARM marker.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(ITEM_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] items_batch: {ITEM_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                && (forced_attempts < 12 || forced_attempts.is_multiple_of(12))
            {
                forced_attempts += 1;
                eprintln!(
                    "[crussty-plugin] items_batch: forcing kernel load of {ITEM_CLASS} (attempt {forced_attempts})"
                );
                crate::improved_noise::force_load_kernel_class(ITEM_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ITEM_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] items_batch: boot marker not seen, hook stays dormant");
            return;
        }
        // TASK-80 crash lesson: settle after Done before any define/retransform.
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] items_batch: server booted, defining ItemBatchOps into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] items_batch: {OPS_NAME} is class major {major} but this JVM supports up to {jvm_major} — rebuild itemsbatch/ via scripts/build_itemsbatch_ops.sh; hook stays dormant"
            );
            return;
        }

        // Phase 1: define the bridge into the kernel loader + register the
        // plane natives + selfTest on the LOCAL ref (queryplane lesson:
        // find_class filters non-INITIALIZED classes, just-defined bridge
        // would be filtered). selfTest false -> NO arm (vanilla bit-in-bit).
        let loader_and_selftest = cplug_sdk::jni_util::with_attached(|env| {
            let Some(item_cls) = cplug_sdk::classes::find_class(ITEM_CLASS) else {
                return (0usize, false);
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return (0usize, false);
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(item_cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return (0usize, false);
            };
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return (0usize, false);
            }
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);

            let mut ok = true;
            let ops_local = env.define_class(OPS_NAME, gref, OPS_BYTES);
            let Some(ops) = ops_local else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] items_batch: define_class({OPS_NAME}) failed");
                ok = false;
                return (0usize, false);
            };
            eprintln!("[crussty-plugin] items_batch: defined {OPS_NAME} in kernel loader");

            let c_probe = std::ffi::CString::new("planeProbe").expect("no NUL");
            let c_decide = std::ffi::CString::new("planeDecide").expect("no NUL");
            let s_probe = std::ffi::CString::new("()I").expect("no NUL");
            let s_decide = std::ffi::CString::new("([DI[I[J)I").expect("no NUL");
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: c_probe.as_ptr(),
                    signature: s_probe.as_ptr(),
                    fnPtr: items_batch_probe as *const std::ffi::c_void
                        as *mut std::ffi::c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: c_decide.as_ptr(),
                    signature: s_decide.as_ptr(),
                    fnPtr: items_batch_decide as *const std::ffi::c_void
                        as *mut std::ffi::c_void,
                },
            ];
            let ops_raw = ops as jvmti_bindings::jni::jclass;
            if let Err(code) = env.register_natives(ops, &natives) {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] items_batch: register_natives failed (code {code}) — hook stays dormant"
                );
                ok = false;
            }

            // selfTest BEFORE retransform (fail-closed): drives planeProbe +
            // planeDecide with synthetic snapshots through the REAL natives.
            let mut selftest = false;
            if ok {
                match env.get_static_method_id(ops_raw, "selfTest", "()Z") {
                    None => {
                        crate::clear_exception(env);
                        eprintln!("[crussty-plugin] items_batch: selfTest method not found");
                    }
                    Some(mid) => {
                        let raw = env.raw();
                        // SAFETY: ops/mid are live; the call has no arguments.
                        selftest = unsafe {
                            let fn_table = &(**raw);
                            (fn_table.CallStaticBooleanMethodA)(
                                raw,
                                ops_raw,
                                mid,
                                [].as_ptr(),
                            )
                        } != 0;
                    }
                }
            }
            let gops = env.new_global_ref(ops);
            env.delete_local_ref(ops);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            (gops as usize, ok && selftest)
        });
        let (ops_gref, selftest_ok) = loader_and_selftest.unwrap_or((0usize, false));
        if !selftest_ok {
            eprintln!(
                "[crussty-plugin] items_batch: selfTest FAILED/skipped — hook stays dormant (vanilla ItemEntity.tick)"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] items_batch: selfTest passed (probe magic + natives live, MethodHandles resolved in kernel loader)"
        );
        let _ = ops_gref; // global ref held for the process lifetime (marker provenance)

        // Capture pristine bytes if the class predates the hook (fast boots).
        if orig_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] items_batch: class predates hook, capturing current bytes via no-op retransform"
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(ITEM_CLASS);
                eprintln!(
                    "[crussty-plugin] items_batch: capture retransform rc={rc} (attempt {attempt})"
                );
                captured = orig_lock()
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .is_some();
                if captured {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !captured {
                eprintln!(
                    "[crussty-plugin] items_batch: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                match resource_stream_capture() {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] items_batch: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
                            Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] items_batch: resource-stream capture failed too, hook stays dormant"
                        );
                        return;
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] items_batch: no original bytes captured even after retransform, hook stays dormant"
            );
            return;
        };

        // Phase 2: compute the whole-body patch on the quiet thread.
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            eprintln!("[crussty-plugin] items_batch: no kernel loader captured");
            return;
        }
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            let spec = cplug_sdk::asm::ReplaceBody {
                method_name: METHOD_NAME,
                method_desc: METHOD_DESC,
                bridge_owner: OPS_NAME,
                bridge_name: METHOD_NAME,
                bridge_desc: BRIDGE_DESC,
                args: &[cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }],
            };
            cplug_sdk::asm::replace_body(
                env,
                loader as jvmti_bindings::jni::jobject,
                &original,
                &spec,
            )
        })
        .flatten();
        let Some(patched) = patched else {
            eprintln!(
                "[crussty-plugin] items_batch: patch computation failed, hook stays dormant"
            );
            return;
        };
        let patch_major =
            crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] items_batch: computed patch for {METHOD_NAME}{METHOD_DESC} ({} -> {} bytes), orig major {} patch major {}",
            original.len(),
            patched.len(),
            crate::improved_noise::class_version(&original).map(|(m, _)| m).unwrap_or(0),
            patch_major
        );
        *patch_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(PatchCache {
            bytes: std::sync::Arc::from(patched),
            major: patch_major,
        });

        // Phase 3: a SINGLE retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_NAME, METHOD_NAME, "cmp446_items items rest-plane v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(ITEM_CLASS);
        // ГРОМКИЙ ARM-МАРКЕР (канон ARM-пруфа): без этой строки нога не-armed.
        eprintln!(
            "[crussty-plugin] cmp446_items: ARMED rest-plane shards=64 recheck=1/32 bulk-jni=1/tick/thread selfTest=true (retransform rc={rc})"
        );
    });
}

/// Last-resort pristine capture: the kernel loader's resource stream yields
/// the original class file bytes with no JVMTI event delivery involved.
fn resource_stream_capture() -> Option<Vec<u8>> {
    cplug_sdk::jni_util::with_attached(|env| {
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            return None;
        }
        let loader_cls = env.find_class("java/lang/ClassLoader")?;
        let garm = env.get_method_id(
            loader_cls,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
        )?;
        let res_name = env.new_string_utf(&format!("{ITEM_CLASS}.class"))?;
        let stream = env.call_object_method(
            loader as jvmti_bindings::jni::jobject,
            garm,
            &[jvmti_bindings::jni::jvalue { l: res_name }],
        );
        if stream.is_null() {
            crate::clear_exception(env);
            env.delete_local_ref(res_name);
            env.delete_local_ref(loader_cls);
            return None;
        }
        let in_cls = env.find_class("java/io/InputStream")?;
        let rab = env.get_method_id(in_cls, "readAllBytes", "()[B")?;
        let arr = env.call_object_method(stream, rab, &[]);
        let out = if arr.is_null() {
            crate::clear_exception(env);
            None
        } else {
            let jarr = arr as jvmti_bindings::jni::jbyteArray;
            let len = env.get_array_length(arr as jvmti_bindings::jni::jarray);
            let mut signed = vec![0i8; len as usize];
            env.get_byte_array_region(jarr, 0, len, &mut signed);
            Some(signed.iter().map(|&b| b as u8).collect::<Vec<u8>>())
        };
        env.delete_local_ref(arr);
        env.delete_local_ref(stream);
        env.delete_local_ref(res_name);
        env.delete_local_ref(loader_cls);
        out
    })
    .flatten()
}

// ---------------------------------------------------------------------------
// Tests (pure decision core; the JNI wrapper is a thin pin/copy shell)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn mk(snap: &mut Vec<f64>, id: i32, vx: f64, pd: i64, flags: i32) {
        snap.push(id as f64);
        snap.extend_from_slice(&[10.5, 64.0, 10.5]);
        snap.extend_from_slice(&[vx, 0.0, 0.0]);
        snap.push(100.0); // age
        snap.push(pd as f64);
        snap.push(5000.0); // tickCount
        snap.push(flags as f64);
        snap.push(0.0);
    }

    /// Runs `calls` consecutive tick sightings of ONE id; stats are PER-CALL
    /// in the plane contract (java EFFECT log), so decisions are accumulated
    /// here from out[0] instead.
    fn run_one(id: i32, calls: usize, vx: f64, pd: i64, flags: i32) -> (i64, i64, i64) {
        let mut snap = Vec::new();
        mk(&mut snap, id, vx, pd, flags);
        let mut out = [FULL; 1];
        let mut stats = [0i64; 3];
        let (mut fulls, mut rests, mut rechecks) = (0i64, 0i64, 0i64);
        for _ in 0..calls {
            assert_eq!(decide_batch(&snap, &mut out, &mut stats), 0);
            match out[0] {
                REST => rests += 1,
                _ => fulls += 1,
            }
            rechecks += stats[2]; // recheck-fulls marked by the plane itself
        }
        (fulls, rests, rechecks)
    }

    /// Landing tick FULL, then REST with a faithful FULL recheck every 32nd
    /// resting tick: 70 calls -> fulls {seq 1, 32, 64} = 3, rests = 67
    /// (matches the java selfTest contract rest>0 && full>0 && total==70).
    #[test]
    fn settled_item_lands_then_rests_with_1_of_32_rechecks() {
        let (full, rest, _recheck) = run_one(0x5E17_0000, 70, 0.0, 0, F_ON_GROUND);
        assert_eq!(full, 3, "fulls at rest_seq 1, 32, 64");
        assert_eq!(rest, 67);
        assert_eq!(full + rest, 70);
    }

    #[test]
    fn pickup_delay_32767_is_rest_eligible() {
        // pd == 32767 (infinite marker) must be treated like pd == 0.
        let (full, rest, _) = run_one(0x5E17_0001, 33, 0.0, 32767, F_ON_GROUND);
        assert_eq!(full, 2); // seq 1, 32
        assert_eq!(rest, 31);
    }

    #[test]
    fn ineligible_snapshots_always_full_and_reset_rest_seq() {
        // in-water
        let (full, rest, _) = run_one(0x5E17_0002, 5, 0.0, 0, F_ON_GROUND | F_IN_WATER);
        assert_eq!((full, rest), (5, 0));
        // in-lava
        let (full, rest, _) = run_one(0x5E17_0003, 5, 0.0, 0, F_ON_GROUND | F_IN_LAVA);
        assert_eq!((full, rest), (5, 0));
        // portal
        let (full, rest, _) = run_one(0x5E17_0004, 5, 0.0, 0, F_ON_GROUND | F_PORTAL);
        assert_eq!((full, rest), (5, 0));
        // removed
        let (full, rest, _) = run_one(0x5E17_0005, 5, 0.0, 0, F_ON_GROUND | F_REMOVED);
        assert_eq!((full, rest), (5, 0));
        // not on ground (falling)
        let (full, rest, _) = run_one(0x5E17_0006, 5, 0.0, 0, 0);
        assert_eq!((full, rest), (5, 0));
        // moving (hdSqr = 1e-4 > 1e-5 vanilla gate)
        let (full, rest, _) = run_one(0x5E17_0007, 5, 0.01, 0, F_ON_GROUND);
        assert_eq!((full, rest), (5, 0));
        // pickup-delayed fresh drop
        let (full, rest, _) = run_one(0x5E17_0008, 5, 0.0, 10, F_ON_GROUND);
        assert_eq!((full, rest), (5, 0));
    }

    /// A pushed item (velocity above the gate) converges back to REST only
    /// through a fresh landing tick (rest_seq reset by the ineligible sight).
    #[test]
    fn push_convergence_restarts_landing() {
        let id = 0x5E17_0001i32;
        let mut snap = Vec::new();
        mk(&mut snap, id, 0.0, 0, F_ON_GROUND);
        let mut out = [FULL; 1];
        let mut stats = [0i64; 3];
        // 35 resting ticks -> REST from seq 2..35 (fulls at 1 and 32).
        for _ in 0..35 {
            decide_batch(&snap, &mut out, &mut stats);
        }
        assert_eq!(out[0], REST);
        // push: velocity above gate -> FULL, rest_seq reset
        snap[4] = 0.2;
        decide_batch(&snap, &mut out, &mut stats);
        assert_eq!(out[0], FULL);
        // next tick the push velocity is still in the snapshot -> FULL
        decide_batch(&snap, &mut out, &mut stats);
        assert_eq!(out[0], FULL);
        // settled again: landing tick FULL first
        snap[4] = 0.0;
        decide_batch(&snap, &mut out, &mut stats);
        assert_eq!(out[0], FULL, "landing tick after push must be FULL");
        decide_batch(&snap, &mut out, &mut stats);
        assert_eq!(out[0], REST, "then REST again");
    }

    #[test]
    fn structural_errors_are_negative() {
        let mut snap = Vec::new();
        mk(&mut snap, 1, 0.0, 0, F_ON_GROUND);
        let mut out = [FULL; 1];
        let mut stats = [0i64; 3];
        // snapshot shorter than n*STRIDE
        assert!(decide_batch(&snap[..5], &mut out, &mut stats) < 0);
        // full-shape call ok
        assert_eq!(decide_batch(&snap, &mut out, &mut stats), 0);
        assert_eq!(stats[0], 1);
        // empty batch is legal (n == 0, out empty)
        let mut empty_out: [i32; 0] = [];
        assert_eq!(decide_batch(&snap, &mut empty_out, &mut stats), 0);
    }

    /// Swept ids re-land with a FULL tick (never a wrong REST classification).
    #[test]
    fn gc_sweep_relanding_is_safe() {
        let id = 0x5E17_0002i32;
        let mut snap = Vec::new();
        mk(&mut snap, id, 0.0, 0, F_ON_GROUND);
        let mut out = [FULL; 1];
        let mut stats = [0i64; 3];
        decide_batch(&snap, &mut out, &mut stats); // seq 1
        decide_batch(&snap, &mut out, &mut stats); // seq 2 -> REST
        assert_eq!(out[0], REST);
        // Simulate staleness: rewind last_epoch beyond the keep window.
        let epoch = EPOCH.load(Ordering::Relaxed);
        for s in shards().iter() {
            let mut g = s.lock().unwrap();
            for (_, last) in g.values_mut() {
                *last = epoch - 4097;
            }
        }
        // The NEXT sweep runs on epoch % 1024 == 0; force one.
        for s in shards().iter() {
            let mut g = s.lock().unwrap();
            g.retain(|_, (_, last)| epoch - *last <= 4096);
        }
        let live: usize = shards().iter().map(|s| s.lock().unwrap().len()).sum();
        assert_eq!(live, 0, "stale entries must be swept");
        // Re-observed id re-lands (FULL), not a stale REST.
        decide_batch(&snap, &mut out, &mut stats);
        assert_eq!(out[0], FULL, "swept id must re-land with a FULL tick");
    }

    /// Delivery gate: the embedded blob must be the compiled bridge (major 65)
    /// carrying the java gate string + native declarations + selfTest.
    #[test]
    fn blob_contract() {
        let b = OPS_BYTES;
        assert_eq!((((b[6] as u16) << 8) | b[7] as u16), 65, "class major must be 65 (--release 21)");
        assert!(contains_bytes(b, b"cmp446_items"), "java gate string missing");
        assert!(contains_bytes(b, b"planeProbe"));
        assert!(contains_bytes(b, b"planeDecide"));
        assert!(contains_bytes(b, b"selfTest"));
        assert!(contains_bytes(b, b"inactiveTick"), "REST path must call the vanilla inactiveTick");
        assert!(contains_bytes(b, b"mergeWithNeighbours"), "merge must use the vanilla private body");
        // S7-163: single classfile (no nested classes) — asserted by the build
        // script guard; here just sanity on the source-of-truth size.
        assert!(b.len() > 10_000, "blob suspiciously small: {}", b.len());
    }

    fn contains_bytes(hay: &[u8], needle: &[u8]) -> bool {
        hay.windows(needle.len()).any(|w| w == needle)
    }
}

