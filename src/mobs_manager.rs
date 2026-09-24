//! Runtime wiring for the MOB-PUSH lever (TASK-400-J, vector mobpush —
//! lever cmp401_soa; bridge mobpush/net/minecraft/world/entity/MobPushOps.java,
//! SoA plane src/mobs_soa.rs, retarget in src/classfile.rs patch_push_entities_mob).
//!
//! ARCHITECTURE (TASK-401-E, vector soa): the mob push lane
//! (`LivingEntity.pushEntities` → `Level.getPushableEntities` → moonrise
//! whole-16³-section scan) is re-broadphased onto a STRUCTURE-OF-ARRAYS flat
//! plane: parallel fixed-capacity vectors x/y/z/hw/hh/flags indexed by dense
//! id + one open-addressed cellKey→chain table. The query reads the flat
//! arrays under ONE global seqlock version check (per-slot v1→data→v2 pairs
//! of the round-400-J grid are gone) and prunes candidates rust-side with a
//! coarse center±radii AABB test — java re-validates exactly (level +
//! AABB.intersects + vanilla EntitySelector.pushableBy). SEPARATE lever —
//! the item subsystem/other levers stay vanilla → clean A/B on the push
//! lane. Positions are pushed to rust per-tick by the bridge itself
//! (self-upsert at query time = end-of-move position, see MobPushOps
//! javadoc). The vanilla tail of pushEntities (cramming/numCollisions/doPush)
//! is untouched bytecode — upstream Paper «optimize entity pushes» port.
//!
//! DELIVERY (alloc_diet pattern): byte hook on LivingEntity captures pristine
//! bytes at first load; the activation worker defines MobPushOps into the
//! kernel loader + RegisterNatives (mobProbe/mobUpsert/mobRemove/mobQuery),
//! computes the length-preserving single-site retarget
//! getPushableEntities→MobPushOps.pushables, flips READY and retransforms
//! LivingEntity once.
//!
//! Fail-closed: flag != "cmp401_soa" → no hook installed at all (byte-
//! indistinguishable from the pre-TASK-400-J plugin); define/registration/
//! patch failure → READY stays false → vanilla; native error codes → the
//! Java bridge falls back per-call (ERR_RANGE) or disarms (ERR_STRUCT).

use std::ffi::c_void;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const LIVING_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const OPS_CLASS: &str = "net/minecraft/world/entity/MobPushOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../mobpush/build/net/minecraft/world/entity/MobPushOps.class");

/// Java-side gate baked into OPS_BYTES (<clinit>: ENABLED =
/// "cmp401_soa" || "cmp402_comp"). The exact-match lever keeps the
/// item-shard family (cmp399_shard) and the item subsystem off — the mobs
/// grid is a separate instance.
const GATE_LEVER: &str = "cmp401_soa";
/// TASK-402-B: the round-402 composite arms the SoA plane together with the
/// sharded mirror grid (src/mobs_grid.rs) — see java_gate_matches.
const GATE_LEVER_COMP: &str = "cmp402_comp";
/// TASK-402-F: stagcomp = композит + stagger (единый флаг раунда).
const GATE_LEVER_STAGCOMP: &str = "cmp402_stagcomp";
/// TASK-403-C: tickplane — whole-body retarget плейн раунда-403 (единый
/// флаг всех сегментов ТЕЛА тика; STRICT eq, пустой флаг = ваниль).
const GATE_LEVER_TICKPLANE: &str = "cmp403_tickplane";
/// TASK-405-F: композит stagcomp⊕tickplane — единый флаг раунда-405.
const GATE_LEVER_STAGTICK: &str = "cmp405_stagtick";
/// TASK-406-D: композит раунда-406 (stagtick ⊕ ai-window) — SoA-плоскость
/// primary push broadphase + популяция для aiEpoch (mob_ai_step window).
const GATE_LEVER_AIBATCH: &str = "cmp406_aibatch";
const GATE_LEVER_MULTI: &str = "cmp409_multi";

/// TASK-406-E: композит раунда-406 (stagtick ⊕ sscan) — SoA-плоскость
/// primary push broadphase + популяция для sscanEpoch (despawn-scan column).
const GATE_LEVER_SSCAN: &str = "cmp406_sscan";

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// TASK-402-B: the hook arms under the legacy soa flag AND the composite.
fn java_gate_matches(f: &str) -> bool {
    // TASK-410-C (eindexq): SoA-плоскость = источник популяции goal-query
    // CSR-снапшота (EntityQueryOps; sscan-прецедент TASK-406-E).
    // TASK-411-C (k4soa): K4 — радиус-ремонт (gate 2.0 / pad 2) + push-лейн
    // из chain-снапшота (MobPushOps.pushCandidates path).
    // TASK-411-C (eqsnap, v2): dirty-дельты (DeltaShard 16×8192, drain
    // O(dirty) один bulk JNI/тик).
    f == GATE_LEVER
        || f == GATE_LEVER_COMP
        || f == GATE_LEVER_STAGCOMP
        || f == GATE_LEVER_TICKPLANE
        || f == GATE_LEVER_STAGTICK
        || f == GATE_LEVER_AIBATCH
        || f == GATE_LEVER_MULTI || f == "cmp412_meganav" || f == "cmp414_cvs"
        // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
        || f == "cmp412_eqsnapv3" || f == "cmp414_cvs" || f == "cmp417_bq"
        // TASK-419-A (colpush): колпаш-носитель — SoA-плоскость + eqsnap
        // (столбцы кормит colpush_plane_refresh, per-entity upsert спит).
        || f == "cmp420_colpush"
        || f == "cmp421_brain" || f == "cmp422_brain2" || f == "cmp423_brain3" || f == "cmp424_mobfeed" || f == "cmp430_inside" || f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4" || f == "cmp444_chunk5"
        || f == GATE_LEVER_SSCAN
        || f == "cmp410_eindexq" || f == "cmp411_k4soa" || f == "cmp411_eqsnap"
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
        self.orig
            .lock()
            .map(|o| o.is_some())
            .unwrap_or(false)
    }
    fn take_orig(&self) -> Option<Vec<u8>> {
        self.orig
            .lock()
            .ok()
            .and_then(|mut o| o.take())
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

static TARGET: std::sync::OnceLock<Target> = std::sync::OnceLock::new();

fn target() -> &'static Target {
    TARGET.get_or_init(|| Target::new(LIVING_CLASS))
}

/// Register the byte hook (call once from cplugin_init). Dormant-invisible:
/// with the lever flag unset/mismatched NOTHING is registered — the plugin
/// stays byte-indistinguishable from vanilla for this vector.
pub fn register() {
    let f = lever_flag();
    if !java_gate_matches(&f) {
        eprintln!(
            "[crussty-plugin] mobs_soa: dormant (set CRUSSTY_LEVER_FLAG={GATE_LEVER} or {GATE_LEVER_COMP} to enable)"
        );
        return;
    }
    let t = target();
    cplug_sdk::hooks::register_bytes(t.name, move |_name, bytes| {
        let t = target();
        if !READY.load(Ordering::Acquire) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker; never rewrite here.
            eprintln!(
                "[crussty-plugin] mobs_soa: pristine sighting {} {} bytes (major {})",
                t.name,
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            t.stash_orig(bytes);
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = t.patch_bytes();
        if !t.served.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] mobs_soa: hook serve {} {} bytes",
                t.name,
                cached.as_ref().map(|c| c.len()).unwrap_or(0)
            );
        }
        cached.map(|c| c.to_vec())
    });
}

/// Background activation: wait for the kernel class + boot quiet, define the
/// MobPushOps bridge into the kernel loader, RegisterNatives, compute the
/// single-site retarget from the pristine bytes, flip READY, retransform.
pub fn activate() {
    let f = lever_flag();
    if !java_gate_matches(&f) {
        return;
    }
    std::thread::spawn(move || {
        let t = target();
        // LivingEntity loads at boot (entity superclass); wait it out.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(t.name).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] mobs_soa: {} not loaded within 180s, hook stays dormant",
                    t.name
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Kernel loader must be quiet before define/retransform (boot-time
        // class-loading storm discipline; fluid_guard TASK-80 lesson).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] mobs_soa: boot marker not seen, hook stays dormant");
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
        let ops_major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if ops_major > jvm_major {
            eprintln!(
                "[crussty-plugin] mobs_soa: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild mobpush/ via scripts/build_mobpush_ops.sh; hook stays dormant"
            );
            return;
        }

        // Pristine bytes: if the class predates the hook (fast boot), capture
        // via no-op retransform (READY=false → stash-only), fluid_guard pattern.
        if !t.orig_is_some() {
            eprintln!(
                "[crussty-plugin] mobs_soa: {} predates hook, capturing via no-op retransform",
                t.name
            );
            for _ in 1..=3 {
                let _ = cplug_sdk::retransform_class(t.name);
                if t.orig_is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !t.orig_is_some() {
                eprintln!(
                    "[crussty-plugin] mobs_soa: no pristine bytes for {}, hook stays dormant",
                    t.name
                );
                return;
            }
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(t.name) else {
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
            let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
                crate::describe_exception(env);
                eprintln!("[crussty-plugin] mobs_soa: define_class({OPS_CLASS}) failed");
                return false;
            };

            // RegisterNatives: mobProbe/mobUpsert/mobRemove/mobQuery
            // (impl — src/mobs_soa.rs) + under the TASK-402-B composite also
            // mobGridProbe/mobGridQuery (impl — src/mobs_grid.rs, the
            // sharded mirror fallback read plane). Провал регистрации
            // → armed()=false (probeOnce не пройдёт magic) → ванильный путь.
            let names = [
                CString::new("mobProbe").expect("no NUL"),
                CString::new("mobUpsert").expect("no NUL"),
                CString::new("mobRemove").expect("no NUL"),
                CString::new("mobQuery").expect("no NUL"),
                CString::new("mobGridProbe").expect("no NUL"),
                CString::new("mobGridQuery").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(IIDDDDD)I").expect("no NUL"),
                CString::new("(I)I").expect("no NUL"),
                CString::new("(DDDDDDI[I)I").expect("no NUL"),
                CString::new("()I").expect("no NUL"),
                CString::new("(DDDDDDI[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: crate::mobs_soa::mob_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: crate::mobs_soa::mob_upsert as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: crate::mobs_soa::mob_remove as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[3].as_ptr(),
                    signature: sigs[3].as_ptr(),
                    fnPtr: crate::mobs_soa::mob_query as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[4].as_ptr(),
                    signature: sigs[4].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_grid_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[5].as_ptr(),
                    signature: sigs[5].as_ptr(),
                    fnPtr: crate::mobs_grid::mob_grid_query as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] mobs_soa: register_natives failed (code {code}) — hook stays dormant"
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
        if !defined.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] mobs_soa: bridge definition failed, hook stays dormant"
            );
            return;
        }

        // Compute the single-site retarget from the pristine bytes (pure
        // rust, length-preserving). Any Err = kernel shape mismatch → fail
        // closed (lever stays vanilla).
        let Some(original) = t.take_orig() else {
            eprintln!("[crussty-plugin] mobs_soa: pristine bytes vanished, hook stays dormant");
            return;
        };
        let major = crate::improved_noise::class_version(&original)
            .map(|(m, _)| m)
            .unwrap_or(0);
        let (patched, outcome) = match crate::classfile::patch_push_entities_mob(&original) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] mobs_soa: pushEntities retarget rejected ({e}), hook stays dormant"
                );
                return;
            }
        };
        let retargeted = matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { .. }
        );
        if !retargeted {
            eprintln!(
                "[crussty-plugin] mobs_soa: unexpected patch outcome for {} ({outcome:?}), hook stays dormant",
                t.name
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] mobs_soa: computed patch for {} ({} -> {} bytes, {outcome:?})",
            t.name,
            original.len(),
            patched.len()
        );
        t.set_patch(PatchCache {
            bytes: Arc::from(patched),
            major,
        });

        // TASK-413-C (NCDFE root-cause cv3-1 35712182885 / cv3-2 35712204518):
        // under the k4soa/eqsnap flags the MobPushOps blob executes
        // `EntityGoalQueryOps.pushCandidates` from the FIRST pushables call
        // (population inject) — the bridge must be defined + natives
        // registered in THIS loader BEFORE LivingEntity goes live with the
        // retarget. A missed define is unrecoverable at runtime (HotSpot
        // caches the NCDFE per constant-pool entry — cv3-1: ×3938 AFTER the
        // late define). HARD publish gate (probe-then-patch: define →
        // register natives → probe → publish): bridge not defined → push
        // lane stays vanilla (fail-closed, no NCDFE). Under other flags the
        // guarded branch is never executed (lazy resolution) → no gate.
        if crate::entity_query::lever_matches() {
            let mut bridge_ok = crate::entity_query::ensure_bridge_early();
            for _ in 0..4 {
                if bridge_ok {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1_000));
                bridge_ok = crate::entity_query::ensure_bridge_early();
            }
            if !bridge_ok {
                eprintln!(
                    "[crussty-plugin] mobs_soa: EntityGoalQueryOps bridge not defined before publish — push lane stays vanilla (fail-closed, NCDFE guard)"
                );
                return;
            }
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        // TASK-402-B: под композитом маркер объявляет ВСЕ суб-механизмы
        // (soa + зеркальный sharded grid; item-половина — в items_manager).
        if f == GATE_LEVER_COMP || f == GATE_LEVER_STAGCOMP || f == GATE_LEVER_TICKPLANE || f == GATE_LEVER_STAGTICK || f == GATE_LEVER_AIBATCH || f == GATE_LEVER_SSCAN || f == GATE_LEVER_MULTI || f == "cmp412_meganav" || f == "cmp414_cvs" || f == "cmp417_bq" || f == "cmp421_brain" || f == "cmp422_brain2" || f == "cmp423_brain3" || f == "cmp424_mobfeed" || f == "cmp430_inside" || f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4" || f == "cmp444_chunk5" {
            eprintln!(
                "[crussty-plugin] {}: ARMED soa=flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=1.0 radius_gate=1.0 rust_prune=coarse-hw-hh + mobgrid=sharded-mirror shards=64 shard_cap=16384 fallback-read=per-call (rust mobs_soa SoA flat x/y/z/hw/hh/flags ⊕ mobs_grid mirror; pushEntities tail untouched vanilla; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)",
                f
            );
        } else if f == "cmp410_eindexq" {
            eprintln!(
                "[crussty-plugin] cmp410_eindexq: ARMED soa-population (flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=1.0 radius_gate=1.0 rust_prune=coarse-hw-hh; rust mobs_soa SoA flat x/y/z/hw/hh/flags = ПОПУЛЯЦИЯ goal-query снапшота EntityGoalQueryOps.eqEpoch; pushEntities tail untouched vanilla; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        } else if f == "cmp411_k4soa" {
            eprintln!(
                "[crussty-plugin] cmp411_k4soa: ARMED soa-population+push-snapshot (flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=2 radius_gate=2.0 rust_prune=coarse-hw-hh; K4 РЕМОНТ: gate 1.0->2.0 покрывает camel 1.1875/iron_golem 1.35/warden 1.45 — oversized-disarm хроники round-406d..410ck3l устранена; pushEntities -> MobPushOps.pushables = сначала chain-снапшот pushCandidates (0 per-query JNI, cell-rect dedup), fallback mobQuery, хвост ванильный; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        } else if f == "cmp411_eqsnap" {
            eprintln!(
                "[crussty-plugin] cmp411_eqsnap: ARMED soa-population+push-snapshot (flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=2 radius_gate=2.0 rust_prune=coarse-hw-hh; dirty-дельты: mob_upsert = append (id,alive,x,y,z,hw,hh) в пер-потоковый DeltaShard (16×8192, 0 локов/seqlock/хэша), eq_epoch СНАЧАЛА drain_eqsnap_shards O(dirty) один WLOCK, ПОТОМ full chain-build; pushEntities -> MobPushOps.pushables лестница eqsnap = снапшот → vanillaFill (cell-цепи плоскости невалидны, легаси mobQuery пропущен); per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        } else if f == "cmp412_eqsnapv3" || f == "cmp414_cvs" || f == "cmp417_bq" || f == "cmp421_brain" || f == "cmp422_brain2" || f == "cmp423_brain3" || f == "cmp424_mobfeed" || f == "cmp430_inside" || f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4" || f == "cmp444_chunk5" {
            eprintln!(
                "[crussty-plugin] cmp412_eqsnapv3: ARMED meganav⊕eqsnap soa-population+push-snapshot (flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=2 radius_gate=2.0 rust_prune=coarse-hw-hh; STRICT OR: плоскости cmp412_meganav (multi⊕navplane+navpool, tickplane, ai-window, sscan, items, stagger, collide-batch) || eqsnap-плоскость; dirty-дельты: mob_upsert = append (id,alive,x,y,z,hw,hh) в пер-потоковый DeltaShard (16×8192, 0 локов/seqlock/хэша), eq_epoch СНАЧАЛА drain_eqsnap_shards O(dirty) один WLOCK, ПОТОМ full chain-build; pushEntities -> MobPushOps.pushables лестница eqsnap = снапшот → vanillaFill (cell-цепи плоскости невалидны, легаси mobQuery/grid пропущены; ai/sscan read-views = состояние ПОСЛЕ drain, ≤1-тик ghost); per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        } else if f == "cmp420_colpush" {
            eprintln!(
                "[crussty-plugin] cmp420_colpush: ARMED soa-population (flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=2 radius_gate=2.0; STRICT OR носитель cmp417_bq-эры + colpush: pushEntities whole-body redirect -> ColpushOps (per-entity mobUpsert спит), плоские колонки кормит colpush_plane_refresh одним WLOCK/тик, eq_epoch chain-build жив; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] cmp401_soa: ARMED soa=flat-arrays seqlock=global-version writer=global-mutex ids_cap=1048576 cell_cap=262144 cell=1.0 pad=1.0 radius_gate=1.0 rust_prune=coarse-hw-hh (rust mobs_soa SoA flat x/y/z/hw/hh/flags; pushEntities tail untouched vanilla; per-call vanilla fallback ERR_RANGE, disarm ERR_STRUCT)"
            );
        }

        // Single retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(OPS_CLASS, "pushables", "mobs_soa v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(t.name);
        eprintln!("[crussty-plugin] mobs_soa: {} armed, retransform rc={rc}", t.name);
        if rc == 0 {
            // TASK-406-D: публикуем сигнал ДЛЯ mobs_ai (составление поверх).
            crate::mobs_ai::note_soa_served();
        }
    });
}
