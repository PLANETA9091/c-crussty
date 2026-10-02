#!/usr/bin/env python3
"""bench4_baseline.py — min-of-2 paired baseline verdict for BENCH-4 legs.

Parses BOTTLENECKS_3.md + run-env.txt from two leg dirs, emits:
  - per-leg: MSPT headline / steady-window avg, TPS steady series, cpu_idx,
    world sha, GC duty, churn delta, FIXTURE-VALIDITY
  - pairing verdict per S7-96d law (world_sha256 + runner_cpu_index)
  - min-of-2 conservative baseline (faster run = the bar for future A/B)

Usage: python3 bench4_baseline.py <legA_dir> <legB_dir>
"""
import json, os, re, sys

def parse_report(d):
    rep = {}
    p = os.path.join(d, "BOTTLENECKS_3.md")
    if not os.path.exists(p):
        return None
    t = open(p, encoding="utf-8", errors="replace").read()
    m = re.search(r"spark tick-monitor MSPT: avg \*\*([\d.]+)ms\*\* / min ([\d.]+)ms / max \*\*([\d.]+)ms\*\*", t)
    if m:
        rep["mspt_headline"] = tuple(float(x) for x in m.groups())
    m = re.search(r"whole run, \[⚡\] lines\)\s*\|\s*([\d.]+)\s*\|(?:[^|]*\|){4}\s*([\d.]+)\s*\|", t)
    if m:
        rep["mspt_window_min_avg"] = (float(m.group(1)), float(m.group(2)))
    m = re.search(r"first-of-window values: \[([^\]]+)\]", t)
    if m:
        vals = [float(x) for x in m.group(1).split(",")]
        rep["tps_first"] = vals[0]
        rep["tps_steady"] = vals[1:]
        rep["tps_steady_min"] = min(vals[1:])
        rep["tps_steady_max"] = max(vals[1:])
    m = re.search(r"total pause: \*\*([\d.]+) ms\*\*, avg \*\*([\d.]+) ms\*\*, max \*\*([\d.]+) ms\*\*", t)
    if m:
        rep["gc"] = tuple(float(x) for x in m.groups())  # total_ms, avg_ms, max_ms
    m = re.search(r"polls=(\d+) total=([\d.]+)\.\.([\d.]+) \(delta ([\d.]+), churn ([\d.]+)%\), summons=(\d+)", t)
    if m:
        rep["churn"] = {"polls": int(m.group(1)), "lo": float(m.group(2)),
                        "hi": float(m.group(3)), "delta": float(m.group(4)),
                        "pct": float(m.group(5)), "summons": int(m.group(6))}
    m = re.search(r"FIXTURE-VALIDITY: (VALID|INVALID)", t)
    rep["fixture"] = m.group(1) if m else "N/A"
    m = re.search(r"boot time ([\d.]+) s", t)
    if m:
        rep["boot_s"] = float(m.group(1))
    env = os.path.join(d, "run-env.txt")
    if os.path.exists(env):
        et = open(env, encoding="utf-8", errors="replace").read()
        m = re.search(r"runner_cpu_index: (\d+)", et)
        rep["cpu_idx"] = int(m.group(1)) if m else None
        m = re.search(r"world_sha256: ([0-9a-f]{16})", et)
        rep["world_sha"] = m.group(1) if m else None
        m = re.search(r"fake_players: (\d+)", et)
        rep["fake_players"] = int(m.group(1)) if m else None
        m = re.search(r"date_utc: (\S+)", et)
        rep["date_utc"] = m.group(1) if m else None
    return rep

def main(a, b):
    ra, rb = parse_report(a), parse_report(b)
    for name, r in ((a, ra), (b, rb)):
        if r is None:
            print(f"FATAL: no BOTTLENECKS_3.md in {name}"); return 2
        head = r.get("mspt_headline")
        win = r.get("mspt_window_min_avg")
        print(f"--- {os.path.basename(name)} (fp={r.get('fake_players')}, cpu_idx={r.get('cpu_idx')}, "
              f"world={r.get('world_sha')}, date={r.get('date_utc')})")
        if head:
            print(f"    MSPT headline: avg {head[0]} / min {head[1]} / max {head[2]}")
        if win:
            print(f"    [⚡] windows: min {win[0]} / avg {win[1]}")
        if "tps_steady" in r:
            print(f"    TPS: first {r['tps_first']}, steady {r['tps_steady_min']}-{r['tps_steady_max']}")
        if "gc" in r:
            print(f"    GC: total {r['gc'][0]}ms, avg {r['gc'][1]}ms, max {r['gc'][2]}ms")
        if "churn" in r:
            c = r["churn"]
            print(f"    churn: polls={c['polls']} delta={c['delta']} ({c['pct']}%) summons={c['summons']}")
        print(f"    fixture: {r['fixture']}")
    # pairing verdict
    print("\n=== PAIRING (S7-96d law) ===")
    ok_w = ra.get("world_sha") and ra.get("world_sha") == rb.get("world_sha")
    same_fp = ra.get("fake_players") == rb.get("fake_players")
    print(f"  world_sha256: {'MATCH' if ok_w else 'MISMATCH/UNKNOWN'} "
          f"({ra.get('world_sha')} vs {rb.get('world_sha')})")
    print(f"  fake_players: {'MATCH' if same_fp else 'MISMATCH'} "
          f"({ra.get('fake_players')} vs {rb.get('fake_players')})")
    print(f"  runner_cpu_index: {ra.get('cpu_idx')} vs {rb.get('cpu_idx')} — "
          f"{'PAIRED RUNNER' if ra.get('cpu_idx') == rb.get('cpu_idx') else 'DIFFERENT RUNNER (cross-run deltas variance-dominated; min-of-2 conservative only)'}")
    # min-of-2 baseline
    ha = ra.get("mspt_headline", [9e9])[0]
    hb = rb.get("mspt_headline", [9e9])[0]
    best, best_d = (ra, a) if ha <= hb else (rb, b)
    worst, worst_d = (rb, b) if ha <= hb else (ra, a)
    print("\n=== MIN-OF-2 BASELINE (conservative bar for future A/B) ===")
    if best.get("mspt_headline"):
        print(f"  baseline run = {os.path.basename(best_d)}")
        print(f"  MSPT: headline {best['mspt_headline'][0]}ms"
              + (f" / [⚡] avg {best['mspt_window_min_avg'][1]}ms" if best.get("mspt_window_min_avg") else ""))
        if "tps_steady" in best:
            print(f"  TPS steady: {best['tps_steady_min']}-{best['tps_steady_max']}")
        spread = abs(ha - hb) / min(ha, hb) * 100 if min(ha, hb) else 0
        print(f"  leg spread: {spread:.1f}% ({ha} vs {hb} headline)")
        print(f"  A/B gate law: any module win must clear >=3% vs THIS baseline, CI, min-of-2, same-boot or paired cpu_idx")
    return 0

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(__doc__); sys.exit(1)
    sys.exit(main(sys.argv[1], sys.argv[2]))
