#!/usr/bin/env python3
"""RECON-24: аллок-атрибуция GC-семейства (свежий ТОП-1: G1 14.6% +
oop-barriers 12.2% = 26.8% scene-CPU) — разложение alloc-оси до под-лейнов
>=5% на свежей валидной ноге s7189 (банк v3 + travel_diet=1).

Методика:
  - вход: alloc-collapsed.txt s7189 (AP alloc-окно 80-100% соака,
    13144 сэмпла, формат листа <fqcn>_[i|k]);
  - leaf-type ценз (ЧТО аллокируется) — нормализация точек в слэши;
  - subsystem-атрибуция (КТО аллокирует): обход стека от листа к корню,
    ПЕРВЫЙ фрейм, матчящий сигнатуру подсистемы, = ближайший владелец
    (deepest-owner-wins, бакеты взаимоисключающие);
  - gc.log: паузы как бюджет TPS (сумма pause / 300s окно).

Использование: python3 scripts/bench4_recon/recon24_gc_alloc_attr.py [run_dir]
Док: research/gc-recon-2026-09-19/RECON24_GC_ALLOC_ATTR.md
"""
import os
import re
import sys
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
DEFAULT = os.path.join(RESDIR, "run-s7189-traveldiet-v2a")
DOC = os.path.join(RESDIR, "RECON24_GC_ALLOC_ATTR.md")

LEAF_TYPE = re.compile(r"([A-Za-z0-9_$.\[\]]+)_\[[ik]\]$")

# упорядоченные сигнатуры подсистем: первый матч фрейма (от листа к корню) = владелец
SUBSYS = [
    ("travel/movement", re.compile(
        r"LivingEntity\.travel|travelInAir|travelInFluid|Entity\.move|Entity\.collide|"
        r"handleRelativeFriction|getInputVector|addCollisionsAlongTravel|CollisionUtil|"
        r"TraverseHector|TraverseOps|collideBoundingBox|checkFallDamage")),
    ("inside-blocks/fluid-scan", re.compile(
        r"checkInsideBlocks|InsideBlockEffectApplier|updateFluidHeightAndDoFluidPushing|"
        r"collidedWithFluid|FluidPushOps|getFluidState")),
    ("pathfinding/ai", re.compile(
        r"PathNavigation|PathFinder|NodeEvaluator|GoalSelector|Brain\b|Behavior|"
        r"registerGoals|Mob\.tick|aiStep|getWalkTarget|LookControl")),
    ("tracking/sync", re.compile(
        r"ServerEntity|sendChanges|SynchedData|Clientbound|Broadcast|ChunkSender|"
        r"EntityTracker")),
    ("chunk-system", re.compile(
        r"ChunkSystem|PalettedContainer|LevelChunk\.|ChunkSerializer|PoiManager|"
        r"ChunkMap|RegionFileStorage|SimpleRegionStorage|DataConverter|NBTMapType|"
        r"DataResult|ChunkHolder|ChunkAccess|SectionPos|LevelChunkSection")),
    ("block/random-tick", re.compile(
        r"tickBlockEntities|blockTick|randomTick|LevelTicks|tickChunk|"
        r"BlockBehaviour\.|FarmBlock|CropBlock")),
    ("entity-mgmt/spawn", re.compile(
        r"addEntity|EntityCallbacks|EntityTickList|Spawner|BenchPopulation|"
        r"addFreshEntity")),
    ("kernel-infra/ops", re.compile(
        r"RegionTickOps|BatchCollector|BlockUpdateOps|TrackerTickOps|RngOps|"
        r"FlushOps|StepBasedCollector|ImprovedNoise|cru/sty")),
    ("moonrise/paper", re.compile(
        r"ca/spottedleaf/moonrise|ca/spottedleaf/concurrentutil")),
    ("jdk-collections/serde", re.compile(
        r"it/unimi/dsi/fastutil|java/util|com/google/common|java/lang/invoke|"
        r"com/mojang/serialization")),
    ("jvm-core", re.compile(r"java/lang|jdk/internal|sun/misc")),
]


def norm(stack: str) -> str:
    return stack.replace(".", "/")


def leaf_type(frame: str) -> str:
    m = LEAF_TYPE.search(frame)
    if not m:
        return None
    t = m.group(1)
    # нормализуем точечную форму к слэшам и срезаем пакеты для типов ядра
    t = t.replace(".", "/")
    return t


def subsystem(stack_norm: str) -> str:
    for fr in reversed(stack_norm.split(";")):
        for name, rx in SUBSYS:
            if rx.search(fr):
                return name
    return "other"


def load(path):
    rows = []
    with open(path, errors="replace") as f:
        for line in f:
            try:
                stack, c = line.rstrip("\n").rsplit(" ", 1)
                rows.append((stack, int(c)))
            except ValueError:
                continue
    return rows


def main():
    run = sys.argv[1] if len(sys.argv) > 1 else DEFAULT
    rows = load(os.path.join(run, "alloc-collapsed.txt"))
    tot = sum(n for _, n in rows)

    types = Counter()
    subs = Counter()
    # кросс: подсистема x топ-тип (для топ-3 подсистем печатаем их типы)
    sub_types = {}
    for stack, n in rows:
        lt = leaf_type(stack.split(";")[-1])
        if lt:
            types[lt] += n
        s = subsystem(norm(stack))
        subs[s] += n
        if lt:
            sub_types.setdefault(s, Counter())[lt] += n

    # gc.log
    gc_path = os.path.join(run, "gc.log")
    pauses = young = full = 0
    total_ms = 0.0
    if os.path.exists(gc_path):
        for line in open(gc_path, errors="replace"):
            if "Pause Young" in line:
                young += 1
            elif "Pause Full" in line:
                full += 1
            m = re.search(r"\) ([0-9.]+)ms", line)
            if m and ("Pause Young" in line or "Pause Full" in line):
                total_ms += float(m.group(1))
        pauses = young + full

    lines = []
    lines.append("# RECON-24 — аллок-атрибуция GC-семейства (свежий ТОП-1)")
    lines.append("")
    lines.append(f"Run: `{os.path.basename(run)}` (банк v3 + travel_diet=1, свежая валидная нога).")
    lines.append(f"Alloc-окно ЖИВОЕ: **{tot} сэмплов**, листов-типов {len(types)} (AP-PID дефект на cpu-окно не влияет).")
    lines.append("")
    lines.append("## ЧТО аллокируется (leaf-типы, топ-12)")
    lines.append("")
    lines.append("| тип | сэмплы | % alloc-оси |")
    lines.append("|---|---|---|")
    for t, n in types.most_common(12):
        lines.append(f"| {t} | {n} | {100.0*n/tot:.1f}% |")
    lines.append("")
    lines.append("## КТО аллокирует (deepest-owner, бакеты взаимоисключающие)")
    lines.append("")
    lines.append("| подсистема | сэмплы | % alloc-оси | >=5% |")
    lines.append("|---|---|---|---|")
    for s, n in subs.most_common():
        mark = "ДА" if 100.0*n/tot >= 5.0 else ""
        lines.append(f"| {s} | {n} | {100.0*n/tot:.1f}% | {mark} |")
    lines.append("")
    lines.append("## Топ-3 подсистемы: их доминирующие типы")
    lines.append("")
    for s, n in subs.most_common(3):
        tt = ", ".join(f"{t} {100.0*x/max(1,sub_types[s].total()):.0f}%"
                       for t, x in sub_types[s].most_common(4))
        lines.append(f"- **{s}** ({100.0*n/tot:.1f}%): {tt}")
    lines.append("")
    if pauses:
        lines.append("## GC-бюджет (gc.log того же лега)")
        lines.append("")
        lines.append(f"- пауз: {pauses} (young {young} / full {full}), сумма {total_ms:.0f} ms "
                     f"= {total_ms/300000.0*100:.1f}% 300s-окна TPS")
        lines.append("")
    lines.append("## ВЫВОДЫ (прегистер выбора рычага после финала лейна #14)")
    lines.append("")
    top = subs.most_common(3)
    lines.append(f"1. Под-лейны ≥5% alloc-оси: "
                 + ", ".join(f"{s} {100.0*n/tot:.1f}%" for s, n in top if 100.0*n/tot >= 5.0)
                 + " — каждый кандидат на архитектурный рычаг (кэш/батчинг/O(n)->O(1)).")
    lines.append("2. Связка осей: cutting под-лейна X% alloc-оси режет пропорционально "
                 "young-evacuation и oop-barriers (12.2% scene-CPU) — двойной эффект на TPS.")
    lines.append("3. Парити-ограничение: любые scalar-replacement-манёвры обязаны учитывать "
                 "урок лейна #14 (C2 сам замещает темпорари — аткаться только на СТРУКТУРНЫЕ "
                 "аллокации: контейнеры/строки/сериализацию, не на скалярные Vec3/AABB продукты).")

    report = "\n".join(lines) + "\n"
    print(report)
    with open(DOC, "w") as f:
        f.write(report)
    print(f"DOC -> {DOC}")


if __name__ == "__main__":
    main()
