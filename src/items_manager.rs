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

/// TASK-399-B (cmp399_shard): the Java-side gate string baked into IM_BYTES
/// (<clinit>: ENABLED = "items_subsys2".equals(trimToEmpty(getenv))). With a
/// cmp399_* lever the gate is CP-patched at define time (classfile::patch_utf8_gate,
/// bytecode-transparent) so the same bridge arms under the round-399 flag.
/// TASK-400-A (cmp399_bfcomp composite): the committed .class is REBUILT from
/// the merged source whose ENABLED/DESPAWN2 already cover cmp399_bfcomp, so
/// the composite flag needs no runtime patch — patching stays on the legacy
/// cmp399_shard path only (byte-identical A/B parity for that flag).
const GATE_LEGACY: &str = "items_subsys2";
const GATE_CMP: &str = "cmp399_shard";

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn lever_flag_matches() -> bool {
    // TASK-399-B: arm the J-subsystem for the legacy flag AND the round-399
    // cmp399_* lever family (cmp399_shard selects the sharded grid arm).
    // TASK-399-F композиция: суб-вектор despawnv2 дополнительно гейтится
    // java-стороной по точному флагу (см. ItemEntityManager.DESPAWN2).
    // TASK-400-A: составной cmp399_bfcomp входит в семейство cmp399_* и
    // включает ОБА суб-вектора (shard-grid + lifetime-heap).
    // TASK-401-H: композитный cmp401_comp — семейство cmp401_* тоже армит
    // (java ENABLED/DESPAWN2 уже покрывают cmp401_comp в этой ветке).
    lever_flag_matches_for(&lever_flag())
}

fn lever_flag_matches_for(f: &str) -> bool {
    f == "items_subsys2" || f.starts_with("cmp399_") || f.starts_with("cmp401_")
}

pub fn activate() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] items_subsys2: dormant (set CRUSSTY_LEVER_FLAG=items_subsys2 to enable)"
        );
        return;
    }
    if crate::region_threads::workers_from_env_pub().is_none() {
        eprintln!(
            "[crussty-plugin] items_subsys2: requires CRUSSTY_REGION_THREADS>=2 (item-фаза встаёт в RegionTickOps.tickBucket), hook stays dormant"
        );
        return;
    }
    let flag = lever_flag();
    let shard = flag == "cmp399_shard";
    let bfcomp = flag == "cmp399_bfcomp";
    // TASK-401-H: композитный флаг армит shard-grid (rust shard_mode),
    // lifetime-heap (DESPAWN2 в java) и маркерится как композит.
    let comp401 = flag == "cmp401_comp";
    let despawn2 = flag == "cmp399_despawn2" || bfcomp || comp401;
    if shard {
        // ГРОМКИЙ ARM-МАРКЕР (TASK-399-B): без этой строки нога не-armed.
        eprintln!(
            "[crussty-plugin] cmp399_shard: ARMED shards=64 seqlock-reads=per-cell-version writer=global-mutex shard_cap=16384 max_ids=1048576 (rust items_index sharded mode; legacy RwLock path intact for items_subsys2)"
        );
    }
    if bfcomp {
        // ГРОМКИЙ ARM-МАРКЕР КОМПОЗИТА (TASK-400-A, обязателен): оба
        // суб-вектора B+F одновременно — shard-grid (B) + lifetime-heap (F).
        eprintln!(
            "[crussty-plugin] cmp399_bfcomp: ARMED shards=64 seqlock-reads=per-cell-version writer=global-mutex shard_cap=16384 max_ids=1048576 heap=lifetime-minheap(rust,vec) push=batch(1/tick) due-poll=1/tick despawn-flow=vanilla (composite B+F)"
        );
    }
    if comp401 {
        // ГРОМКИЙ ARM-МАРКЕР КОМПОЗИТА TASK-401-H (обязателен): shard-grid
        // (B) + lifetime-heap (F) + devirt D1/D2 (G) + mobpush (J400).
        eprintln!(
            "[crussty-plugin] cmp401_comp: ARMED shards=64 seqlock-reads=per-cell-version writer=global-mutex shard_cap=16384 max_ids=1048576 heap=lifetime-minheap(rust,vec) push=batch(1/tick) due-poll=1/tick despawn-flow=vanilla mobpush=sharded-grid devirt=D1+D2 (composite TASK-401-H)"
        );
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

        // TASK-399-B: widen the JAVA arm gate for the exact cmp399_shard flag
        // by CP-patching the embedded bridge class at define time. Since the
        // round-400-A rebuild the committed .class is compiled from the merged
        // source (ENABLED covers the whole cmp399_* family, DESPAWN2 covers
        // cmp399_despawn2 || cmp399_bfcomp), so the composite flag self-arms;
        // the patch below stays for byte-parity of the legacy shard path
        // (works on both artifact generations: "items_subsys2" occurs once).
        // Legacy flag → original bytes (byte-identical arm path, A/B parity).
        // TASK-401-H: cmp401_comp self-arms via the recompiled committed class
        // (ENABLED covers cmp401_*), so NO CP-patch on the composite path.
        let im_bytes: Vec<u8> = if shard {
            match crate::classfile::patch_utf8_gate(IM_BYTES, GATE_LEGACY, GATE_CMP) {
                Ok(b) => {
                    eprintln!(
                        "[crussty-plugin] cmp399_shard: java gate CP-patched ({GATE_LEGACY} -> {GATE_CMP}, {} -> {} bytes)",
                        IM_BYTES.len(),
                        b.len()
                    );
                    b
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] cmp399_shard: java gate patch failed ({e}) — hook stays dormant"
                    );
                    return;
                }
            }
        } else {
            IM_BYTES.to_vec()
        };

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
            let Some(c) = env.define_class(IM_CLASS, gref, &im_bytes) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] items_subsys2: define_class({IM_CLASS}) failed");
                return false;
            };

            // RegisterNatives: idxProbe/idxInsert/idxSetCell/idxRemove/idxQuery
            // (impl — src/items_index.rs) + lifetimePush/lifetimeDue (impl —
            // src/items_lifetime.rs, TASK-399-F despawnv2). Провал регистрации
            // → armed()=false (probeOnce не пройдёт magic) → ванильный путь.
            let names = [
                CString::new("idxProbe").expect("no NUL"),
                CString::new("idxInsert").expect("no NUL"),
                CString::new("idxSetCell").expect("no NUL"),
                CString::new("idxRemove").expect("no NUL"),
                CString::new("idxQuery").expect("no NUL"),
                CString::new("lifetimePush").expect("no NUL"),
                CString::new("lifetimeDue").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(IIIII)I").expect("no NUL"),
                CString::new("(IIIII)I").expect("no NUL"),
                CString::new("(I)I").expect("no NUL"),
                CString::new("(DDDDDDI[I)I").expect("no NUL"),
                CString::new("([JI)I").expect("no NUL"),
                CString::new("(J[J)I").expect("no NUL"),
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
                    fnPtr: crate::items_lifetime::lifetime_push as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[6].as_ptr(),
                    signature: sigs[6].as_ptr(),
                    fnPtr: crate::items_lifetime::lifetime_due as *const c_void as *mut c_void,
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
            // TASK-399-F despawnv2 ARM-маркер (обязателен при флаге
            // cmp399_despawn2 и в композите cmp399_bfcomp): rust
            // lifetime-heap + батч-деспавн.
            if despawn2 {
                eprintln!(
                    "[crussty-plugin] {flag}: ARMED heap=lifetime-minheap(rust,vec) push=batch(1/tick) due-poll=1/tick despawn-flow=vanilla"
                );
            }
        } else {
            eprintln!(
                "[crussty-plugin] items_subsys2: bridge definition failed, hook stays dormant"
            );
        }
    });
}

#[cfg(test)]
mod tests {
    /// TASK-399-B: the runtime CP-patch of the embedded bridge class must be
    /// bytecode-transparent and exactly swap the arm-gate string both ways.
    #[test]
    fn gate_patch_swaps_exactly_one_utf8() {
        let patched = crate::classfile::patch_utf8_gate(
            super::IM_BYTES,
            super::GATE_LEGACY,
            super::GATE_CMP,
        )
        .expect("gate patch must succeed on the committed round-398-J artifact");
        assert_eq!(
            patched.len(),
            super::IM_BYTES.len() - super::GATE_LEGACY.len() + super::GATE_CMP.len()
        );
        // Reverse patch restores the original shape (no pool drift).
        let back = crate::classfile::patch_utf8_gate(
            &patched,
            super::GATE_CMP,
            super::GATE_LEGACY,
        )
        .expect("reverse patch");
        assert_eq!(back.len(), super::IM_BYTES.len());
        // Legacy flag path must stay byte-identical (A/B parity).
        assert_eq!(super::lever_flag_matches_for("items_subsys2"), true);
        assert_eq!(super::lever_flag_matches_for("cmp399_shard"), true);
        // TASK-400-A: composite flag joins the cmp399_* family gate.
        assert_eq!(super::lever_flag_matches_for("cmp399_bfcomp"), true);
        // TASK-401-H: the cmp401_* composite family joins too.
        assert_eq!(super::lever_flag_matches_for("cmp401_comp"), true);
        assert_eq!(super::lever_flag_matches_for(""), false);
        assert_eq!(super::lever_flag_matches_for("cmp399_other"), true);
        assert_eq!(super::lever_flag_matches_for("items_oss"), false);
        assert_eq!(super::lever_flag_matches_for(""), false);
    }
}
