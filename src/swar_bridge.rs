//! SWAR-BRIDGE (AG-247 w530, TASK-458-I iter-2) — java-мост `swarEpoch`,
//! подключающий kernel iter-1 (src/mobs_swa.rs, merge f4a9e40f: 2×lower_bound
//! бинпоиск-окно по xmin-сортированному SoA f32, IEEE-монотонный u32-ключ,
//! переносимый SWAR 8-wide batch-AABB на u64 без intrinsics, bit-for-bit
//! scalar fallback, монотонный insertion-sort SAP push-хвост) к java-стороне
//! broadphase-лейна (java-перечисление pushCandidates = 8.8-10.9% CPU при
//! 47k+ популяции; ожидаемый выигрыш рычага +5..8пп к ноге).
//!
//! ТРЕТИЙ bulk-JNI в EPOCH_LOCK-окне (прецеденты: eqEpoch
//! EntityGoalQueryOps, senseArena SenseOps; window order iter-3: swarEpoch →
//! eqEpoch → senseArena — eq-цепи строятся из СВЕЖИХ колонок, ID-I01/I02).
//! ОДИН нативный вызов на тик: фид живых боксов + query-ректы → rust тянет
//! SAP-хвост и строит CSR-строки кандидатов (superset); java строгий
//! ванильный хвост по строкам (iter-3 wiring). Ноль per-entity переходов
//! (nav_plane/navDecide + colpushTick Err-ladder дисциплина).
//!
//! МОСТ: entityinside/net/minecraft/world/entity/MobSwaOps.java (SELF-CONTAINED,
//! <clinit> plain — NCDFE-канон ×93-indy: в <clinit> НЕТ indy/метод-ссылок/
//! ThreadLocal.withInitial, вложенных классов нет — ранний arm-хук определяет
//! класс ДО первого пуша данных, ARM-AFTER-DEFINE fa9054d9). Лестница
//! fail-closed: мост жив (define) → boot-quiet → selftest → ARM
//! (noteSwaArmed); любой дефект = sticky broken java-стороны (ERR_STRUCT) или
//! класс не армится вовсе → ваниль бит-в-байт.
//!
//! SCALAR-FALLBACK MANDATORY: mode=0 через ТУ ЖЕ связку обязан давать
//! bit-for-bit тот же CSR, что mode=1 (SWAR) — самтесты с обеих сторон
//! (rust: scalar_mode_bit_for_bit_with_swar_mode; java: MobSwaOps.selfTest).
//!
//! LEVER: STRICT-eq `cmp458_swar` (round-400 протокол; один id, никаких
//! союзов по env — swarx-4 урок; совпадает с фактическим STRICT lever id
//! рычага в lib.rs/mobs_manager/java_gate-листах). DORMANT by default: флаг
//! unset/чужой → register/activate no-op, модуль байт-невидим, поведение
//! ванили байт-в-байт.

#![allow(dead_code)]

use jvmti_bindings::jni;
use jvmti_bindings::prelude::JniEnv;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

/// STRICT-eq lever id (round-400; сверено с фактическим кодом: lib.rs:70
/// комментарий рычага + java_gate_matches/ARM-листы mobs_manager.rs).
pub const R_LEVER_FLAG: &str = "cmp458_swar";

/// Мост-класс (пакет net.minecraft.world.entity — define_class в kernel
/// loader, delivery-паттерн InsideBlockOps/InsideBatchOps).
pub const BRIDGE_CLASS: &str = "net/minecraft/world/entity/MobSwaOps";
/// Анкер kernel loader (Entity грузится на старте; в этом лоадере живёт мост).
const ANCHOR_CLASS: &str = "net/minecraft/world/entity/Entity";

/// Встроенный блоб (include_bytes! contract = NESTED путь; ×93 дисциплина:
/// flat-копия legacy + flat==nested byte gate на пересборке).
const BRIDGE_BYTES: &[u8] =
    include_bytes!("../entityinside/build/net/minecraft/world/entity/MobSwaOps.class");

/// swarEpoch(mode, snapshot, nFeed, long[] feedIds, float[] feedBox,
///           nQ, float[] qBox, int[] outOff, long[] outRow, int[] outOvf) -> rc.
/// Сигнатура = объявление натива в MobSwaOps.java БАЙТ-В-БАЙТ (урок 409-E:
/// stray sig = NoSuchMethodError каждый boot, silent sleeping gate).
/// Сверено javap-гейтом скомпилированного блоба: descriptor
/// (III[J[FI[F[I[J[I)I — int[] outOff = [I, long[] outRow = [J, int[] outOvf = [I.
pub const SWAR_EPOCH_SIG: &str = "(III[J[FI[F[I[J[I)I";
/// swarReset() -> rc (selfTest-гигиена; не hot-путь).
pub const SWAR_RESET_SIG: &str = "()I";

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// Канон-константы (RESEARCH-458-I §1; зеркала MobSwaOps.java).
const MARGIN: f32 = 8.0;
/// Rust-сторона капов шире java-констант (java MAXFEED=65536/MAXQ=1024 —
/// rust не режет легальный вызов, ERR_RANGE только на абсурд).
const FEED_CAP: usize = 1 << 20;
const Q_CAP: usize = 4096;
/// Пер-квери кап кандидатов: выше — ovf-флаг кверии (пер-квери vanilla
/// fallback, fail-open superset; фермы/кластеры деградируют локально).
const ROW_MAX_PER_QUERY: usize = 4096;
/// Self-heal порог: moves > n/4 + 64 ⇒ полный re-sort (деградация SAP).
const HEAL_MOVES_BASE: u64 = 64;

// -------------------------------------------------------------------------
// Персистентное состояние моста (SAP-плоскость)
// -------------------------------------------------------------------------

/// Персистентная SAP-плоскость моста: kernel SoA + id→slot карта + max_span.
/// Живёт между epoch-вызовами; фид = upsert/snapshot-evict; монотонный
/// insertion-sort push-хвост с self-heal (kernel-канон iter-1 §d).
#[derive(Default)]
pub struct SwaBridgeState {
    pub soa: crate::mobs_swa::SwaSoa,
    slot_by_id: HashMap<u32, u32>,
    /// max (max_x - min_x) живых боксов (finite only): x_lo окна =
    /// q.min_x - MARGIN - max_span держит superset-преусловие kernel.
    max_span: f32,
    seen: Vec<bool>,
    // G2-телеметрия (монотонные счётчики, никогда не сбрасываются).
    pub calls: u64,
    pub rebuilds: u64,
    pub moves_last: u64,
    pub heals: u64,
    pub evictions: u64,
    pub ovf_queries: u64,
}

impl SwaBridgeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Полный сброс плоскости (selfTest-гигиена; телеметрия не сбрасывается).
    pub fn reset(&mut self) {
        self.soa = crate::mobs_swa::SwaSoa::default();
        self.slot_by_id.clear();
        self.max_span = 0.0;
        self.seen.clear();
        self.rebuilds += 1;
    }

    pub fn span(&self) -> f32 {
        self.max_span
    }

    /// Слот по entity id (None = мёртвый/эвикнутый).
    pub fn slot_of(&self, id: u32) -> Option<u32> {
        self.slot_by_id.get(&id).copied()
    }
}

/// next_up(x) по IEEE bits (x >= 0: bits+1; x < 0: bits-1; ±0/NaN/+inf —
/// инварианты; -inf → -max_finite). Нужно для закрытия exact-boundary дыры
/// полуприкрытого окна (см. epoch).
#[inline]
fn next_up_f32(x: f32) -> f32 {
    if x.is_nan() || x == f32::INFINITY {
        return x;
    }
    let b = x.to_bits();
    if x == 0.0 {
        f32::from_bits(1) // -0.0/+0.0 → min subnormal
    } else if x > 0.0 {
        f32::from_bits(b + 1) // может стать +inf: окно «всё до конца» — sound
    } else {
        f32::from_bits(b - 1)
    }
}

impl SwaBridgeState {
    /// Ядро epoch-вызова (JVM-free: JNI-обёртка только копирует массивы).
    ///
    /// mode: 0 = scalar fallback, 1 = SWAR (обязаны совпадать bit-for-bit —
    /// самтесты с обеих сторон). snapshot: 1 = фид = ПОЛНЫЙ живой сет
    /// (отсутствующие id эвиктятся), 0 = частичный upsert.
    /// feed_box: n_feed*4 (minX,maxX,minZ,maxZ); q_box: n_q*4.
    /// out: out_off (n_q+1 префикс в out_row), out_row (CSR entity-id),
    /// out_ovf (n_q; бит0 = пер-квери переполнение → java ваниль для НЕЁ).
    ///
    /// rc: 0 OK; ERR_RANGE — чужой mode/snapshot/caps/длины/диапазон id;
    /// ERR_STRUCT не возникает здесь (слой JNI). NaN-колонки фида — fail-closed
    /// (лейн не матчится, канон-тест iter-1 nan_and_degenerate_fail_closed);
    /// NaN-кверия → пустое окно (ключ NaN максимален).
    pub fn epoch(
        &mut self,
        mode: i32,
        snapshot: i32,
        n_feed: usize,
        feed_ids: &[i64],
        feed_box: &[f32],
        n_q: usize,
        q_box: &[f32],
        out_off: &mut [i32],
        out_row: &mut [i64],
        out_ovf: &mut [i32],
    ) -> i32 {
        self.calls += 1;
        if mode != 0 && mode != 1 {
            return ERR_RANGE;
        }
        if snapshot != 0 && snapshot != 1 {
            return ERR_RANGE;
        }
        if n_feed > FEED_CAP || n_q > Q_CAP {
            return ERR_RANGE;
        }
        if feed_ids.len() < n_feed
            || feed_box.len() < n_feed * 4
            || q_box.len() < n_q * 4
            || out_off.len() < n_q + 1
            || out_ovf.len() < n_q
            || out_row.is_empty()
        {
            return ERR_RANGE;
        }
        // entity id — java int (getId()): i64 вне u32 = чужой контрак → ваниль.
        for &id in &feed_ids[..n_feed] {
            if id < 0 || id > u32::MAX as i64 {
                return ERR_RANGE;
            }
        }

        // ---- фид: upsert + (snapshot) evict, сохраняя порядок плоскости ----
        self.seen.clear();
        self.seen.resize(self.soa.len(), false);
        for i in 0..n_feed {
            let id = feed_ids[i] as u32;
            let (mnx, mxx, mnz, mxz) = (
                feed_box[i * 4],
                feed_box[i * 4 + 1],
                feed_box[i * 4 + 2],
                feed_box[i * 4 + 3],
            );
            match self.slot_by_id.get(&id) {
                Some(&slot) => {
                    // in-place: тик-дрейф почти не рождает инверсий — SAP-канон
                    self.soa.min_x[slot as usize] = mnx;
                    self.soa.max_x[slot as usize] = mxx;
                    self.soa.min_z[slot as usize] = mnz;
                    self.soa.max_z[slot as usize] = mxz;
                    self.seen[slot as usize] = true;
                }
                None => {
                    // новый слот в КОНЦЕ (insertion-хвост поднимет его в окно)
                    let slot = self.soa.len() as u32;
                    self.soa.min_x.push(mnx);
                    self.soa.max_x.push(mxx);
                    self.soa.min_z.push(mnz);
                    self.soa.max_z.push(mxz);
                    self.soa.ids.push(id);
                    self.slot_by_id.insert(id, slot);
                    self.seen.push(true);
                    self.rebuilds += 0; // append — не rebuild (телеметрия честная)
                }
            }
        }
        if snapshot == 1 {
            // evict + компакция ПОРЯДКА (относительный порядок живых слотов
            // сохраняется — сортировка не портится), карта перестраивается.
            let n = self.soa.len();
            let mut w = 0usize;
            for s in 0..n {
                if self.seen[s] {
                    self.soa.min_x.swap(w, s);
                    self.soa.max_x.swap(w, s);
                    self.soa.min_z.swap(w, s);
                    self.soa.max_z.swap(w, s);
                    self.soa.ids.swap(w, s);
                    w += 1;
                } else {
                    self.evictions += 1;
                }
            }
            if w != n {
                self.soa.min_x.truncate(w);
                self.soa.max_x.truncate(w);
                self.soa.min_z.truncate(w);
                self.soa.max_z.truncate(w);
                self.soa.ids.truncate(w);
                self.slot_by_id = self
                    .soa
                    .ids
                    .iter()
                    .enumerate()
                    .map(|(slot, &id)| (id, slot as u32))
                    .collect();
            }
        }

        // ---- монотонный SAP push-хвост + self-heal (kernel §d) ----
        let n = self.soa.len();
        let moves = crate::mobs_swa::insertion_sort_pass(&mut self.soa);
        self.moves_last = moves;
        if n as u64 / 4 + HEAL_MOVES_BASE < moves {
            // деградация (штампед спавнов/телепорт): полный re-sort по ключу
            self.full_resort();
        }

        // точный max_span (O(n)); non-finite спаны игнорируются — их лейны
        // всё равно fail-closed (NaN) или никогда не в окне (±inf: span inf).
        self.max_span = 0.0f32;
        for i in 0..n {
            let s = self.soa.max_x[i] - self.soa.min_x[i];
            if s.is_finite() && s > self.max_span {
                self.max_span = s;
            }
        }

        // ---- кверии: окно 2×lower_bound + SWAR/scalar маска → CSR ----
        out_off[0] = 0;
        let mut w = 0usize;
        for qi in 0..n_q {
            let (qx0, qx1, qz0, qz1) = (
                q_box[qi * 4],
                q_box[qi * 4 + 1],
                q_box[qi * 4 + 2],
                q_box[qi * 4 + 3],
            );
            let q = crate::mobs_swa::Box2 {
                min_x: qx0,
                min_y: 0.0,
                min_z: qz0,
                max_x: qx1,
                max_y: 0.0,
                max_z: qz1,
            };
            let qi_inf = q.inflate_xz(MARGIN);
            // x_lo держит superset (точный max_span, kernel-преусловие);
            // x_hi расширен ОДНИМ ulp: полуприкрытое окно [lo, hi) исключало
            // бокс с min_x == qi.max_x ровно, а ordered-предикат его матчит
            // (min_x <= qi.max_x — включительно) ⇒ exact-boundary дыра.
            let x_lo = qx0 - MARGIN - self.max_span;
            let x_hi = next_up_f32(qx1 + MARGIN);
            let (wlo, whi) = crate::mobs_swa::window_bounds(&self.soa, x_lo, x_hi);
            let masks = if mode == 1 {
                crate::mobs_swa::swar_window_mask(&self.soa, wlo, whi, &qi_inf)
            } else {
                crate::mobs_swa::scalar_window_mask(&self.soa, wlo, whi, &qi_inf)
            };
            let mut hits = 0usize;
            let mut ovf = false;
            'chunks: for &(base, m) in &masks {
                for lane in 0..8usize {
                    if (m >> lane) & 1 == 1 {
                        if hits >= ROW_MAX_PER_QUERY || w >= out_row.len() {
                            ovf = true;
                            break 'chunks;
                        }
                        out_row[w] = self.soa.ids[base + lane] as i64;
                        w += 1;
                        hits += 1;
                    }
                }
            }
            out_ovf[qi] = if ovf { 1 } else { 0 };
            if ovf {
                self.ovf_queries += 1;
            }
            out_off[qi + 1] = w as i32;
        }
        0
    }

    /// Полный re-sort: перестановка по f32_ordered_key + rebuild id-карты.
    fn full_resort(&mut self) {
        let n = self.soa.len();
        let mut idx: Vec<u32> = (0..n as u32).collect();
        idx.sort_by_key(|&i| crate::mobs_swa::f32_ordered_key(self.soa.min_x[i as usize]));
        let (mut a, mut b, mut c, mut d, mut e) = (
            Vec::with_capacity(n),
            Vec::with_capacity(n),
            Vec::with_capacity(n),
            Vec::with_capacity(n),
            Vec::with_capacity(n),
        );
        for &i in &idx {
            a.push(self.soa.min_x[i as usize]);
            b.push(self.soa.max_x[i as usize]);
            c.push(self.soa.min_z[i as usize]);
            d.push(self.soa.max_z[i as usize]);
            e.push(self.soa.ids[i as usize]);
        }
        self.soa.min_x = a;
        self.soa.max_x = b;
        self.soa.min_z = c;
        self.soa.max_z = d;
        self.soa.ids = e;
        self.slot_by_id = self
            .soa
            .ids
            .iter()
            .enumerate()
            .map(|(slot, &id)| (id, slot as u32))
            .collect();
        self.heals += 1;
    }
}

// -------------------------------------------------------------------------
// Глобальный синглтон (JNI-сторона) + STRICT lever gate
// -------------------------------------------------------------------------

static STATE: OnceLock<Mutex<SwaBridgeState>> = OnceLock::new();
static ARMED: AtomicBool = AtomicBool::new(false);

fn state() -> &'static Mutex<SwaBridgeState> {
    STATE.get_or_init(|| Mutex::new(SwaBridgeState::new()))
}

/// STRICT-eq гейт (пустой/чужой флаг = DORMANT, ваниль байт-в-байт).
pub fn enabled() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == R_LEVER_FLAG)
        .unwrap_or(false)
}

/// Тестируемая форма гейта (мобс-sense прецедент enabled_with).
pub fn enabled_with(flag: &str) -> bool {
    flag.trim() == R_LEVER_FLAG
}

/// Признак живого моста (для логов/телеметрии; поллинга потребителей нет —
/// call-site wiring = iter-3).
pub fn armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

/// Register (один вызов из cplugin_init, ladder-сайт #1). Хуков НЕ ставит —
/// мост определяется ранним arm-хуком activate (NCDFE-канон), байт-патчей
/// у рычага нет (strict-хвост = iter-3).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] swar_bridge: dormant (set CRUSSTY_LEVER_FLAG={R_LEVER_FLAG} to enable)"
        );
        return;
    }
    eprintln!(
        "[crussty-plugin] swar_bridge: owner armed, MobSwaOps define deferred to activate (early arm-hook, NCDFE ARM-AFTER-DEFINE)"
    );
}

/// Background activation (ladder-сайт #2, рядом с mobs_manager::activate):
/// ждать kernel Entity → boot-quiet → guard major → define MobSwaOps в
/// kernel loader → RegisterNatives(swarEpoch/swarReset) → java selfTest →
/// noteSwaArmed. Любая ступень упала ⇒ ARM не публикуется (fail-closed,
/// ваниль; класс без ARM ничего не меняет — swarTick отдаёт RC_DISARMED).
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        // 1. Ждать kernel-класс Entity (точка захвата kernel loader).
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        loop {
            if cplug_sdk::classes::find_class(ANCHOR_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] swar_bridge: {ANCHOR_CLASS} not loaded within 180s, bridge stays undefined"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170) {
                eprintln!(
                    "[crussty-plugin] swar_bridge: forcing kernel load of {ANCHOR_CLASS}"
                );
                crate::improved_noise::force_load_kernel_class(ANCHOR_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ANCHOR_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        // 2. Kernel loader затих до define (boot-storm дисциплина TASK-80).
        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] swar_bridge: boot marker not seen, bridge stays dormant");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(20));

        // 3. Guard: встроенный блоб не новее JVM (урок 408: stale blob = спящий гейт).
        let jvm_major = cplug_sdk::jni_util::with_attached(|env| {
            crate::improved_noise::jvm_class_major(env)
                .or_else(|| crate::improved_noise::jvm_max_class_major(env))
        })
        .flatten()
        .unwrap_or(u16::MAX);
        let major = crate::improved_noise::class_version(BRIDGE_BYTES)
            .map(|(m, _)| m)
            .unwrap_or(0);
        if major > jvm_major {
            eprintln!(
                "[crussty-plugin] swar_bridge: {BRIDGE_CLASS} is class major {major} but JVM supports up to {jvm_major} — rebuild entityinside/ (pinned ECJ command in MobSwaOps javadoc); bridge stays dormant"
            );
            return;
        }

        // 4. Захват kernel loader от Entity + define + RegisterNatives + selfTest + ARM.
        let ok = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ANCHOR_CLASS) else {
                return false;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return false;
            };
            let Some(loader) = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                })
            else {
                crate::clear_exception(env);
                env.delete_local_ref(class_cls);
                return false;
            };
            let gref = env.new_global_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                env.delete_local_ref(loader);
                env.delete_local_ref(class_cls);
                return false;
            }
            let Some(c) = env.define_class(BRIDGE_CLASS, gref, BRIDGE_BYTES) else {
                crate::describe_exception(env);
                eprintln!(
                    "[crussty-plugin] swar_bridge: define_class({BRIDGE_CLASS}) failed — bridge stays dormant"
                );
                return false;
            };
            eprintln!(
                "[crussty-plugin] swar_bridge: defined {BRIDGE_CLASS} in kernel loader (early, before first data push)"
            );
            // RegisterNatives ДО selfTest/ARM (arm-order контракт: define +
            // natives + selfTest, ARM строго последним).
            let natives_ok = register_natives(env, c);
            // asm.rs canon: держим global ref класса до конца (JNI-defined
            // класс не резолвится find_class на приаттаченном native-потоке).
            let cgr = env.new_global_ref(c);
            env.delete_local_ref(c);
            if !natives_ok {
                return false;
            }
            // java selfTest ДО ARM (TASK-437-A паттерн; end-to-end оракул
            // против rust-кернела через СВЯЗКУ: SWAR==scalar bit-for-bit +
            // superset). Падение ⇒ ARM не публикуется (класс мёртв = ваниль).
            if !java_selftest(env, cgr) {
                eprintln!(
                    "[crussty-plugin] swar_bridge: java selfTest=false — bridge stays disarmed (fail-closed, vanilla bit-for-bit)"
                );
                return false;
            }
            // ARM — строго последняя ступень.
            let Some(arm_mid) = env.get_static_method_id(cgr, "noteSwaArmed", "()V") else {
                crate::clear_exception(env);
                eprintln!(
                    "[crussty-plugin] swar_bridge: noteSwaArmed unresolved — bridge stays disarmed (fail-closed)"
                );
                return false;
            };
            env.call_static_void_method(cgr, arm_mid, &[]);
            true
        });
        if !ok.unwrap_or(false) {
            eprintln!(
                "[crussty-plugin] swar_bridge: bridge ladder aborted, stays dormant (fail-closed)"
            );
            return;
        }
        ARMED.store(true, Ordering::Release);
        crate::kernel_policy::audit_wire(
            BRIDGE_CLASS,
            "swarEpoch",
            "cmp458_swar v1 (third bulk-JNI in EPOCH_LOCK window; kernel iter-1 f4a9e40f)",
        );
        eprintln!(
            "[crussty-plugin] {R_LEVER_FLAG}: ARMED (MobSwaOps defined+natives+selfTest+noteSwaArmed; rust swarEpoch = ONE bulk JNI/tick: feed upsert/snapshot-evict -> insertion-sort SAP tail (self-heal) -> 2×lower_bound window -> SWAR 8-wide u64 mask (scalar fallback bit-for-bit) -> CSR rows superset; y not pruned, MARGIN=8.0; call-site wiring + strict vanilla tail = iter-3; empty/foreign flag = vanilla bit-for-bit)"
        );
    });
}

/// RegisterNatives swarEpoch/swarReset на только-что определённом мосте
/// (паттерн inside_batch::register_native; провал ⇒ ladder abort).
fn register_natives(env: &JniEnv, cls: jni::jclass) -> bool {
    let names = [
        CString::new("swarEpoch").expect("no NUL"),
        CString::new("swarReset").expect("no NUL"),
    ];
    let sigs = [
        CString::new(SWAR_EPOCH_SIG).expect("no NUL"),
        CString::new(SWAR_RESET_SIG).expect("no NUL"),
    ];
    let natives = [
        jni::JNINativeMethod {
            name: names[0].as_ptr(),
            signature: sigs[0].as_ptr(),
            fnPtr: swar_epoch as *const c_void as *mut c_void,
        },
        jni::JNINativeMethod {
            name: names[1].as_ptr(),
            signature: sigs[1].as_ptr(),
            fnPtr: swar_reset as *const c_void as *mut c_void,
        },
    ];
    if env.register_natives(cls, &natives).is_err() {
        crate::clear_exception(env);
        eprintln!(
            "[crussty-plugin] swar_bridge: register_natives(swarEpoch/swarReset) failed — bridge stays vanilla"
        );
        return false;
    }
    eprintln!(
        "[crussty-plugin] swar_bridge: natives registered (swarEpoch {SWAR_EPOCH_SIG}, swarReset {SWAR_RESET_SIG})"
    );
    true
}

/// java selfTest на LOCAL ref только-что определённого моста (мобс-sense
/// прецедент: first active use ⇒ <clinit>; find_class-fix TASK-417-C не
/// нужен — свой local ref). Любая пендящая экзепшн = failure (fail-closed).
fn java_selftest(env: &JniEnv, cls: jni::jclass) -> bool {
    let Some(mid) = env.get_static_method_id(cls, "selfTest", "()Z") else {
        crate::clear_exception(env);
        eprintln!("[crussty-plugin] swar_bridge: selfTest method resolution failed");
        return false;
    };
    let rc = env.call_static_int_method(cls, mid, &[]);
    let had_exc = crate::clear_exception(env);
    if had_exc {
        eprintln!(
            "[crussty-plugin] swar_bridge: selfTest threw (late resolution) — fail-closed"
        );
        return false;
    }
    rc != 0
}

// -------------------------------------------------------------------------
// JNI-вход (RegisterNatives; тонкий слой: валидация + копирование массивов)
// -------------------------------------------------------------------------

/// # Safety
/// Вызывается JVM через RegisterNatives; env/class — живые JNI-указатели
/// вызывающего потока.
#[no_mangle]
pub unsafe extern "system" fn swar_epoch(
    env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
    mode: jni::jint,
    snapshot: jni::jint,
    n_feed: jni::jint,
    feed_ids: jni::jlongArray,
    feed_box: jni::jfloatArray,
    n_q: jni::jint,
    q_box: jni::jfloatArray,
    out_off: jni::jintArray,
    out_row: jni::jlongArray,
    out_ovf: jni::jintArray,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    if env.is_null()
        || n_feed < 0
        || n_q < 0
        || feed_ids.is_null()
        || feed_box.is_null()
        || q_box.is_null()
        || out_off.is_null()
        || out_row.is_null()
        || out_ovf.is_null()
    {
        return ERR_STRUCT;
    }
    let n_feed = n_feed as usize;
    let n_q = n_q as usize;
    let vt = unsafe { &*(*env) };
    let len_ids = (vt.GetArrayLength)(env, feed_ids);
    let len_fbox = (vt.GetArrayLength)(env, feed_box);
    let len_qbox = (vt.GetArrayLength)(env, q_box);
    let len_off = (vt.GetArrayLength)(env, out_off);
    let len_row = (vt.GetArrayLength)(env, out_row);
    let len_ovf = (vt.GetArrayLength)(env, out_ovf);
    // JNI-граница: java-массивы обязаны покрывать заявленные n ДО Get*ArrayRegion
    // (иначе pending ArrayIndexOutOfBounds = java-тик ваниль, но дефект вызывающего
    // контракта); ядро повторно валидирует длины rust-копий (один источник истины).
    if n_feed > FEED_CAP
        || n_q > Q_CAP
        || len_ids < n_feed as i32
        || len_fbox < (n_feed * 4) as i32
        || len_qbox < (n_q * 4) as i32
        || len_off < n_q as i32 + 1
        || len_ovf < n_q as i32
        || len_row <= 0
    {
        return ERR_RANGE;
    }
    let mut ids: Vec<i64> = vec![0; n_feed];
    let mut fb: Vec<f32> = vec![0.0; n_feed * 4];
    let mut qb: Vec<f32> = vec![0.0; n_q * 4];
    unsafe {
        (vt.GetLongArrayRegion)(env, feed_ids, 0, n_feed as i32, ids.as_mut_ptr());
        (vt.GetFloatArrayRegion)(env, feed_box, 0, (n_feed * 4) as i32, fb.as_mut_ptr());
        (vt.GetFloatArrayRegion)(env, q_box, 0, (n_q * 4) as i32, qb.as_mut_ptr());
    }
    let mut off: Vec<i32> = vec![0; n_q + 1];
    let mut row: Vec<i64> = vec![0; len_row as usize];
    let mut ovf: Vec<i32> = vec![0; n_q];
    let rc = {
        let mut st = match state().lock() {
            Ok(g) => g,
            Err(_) => return ERR_STRUCT, // poisoned — sticky java-side
        };
        st.epoch(
            mode,
            snapshot,
            n_feed,
            &ids,
            &fb,
            n_q,
            &qb,
            &mut off,
            &mut row,
            &mut ovf,
        )
    };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (vt.SetIntArrayRegion)(env, out_off, 0, (n_q + 1) as i32, off.as_ptr());
        let written = off[n_q] as usize;
        (vt.SetIntArrayRegion)(env, out_ovf, 0, n_q as i32, ovf.as_ptr());
        (vt.SetLongArrayRegion)(env, out_row, 0, written as i32, row.as_ptr());
    }
    0
}

/// # Safety
/// См. swar_epoch.
#[no_mangle]
pub unsafe extern "system" fn swar_reset(
    _env: *mut jni::JNIEnv,
    _clazz: jni::jclass,
) -> jni::jint {
    if !enabled() {
        return ERR_STRUCT;
    }
    match state().lock() {
        Ok(mut st) => {
            st.reset();
            0
        }
        Err(_) => ERR_STRUCT,
    }
}

// -------------------------------------------------------------------------
// Самтесты (JVM-free ядро; JNI-сторону покрывает java selfTest через связку)
// -------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mobs_swa::{f32_ordered_key, Box2};

    /// Детерминированный xorshift64* PRNG (без внешних зависимостей).
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        fn f32_in(&mut self, lo: f32, hi: f32) -> f32 {
            let u = (self.next() >> 40) as f32 / (1u64 << 24) as f32;
            lo + u * (hi - lo)
        }
    }

    const N: usize = 512;
    const MARGIN: f32 = 8.0;
    const MAX_SPAN: f32 = 4.0;

    struct Feed {
        ids: Vec<i64>,
        box_: Vec<f32>,
    }

    /// Фид N боксов (спаны ≤ MAX_SPAN); `order` перемешивает порядок строк —
    /// rust-сторона обязана держать плоскость отсортированной независимо.
    fn build_feed(seed: u64, order: &[usize]) -> Feed {
        let mut rng = Rng(seed);
        let mut f = Feed {
            ids: Vec::with_capacity(N),
            box_: Vec::with_capacity(N * 4),
        };
        let mut rows: Vec<(i64, [f32; 4])> = Vec::with_capacity(N);
        for i in 0..N {
            let cx = rng.f32_in(-512.0, 512.0);
            let cz = rng.f32_in(-512.0, 512.0);
            let hw = rng.f32_in(0.3, MAX_SPAN / 2.0);
            let hd = rng.f32_in(0.3, MAX_SPAN / 2.0);
            rows.push((
                i as i64 + 1,
                [cx - hw, cx + hw, cz - hd, cz + hd],
            ));
        }
        for &o in order {
            let (id, b) = rows[o];
            f.ids.push(id);
            f.box_.extend_from_slice(&b);
        }
        f
    }

    fn run(st: &mut SwaBridgeState, mode: i32, f: &Feed, qs: &[f32], row_cap: usize) -> (Vec<i32>, Vec<i64>, Vec<i32>, i32) {
        let nq = qs.len() / 4;
        let mut off = vec![0i32; nq + 1];
        let mut row = vec![0i64; row_cap];
        let mut ovf = vec![0i32; nq];
        let rc = st.epoch(
            mode,
            1,
            f.ids.len(),
            &f.ids,
            &f.box_,
            nq,
            qs,
            &mut off,
            &mut row,
            &mut ovf,
        );
        (off, row, ovf, rc)
    }

    fn random_queries(seed: u64, n: usize) -> Vec<f32> {
        let mut rng = Rng(seed);
        let mut q = Vec::with_capacity(n * 4);
        for _ in 0..n {
            let x0 = rng.f32_in(-520.0, 500.0);
            let x1 = rng.f32_in(-500.0, 520.0);
            let z0 = rng.f32_in(-520.0, 500.0);
            let z1 = rng.f32_in(-500.0, 520.0);
            q.extend_from_slice(&[x0, x1, z0, z1]);
        }
        q
    }

    /// Строка кверии из CSR (без ovf-бита) как множество id.
    fn row_set(off: &[i32], row: &[i64], qi: usize) -> std::collections::BTreeSet<i64> {
        row[off[qi] as usize..off[qi + 1] as usize]
            .iter()
            .copied()
            .collect()
    }

    #[test]
    fn bridge_rows_match_kernel_direct_call() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0001, &order);
        let qs = random_queries(42, 64);
        let mut st = SwaBridgeState::new();
        let (off, row, ovf, rc) = run(&mut st, 1, &f, &qs, 1 << 16);
        assert_eq!(rc, 0);
        assert!(ovf.iter().all(|&o| o == 0), "selftest scale must fit");
        // прямой вызов kernel-функций над той же плоскостью (та же SoA)
        let mut st2 = SwaBridgeState::new();
        let mut off2 = vec![0i32; off.len()];
        let mut row2 = vec![0i64; 1 << 16];
        let mut ovf2 = vec![0i32; qs.len() / 4];
        assert_eq!(
            st2.epoch(1, 1, f.ids.len(), &f.ids, &f.box_, qs.len() / 4, &qs, &mut off2, &mut row2, &mut ovf2),
            0,
            "independent plane build must succeed"
        );
        assert_eq!(off, off2);
        for qi in 0..qs.len() / 4 {
            assert_eq!(row_set(&off, &row, qi), row_set(&off2, &row2, qi), "q{qi}");
        }
        // окно-инвариант: строки = kernel-маска по тем же границам (widened hi)
        for qi in 0..qs.len() / 4 {
            let (qx0, qx1, qz0, qz1) = (qs[qi * 4], qs[qi * 4 + 1], qs[qi * 4 + 2], qs[qi * 4 + 3]);
            let q = Box2 { min_x: qx0, min_y: 0.0, min_z: qz0, max_x: qx1, max_y: 0.0, max_z: qz1 };
            let qi_inf = q.inflate_xz(MARGIN);
            let x_lo = qx0 - MARGIN - st.span();
            let x_hi = next_up_f32(qx1 + MARGIN);
            let (wlo, whi) = crate::mobs_swa::window_bounds(&st.soa, x_lo, x_hi);
            let masks = crate::mobs_swa::swar_window_mask(&st.soa, wlo, whi, &qi_inf);
            let mut ids = std::collections::BTreeSet::new();
            for &(base, m) in &masks {
                for lane in 0..8 {
                    if (m >> lane) & 1 == 1 {
                        ids.insert(st.soa.ids[base + lane] as i64);
                    }
                }
            }
            assert_eq!(row_set(&off, &row, qi), ids, "q{qi} kernel-window drift");
        }
    }

    #[test]
    fn scalar_mode_bit_for_bit_with_swar_mode() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0002, &order);
        let qs = random_queries(7, 64);
        let mut s1 = SwaBridgeState::new();
        let mut s0 = SwaBridgeState::new();
        let (off1, row1, ovf1, rc1) = run(&mut s1, 1, &f, &qs, 1 << 16);
        let (off0, row0, ovf0, rc0) = run(&mut s0, 0, &f, &qs, 1 << 16);
        assert_eq!(rc1, 0);
        assert_eq!(rc0, 0);
        assert_eq!(off1, off0, "offsets must match bit-for-bit");
        assert_eq!(ovf1, ovf0, "ovf flags must match bit-for-bit");
        for k in 0..off1[off1.len() - 1] as usize {
            assert_eq!(row1[k], row0[k], "CSR row byte {k} differs");
        }
    }

    #[test]
    fn superset_vs_brute_force() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0003, &order);
        let qs = random_queries(11, 64);
        let mut st = SwaBridgeState::new();
        let (off, row, ovf, rc) = run(&mut st, 1, &f, &qs, 1 << 16);
        assert_eq!(rc, 0);
        assert!(ovf.iter().all(|&o| o == 0));
        for qi in 0..qs.len() / 4 {
            // точный предикат кернела: инфлейт MARGIN + 4 ordered cmp (f32)
            let (qmnx, qmnz) = (qs[qi * 4] - MARGIN, qs[qi * 4 + 2] - MARGIN);
            let (qmxx, qmxz) = (qs[qi * 4 + 1] + MARGIN, qs[qi * 4 + 3] + MARGIN);
            let have = row_set(&off, &row, qi);
            for i in 0..N {
                let (mnx, mxx, mnz, mxz) = (
                    f.box_[i * 4],
                    f.box_[i * 4 + 1],
                    f.box_[i * 4 + 2],
                    f.box_[i * 4 + 3],
                );
                if mnx <= qmxx && mxx >= qmnx && mnz <= qmxz && mxz >= qmnz {
                    assert!(
                        have.contains(&(i as i64 + 1)),
                        "exact hit id {} missing in row {qi} (false-negative)",
                        i + 1
                    );
                }
            }
        }
    }

    #[test]
    fn snapshot_eviction_and_partial_upsert() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0004, &order);
        let qs = random_queries(13, 16);
        let mut st = SwaBridgeState::new();
        let (off, row, _ovf, rc) = run(&mut st, 1, &f, &qs, 1 << 16);
        assert_eq!(rc, 0);
        // snapshot без чётных id ⇒ они эвиктятся, строки сузятся
        let mut f2 = Feed { ids: Vec::new(), box_: Vec::new() };
        for i in 0..N {
            if (f.ids[i] & 1) == 0 {
                f2.ids.push(f.ids[i]);
                f2.box_.extend_from_slice(&f.box_[i * 4..i * 4 + 4]);
            }
        }
        let mut off2 = vec![0i32; off.len()];
        let mut row2 = vec![0i64; 1 << 16];
        let mut ovf2 = vec![0i32; qs.len() / 4];
        let rc2 = st.epoch(1, 1, f2.ids.len(), &f2.ids, &f2.box_, qs.len() / 4, &qs, &mut off2, &mut row2, &mut ovf2);
        assert_eq!(rc2, 0);
        assert_eq!(st.soa.len(), N / 2, "snapshot must evict absent ids");
        for qi in 0..qs.len() / 4 {
            let full = row_set(&off, &row, qi);
            let half = row_set(&off2, &row2, qi);
            assert!(half.is_subset(&full), "eviction must shrink rows");
            assert!(half.iter().all(|&id| (id & 1) == 0), "odd ids evicted");
        }
        // partial upsert (snapshot=0) НОВОГО id — ничего не эвиктится
        let st_before = st.soa.len();
        let mut f3 = Feed { ids: vec![0xABCD_i64], box_: vec![-400.0, -399.0, -400.0, -399.0] };
        let _ = &mut f3;
        let mut off3 = vec![0i32; off.len()];
        let mut row3 = vec![0i64; 1 << 16];
        let mut ovf3 = vec![0i32; qs.len() / 4];
        let rc3 = st.epoch(1, 0, 1, &f3.ids, &f3.box_, qs.len() / 4, &qs, &mut off3, &mut row3, &mut ovf3);
        assert_eq!(rc3, 0);
        assert_eq!(st.soa.len(), st_before + 1, "partial upsert appends");
        assert!(st.slot_of(0xABCD).is_some(), "new id registered");
        // КОНТРАКТ (не позиция слота): append-бокс у левого края вызывает
        // insertion-проход и легальный self-heal re-sort — слот = его
        // ОТСОРТИРОВАННАЯ позиция. Проверяем сквозную доступность: запрос ровно
        // по новому боксу обязан вернуть его id (superset, строка живая).
        let qs_new = vec![-400.0f32, -399.0, -400.0, -399.0];
        let mut off4 = vec![0i32; 2];
        let mut row4 = vec![0i64; 1 << 16];
        let mut ovf4 = vec![0i32; 1];
        assert_eq!(st.epoch(1, 0, 0, &[], &[], 1, &qs_new, &mut off4, &mut row4, &mut ovf4), 0);
        assert_eq!(ovf4[0], 0);
        assert!(
            row4[off4[0] as usize..off4[1] as usize].contains(&0xABCD),
            "upserted box must be reachable by its own query"
        );
    }

    #[test]
    fn self_heal_restores_sortedness_after_storm() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0005, &order);
        let qs = random_queries(17, 8);
        let mut st = SwaBridgeState::new();
        let (_, _, _, rc) = run(&mut st, 1, &f, &qs, 1 << 16);
        assert_eq!(rc, 0);
        // штампед: ВСЕ боксы телепортированы в новые случайные места —
        // инверсии огромны ⇒ self-heal обязан отсортировать заново
        let mut rng = Rng(0x5704);
        let mut f2 = Feed { ids: Vec::with_capacity(N), box_: Vec::with_capacity(N * 4) };
        for i in 0..N {
            let cx = rng.f32_in(-512.0, 512.0);
            let cz = rng.f32_in(-512.0, 512.0);
            let hw = rng.f32_in(0.3, MAX_SPAN / 2.0);
            let hd = rng.f32_in(0.3, MAX_SPAN / 2.0);
            f2.ids.push(f.ids[i]);
            f2.box_.extend_from_slice(&[cx - hw, cx + hw, cz - hd, cz + hd]);
        }
        let mut off = vec![0i32; qs.len() / 4 + 1];
        let mut row = vec![0i64; 1 << 16];
        let mut ovf = vec![0i32; qs.len() / 4];
        let rc2 = st.epoch(1, 1, f2.ids.len(), &f2.ids, &f2.box_, qs.len() / 4, &qs, &mut off, &mut row, &mut ovf);
        assert_eq!(rc2, 0);
        for i in 1..st.soa.len() {
            assert!(
                f32_ordered_key(st.soa.min_x[i - 1]) <= f32_ordered_key(st.soa.min_x[i]),
                "plane not sorted after self-heal at {i}"
            );
        }
        assert!(st.heals >= 1, "storm must trigger self-heal, heals={}", st.heals);
    }

    #[test]
    fn overflow_flag_sets_and_rc_stays_ok() {
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0006, &order);
        // 4 кверии на ВСЁ поле: каждая требует N строк, кап 2 ⇒ ovf гарантирован
        // (случайные кверии могут легально попадать в пустое пространство —
        // ovf обязан ставить только дефицит строк, не пустоту окна)
        let qs = vec![-512.0f32, 512.0, -512.0, 512.0, -512.0, 512.0, -512.0, 512.0, -512.0, 512.0, -512.0, 512.0, -512.0, 512.0, -512.0, 512.0];
        let mut st = SwaBridgeState::new();
        // крошечный row-буфер: все кверии переполнятся, rc всё равно 0
        let (off, _row, ovf, rc) = run(&mut st, 1, &f, &qs, 2);
        assert_eq!(rc, 0);
        for (qi, &o) in ovf.iter().enumerate() {
            assert_eq!(o, 1, "query {qi} must flag overflow with 2-row cap");
            assert!(off[qi] <= off[qi + 1], "offsets stay monotone");
        }
        // rc/flags когерентны: offsets последний = столько, сколько влезло
        assert!(off[off.len() - 1] <= 8);
    }

    #[test]
    fn err_paths_fail_closed() {
        let mut st = SwaBridgeState::new();
        let f = Feed { ids: vec![1], box_: vec![0.0, 1.0, 0.0, 1.0] };
        let qs = vec![0.0f32, 1.0, 0.0, 1.0];
        let mut off = vec![0i32; 2];
        let mut row = vec![0i64; 64];
        let mut ovf = vec![0i32; 1];
        // чужой mode/snapshot
        assert_eq!(st.epoch(2, 1, 1, &f.ids, &f.box_, 1, &qs, &mut off, &mut row, &mut ovf), ERR_RANGE);
        assert_eq!(st.epoch(1, 5, 1, &f.ids, &f.box_, 1, &qs, &mut off, &mut row, &mut ovf), ERR_RANGE);
        // короткие массивы
        assert_eq!(st.epoch(1, 1, 2, &f.ids, &f.box_, 1, &qs, &mut off, &mut row, &mut ovf), ERR_RANGE);
        assert_eq!(st.epoch(1, 1, 1, &f.ids, &f.box_[..3], 1, &qs, &mut off, &mut row, &mut ovf), ERR_RANGE);
        assert_eq!(st.epoch(1, 1, 1, &f.ids, &f.box_, 1, &qs, &mut off[..0], &mut row, &mut ovf), ERR_RANGE);
        assert_eq!(st.epoch(1, 1, 1, &f.ids, &f.box_, 1, &qs, &mut off, &mut row, &mut ovf[..0]), ERR_RANGE);
        // id вне u32
        let f_bad = Feed { ids: vec![(u32::MAX as i64) + 1], box_: vec![0.0, 1.0, 0.0, 1.0] };
        assert_eq!(st.epoch(1, 1, 1, &f_bad.ids, &f_bad.box_, 1, &qs, &mut off, &mut row, &mut ovf), ERR_RANGE);
        // NaN-кверия: ключ NaN максимален ⇒ окно пусто, rc=0, строк нет
        let qs_nan = vec![f32::NAN, 1.0, 0.0, 1.0];
        let (o, r, v, rc_nan) = run(&mut st, 1, &f, &qs_nan, 64);
        assert_eq!(rc_nan, 0);
        assert_eq!(o[1], 0);
        assert_eq!(v[0], 0);
        let _ = r;
        // NaN-фид (partial upsert, snapshot=0 — sane-бокс обязан ЖИТЬ рядом с
        // NaN-лейном): лейн fail-closed (никогда не матчится), superset не рвётся
        let f_nan = Feed { ids: vec![9], box_: vec![f32::NAN, f32::NAN, 0.0, 1.0] };
        let mut off4 = vec![0i32; 2];
        let mut row4 = vec![0i64; 64];
        let mut ovf4 = vec![0i32; 1];
        assert_eq!(st.epoch(1, 0, 1, &f_nan.ids, &f_nan.box_, 1, &qs, &mut off4, &mut row4, &mut ovf4), 0);
        assert_eq!(row4[0] as i64, 1, "sane box id 1 still matches");
        assert!(!row4[..off4[1] as usize].contains(&9), "NaN lane must not match");
    }

    #[test]
    fn strict_lever_gate_and_in_place_drift() {
        // STRICT-eq: пустой/чужой флаг = dormant
        assert!(enabled_with("cmp458_swar"));
        assert!(!enabled_with(""));
        assert!(!enabled_with(" "));
        assert!(!enabled_with("cmp401_soa"));
        assert!(!enabled_with("cmp458_swarx"));
        assert!(!enabled_with("cmp458_SWAR"));
        // in-place дрейф: тот же порядок фида тик-за-тиком — вплоть до
        // порога без full-resort; результаты идентичны rebuild-пути
        let order: Vec<usize> = (0..N).collect();
        let f = build_feed(0xFEED_0007, &order);
        let qs = random_queries(23, 16);
        let mut inplace = SwaBridgeState::new();
        let (_off1, _row1, ovf1, rc1) = run(&mut inplace, 1, &f, &qs, 1 << 16);
        assert_eq!(rc1, 0);
        // старт из НЕотсортированного фида легально лечится одним full-resort —
        // порог дрейфа меряем ОТНОСИТЕЛЬНО этого базлайна
        let heals_before = inplace.heals;
        assert_eq!(heals_before, 1, "initial unsorted feed = one self-heal");
        // дрейф всех боксов на ±1.5 (малые инверсии)
        let mut rng = Rng(29);
        let mut f2 = Feed { ids: Vec::with_capacity(N), box_: Vec::with_capacity(N * 4) };
        for i in 0..N {
            let d = rng.f32_in(-1.5, 1.5);
            f2.ids.push(f.ids[i]);
            f2.box_.extend_from_slice(&[
                f.box_[i * 4] + d,
                f.box_[i * 4 + 1] + d,
                f.box_[i * 4 + 2],
                f.box_[i * 4 + 3],
            ]);
        }
        let (off2, row2, ovf2, rc2) = run(&mut inplace, 1, &f2, &qs, 1 << 16);
        assert_eq!(rc2, 0);
        assert_eq!(ovf1, ovf2);
        for qi in 0..qs.len() / 4 {
            // superset-направление: новый (дрейфованный) сет может отличаться
            // только по честному предикату — сравниваем с exact-брутфорсом
            let (qmnx, qmnz) = (qs[qi * 4] - MARGIN, qs[qi * 4 + 2] - MARGIN);
            let (qmxx, qmxz) = (qs[qi * 4 + 1] + MARGIN, qs[qi * 4 + 3] + MARGIN);
            let have = row_set(&off2, &row2, qi);
            for i in 0..N {
                let (mnx, mxx) = (f2.box_[i * 4], f2.box_[i * 4 + 1]);
                let (mnz, mxz) = (f2.box_[i * 4 + 2], f2.box_[i * 4 + 3]);
                if mnx <= qmxx && mxx >= qmnx && mnz <= qmxz && mxz >= qmnz {
                    assert!(have.contains(&(i as i64 + 1)), "drift superset broken q{qi}");
                }
            }
        }
        assert_eq!(inplace.heals, heals_before, "±1.5 drift must stay under self-heal threshold");
        // reset возвращает плоскость в холодное состояние, телеметрия живёт
        let calls_before = inplace.calls;
        inplace.reset();
        assert_eq!(inplace.soa.len(), 0);
        assert_eq!(inplace.calls, calls_before);
    }
}
