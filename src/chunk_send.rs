//! Runtime wiring for the CHUNK-SEND SERIALIZATION SNAPSHOT lever (TASK-438-C,
//! lever cmp437_chunk4, law 8 chunk-loading axis WIDENING — see
//! chunksend/net/minecraft/server/network/ChunkSendOps.java).
//!
//! Ground truth: the vanilla static
//! `PlayerChunkSender.sendChunk(ServerGamePacketListenerImpl, ServerLevel,
//! LevelChunk)` builds a fresh ClientboundLevelChunkWithLightPacket PER SEND —
//! i.e. per player. The serialize side of the chunk pipeline mirrors the
//! parse side (RESEARCH-F: parse codec 33.38% of ALL burst-window alloc bytes
//! — codec 19.06% + paletted-decode 13.98% — and the write path runs the same
//! PalettedContainer/codec machinery in the same pre-poll window). With 4 fake
//! players and overlapping view distances the SAME chunk state is serialized
//! up to 4 times within seconds.
//!
//! The lever redirects that static body (ONE static->static body redirect,
//! exact descriptor) to ChunkSendOps.sendChunk — a snapshot-first sender:
//!   * HIT (snapshot present AND !chunk.isUnsaved()): re-send the stored
//!     packet — ClientboundLevelChunkPacketData extraction (section buffers,
//!     heightmap NBT, block-entity tags) + light packing skipped (zero-copy);
//!   * MISS/invalid: exact vanilla construction, then snapshot;
//!   * anti-xray (shouldModify == true) bypasses the cache byte-vanilla;
//!   * PlayerChunkLoadEvent + debugSynchronizers.startTrackingChunk fire per
//!     send exactly like vanilla.
//!
//! Parity contract (law 4): empty lever flag -> hooks never registered, the
//! method body is byte-identical vanilla (dormant-invisible). isUnsaved() is
//! the vanilla dirty flag (any block/block-entity change marks it BEFORE the
//! next send probes the cache) — a changed chunk can never be served stale.
//! Documented residual: light freshness inside the seconds-wide join burst
//! (vanilla itself never re-serializes a chunk packet on light change; both
//! clients converge via the same subsequent light delta packets).
//!
//! Delivery: ChunkSendOps is defined ALONE into the kernel loader (flat
//! classfile). The rust activator calls ChunkSendOps.selfTest()Z BEFORE READY
//! (selfTest==true до ARM; structural oracle on the pristine kernel classes).
//! Fail-closed: every guard defect leaves the hook dormant.
//!
//! JNI discipline (law 6): ZERO added JNI crossings on the hot path — the
//! plane is Java-side (CHM snapshot + packet reuse); the activator uses
//! exactly one JNI static call (selfTest) at boot. Per-tick <= 1 satisfied
//! trivially (0).
//!
//! ARM marker: "[crussty-plugin] cmp437_chunk4: ARMED chunk-send serialization
//! snapshot". EFFECT markers (Java side): "chunk4 send-snapshot first hit" +
//! "chunk4 snapshot selftest PASS" + periodic "chunk4 stats sent=...". Grep
//! marker: "cmp437_chunk4".

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::classfile::{
    chunksend_pristine_guard, chunksend_resolution_closure, CHUNKSEND_DESC, CHUNKSEND_METHOD,
    CHUNKSEND_OPS_CLASS, CHUNKSEND_TARGET_CLASS,
};

const OPS_BYTES: &[u8] =
    include_bytes!("../chunksend/build/net/minecraft/server/network/ChunkSendOps.class");

const LEVER_ID: &str = "cmp437_chunk4";

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
    TARGET.get_or_init(|| Target::new(CHUNKSEND_TARGET_CLASS))
}

/// STRICT-OR gate (law 7 round-id hygiene): the serialize-side plane arms on
/// its own round id cmp437_chunk4 AND on the TASK-444-B stage-2 carrier
/// cmp444_chunk5 (carrier semantics: chunk5 = chunk4 + encode-cache plane).
/// Older certification ids (cmp420_chunk2, cmp420_colpush, cmp421_*,
/// cmp424_mobfeed, cmp430_inside, cmp434_chunkpl, cmp435_chunk3) MUST NOT gain
/// this plane (their certified semantics are frozen). Empty/foreign flag =
/// vanilla bit-in-bit.
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            v == LEVER_ID || v == "cmp444_chunk5" || v == "cmp452_mega" || v == "cmp450_chunk" || v == "cmp456_chunkmono" || v == "cmp456_chunkmono_p31snap" || v == "cmp466_c98ai" || v == "cmp468_s18fluid" || v == "cmp453_diet" || v == "cmp456_poi"
})
        .unwrap_or(false)
}

/// TASK-450-C evidence marker: print the UNION carrier id when the env flag IS
/// the union (cmp450_chunk legs grep "cmp450_chunk: ARMED ..."), else the
/// plane's birth id (frozen historical markers).
fn marker_id() -> std::borrow::Cow<'static, str> {
    match std::env::var("CRUSSTY_LEVER_FLAG").as_deref() {
        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),
        Ok("cmp452_mega") => std::borrow::Cow::Owned("cmp452_mega".to_string()), // TASK-452-C mega-composite
        Ok("cmp453_diet") => std::borrow::Cow::Owned("cmp453_diet".to_string()), // TASK-453-C diet composite
        _ => std::borrow::Cow::Borrowed(LEVER_ID),
    }
}


/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline: the callback performs NO JNI work — pristine
/// capture at the class's own load, patch served from the precomputed cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla chunk send)"
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
#[allow(dead_code)] // CI/self-diagnosis helper
pub fn delivery_closure_ok() -> bool {
    chunksend_resolution_closure(OPS_BYTES).is_ok()
}

/// Define ChunkSendOps into the kernel loader (anchored at the target
/// class's own loader) and run the structural selfTest oracle via ONE JNI
/// static call on the LOCAL define_class ref.
///
/// Return codes:
///   * Some(true)  — defined AND selfTest()==true;
///   * Some(false) — hard failure (define/selfTest defect; retrying pointless);
///   * None        — transient (anchor class not reachable yet; retry).
fn define_bridge_and_selftest() -> Option<bool> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(CHUNKSEND_TARGET_CLASS) else {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: kernel class {CHUNKSEND_TARGET_CLASS} not reachable yet"
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
        let Some(c) = env.define_class(CHUNKSEND_OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: define_class({CHUNKSEND_OPS_CLASS}) failed"
            );
            return Some(false);
        };
        // selfTest()Z — the structural oracle MUST pass BEFORE the redirect
        // can ever be served (selfTest==true до ARM). One JNI static call at
        // boot total; first static call also initializes the class (JVMS 5.5).
        let ok = env
            .get_static_method_id(c, "selfTest", "()Z")
            .map(|mid| {
                let raw = env.raw();
                let result = unsafe {
                    let fn_table = &(**raw);
                    let call_bool = fn_table.CallStaticBooleanMethodA;
                    (call_bool)(raw, c, mid, [].as_ptr())
                };
                crate::clear_exception(env);
                result != 0
            })
            .unwrap_or_else(|| {
                crate::clear_exception(env);
                false
            });
        if !ok {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: ChunkSendOps.selfTest()==false — hook stays dormant"
            );
        }
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(ok)
    })
    .flatten()
}

/// Background activation: wait for boot, define ChunkSendOps into the KERNEL
/// loader (anchor = the target class's own loader), run the structural
/// selfTest oracle (selfTest==true до ARM), capture pristine bytes, run the
/// pristine guard, compute the static body-redirect, flip READY, retransform
/// exactly once.
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
            "[crussty-plugin] {LEVER_ID}: server booted, defining {CHUNKSEND_OPS_CLASS} into kernel loader"
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
                "[crussty-plugin] {LEVER_ID}: bridge is class major {bridge_major} but JVM supports up to {jvm_major} — rebuild chunksend/ via scripts/build_438c_chunksend.sh; hook stays dormant"
            );
            return;
        }

        // RESOLUTION CLOSURE GUARD: sendChunk carries the EXACT vanilla static
        // descriptor (stack-shape contract), selfTest()Z exists and the bridge
        // is FLAT (no nested classes).
        if let Err(e) = chunksend_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader + run the selfTest oracle
        // (ONE JNI static call on the LOCAL ref). Retried: the anchor class is
        // loaded during spawn-chunk load (before the boot marker), but a
        // hotpatch attach racing a quiet boot must not lose the lever.
        let mut defined = false;
        for attempt in 1..=10 {
            match define_bridge_and_selftest() {
                Some(true) => {
                    defined = true;
                    break;
                }
                Some(false) => break, // hard failure (selfTest false), no retry
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
                "[crussty-plugin] {LEVER_ID}: bridge definition/selfTest failed, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: defined {CHUNKSEND_OPS_CLASS} in kernel loader, selfTest==true (pre-ARM oracle)"
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
        // (static sendChunk with the canonical descriptor + the packet
        // construction site) BEFORE any patch is served.
        if let Err(e) = chunksend_pristine_guard(&original) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: PRISTINE GUARD FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // ONE redirect — the whole sendChunk body -> ChunkSendOps.sendChunk
        // (must land with sites:1; anti-placebo gate: sites>0 else disarm).
        let (patched, outcome) = match crate::classfile::redirect_static_method_body_to_static(
            &original,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
            CHUNKSEND_OPS_CLASS,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: sendChunk redirect rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let ok = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
                | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        );
        if !ok || matches!(outcome, crate::classfile::RetargetOutcome::Retargeted { sites: 0 }) {
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
        let m = marker_id();
        eprintln!(
            "[crussty-plugin] {m}: ARMED chunk-send serialization snapshot (unsaved-keyed packet reuse, zero-copy HIT handoff, anti-xray bypass, per-send events preserved, cap 2048 evict-half, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod chunksend_delivery_tests {
    use super::OPS_BYTES;
    use crate::classfile::{
        chunksend_pristine_guard, chunksend_resolution_closure, CHUNKSEND_DESC, CHUNKSEND_METHOD,
        CHUNKSEND_OPS_CLASS,
    };

    /// s7171 delivery-graph guard mirrored: the bridge source MUST declare
    /// ZERO nested classes (the classfile is defined alone into the kernel
    /// loader; a nested class would detonate as NoClassDefFoundError on the
    /// first send — the offline harness cannot catch a missing nested
    /// classfile).
    #[test]
    fn chunksend_source_declares_no_nested_classes() {
        let src = include_str!("../chunksend/net/minecraft/server/network/ChunkSendOps.java");
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
    fn chunksend_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// s7171-style: the resolution closure MUST accept the embedded bridge
    /// classfile as compiled (sendChunk descriptor + selfTest + flat) so any
    /// future member drift fails offline instead of silently disarming.
    #[test]
    fn chunksend_resolution_closure_accepts_embedded_bridge() {
        chunksend_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded ChunkSendOps bytes");
    }

    /// The bridge sendChunk descriptor must be the EXACT canonical vanilla
    /// static descriptor (redirect stack-shape contract).
    #[test]
    fn chunksend_bridge_descriptor_is_canonical() {
        assert_eq!(
            CHUNKSEND_DESC,
            "(Lnet/minecraft/server/network/ServerGamePacketListenerImpl;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;)V"
        );
        assert_eq!(CHUNKSEND_METHOD, "sendChunk");
        assert!(CHUNKSEND_OPS_CLASS.ends_with("ChunkSendOps"));
    }

    /// REAL kernel fixture: the pristine guard must accept the actual
    /// PlayerChunkSender bytes (tests/fixtures/PlayerChunkSender.class is
    /// sha256-identical to the round-396-a patched-kernel.jar entry —
    /// same fixture source as every other delivery test).
    #[test]
    fn chunksend_pristine_guard_accepts_kernel_fixture() {
        let fixture = include_bytes!("../tests/fixtures/PlayerChunkSender.class");
        chunksend_pristine_guard(fixture)
            .expect("pristine guard must accept the kernel fixture bytes");
    }

    /// The redirect itself must land on the kernel fixture: exactly one
    /// site (the whole sendChunk body), idempotent on re-sight.
    #[test]
    fn chunksend_redirect_applies_to_kernel_fixture() {
        use crate::classfile::redirect_static_method_body_to_static;
        let fixture = include_bytes!("../tests/fixtures/PlayerChunkSender.class");
        let (patched, outcome) = redirect_static_method_body_to_static(
            fixture,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
            CHUNKSEND_OPS_CLASS,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
        )
        .expect("redirect must compute");
        assert_eq!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
            "exactly the sendChunk body must be replaced"
        );
        // Idempotency: re-sighting on the patched bytes must be
        // AlreadyPatched with byte-identical output.
        let (again, o1) = redirect_static_method_body_to_static(
            &patched,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
            CHUNKSEND_OPS_CLASS,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
        )
        .expect("re-redirect must compute");
        assert_eq!(
            o1,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 },
            "sendChunk re-sight must be idempotent"
        );
        assert_eq!(again, patched);
    }

    /// TASK-438-C gate consistency: the cmp437_chunk4 carrier id must ride
    /// BOTH blob constant pools (raw-byte gate, x93 lesson) — the snapshot
    /// plane's own blob AND the parse-plane carrier it widens.
    #[test]
    fn chunksend_gate_carries_437_carrier() {
        let blob = include_bytes!("../chunksend/build/net/minecraft/server/network/ChunkSendOps.class");
        for needle in [
            &b"cmp437_chunk4"[..],
            b"cmp435_chunk3",
            b"chunk4 send-snapshot first hit",
            b"chunk4 snapshot selftest PASS",
            b"chunk4 stats",
        ] {
            assert!(
                blob.windows(needle.len()).any(|w| w == needle),
                "blob constant pool must carry {:?}",
                String::from_utf8_lossy(needle)
            );
        }
        let parse_blob = include_bytes!(
            "../chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class"
        );
        let needle = b"cmp437_chunk4";
        assert!(
            parse_blob.windows(needle.len()).any(|w| w == needle),
            "chunkparse blob constant pool must carry the cmp437_chunk4 carrier union"
        );
    }
}
