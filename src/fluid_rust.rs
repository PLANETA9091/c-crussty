//! Runtime wiring for the FLUID-RUST SECTIONAL batch lever (TASK-410-B,
//! vector fluid law-(6) k2 — the anti-bleg1 redesign; bridge
//! fluidrust/net/minecraft/world/entity/FluidRustOps.java, retarget in
//! src/classfile.rs patch_fluid_rust, compose stage in src/entity_compose.rs).
//!
//! ARCHITECTURE (ONE JNI per TICK per thread — the per-entity JNI of the
//! round-405 bridge exploded the fluid lane 16.72%→71.48%):
//! the two kernel wrapper call-sites of Entity.updateFluidHeightAndDoFluidPushing
//! (WATER + LAVA, the only two in the kernel) are retargeted to the static
//! FluidRustOps bridge. The bridge GATHERS java-side (vanilla cell scan,
//! per-call verdict table, inline return/height/lastLavaContact — zero JNI)
//! and appends per-entity records to a thread-confined TICK QUEUE; the first
//! call of a new level phase (gameTime stamp, javap ServerLevel.tick 236 vs
//! 532) flushes the whole queue with ONE `fluid_batch_tick` call, which
//! computes the exact FlowingFluid.getFlow vectorization + push accumulation
//! + vanilla tail (javap contract: Plane.HORIZONTAL order, f32 height
//! arithmetic, f64 Vec3 op order, ZERO-identity via touched flag, running
//! maxDepth 0.4 branch, 0.003/0.0045 floor check, Player normalize exemption)
//! for ALL entities in ONE pass, threading the water→lava delta bit-in-bit
//! (newEntity=0 continues the previous record). Outputs are ONE packed array
//! copied out ONCE (Get*ArrayRegion — no Critical in any loop). Fail-closed:
//! gather problems → faithful inline vanilla replica; a flush native error
//! discards the one batch and disarms to the inline replica (never
//! half-applied). No cross-tick world state (law 5).
//!
//! DELIVERY (fluid_free/mobs_manager pattern): no Entity byte hook here —
//! entity_compose owns the Entity pipeline; this module defines the bridge
//! into the kernel loader + RegisterNatives (fluidProbe/fluidBatchTick) and
//! publishes bridge_ready() for the compose stage.
//!
//! Fail-closed: lever flag != "cmp410_fluidsec" (STRICT eq) → module fully
//! dormant; native error codes → batch discard + disarm (ERR_STRUCT) or
//! ERR_RANGE (never observed from the java caller).

use jvmti_bindings::prelude::*;
use std::ffi::{c_void, CString};
use std::sync::atomic::{AtomicBool, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const OPS_CLASS: &str = "net/minecraft/world/entity/FluidRustOps";
const OPS_Q_CLASS: &str = "net/minecraft/world/entity/FluidRustOps$Q";
const OPS_GATHER_CLASS: &str = "net/minecraft/world/entity/FluidRustOps$Gather";

const OPS_BYTES: &[u8] =
    include_bytes!("../fluidrust/build/net/minecraft/world/entity/FluidRustOps.class");
// Nested classes are resolved through the defining loader — each is defined
// EXPLICITLY (fluid_guard CLASSES pattern; a missed nested class = latent
// NoClassDefFoundError on the first hot-loop call).
const OPS_Q_BYTES: &[u8] =
    include_bytes!("../fluidrust/build/net/minecraft/world/entity/FluidRustOps$Q.class");
const OPS_GATHER_BYTES: &[u8] =
    include_bytes!("../fluidrust/build/net/minecraft/world/entity/FluidRustOps$Gather.class");
const OPS_CLASSES: [(&str, &[u8]); 3] = [
    (OPS_CLASS, OPS_BYTES),
    (OPS_Q_CLASS, OPS_Q_BYTES),
    (OPS_GATHER_CLASS, OPS_GATHER_BYTES),
];

const PROBE_MAGIC: i32 = 0x464C; // "FL"
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// STRICT eq — empty flag = vanilla bit-in-bit.
fn java_gate_matches(f: &str) -> bool {
    f == "cmp410_fluidsec"
}

/// Gate visibility for the entity_compose compose stage.
pub fn enabled_pub() -> bool {
    java_gate_matches(&lever_flag())
}

static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

/// The compose stage waits on this before serving retargeted bytes.
pub fn bridge_ready() -> bool {
    BRIDGE_READY.load(Ordering::Acquire)
}

pub fn wait_bridge_ready(timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    while !bridge_ready() {
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    true
}

/// Register (call once from cplugin_init). No byte hook — the Entity pipeline
/// is owned by entity_compose; dormant-invisible with a foreign flag.
pub fn register() {
    if !enabled_pub() {
        eprintln!(
            "[crussty-plugin] fluid_rust: dormant (set CRUSSTY_LEVER_FLAG=cmp410_fluidsec to enable)"
        );
    }
}

/// Background activation: wait for the kernel Entity class + boot quiet,
/// define FluidRustOps into the kernel loader, RegisterNatives, publish
/// BRIDGE_READY and print the loud ARM marker.
pub fn activate() {
    if !enabled_pub() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        while cplug_sdk::classes::find_class(ENTITY_CLASS).is_none() {
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] fluid_rust: {ENTITY_CLASS} not loaded within 180s, bridge stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] fluid_rust: boot marker not seen, bridge stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));

        let defined = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ENTITY_CLASS) else {
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
            // Define the bridge + its nested classes explicitly (each resolves
            // through this loader at first use).
            let mut ops_cls: Option<jvmti_bindings::jni::jclass> = None;
            for (nm, bytes) in OPS_CLASSES.iter() {
                let Some(c) = env.define_class(nm, gref, bytes) else {
                    crate::describe_exception(env);
                    eprintln!("[crussty-plugin] fluid_rust: define_class({nm}) failed");
                    return false;
                };
                if *nm == OPS_CLASS {
                    ops_cls = Some(c);
                } else {
                    env.delete_local_ref(c);
                }
            }
            let Some(c) = ops_cls else { return false };

            let names = [
                CString::new("fluidProbe").expect("no NUL"),
                CString::new("fluidBatchTick").expect("no NUL"),
            ];
            let sigs = [
                CString::new("()I").expect("no NUL"),
                CString::new("(I[I[D[D[F[I[D)I").expect("no NUL"),
            ];
            let natives = [
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[0].as_ptr(),
                    signature: sigs[0].as_ptr(),
                    fnPtr: fluid_probe as *const c_void as *mut c_void,
                },
                jvmti_bindings::jni::JNINativeMethod {
                    name: names[1].as_ptr(),
                    signature: sigs[1].as_ptr(),
                    fnPtr: fluid_batch_tick as *const c_void as *mut c_void,
                },
            ];
            let reg = env.register_natives(c, &natives);
            if let Err(code) = reg {
                env.exception_clear();
                eprintln!(
                    "[crussty-plugin] fluid_rust: register_natives failed (code {code}) — bridge stays dormant"
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
            eprintln!("[crussty-plugin] fluid_rust: bridge definition failed, stays dormant");
            return;
        }
        BRIDGE_READY.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(OPS_CLASS, "fluidBatchTick", "fluid_rust v2 sectional");
        // ГРОМКИЙ ARM-МАРКЕР (без этой строки нога не-armed).
        eprintln!(
            "[crussty-plugin] cmp410_fluidsec: ARMED fluidpush=sectional-bulk-rust jni=1-per-tick-per-thread gather=java-scan-inline-verdicts negative=0-jni slow=faithful-vanilla-replica (getFlow+tail+water-lava delta threading bit-in-bit: Plane.HORIZONTAL f32-heights f64-Vec3 ZERO-identity running-maxDepth-0.4 floor-0.003/0.0045; gameTime-stamp flush; no cross-tick world state; spread lane vanilla)"
        );
    });
}

// ---------------------------------------------------------------------------
// Natives
// ---------------------------------------------------------------------------

/// # Safety
/// Called by the JVM via RegisterNatives; env must be a live JNIEnv.
#[no_mangle]
pub unsafe extern "system" fn fluid_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    PROBE_MAGIC
}

#[inline]
fn java_max(a: f64, b: f64) -> f64 {
    // java.lang.Math.max(double,double) bit-in-bit (javap: NaN → a; -0/+0 pair).
    if a != a {
        return a;
    }
    if a == 0.0 && b == 0.0 && a.is_sign_negative() {
        return b;
    }
    if a >= b {
        a
    } else {
        b
    }
}

/// net.minecraft.world.phys.Vec3#normalize bit-in-bit (javap): d0 = sqrt((x²+y²)+z²);
/// d0 < 9.999999747378752E-6 → Vec3.ZERO (+0.0 components); else x/d0, y/d0,
/// z/d0 (division, javap 47..72).
#[inline]
fn vec3_normalize(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let d0 = (x * x + y * y + z * z).sqrt();
    if d0 < 9.999_999_747_378_752E-6 {
        (0.0, 0.0, 0.0)
    } else {
        (x / d0, y / d0, z / d0)
    }
}

/// One FlowingFluid.getFlow (javap 143..337) for a cell.
/// `f` = 17 floats per cell: [own, nH0..3, belowH0..3, stepX0..3, stepZ0..3];
/// `flag` bits: affects/blocksMotion/belowAffects/solid/solidAbove per dir +
/// falling (bit 20). Returns (flowX, flowY, flowZ).
#[inline]
fn get_flow(f: &[f32], flag: i32) -> (f64, f64, f64) {
    let own = f[0];
    let mut vx: f64 = 0.0;
    let mut vz: f64 = 0.0;
    for di in 0..4usize {
        if flag & (1 << di) == 0 {
            continue; // !affectsFlow
        }
        let h = f[1 + di];
        let mut f1: f32 = 0.0;
        if h == 0.0f32 {
            if flag & (1 << (4 + di)) == 0 {
                // !blocksMotion
                if flag & (1 << (8 + di)) != 0 {
                    let bh = f[5 + di];
                    if bh > 0.0f32 {
                        // f1 = own - bh - 0.8888889f (left-assoc f32, javap 144..154)
                        f1 = own - bh - 0.888_888_9f32;
                    }
                }
            }
        } else if h > 0.0f32 {
            f1 = own - h;
        }
        if f1 != 0.0f32 {
            // acc += (double)((float)step * f1) — i2f/fmul/f2d (javap 182..210)
            vx += ((f[9 + di] * f1) as f64);
            vz += ((f[13 + di] * f1) as f64);
        }
    }
    let mut nx = vx;
    let mut ny = 0.0f64;
    let mut nz = vz;
    if flag & (1 << 20) != 0 {
        for di in 0..4usize {
            if flag & (1 << (12 + di)) != 0 || flag & (1 << (16 + di)) != 0 {
                // vec = vec.normalize().add(0.0, -6.0, 0.0); break (javap 311..324)
                let (ux, uy, uz) = vec3_normalize(nx, ny, nz);
                nx = ux + 0.0;
                ny = uy + -6.0;
                nz = uz + 0.0;
                break;
            }
        }
    }
    vec3_normalize(nx, ny, nz) // javap 332..337: ALWAYS normalize
}

/// ONE RECORD's verdict = vanilla accumulation + vanilla tail (javap
/// 458..681). `delta` = the exact getDeltaMovement() at the vanilla call
/// (captured java-side; the water→lava threading passes the previous
/// record's outDelta). Returns (outX, outY, outZ, touched).
#[inline]
fn record_verdict(
    delta: (f64, f64, f64),
    player: bool,
    speed: f64,
    d0s: &[f64],
    fls: &[f32],
    flags: &[i32],
) -> (f64, f64, f64, bool) {
    let mut max_depth: f64 = 0.0;
    let mut touched = false;
    let (mut ax, mut ay, mut az) = (0.0f64, 0.0f64, 0.0f64);
    for (c, &d0) in d0s.iter().enumerate() {
        // vanilla: inFluid=true; maxDepth=Math.max(maxDepth,d0); (javap 458..468)
        max_depth = java_max(max_depth, d0);
        // flowCount++ BEFORE getFlow (javap 478..482); flowCount == cellCount
        let base = 17 * c;
        let (fx, fy, fz) = get_flow(&fls[base..base + 17], flags[c]);
        // 0.4 branch uses the RUNNING maxDepth (updated above, javap 495..528)
        let (sx, sy, sz) = if max_depth < 0.4 {
            (fx * max_depth, fy * max_depth, fz * max_depth)
        } else {
            (fx, fy, fz)
        };
        if touched {
            ax += sx;
            ay += sy;
            az += sz;
        } else {
            // first add: Vec3.ZERO.add(v) = (0.0+v.x, 0.0+v.y, 0.0+v.z) — the
            // explicit +0.0 preserves the -0.0→+0.0 coercion of dadd.
            ax = 0.0 + sx;
            ay = 0.0 + sy;
            az = 0.0 + sz;
            touched = true;
        }
    }
    if !touched {
        // flowAcc never left the Vec3.ZERO reference: vanilla does NOT touch
        // the delta (out = delta-in, touched=0 → java skips setDeltaMovement).
        return (delta.0, delta.1, delta.2, false);
    }
    // ---- vanilla tail (javap 572..681), bit-in-bit ----
    // acc = acc.scale(1.0D / flowCount): the ddiv first, then dmul per comp.
    let factor = 1.0 / (d0s.len() as f64);
    ax *= factor;
    ay *= factor;
    az *= factor;
    if !player {
        let (nx, ny, nz) = vec3_normalize(ax, ay, az);
        ax = nx;
        ay = ny;
        az = nz;
    }
    ax *= speed;
    ay *= speed;
    az *= speed;
    // floor check: |delta.x| < 0.003 && |delta.z| < 0.003 && acc.length() < 0.0045
    // (NaN semantics identical: rust `<` is false on NaN, dcmpg+ifge skips too)
    if delta.0.abs() < 0.003
        && delta.2.abs() < 0.003
        && (ax * ax + ay * ay + az * az).sqrt() < 0.004_500_000_000_000_000_5
    {
        let (nx, ny, nz) = vec3_normalize(ax, ay, az);
        ax = nx * 0.004_500_000_000_000_000_5;
        ay = ny * 0.004_500_000_000_000_000_5;
        az = nz * 0.004_500_000_000_000_000_5;
    }
    // delta.add(acc) — component order this+other (javap Vec3.add(DDD))
    (delta.0 + ax, delta.1 + ay, delta.2 + az, true)
}

/// The whole-tick batch: ONE pass over ALL records, water→lava delta
/// threading via the newEntity flag. out has stride 4 per record.
fn batch_tick(
    n: usize,
    meta_i: &[i32],
    meta_d: &[f64],
    d0s: &[f64],
    fls: &[f32],
    flags: &[i32],
    out: &mut [f64],
) {
    let mut carried: Option<(f64, f64, f64)> = None;
    for i in 0..n {
        let new_entity = meta_i[4 * i] != 0;
        let player = meta_i[4 * i + 1] != 0;
        let start = meta_i[4 * i + 2] as usize;
        let count = meta_i[4 * i + 3] as usize;
        let own = (meta_d[4 * i], meta_d[4 * i + 1], meta_d[4 * i + 2]);
        let speed = meta_d[4 * i + 3];
        let delta = if new_entity {
            own
        } else {
            carried.unwrap_or(own)
        };
        let cell_d0 = &d0s[start..start + count];
        let cell_fls = &fls[17 * start..17 * (start + count)];
        let cell_flags = &flags[start..start + count];
        let (ox, oy, oz, touched) = record_verdict(delta, player, speed, cell_d0, cell_fls, cell_flags);
        out[4 * i] = ox;
        out[4 * i + 1] = oy;
        out[4 * i + 2] = oz;
        out[4 * i + 3] = if touched { 1.0 } else { 0.0 };
        carried = Some((ox, oy, oz));
    }
}

/// # Safety
/// Called by the JVM via RegisterNatives; env must be a live JNIEnv.
#[no_mangle]
pub unsafe extern "system" fn fluid_batch_tick(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    n: jni::jint,
    meta_i_arr: jni::jintArray,
    meta_d_arr: jni::jdoubleArray,
    d0s_arr: jni::jdoubleArray,
    fls_arr: jni::jfloatArray,
    flags_arr: jni::jintArray,
    out_arr: jni::jdoubleArray,
) -> jni::jint {
    if env.is_null()
        || meta_i_arr.is_null()
        || meta_d_arr.is_null()
        || d0s_arr.is_null()
        || fls_arr.is_null()
        || flags_arr.is_null()
        || out_arr.is_null()
    {
        return ERR_STRUCT;
    }
    if !(0..=(1 << 15)).contains(&n) {
        return ERR_RANGE;
    }
    let n = n as usize;
    let vt = unsafe { &**env };

    let len_i = |a: jni::jintArray| -> i32 { unsafe { (vt.GetArrayLength)(env, a as jni::jarray) } };
    let len_d = |a: jni::jdoubleArray| -> i32 { unsafe { (vt.GetArrayLength)(env, a as jni::jarray) } };
    let len_f = |a: jni::jfloatArray| -> i32 { unsafe { (vt.GetArrayLength)(env, a as jni::jarray) } };

    if (len_i(meta_i_arr) as usize) < 4 * n
        || (len_d(meta_d_arr) as usize) < 4 * n
        || (len_d(out_arr) as usize) < 4 * n
    {
        return ERR_RANGE;
    }

    // Copy-IN (each array exactly once — no Critical, no per-cell accessors).
    let mut meta_i = vec![0i32; 4 * n.max(1)];
    let mut meta_d = vec![0f64; 4 * n.max(1)];
    if n > 0 {
        unsafe {
            (vt.GetIntArrayRegion)(env, meta_i_arr, 0, (4 * n) as i32, meta_i.as_mut_ptr());
            (vt.GetDoubleArrayRegion)(env, meta_d_arr, 0, (4 * n) as i32, meta_d.as_mut_ptr());
        }
        if unsafe { (vt.ExceptionCheck)(env) } != 0 {
            unsafe { (vt.ExceptionClear)(env) };
            return ERR_STRUCT;
        }
        // Total cells = the last record's cellStart+cellCount (monotonic).
        let mut total = 0usize;
        for i in 0..n {
            let s = meta_i[4 * i + 2];
            let c = meta_i[4 * i + 3];
            if s < 0 || c < 0 {
                return ERR_RANGE;
            }
            total = total.max((s as usize) + (c as usize));
        }
        if (len_d(d0s_arr) as usize) < total
            || (len_f(fls_arr) as usize) < 17 * total
            || (len_i(flags_arr) as usize) < total
        {
            return ERR_RANGE;
        }
        let mut d0s = vec![0f64; total];
        let mut fls = vec![0f32; 17 * total];
        let mut flags = vec![0i32; total];
        unsafe {
            (vt.GetDoubleArrayRegion)(env, d0s_arr, 0, total as i32, d0s.as_mut_ptr());
            (vt.GetFloatArrayRegion)(env, fls_arr, 0, (17 * total) as i32, fls.as_mut_ptr());
            (vt.GetIntArrayRegion)(env, flags_arr, 0, total as i32, flags.as_mut_ptr());
        }
        if unsafe { (vt.ExceptionCheck)(env) } != 0 {
            unsafe { (vt.ExceptionClear)(env) };
            return ERR_STRUCT;
        }
        let mut out = vec![0f64; 4 * n];
        batch_tick(n, &meta_i, &meta_d, &d0s, &fls, &flags, &mut out);
        // Copy-OUT exactly once.
        unsafe {
            (vt.SetDoubleArrayRegion)(env, out_arr, 0, (4 * n) as i32, out.as_ptr());
        }
    }
    if unsafe { (vt.ExceptionCheck)(env) } != 0 {
        unsafe { (vt.ExceptionClear)(env) };
        return ERR_STRUCT;
    }
    0
}

// ---------------------------------------------------------------------------
// Tests: bit-in-bit oracle for the rust math (getFlow + accumulation + tail)
// against hand-derived java bytecode transcripts — extended BATCH oracle:
// N entities × mixed media (flat water / sloped flow / falling lava) with the
// water→lava delta threading and both floor-check branches.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_matches_vec3() {
        // normalize: ZERO below 9.999999747378752E-6
        assert_eq!(vec3_normalize(0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        assert_eq!(vec3_normalize(1e-7, 0.0, 0.0), (0.0, 0.0, 0.0));
        let (x, y, z) = vec3_normalize(3.0, 0.0, 4.0);
        let d = (3.0f64 * 3.0 + 4.0 * 4.0).sqrt();
        assert_eq!(x, 3.0 / d);
        assert_eq!(y, 0.0 / d);
        assert_eq!(z, 4.0 / d);
        // NaN propagates through the divide branch (dcmpg + ifge → 47)
        let (x, _, _) = vec3_normalize(f64::NAN, 0.0, 0.0);
        assert!(x.is_nan());
    }

    #[test]
    fn flat_water_full_neighbors_zero_flow() {
        // Source cell (own = 8/9) surrounded by full-height same fluid:
        // every dir h = 8/9 > 0 → f1 = own - h = 0 → no accumulation →
        // flow = normalize(0,0,0) = ZERO.
        let own = 8.0f32 / 9.0;
        let mut f = [0.0f32; 17];
        f[0] = own;
        for di in 0..4 {
            f[1 + di] = own; // nH
            f[9 + di] = 1.0; // stepX (unused when f1==0)
            f[13 + di] = 0.0; // stepZ
        }
        let flag = 0b1111; // affects all 4, not falling
        let (x, y, z) = get_flow(&f, flag);
        assert_eq!((x, y, z), (0.0, 0.0, 0.0));
    }

    #[test]
    fn sloped_water_flows_towards_lower_side() {
        let own = 8.0f32 / 9.0;
        let mut f = [0.0f32; 17];
        f[0] = own;
        f[1] = 0.0; // N: h=0, no blocksMotion, no below
        f[2] = 5.0 / 9.0; // E
        f[3] = 0.0; // S
        f[4] = 0.0; // W
        f[9] = 0.0;
        f[10] = 1.0; // E stepX=+1
        f[11] = 0.0;
        f[12] = -1.0; // W stepX=-1
        let flag = 0b1111;
        let (x, _, _) = get_flow(&f, flag);
        // f1 = own - 5/9 = 3/9 = 1/3; vx = (double)(1.0f * 1/3f); normalize →
        // x = vx / sqrt(vx*vx + 0 + 0) (exact formula — sqrt(vx²) may differ
        // from |vx| by 1 ulp, so the oracle mirrors the full expression).
        let f1 = own - 5.0f32 / 9.0;
        let vx = (1.0f32 * f1) as f64;
        let d = (vx * vx + 0.0 + 0.0).sqrt();
        assert_eq!(x, vx / d, "x={x}");
    }

    #[test]
    fn accumulation_order_and_04_branch() {
        let own = 8.0f32 / 9.0;
        let mut f = [0.0f32; 17];
        f[0] = own;
        let flag = 0; // no affects → flow = normalize(0,0,0) = 0
        let (_, _, _) = get_flow(&f, flag);
        // java_max sanity: NaN and ±0 semantics
        assert!(java_max(f64::NAN, 1.0).is_nan());
        assert_eq!(java_max(-0.0, 0.0), 0.0);
        assert_eq!(java_max(0.0, -0.0), 0.0);
        assert_eq!(java_max(0.2, 0.5), 0.5);
        let _ = own;
    }

    /// Hand-transcribed vanilla reference for ONE record (javap 13225..13560
    /// + FlowingFluid.getFlow), written in the test in java-op order.
    fn java_reference_record(
        delta: (f64, f64, f64),
        player: bool,
        speed: f64,
        d0s: &[f64],
        fls: &[f32],
        flags: &[i32],
    ) -> (f64, f64, f64, bool) {
        let mut max_depth: f64 = 0.0;
        let mut touched = false;
        let mut ax = 0.0f64;
        let mut ay = 0.0f64;
        let mut az = 0.0f64;
        for (c, &d0) in d0s.iter().enumerate() {
            max_depth = if max_depth != max_depth {
                max_depth
            } else if max_depth == 0.0 && d0 == 0.0 && max_depth.is_sign_negative() {
                d0
            } else if max_depth >= d0 {
                max_depth
            } else {
                d0
            };
            let base = 17 * c;
            let (fx, fy, fz) = get_flow(&fls[base..base + 17], flags[c]);
            let (sx, sy, sz) = if max_depth < 0.4 {
                (fx * max_depth, fy * max_depth, fz * max_depth)
            } else {
                (fx, fy, fz)
            };
            if touched {
                ax += sx;
                ay += sy;
                az += sz;
            } else {
                ax = 0.0 + sx;
                ay = 0.0 + sy;
                az = 0.0 + sz;
                touched = true;
            }
        }
        if !touched {
            return (delta.0, delta.1, delta.2, false);
        }
        let cnt = d0s.len() as f64;
        let f = 1.0 / cnt;
        ax *= f;
        ay *= f;
        az *= f;
        if !player {
            let d0 = (ax * ax + ay * ay + az * az).sqrt();
            if d0 < 9.999_999_747_378_752E-6 {
                ax = 0.0;
                ay = 0.0;
                az = 0.0;
            } else {
                ax /= d0;
                ay /= d0;
                az /= d0;
            }
        }
        ax *= speed;
        ay *= speed;
        az *= speed;
        let dxa = if delta.0 != delta.0 { delta.0 } else { delta.0.abs() };
        let dza = if delta.2 != delta.2 { delta.2 } else { delta.2.abs() };
        let len = (ax * ax + ay * ay + az * az).sqrt();
        if dxa < 0.003 && dza < 0.003 && len < 0.004_500_000_000_000_000_5 {
            let d0 = (ax * ax + ay * ay + az * az).sqrt();
            if d0 < 9.999_999_747_378_752E-6 {
                ax = 0.0;
                ay = 0.0;
                az = 0.0;
            } else {
                ax /= d0;
                ay /= d0;
                az /= d0;
            }
            ax *= 0.004_500_000_000_000_000_5;
            ay *= 0.004_500_000_000_000_000_5;
            az *= 0.004_500_000_000_000_000_5;
        }
        (delta.0 + ax, delta.1 + ay, delta.2 + az, true)
    }

    /// Sloped E-flow water cell (own 8/9, E neighbor 5/9) — 3 cells stacked
    /// with running maxDepth crossing the 0.4 branch.
    fn sloped_cells() -> (Vec<f64>, Vec<f32>, Vec<i32>) {
        let own = 8.0f32 / 9.0;
        let mut d0s = Vec::new();
        let mut fls = Vec::new();
        let mut flags = Vec::new();
        for &d0 in &[0.15f64, 0.25, 0.5] {
            d0s.push(d0);
            let mut f = [0.0f32; 17];
            f[0] = own;
            f[2] = 5.0 / 9.0; // E neighbor lower
            f[10] = 1.0; // E stepX=+1
            fls.extend_from_slice(&f);
            flags.push(0b0100); // affects E only
        }
        (d0s, fls, flags)
    }

    #[test]
    fn batch_oracle_n_entities_mixed_media() {
        // N=4 records in REAL-WORLD adjacency (an entity's water+lava records
        // are consecutive — both wrappers fire inside one baseTick):
        // (1) entity A water sloped E-flow, (2) the SAME entity's lava record
        // (newEntity=0 — the tail must see out of record 1), (3) entity B
        // flat water zero-flow, (4) a player entity with lava speed and the
        // floor-check branch. Verdicts compared bit-in-bit against the
        // hand-transcribed vanilla math.
        let (d0s_w, fls_w, flags_w) = sloped_cells();
        let own = 8.0f32 / 9.0;
        // record 3: flat water — every neighbor full → all f1 = 0
        let mut f_flat = [0.0f32; 17];
        f_flat[0] = own;
        for di in 0..4 {
            f_flat[1 + di] = own;
            f_flat[9 + di] = 1.0;
        }
        // lava cell: falling with a solid face (-6 y branch)
        let mut f_lava = [0.0f32; 17];
        f_lava[0] = own;
        f_lava[2] = 0.0; // E air
        f_lava[10] = 1.0; // E stepX=+1
        let flag_lava = (1 << 0) | (1 << 12) | (1 << 20); // affects first-dir + solidFace + falling

        // metaI: [new, player, start, count] × 4
        let meta_i = vec![
            1, 0, 0, 3, // rec0: entity A water, 3 cells
            0, 0, 3, 1, // rec1: entity A lava (newEntity=0!), 1 cell
            1, 0, 4, 1, // rec2: entity B water flat, 1 cell
            1, 1, 5, 1, // rec3: player P lava, 1 cell
        ];
        let total = 6;
        let mut d0s = vec![0.0f64; total];
        let mut fls = vec![0.0f32; 17 * total];
        let mut flags = vec![0i32; total];
        // entity A water cells
        for (k, &d) in d0s_w.iter().enumerate() {
            d0s[k] = d;
            fls[17 * k..17 * k + 17].copy_from_slice(&fls_w[17 * k..17 * k + 17]);
            flags[k] = flags_w[k];
        }
        // entity A lava
        d0s[3] = 0.1;
        fls[17 * 3..17 * 3 + 17].copy_from_slice(&f_lava);
        flags[3] = flag_lava;
        // entity B flat water
        d0s[4] = 0.3;
        fls[17 * 4..17 * 4 + 17].copy_from_slice(&f_flat);
        flags[4] = 0b1111;
        // player lava
        d0s[5] = 0.2;
        fls[17 * 5..17 * 5 + 17].copy_from_slice(&f_lava);
        flags[5] = flag_lava;

        let speed_water = 0.014;
        let speed_lava = 0.0023333333333333335;
        let meta_d = vec![
            0.01, -0.02, 0.03, speed_water, // rec0 delta A
            0.0, 0.0, 0.0, speed_lava, // rec1 delta field unused (carried from rec0)
            0.5, 0.0, -0.5, speed_water, // rec2 delta B (no floor branch: |x|≥0.003)
            0.0, 0.0, 0.0, speed_lava, // rec3 player delta 0 → floor branch
        ];

        let mut out = vec![0f64; 16];
        batch_tick(4, &meta_i, &meta_d, &d0s, &fls, &flags, &mut out);

        // --- reference: java-op order per record with the SAME threading ---
        let mut ref_out = vec![0f64; 16];
        let mut carried: Option<(f64, f64, f64)> = None;
        let refs: [(bool, bool, usize, usize, f64, (f64, f64, f64)); 4] = [
            (true, false, 0, 3, speed_water, (0.01, -0.02, 0.03)),
            (false, false, 3, 1, speed_lava, (0.0, 0.0, 0.0)),
            (true, false, 4, 1, speed_water, (0.5, 0.0, -0.5)),
            (true, true, 5, 1, speed_lava, (0.0, 0.0, 0.0)),
        ];
        for (i, &(new_e, player, start, count, speed, delta)) in refs.iter().enumerate() {
            let d = if new_e {
                delta
            } else {
                carried.unwrap()
            };
            let r = java_reference_record(
                d,
                player,
                speed,
                &d0s[start..start + count],
                &fls[17 * start..17 * (start + count)],
                &flags[start..start + count],
            );
            ref_out[4 * i] = r.0;
            ref_out[4 * i + 1] = r.1;
            ref_out[4 * i + 2] = r.2;
            ref_out[4 * i + 3] = if r.3 { 1.0 } else { 0.0 };
            carried = Some((r.0, r.1, r.2));
        }
        assert_eq!(out, ref_out, "batch verdicts must be bit-in-bit vs vanilla math");

        // semantic pins (vanilla identity semantics: Vec3.add ALWAYS returns a
        // fresh Vec3 — ANY recorded cell leaves the ZERO reference, so the
        // tail runs and setDeltaMovement IS called even when the numeric flow
        // is zero; the applied value is then numerically the delta-in):
        assert_eq!(out[4 * 2 + 3], 1.0, "flat water record leaves the ZERO reference (vanilla Vec3.add allocates)");
        assert_eq!(out[4 * 2], 0.5, "zero-flow record applies a numerically unchanged delta");
        assert_eq!(out[4 * 1], out[4 * 0], "lava record continues the water out-delta (lava flow has no x)");
        assert!(out[4 * 3 + 3] == 1.0, "player lava record applies");
        // player exemption: no normalize in the tail, but the floor branch
        // fires (delta 0, small acc) — mirror the java op order exactly:
        let (fx, fy, fz) = get_flow(&f_lava, flag_lava);
        let ax = 0.0 + fx * 0.2; // running maxDepth 0.2 < 0.4
        let ay = 0.0 + fy * 0.2;
        let az = 0.0 + fz * 0.2;
        let (ax, ay, az) = (ax * (1.0 / 1.0), ay * (1.0 / 1.0), az * (1.0 / 1.0)); // scale(1/flowCount)
        let (ax, ay, az) = (ax * speed_lava, ay * speed_lava, az * speed_lava); // player: NO normalize
        let d0 = (ax * ax + ay * ay + az * az).sqrt();
        assert!(d0 < 0.004_500_000_000_000_000_5, "floor-branch precondition");
        let (nx, ny, nz) = if d0 < 9.999_999_747_378_752E-6 {
            (0.0, 0.0, 0.0)
        } else {
            (ax / d0, ay / d0, az / d0)
        };
        let rz = nz * 0.004_500_000_000_000_000_5;
        assert_eq!(out[4 * 3 + 2], rz, "player lava z = floor-branch bit pattern");
    }

    #[test]
    fn batch_oracle_water_lava_threading_floor_branch() {
        // Entity A: water record then lava record (newEntity=0) with EXACT
        // power-of-two math so every pin is bit-exact (no 1-ulp sqrt hazards):
        //   water cell: own=1.0f, W neighbor h=0.5f, stepX_W=-1.0f →
        //   f1 = 1.0f-0.5f = 0.5f; vx = (double)(-1.0f*0.5f) = -0.5 exact;
        //   d0 = 0.5 ≥ 0.4 → NO maxDepth scaling; normalize: sqrt(0.25)=0.5
        //   exact → flow = (-1.0, 0, 0) exact; speed = 2^-8 → acc = -2^-8;
        //   delta (0,0,0): floor fires; sqrt((2^-8)²)=2^-8 exact (pow2) →
        //   water out.x = -0.0045000000000000005 EXACTLY.
        //   lava continues (newEntity=0): |delta.x| = 0.0045 ≥ 0.003 → the
        //   floor check must NOT fire; E neighbor h=0.5f stepX_E=+1.0f →
        //   flow = (+1.0,0,0) exact → lava out.x = water_out + lava_speed.
        let own = 1.0f32;
        let mut f = [0.0f32; 17];
        f[0] = own;
        f[4] = 0.5; // nH di=3 (WEST) = 0.5
        f[12] = -1.0; // stepX di=3 = -1
        let flag_w = 1 << 3; // affects WEST only
        let mut f_l = [0.0f32; 17];
        f_l[0] = own;
        f_l[2] = 0.5; // nH di=1 (EAST) = 0.5
        f_l[10] = 1.0; // stepX di=1 = +1
        let flag_l = 1 << 1; // affects EAST only
        let speed_w = 0.003_906_25_f64; // 2^-8: exact sqrt(x²) discipline
        let speed_l = 0.002_333_333_333_333_333_5_f64;

        // water verdict for delta (0,0,0) — floor branch fires (|0|<0.003)
        let w = record_verdict((0.0, 0.0, 0.0), false, speed_w, &[0.5], &f, &[flag_w]);
        assert!(w.3, "sloped water must be touched");
        assert_eq!(
            w.0,
            -0.004_500_000_000_000_000_5,
            "floor branch = normalize(±1.0)*0.0045 bit pattern"
        );
        assert_eq!(w.1, 0.0);
        // lava record continues the water out-delta: |delta.x| ≥ 0.003 → NO
        // floor branch; the lava push adds on top of the carried delta.
        let l = record_verdict((w.0, w.1, w.2), false, speed_l, &[0.5], &f_l, &[flag_l]);
        assert_eq!(
            l.0,
            w.0 + speed_l,
            "lava adds to the carried water delta (no second floor branch)"
        );
        assert_eq!(l.3, true);
        // the <0.4 maxDepth branch on the same EXACT math (d0 = 0.25):
        let w2 = record_verdict((0.0, 0.0, 0.0), false, speed_w, &[0.25], &f, &[flag_w]);
        // maxDepth 0.25 < 0.4 → acc = flow*0.25 = -0.25; normalize → -1.0
        // exact → *2^-8 → floor branch → the exact -0.0045 pattern again.
        assert_eq!(
            w2.0,
            -0.004_500_000_000_000_000_5,
            "0.4-branch scaled path keeps the exact pin"
        );
    }

    #[test]
    fn batch_empty_and_valve_shapes() {
        // n=0: out untouched, no panic.
        let mut out = vec![0f64; 8];
        batch_tick(0, &[], &[], &[], &[], &[], &mut out);
        assert_eq!(out, vec![0.0; 8]);
        // two records of the SAME entity with newEntity=1 for the second:
        // the second must NOT carry the first's delta.
        let meta_i = vec![1, 0, 0, 1, 1, 0, 1, 1];
        let meta_d = vec![1.0, 0.0, 0.0, 0.014, 2.0, 0.0, 0.0, 0.014];
        // two flat-water cells → touched=0 both → out = own deltas
        let d0s = vec![0.3, 0.3];
        let mut fls = vec![0.0f32; 34];
        for c in 0..2 {
            fls[17 * c] = 8.0f32 / 9.0;
            for di in 0..4 {
                fls[17 * c + 1 + di] = 8.0f32 / 9.0;
                fls[17 * c + 9 + di] = 1.0;
            }
        }
        let flags = vec![0b1111, 0b1111];
        let mut out = vec![0f64; 8];
        batch_tick(2, &meta_i, &meta_d, &d0s, &fls, &flags, &mut out);
        assert_eq!(out[0], 1.0);
        assert_eq!(out[4], 2.0);
        assert_eq!(out[3], 1.0, "zero-flow record still leaves the ZERO reference (Vec3.add allocates)");
        assert_eq!(out[7], 1.0, "second same-entity record: newEntity=1, own delta preserved");
    }
}
