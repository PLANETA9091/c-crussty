//! Runtime wiring for the CHUNK-PARSE SECTION-CACHE lever (TASK-419-C base,
//! TASK-420-C stability deepening, lever cmp420_chunk2, law 8 chunk-loading
//! axis — see chunkparse/net/minecraft/world/level/chunk/storage/
//! ChunkParseOps.java).
//!
//! RECON-13b/13f ground truth: the SerializableChunkData.parse decode path
//! is the TOP alloc lane of the 150k scene — 33.38% of ALL alloc bytes in
//! the 240-300s chunk-reload burst window (45.04% in s7165 burst vs 8.70%
//! steady s7168); decomposition: codec machinery 19.06% (DataResult ←
//! ResourceLocation.read / NbtOps.getMap / MapDecoder.compressedDecode) +
//! paletted-decode 13.98% (ShortArrayList.grow ← SimpleBitStorage).
//!
//! The lever redirects the vanilla block_states section-decode lambda
//! `SerializableChunkData.lambda$parse$5(Codec, ChunkPos, int, CompoundTag)`
//! (bootstrap-verified: indy#4 -> bootstrap#4 -> lambda$parse$5, called
//! right after `ldc "block_states"`) to `ChunkParseOps.parseSection` —
//! a cache-first decoder:
//!   * HIT (same codec instance + structurally equal tag): template.copy()
//!     — the codec/MapDecoder/DataResult/palette/SimpleBitStorage machinery
//!     is skipped entirely;
//!   * MISS: the pristine twin lambda `lambda$parse$7` (biomes site, the
//!     SAME canonical descriptor, byte-identical body semantics, NOT
//!     patched) is invoked reflectively — the exact vanilla decode incl.
//!     promotePartial logErrors + ChunkReadException error path
//!     (parity by construction) — then its result is copied into the cache.
//!
//! Parity contract (law 4): empty lever flag -> hooks never registered, the
//! class is byte-identical vanilla (dormant-invisible). Codec key is
//! IDENTITY-based — anti-xray preset codecs (a fresh codec object per
//! section, ChunkPacketBlockController.getPresetBlockStates != null branch)
//! can never cross-hit; the shared factory codec (presets == null) hits.
//!
//! Delivery: ChunkParseOps is defined ALONE into the kernel loader (flat
//! classfile; resolution closure pins the flat contract). The rust
//! activator injects the twin lambda name via ChunkParseOps.init(String)
//! BEFORE READY flips. Fail-closed: every guard defect leaves the hook
//! dormant (vanilla parse, zero risk).
//!
//! JNI discipline (law 6): ZERO added JNI crossings on the hot path — the
//! plane is Java-side (cache + reflective twin); the activator uses exactly
//! one JNI call (init) at boot. Per-chunk <= 1 satisfied trivially (0).
//!
//! ARM marker: "[crussty-plugin] cmp420_chunk2: ARMED chunk-parse
//! section-cache"; EFFECT markers (Java side): "parse-cache first hit" +
//! "parse-cache selftest PASS". Grep marker: "cmp420_chunk2".

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::classfile::{
    chunkparse_pristine_guard, chunkparse_resolution_closure, CHUNKPARSE_BLOCKS_LAMBDA,
    CHUNKPARSE_OPS_CLASS, CHUNKPARSE_SECTION_LAMBDA_DESC, CHUNKPARSE_TARGET_CLASS,
    CHUNKPARSE_TWIN_LAMBDA,
};

const OPS_BYTES: &[u8] = include_bytes!(
    "../chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class"
);

const LEVER_ID: &str = "cmp420_chunk2";

struct Target {
    name: &'static str,
    orig: Mutex<Option<Vec<u8>>>,
    patch: Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: Mutex::new(None),
            patch: Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }

    fn stash_orig(&self, bytes: &[u8]) {
        let mut g = self.orig.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if g.is_none() {
            *g = Some(bytes.to_vec());
        }
    }

    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn set_patch(&self, bytes: Arc<[u8]>) {
        *self
            .patch
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(bytes);
    }

    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }
}

fn target() -> &'static Target {
    static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
    TARGET.get_or_init(|| Target::new(CHUNKPARSE_TARGET_CLASS))
}

/// MEGA-CARRIER gate (TASK-420 mega, law 7 composition): CRUSSTY_LEVER_FLAG ==
/// "cmp420_chunk2" (own wave id) OR "cmp420_colpush" (the collide+push
/// composite carrier — chunk-parse plane rides it as a disjoint-lane leg).
/// TASK-421-MEGA adds "cmp421_brain" (mega carrier, law 7: brain+chunk
/// disjoint-lane composition rides the same union pattern as tick-420).
/// TASK-421-C adds "cmp421_chunk" (stabilized chunk-axis round; the STRICT
/// union carries the round id, no broadening: empty/foreign flag = vanilla
/// bit-in-bit — no env duplicates by design, RESEARCH-C-419).
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == LEVER_ID || v == "cmp420_colpush" || v == "cmp421_chunk" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3"
        })
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline: the callback performs NO JNI work — pristine
/// capture at the class's own load, patch served from the precomputed cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla chunk parse)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

static READY: AtomicBool = AtomicBool::new(false);

/// Delivery test for offline/CI: the embedded bridge must pass the
/// resolution closure as compiled.
#[allow(dead_code)] // CI/self-diagnosis helper; the delivery tests pin the closure
pub fn delivery_closure_ok() -> bool {
    chunkparse_resolution_closure(OPS_BYTES).is_ok()
}

/// Define ChunkParseOps into the kernel loader (anchored at the target
/// class's own loader) and inject the twin lambda name via ChunkParseOps.
/// init(String) on the LOCAL define_class ref.
///
/// Return codes:
///   * Some(true)  — defined AND twin injected;
///   * Some(false) — hard failure (define/init defect; retrying pointless);
///   * None        — transient (anchor class not reachable yet; retry).
fn define_bridge_and_inject_twin() -> Option<bool> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(CHUNKPARSE_TARGET_CLASS) else {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: kernel class {CHUNKPARSE_TARGET_CLASS} not reachable yet"
            );
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
        let Some(c) = env.define_class(CHUNKPARSE_OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: define_class({CHUNKPARSE_OPS_CLASS}) failed"
            );
            return Some(false);
        };
        // init(twin) — the twin name is injected BEFORE the retransform can
        // serve a MISS (the bridge would throw loudly otherwise; never
        // reached in correct delivery). One JNI static call at boot total.
        let ok = env
            .get_static_method_id(c, "init", "(Ljava/lang/String;)V")
            .and_then(|mid| {
                env.new_string(CHUNKPARSE_TWIN_LAMBDA).map(|jname| {
                    env.call_static_void_method(c, mid, &[jni::jvalue { l: jname }]);
                    !crate::clear_exception(env)
                })
            })
            .unwrap_or_else(|| {
                crate::clear_exception(env);
                false
            });
        if !ok {
            eprintln!("[crussty-plugin] {LEVER_ID}: ChunkParseOps.init call failed");
        }
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(ok)
    })
    .flatten()
}

/// Background activation: wait for boot, define ChunkParseOps into the
/// KERNEL loader (anchor = the target class's own loader), inject the twin
/// lambda name via ChunkParseOps.init(String) on the LOCAL define_class ref
/// (queryplane find_class-lesson: a freshly-defined class is not
/// INITIALIZED-filter-visible to the JVMTI scan), capture pristine bytes,
/// run the pristine guard, compute the static body-redirect, flip READY,
/// retransform exactly once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] {LEVER_ID}: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: server booted, defining {CHUNKPARSE_OPS_CLASS} into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let bridge_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if bridge_major > jvm_major {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: bridge is class major {bridge_major} but JVM supports up to {jvm_major} — rebuild chunkparse/ via scripts/build_chunkparse_ops.sh; hook stays dormant"
            );
            return;
        }

        // RESOLUTION CLOSURE GUARD: parseSection carries the EXACT vanilla
        // lambda descriptor (stack-shape contract), init(String)V exists and
        // the bridge is FLAT (no nested classes).
        if let Err(e) = chunkparse_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader + inject the twin name
        // (ONE JNI static call on the LOCAL ref; first static call also
        // initializes the class per JVMS 5.5). Retried: the anchor class is
        // loaded during spawn-chunk load (before the boot marker), but a
        // hotpatch attach racing a quiet boot must not lose the lever.
        let mut defined = false;
        for attempt in 1..=10 {
            match define_bridge_and_inject_twin() {
                Some(true) => {
                    defined = true;
                    break;
                }
                Some(false) => break, // hard failure, retrying will not help
                None => {
                    if attempt == 10 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(3));
                }
            }
        }
        if !defined {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: bridge definition failed, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: defined {CHUNKPARSE_OPS_CLASS} in kernel loader, twin={CHUNKPARSE_TWIN_LAMBDA} injected"
        );

        // Pristine bytes for a class that may predate the hook: no-op
        // retransform capture (zero_cursor/fluid_guard pattern).
        if t.take_orig().is_none() {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _attempt in 1..=5 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.take_orig().is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            if t.take_orig().is_none() {
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };

        // PRISTINE GUARD: the kernel shape must be the javap-verified one
        // (parse owner + both section lambdas + exactly-2 candidates) BEFORE
        // any patch is served.
        if let Err(e) = chunkparse_pristine_guard(&original) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: PRISTINE GUARD FAILED: {e} — hook stays dormant"
            );
            return;
        }

        let (patched, outcome) = match crate::classfile::redirect_static_method_body_to_static(
            &original,
            CHUNKPARSE_BLOCKS_LAMBDA,
            CHUNKPARSE_SECTION_LAMBDA_DESC,
            CHUNKPARSE_OPS_CLASS,
            "parseSection",
            CHUNKPARSE_SECTION_LAMBDA_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: redirect rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let redirected = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !redirected {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: unexpected redirect outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: computed redirect for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: ARMED chunk-parse section-cache (deep: cap 16384, evict-half, lock-free CHM probe; blocks lambda {CHUNKPARSE_BLOCKS_LAMBDA} -> ChunkParseOps.parseSection, twin {CHUNKPARSE_TWIN_LAMBDA} pristine; identity-codec key, template.copy() HIT path, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod chunkparse_delivery_tests {
    use super::OPS_BYTES;
    use crate::classfile::{
        chunkparse_pristine_guard, chunkparse_resolution_closure, CHUNKPARSE_OPS_CLASS,
        CHUNKPARSE_SECTION_LAMBDA_DESC,
    };

    /// s7171 delivery-graph guard mirrored: the bridge source MUST declare
    /// ZERO nested classes (the classfile is defined alone into the kernel
    /// loader; a nested class would detonate as NoClassDefFoundError on the
    /// first parse — the offline harness cannot catch a missing nested
    /// classfile).
    #[test]
    fn chunkparse_source_declares_no_nested_classes() {
        let src = include_str!("../chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java");
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if (before.contains("static") && !before.contains("//"))
                        || t.starts_with("public static final class")
                    {
                        panic!("nested declaration in bridge source: {t}");
                    }
                }
            }
        }
    }

    /// The embedded bytes must exist and be a real classfile pinned to
    /// major 65 (kernel JVM).
    #[test]
    fn chunkparse_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// s7171-style: the resolution closure MUST accept the embedded bridge
    /// classfile as compiled (parseSection descriptor + init + flat) so any
    /// future member drift fails offline instead of silently disarming.
    #[test]
    fn chunkparse_resolution_closure_accepts_embedded_bridge() {
        chunkparse_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded ChunkParseOps bytes");
    }

    /// The bridge parseSection descriptor must be the EXACT canonical
    /// section-lambda descriptor (redirect stack-shape contract).
    #[test]
    fn chunkparse_bridge_descriptor_is_canonical() {
        chunkparse_resolution_closure(OPS_BYTES).expect("closure");
        // The closure itself pins parseSection to the canonical descriptor;
        // this assertion documents the contract explicitly.
        assert!(CHUNKPARSE_SECTION_LAMBDA_DESC.starts_with("(Lcom/mojang/serialization/Codec;"));
        assert!(CHUNKPARSE_OPS_CLASS.ends_with("ChunkParseOps"));
    }

    /// REAL kernel fixture: the pristine guard must accept the actual
    /// SerializableChunkData bytes (tests/fixtures/SerializableChunkData.class
    /// is sha256-identical to the round-396-a patched-kernel.jar entry —
    /// same fixture source as every other delivery test).
    #[test]
    fn chunkparse_pristine_guard_accepts_kernel_fixture() {
        let fixture = include_bytes!("../tests/fixtures/SerializableChunkData.class");
        chunkparse_pristine_guard(fixture)
            .expect("pristine guard must accept the kernel fixture bytes");
    }

    /// The redirect itself must land on the kernel fixture: exactly one
    /// site (the blocks lambda), idempotent on re-sight.
    #[test]
    fn chunkparse_redirect_applies_to_kernel_fixture() {
        use crate::classfile::redirect_static_method_body_to_static;
        let fixture = include_bytes!("../tests/fixtures/SerializableChunkData.class");
        let (patched, outcome) = redirect_static_method_body_to_static(
            fixture,
            crate::classfile::CHUNKPARSE_BLOCKS_LAMBDA,
            CHUNKPARSE_SECTION_LAMBDA_DESC,
            CHUNKPARSE_OPS_CLASS,
            "parseSection",
            CHUNKPARSE_SECTION_LAMBDA_DESC,
        )
        .expect("redirect must compute");
        assert_eq!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
            "exactly the blocks lambda body must be replaced"
        );
        let (again, outcome2) = redirect_static_method_body_to_static(
            &patched,
            crate::classfile::CHUNKPARSE_BLOCKS_LAMBDA,
            CHUNKPARSE_SECTION_LAMBDA_DESC,
            CHUNKPARSE_OPS_CLASS,
            "parseSection",
            CHUNKPARSE_SECTION_LAMBDA_DESC,
        )
        .expect("re-redirect must compute");
        assert_eq!(
            outcome2,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 },
            "retransform re-sights must be idempotent"
        );
        assert_eq!(again, patched);
    }
}
