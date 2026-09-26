//! alloc_budget — бюджет-лиджер young-gen: carrier-скелет (TASK-463-48,
//! интерфейс ×462-51 / LEDGER-51 ROUND-462, канон young 118→108-112).
//!
//! Принцип «бюджет — учёт, не сборщик»: лиджер НЕ аллоцирует, НЕ пулит,
//! НЕ collects — только consume/verify. Пассы инвариантов ×462-51 (G1-G6
//! неизменны): G3 alloc-дельта плоскости ≤ +0.3%; G4 fail-closed: over_budget
//! => плоскость переключается на scratch/arena (zero-alloc путь, parity-оракул
//! бит-в-байт); G6 суб-шум: самостоятельный norm-гейт НЕ применяется.
//!
//! Контракт поверхностей:
//!   0 heap-аллокаций в этом модуле (только Copy-стек + Cell<u64> — код-ревью
//!   гейт; ни Vec/String/Box/format!/collect) ;
//!   0 JNI-crossings: consume живёт целиком в Rust-плоскости; java-сторона
//!   видит только run-env дамп РАЗ/ОКНО через [`PlaneBudget::run_env_fields`];
//!   fail-closed: budget == 0 или потеря данных => over_budget() == true
//!   (плоскость обязана уйти в zero-alloc путь), consume насыщается
//!   (saturating), паники запрещены.
//!
//! РЕФАКТОРИНГ-ЛИЦЕНЗИЯ 14f: скелет ваниль-семантичен — НИЧЕГО не
//! переключает. P24/P25/P27 — заглушки (мёртвый код без вайринга); живые
//! вайринги — TASK-463-74 (nav-плоскость), TASK-463-79 (P27 send-плоскость),
//! TASK-463-95 (P24/P25 noise GC-debt relief).
//!
//! V2 (TASK-464-54): чистый fail-closed ARM-предикат [`kernel_arm_allowed`] —
//! допуск kernel-arm-пути по бюджету плоскости (бюджет — учёт, не сборщик;
//! ARM — привилегия, не право). План интеграции — 6 сайтов kernel-arm-пути
//! (LEDGER-54, ROUND-464/LAB-STAGE):
//!   S1 src/kernel_policy.rs:728  registration_promotion — chokepoint:
//!      fail-closed бюджет плоскости пары => WIN-символ НЕ выдаётся;
//!   S2 src/kernel_policy.rs:736  promotion_armed — armed == false, когда
//!      ВСЕ плоскости пар в over_budget;
//!   S3 src/kernel_policy.rs:743  log_armed_pairs — boot-дамп добавляет
//!      (budget, consumed, over) тройку плоскости пары;
//!   S4 src/promote_wire.rs:80    selftest_if_armed — self-test пропускает
//!      пару при fail-closed бюджете (без FAIL-шума в selfTest-каноне);
//!   S5 src/lib.rs:443            boot-маркер: armed-строка печатает
//!      бюджет-состояние (0 аллокаций, РАЗ/бут);
//!   S6 src/lib.rs:752/762        JNI-регистрация: conservative-fallback
//!      (безопасное направление) старше promotion, promotion старше бюджета.
//! До вайрингов 74/79/95 предикат СПЯЩИЙ (dormant-invisible канон 14f).

// dormant-invisible канон: весь модуль — скелет до вайрингов 74/79/95,
// dead_code здесь = спящий гейт, не рычаг (урок ×425/×458-F1).
#![allow(dead_code)]

/// Идентификаторы плоскостей (канон ×462-51: broadphase / inside / items /
/// nav / noise-octave / 2d-router / send-scratch; 0 = незарезервирован).
pub mod plane {
    pub const NONE: u16 = 0;
    /// P24 octave scratch-pool (noise, GC-debt relief carrier).
    pub const P24_NOISE_OCTAVE: u16 = 24;
    /// P25 2D-router cache (noise, GC-debt relief carrier).
    pub const P25_2D_ROUTER: u16 = 25;
    /// P27 send scratch-arena (chunk-send, GC-debt carrier).
    pub const P27_SEND_SCRATCH: u16 = 27;
}

/// Бюджет-лиджер одной плоскости (carrier-скелет ×462-51, сигнатуры 1:1).
///
/// Реализации обязаны быть `Sync`-нейтральными по аллокациям: методы не
/// аллоцируют, не блокируют и не пересекают JNI. `plane` в consume/
/// over_budget — маркер вызова (плоскость-инициатор), реализация сверяет его
/// со своим [`PlaneBudget::plane_id`] и fail-closed отбрасывает чужие маркеры
/// (чужой вызов не открывает бюджет).
pub trait PlaneBudget {
    /// Идентификатор плоскости (см. [`plane`]).
    fn plane_id(&self) -> u16;
    /// B_i: байтовый бюджет плоскости на тик (fail-closed зажим, НЕ GC).
    /// 0 = бюджет не выдан => over_budget() == true всегда (zero-alloc путь).
    fn budget_bytes_per_tick(&self) -> u32;
    /// Метроном: +байты за тик (saturating; без аллокаций).
    fn consume(&self, plane: u16, bytes: u64);
    /// true => плоскость переключается на scratch/arena (fail-closed).
    fn over_budget(&self, plane: u16) -> bool;
    /// Экспорт run-env РАЗ/ОКНО: плоский кортеж чисел (0 аллокаций; формат
    /// строки — забота экспортёра run-env, не лиджера).
    /// Порядок: (plane_id, budget_bytes_per_tick, consumed_this_tick, over_budget).
    fn run_env_fields(&self) -> (u16, u32, u64, bool) {
        (
            self.plane_id(),
            self.budget_bytes_per_tick(),
            0,
            self.over_budget(self.plane_id()),
        )
    }
    /// Граница метронома: сброс consumed на новом тике (0 аллокаций).
    fn tick_reset(&self);
}

/// Заглушка P24 octave scratch-pool (noise). Ваниль-семантика: без вайринга
/// consume/over_budget не вызываются из горячего пути (мёртвый код скелета).
pub struct P24NoiseOctaveBudget {
    budget: u32,
    consumed: core::cell::Cell<u64>,
}

impl P24NoiseOctaveBudget {
    pub const fn new(budget_bytes_per_tick: u32) -> Self {
        Self {
            budget: budget_bytes_per_tick,
            consumed: core::cell::Cell::new(0),
        }
    }
}

impl PlaneBudget for P24NoiseOctaveBudget {
    fn plane_id(&self) -> u16 {
        plane::P24_NOISE_OCTAVE
    }
    fn budget_bytes_per_tick(&self) -> u32 {
        self.budget
    }
    fn consume(&self, plane: u16, bytes: u64) {
        if plane == self.plane_id() {
            self.consumed.set(self.consumed.get().saturating_add(bytes));
        }
    }
    fn over_budget(&self, plane: u16) -> bool {
        plane == self.plane_id() && self.is_over()
    }
    fn run_env_fields(&self) -> (u16, u32, u64, bool) {
        (
            self.plane_id(),
            self.budget,
            self.consumed.get(),
            self.is_over(),
        )
    }
    fn tick_reset(&self) {
        self.consumed.set(0);
    }
}

impl P24NoiseOctaveBudget {
    fn is_over(&self) -> bool {
        // fail-closed: бюджет 0 (не выдан) => перерасход немедленно.
        self.budget == 0 || self.consumed.get() > self.budget as u64
    }
}

/// Заглушка P25 2D-router cache (noise). Семантика идентична P24.
pub struct P25Router2DBudget {
    budget: u32,
    consumed: core::cell::Cell<u64>,
}

impl P25Router2DBudget {
    pub const fn new(budget_bytes_per_tick: u32) -> Self {
        Self {
            budget: budget_bytes_per_tick,
            consumed: core::cell::Cell::new(0),
        }
    }
    fn is_over(&self) -> bool {
        self.budget == 0 || self.consumed.get() > self.budget as u64
    }
}

impl PlaneBudget for P25Router2DBudget {
    fn plane_id(&self) -> u16 {
        plane::P25_2D_ROUTER
    }
    fn budget_bytes_per_tick(&self) -> u32 {
        self.budget
    }
    fn consume(&self, plane: u16, bytes: u64) {
        if plane == self.plane_id() {
            self.consumed.set(self.consumed.get().saturating_add(bytes));
        }
    }
    fn over_budget(&self, plane: u16) -> bool {
        plane == self.plane_id() && self.is_over()
    }
    fn run_env_fields(&self) -> (u16, u32, u64, bool) {
        (
            self.plane_id(),
            self.budget,
            self.consumed.get(),
            self.is_over(),
        )
    }
    fn tick_reset(&self) {
        self.consumed.set(0);
    }
}

/// Заглушка P27 send scratch-arena (chunk-send). Семантика идентична P24.
pub struct P27SendScratchBudget {
    budget: u32,
    consumed: core::cell::Cell<u64>,
}

impl P27SendScratchBudget {
    pub const fn new(budget_bytes_per_tick: u32) -> Self {
        Self {
            budget: budget_bytes_per_tick,
            consumed: core::cell::Cell::new(0),
        }
    }
    fn is_over(&self) -> bool {
        self.budget == 0 || self.consumed.get() > self.budget as u64
    }
}

impl PlaneBudget for P27SendScratchBudget {
    fn plane_id(&self) -> u16 {
        plane::P27_SEND_SCRATCH
    }
    fn budget_bytes_per_tick(&self) -> u32 {
        self.budget
    }
    fn consume(&self, plane: u16, bytes: u64) {
        if plane == self.plane_id() {
            self.consumed.set(self.consumed.get().saturating_add(bytes));
        }
    }
    fn over_budget(&self, plane: u16) -> bool {
        plane == self.plane_id() && self.is_over()
    }
    fn run_env_fields(&self) -> (u16, u32, u64, bool) {
        (
            self.plane_id(),
            self.budget,
            self.consumed.get(),
            self.is_over(),
        )
    }
    fn tick_reset(&self) {
        self.consumed.set(0);
    }
}

/// V2 (TASK-464-54) fail-closed ARM-предикат kernel-пути: чистая функция,
/// 0 аллокаций, 0 JNI, паника невозможна (Copy-аргументы). Допуск arm-пути
/// плоскости: бюджет ВЫДАН (≠0) И метроном тика в пределах бюджета. Любая
/// деградация входа (бюджет 0 = не выдан, потеря данных) => false: плоскость
/// остаётся на zero-alloc scratch/arena пути (fail-closed, default-deny).
/// Инвариант обратен [`PlaneBudget::over_budget`] (тот же is_over-предикат).
#[inline]
pub fn kernel_arm_allowed(budget_bytes_per_tick: u32, consumed_this_tick: u64) -> bool {
    budget_bytes_per_tick != 0 && consumed_this_tick <= budget_bytes_per_tick as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_ids_canon() {
        assert_eq!(plane::NONE, 0);
        assert_eq!(plane::P24_NOISE_OCTAVE, 24);
        assert_eq!(plane::P25_2D_ROUTER, 25);
        assert_eq!(plane::P27_SEND_SCRATCH, 27);
    }

    #[test]
    fn stub_ids_and_budget() {
        let p24 = P24NoiseOctaveBudget::new(4096);
        let p25 = P25Router2DBudget::new(0);
        let p27 = P27SendScratchBudget::new(1 << 20);
        assert_eq!(p24.plane_id(), 24);
        assert_eq!(p25.plane_id(), 25);
        assert_eq!(p27.plane_id(), 27);
        assert_eq!(p24.budget_bytes_per_tick(), 4096);
        assert_eq!(p25.budget_bytes_per_tick(), 0);
        assert_eq!(p27.budget_bytes_per_tick(), 1 << 20);
    }

    #[test]
    fn fail_closed_zero_budget_always_over() {
        let p25 = P25Router2DBudget::new(0);
        // Бюджет не выдан => over_budget == true даже при 0 потреблённых байтах.
        assert!(p25.over_budget(p25.plane_id()));
        p25.consume(p25.plane_id(), 0);
        assert!(p25.over_budget(p25.plane_id()));
    }

    #[test]
    fn consume_accumulates_and_gate_flips() {
        let p27 = P27SendScratchBudget::new(100);
        assert!(!p27.over_budget(p27.plane_id()));
        p27.consume(p27.plane_id(), 100); // ровно в бюджет — ещё не перерасход
        assert!(!p27.over_budget(p27.plane_id()));
        p27.consume(p27.plane_id(), 1); // 101 > 100
        assert!(p27.over_budget(p27.plane_id()));
    }

    #[test]
    fn foreign_plane_marker_rejected() {
        let p24 = P24NoiseOctaveBudget::new(10);
        p24.consume(plane::P27_SEND_SCRATCH, 999); // чужой маркер не учитывается
        assert_eq!(p24.run_env_fields().2, 0);
        assert!(!p24.over_budget(plane::P27_SEND_SCRATCH));
        p24.consume(plane::P24_NOISE_OCTAVE, 11);
        assert!(p24.over_budget(plane::P24_NOISE_OCTAVE));
    }

    #[test]
    fn consume_saturates_no_panic() {
        let p24 = P24NoiseOctaveBudget::new(8);
        p24.consume(plane::P24_NOISE_OCTAVE, u64::MAX);
        p24.consume(plane::P24_NOISE_OCTAVE, u64::MAX); // saturating, без panic/overflow
        assert!(p24.over_budget(plane::P24_NOISE_OCTAVE));
    }

    #[test]
    fn tick_reset_clears_metronome() {
        let p27 = P27SendScratchBudget::new(4);
        p27.consume(plane::P27_SEND_SCRATCH, 16);
        assert!(p27.over_budget(plane::P27_SEND_SCRATCH));
        p27.tick_reset();
        assert!(!p27.over_budget(plane::P27_SEND_SCRATCH));
        assert_eq!(p27.run_env_fields().2, 0);
    }

    // --- v2 (TASK-464-54): kernel-arm fail-closed гейт ----------------------

    #[test]
    fn kernel_arm_gate_fail_closed_on_zero_budget() {
        // Бюджет не выдан => arm-путь закрыт даже при нулевом потреблении.
        assert!(!kernel_arm_allowed(0, 0));
        assert!(!kernel_arm_allowed(0, 1));
        assert!(!kernel_arm_allowed(0, u64::MAX));
    }

    #[test]
    fn kernel_arm_gate_admits_boundary_and_blocks_over() {
        assert!(kernel_arm_allowed(100, 0));
        assert!(kernel_arm_allowed(100, 100)); // ровно в бюджет — допуск
        assert!(!kernel_arm_allowed(100, 101)); // 101 > 100 — fail-closed
        assert!(!kernel_arm_allowed(1, 2));
    }

    #[test]
    fn kernel_arm_gate_saturating_inputs_no_panic_flip() {
        // Насыщенный/максимальный метроном не разворачивает предикат в допуск.
        assert!(!kernel_arm_allowed(u32::MAX, u64::MAX));
        assert!(kernel_arm_allowed(u32::MAX, u32::MAX as u64));
    }

    #[test]
    fn kernel_arm_gate_agrees_with_plane_over_budget() {
        // Предикат консистентен с is_over-инвариантом плоскости (P25-стаб).
        let p25 = P25Router2DBudget::new(64);
        p25.consume(plane::P25_2D_ROUTER, 64);
        assert_eq!(
            kernel_arm_allowed(64, 64),
            !p25.over_budget(plane::P25_2D_ROUTER)
        );
        p25.consume(plane::P25_2D_ROUTER, 1);
        assert_eq!(
            kernel_arm_allowed(64, 65),
            !p25.over_budget(plane::P25_2D_ROUTER)
        );
        assert!(!kernel_arm_allowed(0, 0)); // нулевой бюджет = всегда закрыто
    }

    #[test]
    fn run_env_fields_shape_once_per_window() {
        // Экспорт run-env РАЗ/ОКНО: плоский кортеж (id, budget, consumed, over).
        let p24 = P24NoiseOctaveBudget::new(512);
        p24.consume(plane::P24_NOISE_OCTAVE, 64);
        let (id, budget, consumed, over) = p24.run_env_fields();
        assert_eq!((id, budget, consumed, over), (24, 512, 64, false));
    }
}
