//! P43 BRAIN FLAT-MEMORY REGISTRY — rust-носитель реестра Memories (SCAFFOLD,
//! TASK-459-68, идея ID-P43 round-458p-ideas; закон 11 v18.2: scaffold-тик —
//! STRICT-off до оракула, vanilla bytes НЕ трогаются, вердикт = числа).
//!
//! Развитие brainhook-паттерна (F2: flat-snapshot lens по
//! availableBehaviorsByPriority, randomtick/src/BrainOps.java) на
//! memory-плоскость `Brain.memories`:
//!
//!   Map = источник истины; флет = read-зеркало.
//!   - МУТАЦИИ не трогаются: setMemoryInternal/eraseMemory живут в ванильном
//!     HashMap (единственные мутационные сайты по cfdump-скану полей).
//!   - После мутаций тика (post-mutation rebuild, один bulk-вызов на brain)
//!     зеркало перестраивается ОДНИМ Map-обходом: flat[ordinal] = значение
//!     (тот же ref, без клонов) — НЕ snapshot БЕЗ Map-обхода, а snapshot
//!     БЕЗ Map-обхода НА ПУТИ ЧТЕНИЯ: обход платится один раз за тик после
//!     мутаций, а не на каждом getMemory.
//!   - Чтения (getMemory/getMemoryInternal/checkMemory/hasMemoryValue —
//!     javap Brain.cfdump.txt @262-345: каждый = Map.get → checkcast
//!     Optional → INDY Optional.map) уходят в flat[type.ordinal()] — O(1)
//!     array load вместо HashMap.getNode + indy-цепочки.
//!
//! ОРДИНАЛ-РЕЕСТР: MemoryModuleType — BuiltInRegistries-enum (~60 типов,
//! ordinal плотный 0..N). Слот = ordinal; кап REGISTRY_CAP = guard от
//! датапак-роста; ординал вне капа = расхождение → disarm.
//!
//! ПАРИТИ-ЗЕРКАЛО ПОСЛЕ МУТАЦИЙ: fingerprint-пробы (все O(1)):
//!   P1: live map.size() == число non-null слотов зеркала;
//!   P2: identity-проба карты-источника (смена карты = полная перестройка);
//!   P3: точечная parity-проба против свежего rebuild — oracle-only,
//!       на горячем пути не живёт.
//! ЛЮБОЕ расхождение = DISARM-ЛАТЧ (односторонний): чтения навсегда в
//! vanilla Map.get, strict-off до перезапуска (канон «зеркал-расхождение —
//! disarm»; tie-break isEmpty-порядка ванили не нарушается — флет читает
//! ТЕ ЖЕ значения по ТЕМ ЖЕ ключам).
//!
//! STRICT-OFF ДО ОРАКУЛА: body-swap чтений применяется только после
//! selfTestRegistry()==true в kernel loader (TASK-437-A паттерн: selfTest
//! BEFORE arm; BrainOps$Snapshot lazy-resolution урок — nested классы
//! define первыми; здесь их нет вовсе).
//!
//! NCDFE-КАНОН (RC7 / ×422-placebo / ×432-b): define_class в loader Brain'а
//! ТОЛЬКО после фактической загрузки Brain (brainhook::activate poll-луп),
//! всегда init=false (lazy); selfTest до READY-флипа; любой throwable в
//! selfTest/зонде = fail-closed (lane vanilla); rust-CAP — guard, истина —
//! ординалы kernel loader'а.
//!
//! ЭКОНОМИКА (карточка ID-P43): brain-хвост 1-1.5пп CPU × захват 25-35%
//! (доля чтений памяти в лейне, убираемых array-load'ом) → прогноз Δ
//! +0.3-0.5пп. Сайт-счёт по cfdump: 4 горячих читателя + getMemories()
//! (entities-сохранение/цели) — батч-кандидаты iter-2.

use std::sync::atomic::{AtomicBool, Ordering};

/// Ёмкость реестра слотов (java-сторона: BrainFlatOps.REGISTRY_CAP).
/// Vanilla ~60 типов; 512 = запас ×8. Guard, не истина (NCDFE-канон).
pub const REGISTRY_CAP: usize = 512;

const LEVER: &str = "cmp459_p43";

/// STRICT-eq гейт (round-400 lever protocol): scaffold-флаг, пустой/чужой
/// флаг = ваниль бит-в-байт (никаких хуков не регистрируется вовсе).
fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok(LEVER))
}

static SCAFFOLD_LOGGED: AtomicBool = AtomicBool::new(false);

/// Регистрация scaffold-фазы: БЕЗ байт-хуков (закон 11 — vanilla bytes не
/// трогаются до оракула; .class моста ещё не собран, include_bytes появится
/// вместе с build_brainflat_ops.sh в следующем тике). Публичное API для
/// iter-2: register() обязана остаться идемпотентной и без side-effect'ов.
pub fn register() {
    if !enabled() {
        return;
    }
    if !SCAFFOLD_LOGGED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "[crussty-plugin] brain_flat_registry: P43 scaffold armed (lever {LEVER}) — \
             registry model {REGISTRY_CAP} slots; STRICT-off: no byte hooks this tick \
             (oracle selfTestRegistry lands with the BrainFlatOps bridge)"
        );
    }
}

// --------------------------------------------------------------- model
//
// Детерминированная модель зеркала + латча (та же семантика, что контракты
// BrainFlatOps.java): unit-тесты внизу фиксируют parity-инварианты ДО того,
// как java-мост начнёт жить в kernel loader'е (KernelLoaderSim-канон —
// модель A/B до панички define_class).

/// Причина расхождения (grep-ABLE класс для артефакт-верификации).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Divergence {
    /// Ординал вне [0, REGISTRY_CAP).
    SlotOutOfRange,
    /// Коллизия слотов / null-ключ / лишний live — инвариант rebuild.
    RebuildInvariant,
    /// Fingerprint P1: live-счётчик не сошёлся с размером карты.
    SizeProbe,
    /// Fingerprint P2: identity-проба карты-источника не сошлась.
    IdentityProbe,
    /// Fingerprint P3: точечная parity-проба против свежего rebuild.
    ParityProbe,
}

/// Односторонний латч зеркала (канон: расхождение — disarm навсегда).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Latch {
    /// Зеркало синхронно, чтения идут во флет.
    Synced,
    /// Расхождение поймано — rebuild-попытка ещё разрешена (одна).
    Diverged(Divergence),
    /// Латч сработал: чтения навсегда vanilla Map.get (strict-off).
    Disarmed,
}

/// Слот-упаковка: ordinal → индекс флет-массива (контракт с BrainFlatOps).
pub fn slot_of(ordinal: usize) -> Option<usize> {
    (ordinal < REGISTRY_CAP).then_some(ordinal)
}

/// Модель памяти-карты: слот → значение (u32-пейлоад вместо
/// Optional<ExpirableValue>; None = ключа нет — семантика getMemoryInternal).
pub type MapModel = Vec<Option<u64>>;

/// Мутация карты (ванильные сайты: setMemoryInternal put / eraseMemory remove).
#[derive(Debug, Clone, Copy)]
pub enum Mutation {
    Put { ordinal: usize, value: u64 },
    Erase { ordinal: usize },
}

/// Применить мутации к модели карты (Map = источник истины, порядок важен).
pub fn apply(map: &mut MapModel, ops: &[Mutation]) -> Result<(), Divergence> {
    for op in ops {
        match *op {
            Mutation::Put { ordinal, value } => {
                let slot = slot_of(ordinal).ok_or(Divergence::SlotOutOfRange)?;
                if slot >= map.len() {
                    map.resize(slot + 1, None);
                }
                map[slot] = Some(value);
            }
            Mutation::Erase { ordinal } => {
                let slot = slot_of(ordinal).ok_or(Divergence::SlotOutOfRange)?;
                if slot < map.len() {
                    map[slot] = None;
                }
            }
        }
    }
    Ok(())
}

/// Rebuild зеркала ОДНИМ обходом карты (post-mutation): возвращает флет
/// (слот = значение) и live-счётчик; нарушение инвариант = причина расхождения.
pub fn rebuild(map: &MapModel) -> Result<(MapModel, usize), Divergence> {
    let mut flat = vec![None; map.len().max(REGISTRY_CAP)];
    let mut live = 0usize;
    for (slot, value) in map.iter().enumerate() {
        if value.is_some() {
            live += 1;
        }
        flat[slot] = *value;
    }
    // java-мост (BrainFlatOps.rebuildMirror) дополнительно проверяет на
    // общем пути: null-ключ, ординал вне капа, коллизию слотов и
    // live == map.size() — модель держит инвариант по построению.
    Ok((flat, live))
}

/// Fingerprint-пробы P1/P2 (все O(1)) — модель BrainOps.matches для карты
/// памяти: size-проба + identity-проба источника.
pub fn fingerprint(live: usize, map_size: usize, identity_ok: bool) -> Result<(), Divergence> {
    if live != map_size {
        return Err(Divergence::SizeProbe);
    }
    if !identity_ok {
        return Err(Divergence::IdentityProbe);
    }
    Ok(())
}

/// Точечная parity-проба P3 (oracle-only): зеркало против свежего rebuild.
pub fn parity_probe(flat: &MapModel, map: &MapModel) -> Result<(), Divergence> {
    let (fresh, _) = rebuild(map)?;
    if fresh.len() != flat.len() || flat.iter().zip(fresh.iter()).any(|(a, b)| a != b) {
        return Err(Divergence::ParityProbe);
    }
    Ok(())
}

/// Латч-машина: односторонний переход по причине расхождения.
/// Synced → Diverged(why) → Disarmed; Disarmed — терминальное состояние.
pub fn latch_step(current: Latch, why: Divergence) -> Latch {
    match current {
        Latch::Disarmed => Latch::Disarmed,
        Latch::Diverged(_) => Latch::Disarmed,
        Latch::Synced => Latch::Diverged(why),
    }
}

/// Чтение через зеркало с fail-closed фолбэком в Map (модель getFlat):
/// disarmed → всегда Map; иначе flat[slot] (= Map-значение по контракту).
pub fn read_flat(latch: Latch, flat: &MapModel, map: &MapModel, ordinal: usize) -> Option<u64> {
    if matches!(latch, Latch::Disarmed) {
        return map.get(ordinal).copied().flatten();
    }
    let slot = slot_of(ordinal)?;
    flat.get(slot).copied().flatten()
}

// --------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn sync(map: &MapModel) -> (MapModel, Latch) {
        let (flat, live) = rebuild(map).expect("rebuild");
        fingerprint(live, map.iter().filter(|v| v.is_some()).count(), true)
            .expect("fingerprint");
        (flat, Latch::Synced)
    }

    #[test]
    fn slot_cap_guard() {
        assert_eq!(slot_of(0), Some(0));
        assert_eq!(slot_of(REGISTRY_CAP - 1), Some(REGISTRY_CAP - 1));
        assert_eq!(slot_of(REGISTRY_CAP), None);
    }

    #[test]
    fn parity_after_mutation_batch() {
        let mut map: MapModel = vec![None; REGISTRY_CAP];
        apply(
            &mut map,
            &[
                Mutation::Put { ordinal: 3, value: 30 },
                Mutation::Put { ordinal: 17, value: 170 },
                Mutation::Put { ordinal: 59, value: 590 },
            ],
        )
        .expect("apply");
        let (flat, latch) = sync(&map);
        assert_eq!(latch, Latch::Synced);
        assert_eq!(read_flat(latch, &flat, &map, 3), Some(30));
        assert_eq!(read_flat(latch, &flat, &map, 17), Some(170));
        assert_eq!(read_flat(latch, &flat, &map, 1), None); // REGISTERED-различие у читателя
        // erase-партия → rebuild → parity (канон: зеркало ПОСЛЕ мутаций)
        apply(&mut map, &[Mutation::Erase { ordinal: 17 }, Mutation::Put { ordinal: 4, value: 40 }])
            .expect("apply");
        let (flat, latch) = sync(&map);
        assert_eq!(read_flat(latch, &flat, &map, 17), None);
        assert_eq!(read_flat(latch, &flat, &map, 4), Some(40));
        parity_probe(&flat, &map).expect("parity");
    }

    #[test]
    fn divergence_detected_and_disarmed_forever() {
        let mut map: MapModel = vec![None; REGISTRY_CAP];
        apply(&mut map, &[Mutation::Put { ordinal: 6, value: 60 }]).expect("apply");
        let (mut flat, latch) = sync(&map);
        assert_eq!(latch, Latch::Synced);
        // инъекция фантома (java-модель: flat[7] = Some вне карты)
        flat[7] = Some(777);
        assert_eq!(parity_probe(&flat, &map), Err(Divergence::ParityProbe));
        // латч: один шаг = Diverged, второй = Disarmed (терминальное)
        let l1 = latch_step(latch, Divergence::ParityProbe);
        assert!(matches!(l1, Latch::Diverged(Divergence::ParityProbe)));
        let l2 = latch_step(l1, Divergence::ParityProbe);
        assert_eq!(l2, Latch::Disarmed);
        // после disarm чтение = vanilla Map (фантом не утекает)
        assert_eq!(read_flat(l2, &flat, &map, 7), None);
        assert_eq!(read_flat(l2, &flat, &map, 6), Some(60));
        // идемпотентность терминального состояния
        assert_eq!(latch_step(l2, Divergence::SizeProbe), Latch::Disarmed);
    }

    #[test]
    fn out_of_range_ordinal_is_divergence() {
        let mut map: MapModel = vec![None; REGISTRY_CAP];
        assert_eq!(
            apply(&mut map, &[Mutation::Put { ordinal: REGISTRY_CAP + 1, value: 1 }]),
            Err(Divergence::SlotOutOfRange)
        );
    }

    #[test]
    fn size_probe_catches_dropped_slot() {
        // карта живёт, зеркало потеряло слот → P1 бьёт тревогу
        let map: MapModel = vec![Some(9), Some(8), None];
        let live = 1; // потерян один non-null
        assert_eq!(fingerprint(live, 2, true), Err(Divergence::SizeProbe));
        assert_eq!(fingerprint(2, 2, false), Err(Divergence::IdentityProbe));
        assert_eq!(fingerprint(2, 2, true), Ok(()));
        let _ = map;
    }

    #[test]
    fn empty_map_tiebreak_is_empty_mirror() {
        let map: MapModel = vec![];
        let (flat, latch) = sync(&map);
        assert_eq!(latch, Latch::Synced);
        assert!(flat.iter().all(|v| v.is_none()));
    }
}
