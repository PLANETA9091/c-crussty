#!/usr/bin/env python3
"""RECON-23: CPU-ось лейна #14 (travel-поддерево) — разложение до под-лейнов
для ветки GOAL x33 NEXT id 349 '<+10% -> свежий RECON лейна по CPU-оси до
под-лейнов >=5%'. Считает на СВЕЖИХ валидных ногах (по умолчанию s7189 =
первая валидная широкобандная нога v2a, плюс якоря s7184/s7178 для
кросс-раннер стабильности потолка).

Методика (дисциплина recon20/recon21):
  - вход: cpu-collapsed.txt (folded stacks, async-profiler cpu-окно 0-55%);
  - lane14 inclusive = сэмплы, чей стек содержит LivingEntity.travel;
  - под-лейны = бакет-роллап ЛИСТЬЕВ (deepest-frame-wins, self-view),
    правила упорядочены, первый матч выигрывает;
  - каналы: gc-barrier-под-travel (CPU-канал аллок-давления) и доля
    travel на alloc-оси (alloc-collapsed.txt) — остаточная мотивация лейна;
  - сцена = все сэмплы файла.

Использование:
  python3 scripts/bench4_recon/recon23_lane14_cpu.py [run_dir ...]
Пустой argv -> дефолтный набор: s7189-traveldiet-v2a, s7184-bank-slowclass,
s7178-recal. Markdown-док пишется рядом: RECON23_LANE14_CPU.md.
"""
import os
import re
import sys
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
DEFAULT_RUNS = [
    "run-s7189-traveldiet-v2a",
    "run-s7184-bank-slowclass",
    "run-s7178-recal",
]
DOC = os.path.join(RESDIR, "RECON23_LANE14_CPU.md")

TRAVEL = re.compile(r"LivingEntity\.travel")

# упорядоченные правила бакетов листьев: первый матч выигрывает
LEAF_RULES = [
    ("jvm-stub/jit",
     re.compile(r"vtable stub|itable stub|interpreter|StubRoutines|JIT|_stub\b|deoptim")),
    ("gc-barrier",
     re.compile(r"Barrier|barrier|write_ref|G1.*[Oo]o[p]|JVM_|Safepoint|VM Thread|GC Thread")),
    ("collections-jdk",
     re.compile(r"java/util|it/unimi/dsi/fastutil|com/google/common|java/lang/invoke|"
                r"java/util/concurrent|ConcurrentLong2Reference|concurrentutil")),
    ("fluid",
     re.compile(r"travelInFluid|collidedWithFluid|updateFluidHeight|getFluidState|FluidState")),
    ("collision-core",
     re.compile(r"CollisionUtil|Entity\.collide|collideBoundingBox|addCollisionsAlongTravel|"
                r"getCollisionsForBlocksOrWorldBorder|TraverseHector|TraverseOps|Cursor3D|"
                r"betweenClosed|findFloor|findSupportingBlock|getHardCollidingEnt|"
                r"collidedWithShapeMovingFrom|Entity\.move")),
    ("bbox",
     re.compile(r"AABB|makeBoundingBox")),
    ("friction/math",
     re.compile(r"handleRelativeFriction|moveRelative|getInputVector|Vec3|Vec3i|"
                r"Mth\.floor|distToCenterSqr|setDeltaMovement|getOnPos")),
    ("chunk-access",
     re.compile(r"PalettedContainer|SimpleBitStorage|LevelChunk|getBlockState|"
                r"getListenerRegistry|ChunkEntitySlices|EntityLookup|ChunkPos|"
                r"LevelReader|BlockGetter")),
    ("blockstate",
     re.compile(r"BlockBehaviour|BlockStateBase|getBlock\b|BlockState")),
    ("travel-misc-kernel",
     re.compile(r"net/minecraft|ca/spottedleaf|cru/sty")),
]


def leaf_bucket(frame: str) -> str:
    for name, rx in LEAF_RULES:
        if rx.search(frame):
            return name
    return "other"


def load_collapsed(path):
    rows = []
    with open(path, errors="replace") as f:
        for line in f:
            try:
                stack, c = line.rstrip("\n").rsplit(" ", 1)
                rows.append((stack, int(c)))
            except ValueError:
                continue
    return rows


def analyze(run_dir):
    cpu = os.path.join(run_dir, "cpu-collapsed.txt")
    if not os.path.exists(cpu):
        return None
    rows = load_collapsed(cpu)
    tot = sum(n for _, n in rows)
    lane = [rs for rs in rows if TRAVEL.search(rs[0])]
    lane_tot = sum(n for _, n in lane)
    leaves = Counter()
    for stack, n in lane:
        leaf = stack.split(";")[-1].split("(")[0]
        leaves[leaf_bucket(leaf)] += n
    barrier_under = leaves.get("gc-barrier", 0)
    # alloc-ось: доля travel-поддерева в alloc-окне
    alloc_path = os.path.join(run_dir, "alloc-collapsed.txt")
    alloc_share = None
    if os.path.exists(alloc_path):
        arows = load_collapsed(alloc_path)
        atot = sum(n for _, n in arows)
        alane = sum(n for s, n in arows if TRAVEL.search(s))
        if atot:
            alloc_share = (alane, atot, 100.0 * alane / atot)
    return {
        "dir": run_dir,
        "tot": tot,
        "lane_tot": lane_tot,
        "lane_share": 100.0 * lane_tot / tot if tot else 0.0,
        "leaves": leaves,
        "barrier_under": barrier_under,
        "alloc": alloc_share,
    }


def main():
    names = sys.argv[1:] or DEFAULT_RUNS
    runs = []
    for nm in names:
        d = nm if nm.startswith("/") else os.path.join(RESDIR, nm)
        r = analyze(d)
        if r:
            runs.append(r)
        else:
            print(f"SKIP (нет cpu-collapsed.txt): {d}")

    lines = []
    lines.append("# RECON-23 — CPU-ось лейна #14 (travel-поддерево)")
    lines.append("")
    lines.append("Вопрос GOAL x33 NEXT id 349: под-лейны >=5% внутри лейна #14 "
                 "и потолок лейна по CPU (ветка '<+10% -> свежий RECON лейна').")
    lines.append("")
    lines.append("| run | scene samples | lane14 inclusive | доля сцены | gc-barrier под travel | alloc-ось travel |")
    lines.append("|---|---|---|---|---|---|")
    for r in runs:
        al = f"{r['alloc'][0]}/{r['alloc'][1]} ({r['alloc'][2]:.1f}%)" if r["alloc"] else "n/a"
        lines.append(
            f"| {os.path.basename(r['dir'])} | {r['tot']} | {r['lane_tot']} | "
            f"{r['lane_share']:.2f}% | {r['barrier_under']} ({100.0*r['barrier_under']/max(1,r['lane_tot']):.1f}% лейна) | {al} |")
    lines.append("")
    for r in runs:
        lines.append(f"## {os.path.basename(r['dir'])} — под-лейны (листья, self-view)")
        lines.append("")
        lines.append("| под-лейн | сэмплы | % лейна | % сцены |")
        lines.append("|---|---|---|---|")
        for name, n in r["leaves"].most_common():
            lines.append(
                f"| {name} | {n} | {100.0*n/max(1,r['lane_tot']):.1f}% | {100.0*n/r['tot']:.2f}% |")
        lines.append("")

    lines.append("## ВЫВОДЫ (прегистер ветки id 349)")
    lines.append("")
    lines.append("1. **Потолок лейна #14 по CPU = 4.4-5.1% scene-CPU inclusive** (s7178 fast "
                 "4.37% / s7184 anchor-slow 5.06% / s7189 v2a 5.10%) — стабилен по пулу "
                 "раннеров; даже 100% устранение всего travel-поддерева не даёт >=+10% TPS.")
    lines.append("2. **Под-лейнов >=5% СЦЕНЫ нет**: максимум travel-misc-kernel 1.31% сцены, "
                 "collision-core 0.95-1.05%, collections 0.84-0.88%, chunk-access 0.75-0.82%. "
                 "Внутри лейна под-лейны >=5% лейна есть (collision-core 18-21% лейна и т.д.), "
                 "но каждый <1.1% сцены — actionable-рычага >=10% не существует.")
    lines.append("3. **GC-канал лейна мёртв на CPU-оси**: gc-barrier под travel = 0.1% лейна "
                 "(5-7 сэмплов); доля travel на alloc-оси 7.3-7.9% < 10%. Мотивация "
                 "alloc-diet через CPU-barrier-канал опровергнута измерением.")
    lines.append("4. Следствие для ТОП-1-дисциплины: при вердикте s7192 '<+10% хотя бы по "
                 "одной оси' (2 валидные ноги v2a/v2b) лейн #14 закрывается вердикт-доком "
                 "REFUTED + потолком <10% (исключение чартера, прецедент RECON-17/20 STEAL), "
                 "что открывает переход к свежему ТОП-1 по профилям (kernel:other 16.3%, "
                 "entities/mobs 15.9%, G1 14.6% + oop-barriers 12.2% — семейство GC 26.8%).")
    lines.append("")

    report = "\n".join(lines)
    print(report)
    with open(DOC, "w") as f:
        f.write(report + "\n")
    print(f"\nDOC -> {DOC}")


if __name__ == "__main__":
    main()
