//! Pattern hook registry + kernel-ready notifications.
//!
//! The SDK registers a single hook in the runtime pipeline; every class load is
//! dispatched here and passed to registered pattern closures. Callbacks run on
//! the class-loading thread — keep them cheap (spawn if heavy).
//!
//! Concurrency model (copy-on-write snapshots, HOTSPOT_CANDIDATES C2):
//! `dispatch`/`dispatch_bytes` are the JVM-wide ClassFileLoadHook entry point —
//! they used to take a process-wide `Mutex` on EVERY class load, serializing
//! all JVM loader threads (boot storms load thousands of classes in parallel).
//! Now each registry is a `RwLock<Arc<Snapshot>>` holding an immutable
//! snapshot: readers clone the `Arc` under a brief read guard (a refcount bump,
//! no allocation) and then glob-match + fire callbacks with NO lock held;
//! writers (cold path — registration happens during plugin activation only)
//! rebuild the whole snapshot and swap it in under the short write lock. A
//! dispatch always observes one consistent snapshot, taken before or after a
//! concurrent registration completes (same linearization as the old
//! lock-held-for-iteration design).
//!
//! ORDERING CONTRACT: hooks fire in REGISTRATION order. Registration is
//! append-only and each new snapshot preserves the previous snapshot's entry
//! order exactly, so a swap can never reorder hooks. For byte hooks the
//! contract is a chain: hook N receives hook N-1's output, or the original
//! class bytes if no earlier matching hook produced output; the final result
//! is the last produced output (or None if nothing matched/patched).
//!
//! IMPORTANT: nothing here does JNI/JVMTI work synchronously from
//! `cplugin_init` (that runs on the JVMTI OnLoad thread before the VM is
//! ready — GetLoadedClasses there can crash the boot). All class lookups run
//! on background threads.

use crate::sdk_glob;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

type Cb = Arc<dyn Fn(&str) + Send + Sync>;
type ByteCb = Arc<dyn Fn(&str, &[u8]) -> Option<Vec<u8>> + Send + Sync>;
type OnceJob = Option<Box<dyn FnOnce() + Send>>;

/// One immutable registry snapshot, shared with every concurrent reader by an
/// `Arc` clone (atomic refcount bump — no allocation on the read path).
/// Entries are `Arc`-wrapped closures so a writer can clone them out while
/// rebuilding the next snapshot.
type NameSnapshot = Arc<[(String, Cb)]>;
type ByteSnapshot = Arc<[(String, ByteCb)]>;

static HOOKS: OnceLock<RwLock<NameSnapshot>> = OnceLock::new();
static BYTE_HOOKS: OnceLock<RwLock<ByteSnapshot>> = OnceLock::new();

fn registry() -> &'static RwLock<NameSnapshot> {
    HOOKS.get_or_init(|| RwLock::new(Arc::from(Vec::new())))
}

fn byte_registry() -> &'static RwLock<ByteSnapshot> {
    BYTE_HOOKS.get_or_init(|| RwLock::new(Arc::from(Vec::new())))
}

/// Register a pattern hook: `cb(name)` fires on every class load matching
/// `pattern`. Glob syntax: `*` any run (incl. '/'), `?` one char,
/// e.g. "org/bukkit/**".
///
/// Cold path (activation only): clones the current snapshot under the write
/// lock, appends the new hook, and swaps the rebuilt snapshot in. Never blocks
/// JVM class-load readers for longer than one `Arc` refcount bump. The hook is
/// appended at the end — registration order is the firing order (see the
/// ORDERING CONTRACT in the module docs).
pub fn register(pattern: &str, cb: impl Fn(&str) + Send + Sync + 'static) {
    let reg = registry();
    let mut guard = reg.write().unwrap_or_else(|e| e.into_inner());
    let mut next = Vec::with_capacity(guard.len() + 1);
    next.extend(guard.iter().cloned());
    next.push((pattern.to_string(), Arc::new(cb) as Cb));
    *guard = Arc::from(next);
}

/// Dispatch a class name to all matching hooks.
///
/// Hot path (every JVM class load): loads the current snapshot `Arc` under a
/// brief read guard, releases it, then glob-matches and fires callbacks with
/// no lock held — class-loading threads never block each other here. No
/// allocation on this path (`Arc::clone` is a refcount bump).
pub fn dispatch(name: &str) {
    // TASK-22/C1 sighting feed: record the name BEFORE running callbacks so
    // every class load (including ones whose callbacks are not registered)
    // feeds classes::find_class's negative gate. The shard lock inside
    // note_loaded is taken and dropped here — callbacks below never run
    // under any SDK lock (lock-order rule, see classes.rs). Under the COW
    // snapshot design this holds for the whole dispatch: no registry lock is
    // ever held across callbacks or note_loaded.
    crate::classes::note_loaded(name);
    let snapshot = Arc::clone(&registry().read().unwrap_or_else(|e| e.into_inner()));
    for (pat, cb) in snapshot.iter() {
        if sdk_glob::matches(pat, name) {
            cb(name);
        }
    }
}

/// Register a byte-level hook: `cb(name, bytes)` fires on every class load
/// matching `pattern` and may return patched bytes (the SDK allocates the
/// replacement via the runtime's JVMTI allocator and hands it to the JVM).
/// Hooks chain in registration order: each gets the previous output.
///
/// Cold path — see `register` for the snapshot-swap mechanics.
pub fn register_bytes(
    pattern: &str,
    cb: impl Fn(&str, &[u8]) -> Option<Vec<u8>> + Send + Sync + 'static,
) {
    let reg = byte_registry();
    let mut guard = reg.write().unwrap_or_else(|e| e.into_inner());
    let mut next = Vec::with_capacity(guard.len() + 1);
    next.extend(guard.iter().cloned());
    next.push((pattern.to_string(), Arc::new(cb) as ByteCb));
    *guard = Arc::from(next);
}

/// Apply all byte hooks matching `name` to `data`; returns the final patched
/// bytes, or None if no hook modified them.
///
/// Hot path — lock-free like `dispatch`. The whole chain runs against ONE
/// consistent snapshot, so the registration-order chaining semantics are
/// identical to the previous lock-held design.
pub fn dispatch_bytes(name: &str, data: &[u8]) -> Option<Vec<u8>> {
    let snapshot = Arc::clone(&byte_registry().read().unwrap_or_else(|e| e.into_inner()));
    let mut current: Option<Vec<u8>> = None;
    for (pat, cb) in snapshot.iter() {
        if sdk_glob::matches(pat, name) {
            if let Some(p) = cb(name, current.as_deref().unwrap_or(data)) {
                current = Some(p);
            }
        }
    }
    current
}

/// Run `cb` (on a fresh thread) when `class_name` has loaded. Fires exactly
/// once, by background polling — no JVM work in the caller (safe to call from
/// `cplugin_init`).
pub fn on_kernel_ready(class_name: &str, cb: impl FnOnce() + Send + 'static) {
    let slot: Arc<Mutex<OnceJob>> =
        Arc::new(Mutex::new(Some(Box::new(cb) as Box<dyn FnOnce() + Send>)));
    let class_name = class_name.to_string();
    std::thread::spawn(move || {
        // Give the VM time to finish initializing before the first JNI attach;
        // AttachCurrentThread from a fresh thread during VM init can SIGSEGV.
        std::thread::sleep(std::time::Duration::from_millis(3000));
        const POLL_MS: u64 = 200;
        const MAX_MS: u64 = 120_000;
        let mut waited = 0u64;
        let found = loop {
            if let Some(c) = crate::classes::find_class(&class_name) {
                let _ = c;
                break true;
            }
            if waited >= MAX_MS {
                break false;
            }
            std::thread::sleep(std::time::Duration::from_millis(POLL_MS));
            waited += POLL_MS;
        };
        if !found {
            eprintln!("[cplug-sdk] kernel-ready wait timed out for {class_name}");
        }
        let f = slot.lock().unwrap_or_else(|e| e.into_inner()).take();
        if let Some(f) = f {
            f();
        }
    });
}

#[cfg(test)]
mod tests {
    // Tests use per-test glob namespaces ("t23-*/…") so they cannot cross-fire
    // when cargo runs them in parallel against the shared process-global
    // registries.
    use super::{dispatch, dispatch_bytes, register, register_bytes};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    /// Focused test for the COW swap (HOTSPOT_CANDIDATES C2): readers
    /// (dispatch) hammer the registries lock-free on 4 threads while a writer
    /// keeps rebuilding + swapping snapshots. Asserts no panic (a torn,
    /// reordered or lost registration would show up here), forward visibility
    /// (a hook is live on the very next dispatch after its register returns,
    /// even under concurrent reader load) and correct glob matching throughout
    /// (private per-round exact-name hooks must fire exactly once; shared
    /// hooks obey the writer-dispatch lower bound).
    #[test]
    fn concurrent_readers_while_writer_swaps_snapshots() {
        const NS: &str = "t23-concurrent";
        const SHARED_NAME: &str = "t23-concurrent/shared/Target";
        const ROUNDS: usize = 300;

        let stop = Arc::new(AtomicUsize::new(0));
        let mut readers = Vec::new();
        for _ in 0..4 {
            let stop = Arc::clone(&stop);
            readers.push(std::thread::spawn(move || {
                let mut rounds = 0usize;
                while stop.load(Ordering::Relaxed) == 0 {
                    dispatch(SHARED_NAME);
                    rounds += 1;
                }
                rounds
            }));
        }

        let writer = std::thread::spawn(move || {
            let mut private_hits = Vec::new();
            let mut shared_hits = Vec::new();
            for i in 0..ROUNDS {
                // Private hook: exact-name glob that ONLY the writer's own
                // dispatch below ever fires (readers use SHARED_NAME), so its
                // count is deterministic even with 4 readers in flight.
                let ph = Arc::new(AtomicUsize::new(0));
                let ph2 = Arc::clone(&ph);
                register(&format!("{NS}-w{i}"), move |_n| {
                    ph2.fetch_add(1, Ordering::Relaxed);
                });
                dispatch(&format!("{NS}-w{i}"));
                // Linearizability + glob correctness: registered above, must
                // fire exactly once on the next dispatch, and no other
                // registered hook may fire for this name.
                assert_eq!(ph.load(Ordering::Relaxed), 1);

                // Shared hook: matches SHARED_NAME, fired by the writer's
                // dispatch each remaining round (readers only add more hits).
                let sh = Arc::new(AtomicUsize::new(0));
                let sh2 = Arc::clone(&sh);
                register(&format!("{NS}/shared/*"), move |_n| {
                    sh2.fetch_add(1, Ordering::Relaxed);
                });
                shared_hits.push(sh);
                dispatch(SHARED_NAME);
                private_hits.push(ph);
            }
            (private_hits, shared_hits)
        });

        let (private_hits, shared_hits) = writer.join().expect("writer panicked");
        stop.store(1, Ordering::Relaxed);
        let total_rounds: usize = readers.into_iter().map(|r| r.join().unwrap()).sum();

        // Deterministic writer-side counts: each private hook fired exactly
        // once; shared hook j (0-based) fired in every writer dispatch of
        // SHARED_NAME from its registration round to the end.
        assert!(private_hits.iter().all(|c| c.load(Ordering::Relaxed) == 1));
        for (j, c) in shared_hits.iter().enumerate() {
            assert!(
                c.load(Ordering::Relaxed) >= ROUNDS - j,
                "shared hook {j} missed writer dispatches"
            );
        }
        assert!(total_rounds > 0, "readers made no progress");
        // No reader/writer panicked => lock-free readers never observed a
        // torn or reordered snapshot while the writer swapped 2*ROUNDS times.
    }

    /// Regression test for the ordering contract: hooks fire in REGISTRATION
    /// order (never glob specificity or snapshot-swap order), and byte hooks
    /// chain — hook N receives hook N-1's output, else the original bytes.
    #[test]
    fn registration_order_and_byte_chain_preserved() {
        const NAME: &str = "t23-order/Class";
        let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        // Overlapping globs registered in this order: broad first, exact
        // second. Firing order must follow registration even though the
        // exact pattern is more specific.
        let l1 = Arc::clone(&log);
        register("t23-order/*", move |n| {
            l1.lock().unwrap().push(format!("broad:{n}"));
        });
        let l2 = Arc::clone(&log);
        register("t23-order/Class", move |n| {
            l2.lock().unwrap().push(format!("exact:{n}"));
        });
        dispatch(NAME);

        // Byte hooks: first patches, second passes through, third records
        // what it received and patches again.
        let l3 = Arc::clone(&log);
        register_bytes("t23-order/*", move |_n, bytes| {
            l3.lock().unwrap().push(format!(
                "byte1:{}",
                String::from_utf8_lossy(bytes)
            ));
            Some(b"patched-1".to_vec())
        });
        let l4 = Arc::clone(&log);
        register_bytes("t23-order/*", move |_n, bytes| {
            l4.lock().unwrap().push(format!(
                "byte2:{}",
                String::from_utf8_lossy(bytes)
            ));
            None // pass-through must not reset the chain
        });
        let l5 = Arc::clone(&log);
        register_bytes("t23-order/*", move |_n, bytes| {
            l5.lock().unwrap().push(format!(
                "byte3:{}",
                String::from_utf8_lossy(bytes)
            ));
            Some(b"patched-3".to_vec())
        });
        let out = dispatch_bytes(NAME, b"original");

        let log = log.lock().unwrap();
        assert_eq!(log[0], format!("broad:{NAME}"));
        assert_eq!(log[1], format!("exact:{NAME}"));
        assert_eq!(log[2], "byte1:original");
        assert_eq!(log[3], "byte2:patched-1");
        assert_eq!(log[4], "byte3:patched-1");
        assert_eq!(out.as_deref(), Some(b"patched-3".as_slice()));
    }
}

/// TASK-22/C1 sighting-feed tests (the registry/chain order tests live in
/// `mod tests` above; these cover the classes::find_class feed only).
#[cfg(test)]
mod sighting_tests {
    use super::*;

    #[test]
    fn dispatch_records_sighting() {
        let name = "test/only/SightingFeedProbe";
        assert!(!crate::classes::is_sighted(name));
        dispatch(name);
        assert!(crate::classes::is_sighted(name));
        // Dotted form normalizes to the internal (slashed) form.
        assert!(crate::classes::is_sighted("test.only.SightingFeedProbe"));
    }
}
