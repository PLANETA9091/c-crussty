//! Runtime wiring for the CMP401-DEVIRT lever (TASK-401-J, MEGA-ROUND-5,
//! agent J — devirtualization of hot dispatch, read AND write side).
//!
//! One class, two independent retargets, both javap-verified against the
//! round-j2b profile kernel (see src/classfile.rs for the byte contracts):
//!
//!   D1  `SynchedEntityData.get` body fusion — pristine
//!       `getItem(key).getValue()` double-virtual read chain replaced with
//!       a direct `itemsById[accessor.id()].value` read (13-byte
//!       straight-line body). The round-400-c-devirtfix precedent benched
//!       ≈+4пп pair on the B-axis; this port keeps the mechanism identical.
//!
//!   D1b `SynchedEntityData.set(acc,value,force)` — NEW this round: the
//!       write-side twin. The two DataItem virtual hops in the body
//!       (`item.getValue()` in the not-equal guard + `item.setValue(value)`
//!       write) become `getfield value` / `putfield value`. The swap is
//!       BYTE-LENGTH-PRESERVING (3B invokevirtual -> 3B field access), so
//!       branches, offsets, exception table and StackMapTable stay
//!       bit-identical — zero control-flow surgery (lesson G: round-399-g
//!       crashed re-laying out bodies; here layout is untouched, only CP
//!       indices change). `value` is package-private + nestmate-shared,
//!       access control passes bit-for-bit.
//!
//! Delivery: one whole-class byte hook on SynchedEntityData (pristine
//! capture at first load, patch served from cache after READY; vanilla
//! passthrough before). Fail-closed matrix: no pristine bytes -> dormant;
//! either patch rejects or violates its strict contract -> dormant stage
//! (vanilla behaviour). D2 (GoalSelector redirect) intentionally NOT
//! carried over: its ItemEntityManager bridge does not exist on the
//! round-401 base (master) — a fail-closed skip would waste the leg.
//!
//! Gate: CRUSSTY_LEVER_FLAG == "cmp401_devirt" (STRICT eq — other agents'
//! cmp401_* flags do not arm THIS module's sites).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const SED_CLASS: &str = "net/minecraft/network/syncher/SynchedEntityData";

/// Poison recovery (TASK-46): locks wrap plain Vec/Arc stores only.
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
    ready: AtomicBool,
}

#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
}

impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        let mut orig = self.orig.lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(bytes.to_vec());
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn set_patch(&self, bytes: Vec<u8>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(bytes.into_boxed_slice()),
        });
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
            .map(|c| Arc::clone(&c.bytes))
    }
    fn ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }
}

static TARGET_SED: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target_sed() -> &'static Target {
    TARGET_SED.get_or_init(|| Target::new(SED_CLASS))
}

fn flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp401_devirt")
        .unwrap_or(false)
}

/// Register the whole-class byte hook (idempotent; call once from
/// cplugin_init, BEFORE any kernel class loads — pristine capture at the
/// class's own load; the callback does NO class-file work on the loader
/// thread: stash before READY, serve the cached patch after).
pub fn register() {
    if !flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp401_devirt: dormant (set CRUSSTY_LEVER_FLAG=cmp401_devirt to enable)"
        );
        return;
    }
    let t: &'static Target = target_sed();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        if !t.ready() {
            eprintln!(
                "[crussty-plugin] cmp401_devirt: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes)
                    .map(|(m, _)| m)
                    .unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp401_devirt: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
    eprintln!("[crussty-plugin] cmp401_devirt: byte hook registered on {SED_CLASS}");
}

/// Wait until `name` is loaded in the JVM (find_class poll); after
/// `force_after_ms` force a kernel load (the class is side-effect-free
/// statics — LOGGER only — same pattern entity_compose uses for Entity).
fn wait_for_class(name: &str, deadline: std::time::Instant, force_after_ms: u64) -> bool {
    let started = std::time::Instant::now();
    loop {
        if cplug_sdk::classes::find_class(name).is_some() {
            return true;
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        if force_after_ms > 0
            && started.elapsed() > std::time::Duration::from_millis(force_after_ms)
        {
            eprintln!("[crussty-plugin] cmp401_devirt: forcing kernel load of {name}");
            crate::improved_noise::force_load_kernel_class(name);
        }
        let sighted = cplug_sdk::classes::is_sighted(name);
        std::thread::sleep(std::time::Duration::from_millis(if sighted {
            2_000
        } else {
            5_000
        }));
    }
}

/// Capture pristine bytes for a target: either the hook already stashed
/// them at class load, or pull them via no-op retransform (READY=false ->
/// stash-only).
fn capture_pristine(t: &'static Target) -> bool {
    if t.orig_is_some() {
        return true;
    }
    for _attempt in 1..=3 {
        let _ = cplug_sdk::retransform_class(t.name);
        if t.orig_is_some() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    t.orig_is_some()
}

fn strict_one(out: &crate::classfile::RetargetOutcome) -> bool {
    matches!(
        out,
        crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 }
    )
}

fn strict_two(out: &crate::classfile::RetargetOutcome) -> bool {
    matches!(
        out,
        crate::classfile::RetargetOutcome::Retargeted { sites: 2 }
            | crate::classfile::RetargetOutcome::AlreadyPatched { sites: 2 }
    )
}

/// Background activation: wait for boot-quiet, fuse the SynchedEntityData
/// READ side (D1) and the WRITE side (D1b) in ONE composed class patch,
/// arm the hook and retransform exactly once. Emits the ARM marker the
/// bench-пруф грепает: "cmp401_devirt: ARMED sites=<n>".
pub fn activate() {
    if !flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp401_devirt: boot marker not seen, hook stays dormant");
            return;
        }
        // Kernel loader quiet before any define/retransform runs (boot-time
        // class-loading storm discipline).
        std::thread::sleep(std::time::Duration::from_secs(15));

        let sed = target_sed();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        if !wait_for_class(SED_CLASS, deadline, 60_000) || !capture_pristine(sed) {
            eprintln!(
                "[crussty-plugin] cmp401_devirt: {SED_CLASS} not captured within 180s, stage dormant (vanilla)"
            );
            return;
        }

        let Some(orig) = sed.take_orig() else {
            eprintln!("[crussty-plugin] cmp401_devirt: no pristine bytes, stage dormant");
            return;
        };
        let orig_len = orig.len();

        // Compose D1 (get fusion) then D1b (set fusion) on the result. Any
        // rejection -> whole stage dormant (vanilla), fail-closed.
        let composed = match crate::classfile::patch_synched_data_get(&orig) {
            Ok((p1, o1)) if strict_one(&o1) => {
                eprintln!("[crussty-plugin] cmp401_devirt: D1 get-fusion ok ({o1:?})");
                match crate::classfile::patch_synched_data_set(&p1) {
                    Ok((p2, o2)) if strict_two(&o2) => {
                        eprintln!("[crussty-plugin] cmp401_devirt: D1b set-fusion ok ({o2:?})");
                        Some(p2)
                    }
                    Ok((_p, o2)) => {
                        eprintln!(
                            "[crussty-plugin] cmp401_devirt: D1b strict check violated ({o2:?}), stage stays dormant (vanilla)"
                        );
                        None
                    }
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] cmp401_devirt: D1b patch rejected ({e}), stage stays dormant (vanilla)"
                        );
                        None
                    }
                }
            }
            Ok((_p, o1)) => {
                eprintln!(
                    "[crussty-plugin] cmp401_devirt: D1 strict check violated ({o1:?}), stage stays dormant (vanilla)"
                );
                None
            }
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] cmp401_devirt: D1 patch rejected ({e}), stage stays dormant (vanilla)"
                );
                None
            }
        };

        let Some(patched) = composed else {
            return;
        };
        let composed_len = patched.len();
        sed.set_patch(patched);
        sed.ready.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(SED_CLASS);
        crate::kernel_policy::audit_wire(
            "net/minecraft/network/syncher/SynchedEntityData.get + .set",
            "devirt-fusion/read+write",
            "cmp401_devirt v1",
        );
        eprintln!(
            "[crussty-plugin] cmp401_devirt: ARMED sites=2 (D1 get + D1b set), {orig_len} -> {composed_len} bytes, retransform rc={rc}"
        );
    });
}
