//! Runtime wiring for the CHUNK-SEND BURST COALESCING lever (TASK-459-62,
//! ID-P26, law-11 WILD — development of cmp437_chunk4 send-snapshot ⊕
//! cmp444_chunk5 encode-cache; see chunksend/net/minecraft/server/network/
//! ChunkSendOps.java). Wild idea verified by dispatch, not by enumeration.
//!
//! Ground truth (javap tests/fixtures/PlayerChunkSender.class, round-396-a
//! jar): vanilla PlayerChunkSender batches the DECISION side per tick
//! (MIN_CHUNKS_PER_TICK 0.01f, MAX_CHUNKS_PER_TICK 64.0f, START 9.0f,
//! MAX_UNACKNOWLEDGED_BATCHES 10) but NOT the flush side: the tick() loop
//! `invokestatic sendChunk(...)` ends every send with
//! ServerGamePacketListenerImpl.send(packet) — one write+flush per packet,
//! up to 64 flushes per tick per player inside a join burst, although the
//! vanilla batch protocol (ClientboundChunkBatchFinishedPacket tail) already
//! provides a per-tick window delimiter.
//!
//! P26 window coalescing: N chunk sends of one vanilla batch (one window)
//! produce ONE flush at the window boundary; packet BYTES are unchanged
//! (chunk4 snapshot + chunk5 encode-cache guarantee bit-identical payloads)
//! and the send ORDER is the vanilla queue order — only the flush
//! inter-timing changes (krypton flush consolidation; netty
//! FlushConsolidationHandler: "flush operations may trigger a syscall";
//! see RESEARCH-459-P26.md). Window budget: COALESCE_WINDOW_N = 8 sends,
//! COALESCE_WINDOW_NANOS = 1 ms, closed at latest by the vanilla batch
//! boundary => flush latency to the client stays <= 1 tick.
//!
//! Parity contract (law 4): empty lever flag -> hooks never registered, the
//! sendChunk body is byte-identical vanilla (dormant-invisible). STRICT-OR
//! (law 7): this plane arms ONLY on its own round id cmp456_poi_send — the older
//! certification ids (cmp437_chunk4, cmp444_chunk5, cmp450_chunk, cmp452_mega,
//! cmp453_diet) MUST NOT gain the window (their certified semantics are
//! frozen; their hooks keep owning PlayerChunkSender exactly as shipped).
//! anti-xray shouldModify bypass preserved by the shared bridge; per-send
//! events (PlayerChunkLoadEvent + debugSynchronizers.startTrackingChunk)
//! fire on EVERY send exactly like vanilla.
//!
//! Delivery: the SAME ChunkSendOps blob as chunk4 (rebuilt, raw-cp markers
//! cmp456_poi_send + all frozen unions) is defined ALONE into the kernel loader
//! (flat classfile, ZERO nested classes/lambdas — NCDFE canon). The rust
//! activator calls selfTest()Z BEFORE READY (selfTest==true до ARM).
//! Fail-closed: every guard defect leaves the hook dormant.
//!
//! JNI discipline (law 6): ZERO added JNI crossings on the hot path (the
//! window accounting is Java-side plain statics); the activator uses exactly
//! one JNI static call (selfTest) at boot.
//!
//! ARM marker: "[crussty-plugin] cmp456_poi_send: ARMED chunk-send window
//! coalescing". EFFECT markers (Java side): "p26 window stats". Grep marker:
//! "cmp456_poi_send".
//!
//! TASK-460-38 PORT (round-460-poisend-1 @5ecd841a poi carrier): lever id
//! rewired cmp459_p26 -> cmp456_poi_send; historical id cmp459_p26 stays in
//! the Java gate union + blob cp (x93 raw-byte discipline). Run 36157837929
//! (VALID -12.6@7067374) was a STW-host negative, not a code verdict (see
//! RESEARCH-460-P26-SEND.md).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::classfile::{
    chunksend_pristine_guard, chunksend_resolution_closure, CHUNKSEND_DESC, CHUNKSEND_METHOD,
    CHUNKSEND_OPS_CLASS, CHUNKSEND_TARGET_CLASS,
};

const OPS_BYTES: &[u8] =
    include_bytes!("../chunksend/build/net/minecraft/server/network/ChunkSendOps.class");

const LEVER_ID: &str = "cmp456_poi_send";

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

/// STRICT-OR gate (law 7 round-id hygiene): the window plane arms ONLY on its
/// own round id cmp456_poi_send. Older certification ids (cmp420_chunk2,
/// cmp435_chunk3, cmp437_chunk4, cmp444_chunk5, cmp450_chunk, cmp452_mega,
/// cmp453_diet) MUST NOT gain this plane (their certified semantics are
/// frozen) — on those flags this module stays dormant and the frozen
/// chunk_send.rs hook keeps owning PlayerChunkSender. Empty/foreign flag =
/// vanilla bit-in-bit.
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == LEVER_ID)
        .unwrap_or(false)
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// STRICT-OR ownership: under cmp456_poi_send chunk_send.rs does NOT register
/// (its enabled() is disjoint) — exactly ONE bytes hook per target class.
/// Loader-lock discipline: the callback performs NO JNI work — pristine
/// capture at the class's own load, patch served from the precomputed cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla flush timing)"
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
        // is FLAT (no nested classes) — NCDFE canon.
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
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: ARMED chunk-send window coalescing (window N=8/1ms, one flush per window, vanilla queue order, bytes unchanged, snapshot+encode-cache union, selftest fail-closed, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod chunksend6_delivery_tests {
    use super::OPS_BYTES;
    use crate::classfile::{
        chunksend_pristine_guard, chunksend_resolution_closure, CHUNKSEND_DESC, CHUNKSEND_METHOD,
        CHUNKSEND_OPS_CLASS,
    };

    /// NCDFE canon (s7171 delivery-graph guard mirrored): the bridge source
    /// MUST declare ZERO nested classes (the classfile is defined alone into
    /// the kernel loader; a nested class would detonate as
    /// NoClassDefFoundError on the first send).
    #[test]
    fn chunksend6_source_declares_no_nested_classes() {
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
    fn chunksend6_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// s7171-style: the resolution closure MUST accept the embedded bridge
    /// classfile as compiled (sendChunk descriptor + selfTest + flat) so any
    /// future member drift fails offline instead of silently disarming.
    #[test]
    fn chunksend6_resolution_closure_accepts_embedded_bridge() {
        chunksend_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded ChunkSendOps bytes");
    }

    /// The bridge sendChunk descriptor must be the EXACT canonical vanilla
    /// static descriptor (redirect stack-shape contract).
    #[test]
    fn chunksend6_bridge_descriptor_is_canonical() {
        assert_eq!(
            CHUNKSEND_DESC,
            "(Lnet/minecraft/server/network/ServerGamePacketListenerImpl;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;)V"
        );
        assert_eq!(CHUNKSEND_METHOD, "sendChunk");
        assert!(CHUNKSEND_OPS_CLASS.ends_with("ChunkSendOps"));
    }

    /// REAL kernel fixture: the pristine guard must accept the actual
    /// PlayerChunkSender bytes (tests/fixtures/PlayerChunkSender.class).
    #[test]
    fn chunksend6_pristine_guard_accepts_kernel_fixture() {
        let fixture = include_bytes!("../tests/fixtures/PlayerChunkSender.class");
        chunksend_pristine_guard(fixture)
            .expect("pristine guard must accept the kernel fixture bytes");
    }

    /// The redirect itself must land on the kernel fixture: exactly one
    /// site (the whole sendChunk body), idempotent on re-sight.
    #[test]
    fn chunksend6_redirect_applies_to_kernel_fixture() {
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

    /// TASK-459-62 gate consistency: the cmp456_poi_send carrier id must ride the
    /// blob constant pool (raw-byte gate, x93 lesson) AND all frozen union
    /// ids must be preserved (the same blob serves the older carriers).
    #[test]
    fn chunksend6_gate_carries_p26_and_frozen_unions() {
        for needle in [
            &b"cmp456_poi_send"[..],
            b"p26 window stats",
            b"COALESCE_WINDOW_N",
            b"cmp456_poi", // TASK-456-B frozen poi union (base carrier, x93)
            b"cmp437_chunk4",
            b"cmp444_chunk5",
            b"cmp450_chunk",
            b"cmp452_mega",
            b"cmp453_diet",
            b"chunk4 send-snapshot first hit",
            b"chunk4 snapshot selftest PASS",
            b"chunk4 stats",
        ] {
            assert!(
                OPS_BYTES.windows(needle.len()).any(|w| w == needle),
                "blob constant pool must carry {:?}",
                String::from_utf8_lossy(needle)
            );
        }
    }

    /// Window constants pinned in the bridge source (G4: flush latency <= 1
    /// tick — window N=8 sends, budget 1ms, vanilla batch boundary closes it).
    #[test]
    fn chunksend6_window_constants_pinned() {
        let src = include_str!("../chunksend/net/minecraft/server/network/ChunkSendOps.java");
        assert!(
            src.contains("COALESCE_WINDOW_N = 8;"),
            "window N must stay 8"
        );
        assert!(
            src.contains("COALESCE_WINDOW_NANOS = 1_000_000L;"),
            "window budget must stay 1ms (<= 1 tick)"
        );
        assert!(
            src.contains("\"cmp456_poi_send\".equals(System.getenv(\"CRUSSTY_LEVER_FLAG\"))"),
            "Java-side P26 gate must key on the lever env (0 JNI)"
        );
    }
}
