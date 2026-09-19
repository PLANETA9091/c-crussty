#!/usr/bin/env python3
"""RECON-27: дрилл нового ТОП-1 entities/mobs-tick (13.9%/13.7% scene-CPU,
кросс-раннер стабилен, RECON-26) до под-лейнов >=5% на s7194+s7189.

Методика: deepest-DOMAIN-frame-wins внутри семьи — первый доменный фрейм
от листа (min/max фреймы-гварды jdk/GC скипаются), домены:
  ai/goals, nav/pathfinding, baseTick-common, mobTick, sensor/brain,
  spawn/aging, equipment/loot, look/move-control, misc-tick.

Использование: python3 scripts/bench4_recon/recon27_entities_drill.py
Док: research/gc-recon-2026-09-19/RECON27_ENTITIES_DRILL.md
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON27_ENTITIES_DRILL.md")

FAMILY_RX = re.compile(
    r"Mob\.tick|LivingEntity\.tick|Entity\.tick|baseTick|aiStep|serverAiStep|"
    r"GoalSelector|PathNavigation|Brain\b|Behavior|LookControl|MoveControl|"
    r"mobTick|doTick")

SKIP_RX = re.compile(
    r"it/unimi/dsi/fastutil|java/util|com/google/common|java/lang|jdk/internal|"
    r"sun/misc|G1|JVM_|me/lucko/spark|com/sun/jmx|Cgroup|CpuMonitor")

# домены внутри entities/mobs-tick: первый доменный фрейм от листа
DOMAINS = [
    ("nav/pathfinding", re.compile(
        r"PathNavigation|PathFinder|NodeEvaluator|Path\b|followThePath|"
        r"shouldTargetNextNode|moveControl|MoveControl")),
    ("ai/goals-selector", re.compile(
        r"GoalSelector|Goal;|WrappedGoal|targetGoal|getWalkTarget|"
        r"registerGoals|canUse|canContinueToUse|start\(|stop\(|tick\(\)V.*Goal")),
    ("brain/sensors", re.compile(
        r"Brain\b|Behavior|Sensor|MemoryModule|AiStep|activityRequirements")),
    ("look-control", re.compile(r"LookControl|getXRotD|setLookAt")),
    ("movement-ai", re.compile(
        r"MoveControl|GroundPathNavigation|WaterBoundPathNavigation|"
        r"moveControlTick|controller")),
    ("combat/attack", re.compile(
        r"MeleeAttackGoal|RangedAttackGoal|AttackGoal|doHurtTarget|swing|"
        r"performAttack")),
    ("aging/breed", re.compile(
        r"AgeableMob|Breed|ageUp|Animal;|getLoveCause|inLove")),
    ("sense-scan", re.compile(
        r"getNearestPlayer|hasLineOfSight|TargetingConditions|selectEntities|"
        r"getEntitiesOfClass|AabbCLear|EntityGetter")),
    ("equipment/inventory", re.compile(
        r"equip|Inventory|armor|getItemInHand|setItemSlot")),
    ("misc-mob-tick", re.compile(
        r"Mob\.tick|Monster\.tick|PathfinderMob\.tick|AmbientCreature|"
        r"WaterAnimal|BaseEntity")),
    ("baseTick/common", re.compile(
        r"LivingEntity\.baseTick|Entity\.baseTick|Entity\.tick|Mob\.tick|"
        r"LivingEntity\.tick|updateInWater|applyEffectsFromBlocks|"
        r"checkInsideBlocks|updateSwingTime|tickCount")),
]


def domain_of(stack_norm):
    for fr in reversed(stack_norm.split(";")):
        if SKIP_RX.search(fr):
            continue
        for name, rx in DOMAINS:
            if rx.search(fr):
                return name
        return "other:" + fr.split("/")[-1][:50]
    return "root-only"


def in_family(stack_norm):
    return bool(FAMILY_RX.search(stack_norm))


def main():
    lines = ["# RECON-27 — дрилл entities/mobs-tick до под-лейнов (>=5%)", ""]
    cuts = {}
    for name, rd in RUNS.items():
        path = os.path.join(rd, "cpu-collapsed.txt")
        if not os.path.exists(path):
            continue
        doms = Counter()
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
            sn = s.replace(".", "/")
            if in_family(sn):
                doms[domain_of(sn)] += n
        fam_tot = sum(doms.values())
        cuts[name] = (tot, doms)
        lines.append(f"## {name}: семья = {fam_tot} = {100.0*fam_tot/tot:.1f}% scene-CPU")
        lines.append("")
        lines.append("| домен | сэмплы | % scene-CPU | % семьи | >=5% сцены |")
        lines.append("|---|---|---|---|---|")
        for d, n in doms.most_common(12):
            mark = "ДА" if 100.0*n/tot >= 5.0 else ""
            lines.append(f"| {d} | {n} | {100.0*n/tot:.1f}% | {100.0*n/max(1,fam_tot):.1f}% | {mark} |")
        lines.append("")

    if len(cuts) == 2:
        (t1, d1), (t2, d2) = cuts["s7194"], cuts["s7189"]
        lines.append("## Кросс-раннер стабильность доменов")
        lines.append("")
        lines.append("| домен | s7194 % сцены | s7189 % сцены | стабильно |")
        lines.append("|---|---|---|---|")
        for d, _ in d1.most_common(12):
            a, b = 100.0*d1[d]/t1, 100.0*d2.get(d, 0)/t2
            lines.append(f"| {d} | {a:.1f}% | {b:.1f}% | {'ДА' if abs(a-b) <= 2.0 else 'НЕТ'} |")
        lines.append("")

    lines += [
        "## ВЫВОДЫ",
        "",
        "1. Под-лейны >=5% scene-CPU внутри нового ТОП-1 = кандидаты рычага",
        "   (прегистер: javap-контракт -> локстеп-оракул -> прегистер ->",
        "   диспатч, как в эпоху #12).",
        "2. other:*-домены ниже 1% не трогаются (чартер: побочные <5%).",
        "3. Если НЕТ домена >=5% — лейн entities/mobs-tick потенциально",
        "   fractal-мал: нужен deeper RECON (по классам мобов) до >=5%.",
        "",
    ]
    report = "\n".join(lines) + "\n"
    print(report)
    with open(DOC, "w") as f:
        f.write(report)
    print(f"DOC -> {DOC}")


if __name__ == "__main__":
    main()
