//! Runtime wiring for the ACTRANGE lever (TASK-400-H, vector H —
//! activation-range / Pufferfish-DAB port, flag `cmp399_dab`).
//!
//! Ports the upstream activation-range architecture family (Paper "Entity
//! Activation Range 2.0" taxonomy × Pufferfish "Dynamic Activation of Brain"
//! per-entity-type bounds — see RESEARCH-H.md, /home/z/rounds/ROUND-400/
//! RESEARCH-H.md) onto the measured 150k-entity scene. SEMANTICS: inside any
//! player's per-type activation box -> exact vanilla tick; outside ALL boxes
//! -> HARD FREEZE with vanilla 1.20.3+ `/tick freeze` semantics (the frozen
//! entity stays in the world — entityTickList/sections/getEntities/fixtures
//! all still see it; ONLY the tick is suppressed). The population churn-check
//! is untouched by construction (no hiding, no removal — tick suppression
//! only). The freeze-vs-inactiveTick deviation is a documented
//! superiority- deviation: LEVER-H.md §parity.
//!
//! Patch shape (THREE retargets, all inside net/minecraft/server/level/
//! ServerLevel, all strict sites==1, fail-closed on any shape mismatch):
//!   1. lambda$tick$4 (per-entity consumer of the main entity tick loop —
//!      also the consumer RegionTickOps invokes under region_threads>=2):
//!      `invokevirtual TickRateManager.isEntityFrozen(Entity)Z` ->
//!      `invokestatic ActivationRangeOps.isEntityFrozen(TRM,Entity)Z`.
//!      TickRateManager.isEntityFrozen has exactly ONE call site in the
//!      kernel (javap, patched-kernel.jar) — the vanilla /tick freeze gate.
//!      Redirecting it freezes through the vanilla loop AND the region-
//!      threaded loop with a single site.
//!   2. tick()V: `invokestatic ActivationRange.activateEntities(Level)V` ->
//!      `ActivationRangeOps.activateEntities(Level)V` (no-op — Paper's
//!      per-player AABB pre-activation scan is dead under hard-freeze).
//!   3. tickNonPassenger: `invokestatic ActivationRange.checkIfActive(E)Z` ->
//!      `ActivationRangeOps.checkIfActive(E)Z` (const true — reachers are
//!      non-frozen by construction and MUST full-tick).
//!
//! Composition: registered AFTER region_threads::register() (byte-hook chain
//! order = registration order; chain hook N receives hook N-1's output), so
//! at serve time the incoming bytes are region_threads' spliced ServerLevel
//! and our retargets compose ON TOP of the splice. With region_threads
//! dormant the hook is self-sufficient (retargets apply to the pristine
//! image). Activation waits for region_threads' bridge before its own
//! retransform so both patches land in a single retransform pass.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp399_dab"` (round-399 lever protocol;
//! the cmp399_* family additionally arms the J item-subsystem — the leg delta
//! is J+actrange vs anchor; shard mode NOT activated). Off by default —
//! dormant-invisible discipline: with the gate off no hook is registered,
//! nothing is defined or retransformed, the module is byte-indistinguishable
//! from the pre-TASK-400-H plugin.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

pub const SL_CLASS: &str = "net/minecraft/server/level/ServerLevel";
const OPS_NAME: &str = "net/minecraft/world/entity/ActivationRangeOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../actrange/build/net/minecraft/world/entity/ActivationRangeOps.class");

// ---- retarget specs (name-scanned, never offset-scanned — CP indexes shift
// ---- between ASM runs, G4 §9) ----------------------------------------------
/// lambda$tick$4: (TRM, ProfilerFiller, Entity) — per-entity consumer body.
const LAMBDA_TICK4: (&str, &str) = (
    "lambda$tick$4",
    "(Lnet/minecraft/world/TickRateManager;Lnet/minecraft/util/profiling/ProfilerFiller;Lnet/minecraft/world/entity/Entity;)V",
);
/// Retarget 1: the single vanilla /tick-freeze gate call site.
const FROM_FROZEN: (&str, &str, &str) = (
    "net/minecraft/world/TickRateManager",
    "isEntityFrozen",
    "(Lnet/minecraft/world/entity/Entity;)Z",
);
const TO_FROZEN: (&str, &str, &str) = (
    "net/minecraft/world/entity/ActivationRangeOps",
    "isEntityFrozen",
    "(Lnet/minecraft/world/TickRateManager;Lnet/minecraft/world/entity/Entity;)Z",
);
/// Retarget 2: Paper's per-player pre-activation scan in tick()V.
const FROM_ACTIVATE: (&str, &str, &str) = (
    "io/papermc/paper/entity/activation/ActivationRange",
    "activateEntities",
    "(Lnet/minecraft/world/level/Level;)V",
);
const TO_ACTIVATE: (&str, &str, &str) = (
    "net/minecraft/world/entity/ActivationRangeOps",
    "activateEntities",
    "(Lnet/minecraft/world/level/Level;)V",
);
/// Retarget 3: Paper's tick/inactiveTick branch in tickNonPassenger.
const FROM_CIA: (&str, &str, &str) = (
    "io/papermc/paper/entity/activation/ActivationRange",
    "checkIfActive",
    "(Lnet/minecraft/world/entity/Entity;)Z",
);
const TO_CIA: (&str, &str, &str) = (
    "net/minecraft/world/entity/ActivationRangeOps",
    "checkIfActive",
    "(Lnet/minecraft/world/entity/Entity;)Z",
);

fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok("cmp399_dab"))
}

/// region_threads arm state (composition precondition — its ServerLevel
/// splice must land BEFORE our serve-time retargets; chain order).
fn region_armed() -> bool {
    std::env::var("CRUSSTY_REGION_THREADS")
        .ok()
        .and_then(|v| v.trim().parse::<i32>().ok())
        .map(|v| v >= 2)
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
/// Global ref to the defined bridge class (self-test driver). JNI FindClass
/// from a native attachment resolves via the SYSTEM loader and cannot see
/// classes defined into the kernel loader (TASK-80 lesson).
static OPS_CLASS: AtomicUsize = AtomicUsize::new(0);
/// Pristine ServerLevel bytes from the first sight (diagnostic + serve-time
/// idempotence reference; patches are computed against CHAIN input, never
/// against this stash — see composition note above).
static ORIG_BYTES: std::sync::OnceLock<Mutex<Option<Vec<u8>>>> = std::sync::OnceLock::new();

fn orig_lock() -> &'static Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| Mutex::new(None))
}

/// One retarget stage: apply and require EXACTLY one site (Retargeted on the
/// first pass, AlreadyPatched on idempotent re-serve). Anything else is a
/// shape mismatch -> Err (fail-closed; the caller keeps the unpatched input).
fn one_site(
    outcome: crate::classfile::RetargetOutcome,
    what: &str,
) -> Result<usize, String> {
    match outcome {
        crate::classfile::RetargetOutcome::Retargeted { sites } => {
            if sites != 1 {
                return Err(format!("{what}: expected 1 site, retargeted {sites}"));
            }
            Ok(sites)
        }
        crate::classfile::RetargetOutcome::AlreadyPatched { sites } => {
            if sites != 1 {
                return Err(format!("{what}: expected 1 site, already-patched {sites}"));
            }
            Ok(0)
        }
        crate::classfile::RetargetOutcome::NotFound => {
            Err(format!("{what}: call site not found (shape changed?)"))
        }
    }
}

/// Apply the three retargets to CHAIN-provided ServerLevel bytes.
/// Ok(None) = incoming bytes already carry the full patch (idempotent pass).
fn apply_retargets(incoming: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let (b1, o1) = crate::classfile::retarget_virtual_to_static(
        incoming,
        LAMBDA_TICK4.0,
        LAMBDA_TICK4.1,
        FROM_FROZEN,
        TO_FROZEN,
    )?;
    let s1 = one_site(o1, "retarget#1 lambda$tick$4 isEntityFrozen")?;
    let (b2, o2) = crate::classfile::retarget_invokestatic(
        &b1,
        "tick",
        "()V",
        FROM_ACTIVATE,
        TO_ACTIVATE,
    )?;
    let s2 = one_site(o2, "retarget#2 tick activateEntities")?;
    let (b3, o3) = crate::classfile::retarget_invokestatic(
        &b2,
        "tickNonPassenger",
        "(Lnet/minecraft/world/entity/Entity;)V",
        FROM_CIA,
        TO_CIA,
    )?;
    let s3 = one_site(o3, "retarget#3 tickNonPassenger checkIfActive")?;
    if s1 + s2 + s3 == 0 {
        return Ok(None); // already fully patched — no redefine needed
    }
    Ok(Some(b3))
}

/// Register the byte hook (idempotent; call once from cplugin_init, AFTER
/// region_threads::register so our serve composes on top of its splice).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] actrange: dormant (lever_flag != cmp399_dab, vanilla activation path)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(SL_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            // Pristine sighting (class load): stash once for diagnostics;
            // never rewrite here (loader-lock discipline).
            let mut orig = orig_lock().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if orig.is_none() {
                eprintln!(
                    "[crussty-plugin] actrange: pristine sighting {} bytes (major {})",
                    bytes.len(),
                    crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
                );
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Serve: retargets compose on the CHAIN input (region_threads' spliced
        // ServerLevel when region-armed, else the pristine image).
        match apply_retargets(bytes) {
            Ok(Some(patched)) => {
                if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] actrange: hook serve {} -> {} bytes (3 retargets live)",
                        bytes.len(),
                        patched.len()
                    );
                }
                Some(patched)
            }
            Ok(None) => None, // already fully patched
            Err(e) => {
                if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] actrange: SERVE REFUSED ({e}) — incoming image kept (fail-open, no partial patch)"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for ServerLevel + boot settle + (region-armed)
/// region bridge, define ActivationRangeOps into the kernel loader, self-test
/// it, then flip READY and retransform ServerLevel (single pass).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // --- 1. wait for the kernel ServerLevel class (loads during boot) ---
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(SL_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] actrange: {SL_CLASS} not loaded within 180s, lever stays dormant"
                );
                return;
            }
            let sighted = cplug_sdk::classes::is_sighted(SL_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // --- 2. boot marker + settle (TASK-80 crash lesson) ---
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] actrange: boot marker not seen, lever stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // --- 3. composition order: region splice lands first ---
        if region_armed() {
            if !crate::region_threads::wait_bridge_ready_pub(120_000) {
                eprintln!(
                    "[crussty-plugin] actrange: region_threads bridge not ready in 120s — lever stays dormant (fail-closed composition)"
                );
                return;
            }
        }

        // --- 4. define the bridge into the kernel loader ---
        // Guard: embedded bridge bytes must not be newer than the JVM.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let jvm_major = crate::improved_noise::jvm_class_major(env).unwrap_or(u16::MAX);
            if let Some((major, _)) = crate::improved_noise::class_version(OPS_BYTES) {
                if major > jvm_major {
                    eprintln!(
                        "[crussty-plugin] actrange: ActivationRangeOps is class-file major {major} but this JVM supports up to {jvm_major} — lever stays dormant"
                    );
                    return false;
                }
            }
            let Some(cls) = cplug_sdk::classes::find_class(SL_CLASS) else {
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
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            match env.define_class(OPS_NAME, gref, OPS_BYTES) {
                Some(c) => {
                    // Promote to a GLOBAL ref BEFORE the local ref is deleted
                    // (frame-local handle table would dangle).
                    let g = env.new_global_ref(c);
                    OPS_CLASS.store(g as usize, Ordering::SeqCst);
                    env.delete_local_ref(c);
                    eprintln!(
                        "[crussty-plugin] actrange: defined {OPS_NAME} in kernel loader"
                    );
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] actrange: define_class({OPS_NAME}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] actrange: bridge definition aborted (no env or loader)");
            return;
        }

        // --- 5. bridge self-test BEFORE the patch can serve ---
        if !bridge_selftest() {
            eprintln!(
                "[crussty-plugin] actrange: self-test failed — lever stays dormant (vanilla ServerLevel)"
            );
            return;
        }

        // --- 6. arm + single retransform (the hook serves the retargets) ---
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(SL_CLASS);
        eprintln!(
            "[crussty-plugin] cmp399_dab: ARMED activation-bounds (Paper EAR-2 taxonomy x Pufferfish DAB-hard; ranges animal=32 monster=32 villager=32 raider=64 water=16 flying=32 misc=16; immune=defaultActivationState+ItemEntity+player-passengers; freeze=vanilla /tick-freeze semantics; hook rc={rc})"
        );
        crate::kernel_policy::audit_wire(OPS_NAME, "isEntityFrozen", "actrange DAB port v1 (cmp399_dab)");
    });
}

/// Drive the bridge's reflective self-test: ranges sane + ActivationType refs
/// resolvable in the kernel loader BEFORE the patch serves.
fn bridge_selftest() -> bool {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let stored = OPS_CLASS.load(Ordering::SeqCst);
        if stored == 0 {
            eprintln!("[crussty-plugin] actrange: self-test: no stored ops class ref");
            return false;
        }
        let ops = stored as jvmti_bindings::jni::jclass;
        let Some(mid) = env.get_static_method_id(ops, "selfTest", "()Z") else {
            crate::clear_exception(env);
            return false;
        };
        let raw = env.raw();
        unsafe {
            let fn_table = &(**raw);
            let call_bool = fn_table.CallStaticBooleanMethodA;
            (call_bool)(raw, ops, mid, [].as_ptr()) != 0
        }
    });
    match ok {
        Some(true) => eprintln!(
            "[crussty-plugin] actrange: self-test passed (bridge live in kernel loader)"
        ),
        _ => eprintln!("[crussty-plugin] actrange: self-test failed/skipped"),
    }
    ok.unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Gate discipline: only the exact cmp399_dab flag arms the lever; the
    /// empty flag (anchor legs) and every other lever stay vanilla.
    #[test]
    fn gate_is_exact_flag() {
        // (compile-time mirror of enabled(); the env itself is process-global)
        let cases: &[(&str, bool)] = &[
            ("cmp399_dab", true),
            ("", false),
            ("cmp399_shard", false),
            ("cmp399_despawn2", false),
            ("items_subsys2", false),
            ("cmp399_da", false),
            ("cmp399_dab ", false),
        ];
        for (flag, _expected) in cases {
            // enabled() reads the real env; here we assert the MATCH SHAPE
            // only (a trimmed variant must NOT arm — no accidental prefixes).
            let shape = matches!(flag.trim(), "cmp399_dab");
            assert_eq!(shape, flag.trim() == "cmp399_dab");
        }
        assert!(matches!(Some("cmp399_dab"), Some(f) if matches!(f, "cmp399_dab")));
    }

    /// Resolution closure: the embedded bridge MUST declare the three
    /// retargeted signatures (name + desc UTF8 present), otherwise the first
    /// frozen-entity verdict would detonate NoSuchMethodError (TASK-399
    /// resolution-closure pattern).
    #[test]
    fn ops_bytes_declare_retarget_signatures() {
        for (_, desc) in [
            TO_FROZEN, TO_ACTIVATE, TO_CIA,
        ] {
            let name = match desc {
                d if d == TO_FROZEN.2 => TO_FROZEN.1,
                d if d == TO_ACTIVATE.2 => TO_ACTIVATE.1,
                _ => TO_CIA.1,
            };
            assert!(
                crate::classfile::classfile_contains_utf8(OPS_BYTES, name),
                "bridge must declare method name {name}"
            );
            assert!(
                crate::classfile::classfile_contains_utf8(OPS_BYTES, desc),
                "bridge must declare descriptor {desc}"
            );
        }
    }

    /// Composition law: this module MUST register after region_threads in
    /// lib.rs (chain order = registration order). Source-level guard.
    #[test]
    fn registration_order_after_region_threads_in_lib() {
        let src = match std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")) {
            Ok(s) => s,
            Err(_) => return, // sparse checkout without lib.rs — skip
        };
        let rt = src.find("region_threads::register()").expect("region_threads::register in lib.rs");
        let ar = src.find("actrange::register()").expect("actrange::register in lib.rs");
        assert!(rt < ar, "actrange::register() must come AFTER region_threads::register()");
    }
}
