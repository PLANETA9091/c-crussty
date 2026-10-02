#!/usr/bin/env python3
"""absorb_s7157.py — S7-157 absorb: REGION-THREADS leg vs CUMULATIVE base.

Usage: python3 absorb_s7157.py <leg_run_dir> [<base_run_dir>]

Base default: research/inside-cache-2026-09-18/run-s7149b-cumulative
(CUMULATIVE 35330129145, inside_cache=1 + flush_diet=1 — the green baseline).

Checks (preregistered gates §S7-155 §6 / S7-156 §5):
  PG2 0 NoClassDefFoundError in server-stdout.log + ARMED chain markers
      (RegionTickOps defined/BRIDGE_READY, ServerLevel Retargeted{1},
       EntityCallbacks Retargeted{1}+{1}, serve, retransform rc=0)
      + population 150000 VALID;
  PG3 TPS >= +25% vs CUMULATIVE crawl (Amdahl x2.31 ceiling);
      REFUTED if < +10%;
  PG4 young GC count <= baseline + 15% (<= 135 vs 118).

Diagnostics: RegionTickOps lane (main splice + worker buckets), entity-phase
share, fluid family (must stay ~untouched), worker-thread servicing proof
(stacks through RegionTickOps.tickBucket WITHOUT ServerLevel.tick frame).
"""
import os
import re
import sys
import collections

BASE_DEFAULT = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
                "run-s7149b-cumulative")
FLUID_ANCHORS = ("updateFluidHeightAndDoFluidPushing", "FluidPushOps")
OPS_KEY = "RegionTickOps"
ENTITY_FRAME = "tickNonPassenger"
BASELINE_YOUNG_GC = 118


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


def lane_sub(rows, pred):
    """Weighted samples of stacks matching pred, grouped by the deepest
    frame carrying one of the sub-keys."""
    sub = collections.Counter()
    for stack, w in rows:
        if pred(stack):
            for fr in stack.split(";"):
                for key in ("RegionTickOps.forEach", "RegionTickOps.parallelTick",
                            "RegionTickOps.tickBucket", "RegionTickOps.bucketOf",
                            "RegionTickOps.ensureHelpers", "RegionTickOps.onTickingStart",
                            "RegionTickOps.onTickingEnd"):
                    if key in fr:
                        sub[key] += w
                        break
                else:
                    continue
                break
    return sub


def young_gc(path):
    n = 0
    with open(path) as f:
        for ln in f:
            if "[gc,start" in ln and "Pause Young" in ln:
                n += 1
    return n


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
    if not os.path.exists(cpu):
        print("  !! cpu-collapsed.txt missing")
        return None
    total, rows = load_collapsed(cpu)

    def entity_phase(s):
        return ENTITY_FRAME in s or (OPS_KEY in s and "tickBucket" in s)

    ent = weighted(rows, entity_phase)
    ops_all = weighted(rows, lambda s: OPS_KEY in s)
    ops_worker = weighted(rows, lambda s: OPS_KEY in s and "tickBucket" in s
                          and "ServerLevel.tick" not in s)
    ops_main = weighted(rows, lambda s: OPS_KEY in s and "ServerLevel.tick" in s)
    ops_sub = lane_sub(rows, lambda s: OPS_KEY in s)
    fluid = weighted(rows, lambda s: any(a in s for a in FLUID_ANCHORS))
    ie_key = "net/minecraft/world/entity/item/ItemEntity.tick"
    zb_key = "net/minecraft/world/entity/monster/Zombie.tick"
    ie_total = weighted(rows, lambda s: ie_key in s)
    zb_total = weighted(rows, lambda s: zb_key in s)

    print(f"  total={total}")
    print(f"  entity-phase={ent} ({100.0*ent/total:.2f}%)")
    print(f"  RegionTickOps lane: all={ops_all} ({100.0*ops_all/total:.2f}%) "
          f"main-splice={ops_main} worker-buckets={ops_worker}")
    print(f"    ops sub-lanes: {dict(ops_sub)}")
    print(f"  fluid_family={fluid} ({100.0*fluid/total:.2f}%)")
    print(f"  ItemEntity.tick={ie_total} ({100.0*ie_total/total:.2f}%)  "
          f"Zombie.tick={zb_total} ({100.0*zb_total/total:.2f}%)")
    res = {"total": total, "ent": ent, "ops": ops_all, "ops_worker": ops_worker,
           "fluid": fluid, "young_gc": None, "tps": []}
    if os.path.exists(gc):
        g = young_gc(gc)
        res["young_gc"] = g
        print(f"  young_gc={g}")
    if os.path.exists(stdout):
        tps = tps_crawl(stdout)
        vals = tps_vals(tps)
        res["tps"] = vals
        print(f"  TPS crawl ({len(tps)}): {tps}")
        nc = sum(1 for ln in open(stdout, errors="replace")
                 if "NoClassDefFoundError" in ln)
        res["ncdfe"] = nc
        print(f"  NoClassDefFoundError lines={nc}")
        inj = [ln.strip()[:160] for ln in open(stdout, errors="replace")
               if "INJECT" in ln and ("VALID" in ln or "DONE" in ln
                                      or "INVALID" in ln)]
        for ln in inj[-6:]:
            print(f"    INJECT: {ln}")
        armed = [ln.strip()[:200] for ln in open(stdout, errors="replace")
                 if ("RegionTickOps" in ln or "region_threads" in ln
                     or "REGION-THREADS" in ln or "region-threads" in ln
                     or "Retargeted" in ln or "retransform" in ln
                     or "BRIDGE_READY" in ln or "WARN" in ln)]
        for ln in armed[-25:]:
            print(f"    ARMED? {ln}")
    return res


def main():
    leg = sys.argv[1]
    base = sys.argv[2] if len(sys.argv) > 2 else BASE_DEFAULT
    b = report("base", base)
    l = report("leg", leg)
    if not (b and l):
        return 2
    print("\n=== GATES ===")
    # PG2
    pg2_nc = l.get("ncdfe", 99) == 0
    print(f"PG2 NCDFE: {'PASS' if pg2_nc else 'FAIL'} "
          f"(leg NCDFE={l.get('ncdfe')})  [population/ARMED: see INJECT/ARMED lines]")
    # PG3
    bt, lt = b.get("tps") or [], l.get("tps") or []
    if bt and lt:
        bmed = sorted(bt)[len(bt)//2]
        lmed = sorted(lt)[len(lt)//2]
        gain = 100.0 * (lmed - bmed) / bmed
        verdict = "PASS" if gain >= 25.0 else ("REFUTED" if gain < 10.0
                                               else "INCONCLUSIVE(<25%,>=10%)")
        print(f"PG3 TPS: base median {bmed:.2f} -> leg median {lmed:.2f} "
              f"= +{gain:.1f}% (gate >= +25%; REFUTED < +10%) => {verdict}")
        print(f"    raw: base={bt} leg={lt}")
    else:
        print("PG3 TPS: crawl missing")
    # PG4
    bg, lg = b.get("young_gc"), l.get("young_gc")
    if bg and lg:
        cap = BASELINE_YOUNG_GC * 1.15
        verdict = "PASS" if lg <= cap else "FAIL"
        print(f"PG4 young GC: {bg} -> {lg} (cap <= {cap:.0f} = base+15%) "
              f"=> {verdict} ({100.0*(lg-bg)/bg:+.1f}%)")
    else:
        print("PG4 young GC: gc.log missing")
    return 0


if __name__ == "__main__":
    sys.exit(main())
