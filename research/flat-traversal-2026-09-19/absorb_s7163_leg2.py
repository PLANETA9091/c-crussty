#!/usr/bin/env python3
"""absorb_s7163_leg2.py — S7-163 FLAT-TRAVERSAL (lever #9) absorb:
leg#2 (run 35410873485, v4-candidate = v3 bank + flat_traversal=1)
vs BANKED CUMULATIVE v3 (leg 35399980345, run-s7162-leg2-artifact).

Preregistered gates (GOAL СТАТУС + CLAIMS TASK-308, declared BEFORE dispatch):
  PG2  0 NCDFE + pop 150000 VALID + "entity_compose: ARMED chain
       [inside->rng->batch->traversal] ... retransform rc=0" +
       NEW nested-delivery markers: "traverse_ops: defined nested
       ...TraverseOps$LongTable" AND "defined net/minecraft/world/level/
       TraverseOps (+1 nested)";
  PG3  NON-REGRESSION: TPS last-5 median >= 1.60;
  PG4  traversal-lane samples >= -50% vs v3 lane (vanilla
       forEachBlockIntersectedBetween) AND residual guava tail
       (BlockPos$6/$4 iterators) >= -70% AND young GC <= 154;
  CRASH-FREE  0 crash-report, 0 NCDFE; known sendBlockUpdated
       fastutil-NPE noise (parallel region tick, pre-existing 0-5/run)
       logged separately, NOT a traversal lane failure.
Banking: PASS -> CUMULATIVE v4 = v3 + flat_traversal=1;
FAIL -> REFUTED + rollback flat_traversal=0.
Also emits the FRESH TOP-eaters (owner methodology «ТОП-ПОЖИРАТЕЛЬ -> ∞»).
"""
import os
import re
import sys
import collections

V3 = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"
LEG2 = "/home/z/c-crussty/research/flat-traversal-2026-09-19/run-s7163-leg2-artifact"

def read(path):
    return open(path, errors="replace").read() if os.path.exists(path) else ""

def cpu_samples(d):
    """yield (weight:int, frames:list[str])"""
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

def lane_stats(d, tag):
    """traversal lane + guava tail + lane self/applier decomposition"""
    total = 0
    lane = 0
    lane_orch = 0     # lane samples whose DEEPEST MC frame is iterator/dedupe orchestration
    lane_applier = 0  # lane samples inside visit/applier effects (BlockBehaviour etc.)
    tail = 0          # guava BlockPos$6/$4 iterators anywhere in stack
    markers = ("forEachBlockIntersectedBetween",) if tag == "v3" else ("TraverseOps.",)
    for w, frs in cpu_samples(d):
        total += w
        if any(mk in f for f in frs for mk in markers):
            lane += w
            if any("BlockBehaviour" in f or "entityInside" in f or "StepBasedCollector" in f for f in frs):
                lane_applier += w
            else:
                lane_orch += w
        if any(("$6" in f and "BlockPos" in f) or ("$4" in f and "BlockPos" in f) for f in frs):
            tail += w
    return total, lane, tail, lane_orch, lane_applier

def alloc_shares(d):
    """alloc-collapsed is SITE-granular (leaf = allocating method).
    Group: (i) leaf-type families by leaf frame prefix; (ii) lanes by
    stack markers (inside-pipeline / fluid-push / traversal / other)."""
    txt = read(os.path.join(d, "alloc-collapsed.txt"))
    tot = 0
    leaf_fam = collections.Counter()
    lanes = collections.Counter()
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
        frames = m[0].split(";")
        leaf = frames[-1]
        # normalize: v3 leaf = typed "net.minecraft.world.phys.AABB_[i]" (dots),
        # leg2 leaf = site "net/minecraft/world/phys/AABB.inflate" (method)
        leaf = leaf.split("_[")[0].replace(".", "/")
        if leaf.startswith("net/minecraft/world/phys/AABB"):
            leaf_fam["AABB-sites"] += w
        elif leaf.startswith("net/minecraft/world/phys/Vec3"):
            leaf_fam["Vec3-sites"] += w
        elif leaf.startswith("net/minecraft/core/BlockPos"):
            leaf_fam["BlockPos-sites"] += w
        elif leaf.startswith("[L") or "array" in leaf or leaf.startswith("java/util/"):
            leaf_fam["java/container-sites"] += w
        else:
            leaf_fam["other"] += w
        s = m[0]
        if "checkInsideBlocks" in s or "collidedWithFluid" in s or "collidedWithShapeMovingFrom" in s:
            lanes["inside-pipeline"] += w
        elif "updateFluidHeightAndDoFluidPushing" in s or "getFlow" in s:
            lanes["fluid-push"] += w
        elif "TraverseOps" in s or "forEachBlockIntersectedBetween" in s:
            lanes["traversal-only"] += w
        elif "tickNonPassenger" in s or "tickBucket" in s:
            lanes["tickBucket-orch"] += w
        else:
            lanes["other-lanes"] += w
    pct = {k: 100.0 * v / tot for k, v in leaf_fam.items()} if tot else {}
    lpct = {k: 100.0 * v / tot for k, v in lanes.items()} if tot else {}
    return tot, leaf_fam, pct, lanes, lpct

def main():
    print("=" * 76)
    print("S7-163 FLAT-TRAVERSAL absorb: leg#2 (35410873485) vs CUMULATIVE v3")
    print("=" * 76)

    # ---- PG2 ----
    s2 = read(os.path.join(LEG2, "server-stdout.log"))
    armed = "ARMED chain [inside->rng->batch->traversal]" in s2 and "retransform rc=0" in s2
    nested = "traverse_ops: defined nested net/minecraft/world/level/TraverseOps$LongTable in kernel loader" in s2
    nested2 = "defined net/minecraft/world/level/TraverseOps (+1 nested) in kernel loader" in s2
    pop_valid = "POPULATION FIXTURE-VALIDITY: VALID (injected=150000 target=150000)" in s2
    ncdfe = s2.count("NoClassDefFoundError")
    crash_report = os.path.exists(os.path.join(LEG2, "crash-report")) or "crash-report" in s2
    pg2 = armed and nested and nested2 and pop_valid and ncdfe == 0 and not crash_report
    print(f"\nPG2: armed={armed} nested_define={nested} (+1_nested_marker)={nested2} "
          f"pop150k={pop_valid} NCDFE={ncdfe} crash_report={crash_report} -> {'PASS' if pg2 else 'FAIL'}")
    exc = s2.count("Entity threw exception")
    print(f"     caught 'Entity threw exception': {exc} (pre-existing parallel-tick noise range 0-5: s7160=1 s7161=3 v3=0)")

    # ---- PG3 ----
    t2 = tps_vals(os.path.join(LEG2, "server-stdout.log"))
    t3 = tps_vals(os.path.join(V3, "server-stdout.log"))
    m2, m3 = med(t2), med(t3)
    pg3 = m2 >= 1.60
    print(f"\nPG3: TPS last-5 median leg2={m2:.3f} (n={len(t2)})  v3={m3:.3f} (n={len(t3)})  "
          f"delta={100.0*(m2-m3)/m3 if m3 else 0:+.1f}%  -> {'PASS' if pg3 else 'FAIL'}")

    # ---- PG4 ----
    tot3, lane3, tail3, orch3, appl3 = lane_stats(V3, "v3")
    tot2, lane2, tail2, orch2, appl2 = lane_stats(LEG2, "leg2")
    m3pw = lane3 / m3 if m3 else 0.0
    m2pw = lane2 / m2 if m2 else 0.0
    dpw = 100.0 * (m2pw - m3pw) / m3pw if m3pw else 0.0
    dl = 100.0 * (lane2 - lane3) / lane3 if lane3 else 0.0
    dt = 100.0 * (tail2 - tail3) / tail3 if tail3 else 0.0
    y2, full2, ms2 = gc_stats(LEG2)
    y3, full3, ms3 = gc_stats(V3)
    pg4 = dl <= -50.0 and dt <= -70.0 and y2 <= 154
    print(f"\nPG4: CPU total v3={tot3} leg2={tot2}")
    print(f"     traversal lane: v3={lane3} ({100.0*lane3/tot3:.2f}%)  leg2={lane2} ({100.0*lane2/tot2:.2f}%)  "
          f"delta={dl:+.1f}% (gate <= -50%)")
    print(f"     per-work (lane/TPSmedian): v3={m3pw:.0f}  leg2={m2pw:.0f}  delta={dpw:+.1f}%")
    print(f"     lane decomposition: v3 orch={orch3} applier={appl3}  |  leg2 orch={orch2} applier={appl2}")
    print(f"     guava tail (BlockPos$6/$4): v3={tail3} ({100.0*tail3/tot3:.2f}%)  leg2={tail2} ({100.0*tail2/tot2:.2f}%)  "
          f"delta={dt:+.1f}% (gate <= -70%)")
    print(f"     young GC: v3={y3} leg2={y2} (gate <= 154); Full: v3={full3} leg2={full2}")
    if ms2:
        sm = sorted(ms2)
        print(f"     leg2 pause ms: median={sm[len(sm)//2]:.1f} max={sm[-1]:.1f} total={sum(ms2)/1000.0:.2f}s (v3 total={sum(ms3)/1000.0:.2f}s)")
    print(f"     -> {'PASS' if pg4 else 'FAIL'}")

    crash_free = (ncdfe == 0) and (not crash_report) and (full2 == 0)
    print(f"\nCRASH-FREE: NCDFE={ncdfe} crash_report={crash_report} fullGC={full2} -> {'PASS' if crash_free else 'FAIL'}")

    verdict = "PASS" if (pg2 and pg3 and pg4 and crash_free) else "FAIL"
    print("\n" + "#" * 76)
    print(f"VERDICT S7-163 (lever #9 FLAT-TRAVERSAL): {verdict}")
    print("#" * 76)

    # ---- FRESH TOP (owner methodology) ----
    print("\n--- FRESH TOP-eaters: leg#2 (3 axes) ---")
    top_lane = collections.Counter()
    def classify(frs):
        if any(("G1" in f or "OopOopIterate" in f or "oopDesc::size" in f
                or "do_oop_work" in f or "GCTask" in f or "CompileBroker" in f
                or "C2Compile" in f or "CardSet" in f or "refine_buffer" in f) for f in frs):
            return "GC/JIT-native phase"
        if any("RegionTickOps.tickBucket" in f or "tickNonPassenger" in f for f in frs):
            return "entity-tick phase (tickBucket lane)"
        if any("TraverseOps." in f for f in frs):
            return "traversal lane (flat)"
        if any("checkInsideBlocks" in f for f in frs):
            return "inside-pipeline (non-traversal parts)"
        if any("updateFluidHeightAndDoFluidPushing" in f or "getFlow" in f for f in frs):
            return "fluid-push lane"
        if any("EntityCollision" in f or "collide" in f or "push" in f for f in frs):
            return "broadphase/collision"
        if any("tracker" in f.lower() or "Tracker" in f for f in frs):
            return "tracker"
        return None
    for w, frs in cpu_samples(LEG2):
        c = classify(frs)
        if c:
            top_lane[c] += w
    tot2b = tot2
    print("(a) CPU lanes (share of total CPU samples):")
    for k, v in top_lane.most_common():
        print(f"     {k:46s} {v:7d}  {100.0*v/tot2b:5.2f}%")
    uncl = tot2b - sum(top_lane.values())
    print(f"     {'unclassified':46s} {uncl:7d}  {100.0*uncl/tot2b:5.2f}%")

    atot2, afam2, apct2, alane2, alpct2 = alloc_shares(LEG2)
    atot3, afam3, apct3, alane3, alpct3 = alloc_shares(V3)
    print(f"(b) alloc pressure: total samples v3={atot3}  leg2={atot2} ({100.0*(atot2-atot3)/atot3:+.1f}%)")
    print("    leaf-site families (share of alloc):")
    for k in ("AABB-sites", "Vec3-sites", "BlockPos-sites", "java/container-sites", "other"):
        print(f"     {k:46s} v3 {afam3.get(k,0):7d} ({apct3.get(k,0.0):4.1f}%)  leg2 {afam2.get(k,0):7d} ({apct2.get(k,0.0):4.1f}%)")
    print("    lanes (share of alloc):")
    for k in ("inside-pipeline", "fluid-push", "traversal-only", "tickBucket-orch", "other-lanes"):
        print(f"     {k:46s} v3 {alane3.get(k,0):7d} ({alpct3.get(k,0.0):4.1f}%)  leg2 {alane2.get(k,0):7d} ({alpct2.get(k,0.0):4.1f}%)")
    print(f"(c) gc.log: young={y2} (v3 {y3}), median pause "
          f"{sorted(ms2)[len(ms2)//2] if ms2 else 0:.1f}ms (v3 {sorted(ms3)[len(ms3)//2] if ms3 else 0:.1f}ms), TPS median {m2:.2f} (v3 {m3:.2f})")

if __name__ == "__main__":
    main()
