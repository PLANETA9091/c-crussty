//! FLUID-BULK (TASK-416-C, lever cmp416_fluid STRICT-eq): the fluid-update
//! SUBSYSTEM migration control plane (law 6 — whole subsystem, ONE bulk
//! JNI/tick, zero per-entity JNI).
//!
//! ARCHITECTURE (RESEARCH-C-416.md):
//! - DATA PLANE (Java, `FluidBulkOps` blob defined into the kernel loader):
//!   thread-confined cell LUT (key[]/tag[]/val[]) with per-slot packed
//!   (tick,GEN) validity stamps; the vanilla javap-exact sweep body; the
//!   vanilla push tail. The first entity to touch a cell gathers it for the
//!   whole pile (lazy probe-dedup — anti-lesson bl2: NO gather traversal
//!   exists, the gather rides the existing sweeps).
//! - CONTROL PLANE (this module, Rust): natives `fluidProbe`/`fluidBulk`
//!   registered on FluidBulkOps. `fluidBulk(tick, io)` runs ONCE per server
//!   tick (JVM-wide, java EPOCH_LOCK + STAMPED_TICK): drains the census
//!   (calls/hits/wet/slow/presence/fill), enforces the monotonicity policy
//!   (volatile counters are monotonic across real-time reads — any
//!   regression is structural), and answers with the lifetime LUT fill
//!   count. Gross anomalies (>3) or any structural drift -> ERR_STRUCT ->
//!   java disarms the LUT FOREVER (bit-exact vanilla body).
//! - WIRING: the two Entity wrapper call-sites of
//!   `updateFluidHeightAndDoFluidPushing` (updateInWaterStateAndDoWaterCurrentPushing
//!   + updateInWaterStateAndDoFluidPushing — the ONLY two in the kernel,
//!   S7151_CENSUS) retarget to `FluidBulkOps.updateFluidHeightAndDoFluidPushing`
//!   through the entity_compose chain STAGE 11 (`compose_entity`). The ONE
//!   `LevelChunk.setBlockState -> LevelChunkSection.setBlockState` site
//!   retargets to `FluidBulkOps.secWrite` (RECON-43 write-bump contract —
//!   NOT the forbidden fluid_dirty memo / fluid_bitmask: one long GEN bump
//!   on a real FluidState singleton change).
//!
//! VANILLA (law 4): with CRUSSTY_LEVER_FLAG != cmp416_fluid nothing
//! registers, nothing defines, nothing retransforms — Entity and LevelChunk
//! run bit-in-byte vanilla. The carrier gates (cmp414_cvs STRICT OR) are
//! untouched: cmp416_fluid was ADDED to the OR chains, never substituted.

use jvmti_bindings::prelude::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const LEVELCHUNK_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunk";
const OPS_CLASS: &str = "net/minecraft/world/entity/FluidBulkOps";
const OPS_PLANE_CLASS: &str = "net/minecraft/world/entity/FluidBulkOps$Plane";
const OPS_RES_CLASS: &str = "net/minecraft/world/entity/FluidBulkOps$Res";

const OPS_BYTES: &[u8] =
    include_bytes!("../fluid/build/net/minecraft/world/entity/FluidBulkOps.class");
const OPS_PLANE_BYTES: &[u8] =
    include_bytes!("../fluid/build/net/minecraft/world/entity/FluidBulkOps$Plane.class");
const OPS_RES_BYTES: &[u8] =
    include_bytes!("../fluid/build/net/minecraft/world/entity/FluidBulkOps$Res.class");

pub const PROBE_MAGIC: i32 = 0x4642; // "FB"
pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// Lever label for markers (single source of truth for the logs).
pub const LEVER: &str = "cmp416_fluid";

// ---------------------------------------------------------------------------
// Gate (STRICT-eq, env read once)
// ---------------------------------------------------------------------------

/// STRICT-eq gate on the TRIMMED flag. Empty / foreign / case-variant flags
/// = vanilla (round-400 protocol). NOT part of the cmp414_cvs carrier chain:
/// a carrier-only leg must never arm the fluid plane.
pub fn gate_matches(flag: &str) -> bool {
    flag == LEVER
}

fn enabled_once() -> bool {
    match std::env::var("CRUSSTY_LEVER_FLAG") {
        Ok(f) => gate_matches(f.trim()),
        Err(_) => false,
    }
}

static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

pub fn enabled() -> bool {
    *ENABLED.get_or_init(enabled_once)
}

// ---------------------------------------------------------------------------
// State published to entity_compose (stage 11) and the LevelChunk hook
// ---------------------------------------------------------------------------

static OPS_READY: AtomicBool = AtomicBool::new(false);
static LC_READY: AtomicBool = AtomicBool::new(false);

/// True once FluidBulkOps (+$Plane+$Res) is defined in the kernel loader and
/// the natives are bound. entity_compose stage 11 waits on this BEFORE
/// serving an Entity patch that references the bridge (NCDFE-window kill).
pub fn ops_ready() -> bool {
    OPS_READY.load(Ordering::Acquire)
}

/// Poll `ops_ready` with a deadline (entity_compose stage-bridge protocol).
pub fn wait_ops_ready(timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    loop {
        if ops_ready() {
            return true;
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

/// True once the LevelChunk secWrite retarget is serving (GEN-bump live).
pub fn lc_armed() -> bool {
    LC_READY.load(Ordering::Acquire)
}

// ---------------------------------------------------------------------------
// entity_compose stage 11: the two-wrapper retarget
// ---------------------------------------------------------------------------

/// Compose the fluid-bulk stage onto the accumulated Entity bytes: BOTH
/// wrapper call-sites of updateFluidHeightAndDoFluidPushing must retarget or
/// the stage is rejected (fail-dominant, sites==2 STRICT — the S7-151 census
/// proved these are the only two call sites in the kernel).
pub fn compose_entity(bytes: &[u8]) -> Option<Vec<u8>> {
    if !enabled() {
        return None;
    }
    match crate::classfile::patch_fluid_bulk_entity(bytes) {
        Ok((p, outcome)) => match outcome {
            crate::classfile::RetargetOutcome::Retargeted { sites: 2 }
            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 2 } => Some(p),
            other => {
                eprintln!(
                    "[crussty-plugin] {LEVER}: compose_entity strict site-count violated ({other:?}) — stage rejected"
                );
                None
            }
        },
        Err(e) => {
            eprintln!("[crussty-plugin] {LEVER}: compose_entity patch rejected ({e})");
            None
        }
    }
}

// ---------------------------------------------------------------------------
// LevelChunk byte hook (stash/serve; the ONLY LevelChunk hook while
// fluid_dirty is dormant — if both were ever armed the LAST registered
// serves, which is this one: secWrite->FluidBulkOps GEN-bump wins)
// ---------------------------------------------------------------------------

static LC_ORIG: std::sync::OnceLock<Mutex<Option<Vec<u8>>>> = std::sync::OnceLock::new();
static LC_PATCH: std::sync::OnceLock<Mutex<Option<Vec<u8>>>> = std::sync::OnceLock::new();
static LC_SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn lc_orig() -> &'static Mutex<Option<Vec<u8>>> {
    LC_ORIG.get_or_init(|| Mutex::new(None))
}
fn lc_patch() -> &'static Mutex<Option<Vec<u8>>> {
    LC_PATCH.get_or_init(|| Mutex::new(None))
}

/// Register the LevelChunk byte hook (idempotent; call once from
/// cplugin_init — BEFORE the kernel loads LevelChunk so the first sighting
/// stashes pristine bytes). Dormant = no hook, vanilla bit-in-byte.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] fluid_bulk: dormant (lever {LEVER} not set — fluid plane vanilla)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(LEVELCHUNK_CLASS, |_name, bytes| {
        if !LC_READY.load(Ordering::Acquire) {
            let mut orig = lc_orig().lock().unwrap_or_else(PoisonError::into_inner);
            if orig.is_none() {
                eprintln!(
                    "[crussty-plugin] fluid_bulk: pristine sighting {LEVELCHUNK_CLASS} {} bytes",
                    bytes.len()
                );
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        let cached = lc_patch()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        if !LC_SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] fluid_bulk: LevelChunk hook serve {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached
    });
}

// ---------------------------------------------------------------------------
// Activation: define ops + natives EARLY, then arm the LevelChunk retarget
// ---------------------------------------------------------------------------

pub fn activate() {
    if !enabled() {
        return; // register() already logged the dormant notice
    }
    std::thread::Builder::new()
        .name("crussty-fluid-bulk".into())
        .spawn(move || {
            // Wait for the kernel Entity class (loads at boot; the ops class
            // references Entity in its method descriptors).
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] fluid_bulk: {ENTITY_CLASS} not loaded within 180s — ops stay dormant (vanilla)"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(2_000));
            }
            // Kernel loader must be quiet before define_class (boot-storm
            // discipline, improved_noise hardening).
            if !crate::improved_noise::wait_for_boot() {
                eprintln!(
                    "[crussty-plugin] fluid_bulk: boot marker not seen — ops stay dormant (vanilla)"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(2));

            // Embedded blobs must not be newer than the live JVM (major gate).
            let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
                crate::improved_noise::jvm_class_major(env)
                    .or_else(|| crate::improved_noise::jvm_max_class_major(env))
            })
            .flatten()
            .unwrap_or(u16::MAX);
            for (name, bytes) in EMBEDS {
                let bm = crate::improved_noise::class_version(bytes)
                    .map(|(m, _)| m)
                    .unwrap_or(0);
                if bm > jvm_major {
                    eprintln!(
                        "[crussty-plugin] fluid_bulk: embedded {name} is class major {bm} but JVM supports up to {jvm_major} — rebuild fluid/ via scripts/build_fluid_bulk.sh; ops stay dormant"
                    );
                    return;
                }
            }

            let defined = define_ops_once();
            if !defined {
                eprintln!(
                    "[crussty-plugin] fluid_bulk: ops define failed — plane stays vanilla (fail-dominant)"
                );
                return;
            }
            OPS_READY.store(true, Ordering::Release);
            eprintln!(
                "[crussty-plugin] fluid_bulk: ops defined + natives bound (fluidProbe/fluidBulk) — entity_compose stage fluid_bulk may compose"
            );

            arm_levelchunk();

            // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
            eprintln!(
                "[crussty-plugin] {LEVER}: ARMED fluid-bulk (Entity wrapper call-sites -> FluidBulkOps LUT plane: per-slot (tick,GEN) stamp validity, lazy probe-dedup piggyback NO gather pass; LevelChunk.setBlockState -> secWrite GEN-bump invalidation; rust fluidBulk = ONE bulk JNI/tick epoch authority + census policy, ERR_STRUCT = permanent vanilla disarm; empty flag = vanilla bit-in-byte)"
            );
            ops_selftest();
        })
        .ok();
}

const EMBEDS: [(&str, &[u8]); 3] = [
    (OPS_CLASS, OPS_BYTES),
    (OPS_PLANE_CLASS, OPS_PLANE_BYTES),
    (OPS_RES_CLASS, OPS_RES_BYTES),
];

/// Define the ops trio into the kernel loader + RegisterNatives. Idempotent
/// under OPS_READY double-check; failure = False (caller stays vanilla).
fn define_ops_once() -> bool {
    cplug_sdk::jni_util::with_attached(|env| {
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
        // Keep a global ref of the ops class for the self-test (kernel-loader
        // classes are invisible to FindClass from a native attachment).
        let mut ops_local: *mut c_void = std::ptr::null_mut();
        for (name, bytes) in EMBEDS {
            match env.define_class(name, gref, bytes) {
                Some(c) => {
                    if name == OPS_CLASS {
                        ops_local = c;
                    } else {
                        env.delete_local_ref(c);
                    }
                    eprintln!("[crussty-plugin] fluid_bulk: defined {name} in kernel loader");
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] fluid_bulk: define_class({name}) failed");
                    env.delete_local_ref(loader);
                    env.delete_local_ref(class_cls);
                    return false;
                }
            }
        }
        if ops_local.is_null() {
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }

        // RegisterNatives: fluidProbe (magic) + fluidBulk (tick epoch).
        // TASK-409-E lesson: sig must match the java declaration EXACTLY.
        //   fluidProbe()                      -> ()I
        //   fluidBulk(int,long[])             -> (I[J)I
        let names = [
            CString::new("fluidProbe").expect("no NUL"),
            CString::new("fluidBulk").expect("no NUL"),
        ];
        let sigs = [
            CString::new("()I").expect("no NUL"),
            CString::new("(I[J)I").expect("no NUL"),
        ];
        let natives = [
            jvmti_bindings::jni::JNINativeMethod {
                name: names[0].as_ptr(),
                signature: sigs[0].as_ptr(),
                fnPtr: fluid_probe as *const c_void as *mut c_void,
            },
            jvmti_bindings::jni::JNINativeMethod {
                name: names[1].as_ptr(),
                signature: sigs[1].as_ptr(),
                fnPtr: fluid_bulk as *const c_void as *mut c_void,
            },
        ];
        // RegisterNatives on the LOCAL class ref (entity_query pattern).
        let reg = env.register_natives(ops_local, &natives);
        if let Err(code) = reg {
            crate::describe_exception(env);
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] fluid_bulk: register_natives failed (code {code}) — plane stays vanilla"
            );
            env.delete_local_ref(ops_local);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }
        // NOW promote to a global ref for the self-test (before deleting the
        // local — a stored local would dangle after delete_local_ref).
        let g = env.new_global_ref(ops_local);
        OPS_GREF.store(g as usize, Ordering::SeqCst);
        env.delete_local_ref(ops_local);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        true
    })
    .unwrap_or(false)
}

static OPS_GREF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Arm the LevelChunk secWrite retarget: pristine capture (load sighting or
/// no-op retransform), strict 1-site patch, serve, retransform once.
fn arm_levelchunk() {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while cplug_sdk::classes::find_class(LEVELCHUNK_CLASS).is_none() {
        if std::time::Instant::now() > deadline {
            eprintln!(
                "[crussty-plugin] fluid_bulk: {LEVELCHUNK_CLASS} never appeared — secWrite retarget stays dormant (GEN frozen; LUT still per-tick valid)"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    // Pristine capture: if the class predates the hook, no-op retransform
    // delivers the bytes through the stash-only hook (READY=false).
    if lc_orig()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .is_none()
    {
        for _ in 1..=3 {
            let _ = cplug_sdk::retransform_class(LEVELCHUNK_CLASS);
            let has = lc_orig()
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .is_some();
            if has {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        let has = lc_orig()
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some();
        if !has {
            eprintln!(
                "[crussty-plugin] fluid_bulk: no pristine bytes for {LEVELCHUNK_CLASS} — secWrite retarget stays dormant"
            );
            return;
        }
    }
    let original = lc_orig()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let Some(original) = original else {
        return;
    };
    match crate::classfile::patch_fluid_bulk_levelchunk(&original) {
        Ok((patched, outcome)) => match outcome {
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 } => {
                let patched_len = patched.len();
                *lc_patch().lock().unwrap_or_else(PoisonError::into_inner) = Some(patched);
                LC_READY.store(true, Ordering::Release);
                let rc = cplug_sdk::retransform_class(LEVELCHUNK_CLASS);
                eprintln!(
                    "[crussty-plugin] fluid_bulk: LevelChunk secWrite GEN-bump armed ({} -> {} bytes, retransform rc={rc})",
                    original.len(),
                    patched_len
                );
            }
            other => {
                eprintln!(
                    "[crussty-plugin] fluid_bulk: LevelChunk strict site-count violated ({other:?}) — secWrite retarget stays dormant"
                );
            }
        },
        Err(e) => {
            eprintln!(
                "[crussty-plugin] fluid_bulk: LevelChunk patch rejected ({e}) — secWrite retarget stays dormant"
            );
        }
    }
}

/// Drive the java selfTest (probe/pack/stamp/presence algebra) in the live
/// kernel loader via the stored global ref.
fn ops_selftest() {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let stored = OPS_GREF.load(Ordering::SeqCst);
        if stored == 0 {
            return false;
        }
        let hook = stored as jni::jclass;
        let Some(mid) = env.get_static_method_id(hook, "selfTest", "()Z") else {
            crate::clear_exception(env);
            return false;
        };
        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let call_bool = fn_table.CallStaticBooleanMethodA;
            (call_bool)(raw, hook, mid, [].as_ptr()) != 0
        }
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] fluid_bulk: self-test passed (pack/stamp/presence algebra operational in kernel loader)"
        ),
        _ => eprintln!("[crussty-plugin] fluid_bulk: self-test failed/skipped"),
    }
}

// ---------------------------------------------------------------------------
// Census policy (pure, unit-tested) — the Rust side of the ONE bulk JNI/tick
// ---------------------------------------------------------------------------

/// Census snapshot as drained from the java volatile counters. The counters
/// only ever increase; volatile reads across real time are monotonic (JMM
/// synchronization order), so ANY regression is a structural anomaly.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Snap {
    pub calls: i64,
    pub hits: i64,
    pub wet: i64,
    pub slow: i64,
    pub presence: i64,
    pub fill: i64,
}

impl Snap {
    fn as_arr(&self) -> [i64; 6] {
        [self.calls, self.hits, self.wet, self.slow, self.presence, self.fill]
    }
}

/// Gross-anomaly budget: three structural violations = the channel is
/// unreliable -> ERR_STRUCT (java disarms the LUT to vanilla forever).
pub const ANOMALY_LIMIT: i64 = 3;

#[derive(Default)]
pub struct PolicyState {
    pub last_tick: i64,
    pub last_gen: i64,
    pub snap: Snap,
    pub anomalies: i64,
    pub epochs: i64,
    pub disarmed: bool,
}

/// One tick-epoch step. Returns 0 (ok) or ERR_STRUCT (java disarms).
/// `tick` must be non-negative and non-decreasing; every census field and
/// GEN must be non-decreasing (volatile monotonicity contract).
pub fn policy_step(st: &mut PolicyState, tick: i64, cur: &Snap, gen: i64) -> i32 {
    if st.disarmed {
        return ERR_STRUCT;
    }
    if tick < 0 {
        st.disarmed = true;
        return ERR_STRUCT;
    }
    let mut anomaly = 0i64;
    if st.last_tick >= 0 {
        if tick < st.last_tick {
            anomaly += 1; // tick clock regression = structural
        }
        if gen < st.last_gen {
            anomaly += 1; // GEN only increments (write-bump contract)
        }
        let prev = st.snap.as_arr();
        let now = cur.as_arr();
        for (p, n) in prev.iter().zip(now.iter()) {
            if n < p {
                anomaly += 1; // volatile counter regressed = structural
            }
        }
    }
    st.anomalies += anomaly;
    st.snap = *cur;
    st.last_tick = tick;
    st.last_gen = gen;
    st.epochs += 1;
    if st.anomalies >= ANOMALY_LIMIT {
        st.disarmed = true;
        return ERR_STRUCT;
    }
    0
}

static POLICY: Mutex<PolicyState> = Mutex::new(PolicyState {
    last_tick: -1,
    last_gen: -1,
    snap: Snap {
        calls: 0,
        hits: 0,
        wet: 0,
        slow: 0,
        presence: 0,
        fill: 0,
    },
    anomalies: 0,
    epochs: 0,
    disarmed: false,
});

static EFFECT_LOGGED: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Natives (registered on FluidBulkOps by define_ops_once)
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn fluid_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK tick-epoch authority — ONE call per server tick (java EPOCH_LOCK).
/// io layout (8 longs): [0]=tick (informational), [1..6]=census cumulative
/// (calls,hits,wet,slow,presence,fill), [7]in=GEN, [7]out=lifetime LUT fills.
/// Returns 0 (ok) / ERR_RANGE (len != 8) / ERR_STRUCT (gate off, pin or
/// policy failure -> java disarms the LUT permanently).
///
/// # Safety
/// See fluid_probe.
#[no_mangle]
pub unsafe extern "system" fn fluid_bulk(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    io: jni::jlongArray,
) -> jni::jint {
    if !enabled() || env.is_null() || io.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let len = unsafe { (vt.GetArrayLength)(env, io) };
    if len != 8 {
        return ERR_RANGE; // structural drift — java treats rc<0 as disarm=this tick
    }
    let pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, io, std::ptr::null_mut()) };
    if pin.is_null() {
        return ERR_STRUCT;
    }
    let (rc, d_calls, d_hits, d_fill, rate) = {
        let s = unsafe { std::slice::from_raw_parts_mut(pin as *mut jni::jlong, 8) };
        let cur = Snap {
            calls: s[1],
            hits: s[2],
            wet: s[3],
            slow: s[4],
            presence: s[5],
            fill: s[6],
        };
        let gen = s[7];
        let mut st = POLICY.lock().unwrap_or_else(PoisonError::into_inner);
        let first = st.last_tick < 0;
        let prev = st.snap;
        let code = policy_step(&mut st, tick as i64, &cur, gen);
        let dc = if first { cur.calls } else { (cur.calls - prev.calls).max(0) };
        let dh = if first { cur.hits } else { (cur.hits - prev.hits).max(0) };
        let df = if first { cur.fill } else { (cur.fill - prev.fill).max(0) };
        let denom = cur.hits + cur.fill;
        let rate = if denom > 0 {
            cur.hits as f64 * 100.0 / denom as f64
        } else {
            0.0
        };
        if code == 0 && !EFFECT_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] {LEVER}: EFFECT rust-bulk epoch ok (tick {tick}, calls+{dc} hits+{dh} fills+{df} lut-hit_rate {rate:.1}%) — ONE bulk JNI/tick, LUT plane owned by rust epoch"
            );
        }
        // Write-back: io[7] = lifetime LUT fills (java ARM marker prints it).
        if code >= 0 {
            s[7] = cur.fill as jni::jlong;
        }
        (code, dc, dh, df, rate)
    };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, io, pin, 0) };
    rc
}

// ---------------------------------------------------------------------------
// Tests (pure policy/gate logic; JNI paths need a live VM — covered by the
// java selfTest at runtime)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_is_strict_eq() {
        assert!(gate_matches("cmp416_fluid"));
        assert!(!gate_matches(""));
        assert!(!gate_matches("cmp414_cvs"));
        assert!(!gate_matches("cmp412_meganav"));
        assert!(!gate_matches("cmp415_fluid"));
        assert!(!gate_matches("cmp416_fluid2"));
        assert!(!gate_matches("CMP416_FLUID"));
        assert!(!gate_matches(" cmp416_fluid")); // callers trim first
    }

    #[test]
    fn policy_first_epoch_ok() {
        let mut st = PolicyState::default();
        let cur = Snap { calls: 10, hits: 4, wet: 2, slow: 1, presence: 3, fill: 6 };
        assert_eq!(policy_step(&mut st, 42, &cur, 0), 0);
        assert_eq!(st.epochs, 1);
        assert_eq!(st.last_tick, 42);
        assert!(!st.disarmed);
    }

    #[test]
    fn policy_monotonic_growth_ok() {
        let mut st = PolicyState::default();
        let mut t = 0i64;
        for i in 0..50i64 {
            t += 1;
            let cur = Snap {
                calls: 100 * (i + 1),
                hits: 40 * (i + 1),
                wet: 2 * (i + 1),
                slow: i,
                presence: 3 * (i + 1),
                fill: 60 * (i + 1),
            };
            assert_eq!(policy_step(&mut st, t, &cur, 0), 0);
        }
        assert_eq!(st.epochs, 50);
        assert_eq!(st.anomalies, 0);
    }

    #[test]
    fn policy_counter_regression_is_anomaly_then_disarm() {
        let mut st = PolicyState::default();
        let a = Snap { calls: 100, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 1, &a, 0), 0);
        // calls regress: 1 anomaly
        let b = Snap { calls: 90, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 2, &b, 0), 0);
        assert_eq!(st.anomalies, 1);
        // hits regress: 2nd anomaly
        let c = Snap { calls: 110, hits: 30, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 3, &c, 0), 0);
        assert_eq!(st.anomalies, 2);
        // fill regress: 3rd anomaly -> ERR_STRUCT + permanent disarm
        let d = Snap { calls: 120, hits: 40, wet: 2, slow: 0, presence: 3, fill: 50 };
        assert_eq!(policy_step(&mut st, 4, &d, 0), ERR_STRUCT);
        assert!(st.disarmed);
        // disarmed stays disarmed
        assert_eq!(policy_step(&mut st, 5, &d, 0), ERR_STRUCT);
    }

    #[test]
    fn policy_tick_regression_is_anomaly() {
        let mut st = PolicyState::default();
        let a = Snap { calls: 100, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 10, &a, 0), 0);
        assert_eq!(policy_step(&mut st, 9, &a, 0), 0);
        assert_eq!(st.anomalies, 1);
    }

    #[test]
    fn policy_gen_regression_is_anomaly() {
        let mut st = PolicyState::default();
        let a = Snap { calls: 100, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 1, &a, 5), 0);
        assert_eq!(policy_step(&mut st, 2, &a, 4), 0);
        assert_eq!(st.anomalies, 1);
    }

    #[test]
    fn policy_negative_tick_is_immediate_struct() {
        let mut st = PolicyState::default();
        let a = Snap { calls: 100, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, -1, &a, 0), ERR_STRUCT);
        assert!(st.disarmed);
    }

    #[test]
    fn policy_mega_gen_growth_ok() {
        // GEN bumps thousands of times per tick under heavy fluid writes —
        // monotonic growth must never trip.
        let mut st = PolicyState::default();
        let a = Snap { calls: 100, hits: 40, wet: 2, slow: 0, presence: 3, fill: 60 };
        assert_eq!(policy_step(&mut st, 1, &a, 0), 0);
        assert_eq!(policy_step(&mut st, 2, &a, 50_000), 0);
        assert_eq!(policy_step(&mut st, 3, &a, 1048576), 0);
        assert_eq!(st.anomalies, 0);
    }

    #[test]
    fn snap_layout_round_trip() {
        let s = Snap { calls: 1, hits: 2, wet: 3, slow: 4, presence: 5, fill: 6 };
        let a = s.as_arr();
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
    }
}
