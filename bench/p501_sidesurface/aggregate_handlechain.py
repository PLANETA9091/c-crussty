#!/usr/bin/env python3
"""P501 HANDLE-CHAIN report assembler — TASK-157 (agent-7625532f).
Consumes the handle-chain RAW TSV (RESULT/CRASH/SINK grammar) and emits the
FIRST-MEASUREMENT report. All numbers are FIRST measurements: no baseline,
no drift claims, no wiring. Verdict per family: H-PASS / H-SENTINEL / H-CRASH
(pre-registered in CLAIMS TASK-157 before any data)."""
import os
import sys
import collections

HERE = os.path.dirname(os.path.abspath(__file__))

FAMILY_MAP = {
    "perlin": "net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise "
              "(nativeBuildHandle/nativeGetValue/nativeGetValueNoYScale/nativeFreeHandle)",
    "rtree": "net.minecraft.world.level.biome.PaperNativeClimateRTree "
             "(nativeBuildTreeHandle/nativeChecksumTreeHandle/nativeSearch*/nativeFreeTreeHandle)",
}


def load_env():
    p = os.path.join(HERE, "results", "P501_HANDLECHAIN_ENV.txt")
    if not os.path.exists(p):
        return ["(env file missing)"]
    return [l.rstrip("\n") for l in open(p) if l.strip()]


def fmt(ns):
    if ns < 0:
        return "-"
    if ns >= 1e6:
        return f"{ns/1e6:.2f}ms"
    if ns >= 1e3:
        return f"{ns/1e3:.1f}us"
    return f"{ns:.1f}ns"


def main():
    raw = sys.argv[1] if len(sys.argv) > 1 else None
    if not raw:
        cands = sorted(f for f in os.listdir(os.path.join(HERE, "results"))
                       if f.startswith("P501_HANDLECHAIN_RAW_") and f.endswith(".tsv"))
        if not cands:
            sys.exit("no P501_HANDLECHAIN_RAW_*.tsv found")
        raw = os.path.join(HERE, "results", cands[-1])

    fams = collections.defaultdict(list)
    crashes = {}
    sinks = []
    with open(raw) as f:
        for line in f:
            c = line.rstrip("\n").split("\t")
            if c[0] == "RESULT":
                fams[c[1]].append((c[2], float(c[3]), c[4]))
            elif c[0] == "CRASH":
                crashes[c[1]] = c[2]
            elif c[0] == "SINK":
                sinks.append(c[1])

    out = []
    A = out.append
    A("# P501 HANDLE-CHAIN — FIRST MEASUREMENT REPORT (TASK-157, agent-7625532f)")
    A("")
    A("In-JVM build→sentinel-check→consume→free chain for the handle families")
    A("TASK-156 crashed on (per-group JVMs cannot pass handles). Synthetic args,")
    A("canonical batch discipline (8 calls/iter, ~120ms, WARM=2/ROUNDS=5 median;")
    A("single-call medians for destructive free over untimed pools).")
    A("")
    A("**FIRST MEASUREMENT — no baseline, no drift claims, no wiring.**")
    A("")
    A("## Environment")
    A("```")
    for l in load_env():
        A(l)
    A("```")
    A("")
    verdicts = {}
    for fam in ("perlin", "rtree"):
        A(f"## Family {fam} — {FAMILY_MAP[fam]}")
        if fam in crashes:
            verdicts[fam] = "H-CRASH"
            A(f"**CRASH** ({crashes[fam]}) — JVM died; see logs/handlechain_{fam}.log")
            A("")
            continue
        rows = fams.get(fam, [])
        if not rows:
            A("(no rows)")
            A("")
            continue
        build = next((r for r in rows if r[0] == "build"), None)
        if build and ("BUILD-SENTINEL" in build[2] or "BUILD-EXC" in build[2]):
            verdicts[fam] = "H-SENTINEL"
        elif build and "BUILD-OK" in build[2]:
            verdicts[fam] = "H-PASS"
        else:
            verdicts[fam] = "H-UNKNOWN"
        A("| op | ns/op | status |")
        A("|---|---:|---|")
        for op, ns, status in rows:
            A(f"| {op} | {fmt(ns)} | {status} |")
        A("")
    A("## Verdicts (pre-registered tree)")
    A("")
    for fam in ("perlin", "rtree"):
        A(f"- {fam}: **{verdicts.get(fam, 'NO-DATA')}**")
    A("")
    A(f"## Summary")
    A("")
    A(f"- SINK lines (DCE-proof): {len(sinks)} ({','.join(sinks) or 'none'})")
    A("- INJECTS-ONLY: 0 server boots, 0 product changes; sidecar-own probe,")
    A("  canonical p500 + generated trees untouched")
    A("")
    print("\n".join(out))


if __name__ == "__main__":
    main()
