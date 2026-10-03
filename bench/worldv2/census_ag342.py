#!/usr/bin/env python3
"""census_ag342.py — BENCH-V2 per-dim entity census parser (AG-342, wave-515; v2 AG-374 wave-527).

v2 (AG-374, fork of work/AG-344 fix-plan): the console scoreboard census is
dim-ALIASED in this engine — `execute in <dim> as @e[...]` from the console
does NOT re-scope the entity set (AG-344: c_ov==c_ne==c_en byte-identical,
n=2 runs dgw1024/dgw2048). Consequences fixed here:
  (a) alias detection: all triples equal -> instrument is aliased; ne/en
      scoreboard values are overworld aliases, not per-dim data. G-DIM
      loaded=0 for nether/end is used as corroboration.
  (b) TOTAL = overworld count (v1 summed 3 aliased values = 3x overworld).
  (c) fp=0: hard vacuum bar <200 RETIRED on the aliased instrument —
      natural spawns on 20k stands give 0.8-2.3k entities (AG-344), so v1
      emitted false CANON-NOT-VACUUM / H1-REFUTED. v2 reports ov median +
      G-DIM chunk context; vacuum claim needs a true per-dim instrument.
  (d) fp!=0: SPAWN-LANES-ACTIVE requires true per-dim evidence — plugin
      census ([BenchV2Census]) or a non-aliased instrument. Otherwise
      LEG-B-INDETERMINATE (v1 total>=500 was vacuum-blind: 823 naturals
      already pass the bar; AG-291/AG-344 class).
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
# plugin census: [BenchV2Census] dim=minecraft:overworld entities=N players=M
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
# G-DIM plugin census (run_benchv2.sh post-drain call, tail per world):
# "G-DIM world=world loaded=N" — chunk context for natural-spawn normalization
gd = {}
for l in lines:
    m = re.search(r"G-DIM world=([a-z_]+) loaded=(\d+)", l)
    if m:
        gd[m.group(1)] = int(m.group(2))

def med(xs):
    return statistics.median(xs) if xs else None

out = ["# BENCHV2 PER-DIM CENSUS — AG-342 (wave-515) leg fake_players=%s (parser v2 AG-374 alias-aware)\n" % fp]
csv = ["dim,count"]
dims = {"ov": "overworld", "ne": "the_nether", "en": "the_end"}
verdict = "NO-DATA"
alias = None
if sb:
    per = {k: [c for (t, c) in sb if t == k] for k in dims}
    # v2: alias detection on aligned triples (ov,ne,en) in log order
    tri = [tuple(sb[i:i + 3][j][1] for j in range(3))
           for i in range(0, len(sb) - 2, 3)
           if [t for (t, _) in sb[i:i + 3]] == ["ov", "ne", "en"]]
    if tri:
        alias = all(a == b == c for (a, b, c) in tri)
    for k, name in dims.items():
        if per[k]:
            m = med(per[k])
            tag = " (ALIASED=ov value, not per-dim)" if (alias and k != "ov") else ""
            out.append("- %s: census rounds n=%d, median entities=%.1f, max=%d%s"
                       % (name, len(per[k]), m, max(per[k]), tag))
            for c in per[k]:
                csv.append("%s,%d" % (name, c))
        else:
            out.append("- %s: no census samples" % name)
    if alias:
        mov = med(per["ov"]) if per["ov"] else None
        out.append("- CENSUS-ALIAS DETECTED: c_ov==c_ne==c_en byte-identical across %d rounds "
                   "(engine: console `execute in <dim> as @e` does not re-scope @e; AG-344 canon, n=2)" % len(tri))
        out.append("- TOTAL (corrected, overworld-only): median=%s  [v1 printed 3x this value]"
                   % ("%.1f" % mov if mov is not None else "n/a"))
        gdn = gd.get("world_nether"); gde = gd.get("world_the_end")
        if gdn is not None or gde is not None:
            out.append("- G-DIM corroboration: world loaded=%s nether=%s end=%s (0-loaded dims cannot hold entities)"
                       % (gd.get("world"), gdn, gde))
        if fp == "0":
            verdict = ("VACUUM-INDETERMINATE (console census dim-aliased; hard bar <200 RETIRED — "
                       "natural spawns 0.8-2.3k on 20k stands, work/AG-344; ov median=%s; "
                       "vacuum claim needs a true per-dim instrument)"
                       % ("%.1f" % mov if mov is not None else "n/a"))
        else:
            verdict = ("LEG-B-INDETERMINATE (console census dim-aliased + no plugin census lines: "
                       "v1 total>=500 was vacuum-blind — 823 naturals already pass; work/AG-344)")
    else:
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
    pdim_players = {}
    for (dim, n, p) in pc:
        perdim.setdefault(dim, []).append(n)
        pdim_players.setdefault(dim, []).append(p)
    ptotal = med([sum(t) for t in zip(*perdim.values())]) if perdim else None
    players_max = max(max(xs) for xs in pdim_players.values()) if pdim_players else 0
    for dim, xs in perdim.items():
        out.append("- plugin-census %s: n=%d median=%d (true per-dim, players max=%d)"
                   % (dim, len(xs), med(xs), max(pdim_players[dim])))
        csv.extend("%s,%d" % (dim, n) for n in xs)
    out.append("- plugin-census TOTAL (true): median=%s" % ("%.1f" % ptotal if ptotal is not None else "n/a"))
    if fp != "0":
        if players_max > 0 and ptotal is not None and ptotal >= 500:
            verdict = "SPAWN-LANES-ACTIVE (plugin census true total=%.1f >= 500, fake players alive max=%d)" % (ptotal, players_max)
        elif players_max == 0:
            verdict = "LEG-B-DEAD (plugin census: 0 fake players alive in all dims)"
        else:
            verdict = "LEG-B-DEAD (plugin census true total=%.1f < 500 — fixture broken, H1 REFUTED)" % (ptotal or -1)
    elif fp == "0" and alias is not False:
        ptotal = ptotal or 0
        verdict = ("VACUUM-CONFIRMED (plugin census true total=%.1f < 200)" % ptotal) if ptotal < 200 \
                  else ("VACUUM-NOT-CONFIRMED (plugin census true total=%.1f; on 20k stands natural spawns "
                        "give 0.8-2.3k — compare per-1000-loaded-chunks density, work/AG-344)" % ptotal)
if alive:
    out.append("- alive-check: first=%d last=%d (players stayed = %s)"
               % (alive[0], alive[-1], "YES" if alive[-1] == alive[0] else "NO-DROP"))
out.append("- G6-FPV2 verdict: **%s**" % verdict)
out.append("")
open(os.path.join(d, "BENCHV2.md"), "a").write("\n".join(out) + "\n")
open(os.path.join(d, "census.csv"), "w").write("\n".join(csv) + "\n")
print("\n".join(out))
sys.exit(0)
