//! Runtime wiring for the INSIDE-DIET lever (ARCH-ATTACK lever #12 v1,
//! TASK-332 — see entityinside/net/minecraft/world/entity/{InsideDietOps,
//! InsideDietVisitor}.java).
//!
//! The inside-blocks lane is the largest single attackable sub-lane of the
//! TOP-1 entity-tick-core: every moving entity runs
//! Entity.checkInsideBlocks every tick; the vanilla 5-arg body allocates
//! PER CALL: makeBoundingBox + deflate (2 AABB), one AtomicInteger, one
//! 10-capture lambda visitor. Fresh profile (s7173, bank v3 + lever #11
//! measurement): inside-blocks = 31.18% of ALL alloc samples / 9.20% CPU
//! (recon14c_alloc_callers.py / RECON14C_ALLOC_CALLERS.md).
//!
//! The lever body-redirects the private instance method
//! Entity.checkInsideBlocks(Vec3,Vec3,StepBasedCollector,LongSet,int)I to
//! InsideDietOps.checkInsideBlocks(Entity,...) — same observable values
//! (bit-identical box fields, same vanilla static walk, same visit
//! transcription) while the glue drops to ONE box + ONE visitor object per
//! call (v2 will attack the walk itself; a full DDA transcription diverged
//! 21/350k in the offline harness and was REJECTED — see InsideDietOps doc).
//!
//! Delivery: the bridge classes are defined into the KERNEL loader and the
//! body-redirect composes through the entity_compose stage-9 chain (the
//! Entity class is already composed by inside_cache/zero_alloc stages —
//! requires CRUSSTY_REGION_THREADS>=2). Fail-closed: any delivery defect
//! leaves the stage out of the chain (vanilla checkInsideBlocks, zero risk).

use std::sync::atomic::{AtomicBool, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideDietOps";
const VISITOR_CLASS: &str = "net/minecraft/world/entity/InsideDietVisitor";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideDietOps.class");
const VISITOR_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideDietVisitor.class");

const CHECK_INSIDE_NAME: &str = "checkInsideBlocks";
const CHECK_INSIDE_DESC: &str = "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I";
const TARGET_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I";

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_INSIDE_DIET")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "on" | "yes"
    )
}

pub fn enabled_pub() -> bool {
    enabled()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

/// Pollable gate for the entity_compose stage pipeline — the redirected
/// body resolves `InsideDietOps` on first run, so the classes MUST be
/// defined before the Entity retransform is served.
pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    if !enabled() {
        return false;
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while std::time::Instant::now() < deadline {
        if BRIDGE_READY.load(Ordering::Acquire) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    BRIDGE_READY.load(Ordering::Acquire)
}

pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_diet: dormant (set CRUSSTY_INSIDE_DIET=1 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] inside_diet: requires CRUSSTY_REGION_THREADS>=2 (the redirect composes through the entity_compose chain), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_diet: boot marker not seen, hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(15));

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (nm, bytes) in [(OPS_CLASS, OPS_BYTES), (VISITOR_CLASS, VISITOR_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] inside_diet: {nm} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
                );
                return;
            }
        }

        // RESOLUTION CLOSURE GUARD (zero_alloc leg#1 lesson): the embedded
        // bridge bytes MUST declare every member the redirect + the visitor
        // contract emit; otherwise the first entity tick detonates a
        // NoSuchMethodError storm. Fail-closed here -> dormant + loud log.
        if let Err(e) = crate::classfile::insidediet_resolution_closure(OPS_BYTES, VISITOR_BYTES) {
            eprintln!(
                "[crussty-plugin] inside_diet: RESOLUTION CLOSURE FAILED: {e} — hook stays dormant"
            );
            return;
        }

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            for (nm, bytes) in [(OPS_CLASS, OPS_BYTES), (VISITOR_CLASS, VISITOR_BYTES)] {
                let Some(c) = env.define_class(nm, gref, bytes) else {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] inside_diet: define_class({nm}) failed");
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
            eprintln!(
                "[crussty-plugin] inside_diet: bridge definition failed, hook stays dormant"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] inside_diet: defined {OPS_CLASS} + {VISITOR_CLASS} in kernel loader"
        );
        BRIDGE_READY.store(true, Ordering::Release);
    });
}

#[cfg(test)]
mod insidediet_delivery_tests {
    use super::{OPS_BYTES, VISITOR_BYTES};

    /// Delivery-graph guard (zero_cursor pattern): the bridge Java sources
    /// MUST declare ZERO nested classes — the two classfiles are defined
    /// alone into the kernel loader; a nested class would detonate as
    /// NoClassDefFoundError on the first entity tick.
    #[test]
    fn insidediet_sources_declare_no_nested_classes() {
        for src in [
            include_str!("../entityinside/net/minecraft/world/entity/InsideDietOps.java"),
            include_str!("../entityinside/net/minecraft/world/entity/InsideDietVisitor.java"),
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

    /// The embedded bytes must be real classfiles pinned to major 65.
    #[test]
    fn insidediet_embedded_classfiles_present_and_pinned() {
        for bytes in [OPS_BYTES, VISITOR_BYTES] {
            assert_eq!(&bytes[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
            let major = u16::from_be_bytes([bytes[6], bytes[7]]);
            assert_eq!(major, 65, "bridge major must be pinned to 65");
        }
    }

    /// s7171 delivery-defect guard mirrored for lever #12: the resolution
    /// closure MUST accept the embedded bridge bytes as compiled — pins the
    /// member expectations to the real classfiles so any drift fails here,
    /// offline, instead of silently disarming the stage.
    #[test]
    fn insidediet_resolution_closure_accepts_embedded_bridges() {
        crate::classfile::insidediet_resolution_closure(OPS_BYTES, VISITOR_BYTES)
            .expect("resolution closure must accept the embedded InsideDiet bridge bytes");
    }
}
