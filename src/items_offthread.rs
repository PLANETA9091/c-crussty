//! Runtime wiring for the ITEMS-OFFTHREAD lever (MEGA-ROUND-2, TASK-397-D —
//! `items_offthread`: off-thread стадия merge-поиска на worker-ах RegionTickOps).
//!
//! Механизм (раунд-1 вектор E — НИКОГДА не имплементирован, свободен):
//!  - СКАН: воркер RegionTickOps, тикнувший свой бакет, сразу после свопа
//!    сканирует СОБСТВЕННЫЙ бакет (post-sweep позиции = состояние, которое
//!    каждый item видел в свой тик; кросс-поточных чтений entity НЕТ) и
//!    продюсирует merge-решения в per-slot буфер (бакетный хеш cell=2.0,
//!    stable counting placement — детерминизм без сортировки).
//!  - ПРИМЕНЕНИЕ: main после DONE-барьера (phase inactive = ванильный
//!    протокол мутаций EntityTickList) применяет решения в порядке слотов
//!    0..W-1; каждый применённый merge = НАСТОЯЩИЙ приватный vanilla
//!    tryToMerge через MethodHandle (семантика слияния байт-в-байт).
//!
//! Byte hook (строгий, fail-closed): ItemEntity.tick()V — единственный
//! горячий call site `invokevirtual mergeWithNeighbours()V` (census по
//! javap s7204: 1 site в tick, 1 в teleport — teleport остаётся ванильным,
//! редкий путь). Ретаргет на `ItemMergeOps.tickMerge(ItemEntity)V`
//! length-preserving (invokevirtual->invokestatic, 3 байта, CP-append).
//! Dormant (флаг не наш / region_threads<2 / steal-режим) = ванильные байты.
//! Armed без hook-wiring = Java-fallback: точная public-API реплика ванильного
//! тела (parity by construction).
//!
//! Патч-пайплайн повторяет flush_diet/items_index (TASK-395 agent-A):
//! byte hook ловит престины ItemEntity на первой загрузке; фоновый активатор
//! ждёт boot, определяет ItemMergeOps в kernel loader (статик-иниц вешает
//! hooks в RegionTickOps + ARMED), считает патч, один retransform.
//!
//! Gate: `CRUSSTY_LEVER_FLAG == "items_offthread"` + CRUSSTY_REGION_THREADS>=2
//! + CRUSSTY_REGION_STEAL != "1" (scan сидит в tickBucket static-bucket пути).
//! Любой другой флаг — dormant-invisible: ванильный путь по построению.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};

const TARGET_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/item/ItemMergeOps";

const OPS_BYTES: &[u8] = include_bytes!(
    "../entityinside/build/net/minecraft/world/entity/item/ItemMergeOps.class"
);
const OPS_SLOTSTATE_BYTES: &[u8] = include_bytes!(
    "../entityinside/build/net/minecraft/world/entity/item/ItemMergeOps$SlotState.class"
);

fn lever_enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("items_offthread"))
        .unwrap_or(false)
}

fn workers_from_env() -> Option<i64> {
    std::env::var("CRUSSTY_REGION_THREADS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .filter(|w| *w >= 2)
}

fn region_steal_v1() -> bool {
    std::env::var("CRUSSTY_REGION_STEAL")
        .map(|v| v.trim() == "1")
        .unwrap_or(false)
}

fn enabled() -> bool {
    lever_enabled() && workers_from_env().is_some() && !region_steal_v1()
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
/// Loader-lock discipline: the callback does NO JNI work; pristine capture at
/// the class's own load, patch served from the cache computed on the quiet
/// activation worker. ItemEntity is the SOLE owner of this hook in the
/// compose chain (entity_compose stages target Entity, not ItemEntity).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] items_offthread: dormant (set CRUSSTY_LEVER_FLAG=items_offthread \
             + CRUSSTY_REGION_THREADS>=2, REGION_STEAL!=1 to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] items_offthread: pristine sighting {} {} bytes (major {})",
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
                "[crussty-plugin] items_offthread: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for ItemEntity to load, wait for a quiet boot,
/// define ItemMergeOps (+SlotState) into the kernel loader — its static init
/// binds vanilla private MethodHandles and wires the RegionTickOps hooks —
/// compute the length-preserving tick() patch from the pristine bytes, flip
/// READY and retransform once.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::Builder::new()
        .name("crussty-items-offthread".into())
        .spawn(move || {
            let t = target();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
            loop {
                if cplug_sdk::classes::find_class(t.name).is_some() {
                    break;
                }
                if std::time::Instant::now() > deadline {
                    eprintln!(
                        "[crussty-plugin] items_offthread: {} not loaded within 180s, hook stays dormant",
                        t.name
                    );
                    return;
                }
                if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                    eprintln!(
                        "[crussty-plugin] items_offthread: forcing kernel load of {}",
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
                eprintln!(
                    "[crussty-plugin] items_offthread: boot marker not seen, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(20));
            eprintln!(
                "[crussty-plugin] items_offthread: server booted, defining ops pair into kernel loader"
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
                    "[crussty-plugin] items_offthread: {OPS_CLASS} is class major {major} but JVM \
                     supports up to {jvm_major} — rebuild entityinside via javac --release 21; \
                     hook stays dormant"
                );
                return;
            }

            // Define the ops pair into the kernel loader (Entity loader anchor,
            // fluid_bitmask/flush_diet/items_index pattern). Static init runs
            // at define and arms the Java side (MethodHandles + hook wiring
            // with its own retry daemon — RegionTickOps may land after us).
            let defined = cplug_sdk::jni_util::with_attached(|env| {
                let Some(cls) = cplug_sdk::classes::find_class("net/minecraft/world/entity/Entity")
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
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                let mut defined = 0usize;
                for (name, bytes) in [
                    (OPS_CLASS, OPS_BYTES),
                    ("net/minecraft/world/entity/item/ItemMergeOps$SlotState", OPS_SLOTSTATE_BYTES),
                ] {
                    match env.define_class(name, gref, bytes) {
                        Some(c) => {
                            env.delete_local_ref(c);
                            defined += 1;
                            eprintln!(
                                "[crussty-plugin] items_offthread: defined {name} in kernel loader"
                            );
                        }
                        None => {
                            crate::describe_exception(env);
                            eprintln!(
                                "[crussty-plugin] items_offthread: define_class({name}) failed"
                            );
                        }
                    }
                }
                defined == 2
            });
            if !defined.unwrap_or(false) {
                eprintln!(
                    "[crussty-plugin] items_offthread: ops definition aborted, hook stays dormant"
                );
                return;
            }

            // Pristine bytes for a class that predates the hook (fast boot):
            // no-op retransform capture, fluid_guard pattern.
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] items_offthread: {} predates hook, capturing via no-op retransform",
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
                        "[crussty-plugin] items_offthread: no pristine bytes for {}, hook stays dormant",
                        t.name
                    );
                    return;
                }
            }

            let Some(original) = t.take_orig() else {
                return;
            };
            let (patched, outcome) =
                match crate::classfile::patch_itementity_tick_merge(&original) {
                    Ok(pair) => pair,
                    Err(e) => {
                        eprintln!(
                            "[crussty-plugin] items_offthread: patch rejected ({e}), hook stays dormant"
                        );
                        return;
                    }
                };
            let retargeted = matches!(
                outcome,
                crate::classfile::RetargetOutcome::Retargeted { .. }
                    | crate::classfile::RetargetOutcome::AlreadyPatched { .. }
            );
            if !retargeted {
                eprintln!(
                    "[crussty-plugin] items_offthread: unexpected patch outcome ({outcome:?}), hook stays dormant"
                );
                return;
            }
            eprintln!(
                "[crussty-plugin] items_offthread: computed patch for {} ({} -> {} bytes, {outcome:?})",
                t.name,
                original.len(),
                patched.len()
            );
            t.set_patch(Arc::from(patched));

            crate::kernel_policy::audit_wire(OPS_CLASS, "tickMerge", "items_offthread v1");
            READY.store(true, Ordering::Release);
            let rc = cplug_sdk::retransform_class(t.name);
            eprintln!(
                "[crussty-plugin] items_offthread: {} armed, retransform rc={rc}",
                t.name
            );
        })
        .ok();
}
