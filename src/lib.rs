//! crussty — the Crussty CE native surface as a c-plugin.
//!
//! The Crussty CE project ships 4 Rust crates exporting 283 `Java_*` JNI
//! symbols (waypoint/ore/jigsaw/ticket/area_map/noise/chunk-encode/...). Its
//! Java bridge classes are NOT part of the CE repo — so this plugin rebuilds
//! them: for every export we define a `public static native` bridge class in
//! the exact package the export name implies (bootstrap loader) and register
//! the resolved symbol via RegisterNatives. Result: the whole native surface
//! is callable from ANY Paper-family kernel without the fork.
//!
//! Timing: cplugin_init may not touch the JVM, so injection happens on a
//! background thread 3s after init (VM is up by then; define_class with a
//! null loader + RegisterNatives need no kernel classes).
//!
//! The kernel hot-path wirings (area_map update batching etc.) are separate
//! byte hooks on top of this surface — see the project docs.

mod area_map;
mod batch_api;
mod batch_table;
mod bridge_class;
mod classfile;
mod improved_noise;
mod jni_table;
mod kernel_policy;
mod loader;
mod promote_wire;
mod proto_blend_cache;

use cplug_abi::{CPluginApi, JavaVmPtr};
use jvmti_bindings::prelude::*;
use std::collections::HashMap;
use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::ffi::{c_char, c_void, CString};
use std::path::PathBuf;
use std::time::Duration;

const MAIN_LIB: &str = "paper_native_jni";
const CHUNK_LIB: &str = "paper_native_chunk_encode_jni";

// TASK-27 (hygiene): the former lib.rs-local REGRESSED_KERNEL_FALLBACKS /
// kernel_pref_conservative / kernel_pref_fallback duplicates are REMOVED —
// `kernel_policy` (registration_fallback) is the single source of truth for
// the conservative surface binding; two lists had already begun to drift
// (lib.rs carried a 5th entry the policy registry never adopted).

/// Bundled native library filename for this platform: Crussty CE ships
/// `libpaper_native_jni.so` on Linux; Windows builds produce
/// `paper_native_jni.dll` (no `lib` prefix on MSVC).
fn native_lib_name(base: &str) -> String {
    format!("{}{}{}", DLL_PREFIX, base, DLL_SUFFIX)
}

/// The single required export (cplug-abi contract).
///
/// # Safety
/// `api` must point at a valid CPluginApi owned by the agent, `vm` must be
/// the live JavaVM pointer handed to us, `options` a NUL-terminated string
/// owned by the agent for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cplugin_init(
    api: *const CPluginApi,
    vm: JavaVmPtr,
    _options: *const c_char,
) -> i32 {
    // H-01 (hardening audit A11): a panic unwinding across this C boundary
    // aborts the whole JVM. Recover, report, and signal failure to the agent.
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cplugin_init_impl(api, vm, _options)
    }));
    match r {
        Ok(rc) => rc,
        Err(_) => {
            eprintln!("[crussty-plugin] FATAL: cplugin_init panicked");
            -1
        }
    }
}

unsafe fn cplugin_init_impl(api: *const CPluginApi, vm: JavaVmPtr, _options: *const c_char) -> i32 {
    cplug_sdk::init(api, vm);
    cplug_sdk::classes::spawn_stats_dumper(); // TASK-45: gated on CRUSSTY_SDK_STATS (off by default)
    eprintln!("[crussty-plugin] cplugin_init: injecting Crussty CE native surface in background");
    area_map::register();
    improved_noise::register();
    proto_blend_cache::register();
    std::thread::spawn(inject_surface);
    0
}

/// Locate this plugin's directory via dladdr on our own entry symbol.
fn plugin_dir() -> Option<PathBuf> {
    #[repr(C)]
    struct DlInfo {
        dli_fname: *const c_char,
        dli_fbase: *mut c_void,
        dli_sname: *const c_char,
        dli_saddr: *mut c_void,
    }
    unsafe extern "C" {
        fn dladdr(addr: *const c_void, info: *mut DlInfo) -> i32;
    }
    let mut info = DlInfo {
        dli_fname: std::ptr::null(),
        dli_fbase: std::ptr::null_mut(),
        dli_sname: std::ptr::null(),
        dli_saddr: std::ptr::null_mut(),
    };
    let rc = unsafe { dladdr(cplugin_init as *const c_void, &mut info) };
    if rc == 0 || info.dli_fname.is_null() {
        return None;
    }
    let path = unsafe { std::ffi::CStr::from_ptr(info.dli_fname) }
        .to_str()
        .ok()?;
    Some(PathBuf::from(path).parent()?.to_path_buf())
}

/// Background injection worker: dlopen the bundled .so files, define every
/// bridge class, RegisterNatives every export, then fire one live proof call.
fn inject_surface() {
    std::thread::sleep(Duration::from_secs(3));
    let Some(dir) = plugin_dir() else {
        eprintln!("[crussty-plugin] cannot locate plugin dir (dladdr failed)");
        return;
    };

    // Bundled native libs live in <plugin>/native/ (a subdir without
    // module.json so the agent's plugin scan skips them).
    let native_dir = dir.join("native");
    let main_name = native_lib_name(MAIN_LIB);
    let chunk_name = native_lib_name(CHUNK_LIB);
    let main_so = if native_dir.join(&main_name).exists() {
        native_dir.join(&main_name)
    } else {
        dir.join(&main_name)
    };
    let chunk_so = if native_dir.join(&chunk_name).exists() {
        native_dir.join(&chunk_name)
    } else {
        dir.join(&chunk_name)
    };
    if !main_so.exists() {
        eprintln!(
            "[crussty-plugin] missing {main_so:?}: build crussty/native (release) and copy the {} here",
            native_lib_name(MAIN_LIB)
        );
        return;
    }
    let main = match unsafe { loader::NativeLib::new(&main_so) } {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[crussty-plugin] dlopen {} failed: {e}", native_lib_name(MAIN_LIB));
            return;
        }
    };
    let chunk = if chunk_so.exists() {
        match unsafe { loader::NativeLib::new(&chunk_so) } {
            Ok(l) => Some(l),
            Err(e) => {
                eprintln!("[crussty-plugin] dlopen {} failed (continuing without it): {e}", native_lib_name(CHUNK_LIB));
                None
            }
        }
    } else {
        None
    };

    eprintln!("[crussty-plugin] native libs: {main_so:?} (+ {chunk_so:?})");

    // TASK-53 promotion binding boot marker (grep-able, mirrors rollout).
    if kernel_policy::promotion_armed() {
        eprintln!(
            "[crussty-plugin] kernel_promote: CRUSSTY_KERNEL_PROMOTE armed — {} promotion pair(s) re-bind at registration",
            kernel_policy::PROMOTE_PAIRS.len()
        );
        kernel_policy::log_armed_pairs();
    }

    let mut n_classes = 0usize;
    let mut n_natives = 0usize;
    let mut n_missing = 0usize;
    let result = with_attached(|env| {
        eprintln!("[crussty-plugin] attached: injecting bridge classes");
        for (table, lib) in [
            (jni_table::MAIN_JNI_TABLE, Some(&main)),
            (jni_table::CHUNK_JNI_TABLE, chunk.as_ref()),
        ] {
            let Some(lib) = lib else { continue };
            let mut by_class: HashMap<&str, Vec<(&str, &str, &str)>> = HashMap::new();
            for e in table {
                by_class
                    .entry(e.class)
                    .or_default()
                    .push((e.method, e.sig, e.symbol));
            }
            for class in jni_table::MAIN_BRIDGE_CLASSES
                .iter()
                .chain(jni_table::CHUNK_BRIDGE_CLASSES.iter())
            {
                let Some(methods) = by_class.get(class) else {
                    continue;
                };
                match define_and_register(env, lib, class, methods) {
                    Ok((c, n, m)) => {
                        n_classes += c;
                        n_natives += n;
                        n_missing += m;
                    }
                    Err(e) => eprintln!("[crussty-plugin] {class}: {e}"),
                }
            }
        }
        eprintln!("[crussty-plugin] injection loop done");

        // Batch dispatcher (src/batch_api.rs): define the batch bridge class
        // and register run/abiVersion on it. Failure is NON-fatal — the
        // surface above stays live; the batch API just reports
        // ERR_NO_NATIVE_LIB / stays unregistered.
        if let Err(e) = batch_api::init(env, &main) {
            eprintln!("[crussty-plugin] batch: init failed (non-fatal): {e}");
            clear_exception(env);
        }
        Some(())
    });

    if result.is_none() {
        eprintln!("[crussty-plugin] injection aborted: no JNI env (VM not ready?)");
        return;
    }

    eprintln!(
        "[crussty-plugin] native surface live: {n_classes} bridge classes, {n_natives} natives registered ({} symbols unresolved)",
        n_missing
    );

    with_attached(|env| {
        live_proof(env);
        // TASK-53: promotion self-test — armed-only, no-op when dormant.
        promote_wire::selftest_if_armed(env);
        Some(())
    });

    area_map::activate();
    improved_noise::activate();
    // Dormant unless CRUSSTY_NATIVE_BLEND_CACHE is set (see docs/HOOK_BLEND_CACHE.md).
    proto_blend_cache::activate();
}

/// Define one bridge class and register all its natives.
/// Returns (classes, natives, missing symbols).
fn define_and_register(
    env: &JniEnv,
    lib: &loader::NativeLib,
    class: &str,
    methods: &[(&str, &str, &str)],
) -> Result<(usize, usize, usize), String> {
    let pairs: Vec<(&str, &str)> = methods.iter().map(|(m, s, _)| (*m, *s)).collect();
    let bytes = bridge_class::bridge_class_bytes(class, &pairs);
    let Some(cls) = env.define_class(class, std::ptr::null_mut(), &bytes) else {
        clear_exception(env);
        return Err("define_class failed".into());
    };

    let mut names: Vec<CString> = Vec::with_capacity(methods.len());
    let mut sigs: Vec<CString> = Vec::with_capacity(methods.len());
    let mut natives: Vec<jni::JNINativeMethod> = Vec::with_capacity(methods.len());
    let mut missing = 0usize;
    for (m, s, sym) in methods {
        // Kernel selection policy chokepoint (src/kernel_policy.rs): every
        // native the surface registers flows through here. Registration is
        // NOT wiring — this is audit-only (logs in CRUSSTY_KERNEL_POLICY=audit
        // mode, silent otherwise) and never changes behavior.
        kernel_policy::audit_registered(class, m);
        // TASK-04 conservative binding: with CRUSSTY_KERNEL_PREF=old, the
        // implementation pointer of a confirmed-regressed method is swapped
        // to its paired old kernel (same sig, same semantics).
        let fallback = kernel_policy::registration_fallback(class, m);
        // TASK-53 promotion binding: with CRUSSTY_KERNEL_PROMOTE armed, the
        // ORIGINAL bridge method of a promotion pair is bound to its paired
        // P500-WIN kernel (same sig, parity-gate-proven identical semantics;
        // docs/PROVEN_WINS_SYNC.md §4.2). The safety fallback takes
        // precedence if ever both registries matched (they cannot — the
        // kernel_policy disjointness test enforces it).
        let promotion = if fallback.is_some() {
            None
        } else {
            kernel_policy::registration_promotion(class, m)
        };
        let sym: &str = promotion.as_deref().or(fallback.as_deref()).unwrap_or(sym);
        if let Some(win_sym) = &promotion {
            eprintln!("[crussty-plugin] kernel_promote: {class}.{m} bound to win kernel ({win_sym})");
        } else if fallback.is_some() {
            eprintln!("[crussty-plugin] kernel_pref: {class}.{m} bound to old kernel ({sym})");
        }
        let Some(ptr) = lib.symbol(sym) else {
            missing += 1;
            continue;
        };
        names.push(CString::new(*m).map_err(|_| "name has NUL".to_string())?);
        sigs.push(CString::new(*s).map_err(|_| "sig has NUL".to_string())?);
        let name = names.last().unwrap().as_ptr();
        let sig = sigs.last().unwrap().as_ptr();
        natives.push(jni::JNINativeMethod {
            name,
            signature: sig,
            fnPtr: ptr,
        });
    }
    let reg = if natives.is_empty() {
        Ok(())
    } else {
        env.register_natives(cls, &natives)
    };
    env.delete_local_ref(cls);
    if let Err(code) = reg {
        clear_exception(env);
        return Err(format!("register_natives failed (code {code})"));
    }
    Ok((1, natives.len(), missing))
}

/// Live calls through the injected bridge, chosen for determinism:
///  1. PaperNativeNormalNoise.nativeCheck() -> jboolean, always true —
///     proves the bridge class defines + registers correctly and the symbol
///     resolves + executes.
///  2. PaperNativeTicketSetSearch.binarySummary(iter, dst) -> jint — a real
///     benchmark kernel that writes SUMMARY_FIELDS longs into an array
///     (proves array passthrough on the same class).
fn live_proof(env: &JniEnv) {
    // Full internal name — this bridge class lives in net.minecraft.* (the
    // JNI export Java_net_minecraft_..._PaperNativeNormalNoise_* implies it).
    if let Some(cls) =
        env.find_class("net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise")
    {
        if let Some(mid) = env.get_static_method_id(cls, "nativeCheck", "()Z") {
            let ok = env.call_static_int_method(cls, mid, &[]);
            let _ = clear_exception(env);
            eprintln!("[crussty-plugin] live proof: normalNoise.nativeCheck() = {ok}");
        } else {
            let _ = clear_exception(env);
            eprintln!("[crussty-plugin] live proof: nativeCheck unresolved");
        }
        env.delete_local_ref(cls);
    } else {
        let _ = clear_exception(env);
        eprintln!("[crussty-plugin] live proof: find_class(PaperNativeNormalNoise) failed");
    }

    let Some(cls) = env.find_class("PaperNativeTicketSetSearch") else {
        let _ = clear_exception(env);
        eprintln!("[crussty-plugin] live proof: find_class(PaperNativeTicketSetSearch) failed");
        return;
    };
    let Some(mid) = env.get_static_method_id(cls, "binarySummary", "(I[J)I") else {
        let _ = clear_exception(env);
        eprintln!("[crussty-plugin] live proof: get_static_method_id failed");
        env.delete_local_ref(cls);
        return;
    };
    let Some(arr) = env.new_long_array(1) else {
        let _ = clear_exception(env); // C-1: never detach with a pending exception
        eprintln!("[crussty-plugin] live proof: new_long_array failed");
        env.delete_local_ref(cls);
        return;
    };
    let written =
        env.call_static_int_method(cls, mid, &[jni::jvalue { i: 1000 }, jni::jvalue { l: arr }]);
    let _ = clear_exception(env);
    env.delete_local_ref(arr);
    env.delete_local_ref(cls);
    eprintln!("[crussty-plugin] live proof: ticketset binarySummary(1000) wrote {written} long(s)");
}

/// Attach the current thread if needed, run `f` with a JNI env, detach only
/// if we attached. Standard Oracle-JNI GetEnv-first idiom.
///
/// H-10 (hardening audit A11): the detach is a drop-guard, so a panic inside
/// `f` still detaches instead of leaking the thread's attachment slot. The
/// panic itself keeps propagating to the nearest catch_unwind boundary.
fn with_attached<R>(f: impl FnOnce(&JniEnv) -> R) -> Option<R> {
    struct DetachGuard(*mut jni::JavaVM);
    impl DetachGuard {
        fn disarm(&mut self) {
            self.0 = std::ptr::null_mut();
        }
    }
    impl Drop for DetachGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    ((**self.0).DetachCurrentThread)(self.0);
                }
            }
        }
    }
    unsafe {
        let raw_vm = cplug_sdk::vm();
        let vm = raw_vm as *mut jni::JavaVM;
        if vm.is_null() || (*vm).is_null() {
            return None;
        }
        let mut env_ptr: *mut jni::JNIEnv = std::ptr::null_mut();
        let rc = ((**vm).GetEnv)(
            vm,
            &mut env_ptr as *mut *mut jni::JNIEnv as *mut *mut std::ffi::c_void,
            jni::JNI_VERSION_1_6,
        );
        if rc == jni::JNI_OK && !env_ptr.is_null() {
            return Some(f(&JniEnv::from_raw(env_ptr)));
        }
        if rc != jni::JNI_EDETACHED {
            return None;
        }
        let rc = ((**vm).AttachCurrentThread)(
            vm,
            &mut env_ptr as *mut *mut jni::JNIEnv as *mut *mut std::ffi::c_void,
            std::ptr::null_mut(),
        );
        if rc != jni::JNI_OK || env_ptr.is_null() {
            return None;
        }
        let env = JniEnv::from_raw(env_ptr);
        let mut guard = DetachGuard(vm);
        let out = f(&env);
        guard.disarm(); // normal path: detach here, in order
        drop(guard);
        Some(out)
    }
}

/// Clear a pending exception silently. Transient lookups (find_class before
/// the kernel is up, getLogger before Bukkit.server is set) throw routinely;
/// exception_describe would print scary "Exception in thread" traces to the
/// server log for a condition we fully expect.
fn clear_exception(env: &JniEnv) -> bool {
    if env.exception_check() {
        env.exception_clear();
        true
    } else {
        false
    }
}

/// Report a pending exception loudly (describe + clear). For paths where an
/// exception means REAL breakage (define_class of our own bridge, RegisterNatives
/// setup, self-test callables) — the trace is exactly what an operator needs in
/// the server log. Always safe to call: no-op when nothing is pending.
fn describe_exception(env: &JniEnv) -> bool {
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        true
    } else {
        false
    }
}
