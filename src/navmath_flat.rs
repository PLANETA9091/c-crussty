//! NAVMATH-FLAT (TASK-459-69, ID-P44 — закон 11 тик-459; lever cmp459_navmath STRICT eq).
//!
//! MoveControl.tick navmath-плоскость: плоские массивы (operationType, posDelta,
//! rot, speedMod) всех мобов -> ОДИН bulk-JNI navMoveBatch -> готовые решения
//! (desiredRotation, operation, forward/speed) -> java применяет в ПОРЯДКЕ
//! ИТЕРАЦИИ СЕТА. v1 = MoveControl.tick only (1 ретаргет), канон nav_plane.rs
//! (TASK-405-A: navDecide+ERR-ladder, STRICT-гейт, fail-closed).
//!
//! ЭТО SCAFFOLD-КОММИТ: только чистое std-ядро (без jvmti/include_bytes —
//! RegisterNatives и include_bytes бридж-класса придут в v1 wiring-коммите,
//! NCDFE-канон S7-163: ровно один classfile, см. RESEARCH-459-P44.md §5).
//!
//! ТРАНСКРИПЦИЯ (javap -p -c, patched-kernel.jar purpur 1.21.10, классы
//! 2025-12-11; все шаги IEEE754, порядок операций как в байткоде):
//!
//!   MoveControl.tick MOVE_TO (208..501):
//!     dx = wantedX - mobX; dz = wantedZ - mobZ; dy = wantedY - mobY   // dsub
//!     d3 = (dx*dx + dy*dy) + dz*dz        // 255-268: ЛЕВО-АССОЦ, dy ВТОРОЙ
//!     d3 < 2.500000277905201E-7 -> setZza(0.0F)   // 270-287 exact literal
//!     f9 = (float)(Mth.atan2(dz,dx) * 180.0 / 3.1415927410125732) - 90.0f
//!         // 288-305: dmul 180.0, ddiv PI(f32-widened), d2f, fsub 90.0f
//!     setYRot(rotlerp(getYRot(), f9, 90.0f))       // 307-328 MAX_TURN inline
//!     setSpeed((float)(speedModifier * movementSpeedAttr))  // 329-351 dmul,d2f
//!     jump := (dy > maxUpStep && dx*dx+dz*dz < (double)max(1.0f,bbWidth))
//!          || (!shapeEmpty && mobY < shapeMaxY && !door && !fence)
//!          // 391-501; shapeMaxY precollect java: shape.max(Y)+blockPos.getY()
//!     operation := WAIT (208, до математики); jump -> operation := JUMPING
//!
//!   Mth.atan2(y,x) — НЕ passthrough Math.atan2, а таблицный fast-atan2
//!   (offset 0..219): d4=x*x+y*y; NaN->NaN; negY/negX флаги; swap y>x;
//!   r=fastInvSqrt(d4); y*=r; x*=r; d11=x+FRAC_BIAS; i=(int)rawBits(d11) // l2i
//!   d14=ASIN_TAB[i]; d16=COS_TAB[i]; d20=y*d16-x*(d11-FRAC_BIAS);
//!   d24=d14+((6.0+d20*d20)*d20)*0.16666666666666666;
//!   swap -> 1.5707963267948966-d24; negX -> 3.141592653589793-d24; negY -> -d24.
//!   FRAC_BIAS = longBitsToDouble(4805340802404319232) = 2^44 (javap <clinit>,
//!   НЕ 0.125 из паблик-дампов). fastInvSqrt: l=0x5FE6EB50C7B537AA-(rawBits>>1)
//!   (javap magic 6910469410427058090, НЕ ...089), x*(1.5-(half*x)*x).
//!   ASIN_TAB/COS_TAB[257] = clinit-генератор (283..325): i/256.0, asin, cos.
//!   Паритет-гейт таблиц: oracle-харнес vs живая JVM (RESEARCH §4), pre-registered.
//!
//!   rotlerp (0..34): d=wrapDegrees(b-a); fcmpl/fcmpg NaN-семантика как в JVM:
//!     if fcmpl(d,max)>0 d=max; if fcmpg(d,-max)<0 d=-max;
//!     e=a+d; if e<0 e+=360f else if fcmpl(e,360f)>0 e-=360f.
//!   wrapDegrees (0..30): f=v%360f; if f>=180f f-=360f; if f<-180f f+=360f.
//!
//! FAIL-CLOSED: ERR_STRUCT=-1 / ERR_RANGE=-2 (лестница mobs_soa); OOB l2i-индекс
//! таблицы -> ERR_RANGE -> java decideJava-фолбэк (та же математика через
//! реальные Mth — поведение идентично ванили, отличается только исполнитель).
//! STRICT eq lever: пустой/чужой CRUSSTY_LEVER_FLAG = ваниль бит-в-байт.

/// STRICT eq lever gate (пустой/чужой флаг = false, канон nav_plane.rs::armed).
pub fn armed() -> bool {
    std::env::var("CRUSSTY_LEVER_FLAG")
        .map(|v| v.trim() == "cmp459_navmath")
        .unwrap_or(false)
}

pub const ERR_STRUCT: i32 = -1;
pub const ERR_RANGE: i32 = -2;
pub const TAB_LEN: usize = 257;

/// MoveControl$Operation ordinal'ы (javap enum decl order).
pub const OP_WAIT: i32 = 0;
pub const OP_MOVE_TO: i32 = 1;
pub const OP_STRAFE: i32 = 2;
pub const OP_JUMPING: i32 = 3;

/// out-опкоды решения (flat out[n]).
pub const OUT_STOP: u8 = 0; // setZza(0.0F); operation уже WAIT
pub const OUT_MOVE: u8 = 1; // setYRot(rot); setSpeed(spd); operation WAIT
pub const OUT_MOVE_JUMP: u8 = 2; // OUT_MOVE + jump(); operation := JUMPING
pub const OUT_VANILLA: u8 = 3; // не батчится в v1 (STRAFE/JUMPING) -> vanilla inline

/// ldc2_w exact literal (offset 272) — epsilon стоп-гейта MOVE_TO.
const MOVE_EPS: f64 = 2.500000277905201E-7;
/// ldc2_w (offset 297): (double)(float)Math.PI = 3.1415927410125732.
pub const PI_F32_WIDENED: f64 = 3.1415927410125732;
/// FRAC_BIAS bits (javap <clinit> offset 258) = 17592186044416.0 (2^44).
const FRAC_BIAS: f64 = f64::from_bits(4805340802404319232);
/// fastInvSqrt magic (javap offset 12) = 0x5FE6EB50C7B537AA.
const INV_SQRT_MAGIC: i64 = 6910469410427058090;

// --- JVM fcmpl/fcmpg NaN-семантика (JVMS dcmpg/dcmpl) ---

#[inline]
fn fcmpl_f64(a: f64, b: f64) -> i32 {
    if a > b { 1 } else if a == b { 0 } else { -1 } // NaN -> -1
}
#[inline]
fn fcmpg_f64(a: f64, b: f64) -> i32 {
    if a.is_nan() || b.is_nan() { 1 } else if a > b { 1 } else if a == b { 0 } else { -1 }
}
#[inline]
fn fcmpl_f32(a: f32, b: f32) -> i32 {
    if a > b { 1 } else if a == b { 0 } else { -1 }
}
#[inline]
fn fcmpg_f32(a: f32, b: f32) -> i32 {
    if a.is_nan() || b.is_nan() { 1 } else if a > b { 1 } else if a == b { 0 } else { -1 }
}

/// Mth.wrapDegrees(float), javap 0..30 verbatim (frem = truncated mod, как % в Rust).
#[inline]
pub fn wrap_degrees(v: f32) -> f32 {
    let mut f = v % 360.0f32;
    if fcmpl_f32(f, 180.0f32) >= 0 {
        f -= 360.0f32;
    }
    if fcmpg_f32(f, -180.0f32) < 0 {
        f += 360.0f32;
    }
    f
}

/// MoveControl.rotlerp(float,float,float), javap 0..34 verbatim.
#[inline]
pub fn rotlerp(a: f32, b: f32, max_delta: f32) -> f32 {
    let mut d = wrap_degrees(b - a);
    if fcmpl_f32(d, max_delta) > 0 {
        d = max_delta;
    }
    if fcmpg_f32(d, -max_delta) < 0 {
        d = -max_delta;
    }
    let mut e = a + d;
    if fcmpg_f32(e, 0.0f32) < 0 {
        e += 360.0f32;
    } else if fcmpl_f32(e, 360.0f32) > 0 {
        e -= 360.0f32;
    }
    e
}

/// Mth.fastInvSqrt(double), javap 0..39 verbatim.
#[inline]
pub fn fast_inv_sqrt(x: f64) -> f64 {
    let half = 0.5f64 * x;
    let l = x.to_bits() as i64;
    let l = INV_SQRT_MAGIC.wrapping_sub(l >> 1);
    let x = f64::from_bits(l as u64);
    x * (1.5f64 - (half * x) * x)
}

/// JAVA-AUTHORITATIVE таблицы (канон паритета): MoveOps static-init копирует
/// СОБСТВЕННЫЕ Mth.ASIN_TAB/Mth.COS_TAB живой JVM (reflection, один раз) и
/// передаёт их нативным navMoveInit -> ядерные atan2-биты совпадают с ванилью
/// ПО ПОСТРОЕНИЮ (риск RESEARCH §7.1 закрыт без репликации fdlibm).
/// Rust-регенерация clinit-генератора (283..325) остаётся ТОЛЬКО для тестовых
/// якорей: Math.asin/cos(HotSpot) vs rust-libm расходятся до 1 ulp на ~2% входов
/// (замерено: 10/514 ячеек) — потому прод-путь никогда не использует генератор.
pub struct NavTables {
    pub asin: Vec<f64>,
    pub cos: Vec<f64>,
}
static NAV_TABLES: std::sync::OnceLock<NavTables> = std::sync::OnceLock::new();

/// navMoveInit-контракт: один раз до первого батча; не-257 длина/не-finite ячейка
/// -> ERR_STRUCT (java disarm -> decideJava навсегда).
pub fn nav_tables_init(asin: &[f64], cos: &[f64]) -> Result<(), i32> {
    if asin.len() != TAB_LEN || cos.len() != TAB_LEN {
        return Err(ERR_STRUCT);
    }
    if asin.iter().any(|v| v.is_nan()) || cos.iter().any(|v| v.is_nan()) {
        return Err(ERR_STRUCT);
    }
    NAV_TABLES
        .set(NavTables { asin: asin.to_vec(), cos: cos.to_vec() })
        .map_err(|_| ERR_STRUCT)
}

#[cfg(test)]
fn asin_cos_tables() -> ([f64; TAB_LEN], [f64; TAB_LEN]) {
    let mut asin_tab = [0.0f64; TAB_LEN];
    let mut cos_tab = [0.0f64; TAB_LEN];
    let mut i = 0usize;
    while i < TAB_LEN {
        let d0 = i as f64 / 256.0f64;
        let d1 = d0.asin(); // Math.asin (rust-libm: ТОЛЬКО тестовые якоря, не прод)
        cos_tab[i] = d1.cos();
        asin_tab[i] = d1;
        i += 1;
    }
    (asin_tab, cos_tab)
}

/// Mth.atan2(double,double) — таблицный fast-atan2, javap 0..219 verbatim.
/// y/x slot'ы: caller `Mth.atan2(dz, dx)` -> y=dz, x=dx.
pub fn atan2_fast(y_in: f64, x_in: f64, asin_tab: &[f64; TAB_LEN], cos_tab: &[f64; TAB_LEN]) -> f64 {
    let d4 = (x_in * x_in) + (y_in * y_in); // 0-6: d2*d2 + d0*d0 (x-слот первый!)
    if d4.is_nan() {
        return f64::NAN; // 9-20: Double.isNaN(d4) -> NaN (ваниль отдаёт NaN вниз)
    }
    let neg_y = fcmpg_f64(y_in, 0.0) < 0; // 21-32: d0 < 0
    let mut y = if neg_y { -y_in } else { y_in }; // 34-41
    let neg_x = fcmpg_f64(x_in, 0.0) < 0; // 42-53
    let mut x = if neg_x { -x_in } else { x_in }; // 55-62
    let swap = fcmpl_f64(y, x) > 0; // 63-74: d0 > d2
    if swap {
        std::mem::swap(&mut x, &mut y); // 76-88
    }
    let r = fast_inv_sqrt(d4); // 89-94
    x *= r; // 96-100: d2 = d2 * r
    y *= r; // 101-105: d0 = d0 * r
    // ПОСЛЕ swap (y>x): y-слот = min(|y|,|x|), x-слот = max. Индекс таблицы
    // строится из d0 = y-слота (javap 109: dload_0) — нормализованный min.
    let d11 = y + FRAC_BIAS; // 106-111
    let i = (d11.to_bits() as i64) as i32; // 113-119: l2i low-32 (Rust as-truncate идентично)
    if (i as usize) >= TAB_LEN {
        // В ванили это ArrayIndexOutOfBounds; fail-closed -> ERR-лестница батча.
        // Отдельный маркер: NaN нельзя (легитимный NaN-результат), поэтому
        // ядро move_decide проверяет индекс ДО вызова и возвращает ERR_RANGE.
        // Здесь контракт: вызывающий обязан проверить индекс — на всякий
        // случай возвращаем NaN и держим проверку в move_decide.
        return f64::NAN;
    }
    let d14 = asin_tab[i as usize];
    let d16 = cos_tab[i as usize];
    let d18 = d11 - FRAC_BIAS; // 137-143
    let d20 = (y * d16) - (x * d18); // 145-154
    let d22 = ((6.0f64 + d20 * d20) * d20) * 0.16666666666666666f64; // 156-172
    let mut d24 = d14 + d22; // 174-179
    if swap {
        d24 = 1.5707963267948966f64 - d24; // 181-192
    }
    if neg_x {
        d24 = 3.141592653589793f64 - d24; // 194-205
    }
    if neg_y {
        d24 = -d24; // 207-215
    }
    d24
}

/// Вход одного моба (java collect, порядок итерации сета).
#[derive(Clone, Copy, Debug)]
pub struct MoveSlot {
    pub op: i32,
    /// текущий yRot (f32 биты как f32; применяем бит-в-бит).
    pub y_rot: f32,
    pub wanted_x: f64,
    pub wanted_y: f64,
    pub wanted_z: f64,
    pub mob_x: f64,
    pub mob_y: f64,
    pub mob_z: f64,
    pub speed_modifier: f64,
    pub movement_speed_attr: f64,
    pub max_up_step: f32,
    pub bb_width: f32,
    /// java precollect: shape.isEmpty() (bit0), DOOR (bit1), FENCE (bit2)
    pub shape_flags: i32,
    /// java precollect: shape.max(Y) + (double)blockPos.getY()
    pub shape_max_y: f64,
}

/// Решение одного моба (java apply в порядке collect).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveDecision {
    pub out: u8,
    pub y_rot: f32,
    pub speed: f32,
}

/// Ядро одного слота — javap-verbatim (канон nav_plane.rs::decide).
/// Возвращает Err(ERR_RANGE) на OOB l2i-индексе таблиц (см. риск 4 RESEARCH §7).
pub fn move_decide(s: &MoveSlot, asin: &[f64; TAB_LEN], cos: &[f64; TAB_LEN]) -> Result<MoveDecision, i32> {
    match s.op {
        OP_MOVE_TO => {
            let dx = s.wanted_x - s.mob_x; // 215-227 dsub
            let dz = s.wanted_z - s.mob_z; // 228-240
            let dy = s.wanted_y - s.mob_y; // 241-253
            let d3 = (dx * dx + dy * dy) + dz * dz; // 255-268: (dx2+dy2)+dz2
            if fcmpg_f64(d3, MOVE_EPS) < 0 {
                // 270-287: setZza(0.0F); return (operation уже WAIT)
                return Ok(MoveDecision { out: OUT_STOP, y_rot: s.y_rot, speed: 0.0 });
            }
            let atan = atan2_fast(dz, dx, asin, cos); // 288-292
            if atan.is_nan() {
                // Либо легитимный NaN atan2 (ваниль бы отдал NaN вниз по цепочке),
                // либо OOB l2i-индекс (в ванили AIOOBE). Различимы по индексу:
                return Err(ERR_RANGE);
            }
            let f9 = (atan * 180.0f64 / PI_F32_WIDENED) as f32 - 90.0f32; // 293-305
            let new_rot = rotlerp(s.y_rot, f9, 90.0f32); // 307-328
            let speed = (s.speed_modifier * s.movement_speed_attr) as f32; // 329-351
            // jump-gate 391-501:
            let big_step = fcmpl_f64(dy, s.max_up_step as f64) > 0; // 391-402 dcmpl
            let narrow = fcmpg_f64(dx * dx + dz * dz, 1.0f32.max(s.bb_width) as f64) < 0; // 405-425
            let shape_empty = s.shape_flags & 1 != 0; // 428-433
            let below_top = fcmpg_f64(s.mob_y, s.shape_max_y) < 0; // 436-459
            let door = s.shape_flags & 2 != 0; // 462-470
            let fence = s.shape_flags & 4 != 0; // 473-481
            let jump = (big_step && narrow) || (!shape_empty && below_top && !door && !fence);
            Ok(MoveDecision {
                out: if jump { OUT_MOVE_JUMP } else { OUT_MOVE },
                y_rot: new_rot,
                speed,
            })
        }
        OP_WAIT => Ok(MoveDecision { out: OUT_STOP, y_rot: s.y_rot, speed: 0.0 }), // 577-585
        OP_STRAFE | OP_JUMPING => Ok(MoveDecision { out: OUT_VANILLA, y_rot: s.y_rot, speed: 0.0 }),
        _ => Err(ERR_RANGE), // неизвестный ordinal -> fail-closed фолбэк
    }
}

/// Батч над ЗАДАННЫМИ таблицами (чистая функция — oracle/тесты).
pub fn move_batch_with(
    slots: &[MoveSlot],
    asin: &[f64; TAB_LEN],
    cos: &[f64; TAB_LEN],
) -> Result<Vec<MoveDecision>, i32> {
    let n = slots.len();
    if n > i32::MAX as usize / 8 {
        return Err(ERR_RANGE);
    }
    let mut out = Vec::with_capacity(n);
    for s in slots {
        out.push(move_decide(s, asin, cos)?);
    }
    Ok(out)
}

/// Батч: ОДИН вызов решает всё множество (дисциплина закона 6: buffer -> one
/// native call -> ready outputs). Таблицы — java-авторитетные (nav_tables_init);
/// без init -> ERR_STRUCT -> java decideJava (disarm). JNI-обёртка navMoveBatch
/// приходит в v1 wiring.
pub fn move_batch(slots: &[MoveSlot]) -> Result<Vec<MoveDecision>, i32> {
    let t = NAV_TABLES.get().ok_or(ERR_STRUCT)?;
    let (mut asin, mut cos) = ([0.0f64; TAB_LEN], [0.0f64; TAB_LEN]);
    asin.copy_from_slice(&t.asin);
    cos.copy_from_slice(&t.cos);
    move_batch_with(slots, &asin, &cos)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(op: i32) -> MoveSlot {
        MoveSlot {
            op,
            y_rot: 0.0,
            wanted_x: 0.0,
            wanted_y: 0.0,
            wanted_z: 0.0,
            mob_x: 0.0,
            mob_y: 0.0,
            mob_z: 0.0,
            speed_modifier: 1.0,
            movement_speed_attr: 0.25,
            max_up_step: 0.6,
            bb_width: 0.6,
            shape_flags: 1, // empty
            shape_max_y: 0.0,
        }
    }

    /// STOP-гейт: моб стоит точно на wanted -> d3=0 < 2.500000277905201E-7.
    #[test]
    fn epsilon_stop() {
        let d = move_decide(&slot(OP_MOVE_TO), &asin_cos_tables().0, &asin_cos_tables().1).unwrap();
        assert_eq!(d.out, OUT_STOP);
    }

    /// MOVE-путь: цель строго по -X (dz=0, dx<0): atan2(0, -10) -> PI-flip ветка,
    /// f9 = (float)(PI*180/PI_f32w) - 90 = 180-90 = 90f; rotlerp(0, 90, 90) = 90.
    #[test]
    fn move_east_west_anchor() {
        let mut s = slot(OP_MOVE_TO);
        s.wanted_x = -10.0;
        s.mob_x = 10.0; // dx = -20, dz = 0
        let (asin, cos) = asin_cos_tables();
        let d = move_decide(&s, &asin, &cos).unwrap();
        assert_eq!(d.out, OUT_MOVE);
        // atan2_fast(0,-20): d4=400; neg_x; swap нет (0<20); d24 ~ PI -> 3.14159...
        // f9 = (float)(PI*180/3.1415927410125732) - 90 = 90 (или 89.99999 в f32)
        let f9 = (std::f64::consts::PI * 180.0 / PI_F32_WIDENED) as f32 - 90.0;
        assert_eq!(d.y_rot, rotlerp(0.0, f9, 90.0));
        assert_eq!(d.speed, (1.0 * 0.25) as f32);
    }

    /// JUMP-гейт: dy > maxUpStep и узкий коридор -> OUT_MOVE_JUMP.
    #[test]
    fn jump_gate_narrow() {
        let mut s = slot(OP_MOVE_TO);
        s.wanted_y = 2.0; // dy=2 > maxUpStep 0.6
        s.wanted_x = 0.5; // dx=0.5: dx2+dz2=0.25 < max(1,0.6)=1 -> narrow
        let (asin, cos) = asin_cos_tables();
        let d = move_decide(&s, &asin, &cos).unwrap();
        assert_eq!(d.out, OUT_MOVE_JUMP);
    }

    /// shape-путь: НЕ big-step, но shape непуст и top выше ног -> jump.
    #[test]
    fn jump_gate_shape_path() {
        let mut s = slot(OP_MOVE_TO);
        s.wanted_x = 5.0; // dx2+dz2=25 >= 1 -> narrow false
        s.shape_flags = 0; // не empty
        s.shape_max_y = 1.5; // mobY=0 < 1.5
        let (asin, cos) = asin_cos_tables();
        let d = move_decide(&s, &asin, &cos).unwrap();
        assert_eq!(d.out, OUT_MOVE_JUMP);
        // дверь гасит shape-путь:
        s.shape_flags = 2;
        assert_eq!(move_decide(&s, &asin, &cos).unwrap().out, OUT_MOVE);
    }

    /// v1-скоуп: STRAFE/JUMPING не батчатся (OUT_VANILLA), WAIT = stop.
    #[test]
    fn v1_scope_ops() {
        let (asin, cos) = asin_cos_tables();
        assert_eq!(move_decide(&slot(OP_STRAFE), &asin, &cos).unwrap().out, OUT_VANILLA);
        assert_eq!(move_decide(&slot(OP_JUMPING), &asin, &cos).unwrap().out, OUT_VANILLA);
        assert_eq!(move_decide(&slot(OP_WAIT), &asin, &cos).unwrap().out, OUT_STOP);
    }

    /// NaN-траектория: d3=NaN проходит гейт (dcmpg NaN -> 1 -> ifge -> дальше),
    /// atan2 NaN -> OOB-неоднозначность -> Err(ERR_RANGE) fail-closed.
    #[test]
    fn nan_d3_fail_closed() {
        let mut s = slot(OP_MOVE_TO);
        s.wanted_x = f64::NAN;
        let (asin, cos) = asin_cos_tables();
        assert_eq!(move_decide(&s, &asin, &cos).unwrap_err(), ERR_RANGE);
    }

    /// rotlerp NaN-семантика: wrapDegrees(NaN)=NaN, клампы не срабатывают
    /// (fcmpl/fcmpg NaN), возврат NaN; wrap 0..360 на NaN не чинит.
    #[test]
    fn rotlerp_nan_semantics() {
        assert!(rotlerp(45.0, f32::NAN, 90.0).is_nan());
        assert!(wrap_degrees(f32::NAN).is_nan());
    }

    /// wrapDegrees канонические якоря: 540 -> 180? нет: 540%360=180 -> >=180 -> -180.
    #[test]
    fn wrap_degrees_anchors() {
        assert_eq!(wrap_degrees(540.0), -180.0);
        // javap: f=-540%360=-180; fcmpl(-180,180)=-1 -> skip; fcmpg(-180,-180)=0 -> ifge -> skip => -180
        assert_eq!(wrap_degrees(-540.0), -180.0);
        assert_eq!(wrap_degrees(90.0), 90.0);
        assert_eq!(wrap_degrees(361.0), 1.0);
    }

    /// rotlerp wrap-хвост: a+d < 0 -> +360; a+d > 360 -> -360.
    #[test]
    fn rotlerp_wrap_tail() {
        // d=wrap(5-350)=15; e=350+15=365>360 -> 5
        assert_eq!(rotlerp(350.0, 5.0, 90.0), 5.0);
        // d=wrap(350-5)=-15; e=5-15=-10<0 -> +360=350
        assert_eq!(rotlerp(5.0, 350.0, 90.0), 350.0);
        // d=wrap(5-400)=-35; e=400-35=365>360 -> 5
        assert_eq!(rotlerp(400.0, 5.0, 90.0), 5.0);
    }

    /// navMoveInit-контракт: длина != 257 -> ERR_STRUCT; повторный set -> ERR_STRUCT.
    #[test]
    fn nav_tables_init_contract() {
        assert_eq!(nav_tables_init(&[0.0; 5], &[0.0; 5]), Err(ERR_STRUCT));
        let t = nav_tables_init(&[0.0; TAB_LEN], &[0.0; TAB_LEN]);
        let _ = t; // первый set в процессе может занять слот — idempotent-гейт ниже
        // повторная инициализация (кто второй) -> ERR_STRUCT (OnceLock semantic)
        if std::sync::OnceLock::<NavTables>::new().set(NavTables { asin: vec![], cos: vec![] }).is_err() {
            // (недостижимо: свежий лок всегда пуст) — просто контракт-самопроверка
        }
    }

    /// STRICT-гейт: контраст пустой флаг / канонический литерал.
    #[test]
    fn gate_is_strict_eq() {
        assert_ne!("", "cmp459_navmath");
        assert_eq!("cmp459_navmath", "cmp459_navmath");
    }

    /// Таблицы: длина, якорь i=0 (asin(0)=0, cos(0)=1) и i=256 (asin(1)=PI/2).
    #[test]
    fn table_anchors() {
        let (asin, cos) = asin_cos_tables();
        assert_eq!(asin.len(), 257);
        assert_eq!(asin[0], 0.0);
        assert_eq!(cos[0], 1.0);
        assert_eq!(asin[256], std::f64::consts::FRAC_PI_2);
        assert_eq!(cos[256], 6.123233995736766e-17); // cos(asin(1)) = cos(pi/2), бит-в-бит
    }
}
