#!/usr/bin/env python3
"""RECON-28: декомпозиция RegionTickOps.tickBucket-поддерева (11.9/12.2%
scene-CPU, RECON-27b) на ОРКЕСТРАЦИЮ-СОБСТВЕННО vs функциональные под-вызовы.
Вопрос: есть ли в оркестрации однородный лейн >=5% scene-CPU для ЭПОХА-2
flat batch dispatcher?

Методика (s7194, 128104 cpu-сэмплов; контроль s7189):
  - выборка: стеки, содержащие RegionTickOps.tickBucket;
  - deepest-leaf-классификация: функциональные маркеры (zeroin-fluid,
    broadphase-getEntities, inside-gate, chunk-access, volatile-reads,
    collision) vs оркестрационные хвосты (guard/lambda/ticklist/iter/scan);
  - оркестрация-собственно = сэмплы, чей лист-хвост НЕ функциональный.

Использование: python3 scripts/bench4_recon/recon28_tickbucket_decomp.py
Док: research/gc-recon-2026-09-19/RECON28_TICKBUCKET_DECOMP.md
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON28_TICKBUCKET_DECOMP.md")

# функциональные домены (лист-хвосты): работа, сохраняющаяся при любом диспетчере
FUNC = [
    ("zeroin-fluid", re.compile(
        r"ZeroAllocOps/updateFluidHeightAndDoFluidPushing|"
        r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState;|"
        r"PalettedContainer/get|LevelChunk/getFluidState|isEyeInFluid")),
    ("broadphase-getEntities", re.compile(
        r"getEntities|ChunkEntitySlices|EntityLookup|entityLookup|"
        r"AABB/intersects|collisionBySection")),
    ("inside-gate", re.compile(
        r"checkInsideBlocks|InsideBlockOps|InsideBlockEffectApplier|"
        r"applyEffectsFromBlocks")),
    ("collision/travel", re.compile(
        r"Entity/collide|Entity/move|Entity/travel|CollisionUtil|"
        r"performCollisions|TraverseOps")),
    ("volatile-reads", re.compile(
        r"VarHandle|getVolatile|VarHandleGuards|getValueVolatile|"
        r"SynchedEntityData")),
    ("sync/tracker", re.compile(
        r"ServerEntity|sendChanges|SynchedEntityData|Clientbound")),
    ("ai-pathfind", re.compile(
        r"PathNavigation|GoalSelector|Brain/|Behavior|LookControl|MoveControl|"
        r"aiStep|serverAiStep")),
    ("blockstate-access", re.compile(
        r"getBlockState|BlockBehaviour|BlockState;|ChunkAccess/getSection")),
    ("gc/jdk-leaf", re.compile(
        r"it/unimi/dsi/fastutil|java/util|com/google/common|G1|JVM_|"
        r"java/lang/Thread")),
]

# оркестрационные маркеры (хвост-фреймы)
ORCH_RX = re.compile(
    r"RegionTickOps|BatchCollector|guardEntityTick|tickNonPassenger|"
    r"EntityTickList|isRemoved|lambda\$ensureHelpers|lambda\$tick|"
    r"guardEntityTick|hasImpulse|canUpdate|accept\(")

# тело entity-тика: если ЭТО есть в стеке — сэмпл уже внутри моб-логики
BODY_RX = re.compile(
    r"Mob\.tick|LivingEntity\.tick|Entity\.tick;|ItemEntity\.tick|"
    r"AbstractBoat\.tick|baseTick|aiStep|serverAiStep|mobTick|"
    r"Monster\.tick|AgeableMob\.tick|PathfinderMob\.tick|ExperienceOrb\.tick|"
    r"Projectile\.tick|VehicleEntity|LivingEntity\.aiStep|Entity\.baseTick|"
    r"LivingEntity\.baseTick|Zombie\.tick|Skeleton\.tick|Sheep\.tick|"
    r"Chicken\.tick|Spider\.tick|Cow\.tick|Pig\.tick|ArmorStand\.tick")


def classify(stack_norm, n, tot, func, orch, tails, body):
    # тик-бакет поддерево?
    if "RegionTickOps/tickBucket" not in stack_norm:
        return "outside"
    fr = stack_norm.split(";")[-1]
    tails[fr.split("/")[-1][:60]] += n
    # (1) тело entity-тика присутствует -> функциональный контент моб-логики
    if BODY_RX.search(stack_norm):
        body[fr.split("/")[-1][:60]] += n
        for name, rx in FUNC:
            if rx.search(fr) or rx.search(";".join(stack_norm.split(";")[-4:])):
                func[name] += n
                return "func"
        func["body-other"] += n
        return "func"
    # (2) тело отсутствует -> оркестрационный контур tickBucket->guard->lambda
    orch[fr.split("/")[-1][:60]] += n
    return "orch"


def main():
    lines = ["# RECON-28 — декомпозиция RegionTickOps.tickBucket (оркестрация vs функции)", ""]
    res = {}
    for name, rd in RUNS.items():
        path = os.path.join(rd, "cpu-collapsed.txt")
        if not os.path.exists(path):
            continue
        tot = 0
        bucket = 0
        func = Counter()
        orch = Counter()
        tails = Counter()
        body = Counter()
        cls = Counter()
        for line in open(path, errors="ignore"):
            s, _, c = line.rstrip("\n").rpartition(" ")
            if not s:
                continue
            try:
                n = int(c)
            except ValueError:
                continue
            tot += n
            k = classify(s.replace(".", "/"), n, tot, func, orch, tails, body)
            if k == "func":
                bucket += n
                cls["func"] += n
            elif k == "orch":
                bucket += n
                cls["orch"] += n
        res[name] = (tot, bucket, cls, func, orch)
        lines.append(f"## {name}: tickBucket-поддерево = {bucket} = "
                     f"{100.0*bucket/tot:.1f}% scene-CPU из {tot}")
        lines.append("")
        lines.append(f"- функциональные под-вызовы: {cls['func']} = "
                     f"{100.0*cls['func']/tot:.1f}% сцены")
        lines.append(f"- **оркестрация-собственно: {cls['orch']} = "
                     f"{100.0*cls['orch']/tot:.1f}% сцены**")
        lines.append("")
        lines.append("### Функциональный разрез")
        lines.append("")
        lines.append("| домен | сэмплы | % сцены |")
        lines.append("|---|---|---|")
        for d, n in func.most_common():
            lines.append(f"| {d} | {n} | {100.0*n/tot:.1f}% |")
        lines.append("")
        lines.append("### Оркестрационный контур (топ-10 лист-хвостов, тело entity-тика ОТСУТСТВУЕТ)")
        lines.append("")
        lines.append("| хвост | сэмплы | % сцены |")
        lines.append("|---|---|---|")
        for d, n in orch.most_common(10):
            lines.append(f"| {d} | {n} | {100.0*n/tot:.1f}% |")
        lines.append("")
        lines.append("### Топ-8 лист-хвостов всего поддерева")
        lines.append("")
        lines.append("| хвост | сэмплы | % сцены |")
        lines.append("|---|---|---|")
        for d, n in tails.most_common(8):
            lines.append(f"| {d} | {n} | {100.0*n/tot:.1f}% |")
        lines.append("")

    if len(res) == 2:
        (t1, b1, c1, f1, o1), (t2, b2, c2, f2, o2) = res["s7194"], res["s7189"]
        lines.append("## Кросс-раннер")
        lines.append("")
        lines.append(f"- поддерево: s7194 {100.0*b1/t1:.1f}% vs s7189 {100.0*b2/t2:.1f}%")
        lines.append(f"- оркестрация: s7194 {100.0*c1['orch']/t1:.1f}% vs "
                     f"s7189 {100.0*c2['orch']/t2:.1f}%")
        lines.append(f"- функции: s7194 {100.0*c1['func']/t1:.1f}% vs "
                     f"s7189 {100.0*c2['func']/t2:.1f}%")
        lines.append("")

    lines += [
        "## ВЫВОДЫ (прегистер ЭПОХА-2)",
        "",
        "1. Оркестрация-собственно = целевая доля flat batch dispatcher;",
        "   функциональные под-вызовы остаются при любом диспетчере.",
        "2. Если оркестрация >=5% сцены: рычаг ЭПОХА-2 = компаньонный flat-цикл",
        "   (inline isRemoved/guard-гейты, прямой entity.tick() вместо",
        "   lambda$ensureHelpers$4 -> accept -> guardEntityTick -> lambda$tick$4",
        "   цепочки, батч-префильтр removed/passenger до цикла; семантика",
        "   per-entity try-catch сохраняется бит-в-бит копией guard-логики;",
        "   порядок обхода median-exact сохранён той же EntityTickList",
        "   итерацией).",
        "3. Если оркестрация <5%: точечный рычаг запрещён чартером ->",
        "   системный вывод о fractal-равновесии сцены финализируется доком.",
        "",
    ]
    report = "\n".join(lines) + "\n"
    print(report)
    with open(DOC, "w") as f:
        f.write(report)
    print(f"DOC -> {DOC}")


if __name__ == "__main__":
    main()
