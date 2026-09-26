//! LEVER DISPATCHER — single-pass bitmask of armed lever families (x466-C99).
//!
//! ИНВЕРСИЯ (закон «что УБРАТЬ для скорости»): каждый гейт-канон сохранён —
//! НИ ОДНА lever-проверка не удалена; удалена только РАБОТА на проверку:
//!   - было: `std::env::var("CRUSSTY_LEVER_FLAG")` на КАЖДЫЙ вызов гейта
//!     (glibc getenv + heap String alloc) + цепочка 5..40 строковых `==`
//!     с дублями-идентификаторами (x451/x452 retag-эра);
//!   - стало: ОДИН env-чтение на процесс (`flag()`), ОДИН проход по реестру
//!     IDS (`bits_of`), дальше каждый гейт = одна битовая операция
//!     (`armed(MASK)` / `armed_flag(s, MASK)`).
//!
//! Паритет (закон 4, бит-в-бит): `armed_flag(s, MASK)` — чистая функция
//! (флаг-строка ⊕ маска), поэтому все прежние cargo-тесты `enabled_with(..)`
//! остаются валидными без изменений, а tests-модуль ниже pins каждую маску
//! против СТАРОГО строкового списка (скопирован из master до патча —
//! оракул из origin/master 9bb43fe9).
//!
//! javap/ценз x466-C99 (master 9bb43fe9): java-мосты пекут гейты в <clinit>
//! (ENABLED final), per-call = volatile/atomic чтения; PER-TICK env+цепочка
//! жили на RUST-стороне: eq_epoch (1/тик, ~40 eq + dupes), sense_arena
//! (1/тик, ~30 eq), poi_epoch (1/тик, 23 eq), sscan_epoch (1/тик, ~35 eq),
//! sense_epoch (1/тик, ~22 eq), inside_batch_mask (1/тик, 2×env).
//! ≈5 getenv + ≈150 строковых сравнений + 5 String-alloc на тик → 0.

/// Единый реестр легаси lever-идентификаторов (dedup, порядок фиксирован).
/// Бит i == позиция в этом списке. НЕИЗВЕСТНЫЙ id в маске-списке → паника
/// при первом вычислении маски (fail-fast, тише = DELIVERY-FAIL x452 класс).
const IDS: &[&str] = &[
    "items_subsys2",
    "cmp399_despawn2",
    "cmp401_collide",
    "cmp401_soa",
    "cmp401_stagger",
    "cmp402_comp",
    "cmp402_stagcomp",
    "cmp403_tickplane",
    "cmp405_eindex",
    "cmp405_stagtick",
    "cmp406_aibatch",
    "cmp406_sscan",
    "cmp409_multi",
    "cmp410_eindexq",
    "cmp411_eqsnap",
    "cmp411_k4soa",
    "cmp412_b2p1",
    "cmp412_eqsnapv3",
    "cmp412_meganav",
    "cmp414_cvs",
    "cmp415_mcomp",
    "cmp416_mcomp",
    "cmp417_bq",
    "cmp420_colpush",
    "cmp421_brain",
    "cmp421_chunk",
    "cmp422_brain2",
    "cmp423_brain3",
    "cmp423_wgen",
    "cmp424_mobfeed",
    "cmp430_inside",
    "cmp432_inside2",
    "cmp434_chunkpl",
    "cmp435_chunk3",
    "cmp436_ins4",
    "cmp437_chunk4",
    "cmp438_sense",
    "cmp444_chunk5",
    "cmp450_chunk",
    "cmp451_senseins",
    "cmp452_mega",
    "cmp453_diet",
    "cmp455_spawn",
    "cmp456_chunkmono",
    "cmp456_chunkmono_p31snap",
    "cmp456_poi",
    "cmp457_eqsnap2",
    "cmp457_paldelta",
    "cmp458_swar",
    "cmp459_snapreg",
    "cmp464_occ",
    "cmp466_poiun",
];

/// Бит идентификатора в реестре (None = id вне реестра → в маски не входит).
fn bit_of(id: &str) -> Option<u64> {
    IDS.iter()
        .position(|c| *c == id)
        .map(|i| 1u64 << i)
}

/// ОДИН проход по реестру: какие биты задаёт этот флаг. Чистая функция —
/// основа паритета (старые списки vs маски) и тестов `enabled_with`.
pub fn bits_of(flag: &str) -> u64 {
    let mut bits = 0u64;
    for (i, id) in IDS.iter().enumerate() {
        if flag == *id {
            bits |= 1u64 << i;
        }
    }
    bits
}

/// Кэш обрезанного CRUSSTY_LEVER_FLAG (одно чтение env на процесс; env
/// конфигурируется харнессом ДО старта JVM — канон <clinit>-печки java-мостов).
pub fn flag() -> &'static str {
    static FLAG: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    *FLAG.get_or_init(|| match std::env::var("CRUSSTY_LEVER_FLAG") {
        Ok(v) => Box::leak(v.trim().to_string().into_boxed_str()),
        Err(_) => "",
    })
}

/// Биты актуального флага (одно вычисление на процесс).
pub fn bits() -> u64 {
    static BITS: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *BITS.get_or_init(|| bits_of(flag()))
}

/// Гейт: хоть один бит маски armed? (одна load + AND на вызов).
#[inline]
pub fn armed(mask: u64) -> bool {
    bits() & mask != 0
}

/// Гейт для произвольной строки-флага (тесты `enabled_with` + паритет).
#[inline]
pub fn armed_flag(flag: &str, mask: u64) -> bool {
    bits_of(flag) & mask != 0
}

/// Кэшированный truthy-переключатель env-люка (1/true/on/yes, any-case) —
/// канон люков CRUSSTY_SENSE / CRUSSTY_INSIDE_BATCH / CRUSSTY_PALETTED_DEMUX.
pub fn env_switch_cached(name: &str) -> bool {
    static SWITCHES: std::sync::OnceLock<std::collections::HashMap<&'static str, bool>> =
        std::sync::OnceLock::new();
    let m = SWITCHES.get_or_init(|| {
        let mut m = std::collections::HashMap::new();
        for n in ["CRUSSTY_SENSE", "CRUSSTY_INSIDE_BATCH", "CRUSSTY_PALETTED_DEMUX"] {
            let v = std::env::var(n)
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase();
            m.insert(n, v == "1" || v == "true" || v == "on" || v == "yes");
        }
        m
    });
    m.get(name).copied().unwrap_or(false)
}

// --- маски плоскостей (списки = дословно старые OR-цепи, дубли схлопнуты) --

/// entity_query::enabled (flag_enabled) — eq_epoch/sense_arena/probe гейты.
pub const M_EQ: u64 = mask(&[
    "cmp410_eindexq", "cmp411_k4soa", "cmp411_eqsnap",
    "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq",
    "cmp420_colpush", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense", "cmp451_senseins",
    "cmp453_diet", "cmp456_poi", "cmp421_brain",
    "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5",
    "cmp450_chunk", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
]);

/// entity_query::enabled_flag_is_k4.
pub const M_K4: u64 = mask(&["cmp411_k4soa"]);

/// entity_query::enabled_flag_is_eqsnap.
pub const M_EQSNAP: u64 = mask(&["cmp411_eqsnap", "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq", "cmp421_brain"]);

/// entity_query::enabled_flag_is_eqsnapv3.
pub const M_EQSNAPV3: u64 = mask(&["cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq"]);

/// entity_query::enabled_flag_is_sense (sense_arena STRICT-носитель).
pub const M_SENSEQ: u64 = mask(&[
    "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense", "cmp451_senseins",
    "cmp456_chunkmono", "cmp453_diet", "cmp450_chunk",
    "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5",
    "cmp456_chunkmono_p31snap", "cmp456_poi",
]);

/// entity_query::enabled_flag_is_brain2.
pub const M_BRAIN2: u64 = mask(&["cmp422_brain2"]);

/// poi_plane::enabled — poi_epoch/poi_bind_mask/poi_probe гейты.
pub const M_POI: u64 = mask(&[
    "cmp456_poi", "cmp409_multi", "cmp412_meganav", "cmp412_eqsnapv3",
    "cmp414_cvs", "cmp417_bq", "cmp420_colpush", "cmp421_brain",
    "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp438_sense",
    "cmp451_senseins", "cmp453_diet", "cmp452_mega",
    "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
    "cmp444_chunk5", "cmp450_chunk",
]);

/// mobs_sscan::enabled — sscan_probe/sscan_epoch гейты.
pub const M_SSCAN: u64 = mask(&[
    "cmp406_sscan", "cmp409_multi", "cmp412_meganav", "cmp414_cvs",
    "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush", "cmp422_brain2",
    "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside", "cmp432_inside2",
    "cmp436_ins4", "cmp458_swar", "cmp457_paldelta", "cmp457_eqsnap2",
    "cmp438_sense", "cmp451_senseins", "cmp456_chunkmono",
    "cmp453_diet", "cmp450_chunk", "cmp456_chunkmono_p31snap",
    "cmp421_brain", "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
    "cmp444_chunk5", "cmp456_poi",
]);

/// mobs_sense::enabled (после CRUSSTY_SENSE-люка) — sense_probe/sense_epoch.
pub const M_SENSEPLANE: u64 = mask(&[
    "cmp438_sense", "cmp406_sscan", "cmp409_multi", "cmp412_meganav",
    "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq", "cmp420_colpush",
    "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp451_senseins", "cmp458_swar", "cmp457_paldelta",
    "cmp457_eqsnap2", "cmp456_chunkmono", "cmp453_diet", "cmp450_chunk",
    "cmp456_chunkmono_p31snap", "cmp456_poi",
]);

/// inside_batch::enabled — STRICT-eq ветка cmp456_chunkmono_p31snap
/// (люк CRUSSTY_INSIDE_BATCH остаётся отдельным динамическим ИЛИ).
pub const M_INSIDE_BATCH: u64 = mask(&["cmp456_chunkmono_p31snap"]);

/// paletted.rs гейт (paldelta-носитель; сворм-x457: paldelta ⊕ swarx).
pub const M_PALDELTA: u64 = mask(&["cmp457_paldelta", "cmp458_swar"]);

/// inside_snap::lever_flag_matches (cmp432_inside2-семья).
pub const M_INSIDE_SNAP: u64 = mask(&[
    "cmp432_inside2", "cmp430_inside", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense", "cmp451_senseins",
    "cmp456_chunkmono", "cmp456_chunkmono_p31snap", "cmp453_diet",
    "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5",
    "cmp450_chunk", "cmp456_poi",
]);

/// inside_snap::v4_requested (V4 serve-тело).
pub const M_INSIDE_V4: u64 = mask(&[
    "cmp436_ins4", "cmp451_senseins", "cmp458_swar", "cmp457_paldelta",
    "cmp457_eqsnap2", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
    "cmp453_diet", "cmp450_chunk", "cmp456_poi",
]);

/// chunk_sched::lever_flag_matches (cmp456_chunkmono-семья).
pub const M_CHUNKSCHED: u64 = mask(&[
    "cmp456_chunkmono", "cmp420_colpush", "cmp421_chunk", "cmp421_brain",
    "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside",
    "cmp432_inside2", "cmp436_ins4", "cmp458_swar", "cmp451_senseins",
    "cmp453_diet", "cmp450_chunk", "cmp434_chunkpl", "cmp435_chunk3",
    "cmp437_chunk4", "cmp444_chunk5", "cmp452_mega", "cmp455_spawn",
    "cmp456_poi", "cmp466_poiun", "cmp456_chunkmono_p31snap",
]);

/// queryplane::lever_flag_matches (cmp412_b2p1-семья).
pub const M_QUERYPLANE: u64 = mask(&[
    "cmp412_b2p1", "cmp415_mcomp", "cmp416_mcomp", "cmp417_bq",
    "cmp420_colpush", "cmp421_brain", "cmp422_brain2", "cmp423_brain3",
    "cmp424_mobfeed", "cmp430_inside", "cmp432_inside2", "cmp438_sense",
    "cmp451_senseins", "cmp458_swar", "cmp457_paldelta", "cmp457_eqsnap2",
    "cmp466_poiun", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
    "cmp453_diet", "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
    "cmp444_chunk5", "cmp450_chunk", "cmp456_poi",
]);

/// collide_batch::lever_flag_matches.
pub const M_COLLIDE: u64 = mask(&[
    "cmp401_collide", "cmp403_tickplane", "cmp405_stagtick",
    "cmp406_aibatch", "cmp406_sscan", "cmp409_multi", "cmp412_meganav",
    "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush",
    "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp456_chunkmono",
    "cmp456_chunkmono_p31snap", "cmp451_senseins", "cmp453_diet",
    "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl", "cmp435_chunk3",
    "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
]);

/// collide_batch сегментный ARM-маркер (TASK-403-C цепочка).
pub const M_COLLIDE_MARKER: u64 = mask(&[
    "cmp403_tickplane", "cmp405_stagtick", "cmp406_aibatch", "cmp406_sscan",
    "cmp409_multi", "cmp412_meganav", "cmp414_cvs", "cmp412_eqsnapv3",
    "cmp417_bq", "cmp420_colpush", "cmp421_brain", "cmp422_brain2",
    "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside", "cmp432_inside2",
    "cmp436_ins4", "cmp458_swar", "cmp457_paldelta", "cmp457_eqsnap2",
    "cmp456_chunkmono", "cmp456_chunkmono_p31snap", "cmp451_senseins",
    "cmp453_diet", "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl",
    "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
]);

/// mobs_manager::java_gate_matches (SoA-плоскость).
pub const M_SOA: u64 = mask(&[
    "cmp401_soa", "cmp402_comp", "cmp402_stagcomp", "cmp403_tickplane",
    "cmp405_stagtick", "cmp406_aibatch", "cmp409_multi", "cmp412_meganav",
    "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush",
    "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp466_poiun",
    "cmp456_chunkmono", "cmp456_chunkmono_p31snap", "cmp451_senseins",
    "cmp453_diet", "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl",
    "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
    "cmp406_sscan", "cmp410_eindexq", "cmp411_k4soa", "cmp411_eqsnap",
]);

/// items_manager::lever_flag_matches_for — точные id БЕЗ cmp399_-семьи
/// (cmp399_* покрывается starts_with-проверкой на месте вызова).
pub const M_ITEMS: u64 = mask(&[
    "items_subsys2", "cmp402_comp", "cmp402_stagcomp", "cmp403_tickplane",
    "cmp405_stagtick", "cmp406_aibatch", "cmp409_multi", "cmp412_meganav",
    "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush",
    "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
    "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
    "cmp457_paldelta", "cmp457_eqsnap2", "cmp466_poiun",
    "cmp456_chunkmono", "cmp456_chunkmono_p31snap", "cmp451_senseins",
    "cmp453_diet", "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl",
    "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
    "cmp406_sscan",
]);

/// items_manager::activate comp — СОСТОЯНИЕ МАСТЕР-БИТ-В-БИТ: цепь сегодня
/// молча обрезана на cmp420_colpush (битый хвост `|| flag == ...` после `;`
/// парсится как НЕИСПОЛЬЗУЕМЫЙ кложур — 165-е предупреждение unused_must_use;
/// хвостовые id'ы в comp НЕ входят). Восстановление хвоста = смена поведения
/// despawn2 на мастер-носителе → отдельная нога (BOARD REF C99-DEADTAIL).
pub const M_ITEMS_COMP: u64 = mask(&[
    "cmp402_comp", "cmp402_stagcomp", "cmp403_tickplane", "cmp405_stagtick",
    "cmp406_aibatch", "cmp406_sscan", "cmp409_multi", "cmp412_meganav",
    "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush",
]);

/// items_manager::activate despawn2 — флаговый слагаемый (|| bfcomp || comp).
pub const M_ITEMS_DESPAWN2: u64 = mask(&["cmp399_despawn2"]);

/// Константная сборка маски из списка id (все id обязаны быть в IDS;
/// неизвестный id → compile-time паника — fail-fast против drift-класса x452).
const fn mask(ids: &[&str]) -> u64 {
    let mut bits = 0u64;
    let mut i = 0;
    while i < ids.len() {
        let mut j = 0;
        let mut found = false;
        while j < IDS.len() {
            if str_eq(ids[i], IDS[j]) {
                bits |= 1u64 << j;
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            panic!("unknown lever id in plane mask");
        }
        i += 1;
    }
    bits
}

/// const-fn строковое равенство (байт за байтом; ASCII-литералы).
const fn str_eq(a: &str, b: &str) -> bool {
    let (ab, bb) = (a.as_bytes(), b.as_bytes());
    if ab.len() != bb.len() {
        return false;
    }
    let mut i = 0;
    while i < ab.len() {
        if ab[i] != bb[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_no_duplicates() {
        for (i, a) in IDS.iter().enumerate() {
            for b in IDS.iter().skip(i + 1) {
                assert_ne!(a, b, "duplicate id {a}");
            }
        }
    }

    #[test]
    fn bits_of_exact_and_negative() {
        assert_eq!(bits_of("cmp456_chunkmono"), mask(&["cmp456_chunkmono"]));
        assert_eq!(bits_of("cmp456_chunkmono_x"), 0, "негативные _x-пробы = 0");
        assert_eq!(bits_of(""), 0);
        assert_eq!(bits_of(" cmp456_poi "), 0, "trim делается ДО bits_of");
    }

    #[test]
    fn armed_flag_single_bit() {
        assert!(armed_flag("cmp457_paldelta", M_PALDELTA));
        assert!(armed_flag("cmp458_swar", M_PALDELTA));
        assert!(!armed_flag("cmp456_poi", M_PALDELTA));
    }

    #[test]
    fn masks_are_stable_across_calls() {
        // маски — константы времени компиляции: чистые биты, детерминизм.
        assert_eq!(M_K4, M_K4);
        assert!(M_EQ.count_ones() >= 27);
        assert!(M_SOA.count_ones() >= 35);
    }

    // ------------------------------------------------------------------
    // ПАРИТЕТ x466-C99 (закон 4): каждая маска pinned против СТАРОГО
    // строкового OR-списка master 9bb43fe9 (дубли схлопнуты, состав идентичен).
    // Кандидаты = весь реестр IDS (позитив + негатив) + _x-пробы + спец-строки.
    // ------------------------------------------------------------------
    fn candidates() -> Vec<&'static str> {
        let mut v: Vec<&'static str> = IDS.to_vec();
        v.extend([
            "cmp406_aibatch_x", "cmp410_eindexq_x", "cmp411_eqsnap_x",
            "cmp411_k4soa_x", "cmp412_eqsnapv3_x", "cmp417_bq_x",
            "cmp421_brain_x", "cmp430_inside_x", "cmp451_senseins_x",
            "cmp456_chunkmono_x", "cmp456_chunkmono_p31snap_x",
            "cmp456_poi_x", "cmp401_soa_x", "items_oss", "cmp399_other",
            "cmp399_despawn2", "cmp399_shard", "cmp399_bfcomp",
            "", "   ", "cmp412_b2p1_x",
        ]);
        v
    }

    #[test]
    fn parity_entity_query() {
        const OLD: &[&str] = &[
            "cmp410_eindexq", "cmp411_k4soa", "cmp411_eqsnap",
            "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq", "cmp420_colpush",
            "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense",
            "cmp451_senseins", "cmp453_diet", "cmp456_poi", "cmp421_brain",
            "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
            "cmp444_chunk5", "cmp450_chunk", "cmp456_chunkmono",
            "cmp456_chunkmono_p31snap",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_EQ), OLD.contains(&c), "M_EQ drift: {c}");
            assert_eq!(
                armed_flag(c, M_K4),
                c == "cmp411_k4soa",
                "M_K4 drift: {c}"
            );
            assert_eq!(
                armed_flag(c, M_EQSNAP),
                ["cmp411_eqsnap", "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq", "cmp421_brain"].contains(&c),
                "M_EQSNAP drift: {c}"
            );
            assert_eq!(
                armed_flag(c, M_EQSNAPV3),
                ["cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq"].contains(&c),
                "M_EQSNAPV3 drift: {c}"
            );
            assert_eq!(
                armed_flag(c, M_BRAIN2),
                c == "cmp422_brain2",
                "M_BRAIN2 drift: {c}"
            );
        }
    }

    #[test]
    fn parity_sense_arena_carrier() {
        const OLD: &[&str] = &[
            "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense",
            "cmp451_senseins", "cmp456_chunkmono", "cmp453_diet",
            "cmp450_chunk", "cmp434_chunkpl", "cmp435_chunk3",
            "cmp437_chunk4", "cmp444_chunk5", "cmp456_chunkmono_p31snap",
            "cmp456_poi",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_SENSEQ), OLD.contains(&c), "M_SENSEQ drift: {c}");
        }
    }

    #[test]
    fn parity_poi_plane() {
        const OLD: &[&str] = &[
            "cmp456_poi", "cmp409_multi", "cmp412_meganav", "cmp412_eqsnapv3",
            "cmp414_cvs", "cmp417_bq", "cmp420_colpush", "cmp421_brain",
            "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp438_sense",
            "cmp451_senseins", "cmp453_diet", "cmp452_mega",
            "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
            "cmp444_chunk5", "cmp450_chunk",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_POI), OLD.contains(&c), "M_POI drift: {c}");
        }
    }

    #[test]
    fn parity_sscan_sense_inside() {
        const OLD_SSCAN: &[&str] = &[
            "cmp406_sscan", "cmp409_multi", "cmp412_meganav", "cmp414_cvs",
            "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush", "cmp422_brain2",
            "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside",
            "cmp432_inside2", "cmp436_ins4", "cmp458_swar", "cmp457_paldelta",
            "cmp457_eqsnap2", "cmp438_sense", "cmp451_senseins",
            "cmp456_chunkmono", "cmp453_diet", "cmp450_chunk",
            "cmp456_chunkmono_p31snap", "cmp421_brain", "cmp434_chunkpl",
            "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
        ];
        const OLD_SENSEPLANE: &[&str] = &[
            "cmp438_sense", "cmp406_sscan", "cmp409_multi", "cmp412_meganav",
            "cmp412_eqsnapv3", "cmp414_cvs", "cmp417_bq", "cmp420_colpush",
            "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp451_senseins", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp456_chunkmono",
            "cmp453_diet", "cmp450_chunk", "cmp456_chunkmono_p31snap",
            "cmp456_poi",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_SSCAN), OLD_SSCAN.contains(&c), "M_SSCAN drift: {c}");
            assert_eq!(armed_flag(c, M_SENSEPLANE), OLD_SENSEPLANE.contains(&c), "M_SENSEPLANE drift: {c}");
            assert_eq!(
                armed_flag(c, M_INSIDE_BATCH),
                c == "cmp456_chunkmono_p31snap",
                "M_INSIDE_BATCH drift: {c}"
            );
            assert_eq!(
                armed_flag(c, M_PALDELTA),
                ["cmp457_paldelta", "cmp458_swar"].contains(&c),
                "M_PALDELTA drift: {c}"
            );
        }
    }

    #[test]
    fn parity_inside_snap() {
        const OLD_B2: &[&str] = &[
            "cmp432_inside2", "cmp430_inside", "cmp436_ins4", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp438_sense",
            "cmp451_senseins", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
            "cmp453_diet", "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
            "cmp444_chunk5", "cmp450_chunk", "cmp456_poi",
        ];
        const OLD_V4: &[&str] = &[
            "cmp436_ins4", "cmp451_senseins", "cmp458_swar", "cmp457_paldelta",
            "cmp457_eqsnap2", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
            "cmp453_diet", "cmp450_chunk", "cmp456_poi",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_INSIDE_SNAP), OLD_B2.contains(&c), "M_INSIDE_SNAP drift: {c}");
            assert_eq!(armed_flag(c, M_INSIDE_V4), OLD_V4.contains(&c), "M_INSIDE_V4 drift: {c}");
        }
    }

    #[test]
    fn parity_chunksched_queryplane() {
        const OLD_CHUNKSCHED: &[&str] = &[
            "cmp456_chunkmono", "cmp420_colpush", "cmp421_chunk",
            "cmp421_brain", "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
            "cmp451_senseins", "cmp453_diet", "cmp450_chunk", "cmp434_chunkpl",
            "cmp435_chunk3", "cmp437_chunk4", "cmp444_chunk5", "cmp452_mega",
            "cmp455_spawn", "cmp456_poi", "cmp466_poiun",
            "cmp456_chunkmono_p31snap",
        ];
        const OLD_QP: &[&str] = &[
            "cmp412_b2p1", "cmp415_mcomp", "cmp416_mcomp", "cmp417_bq",
            "cmp420_colpush", "cmp421_brain", "cmp422_brain2", "cmp423_brain3",
            "cmp424_mobfeed", "cmp430_inside", "cmp432_inside2",
            "cmp438_sense", "cmp451_senseins", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp466_poiun",
            "cmp456_chunkmono", "cmp456_chunkmono_p31snap", "cmp453_diet",
            "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
            "cmp444_chunk5", "cmp450_chunk", "cmp456_poi",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_CHUNKSCHED), OLD_CHUNKSCHED.contains(&c), "M_CHUNKSCHED drift: {c}");
            assert_eq!(armed_flag(c, M_QUERYPLANE), OLD_QP.contains(&c), "M_QUERYPLANE drift: {c}");
        }
    }

    #[test]
    fn parity_collide_soa_items() {
        const OLD_COLLIDE: &[&str] = &[
            "cmp401_collide", "cmp403_tickplane", "cmp405_stagtick",
            "cmp406_aibatch", "cmp406_sscan", "cmp409_multi",
            "cmp412_meganav", "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq",
            "cmp420_colpush", "cmp421_brain", "cmp422_brain2",
            "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside",
            "cmp432_inside2", "cmp436_ins4", "cmp458_swar", "cmp457_paldelta",
            "cmp457_eqsnap2", "cmp456_chunkmono", "cmp456_chunkmono_p31snap",
            "cmp451_senseins", "cmp453_diet", "cmp450_chunk", "cmp438_sense",
            "cmp434_chunkpl", "cmp435_chunk3", "cmp437_chunk4",
            "cmp444_chunk5", "cmp456_poi",
        ];
        const OLD_SOA: &[&str] = &[
            "cmp401_soa", "cmp402_comp", "cmp402_stagcomp", "cmp403_tickplane",
            "cmp405_stagtick", "cmp406_aibatch", "cmp409_multi",
            "cmp412_meganav", "cmp414_cvs", "cmp412_eqsnapv3", "cmp417_bq",
            "cmp420_colpush", "cmp421_brain", "cmp422_brain2",
            "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside",
            "cmp432_inside2", "cmp436_ins4", "cmp458_swar", "cmp457_paldelta",
            "cmp457_eqsnap2", "cmp466_poiun", "cmp456_chunkmono",
            "cmp456_chunkmono_p31snap", "cmp451_senseins", "cmp453_diet",
            "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl", "cmp435_chunk3",
            "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi", "cmp406_sscan",
            "cmp410_eindexq", "cmp411_k4soa", "cmp411_eqsnap",
        ];
        const OLD_ITEMS: &[&str] = &[
            "items_subsys2", "cmp402_comp", "cmp402_stagcomp",
            "cmp403_tickplane", "cmp405_stagtick", "cmp406_aibatch",
            "cmp409_multi", "cmp412_meganav", "cmp414_cvs", "cmp412_eqsnapv3",
            "cmp417_bq", "cmp420_colpush", "cmp421_brain", "cmp422_brain2",
            "cmp423_brain3", "cmp424_mobfeed", "cmp430_inside",
            "cmp432_inside2", "cmp436_ins4", "cmp458_swar", "cmp457_paldelta",
            "cmp457_eqsnap2", "cmp466_poiun", "cmp456_chunkmono",
            "cmp456_chunkmono_p31snap", "cmp451_senseins", "cmp453_diet",
            "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl", "cmp435_chunk3",
            "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi", "cmp406_sscan",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_COLLIDE), OLD_COLLIDE.contains(&c), "M_COLLIDE drift: {c}");
            assert_eq!(armed_flag(c, M_SOA), OLD_SOA.contains(&c), "M_SOA drift: {c}");
            // items: точные id + префикс-семья cmp399_ (starts_with на месте).
            let old_items = OLD_ITEMS.contains(&c) || c.starts_with("cmp399_");
            assert_eq!(
                c.starts_with("cmp399_") || armed_flag(c, M_ITEMS),
                old_items,
                "M_ITEMS drift: {c}"
            );
        }
        assert!(armed_flag("cmp399_bfcomp", M_ITEMS) == false);
        assert!("cmp399_bfcomp".starts_with("cmp399_"));
        assert!(armed_flag("cmp399_other", M_ITEMS) == false);
        assert!("cmp399_other".starts_with("cmp399_"));
    }

    #[test]
    fn parity_items_comp_truncated_master_state() {
        // Мастер-состояние (9bb43fe9): comp обрезан на cmp420_colpush —
        // хвост после `;` = мёртвый кложур. Пин именно этого состояния.
        const OLD_COMP: &[&str] = &[
            "cmp402_comp", "cmp402_stagcomp", "cmp403_tickplane",
            "cmp405_stagtick", "cmp406_aibatch", "cmp406_sscan",
            "cmp409_multi", "cmp412_meganav", "cmp414_cvs", "cmp412_eqsnapv3",
            "cmp417_bq", "cmp420_colpush",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_ITEMS_COMP), OLD_COMP.contains(&c), "M_ITEMS_COMP drift: {c}");
            assert_eq!(
                armed_flag(c, M_ITEMS_DESPAWN2),
                c == "cmp399_despawn2",
                "M_ITEMS_DESPAWN2 drift: {c}"
            );
        }
        // хвостовые id'ы НЕ входят в comp (мастер-семантика):
        assert!(!armed_flag("cmp456_chunkmono", M_ITEMS_COMP));
        assert!(!armed_flag("cmp421_brain", M_ITEMS_COMP));
        assert!(!armed_flag("cmp432_inside2", M_ITEMS_COMP));
    }

    #[test]
    fn parity_collide_marker() {
        const OLD: &[&str] = &[
            "cmp403_tickplane", "cmp405_stagtick", "cmp406_aibatch",
            "cmp406_sscan", "cmp409_multi", "cmp412_meganav", "cmp414_cvs",
            "cmp412_eqsnapv3", "cmp417_bq", "cmp420_colpush", "cmp421_brain",
            "cmp422_brain2", "cmp423_brain3", "cmp424_mobfeed",
            "cmp430_inside", "cmp432_inside2", "cmp436_ins4", "cmp458_swar",
            "cmp457_paldelta", "cmp457_eqsnap2", "cmp456_chunkmono",
            "cmp456_chunkmono_p31snap", "cmp451_senseins", "cmp453_diet",
            "cmp450_chunk", "cmp438_sense", "cmp434_chunkpl", "cmp435_chunk3",
            "cmp437_chunk4", "cmp444_chunk5", "cmp456_poi",
        ];
        for c in candidates() {
            assert_eq!(armed_flag(c, M_COLLIDE_MARKER), OLD.contains(&c), "M_COLLIDE_MARKER drift: {c}");
        }
    }
}
