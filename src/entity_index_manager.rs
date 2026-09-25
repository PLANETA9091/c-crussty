//! Runtime wiring for the ENTITY-INDEX lever (TASK-405-C, vector eindex —
//! levers cmp405_eindex | cmp458_roar (TASK-458-K: ID-H04 roaring section
//! occupancy + ID-H06 bloom, same bridge, same mirror); bridge
//! entityquery/net/minecraft/world/entity/EntityIndexOps.java, shard mirror
//! src/entity_index.rs, retargets src/classfile.rs patch_eindex_*).
//!
//! DELIVERY (mobs_manager pattern): byte hooks on the 6 target classes
//! (EntityLookup + Entity + the 4 rare setBoundingBox funnel owners) capture
//! pristine bytes at first load; the activation worker defines EntityIndexOps
//! into the kernel loader, RegisterNatives (eidxProbe/eidxFlush/eidxFlushQuery),
//! probes the magic, seeds the mirror from getAllCopy() snapshots, flips java
//! ARMED, computes the EntityLookup patch (4 query-body redirects + 4
//! note-site retargets) and the Entity.setPosRaw(DDDZ) bb-note retarget,
//! flips READY and retransforms the core classes once.
//!
//! The 4 RARE bb owners (Shulker/HangingEntity/LeashFenceKnotEntity/
//! Interaction) load lazily — their single-site retarget is computed at
//! first class load inside the hook (after READY); a shape mismatch there
//! fails CLOSED: java broken=true → exact vanilla replication forever (an
//! un-noted bb site could under-count → bit-for-bit violation risk).
//!
//! Fail-closed: flag not in {cmp405_eindex, cmp458_roar} → nothing registered
//! (byte-indistinguishable from vanilla); define/registration/probe/seed/patch
//! failure → READY stays false → vanilla; native ERR_STRUCT → the bridge
//! disarms permanently (vanillaReplica); ERR_RANGE → per-call fallback.

use std::ffi::c_void;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const LOOKUP_CLASS: &str =
    "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const RARE_CLASSES: [&str; 4] = [
    "net/minecraft/world/entity/monster/Shulker",
    "net/minecraft/world/entity/decoration/HangingEntity",
    "net/minecraft/world/entity/decoration/LeashFenceKnotEntity",
    "net/minecraft/world/entity/Interaction",
];
const RARE_METHODS: [(&str, &str); 4] = [
    (
        "onSyncedDataUpdated",
        "(Lnet/minecraft/network/syncher/EntityDataAccessor;)V",
    ),
    ("recalculateBoundingBox", "()V"),
    ("recalculateBoundingBox", "()V"),
    (
        "readAdditionalSaveData",
        "(Lnet/minecraft/world/level/storage/ValueInput;)V",
    ),
];
const RARE_NOTE_DESCS: [&str; 4] = [
    "(Lnet/minecraft/world/entity/monster/Shulker;Lnet/minecraft/world/phys/AABB;)V",
    "(Lnet/minecraft/world/entity/decoration/HangingEntity;Lnet/minecraft/world/phys/AABB;)V",
    "(Lnet/minecraft/world/entity/decoration/LeashFenceKnotEntity;Lnet/minecraft/world/phys/AABB;)V",
    "(Lnet/minecraft/world/entity/Interaction;Lnet/minecraft/world/phys/AABB;)V",
];
const ENTITY_BB_NOTE_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V";

const OPS_CLASS: &str = "net/minecraft/world/entity/EntityIndexOps";
const OPS_BYTES: &[u8] =
    include_bytes!("../entityquery/build/net/minecraft/world/entity/EntityIndexOps.class");
/// Inner buffer POJO — MUST be defined into the kernel loader BEFORE the
/// outer class links. cleg1 (run 35659765756) evidence: define_class +
/// RegisterNatives succeed, then GetStaticMethodID("eidxProbe") triggers
/// LINKING, linking must resolve EntityIndexOps$Buf (REGISTRY field type /
/// T_BUF generic), the kernel loader has no such class → NoClassDefFoundError
/// → probe resolve failed → hook dormant. The other bridges (MobPushOps etc.)
/// are single-class blobs, which is why this failure mode was new.
const OPS_BUF_CLASS: &str = "net/minecraft/world/entity/EntityIndexOps$Buf";
const OPS_BUF_BYTES: &[u8] =
    include_bytes!("../entityquery/build/net/minecraft/world/entity/EntityIndexOps$Buf.class");

const GATE_LEVER: &str = "cmp405_eindex";
/// TASK-458-K: the roaring/bloom carrier rides the SAME eindex subsystem
/// (law 6: one subsystem, one bulk-JNI/тик) — both flags arm it.
const GATE_LEVER2: &str = "cmp458_roar";
/// TASK-461-66: swarx-1 super-carrier union-widen (STRICT-OR, law 4). Audit
/// fact: swar (push-plane: mobs_soa SoA + eqEpoch + EntityGoalQueryOps/MobPushOps)
/// and roar (per-sec occupancy chains + bloom pre-gate: EntityIndexOps/EntityLookup)
/// have ZERO code-file overlap (git diff 96cc2704..33939931 vs ..eb47e869) —
/// orthogonal broadphase sub-lanes => one flag arms BOTH.
const GATE_LEVER3: &str = "cmp458_swar";

fn lever_matches(f: &str) -> bool {
    f == GATE_LEVER || f == GATE_LEVER2 || f == GATE_LEVER3
}

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

static READY: AtomicBool = AtomicBool::new(false);

struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}

/// Per-class state: pristine capture + computed patch + serve log flag.
struct Target {
    name: &'static str,
    orig: std::sync::Mutex<Option<Vec<u8>>>,
    patch: std::sync::Mutex<Option<PatchCache>>,
    served: AtomicBool,
}

impl Target {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            orig: std::sync::Mutex::new(None),
            patch: std::sync::Mutex::new(None),
            served: AtomicBool::new(false),
        }
    }
    fn stash_orig(&self, bytes: &[u8]) {
        if let Ok(mut orig) = self.orig.lock() {
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
        }
    }
    fn orig_is_some(&self) -> bool {
        self.orig.lock().map(|o| o.is_some()).unwrap_or(false)
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig.lock().ok().and_then(|mut o| o.take())
    }
    fn set_patch(&self, cache: PatchCache) {
        if let Ok(mut p) = self.patch.lock() {
            *p = Some(cache);
        }
    }
    fn patch_bytes(&self) -> Option<Arc<[u8]>> {
        self.patch
            .lock()
            .ok()
            .and_then(|p| p.as_ref().map(|c| Arc::clone(&c.bytes)))
    }
}

static LOOKUP: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static ENTITY: std::sync::OnceLock<Target> = std::sync::OnceLock::new();
static RARE: std::sync::OnceLock<Vec<Target>> = std::sync::OnceLock::new();

fn lookup() -> &'static Target {
    LOOKUP.get_or_init(|| Target::new(LOOKUP_CLASS))
}
fn entity() -> &'static Target {
    ENTITY.get_or_init(|| Target::new(ENTITY_CLASS))
}
fn rare() -> &'static [Target] {
    RARE.get_or_init(|| {
        RARE_CLASSES
            .iter()
            .map(|n| Target::new(Box::leak(n.to_string().into_boxed_str())))
            .collect()
    })
}

/// Global ref of the defined bridge class (for fail-closed disarms from any
/// thread; set once at define time).
static BRIDGE_GREF: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

/// Call a no-arg static void method on the bridge (wrapper API).
fn java_call_bridge_void(env: &jvmti_bindings::env::JniEnv, cls: jvmti_bindings::jni::jclass,
                         name: &str) -> bool {
    let Some(mid) = env.get_static_method_id(cls, name, "()V") else {
        env.exception_clear();
        return false;
    };
    env.call_static_void_method(cls, mid, &[]);
    true
}

/// Flip java broken=true (fail-closed: an un-noted bb funnel site or any
/// structural defect must permanently disable the counts-skip plane).
fn fail_closed_disarm() {
    let Some(gref) = BRIDGE_GREF.get().copied() else {
        return; // bridge not defined yet — nothing to disarm (still vanilla)
    };
    let _ = cplug_sdk::jni_util::with_attached(|env| {
        java_call_bridge_void(env, gref as jvmti_bindings::jni::jclass, "breakNow");
    });
}

/// Register the byte hooks (call once from cplugin_init). Dormant-invisible:
/// with the lever flag unset/mismatched NOTHING is registered — the plugin
/// stays byte-indistinguishable from vanilla for this vector.
pub fn register() {
    let f = lever_flag();
    if !lever_matches(&f) {
        eprintln!(
            "[crussty-plugin] eindex: dormant (set CRUSSTY_LEVER_FLAG={GATE_LEVER}|{GATE_LEVER2} to enable)"
        );
        return;
    }
    // Core targets: pristine sighting until READY, then serve the cache.
    for (i, t) in [lookup(), entity()].into_iter().enumerate() {
        cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
            let t: &'static Target = if i == 0 { lookup() } else { entity() };
            if !READY.load(Ordering::Acquire) {
                t.stash_orig(bytes);
                return None;
            }
            let cached = t.patch_bytes();
            if !t.served.swap(true, Ordering::Relaxed) {
                eprintln!(
                    "[crussty-plugin] eindex: hook serve {} {} bytes",
                    t.name,
                    cached.as_ref().map(|c| c.len()).unwrap_or(0)
                );
            }
            cached.map(|c| c.to_vec())
        });
    }
    // Rare bb owners: compute the retarget at first post-READY load; shape
    // mismatch fails CLOSED (java broken=true → permanent vanilla replica).
    for idx in 0..4 {
        cplug_sdk::hooks::register_bytes(RARE_CLASSES[idx], move |_name, bytes| {
            let t: &'static Target = &rare()[idx];
            t.stash_orig(bytes);
            if t.patch_bytes().is_some() {
                if !t.served.swap(true, Ordering::Relaxed) {
                    eprintln!("[crussty-plugin] eindex: rare hook serve {}", t.name);
                }
                return t.patch_bytes().map(|c| c.to_vec());
            }
            if !READY.load(Ordering::Acquire) {
                return None; // patched later via retransform of loaded classes
            }
            let (name, desc) = RARE_METHODS[idx];
            match crate::classfile::patch_eindex_bb_site(
                bytes,
                name,
                desc,
                t.name,
                RARE_NOTE_DESCS[idx],
            ) {
                Ok((patched, crate::classfile::RetargetOutcome::Retargeted { sites })) => {
                    eprintln!(
                        "[crussty-plugin] eindex: rare bb note {} retargeted ({sites} site)",
                        t.name
                    );
                    let major = crate::improved_noise::class_version(bytes)
                        .map(|(m, _)| m)
                        .unwrap_or(0);
                    t.set_patch(PatchCache { bytes: Arc::from(patched), major });
                    t.served.store(true, Ordering::Relaxed);
                    t.patch_bytes().map(|c| c.to_vec())
                }
                Ok((_, other)) => {
                    eprintln!(
                        "[crussty-plugin] eindex: rare bb note {} unexpected outcome {other:?} — fail-closed disarm",
                        t.name
                    );
                    fail_closed_disarm();
                    None
                }
                Err(e) => {
                    eprintln!(
                        "[crussty-plugin] eindex: rare bb note {} rejected ({e}) — fail-closed disarm",
                        t.name
                    );
                    fail_closed_disarm();
                    None
                }
            }
        });
    }
}

/// Background activation (mobs_manager pattern).
pub fn activate() {
    let f = lever_flag();
    if !lever_matches(&f) {
        return;
    }
    std::thread::spawn(move || {
        let t = lookup();
        // EntityLookup loads with the first level (boot); wait it out.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(t.name).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] eindex: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform (fluid_guard
        // TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] eindex: boot marker not seen, hook stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Guard: embedded bridge bytes must not be newer than the JVM.
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let ops_major =
            crate::improved_noise::class_version(OPS_BYTES).map(|(m, _)| m).unwrap_or(0);
        let buf_major =
            crate::improved_noise::class_version(OPS_BUF_BYTES).map(|(m, _)| m).unwrap_or(0);
        if ops_major.max(buf_major) > jvm_major {
            eprintln!(
                "[crussty-plugin] eindex: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild entityquery/ via scripts/build_entity_index_ops.sh; hook stays dormant"
            );
            return;
        }

        // Pristine bytes for the two core classes (no-op retransform trick
        // for anything that predated the hook).
        for core in [lookup(), entity()] {
            if !core.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] eindex: {} predates hook, capturing via no-op retransform",
                    core.name
                );
                for _ in 1..=3 {
                    let _ = cplug_sdk::retransform_class(core.name);
                    if core.orig_is_some() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                if !core.orig_is_some() {
                    eprintln!(
                        "[crussty-plugin] eindex: no pristine bytes for {}, hook stays dormant",
                        core.name
                    );
                    return;
                }
            }
        }

        // Define the bridge + RegisterNatives.
        let gcls = cplug_sdk::jni_util::with_attached(|env| {
            let Some(lcls) = cplug_sdk::classes::find_class(lookup().name) else {
                return None;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return None;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(lcls.as_jclass(), mid, &[]);
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
            // Inner Buf first: the outer class must not link (probe resolve)
            // before the loader can resolve its inner-class references.
            let Some(buf_c) = env.define_class(OPS_BUF_CLASS, gref, OPS_BUF_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] eindex: define_class({OPS_BUF_CLASS}) failed");
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None;
            };
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] eindex: define_class({OPS_CLASS}) failed");
                env.delete_local_ref(buf_c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None;
            };
            env.delete_local_ref(buf_c);
            let gcls = env.new_global_ref(c);
            if gcls.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None;
            }

            // RegisterNatives: eidxProbe/eidxFlush/eidxFlushQuery
            // (impl — src/entity_index.rs).
            let names = [
                CString::new("eidxProbe").expect("no NUL"),
                CString::new("eidxFlush").expect("no NUL"),
                CString::new("eidxFlushQuery").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(I[I[B[D[I)I").expect("no NUL"),
                CString::new("(I[I[B[D[IDDDDDDIIIIII[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: crate::entity_index::eidx_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: crate::entity_index::eidx_flush as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: crate::entity_index::eidx_flush_query as *const c_void as *mut c_void,
                },
            ];
            if let Err(code) = env.register_natives(c, &natives) {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] eindex: register_natives failed (code {code}) — hook stays dormant"
                );
                env.delete_local_ref(c);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return None;
            }
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            let _ = BRIDGE_GREF.set(gcls as usize);
            Some(gcls)
        });
        let Some(Some(gcls)) = gcls else {
            eprintln!("[crussty-plugin] eindex: bridge definition failed, hook stays dormant");
            return;
        };

        let ok = cplug_sdk::jni_util::with_attached(|env| {
            // Probe sanity (magic "EIDX").
            let Some(mid) = env.get_static_method_id(gcls, "eidxProbe", "()I") else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] eindex: eidxProbe resolve failed");
                return false;
            };
            let magic = env.call_static_int_method(gcls, mid, &[]);
            if magic != crate::entity_index::PROBE_MAGIC {
                eprintln!(
                    "[crussty-plugin] eindex: probe magic {magic:#x} != expected — hook stays dormant"
                );
                return false;
            }
            // Seed the mirror from atomic getAllCopy() snapshots (idempotent
            // by id; buffered notes racing the seed converge on vanilla).
            let Some(mid) = env.get_static_method_id(gcls, "seedAll", "()I") else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] eindex: seedAll resolve failed");
                return false;
            };
            let seeded = env.call_static_int_method(gcls, mid, &[]);
            if seeded != 1 {
                eprintln!("[crussty-plugin] eindex: seed failed ({seeded}) — hook stays dormant");
                return false;
            }
            // ARMED flips only after a good seed; queries cannot reach the
            // bridge before the EntityLookup retransform below anyway.
            java_call_bridge_void(env, gcls, "armNow");
            true
        });
        if !ok.unwrap_or(false) {
            eprintln!("[crussty-plugin] eindex: probe/seed/armed failed, hook stays dormant");
            return;
        }

        // EntityLookup patch: 4 query-body redirects + 4 note-site retargets,
        // all-or-nothing (fail closed → lever stays vanilla).
        let Some(original) = t.take_orig() else {
            eprintln!("[crussty-plugin] eindex: pristine bytes vanished, hook stays dormant");
            return;
        };
        let major =
            crate::improved_noise::class_version(&original).map(|(m, _)| m).unwrap_or(0);
        let lookup_patch = (|| -> Result<Vec<u8>, String> {
            let (p1, _) = crate::classfile::patch_eindex_lookup_redirects(&original)?;
            let (p2, _) = crate::classfile::patch_eindex_lookup_notes(&p1)?;
            Ok(p2)
        })();
        let patched = match lookup_patch {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] eindex: EntityLookup patch rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        eprintln!(
            "[crussty-plugin] eindex: computed EntityLookup patch ({} -> {} bytes)",
            original.len(),
            patched.len()
        );
        t.set_patch(PatchCache { bytes: Arc::from(patched), major });

        // Entity.setPosRaw(DDDZ) bb-note retarget (the walking funnel).
        let et = entity();
        let Some(eorig) = et.take_orig() else {
            eprintln!("[crussty-plugin] eindex: no pristine Entity, hook stays dormant");
            return;
        };
        let emajor =
            crate::improved_noise::class_version(&eorig).map(|(m, _)| m).unwrap_or(0);
        match crate::classfile::patch_eindex_bb_site(
            &eorig,
            "setPosRaw",
            "(DDDZ)V",
            ENTITY_CLASS,
            ENTITY_BB_NOTE_DESC,
        ) {
            Ok((epatched, crate::classfile::RetargetOutcome::Retargeted { sites })) => {
                eprintln!(
                    "[crussty-plugin] eindex: Entity.setPosRaw bb note retargeted ({sites} site)"
                );
                et.set_patch(PatchCache { bytes: Arc::from(epatched), major: emajor });
            }
            Ok((_, other)) => {
                eprintln!(
                    "[crussty-plugin] eindex: Entity.setPosRaw unexpected outcome {other:?} — hook stays dormant"
                );
                return;
            }
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] eindex: Entity.setPosRaw rejected ({e}) — hook stays dormant"
                );
                return;
            }
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        crate::kernel_policy::audit_wire(OPS_CLASS, "eidx", "eindex v2 roar");
        READY.store(true, Ordering::Release);
        let rc1 = cplug_sdk::retransform_class(lookup().name);
        let rc2 = cplug_sdk::retransform_class(entity().name);
        // Already-loaded rare owners: the no-op retransform re-fires the hook
        // with pristine bytes → computed + served there (post-READY).
        for r in rare() {
            if cplug_sdk::classes::find_class(r.name).is_some() {
                let _ = cplug_sdk::retransform_class(r.name);
            }
        }
        eprintln!(
            "[crussty-plugin] {f}: ARMED shards=64 shard_cells=4096 shard_slots=8192 shard_ids=8192 seqlock=per-shard writer=global-mutex sync=threadlocal-buffers+cross-drain jni=fused-flushquery query=counts-skip-rect rect_cap=64x64 sections=64-window+overflow secbits=monotone-u64 bloom=4KB-k4-insertonly-fp<2%-<=3k-chunks selftest=chain_len-vs-walk+sec-consistency+bloomFN@every100q fallback=vanillaReplica (rust entity_index chunk-mirror v2 roar: EntityLookup 4 redirects + 4 note sites + Entity/4rare bb sites; vanilla tail untouched; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT/selftest-fail) retransform rc={rc1}/{rc2}"
        );
    });
}
