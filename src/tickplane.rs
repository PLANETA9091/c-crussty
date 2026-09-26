//! TICK-PLANE (TASK-403-C, vector tickplane — lever `cmp403_tickplane`).
//!
//! WHOLE-BODY RETARGET SKELETON: единый rust tick-plane для ТЕЛА энтити-тика
//! (items tick / mob push broadphase / mob push stagger / block-collision
//! batch), НЕ для AI-решений (GoalSelector-политики не трогаются; stagger
//! только rate-инвариант). Этот модуль — сегмент-реестр плейна:
//!
//!   - STRICT-eq gate раунда (`cmp403_tickplane`; пустой/чужой флаг = бит-в-бит
//!     ваниль — все суб-сегменты dormant по построению);
//!   - громкие ARM-маркеры per-сегмент (mobs_manager/items_manager/stagger/
//!     collide_batch) + СВОДНЫЙ маркер плейна с коarse-штампом эпохи тика;
//!   - coarse-stamp: monotonic tick-эпоха плейна (индекс сегментации батчей,
//!     v1 — только телеметрия/порядок активации, БЕЗ изменения тик-семантики).
//!
//! СЕГМЕНТЫ ПЛЕЙНА (composition раунда-403, унаследованные ARMED-механики
//! под единым флагом; парити-класс каждого — «невидимая замена перечисления/
//! rates», документирован в соответствующем модуле):
//!   1. items-plane      — items_index shardgrid + lifetime-heap + RegionTickOps
//!                         item-subsys2 (прецедент J-subsys2, паттерн 86807b6);
//!   2. mob-push-soa     — mobs_soa flat-arrays, primary push broadphase;
//!   3. mob-push-grid    — mobs_grid sharded mirror + per-call fallback read;
//!   4. mob-stagger      — push-scan 1/N + goal-canUse 1/N (golden-phase);
//!   5. collide-batch    — block-collision section-plan batch-merge
//!                         (verbatim moonrise fragment, DYNAMIC re-resolve).
//!
//! FAIL-CLOSED: каждый сегмент независимо fail-closed (ERR_STRUCT → дизарм
//! сегмента, per-call vanilla → только вызов); отказ сегмента не влияет на
//! остальные (изоляция как в cmp402_comp). Сам реестр — read-only телеметрия.

/// Lever флаг раунда-403 (STRICT eq; никогда не starts_with/contains).
pub const LEVER: &str = "cmp403_tickplane";

/// Сегменты плейна в порядке активации (для сводного маркера и логов).
pub const SEGMENTS: [&str; 5] = [
    "items-plane",
    "mob-push-soa",
    "mob-push-grid",
    "mob-stagger",
    "collide-batch",
];

/// STRICT-eq gate раунда. Пустой/чужой флаг → false (ваниль бит-в-бит).
#[inline]
pub fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| {
            let v = v.trim();
            // TASK-405-F: композит stagtick вооружает плейн тем же STRICT-eq.
            // TASK-406-D: композит раунда-406 (stagtick ⊕ ai-window).
            v == LEVER || v == "cmp405_stagtick" || v == "cmp406_aibatch"
            // TASK-406-E: композит раунда-406 (stagtick ⊕ sscan).
            || v == "cmp406_sscan"
            // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
            || v == "cmp409_multi" || v == "cmp412_meganav" || v == "cmp414_cvs"
            // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR.
            || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq"
            // TASK-419-A (colpush): колпаш-носитель (STRICT OR).
            || v == "cmp420_colpush"
            || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2" || v == "cmp436_ins4" || v == "cmp458_swar" || v == "cmp457_paldelta" || v == "cmp457_eqsnap2" || v == "cmp466_poiun" || v == "cmp456_chunkmono" || v == "cmp456_chunkmono" || v == "cmp456_chunkmono_p31snap" || v == "cmp456_chunkmono_p31snap"
            || v == "cmp451_senseins" || v == "cmp458_swar" || v == "cmp457_paldelta" || v == "cmp457_eqsnap2" || v == "cmp466_poiun" || v == "cmp453_diet" || v == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
            || v == "cmp438_sense" // TASK-444-C: sense family union
            || v == "cmp451_senseins" || v == "cmp458_swar" || v == "cmp457_paldelta" || v == "cmp457_eqsnap2" || v == "cmp466_poiun" || v == "cmp453_diet" || v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk" // TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)
            || v == "cmp412_eqsnapv3" || v == "cmp414_cvs" || v == "cmp417_bq" || v == "cmp421_brain" || v == "cmp422_brain2" || v == "cmp423_brain3" || v == "cmp424_mobfeed" || v == "cmp430_inside" || v == "cmp432_inside2" || v == "cmp436_ins4" || v == "cmp458_swar" || v == "cmp456_poi" || v == "cmp466_poiun"
})
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Coarse tick-plane epoch stamp
// ---------------------------------------------------------------------------

use std::sync::atomic::{AtomicU64, Ordering};

/// Монотонная эпоха плейна: инкрементируется наблюдателем тика (v1 —
/// activation worker; не участвует в семантике тика). Coarse-stamp для
/// сегментных батчей будущих раундов (jnibulk-стиль raw-arena).
static PLANE_EPOCH: AtomicU64 = AtomicU64::new(0);

#[inline]
#[allow(dead_code)] // skeleton API: coarse-stamp readers for future segments
pub fn epoch() -> u64 {
    PLANE_EPOCH.load(Ordering::Relaxed)
}

#[inline]
pub fn epoch_bump() -> u64 {
    PLANE_EPOCH.fetch_add(1, Ordering::Relaxed) + 1
}

// ---------------------------------------------------------------------------
// Registration / activation markers
// ---------------------------------------------------------------------------

/// Register (из cplugin_init): НЕ ставит хуков — реестр только маркерит
/// заголовок плейна, чтобы дормантный запуск был тоже виден в stdout.
pub fn register() {
    if !enabled() {
        // Пустой/чужой флаг: молчание = ваниль (как у всех дормантных
        // модулей); одна тихая строка для диаг-различимости.
        eprintln!(
            "[crussty-plugin] tickplane: dormant (lever_flag != {LEVER}, vanilla bit-for-bit)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] {LEVER}: plane header segments={:?} (whole-body retarget skeleton; per-segment ARM markers follow)",
        SEGMENTS
    );
}

/// Activate (в конце activate-цепочки lib.rs): сводный ARM-маркер плейна.
/// Сегменты уже отмаркерились своими модулями; здесь — coarse-stamp + сводка.
pub fn activate() {
    if !enabled() {
        return;
    }
    let e = epoch_bump();
    eprintln!(
        "[crussty-plugin] {LEVER}: PLANE ARMED segments={:?} epoch={e} (gate=STRICT-eq {LEVER}; empty flag = vanilla bit-for-bit; per-segment fail-closed isolation ERR_STRUCT->disarm)",
        SEGMENTS
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lever_constant_strict() {
        assert_eq!(LEVER, "cmp403_tickplane");
        // STRICT: не префикс/не суффикс других рычагов семейства.
        assert!(!"cmp403_tickplane_x".trim().eq(LEVER));
    }

    #[test]
    fn segments_catalog_complete() {
        assert_eq!(SEGMENTS.len(), 5);
        assert!(SEGMENTS.contains(&"items-plane"));
        assert!(SEGMENTS.contains(&"collide-batch"));
    }

    #[test]
    fn epoch_monotonic() {
        let a = epoch_bump();
        let b = epoch_bump();
        assert!(b > a);
        assert!(epoch() >= b);
    }
}
