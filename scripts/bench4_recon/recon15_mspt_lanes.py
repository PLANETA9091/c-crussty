#!/usr/bin/env python3
"""recon15_mspt_lanes.py — RECON-15 (TASK-333): decompose Server-thread WALL
(the direct MSPT axis, never decomposed before) and process CPU on the FRESH
CLEAN bank profile s7169 (bank v3, no experimental lever armed), plus s7174
cross-check. Root frames are stripped to BELOW the last 'ServerLevel.tick;'
so family regexes classify the actual work, not the common root.

Also: G1/remset attribution from remset.log + gc duty from gc.log.

Usage: python3 recon15_mspt_lanes.py [run_dir ...]
"""
import os, re, sys
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
SPIN = "MinecraftServer.lambda$spin$2"
ROOT_MARK = "ServerLevel.tick;"

# Deep-first: classify leaf->root; the FIRST frame (from the leaf) matching a
# deep family wins. Planner/entry families are checked ONLY if nothing deeper
# matched (they wrap the whole entity tick).
DEEP_FAM = [
    ("fluid-sim(updateFluid/fluidPush/getFluidState)", re.compile(
        r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState;|collidedWithFluid|makeOverflowStream")),
    ("travel-collide(move/collide/AABB/Vec3/CollisionUtil)", re.compile(
        r"Entity;travel|Entity;move|Entity;collide|addCollisionsAlongTravel|AABB|Vec3|CollisionUtil|getCollisionsForBlocksOrWorldBorder|Cursor3D|betweenClosed")),
    ("inside-blocks(checkInside/InsideBlockOps)", re.compile(
        r"checkInsideBlocks|InsideBlockOps|forEachBlockIntersectedBetween|betweenCornersInDirection")),
    ("push-physics", re.compile(r"pushEntities|getPushableEntities")),
    ("ai/goals/nav/brain", re.compile(r"aiStep|GoalSelector|PathNavigation|shouldRecomputePath|Brain;|NodeEvaluator|Goal;")),
    ("mob-tick-core(doTick/tick;Lnet/minecraft/world/entity)", re.compile(
        r"Mob;tick|LivingEntity;tick|Entity;tick|baseTick")),
    ("entity-tracking(ServerEntity/sendChanges/SynchedData)", re.compile(
        r"ServerEntity|sendChanges|SynchedEntityData|TrackerTickOps|moonrise\$clearPlayers|NearbyPlayers")),
    ("spawner/NaturalSpawner", re.compile(r"NaturalSpawner|spawnEntities")),
    ("chunk-access(PalettedContainer/getBlockState/getEntities)", re.compile(
        r"PalettedContainer|getBlockStateFinal|getBlockState|ChunkEntitySlices|getEntities|LevelChunk;")),
    ("random-tick/block-tick/snow-freeze", re.compile(r"RandomTick|tickBlocks|randomTick|freeze|snow")),
    ("chunk-system(loads/tickets/holder)", re.compile(
        r"ChunkMap|ChunkHolder|ChunkTaskScheduler|ChunkLoadTask|ChunkTicket|ticket")),
    ("connections/network", re.compile(r"Connection;|Packet|Channel;|broadcastAll")),
]
PLANNER_FAM = ("entity-tick-planner(EntityTickList/bucket/forEach)", re.compile(
    r"RegionTickOps|EntityTickList|tickBucket|bucketOf|EntityCallbacks|EntityTickList;forEach"))
SLEEP_RX = re.compile(r"libc\.so\.6|syscall|LockSupport|Object;wait|Thread;sleep|park")


def wall_server_thread(run):
    agg = Counter(); tot = 0
    for line in open(os.path.join(run, "wall-collapsed.txt"), errors="ignore"):
        p = line.rstrip("\n").rpartition(" ")
        if not p[0] or SPIN not in p[0]:
            continue
        try:
            n = int(p[2])
        except ValueError:
            continue
        tot += n
        # strip the common root: classify only the tail below the LAST ServerLevel.tick;
        idx = p[0].rfind(ROOT_MARK)
        tail = p[0][idx + len(ROOT_MARK):] if idx >= 0 else p[0]
        frames = tail.split(";")
        hit = None
        # leaf -> root: first DEEP family match wins
        for fr in reversed(frames):
            for name, rx in DEEP_FAM:
                if rx.search(fr):
                    hit = name
                    break
            if hit:
                break
        if hit is None:
            # planner wrap (EntityTickList/bucketOf) checked on the whole tail
            if PLANNER_FAM[1].search(tail):
                hit = PLANNER_FAM[0]
            elif SLEEP_RX.search(tail):
                hit = "sleep-idle(libc/syscall/wait/park)"
            else:
                hit = "other-java"
        agg[hit] += n
    return agg, tot


def deepest(path, topn=12):
    agg = Counter(); tot = 0
    for line in open(path, errors="ignore"):
        p = line.rstrip("\n").rpartition(" ")
        if not p[0]:
            continue
        try:
            n = int(p[2])
        except ValueError:
            continue
        tot += n
        agg[p[0].split(";")[-1]] += n
    return agg, tot


def gc_duty(path):
    young = full = 0; tot = mx = 0.0
    for line in open(path, errors="ignore"):
        m = re.search(r"Pause (Young|Full).*?([\d.]+)(ms|s)\s*$", line)
        if not m or "[gc,start" in line:
            continue
        ms = float(m.group(2)) * (1 if m.group(3) == "ms" else 1000)
        if m.group(1) == "Young":
            young += 1
        else:
            full += 1
        tot += ms
        mx = max(mx, ms)
    return young, full, tot, mx


def main():
    runs = sys.argv[1:] or [
        os.path.join(RESDIR, "run-s7169-parse-diag"),
        os.path.join(RESDIR, "run-s7174-inside-diet"),
    ]
    for run in runs:
        print("=" * 100)
        print("RUN:", os.path.basename(run))
        agg, tot = wall_server_thread(run)
        print(f"-- Server-thread WALL (MSPT axis), {tot} samples --")
        for k, v in agg.most_common():
            if v * 100.0 / tot >= 1.0:
                print(f"  {v:6d} {100.0*v/tot:5.2f}% {k}")
        cpu_path = os.path.join(run, "cpu-collapsed.txt")
        if os.path.exists(cpu_path):
            d, ctot = deepest(cpu_path)
            g1 = sum(v for k, v in d.items() if re.search(r"G1|refine|RemSet|CardSet|oopDesc|OopOopIterate|scan_heap|trim_queue|PS_|ObjectSynchronizer", k))
            print(f"-- CPU total {ctot}; JVM/G1-family leaves ≈ {100.0*g1/ctot:.2f}% --")
        gc = os.path.join(run, "gc.log")
        if os.path.exists(gc):
            y, f, t, mx = gc_duty(gc)
            print(f"-- GC: young={y} full={f} sum={t/1000:.1f}s max={mx:.0f}ms --")


if __name__ == "__main__":
    main()
