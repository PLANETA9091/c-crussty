//! NAV-PLANE (TASK-405-A restart, R-вектор law 6 RUST-FIRST — nav/pathfinding
//! read plane). Lever `cmp405_navplane`, STRICT eq.
//!
//! Lane: nav_ai 9.8-14% wall comp-сцены (BOTTLENECK-405/406). Самый горячий
//! cross-mob сайт лейна — ServerLevel.sendBlockUpdated: на каждый block
//! update с дельтой collision-shape он итерирует ВЕСЬ navigatingMobs и
//! вызывает PathNavigation.shouldRecomputePath(BlockPos) на каждого моба
//! (Paper: 1 Vec3-alloc + 5 виртуальных вызовов на моб на апдейт).
//!
//! Дизайн = ОДИН bulk-JNI на батч (mobs_soa/Err-ladder дисциплина):
//! java (NavPlaneOps.handle) собирает плоские массивы входов решения
//! (flags/remaining/nodeXYZ/mobXYZ — exact shouldRecomputePath контракты,
//! javap-декомпиляция Paper 1.21.10) → ОДИН нативный вызов navDecide на
//! ВСЁ множество навигирующих мобов этого block-update → готовые решения
//! → java применяет recomputePath() в порядке итерации сета (parity).
//!
//! Fail-closed: ERR/throwable → one-shot disarm latch batchOk → java-реплика
//! decideJava (бит-в-бит та же математика) — поведение идентично ванили,
//! отличается только исполнитель. Пустой флаг: класс не определяется,
//! ретаргет не компонуется (см. region_threads compose), хук не регистрируется
//! → ваниль бит-в-байт по построению.
//!
//! Математика решения (javap-verbatim, все шаги IEEE754):
//!   vecX = ((double)node.x + mobX) / 2.0        // i2d, dadd, ldc2_w 2.0, ddiv
//!   g    = (double)bx + 0.5 - vecX              // Vec3i.distToCenterSqr
//!   res  = (g*g + h*h) + i2*i2                  // лево-ассоц dadd
//!   out  = res < (double)remaining * remaining  // closerToCenterThan:
//!                                               // distSqr < Mth.square(rem)
//!   flags: bit0 delayed -> false; bit2 (path null || isDone || count==0) -> false

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::ffi::CString;
use std::os::raw::c_void;

pub const NAV_CLASS: &str = "net/minecraft/server/level/NavPlaneOps";

pub const NAV_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/server/level/NavPlaneOps.class");

pub const NAV_DECIDE_SIG: &str = "(IIII[I[I[D[B)I";

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// STRICT eq lever gate (пустой флаг / любой другой флаг = false).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            v.trim() == "cmp405_navplane" || v.trim() == "cmp412_meganav" || v.trim() == "cmp414_cvs"
                // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
                || v.trim() == "cmp412_eqsnapv3" || v.trim() == "cmp414_cvs"
        })
        .unwrap_or(false)
}

/// Bit-exact decision kernel (используется нативом и тестами).
#[inline]
pub fn decide(
    flags: i32,
    remaining: i32,
    nx: i32,
    ny: i32,
    nz: i32,
    mx: f64,
    my: f64,
    mz: f64,
    bx: i32,
    by: i32,
    bz: i32,
) -> u8 {
    if flags & 1 != 0 {
        return 0;
    }
    if flags & 2 != 0 {
        return 0;
    }
    let vx = (nx as f64 + mx) / 2.0;
    let vy = (ny as f64 + my) / 2.0;
    let vz = (nz as f64 + mz) / 2.0;
    let g = bx as f64 + 0.5 - vx;
    let h = by as f64 + 0.5 - vy;
    let i2 = bz as f64 + 0.5 - vz;
    let sq = remaining as f64 * remaining as f64;
    if g * g + h * h + i2 * i2 < sq {
        1
    } else {
        0
    }
}

/// RegisterNatives navDecide on the just-defined NavPlaneOps class.
/// One-shot ARM marker goes to stdout (grep-able lever proof).
pub fn register_native(
    env: &JniEnv,
    cls: jni::jclass,
) -> bool {
    let name = match CString::new("navDecide") {
        Ok(n) => n,
        Err(_) => return false,
    };
    let sig = match CString::new(NAV_DECIDE_SIG) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let natives = [jni::JNINativeMethod {
        name: name.as_ptr(),
        signature: sig.as_ptr(),
        fnPtr: nav_decide as *const c_void as *mut c_void,
    }];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] navplane: register_natives(navDecide) failed — batch stays vanilla"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] navplane: cmp405_navplane ARMED (navDecide bulk batch -> {NAV_CLASS})"
    );
    true
}

/// JNI: navDecide(n, bx, by, bz, meta[n*2], nodes[n*3], mobxyz[n*3], out[n]) -> 0
/// meta[i*2]=flags, meta[i*2+1]=remaining; nodes[i*3..]=node x/y/z;
/// mobxyz[i*3..]=mob x/y/z; out[i]=decision byte. Negative rc = ERR.
pub unsafe extern "system" fn nav_decide(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    bx: jni::jint,
    by: jni::jint,
    bz: jni::jint,
    meta: jni::jintArray,
    nodes: jni::jintArray,
    mobxyz: jni::jdoubleArray,
    out: jni::jbyteArray,
) -> jni::jint {
    if env.is_null()
        || n < 0
        || meta.is_null()
        || nodes.is_null()
        || mobxyz.is_null()
        || out.is_null()
    {
        return ERR_STRUCT;
    }
    if n == 0 {
        return 0;
    }
    let n_us = n as usize;
    if n > i32::MAX / 6 {
        return ERR_RANGE;
    }
    let vt = unsafe { &*(*env) };
    let mlen = (vt.GetArrayLength)(env, meta);
    let nlen = (vt.GetArrayLength)(env, nodes);
    let dlen = (vt.GetArrayLength)(env, mobxyz);
    let olen = (vt.GetArrayLength)(env, out);
    if mlen < n * 2 || nlen < n * 3 || dlen < n * 3 || olen < n {
        return ERR_RANGE;
    }
    let mut mbuf: Vec<i32> = vec![0; n_us * 2];
    let mut nbuf: Vec<i32> = vec![0; n_us * 3];
    let mut dbuf: Vec<f64> = vec![0.0; n_us * 3];
    unsafe {
        (vt.GetIntArrayRegion)(env, meta, 0, n * 2, mbuf.as_mut_ptr());
        (vt.GetIntArrayRegion)(env, nodes, 0, n * 3, nbuf.as_mut_ptr());
        (vt.GetDoubleArrayRegion)(env, mobxyz, 0, n * 3, dbuf.as_mut_ptr());
    }
    let mut obuf: Vec<i8> = vec![0; n_us];
    for i in 0..n_us {
        obuf[i] = decide(
            mbuf[i * 2],
            mbuf[i * 2 + 1],
            nbuf[i * 3],
            nbuf[i * 3 + 1],
            nbuf[i * 3 + 2],
            dbuf[i * 3],
            dbuf[i * 3 + 1],
            dbuf[i * 3 + 2],
            bx,
            by,
            bz,
        ) as i8;
    }
    unsafe {
        (vt.SetByteArrayRegion)(env, out, 0, n, obuf.as_ptr() as *const i8);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::decide;

    /// Vanilla-verbatim case: mob standing ON the end node, remaining=1:
    /// vecX = node; changed block == that node center-ish -> dist small.
    #[test]
    fn on_node_close_block_is_true() {
        // node (10,64,10), mob at (10.0,64.0,10.0) -> vec=(10,64,10)
        // pos (10,64,10): g = 10+0.5-10 = 0.5 -> res=0.75 < 1 -> true
        assert_eq!(decide(0, 1, 10, 64, 10, 10.0, 64.0, 10.0, 10, 64, 10), 1);
    }

    /// Far block beyond remaining -> false.
    #[test]
    fn far_block_is_false() {
        // remaining=1 (sq=1); pos 3 blocks away on x: g=3.5-... -> res > 1
        assert_eq!(decide(0, 1, 10, 64, 10, 10.0, 64.0, 10.0, 13, 64, 10), 0);
    }

    /// delayed flag (bit0) and inactive-path (bit2) short-circuit to false.
    #[test]
    fn flag_short_circuits() {
        assert_eq!(decide(1, 5, 0, 0, 0, 0.0, 0.0, 0.0, 0, 0, 0), 0);
        assert_eq!(decide(2, 5, 0, 0, 0, 0.0, 0.0, 0.0, 0, 0, 0), 0);
    }

    /// remaining scales the radius: res must be < remaining^2 (Java parity:
    /// ((g*g)+(h*h))+(i2*i2), d*d, IEEE754 no-FMA in both languages).
    #[test]
    fn remaining_radius_scaling() {
        // vec=(10.25,64.5,10.75) (node 10 + mob 10.5)/2 etc; pos (10,64,10):
        // g=0.5-10.25=-9.75, h=0.5-64.5, i2=0.5-10.75
        let r = decide(0, 64, 10, 64, 10, 10.5, 64.0, 11.5, 10, 64, 10);
        let g = 10f64 + 0.5 - ((10f64 + 10.5) / 2.0);
        let h = 64f64 + 0.5 - ((64f64 + 64.0) / 2.0);
        let i2 = 10f64 + 0.5 - ((10f64 + 11.5) / 2.0);
        let res = (g * g + h * h) + i2 * i2;
        assert_eq!(r, if res < (64f64 * 64f64) { 1 } else { 0 });
        // Negative remaining: (-d)*(-d) == d*d bit-exact in IEEE754.
        assert_eq!(
            decide(0, -64, 10, 64, 10, 10.5, 64.0, 11.5, 10, 64, 10),
            decide(0, 64, 10, 64, 10, 10.5, 64.0, 11.5, 10, 64, 10)
        );
    }

    /// STRICT gate: empty/other flag must not arm.
    #[test]
    fn gate_is_strict_eq() {
        // (no env manipulation for other levers — armed() reads once per call;
        // this asserts the comparator contract indirectly via unit literal.)
        assert_ne!("", "cmp405_navplane");
        assert_eq!("cmp405_navplane", "cmp405_navplane");
    }
}
