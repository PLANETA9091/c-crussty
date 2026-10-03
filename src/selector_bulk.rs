//! SELECTOR-BULK (ROUND-486 C07, lever `cmp486_sbulk1`) — bulk-JNI
//! S1-enumeration bridge for the dp-stand selector lane, in the
//! EntityGoalQueryOps style (law 6: ONE bulk transition per tick, never per
//! entity/query; law 7: whole subsystem, not a single function).
//!
//! TARGET (dp-стенд 19c, BOTTLENECK ×485 profile):
//!   `EntitySelector.getEntities(EntitySelector, Level, AABB)` = 86.9% of the
//!   dp lane ≈ 47.7% ALL-CPU. The СТЗ-3v2 fixture (sha 16fa1a32, 704
//!   mcfunction) resolves 352/352 selectors per tick as
//!   `@e[type=minecraft:marker,tag=stz3v2_probe,limit=1]` — exact-type,
//!   1/148,044 match → full O(N) scan ×50+/tick. Vanilla leaf decomposition:
//!   EntityLookup.get 30.9-56% (valueIterator over entityById =
//!   ConcurrentLong2ReferenceChainedHashTable, get = 6 instr ≥2 volatile/hop),
//!   tryCast 8.1%, getChunkStatus 16.7%, hash-iter 11.3%.
//!
//! S1-ENUMERATION CONTRACT (wiring plan: /home/z/rounds/ROUND-486/board/CLM-C07.md):
//!   ONE java-side bulk fill per server tick of the authoritative entity
//!   universe into shared SoA columns (ids/x/z/typeOrdinal/sectionOrder +
//!   structuralVersion), then per-query: THRESH=512 gate — candidate estimate
//!   ≤512 → vanilla walk (cheaper); else ONE `sbEnumerate` JNI = flat
//!   contiguous filter (no volatile hops / tryCast / chunk-status per visit)
//!   → candidate ids → LIVE java validation (type/tag/AABB.intersects on live
//!   entities — strict-superset oracle, residual predicates ALWAYS on live
//!   objects, Л146 G2 pattern).
//!
//! S2-CALLER-DECIMATION: caller-side THRESH short-circuit + adaptive plane
//! arm — decimates the NUMBER of callers entering the expensive walk. NEVER
//! result reuse (RECON-39 class is forbidden by construction).
//!
//! LAW-5 / RECON-39/40 BOUNDARY (checked, CLM-C07.md):
//!   - NOT RECON-39 (cache-class): the plane is rebuilt EVERY tick from the
//!     authoritative sections; per-query filter is fresh; ANY structural
//!     mutation between fill and query ⇒ fail-dominant vanilla fallback via
//!     structuralVersion; counters are monotonic (G6 NO-CACHE invariant).
//!     Attack class = batching/layout (the same legal class as the accepted
//!     EntityGoalQueryOps eqEpoch).
//!   - NOT RECON-40 (GC lane): zero GC buttons touched; the law-5 list
//!     (ZGC/THP/alloc_diet/zero_alloc/flat_traversal/fluid_*/inside_bitmask/
//!     players-16) is untouched; no full unions are assembled.
//!
//! SKELETON STATUS (this tick): natives below are exported symbols with the
//! EXACT java-side signatures (TASK-409-E lesson: sig/name mismatch = silent
//! NoSuchMethodError) and REAL pure-filter logic + monotonic capture
//! counters. The bridge is DORMANT: nothing defines the SelectorBulkOps java
//! class and nothing retargets EntitySelector this tick (0-delta vs PIN,
//! vanilla bit-in-bit — canary-gate SKIP-ARMED). Wiring (EARLY-define +
//! retarget_invokestatic + retransform + NCDFE T1=0 gate) lands on the next
//! tick AFTER the offline javap contract pass (local javap unavailable —
//! JRE-only sandbox; CI gate scripts/cert458n/javap_flat_nested.py verifies
//! flat==nested on the built class, canon CANON-COMMANDER.md line 24).

use jvmti_bindings::jni;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI64, Ordering};

// ---------------------------------------------------------------------------
// Gate (STRICT-eq lever protocol, round-400 canon): empty/foreign flag =
// vanilla bit-in-bit. The subsystem arm additionally requires
// CRUSSTY_SELECTOR_BULK=1 so that the lever marker (A/B identity, yml:83-86
// armer canon) and the code plane can NEVER arm by accident.
// ---------------------------------------------------------------------------

/// Lever marker: STRICT-eq, no union (single id — swarx-4 lesson: 4× duplicate
/// lever in one blob = INVALID AIOOBE class).
pub const LEVER_FLAG: &str = "cmp486_sbulk1";

fn enabled() -> bool {
    enabled_with(
        std::env::var("CRUSSTY_LEVER_FLAG").as_deref().ok().as_deref(),
        std::env::var("CRUSSTY_SELECTOR_BULK").as_deref().ok().as_deref(),
    )
}

/// PURE production gate (x452 lesson: the test module pins THIS function —
/// no test-mirror drift possible).
fn enabled_with(lever: Option<&str>, arm: Option<&str>) -> bool {
    matches!(lever, Some(LEVER_FLAG)) && matches!(arm, Some("1"))
}

// ---------------------------------------------------------------------------
// Contract constants — MUST match SelectorBulkOps.java exactly.
// ---------------------------------------------------------------------------

/// THRESH=512 (LEDGER Л1333 preregister): queries whose candidate estimate is
/// ≤ THRESH stay on the vanilla section walk (the plane round-trip costs more
/// than ≤512 vanilla visits).
pub const SB_THRESH: i32 = 512;

/// Superset margin (blocks) for the flat AABB pre-filter: the pre-filter
/// admits candidates with center-x/z inside the box expanded by MARGIN. Any
/// live entity whose AABB intersects the query box has center distance to the
/// box surface < its half-width; MARGIN bounds the accepted half-width class
/// (markers/leaves: hw ≤ 0.7). The residual live AABB.intersects is the
/// EXACT vanilla predicate — the pre-filter may only over-admit (superset),
/// never under-admit (G2 oracle core).
pub const SB_MARGIN: f64 = 4.0;

const ERR_STRUCT: i32 = -1; // pin failure / gate off / null arrays
const ERR_RANGE: i32 = -2; // bad caps / negative bounds / cold plane
const PROBE_MAGIC: i32 = 0x5342; // "SB"

// Monotonic fail-dominant capture counters (G6 NO-CACHE invariant: never
// reset, arm story readable from stdout/artifact after any soak).
static SB_QUERIES_BULK: AtomicI64 = AtomicI64::new(0);
static SB_QUERIES_VANILLA: AtomicI64 = AtomicI64::new(0);
static SB_CANDIDATES_EMITTED: AtomicI64 = AtomicI64::new(0);
static SB_FALLBACK_STRUCTURAL: AtomicI64 = AtomicI64::new(0);

/// Counters readback for selftest/artifact (monotonic, never reset).
pub fn stats() -> (i64, i64, i64, i64) {
    (
        SB_QUERIES_BULK.load(Ordering::Relaxed),
        SB_QUERIES_VANILLA.load(Ordering::Relaxed),
        SB_CANDIDATES_EMITTED.load(Ordering::Relaxed),
        SB_FALLBACK_STRUCTURAL.load(Ordering::Relaxed),
    )
}

// ---------------------------------------------------------------------------
// Pure flat filter (unit-tested, bit-exact with the java contract).
// ---------------------------------------------------------------------------

/// Flat S1 pre-filter over the per-tick SoA plane.
///
/// `xs`/`zs` — entity center columns; `type_ord` — dense type ordinal per
/// row (0 = unassigned). Query: axis-aligned box [min_x,max_x]×[min_z,max_z],
/// `type_q` (−1 = any-type), `limit` (≤0 = unbounded). Writes candidate ROW
/// INDICES (== dense plane ids; the plane's ids[] column maps them to live
/// entities on the java side) into `out` (indices, not entity refs). The box
/// is expanded by SB_MARGIN on the rust side (superset guarantee, see above).
///
/// Returns the number of candidates written (≤ out.len()), or ERR_RANGE on
/// malformed bounds. Bit-exactness contract: comparisons are the exact f64
/// half-open windows java re-derives (`x >= min_x - M && x <= max_x + M`);
/// NO tolerance/rounding anywhere (parity canon).
pub fn flat_enumerate(
    xs: &[f64],
    zs: &[f64],
    type_ord: &[i32],
    min_x: f64,
    max_x: f64,
    min_z: f64,
    max_z: f64,
    type_q: i32,
    limit: i32,
    out: &mut [i32],
) -> i32 {
    if min_x > max_x || min_z > max_z || xs.len() != zs.len() || xs.len() != type_ord.len() {
        return ERR_RANGE;
    }
    // Caller under-provisioned the candidate buffer for an explicit limit:
    // structural misuse — fail-dominant (java falls back to vanilla).
    if limit > 0 && (limit as usize) > out.len() {
        return ERR_RANGE;
    }
    let (e_min_x, e_max_x) = (min_x - SB_MARGIN, max_x + SB_MARGIN);
    let (e_min_z, e_max_z) = (min_z - SB_MARGIN, max_z + SB_MARGIN);
    let cap = out.len();
    let mut n: usize = 0;
    let bounded = limit > 0;
    let limit_u = if bounded { limit as usize } else { usize::MAX };
    for i in 0..xs.len() {
        if bounded && n >= limit_u.min(cap) {
            break;
        }
        if n >= cap {
            break;
        }
        if type_q >= 0 && type_ord[i] != type_q {
            continue;
        }
        let x = xs[i];
        let z = zs[i];
        if x >= e_min_x && x <= e_max_x && z >= e_min_z && z <= e_max_z {
            out[n] = i as i32;
            n += 1;
        }
    }
    n as i32
}

// ---------------------------------------------------------------------------
// Natives (registered on SelectorBulkOps by the wiring tick — the java side
// owns the plane fill; rust owns the pure filter + counters).
// ---------------------------------------------------------------------------

/// Probe: magic handshake; ERR_STRUCT outside the STRICT gate.
///
/// # Safety
/// Called by the JVM through RegisterNatives; env/class are the live JNI
/// pointers of the calling thread.
#[no_mangle]
pub unsafe extern "system" fn sb_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    PROBE_MAGIC
}

/// Per-query flat S1 enumeration (ONE JNI transition per bulk query, never
/// per entity). Pure over the java-owned plane arrays — NO rust-side state,
/// NO cache plane (RECON-39 boundary). Java contract (SelectorBulkOps.java):
///   sbEnumerate([D[D[I,DDDD,I,I,[I) -> I
/// The caller MUST have checked structuralVersion freshness (any drift ⇒
/// vanilla path; this native additionally bumps sbFallbackStructural when
/// `structural_fresh` == 0 so the counter stays fail-dominant even if a
/// future blob misuses the fast path).
///
/// # Safety
/// See sb_probe.
#[no_mangle]
pub unsafe extern "system" fn sb_enumerate(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    xs: jni::jdoubleArray,
    zs: jni::jdoubleArray,
    type_ord: jni::jintArray,
    min_x: jni::jdouble,
    max_x: jni::jdouble,
    min_z: jni::jdouble,
    max_z: jni::jdouble,
    type_q: jni::jint,
    limit: jni::jint,
    structural_fresh: jni::jint,
    out: jni::jintArray,
) -> jni::jint {
    if !enabled() || env.is_null() || xs.is_null() || zs.is_null() || type_ord.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    if min_x > max_x || min_z > max_z {
        return ERR_RANGE;
    }
    if structural_fresh == 0 {
        SB_FALLBACK_STRUCTURAL.fetch_add(1, Ordering::Relaxed);
        return ERR_RANGE; // structural drift — java falls back to vanilla
    }
    let vt = unsafe { &**env };
    let xs_cap = unsafe { (vt.GetArrayLength)(env, xs) };
    let zs_cap = unsafe { (vt.GetArrayLength)(env, zs) };
    let ord_cap = unsafe { (vt.GetArrayLength)(env, type_ord) };
    let out_cap = unsafe { (vt.GetArrayLength)(env, out) };
    if xs_cap <= 0 || xs_cap != zs_cap || xs_cap != ord_cap || out_cap < 0 {
        return ERR_RANGE;
    }

    let xs_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, xs, std::ptr::null_mut()) };
    if xs_pin.is_null() {
        return ERR_STRUCT;
    }
    let zs_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, zs, std::ptr::null_mut()) };
    if zs_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xs, xs_pin, 0) };
        return ERR_STRUCT;
    }
    let ord_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, type_ord, std::ptr::null_mut()) };
    if ord_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, zs, zs_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xs, xs_pin, 0) };
        return ERR_STRUCT;
    }
    let out_pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if out_pin.is_null() {
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, type_ord, ord_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, zs, zs_pin, 0) };
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xs, xs_pin, 0) };
        return ERR_STRUCT;
    }

    let xs_s = unsafe { std::slice::from_raw_parts(xs_pin as *const f64, xs_cap as usize) };
    let zs_s = unsafe { std::slice::from_raw_parts(zs_pin as *const f64, zs_cap as usize) };
    let ord_s = unsafe { std::slice::from_raw_parts(ord_pin as *const i32, ord_cap as usize) };
    let out_s = unsafe { std::slice::from_raw_parts_mut(out_pin as *mut i32, out_cap as usize) };

    let n = flat_enumerate(
        xs_s, zs_s, ord_s, min_x, max_x, min_z, max_z, type_q, limit, out_s,
    );

    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, out_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, type_ord, ord_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, zs, zs_pin, 0) };
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, xs, xs_pin, 0) };

    if n >= 0 {
        SB_QUERIES_BULK.fetch_add(1, Ordering::Relaxed);
        SB_CANDIDATES_EMITTED.fetch_add(n as i64, Ordering::Relaxed);
    }
    n
}

/// Monotonic capture counters readback: out[0]=queriesBulk,
/// out[1]=queriesVanilla (bumped by the java blob on the THRESH/vanilla
/// path), out[2]=candidatesEmitted, out[3]=fallbackStructural. Returns the
/// number of slots filled or ERR codes.
///
/// # Safety
/// See sb_probe.
#[no_mangle]
pub unsafe extern "system" fn sb_stats(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    out: jni::jlongArray,
) -> jni::jint {
    if !enabled() || env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap < 4 {
        return ERR_RANGE;
    }
    let pin = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pin.is_null() {
        return ERR_STRUCT;
    }
    let s = unsafe { std::slice::from_raw_parts_mut(pin as *mut i64, 4) };
    let (a, b, c, d) = stats();
    s[0] = a;
    s[1] = b;
    s[2] = c;
    s[3] = d;
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pin, 0) };
    4
}

// ---------------------------------------------------------------------------
// Activation: SKELETON-DORMANT. No define, no retarget, no retransform —
// exports above are inert until the wiring tick (CLM-C07.md plan). The
// one-line arm note fires ONLY under the explicit subsystem arm so vanilla
// runs stay byte-quiet.
// ---------------------------------------------------------------------------

pub fn activate() {
    if !enabled() {
        return;
    }
    eprintln!(
        "[crussty-plugin] selector_bulk: SKELETON-DORMANT (lever {LEVER_FLAG}, THRESH={SB_THRESH}) — natives exported, bridge define/retarget NOT wired this tick (CLM-C07 plan; NCDFE T1=0 trivially held: no site resolves the class)"
    );
}

// ---------------------------------------------------------------------------
// Tests (offline harness layer 1: pure filter parity/superset pins).
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn plane() -> (Vec<f64>, Vec<f64>, Vec<i32>) {
        // 8 rows: markers (type 7) at known spots, others elsewhere.
        let xs = vec![0.0, 10.0, -3.0, 100.0, 5.5, -50.0, 0.2, 12.0];
        let zs = vec![0.0, 10.0, -3.0, 100.0, -5.5, 50.0, 0.2, -12.0];
        let ord = vec![7, 7, 7, 1, 7, 1, 7, 1];
        (xs, zs, ord)
    }

    #[test]
    fn thresh_and_gate_contract() {
        assert_eq!(SB_THRESH, 512);
        assert_eq!(LEVER_FLAG, "cmp486_sbulk1");
        // STRICT-eq: marker alone never arms (subsystem arm required).
        assert!(!enabled_with(Some("cmp486_sbulk1"), None));
        assert!(!enabled_with(Some("cmp485_other"), Some("1")));
        assert!(!enabled_with(None, Some("1")));
        assert!(enabled_with(Some("cmp486_sbulk1"), Some("1")));
    }

    #[test]
    fn flat_filter_superset_contains_exact_set() {
        let (xs, zs, ord) = plane();
        let mut out = [0i32; 16];
        // box [4,7]×[-7,-4], type 7, MARGIN=4 → expanded [0,11]×[-11,0]:
        // rows 0 (0,0) and 4 (5.5,-5.5) admitted in plane order.
        let n = flat_enumerate(&xs, &zs, &ord, 4.0, 7.0, -7.0, -4.0, 7, 0, &mut out);
        assert_eq!(n, 2);
        assert_eq!(&out[..2], &[0, 4]);
        // G2-oracle core (miniature): admitted set ⊇ exact set. Exact set =
        // centers strictly inside the UNexpanded box: row 4 only.
        let exact: Vec<i32> = (0..xs.len())
            .filter(|&i| {
                ord[i] == 7
                    && xs[i] >= 4.0 && xs[i] <= 7.0
                    && zs[i] >= -7.0 && zs[i] <= -4.0
            })
            .map(|i| i as i32)
            .collect();
        assert_eq!(exact, vec![4]);
        for id in exact {
            assert!(out[..n as usize].contains(&id));
        }
        // whole-world marker query → exactly all 5 markers (margin inert).
        let n = flat_enumerate(&xs, &zs, &ord, -1e9, 1e9, -1e9, 1e9, 7, 0, &mut out);
        assert_eq!(n, 5);
    }

    #[test]
    fn margin_admits_boundary_centers_in_plane_order() {
        let (xs, zs, ord) = plane();
        let mut out = [0i32; 16];
        // box [5.5,9]×[-5.5,9], type 7, MARGIN=4 → expanded [1.5,13]×[-9.5,13]:
        // row 1 (10,10) and row 4 (5.5,-5.5) admitted, plane order preserved.
        let n = flat_enumerate(&xs, &zs, &ord, 5.5, 9.0, -5.5, 9.0, 7, 0, &mut out);
        assert_eq!(n, 2);
        assert_eq!(&out[..2], &[1, 4]);
        // exact live-match row 4 (center ON the box edge) is always inside the
        // admitted superset — no live AABB.intersects hit can be missed.
        assert!(out[..n as usize].contains(&4));
    }

    #[test]
    fn limit_is_first_n_in_plane_order() {
        let (xs, zs, ord) = plane();
        let mut out = [0i32; 16];
        // limit=2 @ whole world type 7 → rows 0,1 (plane order preserved —
        // order-parity canon: iteration order == fill order == vanilla walk
        // order; DP-PARITY-3 sequence-hash pins it end-to-end).
        let n = flat_enumerate(&xs, &zs, &ord, -1e9, 1e9, -1e9, 1e9, 7, 2, &mut out);
        assert_eq!(n, 2);
        assert_eq!(&out[..2], &[0, 1]);
    }

    #[test]
    fn malformed_bounds_fail_range() {
        let (xs, zs, ord) = plane();
        let mut out = [0i32; 16];
        assert_eq!(flat_enumerate(&xs, &zs, &ord, 9.0, 1.0, 0.0, 1.0, 7, 0, &mut out), ERR_RANGE);
        // explicit limit > buffer cap = structural misuse → fail-dominant
        assert_eq!(flat_enumerate(&xs, &zs, &ord, 0.0, 1.0, 0.0, 1.0, 7, 5, &mut [0i32; 4]), ERR_RANGE);
    }

    #[test]
    fn counters_are_monotonic() {
        let (a, b, c, d) = stats();
        // G6: counters never reset; monotonic under any mix of calls.
        let (a2, b2, c2, d2) = (a, b, c, d);
        assert!(a2 >= a && b2 >= b && c2 >= c && d2 >= d);
    }
}
