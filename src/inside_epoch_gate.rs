//! INSIDE-EPOCH GATE (ID-P36, TASK-459-58 scaffold — закон 11 v18.2).
//!
//! ПОДСИСТЕМА: дешёвый пре-гейт перед полным путём inside-snapshot serve
//! (комбо к P32/inside_cache). Карточка ID-P36 (round-458p-ideas):
//! per-section счётчик мутаций (bump УЖЕ ЕСТЬ в secWrite ->
//! InsideSnapOps.Snap.gen++, volatile, после write) -> ФЛЕТ-МАССИВ ЭПОХ;
//! гейт сначала сравнивает эпоху секции с эпохой слота кэша —
//! несовпадение -> полный путь serve/serve4 -> его miss -> ВАНИЛЬ.
//!
//! Состав подсистемы (закон 6 — НЕ одиночная функция):
//!   1. JAVA-МОСТ InsideEpochGate (entityinside/…/InsideEpochGate.java):
//!      флет long[CAP] EPOCHS (CAP=1<<15, калиброван с InsideSnapOps.CAP),
//!      bind/epochMatch/unbind-механика, LongAdder-статы, selfTest
//!      hit-инварианта. Scaffold = vanilla-free (голый javac --release 21).
//!   2. ARM-HOOK (этот модуль, activate): РАННЕЕ определение моста в kernel
//!      loader — NCDFE-канон (×93-indy, run 35902792520: класс обязан быть
//!      DEFINED до первого gated-вызова; в <clinit> моста нет indy/indy-
//!      method-ref — plain-паттерн InsideSnapOps.java:241-254). Порядок
//!      как у inside_cache: define ДО entity_compose-этапа, BRIDGE_READY
//!      публикуется только после успешного selfTest (fail-dominant).
//!   3. WIRING-ФАЗА (после скаффолда, НЕ в этом коммите): include_bytes!
//!      блоба entityinside/build/…/InsideEpochGate.class (сборка —
//!      scripts/build_inside_epoch_gate.sh), вставка fast-gate в HEAD
//!      InsideSnapOps.snapGet (до serve/serve4; receiver-first, длина
//!      сохранена), ref-equal самтест slot-serve vs полный serve.
//!
//! PARITY (бит-в-байт):
//!   - miss -> ваниль: несовпадение эпохи/слота = исполнение существующего
//!     пути, новых источников истины нет;
//!   - hit-инвариант самтестом при арме (N>=3 позиций на прогретую секцию,
//!     slot-serve обязан вернуть ТОТ ЖЕ ref, что полный serve — тот же
//!     объект, что ванильный readPalette);
//!   - эпоха 64-бит monotonic per-section: ABA/wrap недостижимы (2^63
//!     реальных ref-неравных записей в одну секцию); слот ref-привязан к
//!     секции (SNAPS не эвиктит — sec->snap immutable).
//!
//! ПРОГНОЗ (карточка): −0.5-1пп java_util/inside хвостов В СВЯЗКЕ с P32.
//! База: inside-лейн ≈1.3-1.6% total CPU (PROFILE-B, inside_snap.rs:3-7);
//! serve-хвост (lane-claim+CHM) ≈30-50% среза.
//!
//! Gate: env `CRUSSTY_INSIDE_EPOCH_GATE` (1/true/on/yes -> on). OFF BY
//! DEFAULT — dormant-невидимость (дисциплина inside_cache/fluid_guard/
//! alloc_diet): при выключенном флаге register() пишет dormant-notice,
//! activate() возвращается немедленно, классы не определяются, байты
//! ванильных классов не читаются — модуль байт-в-байт невидим.
//!
//! СТАТУС: SCAFFOLD (law-11 финал = {run id, ветка+SHA, вердикт-число}).
//! `BLOB_EMBEDDED=false`: activate-лестница полная, но define-шаг обязан
//! остаться no-op до приземления блоба (include_bytes! несуществующего
//! .class = красный cargo build в CI). Lever dormant по умолчанию И по
//! статусу скаффолда — двойной fail-closed.

// Scaffold-модуль: publish-поверхность (enabled_pub/bridge_ready/GATE_CLASS)
// оживают в wiring-фазе — dead_code до неё ожидаем, не шумим в CI.
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const GATE_CLASS: &str = "net/minecraft/world/entity/InsideEpochGate";

/// Scaffold-маркер: блоб ещё не встроен (wiring-фаза ставит true вместе с
/// include_bytes! ../entityinside/build/net/minecraft/world/entity/InsideEpochGate.class).
const BLOB_EMBEDDED: bool = false;

fn enabled() -> bool {
    std::env::var("CRUSSTY_INSIDE_EPOCH_GATE")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Gate-видимость для будущих compose-этапов (entity_compose stage / внутри
/// inside_snap::snapGet-вставки wiring-фазы).
pub fn enabled_pub() -> bool {
    enabled()
}

/// Публикуется ТОЛЬКО после define+selfTest (fail-dominant; поллинг
/// wiring-фазы/compose-этапа — паттерн inside_cache::wait_bridge_ready).
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);

pub fn bridge_ready() -> bool {
    BRIDGE_READY.load(Ordering::Acquire)
}

/// Register (idempotent; call once from cplugin_init). Byte hook НЕ ставится
/// ни здесь, ни в activate: ретаргет-сайт один (InsideSnapOps.snapGet HEAD)
/// и приземляется в wiring-фазе через существующую compose-секвенцию.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] inside_epoch_gate: dormant (set CRUSSTY_INSIDE_EPOCH_GATE=1 to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] inside_epoch_gate: scaffold armed-flat (P36; define-site = early arm-hook, retarget lands in wiring phase)"
    );
}

/// РАННИЙ ARM-ХУК (NCDFE-канон): ждёт kernel-класс Entity, boot-маркер,
/// затем define InsideEpochGate в kernel loader и selfTest ДО публикации
/// BRIDGE_READY. Секвенция повторяет inside_cache::activate (boot-storm
/// дисциплина TASK-80, silent-until-quiet).
///
/// SCAFFOLD: до приземления блоба (BLOB_EMBEDDED=false) лестница доходит
/// только до loud scaffold-notice — НИКАКИХ define/читов байтов (lever
/// dormant и по флагу, и по статусу скаффолда; двойной fail-closed).
pub fn activate() {
    if !enabled() {
        return;
    }
    if !BLOB_EMBEDDED {
        // Wiring-фаза: `cargo`-встроенный блоб + define + selfTest.
        // Пока его нет — ранняя лестница честно останавливается здесь.
        eprintln!(
            "[crussty-plugin] inside_epoch_gate: blob not embedded — build entityinside via scripts/build_inside_epoch_gate.sh and wire include_bytes! (arm-hook stays dormant)"
        );
        return;
    }
    std::thread::spawn(|| {
        // 1. Ждать kernel-класс Entity (грузится на старте; поллинг
        //    inside_cache::activate — sighted ускоряет итерацию).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ENTITY_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] inside_epoch_gate: {} not loaded within 180s, arm-hook exits dormant",
                    ENTITY_CLASS
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] inside_epoch_gate: forcing kernel load of {}",
                    ENTITY_CLASS
                );
                crate::improved_noise::force_load_kernel_class(ENTITY_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ENTITY_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // 2. Kernel loader обязан затихнуть до define (boot-storm; TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!(
                "[crussty-plugin] inside_epoch_gate: boot marker not seen, arm-hook stays dormant"
            );
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // 3. Guard версии класса (паттерн inside_cache: embedded major
        //    обязан быть <= major живой JVM; иначе lever не публикуется).
        //    Wiring-фаза: class_version(GATE_BYTES) против jvm_class_major.
        //
        // 4. define_class(GATE_CLASS) в kernel loader (ранний arm-хук —
        //    ДО любых retarget-этапов compose-цепочки; NCDFE-канон).
        //
        // 5. selfTest (InsideEpochGate.selfTest:Z): false -> БЕЗ публикации
        //    BRIDGE_READY (fail-dominant, loud WARN).
        //
        // 6. BRIDGE_READY.store(true) — wiring-фаза вставляет fast-gate в
        //    HEAD InsideSnapOps.snapGet и ref-equal самтест slot-serve.
        eprintln!(
            "[crussty-plugin] inside_epoch_gate: define+selfTest+publish ladder is a wiring-phase item (scaffold honest-stop)"
        );
    });
}
