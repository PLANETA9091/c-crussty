//! Kernel logging via `java.util.logging.Logger` (bootstrap class, resolvable
//! with plain JNI — no GetLoadedClasses needed). Falls back to stderr until
//! the kernel is up.

use crate::classes::{find_class, method, static_method, ClassRef};
use crate::jni_util::with_attached;
use jvmti_bindings::prelude::*;
use std::sync::{Mutex, OnceLock, PoisonError};

enum Level {
    Info,
    Warning,
}

struct LoggerIds {
    bukkit: ClassRef,
    get_logger: usize,
    info: usize,
    warning: usize,
}

/// Plain-scalar copy of the resolved logger plumbing (C7/TASK-27): lets
/// `emit` release the `ids` mutex BEFORE any JNI work instead of holding it
/// across `with_attached` + 3 calls. `bukkit` is the raw jclass value of a
/// process-lifetime global ref (classes.rs leaks those refs on purpose and
/// ClassRef is a non-owning view), so holding the copy past the lock is
/// exactly as safe as the previous borrow-under-lock.
#[derive(Clone, Copy)]
struct LoggerSnapshot {
    bukkit: jni::jclass,
    get_logger: usize,
    info: usize,
    warning: usize,
}

static IDS: OnceLock<Mutex<Option<LoggerIds>>> = OnceLock::new();

fn ids() -> &'static Mutex<Option<LoggerIds>> {
    IDS.get_or_init(|| Mutex::new(None))
}

fn resolve() -> Option<LoggerIds> {
    let bukkit = find_class("org/bukkit/Bukkit")?;
    let logger = find_class("java/util/logging/Logger")?;
    with_attached(|env| {
        let get_logger = static_method(
            env,
            bukkit.as_jclass(),
            "getLogger",
            "()Ljava/util/logging/Logger;",
        )?;
        let info = method(env, logger.as_jclass(), "info", "(Ljava/lang/String;)V")?;
        let warning = method(env, logger.as_jclass(), "warning", "(Ljava/lang/String;)V")?;
        Some(LoggerIds {
            bukkit,
            get_logger,
            info,
            warning,
        })
    })
    .flatten()
}

fn emit(level: Level, msg: &str) {
    // C7 (TASK-27): the ids mutex must not span JNI work. Copy a plain
    // scalar snapshot of the resolved plumbing out under the lock, release
    // the lock, then emit. (Cold-path resolve() stays under the lock exactly
    // as before — it runs once, before the kernel logger exists.)
    let snapshot = {
        // Poison-recovery (TASK-46): emit() is reachable from hook-callback
        // threads (ClassFileLoadHook) and any JNI-attached path; the guard
        // only copies a plain-scalar snapshot / stores resolve()'s result —
        // no user code under the lock, so the mutex stays structurally valid
        // after a panic. A raw .unwrap() here would unwind across JNI = VM
        // abort; with into_inner a poisoned cache degrades to the existing
        // "log dropped" stderr fallback instead.
        let mut cache = ids().lock().unwrap_or_else(PoisonError::into_inner);
        if cache.is_none() {
            *cache = resolve();
        }
        cache
            .as_ref()
            .map(|ids| LoggerSnapshot {
                bukkit: ids.bukkit.as_jclass(),
                get_logger: ids.get_logger,
                info: ids.info,
                warning: ids.warning,
            })
    }; // mutex released here
    let Some(ids) = snapshot else {
        eprintln!("[cplug-sdk] log dropped (kernel not ready): {msg}");
        return;
    };
    let mid = match level {
        Level::Info => ids.info,
        Level::Warning => ids.warning,
    } as jni::jmethodID;
    let get_logger = ids.get_logger as jni::jmethodID;
    let ok = with_attached(|env| {
        let s = env.new_string_utf(msg)?;
        let logger = env.call_static_object_method(ids.bukkit, get_logger, &[]);
        if logger.is_null() {
            env.delete_local_ref(s);
            return None;
        }
        env.call_void_method(logger, mid, &[jni::jvalue { l: s }]);
        env.delete_local_ref(s);
        env.delete_local_ref(logger);
        Some(())
    })
    .is_some();
    if !ok {
        eprintln!("[cplug-sdk] log emit failed: {msg}");
    }
}

/// Log an info message through the kernel logger (stderr fallback before the
/// kernel is up).
pub fn info(msg: &str) {
    emit(Level::Info, msg);
}

/// Log a warning through the kernel logger.
pub fn warn(msg: &str) {
    emit(Level::Warning, msg);
}

pub fn log_info(msg: &str) {
    info(msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TASK-46 poison-recovery: a panic while IDS is locked (e.g. resolve()
    /// blowing up on a JNI edge) must NOT turn the next emit() from a
    /// hook-callback thread into an unwind-across-JNI VM abort. After the
    /// deliberate poison, emit() must recover the (structurally valid)
    /// Option<LoggerIds> and take its normal headless path.
    #[test]
    fn poisoned_ids_lock_recovered_by_emit() {
        // Poison the lock: panic while the guard is held in another thread.
        let poisoner = std::thread::spawn(|| {
            let _guard = ids().lock().unwrap();
            panic!("deliberate: poison the IDS mutex");
        });
        let _ = poisoner.join(); // panic contained in the spawned thread
        // Pre-fix this call would panic on the poisoned lock (real-world
        // analogue: a ClassFileLoadHook thread calling log::info).
        emit(
            Level::Info,
            "poison probe: must recover, not unwind across JNI",
        );
        // The mutex must still be lockable and structurally valid.
        assert!(ids().lock().unwrap_or_else(|e| e.into_inner()).is_none());
    }
}
