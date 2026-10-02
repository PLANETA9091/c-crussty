#!/usr/bin/env python3
"""recon3_s7160.py — S7-160 RECON-3: decomposition of the entity-phase
RESIDUAL sub-lane (39.1% of phase ~ 21% CPU in CUMULATIVE v2, leg #5
35381522360) down to micro-domains. Owner methodology «ТОП-ПОЖИРАТЕЛЬ
-> ∞» (addition 2026-09-19 00:51 +08): fresh artifacts, top-down order,
sub-lanes >=5% of the phase are attackable targets.

Attribution rule: each residual stack is attributed to its DEEPEST
net/minecraft frame (the leaf where CPU actually burned); JVM/native
leafs attribute to the deepest java frame below them.

Artifacts:
  leg#5  = research/region-threads-2026-09-18/run-s7159-leg5-artifact
  basev1 = research/inside-cache-2026-09-18/run-s7149b-cumulative
"""
import os
import sys
import collections

LEG5 = ("/home/z/c-crussty/research/region-threads-2026-09-18/"
        "run-s7159-leg5-artifact")
BASEV1 = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
          "run-s7149b-cumulative")

SUB_LANES = [
    ("fluid-scan", lambda frs: any(
        "updateFluidHeightAndDoFluidPushing" in f or "FluidPushOps" in f
        for f in frs)),
    ("inside-blocks", lambda frs: any(
        "InsideBlockOps" in f or "insideBlock" in f for f in frs)),
    ("broadphase/collision/push", lambda frs: any(
        ".noCollision" in f or ".collide" in f or "push(" in f
        or "getEntities" in f or "getEntityCollisions" in f
        or "EntityLookup" in f or "EntityCollectionBySection" in f
        or "AABB" in f for f in frs)),
    ("movement-integration", lambda frs: any(
        "Entity.move" in f or "LivingEntity.travel" in f
        or ".moveRelative" in f for f in frs)),
    ("navigation/pathfinding", lambda frs: any(
        "PathNavigation" in f or "PathFinder" in f or "moveControl" in f
        for f in frs)),
    ("AI/goal-selector/brain/sensing", lambda frs: any(
        "goalSelector" in f or "world/entity/ai/" in f
        or "Brain" in f or "Behavior" in f or "Sensor" in f or "Sensing" in f
        for f in frs)),
]


def load_collapsed(path):
    total = 0
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
            total += w
            rows.append((stack, w))
    return total, rows


def in_phase(frs):
    return any("RegionTickOps.tickBucket" in f or "tickNonPassenger" in f
               for f in frs)


def is_worker(frs):
    return any("RegionTickOps.tickBucket" in f for f in frs) and not any(
        "MinecraftServer.tickServer" in f for f in frs)


def phase_split(rows, total):
    phase_total = 0
    lane_w = collections.Counter()
    residual = []
    for s, w in rows:
        frs = s.split(";")
        if not in_phase(frs):
            continue
        phase_total += w
        for name, pred in SUB_LANES:
            if pred(frs):
                lane_w[name] += w
                break
        else:
            lane_w["RESIDUAL"] += w
            residual.append((s, w))
    return phase_total, lane_w, residual


def deepest_mc(frs):
    """Deepest net/minecraft frame = attribution point."""
    for f in reversed(frs):
        if f.startswith("net/minecraft/") or f.startswith("ca/spottedleaf/"):
            return f
    return frs[-1] if frs else "<none>"


# residual micro-domains: FIRST match on the attribution frame
DOMAINS = [
    ("fluid-residual (eyes/height/fluidmap/sections)", [
        "updateFluidOnEyes", "isEyeInFluid", "getFluidHeight",
        "wasEyeInWater", "updateSwimming", "FluidState", "getSections",
        "PalettedContainer", "getBlockState", "getBlockIfLoaded",
        "getBlockFloorLevel", "getFluid", "touchingUnloadedChunk",
        "hasChunksAt", "moonrise$areChunksLoaded"]),
    ("data-sync (SynchedEntityData)", [
        "SynchedEntityData", "DataItem", "entityData", "getItem("]),
    ("oldpos-rotate (setOldPos/setOldRot)", [
        "setOldPos", "setOldRot", "setOldPosAndRot"]),
    ("env-lit/collision-lite (isInWall/isFree/suffocate)", [
        "isInWall", "isFree", "wouldNotSuffocate", "makeStuckInBlock",
        "Collisions", "collideBoundingBox", "getBlockCollisions",
        "isAllEmpty", "noBlockCollision", "findCollidable"]),
    ("fall/ground (checkFallDamage/onClimbable/onGround)", [
        "checkFallDamage", "onClimbable", "ladder", "fallDamage",
        "updateGround", "isOnGround", "setOnGround"]),
    ("fire/freeze/damage-env (fireImmune/lavaHurt/powderSnow)", [
        "fireImmune", "lavaIgnite", "lavaHurt", "powderSnow", "freezeTicks",
        "isInLava", "isInPowderSnow", "igniteForSeconds", "burnFromLava"]),
    ("lifecycle (checkDespawn/remove/discard/register)", [
        "checkDespawn", "checkOutOfWorld", "belowVoid", "discard",
        "setRemoved", "unregister", "onClientRemoval"]),
    ("portal/dimension (handlePortal/portalTick)", [
        "handlePortal", "portalTick", "changeDimension", "portalProcess"]),
    ("ride/passenger (isPassenger/rootVehicle)", [
        "isPassenger", "rootVehicle", "getVehicle", "passengerTick",
        "removeVehicle", "isControlledByLocalInstance"]),
    ("sends/mail (sync hash / no-action)", [
        "hasImpulse", "isRemoved", "sendDirtyEntityData", "uuid",
        "getStringUUID", "shouldInformPlayers"]),
    ("tick-orchestration frames (tick/baseTick/aiStep self)", [
        "Entity.baseTick", "Entity.tick", "ItemEntity.tick", "Mob.tick",
        "LivingEntity.tick", "Monster.tick", "Zombie.tick",
        "guardEntityTick", "tickNonPassenger", "lambda$tick$4"]),
]


def domain_of(attr):
    for name, keys in DOMAINS:
        if any(k in attr for k in keys):
            return name
    return "misc-residual"


def residual_recon(residual, total, phase_total, tag):
    attr_w = collections.Counter()
    dom_w = collections.Counter()
    stack_w = collections.Counter()
    worker_w = 0
    for s, w in residual:
        frs = s.split(";")
        a = deepest_mc(frs)
        attr_w[a] += w
        dom_w[domain_of(a)] += w
        stack_w[s] += w
        if is_worker(frs):
            worker_w += w
    res_total = sum(attr_w.values())
    print(f"\n==== RESIDUAL RECON: {tag} ====")
    print(f"residual total={res_total} ({100.0*res_total/total:.2f}% CPU, "
          f"{100.0*res_total/phase_total if phase_total else 0:.1f}% of "
          f"entity-phase); worker share={100.0*worker_w/max(res_total,1):.1f}%")
    print(f"-- micro-domains (first-match on deepest MC frame):")
    for n, w in dom_w.most_common():
        print(f"   {n:58s} {w:7d}  {100.0*w/total:5.2f}% CPU "
              f"({100.0*w/phase_total if phase_total else 0:5.1f}% phase)")
    print(f"-- top-30 attribution frames (deepest net/minecraft):")
    for a, w in attr_w.most_common(30):
        print(f"   {w:7d}  {100.0*w/total:5.2f}%  {a[:120]}")
    print(f"-- top-12 residual full stacks:")
    for s, w in stack_w.most_common(12):
        print(f"   {w:7d}  {s[-230:]}")
    return dom_w, attr_w


def main():
    legdir = sys.argv[1] if len(sys.argv) > 1 else LEG5
    basedir = sys.argv[2] if len(sys.argv) > 2 else BASEV1
    results = {}
    for tag, d in (("base v1 (35330129145)", basedir),
                   ("leg#5 v2 (35381522360)", legdir)):
        cpu = os.path.join(d, "cpu-collapsed.txt")
        if not os.path.exists(cpu):
            print(f"!! {tag}: no cpu-collapsed.txt at {cpu}")
            continue
        total, rows = load_collapsed(cpu)
        phase_total, lane_w, residual = phase_split(rows, total)
        print(f"\n######## {tag} — total={total}, entity-phase="
              f"{phase_total} ({100.0*phase_total/total:.2f}% CPU) ########")
        for n, w in lane_w.most_common():
            lbl = n if n != "RESIDUAL" else "RESIDUAL(to recon below)"
            print(f"   {lbl:55s} {w:7d}  {100.0*w/total:5.2f}% CPU "
                  f"({100.0*w/phase_total:5.1f}% phase)")
        dom_w, attr_w = residual_recon(residual, total, phase_total, tag)
        results[tag] = (total, phase_total, lane_w, dom_w)
    if len(results) == 2:
        (bt, bp, bl, bd), (lt, lp, ll, ld) = results.values()
        print("\n==== DELTA base v1 -> leg#5 v2 (residual domains) ====")
        keys = set(bd) | set(ld)
        for k in sorted(keys, key=lambda x: -ld.get(x, 0)):
            b, l = bd.get(k, 0), ld.get(k, 0)
            dpct = (100.0 * l / lt) - (100.0 * b / bt)
            print(f"   {k:58s} {b:6d}->{l:6d}  CPU% {100.0*b/bt:5.2f}->"
                  f"{100.0*l/lt:5.2f} ({dpct:+5.2f}pp)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
