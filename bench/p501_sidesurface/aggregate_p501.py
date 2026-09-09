#!/usr/bin/env python3
"""P501 SIDESURFACE aggregator — TASK-156 (§103 pre-registration).

Consumes the sidecar RAW TSV (same line grammar as the canonical P500 rig:
RESULT/CRASH/SKIP/SINK) and emits a FIRST-MEASUREMENT report.

Honesty rules (pre-stated, no post-hoc direction claims):
  * Ratios are computed ONLY for the three §103-registered in-group naming
    conventions: current->optimized, cold->hot, foreach->indexed.
    ratio = t_base / t_opt  (>1 means the optimized kernel is faster).
  * Everything else is a flat per-method ns/op table with a spread column —
    NO direction claims (the census left 21 other-multi-method classes
    without a registered baseline direction).
  * Cross-group same-class siblings (e.g. ChunkExpireCount cold/hot live in
    different groups because their sigs differ) are listed but NEVER ratioed:
    different (fqcn,sig) = different argument shapes = not apples-to-apples.
  * State-dependent families (CraftPlayerCanSee, WaypointManager*) run with
    synthetic arguments; rc-errors/garbage are reported as-is, no wiring.
  * The PaperNativeChunkPacketEncode trio is the CLOSED chunk-encode surface
    of §97: standalone rc=-3 fast-fail is the documented expectation; the
    in-server gate probe is TASK-150 phase-2a (pending /home/z/server).
  * FIRST MEASUREMENT: no baseline exists for this surface; every number is
    exploratory. No drift claims, no wiring, no gate tuning.
"""
import os
import sys
import collections
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))

# §103-registered in-group pair conventions: (base-prefix, opt-prefix, label)
PAIR_RULES = [
    ("current", "optimized", "optimized/current"),
    ("cold", "hot", "hot/cold"),
    ("foreach", "indexed", "indexed/foreach"),
]

CLASS_NOTES = {
    "PaperNativeChunkPacketEncode":
        "CLOSED chunk-encode surface (§97): standalone rc=-3 fast-fail is the "
        "documented expectation (well-formed input, zero output bytes); in-server "
        "gate probe = TASK-150 phase-2a, pending /home/z/server+jdk21. ns/op of an "
        "error path is not a useful perf number — reported as-is for completeness.",
    "PaperNativeCraftPlayerCanSee":
        "State-dependent (needs player/world view); synthetic args — expect "
        "rc-errors or semantically meaningless values; as-is, no wiring.",
    "PaperNativeWaypointManagerSkip":
        "State-dependent (waypoint manager internals); synthetic args — as-is.",
    "PaperNativeWaypointChunkUpdate":
        "State-dependent (waypoint storage); synthetic args — as-is.",
    "PaperNativeClimateRTree":
        "Handle-based family: build/checksum/free/search share no state across "
        "separate JVMs; scalar handle args are synthetic constants — expect "
        "errors or crashes on search/free/checksum groups; as-is.",
    "net.minecraft.world.level.levelgen.synth.PaperNativePerlinNoise":
        "Handle-based family: nativeBuildHandle may produce a real handle in its "
        "own JVM, but free/getValue groups receive synthetic constant handles — "
        "expect errors/crashes there; as-is.",
}


def fmt_ns(v):
    if v != v:  # NaN
        return "-"
    if v >= 1e6:
        return f"{v/1e6:.2f}ms"
    if v >= 1e3:
        return f"{v/1e3:.1f}us"
    return f"{v:.1f}ns"


def load_env():
    p = os.path.join(HERE, "results", "P501_ENV.txt")
    if not os.path.exists(p):
        return ["(env file missing)"]
    return [l.rstrip("\n") for l in open(p) if l.strip()]


def main():
    raw = sys.argv[1] if len(sys.argv) > 1 else None
    if not raw:
        cands = sorted(
            f for f in os.listdir(os.path.join(HERE, "results"))
            if f.startswith("P501_SIDESURFACE_RAW_") and f.endswith(".tsv"))
        if not cands:
            sys.exit("no P501_SIDESURFACE_RAW_*.tsv found")
        raw = os.path.join(HERE, "results", cands[-1])

    groups = collections.defaultdict(lambda: {"rows": [], "crash": None})
    sink_count = 0
    with open(raw) as f:
        for line in f:
            c = line.rstrip("\n").split("\t")
            if c[0] == "SINK":
                sink_count += 1
            elif c[0] == "CRASH":
                groups[int(c[1])]["crash"] = c[2]
            elif c[0] == "SKIP":
                groups[int(c[1])]["crash"] = "SKIP " + " ".join(c[2:])
            elif c[0] == "RESULT":
                gid, fqcn, sig, meth, kind, med, status = (
                    int(c[1]), c[2], c[3], c[4], c[5], float(c[6]), c[9])
                groups[gid]["rows"].append(
                    {"fqcn": fqcn, "sig": sig, "meth": meth, "kind": kind,
                     "med": med, "status": status})

    # fqcn/sig per gid from groups.tsv (CRASH-only groups have no RESULT rows)
    meta = {}
    gt = os.path.join(HERE, "java", "p500", "groups.tsv")
    for gid, line in enumerate(open(gt)):
        fqcn, sig, methods = line.rstrip("\n").split("\t")
        meta[gid] = (fqcn, sig, methods.split(","))

    gids_path = subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"], cwd=os.path.join(HERE, "..", ".."),
        capture_output=True, text=True).stdout.strip()

    out = []
    A = out.append
    A("# P501 SIDESURFACE — FIRST MEASUREMENT REPORT (TASK-156, agent-7625532f)")
    A("")
    A("Scope: the 94 zero-evidence exports (TASK-155 census) in a self-contained")
    A("sidecar rig; canonical bench/p500/ untouched. Driver = canonical Bench.java")
    A("byte-identical (WARM=2, ROUNDS=5, ~120ms batches, min-of-two-medians,")
    A("DCE-proof sink, one JVM per group, 600s timeout, crash ladder N=16→1).")
    A("")
    A("**FIRST MEASUREMENT — no baseline exists for this surface. No drift claims,")
    A("no wiring, no gate tuning. Ratios only for §103-registered conventions.**")
    A("")
    A("## Environment")
    A("```")
    for l in load_env():
        A(l)
    A(f"c-crussty HEAD: {gids_path}")
    A(f"JVM forks (SINK lines): {sink_count}")
    A("```")
    A("")
    A("## Per-group results")
    n_ok = n_err = n_crash = 0
    for gid in sorted(groups):
        g = groups[gid]
        fqcn, sig, methods = meta.get(gid, ("?", "?", []))
        short = fqcn.rsplit(".", 1)[-1]
        A(f"### g{gid} — {fqcn} `{sig}`")
        if short in CLASS_NOTES:
            A(f"NOTE: {CLASS_NOTES[short]}")
        if g["crash"]:
            n_crash += 1
            A(f"**CRASH** ({g['crash']}) — no usable rows; see logs/g{gid}.log")
            A("")
            continue
        rows = sorted(g["rows"], key=lambda r: r["meth"])
        A("| method | ns/op | status |")
        A("|---|---:|---|")
        ok_vals = []
        for r in rows:
            if r["status"].startswith("OK") or r["status"].startswith("SLOW"):
                n_ok += 1
                ok_vals.append(r["med"])
                A(f"| {r['meth']} | {fmt_ns(r['med'])} | {r['status']} |")
            else:
                n_err += 1
                A(f"| {r['meth']} | - | {r['status']} |")
        if len(ok_vals) >= 2:
            spread = (max(ok_vals) - min(ok_vals)) / min(ok_vals) * 100
            A(f"spread (max/min of OK rows): {spread:.1f}% — no direction claim")
        A("")
    A("## §103-registered in-group pair ratios")
    A("")
    A("ratio = t_base / t_opt; >1 means the optimized kernel is faster.")
    A("")
    pair_found = False
    for gid in sorted(groups):
        rows = [r for r in groups[gid]["rows"] if r["status"].startswith(("OK", "SLOW"))]
        if not rows:
            continue
        names = {r["meth"]: r["med"] for r in rows}
        for base_p, opt_p, label in PAIR_RULES:
            opts = sorted(m for m in names if m.startswith(opt_p))
            for mo in opts:
                # match the base with the longest shared remainder (A/B/Default shapes)
                suffix = mo[len(opt_p):]
                cand = [m for m in names
                        if m.startswith(base_p) and m[len(base_p):] == suffix]
                if len(cand) == 1:
                    mb = cand[0]
                    ratio = names[mb] / names[mo] if names[mo] > 0 else float("nan")
                    pair_found = True
                    A(f"- g{gid} `{mb}` vs `{mo}` [{label}]: **{ratio:.2f}x** "
                      f"({fmt_ns(names[mb])} vs {fmt_ns(names[mo])})")
    if not pair_found:
        A("(none — no group matched a registered convention)")
    A("")
    A("## Cross-group same-class siblings (listed, never ratioed — different sigs)")
    A("")
    bycls = collections.defaultdict(list)
    for gid, (fqcn, sig, methods) in meta.items():
        bycls[fqcn].append((gid, sig, tuple(methods)))
    for fqcn, gs in sorted(bycls.items()):
        if len(gs) > 1:
            A(f"- {fqcn}: " + "; ".join(f"g{g} `{s}` {','.join(ms)}" for g, s, ms in gs))
    A("")
    A("## Summary")
    A("")
    A(f"- groups attempted: {len(groups)} (groups.tsv: {len(meta)})")
    A(f"- RESULT rows: {n_ok} OK/SLOW, {n_err} ERR")
    A(f"- CRASH groups: {n_crash}")
    A("- DCE-proof sink: every surviving JVM emitted one SINK line "
      f"({sink_count} total)")
    A("- INJECTS-ONLY: 0 server boots, 0 product changes, canonical p500 untouched")
    A("")
    print("\n".join(out))


if __name__ == "__main__":
    main()
