#!/usr/bin/env python3
"""STEP-0 GC-SHAPE-1 sizing: parse gc.log steady-state young-GC stats + alloc rate."""
import re, statistics, sys

def gc_stats(path, label):
    lines = open(path, errors='ignore').read().splitlines()
    pauses, eden = [], []
    for ln in lines:
        m = re.search(r'\]\[gc\s*\] GC\(\d+\) (Pause.*?)\s+([\d.]+)M->([\d.]+)M\((\d+)M\)\s+([\d.]+)ms', ln)
        if m:
            ts = re.search(r'\]\[(\d+\.\d+)s\]', ln)
            pauses.append((float(ts.group(1)) if ts else 0.0, m.group(1), float(m.group(3)), float(m.group(5))))
        e = re.search(r'GC\(\d+\) Eden regions: (\d+)->\d+\((\d+)\)', ln)
        if e:
            ts = re.search(r'\]\[(\d+\.\d+)s\]', ln)
            eden.append((float(ts.group(1)) if ts else 0.0, int(e.group(1)), int(e.group(2))))
    if not pauses:
        print(label, 'no pauses'); return
    t0, t1 = pauses[0][0], pauses[-1][0]
    sstart = t0 + (t1 - t0) / 3.0
    st = [p for p in pauses if p[0] >= sstart]
    ste = [e for e in eden if e[0] >= sstart]
    totp = sum(p[3] for p in st)
    print(f"=== {label}: all pauses={len(pauses)}, steady(start>={sstart:.0f}s)={len(st)}")
    if st:
        span = t1 - sstart
        print(f"    steady young GC duty {totp/span*100:.2f}% of wall ({totp:.0f}ms / {span:.0f}s)")
        print(f"    avg pause {totp/len(st):.1f}ms, max {max(p[3] for p in st):.1f}ms, kinds: {sorted(set(p[1][:24] for p in st))}")
    if len(ste) >= 2:
        fills = [e[1] * 4 for e in ste]  # eden regions x 4MB
        ivs = [b[0] - a[0] for a, b in zip(ste, ste[1:])]
        med_fill, med_iv = statistics.median(fills), statistics.median(ivs) if ivs else 30
        print(f"    eden fill median {med_fill:.0f}MB, interval median {med_iv:.1f}s -> alloc ~{med_fill/max(med_iv,1):.1f}MB/s")
    print("    last 6 steady pauses:", [(f"{p[0]:.0f}s", p[1][:22], f"{p[3]:.0f}ms") for p in st[-6:]])

for p, l in zip(sys.argv[1::2], sys.argv[2::2]):
    gc_stats(p, l)
