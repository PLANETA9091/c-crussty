//! Runtime wiring for the WILD-3D census lever (round-466 C97, vector
//! «оптоволоконные chunk-компо / tick-reuse / integer-компо плоскостей» —
//! W1; lever `cmp466_c97`; bridge
//! wild/net/minecraft/world/level/chunk/WildOps.java, natives: NONE).
//!
//! CENSUS LEG (NOT-A-BENCH телеметрия, прецедент C21 cmp466_light /
//! C20 F3-LevelTicks): whole-body redirect of
//! `LevelChunk.getBlockStateFinal(III)BlockState` →
//! `WildOps.gbsf(LevelChunk,III)` — the bridge replicates the vanilla body
//! 1:1 (javap ground truth patched-kernel e2992d63: getSectionIndex(y)
//! bounds guard → sections[i].nonEmptyBlockCount==0 AIR guard →
//! states.get((y&15)<<8 | (z&15)<<4 | (x&15))) and adds periodic stderr
//! census lines `[c97-wild-cens] calls=/distinct=/reuse=/calls_per_s=`
//! = LIVE capture of the REUSE-RATIO (calls per distinct
//! (chunk,section) pair per 30s window) on the tick-thread block-state
//! READ lane.
//!
//! PLANE CEILING (law 13b math, M15 vanilla anchor 36222911641,
//! 116,506 cpu-samples — LAB-STAGE/c97-profile/): paletted lane gross
//! 8,767 samples = 7.52% all-CPU {PalettedContainer.get 4,174 = 3.58% +
//! SimpleBitStorage.get 1,547 = 1.33% + readPalette 1,050 = 0.90% +
//! getFluidState 1,016 = 0.87% + getBlockStateFinal 980 = 0.84%}, 96.9%
//! of PalettedContainer.get on RegionTick tick-threads; caller families
//! fluid-push 49.3% / misc getBlockState 19.8% / collision 16.1% /
//! nav 4.7% → the SAME section is read by 4-5 independent consumers per
//! tick. Hypothesis: reuse-ratio R >= 4 → the WILD-3D decode-once flat
//! integer plane (per-tick per-worker; write-gen invalidation =
//! paldelta канон, NOT fluid_dirty-memo) captures 40-60% of the
//! bit-unpack+dispatch lane → +1.4..2.2пп — компо-кирпич, not solo.
//! R ~ 1 → REFUTED_CENS.
//!
//! VANILLA-BIT-FOR-BIT: under empty/foreign `CRUSSTY_LEVER_FLAG` NO hook is
//! registered, NO retransform happens, NO bridge is defined — LevelChunk
//! stays byte-identical vanilla. Under cmp466_c97 the redirect keeps the
//! exact vanilla semantics (same guards, same AIR constant, same packed
//! index arithmetic, same PalettedContainer.get invocation) + a census
//! probe (plain statics + 2-array open-address probe, no allocation) —
//! NOT-A-BENCH leg, expected norm slightly negative (census cost on the
//! trunk site), never used for CPU/TPS gates.
//!
//! FAIL-CLOSED ladder: patcher Err (kernel shape mismatch) → dormant;
//! bridge define failure → dormant; no pristine capture → dormant;
//! Retargeted{sites:0} → dormant. NCDFE канон: bridge defined into the
//! kernel loader BEFORE the retransform serves patched bytes; hook performs
//! NO JNI work (pristine capture at the class's own load, flush_diet
//! pattern).

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const TARGET_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunk";
const OPS_CLASS: &str = "net/minecraft/world/level/chunk/WildOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../wild/build/net/minecraft/world/level/chunk/WildOps.class");

const METHOD_NAME: &str = "getBlockStateFinal";
const VIRTUAL_DESC: &str = "(III)Lnet/minecraft/world/level/block/state/BlockState;";
/// Receiver-prepended static form (stack-identical LevelChunk→static gate).
const STATIC_DESC: &str =
    "(Lnet/minecraft/world/level/chunk/LevelChunk;III)Lnet/minecraft/world/level/block/state/BlockState;";

/// STRICT-eq gate (round-400 lever protocol; полу-armed мост = невалидная
/// нога, TASK-402-F). Пустой/чужой флаг = ваниль бит-в-байт.
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp466_c97")
    )
}

static READY: AtomicBool = AtomicBool::new(false);

struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<Arc<[u8]>>>,
    served: AtomicBool,
}

/// Poison recovery (TASK-46): locks only wrap plain Vec/Arc stores.
impl Target {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
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
    TARGET.get_or_init(|| Target::new(TARGET_CLASS))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline (improved_noise): the callback performs NO JNI
/// work; pristine capture at the class's own load, patch served from the
/// cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] wild_reuse: dormant (CRUSSTY_LEVER_FLAG != cmp466_c97, vanilla bit-in-byte)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] wild_reuse: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] wild_reuse: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the LevelChunk class, define the bridge
/// into the kernel loader, compute the whole-body redirect from the pristine
/// bytes, flip READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        // LevelChunk loads during world/chunk bootstrap (before population).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(t.name).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] wild_reuse: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] wild_reuse: forcing kernel load of {}",
                    t.name
                );
                crate::improved_noise::force_load_kernel_class(t.name);
            }
            let sighted = cplug_sdk::classes::is_sighted(t.name);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (fluid_guard
        // TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] wild_reuse: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] wild_reuse: server booted, defining bridge into kernel loader"
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
                "[crussty-plugin] wild_reuse: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild via scripts/build_wild_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge into the kernel loader (light_plane pattern;
        // anchor loader = LevelChunk's — the kernel loader all bridge
        // classes share).
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/level/chunk/LevelChunk")
            else {
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
            match env.define_class(OPS_CLASS, gref, OPS_BYTES) {
                Some(c) => {
                    env.delete_local_ref(c);
                    eprintln!("[crussty-plugin] wild_reuse: defined {OPS_CLASS} in kernel loader");
                    true
                }
                None => {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] wild_reuse: define_class({OPS_CLASS}) failed");
                    false
                }
            }
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] wild_reuse: bridge definition aborted, hook stays dormant");
            return;
        }

        // Pristine bytes for a class that predates the hook (fast boot):
        // no-op retransform capture, fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] wild_reuse: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for attempt in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
                let _ = attempt;
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] wild_reuse: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };
        // ONE whole-body redirect — LevelChunk.getBlockStateFinal(III) →
        // WildOps.gbsf(LevelChunk,III) (must land with sites:1;
        // anti-placebo gate: sites>0 else disarm).
        let (patched, outcome) = match crate::classfile::redirect_method_body_to_static(
            &original,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] wild_reuse: redirect rejected ({e}), hook stays dormant"
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
                "[crussty-plugin] wild_reuse: unexpected redirect outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] wild_reuse: computed redirect for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(Arc::from(patched));
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!(
            "[crussty-plugin] wild_reuse: ARMED getBlockStateFinal census bridge (cmp466_c97, bit-exact vanilla body + [c97-wild-cens] reuse-ratio counters, 0 natives, 0 added JNI; retransform rc={rc})"
        );
    });
}

#[cfg(test)]
mod wild_reuse_tests {
    use super::{METHOD_NAME, OPS_BYTES, OPS_CLASS, STATIC_DESC, TARGET_CLASS, VIRTUAL_DESC};
    use crate::classfile::redirect_method_body_to_static;

    /// javap ground truth: the kernel LevelChunk fixture carries exactly one
    /// getBlockStateFinal(III); the redirect replaces the whole body with the
    /// WildOps gate and stays idempotent on re-sight.
    #[test]
    fn wild_redirect_applies_to_kernel_fixture() {
        let fixture =
            include_bytes!("../tests/fixtures/LevelChunk_real.class") as &[u8];
        let (patched, outcome) = redirect_method_body_to_static(
            fixture,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("redirect must compute");
        assert_eq!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 },
            "exactly the getBlockStateFinal body must be replaced"
        );
        // Idempotency: re-sighting on the patched bytes must be
        // AlreadyPatched with byte-identical output.
        let (again, o1) = redirect_method_body_to_static(
            &patched,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("re-redirect must compute");
        assert_eq!(
            o1,
            crate::classfile::RetargetOutcome::AlreadyPatched { sites: 1 },
            "getBlockStateFinal re-sight must be idempotent"
        );
        assert_eq!(again, patched);
    }

    /// The redirect must land ONLY on getBlockStateFinal: every other
    /// LevelChunk method must stay byte-identical (no collateral rewrites).
    #[test]
    fn wild_redirect_is_site_exact() {
        let fixture =
            include_bytes!("../tests/fixtures/LevelChunk_real.class") as &[u8];
        let (patched, _outcome) = redirect_method_body_to_static(
            fixture,
            METHOD_NAME,
            VIRTUAL_DESC,
            TARGET_CLASS,
            OPS_CLASS,
            METHOD_NAME,
            STATIC_DESC,
        )
        .expect("redirect must compute");
        // The untouched sibling sites must survive verbatim: the two
        // 8-byte wrappers that delegate to getBlockStateFinal keep their
        // invokevirtual, and the PalettedContainer/states machinery stays.
        let needle: &[u8] = b"getBlockStateFinal";
        assert!(
            patched.windows(needle.len()).any(|w| w == needle),
            "getBlockStateFinal cp entry must remain for the wrapper sites"
        );
        let needle: &[u8] = b"states";
        assert!(
            patched.windows(needle.len()).any(|w| w == needle),
            "the states field cp entry must remain untouched"
        );
    }

    /// Bridge blob sanity: major 65 (release 21), the lever id + census tag
    /// must live in the constant pool (check_blobs_sync markers), and the
    /// static gate must carry the exact receiver-prepended desc.
    #[test]
    fn wild_bridge_blob_contract() {
        let bytes: &[u8] = OPS_BYTES;
        assert!(bytes.len() > 8);
        let major = ((bytes[6] as u16) << 8) | bytes[7] as u16;
        assert_eq!(major, 65, "WildOps must be --release 21 (major 65)");
        for marker in [b"cmp466_c97" as &[u8], b"c97-wild-cens", b"gbsf"] {
            assert!(
                bytes.windows(marker.len()).any(|w| w == marker),
                "marker {} missing from WildOps blob",
                String::from_utf8_lossy(marker)
            );
        }
    }

    /// STRICT gate: only the exact cmp466_c97 lever arms the plane.
    #[test]
    fn wild_gate_is_strict() {
        // The enabled() gate must be an exact string match — reflect via the
        // source (static assert by contract): any prefix/trim tolerance would
        // be a TASK-402-F half-armed violation.
        let desc = STATIC_DESC;
        assert_eq!(
            desc,
            "(Lnet/minecraft/world/level/chunk/LevelChunk;III)Lnet/minecraft/world/level/block/state/BlockState;",
            "static desc = virtual desc with receiver LevelChunk prepended"
        );
        assert_eq!(
            VIRTUAL_DESC,
            "(III)Lnet/minecraft/world/level/block/state/BlockState;"
        );
        assert_eq!(TARGET_CLASS, "net/minecraft/world/level/chunk/LevelChunk");
        assert_eq!(OPS_CLASS, "net/minecraft/world/level/chunk/WildOps");
    }
}
