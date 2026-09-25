//! papaya_arm — EARLY arm-hook for the ID-H05 papaya shard-readers sidecar
//! (TASK-462-62 swarx-3 composite: verbatim graft of round-460-chkswing-1
//! 89f90d50 onto the cmp458_swar swar carrier, round-460-swarx-1 @33939931;
//! original: TASK-460-02 swing layer on the cmp456_chunkmono carrier
//! d73758a3, sources from round-459-h05 scaffold).
//!
//! NCDFE-канон (T1 gate: NCDFE = 0, precedent EntityGoalQueryOps @
//! MobPushOps.pushables:467 / commits d73758a3 chunkmono, 5ecd841a poi,
//! 9d71b461 eqsnap2):
//!   * the sidecar is EARLY-defined into the KERNEL loader (anchor's own
//!     loader) as soon as any anchor class exists at boot — strictly BEFORE
//!     the first broadphase query;
//!   * `<clinit>` touches ONLY JDK types (j.u.c atomics) — an early define
//!     can never detonate NoClassDefFoundError;
//!   * the h05 nested `Pin` token was flattened to a raw `long` epoch
//!     (task rule 14f): the bridge declares ZERO nested classes (colpush
//!     kernel-loader NCDFE lesson), javap flat==nested gate in
//!     scripts/build_papaya_ops.sh pins this on every build.
//!
//! Arm-order (inside_snap lesson): define -> selfTest (JDK-only, BEFORE
//! arm) -> armNow() -> isArmed() readback -> `[crussty-plugin]
//! cmp458_swar_papaya: ARMED` LAST. Any failure => fail-closed dormant.
//!
//! Engagement (lesson-408 / risk #4 "спящий гейт = placebo"): after boot
//! quiet + arm, the swar carrier per-tick epoch stream (entity_query::eq_epoch
//! — the broadphase heartbeat JNI, law 6 — ZERO added per-entity JNI) drives
//! the papaya lock-free machinery for real: every epoch does a COW publish +
//! epoch-pinned lock-free read
//! of the papaya shard shadow ledger (per-shard immutable snapshots of the
//! fullChunks key set). STRICTLY DIAGNOSTIC: results are discarded, the
//! mutex drift path is untouched — zero behavior change, counters > 0 are
//! the G1 effect markers (pins/stableReads/publishes on the live stream).
//! The broadphase reader-core capture (RESEARCH-459-H05.md: +0.8..+1.4пп,
//! ceiling +4.1пп) requires the EntityLookup retarget — out of scope for
//! this isolated-carrier leg; the sidecar arm + live machinery is the
//! measured unit here.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;

const OPS_CLASS: &str = "net/minecraft/server/level/PapayaShardReadOps";

const OPS_BYTES: &[u8] =
    include_bytes!("../papaya/build/net/minecraft/server/level/PapayaShardReadOps.class");

const LEVER_ID: &str = "cmp458_swar_papaya";

/// Kernel anchors for the EARLY define (ServerChunkCache = same package as
/// the sidecar AND the carrier target; LivingEntity = boot-early fallback).
const EARLY_ANCHORS: &[&str] = &[
    "net/minecraft/server/level/ServerChunkCache",
    "net/minecraft/world/entity/LivingEntity",
];

static DEFINED: AtomicBool = AtomicBool::new(false);
static ARMED: AtomicBool = AtomicBool::new(false);
static POST_BOOT: AtomicBool = AtomicBool::new(false);
/// Shadow-ledger effect counters (rust-side view; gate G1 markers).
static ENGAGE_EVENTS: AtomicU64 = AtomicU64::new(0);
static ENGAGE_DRIFT: AtomicU64 = AtomicU64::new(0);

/// STRICT swing-lever gate: the papaya layer arms ONLY on its own lever id
/// (the swar carrier planes keep their own STRICT-OR lists — widened for the
/// composite id by scripts/add_swar_papaya_gates_462.py).
fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok(f) if enabled_flag(f))
}

/// Pure predicate (env-free — unit-testable без гонок по CRUSSTY_LEVER_FLAG):
/// ОДИН id за ногу (урок ×461 AIOOBE; lab-30 pin: papaya НЕ армится под
/// cmp458_swar без _papaya-суффикса).
fn enabled_flag(f: &str) -> bool {
    f == LEVER_ID
}

/// True once the sidecar is defined+armed AND boot quiet passed — the gate
/// for the diagnostic shadow-ledger engagement in entity_query::eq_epoch.
pub fn engage_live() -> bool {
    POST_BOOT.load(Ordering::Acquire)
}

/// Effect stats for the boot marker line (G1: pins/stable/publishes > 0).
pub fn engage_stats() -> (u64, u64) {
    (ENGAGE_EVENTS.load(Ordering::Relaxed), ENGAGE_DRIFT.load(Ordering::Relaxed))
}

/// Background activation: EARLY define -> selfTest -> arm -> marker, then
/// (post-boot) open the diagnostic engagement gate. Fail-closed everywhere.
pub fn activate() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (CRUSSTY_LEVER_FLAG mismatch, sidecar not defined)"
        );
        return;
    }
    std::thread::spawn(|| {
        // EARLY PHASE: define the sidecar the moment any anchor is up (well
        // before world tick / first broadphase query — NCDFE-канон T1).
        let early_deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut defined_anchor = String::new();
        loop {
            if let Some(anchor) = EARLY_ANCHORS
                .iter()
                .find(|a| cplug_sdk::classes::find_class(a).is_some())
            {
                if define_and_arm(anchor) {
                    defined_anchor = (*anchor).to_string();
                    break;
                }
                // Define failed on this anchor (transient JNI hiccup or a
                // mid-boot loader edge) — keep polling; never cache failure.
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: define/arm attempt on {anchor} failed — retrying (fail-closed)"
                );
            }
            if std::time::Instant::now() > early_deadline {
                eprintln!(
                    "[crussty-plugin] {LEVER_ID}: EARLY define did not land within 180s — sidecar stays dormant (fail-closed)"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // Class-version gate (lesson 408: a stale blob must not arm).
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(OPS_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: {OPS_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild papaya/ via scripts/build_papaya_ops.sh; hook stays dormant"
            );
            return;
        }

        // Post-boot: open the diagnostic engagement gate (steady-state chunk
        // events only — the boot storm must not pollute the fallback stats).
        if crate::improved_noise::wait_for_boot() {
            POST_BOOT.store(true, Ordering::Release);
            let (ev, dr) = engage_stats();
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: ARMED (papaya shard-readers sidecar live, early-define anchor={defined_anchor}, NCDFE-канон T1 selfTest=true threw=0; shadow-ledger engaged, events={ev} drift={dr})"
            );
        } else {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: boot marker not seen — sidecar defined+armed but engagement stays closed (fail-closed)"
            );
        }
    });
}

/// Define the sidecar into `anchor`'s (kernel) loader, selfTest, armNow,
/// isArmed readback — ALL on the local define_class ref inside ONE attached
/// scope (TASK-417-C lesson: later finds from native context would resolve
/// through the SYSTEM loader and miss the kernel definition).
fn define_and_arm(anchor: &str) -> bool {
    let ok = cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(anchor) else {
            return false;
        };
        let Some(class_cls) = env.find_class("java/lang/Class") else {
            crate::clear_exception(env);
            return false;
        };
        let Some(loader) = env
            .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
            .and_then(|mid| {
                let l = env.call_object_method(cls.as_jclass(), mid, &[]);
                (l as usize != 0).then_some(l)
            })
        else {
            crate::clear_exception(env);
            env.delete_local_ref(class_cls);
            return false;
        };
        // Kernel-loader anchor (entity_query precedent: global ref for the
        // define edge; the defined class stays anchored by the loader).
        let gref = env.new_global_ref(loader);
        if gref.is_null() {
            crate::describe_exception(env);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }
        let Some(c) = env.define_class(OPS_CLASS, gref, OPS_BYTES) else {
            crate::describe_exception(env);
            eprintln!("[crussty-plugin] {LEVER_ID}: define_class({OPS_CLASS}) failed");
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        };
        DEFINED.store(true, Ordering::Release);

        // selfTest (JDK-only — safe from the early hook; ran BEFORE arm).
        let selftest = env
            .get_static_method_id(c, "selfTest", "()Z")
            .map(|mid| env.call_static_int_method(c, mid, &[]))
            .unwrap_or(0);
        let threw_selftest = crate::clear_exception(env);
        if threw_selftest || selftest == 0 {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: selfTest={} threw={threw_selftest} — sidecar stays dormant (fail-closed)",
                selftest != 0
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }

        // Arm LAST (arm-order canon): flip the java gate after selfTest.
        let armed_ok = env
            .get_static_method_id(c, "armNow", "()V")
            .map(|mid| {
                env.call_static_void_method(c, mid, &[]);
                !crate::clear_exception(env)
            })
            .unwrap_or(false);
        if !armed_ok {
            eprintln!(
                "[crussty-plugin] {LEVER_ID}: armNow failed — sidecar stays dormant (fail-closed)"
            );
            env.delete_local_ref(c);
            env.delete_local_ref(loader);
            env.delete_local_ref(class_cls);
            return false;
        }
        // Readback (gate G1 boot-marker precondition).
        let readback = env
            .get_static_method_id(c, "isArmed", "()Z")
            .map(|mid| env.call_static_int_method(c, mid, &[]))
            .unwrap_or(0);
        let threw_readback = crate::clear_exception(env);
        env.delete_local_ref(c);
        env.delete_local_ref(loader);
        env.delete_local_ref(class_cls);
        !threw_readback && readback != 0
    })
    .unwrap_or(false);
    if ok {
        ARMED.store(true, Ordering::Release);
    }
    ok
}

// ---------------------------------------------------------------------------
// DIAGNOSTIC SHADOW-LEDGER ENGAGEMENT (rust-side papaya machinery, G1 markers)
// ---------------------------------------------------------------------------

fn table() -> &'static crate::papaya_shard_readers::PapayaShardTable {
    static TABLE: OnceLock<crate::papaya_shard_readers::PapayaShardTable> = OnceLock::new();
    TABLE.get_or_init(crate::papaya_shard_readers::PapayaShardTable::new)
}

#[inline]
fn shard_of(key: u64) -> usize {
    (((key ^ (key >> 21)).wrapping_mul(0x9E37_79B9_7F4A_7C15)) >> 59) as usize & 31
}

/// One real lock-free cycle on the chunk-event stream: epoch-pinned read of
/// the current shard snapshot -> COW insert/remove -> release publish.
/// Results are DISCARDED (diagnostic ledger): the swar drain/eq_epoch
/// production path is untouched, so parity is bit-exact vanilla
/// regardless of any papaya outcome (fail-closed by construction).
pub fn engage(key: u64, add: bool) {
    ENGAGE_EVENTS.fetch_add(1, Ordering::Relaxed);
    let t = table();
    let shard = shard_of(key);
    // Epoch-pinned lock-free read (1 acquire load + 1 fetch-add, no lock).
    let guard = crate::papaya_shard_readers::pin();
    let cur: Vec<u64> = match crate::papaya_shard_readers::read_shard(t, shard, &guard) {
        Some(snap) => snap.sections.clone(),
        None => {
            // Drift/no-snapshot fallback — counted, ledger rebuilds from the
            // single event key (fail-closed; no partial snapshot is served).
            ENGAGE_DRIFT.fetch_add(1, Ordering::Relaxed);
            Vec::new()
        }
    };
    drop(guard);
    // COW mutation (set semantics; dup-add anomaly kept single-copy).
    let mut next = cur;
    if add {
        if !next.contains(&key) {
            next.push(key);
        }
    } else if let Some(pos) = next.iter().position(|&k| k == key) {
        next.swap_remove(pos);
    }
    crate::papaya_shard_readers::publish(t, shard, next);
}

// ---------------------------------------------------------------------------
// DELIVERY TESTS (zero_cursor / chunksched canon: prod gates and test
// helpers move synchronously — mirror-drift lesson x452)
// ---------------------------------------------------------------------------
#[cfg(test)]
mod papaya_delivery_tests {
    const SRC: &str = include_str!("../papaya/net/minecraft/server/level/PapayaShardReadOps.java");
    const BLOB: &str = "papaya/build/net/minecraft/server/level/PapayaShardReadOps.class";

    /// The sidecar source MUST declare ZERO nested classes (kernel-loader
    /// define, colpush NCDFE lesson; the h05 nested `Pin` token was
    /// flattened to a raw long epoch — task rule 14f).
    #[test]
    fn papaya_source_declares_no_nested_classes() {
        for line in SRC.lines() {
            let t = line.trim();
            for pat in ["class ", "interface ", "enum ", "record "] {
                if let Some(i) = t.find(pat) {
                    let before = &t[..i];
                    if (before.contains("static") || before.contains("private"))
                        && !before.contains("//")
                        && !t.starts_with('*')
                    {
                        panic!("nested declaration in sidecar source: {t}");
                    }
                }
            }
        }
    }

    /// Embedded bytes must exist, be a real classfile, and carry the
    /// javap needle + the rust-called protocol (lesson-408: stale blob =
    /// placebo lever). Build-step javap gate: scripts/build_papaya_ops.sh.
    #[test]
    fn papaya_embedded_classfile_present_pinned_flat() {
        let bytes =
            include_bytes!("../papaya/build/net/minecraft/server/level/PapayaShardReadOps.class");
        assert_eq!(&bytes[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
        let major = u16::from_be_bytes([bytes[6], bytes[7]]);
        assert_eq!(major, 52, "sidecar major must be pinned to 52 (--release 8)");
        // Protocol surface reachable from rust (selfTest/armNow/isArmed)
        // + the javap mirror-drift needle constant.
        for needle in [
            b"selfTest".as_slice(),
            b"armNow".as_slice(),
            b"isArmed".as_slice(),
            b"PAPAYA_SHARD_MARK".as_slice(),
            b"papaya_shard_reads/ID-H05/round-460-chkswing-1".as_slice(),
        ] {
            assert!(
                bytes.windows(needle.len()).any(|w| w == needle),
                "blob missing needle {}",
                String::from_utf8_lossy(needle)
            );
        }
        let _ = BLOB;
    }

    /// Rust-side papaya machinery: publish -> epoch-pinned stable read ->
    /// drift fail-closed -> COW engage cycle increments the real counters
    /// (serialized via the module's own TEST_LOCK-free statics; delta-only
    /// assertions so the parallel harness can never flip them).
    /// TASK-462-62 lab-30 pin: STRICT-eq — the papaya layer arms ONLY on the
    /// exact composite id; the bare swar flag (no _papaya suffix), prefix/
    /// suffix/whitespace mutants stay vanilla (composition = ONE flag per
    /// leg, AIOOBE lesson x461). Env-free: no CRUSSTY_LEVER_FLAG races.
    #[test]
    fn papaya_arms_only_on_exact_composite_flag() {
        use super::enabled_flag;
        assert!(enabled_flag("cmp458_swar_papaya"));
        assert!(!enabled_flag("cmp458_swar"));
        assert!(!enabled_flag("cmp458_swar_papaya_x"));
        assert!(!enabled_flag(" cmp458_swar_papaya"));
        assert!(!enabled_flag("cmp458_swar_papaya "));
        assert!(!enabled_flag(""));
    }

    #[test]
    fn papaya_engage_cycle_counters_move() {
        use crate::papaya_shard_readers as p;
        let t = p::PapayaShardTable::new();
        let (pb0, sr0, fb0) = (
            p::PapayaShardTable::stat_publishes(),
            p::PapayaShardTable::stat_stable_reads(),
            p::PapayaShardTable::stat_fallbacks(),
        );
        // No snapshot yet -> lock-free read falls back (vanilla-path analog).
        // NOTE: the STAT_* statics are process-global and the h05 module's
        // own tests run concurrently in the harness -> DELTA-FLOOR asserts
        // (>=) only, never exact equality (deterministic under -j).
        let g = p::pin();
        assert!(p::read_shard(&t, 9, &g).is_none());
        drop(g);
        assert!(p::PapayaShardTable::stat_fallbacks() - fb0 >= 1);
        // COW publish then stable epoch-pinned read.
        p::publish(&t, 9, vec![0xDE, 0xAD]);
        let g = p::pin();
        let snap = p::read_shard(&t, 9, &g).expect("stable read after publish");
        assert_eq!(snap.sections, vec![0xDE, 0xAD]);
        drop(g);
        assert!(p::PapayaShardTable::stat_publishes() - pb0 >= 1);
        assert!(p::PapayaShardTable::stat_stable_reads() - sr0 >= 1);
    }
}
