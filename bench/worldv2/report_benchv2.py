#!/usr/bin/env python3
"""report_benchv2.py — BENCH-V2 metric report (AG-433, wave-515).
Parses server-stdout.log: marked chunks (all dims), drain ch/s, TPS/MSPT
samples, NCDFE gate; emits BENCHV2.md with preregistered gates G1-G5."""
import re, sys, os

d, first_ts, drain_ts = sys.argv[1], sys.argv[2], sys.argv[3]
logp = os.path.join(d, "server-stdout.log")
lines = open(logp, encoding="utf-8", errors="replace").read().splitlines()

# AG-496 x522: radius-aware G4 (hardcoded 20449 broke any radius!=71 leg: perfect
# pregen run still FAILed the 58272 gate). Read radius_blocks from run-env.txt
# (written by run_benchv2.sh at $WORK/run-env.txt, one level above server dir).
# Canon radius 1136 blocks => side=71 => expect=20449 (byte-identical old gate).
expect_pd = 20449
_envp = os.path.join(os.path.dirname(d.rstrip('/')), "run-env.txt")
try:
    for _l in open(_envp, encoding="utf-8", errors="replace"):
        _m = re.match(r"radius_blocks=(\d+)", _l.strip())
        if _m:
            _rb = int(_m.group(1))
            _side = (2 * ((_rb + 15) // 16) + 1)
            expect_pd = _side * _side
            break
except OSError:
    pass
# AG-120 x523: dims-aware G4 (hardcoded x3 broke legal single-dim legs:
# marked<=20449 can never reach 0.95*3*20449 even on a perfect pregen run).
n_dims = 3
try:
    for _l in open(_envp, encoding="utf-8", errors="replace"):
        _md = re.search(r"dims=([^#\n]+)", _l.strip())
        if _md:
            n_dims = max(1, len([x for x in _md.group(1).split(",") if x.strip()]))
            break
except OSError:
    pass
g4_target = int(0.95 * n_dims * expect_pd)

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
g4_pass = marked >= g4_target
g5_pass = bool(drain_ts) and drain_ts != "None"

rep = []
rep.append("# BENCHV2 — AG-433 wave-515 heavy stand (AG-496 x522: radius-aware gates, pregen-v3)\n")
rep.append(f"- ch/s (drain-def: marked chunks / (drain_ts − first_ts)): **{ch_s:.2f}**" if ch_s else "- ch/s: DRAIN-TIMEOUT (lower bound only)")
rep.append(f"- forceload-marked chunks total: **{marked}** (expect ≥{g4_target} = 0.95×{n_dims}×{expect_pd}; radius-blocks side={2*((expect_pd ** 0.5) - 1) / 2 + 1:.0f})")
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
