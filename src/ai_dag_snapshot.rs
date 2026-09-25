//! AI-DAG SNAPSHOT scheduler — C-X2 scaffold (TASK-459-81, WILD round-459).
//!
//! Идея C-X2 (своё безумие, BLACKBOARD-строка 81): построить DAG зависимостей
//! моба goals→sensors→brain ОДИН раз за [`REBUILD_PERIOD`] тиков (snapshot-фаза),
//! тики между — ПЛОСКИЙ проход по замороженному топологическому порядку
//! ([`AiDagSnapshot::flat_pass`]) без перестроения структур и итератор-машинерии;
//! любая мутация топологии (add/remove goal, behavior add, activity switch)
//! бампает epoch-счётчик ([`AiDagSnapshot::invalidate`]) → следующий
//! window-тик перестраивает (epoch-инвалидация, dirty-flag).
//!
//! SLICES (мандат, RESEARCH-459-CX2.md §2/§3): nav/ai остаток 2.75-3.2пп
//! (после cmp406_aibatch окна 1/4: 14.16% → 3.31%, ABSORB chkmono457-14) +
//! brain 1-1.5пп (Brain.tick 0.54% @559 сэмплов + tickEachRunningBehavior 0.10%
//! @101 + startEachNonRunning-плечо; BOTTLENECKS_3.md).
//! Δ-прогноз при capture 35-55%/60-80%: +1.6..3.0пп, потолок +4.7пп.
//!
//! SOURCES (рисёрч, HTTP-verified): GDC Vault "Three States and a Plan: The AI
//! of F.E.A.R." (Orkin, GOAP: план строится редко — исполняется часто);
//! GameAIPro ch.6 "The Behavior Tree Starter Kit" (плоская память узлов +
//! стабильный порядок обхода); robohub.org BT-tick-семантика; gamedeveloper.com
//! Simpson (event-driven реакция на мутацию = epoch-инвалидация).
//!
//! COMPOSITION (cplug-sdk ordering contract, образец goal_selector.rs): эта
//! единица — DATA-плоскость будущей фазы вайринга; на serve-времени она
//! комбинируется В ПОЛУЧЕННЫХ байтах Mob.serverAiStep()V (4 сайта: GoalSelector
//! .tick()V ×2 @161/@183 чётные, tickRunningGoals(Z)V ×2 @113/@136 нечётные),
//! не трогая retarget'ы mobs_soa/mobs_sscan/goal_selector (stash-цепочка).
//!
//! STRICT DORMANT (закон гейтов ×406/×421): lever = env `CRUSSTY_LEVER_FLAG`
//! STRICT-eq [`LEVER`] ("cmp459_cx2"); пустой/чужой флаг = данные без вайринга,
//! byte-indistinguishable от ванили. FAIL-CLOSED: цикл в DAG → topo неполный
//! ([`AiDagSnapshot::topo_complete`] == false) → джава-гейт обязан звать ваниль
//! весь тик; epoch-drift между java-сторой и снапшотом → ваниль на тик.
//!
//! zero unsafe, std-only: модуль компилируется в cdylib без поведенческих
//! эффектов и держит контракт паритета порядка = (tier, insertion-индекс)
//! (insertion-порядок == ванильный порядок обхода множеств — G2 lockstep).

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env;

/// STRICT-гейт lever (env `CRUSSTY_LEVER_FLAG`), образец cmp406_aibatch/cmp421_brain.
pub const LEVER: &str = "cmp459_cx2";

/// Период snapshot-фазы в тиках (согласован с golden-window cmp406 N=4).
pub const REBUILD_PERIOD: u32 = 4;

/// Потолок узлов DAG (моб: goals+sensors+brain-behaviors много меньше этого).
pub const MAX_NODES: usize = 1 << 16;

/// Ярус DAG: сенсоры питают цели, цели питают brain-поведение.
/// Порядок ранга = порядок ванильного прохода serverAiStep:
/// sensing → targeting/goals → brain (goal_selector.rs javap-контракт).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tier {
    Sensor = 0,
    Goal = 1,
    Brain = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct DagNode {
    pub tier: Tier,
    pub dense_id: u32,
    /// Локальная эпоха узла: бампится при invalidate (мутация слота).
    pub epoch: u32,
    pub live: bool,
}

/// Snapshot-DAG планировщика mob-ai. Счётчики — u32 с wrap-around
/// (детерминизм важнее монотонности: wrap — тоже валидная эпоха при eq).
pub struct AiDagSnapshot {
    nodes: Vec<DagNode>,
    /// (from, to): from — поставщик (sensor), to — потребитель (goal/brain).
    edges: Vec<(u32, u32)>,
    /// Замороженный плоский порядок (Kahn по ключу (tier, insertion-индекс)).
    topo: Vec<u32>,
    /// Полный ли топосорт (false = цикл или мёртвый поставщик → FAIL-CLOSED).
    topo_complete: bool,
    /// Глобальная эпоха ПОСТРОЕНИЯ (растёт на rebuild).
    build_epoch: u32,
    /// Глобальная эпоха ИНВАЛИДАЦИИ (растёт на любой мутации топологии).
    invalid_epoch: u32,
    dirty: bool,
    ticks_since_build: u32,
}

impl AiDagSnapshot {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            topo: Vec::new(),
            topo_complete: true,
            build_epoch: 0,
            invalid_epoch: 0,
            dirty: true,
            ticks_since_build: REBUILD_PERIOD, // пустой DAG = сразу rebuild-кандидат
        }
    }

    /// Добавляет узел (дедуп по (tier, dense_id)); возвращает индекс.
    /// Мутация топологии → dirty + invalid_epoch++.
    pub fn add_node(&mut self, tier: Tier, dense_id: u32) -> Option<u32> {
        if self.nodes.len() >= MAX_NODES {
            return None;
        }
        if let Some(i) = self
            .nodes
            .iter()
            .position(|n| n.tier == tier && n.dense_id == dense_id)
        {
            if !self.nodes[i].live {
                self.nodes[i].live = true;
                self.mark_dirty();
            }
            return Some(i as u32);
        }
        let idx = self.nodes.len() as u32;
        self.nodes.push(DagNode {
            tier,
            dense_id,
            epoch: 0,
            live: true,
        });
        self.mark_dirty();
        Some(idx)
    }

    /// Ребро зависимости from→to (sensor→goal, goal→brain). Дедуп + bounds.
    pub fn add_edge(&mut self, from: u32, to: u32) -> bool {
        if from == to || from as usize >= self.nodes.len() || to as usize >= self.nodes.len() {
            return false;
        }
        if self.edges.contains(&(from, to)) {
            return true;
        }
        self.edges.push((from, to));
        self.mark_dirty();
        true
    }

    /// EPOCH-ИНВАЛИДАЦИЯ: мутация слота (goal add/remove, behavior change).
    /// Замороженный порядок ПРОДОЛЖАЕТ обслуживаться до окна rebuild —
    /// снапшот-семантика C-X2 (flat_pass читает frozen `topo`, не живой граф).
    pub fn invalidate(&mut self, idx: u32) -> bool {
        let n = match self.nodes.get_mut(idx as usize) {
            Some(n) => n,
            None => return false,
        };
        n.epoch = n.epoch.wrapping_add(1);
        n.live = false;
        self.invalid_epoch = self.invalid_epoch.wrapping_add(1);
        self.dirty = true;
        true
    }

    pub fn invalidate_all(&mut self) {
        for n in self.nodes.iter_mut() {
            n.epoch = n.epoch.wrapping_add(1);
            n.live = false;
        }
        self.invalid_epoch = self.invalid_epoch.wrapping_add(1);
        self.dirty = true;
    }

    /// Тик-гейт плоского прохода: true = пора перестроить (окно/грязь).
    pub fn note_tick(&mut self) -> bool {
        self.ticks_since_build = self.ticks_since_build.wrapping_add(1);
        self.should_rebuild()
    }

    pub fn should_rebuild(&self) -> bool {
        self.dirty || self.ticks_since_build >= REBUILD_PERIOD
    }

    /// SNAPSHOT-ФАЗА: детерминированный Kahn-топосорт по (tier, index).
    /// Замораживает `topo`; flat_pass между окнами ходит ТОЛЬКО по нему.
    /// Рёбра мёртвых узлов исключаются целиком (in_deg и relax — согласованные
    /// стороны одного фильтра live→live, иначе переполнение in_deg).
    pub fn rebuild(&mut self) {
        let live_edge = |e: &(u32, u32)| -> bool {
            self.nodes[e.0 as usize].live && self.nodes[e.1 as usize].live
        };
        let mut in_deg = vec![0u32; self.nodes.len()];
        for &e in self.edges.iter() {
            if live_edge(&e) {
                in_deg[e.1 as usize] += 1;
            }
        }
        let mut heap: BinaryHeap<Reverse<(u8, u32)>> = BinaryHeap::new();
        for (i, n) in self.nodes.iter().enumerate() {
            if in_deg[i] == 0 && n.live {
                heap.push(Reverse((n.tier as u8, i as u32)));
            }
        }
        let mut topo = Vec::with_capacity(self.nodes.len());
        let mut deg = in_deg;
        while let Some(Reverse((_, i))) = heap.pop() {
            let i = i as usize;
            topo.push(i as u32);
            // Исходящие рёбра: линейный скан (E мал; scaffold-контракт).
            for &e in self.edges.iter() {
                if e.0 as usize == i && live_edge(&e) {
                    let t = e.1 as usize;
                    deg[t] = deg[t].saturating_sub(1);
                    if deg[t] == 0 {
                        let n = &self.nodes[t];
                        if n.live {
                            heap.push(Reverse((n.tier as u8, t as u32)));
                        }
                    }
                }
            }
        }
        let live_total = self.nodes.iter().filter(|n| n.live).count();
        self.topo_complete = topo.len() == live_total;
        self.topo = topo;
        self.build_epoch = self.build_epoch.wrapping_add(1);
        self.dirty = false;
        self.ticks_since_build = 0;
    }

    /// ПЛОСКИЙ ПРОХОД по замороженному порядку — ровно то, что исполняется
    /// в тиках между snapshot-фазами. Порядок = замороженный, НЕ живой граф.
    pub fn flat_pass<F: FnMut(u32)>(&self, mut f: F) {
        for &i in self.topo.iter() {
            if self.nodes[i as usize].live {
                f(i);
            }
        }
    }

    pub fn topo_len(&self) -> usize {
        self.topo.len()
    }

    pub fn topo_complete(&self) -> bool {
        self.topo_complete
    }

    pub fn build_epoch(&self) -> u32 {
        self.build_epoch
    }

    pub fn invalid_epoch(&self) -> u32 {
        self.invalid_epoch
    }

    pub fn node_epoch(&self, idx: u32) -> Option<u32> {
        self.nodes.get(idx as usize).map(|n| n.epoch)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn mark_dirty(&mut self) {
        self.invalid_epoch = self.invalid_epoch.wrapping_add(1);
        self.dirty = true;
    }
}

impl Default for AiDagSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// STRICT-гейт: lever STRICT-eq; пустой/чужой флаг = false (ваниль бит-в-байт).
pub fn lever_enabled() -> bool {
    matches!(env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok(v) if v == LEVER)
}

/// Статус-маркер для ARM-гейта G1 (grep-цепочка defined→armed→PATCHED).
pub fn status() -> &'static str {
    if lever_enabled() {
        "armed-pending-wiring"
    } else {
        "dormant"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_triangle() -> AiDagSnapshot {
        let mut d = AiDagSnapshot::new();
        let s = d.add_node(Tier::Sensor, 7).unwrap();
        let g = d.add_node(Tier::Goal, 3).unwrap();
        let b = d.add_node(Tier::Brain, 5).unwrap();
        assert!(d.add_edge(s, g));
        assert!(d.add_edge(g, b));
        d.rebuild();
        d
    }

    #[test]
    fn topo_order_is_tier_deterministic() {
        let d = build_triangle();
        assert!(d.topo_complete());
        let mut order = Vec::new();
        d.flat_pass(|i| order.push(i));
        // Sensor < Goal < Brain независимо от порядка вставки dense_id.
        assert_eq!(order, vec![0, 1, 2]);
        assert_eq!(d.build_epoch(), 1);
    }

    #[test]
    fn rebuild_is_idempotent_stable() {
        let mut d = build_triangle();
        let e0 = d.build_epoch();
        d.rebuild();
        let mut o1 = Vec::new();
        d.flat_pass(|i| o1.push(i));
        d.rebuild();
        let mut o2 = Vec::new();
        d.flat_pass(|i| o2.push(i));
        assert_eq!(o1, o2);
        assert_eq!(d.build_epoch(), e0 + 2);
    }

    #[test]
    fn epoch_invalidation_marks_dirty_but_freezes_order() {
        let mut d = build_triangle();
        let inv_before = d.invalid_epoch();
        let mut before = Vec::new();
        d.flat_pass(|i| before.push(i));
        // Мутация: цель умирает → снапшот ГРЯЗНЫЙ, но порядок заморожен.
        assert!(d.invalidate(1));
        assert!(d.dirty);
        assert_eq!(d.invalid_epoch(), inv_before + 1);
        let mut after = Vec::new();
        d.flat_pass(|i| after.push(i));
        assert_eq!(before, after, "flat_pass обслуживает замороженный порядок");
        // Окно истекло → rebuild снимает мёртвый узел.
        assert!(d.note_tick());
        d.rebuild();
        let mut live = Vec::new();
        d.flat_pass(|i| live.push(i));
        assert_eq!(live, vec![0, 2]);
        assert_eq!(d.topo_len(), 2);
    }

    #[test]
    fn period_rebuild_every_n_ticks() {
        let mut d = build_triangle();
        d.rebuild();
        assert!(!d.should_rebuild());
        assert!(!d.note_tick());
        assert!(!d.note_tick());
        assert!(!d.note_tick());
        assert!(d.note_tick(), "N-й тик = окно snapshot-фазы");
        assert_eq!(d.ticks_since_build, 4);
    }

    #[test]
    fn cycle_is_fail_closed_incomplete() {
        let mut d = AiDagSnapshot::new();
        let a = d.add_node(Tier::Goal, 1).unwrap();
        let b = d.add_node(Tier::Goal, 2).unwrap();
        assert!(d.add_edge(a, b));
        assert!(d.add_edge(b, a));
        d.rebuild();
        assert!(!d.topo_complete(), "цикл → FAIL-CLOSED → ваниль весь тик");
    }

    #[test]
    fn lever_is_strict_dormant_by_default() {
        // В тестовой среде флаг не выставлен → dormant (STRICT eq).
        if env::var("CRUSSTY_LEVER_FLAG").is_err() {
            assert!(!lever_enabled());
            assert_eq!(status(), "dormant");
        }
    }
}
