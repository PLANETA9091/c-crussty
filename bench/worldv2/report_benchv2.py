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
_envp = os.path.join(d.rstrip('/'), "run-env.txt")  # AG-370 w526 B-canon: run-env also in server dir (artifact-canonical) - read FIRST
if not os.path.exists(_envp):  # legacy/A-variant fallback: work-dir copy one level above
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
# AG-116 w528 (#16g-sibling): 'Marked N chunks' is a world-COMPLETION line (DimForceload
# prints it only when a world finishes pregen). A DRAIN-BOUND leg that times out mid-gen
# prints ZERO completion lines -> report marked=0, G4 FAIL, ch/s=None even when gen was
# healthy (leg 37026771618: completion 0, [DF] PROGRESS truth 23113@DRAIN-TOUT=9.58 ch/s
# in-window, 26590@last=9.74 — healthy band 9.1-12.0, AG-73). Fallback: per-world MAX of
# monotone PROGRESS marked counter; used only when it exceeds completion-sum.
prog = {}
last_prog_ts = None
gen_start_ts = None
for l in lines:
    _pm = re.search(r"\[DF\] PROGRESS world=(\S+) marked=(\d+)/(\d+)", l)
    if _pm:
        _w = _pm.group(1); _v = int(_pm.group(2))
        if _v > prog.get(_w, 0):
            prog[_w] = _v
        last_prog_ts = re.search(r"\[(\d\d:\d\d:\d\d)", l)
    elif gen_start_ts is None:
        _gs = re.search(r"\[(\d\d:\d\d:\d\d) INFO\].*\[DF\] GEN-START world=", l)
        if _gs:
            gen_start_ts = _gs.group(1)
marked_prog = sum(prog.values())
if marked_prog > marked:
    marked = marked_prog
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
window_s = None
if first_ts and drain_ts and int(drain_ts) > int(first_ts) and marked:
    window_s = int(drain_ts) - int(first_ts)
    ch_s = marked / window_s
# AG-116 w528: DRAIN-BOUND ch/s recovery (lower-bound canon AG-400/AG-71): when the leg
# timed out (drain_ts missing) but PROGRESS shows real marked progress, recover ch/s from
# the log's own wall-clock GEN-START -> last-PROGRESS window (self-contained, no epoch/tz
# mix; midnight-wrapped deltas +86400). Provenance-flagged in BENCHV2.md.
ch_prog_used = False
if ch_s is None and marked_prog and gen_start_ts and last_prog_ts:
    def _t2s(_t):
        _h, _m, _s = (int(x) for x in _t.split(":")); return _h * 3600 + _m * 60 + _s
    _d = _t2s(last_prog_ts.group(1)) - _t2s(gen_start_ts)
    if _d <= 0:
        _d += 86400
    if _d > 0:
        ch_s = marked_prog / _d
        window_s = _d
        ch_prog_used = True

# AG-372 w527 FALSE-DRAIN autogate (class AG-196 phantom 730.32 / AG-126 r576
# win249<floor254 / AG-395 'ch/s без drain-s мусор'): window shorter than the
# physical floor marked/RATE_MAX => ch/s inflated phantom, not a verdict.
# RATE_MAX = max honest band 21.5 ch/s (r576 21.40 AG-126; AG-395 21.46 @953s).
# Flag-only: annotates BENCHV2.md, exit-code untouched (leg stays TPS-valid).
RATE_MAX = 21.5
try:
    for _l in open(_envp, encoding="utf-8", errors="replace"):
        _mr = re.search(r"false_drain_rate_max=(\d+\.?\d*)", _l.strip())
        if _mr:
            RATE_MAX = float(_mr.group(1)); break
except OSError:
    pass
floor_s = (marked / RATE_MAX) if marked else None
false_drain = bool(window_s and floor_s and window_s < floor_s)

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
rep.append((f"- ch/s (drain-def: marked chunks / (drain_ts − first_ts)): **{ch_s:.2f}**" + (" (PROGRESS-recovery AG-116, DRAIN-BOUND: drain_ts missing)" if ch_prog_used else "")) if ch_s else "- ch/s: DRAIN-TIMEOUT (lower bound only)")
rep.append(f"- FALSE-DRAIN gate (AG-372): window_s={window_s}, floor_s={floor_s:.0f} (=marked/{RATE_MAX:.1f}): " + ("FLAG-INFLATED — ch/s NOT a verdict (window < physical floor)" if false_drain else "PASS") if window_s else "- FALSE-DRAIN gate (AG-372): no window (DRAIN-TIMEOUT) — n/a")
rep.append(f"- forceload-marked chunks total: **{marked}** (expect ≥{g4_target} = 0.95×{n_dims}×{expect_pd}; radius-blocks side={2*((expect_pd ** 0.5) - 1) / 2 + 1:.0f})")
rep.append(f"- MSPT: idle≈{idle}, sustain-median≈{med} (spark mspt samples n={len(mspts)})")
rep.append(f"- TPS samples (spark tps): n={len(tps)}" + (f", min={min(tps)}, last={tps[-1]}" if tps else ""))
rep.append(f"- entity-tick share: see sparkprofile artifact (offline analysis)")
rep.append(f"- NCDFE={ncdfe} (canon T1=0 gate: {'PASS' if ncdfe == 0 else 'FAIL'}), AIOOBE={aioobe}")
rep.append(f"- G3 datapacks-enabled markers: {g3}/4 ({'PASS' if g3 >= 4 else 'FAIL'})")
rep.append(f"- G4 marked≥95%: {'PASS' if g4_pass else 'FAIL'}; G5 drain: {'PASS' if g5_pass else 'DRAIN-TIMEOUT'}")
rep.append(f"- drain window: first_ts={first_ts} drain_ts={drain_ts}")
_host = ""
if os.path.exists(_envp):
    _kv = {}
    for _l in open(_envp, encoding="utf-8", errors="replace"):
        for _m in re.finditer(r"([a-z_]+)=(\S+)", _l.strip()):  # AG-370: finditer - heredoc packs several k=v on one line
            if _m.group(1) not in _kv: _kv[_m.group(1)] = _m.group(2)
    _host = " ".join(f"{k}={_kv[k]}" for k in ("runner_cpu_index","runner_name","run_id","attempt") if k in _kv)
if _host:
    rep.append(f"- host: {_host} (AG-370 w526: AG-233 host-census enabler, cpu_index in-report; AG-324 v4 idea)")
open(os.path.join(d, "BENCHV2.md"), "w").write("\n".join(rep) + "\n")
print("\n".join(rep))
if ncdfe > 0 or g3 < 4 or not g4_pass:
    sys.exit(1)
