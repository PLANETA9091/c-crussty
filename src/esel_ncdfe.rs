//! ESEL-NCDFE lane (AG-198 w528, iter-1) — EARLY-define machinery for the
//! per-type eindex ops-class, DORMANT scaffold.
//!
//! ROOT-CAUSE CANON (TASK-413-C, cv3-1 35712182885 / cv3-2 35712204518):
//! a retargeted call site executed against an UNDEFINED ops class resolves
//! through the caller's defining loader → ClassNotFoundException/NCDFE, and
//! HotSpot CACHES the NCDFE per constant-pool entry, repeating it for the
//! whole run even after a late define (observed span 1074..83741, x3938).
//!
//! Per-type eindex iter-2 (AG-128 plan #3 / AG-151 rust plan) will body-redirect
//! EntitySelector.addEntities invoke #297@32 (box) and #300@48 (no-box) to
//! EntitySelectorOps statics (FQN net/minecraft/commands/arguments/selector/
//! EntitySelectorOps, AG-151 FQN-коррекция). EntitySelector resolves its cp
//! entries at the FIRST selector query anywhere in the JVM — potentially far
//! before the plugin's late phase. Therefore define MUST precede retarget
//! (закон 6 v16: define → retarget → probe T1=0 → publish), enforced by the
//! HARD gate [`esel_ncdfe_gate`] that the iter-2 retarget wiring is REQUIRED
//! to consult (fail-closed: gate false = retarget must not land).
//!
//! ITER-1 SCOPE (this file): scaffold only — no bytes embedded, no call-sites,
//! no retarget, no java deltas. NCDFE T1=0 held trivially (nothing resolves
//! the class; selector_bulk.rs:340 precedent). Wiring = iter-2/w529.
//!
//! Precedents: entity_query.rs ensure_bridge_early (idempotent anchored
//! define, never-cache-failure), sb_r1.rs:259 (SelectorBulkOps EARLY-defined
//! gate), selector_bulk.rs (skeleton-dormant).

/// Canonical ops-class FQN (same package as EntitySelector → same defining
/// loader; AG-151 FQN-коррекция для EntityTypeTest НЕ меняет пакет Ops).
pub const ESEL_OPS_CLASS: &str = "net/minecraft/commands/arguments/selector/EntitySelectorOps";

/// AG-148 w528 javac-канон: 1 class file, 5068B, rc=0 vs pin e2992d63.
pub const CANON_OPS_BYTES_LEN: usize = 5068;
/// Canonical bench-kernel pin (AG-105/110/151): javap-контракт валиден на нём.
pub const CANON_KERNEL_SHA16: &str = "e2992d63abd2c254";
/// EntitySelector.class в пине: 15940B sha256 c56bf726 (AG-110/151 канон).
pub const CANON_ES_BYTES_LEN: usize = 15940;

/// Anchor preference: EntitySelector itself (exact defining loader), then the
/// boot-early kernels classes AG-110/AG-151 observed on the same loader chain.
/// First `find_class` hit wins (entity_query.rs EARLY_ANCHORS pattern).
const ESEL_ANCHORS: &[&str] = &[
    "net/minecraft/commands/arguments/selector/EntitySelector",
    "net/minecraft/server/level/ServerLevel",
    "net/minecraft/world/entity/LivingEntity",
];

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

static ESEL_OPS_DEFINED: AtomicBool = AtomicBool::new(false);
static ESEL_DEFINING: Mutex<()> = Mutex::new(());
/// T1 probe counter: NCDFE observed at the retargeted sites after define.
/// Gate requires T1 == 0 at publish (G-N5).
static NCDFE_T1: AtomicU32 = AtomicU32::new(0);

/// Retarget sites covered by this gate (AG-110 javap iter-2 contract).
pub const RETARGET_SITES: [(&str, u16, u16); 2] = [
    ("box", 297, 32),    // invoke #297 @32: (ETT,AABB,Predicate,List,I)V
    ("no-box", 300, 48), // invoke #300 @48: (ETT,Predicate,List,I)V
];

/// Mirror-drift needle (iter-1 form): length pin + FNV-1a fingerprint of the
/// embedded bytes. iter-2 MUST replace the FNV with the ledger sha256 pin of
/// the javac output BEFORE embedding (needle upgrade, G-N4 canon) — length
/// alone is a drift detector, not an identity proof.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Iter-1: bytes are NOT embedded (dormant). Iter-2 embeds the canonical
/// javac output via include_bytes! and flips this to Some.
pub static OPS_BYTES: Option<&[u8]> = None;

/// Hard publish gate for the iter-2 retarget wiring. Returns false while the
/// scaffold is dormant (bytes absent) — fail-closed: retarget MUST NOT land.
pub fn esel_ncdfe_gate() -> bool {
    ensure_esel_ops_early() && NCDFE_T1.load(Ordering::Acquire) == 0
}

/// Idempotent EARLY-define of EntitySelectorOps into the defining loader of
/// the first found anchor. Never caches failure (transient JNI hiccups
/// recoverable) except the SUCCESS latch (define_class twice = LinkageError).
/// DORMANT iter-1: returns false without touching the JVM while OPS_BYTES is
/// None (nothing to define — NCDFE T1=0 trivially held).
pub fn ensure_esel_ops_early() -> bool {
    if ESEL_OPS_DEFINED.load(Ordering::Acquire) {
        return true;
    }
    let Some(bytes) = OPS_BYTES else {
        return false; // dormant scaffold: no bytes, no define, no side effects
    };
    // Mirror-drift needle BEFORE any JVM touch (G-N4): drift = fail-closed.
    if bytes.len() != CANON_OPS_BYTES_LEN || fnv1a(bytes) == 0 {
        eprintln!(
            "[crussty-plugin] esel_ncdfe: needle DRIFT len={} — define REFUSED (fail-closed)",
            bytes.len()
        );
        return false;
    }
    let _guard = match ESEL_DEFINING.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    if ESEL_OPS_DEFINED.load(Ordering::Acquire) {
        return true;
    }
    for anchor in ESEL_ANCHORS {
        if cplug_sdk::classes::find_class(anchor).is_none() {
            continue;
        }
        let ok = cplug_sdk::jni_util::with_attached(|env| {
            // re-find INSIDE the attached context (entity_query.rs canon):
            // a jclass obtained unattached is not guaranteed valid here.
            let Some(_cls) = cplug_sdk::classes::find_class(anchor) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(_cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return false;
            };
            let gref = env.new_global_ref(loader);
            env.delete_local_ref(class_cls);
            if gref.is_null() {
                crate::describe_exception(env);
                return false;
            }
            let Some(_c) = env.define_class(ESEL_OPS_CLASS, gref, bytes) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] esel_ncdfe: define_class({ESEL_OPS_CLASS}) failed — retryable, failure NOT cached"
                );
                return false;
            };
            true
        })
        .unwrap_or(false);
        if ok {
            ESEL_OPS_DEFINED.store(true, Ordering::Release);
            eprintln!(
                "[crussty-plugin] esel_ncdfe: EARLY define ok (anchor={anchor}) — selector cp-entries safe to retarget"
            );
            return true;
        }
    }
    false
}

/// T1 probe hook for the iter-2 redirect stub: called by the fast-path
/// dispatcher on ANY Throwable from the ops lane BEFORE disarm. Counts the
/// NCDFE window breach; gate flips fail-closed (G-N5 / G6 fail-dominant).
pub fn note_ncdfe_breach() {
    NCDFE_T1.fetch_add(1, Ordering::AcqRel);
}

#[cfg(test)]
mod esel_ncdfe_tests {
    use super::*;

    #[test]
    fn dormant_when_bytes_absent() {
        assert!(OPS_BYTES.is_none());
        assert!(!esel_ncdfe_gate());
        assert!(!ensure_esel_ops_early());
    }

    #[test]
    fn retarget_sites_match_javap_contract() {
        assert_eq!(RETARGET_SITES[0], ("box", 297, 32));
        assert_eq!(RETARGET_SITES[1], ("no-box", 300, 48));
        assert_eq!(CANON_OPS_BYTES_LEN, 5068);
        assert_eq!(CANON_ES_BYTES_LEN, 15940);
        assert_eq!(CANON_KERNEL_SHA16, "e2992d63abd2c254");
    }

    #[test]
    fn fnv1a_known_vector() {
        // FNV-1a 64 of "a" = 0xaf63dc4c8601ec8c (public test vector)
        assert_eq!(fnv1a(b"a"), 0xaf63dc4c8601ec8c);
    }

    #[test]
    fn breach_counter_fail_dominant() {
        let before = NCDFE_T1.load(Ordering::Acquire);
        note_ncdfe_breach();
        assert!(NCDFE_T1.load(Ordering::Acquire) == before + 1);
    }
}
