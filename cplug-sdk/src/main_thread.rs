//! Run closures on the server's main thread — WITHOUT any plugin.
//!
//! Bukkit's scheduler requires an org.bukkit.plugin.Plugin on every overload
//! (native plugins have none). Instead we use the vanilla main-thread task
//! queue: `net.minecraft.server.MinecraftServer` is a BlockableEventLoop and
//! `public static MinecraftServer getServer()` + inherited
//! `execute(Runnable)` schedule work on the server thread.
//!
//! The Runnable is a tiny class we define ourselves (bootstrap loader,
//! references only java/lang/Object + java/lang/Runnable) with one native
//! method whose implementation pops a queued closure.

use crate::classes::{find_class, method, static_method, ClassRef};
use crate::jni_util::with_attached;
use jvmti_bindings::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

const RUNNABLE_NAME_PREFIX: &str = "dev/dist/SdkNativeRunnable";
const KERNEL_CLASS: &str = "net/minecraft/server/MinecraftServer";

/// C6 (TASK-27): max jobs popped per `SdkNativeRunnable.run()` dispatch —
/// one MinecraftServer.execute round-trip amortizes over up to this many
/// jobs. FIFO order is preserved regardless of the batch size: the main
/// thread runs queued runnables sequentially, so a later runnable can never
/// overtake an in-progress drain.
const JOBS_PER_RUNNABLE: usize = 8;

type Job = Box<dyn FnOnce(&JniEnv) + Send>;

static QUEUE: Mutex<VecDeque<Job>> = Mutex::new(VecDeque::new());
static RUNNABLE_CLS: OnceLock<ClassRef> = OnceLock::new();
static RUNNABLE_NAME: OnceLock<Box<str>> = OnceLock::new();
static FLUSH_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Plain-scalar snapshot of the main-thread delivery plumbing (C6/TASK-27),
/// cached after the first fully-successful resolution instead of being
/// re-resolved for every queued job. `mc`/`runnable_cls` are raw jclass
/// values of process-lifetime global refs (the classes.rs CACHE and
/// `RUNNABLE_CLS` both leak them by design; ClassRef is a non-owning view),
/// so holding the copy outside the mutex is exactly as safe as the old
/// per-job re-resolution. Any failed delivery drops the cache conservatively
/// (see `deliver`), so the next attempt re-resolves from scratch.
#[derive(Clone, Copy)]
struct MainIds {
    mc: jni::jclass,
    /// MinecraftServer.getServer()Lnet/minecraft/server/MinecraftServer;
    get_server: usize,
    /// BlockableEventLoop.execute(Ljava/lang/Runnable;)V
    execute: usize,
    runnable_cls: jni::jclass,
    /// SdkNativeRunnable.<init>()V
    runnable_init: usize,
}

static MAIN_IDS: Mutex<Option<MainIds>> = Mutex::new(None);

// Same contract as ClassRef (classes.rs): the jclass values are
// process-lifetime global refs — usable from any thread by JNI contract —
// and the jmethodIDs stay valid for the lifetime of those (globally
// referenced) classes. Copying the scalars transfers no ownership.
unsafe impl Send for MainIds {}

/// The runnable class name for THIS SDK copy. Every module carries its own
/// copy of the SDK (RTLD_LOCAL => private statics, private natives), so the
/// class it defines in the bootstrap loader must be unique per module —
/// otherwise the second module's define_class hits
/// `LinkageError: duplicate class definition`. The address of our own QUEUE
/// static differs per .so, giving each module a stable unique name.
pub fn runnable_class_name() -> &'static str {
    RUNNABLE_NAME.get_or_init(|| {
        let addr = &QUEUE as *const _ as usize;
        format!("{RUNNABLE_NAME_PREFIX}{addr:x}").into_boxed_str()
    })
}

/// Queue `f` to run on the main thread (with an attached JNI env). Safe to
/// call from any thread, any time — before the kernel is up, jobs wait in the
/// queue and are flushed as soon as the server becomes reachable.
pub fn run_on_main_thread<F>(f: F)
where
    F: FnOnce(&JniEnv) + Send + 'static,
{
    QUEUE.lock().unwrap().push_back(Box::new(f));
    ensure_flush();
}

/// One background flush worker at a time; exits when the queue is empty, then
/// re-arms on the next use. Polls `deliver` so jobs handed over before the
/// kernel is ready still go out once it is.
fn ensure_flush() {
    if !FLUSH_ACTIVE.swap(true, Ordering::SeqCst) {
        std::thread::spawn(flush_loop);
    }
}

fn flush_loop() {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    loop {
        if QUEUE.lock().unwrap().is_empty() {
            FLUSH_ACTIVE.store(false, Ordering::SeqCst);
            return;
        }
        let ok = deliver();
        if ok {
            // The runnable executes asynchronously on the main thread; give
            // it time to drain the queue before delivering again.
            std::thread::sleep(std::time::Duration::from_millis(50));
            continue;
        }
        if std::time::Instant::now() > deadline {
            eprintln!(
                "[cplug-sdk] main-thread flush timed out ({} queued)",
                QUEUE.lock().unwrap().len()
            );
            FLUSH_ACTIVE.store(false, Ordering::SeqCst);
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

/// True once the runnable skeleton class is defined and its natives
/// registered (set by the first successful deliver). Byte hooks may consult
/// this to defer patches that depend on the runnable class (see classes::retransform).
pub fn runnable_defined() -> bool {
    RUNNABLE_CLS.get().is_some()
}

/// Cached delivery plumbing (C6/TASK-27). Fast path copies the snapshot out
/// under the lock — no JNI work happens while any lock is held. Cold path
/// resolves OUTSIDE the lock and only then publishes (two flush attempts
/// racing a cold resolve would both just do the same idempotent lookups).
fn main_ids(env: &JniEnv) -> Option<MainIds> {
    if let Some(ids) = *MAIN_IDS.lock().unwrap_or_else(|e| e.into_inner()) {
        return Some(ids);
    }
    let ids = resolve_main_ids(env)?;
    *MAIN_IDS.lock().unwrap_or_else(|e| e.into_inner()) = Some(ids);
    Some(ids)
}

/// Resolve the delivery plumbing: kernel class reachable, runnable class
/// defined + natives registered, and the three method IDs resolved. No
/// caching here — see `main_ids`.
fn resolve_main_ids(env: &JniEnv) -> Option<MainIds> {
    let mc = find_class(KERNEL_CLASS)?;
    let runnable_cls = RUNNABLE_CLS.get_or_init(|| ClassRef(define_runnable_class(env)));
    if runnable_cls.0.is_null() {
        return None;
    }
    let get_server = static_method(
        env,
        mc.as_jclass(),
        "getServer",
        "()Lnet/minecraft/server/MinecraftServer;",
    )?;
    let execute = method(env, mc.as_jclass(), "execute", "(Ljava/lang/Runnable;)V")?;
    let runnable_init = method(env, runnable_cls.0, "<init>", "()V")?;
    Some(MainIds {
        mc: mc.as_jclass(),
        get_server,
        execute,
        runnable_cls: runnable_cls.as_jclass(),
        runnable_init,
    })
}

/// Hand queued jobs to the main thread via MinecraftServer.execute.
/// Returns false while the kernel is unreachable (class not loaded / server
/// object not created yet). The class refs + three method IDs are cached
/// across deliveries (C6/TASK-27); the `getServer` OBJECT is still fetched
/// fresh on every delivery (the server instance is the kernel's to create).
fn deliver() -> bool {
    let ok = with_attached(|env| {
        // Never enter JNI with a stale pending exception from a previous
        // iteration — that is UB and crashes the VM.
        let _ = crate::jni_util::clear_exception(env);
        let Some(ids) = main_ids(env) else {
            return Some(false);
        };
        let server =
            env.call_static_object_method(ids.mc, ids.get_server as jni::jmethodID, &[]);
        if server.is_null() {
            let _ = crate::jni_util::clear_exception(env);
            return Some(false);
        }
        let Some(obj) = env.new_object(ids.runnable_cls, ids.runnable_init as jni::jmethodID, &[])
        else {
            return Some(false);
        };
        env.call_void_method(server, ids.execute as jni::jmethodID, &[jni::jvalue { l: obj }]);
        let _ = crate::jni_util::clear_exception(env);
        env.delete_local_ref(obj);
        env.delete_local_ref(server);
        Some(true)
    })
    .flatten()
    .unwrap_or(false);
    if !ok {
        // Conservative invalidation: any failed delivery (kernel class
        // missing, runnable define/register failed, method unresolved,
        // server still null, attach failed) drops the cached plumbing so
        // the next attempt re-resolves from scratch — the same cost profile
        // as the pre-cache code, which re-resolved on EVERY job.
        *MAIN_IDS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
    ok
}

/// Define + RegisterNatives the runnable skeleton once per process. Returns a
/// global ref (leaked by design — lives for the process) or null on failure.
fn define_runnable_class(env: &JniEnv) -> jni::jclass {
    let name = runnable_class_name();
    let bytes = runnable_class_bytes(name);
    let Some(cls) = env.define_class(name, std::ptr::null_mut(), &bytes) else {
        eprintln!("[cplug-sdk] define_class failed for {name}");
        return std::ptr::null_mut();
    };
    let natives = [
        jni::JNINativeMethod {
            name: c"run".as_ptr(),
            signature: c"()V".as_ptr(),
            fnPtr: sdk_run_trampoline as *mut std::ffi::c_void,
        },
        jni::JNINativeMethod {
            name: c"weaveMark".as_ptr(),
            signature: c"()V".as_ptr(),
            fnPtr: sdk_weave_mark_trampoline as *mut std::ffi::c_void,
        },
    ];
    if env.register_natives(cls, &natives).is_err() {
        eprintln!("[cplug-sdk] register_natives failed");
        env.delete_local_ref(cls);
        return std::ptr::null_mut();
    }
    let gref = env.new_global_ref(cls);
    env.delete_local_ref(cls);
    gref
}

/// JNI native entry for `SdkNativeRunnable.run()`: pops up to
/// JOBS_PER_RUNNABLE queued jobs and runs them with the caller's env (the
/// main thread is already attached). Draining in batches cuts the number of
/// main-thread round-trips (C6/TASK-27); jobs still execute in FIFO order
/// on the main thread.
unsafe extern "system" fn sdk_run_trampoline(env_raw: *mut jni::JNIEnv, _obj: jni::jobject) {
    let env = JniEnv::from_raw(env_raw);
    // H-01: a panicking main-thread job must not unwind across JNI (= abort).
    // Mutex poisoning is recovered: a panic while holding the queue lock does
    // not invalidate the deque contents.
    for _ in 0..JOBS_PER_RUNNABLE {
        let job = QUEUE
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pop_front();
        let Some(job) = job else { break };
        // A previous job (or the dispatch itself) may have left a pending
        // exception; entering a job with one pending is UB — same rule the
        // flush side follows in deliver().
        let _ = crate::jni_util::clear_exception(&env);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job(&env)));
        if r.is_err() {
            eprintln!("[cplug-sdk] main-thread job panicked (recovered)");
        }
    }
}

/// JNI native entry for `SdkNativeRunnable.weaveMark()`: a zero-arg static
/// marker used as the invokestatic target of woven bytecode (see weave.rs) —
/// its execution proves a live patch fired.
unsafe extern "system" fn sdk_weave_mark_trampoline(
    _env_raw: *mut jni::JNIEnv,
    _obj: jni::jobject,
) {
    eprintln!("[cplug-sdk] WEAVE-MARK: woven invokestatic target executed");
}

/// Bytecode for:
/// ```java
/// package dev.dist;
/// public class SdkNativeRunnable<addr> implements Runnable {
///     public native void run();
///     public static native void weaveMark();
///     public SdkNativeRunnable<addr>() { super(); }
/// }
/// ```
/// Java 8 target (major 52) so the verifier needs no StackMapTable for this
/// trivial constructor. The name is per-module unique (see
/// `runnable_class_name`) so multiple modules never collide on the bootstrap
/// loader class definition.
pub fn runnable_class_bytes(name: &str) -> Vec<u8> {
    let mut c = Vec::new();
    c.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]); // magic
    c.extend_from_slice(&[0, 0]); // minor
    c.extend_from_slice(&[0, 52]); // major 52 (Java 8)
    c.extend_from_slice(&[0, 15]); // cp_count

    // cp[1] Utf8 "<name>"
    push_utf8(&mut c, name);
    // cp[2] Utf8 "java/lang/Object"
    push_utf8(&mut c, "java/lang/Object");
    // cp[3] Utf8 "<init>"
    push_utf8(&mut c, "<init>");
    // cp[4] Utf8 "()V"
    push_utf8(&mut c, "()V");
    // cp[5] Utf8 "java/lang/Runnable"
    push_utf8(&mut c, "java/lang/Runnable");
    // cp[6] Utf8 "run"
    push_utf8(&mut c, "run");
    // cp[7] Utf8 "Code"
    push_utf8(&mut c, "Code");
    // cp[8] Class -> #1
    push_const(&mut c, 7, &[0, 1]);
    // cp[9] Class -> #2
    push_const(&mut c, 7, &[0, 2]);
    // cp[10] Class -> #5
    push_const(&mut c, 7, &[0, 5]);
    // cp[11] NameAndType -> #3 #4
    push_const(&mut c, 12, &[0, 3, 0, 4]);
    // cp[12] NameAndType -> #6 #4
    push_const(&mut c, 12, &[0, 6, 0, 4]);
    // cp[13] Methodref -> Object.<init>()V
    push_const(&mut c, 10, &[0, 9, 0, 11]);
    // cp[14] Utf8 "weaveMark"
    push_utf8(&mut c, "weaveMark");

    // access_flags: public | super
    c.extend_from_slice(&[0, 0x21]);
    // this_class = #8, super_class = #9
    c.extend_from_slice(&[0, 8, 0, 9]);
    // interfaces_count = 1: java/lang/Runnable (#10)
    c.extend_from_slice(&[0, 1, 0, 10]);
    // fields_count = 0
    c.extend_from_slice(&[0, 0]);

    // methods_count = 3
    c.extend_from_slice(&[0, 3]);
    // method 1: public <init>()V with Code
    c.extend_from_slice(&[0, 1, 0, 3, 0, 4]); // access, name#3, desc#4
    c.extend_from_slice(&[0, 1]); // attributes_count
    c.extend_from_slice(&[0, 7]); // attribute name "Code"
    c.extend_from_slice(&[0, 0, 0, 17]); // attribute length
    c.extend_from_slice(&[0, 1]); // max_stack
    c.extend_from_slice(&[0, 1]); // max_locals
    c.extend_from_slice(&[0, 0, 0, 5]); // code_length
                                        // aload_0; invokespecial #13; return
    c.extend_from_slice(&[0x2a, 0xb7, 0, 13, 0xb1]);
    c.extend_from_slice(&[0, 0]); // exception_table_length
    c.extend_from_slice(&[0, 0]); // code attributes

    // method 2: public native run()V, no attributes
    c.extend_from_slice(&[0x01, 0x01, 0, 6, 0, 4]);
    c.extend_from_slice(&[0, 0]);

    // method 3: public static native weaveMark()V (access 0x0109 = pub|static|native)
    c.extend_from_slice(&[0x01, 0x09, 0, 14, 0, 4]);
    c.extend_from_slice(&[0, 0]);

    // class attributes_count = 0
    c.extend_from_slice(&[0, 0]);
    c
}

fn push_const(c: &mut Vec<u8>, tag: u8, data: &[u8]) {
    c.push(tag);
    c.extend_from_slice(data);
}

fn push_utf8(c: &mut Vec<u8>, s: &str) {
    c.push(1);
    let b = s.as_bytes();
    c.extend_from_slice(&(b.len() as u16).to_be_bytes());
    c.extend_from_slice(b);
}
