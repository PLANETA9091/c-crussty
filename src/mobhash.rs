//! MOBHASH (TASK-401-A — vector spatial-hash neighbors; lever cmp401_mobhash).
//!
//! Секционно-хешированный пространственный индекс мобов для мобового
//! push-лейна (`LivingEntity.pushEntities` → `Level.getPushableEntities`):
//! замена перечисления кандидатов на запрос плоского 64-шардного
//! seqlock-грида 1.0-ячеек. Точные ванильные фильтры (level, AABB.intersects,
//! EntitySelector.pushableBy, self) выполняются java-стороной
//! (mobhash/net/minecraft/world/entity/MobHashOps.java) на каждом кандидате.
//!
//! ТЕХНИКА (своя; паттерн-референс — items_index.rs flat
//! `cellKey -> chain-head` open-addressing + intrusive per-id next/cell,
//! fail-closed ERR_*): шард = mix64(cell_key) & 63 — ХЕШ-шардирование
//! (пространственное деградировало бы в одну блокировку на плотной бенч-
//! популяции в боксах). Читатель — seqlock per-shard: v1(Acquire) → скан
//! ячейки → v2(Acquire); v1==v2 и чётная → данные согласованы, иначе
//! cell-local retry (≤4096 → ERR_RANGE per-call vanilla). Писатель — per-shard
//! spinlock; версия бампается нечёт→данные→чёт вокруг мутации СВОЕГО шарда;
//! кросс-шардные upsert'ы берут два спинлока в возрастающем порядке индексов
//! (deadlock-free по построению). Отличие от cmp399_shard-семейства: per-shard
//! версия+спинлок (параллельные писатели разных шардов) вместо per-cell версий
//! + глобального writer-mutex.
//!
//! ПАМЯТЬ ФИКСИРОВАННАЯ (без realloc ⇒ UAF невозможен по построению): per-shard
//! inline-таблицы `[i64; 16384] keys` + `[i32; 16384] head` (0 = свободно /
//! пустая цепочка), id-пространство next/cell по 1<<20. Переполнение →
//! ERR_STRUCT → java-бридж дизармит рычаг навсегда (ванильный путь).
//!
//! ТОЧНОСТЬ ОКНА: запрос-бокс → ячейки floor(min)−1 .. floor(max)+1 (pad ±1).
//! Корректность pad: кандидат с bb∩query≠∅ имеет центр в СВОЕМ bb, а его
//! half-extents ≤ r_eff ≤ 1.0 (radius-гейт java-стороны) ⇒ по каждой оси центр
//! не дальше 1.0 от query-бокса ⇒ ячейка центра всегда в окне; floor(x±1.0)
//! точен. Надмножество отфильтровывается точными ванильными тестами java.
//!
//! СВЕЖЕСТЬ: self-upsert-at-query (бридж апсертит SELF end-of-move позицией в
//! момент запроса; pushEntities в aiStep ПОСЛЕ travel/move — javap offset 850).
//! Взаимная свежесть в однопроходном тике: A спрашивает после тика B — B уже
//! апсертнулся; до тика B — B ещё не двигался (позиция = end-of-(T-1)).
//! Кросс-шардный upsert оставляет окно ~100нс, в котором id отсутствует в обеих
//! цепочках — потенциальный скип одного пуш-кандидата на тик (невидимая,
//! задокументированная в RESEARCH-A §2 дельта).
//!
//! ПАРИТЕТ: поведение неотличимо на бенч-популяции (150k мобов в боксах);
//! порядок кандидатов = порядок цепочек ячеек (документированная дельта класса
//! items_subsys2). Юниверс = сущности, выполняющие pushEntities (LivingEntity);
//! не-living (items/боаты) не входят в индекс — в бенч-структуре отсутствуют,
//! в живом мире = задокументированная дельта (RESEARCH-A §2).
//!
//! SAFETY-КОНТРАКТ UnsafeCell-статиков: все обращения — сырые указатели через
//! `(*ptr)[i]`; мутации только под спинлоком+версией шарда-владельца, чтения —
//! под seqlock-версией. Никаких realloc/смена адресов никогда (inline-массивы).

use jvmti_bindings::jni;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Константы ёмкостей (фиксированная память)
// ---------------------------------------------------------------------------

const SHARDS: usize = 64;
const SHARD_SLOTS: usize = 16384; // ячеек на шард (150k мобов ⇒ ~2.3k/шард avg)
const ID_CAP: usize = 1 << 20; // максимум id (мобы бенча ~150k)
const MAX_RETRIES: u32 = 4096; // seqlock-ретраи ячейки → ERR_RANGE
const MAX_WINDOW: i32 = 8; // макс. ширина окна ячеек по оси (pad ±1 включён)

/// Результат-коды нативов: >=0 ok/количество; -1 ERR_STRUCT (дизарм);
/// -2 ERR_RANGE (per-call vanilla); -(out_cap) overflow (grow + retry).
const ERR_STRUCT: i32 = -1;
const ERR_RANGE: i32 = -2;

// ---------------------------------------------------------------------------
// Шард: inline-таблицы + seqlock
// ---------------------------------------------------------------------------

#[repr(C, align(64))]
struct Shard {
    /// Seqlock-версия шарда: чётная = покой, нечётная = мутация.
    ver: AtomicU64,
    /// Writer-spinlock шарда (мутации одного шарда сериализуются).
    lock: AtomicBool,
    /// Занятые слоты key-таблицы (учёт загрузки).
    used: AtomicUsize,
    /// Open-addressed ключи ячеек; 0 = свободный слот (реальные ключи != 0).
    keys: UnsafeCell<[i64; SHARD_SLOTS]>,
    /// Голова цепочки ячейки: id+1 (0 = пусто).
    head: UnsafeCell<[i32; SHARD_SLOTS]>,
}
unsafe impl Sync for Shard {}

const SHARD0: Shard = Shard {
    ver: AtomicU64::new(0),
    lock: AtomicBool::new(false),
    used: AtomicUsize::new(0),
    keys: UnsafeCell::new([0; SHARD_SLOTS]),
    head: UnsafeCell::new([0; SHARD_SLOTS]),
};

static SHARD_ARR: [Shard; SHARDS] = [SHARD0; SHARDS];

/// Id-пространство: intrusive-звенья цепочек (next) и текущая ячейка (cell).
/// Фиксировано навсегда (никогда не realloc — читатели ходят по цепочкам без
/// блокировок под защитой seqlock-версии шарда-владельца).
struct Ids {
    next: UnsafeCell<[i32; ID_CAP]>, // следующий id+1 в той же ячейке (0 = конец)
    cell: UnsafeCell<[i64; ID_CAP]>, // текущий ключ ячейки (0 = вне индекса)
}
unsafe impl Sync for Ids {}

static IDS: Ids = Ids {
    next: UnsafeCell::new([0; ID_CAP]),
    cell: UnsafeCell::new([0; ID_CAP]),
};

// ---------------------------------------------------------------------------
// Ключи/шардирование
// ---------------------------------------------------------------------------

#[inline]
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Ключ ячейки: 21-битные симметричные поля cx/cy/cz, смешанные с lid.
/// Переполнение полей (|coord| > 1M) безвредно: ключ остаётся хешем,
/// кандидаты фильтруются точно java-стороной.
#[inline]
fn cell_key(lid: i32, cx: i32, cy: i32, cz: i32) -> i64 {
    const OFF: i64 = 1 << 20;
    const MSK: i64 = (1 << 21) - 1;
    let a = ((cx as i64) + OFF) & MSK;
    let b = ((cy as i64) + OFF) & MSK;
    let c = ((cz as i64) + OFF) & MSK;
    let packed = a | (b << 21) | (c << 42);
    let h = mix64((packed as u64) ^ ((lid as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)));
    let h = if h == 0 { 0x9E37_79B9_7F4A_7C15 } else { h };
    h as i64
}

#[inline]
fn shard_of(key: i64) -> usize {
    (mix64(key as u64) as usize) & (SHARDS - 1)
}

// ---------------------------------------------------------------------------
// Seqlock-примитивы шарда
// ---------------------------------------------------------------------------

#[inline]
fn shard_lock(s: &Shard) {
    while s
        .lock
        .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        std::hint::spin_loop();
    }
}

#[inline]
fn shard_unlock(s: &Shard) {
    s.lock.store(false, Ordering::Release);
}

#[inline]
fn ver_begin(s: &Shard) {
    s.ver.fetch_or(1, Ordering::AcqRel);
}

#[inline]
fn ver_end(s: &Shard) {
    // нечёт → чёт с инкрементом значения (анти-ABA: 64-бит счётчик)
    s.ver.fetch_add(1, Ordering::Release);
}

// ---------------------------------------------------------------------------
// Цепочки (все мутации — под спинлоком + версионированием шарда-владельца)
// ---------------------------------------------------------------------------

/// Open-addressing поиск слота ключа в шардовой таблице (фиксированная ёмкость,
/// поэтому пробинг ограничен числом слотов). ТОЛЬКО сырые обращения через
/// UnsafeCell-указатель (никаких промежуточных ссылок — читатели и писатели
/// работают параллельно под seqlock-контрактом).
#[inline]
fn find_slot(s: &Shard, k: i64) -> Result<usize, usize> {
    let mask = SHARD_SLOTS - 1;
    let mut t = (mix64(k as u64) as usize) & mask;
    let mut steps = 0usize;
    loop {
        let cur = unsafe { (*s.keys.get())[t] };
        if cur == k {
            return Ok(t);
        }
        if cur == 0 {
            return Err(t);
        }
        t = (t + 1) & mask;
        steps += 1;
        if steps >= SHARD_SLOTS {
            return Err(t); // пробинг исчерпан — шард полон
        }
    }
}

/// Link id головой цепочки ячейки k (id обязан быть unlink'нут; шард залочен).
/// Ok(()) / Err(()) — нет свободного слота / перегрузка шарда.
unsafe fn link(s: &Shard, id: usize, k: i64) -> Result<(), ()> {
    let slot = match find_slot(s, k) {
        Ok(t) => t,
        Err(t) => {
            if (*s.keys.get())[t] != 0 {
                return Err(()); // пробинг исчерпан — шард полон
            }
            let used = s.used.load(Ordering::Relaxed);
            if used + 1 > SHARD_SLOTS * 3 / 4 {
                return Err(()); // >75% загрузки — fail-closed, без grow
            }
            (*s.keys.get())[t] = k;
            s.used.store(used + 1, Ordering::Relaxed);
            t
        }
    };
    (*IDS.next.get())[id] = (*s.head.get())[slot];
    (*s.head.get())[slot] = (id as i32) + 1;
    (*IDS.cell.get())[id] = k;
    Ok(())
}

/// Unlink id из его текущей цепочки. Ok(()) или Err(()) — висячая ссылка.
unsafe fn unlink(s: &Shard, id: usize) -> Result<(), ()> {
    let k = (*IDS.cell.get())[id];
    if k == 0 {
        return Err(());
    }
    let slot = match find_slot(s, k) {
        Ok(t) => t,
        Err(_) => return Err(()),
    };
    let mut prev: i32 = 0;
    let mut cur = (*s.head.get())[slot];
    while cur != 0 {
        let cid = (cur - 1) as usize;
        if cid >= ID_CAP {
            return Err(()); // коррупция цепочки — fail-closed
        }
        if cid == id {
            if prev == 0 {
                (*s.head.get())[slot] = (*IDS.next.get())[cid];
            } else {
                (*IDS.next.get())[(prev - 1) as usize] = (*IDS.next.get())[cid];
            }
            (*IDS.next.get())[cid] = 0;
            (*IDS.cell.get())[cid] = 0;
            if (*s.head.get())[slot] == 0 {
                (*s.keys.get())[slot] = 0;
                let u = s.used.load(Ordering::Relaxed);
                s.used.store(u.saturating_sub(1), Ordering::Relaxed);
            }
            return Ok(());
        }
        prev = cur;
        cur = (*IDS.next.get())[cid];
    }
    Err(())
}

// ---------------------------------------------------------------------------
// Нативы (регистрация на net/minecraft/world/entity/MobHashOps)
// ---------------------------------------------------------------------------

#[inline]
fn spin_pause() {
    std::hint::spin_loop();
}

/// # Safety
/// Вызывается JVM через RegisterNatives; env/class — живые JNI-указатели.
#[no_mangle]
pub unsafe extern "system" fn mh_probe(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    0x4D48 // "MH"
}

/// Апсерт id в ячейку центра (x,y,z). Same-cell → 0 без блокировок.
/// Кросс-шард: unlink+link, спинлоки в возрастающем порядке индексов.
///
/// # Safety
/// См. mh_probe.
#[no_mangle]
pub unsafe extern "system" fn mh_upsert(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
    lid: jni::jint,
    x: jni::jdouble,
    y: jni::jdouble,
    z: jni::jdouble,
) -> jni::jint {
    if id < 0 || (id as usize) >= ID_CAP {
        return ERR_STRUCT;
    }
    let id = id as usize;
    let k = cell_key(lid, x.floor() as i32, y.floor() as i32, z.floor() as i32);
    let cur_cell = (*IDS.cell.get())[id];
    if cur_cell == k {
        return 0; // same-cell fast path — нет блокировок и версий
    }
    let sn = shard_of(k);
    if cur_cell != 0 {
        let so = shard_of(cur_cell);
        if so == sn {
            let s = &SHARD_ARR[sn];
            shard_lock(s);
            ver_begin(s);
            let r = unlink(s, id).and_then(|_| link(s, id, k));
            ver_end(s);
            shard_unlock(s);
            return if r.is_ok() { 0 } else { ERR_STRUCT };
        }
        let (lo, hi) = if so < sn { (so, sn) } else { (sn, so) };
        let slo = &SHARD_ARR[lo];
        let shi = &SHARD_ARR[hi];
        shard_lock(slo);
        shard_lock(shi);
        // unlink из старого шарда под его версией
        let sold = &SHARD_ARR[so];
        ver_begin(sold);
        let r = unlink(sold, id);
        ver_end(sold);
        // link в новый шард под его версией
        let snew = &SHARD_ARR[sn];
        ver_begin(snew);
        let r2 = link(snew, id, k);
        ver_end(snew);
        shard_unlock(shi);
        shard_unlock(slo);
        return if r.is_ok() && r2.is_ok() { 0 } else { ERR_STRUCT };
    }
    // id ещё не в индексе — только link
    let s = &SHARD_ARR[sn];
    shard_lock(s);
    ver_begin(s);
    let r = link(s, id, k);
    ver_end(s);
    shard_unlock(s);
    if r.is_ok() { 0 } else { ERR_STRUCT }
}

/// Удаление id из индекса (graveyard-свип java-стороны).
///
/// # Safety
/// См. mh_probe.
#[no_mangle]
pub unsafe extern "system" fn mh_remove(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    id: jni::jint,
) -> jni::jint {
    if id < 0 || (id as usize) >= ID_CAP {
        return ERR_STRUCT;
    }
    let id = id as usize;
    let k = (*IDS.cell.get())[id];
    if k == 0 {
        return 0; // не в индексе — нечего делать
    }
    let s = &SHARD_ARR[shard_of(k)];
    shard_lock(s);
    ver_begin(s);
    let r = unlink(s, id);
    ver_end(s);
    shard_unlock(s);
    if r.is_ok() { 0 } else { ERR_STRUCT }
}

/// Запрос кандидатов, чьи ЦЕНТРЫ попадают в окно ячеек
/// [qx0..qx1]×[qy0..qy1]×[qz0..qz1] с pad ±1 ячейки (корректность pad — шапка
/// модуля). Пишет id в pinned `out`, возвращает количество; -(out_cap) при
/// переполнении (caller растит скретч и ретраит), ERR_RANGE — абсурдное окно /
/// исчерпание seqlock-ретраев (per-call vanilla), ERR_STRUCT — JNI-неприятности
/// или коррупция цепочки (дизарм).
///
/// # Safety
/// См. mh_probe.
#[no_mangle]
pub unsafe extern "system" fn mh_query(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    qx0: jni::jdouble,
    qy0: jni::jdouble,
    qz0: jni::jdouble,
    qx1: jni::jdouble,
    qy1: jni::jdouble,
    qz1: jni::jdouble,
    lid: jni::jint,
    out: jni::jintArray,
) -> jni::jint {
    if env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };

    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_RANGE;
    }
    let cap = cap as i32;

    // Окно ячеек с pad ±1; гард абсурдных окон (радиус-гейт java-стороны).
    let cx0 = qx0.floor() as i32 - 1;
    let cx1 = qx1.floor() as i32 + 1;
    let cy0 = qy0.floor() as i32 - 1;
    let cy1 = qy1.floor() as i32 + 1;
    let cz0 = qz0.floor() as i32 - 1;
    let cz1 = qz1.floor() as i32 + 1;
    if (cx1 - cx0) > MAX_WINDOW || (cy1 - cy0) > MAX_WINDOW || (cz1 - cz0) > MAX_WINDOW {
        return ERR_RANGE;
    }

    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jint, cap as usize) };

    let mut n: i32 = 0;
    'outer: for cz in cz0..=cz1 {
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                let k = cell_key(lid, cx, cy, cz);
                let s = &SHARD_ARR[shard_of(k)];
                // --- seqlock-чтение ячейки (cell-local retry) ---
                let mut tries: u32 = 0;
                loop {
                    if n >= cap {
                        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
                        return -cap;
                    }
                    tries += 1;
                    if tries > MAX_RETRIES {
                        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
                        return ERR_RANGE;
                    }
                    let v1 = s.ver.load(Ordering::Acquire);
                    if v1 & 1 == 1 {
                        spin_pause();
                        continue;
                    }
                    // --- согласованное чтение ячейки (без блокировок) ---
                    let mut walk: i32 = match find_slot(s, k) {
                        Ok(slot) => (*s.head.get())[slot],
                        Err(_) => 0,
                    };
                    while walk != 0 {
                        let id = (walk - 1) as usize;
                        if id >= ID_CAP {
                            // коррупция: id вне пространства — fail-closed
                            unsafe {
                                (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0)
                            };
                            return ERR_STRUCT;
                        }
                        if (*IDS.cell.get())[id] == k {
                            if n >= cap {
                                unsafe {
                                    (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0)
                                };
                                return -cap;
                            }
                            dst[n as usize] = walk - 1;
                            n += 1;
                        }
                        walk = (*IDS.next.get())[id];
                    }
                    let v2 = s.ver.load(Ordering::Acquire);
                    if v1 == v2 {
                        continue 'outer; // ячейка прочитана согласованно
                    }
                    spin_pause(); // v1 != v2 — retry ячейки
                }
            }
        }
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
    n
}
