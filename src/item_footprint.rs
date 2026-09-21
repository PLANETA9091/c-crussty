//! Runtime wiring for the ITEM-FOOTPRINT lever (round-397-g-footprint,
//! TASK-397-G — the items/ItemEntity.tick footprint lane; mechanism dropped
//! unimplemented from round-1 G, done properly here).
//!
//! TOP-1 bottleneck: items/ItemEntity.tick 31.17% java (bank v4 fp=4) with
//! the hot scalar fields age / pickupDelay scattered across ~150k heap
//! objects (one Entity-sized object region ahead of them each). This lever
//! packs the hot scalar state into ONE flat cache-line-dense arena and
//! front-loads the object fields, flag-gated:
//!
//!   * LEVEL 1 — field reorder (first-load only): `age` and `pickupDelay`
//!     move to the FRONT of the ItemEntity field table (HotSpot packs fields
//!     in size groups preserving declaration order within a group, so they
//!     head the int region of the ItemEntity section). SCHEMA change =>
//!     served ONLY at the first-load ClassFileLoadHook sighting, never via
//!     retransform (JVMTI redefinition forbids field changes).
//!   * LEVEL 2 — SoA bridge (retransform): every getfield/putfield of
//!     age/pickupDelay inside ItemEntity (48 sites, opcode-aware census) is
//!     retargeted 3B->3B to `ItemFootprintOps.getAge/putAge/getPickupDelay/
//!     putPickupDelay` — a java-static + sun.misc.Unsafe accessor over an
//!     off-heap stride-4-int arena OWNED BY RUST (one native `ensure` at
//!     init; 16 bytes per entity — age@+0, pickupDelay@+4, migration stamp
//!     @+8, 64B-aligned base, 4 entities per cache line; index = monotonic
//!     Entity id). Writes are write-through (`e.age = v` after the arena
//!     store) so out-of-class readers of the vanilla field (CraftBukkit
//!     mirrors) stay fresh. NO JNI on the hot path (the alloc_diet
//!     refutation: per-access JNI transitions lose at 150k x 6 accesses/tick).
//!     MIGRATION PARITY: entities predating the retarget (id <= hwm captured
//!     java-side from Entity.ENTITY_COUNTER at bridge init) are lazily
//!     migrated fields->arena on first post-retarget access (stamp-gated,
//!     one-shot) — without it the retarget would silently reset the bench
//!     population's pickupDelay=32767 (never-pickup) to arena-zero 0 and
//!     reset despawn timers of in-flight items.
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_footprint"` (the MEGA-ROUND
//! generic lever id; anything else = the EXACT vanilla path — no hook is
//! installed, no byte is touched, the module is byte-invisible).
//!
//! Fail-closed matrix: patcher Err (kernel shape mismatch / field renamed /
//! census disagreement) -> no retarget, hook serves vanilla bytes; bridge
//! define or RegisterNatives failure -> dormant; no pristine capture ->
//! dormant; arena allocation failure -> bridge degenerates to vanilla field
//! semantics inside the retarget (sticky, java-side).
//!
//! Ordering contract (why the two-phase hook is safe): the retarget patch is
//! computed from the PRISTINE bytes; if a reorder was already served at
//! first load, the retarget is applied ON TOP of the reordered bytes so the
//! live schema (reordered) and the retransformed schema stay identical —
//! redefinition sees a Code-only change.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

const ITEM_ENTITY: &str = crate::classfile::ITEM_ENTITY_CLASS;
const OPS_CLASS: &str = crate::classfile::ITEM_FOOTPRINT_OPS_CLASS;

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/item/ItemFootprintOps.class");

/// Rust-side flat arena: stride 8 bytes per entity (age@+0, pickupDelay@+4),
/// 64-byte aligned, calloc-zeroed, NEVER freed (leak-on-purpose — any
/// (base, cap) snapshot handed to the java side stays valid forever, which
/// is what makes the unsynchronized java-side (base, cap) read safe).
struct ArenaState {
    /// 64B-aligned arena base (usize for Send; handed to java as jlong).
    base: usize,
    cap_bytes: usize,
}

static ARENA: Mutex<Option<ArenaState>> = Mutex::new(None);

fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq_ignore_ascii_case("items_footprint"))
        .unwrap_or(false)
}

static READY: AtomicBool = AtomicBool::new(false);

/// Set while the predate-capture no-op retransform runs: the hook must
/// STASH the bytes but NEVER serve the field reorder through retransform —
/// a schema change is rejected by the JVM (JVMTI redefinition law) and the
/// rejected-but-served reorder would poison the whole lever (the follow-up
/// retarget would be computed against a reordered schema that is not live).
static PREDATES_CAPTURE: AtomicBool = AtomicBool::new(false);

struct Target {
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served_reorder: AtomicBool,
}

impl Target {
    fn new() -> Self {
        Self {
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served_reorder: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        let mut orig = self.orig.lock().unwrap_or_else(PoisonError::into_inner);
        if orig.is_none() {
            *orig = Some(bytes.to_vec());
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig.lock().unwrap_or_else(PoisonError::into_inner).is_some()
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
    fn set_patch(&self, bytes: Arc<[u8]>) {
        *self.patch.lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(Target::new)
}

/// The single rust-side native: allocate (or grow) the arena so it holds at
/// least `min_entities` slots (stride 16B: age@+0, pickupDelay@+4, stamp@+8);
/// returns the 64B-aligned base or 0 on failure. Monotonic: previous arenas
/// are copied, never freed.
///
/// # Safety
/// JNI entry — called by the JVM on an attached thread; no JVM state touched
/// (pure memory op), panics contained to 0 (never unwind across JNI).
unsafe extern "system" fn Java_net_minecraft_world_entity_item_ItemFootprintOps_ensure(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    min_entities: jni::jint,
) -> jni::jlong {
    if min_entities <= 0 {
        return 0;
    }
    let need_bytes = match usize::try_from(min_entities) {
        Ok(n) => match n.checked_mul(16) {
            Some(b) if b <= (1usize << 33) => b, // hard cap 8GiB: refuse, java falls back to fields
            _ => return 0,
        },
        Err(_) => return 0,
    };
    std::panic::catch_unwind(|| {
        let mut g = ARENA.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(a) = g.as_ref() {
            if a.cap_bytes >= need_bytes {
                return a.base as jni::jlong;
            }
        }
        // # Safety: layout is non-zero, 8-byte-multiple, 64B-aligned.
        let layout = match std::alloc::Layout::from_size_align(need_bytes, 64) {
            Ok(l) => l,
            Err(_) => return 0 as jni::jlong,
        };
        let fresh = unsafe { std::alloc::alloc_zeroed(layout) } as usize;
        if fresh == 0 {
            return 0 as jni::jlong;
        }
        match g.as_mut() {
            Some(old) => {
                // Copy the old arena contents; the old block leaks on purpose.
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        old.base as *const u8,
                        fresh as *mut u8,
                        old.cap_bytes.min(need_bytes),
                    )
                };
                old.base = fresh;
                old.cap_bytes = need_bytes;
            }
            None => {
                *g = Some(ArenaState {
                    base: fresh,
                    cap_bytes: need_bytes,
                });
            }
        }
        fresh as jni::jlong
    })
    .unwrap_or(0)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise / flush_diet): the callback does
/// NO JNI work; it stashes pristine bytes and (LEVEL 1) serves the field
/// reorder at the class's own first load — the reorder is a pure classfile
/// splice with no bridge dependency, so it is legal and cheap right there.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] item_footprint: dormant (lever_flag != items_footprint)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t_name(), move |_name, bytes| {
        let t = target();
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] item_footprint: pristine sighting {ITEM_ENTITY} {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            // Capture-only sighting (predate-capture retransform): stash
            // ONLY — serving the schema reorder through retransform would be
            // rejected by the JVM and poison the lever.
            if PREDATES_CAPTURE.load(Ordering::Acquire) {
                eprintln!(
                    "[crussty-plugin] item_footprint: capture-only sighting, reorder withheld"
                );
                return None;
            }
            // LEVEL 1 (first-load only): hot fields to the front of the
            // field table. Failure here = vanilla bytes served, dormant.
            return match crate::classfile::reorder_item_entity_fields(bytes) {
                Ok((reordered, n)) => {
                    eprintln!(
                        "[crussty-plugin] item_footprint: level-1 reorder served at first load ({n} hot fields -> front)"
                    );
                    t.served_reorder.store(true, Ordering::Release);
                    Some(reordered)
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] item_footprint: reorder rejected ({e}), first-load bytes stay vanilla"
                    );
                    None
                }
            };
        }
        // Retransform sighting (or repeat): serve the computed retarget patch.
        t.patch_bytes().map(|c| c.to_vec())
    });
    // Predate-capture gate is per-campaign, not per-sighting: the capture
    // window opens only inside activate()'s no-op retransform loop.
    let _ = PREDATES_CAPTURE;
}

fn t_name() -> &'static str {
    ITEM_ENTITY
}

/// Background activation: wait for the class + quiet boot, define the
/// ItemFootprintOps bridge into the kernel loader, RegisterNatives the
/// arena `ensure`, compute the retarget patch on top of whatever schema is
/// live (reordered if level-1 served), flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ITEM_ENTITY).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] item_footprint: {ITEM_ENTITY} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] item_footprint: forcing kernel load of {ITEM_ENTITY}"
                );
                crate::improved_noise::force_load_kernel_class(ITEM_ENTITY);
            }
            let sighted = cplug_sdk::classes::is_sighted(ITEM_ENTITY);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] item_footprint: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] item_footprint: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] item_footprint: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_item_footprint_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (flush_diet pattern) and
        // RegisterNatives the arena ensure() while the local ref is live.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity")
            else {
                return None;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return None;
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
                return None;
            };
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None;
            }
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(ops_cls) => {
                    // RegisterNatives ensure(I)J before dropping the local ref.
                    let name = std::ffi::CString::new("ensure").ok()?;
                    let sig = std::ffi::CString::new("(I)J").ok()?;
                    let natives = [jni::JNINativeMethod {
                        name: name.as_ptr(),
                        signature: sig.as_ptr(),
                        fnPtr: Java_net_minecraft_world_entity_item_ItemFootprintOps_ensure
                            as *const std::ffi::c_void as *mut std::ffi::c_void,
                    }];
                    if let Err(code) = env.register_natives(ops_cls, &natives) {
                        env.exception_clear();
                        eprintln!(
                            "[crussty-plugin] item_footprint: register_natives(ensure) failed (code {code})"
                        );
                        env.delete_local_ref(ops_cls);
                        return None;
                    }
                    env.delete_local_ref(ops_cls);
                    Some(())
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!(
                        "[crussty-plugin] item_footprint: define_class({OPS_CLASS}) failed"
                    );
                    None
                }
            }
        });
        if defined.is_none() {
            eprintln!(
                "[crussty-plugin] item_footprint: bridge definition aborted, hook stays dormant"
            );
            return;
        }

        // Pristine bytes for a class that predates the hook (fast boot):
        // no-op retransform capture, fluid_guard pattern. The capture window
        // is flagged so the hook stashes bytes WITHOUT serving the level-1
        // reorder (a schema change is illegal through retransform).
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] item_footprint: {ITEM_ENTITY} predates hook, capturing via no-op retransform"
            );
            PREDATES_CAPTURE.store(true, Ordering::Release);
            for attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(ITEM_ENTITY);
                if t.orig_is_some() {
                    break;
                }
                let _ = attempt;
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            PREDATES_CAPTURE.store(false, Ordering::Release);
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] item_footprint: no pristine bytes for {ITEM_ENTITY}, hook stays dormant"
                );
                return;
            }
        }
        let Some(original) = t.take_orig() else {
            return;
        };

        // Retarget on top of the LIVE schema: reordered bytes if (and only
        // if) level-1 was served at first load — otherwise vanilla bytes
        // (a retransform must never change the field table).
        let base_bytes: Vec<u8> = if t.served_reorder.load(Ordering::Acquire) {
            match crate::classfile::reorder_item_entity_fields(&original) {
                Ok((reordered, _)) => reordered,
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] item_footprint: reorder step rejected ({e}), retarget on vanilla schema"
                    );
                    original.clone()
                }
            }
        } else {
            original.clone()
        };

        let (patched, outcome, census) = match crate::classfile::patch_item_footprint(&base_bytes) {
            Ok(triple) => triple,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] item_footprint: patch rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let retargeted = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !retargeted {
            eprintln!(
                "[crussty-plugin] item_footprint: unexpected patch outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] item_footprint: computed patch for {ITEM_ENTITY} ({} -> {} bytes, {outcome:?})",
            base_bytes.len(),
            patched.len()
        );
        eprintln!(
            "[crussty-plugin] item_footprint: census (opcode-aware) getfield age={} putfield age={} getfield pickupDelay={} putfield pickupDelay={} (invokespecial class-wide={}, 0 on hot-field sites; {} sites retargeted)",
            census.getfield_age,
            census.putfield_age,
            census.getfield_pickup_delay,
            census.putfield_pickup_delay,
            census.invokespecial_class_wide,
            census.hot_sites()
        );
        t.set_patch(Arc::from(patched));

        crate::kernel_policy::audit_wire(OPS_CLASS, "getAge", "item_footprint v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(ITEM_ENTITY);
        eprintln!(
            "[crussty-plugin] item_footprint: {ITEM_ENTITY} armed (level-2 SoA retarget), retransform rc={rc}"
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The lever MUST be byte-invisible unless CRUSSTY_LEVER_FLAG exactly
    /// names it (round contract: empty/foreign flag = exact vanilla path).
    #[test]
    fn gate_is_exact() {
        std::env::remove_var("CRUSSTY_LEVER_FLAG");
        assert!(!enabled());
        std::env::set_var("CRUSSTY_LEVER_FLAG", "");
        assert!(!enabled());
        std::env::set_var("CRUSSTY_LEVER_FLAG", "items_soa");
        assert!(!enabled());
        std::env::set_var("CRUSSTY_LEVER_FLAG", "items_footprint");
        assert!(enabled());
        std::env::remove_var("CRUSSTY_LEVER_FLAG");
    }

    /// Bridge bytes must be present and parse (major guard sanity).
    #[test]
    fn bridge_bytes_parse() {
        assert_eq!(
            crate::improved_noise::class_version(OPS_BYTES).map(|(m, _)| m),
            Some(65)
        );
    }
}

#[cfg(test)]
mod kernel_tests {
    use super::*;

    /// End-to-end byte check against the real kernel class (skipped unless
    /// CRUSSTY_ITEM_ENTITY_CLASS points at the extracted ItemEntity.class):
    /// run BOTH transforms, re-run both (idempotent / stable), verify the
    /// 48-site census contract on the actual kernel bytes.
    #[test]
    fn real_kernel_item_entity_patch() {
        let path = match std::env::var("CRUSSTY_ITEM_ENTITY_CLASS") {
            Ok(p) if !p.is_empty() => p,
            _ => return, // kernel class not present in this environment: skip
        };
        let class_bytes = std::fs::read(&path).expect("ItemEntity.class reads");

        // Level 1: reorder (2 hot fields moved to front, length-neutral).
        let (reordered, n) = crate::classfile::reorder_item_entity_fields(&class_bytes)
            .expect("reorder on real kernel bytes");
        assert_eq!(n, 2);
        assert_eq!(reordered.len(), class_bytes.len(), "reorder must be length-neutral");

        // Level 2: retarget all census sites.
        let (patched, outcome, census) = crate::classfile::patch_item_footprint(&reordered)
            .expect("patch on real kernel bytes");
        match outcome {
            crate::classfile::RetargetOutcome::Retargeted { sites } => {
                assert_eq!(sites, 48, "census contract: 48 sites in the kernel class");
            }
            other => panic!("unexpected outcome {other:?}"),
        }
        assert_eq!(census.hot_sites(), 48);
        assert_eq!(census.getfield_age, 12);
        assert_eq!(census.putfield_age, 10);
        assert_eq!(census.getfield_pickup_delay, 15);
        assert_eq!(census.putfield_pickup_delay, 11);
        assert!(patched.len() > reordered.len(), "pool grew");

        // Idempotency: re-patching patched bytes is AlreadyPatched; the
        // reorder is stable on the patched schema.
        let (again, outcome2, _c2) =
            crate::classfile::patch_item_footprint(&patched).expect("re-patch");
        assert!(matches!(
            outcome2,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
        assert_eq!(again, patched);
        let (_r2, n2) = crate::classfile::reorder_item_entity_fields(&patched)
            .expect("reorder stable on patched bytes");
        assert_eq!(n2, 2);

        // Optional byte dump for the JVM-verifier harness (scripts/jvm
        // verify pass): set CRUSSTY_ITEM_ENTITY_DUMP=<prefix> to write
        // <prefix>.reordered.class / <prefix>.patched.class.
        if let Ok(prefix) = std::env::var("CRUSSTY_ITEM_ENTITY_DUMP") {
            if !prefix.is_empty() {
                std::fs::write(format!("{prefix}.reordered.class"), &reordered)
                    .expect("dump reordered");
                std::fs::write(format!("{prefix}.patched.class"), &patched).expect("dump patched");
            }
        }
    }
}
