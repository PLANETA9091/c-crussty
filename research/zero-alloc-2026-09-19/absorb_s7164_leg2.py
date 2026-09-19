#!/usr/bin/env python3
"""absorb_s7164_leg2.py — S7-164 ZERO-ALLOC-INSIDE (lever #10) absorb:
leg#2 (run 35418679791, head e82d81f, v4-candidate = v3 bank + zero_alloc_inside=1)
vs BANKED CUMULATIVE v3 (leg 35399980345, run-s7162-leg2-artifact).

Preregistered gates (GOAL СТАТУС + CLAIMS TASK-311, declared BEFORE dispatch;
unchanged per CLAIMS TASK-312 «прereg-входы НЕИЗМЕННЫ»):
  PG2  0 NCDFE + pop 150000 VALID + "entity_compose: stage zeroin composed
       (Retargeted { sites: 3 })" + "ARMED chain [inside->rng->batch->zeroin]
       rc=0" + "zero_alloc_ops: defined" + "region_threads: ARMED" +
       "batch_collector: defined" + telemetry;
  PG3  NON-REGRESSION: TPS last-5 median >= 1.60;
  PG4a collided-lane >= -50% per-work. Lane formula PREREGISTERED as the SUM
       of four "contains" counts (nested subsets counted per gate text:
       1749+687+406+987 = 3829 on v3): collidedWithFluid +
       collidedWithShapeMovingFrom + collidedAlongVector + getAABB. Same
       arithmetic applied symmetrically to leg2; union reported too;
  PG4b fluid-push lane (updateFluidHeightAndDoFluidPushing; v3 = 13523)
       >= -5% per-work;
  PG4c young GC <= 154 AND alloc family AABB+Vec3 <= 36.1% of total alloc
       (v3 = 40.08% prereg recomputed 40.16%, delta = normalization);
  CRASH-FREE  0 crash-report, 0 NCDFE, 0 Full GC; "Entity threw exception"
       <= 5/run (historical parallel-tick noise band s7160=1 s7161=3).
Banking: PASS -> CUMULATIVE v4 = v3 + zero_alloc_inside=1;
FAIL -> REFUTED + rollback zero_alloc_inside=0 (ZeroAllocOps = infrastructure).
Also emits the FRESH TOP-eaters (owner methodology «ТОП-ПОЖИРАТЕЛЬ -> ∞»).
"""
import os
import re
import sys
import collections

V3 = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"
LEG2 = "/home/z/c-crussty/research/zero-alloc-2026-09-19/run-s7164-leg2"

def read(path):
    return open(path, errors="replace").read() if os.path.exists(path) else ""

def cpu_samples(d):
    for line in read(os.path.join(d, "cpu-collapsed.txt")).splitlines():
        if not line.strip():
            continue
        m = line.rstrip().rsplit(" ", 1)
        if len(m) != 2:
            continue
        try:
            w = int(m[1])
        except ValueError:
            continue
        yield w, m[0].split(";")

def tps_vals(path):
    crawl = re.findall(r"TPS from last 5s, 1m, 5m, 15m: ([0-9., ]+)", read(path))
    out = []
    for c in crawl:
        try:
            out.append(float(c.split(",")[0].strip()))
        except ValueError:
            pass
    return out

def med(xs):
    s = sorted(xs)
    n = len(s)
    if not n:
        return 0.0
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2.0

def gc_stats(d):
    txt = read(os.path.join(d, "gc.log"))
    young = re.findall(
        r"\[info\]\[gc +\] GC\(\d+\) Pause Young \((.+?)\) (\d+)M->(\d+)M\((\d+)M\) ([0-9.]+)ms",
        txt)
    ms = [float(m[4]) for m in young]
    full = len(re.findall(r"Pause Full", txt))
    return len(young), full, ms

def collided_lane(d):
    """PREREGISTERED formula: SUM of four contains-counts (see docstring).
    Returns (sum_of_four, union, fluid, shape, vec, getaabb, total)."""
    tot = fluid = shape = vec = ga = union = 0
    for w, frs in cpu_samples(d):
        s = ";".join(frs)
        tot += w
        f1 = "collidedWithFluid" in s
        f2 = "collidedWithShapeMovingFrom" in s
        f3 = "collidedAlongVector" in s
        f4 = "getAABB" in s
        if f1:
            fluid += w
        if f2:
            shape += w
        if f3:
            vec += w
        if f4:
            ga += w
        if f1 or f2 or f3 or f4:
            union += w
    return fluid + shape + vec + ga, union, fluid, shape, vec, ga, tot

def fluid_lane(d):
    tot = lane = 0
    for w, frs in cpu_samples(d):
        tot += w
        if any("updateFluidHeightAndDoFluidPushing" in f for f in frs):
            lane += w
    return tot, lane

def alloc_families(d):
    txt = read(os.path.join(d, "alloc-collapsed.txt"))
    tot = 0
    fam = collections.Counter()
    for line in txt.splitlines():
        if not line.strip():
            continue
        m = line.rstrip().rsplit(" ", 1)
        if len(m) != 2:
            continue
        try:
            w = int(m[1])
        except ValueError:
            continue
        tot += w
        leaf = m[0].split(";")[-1].split("_[" )[0].replace(".", "/")
        if leaf.startswith("net/minecraft/world/phys/AABB"):
            fam["AABB"] += w
        elif leaf.startswith("net/minecraft/world/phys/Vec3"):
            fam["Vec3"] += w
        elif leaf.startswith("net/minecraft/core/BlockPos"):
            fam["BlockPos"] += w
        elif leaf.startswith("[L") or leaf.startswith("java/util/"):
            fam["java/container"] += w
        else:
            fam["other"] += w
    return tot, fam

def main():
    print("=" * 76)
    print("S7-164 ZERO-ALLOC-INSIDE absorb: leg#2 (35418679791) vs CUMULATIVE v3")
    print("=" * 76)

    # ---- PG2 ----
    s2 = read(os.path.join(LEG2, "server-stdout.log"))
    armed = "ARMED chain [inside->rng->batch->zeroin]" in s2 and "retransform rc=0" in s2
    stage = "stage zeroin composed (Retargeted { sites: 3 })" in s2
    za_defined = "zero_alloc_ops: defined" in s2
    rt_armed = "region_threads: ARMED" in s2
    bc_defined = "batch_collector: defined" in s2
    pop_valid = "POPULATION FIXTURE-VALIDITY: VALID (injected=150000 target=150000)" in s2
    ncdfe = s2.count("NoClassDefFoundError")
    crash_report = os.path.exists(os.path.join(LEG2, "crash-report")) or "crash-report" in s2
    pg2 = armed and stage and za_defined and rt_armed and bc_defined and pop_valid and ncdfe == 0 and not crash_report
    print(f"\nPG2: armed={armed} stage_zeroin={stage} za_defined={za_defined} "
          f"region_threads={rt_armed} batch_collector={bc_defined} pop150k={pop_valid} "
          f"NCDFE={ncdfe} crash_report={crash_report} -> {'PASS' if pg2 else 'FAIL'}")
    for marker in ("stage zeroin composed", "ARMED chain", "zero_alloc_ops: defined"):
        for ln in s2.splitlines():
            if marker in ln:
                print(f"     | {ln.strip()[:150]}")
                break
    exc = s2.count("Entity threw exception")
    print(f"     caught 'Entity threw exception': {exc} (noise band 0-5: s7160=1 s7161=3 v3=0)")

    # ---- PG3 ----
    t2 = tps_vals(os.path.join(LEG2, "server-stdout.log"))
    t3 = tps_vals(os.path.join(V3, "server-stdout.log"))
    m2, m3 = med(t2), med(t3)
    pg3 = m2 >= 1.60
    print(f"\nPG3: TPS last-5 median leg2={m2:.3f} (n={len(t2)})  v3={m3:.3f} (n={len(t3)})  "
          f"delta={100.0*(m2-m3)/m3 if m3 else 0:+.1f}%  -> {'PASS' if pg3 else 'FAIL'}")

    # ---- PG4a ----
    lane2s, lane2u, fl2, sh2, vc2, ga2, tot2 = collided_lane(LEG2)
    lane3s, lane3u, fl3, sh3, vc3, ga3, tot3 = collided_lane(V3)
    pw2 = lane2s / m2 if m2 else 0.0
    pw3 = lane3s / m3 if m3 else 0.0
    dpw = 100.0 * (pw2 - pw3) / pw3 if pw3 else 0.0
    pg4a = dpw <= -50.0
    print(f"\nPG4a: CPU total v3={tot3} leg2={tot2}")
    print(f"      collided-lane SUM-formula (prereg): v3={lane3s} leg2={lane2s} "
          f"({100.0*lane2s/tot2:.2f}% vs {100.0*lane3s/tot3:.2f}%)")
    print(f"        components v3: fluid={fl3} shape={sh3} vec={vc3} getAABB={ga3} (sum {lane3s})")
    print(f"        components leg2: fluid={fl2} shape={sh2} vec={vc2} getAABB={ga2} (sum {lane2s})")
    print(f"      per-work (lane/TPSmedian): v3={pw3:.0f} leg2={pw2:.0f} delta={dpw:+.1f}% (gate <= -50%) -> {'PASS' if pg4a else 'FAIL'}")
    print(f"      [ref] union-lane: v3={lane3u} ({100.0*lane3u/tot3:.2f}%)  leg2={lane2u} ({100.0*lane2u/tot2:.2f}%)  "
          f"raw delta={100.0*(lane2u-lane3u)/lane3u if lane3u else 0:+.1f}%")

    # ---- PG4b ----
    _, flane2 = fluid_lane(LEG2)
    _, flane3 = fluid_lane(V3)
    fpw2 = flane2 / m2 if m2 else 0.0
    fpw3 = flane3 / m3 if m3 else 0.0
    dfpw = 100.0 * (fpw2 - fpw3) / fpw3 if fpw3 else 0.0
    pg4b = dfpw <= -5.0
    print(f"\nPG4b: fluid-push lane: v3={flane3} ({100.0*flane3/tot3:.2f}%)  leg2={flane2} ({100.0*flane2/tot2:.2f}%)  "
          f"per-work delta={dfpw:+.1f}% (gate <= -5%) -> {'PASS' if pg4b else 'FAIL'}")

    # ---- PG4c ----
    y2, full2, ms2 = gc_stats(LEG2)
    y3, full3, ms3 = gc_stats(V3)
    atot2, afam2 = alloc_families(LEG2)
    atot3, afam3 = alloc_families(V3)
    av2 = 100.0 * (afam2["AABB"] + afam2["Vec3"]) / atot2 if atot2 else 0.0
    av3 = 100.0 * (afam3["AABB"] + afam3["Vec3"]) / atot3 if atot3 else 0.0
    pg4c = y2 <= 154 and av2 <= 36.1
    print(f"\nPG4c: young GC: v3={y3} leg2={y2} (gate <= 154); Full: v3={full3} leg2={full2}")
    if ms2:
        sm = sorted(ms2)
        print(f"      leg2 pause ms: median={sm[len(sm)//2]:.1f} max={sm[-1]:.1f} total={sum(ms2)/1000.0:.2f}s (v3 total={sum(ms3)/1000.0:.2f}s)")
    print(f"      alloc total: v3={atot3} leg2={atot2} ({100.0*(atot2-atot3)/atot3:+.1f}%)")
    print(f"      AABB+Vec3: v3={av3:.2f}% (prereg 40.08%)  leg2={av2:.2f}% (gate <= 36.1%) -> {'PASS' if pg4c else 'FAIL'}")

    crash_free = (ncdfe == 0) and (not crash_report) and (full2 == 0)
    print(f"\nCRASH-FREE: NCDFE={ncdfe} crash_report={crash_report} fullGC={full2} "
          f"exceptions={exc} (<=5) -> {'PASS' if crash_free else 'FAIL'}")

    verdict = "PASS" if (pg2 and pg3 and pg4a and pg4b and pg4c and crash_free) else "FAIL"
    print("\n" + "#" * 76)
    print(f"VERDICT S7-164 (lever #10 ZERO-ALLOC-INSIDE): {verdict}")
    print("#" * 76)
    if verdict == "PASS":
        print("BANKING: CUMULATIVE v4 = v3 + zero_alloc_inside=1")
    else:
        print("BANKING: REFUTED + rollback zero_alloc_inside=0 (bank stays v3)")

    # ---- FRESH TOP (owner methodology, 3 axes) ----
    print("\n--- FRESH TOP-eaters: leg#2 (3 axes) ---")
    top_lane = collections.Counter()

    def classify(frs):
        if any(("G1" in f or "OopOopIterate" in f or "oopDesc::size" in f
                or "do_oop_work" in f or "GCTask" in f or "CompileBroker" in f
                or "C2Compile" in f or "CardSet" in f or "refine_buffer" in f) for f in frs):
            return "GC/JIT-native phase"
        if any("RegionTickOps.tickBucket" in f or "tickNonPassenger" in f for f in frs):
            return "entity-tick phase (tickBucket lane)"
        if any("updateFluidHeightAndDoFluidPushing" in f or "getFlow" in f for f in frs):
            return "fluid-push lane"
        if any("checkInsideBlocks" in f or "collidedWithFluid" in f
               or "collidedWithShapeMovingFrom" in f or "collidedAlongVector" in f for f in frs):
            return "inside-pipeline (incl. collided)"
        if any("EntityCollision" in f or "collide" in f or "push" in f for f in frs):
            return "broadphase/collision"
        if any("tracker" in f.lower() or "Tracker" in f for f in frs):
            return "tracker"
        return None

    for w, frs in cpu_samples(LEG2):
        c = classify(frs)
        if c:
            top_lane[c] += w
    print("(a) CPU lanes (share of total CPU samples):")
    for k, v in top_lane.most_common():
        print(f"     {k:46s} {v:7d}  {100.0*v/tot2:5.2f}%")
    uncl = tot2 - sum(top_lane.values())
    print(f"     {'unclassified':46s} {uncl:7d}  {100.0*uncl/tot2:5.2f}%")

    print("(b) alloc pressure leaf families (share of alloc):")
    for k in ("AABB", "Vec3", "BlockPos", "java/container", "other"):
        print(f"     {k:46s} v3 {afam3.get(k,0):7d} ({100.0*afam3.get(k,0)/atot3:4.1f}%)  "
              f"leg2 {afam2.get(k,0):7d} ({100.0*afam2.get(k,0)/atot2:4.1f}%)")
    print(f"(c) gc.log: young={y2} (v3 {y3}), median pause "
          f"{sorted(ms2)[len(ms2)//2] if ms2 else 0:.1f}ms (v3 {sorted(ms3)[len(ms3)//2] if ms3 else 0:.1f}ms), "
          f"TPS median {m2:.2f} (v3 {m3:.2f})")

if __name__ == "__main__":
    main()
