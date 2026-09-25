//! Runtime wiring for the CHUNK-PACKET ENCODE CACHE lever (TASK-444-B,
//! lever cmp444_chunk5, law 8 chunk-loading axis STAGE-2 — see
//! chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java).
//!
//! Ground truth (javap-census 2026-09-24): the vanilla private
//! `ClientboundLevelChunkWithLightPacket.write(RegistryFriendlyByteBuf)` body
//! is `buf.writeInt(x); buf.writeInt(z); chunkData.write(buf);
//! lightData.write(buf);` — a pure function of the packet instance (final
//! fields, no per-player input). With the cmp437_chunk4 snapshot the SAME
//! packet OBJECT is sent to every fake player (up to 4 sends per instance), so
//! the per-player encode (heightmap codec + block-entity codec + light
//! packing) is duplicated up to 4x for identical bytes.
//!
//! Stage-2 dedup: chunk4 dedups the CONSTRUCTION (one packet object per chunk
//! revision), chunk5 dedups the ENCODE — the instance->static body redirect
//! swaps write() for ChunkPacketEncodeOps.write: first call captures the
//! vanilla-equivalent encode into a standalone byte[] (scratch buffer, exact
//! vanilla body via public getters), subsequent calls on the SAME instance
//! replay the cached bytes (single writeBytes — bit-identical). Online
//! selftest: the first 2 captures are re-encoded a SECOND time and compared
//! bit-in-bit; a mismatch flips the DISABLED latch (fail-closed, vanilla body
//! forever). Profile honesty (RESEARCH-A canon): the players_packets lane is
//! 0.01% in the bench — this plane is a monotone, parity-transparent member of
//! the law-8 axis, not a wall-clock heavyweight; the legs' verdict rides the
//! cmp437_chunk4 carrier it widens.
//!
//! Parity contract (law 4): empty lever flag -> hooks never registered, the
//! write() body is byte-identical vanilla (dormant-invisible). The cached
//! payload IS the vanilla output for the same instance (verified bit-in-bit
//! online). Cache = reference-keyed CHM capped at 2048 (mirrors the chunk4
//! snapshot working set 1:1) with evict-half.
//!
//! Delivery: ChunkPacketEncodeOps is defined ALONE into the kernel loader
//! (flat classfile, ZERO nested classes/lambdas). The rust activator calls
//! selfTest()Z BEFORE READY (selfTest==true до ARM). Fail-closed: every guard
//! defect leaves the hook dormant.
//!
//! JNI discipline (law 6): ZERO added JNI crossings on the hot path (the
//! plane is Java-side); the activator uses exactly one JNI static call
//! (selfTest) at boot.
//!
//! ARM marker: "[crussty-plugin] cmp444_chunk5: ARMED chunk-packet encode
//! cache". EFFECT markers (Java side): "chunk5 payload-cache first hit" +
//! "chunk5 payload selftest PASS" + periodic "chunk5 stats writes=...". Grep
//! marker: "cmp444_chunk5".

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::classfile::{
    chunkpacket_pristine_guard, chunkpacket_resolution_closure, CHUNKPACKET_METHOD,
    CHUNKPACKET_OPS_CLASS, CHUNKPACKET_STATIC_DESC, CHUNKPACKET_TARGET_CLASS,
    CHUNKPACKET_VIRTUAL_DESC,
};

const OPS_BYTES: &[u8] =
    include_bytes!("../chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class");

const LEVER_ID: &str = "cmp444_chunk5";

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
    TARGET.get_or_init(|| Target::new(CHUNKPACKET_TARGET_CLASS))
}

/// STRICT-OR gate (law 7 round-id hygiene): the encode-side plane arms ONLY
/// on its own round id cmp444_chunk5 — older certification ids (cmp420_chunk2,
/// cmp420_colpush, cmp421_*, cmp424_mobfeed, cmp430_inside, cmp434_chunkpl,
/// cmp435_chunk3, cmp437_chunk4) MUST NOT gain this plane (their certified
/// semantics are frozen) EXCEPT the TASK-450-C union carrier cmp450_chunk
/// semantics are frozen). Empty/foreign flag = vanilla bit-in-bit.
fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == LEVER_ID || v.trim() == "cmp450_chunk" || v.trim() == "cmp456_poi" || v.trim() == "cmp456_poi_wide" || v.trim() == "cmp452_mega")
        .unwrap_or(false)
}

/// TASK-450-C evidence marker: print the UNION carrier id when the env flag IS
/// the union (cmp450_chunk legs grep "cmp450_chunk: ARMED ..."), else the
/// plane's birth id (frozen historical markers).
fn marker_id() -> std::borrow::Cow<'static, str> {
    match std::env::var("CRUSSTY_LEVER_FLAG").as_deref() {
        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),
        Ok("cmp452_mega") => std::borrow::Cow::Owned("cmp452_mega".to_string()), // TASK-452-C mega-composite
        _ => std::borrow::Cow::Borrowed(LEVER_ID),
    }
}


/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline: the callback performs NO JNI work — pristine
/// capture at the class's own load, patch served from the precomputed cache.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla chunk packet encode)"
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
    chunkpacket_resolution_closure(OPS_BYTES).is_ok()
}

/// Define ChunkPacketEncodeOps into the kernel loader (anchored at the target
/// class's own loader) and run the structural selfTest oracle via ONE JNI
/// static call on the LOCAL define_class ref.
///
/// Return codes:
///   * Some(true)  — defined AND selfTest()==true;
///   * Some(false) — hard failure (define/selfTest defect; retrying pointless);
///   * None        — transient (anchor class not reachable yet; retry).
fn define_bridge_and_selftest() -> Option<bool> {
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(CHUNKPACKET_TARGET_CLASS) else {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: kernel class {CHUNKPACKET_TARGET_CLASS} not reachable yet"
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
        let Some(c) = env.define_class(CHUNKPACKET_OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: define_class({CHUNKPACKET_OPS_CLASS}) failed"
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
                "[crussty-plugin] {LEVER_ID}: ChunkPacketEncodeOps.selfTest()==false — hook stays dormant"
            );
        }
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        Some(ok)
    })
    .flatten()
}

/// Background activation: wait for boot, define ChunkPacketEncodeOps into the
/// KERNEL loader (anchor = the target class's own loader), run the structural
/// selfTest oracle (selfTest==true до ARM), capture pristine bytes, run the
/// pristine guard, compute the instance->static body-redirect, flip READY,
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
            "[crussty-plugin] {LEVER_ID}: server booted, defining {CHUNKPACKET_OPS_CLASS} into kernel loader"
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

        // RESOLUTION CLOSURE GUARD: write carries the EXACT receiver-prepended
        // static descriptor (stack-shape contract), selfTest()Z exists and the
        // bridge is FLAT (no nested classes).
        if let Err(e) = chunkpacket_resolution_closure(OPS_BYTES) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader + run the selfTest oracle
        // (ONE JNI static call on the LOCAL ref). Retried: the packet class is
        // loaded at boot (protocol registration) — a hotpatch attach racing a
        // quiet boot must not lose the lever.
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
            "[crussty-plugin] {LEVER_ID}: defined {CHUNKPACKET_OPS_CLASS} in kernel loader, selfTest==true (pre-ARM oracle)"
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
        // (private write + the public getter encode surface) BEFORE any patch
        // is served.
        if let Err(e) = chunkpacket_pristine_guard(&original) {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: PRISTINE GUARD FAILED: {e} — hook stays dormant"
            );
            return;
        }

        // ONE redirect — the whole write() body -> ChunkPacketEncodeOps.write
        // (must land with sites:1; anti-placebo gate: sites>0 else disarm).
        let (patched, outcome) = match crate::classfile::redirect_method_body_to_static(
            &original,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_VIRTUAL_DESC,
            CHUNKPACKET_TARGET_CLASS,
            CHUNKPACKET_OPS_CLASS,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_STATIC_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: write() redirect rejected ({e}), hook stays dormant"
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
            "[crussty-plugin] {m}: ARMED chunk-packet encode cache (instance-keyed payload replay, encode-once per packet, selftest fail-closed, cap 2048 evict-half, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod chunkpacket_delivery_tests {
    use super::OPS_BYTES;
    use crate::classfile::{
        chunkpacket_pristine_guard, chunkpacket_resolution_closure, CHUNKPACKET_METHOD,
        CHUNKPACKET_OPS_CLASS, CHUNKPACKET_STATIC_DESC, CHUNKPACKET_TARGET_CLASS,
        CHUNKPACKET_VIRTUAL_DESC,
    };

    /// s7171 delivery-graph guard mirrored: the bridge source MUST declare
    /// ZERO nested classes (the classfile is defined alone into the kernel
    /// loader; a nested class would detonate as NoClassDefFoundError on the
    /// first write — the offline harness cannot catch a missing nested
    /// classfile).
    #[test]
    fn chunkpacket_source_declares_no_nested_classes() {
        let src =
            include_str!("../chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java");
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
    fn chunkpacket_embedded_classfile_present_and_pinned() {
        assert_eq!(&OPS_BYTES[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([OPS_BYTES[6], OPS_BYTES[7]]);
        assert_eq!(major, 65, "bridge major must be pinned to 65");
    }

    /// s7171-style: the resolution closure MUST accept the embedded bridge
    /// classfile as compiled (receiver-prepended write descriptor + selfTest +
    /// flat) so any future member drift fails offline instead of silently
    /// disarming.
    #[test]
    fn chunkpacket_resolution_closure_accepts_embedded_bridge() {
        chunkpacket_resolution_closure(OPS_BYTES)
            .expect("resolution closure must accept the embedded ChunkPacketEncodeOps bytes");
    }

    /// The bridge write descriptor must be the EXACT canonical virtual desc
    /// with the receiver class prepended (redirect stack-shape contract).
    #[test]
    fn chunkpacket_bridge_descriptor_is_canonical() {
        let expect = format!("(L{CHUNKPACKET_TARGET_CLASS};{}", &CHUNKPACKET_VIRTUAL_DESC[1..]);
        assert_eq!(CHUNKPACKET_STATIC_DESC, expect);
        assert_eq!(CHUNKPACKET_METHOD, "write");
        assert!(CHUNKPACKET_OPS_CLASS.ends_with("ChunkPacketEncodeOps"));
    }

    /// REAL kernel fixture: the pristine guard must accept the actual
    /// ClientboundLevelChunkWithLightPacket bytes (tests/fixtures/
    /// ClientboundLevelChunkWithLightPacket.class, extracted from the
    /// round-396-a patched-kernel.jar — same fixture source as every other
    /// delivery test).
    #[test]
    fn chunkpacket_pristine_guard_accepts_kernel_fixture() {
        let fixture = include_bytes!("../tests/fixtures/ClientboundLevelChunkWithLightPacket.class");
        chunkpacket_pristine_guard(fixture)
            .expect("pristine guard must accept the kernel fixture bytes");
    }

    /// The redirect itself must land on the kernel fixture: exactly one
    /// site (the whole write body), idempotent on re-sight.
    #[test]
    fn chunkpacket_redirect_applies_to_kernel_fixture() {
        use crate::classfile::redirect_method_body_to_static;
        let fixture = include_bytes!("../tests/fixtures/ClientboundLevelChunkWithLightPacket.class");
        let (patched, outcome) = redirect_method_body_to_static(
            fixture,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_VIRTUAL_DESC,
            CHUNKPACKET_TARGET_CLASS,
            CHUNKPACKET_OPS_CLASS,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_STATIC_DESC,
        )
        .expect("redirect must compute");
        assert_eq!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
            "exactly the write body must be replaced"
        );
        // Idempotency: re-sighting on the patched bytes must be
        // AlreadyPatched with byte-identical output.
        let (again, o1) = redirect_method_body_to_static(
            &patched,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_VIRTUAL_DESC,
            CHUNKPACKET_TARGET_CLASS,
            CHUNKPACKET_OPS_CLASS,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_STATIC_DESC,
        )
        .expect("re-redirect must compute");
        assert_eq!(
            o1,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 },
            "write re-sight must be idempotent"
        );
        assert_eq!(again, patched);
    }

    /// TASK-444-B gate consistency: the cmp444_chunk5 carrier id must ride
    /// BOTH blob constant pools (raw-byte gate, x93 lesson) — the encode
    /// plane's own blob AND the chunk4 send-plane blob it widens.
    #[test]
    fn chunkpacket_gate_carries_444_carrier() {
        let blob = include_bytes!(
            "../chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class"
        );
        for needle in [
            &b"cmp444_chunk5"[..],
            b"cmp437_chunk4",
            b"cmp435_chunk3",
            b"chunk5 payload-cache first hit",
            b"chunk5 payload selftest PASS",
            b"chunk5 stats",
        ] {
            assert!(
                blob.windows(needle.len()).any(|w| w == needle),
                "blob constant pool must carry {:?}",
                String::from_utf8_lossy(needle)
            );
        }
        let send_blob =
            include_bytes!("../chunksend/build/net/minecraft/server/network/ChunkSendOps.class");
        // chunk4 plane rides the chunk5 carrier too (carrier semantics):
        let needle = b"cmp444_chunk5";
        assert!(
            send_blob.windows(needle.len()).any(|w| w == needle),
            "ChunkSendOps blob constant pool must carry the cmp444_chunk5 carrier union"
        );
    }
}
