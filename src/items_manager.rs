//! Runtime wiring for the ITEM-MANAGER lever (TASK-395 mega-round, agent J —
//! vector items_manager; see entityinside/.../ItemEntityManager.java and the
//! ITEM-MANAGER block in RegionTickOps.java).
//!
//! ARCHITECTURE: полная замена диспетч-структуры item-фазы. ItemEntity
//! исключаются из общего entity-tick dispatch (guardEntityTick →
//! tickNonPassenger → ItemEntity.tick) и тикаются батч-фазами из
//! RegionTickOps.tickBucket через ItemEntityManager.tickSlot:
//!   (1) gather — per-slot плотные массивы в порядке снапшота EntityTickList
//!       (RegionTickOps.fill);
//!   (2) batch-движение — побайтная реплика ItemEntity.tick через ванильные
//!       Entity-методы (baseTick/move/applyEffectsFromBlocks/...);
//!   (3) merge — ВАНИЛЬНЫЙ приватный mergeWithNeighbours по MethodHandle,
//!       вызов в точном ванильном месте цикла;
//!   (4) age/despawn — в теле прохода, ванильные ItemDespawnEvent/discard;
//!   (5) применение к ядру — только ванильные методы/коллбеки.
//!
//! DELIVERY: define-only мост (паттерн batch_collector S7-160) — класс
//! ItemEntityManager определяется в KERNEL loader при активации; армирование
//! происходит в RegionTickOps.itemsManagerArmed() (lazy, fail-closed) при
//! CRUSSTY_LEVER_FLAG=items_manager и region_threads>=2 (статический режим).
//! Никаких новых retarget'ов kernel-классов не требуется: точка входа — уже
//! ретаргеченный RegionTickOps.forEach, правки только внутри bridge-классов.
//!
//! Fail-closed: define failure -> класс не в loader'е -> NoClassDefFoundError
//! ловится в itemsManagerArmed() -> ванильный путь; пустой CRUSSTY_LEVER_FLAG
//! -> armed()=false -> ванильный путь по построению.

use std::sync::atomic::{AtomicBool, Ordering};

const IM_CLASS: &str = "net/minecraft/world/entity/ItemEntityManager";

const IM_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemEntityManager.class");

fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("items_manager"))
        .unwrap_or(false)
}

pub fn activate() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] items_manager: dormant (set CRUSSTY_LEVER_FLAG=items_manager to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] items_manager: requires CRUSSTY_REGION_THREADS>=2 (item-фаза встаёт в RegionTickOps.tickBucket), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(|| {
        // Boot discipline: same as batch_collector (quiet loader before define).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] items_manager: boot marker not seen, hook stays dormant"
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
        let major = crate::improved_noise::class_version(IM_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] items_manager: {IM_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant"
            );
            return;
        }

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
            let Some(c) = env.define_class(IM_CLASS, gref, IM_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] items_manager: define_class({IM_CLASS}) failed");
                return false;
            };
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] items_manager: defined {IM_CLASS} in kernel loader");
        } else {
            eprintln!(
                "[crussty-plugin] items_manager: bridge definition failed, hook stays dormant"
            );
        }
    });
}
