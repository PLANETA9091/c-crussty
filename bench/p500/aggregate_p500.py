#!/usr/bin/env python3
"""P500 aggregator — turns raw bench TSV (RESULT/SKIP/CRASH lines) into a
markdown report with per-group ns/op tables and old-vs-optimized speedups.

Fast kernels are near the noise floor on shared CI hardware; the report
therefore prints speedups with the caveat that only ratios > 1.15x and
< 0.85x are treated as meaningful (|Δ| within ±15% is "parity").
"""
import sys, collections, statistics, os, datetime

def load(path):
    results, notes = [], []
    with open(path) as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("RESULT\t"):
                _, gid, fqcn, sig, method, kind, med, mn, mx, status = line.split("\t")
                med, mn, mx = float(med), float(mn), float(mx)
                results.append(dict(gid=int(gid), fqcn=fqcn, sig=sig, method=method,
                                    kind=kind, med=med, mn=mn, mx=mx, status=status))
            elif line.startswith(("SKIP\t", "CRASH\t", "SINK\t")):
                notes.append(line)
    return results, notes

def fmt_ns(v):
    if v != v:  # NaN
        return "-"
    for unit, div in (("ns", 1), ("µs", 1e3), ("ms", 1e6)):
        if v < 1000 * div or unit == "ms":
            return f"{v/div:,.1f} {unit}"
    return f"{v:,.1f} ns"

def main(path):
    results, notes = load(path)
    # dedupe (gid, method): keep the LAST occurrence (retry runs are fresher)
    best = {}
    for r in results:
        best[(r["gid"], r["method"])] = r
    results = sorted(best.values(), key=lambda r: (r["gid"], r["method"]))
    now = datetime.datetime.now(datetime.timezone.utc)
    out = []
    w = out.append
    w("# P500 benchmark report — Crussty CE native kernels (old vs optimized)")
    w("")
    w(f"* Generated: {now.isoformat(timespec='seconds')}")
    w(f"* Raw data: `{os.path.basename(path)}` — one JVM fork per group, "
      "time-bounded batches (~120 ms), median of 5, identical synthesized args per group.")
    w("* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.")
    w("")
    crashes = [n for n in notes if n.startswith("CRASH")]
    skips = [n for n in notes if n.startswith("SKIP")]
    w(f"* Groups measured: {len(set(r['gid'] for r in results))}, "
      f"skipped: {len(skips)}, crashed: {len(crashes)}, kernels measured: {len(results)}")
    w("")

    by_group = collections.defaultdict(list)
    for r in results:
        by_group[r["gid"]].append(r)

    speedups = []  # (speedup, fqcn, sig, oldname, altname, oldmed, altmed)

    w("## Per-group results")
    w("")
    for gid in sorted(by_group):
        rs = sorted(by_group[gid], key=lambda r: r["med"])
        fqcn = rs[0]["fqcn"]; sig = rs[0]["sig"]
        w(f"### {gid}. `{fqcn}` `{sig}`")
        w("")
        w("| kernel | kind | median ns/op | min | max |")
        w("|---|---|---:|---:|---:|")
        old = next((r for r in rs if r["kind"] == "old"), None)
        for r in rs:
            w(f"| `{r['method']}` | {r['kind']} | {fmt_ns(r['med'])} | {fmt_ns(r['mn'])} | {fmt_ns(r['mx'])} |")
        if old and old["status"].startswith("OK"):
            for r in rs:
                if r["kind"] == "alt" and r["status"].startswith("OK") and r["med"] > 0:
                    sp = old["med"] / r["med"]
                    speedups.append((sp, fqcn, sig, old["method"], r["method"], old["med"], r["med"]))
        w("")

    speedups.sort(reverse=True)
    w("## Top wins (optimized faster than old)")
    w("")
    w("| speedup | class | old kernel | optimized kernel | old | optimized |")
    w("|---|---:|---|---|---:|---:|")
    for sp, fqcn, sig, om, am, omed, amed in speedups:
        if sp >= 1.15:
            w(f"| {sp:,.2f}x | `{fqcn}` | `{om}` | `{am}` | {fmt_ns(omed)} | {fmt_ns(amed)} |")
    if not any(sp >= 1.15 for sp, *_ in speedups):
        w("| — no wins beyond noise floor — | | | | | |")
    w("")
    w("## Regressions (optimized SLOWER than old) — optimization targets")
    w("")
    w("| speedup | class | old kernel | optimized kernel | old | optimized |")
    w("|---|---:|---|---|---:|---:|")
    reg = [s for s in speedups if s[0] <= 0.85]
    for sp, fqcn, sig, om, am, omed, amed in reg:
        w(f"| {sp:,.2f}x | `{fqcn}` | `{om}` | `{am}` | {fmt_ns(omed)} | {fmt_ns(amed)} |")
    if not reg:
        w("| — no regressions beyond noise floor — | | | | | |")
    w("")
    w("## Parity (within ±15%)")
    w("")
    par = [s for s in speedups if 0.85 < s[0] < 1.15]
    w(", ".join(f"`{fqcn}.{am}` ({sp:.2f}x)" for sp, fqcn, sig, om, am, *_ in par) or "—")
    w("")
    if skips or crashes:
        w("## Notes")
        w("")
        for n in skips + crashes:
            w(f"* `{n}`")
        w("")
    print("\n".join(out))

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "results/p500_raw.tsv")
