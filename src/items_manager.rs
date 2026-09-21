//! Runtime wiring for the ITEM-SUBSYS2 lever (TASK-397 mega-round-2, agent J —
//! vector subsys_index; see entityinside/.../ItemEntityManager.java,
//! the ITEM-SUBSYS2 block in RegionTickOps.java and src/items_index.rs).
//!
//! ARCHITECTURE: эволюция round-1 items_manager. Items остаются в общих
//! bucket-массивах и тикаются ИНЛАЙН в RegionTickOps.tickBucket (один проход,
//! ванильный порядок снапшота); merge-кандидаты — из СОБСТВЕННОГО 1.0-grid
//! пространственного индекса на rust-стороне (плоские массивы, открытая
//! адресация, интрузивные цепочки — src/items_index.rs), что устраняет
//! item-driven broadphase (getEntitiesOfClass) целиком. Нативы idxProbe/
//! idxInsert/idxSetCell/idxRemove/idxQuery регистрируются на определённом
//! здесь классе ItemEntityManager через RegisterNatives.
//!
//! DELIVERY: define-only мост (паттерн batch_collector S7-160) + RegisterNatives;
//! армирование — RegionTickOps.itemsManagerArmed() (lazy с ретраем, fail-closed)
//! при CRUSSTY_LEVER_FLAG=items_subsys2 и region_threads>=2 (статический режим).
//! Никаких новых retarget'ов kernel-классов: точки входа — уже ретаргеченные
//! RegionTickOps.forEach/onTickingStart/onTickingEnd, правки только внутри
//! bridge-классов.
//!
//! Fail-closed: define/registration failure -> armed()=false -> ванильный
//! путь; пустой/чужой CRUSSTY_LEVER_FLAG -> ванильный путь по построению.

use std::ffi::c_void;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};

const IM_CLASS: &str = "net/minecraft/world/entity/ItemEntityManager";

const IM_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/ItemEntityManager.class");

fn lever_tokens() -> Vec<String> {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            v.trim()
                .split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// TASK-399-E: база J (items_subsys2) армится и своим флагом, и любым
/// cmp399_* (композитная композиция J+rustpre): f.eq("items_subsys2") →
/// f.eq("items_subsys2") || f.starts_with("cmp399_").
fn lever_flag_matches() -> bool {
    lever_tokens()
        .iter()
        .any(|f| f == "items_subsys2" || f.starts_with("cmp399_"))
}

/// Свой вектор E: точный AABB pre-filter в rust (перенос narrow-phase
/// candidates-теста из java в idx_query, см. src/items_index.rs).
fn rustpre_enabled() -> bool {
    lever_tokens().iter().any(|f| f == "cmp399_rustpre")
}

pub fn activate() {
    let rustpre = rustpre_enabled();
    // Флаг включает/выключает ТОЛЬКО rust-фильтр; сама J-подсистема (grid)
    // живёт при любом армирующем флаге. Включаем до spawn — queries ещё нет.
    crate::items_index::set_rustpre(rustpre);
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] items_subsys2: dormant (set CRUSSTY_LEVER_FLAG=items_subsys2 to enable)"
        );
        return;
    }
    if rustpre {
        eprintln!(
            "[crussty-plugin] cmp399_rustpre: enabling rust-side AABB pre-filter (idx_query narrow-phase, per-id aabb snapshots)"
        );
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] items_subsys2: requires CRUSSTY_REGION_THREADS>=2 (item-фаза встаёт в RegionTickOps.tickBucket), hook stays dormant"
        );
        return;
    }
    std::thread::spawn(move || {
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
            eprintln!("[crussty-plugin] items_subsys2: {IM_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/; hook stays dormant");
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
                eprintln!("[crussty-plugin] items_subsys2: define_class({IM_CLASS}) failed");
                return false;
            };

            // RegisterNatives: idxProbe/idxInsert/idxSetCell/idxRemove/idxQuery
            // (impl — src/items_index.rs) + TASK-399-E idxSetAabb. Провал
            // регистрации → armed()=false (probeOnce не пройдёт magic) →
            // ванильный путь.
            let names = [
                CString::new("idxProbe").expect("no NUL"),
                CString::new("idxInsert").expect("no NUL"),
                CString::new("idxSetCell").expect("no NUL"),
                CString::new("idxRemove").expect("no NUL"),
                CString::new("idxQuery").expect("no NUL"),
                CString::new("idxSetAabb").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                // TASK-399-E: insert carries the aabb snapshot (6×double).
                CString::new("(IIIIIDDDDDD)I").expect("no NUL"),
                CString::new("(IIIII)I").expect("no NUL"),
                CString::new("(I)I").expect("no NUL"),
                CString::new("(DDDDDDI[I)I").expect("no NUL"),
                CString::new("(IDDDDDD)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: crate::items_index::idx_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: crate::items_index::idx_insert as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: crate::items_index::idx_set_cell as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[3].as_ptr(),
                    signature: sigs[3].as_ptr(),
                    fnPtr: crate::items_index::idx_remove as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[4].as_ptr(),
                    signature: sigs[4].as_ptr(),
                    fnPtr: crate::items_index::idx_query as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[5].as_ptr(),
                    signature: sigs[5].as_ptr(),
                    fnPtr: crate::items_index::idx_set_aabb as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] items_subsys2: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            true
        });
        if defined.unwrap_or(false) {
            eprintln!("[crussty-plugin] items_subsys2: defined {IM_CLASS} in kernel loader + registered index natives");
            if rustpre {
                eprintln!("[crussty-plugin] cmp399_rustpre: ARMED rust-side AABB pre-filter (per-id aabb snapshots + idx_query narrow-phase, javap bit-match AABB.intersects)");
            }
        } else {
            eprintln!(
                "[crussty-plugin] items_subsys2: bridge definition failed, hook stays dormant"
            );
        }
    });
}
