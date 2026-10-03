//! ES-PT (TASK-529 per-type index, lever `cmp529_espt`) — CONTRACT-DORMANT leg.
//!
//! Javap-контракт AG-110 iter-2 (canon jar e2992d63abd2c254, 29,386,794B):
//! единственные getEntities-сайты `EntitySelector.addEntities` =
//! invokevirtual #297@32 (box-ветка) и #300@48 (no-box-ветка). Wiring-тик
//! ретаргетит ТОЛЬКО эти 2 сайта на `EntitySelectorOps.esGetEntitiesBox` /
//! `EntitySelectorOps.esGetEntities` (стек-форма + prepended EntitySelector
//! receiver), дельта jar = +1 ops-class, 0 иных классов (G1 BYTE-CONTRACT).
//!
//! THIS TICK = contract заготовка, DORMANT by construction (канон sb_r1.rs /
//! chunk_sched.rs): java-блоб скомпилирован javac-21 против канон-jar и
//! кладётся в `entityselector/build/...` под будущий include_bytes!; ЗДЕСЬ
//! ZERO define/retarget/retransform, ZERO `#[no_mangle]`, ZERO JNI —
//! экспортная поверхность и ванильный путь бит-в-бит не тронуты. Мод в
//! lib.rs НЕ подключается этим тиком (wiring-тик делает `mod es_pt;` +
//! инсталл вместе с define/retarget — одиночная атомарная нога).
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
