#!/usr/bin/env python3
# TASK-459-L09: pre-soak GC-цензус из server-stdout (блок "Garbage Collector statistics")
import os, re, json, datetime as dt

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"
rows = json.load(open("/home/z/c-crussty/research/round-459/gc_stw_table.json"))

pat_full = re.compile(r"PS MarkSweep collector:\s*\n\s*([\d.]+)( ms| s) avg, (\d+) total collections\s*\n\s*(?:(\d+)h )?(?:(\d+)m )?([\d.]+)s avg frequency")
pat_young = re.compile(r"PS Scavenge collector:\s*\n\s*([\d.]+)( ms| s) avg, (\d+) total collections")

for r in rows:
    d = os.path.join(BASE, r["leg"], "server-stdout.log")
    if not os.path.isfile(d):
        r["pre_full"] = None; continue
    txt = open(d, errors="ignore").read()
    m = pat_full.search(txt)
    if m:
        avg_ms = float(m.group(1)) * (1 if m.group(2) == " ms" else 1000)
        r["pre_full_n"] = int(m.group(3)); r["pre_full_avgms"] = round(avg_ms)
    else:
        r["pre_full_n"] = None; r["pre_full_avgms"] = None
    m = pat_young.search(txt)
    if m:
        avg_ms = float(m.group(1)) * (1 if m.group(2) == " ms" else 1000)
        r["pre_young_n"] = int(m.group(3)); r["pre_young_avgms"] = round(avg_ms)
    else:
        r["pre_young_n"] = None; r["pre_young_avgms"] = None
    # Fulls в soak = total(absorb/gc.log) - pre
    if r["pre_full_n"] is not None and r["full"] is not None:
        r["soak_full"] = r["full"] - r["pre_full_n"]

json.dump(rows, open("/home/z/c-crussty/research/round-459/gc_stw_table.json", "w"), indent=1, ensure_ascii=False)
print(f"{'leg':28s} {'norm':>7s} {'Full_tot':>8s} {'preFull':>7s} {'preAvg':>7s} {'soakFull':>8s} {'preScav':>7s} {'scavAvg':>7s} {'max':>5s}")
for r in rows:
    print(f"{r['leg']:28s} {r['norm']:+7.1f} {r['full']:8d} {str(r.get('pre_full_n')):>7s} "
          f"{str(r.get('pre_full_avgms')):>7s} {str(r.get('soak_full')):>8s} "
          f"{str(r.get('pre_young_n')):>7s} {str(r.get('pre_young_avgms')):>7s} {r['max_ms']:5d}")
