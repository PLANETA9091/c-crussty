//! ES-PT (TASK-529 per-type index, lever `cmp529_espt`) — CONTRACT-DORMANT leg.
//!
//! Javap-контракт AG-110 iter-2 (canon jar e2992d63abd2c254, 29,386,794B):
//! единственные getEntities-сайты `EntitySelector.addEntities` =
//! invokevirtual #297@32 (box-ветка) и #300@48 (no-box-ветка). Wiring-тик
//! ретаргетит ТОЛЬКО эти 2 сайта на `EntitySelectorOps.esGetEntitiesBox` /
//! `EntitySelectorOps.esGetEntities` (стек-форма + prepended EntitySelector
//! receiver), дельта jar = +1 ops-class, 0 иных классов (G1 BYTE-CONTRACT).
//!
//! THIS TICK = contract заготовка + iter-3 chain-source leg (AG-249 w530):
//! the module is NOW wired (`mod es_pt;` in lib.rs) and owns the chain-source
//! registry that feeds esel_bind's bind path (install_chain_source /
//! chains_snapshot with the AG-19 capture-arm gate). The EntitySelectorOps
//! define/retarget itself stays FUTURE (DORMANT by construction): java-блоб
//! скомпилирован javac-21 против канон-jar и кладётся в
//! `entityselector/build/...` под будущий include_bytes!; ЗДЕСЬ ZERO
//! define/retarget/retransform, ZERO `#[no_mangle]`, ZERO JNI — экспортная
//! поверхность и ванильный путь бит-в-бит не тронуты.
//!
//! FAST-PATH ПРЕДИКАТ (AG-110 spec): `type != ANY_TYPE && limit == 1`.
//! ANY_TYPE — приватный статик EntitySelector: java setAccessible запрещён,
//! JNI GetStaticObjectField снимает reference ОДИН раз на инсталле
//! (`install(anyTypeRef, belt)`), хот-путь = reference-compare, 0 аллокаций.
//!
//! ПОРЯДОК-СТЕНА (Л58/Л146): rust строит per-type цепи в E-порядке dense-id
//! (subsequence-инвариант к ванильному walk); первый прошедший pred кандидат
//! == первый ванильный match. Предикат/живость — ПОСЛЕ индексного фетча на
//! живом Entity (superset-oracle G3, parity lockstep ≥20k ops — wiring-гейт).
//!
//! NO-CACHE (RECON-39/40): цепи = refresh-плоскость за тик (1 bulk JNI/тик,
//! law 6), кэша результатов нет; счётчики монотонные fail-dominant (G6), не
//! сбрасываются; любой отказ → vanilla delegate + FB_* причина (fail-closed).
//!
//! ARM-СТРАЖ (G2): `es_pt: ARMED cmp529_espt` в stdout при install(); без
//! маркера нога невалидна (navmath-1 урок).
//!
//! CAPTURE-МАТЕМАТИКА (AG-19/AG-110, prereg clm/AG-169): dp50k
//! 11.6-16.9% CPU × капчур 70-95% = +8.1-16.1пп CPU → TPS@dp50k 3.4 →
//! 3.7-4.0; pop150k 30.5-57.6пп (компо-резерв). Пара-бар +20 min-of-3.

#![allow(dead_code)]

use std::sync::atomic::AtomicI64;
use std::sync::{Arc, RwLock};

/// STRICT-eq lever id (swarx-4 урок: один id, никаких союзов по env).
pub const R_LEVER_FLAG: &str = "cmp529_espt";
/// Subsystem arm env (wiring-тик; java-сторона: CRUSSTY_ES_PT echo).
pub const R_ARM_ENV: &str = "CRUSSTY_ES_PT";

/// Слот-кап per-type голов (java SLOT_CAP — MUST match EntitySelectorOps).
pub const SLOT_CAP: usize = 1024;

/// G6-семейство счётчиков (монотонные, fail-dominant; java AtomicLong —
/// rust-инкременты через bulk-отчёт раз в тик, не per-query JNI).
pub struct EsPtCounters {
    pub index_hit: AtomicI64,
    pub fb_disarm: AtomicI64,
    pub fb_snapshot: AtomicI64,
    pub fb_slot: AtomicI64,
    pub fb_chain_empty: AtomicI64,
    pub fb_struct: AtomicI64,
    pub fb_err: AtomicI64,
    pub vanilla_delegate: AtomicI64,
}

/// Ретаргет-спек wiring-тика (пин байт-кода, G1): класс-владелец сайта,
/// оффсет invoke, целевой статик ops-класса. #297/#300 — из javap-контракта.
pub struct RetargetSpec {
    pub owner: &'static str,
    pub method: &'static str,
    pub site_offset: u16,
    pub target: &'static str,
}

pub const RETARGETS: [RetargetSpec; 2] = [
    RetargetSpec {
        owner: "net/minecraft/commands/arguments/selector/EntitySelector",
        method: "addEntities",
        site_offset: 32, // invokevirtual #297 (box-ветка)
        target: "net/minecraft/commands/arguments/selector/EntitySelectorOps.esGetEntitiesBox",
    },
    RetargetSpec {
        owner: "net/minecraft/commands/arguments/selector/EntitySelector",
        method: "addEntities",
        site_offset: 48, // invokevirtual #300 (no-box-ветка)
        target: "net/minecraft/commands/arguments/selector/EntitySelectorOps.esGetEntities",
    },
];

/// Границы (Л58/Л146 + G5/G6): бокс-ветка НЕ обслуживается (E-порядок цепей
/// не обязан совпадать с ванильным box-walk порядком) — честный vanilla
/// delegate; NO-CACHE-инвариант RECON-39/40 не пересекается.
pub const BOX_BRANCH_SERVED: bool = false;
pub const PARITY_LOCKSTEP_MIN_OPS: u64 = 20_000;
/// Порог пары (AG-44 aa480s1): +20пп worst-of-3 при sigma<=7пп.
pub const PAIR_BAR_PP: i64 = 20;

/// Капчур-пин prereg (AG-19): arm при капчуре >=70%; ниже — wiring-тик
/// обязан рефузить инсталл (fail-dominant, экономия arm-бюджета).
pub const CAPTURE_ARM_MIN_PPCT: i64 = 70;
pub const DP50K_CPU_PLAN_PCT: (i64, i64) = (116, 169); // 11.6-16.9% x10

// ---------------------------------------------------------------------
// iter-3 wiring (AG-249 w530): chain-source registry — the bind-path feed
// for esel_bind (per-type chains → slotType/counts/singles before publish).
// DORMANT by construction: until a source is installed (by the future
// noteAdd/Remove/Move data plane, AG-110 contract), chains_snapshot() is
// None → the esel_bind ladder honest-stops and ESEL_ARMED stays false.
// ---------------------------------------------------------------------

/// Snapshot the installed chain source produces on demand. `chains` rows
/// mirror esel_bind::PerTypeChain (count==1 ⇔ single!=0 — validated on BOTH
/// sides before publish); `epoch` is monotonic per source.
#[derive(Debug, Clone)]
pub struct ChainSnapshot {
    pub epoch: u64,
    /// Capture of the mode-2 EntityType-getEntities population, percent
    /// ×1 (AG-19 prereg pin: below CAPTURE_ARM_MIN_PPCT the bind path
    /// refuses — fail-dominant, arm budget is not burned on a partial
    /// plane).
    pub capture_ppct: i64,
    pub chains: Vec<crate::esel_bind::PerTypeChain>,
}

pub type ChainSource = Arc<dyn Fn() -> Option<ChainSnapshot> + Send + Sync>;

static CHAIN_SOURCE: RwLock<Option<ChainSource>> = RwLock::new(None);

static SOURCE_REFUSALS: AtomicI64 = AtomicI64::new(0);

/// Install the chain source (idempotent: FIRST install wins — the source is
/// boot-critical, no hot swaps; later installs are counted and refused).
pub fn install_chain_source(src: ChainSource) -> bool {
    let mut slot = match CHAIN_SOURCE.write() {
        Ok(g) => g,
        Err(_) => {
            SOURCE_REFUSALS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            return false;
        }
    };
    if slot.is_some() {
        SOURCE_REFUSALS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        return false;
    }
    *slot = Some(src);
    true
}

/// Bind-path feed (esel_bind ladder step-5): None = chains not live / source
/// absent / source refused / capture below the AG-19 arm-min → the ladder
/// honest-stops (fail-closed, zero java touch).
pub fn chains_snapshot() -> Option<ChainSnapshot> {
    let src = CHAIN_SOURCE
        .read()
        .ok()
        .and_then(|g| g.clone())?;
    let snap = (src)()?;
    if !capture_arm_ok(snap.capture_ppct) {
        // AG-19 capture pin: a partial plane must not arm (G6-adjacent:
        // the refusal is counted, not sticky — the source may re-serve a
        // higher-capture snapshot on a later epoch).
        SOURCE_REFUSALS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        return None;
    }
    Some(snap)
}

/// Pure capture-arm predicate (AG-19 pin, tested): below CAPTURE_ARM_MIN_PPCT
/// the bind path must refuse.
pub fn capture_arm_ok(capture_ppct: i64) -> bool {
    capture_ppct >= CAPTURE_ARM_MIN_PPCT
}

/// Chains live right now (bind path may proceed). Diagnostics only — the
/// ladder consumes the snapshot itself.
pub fn chains_live() -> bool {
    chains_snapshot().is_some()
}

/// G2-adjacent monotonic counter: source refusals (re-install attempts +
/// below-capture snapshots). Never reset.
pub fn source_refusals() -> i64 {
    SOURCE_REFUSALS.load(std::sync::atomic::Ordering::Acquire)
}

#[cfg(test)]
mod esel_chain_source_tests {
    use super::*;
    use crate::esel_bind::PerTypeChain;

    fn snap(epoch: u64, capture: i64) -> ChainSnapshot {
        ChainSnapshot {
            epoch,
            capture_ppct: capture,
            chains: vec![PerTypeChain { type_key: 33, count: 1, single: 9001 }],
        }
    }

    #[test]
    fn esel_chain_source_first_install_wins() {
        assert!(!chains_live()); // dormant by construction
        let a = install_chain_source(Arc::new(|| Some(snap(7, 80))));
        assert!(a);
        let b = install_chain_source(Arc::new(|| Some(snap(8, 80))));
        assert!(!b); // no hot swaps
        assert!(source_refusals() >= 1);
        let s = chains_snapshot().expect("first source serves");
        assert_eq!(s.epoch, 7); // FIRST source, not the second
    }

    #[test]
    fn esel_chain_capture_arm_predicate_ag19() {
        // AG-19 prereg pin: arm at >=70, refuse below (fail-dominant).
        assert!(!capture_arm_ok(CAPTURE_ARM_MIN_PPCT - 1));
        assert!(!capture_arm_ok(0));
        assert!(!capture_arm_ok(i64::MIN));
        assert!(capture_arm_ok(CAPTURE_ARM_MIN_PPCT));
        assert!(capture_arm_ok(95)); // upper plan point of the 70-95 band
        assert!(capture_arm_ok(i64::MAX));
    }
}
