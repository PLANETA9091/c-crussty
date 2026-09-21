//! Runtime wiring for the SENSEBATCH lever (TASK-399-D, mega-round-3, agent D
//! — vector cmp399_sensebatch; see
//! entityinside/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/
//! SenseBatchOps.java, the cmp399_sensebatch blocks in RegionTickOps.java and
//! src/classfile.rs::patch_entitylookup_sensebatch).
//!
//! ARCHITECTURE: batched sensing transport. The two EntityLookup getEntities
//! funnels (Entity-source and Class-source overloads — the ONLY transport
//! methods every mob targeting/sensing query resolves through, proven by
//! javap on patched-kernel.jar, RESEARCH-D.md §fanнель) are body-swapped to
//! delegate to SenseBatchOps. During a RegionTickOps bucket AI phase
//! (SenseBatchOps.beginBucket/endBucket around tickBucket/stealChunks), each
//! (ChunkEntitySlices, query-class) pair is scanned exactly ONCE per bucket
//! by vanilla ChunkEntitySlices.getEntities over the full section column;
//! per-sensor precise filters (source-skip / AABB.intersects / predicate)
//! stay per-query in vanilla semantics and vanilla order. NOT a broadphase
//! memo: nothing survives the bucket (generation counter invalidates slabs);
//! the reuse window is one tick-bucket on one ticking thread — a transport
//! collapse of duplicate section scans, not a cross-tick result cache.
//!
//! DELIVERY: SenseBatchOps (+ nested Key/Entry/Slab) is defined into the
//! kernel loader by region_threads' bridge list (FIRST, before RegionTickOps
//! — RegionTickOps's beginBucket/endBucket call sites resolve it lazily at
//! first bucket tick; defining it in the same synchronous list removes the
//! race). This module: registers the EntityLookup byte hook (flag-gated),
//! waits for EntityLookup + SenseBatchOps to be loaded, arms
//! SenseBatchOps.arm() via JNI, flips READY and retransforms EntityLookup
//! (load-hook + explicit retransform = belt and suspenders, brainhook
//! pattern).
//!
//! Fail-closed: flag absent -> hook unregistered, ARMED stays false,
//! beginBucket/endBucket no-op, EntityLookup bytes pristine. Define/patch
//! failure -> markers + dormant. Any SenseBatchOps method invoked while
//! !ARMED or outside a phase takes the byte-exact vanilla replication path.

use crate::classfile;
use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const LOOKUP_CLASS: &str = classfile::ENTITY_LOOKUP_CLASS;
pub const OPS_NAME: &str = classfile::SENSE_BATCH_OPS_CLASS;

const OPS_DIR: &str =
    "../entityinside/build/ca/spottedleaf/moonrise/patches/chunk_system/level/entity";

pub const OPS_BYTES: &[u8] = include_bytes!(concat!(
    "../entityinside/build/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/SenseBatchOps.class"
));
const KEY_BYTES: &[u8] =
    include_bytes!("../entityinside/build/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/SenseBatchOps$Key.class");
const ENTRY_BYTES: &[u8] =
    include_bytes!("../entityinside/build/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/SenseBatchOps$Entry.class");
const SLAB_BYTES: &[u8] =
    include_bytes!("../entityinside/build/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/SenseBatchOps$Slab.class");

static READY: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

fn lever_flag_matches() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim().eq("cmp399_sensebatch"))
        .unwrap_or(false)
}

/// Byte hook on the kernel EntityLookup: applies the two-funnel body swap
/// exactly once, only after the bridge is defined and armed.
pub fn register() {
    if !lever_flag_matches() {
        return;
    }
    cplug_sdk::hooks::register_bytes(LOOKUP_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            return None;
        }
        if PATCHED.swap(true, Ordering::SeqCst) {
            return None;
        }
        match classfile::patch_entitylookup_sensebatch(bytes) {
            Ok(b) => {
                eprintln!(
                    "[crussty-plugin] cmp399_sensebatch: patched {name} getEntities funnels ({} -> {} bytes)",
                    bytes.len(),
                    b.len()
                );
                Some(b)
            }
            Err(e) => {
                PATCHED.store(false, Ordering::SeqCst);
                eprintln!("[crussty-plugin] cmp399_sensebatch: patch failed: {e}");
                None
            }
        }
    });
}

/// Background activation: wait for kernel EntityLookup, ensure SenseBatchOps
/// is defined in the same loader (normally already done by region_threads'
/// bridge list; define here as fallback if region_threads is dormant), arm()
/// it, flip READY and retransform EntityLookup.
pub fn activate() {
    if !lever_flag_matches() {
        eprintln!(
            "[crussty-plugin] cmp399_sensebatch: dormant (set CRUSSTY_LEVER_FLAG=cmp399_sensebatch to enable)"
        );
        return;
    }
    std::thread::spawn(|| {
        // 1) Wait for the kernel EntityLookup (loads during level creation).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(LOOKUP_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] cmp399_sensebatch: {LOOKUP_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // 2) Ensure SenseBatchOps is defined in the kernel loader. region_threads
        //    defines it synchronously in its bridge list before RegionTickOps;
        //    if that lever is dormant, define it here (EntityLookup's loader).
        let defined_here = std::time::Instant::now();
        loop {
            if cplug_sdk::classes::find_class(OPS_NAME).is_some() {
                break;
            }
            if defined_here.elapsed() > std::time::Duration::from_secs(45) {
                // region_threads is not carrying the bridge (dormant) — define
                // ourselves. Nested classes FIRST (lazy resolution through
                // SenseBatchOps's defining loader would otherwise hit the
                // kernel classpath and NoClassDefFoundError on first use).
                let ok = cplug_sdk::jni_util::with_attached(|env| {
                    let Some(cls) = cplug_sdk::classes::find_class(LOOKUP_CLASS) else {
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
                    let mut ok = true;
                    for (nm, bytes) in [
                        (format!("{OPS_NAME}$Key"), KEY_BYTES),
                        (format!("{OPS_NAME}$Entry"), ENTRY_BYTES),
                        (format!("{OPS_NAME}$Slab"), SLAB_BYTES),
                        (OPS_NAME.to_string(), OPS_BYTES),
                    ] {
                        match env.define_class(&nm, loader, bytes) {
                            Some(c) => {
                                env.delete_local_ref(c);
                                eprintln!(
                                    "[crussty-plugin] cmp399_sensebatch: defined {nm} in kernel loader (fallback path)"
                                );
                            }
                            None => {
                                crate::clear_exception(env);
                                eprintln!(
                                    "[crussty-plugin] cmp399_sensebatch: define_class({nm}) failed"
                                );
                                ok = false;
                                break;
                            }
                        }
                    }
                    env.delete_local_ref(loader);
                    env.delete_local_ref(class_cls);
                    ok
                });
                if !ok.unwrap_or(false) {
                    eprintln!(
                        "[crussty-plugin] cmp399_sensebatch: bridge definition failed, hook stays dormant"
                    );
                    return;
                }
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1_000));
        }

        // 3) Arm: SenseBatchOps.arm() sets ARMED=true and prints the java-side
        //    marker. This MUST happen before READY (the swapped bodies resolve
        //    SenseBatchOps on first call — already guaranteed defined above).
        let armed = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(OPS_NAME) else {
                return false;
            };
            let Some(mid) = env.get_static_method_id(cls.as_jclass(), "arm", "()V") else {
                crate::clear_exception(env);
                return false;
            };
            env.call_static_void_method(cls.as_jclass(), mid, &[]);
            crate::clear_exception(env);
            true
        })
        .unwrap_or(false);
        if !armed {
            eprintln!("[crussty-plugin] cmp399_sensebatch: arm() failed, hook stays dormant");
            return;
        }

        // 4) READY -> retransform (covers both pre-READY pristine loads and
        //    late loads; the hook itself covers first loads after READY).
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(LOOKUP_CLASS);
        eprintln!(
            "[crussty-plugin] cmp399_sensebatch: hook armed, retransform rc={rc}"
        );
        std::thread::sleep(std::time::Duration::from_millis(250));
        if PATCHED.load(Ordering::SeqCst) {
            eprintln!(
                "[crussty-plugin] cmp399_sensebatch: ARMED (EntityLookup.getEntities Entity/Class funnels -> SenseBatchOps batched transport)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp399_sensebatch: NOT APPLIED after retransform (kernel build mismatch? — see patch-failed line above)"
            );
        }
    });
}
