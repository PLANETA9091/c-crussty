//! Class resolution that works across ALL class loaders.
//!
//! JNI FindClass from a native thread only sees the system loader — kernel
//! classes (org/bukkit/*, net/minecraft/*) live in Paper's own loader and are
//! invisible to it. The SDK resolves them via JVMTI GetLoadedClasses and caches
//! a process-lifetime global ref per class.

use crate::jni_util::{clear_exception, with_attached};
use jvmti_bindings::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

/// A jclass global ref to a loaded class, safe across threads.
///
/// The ref is owned by the process-lifetime cache (find_class leaks the ref
/// into CACHE on purpose). ClassRef itself is a non-owning view — it must NOT
/// delete the ref on drop, or the next find_class from cache would hand out a
/// dangling jclass (GetMethodID on it SIGSEGVs at +0x10).
pub struct ClassRef(pub jni::jclass);

// JNI global refs are thread-safe by contract (delete from any thread), so
// raw *mut c_void wrapped here may cross threads.
unsafe impl Send for ClassRef {}
unsafe impl Sync for ClassRef {}

impl ClassRef {
    pub fn as_jclass(&self) -> jni::jclass {
        self.0
    }
}

static CACHE: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, usize>> {
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

// ---------------------------------------------------------------------------
// ClassFileLoadHook sighting feed (TASK-22 / HOTSPOT_CANDIDATES C1).
//
// `hooks::dispatch` sees the internal name of EVERY class the JVM loads (and
// every retransform). find_class consults this record BEFORE doing a JVMTI
// GetLoadedClasses scan: a name that was never sighted cannot have been loaded
// since the hook pipeline armed, so the scan is skipped entirely and the
// caller gets a fast miss. This removes the per-poll full-heap rescans from
// the boot class-loading storm (the area_map/improved_noise pollers used to
// rescan up to ~85x per 180s window; each scan walks 10k-30k classes with
// GetClassStatus/GetClassSignature per class, on the same 2 CPUs the JVM uses
// to load the classes being scanned).
//
// The feed has one documented blind window: classes that finished loading
// BEFORE the runtime's ClassFileLoadHook pipeline armed are never sighted
// (ARCHITECTURE.md G6). To keep find_class results identical to the ungated
// scan, the gate is advisory with a bounded fallback: the FIRST unsighted
// call for every name still scans, and every 8th unsighted call after that
// scans again, so a pre-hook load is discovered within ~8 poll intervals —
// far inside every caller's deadline. The JVMTI INITIALIZED-status guard in
// find_class is UNCHANGED and still applies to every scan: a sighting means
// "bytes were delivered", not "class is initialized", and GetMethodID on a
// not-yet-initialized class SIGSEGVs.
//
// LOCK ORDER (SDK-wide rule: no SDK lock is held while acquiring another):
// each shard mutex below is taken alone and released before anything else
// runs; find_class takes the shard lock and the CACHE lock sequentially,
// never nested. hooks::dispatch records the sighting with the shard lock
// taken and dropped BEFORE running any callback, so a callback may freely
// call find_class (shard + CACHE) without re-entering a held lock.
// ---------------------------------------------------------------------------

/// Sharded "names seen through the hook pipeline" set. Shards keep the
/// per-class-load write (from hooks::dispatch) off a single contention
/// point during parallel boot-time loading. Bounded: once a shard is full
/// new names are not recorded — the gate then falls back to the bounded
/// scan above (correct, just less optimized); the bound protects long-lived
/// servers that synthesize classes (lambdas) forever.
const SIGHTING_SHARDS: usize = 16;
const SIGHTING_SHARD_CAP: usize = 4096;

static SIGHTINGS: OnceLock<[Mutex<HashSet<Box<str>>>; SIGHTING_SHARDS]> = OnceLock::new();

fn sighting_shard(name: &str) -> &'static Mutex<HashSet<Box<str>>> {
    // FNV-1a: allocation-free, good-enough spread for shard picking.
    let mut h: u32 = 0x811c_9dc5;
    for b in name.as_bytes() {
        h ^= u32::from(*b);
        h = h.wrapping_mul(0x0100_0193);
    }
    &SIGHTINGS.get_or_init(|| std::array::from_fn(|_| Mutex::new(HashSet::new())))
        [(h as usize) % SIGHTING_SHARDS]
}

/// Record a class-load sighting. Called from `hooks::dispatch` on the
/// class-load thread; the shard lock is taken and released here, nothing
/// else runs while it is held (see the lock-order note above).
pub fn note_loaded(name: &str) {
    let shard = sighting_shard(name);
    let mut set = shard.lock().unwrap_or_else(|e| e.into_inner());
    if set.len() < SIGHTING_SHARD_CAP {
        set.insert(Box::from(name));
    }
}

/// Whether the hook pipeline has ever delivered `name` (dotted or internal
/// form). Advisory-only: false does not mean "not loaded" (pre-hook blind
/// window), which is why find_class keeps its bounded fallback scan.
pub fn is_sighted(name: &str) -> bool {
    let internal = crate::jni_util::to_internal(name);
    sighting_shard(&internal)
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .contains(internal.as_str())
}

/// Per-name polling state for the sighting gate (cold path: touched only on
/// cache misses from poller threads).
struct PollState {
    /// Unsighted calls since the last JVMTI scan; starts at the skip budget
    /// so the FIRST unsighted call scans (covers pre-hook loads).
    skips: u8,
    /// Unsighted calls answered WITHOUT a JVMTI heap scan (scans avoided).
    avoided: u64,
}

/// Skip budget between fallback scans: 7 skips between scans = one scan per
/// 8 unsighted calls.
const UNSIGHTED_SKIP_BUDGET: u8 = 7;

static POLL_STATE: OnceLock<Mutex<HashMap<String, PollState>>> = OnceLock::new();

fn poll_state() -> &'static Mutex<HashMap<String, PollState>> {
    POLL_STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Sighting gate for find_class: returns true when a JVMTI scan must run.
fn unsighted_scan_due(internal: &str) -> bool {
    let mut state = poll_state().lock().unwrap_or_else(|e| e.into_inner());
    let s = state
        .entry(internal.to_string())
        .or_insert(PollState {
            skips: UNSIGHTED_SKIP_BUDGET,
            avoided: 0,
        });
    if s.skips >= UNSIGHTED_SKIP_BUDGET {
        s.skips = 0;
        true
    } else {
        s.skips += 1;
        s.avoided += 1;
        false
    }
}

/// How many JVMTI full-heap scans the sighting gate avoided for `name` so
/// far (diagnostic for the one-line activation log in the pollers).
pub fn scans_avoided(name: &str) -> u64 {
    let internal = crate::jni_util::to_internal(name);
    poll_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&internal)
        .map(|s| s.avoided)
        .unwrap_or(0)
}

/// Resolve a loaded class by name ("org/bukkit/Bukkit" or dotted) and cache
/// it forever. Returns None if the class is not loaded yet.
pub fn find_class(name: &str) -> Option<ClassRef> {
    let internal = crate::jni_util::to_internal(name);
    if let Some(addr) = cache().lock().unwrap().get(&internal) {
        return Some(ClassRef(*addr as jni::jclass));
    }
    // TASK-22/C1 sighting gate: while the hook pipeline has never sighted
    // this name, skip the full JVMTI heap scan and answer "not loaded" from
    // the feed. Advisory — `unsighted_scan_due` forces the bounded fallback
    // scan (first call + every 8th) so pre-hook loads are still found.
    if !unsighted_scan_due(&internal) {
        return None;
    }
    let signature = format!("L{internal};");
    with_attached(|env| {
        let jvmti = Jvmti::new(crate::vm() as *mut jni::JavaVM).ok()?;
        let classes = jvmti.get_loaded_classes().ok()?;
        let mut found = None;
        for cls in &classes {
            // Only accept classes that have FINISHED definition: throughout
            // init (Paper's remap redefines many boot classes) GetLoadedClasses
            // can return a class whose InstanceKlass is not laid out yet —
            // GetMethodID on it derefs null fields (SIGSEGV).
            let Ok(status) = jvmti.get_class_status(*cls) else {
                env.delete_local_ref(*cls);
                continue;
            };
            const INITIALIZED: i32 = 4; // JVMTI_CLASS_STATUS_INITIALIZED (jvmti.h)
            if status & INITIALIZED == 0 {
                env.delete_local_ref(*cls);
                continue;
            }
            if let Ok((sig, _)) = jvmti.get_class_signature(*cls) {
                if sig == signature {
                    found = Some(*cls);
                    // TASK-22/C1: stop at the FIRST match (the old `continue`
                    // walked the whole loaded-class array and overwrote
                    // `found`, abandoning the earlier local ref). Local refs
                    // of unvisited classes are freed when this attached
                    // thread detaches, same as the pre-existing ref kept on
                    // the match path.
                    break;
                }
            }
            env.delete_local_ref(*cls);
        }
        let cls = found?;
        let gref = env.new_global_ref(cls);
        env.delete_local_ref(cls);
        let _ = clear_exception(env);
        cache()
            .lock()
            .unwrap()
            .insert(internal.clone(), gref as usize);
        Some(ClassRef(gref))
    })
    .flatten()
}

/// Poll for the class until it is loaded (kernel-ready convenience):
/// returns immediately if present, otherwise retries every 200ms up to
/// `timeout_ms`. Returns None on timeout.
pub fn wait_class(name: &str, timeout_ms: u64) -> Option<ClassRef> {
    let mut waited = 0u64;
    loop {
        if let Some(c) = find_class(name) {
            return Some(c);
        }
        waited += 200;
        if waited >= timeout_ms {
            return None;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

/// Resolve a method id on a class (instance method). Clears pending
/// exceptions (NoSuchMethodError) so callers can retry cleanly.
pub fn method(env: &JniEnv, cls: jni::jclass, name: &str, desc: &str) -> Option<usize> {
    let m = env.get_method_id(cls, name, desc);
    if m.is_none() {
        let _ = clear_exception(env);
    }
    m.map(|mid| mid as usize)
}

/// Resolve a static method id. Clears pending exceptions on failure.
pub fn static_method(env: &JniEnv, cls: jni::jclass, name: &str, desc: &str) -> Option<usize> {
    let m = env.get_static_method_id(cls, name, desc);
    if m.is_none() {
        let _ = clear_exception(env);
    }
    m.map(|mid| mid as usize)
}

/// Resolve + call a static method that returns a String, and read it.
/// Clears any pending exception either way.
pub fn static_string(env: &JniEnv, cls: jni::jclass, name: &str, desc: &str) -> Option<String> {
    let mid = static_method(env, cls, name, desc)?;
    let obj = env.call_static_object_method(cls, mid as jni::jmethodID, &[]);
    if obj.is_null() {
        let _ = clear_exception(env);
        return None;
    }
    let s = env.get_string_utf(obj as jni::jstring);
    let _ = clear_exception(env);
    s
}

/// Re-run the class-file hook chain for a loaded class (JVMTI
/// RetransformClasses): the VM calls every byte hook again, synchronously on
/// the calling thread, with the class's ORIGINAL bytes. The canonical way to
/// instrument a class that was already loaded (and possibly already used)
/// without a definition race. Must be called from an attached thread (e.g.
/// inside run_on_main_thread).
pub fn retransform(name: &str) -> bool {
    let Some(cls) = find_class(name) else {
        return false;
    };
    with_attached(|env| {
        let jvmti = Jvmti::new(crate::vm() as *mut jni::JavaVM).ok()?;
        // Capabilities are PER-ENVIRONMENT: GetEnv hands out a fresh env
        // each call, so the runtime's can_retransform_classes does not apply
        // here — this env must add it itself (legal in the live phase).
        if let Err(e) = jvmti.add_capabilities_with(|caps| {
            caps.set_can_retransform_classes(true);
        }) {
            eprintln!("[cplug-sdk] add_capabilities failed: {e:?}");
            return Some(false);
        }
        let r = jvmti.retransform_classes(&[cls.as_jclass()]);
        let _ = clear_exception(env);
        match r {
            Ok(()) => Some(true),
            Err(e) => {
                eprintln!("[cplug-sdk] retransform {name} failed: {e:?}");
                Some(false)
            }
        }
    })
    .flatten()
    .unwrap_or(false)
}
