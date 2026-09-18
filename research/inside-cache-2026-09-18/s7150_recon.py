#!/usr/bin/env python3
"""s7150_recon.py — S7-150 пост-эра: ранжирование под-лейнов entity-tick
и unclassified по свежему профилю CUMULATIVE (ран 35330129145,
inside_cache=1 + flush_diet=1, валидная зелёная база эры).

Вход:  research/inside-cache-2026-09-18/run-s7149b-cumulative/cpu-collapsed.txt
Выход: research/inside-cache-2026-09-18/S7150_RECON.md

Классификация (path-based, leaf-first; документирована в самом скрипте):
  ENTITY lane  : стек проходит через ServerLevel.tickNonPassenger
                 (внутри guardEntityTick → lambda цепочки тика сущностей)
  SUB-LANE     : в пределах entity-стека первый якорь ниже
                 net/minecraft/world/entity/item/ItemEntity.tick
                 (или другого класс-tick-кадра) по списку приоритетов.
  UNCLASSIFIED : прочие стеки главного тика (не entity), группируются
                 по корням поддеревьев (сеть, chunk system, JIT, GC,
                 LevelTicks, block entities, misc-main).

Выходные таблицы:
  1. Топ-25 attackable kernel-функций по self-time (без JVM/GC/native).
  2. Разбивка entity-фазы по классам сущностей (tick-кадр класса).
  3. Под-лейны ItemEntity.tick (якорные группы ниже tick-кадра).
  4. Под-лейны Zombie.tick.
  5. Группы unclassified-фазы.
"""
import re
import os
import sys
import collections

RUN_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "run-s7149b-cumulative")
CPU = os.path.join(RUN_DIR, "cpu-collapsed.txt")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "S7150_RECON.md")

# --- классификация native/JVM-internal (не attackable архитектурой) ---
NATIVE_KEYS = (
    "G1", "OopOopIterateDispatch", "oopDesc::", "CompileBroker", "Compilation::",
    "thread_native_entry", "JavaThread::", "vdso", "libc.so.6", "GCTask",
    "PST::", "PSParallelCompact", "CardTable", "Monitor::", "Mutex::",
    "os::", "Semaphore::", "ParkEvent", "SafepointSynchronize", "Thread::",
    "WeakProcessor", "ReferenceProcessor", "JVM_", "ciEnv::", "ciMethod",
    "Node::", "Phase", "Matcher::", "RegMask", "Arena::", "Chunk::",
    "void Runtime1::", "SharedRuntime::", "Interpreter", "vtable stub",
    "itable stub", "native/JVM-internal", "Deoptimization::", "nmethod::",
    "CodeCache", "StubQueue", "StringDedup", "NMethodSweeper", "HandshakeState",
    "VMThread::", "CompilerOracle", "Taskqueue", "GrowableArray", "ResourceArea",
    "Metaspace", "ClassFileParser", "SystemDictionary", "Dictionary::",
    "ObjArrayKlass", "InstanceKlass", "ConstantPool", "LinkageError",
    "resolve", "verification", "TypeProfile", "CallSite", "MethodHandle",
    "VarHandleAbstractFieldArray", "DirectBuffer", "Cleaner", "Bits::",
    "AsyncGetCallTrace", "profiler", "signal", "JavaCalls::", "Bytecodes::",
    "Tiered", "AdvancedThresholdPolicy", "Compiler", "MemRegion", "FreeRegion",
    "HeapRegion", "G1RemSet", "G1CardSet", "G1ConcurrentRefine", "G1CMTask",
    "G1ParScanThreadState", "G1ScanCardClosure", "G1ScanHRForRegionClosure",
    "G1RebuildRSAndScrubTask", "G1RootProcessor", "G1CollectedHeap",
    "SplitKlass", "trim_queue", "scan_heap_roots", "do_heap_region",
    "drain", "evacuate", "copy_strong_roots", "process_grey_task_entry",
)

ENTITY_LOOP_KEY = "ServerLevel.tickNonPassenger"
ENTITY_TICK_PREFIX = "net/minecraft/world/entity/"

# якоря под-лейнов ниже класс-tick-кадра, приоритет сверху вниз
SUB_LANE_ANCHORS = [
    ("merge-search (getEntities/broadphase)", (
        "getEntitiesOfClass", "ChunkEntitySlices", "EntityCollectionBySection",
        "getEntities", "getEntityGetter", "moonrise$getEntities")),
    ("fluid-push/height", (
        "updateFluidHeightAndDoFluidPushing", "collidedWithFluid",
        "getFluidState", "updateInWaterStateAndDoFluidPushing")),
    ("move/collision", (
        "Entity.move", "CollisionUtil", "AABB.intersects",
        "collideWithShapes", "collide", "getCollisionsForBlocksOrWorldBorder",
        "collectCollisions", "LivingEntity.travel", "moveRelative")),
    ("inside-blocks", (
        "checkInsideBlocks", "forEachBlockIntersectedBetween",
        "InsideBlockEffectApplier")),
    ("palette/blockstate reads", (
        "PalettedContainer", "SimpleBitStorage", "getBlockState",
        "LevelChunk.getFluid", "getChunkNow", "readPalette")),
    ("data-watcher/sync", (
        "SynchedEntityData", "sendChanges", "ServerEntity")),
    ("AI/goals/brain", (
        "GoalSelector", "Brain", "Behavior", "PathNavigation", "goal",
        "attribute", "AttributeMap", "Sensing")),
    ("item/stack logic", (
        "ItemStack", "IItemCapable", "mergeWithNeighbours", "tryMergeTo",
        "ItemEntity.tick", "getPickupDelay", "hasPickUpDelay")),
    ("base-tick misc", (
        "Entity.baseTick", "Entity.tick", "setOldPos", "playStepSound",
        "walkAnimation", "applyEffectsFromBlocks", "fireImmune",
        "updateSwingTime", "checkFallDamage", "isInWater", "isInLava",
        "handleAirSupply", "updateAirSupply")),
]

TOTAL = 0
STACKS = []  # (stack_str, n, leaf)


def parse():
    global TOTAL
    with open(CPU, encoding="utf-8", errors="replace") as fh:
        for ln in fh:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            m = re.match(r"^(.*)\s(\d+)$", ln)
            if not m:
                continue
            stack, n = m.group(1), int(m.group(2))
            leaf = stack.rsplit(";", 1)[-1]
            TOTAL += n
            STACKS.append((stack, n, leaf))


def is_native(stack):
    return any(k in stack for k in NATIVE_KEYS)


def entity_class_tick(stack):
    """Вернуть класс-tick-кадр сущности (если стек в entity-цикле)."""
    if ENTITY_LOOP_KEY not in stack:
        return None, None
    frames = stack.split(";")
    idx = None
    for i, f in enumerate(frames):
        if ENTITY_LOOP_KEY in f:  # кадр = net/minecraft/server/level/ServerLevel.tickNonPassenger
            idx = i
            break
    if idx is None:
        return None, None
    # первый класс-tick ниже tickNonPassenger
    for f in frames[idx + 1:]:
        if f.startswith(ENTITY_TICK_PREFIX) and f.endswith(".tick") and "$" not in f.split("/")[-1]:
            return f, frames
    # пассажиры/редко: любой *.tick ниже
    for f in frames[idx + 1:]:
        if f.startswith(ENTITY_TICK_PREFIX) and f.endswith(".tick"):
            return f, frames
    return "entity-loop (no class tick)", frames


def sub_lane(frames, anchor_idx):
    """Первый якорь ниже anchor_idx; иначе self/misc."""
    deeper = frames[anchor_idx + 1:]
    # leaf == сам tick-кадр → self
    if not deeper:
        return "self (tick body)"
    for name, keys in SUB_LANE_ANCHORS:
        for f in deeper:
            if any(k in f for k in keys):
                return name
    return "sub-lane misc (unanchored)"


def main():
    parse()

    # --- 1. топ attackable kernel-листьев ---
    top_leaf = collections.Counter()
    native_total = 0
    java_total = 0
    for stack, n, leaf in STACKS:
        if is_native(stack):
            native_total += n
        else:
            java_total += n
            top_leaf[leaf] += n

    # --- 2. entity-фаза по классам ---
    class_cnt = collections.Counter()
    entity_total = 0
    for stack, n, leaf in STACKS:
        cls, frames = entity_class_tick(stack)
        if cls is not None:
            entity_total += n
            class_cnt[cls] += n

    # --- 3/4. под-лейны ItemEntity.tick и Zombie.tick ---
    item_sub = collections.Counter()
    item_total = 0
    zombie_sub = collections.Counter()
    zombie_total = 0
    for stack, n, leaf in STACKS:
        if ENTITY_LOOP_KEY not in stack:
            continue
        frames = stack.split(";")
        for target, store in (("item/ItemEntity.tick", None), ("monster/Zombie.tick", None)):
            pass
        # ItemEntity
        if any(f == "net/minecraft/world/entity/item/ItemEntity.tick" for f in frames):
            idx = frames.index("net/minecraft/world/entity/item/ItemEntity.tick")
            item_total += n
            item_sub[sub_lane(frames, idx)] += n
        # Zombie
        if any(f == "net/minecraft/world/entity/monster/Zombie.tick" for f in frames):
            idx = frames.index("net/minecraft/world/entity/monster/Zombie.tick")
            zombie_total += n
            zombie_sub[sub_lane(frames, idx)] += n

    # --- 5. unclassified-группы ---
    uncl = collections.Counter()
    uncl_total = 0
    main_misc_leaves = collections.Counter()
    main_misc_anchors = collections.Counter()
    main_misc_total = 0
    for stack, n, leaf in STACKS:
        cls, _ = entity_class_tick(stack)
        if cls is not None:
            continue
        uncl_total += n
        is_main_misc = False
        if "ServerLevel.tick;" in stack + ";" and ENTITY_LOOP_KEY not in stack:
            uncl["ServerLevel.tick (вне entity-цикла)"] += n
        elif "LevelTicks.tick" in stack:
            uncl["world-ticks (LevelTicks block/fluid)"] += n
        elif "blockEntity" in stack or "BlockEntity" in stack:
            uncl["block entities"] += n
        elif "tickChildren" in stack and "ServerLevel.tick" not in stack:
            uncl["tickChildren (прочие уровни/ветки)"] += n
        elif "netty" in stack or "NetworkSystem" in stack or "Connection" in stack or "Packet" in stack or "packet" in stack:
            uncl["network"] += n
        elif "chunk_system" in stack or "ChunkHolder" in stack or "ChunkMap" in stack or "moonrise" in stack or "ChunkTaskScheduler" in stack or "NewChunkHolder" in stack:
            uncl["chunk system"] += n
        elif "CompileBroker" in stack or "JIT" in stack:
            uncl["JIT"] += n
        elif is_native(stack):
            uncl["native/JVM (GC, runtime, threads)"] += n
        elif "MinecraftServer.tickServer" in stack or "tickServer" in stack:
            uncl["main-tick misc (server)"] += n
            is_main_misc = True
        else:
            uncl["other threads/misc"] += n
        if is_main_misc:
            main_misc_total += n
            main_misc_leaves[leaf] += n
            for f in stack.split(";"):
                if f.startswith("net/minecraft/") and not f.endswith(".run") \
                        and "tickServer" not in f and "runServer" not in f \
                        and "spin" not in f and "Lambda" not in f:
                    main_misc_anchors[f] += n

    pct = lambda a, b: (100.0 * a / b) if b else 0.0

    lines = []
    w = lines.append
    w("# S7-150 RECON — ранжирование пост-эры по CUMULATIVE 35330129145")
    w("")
    w("- вход: `run-s7149b-cumulative/cpu-collapsed.txt`, всего self-time сэмплов: **%d**" % TOTAL)
    w("- JVM-Java (attackable): **%d** (%.1f%%) · native/JVM-internal: **%d** (%.1f%%)" % (
        java_total, pct(java_total, TOTAL), native_total, pct(native_total, TOTAL)))
    w("- entity-фаза (стеки через tickNonPassenger): **%d** (%.1f%%)" % (
        entity_total, pct(entity_total, TOTAL)))
    w("- unclassified-фаза (не-entity стеки): **%d** (%.1f%%)" % (
        uncl_total, pct(uncl_total, TOTAL)))
    w("")
    w("## 1. Топ-25 attackable kernel-функций (self-time, без native/JVM)")
    w("")
    w("| leaf frame | samples | %% of total |")
    w("|---|---|---|")
    for leaf, n in top_leaf.most_common(25):
        w("| `%s` | %d | %.2f%% |" % (leaf, n, pct(n, TOTAL)))
    w("")
    w("## 2. Entity-фаза по классам сущностей (stack presence под tickNonPassenger)")
    w("")
    w("| класс-tick | samples | % entity-фазы | % всего CPU |")
    w("|---|---|---|---|")
    for cls, n in class_cnt.most_common(15):
        w("| `%s` | %d | %.1f%% | %.2f%% |" % (cls, n, pct(n, entity_total), pct(n, TOTAL)))
    w("")
    w("## 3. Под-лейны ItemEntity.tick (топ-1 класс, presence %d = %.2f%% CPU)" % (
        item_total, pct(item_total, TOTAL)))
    w("")
    w("| под-лейн | samples | % лейна |")
    w("|---|---|---|")
    for name, n in item_sub.most_common(20):
        w("| %s | %d | %.1f%% |" % (name, n, pct(n, item_total)))
    w("")
    w("## 4. Под-лейны Zombie.tick (presence %d = %.2f%% CPU)" % (
        zombie_total, pct(zombie_total, TOTAL)))
    w("")
    w("| под-лейн | samples | % лейна |")
    w("|---|---|---|")
    for name, n in zombie_sub.most_common(20):
        w("| %s | %d | %.1f%% |" % (name, n, pct(n, zombie_total)))
    w("")
    w("## 5. Unclassified-фаза: группировка по корням (%d = %.1f%% CPU)" % (
        uncl_total, pct(uncl_total, TOTAL)))
    w("")
    w("| группа | samples | % фазы | % всего CPU |")
    w("|---|---|---|---|")
    for name, n in uncl.most_common(25):
        w("| %s | %d | %.1f%% | %.2f%% |" % (name, n, pct(n, uncl_total), pct(n, TOTAL)))
    w("")
    w("### 5b. main-tick misc (%d = %.1f%% CPU): топ-12 якорей и топ-12 листьев" % (
        main_misc_total, pct(main_misc_total, TOTAL)))
    w("")
    w("| якорь (net/minecraft кадры, stack presence) | samples |")
    w("|---|---|")
    for name, n in main_misc_anchors.most_common(12):
        w("| `%s` | %d |" % (name, n))
    w("")
    w("| лист | samples |")
    w("|---|---|")
    for name, n in main_misc_leaves.most_common(12):
        w("| `%s` | %d |" % (name, n))
    w("")

    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    print("\nOK →", OUT)


if __name__ == "__main__":
    main()
