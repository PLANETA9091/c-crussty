#!/usr/bin/env python3
"""analyze_absorb_s7147.py — §156 gate analysis for leg #2' INSIDE-CACHE
(run 35314220731) vs base census 35275967738.

Usage: python3 analyze_absorb_s7147.py <base_dir> <leg_dir>

Families (S7-134b census taxonomy, path-based on collapsed stacks):
  inside-blocks   : stack ~ checkInsideBlocks | forEachBlockIntersectedBetween
  movement-geom   : stack ~ collidedWithFluid | collidedWithShapeMovingFrom |
                            updateFluidHeightAndDoFluidPushing
  flushStep       : stack ~ flushStep
Leaves tracked: LongOpenHashSet, BlockPos$6, Vec3, AABB, Arrays.copyOf.
CPU lane: leaf PalettedContainer.get share of total cpu samples.
GC lane: 'Pause Young' pause count.
Fixture: INJECT DONE / FIXTURE-VALIDITY / alive-check from server-stdout.log.
"""
import re
import sys
import os
import collections


def parse_collapsed(path):
    """Return (total, by_leaf, by_stack_sub_counter) for async-profiler collapsed txt."""
    total = 0
    by_leaf = collections.Counter()
    paths = collections.Counter()
    if not os.path.exists(path):
        return total, by_leaf, paths
    PATH_KEYS = {
        "inside-blocks": ("checkInsideBlocks", "forEachBlockIntersectedBetween"),
        "movement-geom": ("collidedWithFluid", "collidedWithShapeMovingFrom",
                          "updateFluidHeightAndDoFluidPushing"),
        "flushStep": ("flushStep",),
    }
    with open(path, encoding="utf-8", errors="replace") as fh:
        for ln in fh:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            m = re.match(r"^(.*)\s(\d+)$", ln)
            if not m:
                continue
            stack, n = m.group(1), int(m.group(2))
            leaf = stack.rsplit(";", 1)[-1]
            total += n
            by_leaf[leaf] += n
            for fam, keys in PATH_KEYS.items():
                if any(k in stack for k in keys):
                    paths[fam] += n
    return total, by_leaf, paths


def cpu_paletted_get(path):
    total = 0
    get = 0
    readpalette = 0
    if not os.path.exists(path):
        return total, get, readpalette
    with open(path, encoding="utf-8", errors="replace") as fh:
        for ln in fh:
            m = re.match(r"^(.*)\s(\d+)$", ln.rstrip("\n"))
            if not m:
                continue
            stack, n = m.group(1), int(m.group(2))
            leaf = stack.rsplit(";", 1)[-1]
            total += n
            if leaf == "net/minecraft/world/level/PalettedContainer.get":
                get += n
            if leaf.endswith("PalettedContainer.get"):
                get += n
            if "readPalette" in leaf:
                readpalette += n
    return total, get, readpalette


def young_gc(path):
    """Count young GC starts (gc,start lines only — each GC logs 2 lines)."""
    n = 0
    with open(path, encoding="utf-8", errors="replace") as fh:
        for ln in fh:
            if "[gc,start" in ln and "Pause Young" in ln:
                n += 1
    return n


def fixture(path):
    out = {}
    if not os.path.exists(path):
        return {"MISSING server-stdout.log": True}
    txt = open(path, encoding="utf-8", errors="replace").read()
    m = re.search(r"POPULATION INJECT DONE target=(\d+) injected=(\d+)", txt)
    if m:
        out["inject"] = f"{m.group(1)}/{m.group(2)}"
    m = re.search(r"FIXTURE-VALIDITY: (\w+)", txt)
    if m:
        out["validity"] = m.group(1)
    alives = re.findall(r"alive-check: level\.players\(\)=(\d+)", txt)
    out["alive_last"] = alives[-1] if alives else None
    out["armed_lines"] = [ln.strip()[:140] for ln in txt.splitlines()
                          if re.search(r"InsideBlockOps|Retargeted|retransform|ARMED|inside.?cache|INSIDE.?CACHE", ln, re.I)][:12]
    return out


def tps_crawl(path):
    if not os.path.exists(path):
        return []
    txt = open(path, encoding="utf-8", errors="replace").read()
    tps = re.findall(r"TPS from last 5s, 1m, 5m, 15m: ([0-9., ]+)", txt)
    return tps


def bottlenecks(path):
    out = {}
    if not os.path.exists(path):
        return out
    txt = open(path, encoding="utf-8", errors="replace").read()
    m = re.search(r"entity totals seen: \[([^\]]+)\]", txt)
    if m:
        out["entity_totals"] = m.group(1)[:80]
    phases = re.findall(r"\| (phase: [^|]+) \| (\d+) \| ([\d.]+)% \|", txt)
    out["phases"] = [(p.strip(), int(s), sh) for p, s, sh in phases][:8]
    return out


def main():
    base_dir, leg_dir = sys.argv[1], sys.argv[2]
    bt, bleaf, bpaths = parse_collapsed(os.path.join(base_dir, "alloc-collapsed.txt"))
    lt, lleaf, lpaths = parse_collapsed(os.path.join(leg_dir, "alloc-collapsed.txt"))
    print(f"=== ALLOC (base total {bt} / leg total {lt}) ===")
    for fam in ("inside-blocks", "movement-geom", "flushStep"):
        b, l = bpaths.get(fam, 0), lpaths.get(fam, 0)
        d = (100 * (b - l) / b) if b else float("nan")
        bs = f"{100*b/bt:5.2f}%" if bt else "  n/a"
        ls = f"{100*l/lt:5.2f}%" if lt else "  n/a"
        print(f"{fam:15s}: base {b:6d} ({bs})  leg {l:6d} ({ls})  delta {d:+.1f}%")
    for leaf in ("LongOpenHashSet", "BlockPos$6", "net.minecraft.world.phys.Vec3",
                 "net.minecraft.world.phys.AABB", "Object[]"):
        b = sum(v for k, v in bleaf.items() if leaf in k)
        l = sum(v for k, v in lleaf.items() if leaf in k)
        d = (100 * (b - l) / b) if b else float("nan")
        print(f"leaf {leaf:34s}: base {b:6d}  leg {l:6d}  delta {d:+.1f}%")
    print("=== CPU ===")
    for tag, d in (("base", base_dir), ("leg", leg_dir)):
        t, g, rp = cpu_paletted_get(os.path.join(d, "cpu-collapsed.txt"))
        gs = f"{100*g/t:.2f}%" if t else "n/a"
        print(f"{tag}: total {t}  PalettedContainer.get {g} ({gs})  readPalette {rp}")
    print("=== YOUNG GC ===")
    print(f"base: {young_gc(os.path.join(base_dir, 'gc.log'))}")
    print(f"leg : {young_gc(os.path.join(leg_dir, 'gc.log'))}")
    print("=== FIXTURE/ARMED (leg) ===")
    for k, v in fixture(os.path.join(leg_dir, "server-stdout.log")).items():
        print(f"{k}: {v}")
    print("=== TPS crawl (post lines) ===")
    for tag, d in (("base", base_dir), ("leg", leg_dir)):
        t = tps_crawl(os.path.join(d, "server-stdout.log"))
        print(f"{tag}: {t[-6:]}")
    print("=== BOTTLENECKS_3 (entity totals / phases) ===")
    for tag, d in (("base", base_dir), ("leg", leg_dir)):
        b = bottlenecks(os.path.join(d, "BOTTLENECKS_3.md"))
        print(tag, "entity_totals:", b.get("entity_totals"))
        for p, s, sh in b.get("phases", []):
            print(f"  {tag} {p}: {s} ({sh}%)")
    print("=== entity-recon F4 totals (first totals lines) ===")
    for tag, d in (("base", base_dir), ("leg", leg_dir)):
        p = os.path.join(d, "entity-recon.txt")
        if os.path.exists(p):
            lines = [ln for ln in open(p, encoding="utf-8", errors="replace")
                     if re.search(r"total|count|Count", ln)][:6]
            print(tag + ":", *lines[:6], sep="\n  ")


if __name__ == "__main__":
    main()
