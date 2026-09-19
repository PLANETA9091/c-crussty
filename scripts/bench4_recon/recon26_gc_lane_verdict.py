#!/usr/bin/env python3
"""RECON-26: (a) свежий CPU-ТОП на s7194 (последняя валидная нога,
zeroin COMPOSED sites:3, threw=0) после финала лейна #14 и REFUTED #10;
(b) вердикт-док GC-семейства (ТОП-1 эпохи после #14): потолок TPS-конверсии.

Эмпирическая база потолка GC-лейна (пегистер GOAL):
  - 5 архитектурных рычагов аллок-давления REFUTED по TPS при ПОДТВЕРЖДЁННОМ
    падении аллок/CPU лейнов: #9 flat_traversal (orchestration −70% цели
    PG4a не достигнута/TPS −5.6% шум), #11 zero_cursor (alloc −21%, TPS −22%),
    #12 inside_diet (alloc −47%/CPU −66% лейна, TPS −17..−6% min-of-2),
    #14 travel_diet (3 валидные ноги, ЛУЧШИЙ случай +0.0% absolute),
    #10 zero_alloc (−11.9%/−12.5% НАОБОРОТ при zeroin ARMED);
  - GC-бюджет пауз = 6.5% 300s-окна (RECON-24: 316 young = 6.5%);
  - young-эвакуация в диет-ногах 140-144 vs банк 154 (−9%) — TPS 0%.

Использование: python3 scripts/bench4_recon/recon26_gc_lane_verdict.py
Док: research/gc-recon-2026-09-19/RECON26_GC_LANE_VERDICT.md
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON26_GC_LANE_VERDICT.md")

# упорядоченные CPU-семьи (deepest-frame-wins, взаимоисключающие)
FAM = [
    ("GC/jvm(G1+barriers)", re.compile(
        r"G1|JVM_|garbage|ZPage|PSWeak|StringDedup|Barrier|barrierSet|"
        r"Interpreter|_gen_|c2_|C2 |Compilation|jcstress|jdk/internal/vm")),
    ("jdk-collections/serde", re.compile(
        r"it/unimi/dsi/fastutil|java/util|com/google/common|java/lang/invoke|"
        r"com/mojang/serialization|java/io|java/nio")),
    ("kernel:other", re.compile(
        r"RegionTickOps|BatchCollector|BlockUpdateOps|TrackerTickOps|RngOps|"
        r"FlushOps|StepBasedCollector|InsideDietOps|TraverseOps|ZeroAllocOps|"
        r"TravelDietOps|cru/sty|crussty")),
    ("entities/mobs-tick", re.compile(
        r"Mob\.tick|LivingEntity\.tick|Entity\.tick|baseTick|aiStep|serverAiStep|"
        r"GoalSelector|PathNavigation|Brain\b|Behavior|LookControl|MoveControl|"
        r"mobTick|doTick")),
    ("inside-pipeline", re.compile(
        r"checkInsideBlocks|InsideBlockOps|InsideBlockEffectApplier|"
        r"forEachBlockIntersectedBetween|betweenCornersInDirection")),
    ("fluid-sim", re.compile(
        r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState;|"
        r"collidedWithFluid|FluidPushOps|updateFluidOnEyes")),
    ("travel/movement", re.compile(
        r"Entity\.travel|Entity\.move|Entity\.collide|handleRelativeFriction|"
        r"addCollisionsAlongTravel|CollisionUtil|performCollisions|"
        r"TravelDietOps")),
    ("chunk-system", re.compile(
        r"PalettedContainer|LevelChunk\.|ChunkMap|ChunkSerializer|PoiManager|"
        r"ChunkHolder|SectionPos|getBlockState")),
    ("tracking/sync", re.compile(
        r"ServerEntity|sendChanges|SynchedEntityData|Clientbound|Broadcast")),
    ("spark/jmx-monitor", re.compile(
        r"me/lucko/spark|com/sun/jmx|CpuMonitor|Cgroup|UnixFileSystem")),
]


def fam_of(stack):
    frames = stack.split(";")
    for fr in reversed(frames):
        for name, rx in FAM:
            if rx.search(fr):
                return name
    return "other"


def cpu_cut(run_name, run_dir):
    path = os.path.join(run_dir, "cpu-collapsed.txt")
    if not os.path.exists(path):
        return None
    fam = Counter()
    tot = 0
    for line in open(path, errors="ignore"):
        s, _, c = line.rstrip("\n").rpartition(" ")
        if not s:
            continue
        try:
            n = int(c)
        except ValueError:
            continue
        tot += n
        fam[fam_of(s.replace(".", "/"))] += n
    return tot, fam


def gc_budget(run_dir):
    path = os.path.join(run_dir, "gc.log")
    young = full = 0
    total_ms = 0.0
    if not os.path.exists(path):
        return young, full, total_ms
    for line in open(path, errors="replace"):
        if "Pause Young" in line:
            young += 1
        elif "Pause Full" in line:
            full += 1
        m = re.search(r"\) ([0-9.]+)ms", line)
        if m and ("Pause Young" in line or "Pause Full" in line):
            total_ms += float(m.group(1))
    return young, full, total_ms


def main():
    lines = ["# RECON-26 — свежий CPU-ТОП (s7194) + ВЕРДИКТ GC-семейства (ТОП-1 эпохи)", ""]
    cuts = {}
    for name, rd in RUNS.items():
        res = cpu_cut(name, rd)
        if not res:
            continue
        tot, fam = res
        cuts[name] = (tot, fam)
        lines.append(f"## CPU-ось {name} ({os.path.basename(rd)}): {tot} сэмплов")
        lines.append("")
        lines.append("| семья | сэмплы | % оси | >=5% |")
        lines.append("|---|---|---|---|")
        for s, n in fam.most_common():
            mark = "ДА" if 100.0*n/tot >= 5.0 else ""
            lines.append(f"| {s} | {n} | {100.0*n/tot:.1f}% | {mark} |")
        lines.append("")

    # кросс-стабильность
    if len(cuts) == 2:
        (t1, f1), (t2, f2) = cuts["s7194"], cuts["s7189"]
        lines.append("## Кросс-раннер: s7194 vs s7189")
        lines.append("")
        lines.append("| семья | s7194 % | s7189 % | стабильно |")
        lines.append("|---|---|---|---|")
        for s, _ in f1.most_common():
            a, b = 100.0*f1[s]/t1, 100.0*f2.get(s, 0)/t2
            lines.append(f"| {s} | {a:.1f}% | {b:.1f}% | {'ДА' if abs(a-b) <= 4.0 else 'НЕТ'} |")
        lines.append("")

    y, f, ms = gc_budget(RUNS["s7194"])
    lines += [
        "## ВЕРДИКТ: GC-семейство — НЕ TPS-рычаг, потолок TPS-конверсии <10%",
        "",
        f"GC-бюджет s7194: {y} young / {f} Full = {ms:.0f} ms = "
        f"{ms/300000.0*100:.1f}% 300s-окна (RECON-24: 6.5%).",
        "",
        "### Основание закрытия (четвёртый прецедент чартера после RECON-17/20/23)",
        "",
        "1. **Пять архитектурных рычагов GC-давления REFUTED по TPS при",
        "   ПОДТВЕРЖДЁННОМ структурном эффекте на лейны**: #9 flat_traversal",
        "   (бит-в-бит TraverseOps, TPS −5.6% шум → FAIL PG4a), #11 zero_cursor",
        "   (alloc −21.3%, TPS −22.2%), #12 inside_diet (alloc −47.2% / CPU",
        "   −66..−67% ОБА лега min-of-2, TPS −17..−6%), #14 travel_diet (3",
        "   валидные ноги: −21.6/−18.8, −1.5/+0.0, −3.9/+0.0), #10 zero_alloc",
        "   (−11.9/−12.5% НАОБОРОТ при zeroin COMPOSED sites:3 + threw=0).",
        "2. **Механизм**: молодое GC-давление конвертируется в young-паузы",
        "   (STW) на уровне 6.5% окна; параллельные G1/barrier-CPU ядра",
        "   вне критического пути MSPT (region-воркеры не блокируются).",
        "   Диеты реально снижают evac/barrier-работу (young 140-144 vs 154)",
        "   — TPS неподвижен: стену держит НЕ GC.",
        "3. **RECON-25**: в крупнейшем аллок-под-лейне jdk-collections/serde",
        "   нет домена >=5% вне already-REFUTED #9 (inside-scan-visitset",
        "   7.2/8.3% = visit-set vanilla-тела forEachBlockIntersectedBetween,",
        "   убиваемый TraverseOps); остальные домены <3%. Дрилл исчерпан.",
        "",
        "### Следствие (прегистрированный выбор нового ТОП-1)",
        "",
        "GC-лейн закрыт как TPS-цель (потолок <10% + 5 REFUTED). Новый ТОП-1",
        "= старшая стабильная CPU-семья s7194 из таблицы выше (candidates:",
        "entities/mobs-tick, kernel:other, travel/movement) с RECON-дриллом",
        "до под-лейнов >=5% до реализации рычага. spark/jmx — не игровой,",
        "вычитается.",
        "",
    ]
    report = "\n".join(lines) + "\n"
    print(report)
    with open(DOC, "w") as f2:
        f2.write(report)
    print(f"DOC -> {DOC}")


if __name__ == "__main__":
    main()
