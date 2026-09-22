//! Runtime wiring for the GOAL-SELECTOR BATCH plane (TASK-414-C2, vector
//! «pathfinder/brain виток-2: GoalSelector tick-scheduling batch» — lever
//! `cmp416_gsel3`; bridge gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java,
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
//! RUST-FIRST (закон 6): rust-плоскость держит DENSE SoA-реестр селекторов
//! (коло́нки goals_col/flags_col/prios_col + offsets, selId = индекс; bulk
//! `gselRegister` sync при редкой регистрации) и обрабатывает ОДИН bulk JNI
//! за серверный тик. ИТЕРАЦИЯ-4 (TASK-416-B, бисекция по round-g3b
//! collapsed-профилям): gselEpoch стал СКАЛЯРНЫМ `(IJ)I` — ноль JNI-копий
//! массивов (раньше GetIntArrayRegion/GetLongArrayRegion на 65k×3 элементов
//! под глобальным java-локом на каждый тик); DOD-проход по коло́нкам суммирует
//! реестр локально в Rust. ZERO per-entity JNI, zero per-tick JNI array copy.
//! Последовательные решения (canUse/canContinueToUse/start/stop) остаются
//! java-ваниль над плоским зеркалом — порядок и решения бит-в-байт
//! (jsel contract в GoalBatchOps.java).
//!
//! ИТЕРАЦИЯ-3 ВЕРДИКТ (анти-урок, встроен в iter-4): nav_ai-эффект РЕАЛЕН
//! (9.31 → 6.63-7.04), но батч-цикл (мутация/публикация/аллокации) съедал
//! лейн-выигрыш ×6: Integer-боксинг selId + System.getenv byte[] на КАЖДОМ
//! гейте (×~300k/тик) = GC-churn (avg-pause 156мс vs cvs 22-24мс),
//! BATCH_LOCK monitor-park на 4 region-workers, JNI-копии 65k×3 под локом.
//! iter-4 java: lever-флаг static-final кэш, ОДНА identity-CHM на гейт,
//! LongAdder-гейты, скалярный epoch; rust: коло́ночный реестр вместо
//! HashMap<i32,SelReg>.
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
//! Gate: env `CRUSSTY_LEVER_FLAG == "cmp416_gsel3"` (STRICT eq; пустой/чужой
//! флаг = hook не регистрируется вообще — ваниль бит-в-байт).

use jvmti_bindings::jni;
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

/// iter-4: gselEpoch — скалярный дескриптор (tick:int, gatesSum:long) -> int;
/// ноль JNI-массивов (бисекция round-g3b). Institutional: тест
/// epoch_scalar_sig_contract держит контракт вместе с
/// scripts/check_gsel_native_sigs.sh (javap ground truth).
const EPOCH_SIG: &str = "(IJ)I";

/// STRICT-eq gate: только флаг этого вектора (cmp416_gsel3).
fn enabled() -> bool {
    matches!(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref(),
        Ok("cmp416_gsel3")
    )
}

static READY: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Rust DENSE SoA registry (подсистемные данные: коло́нки по всем селекторам,
// selId = индекс; iter-4 — замена HashMap<i32, SelReg>, DOD-проход по коло́нке
// — последовательное чтение, prefetch-friendly)
// ---------------------------------------------------------------------------

/// Колоночный реестр: offsets.len() == goals_col.len() + 1, offsets[i] —
/// старт селектора i в flags_col/prios_col (инвариант offsets[0] == 0).
#[derive(Default)]
struct SoaRegistry {
    goals_col: Vec<u32>,
    flags_col: Vec<i64>,
    prios_col: Vec<i32>,
    offsets: Vec<u32>,
}

impl SoaRegistry {
    const fn new() -> Self {
        Self {
            goals_col: Vec::new(),
            flags_col: Vec::new(),
            prios_col: Vec::new(),
            offsets: Vec::new(),
        }
    }

    fn slots(&self) -> usize {
        self.goals_col.len()
    }

    /// Append (sel_id == slots, живой путь) / gap-fill + append (defensive:
    /// сожжённый id после неудачного register) / замена диапазона (rare
    /// re-register). Err = разной длины flags/prios (недостижимо с java).
    fn upsert(&mut self, sel_id: usize, flags: Vec<i64>, prios: Vec<i32>) -> Result<(), ()> {
        if flags.len() != prios.len() {
            return Err(());
        }
        while sel_id > self.slots() {
            // gap: пустой слот (walk-сумма не затрагивается, плотность ОК)
            self.append(0, &[], &[]);
        }
        if sel_id == self.slots() {
            self.append(flags.len(), &flags, &prios);
            return Ok(());
        }
        // replace range (re-register): сохранить плотность коло́нок
        let start = self.offsets[sel_id] as usize;
        let end = self.offsets[sel_id + 1] as usize;
        let n = flags.len();
        let delta = n as i64 - (end - start) as i64;
        self.flags_col.splice(start..end, flags);
        self.prios_col.splice(start..end, prios);
        self.goals_col[sel_id] = n as u32;
        if delta != 0 {
            for off in self.offsets[sel_id + 1..].iter_mut() {
                *off = ((*off as i64) + delta) as u32;
            }
        }
        Ok(())
    }

    fn append(&mut self, n: usize, flags: &[i64], prios: &[i32]) {
        if self.offsets.is_empty() {
            // инвариант: offsets[0] == 0 (старт селектора 0), len == slots + 1
            self.offsets.push(0u32);
        }
        let start = (*self.offsets.last().unwrap()) as usize;
        self.flags_col.extend_from_slice(flags);
        self.prios_col.extend_from_slice(prios);
        self.goals_col.push(n as u32);
        self.offsets.push((start + n) as u32);
    }

    /// DOD-проход по коло́нке целей (последовательный, ~2ns/элемент).
    fn walk_goal_sum(&self) -> u64 {
        self.goals_col.iter().map(|&g| g as u64).sum()
    }
}

static REGISTRY: Mutex<SoaRegistry> = Mutex::new(SoaRegistry::new());

static EPOCH_COUNT: AtomicU64 = AtomicU64::new(0);
static SEL_TOTAL: AtomicU64 = AtomicU64::new(0);
static GOALS_TOTAL: AtomicU64 = AtomicU64::new(0);
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
            "[crussty-plugin] goal_batch: dormant (lever_flag != cmp416_gsel3, vanilla goal selectors)"
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
                // ROOT-CAUSE pfb1 (TASK-415-B iter-2): опечатанные sig-строки
                // → RegisterNatives = JNI_ERR(-1) → hook dormant. Точные
                // дескрипторы (javap -s ground truth, iter-4):
                //   gselProbe()I / gselRegister(II[J[I)I / gselEpoch(IJ)I
                CString::new("(II[J[I)I").expect("no NUL"),
                CString::new("(IJ)I").expect("no NUL"), // == EPOCH_SIG (javap-gate greps литерал)
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
            "[crussty-plugin] cmp416_gsel3: ARMED gsel-batch iter-4 fastpath (Mob.serverAiStep x2 + Mob.inactiveTick x2 GoalSelector.tick sites -> GoalBatchOps.tickGate; rust dense-SoA registry + SCALAR gselEpoch(IJ)I = ONE bulk JNI/tick zero array copies; java fastpath: cached lever flag, single identity-CHM, no hot-path monitor, LongAdder gates; scheduler decisions vanilla bit-for-bit over flat mirror; empty flag = vanilla bit-for-bit)"
        );

        crate::kernel_policy::audit_wire(OPS_CLASS, "tickGate", "cmp416_gsel3 v1");
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

/// BULK SoA sync (rare: first-sighting registration). java -> rust: selId,
/// n, flags[n], priorities[n]. Writes the DENSE SoA columns (selId = index,
/// append-only на живом пути — java гарантирует плотную нумерацию);
/// returns n or ERR.
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
    match REGISTRY.lock() {
        Ok(mut reg) => {
            let fresh = (sel_id as usize) == reg.slots();
            if reg.upsert(sel_id as usize, fbuf, pbuf).is_err() {
                return ERR_RANGE;
            }
            if fresh {
                REG_SLOTS.fetch_add(1, Ordering::Relaxed);
            }
        }
        Err(_) => return ERR_STRUCT,
    }
    n
}

/// BULK epoch — ONE transition per server tick (never per goal/selector from
/// java's perspective), СКАЛЯРНЫЙ (iter-4): java передаёт только (tick,
/// gatesSum) — ноль JNI-копий массивов (iter-3 платил GetArrayRegion на
/// 65k×3 элементов под глобальным java-локом каждый тик; бисекция g3b).
/// DOD-проход по коло́нкам SoA-реестра: сумма целей по всем слотам —
/// последовательное чтение ~2ns/элемент. Decisions were already applied
/// java-side (vanilla bit-for-bit) — this plane is the subsystem
/// data-residency + telemetry leg of the lever.
///
/// Returns the number of registered slots or ERR_STRUCT/ERR_RANGE.
///
/// # Safety
/// See gsel_probe.
#[no_mangle]
pub unsafe extern "system" fn gsel_epoch(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    tick: jni::jint,
    gates_sum: jni::jlong,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    if tick < 0 || gates_sum < 0 {
        return ERR_RANGE;
    }
    let registry = match REGISTRY.lock() {
        Ok(g) => g,
        Err(_) => return ERR_STRUCT,
    };
    let slots = registry.slots() as u64;
    let goals_sum = registry.walk_goal_sum();
    drop(registry);

    EPOCH_COUNT.fetch_add(1, Ordering::Relaxed);
    SEL_TOTAL.fetch_add(slots, Ordering::Relaxed);
    GOALS_TOTAL.fetch_add(goals_sum, Ordering::Relaxed);
    LAST_EPOCH_TICK.store(tick as i64, Ordering::Relaxed);

    let ep = EPOCH_COUNT.load(Ordering::Relaxed);
    if ep <= 3 || ep % 600 == 0 {
        eprintln!(
            "[crussty-plugin] gsel: epoch t={tick} slots={slots} goals={goals_sum} gates_java={gates_sum} (rust dense-SoA plane alive, ONE scalar bulk JNI/tick, zero array copies)"
        );
    }
    slots as jni::jint
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
    fn soa_append_and_walk() {
        // коло́ночный реестр: append 3 селекторов, инвариант offsets + walk-сумма
        let mut reg = SoaRegistry::new();
        reg.upsert(0, vec![1, 2], vec![0, 1]).unwrap();
        reg.upsert(1, vec![4], vec![2]).unwrap();
        reg.upsert(2, vec![], vec![]).unwrap();
        assert_eq!(reg.slots(), 3);
        assert_eq!(reg.offsets, vec![0, 2, 3, 3], "offsets invariant (len=slots+1, monotone)");
        assert_eq!(reg.walk_goal_sum(), 3, "DOD-сумма по коло́нке целей");
    }

    #[test]
    fn soa_gap_fill_defensive() {
        // сожжённый id после неудачного register не ломает плотность:
        // gap-fill пустыми слотами, walk-сумма не затрагивается
        let mut reg = SoaRegistry::new();
        reg.upsert(0, vec![1], vec![0]).unwrap();
        reg.upsert(3, vec![5, 5], vec![1, 1]).unwrap(); // gap 1..3
        assert_eq!(reg.slots(), 4);
        assert_eq!(reg.goals_col, vec![1, 0, 0, 2]);
        assert_eq!(reg.offsets, vec![0, 1, 1, 1, 3]);
        assert_eq!(reg.walk_goal_sum(), 3);
    }

    #[test]
    fn soa_replace_preserves_density() {
        // re-register (rare rebuild path): замена диапазона сохраняет
        // плотность коло́нок при изменении n
        let mut reg = SoaRegistry::new();
        reg.upsert(0, vec![1, 2, 3], vec![0, 0, 0]).unwrap();
        reg.upsert(1, vec![7, 7], vec![1, 1]).unwrap();
        // заменяем sel 0: n 3 → 1
        reg.upsert(0, vec![9], vec![5]).unwrap();
        assert_eq!(reg.slots(), 2);
        assert_eq!(reg.offsets, vec![0, 1, 3]);
        assert_eq!(reg.flags_col, vec![9, 7, 7]);
        assert_eq!(reg.prios_col, vec![5, 1, 1]);
        assert_eq!(reg.walk_goal_sum(), 3);
        // заменяем sel 0 обратно: n 1 → 2
        reg.upsert(0, vec![3, 3], vec![8, 8]).unwrap();
        assert_eq!(reg.flags_col, vec![3, 3, 7, 7]);
        assert_eq!(reg.offsets, vec![0, 2, 4]);
    }

    #[test]
    fn epoch_scalar_sig_contract() {
        // iter-4: gselEpoch СКАЛЯРНЫЙ (IJ)I — ноль JNI-массивов в дескрипторе
        // (бисекция g3b: GetArrayRegion 65k×3 под java-локом = overhead)
        assert_eq!(EPOCH_SIG, "(IJ)I");
        assert!(!EPOCH_SIG.contains('['), "epoch desc не должен нести массивы");
        // старый батч-дескриптор удалён (склеиваем литерал — сам тест его содержит)
        let src = include_str!("../src/goal_batch.rs");
        let old = format!("(II[I[{}[{}I)I", "J", "J");
        assert!(!src.contains(&old), "старый батч-дескриптор удалён");
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
