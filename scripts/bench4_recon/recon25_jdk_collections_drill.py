#!/usr/bin/env python3
"""RECON-25: дрилл под-лейна jdk-collections/serde (25.5% alloc-оси, RECON-24)
до домен-клиентов >=5% на валидных ногах s7189 + s7194 (кросс-раннер).

Вопрос RECON'а: ЧЬИ вызовы рождают jdk-коллекции/сериализацию?
  - фильтр: аллок-сэмплы, чей deepest-owner = jdk-collections/serde
    (fastutil | java/util | guava | java/lang/invoke | mojang-serialization);
  - дрилл 1 (ЧТО): leaf-типы внутри бакета;
  - дрилл 2 (ДЛЯ ЧЕГО): первый фрейм ОТ ЛИСТА, не матчящий jdk-сигнатуру
    = доменный клиент (deepest-client-wins); клиенты группируются в
    домен-бакеты; spark/jmx-мониторинг выделяется отдельно (не игровой);
  - пороги: >=5% alloc-ОСИ (и % бакета для читаемости).

Использование: python3 scripts/bench4_recon/recon25_jdk_collections_drill.py
Док: research/gc-recon-2026-09-19/RECON25_JDK_COLLECTIONS.md
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
}
DOC = os.path.join(RESDIR, "RECON25_JDK_COLLECTIONS.md")

LEAF_TYPE = re.compile(r"([A-Za-z0-9_$.\[\]]+)_\[[ik]\]$")
JDK_SIG = re.compile(
    r"it/unimi/dsi/fastutil|java/util|com/google/common|java/lang/invoke|"
    r"com/mojang/serialization")

SPARK_SIG = re.compile(
    r"me/lucko/spark|com/sun/jmx|CpuMonitor|OperatingSystemImpl|CgroupV2|"
    r"CgroupMetrics|CgroupUtil|readStringValue|getSystemCpuLoad|"
    r"getProcessCpuLoad|sun/nio/fs|UnixFileSystem")

# домен-бакеты клиентов (первый не-jdk фрейм стека)
DOMAINS = [
    ("inside-scan-visitset", re.compile(
        r"forEachBlockIntersectedBetween|checkInsideBlocks")),
    ("fluid-tags/eyefluid", re.compile(
        r"updateFluidOnEyes|FluidState\.getTags|FluidPushOps|"
        r"updateInWaterStateAndDoFluidPushing|collidedWithFluid|getFluidState")),
    ("inside-blocks", re.compile(
        r"checkInsideBlocks|InsideBlockEffectApplier")),
    ("travel/movement", re.compile(
        r"LivingEntity\.travel|travelInAir|travelInFluid|Entity\.move|Entity\.collide|"
        r"handleRelativeFriction|CollisionUtil|addCollisionsAlongTravel")),
    ("pathfinding/ai", re.compile(
        r"PathNavigation|PathFinder|NodeEvaluator|GoalSelector|Brain\b|Behavior|"
        r"LookControl|MoveControl|aiStep|serverAiStep|registerGoals")),
    ("chunk-serde/persistence", re.compile(
        r"ChunkSerializer|PalettedContainer|ChunkMap|RegionFileStorage|"
        r"SimpleRegionStorage|NBTMapType|DataResult|DynamicOps|ChunkHolder|"
        r"PoiManager|LevelChunkSection|ChunkAccess")),
    ("tracking/sync", re.compile(
        r"ServerEntity|sendChanges|SynchedEntityData|Clientbound|Broadcast|"
        r"EntityTracker|ChunkSender")),
    ("kernel-ops", re.compile(
        r"RegionTickOps|BatchCollector|BlockUpdateOps|TrackerTickOps|RngOps|"
        r"FlushOps|StepBasedCollector|cru/sty")),
    ("entity-mgmt/spawn", re.compile(
        r"addEntity|EntityCallbacks|EntityTickList|Spawner|BenchPopulation|"
        r"addFreshEntity")),
    ("world/block-tick", re.compile(
        r"tickBlockEntities|blockTick|randomTick|LevelTicks|tickChunk|"
        r"BlockBehaviour\.|ServerLevel\.tick|Level\.guardEntityTick")),
    ("spark/jmx-monitor", re.compile(
        r"me/lucko/spark|com/sun/jmx|CpuMonitor|OperatingSystemImpl|CgroupV2|"
        r"CgroupMetrics|CgroupUtil")),
]


def norm(stack):
    return stack.replace(".", "/")


def leaf_type(frame):
    m = LEAF_TYPE.search(frame)
    if not m:
        return None
    return m.group(1).replace(".", "/")


REJECT_OTHER = [
    re.compile(rx) for rx in (
        r"LivingEntity\.travel|travelInAir|travelInFluid|Entity\.move|"
        r"Entity\.collide|handleRelativeFriction|CollisionUtil",
        r"InsideBlockEffectApplier|updateFluidHeightAndDoFluidPushing|getFluidState",
        r"PathNavigation|GoalSelector|Brain\b|Behavior|LookControl",
        r"ServerEntity|sendChanges|SynchedEntityData|Clientbound",
        r"ChunkSystem|PalettedContainer|LevelChunk\.|ChunkSerializer|"
        r"PoiManager|ChunkMap|RegionFileStorage",
        r"tickBlockEntities|blockTick|randomTick|LevelTicks",
        r"addEntity|EntityCallbacks|Spawner|addFreshEntity",
        r"RegionTickOps|BatchCollector|BlockUpdateOps|TrackerTickOps|"
        r"RngOps|FlushOps|StepBasedCollector|cru/sty",
        r"ca/spottedleaf/moonrise|ca/spottedleaf/concurrentutil",
    )
]


def jdk_bucket(stack_norm):
    """deepest-owner == jdk-collections/serde: первый jdk-фрейм в пределах 4
    фреймов от листа, и между листом и ним НЕТ фреймов других подсистем.
    Так LongOpenHashSet.<init> под checkInsideBlocks попадает в бакет, а
    Vec3 под checkInsideBlocks (jdk-фрейм далеко/чужая подсистема рядом) — нет."""
    for i, fr in enumerate(reversed(stack_norm.split(";"))):
        if i > 0 and any(rx.search(fr) for rx in REJECT_OTHER):
            return False
        if JDK_SIG.search(fr):
            return i <= 4
    return False


def domain(stack_norm):
    # spark/jmx-мониторинг определяется по ЛЮБОМУ фрейму стека (не игровой)
    if SPARK_SIG.search(stack_norm):
        return "spark/jmx-monitor"
    frames = stack_norm.split(";")
    # последний фрейм = leaf-тип аллокации (char[]_[k] и т.п.) — пропускаем
    for fr in reversed(frames[:-1] if LEAF_TYPE.search(frames[-1]) else frames):
        if JDK_SIG.search(fr):
            continue
        for name, rx in DOMAINS:
            if rx.search(fr):
                return name
        return "other:" + fr.split("/")[-1][:60]
    return "root-jdk-only"


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


def drill(run_name, run_dir, tot_all):
    rows = load(os.path.join(run_dir, "alloc-collapsed.txt"))
    tot = sum(n for _, n in rows)
    b_rows = [(s, n) for s, n in rows if jdk_bucket(norm(s))]
    b_tot = sum(n for _, n in b_rows)

    types = Counter()
    doms = Counter()
    dom_examples = {}
    for s, n in b_rows:
        lt = leaf_type(s.split(";")[-1])
        if lt:
            types[lt] += n
        d = domain(norm(s))
        doms[d] += n
        dom_examples.setdefault(d, Counter())[s.split(";")[-2] if ";" in s else s] += n

    lines = [f"## {run_name}: {os.path.basename(run_dir)}", ""]
    lines.append(f"Alloc-ось: {tot} сэмплов; бакет jdk-collections/serde: "
                 f"**{b_tot} = {100.0*b_tot/tot:.1f}% alloc-оси** "
                 f"(RECON-24 дал 25.5% на s7189).")
    lines.append("")
    lines.append(f"### ЧТО (leaf-типы бакета, топ-8; порог 5% ОСИ = {0.05*tot:.0f} сэмплов)")
    lines.append("")
    lines.append("| тип | сэмплы | % оси | % бакета | >=5% оси |")
    lines.append("|---|---|---|---|---|")
    for t, n in types.most_common(8):
        mark = "ДА" if 100.0*n/tot >= 5.0 else ""
        lines.append(f"| {t} | {n} | {100.0*n/tot:.1f}% | {100.0*n/max(1,b_tot):.1f}% | {mark} |")
    lines.append("")
    lines.append("### ДЛЯ ЧЕГО (домен-клиент, deepest-client-wins)")
    lines.append("")
    lines.append("| домен | сэмплы | % оси | % бакета | >=5% оси |")
    lines.append("|---|---|---|---|---|")
    for d, n in doms.most_common(12):
        mark = "ДА" if 100.0*n/tot >= 5.0 else ""
        lines.append(f"| {d} | {n} | {100.0*n/tot:.1f}% | {100.0*n/max(1,b_tot):.1f}% | {mark} |")
    lines.append("")
    lines.append("### Ключевые call-sites топ-доменов (лист-1 фрейм, топ-3)")
    lines.append("")
    for d, n in doms.most_common(4):
        if 100.0 * n / tot < 1.0:
            continue
        ex = "; ".join(f"`{fr}` {c}" for fr, c in dom_examples[d].most_common(3))
        lines.append(f"- **{d}** ({100.0*n/tot:.1f}% оси): {ex}")
    lines.append("")
    return lines, (tot, b_tot, types, doms)


def main():
    all_lines = ["# RECON-25 — дрилл jdk-collections/serde до домен-клиентов (>=5%)", ""]
    results = {}
    for name, rd in RUNS.items():
        if not os.path.exists(os.path.join(rd, "alloc-collapsed.txt")):
            all_lines.append(f"## {name}: НЕТ alloc-collapsed.txt — пропущен")
            all_lines.append("")
            continue
        lines, res = drill(name, rd, 0)
        results[name] = res
        all_lines += lines

    # кросс-раннер стабильность
    if len(results) == 2:
        (t1, b1, ty1, d1) = results["s7189"]
        (t2, b2, ty2, d2) = results["s7194"]
        all_lines.append("## Кросс-раннер стабильность")
        all_lines.append("")
        all_lines.append(f"- бакет jdk-collections/serde: s7189 {100.0*b1/t1:.1f}% vs "
                         f"s7194 {100.0*b2/t2:.1f}% alloc-оси")
        doms = set(d1) | set(d2)
        rows = sorted(((d, 100.0*d1.get(d,0)/t1, 100.0*d2.get(d,0)/t2) for d in doms),
                      key=lambda x: -max(x[1], x[2]))
        all_lines.append("")
        all_lines.append("| домен | s7189 % оси | s7194 % оси | стабилен |")
        all_lines.append("|---|---|---|---|")
        for d, a, b in rows[:12]:
            stable = "ДА" if abs(a-b) <= 3.0 else "НЕТ"
            all_lines.append(f"| {d} | {a:.1f}% | {b:.1f}% | {stable} |")
        all_lines.append("")

    all_lines += [
        "## ВЫВОДЫ (прегистер следующего рычага)",
        "",
        "1. Крупнейший домен-клиент >=5% alloc-оси = следующий рычаг ТОП-1 GC-семейства",
        "   (архитектурный: кэш/мемоизация/батчинг/O(n)->O(1); структурные аллокации",
        "   только — урок C2 scalar-replacement лейна #14).",
        "2. spark/jmx-мониторинг в доменах — НЕ игровой код: при выборе рычага",
        "   вычитается из рассмотрения (не парити-релевантен).",
        "3. Кросс-раннер стабильность доменов обязательна (пул раннеров 1.9x",
        "   дневная дисперсия, GOAL x30-ADD/x31).",
        "",
    ]
    report = "\n".join(all_lines) + "\n"
    print(report)
    with open(DOC, "w") as f:
        f.write(report)
    print(f"DOC -> {DOC}")


if __name__ == "__main__":
    main()
