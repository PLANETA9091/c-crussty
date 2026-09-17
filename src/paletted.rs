//! Runtime wiring for the PALETTED-DEMUX hook (S7-131, ARCH-ATTACK lever #1
//! — the owner's top-1 bottleneck function).
//!
//! Target: `net/minecraft/world/level/chunk/PalettedContainer.get(int)` — the
//! top-1 kernel JVM-Java leaf across every X150K profile (4.20% @10k smoke,
//! 3.62% @fp4 pack, 3.3% @150k prime; + readPalette 0.8% +
//! SimpleBitStorage.get 0.8% = the ~4.9% "palette lane"). Owner's bar
//! (x150000 = the top-1 function's execution speed): the function must
//! DISAPPEAR from the profile. Mechanism: a per-container flat read-snapshot
//! (demux) so hot reads become `vals[demux[index]]` — the bit decode +
//! palette indirection vanish; writes bump a generation counter that
//! invalidates the snapshot at near-zero cost. Full protocol and race
//! analysis: the PALETTED-DEMUX header in `src/classfile.rs` and the ASM
//! patcher `paletted/tools/PalettedPatchTool.java`.
//!
//! Pipeline shape (differs from F1-F3/guard: FIELD INJECTION = shape change,
//! so retransform is unavailable — the patch MUST apply at FIRST LOAD):
//!
//! 1. BUILD TIME (offline, sanctioned precedent = precompute_patches.py):
//!    `paletted/build_paletted_ops.sh` runs the ASM COMPUTE_FRAMES patcher
//!    over the pinned kernel's PalettedContainer.class and compiles
//!    PalettedContainerOps. The JVM parity harness (vanilla vs patched
//!    lockstep, 20k random ops + resize ladder + demux lifecycle +
//!    concurrency smoke) gates the artifacts — ALL PASS is the banked
//!    contract (research/paletted-demux-2026-09-18/).
//! 2. RUNTIME: the byte hook serves the EMBEDDED patched class bytes when
//!    the incoming bytes fingerprint-match the pinned kernel image
//!    (length + cp probes, fail-closed on any mismatch — a kernel update
//!    degrades to vanilla, never to a bad class). READY gates the serve:
//!    PalettedContainerOps must be defined into the kernel loader BEFORE
//!    the patched class loads (its invokestatic resolves lazily at first
//!    get execution, but serving unconditionally would risk a
//!    NoClassDefFoundError on the hottest path in the server if Ops were
//!    missing at first execution — unacceptably sharp an edge).
//! 3. ACTIVATION: a definer thread started at cplugin_init polls for the
//!    earliest launch-loader classes (PluginInitializerManager/Bukkit —
//!    both load before the registry/chunk bootstrap that pulls in
//!    PalettedContainer), defines Ops into that loader and flips READY.
//!
//! Markers: "paletted: PATCHED" (hook served), "paletted: pristine sighting"
//! (class loaded pre-READY — dormant, vanilla), "paletted: PALETTED-DEMUX
//! ARMED" (final one-line state), "paletted: dormant".

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const PALETTED_CLASS: &str = crate::classfile::PALETTED_CLASS;
const OPS_NAME: &str = crate::classfile::PALETTED_OPS_CLASS;

/// Embedded patched image (ASM COMPUTE_FRAMES output over the pinned kernel).
const PATCHED_BYTES: &[u8] = include_bytes!("../paletted/build/PalettedContainer.patched.class");
/// Ops helper compiled against the patched shapes (stub jar).
const OPS_BYTES: &[u8] = include_bytes!("../paletted/build/net/minecraft/world/level/chunk/PalettedContainerOps.class");

/// Original (pinned) kernel image length — cheap fingerprint leg.
const ORIG_LEN: usize = 30967;

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// env gate (off by default — dormant-invisible discipline).
fn enabled() -> bool {
    std::env::var("CRUSSTY_PALETTED_DEMUX")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Fingerprint of the pinned kernel image: length + header + cp probes.
/// The goal is NOT cryptographic binding — it is "any kernel rebuild that
/// shifts the class layout fails closed to vanilla instead of serving a
/// stale patch against a moved target".
fn fingerprint_matches(bytes: &[u8]) -> bool {
    if bytes.len() != ORIG_LEN || bytes.len() < 10 {
        return false;
    }
    if u16::from_be_bytes([bytes[6], bytes[7]]) != 65 {
        return false; // Java 21 class file (pinned kernel)
    }
    // quick utf8 probes: parse the constant pool once via the classfile parser
    let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
    let Some((pool, _)) = crate::classfile::Pool::parse(bytes, 10, cp_count) else {
        return false;
    };
    pool.find_utf8("readPaletteSlow").is_some()
        && pool.find_utf8("createOrReuseData").is_some()
        && pool.find_utf8("crusstySnap").is_none() // not already patched
        && pool.find_utf8("crusstySnapGen").is_none()
        && bytes.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE])
}

/// Register the byte hook (call once from cplugin_init).
pub fn register() {
    if !enabled() {
        eprintln!("[crussty-plugin] paletted: dormant (set CRUSSTY_PALETTED_DEMUX=1 to enable)");
        return;
    }
    cplug_sdk::hooks::register_bytes(PALETTED_CLASS, |name, bytes| {
        if !READY.load(Ordering::Acquire) {
            eprintln!(
                "[crussty-plugin] paletted: pristine sighting {name} ({} bytes) — Ops not defined yet, staying vanilla",
                bytes.len()
            );
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None; // already served (retransform re-sight)
        }
        if !fingerprint_matches(bytes) {
            eprintln!(
                "[crussty-plugin] paletted: fingerprint MISMATCH ({} bytes) — kernel drift, staying vanilla",
                bytes.len()
            );
            PATCHED.store(false, Ordering::SeqCst);
            return None;
        }
        eprintln!(
            "[crussty-plugin] paletted: PATCHED {name} ({} -> {} bytes, demux fields + fast-path get + guarded mutators)",
            bytes.len(),
            PATCHED_BYTES.len()
        );
        Some(PATCHED_BYTES.to_vec())
    });
}

/// Background activation: define PalettedContainerOps into the kernel loader
/// as early as possible, then flip READY so the hook serves the patched
/// image when PalettedContainer loads.
pub fn activate() {
    if !enabled() {
        return; // register() already logged the dormant notice
    }
    std::thread::spawn(|| {
        // The kernel launch loader is reachable the moment ANY of its early
        // classes load. PluginInitializerManager logs before the registry
        // bootstrap; Bukkit is the paper-init fallback probe.
        let probes = [
            "io/papermc/paper/plugin/PluginInitializerManager",
            "org/bukkit/Bukkit",
            "io/papermc/paperclip/Main",
        ];
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        let mut loader_found = false;
        loop {
            if probes.iter().any(|p| cplug_sdk::classes::find_class(p).is_some()) {
                loader_found = true;
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!("[crussty-plugin] paletted: no launch-loader probe sighted within 120s, hook stays dormant");
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        let _ = loader_found;

        // Grab the launch loader from the first sighted probe class.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            for probe in probes {
                let Some(cls) = cplug_sdk::classes::find_class(probe) else { continue };
                let Some(class_cls) = env.find_class("java/lang/Class") else {
                    crate::clear_exception(env);
                    continue;
                };
                let Some(mid) = env.get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;") else {
                    crate::clear_exception(env);
                    env.delete_local_ref(class_cls);
                    continue;
                };
                let loaded = env.call_object_method(cls.as_jclass(), mid, &[]);
                if loaded as usize == 0 {
                    crate::clear_exception(env);
                    env.delete_local_ref(class_cls);
                    continue;
                }
                let ok = match env.define_class(OPS_NAME, loaded, OPS_BYTES) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] paletted: defined {OPS_NAME} in launch loader (probe {probe})");
                        true
                    }
                    None => {
                        crate::clear_exception(env);
                        eprintln!("[crussty-plugin] paletted: define_class({OPS_NAME}) failed via {probe}");
                        false
                    }
                };
                env.delete_local_ref(loaded);
                env.delete_local_ref(class_cls);
                if ok {
                    return true;
                }
            }
            false
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] paletted: Ops definition failed — hook stays dormant (vanilla reads)");
            return;
        }
        READY.store(true, Ordering::Release);
        eprintln!(
            "[crussty-plugin] paletted: PALETTED-DEMUX ARMED (ready to serve the patched {} at first load; {} Ops bytes)",
            PALETTED_CLASS,
            OPS_BYTES.len()
        );
    });
}
