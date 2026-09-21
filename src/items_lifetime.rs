//! Lifetime-heap for batch item despawn (TASK-399-F — vector despawnv2, lever
//! cmp399_despawn2; base round-398-j-subsys2 ITEM-SUBSYS2).
//!
//! МЕХАНИКА: деспавн-дедлайны items живут В RUST как min-heap
//! `(i64 due_tick, i32 id)` на плоском Vec (ручной sift — Vec::new() const,
//! статическая инициализация без lazycell). Java пушит дедлайны БАТЧЕМ (один
//! native-вызов на тик: буфер indexAdd-ов + enmass при активации) и раз в тик
//! ОДНИМ native-вызовом забирает все due-id (`due <= now_tick`). Применение —
//! на java-стороне, строго ванильный flow на каждый due-id:
//! `age >= despawnRate` (live-верификация) → CraftEventFactory.
//! callItemDespawnEvent → cancel? age=0 (+re-push rate) : discard(DESPAWN);
//! stale/early запись (merge делает `survivor.age = min(...)` — javap
//! merge(...4 args) @32..42 — или внешний setAge) → re-push с live-возрастом.
//! Событие не пропускается никогда — паритет по построению.
//!
//! ЧТО УБРАНО ИЗ JAVA-ХВОСТА: per-item despawn-гейт в ItemEntityManager.
//! tickBody (реплика offsets 544..588: isClientSide + age >= despawnRate +
//! MethodHandle-getter despawnRate) исполняется только пока despawn2 неактивен
//! (fail-closed). age++ остаётся per-item (public-поле, наблюдаемо merge).
//!
//! КОНТРАСТ С H2 (round-397-h-despawn_heap, −19.7% RED): там java.util.
//! PriorityQueue + per-op JNI на 150k items = alloc-churn + JNI-шторм. Здесь
//! куча живёт в rust, JNI-трафик = 2 вызова/тик (батч-пуш + drain) независимо
//! от населения.
//!
//! THREADING: все вызовы — main-поток (RegionTickOps.forEach после join фазы;
//! буфер indexAdd наполняется и воркерами через lazy indexAdd — java-сторона
//! синхронизирует буфер). Mutex внутри — от слова совсем.
//!
//! FAIL-CLOSED: любой Throwable на java-стороне гасит despawn2Active →
//! ванильная despawn-ветка tickBody возвращается; недренированные записи
//! подчищаются последующими поллами (верификация отсекает мёртвые id).

use jvmti_bindings::jni;

/// Плоская binary min-heap по `(due, id)`. Корень — минимальный due.
struct HeapState {
    v: Vec<(i64, i32)>,
    /// Недоставленные due-id прошлого drain-вызова (grow-retry паттерн idxQuery).
    staging: Vec<(i64, i32)>,
}

impl HeapState {
    const fn new() -> Self {
        HeapState { v: Vec::new(), staging: Vec::new() }
    }

    #[inline]
    fn push(&mut self, due: i64, id: i32) {
        let v = &mut self.v;
        let mut i = v.len();
        v.push((due, id));
        // sift-up
        while i > 0 {
            let p = (i - 1) / 2;
            if v[p].0 <= v[i].0 {
                break;
            }
            v.swap(p, i);
            i = p;
        }
    }

    /// Снять корень (минимум) — предполагает !v.is_empty().
    #[inline]
    fn pop_min(&mut self) -> (i64, i32) {
        let v = &mut self.v;
        let last = v.len() - 1;
        v.swap(0, last);
        let min = v.pop().expect("non-empty");
        // sift-down
        let n = v.len();
        let mut i = 0usize;
        loop {
            let l = 2 * i + 1;
            if l >= n {
                break;
            }
            let r = l + 1;
            let m = if r < n && v[r].0 < v[l].0 { r } else { l };
            if v[i].0 <= v[m].0 {
                break;
            }
            v.swap(i, m);
            i = m;
        }
        min
    }
}

static HEAP: std::sync::Mutex<HeapState> = std::sync::Mutex::new(HeapState::new());

#[inline]
fn heap() -> std::sync::MutexGuard<'static, HeapState> {
    // Poisoning не разбираем: паника в native под локом = недопустимое
    // состояние кучи, java-сторона всё равно fail-closed по верификации.
    match HEAP.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

const ERR_STRUCT: i32 = -1;

/// Батч-пуш дедлайнов. Каждый long: `(id as i64) << 32 | (due & 0xFFFFFFFF)`.
/// due — абсолютный тик (MinecraftServer.currentTick + remaining), шкала java.
///
/// # Safety
/// Вызывается JVM через RegisterNatives; env/clazz — живые JNI-указатели
/// вызывающего потока.
#[no_mangle]
pub unsafe extern "system" fn lifetime_push(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    batch: jni::jlongArray,
    n: jni::jint,
) -> jni::jint {
    if env.is_null() || batch.is_null() || n < 0 {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, batch) };
    if n as i64 > cap as i64 {
        return ERR_STRUCT;
    }
    let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, batch, std::ptr::null_mut()) };
    if pinned.is_null() {
        return ERR_STRUCT;
    }
    let src = unsafe { std::slice::from_raw_parts(pinned as *const jni::jlong, n as usize) };
    {
        let mut g = heap();
        for &l in src {
            let id = (l >> 32) as i32;
            let due = (l as i32) as i64; // младшие 32 бита, знаковое расширение
            if id < 0 {
                continue; // мусорная запись — java не присылает такое
            }
            g.push(due, id);
        }
    }
    unsafe { (vt.ReleasePrimitiveArrayCritical)(env, batch, pinned, 0) };
    0
}

/// Drain всех записей с `due <= now` в `out` (long в том же пакете id<<32|due).
/// Возвращает число записей; если out меньше — возвращает `-(общее число due)`
/// и НЕ копирует (записи ждут в staging; ретрай java-стороны с большим
/// массивом просто их заберёт — паттерн idxQuery).
///
/// # Safety
/// См. lifetime_push.
#[no_mangle]
pub unsafe extern "system" fn lifetime_due(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    now: jni::jlong,
    out: jni::jlongArray,
) -> jni::jint {
    if env.is_null() || out.is_null() {
        return ERR_STRUCT;
    }
    let vt = unsafe { &**env };
    let cap = unsafe { (vt.GetArrayLength)(env, out) };
    if cap <= 0 {
        return ERR_STRUCT;
    }
    let cap = cap as usize;

    let mut g = heap();
    // 1) продолжить/завершить drain в staging
    loop {
        let min_due = match g.v.first() {
            Some(&(d, _)) => d,
            None => break,
        };
        if min_due > now {
            break;
        }
        let e = g.pop_min();
        g.staging.push(e);
    }
    // 2) выдать
    let m = g.staging.len();
    if m > cap {
        return -(m as i32); // staging ждёт ретрая
    }
    if m > 0 {
        let pinned = unsafe { (vt.GetPrimitiveArrayCritical)(env, out, std::ptr::null_mut()) };
        if pinned.is_null() {
            return ERR_STRUCT;
        }
        let dst = unsafe { std::slice::from_raw_parts_mut(pinned as *mut jni::jlong, cap) };
        for (i, &(due, id)) in g.staging.iter().enumerate() {
            dst[i] = ((id as i64) << 32) | ((due & 0xFFFF_FFFF) as i64);
        }
        unsafe { (vt.ReleasePrimitiveArrayCritical)(env, out, pinned, 0) };
        g.staging.clear();
    }
    m as i32
}
