//! Class resolution that works across ALL class loaders.
//!
//! JNI FindClass from a native thread only sees the system loader — kernel
//! classes (org/bukkit/*, net/minecraft/*) live in Paper's own loader and are
//! invisible to it. The SDK resolves them via JVMTI GetLoadedClasses and caches
//! a process-lifetime global ref per class.

use crate::jni_util::{clear_exception, with_attached};
use jvmti_bindings::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
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

// D4/TASK-43: BOTH CACHE lock sites (find_class hit path + insert path)
// recover poisoning via `unwrap_or_else(|e| e.into_inner())` instead of the
// historical `.unwrap()`. Recovery is safe here because the guard only ever
// wraps plain HashMap get/insert — no user code runs under it — so a panic
// while the lock is held can at worst drop one in-flight insert; the map is
// structurally valid either way (a HashMap operation either completes or
// leaves the map unchanged, never half-mutated). The old `.unwrap()` would
// panic the NEXT find_class caller, and find_class is reachable from
// ClassFileLoadHook callback threads (hooks::dispatch callbacks may call
// find_class) — an unwind crossing the JNI boundary aborts the VM. This
// matches the poison-recovery rule every other SDK lock has followed since
// C1 (sighting shards, POLL_STATE, MAIN_IDS, QUEUE).

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
    /// Unsighted calls answered without a scan since the last scan; the
    /// next unsighted call at this budget scans again. (D2/TASK-43: entries
    /// are created only AFTER a call already returned "scan due", so a fresh
    /// entry starts at 0 — the historical starts-at-budget sentinel is
    /// folded into the absent-path of `unsighted_scan_due`.)
    skips: u8,
    /// Unsighted calls answered WITHOUT a JVMTI heap scan (scans avoided).
    avoided: u64,
}

/// Skip budget between fallback scans: 7 skips between scans = one scan per
/// 8 unsighted calls.
const UNSIGHTED_SKIP_BUDGET: u8 = 7;

static POLL_STATE: OnceLock<Mutex<HashMap<String, PollState>>> = OnceLock::new();

// ---------------------------------------------------------------------------
// TASK-45: always-on scan-avoidance counters.
//
// The increments are bare atomic adds on the find_class path (~1 ns each);
// nothing READS them unless `CRUSSTY_SDK_STATS` is set (see
// `spawn_stats_dumper`), so with the gate off the only cost is the counters
// themselves. Schema is shared verbatim by the A' instrumentation on the
// pre-TASK-22 module (bench/bootab TASK-45 A/B), so the two arms emit
// identical lines and the deltas are apples-to-apples:
//   find_calls     — every find_class entry
//   cache_hits     — answered from the process-lifetime CACHE
//   feed_skips     — sighting gate answered "not loaded" with NO scan
//   scans          — full JVMTI GetLoadedClasses scans issued
//   classes_walked — classes visited inside those scans
// ---------------------------------------------------------------------------

/// Cumulative counters (see schema above). Monotonic, never reset.
pub struct SdkStats {
    pub find_calls: AtomicU64,
    pub cache_hits: AtomicU64,
    pub feed_skips: AtomicU64,
    pub scans: AtomicU64,
    pub classes_walked: AtomicU64,
}

/// Process-lifetime stats singleton.
pub static STATS: SdkStats = SdkStats {
    find_calls: AtomicU64::new(0),
    cache_hits: AtomicU64::new(0),
    feed_skips: AtomicU64::new(0),
    scans: AtomicU64::new(0),
    classes_walked: AtomicU64::new(0),
};

/// Cumulative snapshot `[find_calls, cache_hits, feed_skips, scans, classes_walked]`.
pub fn stats_snapshot() -> [u64; 5] {
    [
        STATS.find_calls.load(Ordering::Relaxed),
        STATS.cache_hits.load(Ordering::Relaxed),
        STATS.feed_skips.load(Ordering::Relaxed),
        STATS.scans.load(Ordering::Relaxed),
        STATS.classes_walked.load(Ordering::Relaxed),
    ]
}

/// Background dumper (TASK-45): prints cumulative sdk_stats lines at fixed
/// offsets from plugin init so an A/B reader can diff consecutive lines into
/// windows (the post-marker 60 s window = delta between the t=65 s and
/// t=120 s lines). Gated by `CRUSSTY_SDK_STATS` (default off = no thread,
/// zero output); the dumper itself never calls find_class (no self-pollution
/// of the counters).
pub fn spawn_stats_dumper() {
    let on = std::env::var("CRUSSTY_SDK_STATS")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false);
    if !on {
        return;
    }
    let _ = std::thread::Builder::new().name("cplug-sdk-stats".into()).spawn(|| {
        let mut last = 0u64;
        for t in [25u64, 65, 120, 180] {
            let wait = t - last;
            std::thread::sleep(std::time::Duration::from_secs(wait));
            last = t;
            let s = stats_snapshot();
            eprintln!(
                "[crussty-plugin] sdk_stats: t={t}s find_calls={} cache_hits={} feed_skips={} scans={} classes_walked={} (cumulative)",
                s[0], s[1], s[2], s[3], s[4]
            );
        }
    });
}

/// Hard cap on POLL_STATE entries (D2/TASK-43), sized to match the sightings
/// bound (SIGHTING_SHARDS x SIGHTING_SHARD_CAP = 65,536): the map holds one
/// entry per DISTINCT name that ever reached the gate unresolved. The SDK is
/// exported to every module and on_kernel_ready/wait_class accept arbitrary
/// names, so a module probing many never-loaded names would otherwise grow
/// the map unbounded. Bounded insert-only-below-cap (no eviction, mirroring
/// the sighting shards): at cap, NEW names are simply not recorded and their
/// gate degrades to the pre-C1 shape — every call scans (correct, just less
/// optimized; the JVMTI INITIALIZED-status guard in find_class still applies
/// to every scan, so the C1-era SIGSEGV race stays closed). Already-recorded
/// names keep their exact cadence. Dead entries (names that became
/// sighted/cached) are deliberately NOT dropped: `scans_avoided` must stay
/// readable for the one-line activation log, they are bounded by this cap,
/// and they cost ~100B each.
const POLL_STATE_CAP: usize = SIGHTING_SHARDS * SIGHTING_SHARD_CAP;

fn poll_state() -> &'static Mutex<HashMap<String, PollState>> {
    POLL_STATE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Sighting gate for find_class: returns true when a JVMTI scan must run.
///
/// D2/TASK-43: `get()` first — the old `entry(internal.to_string())`
/// allocated a key String on EVERY cache miss (every poll tick of every
/// activation poller) even when the entry already existed. The allocation
/// now happens only on the FIRST miss per name. Cadence and the `avoided`
/// counter (consumed by `scans_avoided` -> area_map/improved_noise
/// activation log lines) are identical to the previous behavior below cap.
fn unsighted_scan_due(internal: &str) -> bool {
    let mut state = poll_state().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(s) = state.get_mut(internal) {
        if s.skips >= UNSIGHTED_SKIP_BUDGET {
            s.skips = 0;
            true
        } else {
            s.skips += 1;
            s.avoided += 1;
            false
        }
    } else {
        // First miss for this name: this call IS the scan the cadence would
        // have produced (fresh entries historically started at the budget
        // and immediately reset), so the entry starts at 0. Inserted only
        // below cap — at cap the name stays unrecorded and every future
        // call returns true here (always-scan fallback, the documented
        // degraded-but-correct mode).
        if state.len() < POLL_STATE_CAP {
            state.insert(
                internal.to_string(),
                PollState {
                    skips: 0,
                    avoided: 0,
                },
            );
        }
        true
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
    STATS.find_calls.fetch_add(1, Ordering::Relaxed);
    let internal = crate::jni_util::to_internal(name);
    // D4/TASK-43: poison recovery (rationale next to `cache()`) — was
    // `.unwrap()`, a poisoned lock would unwind across JNI on hook threads.
    if let Some(addr) = cache().lock().unwrap_or_else(|e| e.into_inner()).get(&internal) {
        STATS.cache_hits.fetch_add(1, Ordering::Relaxed);
        return Some(ClassRef(*addr as jni::jclass));
    }
    // TASK-22/C1 sighting gate: while the hook pipeline has never sighted
    // this name, skip the full JVMTI heap scan and answer "not loaded" from
    // the feed. Advisory — `unsighted_scan_due` forces the bounded fallback
    // scan (first call + every 8th) so pre-hook loads are still found.
    if !unsighted_scan_due(&internal) {
        STATS.feed_skips.fetch_add(1, Ordering::Relaxed);
        return None;
    }
    let signature = format!("L{internal};");
    with_attached(|env| {
        let jvmti = Jvmti::new(crate::vm() as *mut jni::JavaVM).ok()?;
        let classes = jvmti.get_loaded_classes().ok()?;
        STATS.scans.fetch_add(1, Ordering::Relaxed);
        STATS.classes_walked.fetch_add(classes.len() as u64, Ordering::Relaxed);
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
        // D4/TASK-43: poison recovery (rationale next to `cache()`).
        cache()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
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

#[cfg(test)]
mod tests {
    use super::*;

    // POLL_STATE / CACHE are process-wide statics shared by every test in
    // this file; cargo runs tests in parallel threads, so the gate tests
    // serialize on this mutex (the cap test fills POLL_STATE, which would
    // otherwise race the cadence tests' first-insert).
    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    #[test]
    fn unsighted_gate_scans_first_then_every_eighth() {
        let _serial = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let name = "test/gate/ScanDueProbe";
        // First unsighted call scans (covers pre-hook loads, G6 window).
        assert!(unsighted_scan_due(name));
        // The next 7 are answered from the sighting feed (scans avoided).
        for _ in 0..7 {
            assert!(!unsighted_scan_due(name));
        }
        // The 8th unsighted call falls back to a bounded scan.
        assert!(unsighted_scan_due(name));
        // Exactly the 7 skipped calls were counted as avoided.
        assert_eq!(scans_avoided(name), 7);
    }

    #[test]
    fn sightings_normalize_dotted_names() {
        let name = "test/gate/NormalizeProbe";
        note_loaded(name);
        assert!(is_sighted(name));
        assert!(is_sighted("test.gate.NormalizeProbe"));
        assert!(!is_sighted("test/gate/NeverSightedProbe"));
    }

    /// D2/TASK-43: repeated misses take the get()-first path and must
    /// advance the SAME counters as the old entry()-based code — identical
    /// cadence across cycles. This is what keeps the scans-avoided number
    /// in the area_map/improved_noise activation log lines meaningful
    /// (scans_avoided is read once, after the poller exits).
    #[test]
    fn poll_state_cadence_identical_across_repeated_misses() {
        let _serial = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let name = "test/gate/HitPathProbe";
        for cycle in 1..=3 {
            assert!(unsighted_scan_due(name), "cycle {cycle}: scan due");
            for _ in 0..7 {
                assert!(!unsighted_scan_due(name), "cycle {cycle}: skip");
            }
        }
        // Exactly the 7 skipped calls of each of the 3 cycles were avoided.
        assert_eq!(scans_avoided(name), 21);
    }

    /// D2/TASK-43: at POLL_STATE_CAP the map refuses NEW entries (no panic,
    /// insert-only-below-cap) and the gate degrades to the always-scan
    /// fallback for unrecorded names — the same correct-but-less-optimized
    /// semantics as the sightings cap.
    #[test]
    fn poll_state_cap_inserts_refused_fallback_always_scans() {
        let _serial = TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        // Fill POLL_STATE to the cap.
        {
            let mut state = poll_state().lock().unwrap_or_else(|e| e.into_inner());
            let mut i = 0u64;
            while state.len() < POLL_STATE_CAP {
                state.insert(
                    format!("test/cap/Filler{i}"),
                    PollState {
                        skips: 0,
                        avoided: 0,
                    },
                );
                i += 1;
            }
        }
        // Beyond the cap: a fresh name must not panic; its gate must take
        // the fallback (scan due on EVERY call) and record nothing (the
        // scans-avoided counter stays 0 — nothing to report in the log
        // line for a never-recorded name).
        let fresh = "test/cap/BeyondCapProbe";
        assert!(unsighted_scan_due(fresh));
        assert_eq!(scans_avoided(fresh), 0, "at cap: no entry recorded");
        assert!(unsighted_scan_due(fresh), "at cap: every call scans");
        // Trim the fillers so the shared static stays small for other tests.
        poll_state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|k, _| !k.starts_with("test/cap/Filler"));
        // Below cap again, the fresh name records normally: first call scans
        // and inserts, the next call is answered from the feed.
        assert!(unsighted_scan_due(fresh));
        assert!(!unsighted_scan_due(fresh));
        assert_eq!(scans_avoided(fresh), 1);
    }

    /// D4/TASK-43: a poisoned CACHE lock must be RECOVERED, not panicked
    /// on: find_class is reachable from ClassFileLoadHook callback threads,
    /// and a panic unwinding across the JNI boundary aborts the VM. Poison
    /// deliberately by panicking in another thread while holding the lock,
    /// then call the cache-hit path of find_class (a cache hit needs no
    /// JVM, so this is testable in-process).
    #[test]
    fn poisoned_cache_lock_recovered_by_find_class() {
        let probe = "test/poison/CacheProbe";
        let addr = 0x0007_0000usize;
        // Seed the cache so find_class takes the early hit path (no JVM).
        cache()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(probe.to_string(), addr);
        // Poison the lock: panic while the guard is held in another thread.
        let poisoner = std::thread::spawn(|| {
            let _guard = cache().lock().unwrap_or_else(|e| e.into_inner());
            panic!("deliberate: poison the CACHE mutex");
        });
        let _ = poisoner.join(); // panic contained in the spawned thread
        // find_class must recover the (structurally valid) map and return
        // the cached ref — NOT panic on the poisoned lock.
        let hit = find_class(probe);
        assert!(hit.is_some(), "find_class must recover a poisoned CACHE");
        assert_eq!(hit.unwrap().as_jclass() as usize, addr);
        // The map content survived the poisoning intact.
        assert!(
            cache()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .contains_key(probe)
        );
    }
}
