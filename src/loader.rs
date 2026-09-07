//! dlopen wrapper for the bundled Crussty CE native libraries
//! (libpaper_native_jni.so / libpaper_native_chunk_encode_jni.so).

use libloading::Library;
use std::ffi::c_void;
use std::path::Path;
use std::sync::{Mutex, OnceLock, PoisonError};

/// Libraries must outlive the JVM: `register_natives` stashes raw function
/// pointers into method entries, so dlclose when the plugin's handle goes
/// out of scope would leave them dangling. Keep every dlopen'd handle alive
/// for the process lifetime (leaked on purpose).
static KEEP_ALIVE: OnceLock<Mutex<Vec<&'static Library>>> = OnceLock::new();

pub struct NativeLib {
    lib: &'static Library,
}

impl NativeLib {
    /// # Safety
    /// `path` must name a valid, position-independent shared object that is
    /// safe to dlopen into this process.
    pub unsafe fn new(path: &Path) -> Result<Self, String> {
        let lib = unsafe { Library::new(path) }.map_err(|e| e.to_string())?;
        let lib: &'static Library = Box::leak(Box::new(lib));
        // Poison-recovery (TASK-46): reachable at runtime via batch_api's
        // lazy NativeLib::new, not just init. The guard only pushes a leaked
        // &'static Library (no user code under the lock), so the Vec is
        // structurally valid after a panic; a raw .unwrap() could panic on a
        // JNI-attached caller thread → unwind across JNI = VM abort.
        KEEP_ALIVE
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(lib);
        Ok(Self { lib })
    }

    /// Resolve an exported symbol by exact name (a JNI `Java_...` symbol).
    pub fn symbol(&self, name: &str) -> Option<*mut c_void> {
        unsafe {
            self.lib
                .get::<*mut c_void>(name.as_bytes())
                .ok()
                .map(|s| *s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TASK-46 poison-recovery: a panic while KEEP_ALIVE is locked must not
    /// make the next NativeLib::new (reachable at runtime via batch_api's
    /// lazy dlopen on a JNI-attached thread) panic on the poisoned mutex.
    #[test]
    fn poisoned_keep_alive_recovered_by_new() {
        let poisoner = std::thread::spawn(|| {
            let _guard = KEEP_ALIVE.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap();
            panic!("deliberate: poison the KEEP_ALIVE mutex");
        });
        let _ = poisoner.join();
        // dlopen a guaranteed-present system lib (glibc): the recovered Vec
        // must accept the push and the handle must resolve symbols.
        let lib = unsafe { NativeLib::new(Path::new("libc.so.6")) }.expect("libc.so.6 dlopen");
        assert!(lib.symbol("open").is_some(), "symbol lookup after poison recovery");
    }
}
