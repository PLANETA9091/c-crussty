#!/usr/bin/env python3
"""absorb_s7153.py — S7-153 absorb: FLUID-DIRTY leg vs CUMULATIVE base.

Usage: python3 absorb_s7153.py <leg_run_dir> [<base_run_dir>]

Base default: research/inside-cache-2026-09-18/run-s7149b-cumulative
(CUMULATIVE 35330129145, inside_cache=1 + flush_diet=1 — the green baseline).

Checks (preregistered gates §S7-150, protocol S7-148):
  G1 fluid-push family CPU ↓ >= 60% (weighted collapsed-stack samples of
     stacks passing through Entity.updateFluidHeightAndDoFluidPushing OR the
     FluidPushOps bridge), ItemEntity / Zombie sub-lane shares;
  G2 0 NoClassDefFoundError in server-stdout.log + ARMED chain markers
     (defined FluidPushOps / Retargeted / serve / retransform rc=0);
  G3 population parity (INJECT DONE + VALID + TOPUP-SCAN alive twins);
  G4 TPS crawl not below base min-of-2 (regress > 10% => REFUTED);
  G6 young GC count (gc,start + Pause Young lines) not above base.
"""
import os
import re
import sys
import collections

BASE_DEFAULT = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
                "run-s7149b-cumulative")
FLUID_ANCHORS = ("updateFluidHeightAndDoFluidPushing", "FluidPushOps")
ENTITY_LOOP_KEY = "ServerLevel.tickNonPassenger"


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


def lane_stats(rows, anchor_pred, class_pred=None):
    """Weighted samples of stacks matching anchor; optionally scoped to a
    class tick frame (class_pred on the stack string)."""
    hits = 0
    sub = collections.Counter()
    for stack, w in rows:
        if class_pred is not None and not class_pred(stack):
            continue
        if anchor_pred(stack):
            hits += w
            # first anchor below the entity tick frame (sub-lane shape)
            frames = stack.split(";")
            below = False
            for fr in frames:
                if ENTITY_LOOP_KEY in fr:
                    below = True
                elif below and anchor_pred(fr):
                    key = fr.split(".")[-1] if "." in fr else fr
                    sub[key] += w
                    break
    return hits, sub


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


def pop_counts(path):
    out = []
    for ln in open(path, errors="replace"):
        if "TOPUP-SCAN" in ln or "POPULATION" in ln:
            out.append(ln.strip()[:160])
    return out


def armed_lines(path):
    out = []
    for ln in open(path, errors="replace"):
        low = ln.lower()
        if any(k in low for k in ("fluidpushops", "fluid_dirty", "fluiddirty",
                                  "retargeted", "retransform")):
            out.append(ln.strip()[:200])
    return out


def section(name, d, is_dir=True):
    p = os.path.join(d, name)
    return p


def report(tag, d):
    cpu = section("cpu-collapsed.txt", d)
    stdout = section("server-stdout.log", d)
    gc = section("gc.log", d)
    print(f"### {tag}: {d}")
    if not os.path.exists(cpu):
        print("  !! cpu-collapsed.txt missing")
        return None
    total, rows = load_collapsed(cpu)
    fluid, _ = lane_stats(rows, lambda s: any(a in s for a in FLUID_ANCHORS))
    ie_key = "net/minecraft/world/entity/item/ItemEntity.tick"
    zb_key = "net/minecraft/world/entity/monster/Zombie.tick"
    fluid_ie, _ = lane_stats(rows, lambda s: any(a in s for a in FLUID_ANCHORS),
                             lambda s: ie_key in s)
    fluid_zb, _ = lane_stats(rows, lambda s: any(a in s for a in FLUID_ANCHORS),
                             lambda s: zb_key in s)
    ie_total = sum(w for s, w in rows if ie_key in s)
    zb_total = sum(w for s, w in rows if zb_key in s)
    pcg = sum(w for s, w in rows if "PalettedContainer.get" in s)
    print(f"  total={total} fluid_family={fluid} ({100.0*fluid/total:.2f}%)")
    print(f"  ItemEntity.tick total={ie_total} fluid={fluid_ie} "
          f"({100.0*fluid_ie/max(ie_total,1):.1f}% lane)")
    print(f"  Zombie.tick      total={zb_total} fluid={fluid_zb} "
          f"({100.0*fluid_zb/max(zb_total,1):.1f}% lane)")
    print(f"  PalettedContainer.get={pcg} ({100.0*pcg/total:.2f}%)")
    if os.path.exists(gc):
        print(f"  young_gc={young_gc(gc)}")
    if os.path.exists(stdout):
        tps = tps_crawl(stdout)
        print(f"  TPS crawl: {tps}")
        nc = sum(1 for ln in open(stdout, errors="replace")
                 if "NoClassDefFoundError" in ln)
        print(f"  NoClassDefFoundError lines={nc}")
        if tag.startswith("leg"):
            for ln in armed_lines(stdout)[-25:]:
                print(f"    ARMED? {ln}")
    return {"total": total, "fluid": fluid, "young_gc": None}


def main():
    leg = sys.argv[1]
    base = sys.argv[2] if len(sys.argv) > 2 else BASE_DEFAULT
    b = report("base", base)
    l = report("leg", leg)
    if b and l and b["fluid"]:
        red = 100.0 * (b["fluid"] - l["fluid"]) / b["fluid"]
        print(f"\n=== G1 fluid-family reduction: {b['fluid']} -> {l['fluid']} "
              f"= -{red:.1f}% (gate >= 60%) ===")
    return 0


if __name__ == "__main__":
    sys.exit(main())
