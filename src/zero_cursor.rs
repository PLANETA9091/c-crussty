//! Runtime wiring for the ZERO-CURSOR lever (ARCH-ATTACK lever #11 v1,
//! TASK-330 — see entityinside/net/minecraft/core/{ZeroCursorIter,
//! ZeroCursorOps}.java).
//!
//! The chunk-cursor lane is the largest single attackable sub-lane of the
//! TOP-1 entity-tick-core: every Entity.checkInsideBlocks ->
//! BlockGetter.forEachBlockIntersectedBetween call allocates a fresh
//! betweenCornersInDirection iterator (javac BlockPos$6) + a fresh
//! MutableBlockPos, and addCollisionsAlongTravel allocates one PER TRAVEL
//! STEP. Fresh profile (s7169, bank v3): cursor family = 29.18% of ALL
//! alloc samples, 6.42% CPU (recon14_fresh_top.py / RECON14_FRESH_TOP.md).
//!
//! The lever redirects the synthetic factory
//! BlockPos.lambda$betweenCornersInDirection$8 (the ONLY site that news
//! BlockPos$6 — cp census, recon13g) to ZeroCursorOps.lambda8, which returns
//! a pooled, per-call-reset ZeroCursorIter — a bit-exact replica of the
//! vanilla walk (CursorLockstepHarness: 350k scenarios / 31.7M positions
//! identical; the redirected factory receives the IDENTICAL arguments the
//! untouched vanilla 314B method computes, so the emission order cannot
//! drift). Pure observability of behavior: same positions, same order, same
//! visitor contract — only the allocation churn disappears.
//!
//! This module: (1) registers a byte hook on BlockPos (pristine capture at
//! the class's own load — it loads during early boot, before the plugin's
//! boot marker); (2) at boot+20s defines BOTH bridge classes into the KERNEL
//! loader (s7168 lesson: the dump-task arm needs the freshly-defined class
//! in hand — here we define, verify the resolution closure, patch, and
//! retransform in one attachment chain); (3) computes the length-preserving
//! static body-redirect and serves it on retransform. Fail-closed: any
//! delivery defect leaves the hook dormant (vanilla iterator, zero risk).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

const TARGET_CLASS: &str = "net/minecraft/core/BlockPos";
const ITER_CLASS: &str = "net/minecraft/core/ZeroCursorIter";
const OPS_CLASS: &str = "net/minecraft/core/ZeroCursorOps";

const ITER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/core/ZeroCursorIter.class");
const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/core/ZeroCursorOps.class");

const LAMBDA_NAME: &str = "lambda$betweenCornersInDirection$8";
const LAMBDA_DESC: &str =
    "(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIIII)Ljava/util/Iterator;";

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
        let mut g = self.orig.lock().unwrap();
        if g.is_none() {
            *g = Some(bytes.to_vec());
        }
    }

    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig.lock().unwrap().clone()
    }

    fn set_patch(&self, bytes: Arc<[u8]>) {
        *self.patch.lock().unwrap() = Some(bytes);
    }

    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch.lock().unwrap().clone()
    }
}

use std::sync::Arc;

fn target() -> &'static Target {
    static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
    TARGET.get_or_init(|| Target::new(TARGET_CLASS))
}

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_ZERO_CURSOR")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "on" | "yes"
    )
}

/// Register the byte hook (idempotent; call once from cplugin_init).
/// Loader-lock discipline: the callback performs NO JNI work.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] zero_cursor: dormant (set CRUSSTY_ZERO_CURSOR=1 to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] zero_cursor: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] zero_cursor: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

static READY: AtomicBool = AtomicBool::new(false);

/// Background activation: wait for boot, define BOTH bridges into the kernel
/// loader, capture pristine BlockPos via no-op retransform if the hook never
/// saw it, compute the static body-redirect, flip READY, retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let t = target();
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] zero_cursor: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!("[crussty-plugin] zero_cursor: server booted, defining bridges into kernel loader");

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (nm, bytes) in [(ITER_CLASS, ITER_BYTES), (OPS_CLASS, OPS_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] zero_cursor: {nm} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
                );
                return;
            }
        }

        // RESOLUTION CLOSURE GUARD: the redirected lambda body invokes
        // ZeroCursorOps.lambda8 with the factory descriptor; the bridge must
        // declare it (and ZeroCursorOps must reference ZeroCursorIter).
        if let Err(e) = crate::classfile::zerocursor_resolution_closure(OPS_BYTES, ITER_BYTES) {
            eprintln!(
                "[crussty-plugin] zero_cursor: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/core/BlockPos") else {
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
            let mut ok = true;
            for (nm, bytes) in [(ITER_CLASS, ITER_BYTES), (OPS_CLASS, OPS_BYTES)] {
                let Some(c) = env.define_class(nm, gref, bytes) else {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] zero_cursor: define_class({nm}) failed");
                    ok = false;
                    break;
                };
                env.delete_local_ref(c);
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            ok.then_some(())
        });
        if defined.is_none() {
            eprintln!("[crussty-plugin] zero_cursor: bridge definition failed, hook stays dormant");
            return;
        }
        eprintln!("[crussty-plugin] zero_cursor: defined {ITER_CLASS} + {OPS_CLASS} in kernel loader");

        // Pristine bytes for a class that predates the hook: no-op
        // retransform capture (fluid_guard pattern).
        if t.take_orig().is_none() {
            eprintln!(
                "[crussty-plugin] zero_cursor: {} predates hook, capturing via no-op retransform",
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
                    "[crussty-plugin] zero_cursor: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        let Some(original) = t.take_orig() else {
            return;
        };
        let (patched, outcome) = match crate::classfile::redirect_static_method_body_to_static(
            &original,
            LAMBDA_NAME,
            LAMBDA_DESC,
            OPS_CLASS,
            "lambda8",
            LAMBDA_DESC,
        ) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] zero_cursor: redirect rejected ({e}), hook stays dormant"
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
                "[crussty-plugin] zero_cursor: unexpected redirect outcome ({outcome:?}), hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] zero_cursor: computed redirect for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(std::sync::Arc::from(patched));
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] zero_cursor: {} armed, retransform rc={rc}", t.name);
    });
}

#[cfg(test)]
mod zerocursor_delivery_tests {
    /// S7-163/S7-164 delivery-graph guard mirrored for lever #11: EACH
    /// bridge source MUST declare ZERO nested classes (the two classfiles
    /// are defined alone into the kernel loader; a nested class would
    /// detonate as NoClassDefFoundError on the first cursor walk — the
    /// offline lockstep harness cannot catch a missing nested classfile).
    #[test]
    fn zerocursor_sources_declare_no_nested_classes() {
        for src in [
            include_str!("../entityinside/net/minecraft/core/ZeroCursorIter.java"),
            include_str!("../entityinside/net/minecraft/core/ZeroCursorOps.java"),
        ] {
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
    }

    /// The embedded bytes must exist and be real classfiles (build-step
    /// ran; major 65 = kernel JVM).
    #[test]
    fn zerocursor_embedded_classfiles_present_and_pinned() {
        let blobs: &[&[u8]] = &[
            include_bytes!("../entityinside/build/net/minecraft/core/ZeroCursorIter.class"),
            include_bytes!("../entityinside/build/net/minecraft/core/ZeroCursorOps.class"),
        ];
        for bytes in blobs {
            assert_eq!(&bytes[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
            let major = u16::from_be_bytes([bytes[6], bytes[7]]);
            assert_eq!(major, 65, "bridge major must be pinned to 65");
        }
    }
}
