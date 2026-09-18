#!/usr/bin/env python3
"""absorb_s7160.py — S7-160 absorb: BATCH-COLLECTOR leg (v2 + batch_collector=1)
vs the BANKED CUMULATIVE v2 (leg #5 = 35381522360).

Preregistered gates (GOAL СТАТУС S7-160, declared BEFORE dispatch):
  PG2   0 NCDFE + pop 150000 VALID + ARMED: v2 chain (inside_cache +
        region_threads retransform rc=0) + "batch_collector: defined" +
        "batch_collector: ARMED first-swap";
  PG3   NON-REGRESSION: TPS median >= 1.60 (worst banked v2 leg);
  PG4'' young GC <= 180 (leg #5 level) AND collector-family CPU
        (flushStep/advanceStep/applyAndClear leaf samples) <= 55% of
        leg #5 (2143 samples -> <= ~1180);
  CRASH-FREE  0 tracker-NPE, 0 uuid-dup, navigatingMobs watchlist.
Banking: full PASS -> CUMULATIVE v3 = v2 + batch_collector=1;
gate FAIL -> REFUTED-BY-ECONOMICS + rollback batch_collector=0.

Also emits the FRESH TOP-eaters table (owner methodology «ТОП-ПОЖИРАТЕЛЬ
-> ∞»: CPU lanes + entity-phase decomposition + GC axis) for the next
cycle's attack ordering.
"""
import os
import re
import sys
import collections

LEG5 = ("/home/z/c-crussty/research/region-threads-2026-09-18/"
        "run-s7159-leg5-artifact")

LANES = [
    ("GC/JIT-native (G1/JIT symbols)", lambda frs: any(
        ("G1" in f or "OopOopIterate" in f or "oopDesc::size" in f
         or "do_oop_work" in f or "GCTask" in f or "CompileBroker" in f
         or "C2Compile" in f or "c2_init" in f or "CardSet" in f)
        for f in frs)),
    ("entity-tick (tickBucket lane incl. workers)", lambda frs: any(
        "RegionTickOps.tickBucket" in f or "tickNonPassenger" in f
        for f in frs)),
    ("region-threads-infra (fill/barriers, no tickBucket)", lambda frs: any(
        "RegionTickOps" in f and "tickBucket" not in f for f in frs)),
    ("fluid-scan", lambda frs: any(
        "updateFluidHeightAndDoFluidPushing" in f or "FluidPushOps" in f
        for f in frs)),
    ("inside-blocks", lambda frs: any(
        "InsideBlockOps" in f or "insideBlock" in f for f in frs)),
    ("inside-pipeline (traversal+collector)", lambda frs: any(
        "checkInsideBlocks" in f or "forEachBlockIntersectedBetween" in f
        or "StepBasedCollector" in f or "applyEffectsFromBlocks" in f
        or "BatchCollector" in f for f in frs)),
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
    ("tracker (ChunkMap/ServerEntity)", lambda frs: any(
        "ChunkMap" in f and "Tracker" in f or "ServerEntity" in f
        or "TrackerTickOps" in f or "newTrackerTick" in f for f in frs)),
    ("chunk-system workers", lambda frs: any(
        "ServerChunkCache" in f or "ChunkTaskScheduler" in f
        or "world/threaded" in f for f in frs)),
    ("main-server-tick-other", lambda frs: any(
        "MinecraftServer.tickServer" in f or "ServerLevel.tick" in f
        for f in frs)),
]

# collector-family leaf anchors (S7-160 attack surface)
COLLECTOR_ANCHORS = ("StepBasedCollector", "BatchCollector")


def load_collapsed(path):
    total, rows = 0, []
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


def lane_table(rows, total):
    counts = collections.Counter()
    for s, w in rows:
        frs = s.split(";")
        for name, pred in LANES:
            if pred(frs):
                counts[name] += w
                break
        else:
            counts["other/uncategorized"] += w
    out = sorted(counts.items(), key=lambda t: -t[1])
    return [(n, c, 100.0 * c / total) for n, c in out]


def phase_breakdown(rows, total):
    def in_phase(frs):
        return any("RegionTickOps.tickBucket" in f or "tickNonPassenger" in f
                   for f in frs)
    counts = collections.Counter()
    phase_total = 0
    for s, w in rows:
        frs = s.split(";")
        if not in_phase(frs):
            continue
        phase_total += w
        for name, pred in LANES[3:]:
            if pred(frs):
                counts[name] += w
                break
        else:
            counts["entity-residual"] += w
    return phase_total, [(n, c, 100.0 * c / total, 100.0 * c / phase_total)
                         for n, c in counts.most_common()]


def collector_family(rows):
    """Preregister S7-160 metric: CPU attributed to the collector-method
    LEAF frames (deepest MC frame contains the anchor) — flushStep +
    advanceStep + applyAndClear + recorded self-time. leg #5 reference
    = 2143 (GOAL preregister)."""
    total = 0
    for s, w in rows:
        for f in reversed(s.split(";")):
            if f.startswith("net/minecraft/"):
                if any(a in f for a in COLLECTOR_ANCHORS):
                    total += w
                break
        else:
            if any(a in s for a in COLLECTOR_ANCHORS):
                total += w
    return total


def young_gc_stats(path):
    n, total_ms, worst, full = 0, 0.0, 0.0, 0
    with open(path, errors="replace") as f:
        for ln in f:
            if "[gc,start" in ln:
                continue
            if "Pause Young" in ln:
                n += 1
                m = re.search(r"([0-9.]+)ms\s*$", ln.strip())
                if m:
                    ms = float(m.group(1))
                    total_ms += ms
                    worst = max(worst, ms)
            elif "Pause Full" in ln:
                full += 1
    return n, total_ms, worst, full


def tps_vals(path):
    txt = open(path, errors="replace").read()
    crawl = re.findall(r"TPS from last 5s, 1m, 5m, 15m: ([0-9., ]+)", txt)
    out = []
    for c in crawl:
        try:
            out.append(float(c.split(",")[0].strip()))
        except ValueError:
            pass
    return out


def report(tag, d):
    cpu = os.path.join(d, "cpu-collapsed.txt")
    stdout = os.path.join(d, "server-stdout.log")
    gc = os.path.join(d, "gc.log")
    print(f"### {tag}: {d}")
    if not os.path.exists(cpu):
        print("  !! cpu-collapsed.txt missing")
        return None
    total, rows = load_collapsed(cpu)
    print(f"  total={total}")
    for name, w, pct in lane_table(rows, total):
        if w:
            print(f"     {name:55s} {w:7d}  {pct:6.2f}%")
    phase_total, tbl2 = phase_breakdown(rows, total)
    print(f"  -- entity-phase breakdown ({phase_total} = "
          f"{100.0*phase_total/total:.2f}% CPU):")
    for name, c, pct_cpu, pct_phase in tbl2:
        print(f"     {name:55s} {c:7d}  {pct_cpu:5.2f}% CPU "
              f"({pct_phase:5.1f}% phase)")
    cf = collector_family(rows)
    print(f"  collector-family={cf} ({100.0*cf/total:.2f}%)")
    res = {"total": total, "collector": cf}
    if os.path.exists(gc):
        n, tms, worst, fullg = young_gc_stats(gc)
        res.update(young=n, worst=worst, full=fullg)
        print(f"  young_gc={n} total={tms:.0f}ms worst={worst:.1f}ms "
              f"full={fullg}")
    if os.path.exists(stdout):
        vals = tps_vals(stdout)
        med = sorted(vals)[len(vals) // 2] if vals else 0.0
        res["tps_median"] = med
        txt = open(stdout, errors="replace").read()
        res["ncdfe"] = txt.count("NoClassDefFoundError")
        res["tracker_npe"] = txt.count('because "entity" is null')
        res["uuid_dup"] = txt.count("Entity uuid already exists")
        res["navmob"] = txt.count("sendBlockUpdated:1883")
        res["valid"] = ("POPULATION FIXTURE-VALIDITY: VALID" in txt
                        and "INVALID" not in txt)
        m = re.search(r"POPULATION INJECT DONE target=(\d+) injected=(\d+)",
                      txt)
        res["pop"] = int(m.group(2)) if m else -1
        res["bc_defined"] = "batch_collector: defined" in txt
        res["bc_armed"] = "batch_collector: ARMED first-swap" in txt
        print(f"  TPS median={med:.2f} (crawl n={len(vals)})")
        print(f"  NCDFE={res['ncdfe']} NPE={res['tracker_npe']} "
              f"uuid={res['uuid_dup']} navmob={res['navmob']} "
              f"pop={res['pop']} VALID={res['valid']}")
        print(f"  BC defined={res['bc_defined']} armed={res['bc_armed']}")
    return res


def main():
    leg = sys.argv[1] if len(sys.argv) > 1 else (
        "/home/z/c-crussty/research/batch-collector-2026-09-19/"
        "run-s7160-batch-collector-artifact")
    b = report("base v2 leg#5 (35381522360)", LEG5)
    l = report("leg v2+bc (S7-160)", leg)
    if not (b and l):
        return 2
    print("\n=== GATES (preregistered S7-160) ===")
    pg2 = (l.get("ncdfe", 99) == 0 and l.get("valid", False)
           and l.get("pop", -1) == 150000 and l.get("bc_defined", False)
           and l.get("bc_armed", False))
    print(f"PG2: {'PASS' if pg2 else 'FAIL'} (NCDFE={l.get('ncdfe')}, "
          f"pop={l.get('pop')}, VALID={l.get('valid')}, "
          f"defined={l.get('bc_defined')}, armed={l.get('bc_armed')})")
    med = l.get("tps_median", 0.0)
    pg3 = med >= 1.60
    print(f"PG3 non-regression: median={med:.2f} (cap >= 1.60) => "
          f"{'PASS' if pg3 else 'FAIL'}")
    yg = l.get("young", 999999)
    cf = l.get("collector", 1 << 30)
    cap_cf = 0.55 * b.get("collector", 2143)
    pg4 = (yg <= 180) and (cf <= cap_cf)
    print(f"PG4'': young GC={yg} (cap 180) "
          f"{'OK' if yg <= 180 else 'FAIL'}; collector-family "
          f"{b.get('collector')}->{cf} (cap <= {cap_cf:.0f}, "
          f"{100.0*(b.get('collector',1)-cf)/max(b.get('collector',1),1):+.1f}%) "
          f"=> {'PASS' if pg4 else 'FAIL'}")
    crash = (l.get("tracker_npe", 99) == 0 and l.get("uuid_dup", 99) == 0)
    print(f"CRASH-FREE: {'PASS' if crash else 'FAIL'} "
          f"(navmob watchlist={l.get('navmob')})")
    ok = pg2 and pg3 and pg4 and crash
    print("\n=== VERDICT S7-160 ===")
    print(("ALL GATES PASS -> CUMULATIVE v3 = v2 + batch_collector=1 BANKED"
           if ok else
           "GATES NOT CLEAN -> REFUTED-BY-ECONOMICS + rollback "
           "batch_collector=0 (v2 stays banked)"))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
