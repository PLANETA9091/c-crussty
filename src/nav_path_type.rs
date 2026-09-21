//! Rust-side canonical BlockState → PathType memo for the nav subsystem
//! (TASK-401-C, vector cmp401_navsys).
//!
//! TABLE: dense flat array indexed by `Block.getId(state)` — the
//! BLOCK_STATE_REGISTRY id of the canonical (immutable, interned) state.
//! Values are `ordinal + 1` in an i8 cell (0 = uncached; PathType has < 100
//! constants). PathType-from-state is a pure function of the canonical state,
//! so the memo can never go stale — no invalidation surface exists.
//!
//! THREADING: pathfinding runs on the tick/region worker threads (multi).
//! Reads take an atomic snapshot of the table (Arc clone) + Relaxed load;
//! the rare grow (new state ids appear only at registry/boot time) happens
//! under a mutex and publishes a fresh Arc. No allocation on hits.
//!
//! FAIL-CLOSED: out-of-range/rejected ids return an error code; the Java
//! bridge (NavOps.computeVanillaPathType) recomputes vanilla-semantics and
//! simply skips the cache — behavior stays vanilla by construction.

use jvmti_bindings::jni;
use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const NAV_PROBE_MAGIC: i32 = 0x401C;
/// Hard ceiling on the id space (BLOCK_STATE_REGISTRY is ~30k; guard against
/// absurd ids so a hostile/foreign caller can't balloon memory).
const MAX_STATES: usize = 1 << 22;
const INIT_STATES: usize = 65_536;

struct Table {
    cells: Vec<AtomicI8>,
}

static TABLE: OnceLock<Mutex<Arc<Table>>> = OnceLock::new();

fn table_cell() -> &'static Mutex<Arc<Table>> {
    TABLE.get_or_init(|| Mutex::new(Arc::new(Table {
        cells: (0..INIT_STATES).map(|_| AtomicI8::new(0)).collect(),
    })))
}

fn snapshot() -> Arc<Table> {
    match table_cell().lock() {
        Ok(g) => Arc::clone(&g),
        Err(p) => Arc::clone(p.get_ref()),
    }
}

/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn nav_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    NAV_PROBE_MAGIC
}

/// # Safety
/// See nav_probe.
#[no_mangle]
pub unsafe extern "system" fn nav_type_get(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
) -> jni::jint {
    if id < 0 {
        return -1;
    }
    let t = snapshot();
    let cells = &t.cells;
    if (id as usize) >= cells.len() {
        return -1; // not cached yet (grow is done by put)
    }
    // SAFETY: id < cells.len(); Relaxed is enough — the value is a pure
    // function of the canonical state and never changes once written.
    let v = unsafe { cells.get_unchecked(id as usize) }.load(Ordering::Relaxed);
    if v == 0 {
        -1
    } else {
        (v as i32) - 1
    }
}

/// # Safety
/// See nav_probe.
#[no_mangle]
pub unsafe extern "system" fn nav_type_put(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    ordinal: jni::jint,
) -> jni::jint {
    if id < 0 || (id as usize) >= MAX_STATES || ordinal < 0 || ordinal > 126 {
        return -1;
    }
    let needed = id as usize + 1;
    let mut cur = snapshot();
    if (cur.cells.len() as i64) < needed as i64 {
        // Rare grow: new canonical ids effectively stop appearing after the
        // registry boots. Double, floor at INIT_STATES, publish a new Arc.
        let ncap = (cur.cells.len() * 2).max(INIT_STATES).max(needed).min(MAX_STATES);
        if ncap < needed {
            return -1;
        }
        let mut cells = Vec::with_capacity(ncap);
        for c in cur.cells.iter() {
            cells.push(AtomicI8::new(c.load(Ordering::Relaxed)));
        }
        while cells.len() < ncap {
            cells.push(AtomicI8::new(0));
        }
        let next = Arc::new(Table { cells });
        {
            let mut g = match table_cell().lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            *g = Arc::clone(&next);
        }
        cur = next;
    }
    if (id as usize) >= cur.cells.len() {
        return -1;
    }
    // SAFETY: bounds checked above.
    unsafe { cur.cells.get_unchecked(id as usize) }.store((ordinal + 1) as i8, Ordering::Relaxed);
    0
}
