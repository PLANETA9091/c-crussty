//! Batch-dispatch bridge — amortize the ~115ns Java→JNI transition floor over
//! many kernel invocations (P500 NEXT item 3: "batch-API for the JNI floor").
//!
//! # Motivation
//!
//! P500 found ~40 plugin/loading groups pinned at the ~115ns JNI transition
//! floor (`results/P500_REPORT.md`): the kernel work itself is nanoseconds,
//! the transition dominates. Micro-optimizing the closed kernels is
//! pointless; the lever is ONE transition covering N kernel calls.
//!
//! This module adds a single extra native method on a dedicated bridge class
//! (the kernels themselves stay untouched — the same registered natives of
//! `MAIN_JNI_TABLE` are called through resolved function pointers):
//!
//! ```java
//! package crussty.batch;
//! public class PaperNativeBatchDispatch {
//!     public static native int run(int[] kernelIds, long[] args0, long[] args1,
//!                                  int[] argCounts, long[] outs, int[] outOffsets);
//! }
//! ```
//!
//! # Calling convention
//!
//! `run` executes `kernelIds.length` ops, op `i` being kernel
//! `kernelIds[i]` (see `batch_table::KERNELS` for the id table):
//!
//! **Kernel-policy gate** (kernel_policy::decide, enforced infra): before any
//! op runs, every id is checked against the policy verdict computed once per
//! process. A refused id (do-not-wire regression, or not explicitly proven)
//! aborts the whole batch with [`ERR_KERNEL_REFUSED`] — no op executes, `outs`
//! untouched (structural-error model, proposal §7). Table membership alone
//! never grants allowance; `kernel_policy::PROVEN_WINS` is the single source
//! of truth (see docs/BATCH_WIRING_PLAN.md).
//!
//! - `args0[i]` — scalar argument of op `i` (long on the wire; kernels of
//!   shape A receive it narrowed to `jint`, P500 kernels use small ints).
//! - `args1` — packed INPUT arena for kernels that read a `long[]`
//!   (shape B `([J[J)J`): op `i` consumes the `argCounts[i]` longs starting
//!   at the prefix sum of `argCounts[0..i]`. Kernels never mutate the arena
//!   visible to the caller: each op's slice is copied into a per-thread
//!   scratch array first (some closed kernels mutate their inputs — P500
//!   "FRESH ARGS" fairness rule).
//! - `argCounts[i]` — dual meaning, per shape:
//!   - shape A `(I[J)I`: OUTPUT capacity — op `i` may write at most
//!     `argCounts[i]` longs (guarded; kernel returns the count written).
//!   - shape B `([J[J)J`: INPUT length (see `args1` above); the op writes
//!     exactly one result long.
//! - `outs` / `outOffsets` — shared OUTPUT arena: op `i` writes its results
//!   at `outOffsets[i]` (shape A: `count` longs; shape B: 1 long).
//!   `outOffsets[i] + written` must stay within `outs.length` (guarded).
//!
//! Return value: number of ops executed (`kernelIds.length`) on success, or
//! a negative error code (see the `ERR_*` constants; codes
//! `<= ERR_KERNEL_THREW_BASE` encode the op index that threw a Java
//! exception: `op = -ret - 1_000_000`, exception left PENDING for the JVM).
//!
//! # Execution model (why two phases)
//!
//! The closed kernels are JNI natives: they call back into JNI
//! (`GetLongArrayElements`/`Release...`) internally. JNI forbids calling ANY
//! JNI function between `GetPrimitiveArrayCritical` and its release, and a
//! kernel doing its own array access while we hold a critical section on the
//! same thread is exactly that violation (HotSpot may deadlock or crash under
//! GC). Therefore:
//!
//! - **Phase 1 (per op)**: kernel is called with a per-thread GLOBAL-REF
//!   scratch array as dst (`argCounts[i]`-long input slices are copied into a
//!   second scratch array first); the result longs are read out with
//!   `GetLongArrayRegion` into a staging buffer. No critical section held.
//! - **Phase 2 (once per batch)**: `GetPrimitiveArrayCritical(outs)` — pure
//!   `memcpy` of every op's staged results into the arena (NO JNI calls
//!   inside the critical window) — then `ReleasePrimitiveArrayCritical`.
//!
//! `GetPrimitiveArrayCritical` IS exposed by the vendored bindings
//! (jvmti-bindings 2.2.1, vtable index 222 — verified in the crate source), so
//! it is used. If it ever returns NULL (array too big / OOM inside the VM),
//! the code falls back to per-op `SetLongArrayRegion` copies — the
//! documented copy-semantics trade-off (2 VM memcpys per op instead of 1
//! critical + N memcpys; same asymptotics, slightly worse constant).
//!
//! # Per-op cost budget (vs an individual JNI call)
//!
//! individual: 1 Java→native transition (~115ns) + kernel-internal
//! GetLongArrayElements/Release. batch: 1 C call through a function pointer +
//! 1 `GetLongArrayRegion` (VM memcpy) + amortized critical memcpy. No
//! allocation, no local-ref churn (scratch arrays are per-thread global
//! refs; since TASK-24 the control planes live in the same per-thread
//! scratch — clear()-and-reuse at the capacity high-water mark).
//! `bench/batch/java/BatchFloorBench.java` (via
//! `bench/batch/run_batch_floor.sh`) measures the dispatcher delta at
//! batch sizes 1/8/16/64/256.
//!
//! # Errors and safety
//!
//! All array indices/lengths are validated in Rust BEFORE any write; a kernel
//! throwing clears nothing — the exception stays pending and the batch
//! returns the op-index error code (partial results ARE flushed to `outs`).
//! The scratch arrays are per-thread (two threads never share scratch); their
//! global refs leak on thread death by design (bounded: ~32KB/thread).
//!
//! Only vetted small-output kernels (<= `OUT_SCRATCH_CAP` longs) are listed
//! in `batch_table::KERNELS`; a kernel writing more than the scratch capacity
//! would be a closed-source bug we cannot guard against — hence the static
//! table instead of an open registry.

use crate::batch_table::{Shape, TABLE_VERSION, BATCH_KERNELS, KERNEL_COUNT};
use crate::loader;
use jvmti_bindings::prelude::*;
use std::cell::RefCell;
use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::ffi::{c_void, CString};
use std::path::Path;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Public constants (bridge class contract; see batch_table module docs for
// the ready-to-paste jni_table.rs entry and the lib.rs wiring).
// ---------------------------------------------------------------------------

/// Bridge class internal name (dedicated package, bootstrap loader).
pub const BATCH_CLASS: &str = "crussty/batch/PaperNativeBatchDispatch";
/// The single batch entry point.
pub const RUN_METHOD: &str = "run";
/// `run(int[] kernelIds, long[] args0, long[] args1, int[] argCounts, long[] outs, int[] outOffsets) -> int`
pub const RUN_SIG: &str = "([I[J[J[I[J[I)I";
/// Our own cdylib export (NOT in libpaper_native_jni.so). Contract
/// documentation: the `#[unsafe(no_mangle)]` fn below must keep this name.
#[allow(dead_code)]
pub const RUN_SYMBOL: &str = "Java_crussty_batch_PaperNativeBatchDispatch_run";
/// ABI guard (A7 §4.5): returns `(TABLE_VERSION << 16) | KERNEL_COUNT`.
pub const ABI_METHOD: &str = "abiVersion";
pub const ABI_SIG: &str = "()I";
/// Contract documentation: the `#[unsafe(no_mangle)]` fn below must keep this name.
#[allow(dead_code)]
pub const ABI_SYMBOL: &str = "Java_crussty_batch_PaperNativeBatchDispatch_abiVersion";

/// Fn-pointer table not resolved (and self-init failed) — call `init` first.
pub const ERR_NOT_INITIALIZED: i32 = -1;
/// One of the six array arguments is null.
pub const ERR_NULL_ARRAY: i32 = -2;
/// `kernelIds[i]` outside `0..KERNEL_COUNT`.
pub const ERR_BAD_KERNEL_ID: i32 = -3;
/// A per-op array is shorter than `kernelIds`, or the `args1` arena cannot
/// hold the packed shape-B input slices.
pub const ERR_LENGTH_MISMATCH: i32 = -4;
/// Op would write outside its `outOffsets[i] .. +argCounts[i]` window in
/// `outs` (or the kernel reports more longs than the scratch can hold).
pub const ERR_OUTPUT_CAPACITY: i32 = -5;
/// Shape-B input slice longer than `IN_SCRATCH_CAP`.
pub const ERR_INPUT_CAPACITY: i32 = -6;
/// A Java exception was already pending on entry (nothing was touched).
pub const ERR_PENDING_EXCEPTION: i32 = -7;
/// Kernel symbols could not be resolved (native lib missing/broken).
pub const ERR_NO_NATIVE_LIB: i32 = -8;
/// Per-thread scratch arrays could not be created (OOM in the VM).
pub const ERR_NO_SCRATCH: i32 = -9;
/// At least one `kernelIds[i]` references a kernel REFUSED by the
/// kernel-selection policy (`kernel_policy::decide`: do-not-wire regression
/// or not explicitly proven). NO ops were executed (structural-error model,
/// proposal §7: no partial execution) and `outs` was not touched.
pub const ERR_KERNEL_REFUSED: i32 = -10;
/// Base for kernel-threw codes: ret = `ERR_KERNEL_THREW_BASE - op_index`,
/// decode `op = -ret - 1_000_000`. The exception is left pending for the JVM.
pub const ERR_KERNEL_THREW_BASE: i32 = -1_000_000;

// ---------------------------------------------------------------------------
// Kernel function-pointer types (exact C ABI of the closed .so exports).
// ---------------------------------------------------------------------------

/// Shape A — `(I[J)I`: `jint Java_...(JNIEnv*, jclass, jint, jlongArray)`.
pub type ShapeAFn = unsafe extern "system" fn(
    env: *mut jni::JNIEnv,
    clazz: jni::jclass,
    scalar: jni::jint,
    dst: jni::jlongArray,
) -> jni::jint;

/// Shape B — `([J[J)J`: `jlong Java_...(JNIEnv*, jclass, jlongArray, jlongArray)`.
pub type ShapeBFn = unsafe extern "system" fn(
    env: *mut jni::JNIEnv,
    clazz: jni::jclass,
    src: jni::jlongArray,
    dst: jni::jlongArray,
) -> jni::jlong;

/// Scratch capacity for kernel OUTPUT (longs). P500 stubs allocate
/// `long[64]` as dst — the table only lists kernels with outputs within it.
const OUT_SCRATCH_CAP: usize = 64;
/// Scratch capacity for shape-B INPUT slices (longs, 32KB).
const IN_SCRATCH_CAP: usize = 4096;

/// Resolved kernel entry: typed per shape (no per-call transmute).
#[derive(Clone, Copy)]
enum KernelFn {
    A(ShapeAFn),
    B(ShapeBFn),
}

/// Kernel symbols resolved ONCE (init or first-call self-init).
static FNS: OnceLock<[KernelFn; KERNEL_COUNT]> = OnceLock::new();
/// Per-kernel kernel-policy verdict (computed ONCE from
/// `kernel_policy::decide(class, method)`; true = batch dispatch may execute
/// the kernel). Table membership does NOT imply allowance: the policy
/// registry is the single source of truth, so a future batch-table edit that
/// adds a do-not-wire kernel is refused at dispatch time (and caught by the
/// `kernel_policy` drift-guard test at CI time).
static POLICY_ALLOWED: OnceLock<[bool; KERNEL_COUNT]> = OnceLock::new();

/// The policy verdict per table id. Pure, allocation-free (decide() scans
/// static slices); called exactly once per process via `OnceLock`.
fn policy_flags() -> [bool; KERNEL_COUNT] {
    let mut flags = [false; KERNEL_COUNT];
    for k in BATCH_KERNELS {
        flags[k.id as usize] =
            crate::kernel_policy::decide(k.class, k.method).is_allowed();
    }
    flags
}
/// Per-kernel bridge-class global refs (A7 §4.4: hand each kernel the same
/// `jclass` it was registered against). Index 0 = unresolved → fall back to
/// the batch bridge class / invoking class.
static CLASSES: OnceLock<[usize; KERNEL_COUNT]> = OnceLock::new();
/// Global ref of the batch bridge class — the default `jclass` argument and
/// the fallback for kernels whose own bridge class could not be resolved.
static BATCH_CLASS_GREF: AtomicUsize = AtomicUsize::new(0);

/// Per-thread scratch: two global-ref arrays + a readback buffer + the
/// reusable batch control planes (TASK-24). Global refs survive across
/// native frames (local refs would dangle); per-thread because the closed
/// kernels keep per-call state in the dst array.
///
/// Control-plane discipline: `run()` copies every plane fresh from the wire
/// arrays on each call, so the `Vec`s carry NO state between batches (kernels
/// keep per-call state only in the global-ref arrays, never here) — reuse
/// cannot leak anything; it only keeps the allocation. Capacity follows a
/// high-water mark: a buffer only (re)grows when a larger batch arrives on
/// the same thread than any previous one, making the steady-state path
/// allocation-free.
struct Scratch {
    out_arr: jni::jlongArray, // OUT_SCRATCH_CAP longs — kernel dst
    in_arr: jni::jlongArray,  // IN_SCRATCH_CAP longs — shape-B src
    buf: Vec<jni::jlong>,     // GetLongArrayRegion readback target
    // ---- control planes, refilled from the wire arrays every run() ----
    ids: Vec<jni::jint>,      // kernelIds
    counts: Vec<jni::jint>,   // argCounts (A: out capacity, B: input length)
    offs: Vec<jni::jint>,     // outOffsets
    scalars: Vec<jni::jlong>, // args0
    in_starts: Vec<usize>,    // shape-B prefix sums over counts
    arena: Vec<jni::jlong>,   // packed shape-B input copy (args1 prefix)
    staging: Vec<jni::jlong>, // phase-1 results, scattered in phase 2
    ranges: Vec<(jni::jint, usize, jni::jint)>, // (out_off, staging_start, len)
}

thread_local! {
    static SCRATCH: RefCell<Option<Scratch>> = const { RefCell::new(None) };
}

// ---------------------------------------------------------------------------
// init — called from lib.rs::inject_surface (has an env + the open main lib).
// ---------------------------------------------------------------------------

/// Resolve every kernel symbol from `main_lib` once, define the
/// `crussty/batch/PaperNativeBatchDispatch` bridge class and register our
/// `run` export on it. Idempotent (second call is a no-op Ok).
///
/// # Safety
/// `env` must be a live JNI env of the CURRENT thread; `main_lib` must be the
/// dlopen'd `libpaper_native_jni.so`.
pub fn init(env: &JniEnv, main_lib: &loader::NativeLib) -> Result<(), String> {
    if FNS.get().is_some() {
        return Ok(());
    }
    let fns = resolve_fns(main_lib)?;
    if FNS.set(fns).is_err() {
        return Ok(()); // raced with another init — fine
    }
    if env.exception_check() {
        env.exception_clear();
        return Err("pending exception at batch init".into());
    }

    // Define the bridge class exactly like the surface loop does (two
    // natives: run + the ABI guard).
    let bytes = crate::bridge_class::bridge_class_bytes(
        BATCH_CLASS,
        &[(RUN_METHOD, RUN_SIG), (ABI_METHOD, ABI_SIG)],
    );
    let Some(cls) = env.define_class(BATCH_CLASS, ptr::null_mut(), &bytes) else {
        env.exception_clear();
        return Err(format!("define_class({BATCH_CLASS}) failed"));
    };

    let names = [
        CString::new(RUN_METHOD).expect("static name has no NUL"),
        CString::new(ABI_METHOD).expect("static name has no NUL"),
    ];
    let sigs = [
        CString::new(RUN_SIG).expect("static sig has no NUL"),
        CString::new(ABI_SIG).expect("static sig has no NUL"),
    ];
    let natives = [
        jni::JNINativeMethod {
            name: names[0].as_ptr(),
            signature: sigs[0].as_ptr(),
            fnPtr: Java_crussty_batch_PaperNativeBatchDispatch_run as *const c_void as *mut c_void,
        },
        jni::JNINativeMethod {
            name: names[1].as_ptr(),
            signature: sigs[1].as_ptr(),
            fnPtr: Java_crussty_batch_PaperNativeBatchDispatch_abiVersion as *const c_void
                as *mut c_void,
        },
    ];
    let reg = env.register_natives(cls, &natives);

    // Keep the class as a global ref — it is the `clazz` argument handed to
    // every kernel call for the process lifetime.
    let gref = env.new_global_ref(cls);
    env.delete_local_ref(cls);

    if let Err(code) = reg {
        env.exception_clear();
        return Err(format!("register_natives(run) failed (code {code})"));
    }
    BATCH_CLASS_GREF.store(gref as usize, Ordering::Release);

    // Cache each kernel's OWN bridge class (best effort; unresolved entries
    // fall back to the batch class at dispatch time).
    let mut classes = [0usize; KERNEL_COUNT];
    for k in BATCH_KERNELS {
        if let Some(c) = env.find_class(k.class) {
            classes[k.id as usize] = env.new_global_ref(c) as usize;
            env.delete_local_ref(c);
        } else {
            env.exception_clear();
        }
    }
    let _ = CLASSES.set(classes);

    eprintln!(
        "[crussty-plugin] batch: {} kernels resolved, run() + abiVersion() registered on {BATCH_CLASS}",
        KERNEL_COUNT
    );

    // Diagnose policy refusals ONCE at init (regardless of mode): a refused
    // id makes every batch referencing it return ERR_KERNEL_REFUSED without
    // executing anything — the operator should see WHY at boot, not chase a
    // negative return code at runtime.
    let flags = POLICY_ALLOWED.get_or_init(policy_flags);
    for k in BATCH_KERNELS {
        if !flags[k.id as usize] {
            eprintln!(
                "[crussty-plugin] batch: kernel {}.{} REFUSED by kernel-policy — \
                 run() will return ERR_KERNEL_REFUSED ({ERR_KERNEL_REFUSED}) for batches referencing it",
                k.class, k.method
            );
        }
    }
    Ok(())
}

/// dlsym every kernel in the compile-time table and type it by shape.
fn resolve_fns(lib: &loader::NativeLib) -> Result<[KernelFn; KERNEL_COUNT], String> {
    let mut slots: [Option<KernelFn>; KERNEL_COUNT] = [None; KERNEL_COUNT];
    for k in BATCH_KERNELS {
        if k.sig != k.shape.sig() {
            return Err(format!("batch: kernel {} sig/shape mismatch", k.symbol));
        }
        let addr = lib
            .symbol(k.symbol)
            .ok_or_else(|| format!("batch: kernel symbol {} unresolved", k.symbol))?;
        let f = match k.shape {
            Shape::A => KernelFn::A(unsafe { std::mem::transmute::<*mut c_void, ShapeAFn>(addr) }),
            Shape::B => KernelFn::B(unsafe { std::mem::transmute::<*mut c_void, ShapeBFn>(addr) }),
            Shape::Z => {
                return Err(format!(
                    "batch: shape Z kernel {} not supported yet (reserved)",
                    k.symbol
                ))
            }
        };
        slots[k.id as usize] = Some(f);
    }
    if slots.iter().any(|s| s.is_none()) {
        return Err("batch: kernel id table has holes".into());
    }
    Ok(std::array::from_fn(|i| slots[i].expect("checked above")))
}

/// Standalone fallback (bench without the runtime / init race): dlopen the
/// main lib ourselves and resolve the kernels. Does NOT define the bridge
/// class — whoever bound us already has one; the `clazz` argument of `run`
/// is used as the kernel class argument instead.
fn self_init() -> Option<&'static [KernelFn; KERNEL_COUNT]> {
    if let Some(f) = FNS.get() {
        return Some(f);
    }
    let path = std::env::var("CRUSSTY_BATCH_NATIVE_LIB").ok().unwrap_or_else(|| {
        // Bundled name, same convention as lib.rs::native_lib_name.
        format!("{DLL_PREFIX}paper_native_jni{DLL_SUFFIX}")
    });
    let lib = unsafe { loader::NativeLib::new(Path::new(&path)) }.ok()?;
    let fns = resolve_fns(&lib).ok()?;
    // set(): loses the race politely; get() then returns the winner's table.
    FNS.set(fns).ok()?;
    FNS.get()
}

/// `clazz` to hand a kernel: its own bridge class (registered against), else
/// the batch bridge class, else the class this export was invoked on.
fn kernel_class_for(id: usize, invoking: jni::jclass) -> jni::jclass {
    let own = CLASSES.get().map(|c| c[id]).unwrap_or(0);
    if own != 0 {
        return own as jni::jclass;
    }
    match BATCH_CLASS_GREF.load(Ordering::Acquire) {
        0 => invoking,
        a => a as jni::jclass,
    }
}

/// ABI guard export: `(TABLE_VERSION << 16) | KERNEL_COUNT`. Callers that see
/// a different value than their helper expects must fall back to per-op calls
/// (the packed layout may have drifted).
///
/// # Safety
/// Standard JNI contract (no env access needed).
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_crussty_batch_PaperNativeBatchDispatch_abiVersion(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    ((TABLE_VERSION as jni::jint) << 16) | KERNEL_COUNT as jni::jint
}

/// Create the per-thread scratch arrays (global refs, zeroed).
fn create_scratch(env: &JniEnv) -> Option<Scratch> {
    let out_local = env.new_long_array(OUT_SCRATCH_CAP as jni::jsize)?;
    let in_local = env.new_long_array(IN_SCRATCH_CAP as jni::jsize)?;
    let zeros_out = [0 as jni::jlong; OUT_SCRATCH_CAP];
    env.set_long_array_region(out_local, 0, OUT_SCRATCH_CAP as jni::jsize, &zeros_out);
    let zeros_in = vec![0 as jni::jlong; IN_SCRATCH_CAP];
    env.set_long_array_region(in_local, 0, IN_SCRATCH_CAP as jni::jsize, &zeros_in);
    let out_arr = env.new_global_ref(out_local);
    env.delete_local_ref(out_local);
    let in_arr = env.new_global_ref(in_local);
    env.delete_local_ref(in_local);
    if out_arr.is_null() || in_arr.is_null() {
        return None;
    }
    Some(Scratch {
        out_arr,
        in_arr,
        buf: zeros_out.to_vec(),
        // Control planes start empty and grow on first use (the high-water
        // capacity is then retained across calls — the TASK-24 allocation-
        // free steady state); `Vec::new()` allocates nothing here.
        ids: Vec::new(),
        counts: Vec::new(),
        offs: Vec::new(),
        scalars: Vec::new(),
        in_starts: Vec::new(),
        arena: Vec::new(),
        staging: Vec::new(),
        ranges: Vec::new(),
    })
}

// ---------------------------------------------------------------------------
// The batch entry point (our cdylib export — see RUN_SYMBOL).
// ---------------------------------------------------------------------------

/// JNI export of `crussty/batch/PaperNativeBatchDispatch.run`.
///
/// # Safety
/// Standard JNI contract: `env`/arrays must come from the JVM's call; the six
/// array arguments must be non-null (guarded) and owned by the caller.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_crussty_batch_PaperNativeBatchDispatch_run(
    env: *mut jni::JNIEnv,
    clazz: jni::jclass,
    kernel_ids: jni::jintArray,
    args0: jni::jlongArray,
    args1: jni::jlongArray,
    arg_counts: jni::jintArray,
    outs: jni::jlongArray,
    out_offsets: jni::jintArray,
) -> jni::jint {
    if env.is_null() {
        return ERR_NOT_INITIALIZED;
    }
    let fns = match FNS.get() {
        Some(f) => f,
        None => match self_init() {
            Some(f) => f,
            None => return ERR_NO_NATIVE_LIB,
        },
    };
    let vtable = unsafe { *env };
    let vt = unsafe { &*vtable };

    if kernel_ids.is_null()
        || args0.is_null()
        || args1.is_null()
        || arg_counts.is_null()
        || outs.is_null()
        || out_offsets.is_null()
    {
        return ERR_NULL_ARRAY;
    }
    // An exception pending on entry is the caller's business — leave it.
    if unsafe { (vt.ExceptionCheck)(env) } != 0 {
        return ERR_PENDING_EXCEPTION;
    }

    // ---- lengths & in-bounds validation (all reads stay in [0..n)) ----
    let n = unsafe { (vt.GetArrayLength)(env, kernel_ids) };
    if n < 0 {
        return ERR_LENGTH_MISMATCH;
    }
    let n = n as usize;
    if n == 0 {
        return 0;
    }
    let ni = n as jni::jsize;
    if unsafe { (vt.GetArrayLength)(env, args0) } < n as i32
        || unsafe { (vt.GetArrayLength)(env, arg_counts) } < n as i32
        || unsafe { (vt.GetArrayLength)(env, out_offsets) } < n as i32
    {
        return ERR_LENGTH_MISMATCH;
    }
    let outs_len = unsafe { (vt.GetArrayLength)(env, outs) };
    let args1_len = unsafe { (vt.GetArrayLength)(env, args1) };
    if outs_len < 0 || args1_len < 0 {
        return ERR_LENGTH_MISMATCH;
    }
    let (outs_len, args1_len) = (outs_len as usize, args1_len as usize);

    SCRATCH.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            // The export receives a raw env; create_scratch wants the
            // wrapper type (same as with_attached in lib.rs — the env is
            // live for the duration of this call).
            let jenv = JniEnv::from_raw(env);
            match create_scratch(&jenv) {
                Some(s) => *slot = Some(s),
                None => return ERR_NO_SCRATCH,
            }
        }
        let scratch = slot.as_mut().expect("just filled");
        // TASK-24: every control plane below lives in the per-thread scratch
        // (see `Scratch`) — clear() + resize()/reserve() reuse at the
        // capacity high-water mark. Steady state (batch no larger than the
        // largest this thread has run): ZERO heap allocations for the whole
        // dispatcher. The `resize(n, 0)` zero-fill on a reused buffer is the
        // deliberate soundness choice over `set_len` on uninitialized memory
        // (the JNI region copies overwrite every element anyway; the residual
        // memset is ~36 B/op — nanoseconds).
        let Scratch { ids, counts, offs, scalars, in_starts, arena, staging, ranges, .. } =
            scratch;

        // ---- copy the control planes into Rust (O(n), no critical sections) ----
        ids.clear();
        ids.resize(n, 0);
        counts.clear();
        counts.resize(n, 0);
        offs.clear();
        offs.resize(n, 0);
        scalars.clear();
        scalars.resize(n, 0);
        unsafe {
            (vt.GetIntArrayRegion)(env, kernel_ids, 0, ni, ids.as_mut_ptr());
            (vt.GetIntArrayRegion)(env, arg_counts, 0, ni, counts.as_mut_ptr());
            (vt.GetIntArrayRegion)(env, out_offsets, 0, ni, offs.as_mut_ptr());
            (vt.GetLongArrayRegion)(env, args0, 0, ni, scalars.as_mut_ptr());
        }

        let allowed = POLICY_ALLOWED.get_or_init(policy_flags);
        for &id in ids.iter() {
            if id < 0 || id as usize >= KERNEL_COUNT {
                return ERR_BAD_KERNEL_ID;
            }
            // Kernel-policy gate: a refused id aborts the WHOLE batch before
            // any op runs (structural error, no partial execution — proposal
            // §7). Cost: one bool index per op; zero syscalls, zero allocation.
            if !allowed[id as usize] {
                return ERR_KERNEL_REFUSED;
            }
        }
        for &c in counts.iter() {
            if c < 0 {
                return ERR_OUTPUT_CAPACITY;
            }
        }

        // Shape-B input slices: packed prefix sums over argCounts, then one
        // region copy of exactly the used prefix of args1. The same pass
        // computes the EXACT staging upper bound: shape A may write at most
        // min(argCounts[i], OUT_SCRATCH_CAP) longs (both guarded before any
        // staging push below), shape B exactly one. Pre-sizing from this
        // bound replaces the old `n * 8` guess, so phase 1 cannot reallocate
        // mid-loop no matter what counts the kernels return.
        in_starts.clear();
        in_starts.resize(n, 0);
        let mut total_in = 0usize;
        let mut staging_cap = 0usize;
        for i in 0..n {
            if BATCH_KERNELS[ids[i] as usize].shape == Shape::B {
                in_starts[i] = total_in;
                total_in += counts[i] as usize;
                staging_cap += 1;
            } else {
                staging_cap += (counts[i] as usize).min(OUT_SCRATCH_CAP);
            }
        }
        if total_in > args1_len {
            return ERR_LENGTH_MISMATCH;
        }
        arena.clear();
        arena.resize(total_in, 0);
        if total_in > 0 {
            unsafe { (vt.GetLongArrayRegion)(env, args1, 0, total_in as jni::jsize, arena.as_mut_ptr()) };
        }

        // ---- phase 1: run the ops into staging (no critical sections) ----
        staging.clear();
        staging.reserve(staging_cap);
        // (out_offset, staging_start, len) per op, for the phase-2 scatter.
        ranges.clear();
        ranges.reserve(n);
        let mut ret = n as jni::jint;

        for i in 0..n {
            let id = ids[i] as usize;
            let off = offs[i];
            let cap = counts[i] as usize;
            let res_len: usize = match fns[id] {
                KernelFn::A(f) => {
                    if off < 0 || (off as usize) + cap > outs_len {
                        ret = ERR_OUTPUT_CAPACITY;
                        break;
                    }
                    let written = unsafe {
                        f(env, kernel_class_for(id, clazz), scalars[i] as jni::jint, scratch.out_arr)
                    };
                    // Kernels return the count written; clamp negatives to 0
                    // defensively (they are C ints from closed code).
                    let written = if written < 0 { 0 } else { written as usize };
                    if written > OUT_SCRATCH_CAP || written > cap {
                        ret = ERR_OUTPUT_CAPACITY;
                        break;
                    }
                    unsafe {
                        (vt.GetLongArrayRegion)(
                            env,
                            scratch.out_arr,
                            0,
                            written as jni::jsize,
                            scratch.buf.as_mut_ptr(),
                        );
                    }
                    staging.extend_from_slice(&scratch.buf[..written]);
                    written
                }
                KernelFn::B(f) => {
                    let start = in_starts[i];
                    let len = cap; // argCounts[i] = input length for shape B
                    if len > IN_SCRATCH_CAP {
                        ret = ERR_INPUT_CAPACITY;
                        break;
                    }
                    if off < 0 || (off as usize) + 1 > outs_len {
                        ret = ERR_OUTPUT_CAPACITY;
                        break;
                    }
                    unsafe {
                        (vt.SetLongArrayRegion)(
                            env,
                            scratch.in_arr,
                            0,
                            len as jni::jsize,
                            arena[start..start + len].as_ptr(),
                        );
                    }
                    let res =
                        unsafe { f(env, kernel_class_for(id, clazz), scratch.in_arr, scratch.out_arr) };
                    staging.push(res);
                    1
                }
            };
            ranges.push((off, staging.len() - res_len, res_len as jni::jint));

            // Per-op exception gate: a throwing kernel aborts the batch; the
            // exception stays pending, results so far are still flushed.
            if unsafe { (vt.ExceptionCheck)(env) } != 0 {
                ret = ERR_KERNEL_THREW_BASE - i as jni::jint;
                break;
            }
        }

        // ---- phase 2: scatter staging into outs (ONE critical window) ----
        if !ranges.is_empty() {
            let mut is_copy: jni::jboolean = 0;
            let crit =
                unsafe { (vt.GetPrimitiveArrayCritical)(env, outs, &mut is_copy) };
            if crit.is_null() {
                // Fallback: per-op region copies (documented trade-off).
                for &(off, start, len) in ranges.iter() {
                    unsafe {
                        (vt.SetLongArrayRegion)(
                            env,
                            outs,
                            off,
                            len,
                            staging[start..start + len as usize].as_ptr(),
                        );
                    }
                }
            } else {
                let dst = crit as *mut jni::jlong;
                // NO JNI calls below: critical sections must not call JNI.
                for &(off, start, len) in ranges.iter() {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            staging.as_ptr().add(start),
                            dst.add(off as usize),
                            len as usize,
                        );
                    }
                }
                unsafe { (vt.ReleasePrimitiveArrayCritical)(env, outs, crit, 0) };
            }
        }
        ret
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch_table::BATCH_KERNELS;

    /// Every kernel in the compile-time batch table must be ALLOWED by the
    /// kernel-selection policy — otherwise the shipped batch surface is dead
    /// (every batch returns ERR_KERNEL_REFUSED). This is the runtime mirror
    /// of the `kernel_policy` drift-guard test; both must stay in sync.
    #[test]
    fn policy_allows_every_batch_table_kernel() {
        let flags = policy_flags();
        for k in BATCH_KERNELS {
            assert!(
                flags[k.id as usize],
                "batch kernel {}.{} is REFUSED by kernel-policy — the batch surface is dead; \
                 either add an honest PROVEN_WINS entry (with evidence) or remove it from the table",
                k.class, k.method
            );
        }
    }

    /// The flags must be EXACTLY `decide(...)` for each id (no drift between
    /// policy_flags() and the policy itself, e.g. a copy-pasted list).
    #[test]
    fn policy_flags_match_decide_for_every_id() {
        let flags = policy_flags();
        for k in BATCH_KERNELS {
            let want = crate::kernel_policy::decide(k.class, k.method).is_allowed();
            assert_eq!(
                flags[k.id as usize], want,
                "flag drift for id {} ({}.{})",
                k.id, k.class, k.method
            );
        }
    }

    /// A hypothetical do-not-wire kernel must NOT be granted allowance by
    /// table membership: simulate by checking the policy directly for every
    /// DO_NOT_WIRE entry against every batch-table class (name-collision
    /// guard: `cachedSummary` on another class stays unproven, but a table
    /// row carrying the exact regressed pair must stay false).
    #[test]
    fn refused_registry_kernels_would_not_pass_the_gate() {
        for r in crate::kernel_policy::DO_NOT_WIRE {
            assert!(
                !crate::kernel_policy::decide(r.class, r.kernel).is_allowed(),
                "do-not-wire kernel {}.{} must stay refused",
                r.class, r.kernel
            );
            // And no batch-table row may carry this exact (class, kernel).
            assert!(
                !BATCH_KERNELS
                    .iter()
                    .any(|k| k.class == r.class && k.method == r.kernel),
                "batch table carries do-not-wire kernel {}.{}",
                r.class, r.kernel
            );
        }
    }

    /// Error-code contract: the refusal sentinel is distinct from every other
    /// structural code (callers branch on it — never alias).
    #[test]
    fn refusal_code_is_distinct_and_negative() {
        let codes = [
            ERR_NOT_INITIALIZED,
            ERR_NULL_ARRAY,
            ERR_BAD_KERNEL_ID,
            ERR_LENGTH_MISMATCH,
            ERR_OUTPUT_CAPACITY,
            ERR_INPUT_CAPACITY,
            ERR_PENDING_EXCEPTION,
            ERR_NO_NATIVE_LIB,
            ERR_NO_SCRATCH,
        ];
        assert!(ERR_KERNEL_REFUSED < 0);
        for c in codes {
            assert_ne!(ERR_KERNEL_REFUSED, c, "error code aliasing");
        }
        assert_ne!(ERR_KERNEL_REFUSED, ERR_KERNEL_THREW_BASE);
    }
}
