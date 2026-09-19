#!/usr/bin/env python3
# RECON-11: атрибуция аллок-семей (Vec3/AABB/BlockPos) по родительским call-деревьям v3.
# Цель: оценить адресуемую долю для рычага #13 (скаляризация live-полей сущностей =
# меньше old->young card-dirt + меньше живых объектов в oop-scan + меньше young-аллока).
import re, collections

RUN = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"
tot = 0
by_type = collections.defaultdict(lambda: collections.Counter())
STACK_OF_INTEREST = {
    "Vec3": re.compile(r"net\.minecraft\.world\.phys\.Vec3_\["),
    "AABB": re.compile(r"net\.minecraft\.world\.phys\.AABB_\["),
    "BlockPos-все": re.compile(r"net\.minecraft\.core\.BlockPos"),
    "ArrayList": re.compile(r"java\.util\.ArrayList_\["),
    "long[]": re.compile(r"long\[]_\["),
}

def key_frames(stack):
    """возвращает множество интересных фреймов-родителей в стеке"""
    fs = stack.split(";")
    hits = set()
    for f in fs:
        f = f.replace("/", ".")
        for m in [
            ("setDeltaMovement", r"\.setDeltaMovement"),
            ("setBoundingBox", r"\.setBoundingBox"),
            ("setPos/setPosRaw", r"\.setPos(Raw)?\("),
            ("getFlow/getFluid", r"getFlow|getFluidState|getHeight\("),
            ("updateFluid", r"updateFluidHeightAndDoFluidPushing"),
            ("collide/move-лестница", r"\.collide\(|\.move\(|performCollisions|getCollisionsForBlocksOrWorldBorder|collideBoundingBox"),
            ("inside/checkInside", r"checkInsideBlocks|applyEffectsFromBlocks|forEachBlockIntersected"),
            ("travel/handleRel", r"travel\(|handleRelativeFriction"),
            ("aiStep/tick-оркестр", r"aiStep|\.tick\(\)|entityTick"),
            ("sendChanges/tracker", r"sendChanges|ServerEntity"),
            ("spawning/деспавн", r"spawn|despawn|addEntity|removeEntity"),
            ("chunk/palette", r"PalettedContainer|LevelChunkSection|ChunkAccess"),
            ("goal/navigation", r"Goal|Navigation|createPath|RandomStroll"),
            ("sensing/brain", r"Sensing|Brain"),
        ]:
            name, rx = m
            if re.search(rx, f):
                hits.add(name)
    return hits

for line in open(f"{RUN}/alloc-collapsed.txt", encoding="utf-8", errors="replace"):
    m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
    if not m:
        continue
    stack, n = m.group(1), int(m.group(2))
    tot += n
    for tname, rx in STACK_OF_INTEREST.items():
        if rx.search(stack.split(";")[-1]):
            for h in key_frames(stack):
                by_type[tname][h] += n

for tname, counter in by_type.items():
    t = sum(counter.values())
    print(f"\n[{tname}] (покрыто интересными деревьями: {t} сэмплов)")
    for h, n in counter.most_common(12):
        print(f"   {n:6d}  {h}")
