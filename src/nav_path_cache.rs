//! NAV-PATH-CACHE (TASK-459-66, ID-P41, law-11 — NodeEvaluator neighbor
//! cache, lithium nether-pathfinding style). Lever `cmp459_p41`, STRICT eq,
//! OFF до оракула 10k путей.
//!
//! Lane: nav_ai остаток 2.75-3.2% wall (BOTTLENECK-459). Vanilla
//! WalkNodeEvaluator.getNeighbors re-traverses block data на каждого соседа
//! при каждом раскрытии A*-узла; повторные запросы узла (внутри пути +
//! между ретаргетами) идентичны, пока chunk-ревизия не менялась.
//!
//! Дизайн (RESEARCH-459-P41.md):
//!   posKey(i64, vanilla-BlockPos.asLong layout) -> neighborMask(u8, бит i =
//!   направление i возможно-проходимо), флет-таблица с linear probe,
//!   инвалидация по chunk-ревизии (secKey(x>>4,y>>4,z>>4) -> rev u32).
//!   HIT -> java пропускает re-traversal; MISS -> ваниль бит-в-байт.
//!
//! Паритет: маска = SUPERSET соседей (лишний кандидат отсечётся ванильным
//! accept-тестом findAcceptedNode; потерянный сосед сломал бы путь — поэтому
//! вставка маски только после полной ванильной выкладки, ни одна выкладка не
//! заменяется кэшем). Порядок обхода направлений при miss = ванильный.
//! STRICT-off: любой флаг кроме `cmp459_p41` = класс не определяется,
//! ретаргет не компонуется, таблица мертва — ваниль по построению.
//!
//! NCDFE-канон: define-before-arm (класс определяется в kernel loader до
//! латча ENABLED; selfTest не достигается через неразрешённые символы — JVM
//! кэширует NCDFE per constant-pool entry навсегда, root-cause cv3-1
//! 35712182885); fail-closed ERR -> one-shot disarm -> ваниль-ветка java.
//!
//! iter-2 wiring (за пределами scaffold): include_bytes! скомпилированного
//! PathOps.class, register_native в region_threads compose-гейте, hook
//! WalkNodeEvaluator.getNeighbors. Здесь — движок + JNI-контракт + тесты.

/// Lever: STRICT eq, только собственный флаг (никаких OR-семейств — соседние
/// nav-носители neighbor-маску оракулом не подтверждали).
pub const P41_LEVER: &str = "cmp459_p41";

/// Класс java-стороны таблицы (пакет NodeEvaluator — Mojang mappings
/// net.minecraft.world.level.pathfinder).
pub const P41_CLASS: &str = "net/minecraft/world/level/pathfinder/PathOps";

/// Bulk-JNI сигнатура iter-2:
/// navNeighborBatch(n, keys jlong[n], revs jint[n], out jint[n]) -> hits.
/// out[i] = mask (0..=255) при хите, -1 при MISS (ваниль-ветка java).
pub const P41_BATCH_SIG: &str = "(II[I[I)I";

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// STRICT eq gate (пустой/чужой флаг = false = ваниль).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == P41_LEVER)
        .unwrap_or(false)
}

/// posKey: layout vanilla BlockPos.asLong (x26 z26 y12) — полное покрытие
/// координат без коллизий Node.createHash (канон-комментарий в nav_pool):
/// ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF).
/// Ни один posKey не равен KEY_SENTINEL (i64::MIN): старший бит всегда 0.
#[inline]
pub fn pos_key(x: i32, y: i32, z: i32) -> i64 {
    let xi = (x as i64) & 0x3FF_FFFF;
    let zi = (z as i64) & 0x3FF_FFFF;
    let yi = (y as i64) & 0xFFF;
    (xi << 38) | (zi << 12) | yi
}

/// secKey: чанк-секция 16×16×16 → (x>>4 | z>>4 | y>>4). rev = chunk-ревизия
/// (счётчик мутаций секции); mismatch = miss + освобождение слота.
#[inline]
pub fn section_key(pos: i64) -> i64 {
    let x = decode_x(pos);
    let y = decode_y(pos);
    let z = decode_z(pos);
    let sx = ((x >> 4) as i64) & 0x3F_FFFF;
    let sz = ((z >> 4) as i64) & 0x3F_FFFF;
    let sy = ((y >> 4) as i64) & 0x3FF;
    (sx << 32) | (sz << 10) | sy
}

#[inline]
pub fn decode_x(k: i64) -> i32 {
    let v = (k >> 38) & 0x3FF_FFFF;
    if v & 0x200_0000 != 0 { (v - 0x400_0000) as i32 } else { v as i32 }
}
#[inline]
pub fn decode_y(k: i64) -> i32 {
    let v = k & 0xFFF;
    if v & 0x800 != 0 { (v - 0x1000) as i32 } else { v as i32 }
}
#[inline]
pub fn decode_z(k: i64) -> i32 {
    let v = (k >> 12) & 0x3FF_FFFF;
    if v & 0x200_0000 != 0 { (v - 0x400_0000) as i32 } else { v as i32 }
}

pub const TABLE_CAP: usize = 1 << 17; // 131072 слотов, power-of-two
const KEY_SENTINEL: i64 = i64::MIN; // posKey/sectionKey его не порождают

/// Флет-таблица posKey → (rev, mask). Linear probe, cap TABLE_CAP;
/// заполнение под крышку = miss-фоллбэк (bounded память, ваниль-паритет
/// на overflow-пути — дисциплина nav_pool MAP_CAP).
pub struct NeighborCache {
    keys: Vec<i64>,
    vals: Vec<u64>, // (rev << 8) | mask
    section_keys: Vec<i64>,
    section_revs: Vec<u32>,
    pub overflow_misses: u64,
    pub lookups: u64,
    pub hits: u64,
}

impl NeighborCache {
    pub fn new() -> Self {
        NeighborCache {
            keys: vec![KEY_SENTINEL; TABLE_CAP],
            vals: vec![0; TABLE_CAP],
            section_keys: vec![KEY_SENTINEL; 1 << 14],
            section_revs: vec![0; 1 << 14],
            overflow_misses: 0,
            lookups: 0,
            hits: 0,
        }
    }

    fn probe(&self, k: i64) -> Option<usize> {
        debug_assert!(k != KEY_SENTINEL);
        let mask = (TABLE_CAP - 1) as u64;
        let mut i = (Self::hash(k) & mask) as usize;
        for _ in 0..TABLE_CAP {
            let slot = self.keys[i];
            if slot == k {
                return Some(i);
            }
            if slot == KEY_SENTINEL {
                return None;
            }
            i = (i + 1) & (TABLE_CAP - 1);
        }
        None
    }

    fn hash(k: i64) -> u64 {
        // splitmix64-финализация (низкие биты напрямую — плохо для probe).
        let mut z = (k as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Chunk-ревизия секции (0 = ещё не видели; регистрируется при первом
    /// обращении).
    pub fn section_rev(&mut self, sec: i64) -> u32 {
        debug_assert!(sec != KEY_SENTINEL);
        let smask = (self.section_keys.len() - 1) as u64;
        let mut i = (Self::hash(sec) & smask) as usize;
        loop {
            let slot = self.section_keys[i];
            if slot == sec {
                return self.section_revs[i];
            }
            if slot == KEY_SENTINEL {
                self.section_keys[i] = sec;
                self.section_revs[i] = 0;
                return 0;
            }
            i = (i + 1) & (smask as usize);
        }
    }

    /// Bump chunk-ревизии секции: последующие lookup её узлов = miss
    /// (lazy expiry — слот маски высвобождается при первом же lookup).
    pub fn invalidate_section(&mut self, sec: i64, rev: u32) {
        debug_assert!(sec != KEY_SENTINEL);
        let smask = (self.section_keys.len() - 1) as u64;
        let mut i = (Self::hash(sec) & smask) as usize;
        loop {
            let slot = self.section_keys[i];
            if slot == sec {
                self.section_revs[i] = rev;
                return;
            }
            if slot == KEY_SENTINEL {
                self.section_keys[i] = sec;
                self.section_revs[i] = rev;
                return;
            }
            i = (i + 1) & (smask as usize);
        }
    }

    /// LOOKUP: Some(mask) при хите с совпадающей ревизией, None = MISS
    /// (ваниль). mismatch ревизии высвобождает слот.
    pub fn lookup(&mut self, pos: i64, rev: u32) -> Option<u8> {
        debug_assert!(pos != KEY_SENTINEL);
        self.lookups += 1;
        let idx = self.probe(pos)?;
        let v = self.vals[idx];
        if (v >> 8) as u32 != rev {
            // stale: секция мутировала — слот мёртв (superset-инвариант
            // обслуживается инвалидацией, тут только высвобождение).
            self.keys[idx] = KEY_SENTINEL;
            self.vals[idx] = 0;
            return None;
        }
        self.hits += 1;
        Some((v & 0xFF) as u8)
    }

    /// STORE: маска вставляется только после полной ванильной выкладки
    /// (кэш не заменяет выкладку — только re-traversal).
    pub fn store(&mut self, pos: i64, rev: u32, mask: u8) -> bool {
        debug_assert!(pos != KEY_SENTINEL);
        if let Some(idx) = self.probe(pos) {
            self.vals[idx] = ((rev as u64) << 8) | mask as u64;
            return true;
        }
        let mask_tbl = (TABLE_CAP - 1) as u64;
        let mut i = (Self::hash(pos) & mask_tbl) as usize;
        for _ in 0..TABLE_CAP {
            if self.keys[i] == KEY_SENTINEL {
                self.keys[i] = pos;
                self.vals[i] = ((rev as u64) << 8) | mask as u64;
                return true;
            }
            i = (i + 1) & (TABLE_CAP - 1);
        }
        // Таблица под крышку: miss-фоллбэк (bounded память, nav_pool
        // MAP_CAP-дисциплина).
        self.overflow_misses += 1;
        false
    }

    pub fn clear(&mut self) {
        self.keys.iter_mut().for_each(|k| *k = KEY_SENTINEL);
        self.vals.iter_mut().for_each(|v| *v = 0);
    }
}

impl Default for NeighborCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Bulk-JNI контракт iter-2 (сигнатура P41_BATCH_SIG). Реализация появится
/// вместе с include_bytes!/register_native (region_threads compose-гейт);
/// здесь фиксируется ERR-ladder: rc<0 = ERR (STRUCT=-1, RANGE=-2), rc>=0 =
/// число хитов; out[i] = mask | -1(MISS).
pub mod contract {
    use super::{NeighborCache, ERR_RANGE, ERR_STRUCT};

    pub fn batch_out(
        n: usize,
        keys: &[i64],
        revs: &[u32],
        out: &mut [i32],
        c: &mut NeighborCache,
    ) -> Result<usize, i32> {
        if keys.len() < n || revs.len() < n || out.len() < n {
            return Err(ERR_STRUCT);
        }
        if n > i32::MAX as usize / 4 {
            return Err(ERR_RANGE);
        }
        let mut hits = 0usize;
        for i in 0..n {
            match c.lookup(keys[i], revs[i]) {
                Some(m) => {
                    out[i] = m as i32;
                    hits += 1;
                }
                None => out[i] = -1,
            }
        }
        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_key_roundtrip_full_coverage() {
        // range покрытия layout: x/z ∈ [-2^25, 2^25-1], y ∈ [-2048, 2047]
        // (y12 sign-бит 0x800 → диапазон ±2^11; докручивается sign-extension
        // для MC-высот ±512 с запасом).
        for &(x, y, z) in &[
            (0, 0, 0),
            (-1, -1, -1),
            (33_554_431, 511, 33_554_431),
            (-33_554_432, -512, -33_554_432),
            (123_456, 319, -65_432),
        ] {
            let k = pos_key(x, y, z);
            assert_eq!((decode_x(k), decode_y(k), decode_z(k)), (x, y, z));
            assert_ne!(k, KEY_SENTINEL);
        }
    }

    #[test]
    fn section_key_groups_by_16() {
        let a = section_key(pos_key(5, 64, 9));
        let b = section_key(pos_key(15, 79, 15));
        let c = section_key(pos_key(16, 64, 9));
        let d = section_key(pos_key(5, 80, 9));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn hit_miss_and_stale_rev() {
        let mut c = NeighborCache::new();
        let p = pos_key(10, 64, 10);
        // miss на пустой таблице
        assert_eq!(c.lookup(p, 7), None);
        // store+hit
        assert!(c.store(p, 7, 0b1010_0011));
        assert_eq!(c.lookup(p, 7), Some(0b1010_0011));
        assert_eq!(c.lookups, 2);
        assert_eq!(c.hits, 1);
        // bump ревизии секции -> miss, слот высвобожден
        let sec = section_key(p);
        c.invalidate_section(sec, 8);
        assert_eq!(c.lookup(p, 8), None);
        assert_eq!(c.lookup(p, 7), None); // старая ревизия не воскрешает
        // store под новую ревизию -> hit
        assert!(c.store(p, 8, 0xFF));
        assert_eq!(c.lookup(p, 8), Some(0xFF));
    }

    #[test]
    fn superset_mask_byte_roundtrip() {
        // маска u8 несёт 8 направлений (4 латерали + 4 диагонали); полнота
        // 0..=255 сохраняется (маска 0 = «соседей нет» — валидный хит).
        let mut c = NeighborCache::new();
        let p = pos_key(-7, 33, 41);
        assert!(c.store(p, 3, 0));
        assert_eq!(c.lookup(p, 3), Some(0));
        assert!(c.store(p, 3, 0xFF));
        assert_eq!(c.lookup(p, 3), Some(0xFF));
    }

    #[test]
    fn table_cap_overflow_is_miss_not_panic() {
        let mut c = NeighborCache::new();
        let n = TABLE_CAP as i64;
        for i in 0..(n + 16) {
            let p = pos_key((i % 2_000_003) as i32, 64, (i / 2_000_003) as i32);
            c.store(p, 1, (i & 0xFF) as u8);
        }
        assert!(c.overflow_misses > 0);
        // lookup после overflow не паникует; ранний ключ жив.
        assert_eq!(c.lookup(pos_key(0, 64, 0), 1), Some(0));
    }

    #[test]
    fn contract_err_ladder_and_miss_semantics() {
        let mut c = NeighborCache::new();
        let keys = [pos_key(1, 2, 3), pos_key(4, 5, 6)];
        let revs = [1u32, 1];
        let mut out = [-1i32; 2];
        // оба miss
        assert_eq!(contract::batch_out(2, &keys, &revs, &mut out, &mut c), Ok(0));
        assert_eq!(out, [-1, -1]);
        // один hit
        c.store(keys[0], 1, 0b1);
        assert_eq!(contract::batch_out(2, &keys, &revs, &mut out, &mut c), Ok(1));
        assert_eq!(out, [1, -1]);
        // ERR_STRUCT: out короче n
        let mut short = [-1i32; 1];
        assert_eq!(
            contract::batch_out(2, &keys, &revs, &mut short, &mut c),
            Err(ERR_STRUCT)
        );
    }

    #[test]
    fn gate_is_strict_eq_single_lever() {
        // STRICT-off до оракула: только cmp459_p41; пустой/чужой = ваниль.
        assert_eq!(P41_LEVER, "cmp459_p41");
        assert_ne!("", P41_LEVER);
        assert_ne!("cmp405_navplane", P41_LEVER);
    }
}
