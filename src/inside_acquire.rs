//! INSIDE-ACQUIRE (ID-P33, TASK-459-73 scaffold — закон 11 v18.2).
//!
//! ПОДСИСТЕМА: volatile-demotion остаточных volatile-чтений MISS-хвоста
//! inside-лейна. Карточка ID-P33 (round-458p-ideas): в retarget-теле
//! inside-сайта (Level.getBlockState → InsideSnapOps.snapGet уже live,
//! cmp424_inside) + в PalettedContainerOps-классе читать поля через
//! VarHandle.getAcquire — JMM-эквивалент volatile-read по наблюдаемости
//! (x86 = plain load, C2-перестановки контролируемы).
//!
//! Состав подсистемы (закон 6 — НЕ одиночная функция):
//!   1. JAVA-МОСТ InsideAcquireOps (entityinside/…/InsideAcquireOps.java):
//!      VarHandle-плоскость acquire-читателей — seqlock-пара gen/pending
//!      и publication-пара snap/snapGen (на собственных полях-эмулях;
//!      реальные сайты InsideSnapOps.Snap / PalettedContainer — wiring-фаза),
//!      selfTest single-writer наблюдаемости. Vanilla-free = голый javac.
//!   2. ARM-HOOK (этот модуль, activate): РАННЕЕ определение моста в kernel
//!      loader — NCDFE-канон (×93-indy, run 35902792520). getAcquire =
//!      signature-polymorphic invokevirtual (JEP 193), НЕ invokedynamic —
//!      <clinit>-findVarHandle не создаёт indy-констант, шторм невозможен
//!      структурно.
//!   3. WIRING-ФАЗА (после скаффолда, НЕ в этом коммите): перепись читателей
//!      InsideSnapOps.serve/serve4 (Snap.gen, Snap.pending) и
//!      PalettedContainerOps.get (PalettedContainer.data + гейт-поля
//!      crusstySnap/crusstySnapGen/crusstyGen) на getAcquire; рандом-поп
//!      оракул single-writer наблюдаемости до публикации lever.
//!
//! JMM-КОНТРАКТ v1 (карточка): getAcquire, НЕ opaque. Лестница режимов
//! кумулятивна (Lea j9mm: Plain < Opaque < Release/Acquire < Volatile) —
//! замена volatile-load на acquire-load сохраняет ВСЕ гарантии читателя;
//! writer-сторона (secWrite s.gen++ ПОСЛЕ write, PalettedContainerOps
//! publication snap→snapGen-last) НЕ трогается — StoreLoad-драин остаётся там,
//! где он и нужен (seqlock-смысл). Выигрыш x86 не в фенсах (volatile-load на
//! x86 = plain load, Brooker), а в снятии C2-запретов hoist/койлесинга
//! планировщика на горячем читателе; дольные long-тонкости — только
//! acquire-режим до рандом-поп оракула (карточка).
//!
//! Сайты-демоты (javap-цели wiring-фазы; подробнее RESEARCH-459-P33.md §3):
//!   SITE 1: InsideSnapOps.serve/serve4  <- Snap.gen, Snap.pending (getAcquire)
//!   SITE 2: PalettedContainerOps.get    <- PalettedContainer.data (getAcquire)
//!   SITE 3: PalettedContainerOps.get    <- crusstySnap/crusstySnapGen/crusstyGen
//!   SITE 4: стат-счётчики BUILDS/ABORTS/CAPPED — v1 НЕ трогается (не горячо).
//! Анти-плейсебо: писателей не трогаем; CHM-lookup не volatile — вне скоупа.
//!
//! ПРОГНОЗ (карточка): фенс-хвост MISS 2-4пп × 30% → +0.6-1.2пп. База:
//! inside-лейн ≈1.3-1.6% total CPU (PROFILE-B, inside_snap.rs:3-7).
//!
//! Gate: env `CRUSSTY_INSIDE_ACQUIRE` (1/true/on/yes -> on). OFF BY DEFAULT —
//! dormant-невидимость (дисциплина inside_cache/inside_epoch_gate): при
//! выключенном флаге register() пишет dormant-notice, activate() возвращается
//! немедленно, классы не определяются, байты чужих классов не читаются.
//!
//! СТАТУС: SCAFFOLD (law-11 финал = {run id, ветка+SHA, вердикт-число}).
//! `BLOB_EMBEDDED=false`: activate-лестница полная, но define-шаг обязан
//! остаться no-op до приземления блоба (include_bytes! несуществующего .class
//! = красный cargo build в CI). Lever dormant по умолчанию И по статусу
//! скаффолда — двойной fail-closed.

// Scaffold-модуль: publish-поверхность (enabled_pub/bridge_ready/ACQUIRE_CLASS)
// оживают в wiring-фазе — dead_code до неё ожидаем, не шумим в CI.
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

const GATE_CLASS: &str = "net/minecraft/world/entity/InsideAcquireOps";
const SNAP_OPS_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps";

/// Scaffold-маркер: блоб ещё не встроен (wiring-фаза ставит true вместе с
/// include_bytes! ../entityinside/build/net/minecraft/world/entity/InsideAcquireOps.class).
const BLOB_EMBEDDED: bool = false;

/// Режим-лестница JMM (JEP 193 / Lea j9mm), кумулятивная — кодируется как
/// монотонный ранг: контракт теста mode_ladder_monotonic (см. #[cfg(test)]).
pub const MODE_PLAIN: u8 = 0;
pub const MODE_OPAQUE: u8 = 1;
pub const MODE_ACQUIRE: u8 = 2;
pub const MODE_VOLATILE: u8 = 3;

/// Демот-таблица v1 (RESEARCH-459-P33.md §3). Каждая строка: сайт, поле,
/// режим v1, что НЕ трогается (writer/анти-плейсебо).
pub const DEMOTION_SITES: &[(&str, &str, u8, &str)] = &[
    (
        "InsideSnapOps.serve/serve4",
        "Snap.gen / Snap.pending",
        MODE_ACQUIRE,
        "writer secWrite s.gen++ volatile — не трогается",
    ),
    (
        "PalettedContainerOps.get",
        "PalettedContainer.data",
        MODE_ACQUIRE,
        "vanilla torn-read окно сохранено бит-в-бит",
    ),
    (
        "PalettedContainerOps.get (gate)",
        "crusstySnap/crusstySnapGen/crusstyGen",
        MODE_ACQUIRE,
        "publication snap->snapGen last — writer volatile",
    ),
    (
        "PalettedContainerOps statics",
        "BUILDS/ABORTS/CAPPED/NEXT_LOG_AT",
        MODE_VOLATILE,
        "v1 НЕ трогается (не горячо; дольные long — до оракула)",
    ),
];

fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_ACQUIRE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate-видимость для wiring-фазы (serve/serve4-перепись читателей, classfile
/// терпólacja гейта PalettedContainerOps).
pub fn enabled_pub() -> bool {
    enabled()
}

/// Публикуется ТОЛЬКО после define+selfTest рандом-поп оракула (fail-dominant;
/// поллинг wiring-фазы — паттерн inside_cache::wait_bridge_ready).
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

pub fn bridge_ready() -> bool {
    BRIDGE_READY.load(Ordering::Acquire)
}

/// Контракт v1: все демоты — acquire (ниже volatile нельзя опускаться до
/// оракула; выше — значит НЕ демотировали). Возвращает false на любом сайте,
/// чей режим выпал из acquire-only канона карточки.
pub fn v1_acquire_only_contracts() -> bool {
    DEMOTION_SITES
        .iter()
        .all(|(_, _, mode, _)| *mode == MODE_ACQUIRE || *mode == MODE_VOLATILE)
}

/// Register (idempotent; call once from cplugin_init). Byte hook НЕ ставится
/// ни здесь, ни в activate: demotion — перепись ЧТЕНИЙ внутри уже
/// retarget-тел (inside_snap serve-тела + PalettedContainerOps.get), не новый
/// retarget — приземляется в wiring-фазе.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_acquire: dormant (set CRUSSTY_INSIDE_ACQUIRE=1 to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_acquire: scaffold armed-flat (P33 getAcquire demotion; reader-rewrite lands in wiring phase)"
    );
}

/// РАННИЙ ARM-ХУК (NCDFE-канон): ждёт kernel-класс InsideSnapOps (грузится в
/// inside_snap-секвенции), boot-маркер, затем define InsideAcquireOps в kernel
/// loader и selfTest ДО публикации BRIDGE_READY. Секвенция повторяет
/// inside_epoch_gate/inside_cache (boot-storm дисциплина TASK-80).
///
/// SCAFFOLD: до приземления блоба (BLOB_EMBEDDED=false) лестница доходит
/// только до loud scaffold-notice — НИКАКИХ define/чтения чужих байтов (lever
/// dormant и по флагу, и по статусу скаффолда; двойной fail-closed).
pub fn activate() {
    if !enabled() {
        return;
    }
    if !BLOB_EMBEDDED {
        // Wiring-фаза: cargo-встроенный блоб + define + selfTest рандом-поп
        // оракула. Пока его нет — ранняя лестница честно останавливается здесь.
        eprintln!(
            "[crussty-plugin] inside_acquire: blob not embedded — build entityinside via scripts/build_inside_acquire.sh and wire include_bytes! (arm-hook stays dormant)"
        );
        return;
    }
    std::thread::spawn(|| {
        // 1. Ждать kernel-класс InsideSnapOps (владелец demot-тела site 1;
        //    поллинг inside_cache::activate — sighted ускоряет итерацию).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(SNAP_OPS_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_acquire: {} not loaded within 180s, arm-hook exits dormant",
                    SNAP_OPS_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_acquire: forcing kernel load of {}",
                    SNAP_OPS_CLASS
                );
                crate::improved_noise::force_load_kernel_class(SNAP_OPS_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(SNAP_OPS_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // 2. Kernel loader обязан затихнуть до define (boot-storm; TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_acquire: boot marker not seen, arm-hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // 3. Guard версии класса: embedded major <= major живой JVM (JVM 21 =
        //    65; иначе lever не публикуется).
        //
        // 4. define_class(GATE_CLASS) в kernel loader (ранний arm-хук — ДО
        //    любых read-переписей wiring-фазы; NCDFE-канон).
        //
        // 5. selfTest (InsideAcquireOps.selfTest:Z — single-writer
        //    наблюдаемость publication-пары; false -> БЕЗ публикации
        //    BRIDGE_READY, fail-dominant loud WARN).
        //
        // 6. BRIDGE_READY.store(true) — wiring-фаза переписывает читателей
        //    serve/serve4 + PalettedContainerOps.get на getAcquire и гонит
        //    рандом-поп оракул.
        eprintln!(
            "[crussty-plugin] inside_acquire: define+selfTest+publish ladder is a wiring-phase item (scaffold honest-stop)"
        );
    });
}

// ---------------------------------------------------------------- тесты ----

#[cfg(test)]
mod tests {
    use super::*;

    /// Лестница режимов JMM кумулятивна: Plain < Opaque < Acquire < Volatile
    /// (Lea j9mm; JEP 193). Ранги монотонны и uniq.
    #[test]
    fn mode_ladder_monotonic() {
        assert!(MODE_PLAIN < MODE_OPAQUE);
        assert!(MODE_OPAQUE < MODE_ACQUIRE);
        assert!(MODE_ACQUIRE < MODE_VOLATILE);
    }

    /// Канон карточки v1: демоты — ТОЛЬКО acquire (стат-счётчики остаются
    /// volatile = не демотированы). Ни один сайт не ниже acquire.
    #[test]
    fn v1_acquire_only() {
        assert!(v1_acquire_only_contracts());
        for (site, field, mode, note) in DEMOTION_SITES {
            assert!(
                *mode == MODE_ACQUIRE || *mode == MODE_VOLATILE,
                "site {site} field {field}: mode {mode} вне acquire-канона ({note})"
            );
        }
        // Анти-плейсебо: писатель seqlock остаётся volatile — строка
        // writer-side в таблице отсутствует, проверяем фиксацией счётчика.
        assert_eq!(DEMOTION_SITES.len(), 4);
    }

    /// Dormant-невидимость: gate по умолчанию ВЫКЛ (env не задан в тест-раннере
    /// CI) — enabled_pub()=false, bridge_ready()=false, scaffold честно
    /// fail-closed; таблица сайтов не пуста (контракт жив даже dormant).
    #[test]
    fn dormant_fail_closed() {
        // В CI CRUSSTY_INSIDE_ACQUIRE не ставится; если локально выставлен —
        // тест о невидимости не имеет смысла, пропускаем.
        if std::env::var("CRUSSTY_INSIDE_ACQUIRE").is_ok() {
            return;
        }
        assert!(!enabled_pub());
        assert!(!bridge_ready());
        assert!(!BLOB_EMBEDDED, "scaffold обязан быть honest-stop");
        assert!(!DEMOTION_SITES.is_empty());
    }

    /// NCDFE-индуктивность: жёсткие константы классов совпадают с java-мостом
    /// и сайтом-владельцем demot-тела; FQCN-оверклэпа с живыми мостами нет.
    #[test]
    fn class_names_stable() {
        assert_eq!(GATE_CLASS, "net/minecraft/world/entity/InsideAcquireOps");
        assert_eq!(SNAP_OPS_CLASS, "net/minecraft/world/entity/InsideSnapOps");
    }
}
