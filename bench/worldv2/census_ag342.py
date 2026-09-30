#!/usr/bin/env python3
"""census_ag342.py — BENCH-V2 per-dim entity census parser (AG-342, wave-515).

Parses the scoreboard census rounds (console, identical instrument in both
legs) + [BenchV2Census] plugin lines (fake-player leg) from server-stdout.log,
emits census.csv and appends a PER-DIM CENSUS section to BENCHV2.md.
G6-FPV2 prereg (work/AG-342/SPEC.md): leg-A (fake_players=0) sustain entity
total across dims expected < 200 (spawn vacuum, task170 precedent); leg-B
(fake_players=6) expected >= 500 total with growth (spawn lanes active).
Informational exit 0 always — verdict math is offline, never breaks G-gates.
"""
import os, re, statistics, sys

d = sys.argv[1]
logp = os.path.join(d, "server-stdout.log")
lines = open(logp, encoding="utf-8", errors="replace").read().splitlines()

env = {}
envp = os.path.join(d, "run-env.txt")
if os.path.exists(envp):
    for l in open(envp):
        for tok in l.split():
            if "=" in tok:
                k, v = tok.split("=", 1)
                env[k] = v
fp = env.get("fake_players", "?")

# scoreboard get output: "#c_ov ... <N>" (dim-tagged fake player name)
sb = []
for l in lines:
    m = re.search(r"#c_(ov|ne|en).*?(\d+)\s*\[?benchv2c", l)
    if m:
        sb.append((m.group(1), int(m.group(2))))
# plugin census (leg-B only): [BenchV2Census] dim=minecraft:overworld entities=N players=M
pc = []
for l in lines:
    m = re.search(r"\[BenchV2Census\] dim=(\S+) entities=(\d+) players=(\d+)", l)
    if m:
        pc.append((m.group(1), int(m.group(2)), int(m.group(3))))
alive = []
for l in lines:
    m = re.search(r"\[BenchFakePlayers\] alive-check: injected=(\d+)", l)
    if m:
        alive.append(int(m.group(1)))

def med(xs):
    return statistics.median(xs) if xs else None

out = ["# BENCHV2 PER-DIM CENSUS — AG-342 (wave-515) leg fake_players=%s\n" % fp]
csv = ["dim,count"]
dims = {"ov": "overworld", "ne": "the_nether", "en": "the_end"}
verdict = "NO-DATA"
if sb:
    per = {k: [c for (t, c) in sb if t == k] for k in dims}
    for k, name in dims.items():
        m = med(per[k])
        out.append("- %s: census rounds n=%d, median entities=%.1f, max=%d"
                   % (name, len(per[k]), m, max(per[k])) if per[k] else "- %s: no census samples" % name)
        for c in per[k]:
            csv.append("%s,%d" % (name, c))
    tot = [sum(t) for t in zip(*[per[k] for k in dims if per[k]])] if any(per.values()) else []
    mtot = med(tot) if tot else None
    growth = (max(sum(t) for t in zip(*[per[k] for k in dims if per[k]])) -
              min(sum(t) for t in zip(*[per[k] for k in dims if per[k]]))) if tot else 0
    out.append("- TOTAL across dims: median=%.1f (spread %d)" % (mtot, growth) if mtot is not None else "- TOTAL: n/a")
    if fp == "0":
        verdict = ("SPAWN-VACUUM-CONFIRMED (canon leg total=%.1f < 200)" % mtot) if mtot is not None and mtot < 200 \
                  else "CANON-NOT-VACUUM (total=%.1f >= 200 — H1 REFUTED)" % (mtot or -1)
    else:
        verdict = ("SPAWN-LANES-ACTIVE (leg-B total=%.1f >= 500)" % mtot) if mtot is not None and mtot >= 500 \
                  else "LEG-B-DEAD (total=%.1f < 500 — fixture broken, H1 REFUTED)" % (mtot or -1)
else:
    out.append("- no scoreboard census parsed (console format drift?)")
if pc:
    perdim = {}
    for (dim, n, p) in pc:
        perdim.setdefault(dim, []).append(n)
    for dim, xs in perdim.items():
        out.append("- plugin-census %s: n=%d median=%d" % (dim, len(xs), med(xs)))
        csv.extend("%s,%d" % (dim, n) for n in xs)
if alive:
    out.append("- alive-check: first=%d last=%d (players stayed = %s)"
               % (alive[0], alive[-1], "YES" if alive[-1] == alive[0] else "NO-DROP"))
out.append("- G6-FPV2 verdict: **%s**" % verdict)
out.append("")
open(os.path.join(d, "BENCHV2.md"), "a").write("\n".join(out) + "\n")
open(os.path.join(d, "census.csv"), "w").write("\n".join(csv) + "\n")
print("\n".join(out))
sys.exit(0)
