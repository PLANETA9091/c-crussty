//! Runtime wiring for the INSIDE-CACHE lever (S7-135, TASK-271 — ARCH-ATTACK
//! lever #3, the inside-blocks/fluid discovery lane).
//!
//! Patches ONE kernel call site (see src/classfile.rs INSIDE-CACHE section
//! for the javap contract): the method-entry gate
//! `Entity.isAffectedByBlocks` INSIDE
//! `Entity.checkInsideBlocks(List, StepBasedCollector)` retargeted to the
//! static `InsideBlockOps.gate(Entity)Z` (receiver-first, 3B→3B,
//! length-preserving). For static entities (deltaMovement==0, position
//! bit-equal to the cached tick) the bridge serves the whole discovery
//! from flat primitive slot arrays (replay of the vanilla effect calls);
//! anything else falls through to `e.isAffectedByBlocks()` — the vanilla
//! body runs untouched.
//!
//! S7-162 (single compose-chain): this module now only OWNS THE BRIDGE —
//! it defines `InsideBlockOps` + `InsideBlockOps$Recorder` into the
//! kernel loader and publishes BRIDGE_READY for the compose worker. The
//! Entity byte patch itself moved to entity_compose (stage 1): two hooks
//! on one class serve whole-class caches and SUPERSEDE each other (leg #5
//! 35381522360 evidence lines 886/895 — the inside bytes 205522 were
//! overwritten by the region serve 205494, the inside_cache gate was
//! silently dead in the v2 bank).
//!
//! Gate: env `CRUSSTY_INSIDE_CACHE` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline (same as fluid_guard/alloc_diet): with the
//! gate off, register() logs a dormant notice and NO byte hook exists,
//! activate() returns immediately, the module is
//! byte-indistinguishable from the pre-S7-135 plugin.
//!
//! Fail-closed matrix: bridge define failure → BRIDGE_READY never set →
//! the compose chain continues WITHOUT inside (fail-dominant, loud WARN);
//! Entity patch-level failures are handled by the entity_compose stage
//! (strict Retargeted/AlreadyPatched check, patcher Err → stage skipped).

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps";
const RECORDER_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps$Recorder";

const OPS_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBlockOps.class");
const RECORDER_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBlockOps$Recorder.class");

fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_CACHE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate visibility for the fluid_dirty chain WARN (S7-151) and the
/// entity_compose stage enablement (S7-162).
pub fn enabled_pub() -> bool {
    enabled()
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel classloader, captured at activation (0 = none).
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// S7-162: pollable gate for the entity_compose stage pipeline — the
/// compose worker needs the bridge classes DEFINED before the composed
/// Entity bytes resolve InsideBlockOps on the first gated call.
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

/// Register (idempotent; call once from cplugin_init). S7-162: NO byte hook
/// is installed here — the Entity patch composes through entity_compose.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_cache: dormant (set CRUSSTY_INSIDE_CACHE=1 to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_cache: bridge owner armed, Entity stage delegated to entity_compose (S7-162 single compose-chain)"
    );
}

/// Background activation: wait for the kernel Entity class (the kernel
/// loader capture point), define both bridge classes into the kernel
/// loader, publish BRIDGE_READY. The Entity patch is computed and applied
/// by entity_compose (stage 1).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // Wait for the kernel Entity class (loads at boot).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_cache: {} not loaded within 180s, bridge stays undefined",
                    ENTITY_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_cache: forcing kernel load of {}",
                    ENTITY_CLASS
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] inside_cache: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!(
            "[crussty-plugin] inside_cache: server booted, defining bridge into kernel loader"
        );

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (RECORDER_CLASS, RECORDER_BYTES)] {
            let major = crate::improved_noise::class_version(bytes)
                .map(|(m, _)| m)
                .unwrap_or(0);
            if major > jvm_major {
                eprintln!(
                    "[crussty-plugin] inside_cache: {name} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ via scripts/build_inside_block_ops.sh; hook stays dormant"
                );
                return;
            }
        }

        // Capture the kernel loader global ref from Entity.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            let mut ok = true;
            for (name, bytes) in [(OPS_CLASS, OPS_BYTES), (RECORDER_CLASS, RECORDER_BYTES)] {
                match env.define_class(name, gref, bytes) {
                    Some(c) => {
                        env.delete_local_ref(c);
                        eprintln!("[crussty-plugin] inside_cache: defined {name} in kernel loader");
                    }
                    None => {
                        crate::describe_exception(env);
                        eprintln!("[crussty-plugin] inside_cache: define_class({name}) failed");
                        ok = false;
                        break;
                    }
                }
            }
            ok
        });
        if !defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] inside_cache: bridge definition aborted, hook stays dormant");
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "gate", "inside_cache v1");
        eprintln!(
            "[crussty-plugin] inside_cache: bridge defined, BRIDGE_READY (Entity stage composes via entity_compose)"
        );
    });
}
