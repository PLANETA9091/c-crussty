#!/usr/bin/env python3
"""report_benchv2.py — BENCH-V2 metric report (AG-433, wave-515).
Parses server-stdout.log: marked chunks (all dims), drain ch/s, TPS/MSPT
samples, NCDFE gate; emits BENCHV2.md with preregistered gates G1-G5."""
import re, sys, os

d, first_ts, drain_ts = sys.argv[1], sys.argv[2], sys.argv[3]
logp = os.path.join(d, "server-stdout.log")
lines = open(logp, encoding="utf-8", errors="replace").read().splitlines()

# AG-395 fix (blocker #4): inner loop var was also the walrus target 'm' ->
# SyntaxError: comprehension inner loop cannot rebind assignment expression target 'm'
# (report crashed at parse time, FAIL=1, no BENCHV2 metrics — run-36794417339). Inner var
# renamed m2; inner mspt regex given a capturing group so group(1) is valid.
# AG-342 fix (blocker #11 parse, canary-3 233KB log: 0 'mspt' lines, n=0 samples):
# console spark on Purpur 1.21.10 prints 'TPS from last 5s, 10s, 1m, 5m, 15m:' with the
# float row on the NEXT line ('[⚡]  20.0, 20.0, ...') and 'Tick durations
# (min/med/95%ile/max ms) from last 10s, 1m:' with values 'min/med/95/max; min/med/95/max'
# on the NEXT spark line (log lines 4143-4250 proof). Parse next-line formats.
marked = sum(int(m.group(1)) for l in lines if (m := re.search(r"Marked (\d+) chunks", l)))
tps = []
mspts = []
for _i, _l in enumerate(lines):
    if "TPS from last" in _l and _i + 1 < len(lines):
        _m = re.search(r"([0-9]+\.[0-9]+)", lines[_i + 1])
        if _m:
            tps.append(float(_m.group(1)))
    if "Tick durations" in _l:
        for _j in range(_i + 1, min(_i + 4, len(lines))):
            _m = re.search(r"([0-9.]+)/([0-9.]+)/([0-9.]+)/([0-9.]+)", lines[_j])
            if _m:
                mspts.append(float(_m.group(2)))   # med ms, last-10s group
                break
ncdfe = sum("NoClassDefFoundError" in l for l in lines)
aioobe = sum("ArrayIndexOutOfBoundsException" in l for l in lines)

ch_s = None
if first_ts and drain_ts and int(drain_ts) > int(first_ts) and marked:
    ch_s = marked / (int(drain_ts) - int(first_ts))

idle = mspts[0] if mspts else None
sust = mspts[-12:] if len(mspts) > 3 else mspts
med = sorted(sust)[len(sust) // 2] if sust else None

# AG-342 fix (blocker #10, canary-3 BENCHV2.md G3=1/4 false-fail): Paper lists all 4 packs
# on ONE line ('There are 7 data pack(s) enabled: [file/terralith.zip (world)], ...') ->
# counting LINES gives 1; count OCCURRENCES like the boot gate (grep -o | wc -l canon, blocker #3).
g3 = sum(len(re.findall(r"\[(file/)?(terralith|tectonic|incendium|stellarity)", l)) for l in lines)
g4_pass = marked >= int(0.95 * 3 * 20449)
g5_pass = bool(drain_ts) and drain_ts != "None"

rep = []
rep.append("# BENCHV2 — AG-433 wave-515 (heavy stand, 3 dims × 20449 chunks)\n")
rep.append(f"- ch/s (drain-def: marked chunks / (drain_ts − first_ts)): **{ch_s:.2f}**" if ch_s else "- ch/s: DRAIN-TIMEOUT (lower bound only)")
rep.append(f"- forceload-marked chunks total: **{marked}** (expect ≥58272 = 0.95×3×20449)")
rep.append(f"- MSPT: idle≈{idle}, sustain-median≈{med} (spark mspt samples n={len(mspts)})")
rep.append(f"- TPS samples (spark tps): n={len(tps)}" + (f", min={min(tps)}, last={tps[-1]}" if tps else ""))
rep.append(f"- entity-tick share: see sparkprofile artifact (offline analysis)")
rep.append(f"- NCDFE={ncdfe} (canon T1=0 gate: {'PASS' if ncdfe == 0 else 'FAIL'}), AIOOBE={aioobe}")
rep.append(f"- G3 datapacks-enabled markers: {g3}/4 ({'PASS' if g3 >= 4 else 'FAIL'})")
rep.append(f"- G4 marked≥95%: {'PASS' if g4_pass else 'FAIL'}; G5 drain: {'PASS' if g5_pass else 'DRAIN-TIMEOUT'}")
rep.append(f"- drain window: first_ts={first_ts} drain_ts={drain_ts}")
open(os.path.join(d, "BENCHV2.md"), "w").write("\n".join(rep) + "\n")
print("\n".join(rep))
if ncdfe > 0 or g3 < 4 or not g4_pass:
    sys.exit(1)
