#!/usr/bin/env python3
"""s7154_recon2.py — RECON-2: раскладка НЕ-entity main-tick (ServerLevel.tick
вне entity-цикла, 4221 сэмплов = 8.06% CPU в CUMULATIVE) до классов поведения
+ сводка broadphase/getEntities-семейства по всем entity-лейнам.

Вход:  research/inside-cache-2026-09-18/run-s7149b-cumulative/cpu-collapsed.txt
       (+ опционально лег 35341241628 для перекрёстной проверки)
Выход: research/fluid-dirty-2026-09-18/S7154_RECON2.md

Цель (S7-154): найти следующий attackable подлейн >= 5% total CPU внутри
последнего крупного непокрытого блока эры.
"""
import os
import sys
import collections

BASE = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
        "run-s7149b-cumulative/cpu-collapsed.txt")
LEG = ("/home/z/c-crussty/research/fluid-dirty-2026-09-18/"
       "run-s7153-fluid-dirty/cpu-collapsed.txt")
OUT = ("/home/z/c-crussty/research/fluid-dirty-2026-09-18/S7154_RECON2.md")

ENTITY_KEY = "ServerLevel.tickNonPassenger"
MAIN_KEY = "ServerLevel.tick;"
LOOP_KEY = "MinecraftServer.tickServer"

NATIVE_KEYS = (
    "G1", "OopOopIterateDispatch", "oopDesc::", "CompileBroker", "Compilation::",
    "thread_native_entry", "JavaThread::", "vdso", "libc.so.6", "GCTask",
    "PST::", "PSParallelCompact", "CardTable", "Monitor::", "Mutex::",
    "os::", "Semaphore::", "ParkEvent", "SafepointSynchronize", "Thread::",
    "WeakProcessor", "ReferenceProcessor", "JVM_", "ciEnv::", "ciMethod",
    "Node::", "Phase", "Matcher::", "RegMask", "Arena::", "Chunk::",
    "Runtime1::", "SharedRuntime::", "Interpreter", "vtable stub",
    "itable stub", "Deoptimization::", "nmethod::", "CodeCache", "StubQueue",
    "StringDedup", "NMethodSweeper", "HandshakeState", "VMThread::",
    "Taskqueue", "GrowableArray", "ResourceArea", "Metaspace",
    "ClassFileParser", "SystemDictionary", "ObjArrayKlass", "InstanceKlass",
    "ConstantPool", "LinkageError", "TypeProfile", "CallSite", "MethodHandle",
    "LambdaForm", "InvokerBytecodeGenerator", "libjvm.so", "libpthread",
    "PerfLong", "Event", "Jfr", "AllocTracer", "BarrierSet", "PSYoung",
    "PSCardTable", "ObjQueue", "PSAdj", "MachineCode", "JavaCall",
)


def is_native_frame(fr):
    return any(k in fr for k in NATIVE_KEYS)


def load(path):
    rows = []
    with open(path) as f:
        for ln in f:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            stack, _, w = ln.rpartition(" ")
            try:
                w = int(w)
            except ValueError:
                continue
            rows.append((stack, w))
    return rows


def split_entity(rows):
    ent = []
    other = []
    for stack, w in rows:
        (ent if ENTITY_KEY in stack else other).append((stack, w))
    return ent, other


def nonentity_main_class(stack):
    """Классификация стека, прошедшего через main tick (tickServer), но НЕ
    через entity-цикл. Якорь = первый net/minecraft|ca/spottedleaf кадр ниже
    ServerLevel.tick (или другой ветки tickChildren)."""
    frames = stack.split(";")
    # срезаем всё до и включая DedicatedServer.tickServer / MinecraftServer.tickServer
    start = 0
    for i, fr in enumerate(frames):
        if LOOP_KEY in fr:
            start = i + 1
            break
    for fr in frames[start:]:
        if is_native_frame(fr):
            continue
        if "MinecraftServer" in fr and ("tickChildren" in fr or "spin" in fr
                                        or "runServer" in fr or "lambda" in fr):
            continue
        if "DedicatedServer" in fr:
            continue
        # первый содержательный кадр после main-loop
        for k in ("ServerLevel.tick;", "ServerLevel.tick$"):
            pass
        if "net/minecraft/server/level/ServerLevel.tick;" in fr + ";":
            continue
        return fr
    return "(main-frame self)"


# детерминированные группы (anchor-first ниже ServerLevel.tick)
GROUP_RULES = (
    ("random-tick (blocks/sections)", ("randomTick", "RandomTick")),
    ("scheduled-ticks (LevelTicks/TickBlockOps)", ("LevelTicks", "TickBlockOps")),
    ("block-entities (tickers/RepeatingTickBlock)",
     ("blockEntity", "BlockEntity", "Repeating", "ticker")),
    ("chunk-system tick (moonrise spawn/tick chunks)",
     ("ServerChunkCache.tick", "ChunkSystem", "chunkTicks", "moonrise")),
    ("natural-spawn (SpawnerCreature/MobCategory)",
     ("SpawnerCreature", "NaturalSpawner", "MobCategory")),
    ("weather/thunder/lightning", ("thunder", "Thunder", "lightning", "Lightning",
                                   "rain", "Rain", "weather", "Weather")),
    ("game-events", ("GameEvent", "gameEvent")),
    ("raids/patrols/wandering", ("Raid", "raid", "Patrol", "patrol",
                                 "WanderingTrader")),
    ("portal/tickPOI/voxel", ("Poi", "POI", "portal", "Portal")),
    ("sleep/dragon/boss-global", ("DragonFight", "dragon", "EnderDragon",
                                 "sleep", "Sleep")),
    ("world-border/global-objects", ("WorldBorder", "border")),
)


def classify_nonentity_main(rows):
    groups = collections.Counter()
    anchors = collections.Counter()
    leaves = collections.Counter()
    total = 0
    for stack, w in rows:
        if ENTITY_KEY in stack:
            continue
        if LOOP_KEY not in stack:
            continue
        total += w
        top = nonentity_main_class(stack)
        groups[top] += w
        gname = None
        for name, keys in GROUP_RULES:
            if any(k in stack for k in keys):
                gname = name
                break
        groups[f"RULE::{gname}"] += w if gname else 0
        if gname is None and "net/minecraft" in stack:
            # якорь: первый net/minecraft кадр ниже main-loop
            anchor = nonentity_main_class(stack)
            anchors[anchor] += w
            # leaf: последний net/minecraft кадр
            frames = [f for f in stack.split(";")
                      if f.startswith("net/minecraft/") or f.startswith("ca/spottedleaf/")]
            if frames:
                leaves[frames[-1]] += w
    return total, groups, anchors, leaves


def broadphase_family(rows):
    """getEntities/broadphase/pushable-семейство по ВСЕМ стекам (entity + не-entity)."""
    fam = collections.Counter()
    for stack, w in rows:
        for key in ("ChunkEntitySlices$EntityCollectionBySection.getEntities",
                    "ChunkEntitySlices.getEntities",
                    "EntityGetter.getEntities",
                    "ServerLevel.getEntities",
                    "pushEntities",
                    "AABB.intersects",
                    "CollisionUtil.getCollisions",
                    "PartitionedEntitySection"):
            if key in stack:
                fam[key] += w
                break
    return fam


BROAD_KEYS = ("ChunkEntitySlices$EntityCollectionBySection.getEntities",
              "ChunkEntitySlices.getEntities",
              "CollisionUtil.getCollisions",
              "pushEntities")


def broad_caller_drill(rows):
    """Кто зовёт broadphase-семейство: якорь = класс-tick сущности (или
    не-entity), мид-кадр = ближайший вызывающий выше broadphase-кадра."""
    anchor = collections.Counter()
    caller = collections.Counter()
    total = 0
    for stack, w in rows:
        hit = None
        for k in BROAD_KEYS:
            if k in stack:
                hit = k
                break
        if hit is None:
            continue
        total += w
        frames = stack.split(";")
        idx = next(i for i, f in enumerate(frames) if hit in f)
        # класс-tick якорь
        a = "(non-entity)"
        for f in frames:
            if f.endswith(".tick") and f.startswith("net/minecraft/world/entity/"):
                a = f
                break
        anchor[a] += w
        # ближайший содержательный вызыватель выше
        c = "(root)"
        for f in reversed(frames[:idx]):
            if is_native_frame(f) or f.startswith("java/") or "Lambda" in f \
               or f.startswith("jdk/") or f.startswith("it/unimi/"):
                continue
            c = f
            break
        caller[c] += w
    return total, anchor, caller


def fmt_tbl(title, pairs, total, limit=30):
    out = [f"\n## {title}\n", "| элемент | samples | % фазы | % всего CPU |",
           "|---|---|---|---|"]
    for name, w in pairs[:limit]:
        out.append(f"| `{name}` | {w} | {100.0*w/max(total,1):.1f}% | "
                   f"{100.0*w/TOTAL_CPU:.2f}% |")
    return "\n".join(out)


def main():
    global TOTAL_CPU
    rows = load(BASE)
    TOTAL_CPU = sum(w for _, w in rows)
    ent, other = split_entity(rows)
    print(f"total={TOTAL_CPU} entity={sum(w for _,w in ent)} "
          f"non-entity={sum(w for _,w in other)}")

    total_ne, groups, anchors, leaves = classify_nonentity_main(rows)
    print(f"non-entity main-tick total={total_ne}")

    fam = broadphase_family(rows)
    print("\n=== broadphase/getEntities family (all stacks) ===")
    for k, v in fam.most_common(15):
        print(f"  {k}: {v} ({100.0*v/TOTAL_CPU:.2f}% CPU)")

    btotal, banchor, bcaller = broad_caller_drill(rows)
    print(f"\n=== broadphase drill (total {btotal} = "
          f"{100.0*btotal/TOTAL_CPU:.2f}% CPU) ===")
    print("-- anchors (class tick) --")
    for k, v in banchor.most_common(12):
        print(f"  {k}: {v} ({100.0*v/btotal:.1f}% fam)")
    print("-- callers (nearest above) --")
    for k, v in bcaller.most_common(15):
        print(f"  {k}: {v} ({100.0*v/btotal:.1f}% fam)")

    # перекрёстная проверка на леге
    if os.path.exists(LEG):
        lrows = load(LEG)
        ltotal = sum(w for _, w in lrows)
        lb = broadphase_family(lrows)
        lsum = sum(lb.values())
        print(f"\n=== LEG 35341241628 cross-check: total={ltotal} "
              f"broadphase-presence={lsum} ({100.0*lsum/ltotal:.2f}%) ===")

    # ядерная утилизация: распределение по потокам (первый кадр стека)
    thr = collections.Counter()
    for stack, w in rows:
        thr[stack.split(";")[0]] += w
    print(f"\n=== потоки (утеризация ядер, total {TOTAL_CPU}) ===")
    for k, v in thr.most_common(12):
        print(f"  {k[:90]}: {v} ({100.0*v/TOTAL_CPU:.1f}%)")

    print("\n=== anchors (non-entity main, first mc frame below main loop) ===")
    for k, v in anchors.most_common(25):
        print(f"  {k}: {v} ({100.0*v/TOTAL_CPU:.2f}% CPU)")

    print("\n=== leaves (non-entity main, last mc frame) ===")
    for k, v in leaves.most_common(25):
        print(f"  {k}: {v} ({100.0*v/TOTAL_CPU:.2f}% CPU)")

    if len(sys.argv) > 1 and sys.argv[1] == "--md":
        lines = ["# S7-154 RECON-2 — раскладка non-entity main-tick "
                 "(CUMULATIVE 35330129145)",
                 f"", f"- total CPU: **{TOTAL_CPU}**",
                 f"- non-entity main-tick (tickServer без entity-цикла): "
                 f"**{total_ne} = {100.0*total_ne/TOTAL_CPU:.2f}% CPU**",
                 fmt_tbl("Якоря (первый mc-кадр ниже main-loop)",
                         anchors.most_common(), total_ne),
                 fmt_tbl("Листья (последний mc-кадр)", leaves.most_common(),
                         total_ne),
                 fmt_tbl("broadphase/getEntities-семейство (все стеки)",
                         fam.most_common(), TOTAL_CPU),
                 ]
        open(OUT, "w").write("\n".join(lines) + "\n")
        print(f"\nwritten {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
