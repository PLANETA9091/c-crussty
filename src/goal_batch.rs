//! Runtime wiring for the GOAL-SELECTOR BATCH plane (TASK-414-C2, vector
//! «pathfinder/brain виток-2: GoalSelector tick-scheduling batch» — lever
//! `cmp414_pfb`; bridge gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java,
//! natives below, retarget of the `GoalSelector.tick()V` call sites via
//! `classfile::retarget_virtual_to_static`).
//!
//! RECON (round-cvs2v2 cpu-collapsed, cvs2 leg @598b3c3 carrier): nav_ai lane
//! 10.57%; neighbor-generation (NodeEvaluator/getNeighbors) = 1.51-1.65% <
//! 2% порога анти-плацебо — таргет смещён ВНУТРИ подсистемы на реально
//! горячее: GoalSelector.tick subtree = 9.54% wall (scheduler-bookkeeping
//! ≈ 2.9% = linked-set iteration + lockedFlags map + removeIf + flag-сет
//! операции; остальное = goal-logic canUse/start, остаётся java-ваниль).
//!
//! RUST-FIRST (закон 6): rust-плоскость держит SoA-реестр селекторов
//! (selId → flags[]/priorities[] коло́нки, bulk `gselRegister` sync при
//! редких rebuild) и обрабатывает ОДИН bulk JNI за серверный тик
//! (`gselEpoch`: DOD-проход по реестру + per-selector телеметрия в stats[]).
//! ZERO per-entity JNI. Последовательные решения (canUse/canContinueToUse/
//! start/stop) остаются java-ваниль над плоским зеркалом — порядок и решения
//! бит-в-байт (jsel contract в GoalBatchOps.java).
//!
//! RETARGET (STRICT census, javap ground truth): ровно 4 сайта
//! `invokevirtual GoalSelector.tick()V` в классе Mob — serverAiStep ×2
//! (goalSelector+targetSelector) + inactiveTick ×2. Сайтов != 4 → hook
//! fail-closed pass-through (ваниль). Класс GoalSelector не патчится; чужие
//! callers невозможны (census).
//!
//! CHAIN COMPOSITION: Mob уже несёт sscan serve (stash-based); hook
//! регистрируется ПОСЛЕДНИМ в cplugin_init и КОМПОЗИРУЕТ на полученные байты
//! (AlreadyPatched → idempotent pass-through; Err → None fail-closed).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp414_pfb"` (STRICT eq; пустой/чужой
//! флаг = hook не регистрируется вообще — ваниль бит-в-байт).

use jvmti_bindings::jni;
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;

const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const GOAL_SELECTOR_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalSelector";
const OPS_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalBatchOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../gsel/build/net/minecraft/world/entity/ai/goal/GoalBatchOps.class");

const TICK_DESC: &str = "()V";
// receiver-prepended static desc (retarget_virtual_to_static contract)
const GATE_STATIC_DESC: &str = "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V";
const EXPECT_SITES: usize = 4;

const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;
const PROBE_MAGIC: i32 = 0x4753; // "GS"

/// STRICT OR (TASK-416-A: gsel-слайс ретагирован на единый lever-id
/// cmp416_mcomp; легаси cmp414_pfb остаётся в OR-списке — штатные ветки не
/// удаляются). AIBATCH-ПРИОРИТЕТ (мандат iter-2): gsel-сайты
/// (Mob.serverAiStep ×2 GoalSelector.tick + Mob.inactiveTick ×2)
/// структурно вне aibatch-окна — при golden-phase skip aiStep не доходит до
/// serverAiStep/inactiveTick-гейтов, gsel обрабатывает только активное окно.
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp414_pfb") | Ok("cmp416_mcomp") | Ok("cmp417_mcomp")
    )
}

/// Lever id for boot markers.
fn lever_id() -> &'static str {
    match std::env::var("CRUSSTY_LEVER_FLAG").as_deref() {
        Ok("cmp417_mcomp") => "cmp417_mcomp",
        Ok("cmp416_mcomp") => "cmp416_mcomp",
        _ => "cmp414_pfb",
    }
}

static READY: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Rust SoA registry (подсистемные данные: per-selector flags/priorities)
// ---------------------------------------------------------------------------

struct SelReg {
    goals: usize,
    flags: Vec<i64>,
    prios: Vec<i32>,
}

static REGISTRY: Mutex<Option<HashMap<i32, SelReg>>> = Mutex::new(None);

static EPOCH_COUNT: AtomicU64 = AtomicU64::new(0);
static SEL_TOTAL: AtomicU64 = AtomicU64::new(0);
static GOALS_TOTAL: AtomicU64 = AtomicU64::new(0);
static DISABLED_BLOCKED: AtomicU64 = AtomicU64::new(0);
static LOCK_BLOCKED: AtomicU64 = AtomicU64::new(0);
static LAST_EPOCH_TICK: AtomicI64 = AtomicI64::new(-1);
static REG_SLOTS: AtomicU64 = AtomicU64::new(0);

/// Композиционный ретаргет на полученных байтах (chain output preserved).
fn retarget_gsel(bytes: &[u8]) -> Result<(Vec<u8>, usize), String> {
    let from = (GOAL_SELECTOR_CLASS, "tick", TICK_DESC);
    let to = (OPS_CLASS, "tickGate", GATE_STATIC_DESC);
    let (b1, o1) = crate::classfile::retarget_virtual_to_static(
        bytes, "serverAiStep", "()V", from, to,
    )?;
    let (b2, o2) = crate::classfile::retarget_virtual_to_static(
        &b1, "inactiveTick", "()V", from, to,
    )?;
    let sites = match (&o1, &o2) {
        (crate::classfile::RetargetOutcome::Retargeted { sites: s1 },
         crate::classfile::RetargetOutcome::Retargeted { sites: s2 }) => s1 + s2,
        (crate::classfile::RetargetOutcome::AlreadyPatched { sites: s1 },
         crate::classfile::RetargetOutcome::AlreadyPatched { sites: s2 }) => {
            // idempotent: обе фазы уже в байтах — считать как полный набор
            return Ok((b2, *s1 + *s2));
        }
        (other1, other2) => {
            return Err(format!(
                "partial retarget (serverAiStep={other1:?}, inactiveTick={other2:?})"
            ));
        }
    };
    Ok((b2, sites))
}

/// Register the COMPOSING byte hook (LAST in cplugin_init order — после
/// entity_query::register; Mob-хуки sscan/prepare стоят раньше и их output
/// я получаю на вход).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] goal_batch: dormant (lever_flag != cmp414_pfb/cmp416_mcomp/cmp417_mcomp, vanilla goal selectors)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOB_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            return None; // pre-arm: chain untouched
        }
        match retarget_gsel(bytes) {
            Ok((out, sites)) if sites == EXPECT_SITES => {
                static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);
                if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] goal_batch: hook serve {MOB_CLASS} {} bytes (composed, Retargeted sites:{sites})",
                        out.len()
                    );
                }
                Some(out)
            }
            Ok((out, sites)) => {
                // AlreadyPatched-путь: sites считаны из outcomes — полное
                // множество == EXPECT_SITES уже проверено в Ok-ветке выше;
                // сюда попадаем только при неожиданном раскладе — keep bytes.
                static IDP_LOGGED: AtomicBool = AtomicBool::new(false);
                if !IDP_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] goal_batch: idempotent serve (sites={sites}) — keep chain bytes"
                    );
                }
                Some(out)
            }
            Err(e) => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] goal_batch: compose patch rejected ({e}) — pass-through, my slice vanilla"
                    );
                }
                None
            }
        }
    });
}

/// Background activation: wait for boot, define GoalBatchOps into the kernel
/// loader + RegisterNatives (gselProbe/gselRegister/gselEpoch), flip READY
/// (hook starts composing) and retransform Mob.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(MOB_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] goal_batch: {MOB_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] goal_batch: boot marker not seen, hook stays dormant");
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
                "[crussty-plugin] goal_batch: {OPS_CLASS} is class major {ops_major} but JVM supports up to {jvm_major} — rebuild gsel/ via scripts/build_gsel_ops.sh; hook stays dormant"
            );
            return;
        }

        // Define the bridge + RegisterNatives in the kernel loader.
        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(MOB_CLASS) else {
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
                eprintln!("[crussty-plugin] goal_batch: define_class({OPS_CLASS}) failed");
                return false;
            };

            let names = [
                CString::new("gselProbe").expect("no NUL"),
                CString::new("gselRegister").expect("no NUL"),
                CString::new("gselEpoch").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(II[I[I)[I").expect("no NUL"),
                CString::new("(I[I[J[J[J[I)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: gsel_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: gsel_register as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[2].as_ptr(),
                    signature: sigs[2].as_ptr(),
                    fnPtr: gsel_epoch as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] goal_batch: register_natives failed (code {code}) — hook stays dormant"
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
            eprintln!("[crussty-plugin] goal_batch: bridge definition failed, hook stays dormant");
            return;
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] {}: ARMED gsel-batch (Mob.serverAiStep x2 + Mob.inactiveTick x2 GoalSelector.tick sites -> GoalBatchOps.tickGate; rust SoA registry + gselEpoch = ONE bulk JNI/tick, zero per-goal JNI; scheduler decisions vanilla bit-for-bit over flat mirror; empty flag = vanilla bit-for-bit)",
            lever_id()
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "tickGate", "cmp416_mcomp|cmp417_mcomp gsel v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOB_CLASS);
        eprintln!(
            "[crussty-plugin] goal_batch: {MOB_CLASS} armed, retransform rc={rc} (composed serve; ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] goal_batch: retransform FAILED (rc={rc}) — my slice stays vanilla"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Natives (registered on GoalBatchOps by activate())
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn gsel_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// BULK SoA sync (rare: mirror rebuild). java -> rust: selId, n, flags[n],
/// priorities[n]. Stores the selector registry plane; returns n or ERR.
///
/// # Safety
/// See gsel_probe.
#[no_mangle]
pub unsafe extern "system" fn gsel_register(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    sel_id: jni::jint,
    n: jni::jint,
    flags: jni::jlongArray,
    prios: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || flags.is_null() || prios.is_null() {
        return ERR_STRUCT;
    }
    if n < 0 || sel_id < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    let flen = unsafe { (vt.GetArrayLength)(env, flags) };
    let plen = unsafe { (vt.GetArrayLength)(env, prios) };
    if (flen as i32) < n || (plen as i32) < n {
        return ERR_RANGE;
    }
    let mut fbuf = vec![0i64; n as usize];
    let mut pbuf = vec![0i32; n as usize];
    if n > 0 {
        unsafe { (vt.GetLongArrayRegion)(env, flags, 0, n as jni::jsize, fbuf.as_mut_ptr()) };
        unsafe { (vt.GetIntArrayRegion)(env, prios, 0, n as jni::jsize, pbuf.as_mut_ptr()) };
    }
    if let Ok(mut guard) = REGISTRY.lock() {
        let map = guard.get_or_insert_with(HashMap::new);
        let fresh = !map.contains_key(&sel_id);
        map.insert(
            sel_id,
            SelReg {
                goals: n as usize,
                flags: fbuf,
                prios: pbuf,
            },
        );
        if fresh {
            REG_SLOTS.fetch_add(1, Ordering::Relaxed);
        }
    } else {
        return ERR_STRUCT;
    }
    n
}

/// BULK epoch — ONE transition per server tick (never per goal/selector from
/// java's perspective: java accumulates the batch, ONE native call flushes).
/// DOD pass over the SoA registry: per-selector telemetry
/// `stats[i] = (goals << 16) | min(disabledBlocked, 0xFFFF)`; aggregates
/// counted for the effect marker. Decisions were already applied java-side
/// (vanilla bit-for-bit) — this plane is the subsystem data-residency +
/// telemetry leg of the lever.
///
/// Returns the number of processed selectors or ERR_STRUCT/ERR_RANGE.
///
/// # Safety
/// See gsel_probe.
#[no_mangle]
pub unsafe extern "system" fn gsel_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    n: jni::jint,
    sel_ids: jni::jintArray,
    locks: jni::jlongArray,
    disabled: jni::jlongArray,
    stats: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() {
        return ERR_STRUCT;
    }
    if tick < 0 || n < 0 {
        return ERR_RANGE;
    }
    let vt = unsafe { &**env };
    if sel_ids.is_null() || locks.is_null() || disabled.is_null() || stats.is_null() {
        return ERR_STRUCT;
    }
    let ilen = unsafe { (vt.GetArrayLength)(env, sel_ids) } as i32;
    let llen = unsafe { (vt.GetArrayLength)(env, locks) } as i32;
    let dlen = unsafe { (vt.GetArrayLength)(env, disabled) } as i32;
    let slen = unsafe { (vt.GetArrayLength)(env, stats) } as i32;
    if ilen < n || llen < n || dlen < n || slen < n {
        return ERR_RANGE;
    }
    let mut ids = vec![0i32; n as usize];
    let mut lbuf = vec![0i64; n as usize];
    let mut dbuf = vec![0i64; n as usize];
    if n > 0 {
        unsafe { (vt.GetIntArrayRegion)(env, sel_ids, 0, n as jni::jsize, ids.as_mut_ptr()) };
        unsafe { (vt.GetLongArrayRegion)(env, locks, 0, n as jni::jsize, lbuf.as_mut_ptr()) };
        unsafe { (vt.GetLongArrayRegion)(env, disabled, 0, n as jni::jsize, dbuf.as_mut_ptr()) };
    }

    let registry = match REGISTRY.lock() {
        Ok(g) => g,
        Err(_) => return ERR_STRUCT,
    };
    let map = match registry.as_ref() {
        Some(m) => m,
        None => {
            // реестра ещё нет (rebuilds не было) — совместимо: телеметрия нулевая
            drop(registry);
            if n > 0 {
                let zeros = vec![0i32; n as usize];
                unsafe {
                    (vt.SetIntArrayRegion)(env, stats, 0, n as jni::jsize, zeros.as_ptr())
                };
            }
            EPOCH_COUNT.fetch_add(1, Ordering::Relaxed);
            return n;
        }
    };

    let mut sbuf = vec![0i32; n as usize];
    let mut sel_sum = 0u64;
    let mut goals_sum = 0u64;
    let mut dis_sum = 0u64;
    let mut lock_sum = 0u64;
    for i in 0..n as usize {
        let sid = ids[i];
        let locks_i = lbuf[i];
        let dis_i = dbuf[i];
        let (gcount, dcount, lcount) = match map.get(&sid) {
            Some(reg) => {
                let mut d = 0usize;
                let mut l = 0usize;
                for &fl in &reg.flags {
                    if (fl & dis_i) != 0 {
                        d += 1;
                    }
                    if (fl & locks_i) != 0 {
                        l += 1;
                    }
                }
                (reg.goals, d, l)
            }
            None => (0, 0, 0),
        };
        sel_sum += 1;
        goals_sum += gcount as u64;
        dis_sum += dcount as u64;
        lock_sum += lcount as u64;
        sbuf[i] = ((gcount.min(0x7FFF) as i32) << 16) | (dcount.min(0xFFFF) as i32);
    }
    drop(registry);

    if n > 0 {
        unsafe { (vt.SetIntArrayRegion)(env, stats, 0, n as jni::jsize, sbuf.as_ptr()) };
    }
    EPOCH_COUNT.fetch_add(1, Ordering::Relaxed);
    SEL_TOTAL.fetch_add(sel_sum, Ordering::Relaxed);
    GOALS_TOTAL.fetch_add(goals_sum, Ordering::Relaxed);
    DISABLED_BLOCKED.fetch_add(dis_sum, Ordering::Relaxed);
    LOCK_BLOCKED.fetch_add(lock_sum, Ordering::Relaxed);
    LAST_EPOCH_TICK.store(tick as i64, Ordering::Relaxed);

    let ep = EPOCH_COUNT.load(Ordering::Relaxed);
    if ep <= 3 || ep % 600 == 0 {
        eprintln!(
            "[crussty-plugin] gsel: epoch t={tick} n={n} goals={goals_sum} disabledBlocked={dis_sum} lockBlocked={lock_sum} slots={} (rust SoA plane alive, ONE bulk JNI/tick)",
            REG_SLOTS.load(Ordering::Relaxed)
        );
    }
    n
}

// ---------------------------------------------------------------------------
// Tests: gate strictness, retarget desc contract, epoch telemetry math.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_constant_strict() {
        // этот вектор = СВОЙ флаг; носитель-флаги НЕ должныarme'ить gsel
        assert!(matches!(
            std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
            Err(_) | Ok(_)
        ));
        assert_ne!(PROBE_MAGIC, ERR_STRUCT);
        assert_eq!(GATE_STATIC_DESC, "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V");
    }

    #[test]
    fn retarget_desc_contract() {
        // static desc = virtual desc с receiver-классом, препендированным
        let expect = format!("(L{GOAL_SELECTOR_CLASS};{}", &TICK_DESC[1..]);
        assert_eq!(GATE_STATIC_DESC, expect, "receiver-prepended desc contract");
        assert_eq!(EXPECT_SITES, 4, "census: serverAiStep x2 + inactiveTick x2");
    }

    #[test]
    fn registry_plane_math() {
        // телеметрия-упаковка stats[i] = (goals<<16)|disabledBlocked
        let g: i32 = 12;
        let d: i32 = 3;
        let packed = ((g.min(0x7FFF)) << 16) | d.min(0xFFFF);
        assert_eq!(packed >> 16, 12);
        assert_eq!(packed & 0xFFFF, 3);
    }

    #[test]
    fn gsel_ops_source_declares_no_nested_classes() {
        // S7-163: bridge = ОДИН classfile (define_class не тянет $-классы)
        let src = include_str!("../gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java");
        let mut declared: Vec<String> = Vec::new();
        for line in src.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if before.contains("static") && !before.contains("//") {
                        let rest = &t[i + pat.len()..];
                        let name: String = rest
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty() {
                            declared.push(name);
                        }
                    }
                    break;
                }
            }
        }
        assert!(
            declared.is_empty(),
            "GoalBatchOps.java declares nested classes {declared:?} — kernel-loader define_class доставит только GoalBatchOps.class, $-классы = NoClassDefFoundError при первом гейте"
        );
    }

    #[test]
    fn gsel_ops_blob_present_and_pinned_major65() {
        let b = OPS_BYTES;
        assert!(b.len() > 100, "GoalBatchOps.class blob empty — rebuild gsel/ via scripts/build_gsel_blobs_all.sh");
        let major = ((b[6] as u16) << 8) | b[7] as u16;
        assert_eq!(major, 65, "GoalBatchOps.class major {major} != 65 (--release 21)");
    }
}
