//! Runtime wiring for the COLLIDE BATCH-MERGE lever (TASK-401-B, round-401
//! vector B — cmp401_collide): per-tick per-worker SECTION PLAN cache for the
//! kernel block-collision scan (collide 8.68-9.45% java @150k ×
//! region_threads=4 — BOTTLENECK/RESEARCH-B).
//!
//! ARCHITECTURE (zero_cursor/nav_sys precedent — whole-body static redirect):
//! 1. Bridge: `net/minecraft/world/entity/CollideBatchOps.java` (defined into
//!    the kernel loader at boot+quiet). `blockCollisions` mirrors the
//!    round-400-F javap port of
//!    `CollisionUtil.getCollisionsForBlocksOrWorldBorder` VERBATIM below the
//!    plan threshold and serves dense sections from a per-tick per-worker
//!    section plan — the DELTA mechanism vs round-400-F is the INTER-QUERY
//!    dedup of block reads (thousands of entities scan the SAME sections per
//!    tick; the plan is built once per tick per section per worker and walked
//!    with zero paletted reads). Entity.collide stays vanilla; consumers are
//!    order-free (min/max composition + boolean) — parity contract in the
//!    bridge javadoc.
//! 2. Byte hook: `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil`
//!    pristine capture at first load; after the bridge is defined the hook
//!    serves the WHOLE-BODY REDIRECT of the static scan to
//!    `CollideBatchOps.blockCollisions` (identical erased descriptor —
//!    stack shape unchanged; classfile::patch_collision_batch).
//! 3. Fail-closed: patch pattern mismatch → pristine bytes → vanilla scan;
//!    bridge definition failure → hook stays dormant; plan table overflow →
//!    OFF_VANILLA per section (vanilla fragment inside the bridge). Vanilla
//!    behavior by construction; empty flag = this module never arms and the
//!    bridge class is never defined (vanilla bit-for-bit).
//!
//! ARM markers (server stdout):
//!   "[crussty-plugin] cmp401_collide: defined net/minecraft/world/entity/CollideBatchOps in kernel loader"
//!   "[crussty-plugin] cmp401_collide: ARMED (section-plan batch-merge; retransform rc=...)"
//!   "[crussty-plugin] cmp401_collide: PATCHED CollisionUtil.getCollisionsForBlocksOrWorldBorder (...)"
//!   "[crussty-plugin] cmp401_collide: applied (blockCollisions; plan = per-tick per-worker, DYNAMIC entries re-resolve vanilla)"

use crate::classfile;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const TARGET_CLASS: &str = "ca/spottedleaf/moonrise/patches/collisions/CollisionUtil";
const OPS_CLASS: &str = "net/minecraft/world/entity/CollideBatchOps";
const SCAN_NAME: &str = "getCollisionsForBlocksOrWorldBorder";
const SCAN_DESC: &str = classfile::CB_SCAN_DESC;

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/CollideBatchOps.class");

static READY: AtomicBool = AtomicBool::new(false);

struct Target {
    orig: Mutex<Option<Vec<u8>>>,
    patch: Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

fn target() -> &'static Target {
    static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
    TARGET.get_or_init(|| Target {
        orig: Mutex::new(None),
        patch: Mutex::new(None),
        served: AtomicBool::new(false),
    })
}

fn stash_orig(bytes: &[u8]) {
    let mut g = target().orig.lock().unwrap();
    if g.is_none() {
        *g = Some(bytes.to_vec());
    }
}

fn lever_flag_matches() -> bool {
    // TASK-403-C: tickplane включает collide-сегмент плейна (STRICT eq).
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == "cmp401_collide" || v == "cmp403_tickplane" || v == "cmp405_stagtick"
                // TASK-406-D: композит раунда-406 включает collide-сегмент.
                || v == "cmp406_aibatch"
                // TASK-406-E: композит раунда-406 включает collide-сегмент.
                || v == "cmp406_sscan"
                // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                || v == "cmp409_multi" || v == "cmp412_meganav" || v == "cmp414_cvs"
                // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
                || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq"
                // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
                || v == "cmp420_colpush"
                || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2" || v == "cmp436_ins4"
                || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp450_chunk" || v == "cmp457_noisesimd" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
                || v == "cmp438_sense" // TASK-444-C: sense family union
                || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" || v == "cmp457_noisesimd" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
        })
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Chain-safe with alloc_diet's hook on the SAME class (hooks fire in
/// registration order; each receives the previous output — alloc_diet is
/// dormant without CRUSSTY_ALLOC_DIET and passes vanilla bytes through).
pub fn register() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp401_collide: dormant (lever_flag != cmp401_collide, vanilla scan)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(TARGET_CLASS, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Acquire) {
            stash_orig(bytes);
            return None;
        }
        let cached = t.patch.lock().unwrap().clone();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] cmp401_collide: hook serve {TARGET_CLASS} {} bytes",
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for a quiet boot, define the bridge into the
/// kernel loader, verify the resolution closure, compute the whole-body
/// redirect from pristine bytes, flip READY, retransform CollisionUtil.
pub fn activate() {
    if !lever_flag_matches() {
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] cmp401_collide: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

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
                "[crussty-plugin] cmp401_collide: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

        // S7-164 NoSuchMethodError-storm guard: the delivered CollideBatchOps
        // classfile must declare blockCollisions exactly (redirect target).
        if let Err(e) = classfile::collidebatch_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] cmp401_collide: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the KERNEL loader (its dependencies —
        // moonrise collision patches, PalettedContainer, AABB — all live
        // there; Entity is the loader anchor, same as the nav bridge).
        let defined = define_bridge();
        if !defined {
            eprintln!(
                "[crussty-plugin] cmp401_collide: bridge definition failed, hook stays dormant"
            );
            return;
        }
        eprintln!("[crussty-plugin] cmp401_collide: defined {OPS_CLASS} in kernel loader");

        // Pristine bytes for a class that predates the hook registration
        // window: no-op retransform capture (fluid_guard pattern).
        if stash_none() {
            eprintln!(
                "[crussty-plugin] cmp401_collide: {TARGET_CLASS} predates hook, capturing via no-op retransform"
            );
            for _attempt in 1..=5 {
                let _ = cplug_sdk::retransform_class(TARGET_CLASS);
                if !stash_none() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            if stash_none() {
                eprintln!(
                    "[crussty-plugin] cmp401_collide: no pristine bytes for {TARGET_CLASS}, hook stays dormant"
                );
                return;
            }
        }
        let Some(original) = take_orig() else {
            return;
        };
        let (patched, outcome) = match classfile::patch_collision_batch(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] cmp401_collide: redirect rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let redirected = matches!(
            outcome,
            classfile::RetargetOutcome::Retargeted { .. }
                | classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !redirected {
            eprintln!(
                "[crussty-plugin] cmp401_collide: unexpected redirect outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] cmp401_collide: PATCHED {TARGET_CLASS}.{SCAN_NAME} ({} -> {} bytes; whole-body redirect -> CollideBatchOps.blockCollisions, section-plan batch-merge)",
            original.len(),
            patched.len()
        );
        *target().patch.lock().unwrap() = Some(Arc::from(patched));
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(TARGET_CLASS);
        eprintln!(
            "[crussty-plugin] cmp401_collide: ARMED (section-plan batch-merge; retransform rc={rc})"
        );
        // TASK-403-C: сегментный маркер плейна с фактическим флагом раунда.
        if std::env::var("CRUSSTY_LEVER_FLAG")
            .map(|v| {
                let v = v.trim();
                v == "cmp403_tickplane" || v == "cmp405_stagtick"
                    // TASK-406-D: композит раунда-406 (сегментный маркер).
                    || v == "cmp406_aibatch"
                    // TASK-406-E: композит раунда-406 (сегментный маркер).
                    || v == "cmp406_sscan"
                    // TASK-409: мультикомпозит.
                    || v == "cmp409_multi" || v == "cmp412_meganav" || v == "cmp414_cvs"
                // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
                || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2" || v == "cmp436_ins4"
                || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp450_chunk" || v == "cmp457_noisesimd" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
                || v == "cmp438_sense" // TASK-444-C: sense family union
                || v == "cmp451_senseins" || v == "cmp453_diet" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" || v == "cmp457_noisesimd" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
            })
            .unwrap_or(false)
        {
            eprintln!(
                "[crussty-plugin] {} segment collide-batch ARMED (retransform rc={rc})",
                std::env::var("CRUSSTY_LEVER_FLAG").unwrap_or_default().trim()
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        if target().served.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp401_collide: applied (blockCollisions; plan = per-tick per-worker, THRESH 24, DYNAMIC entries re-resolve vanilla per query)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp401_collide: NOT APPLIED after retransform (kernel build mismatch?) — vanilla behavior"
            );
        }
    });
}

fn stash_none() -> bool {
    target().orig.lock().unwrap().is_none()
}

fn take_orig() -> Option<Vec<u8>> {
    target().orig.lock().unwrap().take()
}

fn define_bridge() -> bool {
    let defined = cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity") else {
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
            eprintln!("[crussty-plugin] cmp401_collide: define_class({OPS_CLASS}) failed");
            return false;
        };
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        true
    });
    defined.unwrap_or(false)
}

#[cfg(test)]
mod collidebatch_delivery_tests {
    use super::OPS_BYTES;

    /// S7-163 delivery-graph guard: the bridge source MUST declare ZERO
    /// nested classes (the classfile is defined ALONE into the kernel loader;
    /// a nested class would detonate as NoClassDefFoundError on the first
    /// collision scan — offline javac cannot catch a missing nested
    /// classfile).
    #[test]
    fn collidebatch_source_declares_no_nested_classes() {
        let src = include_str!("../entityinside/net/minecraft/world/entity/CollideBatchOps.java");
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") {
                        panic!("nested declaration in bridge source: {t}");
                    }
                }
            }
        }
    }

    /// The embedded bytes must exist and be a real classfile (build step ran;
    /// major 65 = kernel JVM).
    #[test]
    fn collidebatch_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// S7-164 resolution-closure guard pinned to the REAL bytes: the bridge
    /// must declare `blockCollisions` with the exact CB_SCAN_DESC the
    /// bytecode surgery redirects to — member drift fails here, offline,
    /// instead of silently disarming the leg.
    #[test]
    fn collidebatch_resolution_closure_accepts_embedded_bridge() {
        crate::classfile::collidebatch_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded CollideBatchOps bytes");
    }

    /// Roundtrip on the REAL kernel classfile fixture: exactly the one scan
    /// site redirects; a re-sight is AlreadyPatched (idempotent retransform
    /// cycles must not corrupt); a foreign class fails closed NotFound with
    /// original bytes.
    #[test]
    fn collidebatch_redirect_roundtrip_on_kernel_fixture() {
        const COLLISION: &[u8] = include_bytes!("../tests/fixtures/CollisionUtil.class");
        let (out, outcome) = crate::classfile::patch_collision_batch(COLLISION)
            .expect("redirect must apply to the real CollisionUtil fixture");
        assert!(matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
        ));
        assert!(out.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // The helper rewrites the Code attribute (6-byte load chain +
        // invokestatic) and drops LineNumber/LocalVariable debug tables, so
        // the delivered file can be SMALLER than the original — assert the
        // body actually changed instead.
        assert_ne!(out, COLLISION.to_vec());
        // Idempotent re-sight (retransform cycle on already-patched bytes).
        let (_, again) = crate::classfile::patch_collision_batch(&out)
            .expect("repatch must not error");
        assert!(matches!(
            again,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
        // Foreign class: fail closed, bytes untouched.
        let FOREIGN: &[u8] = include_bytes!("../tests/fixtures/PalettedContainer.class");
        let (same, nf) = crate::classfile::patch_collision_batch(FOREIGN)
            .expect("notfound path must not error");
        assert_eq!(nf, crate::classfile::RetargetOutcome::NotFound);
        assert_eq!(same, FOREIGN.to_vec());
    }
}
