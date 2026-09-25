//! REGION-IO SECTOR POOL + READ-AHEAD (ID-P23, TASK-459-61 — WILD закон-11,
//! тик-459; carrier-член reload/chunk-оси, инертно на soak-CPU).
//!
//! Идея (карточка RESEARCH-458-P.md ID-P23): пул 4KiB-секторных буферов
//! + read-ahead соседних чанков региона, батч-вызов на группу запросов;
//! java читает из пула, miss -> ванильный FileChannel. Прогноз +0.5-1пп
//! reload + GC-debt relief (см. RESEARCH-459-P23.md — механика/parity/риски).
//!
//! СОСТАВ ПОДСИСТЕМЫ (закон 6 — НЕ одиночная функция):
//!   1. РЕГИОН-ЗАГОЛОВОК: разбор 8KiB-шапки (1024 локаций BE32:
//!      3B offset-in-sectors + 1B sector-count; 1024 таймстампов) с
//!      валидацией границ (offset >= HEADER_SECTORS, 1 <= count <= 255,
//!      offset+count не выше хвоста региона).
//!   2. ПЛАН READ-AHEAD: 8-соседство чанка на сетке региона -> дедуп + сортированные
//!      секторные диапазоны [start..start+count) для батч-префетча.
//!   3. СЕКТОРНЫЙ ПУЛ: fixed-cap free-list `Box<[u8; 4096]>`; выдача =
//!      ПЕРЕДАЧА ВЛАДЕНИЯ (одна копия на выдачу, пул НЕ алиасится —
//!      карточка P23; внутренний буфер никогда не отдаётся по ссылке);
//!      возврат = переиспользование. Кап фиксирован (дефолт 4096 секторов
//!      = 16MB java-heap), overflow -> miss -> ваниль, эвикции НЕТ.
//!   4. LEN+CRC ВЕРИФИКАТОР: payload-инварианты формата (4B BE len,
//!      compression-байт, 4096B-паддинг) + CRC32 (crc32ieee) для selftest-окна
//!      (байты пула против ванильного FileChannel-референса; расхождение =
//!      DISARM навсегда). Байты НЕ мутируются — parity бит-в-байт.
//!
//! v1 = control-plane (этот модуль): НИКАКОГО fd/файла в нативе
//! (FileChannel в нативе опасен — карточка P23; все fd остаются на
//! java-стороне). JNI-бридж волны 2 (bulk: список чанков -> HIT/MISS +
//! сектора) регистрируется здесь же; descriptor-контракт в javadoc ниже.
//!
//! JNI-SURFACE (волна 2, зафиксировано; нативы инжектятся jni_table):
//!   Java: net.minecraft.world.level.chunk.storage.ChunkParseOps
//!         (dormant-хук regionSectorPoolGet уже добавлен — точка входа
//!         карточки; MISS-константа -> ванильный FileChannel в коллере).
//!   native long  regionioPlan(long regionKey, int[] chunkIndices)
//!         -> план (CSR: offsets/секторные диапазоны) или 0 = ERR.
//!   native int   regionioHandout(long planRef, int chunkIndex, byte[] dst)
//!         -> 1 HIT (dst заполнен, len+CRC проверены) / 0 MISS / -1 ERR;
//!         dst ВСЕГДА java-heap (копия на выдачу).
//!   native void  regionioRelease(long planRef)
//!         -> возврат секторов в пул (re-init = zero-copy).
//!   FAIL-CLOSED: любой ERR/капа/невалидный диапазон/stale-эпоха (mtime
//!   java-стороны) -> MISS -> ванильное чтение.
//!
//! NCDFE-канон: бридж-класс (волна 2) объявляет ZERO nested classes и
//! определяется EARLY в kernel loader (протокол round-3, run 35902792520:
//! каждое `$Nested` из констант-пула обязано попасть в define_class; тут
//! nested-ов нет по дизайну — класс один, хуки статические).
//!
//! ГЕЙТ: CRUSSTY_LEVER_FLAG STRICT eq "cmp459_p23" (пустой/чужой флаг =
//! ваниль бит-в-байт: модуль логирует dormant-маркер и не активируется).
//!
//! ARM markers (server stdout):
//!   "[crussty-plugin] cmp459_p23: dormant (lever_flag != cmp459_p23, vanilla region IO)"
//!   "[crussty-plugin] cmp459_p23: region header model + read-ahead plan + sector pool READY (v1 control-plane)"
//!   java (волна 2): "region_io: first pool HIT served"

/// 4KiB-сектор — константа формата (RegionFile.SECTOR_BYTES кернела; wiki.vg
/// Region File format: файл целиком кратен 4KiB).
pub const SECTOR_SIZE: usize = 4096;
/// Сектора заголовка: location table + timestamps (8KiB шапка).
pub const HEADER_SECTORS: u32 = 2;
/// Локаций в таблице (32x32 чанка).
pub const LOCATION_ENTRIES: usize = 1024;
/// Максимальный размер чанка в секторах (младший байт локации).
pub const MAX_CHUNK_SECTORS: u32 = 255;
/// Внешний чанк (.mcc) — v1 всегда miss -> ваниль.
pub const EXTERNAL_STREAM_FLAG: u8 = 128;
/// Кап пула в секторах (16MB java-heap) — константа v1 (карточка: капы ->
/// miss -> ваниль).
pub const POOL_CAP_SECTORS: usize = 4096;
/// Радиус read-ahead по чанк-сетке (8-соседство = 1).
pub const READ_AHEAD_RADIUS: i32 = 1;

pub const LEVER_ID: &str = "cmp459_p23";

/// compression-байты payload (wiki.vg Region File format; 2 = дефолт ванили).
pub const COMPRESSION_GZIP: u8 = 1;
pub const COMPRESSION_ZLIB: u8 = 2;
pub const COMPRESSION_NONE: u8 = 3;
pub const COMPRESSION_LZ4: u8 = 4;
pub const COMPRESSION_CUSTOM: u8 = 127;

/// Локация чанка в регионе (распакованная запись таблицы).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionLoc {
    /// Начальный сектор тела от НАЧАЛА ФАЙЛА (уже >= HEADER_SECTORS).
    pub offset_sectors: u32,
    /// Размер тела в секторах (1..=255).
    pub count_sectors: u32,
}

impl RegionLoc {
    /// Полуоткрытый диапазон секторов тела [start, end).
    pub fn sector_range(&self) -> std::ops::Range<u32> {
        self.offset_sectors..self.offset_sectors + self.count_sectors
    }
}

/// Ошибка разбора заголовка региона (fail-closed -> miss -> ваниль).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrHeader;

/// Модель 8KiB-шапки региона (не владеет файлом; вход = прочитанные
/// java-стороной 8192 байта).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionHeader {
    locations: [Option<RegionLoc>; LOCATION_ENTRIES],
}

impl RegionHeader {
    /// Разбор 8192B шапки: [0..4096) локации, [4096..8192) таймстампы.
    /// Формат бит-в-байт по wiki.vg Region File format (BE32: 3B offset +
    /// 1B count; пустая запись = все нули). Валидация: offset >=
    /// HEADER_SECTORS, 1 <= count <= MAX_CHUNK_SECTORS, overflow-контроль.
    pub fn parse(header8k: &[u8]) -> Result<RegionHeader, ErrHeader> {
        if header8k.len() < (HEADER_SECTORS as usize) * SECTOR_SIZE {
            return Err(ErrHeader);
        }
        let mut locations: [Option<RegionLoc>; LOCATION_ENTRIES] = [None; LOCATION_ENTRIES];
        for i in 0..LOCATION_ENTRIES {
            let b = &header8k[i * 4..i * 4 + 4];
            let raw = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            if raw == 0 {
                continue; // пустая локация
            }
            let offset = raw >> 8; // старшие 3 байта
            let count = (raw & 0xFF) as u32; // младший байт
            if offset < HEADER_SECTORS
                || count < 1
                || count > MAX_CHUNK_SECTORS
                || offset > u32::MAX - count
            {
                return Err(ErrHeader); // битый/недоверенный заголовок -> ваниль
            }
            locations[i] = Some(RegionLoc { offset_sectors: offset, count_sectors: count });
        }
        Ok(RegionHeader { locations })
    }

    /// Локация по чанк-индексу (index = (chunkX & 31) + 32 * (chunkZ & 31),
    /// wiki.vg: i = x + 32*z).
    pub fn location(&self, chunk_index: usize) -> Option<RegionLoc> {
        self.locations.get(chunk_index).copied().flatten()
    }

    /// Индекс чанка в регионе из мировых координат (арефметический сдвиг
    /// спецификации: x & 31, z & 31).
    pub fn chunk_index(chunk_x: i32, chunk_z: i32) -> usize {
        ((chunk_x & 31) + 32 * (chunk_z & 31)) as usize
    }

    /// ПЛАН READ-AHEAD: 8-соседство (radius 1) на чанк-сетке региона вокруг
    /// (chunk_x, chunk_z) -> отсортированные без-пересечений-дедуп
    /// секторные диапазоны. Собственный чанк тоже включён (сам запрос).
    /// Возврат: пустой вектор = план не нужен (все соседи вне региона/пусты).
    pub fn read_ahead_plan(
        &self,
        chunk_x: i32,
        chunk_z: i32,
        radius: i32,
    ) -> Vec<std::ops::Range<u32>> {
        let mut secs: Vec<u32> = Vec::with_capacity(16);
        for dz in -radius..=radius {
            for dx in -radius..=radius {
                let nx = chunk_x + dx;
                let nz = chunk_z + dz;
                if let Some(loc) = self.location(Self::chunk_index(nx, nz)) {
                    secs.extend(loc.sector_range());
                }
            }
        }
        if secs.is_empty() {
            return Vec::new();
        }
        secs.sort_unstable();
        secs.dedup();
        // сплёт секторов в непрерывные диапазоны
        let mut plan: Vec<std::ops::Range<u32>> = Vec::with_capacity(8);
        let mut it = secs.iter().copied();
        let mut start = it.next().expect("non-empty");
        let mut prev = start;
        for s in it {
            if s != prev + 1 {
                plan.push(start..prev + 1);
                start = s;
            }
            prev = s;
        }
        plan.push(start..prev + 1);
        plan
    }
}

/// Выданный сектор: владение буфером передано вызывающему (пул НЕ алиасится
/// — одна копия на выдачу, карточка P23). Возврат — обратно в пул через
/// `SectorPool::release` (re-init под следующий сектор).
pub struct Sector {
    buf: Box<[u8; SECTOR_SIZE]>,
}

impl Sector {
    /// Ссылка на буфер (ТОЛЬКО читатель данных; байты пула не мутируются).
    pub fn bytes(&self) -> &[u8; SECTOR_SIZE] {
        &self.buf
    }
    /// Заполнение сектора данными (одна запись перед выдачей; затем буфер
    /// читается как неизменяемый).
    pub fn fill(&mut self, data: &[u8]) -> Result<(), ErrHeader> {
        if data.len() > SECTOR_SIZE {
            return Err(ErrHeader);
        }
        self.buf[..data.len()].copy_from_slice(data);
        if data.len() < SECTOR_SIZE {
            self.buf[data.len()..].fill(0); // паддинг формата: файл кратен 4KiB
        }
        Ok(())
    }
}

/// Пул 4KiB-секторных буферов: fixed-cap free-list, overflow = miss ->
/// ваниль (никакой эвикции/роста). Потокобезопасность — волна 2 (JNI-план
/// захватывает пул под sscan/region-очередями); v1 = однопоточный контракт.
pub struct SectorPool {
    free: Vec<Box<[u8; SECTOR_SIZE]>>,
    cap: usize,
    live: usize,
}

impl SectorPool {
    /// Пул на `cap` секторов (дефолт POOL_CAP_SECTORS = 16MB java-heap).
    pub fn new(cap: usize) -> SectorPool {
        SectorPool { free: Vec::with_capacity(cap.min(64)), cap, live: 0 }
    }

    /// Выдача сектора: передача владения (no-alias). None = пул исчерпан ->
    /// miss -> ваниль.
    pub fn acquire(&mut self) -> Option<Sector> {
        if let Some(buf) = self.free.pop() {
            self.live += 1;
            return Some(Sector { buf });
        }
        if self.live + self.free.len() >= self.cap {
            return None; // кап -> miss -> ваниль
        }
        self.live += 1;
        Some(Sector { buf: Box::new([0u8; SECTOR_SIZE]) })
    }

    /// Возврат сектора в пул (буфер переиспользуется; содержимое затирается
    /// при следующем fill).
    pub fn release(&mut self, sector: Sector) {
        if self.free.len() < self.cap {
            self.free.push(sector.buf);
        }
        self.live = self.live.saturating_sub(1);
    }

    /// Текущая занятость (метрика гейта волны 2: hit-rate).
    pub fn live(&self) -> usize {
        self.live
    }
}

/// CRC32 (crc32ieee) — selftest-сверка байтов пула с ванильным референсом
/// (parity бит-в-байт; расхождение = DISARM навсегда).
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Payload-инварианты формата над секторным диапазоном тела:
///   [0..4)  length BE — длина ОСТАЛЬНОГО тела (паддинг НЕ включён),
///   [4]     compression (1/2/3/4/127; >=128 = external .mcc -> miss),
///   4 + length <= секторов * 4096 (иначе битый диапазон -> miss).
/// Возврат: Ok(payload_len) — верифицированная длина сжатого тела.
/// CRC32 тела для selftest-окна: `crc32(&body[5..4+payload_len])`.
/// Байты НЕ мутируются (пул только читает) — parity бит-в-байт.
pub fn verify_payload(body_sectors: &[u8]) -> Result<usize, ErrHeader> {
    if body_sectors.len() < 5 || body_sectors.len() % SECTOR_SIZE != 0 {
        return Err(ErrHeader);
    }
    let len = u32::from_be_bytes([
        body_sectors[0],
        body_sectors[1],
        body_sectors[2],
        body_sectors[3],
    ]) as usize;
    let compression = body_sectors[4];
    match compression {
        COMPRESSION_GZIP | COMPRESSION_ZLIB | COMPRESSION_NONE | COMPRESSION_LZ4
        | COMPRESSION_CUSTOM => {}
        c if c >= EXTERNAL_STREAM_FLAG => return Err(ErrHeader), // .mcc -> ваниль
        _ => return Err(ErrHeader),
    }
    if len < 1 || len > body_sectors.len().saturating_sub(4) {
        return Err(ErrHeader);
    }
    Ok(len)
}

/// ГЕЙТ: CRUSSTY_LEVER_FLAG STRICT eq "cmp459_p23" (пустой/чужой флаг =
/// ваниль бит-в-байт).
pub fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG").map(|v| v.trim() == LEVER_ID).unwrap_or(false)
}

/// Регистрация (idempotent; вызывается из cplugin_init_impl). v1
/// control-plane: только гейт + маркеры; никаких хуков/JNI/файлов.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] {LEVER_ID}: dormant (lever_flag != {LEVER_ID}, vanilla region IO)"
        );
        return;
    }
    // v1 self-checks (fail-loud на ошибке контракта — безопасно: до этого
    // места доходит только включённый гейт; пул ещё не активен).
    let mut pool = SectorPool::new(POOL_CAP_SECTORS);
    assert_eq!(pool.acquire().map(|_| 1), Some(1), "pool acquire must serve");
    assert_eq!(
        crc32(b"123456789"),
        0xCBF4_3926,
        "crc32ieee check vector (selftest-контракт parity)"
    );
    eprintln!(
        "[crussty-plugin] {LEVER_ID}: region header model + read-ahead plan + sector pool READY (v1 control-plane)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc_raw(offset: u32, count: u32) -> [u8; 4] {
        ((offset << 8) | (count & 0xFF)).to_be_bytes()
    }

    #[test]
    fn header_parse_and_lookup() {
        let mut h = vec![0u8; 2 * SECTOR_SIZE];
        // chunk (0,0) -> index 0: offset 3, count 2
        h[0..4].copy_from_slice(&loc_raw(3, 2));
        // chunk (-1,-1) -> локальные (31,31) -> index 31 + 32*31 = 1023: offset 10, count 1
        let i1023 = 1023 * 4;
        h[i1023..i1023 + 4].copy_from_slice(&loc_raw(10, 1));
        let hdr = RegionHeader::parse(&h).expect("parse");
        let l0 = hdr.location(0).expect("loc0");
        assert_eq!((l0.offset_sectors, l0.count_sectors), (3, 2));
        assert_eq!(l0.sector_range(), 3..5);
        let l1023 = hdr.location(1023).expect("loc1023");
        assert_eq!((l1023.offset_sectors, l1023.count_sectors), (10, 1));
        assert!(hdr.location(1).is_none());
        assert_eq!(RegionHeader::chunk_index(-1, -1), 1023);
    }

    #[test]
    fn header_rejects_bad_entries() {
        // offset < HEADER_SECTORS -> Err (fail-closed -> ваниль)
        let mut h = vec![0u8; 2 * SECTOR_SIZE];
        h[0..4].copy_from_slice(&loc_raw(1, 2));
        assert_eq!(RegionHeader::parse(&h), Err(ErrHeader));
        // count = 0 при ненулевом offset -> Err
        let mut h2 = vec![0u8; 2 * SECTOR_SIZE];
        h2[0..4].copy_from_slice(&loc_raw(3, 0));
        assert_eq!(RegionHeader::parse(&h2), Err(ErrHeader));
        // короткая шапка -> Err
        assert_eq!(RegionHeader::parse(&[0u8; 100]), Err(ErrHeader));
    }

    #[test]
    fn read_ahead_plan_covers_neighbors_dedup_sorted() {
        let mut h = vec![0u8; 2 * SECTOR_SIZE];
        // центр (5,5) idx=165: 10..12; сосед (6,5) idx=166: 10..11 (пересекается — дедуп);
        // сосед (4,5) idx=164: 20..22
        for (idx, o, c) in [(165usize, 10u32, 2u32), (166, 10, 1), (164, 20, 2)] {
            let i = idx * 4;
            h[i..i + 4].copy_from_slice(&loc_raw(o, c));
        }
        let hdr = RegionHeader::parse(&h).expect("parse");
        let plan = hdr.read_ahead_plan(5, 5, READ_AHEAD_RADIUS);
        // непрерывный кусок 10..12 + кусок 20..22
        assert_eq!(plan, vec![10..12, 20..22], "план = сортированные без-дуп диапазоны");
    }

    #[test]
    fn pool_no_alias_handout_is_ownership() {
        let mut pool = SectorPool::new(4);
        let mut s1 = pool.acquire().expect("acquire 1");
        s1.fill(&[7u8; 100]).expect("fill");
        assert_eq!(s1.bytes()[0], 7);
        assert_eq!(s1.bytes()[SECTOR_SIZE - 1], 0); // паддинг затёрт
        let mut s2 = pool.acquire().expect("acquire 2");
        s2.fill(&[9u8; 10]).expect("fill");
        assert_eq!(s2.bytes()[0], 9);
        assert_ne!(s2.bytes()[0], s1.bytes()[0], "сектора независимы (no-alias)");
        pool.release(s1);
        pool.release(s2);
        assert_eq!(pool.live(), 0);
        // переиспользование после возврата
        let s3 = pool.acquire().expect("acquire 3");
        assert_eq!(pool.live(), 1);
        drop(s3);
        let s4 = pool.acquire().expect("acquire 4");
        pool.release(s4);
        let mut pool_cap = SectorPool::new(1);
        let _a = pool_cap.acquire().expect("first");
        assert!(pool_cap.acquire().is_none(), "кап исчерпан -> miss -> ваниль");
    }

    #[test]
    fn payload_verify_len_compression() {
        // len=4, zlib, тело 4 байта, паддинг до 4096
        let mut body = vec![0u8; SECTOR_SIZE];
        body[0..4].copy_from_slice(&4u32.to_be_bytes());
        body[4] = COMPRESSION_ZLIB;
        body[5..9].copy_from_slice(&[1, 2, 3, 4]);
        assert_eq!(verify_payload(&body), Ok(4));
        assert_eq!(crc32(&body[5..9]), crc32(&[1, 2, 3, 4]));
        // external (.mcc) -> Err -> miss -> ваниль
        let mut ext = body.clone();
        ext[4] = 128;
        assert_eq!(verify_payload(&ext), Err(ErrHeader));
        // битая длина -> Err
        let mut bad = body.clone();
        bad[0..4].copy_from_slice(&u32::MAX.to_be_bytes());
        assert_eq!(verify_payload(&bad), Err(ErrHeader));
    }
}
