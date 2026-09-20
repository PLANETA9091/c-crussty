//! Runtime wiring for FLUID-BITMASK (RECON-43 ARCH-LEVER #16, TASK-389).
//!
//! The owner directive 2026-09-21 «заменяй реальные архитектуры» + анти-повтор
//! протокол: the flagged flagship of RECON-43 (contract ec880c2). This module
//! ONLY defines `FluidBitmaskOps` (+$Entry) into the KERNEL loader — the gate
//! itself lives inside the banked FluidPushGuardHook body (fluid_guard, TASK-80),
//! which consults it BEFORE its own cache/slow path. No byte hook, no
//! retransform, no Entity retarget: the Entity bytes are untouched, the
//! entity_compose chain is untouched.
//!
//! Invalidation: FluidPushOps.LEDGER dirty stamps (bumped by the fluid_dirty
//! LevelChunk.setBlockState retarget ONLY on real fluid-state changes) +
//! section object identity. The leg therefore arms CRUSSTY_FLUID_DIRTY_LEDGER=1
//! (ledger-only split, no refuted memo stage) alongside this gate.
//!
//! Ordering: activate() is called BEFORE fluid_guard::activate() in lib.rs so
//! the ops class is defined before the hook is (re)defined. Any residual race
//! is absorbed by the hook's NoClassDefFoundError-tolerant consult (retry per
//! invocation, fail-dominant on real Throwables).
//!
//! Gate: env `CRUSSTY_FLUID_BITMASK` (1/true/on/yes -> on). Off by default —
//! dormant-invisible discipline.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/FluidBitmaskOps";
const OPS_ENTRY_CLASS: &str = "net/minecraft/world/entity/FluidBitmaskOps$Entry";

const OPS_BYTES: &[u8] =
    include_bytes!("../fluid/build/net/minecraft/world/entity/FluidBitmaskOps.class");
const OPS_ENTRY_BYTES: &[u8] =
    include_bytes!("../fluid/build/net/minecraft/world/entity/FluidBitmaskOps$Entry.class");

static ARMED: AtomicBool = AtomicBool::new(false);

pub fn armed() -> bool {
    ARMED.load(Ordering::Relaxed)
}

fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_FLUID_BITMASK").as_deref(),
        Ok("1") | Ok("true") | Ok("on") | Ok("yes")
    )
}

/// Define the ops pair into the kernel loader as soon as Entity is visible
/// (kernel loader live). Runs in background like every other wiring module.
pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] fluid_bitmask: dormant (set CRUSSTY_FLUID_BITMASK=1 to enable)"
        );
        return;
    }
    std::thread::Builder::new()
        .name("crussty-fluid-bitmask".into())
        .spawn(move || {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                let ok = cplug_sdk::jni_util::with_attached(|env| {
                    let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
                        crate::clear_exception(env);
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
                    env.delete_local_ref(loader);
                    env.delete_local_ref(class_cls);
                    let mut defined = 0usize;
                    for (name, bytes) in [
                        (OPS_CLASS, OPS_BYTES),
                        (OPS_ENTRY_CLASS, OPS_ENTRY_BYTES),
                    ] {
                        match env.define_class(name, gref, bytes) {
                            Some(c) => {
                                env.delete_local_ref(c);
                                defined += 1;
                                eprintln!(
                                    "[crussty-plugin] fluid_bitmask: defined {name} in kernel loader"
                                );
                            }
                            None => {
                                crate::describe_exception(env);
                                eprintln!(
                                    "[crussty-plugin] fluid_bitmask: define_class({name}) failed"
                                );
                            }
                        }
                    }
                    defined == 2
                });
                if ok {
                    ARMED.store(true, Ordering::SeqCst);
                    eprintln!("[crussty-plugin] fluid_bitmask: gate armed (hook consult live)");
                    return;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] fluid_bitmask: kernel loader never went quiet in 180s — gate stays uninstalled (fail-dominant)"
                    );
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        })
        .ok();
}
