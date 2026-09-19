#!/usr/bin/env python3
"""recon9_travel_alloc.py — RECON-9: аллокационная и CPU-атрибуция travel-пути
(рычаг #11 ZERO-ALLOC-TRAVEL, кандидат из RECON-8) на банковом профиле CUMULATIVE v3.

Методика «ТОП-ПОЖИРАТЕЛЬ → ∞»: двойная ось (CPU из cpu-collapsed, alloc из
alloc-collapsed), exact deepest-match по полному фрейму.

Вердикт-входные данные: ceiling = сумма не-атакованных аллокаторных сайтов
travel-семьи; сравнение с полным alloc-давлением и young-GC STW wall-share.
"""
import collections

V = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"

CORE = [
    ("collidedWithFluid", "/Entity.collidedWithFluid"),
    ("collidedShapeMovingFrom", "/Entity.collidedWithShapeMovingFrom"),
    ("AABB.collidedAlongVector", "/AABB.collidedAlongVector"),
    ("checkInsideBlocks", "/Entity.checkInsideBlocks"),
    ("applyEffectsFromBlocks", "/Entity.applyEffectsFromBlocks"),
    ("Entity.collide", "/Entity.collide"),
    ("Entity.move", "/Entity.move"),
    ("LivingEntity.travel", "/LivingEntity.travel"),
    ("travelInFluid", "/LivingEntity.travelInFluid"),
    ("travelInAir", "/LivingEntity.travelInAir"),
    ("handleRelFrict", "handleRelativeFrictionAndCalculateMovement"),
    ("moveRelative", "/Entity.moveRelative"),
    ("findSupportingBlock", "Level.findSupportingBlock"),
    ("setPosRaw", "/Entity.setPosRaw"),
    ("makeBoundingBox", "makeBoundingBox"),
    ("CollisionUtil", "CollisionUtil"),
    ("PalettedContainer.get", "PalettedContainer.get"),
]

def rows(path):
    for line in open(path, errors="replace"):
        line = line.strip()
        if not line:
            continue
        m = line.rsplit(" ", 1)
        if len(m) != 2:
            continue
        try:
            w = int(m[1])
        except ValueError:
            continue
        yield w, m[0].split(";")

def deepest(frs):
    for f in reversed(frs[:-1]):
        for name, pat in CORE:
            if pat in f:
                return name
    return "unmatched"

def main():
    # ---- alloc axis: AABB+Vec3 leafs under entity tick ----
    site = collections.Counter()
    ent_alloc_tot = 0
    for w, frs in rows(V + "/alloc-collapsed.txt"):
        s = ";".join(frs)
        leaf = frs[-1]
        if not (leaf.startswith("net.minecraft.world.phys.AABB") or
                leaf.startswith("net.minecraft.world.phys.Vec3")):
            continue
        if not ("tickBucket" in s or "tickNonPassenger" in s):
            continue
        ent_alloc_tot += w
        site[deepest(frs)] += w
    print(f"entity-tick AABB+Vec3 alloc: {ent_alloc_tot} samples")
    for k, v in site.most_common(20):
        print(f"  {v:5d} ({100.0*v/ent_alloc_tot:4.1f}%)  {k}")

    travel_nonattacked = sum(v for k, v in site.items() if k in (
        "Entity.collide", "Entity.move", "LivingEntity.travel", "travelInFluid",
        "travelInAir", "handleRelFrict", "moveRelative", "findSupportingBlock",
        "setPosRaw", "makeBoundingBox", "unmatched"))
    print(f"\ntravel-family + unmatched (не-атаковано #10): {travel_nonattacked} "
          f"({100.0*travel_nonattacked/ent_alloc_tot:.1f}% entity AABB+Vec3)")

    # ---- whole-server alloc total ----
    tot_all = 0
    for w, frs in rows(V + "/alloc-collapsed.txt"):
        tot_all += w
    print(f"whole-server alloc samples: {tot_all}; "
          f"entity AABB+Vec3 share = {100.0*ent_alloc_tot/tot_all:.1f}%")

    # ---- CPU axis ----
    cpu = collections.Counter()
    tot = ent = 0
    for w, frs in rows(V + "/cpu-collapsed.txt"):
        s = ";".join(frs)
        tot += w
        if not ("tickBucket" in s or "tickNonPassenger" in s):
            continue
        ent += w
        cpu[deepest(frs)] += w
    print(f"\nCPU total={tot} entity={ent} ({100.0*ent/tot:.2f}%)")
    for k, v in cpu.most_common(14):
        print(f"  {v:6d} ({100.0*v/ent:4.1f}% ent / {100.0*v/tot:4.2f}% all)  {k}")

    collide_cpu = sum(v for k, v in cpu.items() if k in (
        "Entity.collide", "Entity.move", "LivingEntity.travel", "travelInFluid",
        "travelInAir", "handleRelFrict", "moveRelative", "findSupportingBlock",
        "setPosRaw", "makeBoundingBox"))
    print(f"\ncollide/travel CPU-семья (без CollisionUtil-внутр.): {collide_cpu} "
          f"= {100.0*collide_cpu/tot:.2f}% всего CPU")

    # ---- ceiling estimate ----
    ceil_alloc = 100.0 * travel_nonattacked / tot_all
    print(f"\nCEILING #11 (идеальный zero-alloc travel, все аллокации сняты):")
    print(f"  alloc-pressure share: <= {ceil_alloc:.1f}% всего аллок-давления")
    print(f"  при young-GC STW <= 7.1% wall -> выигрыш <= {100*0.071*(ceil_alloc/100):.2f}% wall-clock")
    hard = sum(v for k, v in site.items() if k not in ("unmatched",))
    hard_share = 100.0 * hard / tot_all
    print(f"  консервативно (без unmatched, только конкретные travel-сайты): {hard} "
          f"({hard_share:.1f}% давления) -> выигрыш <= {100*0.071*(hard_share/100):.2f}% wall-clock")
    print(f"  CPU-ось не конвертируется (урок №10-обобщение: тело в скаляре = та же доля CPU, доказано #10: 18.7%==18.7%)")

if __name__ == "__main__":
    main()
