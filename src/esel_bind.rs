//! ESEL-BIND (AG-245 w529, iter-2 RUST side of the sel/getEnt plane —
//! lever `cmp529_esel`) — per-type view binding + ARM protocol for the
//! ESEL-C3 fast path (java iter-1 already in master @806fe8e4:
//! entityquery/net/minecraft/world/entity/EntityIndexOps.java).
//!
//! JAVA CONTRACT (EntityIndexOps.java iter-1, verbatim obligations):
//!  - `ESEL_VIEW`: "Bound by the rust manager after per-type chains go live
//!    (iter-2)" — this module is that binder;
//!  - `ESEL_ARMED`: "Flipped by the rust manager AFTER chains are live
//!    (never in iter-1)" — flip is the LAST step, after define-bridge live +
//!    rust selftest + java publish all succeeded (ARM-AFTER-DEFINE canon,
//!    esel_ncdfe.rs: define → probe → publish → arm);
//!  - view semantics: typeCount = EXACT count of live entities of the type
//!    in slices with status.isOrAfter(FULL) — the population the vanilla
//!    rect walk can reach; typeSingle = the single entity under count==1;
//!  - fail-closed java side: ESEL_VIEW==null || !ESEL_ARMED || eselBroken →
//!    counts-skip walk; any Throwable → eselBroken sticky.
//!
//! BIND PATH (bulk-bind on the epoch event): the rust manager recomputes
//! per-type chains over the FULL-slice population, validates the singleton
//! invariants, publishes the view, then — strictly after — flips ESEL_ARMED.
//! Bind is idempotent per epoch (double-bind = no-op), epochs are monotonic
//! (stale bind/unbind refused), any invariant defect is sticky-broken (G6
//! fail-dominant — a mis-built chain would produce silent wrong query
//! results, so the lane must not recover in-process).
//!
//! JAVA-PUBLISHER REALITY (iter-2): the iter-1 blob has NO static publisher
//! (no `eselPublish`, no `eselArmNow` — `ESEL_VIEW` is a public static
//! volatile TypeIndexView FIELD; filling it needs a java-side constructor).
//! Java blobs are FROZEN this tick (owner mandate: zero java deltas) → the
//! runtime ladder PROBES the two method ids read-only (GetStaticMethodID,
//! exception-cleared) and honest-stops on absence with a sticky
//! `publisher_missing` latch + the iter-3 site-spec (work/AG-245/result.md).
//! ZERO java mutation on this path: an ARMED flip without a live view would
//! NPE in eselFast → eselBroken sticky → the whole ESEL plane burned for
//! iter-3 (rust must not break the java contract). The full
//! publish_and_arm_java push path (arrays → eselPublish → eselArmNow) is
//! implemented and becomes reachable the moment the iter-3 blob lands AND
//! per-type chains go live (es_pt wiring).
//!
//! LEVER: STRICT-eq `cmp529_esel` (round-400 protocol; один id, никаких
//! союзов по env — swarx-4 урок). DORMANT by default: flag unset/foreign →
//! register/activate no-op, module byte-invisible. Under cmp405_eindex the
//! iter-1 behavior is untouched (STRICT mismatch → this lane asleep).

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// STRICT-eq lever id (free at iter-2 open: `rg -c "cmp529_esel" src/` == 0;
/// sibling `cmp529_espt` is the es_pt contract leg — different lever).
pub const R_LEVER_FLAG: &str = "cmp529_esel";

/// Host class of the ESEL-C3 fast path (defined by the cmp405_eindex
/// manager; presence = "define-bridge alive" for the ARM guard).
const OPS_CLASS: &str = "net/minecraft/world/entity/EntityIndexOps";

/// iter-3 java site probes (site-spec: work/AG-245/result.md).
const PUBLISHER_METHOD: &str = "eselPublish";
/// (int[] slotType, int[] counts, long[] singles) -> int (1 = published).
const PUBLISHER_SIG: &str = "([I[I[J)I";
const ARM_METHOD: &str = "eselArmNow";
const ARM_SIG: &str = "()V";

/// Slot cap guard against absurd registries (bind-time, fail-closed).
pub const MAX_SLOTS: usize = 4096;

// ---------------------------------------------------------------------
// Per-type chain row + view model (rust mirror of TypeIndexView)
// ---------------------------------------------------------------------

/// One per-type chain row (a slot of the rust per-type registry).
/// `count` is the EXACT FULL-slice population of the type (bind-time
/// contract); `single` is the live entity descriptor (global entity id)
/// under the count==1 contract, else 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerTypeChain {
    pub type_key: u64,
    pub count: u32,
    pub single: u64,
}

/// Rust TypeIndexView model — a FROZEN snapshot per bind epoch (no
/// per-query JNI, law 6: the epoch event refreshes the whole view in one
/// bulk transition; queries read the published snapshot).
#[derive(Debug)]
pub struct EselView {
    pub epoch: u64,
    rows: Vec<PerTypeChain>,
}

impl EselView {
    /// EXACT count of live entities of this type in FULL slices
    /// (unknown type ⇒ 0 ⇒ java eselFast takes the unchanged walk).
    pub fn type_count(&self, type_key: u64) -> i32 {
        self.rows
            .iter()
            .find(|r| r.type_key == type_key)
            .map(|r| r.count as i32)
            .unwrap_or(0)
    }

    /// The single entity under the count==1 contract, or None (defect /
    /// count != 1). Two calls for the same key return the SAME descriptor.
    pub fn type_single(&self, type_key: u64) -> Option<u64> {
        self.rows
            .iter()
            .find(|r| r.type_key == type_key)
            .filter(|r| r.count == 1 && r.single != 0)
            .map(|r| r.single)
    }

    pub fn slot_count(&self) -> usize {
        self.rows.len()
    }

    /// Row accessor by index (stored row order — publish path uses it).
    pub fn row(&self, i: usize) -> (u64, i32, u64) {
        let r = &self.rows[i];
        (r.type_key, r.count as i32, r.single)
    }
}

/// Bind-time invariant validation (G6 fail-dominant):
///  - non-empty registry, bounded slots;
///  - unique type keys (a duplicate would silently alias two chains);
///  - count==1 ⇔ single!=0 (count==1 without a single = the java view
///    defect class; single set without count==1 = malformed row).
pub fn validate(chains: &[PerTypeChain]) -> Result<(), &'static str> {
    if chains.is_empty() {
        return Err("empty registry");
    }
    if chains.len() > MAX_SLOTS {
        return Err("too many slots");
    }
    for (i, c) in chains.iter().enumerate() {
        if c.count == 1 && c.single == 0 {
            return Err("count==1 with null single");
        }
        if c.count != 1 && c.single != 0 {
            return Err("single set without count==1");
        }
        if chains[i + 1..].iter().any(|o| o.type_key == c.type_key) {
            return Err("duplicate type key");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------
// Bind state machine
// ---------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum BindOutcome {
    /// First successful bind for this epoch (rust-side view published).
    Bound,
    /// Double-bind with the same epoch — no-op, view untouched (idempotent).
    Idempotent,
    /// Epoch older than the bound one — refused (epochs are monotonic).
    StaleEpoch,
    /// Invariant defect → bind refused AND sticky broken (G6).
    Refused(&'static str),
    /// Already sticky-broken — every future bind refused.
    Broken,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArmDecision {
    /// ARM latch flipped (runtime: java publish + ESEL_ARMED flip done too).
    Armed,
    /// Refused with the fail-closed reason (ESEL_ARMED stays false).
    Refused(&'static str),
    /// Sticky-broken state.
    Broken,
}

struct BindInner {
    view: Option<Arc<EselView>>,
    epoch: u64,
}

/// Bind + ARM protocol state (instantiable: selftests run on local
/// instances; the runtime ladder uses the `state()` singleton).
pub struct EselBindState {
    inner: Mutex<BindInner>,
    broken: AtomicBool,
    armed: AtomicBool,
    publisher_missing: AtomicBool,
    // G2 ARM-СТРАЖ analog: monotonic rust-side counters (never reset).
    binds: AtomicI64,
    refusals: AtomicI64,
    defects: AtomicI64,
    unbinds: AtomicI64,
    arms: AtomicI64,
}

impl EselBindState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(BindInner { view: None, epoch: 0 }),
            broken: AtomicBool::new(false),
            armed: AtomicBool::new(false),
            publisher_missing: AtomicBool::new(false),
            binds: AtomicI64::new(0),
            refusals: AtomicI64::new(0),
            defects: AtomicI64::new(0),
            unbinds: AtomicI64::new(0),
            arms: AtomicI64::new(0),
        }
    }

    /// Bulk-bind: validate + publish the rust-side view snapshot.
    /// Idempotent per epoch; stale epochs refused; defects sticky-broken.
    pub fn bind(&self, chains: &[PerTypeChain], epoch: u64) -> BindOutcome {
        if self.broken.load(Ordering::Acquire) {
            return BindOutcome::Broken;
        }
        if let Err(why) = validate(chains) {
            self.defects.fetch_add(1, Ordering::AcqRel);
            self.broken.store(true, Ordering::Release);
            return BindOutcome::Refused(why);
        }
        let mut inner = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => {
                self.refusals.fetch_add(1, Ordering::AcqRel);
                return BindOutcome::Refused("poisoned bind lock");
            }
        };
        if inner.epoch == epoch && inner.view.is_some() {
            return BindOutcome::Idempotent; // double-bind: no-op, no replace
        }
        if epoch < inner.epoch {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return BindOutcome::StaleEpoch;
        }
        *inner = BindInner {
            view: Some(Arc::new(EselView { epoch, rows: chains.to_vec() })),
            epoch,
        };
        self.binds.fetch_add(1, Ordering::AcqRel);
        BindOutcome::Bound
    }

    /// unbind → published view becomes None (java-safe: ESEL_VIEW==null
    /// with ESEL_ARMED==true takes the walk — eselFast checks the null
    /// BEFORE touching the view). Idempotent; stale unbind refused.
    pub fn unbind(&self, epoch: u64) -> bool {
        let mut inner = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        if inner.view.is_none() {
            return false; // idempotent
        }
        if epoch < inner.epoch {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return false;
        }
        inner.view = None;
        inner.epoch = epoch;
        self.armed.store(false, Ordering::Release);
        self.unbinds.fetch_add(1, Ordering::AcqRel);
        true
    }

    /// Published view snapshot (None after unbind — "unbind→null").
    pub fn published(&self) -> Option<Arc<EselView>> {
        self.inner.lock().ok().and_then(|i| i.view.clone())
    }

    pub fn armed(&self) -> bool {
        self.armed.load(Ordering::Acquire)
    }

    pub fn broken(&self) -> bool {
        self.broken.load(Ordering::Acquire)
    }

    pub fn publisher_missing(&self) -> bool {
        self.publisher_missing.load(Ordering::Acquire)
    }

    /// ARM protocol (fail-closed): armed ONLY with a live define-bridge,
    /// a passed selftest, a bound view and no publisher-missing latch.
    /// `bridge_alive` is injected so the protocol is testable JVM-free;
    /// the runtime ladder passes the EntityIndexOps presence probe.
    pub fn arm(&self, bridge_alive: bool, selftest_ok: bool) -> ArmDecision {
        if self.broken.load(Ordering::Acquire) {
            return ArmDecision::Broken;
        }
        if self.publisher_missing.load(Ordering::Acquire) {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return ArmDecision::Refused("java publisher absent (iter-3 site)");
        }
        if !bridge_alive {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return ArmDecision::Refused("define-bridge not live");
        }
        if !selftest_ok {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return ArmDecision::Refused("selftest failed");
        }
        if self.published().is_none() {
            self.refusals.fetch_add(1, Ordering::AcqRel);
            return ArmDecision::Refused("no bound view");
        }
        if !self.armed.swap(true, Ordering::AcqRel) {
            self.arms.fetch_add(1, Ordering::AcqRel);
        }
        ArmDecision::Armed
    }

    /// G2 counters snapshot (monotonic; for the loud arm marker).
    /// [binds, refusals, defects, unbinds, arms]
    pub fn counters(&self) -> [i64; 5] {
        [
            self.binds.load(Ordering::Acquire),
            self.refusals.load(Ordering::Acquire),
            self.defects.load(Ordering::Acquire),
            self.unbinds.load(Ordering::Acquire),
            self.arms.load(Ordering::Acquire),
        ]
    }
}

impl Default for EselBindState {
    fn default() -> Self {
        Self::new()
    }
}

static INSTANCE: OnceLock<EselBindState> = OnceLock::new();

/// Runtime singleton (ladder + iter-3 epoch wiring target).
pub fn state() -> &'static EselBindState {
    INSTANCE.get_or_init(EselBindState::new)
}

/// STRICT-eq lever check (pure, testable): trim, then exact equality.
pub fn lever_matches(flag: &str) -> bool {
    flag.trim() == R_LEVER_FLAG
}

fn lever_flag() -> String {
    std::env::var("CRUSSTY_LEVER_FLAG").unwrap_or_default()
}

/// Epoch-event hook (iter-3 wiring target): bulk-bind the per-type chains
/// on the entity-composition epoch event. iter-2: exported + selftested,
/// no runtime caller (chains go live with the es_pt wiring tick).
pub fn on_epoch_event(epoch: u64, chains: &[PerTypeChain]) -> BindOutcome {
    if !lever_matches(&lever_flag()) {
        return BindOutcome::Refused("lever dormant");
    }
    state().bind(chains, epoch)
}

// ---------------------------------------------------------------------
// Java publisher probe / push (read-only until the iter-3 site lands)
// ---------------------------------------------------------------------

/// Outcome of the read-only java site probe.
enum ProbeOutcome {
    /// Either method id missing — sticky publisher-missing, zero mutation.
    PublisherAbsent,
    /// Both method ids resolve (future iter-3 blob) — push path ready.
    SitePresent,
}

/// READ-ONLY probe: find the live EntityIndexOps bridge, resolve the two
/// iter-3 method ids. GetStaticMethodID failures leave a pending
/// NoSuchMethodError — cleared immediately (entity_index_manager canon).
fn probe_java_publisher(env: &jvmti_bindings::env::JniEnv) -> ProbeOutcome {
    let Some(cls) = cplug_sdk::classes::find_class(OPS_CLASS) else {
        return ProbeOutcome::PublisherAbsent;
    };
    let has = |name: &str, sig: &str| -> bool {
        match env.get_static_method_id(cls.as_jclass(), name, sig) {
            Some(_) => true,
            None => {
                env.exception_clear(); // NoSuchMethodError pending — clear
                false
            }
        }
    };
    let pub_ok = has(PUBLISHER_METHOD, PUBLISHER_SIG);
    let arm_ok = has(ARM_METHOD, ARM_SIG);
    if pub_ok && arm_ok {
        ProbeOutcome::SitePresent
    } else {
        ProbeOutcome::PublisherAbsent
    }
}

/// Full push path (reachable ONLY when the iter-3 java site exists AND the
/// per-type chains are live): publish the bound view rows into java
/// (arrays → eselPublish → java constructs TypeIndexView over them →
/// ESEL_VIEW=…), THEN flip ESEL_ARMED via eselArmNow (ARM-after-publish —
/// publishing without arming is fail-closed-safe: java checks !ESEL_ARMED
/// first; arming without publishing would burn the plane).
fn publish_and_arm_java(state: &EselBindState) -> bool {
    let Some(view) = state.published() else {
        return false;
    };
    let n = view.slot_count();
    let mut slot_type: Vec<i32> = Vec::with_capacity(n);
    let mut counts: Vec<i32> = Vec::with_capacity(n);
    let mut singles: Vec<i64> = Vec::with_capacity(n);
    for i in 0..n {
        let (k, c, s) = view.row(i);
        slot_type.push(k as i32);
        counts.push(c);
        singles.push(s as i64);
    }
    cplug_sdk::jni_util::with_attached(|env| {
        let Some(cls) = cplug_sdk::classes::find_class(OPS_CLASS) else {
            return false;
        };
        let Some(slot_arr) = env.new_int_array(n as i32) else {
            env.exception_clear();
            return false;
        };
        let Some(cnt_arr) = env.new_int_array(n as i32) else {
            env.exception_clear();
            return false;
        };
        let Some(sgl_arr) = env.new_long_array(n as i32) else {
            env.exception_clear();
            return false;
        };
        env.set_int_array_region(slot_arr, 0, n as i32, &slot_type);
        env.set_int_array_region(cnt_arr, 0, n as i32, &counts);
        env.set_long_array_region(sgl_arr, 0, n as i32, &singles);
        let Some(pub_mid) =
            env.get_static_method_id(cls.as_jclass(), PUBLISHER_METHOD, PUBLISHER_SIG)
        else {
            env.exception_clear();
            return false;
        };
        let args = [
            jvmti_bindings::jni::jvalue { l: slot_arr },
            jvmti_bindings::jni::jvalue { l: cnt_arr },
            jvmti_bindings::jni::jvalue { l: sgl_arr },
        ];
        let rc = env.call_static_int_method(cls.as_jclass(), pub_mid, &args);
        if rc != 1 {
            env.exception_clear();
            eprintln!(
                "[crussty-plugin] esel_bind: eselPublish rc={rc} — ESEL_ARMED NOT flipped (fail-closed)"
            );
            return false;
        }
        // ARM strictly AFTER publish (java contract; ARM-AFTER-DEFINE canon).
        let Some(arm_mid) =
            env.get_static_method_id(cls.as_jclass(), ARM_METHOD, ARM_SIG)
        else {
            env.exception_clear();
            return false;
        };
        env.call_static_void_method(cls.as_jclass(), arm_mid, &[]);
        true
    })
    .unwrap_or(false)
}

// ---------------------------------------------------------------------
// Runtime ladder (register / activate)
// ---------------------------------------------------------------------

/// Register (call once from cplugin_init). Dormant-invisible unless the
/// lever flag STRICT-equals cmp529_esel.
pub fn register() {
    if !lever_matches(&lever_flag()) {
        eprintln!(
            "[crussty-plugin] esel_bind: dormant (set CRUSSTY_LEVER_FLAG={R_LEVER_FLAG} to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] esel_bind: lever {R_LEVER_FLAG} seen — ladder queued (ARM strictly after bind+publish+selftest)"
    );
}

/// Activation ladder (background; boot-storm discipline TASK-80):
///  1. EntityIndexOps bridge live? (defined by the cmp405_eindex manager)
///     — absent → honest-stop (nothing in the JVM to bind into);
///  2. boot quiet;
///  3. rust protocol selftest — RED → honest-stop (fail-closed);
///  4. READ-ONLY probe of the iter-3 java site (eselPublish + eselArmNow)
///     — absent (iter-1 blob) → sticky publisher-missing + site-spec
///     notice, ESEL_ARMED untouched (this IS the expected iter-2 path);
///  5. site present (future) → per-type chains must be live too; without a
///     chain source (es_pt wiring iter-3) honest-stop — never publish an
///     empty/derivative view.
pub fn activate() {
    if !lever_matches(&lever_flag()) {
        return;
    }
    std::thread::spawn(move || {
        // 1. Define-bridge alive probe (poll; EntityLookup/EntityIndexOps
        //    load with the first level under the cmp405_eindex lever).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(OPS_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] esel_bind: {OPS_CLASS} not live within 180s (cmp405_eindex manager did not define it) — bind ladder honest-stop, ESEL stays dormant"
                );
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(2_000));
        }

        // 2. Kernel loader must be quiet before any java touch.
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] esel_bind: boot marker not seen — ladder honest-stop");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // 3. Rust protocol selftest (fail-closed gate for ARM).
        if !esel_bind_selftest() {
            eprintln!(
                "[crussty-plugin] esel_bind: selftest RED — arm refused (fail-closed), ESEL_ARMED stays false"
            );
            return;
        }
        eprintln!(
            "[crussty-plugin] esel_bind: selftest GREEN (bind/idempotent/unbind/defect-sticky/arm-protocol)"
        );

        // 4. Read-only java site probe.
        let probed = cplug_sdk::jni_util::with_attached(probe_java_publisher);
        match probed {
            Some(ProbeOutcome::PublisherAbsent) | None => {
                state().publisher_missing.store(true, Ordering::Release);
                eprintln!(
                    "[crussty-plugin] esel_bind: java publisher ABSENT in EntityIndexOps iter-1 blob (site-spec: work/AG-245/result.md) — sticky publisher-missing, ZERO java mutation, ESEL_ARMED stays false (dormant iter-3)"
                );
                return;
            }
            Some(ProbeOutcome::SitePresent) => {}
        }

        // 5. Java site exists (future iter-3 blob): chains must be live.
        //    publish_and_arm_java(state()) is the wiring-tick entry (bind
        //    from live chains first, then push). Until chains go live the
        //    singleton has NO bound view → the push path is a no-op by
        //    construction (published()==None → return false, zero java
        //    touch); we do NOT call it here — honest-stop instead.
        eprintln!(
            "[crussty-plugin] esel_bind: java site present but per-type chains NOT live (es_pt wiring = iter-3) — publish deferred, ESEL_ARMED stays false (fail-closed: no chain source, no view data)"
        );
    });
}

// ---------------------------------------------------------------------
// Selftest (protocol-level, JVM-free; same suite as cargo test esel)
// ---------------------------------------------------------------------

/// Runtime selftest for the ARM ladder: runs the full scenario suite on
/// LOCAL bind states (the global singleton is untouched). true = all
/// invariants hold.
pub fn esel_bind_selftest() -> bool {
    let mut ok = true;

    // Synthetic per-type registry: 3 types, one with count==1.
    let reg = [
        PerTypeChain { type_key: 11, count: 0, single: 0 },
        PerTypeChain { type_key: 22, count: 5, single: 0 },
        PerTypeChain { type_key: 33, count: 1, single: 9001 },
    ];

    // bind → invariants.
    let s1 = EselBindState::new();
    ok &= s1.bind(&reg, 7) == BindOutcome::Bound;
    let Some(v) = s1.published() else { return false };
    ok &= v.type_count(11) == 0;
    ok &= v.type_count(22) == 5;
    ok &= v.type_count(33) == 1;
    ok &= v.type_count(999) == 0; // unknown type ⇒ walk-safe 0
    ok &= v.type_single(33) == Some(9001);
    ok &= v.type_single(33) == v.type_single(33); // same descriptor twice
    ok &= v.type_single(22).is_none();
    ok &= v.type_single(11).is_none();
    ok &= s1.arm(true, true) == ArmDecision::Armed;
    ok &= s1.armed();
    ok &= s1.counters()[4] == 1; // exactly one arm flip

    // Double-bind idempotent.
    let s2 = EselBindState::new();
    s2.bind(&reg, 7);
    let v2a = s2.published();
    ok &= s2.bind(&reg, 7) == BindOutcome::Idempotent;
    let v2b = s2.published();
    ok &= match (v2a, v2b) {
        (Some(a), Some(b)) => Arc::ptr_eq(&a, &b),
        _ => false,
    };

    // Stale epoch refused.
    let s3 = EselBindState::new();
    s3.bind(&reg, 7);
    ok &= s3.bind(&reg, 6) == BindOutcome::StaleEpoch;

    // unbind → null; idempotent unbind; rebind after unbind.
    let s4 = EselBindState::new();
    s4.bind(&reg, 7);
    ok &= s4.unbind(8);
    ok &= s4.published().is_none();
    ok &= !s4.unbind(8); // idempotent
    ok &= s4.bind(&reg, 9) == BindOutcome::Bound;
    ok &= s4.published().is_some();

    // Defect rows → refused AND sticky broken; arm on broken → Broken.
    let defect = [PerTypeChain { type_key: 1, count: 1, single: 0 }];
    let s5 = EselBindState::new();
    ok &= s5.bind(&defect, 7) == BindOutcome::Refused("count==1 with null single");
    ok &= s5.broken();
    ok &= s5.bind(&reg, 8) == BindOutcome::Broken;
    ok &= s5.arm(true, true) == ArmDecision::Broken;

    // Duplicate keys / single-without-count refused.
    let dup = [
        PerTypeChain { type_key: 5, count: 1, single: 1 },
        PerTypeChain { type_key: 5, count: 2, single: 0 },
    ];
    let s6 = EselBindState::new();
    ok &= s6.bind(&dup, 7) == BindOutcome::Refused("duplicate type key");
    let rogue = [PerTypeChain { type_key: 6, count: 4, single: 77 }];
    let s7 = EselBindState::new();
    ok &= s7.bind(&rogue, 7) == BindOutcome::Refused("single set without count==1");

    // ARM protocol fail-closed ladder.
    let s8 = EselBindState::new();
    ok &= s8.arm(true, true) == ArmDecision::Refused("no bound view");
    s8.bind(&reg, 7);
    ok &= s8.arm(false, true) == ArmDecision::Refused("define-bridge not live");
    ok &= s8.arm(true, false) == ArmDecision::Refused("selftest failed");
    ok &= !s8.armed();
    ok &= s8.arm(true, true) == ArmDecision::Armed;

    // STRICT-eq lever.
    ok &= !lever_matches("");
    ok &= !lever_matches("cmp529_espt");
    ok &= !lever_matches("cmp529_eselx");
    ok &= lever_matches("cmp529_esel");
    ok &= lever_matches("  cmp529_esel  ");

    ok
}

#[cfg(test)]
mod esel_bind_tests {
    use super::*;

    fn reg3() -> [PerTypeChain; 3] {
        [
            PerTypeChain { type_key: 11, count: 0, single: 0 },
            PerTypeChain { type_key: 22, count: 5, single: 0 },
            PerTypeChain { type_key: 33, count: 1, single: 9001 },
        ]
    }

    #[test]
    fn bind_singleton_registry_invariants() {
        let s = EselBindState::new();
        assert_eq!(s.bind(&reg3(), 7), BindOutcome::Bound);
        let v = s.published().expect("view published");
        assert_eq!(v.type_count(11), 0);
        assert_eq!(v.type_count(22), 5);
        assert_eq!(v.type_count(33), 1);
        assert_eq!(v.type_count(999), 0);
        assert_eq!(v.type_single(33), Some(9001));
        assert_eq!(v.type_single(33), v.type_single(33));
        assert_eq!(v.type_single(22), None);
        assert_eq!(v.type_single(11), None);
        assert_eq!(v.slot_count(), 3);
        assert_eq!(s.arm(true, true), ArmDecision::Armed);
        assert!(s.armed());
    }

    #[test]
    fn double_bind_idempotent_same_view() {
        let s = EselBindState::new();
        assert_eq!(s.bind(&reg3(), 7), BindOutcome::Bound);
        let a = s.published();
        assert_eq!(s.bind(&reg3(), 7), BindOutcome::Idempotent);
        let b = s.published();
        assert!(matches!((a, b), (Some(x), Some(y)) if Arc::ptr_eq(&x, &y)));
        assert_eq!(s.counters()[0], 1); // one bind, one publication
    }

    #[test]
    fn stale_epoch_refused() {
        let s = EselBindState::new();
        s.bind(&reg3(), 7);
        assert_eq!(s.bind(&reg3(), 6), BindOutcome::StaleEpoch);
        assert!(!s.broken()); // refusal, not defect
    }

    #[test]
    fn unbind_nulls_view_idempotent_rebind() {
        let s = EselBindState::new();
        s.bind(&reg3(), 7);
        assert!(s.unbind(8));
        assert!(s.published().is_none()); // unbind → null
        assert!(!s.unbind(8)); // idempotent
        assert_eq!(s.bind(&reg3(), 9), BindOutcome::Bound);
        assert!(s.published().is_some());
    }

    #[test]
    fn defect_rows_fail_closed_sticky() {
        let defect = [PerTypeChain { type_key: 1, count: 1, single: 0 }];
        let s = EselBindState::new();
        assert_eq!(
            s.bind(&defect, 7),
            BindOutcome::Refused("count==1 with null single")
        );
        assert!(s.broken());
        assert_eq!(s.bind(&reg3(), 8), BindOutcome::Broken); // sticky
        assert_eq!(s.arm(true, true), ArmDecision::Broken);
    }

    #[test]
    fn duplicate_keys_and_rogue_single_refused() {
        let dup = [
            PerTypeChain { type_key: 5, count: 1, single: 1 },
            PerTypeChain { type_key: 5, count: 2, single: 0 },
        ];
        let s = EselBindState::new();
        assert_eq!(s.bind(&dup, 7), BindOutcome::Refused("duplicate type key"));
        let rogue = [PerTypeChain { type_key: 6, count: 4, single: 77 }];
        let s2 = EselBindState::new();
        assert_eq!(
            s2.bind(&rogue, 7),
            BindOutcome::Refused("single set without count==1")
        );
    }

    #[test]
    fn arm_protocol_fail_closed_ladder() {
        let s = EselBindState::new();
        // no view → refuse (arming a null view would burn the java plane)
        assert_eq!(s.arm(true, true), ArmDecision::Refused("no bound view"));
        s.bind(&reg3(), 7);
        // bridge not live → refuse
        assert_eq!(
            s.arm(false, true),
            ArmDecision::Refused("define-bridge not live")
        );
        // selftest failed → refuse
        assert_eq!(s.arm(true, false), ArmDecision::Refused("selftest failed"));
        assert!(!s.armed());
        assert_eq!(s.arm(true, true), ArmDecision::Armed);
        assert!(s.armed());
        // publisher-missing sticky overrides everything
        let s2 = EselBindState::new();
        s2.publisher_missing.store(true, Ordering::Release);
        s2.bind(&reg3(), 7);
        assert_eq!(
            s2.arm(true, true),
            ArmDecision::Refused("java publisher absent (iter-3 site)")
        );
        assert!(!s2.armed());
    }

    #[test]
    fn lever_strict_eq() {
        assert!(!lever_matches(""));
        assert!(!lever_matches("cmp529_espt"));
        assert!(!lever_matches("cmp529_eselx"));
        assert!(lever_matches("cmp529_esel"));
        assert!(lever_matches("  cmp529_esel  "));
    }

    #[test]
    fn runtime_selftest_suite_green() {
        assert!(esel_bind_selftest());
    }

    #[test]
    fn validate_bounds_hold() {
        let long: Vec<PerTypeChain> = (0..=MAX_SLOTS)
            .map(|i| PerTypeChain { type_key: i as u64, count: 0, single: 0 })
            .collect();
        assert_eq!(validate(&long), Err("too many slots"));
        assert_eq!(validate(&[]), Err("empty registry"));
    }
}
