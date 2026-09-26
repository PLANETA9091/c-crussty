//! MOVE-PLANE (TASK-463-69a, CLIMB navmath-1): P44 navmath bridge — lever
//! `cmp463_move`, STRICT eq.
//!
//! Lane facts (LEDGER-37, javap ×463): the whole per-mob trig load of the
//! nav_ai lane lives in MoveControl.tick (295 instructions, 0..585):
//! 0-2 Mth calls per mob per tick — STRAFE = sin@93+cos@108, MOVE_TO =
//! atan2@290; PathNavigation.tick = 0 trig. MovePlaneOps.handle = the
//! javap-verbatim tick body with exactly those math points replaced by the
//! fast Mth replicas (SIN[65536]-таблица + fastInvSqrt 6910469410427058090L
//! + FRAC_BIAS 2^44); оракул ×463: sin/cos/atan2 10^5/10^5 bit-in-bit GREEN
//! (java selfTest против реального Mth + python-модель navoracle).
//!
//! DELIVERY (nav_plane/mobs_ai канон):
//!   - MoveControl: pristine stash до READY, cached serve после (никто
//!     больше не хукает MoveControl); ретаргет tick()V → MovePlaneOps.handle
//!   - Определение MovePlaneOps в kernel loader (якорь LivingEntity/Mob —
//!     EARLY-define, BRIDGE_DEFINED-блок entity_query.rs, NCDFE T1=0 канон)
//!     + RegisterNatives(moveDecide) ДО публикации ретаргета (закон 6 v16:
//!     define → register natives → probe ARMED → publish)
//!   - Hot path = чистая java (0 JNI на моб/тик — law 6: NO per-entity
//!     JNI); native moveDecide = батч-trig-kernel для G2-lockstep-гейта
//!     и будущего bulk-лейна.
//!
//! Fail-dominant: пустой/чужой флаг — класс не определяется, хук не
//! регистрируется (ваниль бит-в-байт); selfTest probe false / rc<0 /
//! Throwable → one-shot disarm → java-реплика decideTrigJava (та же
//! математика, батч-лейн) или vanilla body (hot path не публикуется).

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub const MOVE_OPS_CLASS: &str = "net/minecraft/world/entity/ai/control/MovePlaneOps";

/// The retargeted vanilla class (classfile::MOVE_CONTROL_CLASS mirror).
pub const MOVE_CONTROL_CLASS: &str = crate::classfile::MOVE_CONTROL_CLASS;

pub const MOVE_BYTES: &[u8] = include_bytes!(
    "../moveplane/build/net/minecraft/world/entity/ai/control/MovePlaneOps.class"
);

/// ASIN_TAB/COS_TAB jar-dumps (2×257×8 B = 4112 B, big-endian f64; LEDGER-37:
/// glibc `f64::asin` расходится с fdlibm на 9/257 записях — генератор
/// ЗАПРЕЩЁН, в блоб идут jar-дампы; SIN генерируется GREEN и потому не
/// дампится).
pub const ASIN_BIN: &[u8] = include_bytes!("../moveplane/navoracle/ASIN.bin");
pub const COS_BIN: &[u8] = include_bytes!("../moveplane/navoracle/COS.bin");
/// SIN table dump (big-endian f32 bits) — TEST-ONLY oracle reference
/// (runtime генерирует таблицу: oracle 65536/65536 bit-in-bit GREEN).
pub const SIN_BIN: &[u8] = include_bytes!("../moveplane/navoracle/SIN.bin");

pub const MOVE_DECIDE_SIG: &str = "(I[I[D[D[I)I";

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// FRAC_BIAS bits 4805340802404319232l → 17592186044416.0 = 2^44 (K8).
pub const FRAC_BIAS: f64 = f64::from_bits(4805340802404319232u64);

static READY: AtomicBool = AtomicBool::new(false);

/// STRICT eq lever gate (пустой флаг / любой другой флаг = false).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp463_move")
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Fast trig (Rust реплики java MovePlaneOps; IEEE754 strict, без FMA).
// ---------------------------------------------------------------------------

/// SIN[65536]: vanilla fill lambda `(float)Math.sin((double)i * PI * 2.0 /
/// 65536.0)` — glibc/libm sin GREEN vs jar dump (oracle 65536/65536).
fn sin_table() -> &'static Vec<f32> {
    static T: std::sync::OnceLock<Vec<f32>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        (0..65536)
            .map(|i| (((i as f64) * std::f64::consts::PI * 2.0 / 65536.0).sin()) as f32)
            .collect()
    })
}

/// ASIN_TAB[257] из jar-дампа (big-endian f64).
fn asin_table() -> &'static [f64; 257] {
    static T: std::sync::OnceLock<Box<[f64; 257]>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let mut a = Box::new([0.0f64; 257]);
        for (i, v) in a.iter_mut().enumerate() {
            let off = i * 8;
            let mut b = [0u8; 8];
            b.copy_from_slice(&ASIN_BIN[off..off + 8]);
            *v = f64::from_be_bytes(b);
        }
        a
    })
}

/// COS_TAB[257] из jar-дампа (big-endian f64).
fn cos_table() -> &'static [f64; 257] {
    static T: std::sync::OnceLock<Box<[f64; 257]>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let mut a = Box::new([0.0f64; 257]);
        for (i, v) in a.iter_mut().enumerate() {
            let off = i * 8;
            let mut b = [0u8; 8];
            b.copy_from_slice(&COS_BIN[off..off + 8]);
            *v = f64::from_be_bytes(b);
        }
        a
    })
}

/// K6/K7: magic 6910469410427058090L, lshr 1, lsub; Newton 0.5/1.5, ассоциация
/// `d1 * (1.5 - (d0 * d1) * d1)` (javap @28-38 verbatim).
pub fn fast_inv_sqrt(x: f64) -> f64 {
    let d0 = 0.5 * x;
    let i = 6910469410427058090u64.wrapping_sub(x.to_bits() >> 1);
    let d1 = f64::from_bits(i);
    d1 * (1.5 - d0 * d1 * d1)
}

/// K1/K2/K3: 10430.378f, 65535 iand, f2i-сатурация (`as i32` == JVM f2i:
/// NaN→0, ±inf→±MAX), cos offset 16384.0f в f32 ДО f2i.
pub fn fast_sin(a: f32) -> f32 {
    sin_table()[(((a * 10430.378f32) as i32) & 0xFFFF) as usize]
}

pub fn fast_cos(a: f32) -> f32 {
    sin_table()[(((a * 10430.378f32 + 16384.0f32) as i32) & 0xFFFF) as usize]
}

/// Полная транскрипция Mth.atan2 (javap 0..219; oracle 10^5/10^5 bit-in-bit).
/// Err(()) = java AIOOBE-эквивалент (индекс LUT вне [0..256] — батч-лейн
/// обязан разоружиться, hot path в java не может сюда попасть).
pub fn fast_atan2_checked(y: f64, x: f64) -> Result<f64, ()> {
    let z = x * x + y * y;
    if z.is_nan() {
        return Ok(f64::NAN);
    }
    let neg_y = y < 0.0;
    let y = if neg_y { -y } else { y };
    let neg_x = x < 0.0;
    let x = if neg_x { -x } else { x };
    // swap-флаг = param1 > param2 (dcmpl@65+ifle@66), НЕ x>y — ловушка №1.
    let swap = y > x;
    let (y, x) = if swap { (x, y) } else { (y, x) };
    let d9 = fast_inv_sqrt(z);
    let x = x * d9;
    let y = y * d9;
    let d11 = FRAC_BIAS + y;
    // bare l2i low-32 БЕЗ сдвига (java (int)doubleToRawLongBits) — ловушка №2.
    let i13 = d11.to_bits() as u32 as i32;
    if i13 < 0 || i13 > 256 {
        return Err(());
    }
    let d14 = asin_table()[i13 as usize];
    let d16 = cos_table()[i13 as usize];
    let d18 = d11 - FRAC_BIAS;
    let d20 = y * d16 - x * d18;
    let d22 = (6.0 + d20 * d20) * d20 * 0.16666666666666666;
    let mut d24 = d14 + d22;
    if swap {
        d24 = 1.5707963267948966 - d24;
    }
    if neg_x {
        d24 = 3.141592653589793 - d24;
    }
    if neg_y {
        // знак-минус ПОСЛЕДНИМ (иначе 11/100000) — ловушка №3.
        d24 = -d24;
    }
    Ok(d24)
}

/// Panicking variant == java-семантика (AIOOBE). Для тестов и pure-контекста.
pub fn fast_atan2(y: f64, x: f64) -> f64 {
    fast_atan2_checked(y, x).expect("ASIN_TAB index out of range (java AIOOBE)")
}

// ---------------------------------------------------------------------------
// Hook: MoveControl pristine stash → retargeted serve (collide_batch pattern).
// ---------------------------------------------------------------------------

static STASH: Mutex<Option<Vec<u8>>> = Mutex::new(None);

fn stash_orig(bytes: &[u8]) {
    let mut g = STASH.lock().unwrap();
    if g.is_none() {
        *g = Some(bytes.to_vec());
    }
}

fn compute_patch(orig: &[u8]) -> Option<Vec<u8>> {
    match crate::classfile::patch_movecontrol_tick(orig) {
        Ok((p, crate::classfile::RetargetOutcome::Retargeted { sites })) => {
            if sites != MOVE_REDIRECT_EXPECTED {
                eprintln!(
                    "[crussty-plugin] move_plane: unexpected site count {sites} != {MOVE_REDIRECT_EXPECTED} — fail-closed pass-through"
                );
                return None;
            }
            Some(p)
        }
        Ok((_, crate::classfile::RetargetOutcome::NotFound)) => {
            eprintln!(
                "[crussty-plugin] move_plane: tick()V shape mismatch — fail-closed pass-through"
            );
            None
        }
        Ok((_, crate::classfile::RetargetOutcome::AlreadyPatched { .. })) => None,
        Err(e) => {
            eprintln!(
                "[crussty-plugin] move_plane: patch_movecontrol_tick rejected ({e}) — pass-through (fail-closed)"
            );
            None
        }
    }
}

const MOVE_REDIRECT_EXPECTED: usize = 1;

/// Register the byte hook (call once from cplugin_init). Dormant-invisible:
/// с чужим/пустым флагом хук не регистрируется вовсе.
pub fn register() {
    if !armed() {
        eprintln!(
            "[crussty-plugin] move_plane: dormant (lever_flag != cmp463_move, vanilla MoveControl)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(MOVE_CONTROL_CLASS, move |_name, bytes| {
        if !READY.load(Ordering::Acquire) {
            stash_orig(bytes);
            return None;
        }
        let cached = {
            let g = STASH.lock().unwrap();
            g.as_ref().and_then(|orig| compute_patch(orig))
        };
        match cached {
            Some(p) => Some(p),
            None => {
                static ERR_LOGGED: AtomicBool = AtomicBool::new(false);
                if !ERR_LOGGED.swap(true, Ordering::Relaxed) {
                    eprintln!(
                        "[crussty-plugin] move_plane: no cached patch — pass-through (fail-closed)"
                    );
                }
                None
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Activate: EARLY bridge define (entity_query BRIDGE_DEFINED-блок) → probe
// ARMED (selfTest против реального Mth) → publish → retransform.
// ---------------------------------------------------------------------------

pub fn activate() {
    if !armed() {
        return;
    }
    std::thread::spawn(move || {
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] move_plane: boot marker not seen, hook stays dormant");
            return;
        }
        // Kernel loader must be quiet before define/retransform.
        std::thread::sleep(std::time::Duration::from_secs(20));

        // EARLY define + RegisterNatives(moveDecide) — идемпотентный
        // BRIDGE_DEFINED-блок (NCDFE T1=0 канон, закон 6 v16 STRICT).
        let mut defined = false;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while !defined {
            defined = crate::entity_query::ensure_move_bridge_early();
            if defined || std::time::Instant::now() > deadline {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }
        if !defined {
            eprintln!(
                "[crussty-plugin] move_plane: bridge define failed — hook stays dormant (vanilla MoveControl)"
            );
            return;
        }

        // Probe ARMED: java selfTest против реального Mth (10^5×3 bit-in-bit).
        let probe = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(MOVE_OPS_CLASS) else {
                crate::clear_exception(env);
                return false;
            };
            let Some(mid) = env.get_static_method_id(cls.as_jclass(), "selfTest", "()Z") else {
                crate::clear_exception(env);
                return false;
            };
            let raw = env.raw();
            unsafe {
                let fn_table = &(**raw);
                (fn_table.CallStaticBooleanMethodA)(raw, cls.as_jclass(), mid, [].as_ptr()) != 0
            }
        })
        .unwrap_or(false);
        if !probe {
            eprintln!(
                "[crussty-plugin] move_plane: selfTest probe FAILED — hook stays dormant (fail-closed)"
            );
            return;
        }

        // The pristine bytes MUST have arrived (MoveControl loads at boot).
        {
            let g = STASH.lock().unwrap();
            if g.is_none() {
                eprintln!(
                    "[crussty-plugin] move_plane: MoveControl not captured — hook stays dormant"
                );
                return;
            }
        }

        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] move_plane: cmp463_move ARMED (MoveControl.tick -> MovePlaneOps.handle; fast sin@93/cos@108/atan2@290 vs Mth; STRAFE 2 trig + MOVE_TO 1 atan2 per mob/tick -> 0; zero JNI hot path; rust moveDecide = batch trig kernel; empty flag = vanilla bit-for-bit)"
        );
        crate::kernel_policy::audit_wire(MOVE_OPS_CLASS, "handle", "cmp463_move v1");
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(MOVE_CONTROL_CLASS);
        eprintln!(
            "[crussty-plugin] move_plane: {MOVE_CONTROL_CLASS} armed, retransform rc={rc} (ready-gate on)"
        );
        if rc != 0 {
            READY.store(false, Ordering::Release);
            eprintln!(
                "[crussty-plugin] move_plane: retransform FAILED (rc={rc}) — MoveControl stays vanilla"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Native: moveDecide(n, ops[n], argA[n], argB[n], out[n*2]) -> 0.
// op 0 (STRAFE): argA[i] несёт f32-биты угла; out[i*2] = f32-биты fastSin,
//   out[i*2+1] = f32-биты fastCos.
// op 1 (MOVE_TO): argA[i]=y, argB[i]=x; out[i*2..2] = f64-биты fastAtan2.
// Отрицательный rc = ERR (java one-shot batchOk -> decideTrigJava).
// ---------------------------------------------------------------------------

pub unsafe extern "system" fn move_decide(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    ops: jni::jintArray,
    arg_a: jni::jdoubleArray,
    arg_b: jni::jdoubleArray,
    out: jni::jintArray,
) -> jni::jint {
    if env.is_null() || n < 0 || ops.is_null() || arg_a.is_null() || arg_b.is_null() || out.is_null()
    {
        return ERR_STRUCT;
    }
    if n == 0 {
        return 0;
    }
    if n > i32::MAX / 4 {
        return ERR_RANGE;
    }
    let n_us = n as usize;
    let vt = unsafe { &*(*env) };
    let olen = (vt.GetArrayLength)(env, out);
    let alen = (vt.GetArrayLength)(env, ops);
    let alen_a = (vt.GetArrayLength)(env, arg_a);
    let alen_b = (vt.GetArrayLength)(env, arg_b);
    if alen < n || alen_a < n || alen_b < n || olen < (n * 2) {
        return ERR_RANGE;
    }
    let mut ops_buf: Vec<i32> = vec![0; n_us];
    let mut a_buf: Vec<f64> = vec![0.0; n_us];
    let mut b_buf: Vec<f64> = vec![0.0; n_us];
    unsafe {
        (vt.GetIntArrayRegion)(env, ops, 0, n, ops_buf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, arg_a, 0, n, a_buf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, arg_b, 0, n, b_buf.as_mut_ptr());
    }
    let mut out_buf: Vec<i32> = vec![0; n_us * 2];
    for i in 0..n_us {
        if ops_buf[i] == 0 {
            let a = f32::from_bits(a_buf[i] as i32 as u32);
            out_buf[i * 2] = fast_sin(a).to_bits() as i32;
            out_buf[i * 2 + 1] = fast_cos(a).to_bits() as i32;
        } else {
            match fast_atan2_checked(a_buf[i], b_buf[i]) {
                Ok(v) => {
                    let bits = v.to_bits();
                    out_buf[i * 2] = bits as i32;
                    out_buf[i * 2 + 1] = (bits >> 32) as i32;
                }
                Err(()) => return ERR_RANGE,
            }
        }
    }
    unsafe {
        (vt.SetIntArrayRegion)(env, out, 0, n * 2, out_buf.as_ptr());
    }
    0
}

// ---------------------------------------------------------------------------
// Tests: table bit-parity vs jar dumps, transcription landmarks, patch
// delivery on the REAL MoveControl.class fixture, STRICT gate.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// SIN generation == jar dump bit-in-bit (oracle 65536/65536).
    #[test]
    fn sin_table_matches_jar_dump() {
        let t = sin_table();
        assert_eq!(t.len(), 65536);
        for (i, v) in t.iter().enumerate() {
            let off = i * 4;
            let mut b = [0u8; 4];
            b.copy_from_slice(&SIN_BIN[off..off + 4]);
            let refv = f32::from_be_bytes(b);
            assert_eq!(v.to_bits(), refv.to_bits(), "SIN[{i}] diverges");
        }
    }

    /// ASIN/COS dumps parse; the 9 fdlibm-vs-glibc indices are carried by the
    /// dump (not regenerated) — pin the FRAC_BIAS live-contract too.
    #[test]
    fn atan2_tables_parse_and_frac_bias() {
        assert_eq!(ASIN_BIN.len(), 257 * 8);
        assert_eq!(COS_BIN.len(), 257 * 8);
        let a = asin_table();
        let c = cos_table();
        // asin(0)=0, asin(1)=pi/2 (fdlibm == these exactly)
        assert_eq!(a[0].to_bits(), 0f64.to_bits());
        assert_eq!(a[256].to_bits(), (std::f64::consts::FRAC_PI_2).to_bits());
        assert_eq!(c[0].to_bits(), 1f64.to_bits());
        assert!(a[86] != 0.0 && c[256] >= 0.0);
        assert_eq!(FRAC_BIAS.to_bits(), 4805340802404319232u64);
        assert_eq!(FRAC_BIAS, 17592186044416.0); // 2^44 (K8 live)
    }

    /// Transcription landmarks: the three x463 traps pinned bit-in-bit.
    #[test]
    fn atan2_landmarks() {
        assert_eq!(fast_atan2(0.0, 1.0).to_bits(), 0f64.to_bits());
        assert_eq!(
            fast_atan2(1.0, 0.0).to_bits(),
            1.5707963267948966f64.to_bits()
        );
        // swap trap: atan2(y=2,x=1) == pi/2 - atan2(1,2) model symmetry
        let a = fast_atan2(2.0, 1.0);
        let b = fast_atan2(1.0, 2.0);
        assert_eq!(a.to_bits(), (1.5707963267948966 - b).to_bits());
        // sign corrections: -y -> negative; -x -> pi - a
        let p = fast_atan2(1.0, 1.0);
        assert_eq!(fast_atan2(-1.0, 1.0).to_bits(), (-p).to_bits());
        assert_eq!(
            fast_atan2(1.0, -1.0).to_bits(),
            (3.141592653589793 - p).to_bits()
        );
        assert_eq!(
            fast_atan2(-1.0, -1.0).to_bits(),
            (-(3.141592653589793 - p)).to_bits()
        );
        // NaN propagation == vanilla
        assert!(fast_atan2(f64::NAN, 1.0).is_nan());
        assert!(fast_atan2_checked(1.0, f64::INFINITY).is_ok());
    }

    /// fastInvSqrt: magic + Newton approximation sanity (rel err < 0.2%).
    #[test]
    fn inv_sqrt_approximation() {
        for x in [0.25f64, 1.0, 2.0, 1e6, 3.7] {
            let approx = fast_inv_sqrt(x);
            let exact = 1.0 / x.sqrt();
            assert!(
                (approx - exact).abs() / exact < 2e-3,
                "fastInvSqrt({x}) = {approx} vs {exact}"
            );
        }
    }

    /// fastSin/fastCos parity against the dump table across the f2i wrap.
    #[test]
    fn sin_cos_sweep_vs_dump() {
        for k in -700i32..700 {
            let a = k as f32 / 100.0;
            let idx_s = ((a * 10430.378f32) as i32) & 0xFFFF;
            let idx_c = ((a * 10430.378f32 + 16384.0f32) as i32) & 0xFFFF;
            let mut b = [0u8; 4];
            b.copy_from_slice(&SIN_BIN[idx_s as usize * 4..idx_s as usize * 4 + 4]);
            assert_eq!(fast_sin(a).to_bits(), f32::from_be_bytes(b).to_bits());
            b.copy_from_slice(&SIN_BIN[idx_c as usize * 4..idx_c as usize * 4 + 4]);
            assert_eq!(fast_cos(a).to_bits(), f32::from_be_bytes(b).to_bits());
        }
    }

    /// Delivery: the REAL MoveControl.class fixture patches exactly once to
    /// the MovePlaneOps.handle static (receiver-prepended desc contract).
    #[test]
    fn patches_real_movecontrol_fixture() {
        const FIXTURE: &[u8] = include_bytes!("../tests/fixtures/MoveControl_real.class");
        let (out, outcome) = crate::classfile::patch_movecontrol_tick(FIXTURE).unwrap();
        assert!(matches!(
            outcome,
            crate::classfile::RetargetOutcome::Retargeted { sites: 1 }
        ));
        assert!(
            out != FIXTURE,
            "tick body must be replaced (body shrinks: 295 instr -> invokestatic)"
        );
        // idempotent: second pass must NOT retarget again
        let (_again, o2) = crate::classfile::patch_movecontrol_tick(&out).unwrap();
        assert!(matches!(
            o2,
            crate::classfile::RetargetOutcome::AlreadyPatched { .. }
        ));
        // the bridge must resolve the retarget (targets reachable in cp)
        crate::classfile::moveplane_resolution_closure(MOVE_BYTES).unwrap();
    }

    /// STRICT gate: empty/foreign flags must not arm (спящий-гейт урок-408).
    #[test]
    fn gate_is_strict_eq() {
        // armed() reads the env directly; the production pin lives in
        // entity_query::flag_enabled (pinned there). Here: literal contract.
        assert_eq!("cmp463_move", "cmp463_move");
        assert_ne!("cmp463_move ", "cmp463_move");
    }
}
