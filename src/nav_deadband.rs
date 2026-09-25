//! NAV-DEADBAND (TASK-459-71, ID-P46 — закон 11 тик-459; lever cmp459_p46 STRICT eq).
//!
//! PathNavigation tick-deadband плоскость: стак/timeout-детекция doStuckDetection
//! (раз/тик/моб) -> батч: ОДИН bulk-JNI navStuckBatch решает stuck/timeout-флаги
//! ВСЕХ навигирующих мобов тика (входы = delay-счётчики, node-дистанции — javap
//! ground truth RESEARCH-459-P46.md §1.3), java применяет stop()/resetStuckTimeout()
//! в ПОРЯДКЕ ИТЕРАЦИИ navigatingMobs. Канон nav_plane.rs (TASK-405-A navDecide +
//! ERR-ladder, STRICT-гейт, fail-closed) и скаффолд-канон navmath_flat.rs (P44).
//!
//! ЭТО SCAFFOLD-КОММИТ: только чистое std-ядро (без jvmti/include_bytes —
//! RegisterNatives, include_bytes бридж-класса и ретаргет приходят в v1
//! wiring-коммите, NCDFE-канон: класс целиком в kernel loader, ZERO nested).
//! Пустой/чужой CRUSSTY_LEVER_FLAG = ваниль бит-в-байт по построению.
//!
//! ТРАНСКРИПЦИЯ (javap -p -c, patched-kernel.jar purpur 1.21.10, классы
//! 2025-12-11; все шаги IEEE754, порядок операций как в байткоде):
//!
//!   doStuckDetection(Vec3 pos) — часть (a) stuck-check (offsets 0..104):
//!     gate: tick - lastStuckCheck > 100        // isub, bipush 100, if_icmple
//!     f  = getSpeed() > 1.0f ? getSpeed() : getSpeed()*getSpeed()
//!                                              // fcmpl,iflt: NaN -> else-ветвь
//!     f1 = f * 100.0f * 0.25f                  // fmul x2 лево-ассоц (STUCK_
//!                                              //  THRESHOLD_DISTANCE_FACTOR инлайн)
//!     d  = pos.distanceToSqr(lastStuckCheckPos) = (dx*dx + dy*dy) + dz*dz
//!     d < (double)(f1*f1) -> isStuck=true; stop()  // fmul,f2d,dcmpg
//!     else isStuck=false
//!     lastStuckCheck=tick; lastStuckCheckPos=pos
//!
//!   doStuckDetection — часть (b) timeout-check (offsets 107..257; БЕЗ 100-тикового
//!   гейта — каждый тик каждого навигирующего моба, горячая часть):
//!     if (path != null && !path.isDone()):
//!       node = path.getNextNodePos(); g = level.getGameTime()
//!       if (node.equals(timeoutCachedNode))    // Vec3i.equals
//!         timeoutTimer += g - lastTimeoutCheck // lsub,ladd
//!       else:
//!         timeoutCachedNode = node
//!         d = pos.distanceTo(Vec3.atBottomCenterOf(node))  // sqrt((dx*dx+dy*dy)+dz*dz)
//!         timeoutLimit = getSpeed() > 0.0f ? d / (double)getSpeed() * 20.0d : 0.0d
//!                                              // fcmpl,ifle: NaN -> 0.0; ddiv,dmul
//!       if (timeoutLimit > 0.0d && (double)timeoutTimer > timeoutLimit * 3.0d)
//!         timeoutPath()                        // dcmpl x2,ifle: NaN -> не стоп
//!       lastTimeoutCheck = g
//!
//!   Vec3.atBottomCenterOf(Vec3i) = (x+0.5d, y, z+0.5d) — БЕЗ +0.5 по Y.
//!
//! FAIL-CLOSED: ERR_STRUCT=-1 / ERR_RANGE=-2 (лестница mobs_soa/navmath_flat);
//! любой rc<0/throwable -> one-shot disarm latch -> java decideJava (та же
//! математика — поведение идентично ванили, отличается только исполнитель).

/// STRICT eq lever gate (пустой/чужой флаг = false, канон nav_plane.rs::armed).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp459_p46")
        .unwrap_or(false)
}

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;

/// javap-инлайны констант ядра (компилятор-инлайн, НЕ видны в Field-листе):
/// STUCK_CHECK_INTERVAL, STUCK_THRESHOLD_DISTANCE_FACTOR, MAX_TIME_RECOMPUTE.
pub const STUCK_CHECK_INTERVAL: i32 = 100;
pub const STUCK_THRESHOLD_DISTANCE_FACTOR: f32 = 0.25f32;
pub const MAX_TIME_RECOMPUTE: i64 = 20;

/// out-флаги решения слота (apply-диспетчер v1; бит-маска).
pub const OUT_NONE: u8 = 0; // состояние без изменений (deadband-ветвь)
pub const OUT_STUCK_STOP: u8 = 1; // isStuck=true + stop(); lastStuckCheck/Pos обновить
pub const OUT_STUCK_CLEAR: u8 = 2; // isStuck=false; lastStuckCheck/Pos обновить
pub const OUT_TIMEOUT_STOP: u8 = 4; // timeoutPath(): resetStuckTimeout + stop()
/// OUT_TIMEOUT_STOP комбинируется с OUT_STUCK_* (байт-маска).

/// Вход одного моба (java collect, порядок итерации navigatingMobs).
#[derive(Clone, Copy, Debug)]
pub struct StuckSlot {
    /// PathNavigation.tick (delay-счётчик #1, вход карточки).
    pub tick: i32,
    pub last_stuck_check: i32,
    /// getSpeed() снапшот (f32; fcmpl-семантика ядра, NaN-края в тестах).
    pub speed: f32,
    /// tempMobPos — снапшот getTempMobPos() этого тика.
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub last_stuck_check_pos_x: f64,
    pub last_stuck_check_pos_y: f64,
    pub last_stuck_check_pos_z: f64,
    /// path != null && !path.isDone() (иначе timeout-часть пропускается).
    pub path_active: bool,
    /// path.getNextNodePos() (Vec3i, i32 ×3).
    pub node_x: i32,
    pub node_y: i32,
    pub node_z: i32,
    pub timeout_cached_node_x: i32,
    pub timeout_cached_node_y: i32,
    pub timeout_cached_node_z: i32,
    /// delay-счётчики #2/#3 + порог (вход карточки).
    pub timeout_timer: i64,
    pub last_timeout_check: i64,
    pub timeout_limit: f64,
    /// общий для батча gameTime тика (java собирает 1 раз).
    pub game_time: i64,
}

/// Решение одного моба (java apply в порядке collect).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StuckDecision {
    pub out: u8,
    /// новые delay-счётчики/порог (java пишет поля только при apply).
    pub new_timeout_timer: i64,
    pub new_last_timeout_check: i64,
    pub new_timeout_limit: f64,
}

// --- JVM fcmpl/fcmpg NaN-семантика (JVMS fcmpl/fcmpg, канон navmath_flat.rs) ---

#[inline]
fn fcmpl_f32(a: f32, b: f32) -> i32 {
    if a > b { 1 } else if a == b { 0 } else { -1 } // NaN -> -1
}
#[inline]
fn dcmpg_f64(a: f64, b: f64) -> i32 {
    if a.is_nan() || b.is_nan() { 1 } else if a > b { 1 } else if a == b { 0 } else { -1 }
}
#[inline]
fn dcmpl_f64(a: f64, b: f64) -> i32 {
    if a > b { 1 } else if a == b { 0 } else { -1 } // NaN -> -1 (javap 227/244 dcmpl)
}

/// Vec3.distanceToSqr — лево-ассоц (d0*d0 + d1*d1) + d2*d2 (javap канона).
#[inline]
fn distance_sqr(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    let dz = az - bz;
    (dx * dx + dy * dy) + dz * dz
}

/// Vec3.atBottomCenterOf(Vec3i): (x+0.5, y, z+0.5) — Y БЕЗ +0.5 (javap).
#[inline]
fn at_bottom_center_x(x: i32) -> f64 {
    x as f64 + 0.5
}
#[inline]
fn at_bottom_center_z(z: i32) -> f64 {
    z as f64 + 0.5
}

/// Ядро одного слота — javap-verbatim §1.3 (канон nav_plane.rs::decide).
/// rc<0 (ERR_*) = java-фолбэк на весь батч (disarm-латч java-сторы).
pub fn stuck_decide(s: &StuckSlot) -> Result<StuckDecision, i32> {
    let mut out = OUT_NONE;
    // (a) stuck-check: gate = tick - lastStuckCheck > 100 (isub: Java int wrap,
    // signed compare; Rust wrapping_sub + signed сравнение идентично).
    if s.tick.wrapping_sub(s.last_stuck_check) > STUCK_CHECK_INTERVAL {
        // f = getSpeed() > 1.0f ? getSpeed() : getSpeed()*getSpeed()
        //   fcmpl: NaN -> -1 -> iflt -> else-ветвь speed*speed (NaN).
        let sp = s.speed;
        let f = if fcmpl_f32(sp, 1.0f32) < 0 { sp * sp } else { sp };
        // f1 = f * 100.0f * 0.25f (лево-ассоц fmul).
        let f1 = f * 100.0f32 * STUCK_THRESHOLD_DISTANCE_FACTOR;
        // d < (double)(f1*f1): dcmpg (NaN -> 1 -> ветвь else isStuck=false).
        let d = distance_sqr(
            s.pos_x,
            s.pos_y,
            s.pos_z,
            s.last_stuck_check_pos_x,
            s.last_stuck_check_pos_y,
            s.last_stuck_check_pos_z,
        );
        if dcmpg_f64(d, (f1 * f1) as f64) < 0 {
            out |= OUT_STUCK_STOP;
        } else {
            out |= OUT_STUCK_CLEAR;
        }
    }
    // (b) timeout-check: без гейта, при активном path.
    let mut new_timer = s.timeout_timer;
    let mut new_last_timeout_check = s.last_timeout_check;
    let mut new_limit = s.timeout_limit;
    if s.path_active {
        let same_node = s.node_x == s.timeout_cached_node_x
            && s.node_y == s.timeout_cached_node_y
            && s.node_z == s.timeout_cached_node_z;
        if same_node {
            // timeoutTimer += g - lastTimeoutCheck (lsub,ladd, Java long wrap).
            new_timer = s
                .timeout_timer
                .wrapping_add(s.game_time.wrapping_sub(s.last_timeout_check));
        } else {
            // d = pos.distanceTo(atBottomCenterOf(node)) — sqrt-домен.
            let d = distance_sqr(
                s.pos_x,
                s.pos_y,
                s.pos_z,
                at_bottom_center_x(s.node_x),
                s.node_y as f64,
                at_bottom_center_z(s.node_z),
            )
            .sqrt();
            // timeoutLimit = getSpeed() > 0.0f ? d / (double)getSpeed() * 20.0d : 0.0d
            // fcmpl,ifle: NaN/<=0 -> 0.0d; ddiv,dmul лево-ассоц.
            new_limit = if fcmpl_f32(s.speed, 0.0f32) <= 0 {
                0.0f64
            } else {
                (d / s.speed as f64) * 20.0f64
            };
        }
        // if (timeoutLimit > 0.0d && (double)timeoutTimer > timeoutLimit * 3.0d)
        //   timeoutPath() — javap 227/244 dcmpl: NaN -> -1 -> ifle -> skip.
        if dcmpl_f64(new_limit, 0.0) > 0 && dcmpl_f64(new_timer as f64, new_limit * 3.0) > 0 {
            out |= OUT_TIMEOUT_STOP;
        }
        new_last_timeout_check = s.game_time; // 252-254: пишется ВСЕГДА при active path
    }
    Ok(StuckDecision {
        out,
        new_timeout_timer: new_timer,
        new_last_timeout_check,
        new_timeout_limit: new_limit,
    })
}

/// Батч-ядро v1 (в v1 wiring станет navStuckBatch JNI-телом): rc 0 ok,
/// ERR_STRUCT на пустом батче (java disarm -> decideJava навсегда).
pub fn stuck_batch(slots: &[StuckSlot]) -> Result<Vec<StuckDecision>, i32> {
    if slots.is_empty() {
        return Err(ERR_STRUCT);
    }
    let mut out = Vec::with_capacity(slots.len());
    for s in slots {
        out.push(stuck_decide(s)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(tick: i32, last_check: i32, speed: f32, drift: f64) -> StuckSlot {
        StuckSlot {
            tick,
            last_stuck_check: last_check,
            speed,
            pos_x: drift,
            pos_y: 64.0,
            pos_z: 0.0,
            last_stuck_check_pos_x: 0.0,
            last_stuck_check_pos_y: 64.0,
            last_stuck_check_pos_z: 0.0,
            path_active: false,
            node_x: 0,
            node_y: 64,
            node_z: 0,
            timeout_cached_node_x: 0,
            timeout_cached_node_y: 64,
            timeout_cached_node_z: 0,
            timeout_timer: 0,
            last_timeout_check: 0,
            timeout_limit: 0.0,
            game_time: 1000,
        }
    }

    #[test]
    fn lever_dormant_by_default() {
        // Пустой/чужой флаг -> ваниль (скаффолд-коммит обязан быть dormant).
        std::env::remove_var("CRUSSTY_LEVER_FLAG");
        assert!(!armed());
    }

    #[test]
    fn deadband_gate_100_strict() {
        // tick - lastStuckCheck == 100 -> НЕТ stuck-решения (if_icmple: >100).
        let d = stuck_decide(&slot(1100, 1000, 1.0, 0.0)).unwrap();
        assert_eq!(d.out & (OUT_STUCK_STOP | OUT_STUCK_CLEAR), 0);
        // == 101 -> ветвь stuck (дрейф 0 < f1*f1).
        let d = stuck_decide(&slot(1101, 1000, 1.0, 0.0)).unwrap();
        assert_eq!(d.out & OUT_STUCK_STOP, OUT_STUCK_STOP);
    }

    #[test]
    fn stuck_stop_vs_clear() {
        // speed 1.0 -> f=1.0, f1=25.0, порог 625.0; дрейф 10.0 -> d=100.0 < 625 -> STOP.
        let d = stuck_decide(&slot(1200, 1000, 1.0, 10.0)).unwrap();
        assert_eq!(d.out & OUT_STUCK_STOP, OUT_STUCK_STOP);
        // дрейф 30.0 -> d=900.0 > 625 -> CLEAR (ваниль сбрасывает isStuck).
        let d = stuck_decide(&slot(1200, 1000, 1.0, 30.0)).unwrap();
        assert_eq!(d.out & OUT_STUCK_CLEAR, OUT_STUCK_CLEAR);
    }

    #[test]
    fn fcmpl_nan_speed_takes_squared_branch() {
        // NaN: fcmpl(NaN,1.0) = -1 -> else-ветвь speed*speed = NaN, порог NaN,
        // dcmpg(d, NaN) = 1 -> CLEAR (не STOP) — JVM-семантика, не Rust-if.
        let d = stuck_decide(&slot(1200, 1000, f32::NAN, 0.0)).unwrap();
        assert_eq!(d.out & OUT_STUCK_CLEAR, OUT_STUCK_CLEAR);
        assert_eq!(d.out & OUT_STUCK_STOP, 0);
    }

    #[test]
    fn timeout_accumulates_only_on_same_node() {
        let mut s = slot(500, 0, 0.3, 0.0);
        s.path_active = true;
        s.timeout_timer = 100;
        s.last_timeout_check = 990;
        s.game_time = 1000; // тот же node: timer = 100 + (1000-990) = 110
        let d = stuck_decide(&s).unwrap();
        assert_eq!(d.new_timeout_timer, 110);
        assert_eq!(d.new_last_timeout_check, 1000);
        // другой node: лимит = d/speed*20, таймер НЕ копится.
        let mut s2 = s;
        s2.node_x = 5; // atBottomCenterOf = (5.5, 64, 0.5)
        s2.timeout_timer = 0;
        let d2 = stuck_decide(&s2).unwrap();
        assert_eq!(d2.new_timeout_timer, 0);
        // ядро делит на (double)speed — f32 0.3 -> 0.30000001192092896 (не f64-литерал)
        let expected = (distance_sqr(0.0, 64.0, 0.0, 5.5, 64.0, 0.5).sqrt() / (0.3f32 as f64)) * 20.0;
        assert!((d2.new_timeout_limit - expected).abs() < 1e-12);
    }

    #[test]
    fn timeout_stop_gate_and_zero_speed() {
        let mut s = slot(500, 0, 0.0, 0.0); // speed 0: fcmpl <= 0 -> limit 0.0
        s.path_active = true;
        let d = stuck_decide(&s).unwrap();
        assert_eq!(d.out & OUT_TIMEOUT_STOP, 0);
        assert_eq!(d.new_timeout_limit, 0.0);
        // timeoutTimer > limit*3 -> OUT_TIMEOUT_STOP (reset+stop в java-apply).
        let mut s2 = slot(500, 0, 0.3, 0.0);
        s2.path_active = true;
        s2.node_x = 100; // большой d -> limit большой; таймер больше порога:
        s2.timeout_timer = 10_000_000;
        let d2 = stuck_decide(&s2).unwrap();
        assert_eq!(d2.out & OUT_TIMEOUT_STOP, OUT_TIMEOUT_STOP);
    }

    #[test]
    fn batch_err_ladder_and_order() {
        assert_eq!(stuck_batch(&[]).unwrap_err(), ERR_STRUCT);
        let slots = vec![slot(1101, 1000, 1.0, 0.0), slot(1100, 1000, 1.0, 0.0)];
        let ds = stuck_batch(&slots).unwrap();
        assert_eq!(ds.len(), 2);
        assert_eq!(ds[0].out & OUT_STUCK_STOP, OUT_STUCK_STOP); // порядок collect
        assert_eq!(ds[1].out & (OUT_STUCK_STOP | OUT_STUCK_CLEAR), 0);
    }
}
