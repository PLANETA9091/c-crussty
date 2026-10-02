#!/usr/bin/env python3
"""absorb_s7158_leg3.py — S7-158 absorb: REGION-THREADS leg #3 (min-of-2 #1)
vs CUMULATIVE base + FRESH TOP-EATERS (owner directive 2026-09-19 00:51 +08
«ТОП-ПОЖИРАТЕЛЬ -> ∞»: top of eaters by (a) CPU% lanes, (b) GC pressure,
(c) MSPT/TPS contribution; attack order strictly top-down).

Usage: python3 absorb_s7158_leg3.py <leg_run_dir> [<base_run_dir>]

Base default: research/inside-cache-2026-09-18/run-s7149b-cumulative
(CUMULATIVE 35330129145, inside_cache=1 + flush_diet=1).

Gates (preregistered, dispatch_s7158.py docstring):
  PG2  0 NoClassDefFoundError + ARMED (RegionTickOps x3, ChunkMap
       Retargeted{1} TrackerTickOps, Entity Retargeted{1} RngOps,
       retransform rc=0) + population 150000 VALID;
  PG3  TPS >= +25% vs CUMULATIVE crawl (REFUTED < +10%);
  PG4  young GC <= 135 (base 118, cap +15%);
  CRASH-FREE: 0 tracker-race NPE ("entity is null" @ newTrackerTick),
       0 UUID-dup WARN; watchlist navigatingMobs sendBlockUpdated:1883
       (contained; >1/hour => S7-159 bridge).
"""
import os
import re
import sys
import collections

BASE_DEFAULT = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
                "run-s7149b-cumulative")
BASELINE_YOUNG_GC = 118

# CPU lanes: priority order — first matching category wins (inclusive
# attribution per stack; a stack is counted in exactly one lane).
# NB: collapsed stacks carry no thread-name frames — native GC/JIT samples
# are classified by symbol content (G1 internals, oop iterate, JIT).
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
    ("broadphase/collision/push", lambda frs: any(
        ".noCollision" in f or ".collide" in f or "push(" in f
        or "getEntities" in f or "getEntityCollisions" in f
        or "moonrise" in f.lower() and "EntityLookup" in f for f in frs)),
    ("navigation/pathfinding", lambda frs: any(
        "PathNavigation" in f or "PathFinder" in f or ".moveControl" in f
        or "Goali" in f for f in frs)),
    ("AI/goal-selector/brain/sensing", lambda frs: any(
        "goalSelector" in f or "net/minecraft/world/entity/ai/" in f
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


def weighted(rows, pred):
    return sum(w for s, w in rows if pred(s))


def lane_table(rows, total):
    """Mutually exclusive: each stack -> FIRST matching lane (true TOP)."""
    counts = collections.Counter()
    for s, w in rows:
        frs = s.split(";")
        for name, pred in LANES:
            if pred(frs):
                counts[name] += w
                break
        else:
            counts["other/uncategorized"] += w
    out = [(n, c, 100.0 * c / total) for n, c in counts.items()]
    out.sort(key=lambda t: -t[1])
    return out


# Level-2: breakdown INSIDE the entity-tick phase (also first-match).
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


def phase_breakdown(rows, total):
    """Level-2 decomposition of the entity-tick phase (first-match)."""
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
        for name, pred in SUB_LANES:
            if pred(frs):
                counts[name] += w
                break
        else:
            counts["entity-residual (tick/baseTick/data/sync)"] += w
    tbl = [(n, c, 100.0 * c / total, 100.0 * c / phase_total if phase_total
            else 0.0) for n, c in counts.items()]
    tbl.sort(key=lambda t: -t[1])
    return phase_total, tbl


def top_leaves(rows, total, n=25):
    leaf_w = collections.Counter()
    for s, w in rows:
        leaf_w[s.split(";")[-1]] += w
    return leaf_w.most_common(n), 100.0 * sum(leaf_w.values()) / total


def young_gc_stats(path):
    """Durations live on the [gc ] completion lines:
    ...][info][gc     ] GC(0) Pause Young (Normal) (...) 745M->5M(4104M) 3.469ms"""
    n, total_ms, worst = 0, 0.0, 0.0
    full = 0
    with open(path, errors="replace") as f:
        for ln in f:
            # tag padding varies across runs ([gc     ] vs [gc          ]);
            # completion lines carry the duration, start lines don't
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


def tps_crawl(path):
    txt = open(path, errors="replace").read()
    return re.findall(r"TPS from last 5s, 1m, 5m, 15m: ([0-9., ]+)", txt)


def tps_vals(crawl):
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
    res = {"dir": d}
    if not os.path.exists(cpu):
        print("  !! cpu-collapsed.txt missing")
        return None
    total, rows = load_collapsed(cpu)
    res["total"] = total
    print(f"  total={total}")

    print("  -- CPU lanes (mutually exclusive, first-match = true TOP):")
    for name, w, pct in lane_table(rows, total):
        if w:
            print(f"     {name:55s} {w:7d}  {pct:6.2f}%")
    phase_total, tbl2 = phase_breakdown(rows, total)
    print(f"  -- entity-phase breakdown ({phase_total} samples = "
          f"{100.0*phase_total/total:.2f}% CPU; first-match):")
    for name, c, pct_cpu, pct_phase in tbl2:
        print(f"     {name:55s} {c:7d}  {pct_cpu:5.2f}% CPU "
              f"({pct_phase:5.1f}% phase)")
    leaves, selfpct = top_leaves(rows, total)
    print(f"  -- top-15 leaf frames (self-time proxy, {selfpct:.1f}% of total):")
    for fr, w in leaves[:15]:
        print(f"     {w:7d}  {100.0*w/total:5.2f}%  {fr[:110]}")

    for key, anchors in (
        ("fluid_family", ("updateFluidHeightAndDoFluidPushing", "FluidPushOps")),
        ("inside_block_ops", ("InsideBlockOps",)),
        ("tracker_ops", ("TrackerTickOps",)),
        ("rng_ops", ("RngOps",)),
    ):
        w = weighted(rows, lambda s, a=anchors: any(x in s for x in a))
        res[key] = w
        print(f"  {key}={w} ({100.0*w/total:.2f}%)")

    ie = weighted(rows, lambda s: "item/ItemEntity.tick" in s)
    zb = weighted(rows, lambda s: "monster/Zombie.tick" in s)
    print(f"  ItemEntity.tick={ie} ({100.0*ie/total:.2f}%)  "
          f"Zombie.tick={zb} ({100.0*zb/total:.2f}%)")

    MAIN_FRAME = "net/minecraft/server/level/ServerLevel.tick"

    def is_worker(s):
        return ("RegionTickOps" in s and "tickBucket" in s
                and MAIN_FRAME not in s.split(";"))

    ops_worker = weighted(rows, is_worker)
    ops_all = weighted(rows, lambda s: "RegionTickOps" in s)
    res["offload"] = 100.0 * ops_worker / ops_all if ops_all else 0.0
    res["ops_worker"] = ops_worker
    print(f"  RegionTickOps lane all={ops_all} "
          f"({100.0*ops_all/total:.2f}%), worker={ops_worker} "
          f"(offload={res['offload']:.1f}%)")

    if os.path.exists(gc):
        n, tms, worst, fullg = young_gc_stats(gc)
        res["young_gc"], res["gc_ms"], res["gc_worst"], res["full_gc"] = \
            n, tms, worst, fullg
        print(f"  young_gc={n}  young_pause_total={tms:.0f}ms  "
              f"worst={worst:.1f}ms  full={fullg}")

    if os.path.exists(stdout):
        tps = tps_crawl(stdout)
        vals = tps_vals(tps)
        res["tps"] = vals
        print(f"  TPS crawl ({len(tps)}): {tps}")
        txt = open(stdout, errors="replace").read()
        res["ncdfe"] = txt.count("NoClassDefFoundError")
        # leg #2 markers (actual formats):
        #  NPE: Caused by: java.lang.NullPointerException: ... because "entity" is null
        #  UUID: Entity uuid already exists: <uuid>, mapped to ...
        res["tracker_npe"] = txt.count('because "entity" is null')
        res["uuid_dup"] = txt.count("Entity uuid already exists")
        res["navmob_watch"] = txt.count("sendBlockUpdated:1883")
        res["inject_valid"] = (txt.count("POPULATION FIXTURE-VALIDITY: VALID")
                               if "POPULATION FIXTURE-VALIDITY: INVALID"
                               not in txt else 0)
        m = re.search(r"POPULATION INJECT DONE target=(\d+) injected=(\d+)", txt)
        res["pop_injected"] = int(m.group(2)) if m else -1
        print(f"  NCDFE={res['ncdfe']}  tracker-NPE={res['tracker_npe']}  "
              f"uuid-dup={res['uuid_dup']}  "
              f"navmob-watch={res['navmob_watch']}  pop={res['pop_injected']} "
              f"VALID={res['inject_valid'] == 1}")
        armed = [ln.strip()[:190] for ln in open(stdout, errors="replace")
                 if ("TrackerTickOps" in ln or "RngOps" in ln
                     or "RegionTickOps" in ln or "Retargeted" in ln
                     or "retransform" in ln or "BRIDGE_READY" in ln)]
        for ln in armed[-18:]:
            print(f"    ARMED? {ln}")
    return res


def main():
    leg = sys.argv[1]
    base = sys.argv[2] if len(sys.argv) > 2 else BASE_DEFAULT
    b = report("base", base)
    l = report("leg", leg)
    if not (b and l):
        return 2
    print("\n=== GATES (preregistered §S7-158) ===")
    pg2 = (l.get("ncdfe", 99) == 0 and l.get("inject_valid", 0) == 1
           and l.get("pop_injected", -1) == 150000)
    print(f"PG2 NCDFE/POPULATION: {'PASS' if pg2 else 'FAIL'} "
          f"(NCDFE={l.get('ncdfe')}, pop={l.get('pop_injected')}/150000, "
          f"VALID={l.get('inject_valid') == 1}; ARMED: see lines above; "
          f"live TrackerTickOps samples={l.get('tracker_ops')}, "
          f"RngOps samples={l.get('rng_ops')})")
    bt, lt = b.get("tps") or [], l.get("tps") or []
    if bt and lt:
        bmed = sorted(bt)[len(bt) // 2]
        lmed = sorted(lt)[len(lt) // 2]
        gain = 100.0 * (lmed - bmed) / bmed
        verdict = ("PASS" if gain >= 25.0 else
                   "REFUTED" if gain < 10.0 else "INCONCLUSIVE")
        print(f"PG3 TPS: base median {bmed:.2f} -> leg median {lmed:.2f} = "
              f"+{gain:.1f}% (gate >= +25%) => {verdict}")
    else:
        gain = None
        print("PG3 TPS: crawl missing")
    bg, lg = b.get("young_gc"), l.get("young_gc")
    if bg and lg:
        cap = BASELINE_YOUNG_GC * 1.15
        verdict = "PASS" if lg <= cap else "FAIL"
        print(f"PG4 young GC: {bg} -> {lg} (cap <= {cap:.0f}) => {verdict} "
              f"({100.0*(lg-bg)/bg:+.1f}%)")
    else:
        print("PG4 young GC: gc.log missing")
    crashfree = (l.get("tracker_npe", 99) == 0 and l.get("uuid_dup", 99) == 0)
    print(f"CRASH-FREE (0 tracker-NPE, 0 uuid-dup): "
          f"{'PASS' if crashfree else 'FAIL'} "
          f"(tracker_npe={l.get('tracker_npe')}, uuid_dup={l.get('uuid_dup')}; "
          f"watchlist navigatingMobs={l.get('navmob_watch')}; "
          f"worker offload={l.get('offload', 0):.1f}%)")
    print("\n=== VERDICT S7-158 leg #3 ===")
    ok = pg2 and crashfree and lg is not None and lg <= BASELINE_YOUNG_GC * 1.15
    ok = ok and (gain is not None and gain >= 25.0)
    print(("ALL GATES PASS -> leg #4 (min-of-2 #2) NEXT, then banking"
           if ok else "GATES NOT CLEAN -> root-cause before leg #4"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
