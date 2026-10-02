#!/usr/bin/env python3
"""n_scaling_verdict.py — N-scaling lane verdict between two bench-4 legs.

Parses recon_lanes.py output files (lane tables + spawn signature + churn from
BOTTLENECKS_3.md) of leg A (N=4 baseline) and leg B (N=16 probe), emits:
  - per-lane share + absolute-samples delta (share alone lies when totals differ)
  - spawn-lane totals, network lane, visibility leaf (sendChanges)
  - fixture/churn at both N
  - verdict vs >=3% MSPT replaceable-core gate

Usage: n_scaling_verdict.py <lanesA.txt> <bottlenecksA.md> <lanesB.txt> <bottlenecksB.md>
"""
import re, sys

def parse_lanes(path):
    lanes, leaves, spawn = {}, {}, {}
    cur_lane = None
    total = None
    in_leaves = False
    for line in open(path, encoding="utf-8", errors="replace"):
        m = re.match(r"^=== (\S+) \(cpu=(\d+)", line)
        if m:
            total = {"samples": int(m.group(2))}
            continue
        m = re.match(r"\| (\S[^|]*?) \| (\d+) \| ([\d.]+)% \|", line)
        if m and total is not None and not in_leaves:
            lanes[m.group(1).strip()] = (int(m.group(2)), float(m.group(3)))
            continue
        if line.strip().startswith("top leaves per lane"):
            in_leaves = True
            continue
        m = re.match(r"\s+\[(.+?)\] \(([\d.]+)%\)", line)
        if m and in_leaves:
            cur_lane = m.group(1)
            continue
        m = re.match(r"\s+(\d+)\s+(\S+)$", line)
        if m and in_leaves and cur_lane:
            leaves.setdefault(cur_lane, []).append((m.group(2), int(m.group(1))))
            continue
        m = re.match(r"\s+- (spawn|despawn): (\S+): (\d+) samples = ([\d.]+)%", line)
        if m:
            spawn[m.group(2)] = (int(m.group(3)), float(m.group(4)))
    return total, lanes, leaves, spawn

def parse_churn(path):
    t = open(path, encoding="utf-8", errors="replace").read()
    out = {}
    m = re.search(r"polls=(\d+) total=([\d.]+)\.\.([\d.]+) \(delta ([\d.]+), churn ([\d.]+)%\), summons=(\d+)", t)
    if m:
        out["churn_delta"] = float(m.group(4)); out["churn_pct"] = float(m.group(5))
    m = re.search(r"entity totals seen: \[([^\]]+)\]", t)
    if m:
        out["entities"] = [int(x) for x in m.group(1).split(",")]
    m = re.search(r"spark tick-monitor MSPT: avg \*\*([\d.]+)ms\*\*", t)
    if m:
        out["mspt"] = float(m.group(1))
    m = re.search(r"runner_cpu_index: (\d+)", t)
    if m:
        out["cpu_idx"] = int(m.group(1))
    return out

def main(fa, fb, fb_md=None, fa_md=None):
    (args) = sys.argv[1:]
    fa_lanes, fa_md, fb_lanes, fb_md = args[0], args[1], args[2], args[3]
    tA, lanesA, leavesA, spawnA = parse_lanes(fa_lanes)
    tB, lanesB, leavesB, spawnB = parse_lanes(fb_lanes)
    mA, mB = parse_churn(fa_md), parse_churn(fb_md)
    print(f"=== legA total_samples={tA['samples'] if tA else '?'} mspt={mA.get('mspt')} cpu={mA.get('cpu_idx')} "
          f"churn={mA.get('churn_delta')} ({mA.get('churn_pct')}%)")
    print(f"=== legB total_samples={tB['samples'] if tB else '?'} mspt={mB.get('mspt')} cpu={mB.get('cpu_idx')} "
          f"churn={mB.get('churn_delta')} ({mB.get('churn_pct')}%)")
    print("\n=== LANE DELTA (share_pp = B - A share points; abs = samples scaled to equal totals) ===")
    totA = tA['samples'] if tA else 1
    totB = tB['samples'] if tB else 1
    rows = []
    for lane, (sa, sha) in lanesA.items():
        sb, shb = lanesB.get(lane, (0, 0.0))
        absA = sa / totA; absB = sb / totB
        rows.append((shb - sha, lane, sha, shb, absA, absB))
    for dpp, lane, sha, shb, absA, absB in sorted(rows, reverse=True):
        flag = " <-- GROWTH" if dpp >= 0.5 and absB > absA * 1.25 else (" <-- SHRINK" if dpp <= -0.5 else "")
        print(f"  {lane:42s} {sha:6.2f}% -> {shb:6.2f}% (pp {dpp:+.2f}; abs/1000t {absA*1000:.1f} -> {absB*1000:.1f}){flag}")
    print("\n=== SPAWN LANE (bench-4 signature) ===")
    for k in sorted(set(spawnA) | set(spawnB)):
        sa, sha = spawnA.get(k, (0, 0.0)); sb, shb = spawnB.get(k, (0, 0.0))
        print(f"  {k:28s} {sha:6.2f}% -> {shb:6.2f}% (abs/1000t {sa/totA*1000:.2f} -> {sb/totB*1000:.2f})")
    print("\n=== VISIBILITY/NETWORK LEAVES ===")
    for lane in ("kernel: other", "network (kernel)", "moonrise/paper patches", "entities/mobs (kernel)"):
        for name, sa in leavesA.get(lane, []):
            sb = dict(leavesB.get(lane, [])).get(name, 0)
            if any(x in name for x in ("sendChanges", "Connection", "send", "Clientbound", "KeepAlive")):
                print(f"  [{lane}] {name}: {sa} -> {sb} (abs/1000t {sa/totA*1000:.2f} -> {sb/totB*1000:.2f})")
    print("\n=== VERDICT ===")
    print("  gate: a lane earns STEP-0 if replaceable core >= 3% MSPT at realistic N")
    print("  (see lane deltas above; family-aggregate or owner-gated paths otherwise)")

if __name__ == "__main__":
    main(*sys.argv[1:])
