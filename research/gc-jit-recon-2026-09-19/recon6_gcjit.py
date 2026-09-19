#!/usr/bin/env python3
"""RECON-6: decompose GC/JIT-native phase (TOP-2, 35.49% CPU in v3 bank leg)
into sub-lanes by three axes: (a) CPU frames, (b) gc.log pauses + alloc rate,
(c) alloc-collapsed allocation sites.

Artifact: run-s7162-leg2-artifact (run 35399980345, CUMULATIVE v3 bank).
Phase classifier kept IDENTICAL to absorb_s7162.py (GC/JIT-native lane).
"""
import re
import os
import statistics
import collections

ART = ("/home/z/c-crussty/research/batch-collector-2026-09-19/"
       "run-s7162-leg2-artifact")
OUT = "/home/z/c-crussty/research/gc-jit-recon-2026-09-19"

# ---- classifiers (first match wins for sub-lanes) --------------------------
def is_gcjit_frame(f):
    return ("G1" in f or "OopOopIterate" in f or "oopDesc::size" in f
            or "do_oop_work" in f or "GCTask" in f or "CompileBroker" in f
            or "C2Compile" in f or "c2_init" in f or "CardSet" in f)

def is_gcjit_row(frames):
    return any(is_gcjit_frame(f) for f in frames)

# thread-root based sub-lanes (first match wins)
SUB_LANES = [
    ("GC-concurrent REFINEMENT (G1ConcurrentRefineThread)", lambda fs: any(
        "G1ConcurrentRefineThread" in f for f in fs)),
    ("GC-concurrent MARKING (G1CMTask)", lambda fs: any(
        ("G1CMTask" in f or "G1CMConcurrentMarkingTask" in f) for f in fs)),
    ("JIT-compile (C1/C2/broker)", lambda fs: any(
        ("CompileBroker" in f or "C2Compile" in f or "c2_init" in f
         or "ciEnv" in f or "PhaseIdealLoop" in f or "CompileTask" in f
         or "DirectiveSet" in f or "ciMethod" in f or "Matcher" in f
         or "Compile::" in f) for f in fs)),
    ("GC STW-worker EVACUATION (scan_roots/trim)", lambda fs: any(
        ("G1EvacuateRegions" in f or "G1ParScanThreadState" in f
         or "G1ScanHRForRegion" in f or "G1ScanCardClosure" in f
         or "scan_heap_roots" in f) for f in fs)),
    ("GC STW-worker REBUILD-RS/scrub", lambda fs: any(
        ("G1RebuildRSAndScrub" in f or "G1RebuildRemSet" in f
         or "HeapRegionManager::par_iterate" in f) for f in fs)),
    ("GC other STW (other GCTask/work)", lambda fs: any(
        ("GCTask" in f or "WorkerThread" in f or "do_oop_work" in f
         or "OopOopIterate" in f or "oopDesc::size" in f) for f in fs)),
    ("GC remset/card other (CardSet/refine misc)", lambda fs: any(
        ("CardSet" in f or "G1RemSet" in f or "G1Update" in f
         or "G1ConcurrentRefine" in f or "G1CardTable" in f) for f in fs)),
    ("GC service/other (G1Service/oldgen)", lambda fs: any(
        ("G1Service" in f or "G1OldGen" in f or "G1Monitoring" in f
         or "G1Periodic" in f) for f in fs)),
    ("VM-thread/safepoint (stw infra)", lambda fs: any(
        ("VMThread" in f or "VM_Operation" in f or "SafepointSynchronize" in f
         or "HandshakeState" in f or "VMError" in f) for f in fs)),
]
UNCLASS = "unclassified-in-phase"


def load_collapsed(path):
    rows, total = [], 0
    with open(path, errors="replace") as f:
        for ln in f:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            m = re.match(r"^(.*) (\d+)$", ln)
            if not m:
                continue
            frames = m.group(1).split(";")
            w = int(m.group(2))
            rows.append((frames, w))
            total += w
    return rows, total


def lane_of(frames):
    for name, fn in SUB_LANES:
        if fn(frames):
            return name
    return UNCLASS


def main():
    os.makedirs(OUT, exist_ok=True)
    out_lines = []

    def say(s=""):
        print(s)
        out_lines.append(s)

    say("# RECON-6: GC/JIT-native phase decomposition (v3 bank leg 35399980345)")
    say()

    # ---------- axis (a): CPU sub-lanes ----------
    cpu_path = os.path.join(ART, "cpu-collapsed.txt")
    rows, total = load_collapsed(cpu_path)
    say(f"## (a) CPU axis — cpu-collapsed total {total} samples")
    say()

    phase_rows = [(fs, w) for fs, w in rows if is_gcjit_row(fs)]
    phase_total = sum(w for _, w in phase_rows)
    say(f"GC/JIT-native phase (classifier as absorb_s7162.py): "
        f"{phase_total} samples = {100.0*phase_total/total:.2f}% CPU")
    say()
    say("| sub-lane | samples | % CPU total | % phase |")
    say("|---|---|---|---|")
    lanes = collections.Counter()
    for fs, w in phase_rows:
        lanes[lane_of(fs)] += w
    for name in [n for n, _ in SUB_LANES] + [UNCLASS]:
        w = lanes.get(name, 0)
        if w:
            say(f"| {name} | {w} | {100.0*w/total:.2f}% | {100.0*w/phase_total:.2f}% |")
    say()

    # frame frequency inside phase
    say("### top-40 frames inside GC/JIT phase")
    say()
    say("| frame | occurrences (weighted) | % phase |")
    say("|---|---|---|")
    fr = collections.Counter()
    for fs, w in phase_rows:
        for f in fs:
            fr[f] += w
    for f, w in fr.most_common(40):
        say(f"| `{f}` | {w} | {100.0*w/phase_total:.2f}% |")
    say()

    # top-15 full stacks of phase
    say("### top-15 collapsed stacks inside phase")
    say()
    for fs, w in sorted(phase_rows, key=lambda x: -x[1])[:15]:
        say(f"- {w} ({100.0*w/phase_total:.2f}% phase): `{';'.join(fs[-8:])}`")
    say()

    # ---------- axis (b): gc.log ----------
    say("## (b) gc.log axis — pauses and allocation rate")
    say()
    gc_path = os.path.join(ART, "gc.log")
    young = []           # (gcid, iso_time, uptime_s, cause, before, after, cap, ms)
    concurrent = 0
    full = 0
    pat = re.compile(
        r"\[([0-9T:.+\-]+)\]\[([0-9.]+)s\]\[info\]\[gc +\] GC\((\d+)\) "
        r"Pause Young \((.+?)\) (\d+)M->(\d+)M\((\d+)M\) ([0-9.]+)ms")
    with open(gc_path, errors="replace") as f:
        for ln in f:
            if "[gc,start" in ln:
                continue
            if "Pause Full" in ln:
                full += 1
                continue
            if re.search(r"\[gc +\] GC\(\d+\) Pause (Concurrent|Remark|Cleanup)", ln):
                concurrent += 1
            m = pat.search(ln)
            if not m:
                continue
            t_iso, up, gcid, cause, before, after, cap, ms = m.groups()
            young.append((int(gcid), t_iso, float(up), cause,
                          int(before), int(after), int(cap), float(ms)))

    n = len(young)
    ms_all = [y[7] for y in young]
    say(f"young events (whole run): {n}  concurrent-mark pauses: {concurrent}  "
        f"full: {full}")
    if n:
        say(f"pause ms: total={sum(ms_all):.0f} mean={statistics.mean(ms_all):.2f} "
            f"median={statistics.median(ms_all):.2f} p90="
            f"{sorted(ms_all)[int(0.9*n)]:.2f} max={max(ms_all):.1f}")
    causes = collections.Counter(y[3] for y in young)
    for c, k in causes.most_common():
        say(f"  cause '{c}': {k}")
    say()

    # live-scene segment: use soak boundary (TPS line 22:15:47 = pre-inject idle)
    SOAK_T = "22:15:47"
    live = [y for y in young if y[1][11:19] >= SOAK_T]
    say(f"live-scene segment (t >= {SOAK_T} UTC, soak+post-soak): "
        f"{len(live)} events")
    if live:
        ml = [y[7] for y in live]
        say(f"live pauses ms: total={sum(ml):.0f} mean={statistics.mean(ml):.2f} "
            f"median={statistics.median(ml):.2f} max={max(ml):.1f}")
        # allocation rate per interval (eden fill = before_i)
        rates = []
        prev_t = None
        for gcid, t_iso, up, cause, before, after, cap, ms in live:
            if prev_t is not None and up - prev_t > 0:
                rates.append(before / (up - prev_t))
            prev_t = up
        if rates:
            say(f"alloc rate (MB/s between consecutive young GCs): "
                f"mean={statistics.mean(rates):.1f} median="
                f"{statistics.median(rates):.1f} max={max(rates):.1f} "
                f"(n={len(rates)})")
        # per-minute GC count and pause share
        span_s = live[-1][2] - live[0][2]
        if span_s > 0:
            stw_s = sum(ml) / 1000.0
            say(f"span {span_s:.0f}s: STW wall-share {100.0*stw_s/span_s:.2f}% "
                f"({sum(ml)/span_s:.1f} ms pause per second wall)")
    say()

    # inter-GC interval histogram (live segment)
    if live:
        gaps = [live[i][2] - live[i-1][2] for i in range(1, len(live))]
        say("inter-GC gap s: min=%.2f p25=%.2f median=%.2f p75=%.2f max=%.2f"
            % (min(gaps), sorted(gaps)[len(gaps)//4],
               statistics.median(gaps), sorted(gaps)[3*len(gaps)//4], max(gaps)))
        say()

    # ---------- axis (c): alloc-collapsed ----------
    say("## (c) alloc axis — top allocation sites (bytes, whole profile window)")
    say()
    alloc_path = os.path.join(ART, "alloc-collapsed.txt")
    arows, atot = load_collapsed(alloc_path)
    leaves = collections.Counter()
    for fs, w in arows:
        leaves[fs[-1]] += w
    say(f"total bytes weight: {atot:,} (units = bytes of collapsed counts)")
    say()
    say("| # | allocation leaf | weight | % total |")
    say("|---|---|---|---|")
    for i, (f, w) in enumerate(leaves.most_common(40), 1):
        say(f"| {i} | `{f}` | {w:,} | {100.0*w/atot:.2f}% |")
    say()

    # map top-60 leaves to v3 lanes (entity phases)
    LANE_SIGS = [
        ("inside-pipeline", lambda fs: any(
            ("checkInsideBlocks" in f or "forEachBlockIntersectedBetween" in f
             or "collectTrav" in f or "TraverseOps" in f) for f in fs)),
        ("collector/batch", lambda fs: any(
            ("StepBasedCollector" in f or "BatchCollector" in f
             or "flushStep" in f or "advanceStep" in f) for f in fs)),
        ("fluid-scan", lambda fs: any("collidedWithFluid" in f
                                      or "FluidState" in f for f in fs)),
        ("broadphase/collision/push", lambda fs: any(
            ("Level碰撞" in f or "getCollisions" in f or "push" in f.lower()
             or "EntityGetter" in f or "CollisionGetter" in f) for f in fs)),
        ("tickBucket-orch", lambda fs: any("RegionTickOps" in f for f in fs)),
        ("AI/brain/sensing", lambda fs: any(
            ("Brain" in f or "Behavior" in f or "goalSelector" in f
             or "GoalSelector" in f or "sensing" in f.lower()) for f in fs)),
        ("movement/nav", lambda fs: any(
            ("navigate" in f.lower() or "moveControl" in f.lower()
             or "travel" in f or "AiStep" in f) for f in fs)),
        ("chunk/lightsky", lambda fs: any(
            ("Chunk" in f or "lighten" in f or "LightEngine" in f) for f in fs)),
    ]
    sig_count = collections.Counter()
    sig_weight = collections.Counter()
    for fs, w in arows:
        leaf = fs[-1]
        rank = [i for i, (lf, _) in enumerate(leaves.most_common(60))
                if lf == leaf]
        if not rank:
            continue
        for name, fn in LANE_SIGS:
            if fn(fs):
                sig_count[name] += 1
                sig_weight[name] += w
                break
        else:
            sig_count["other-lanes"] += 1
            sig_weight["other-lanes"] += w
    say("### top-60 alloc leaves mapped to lanes")
    say()
    say("| lane | leaves | weight | % top60 |")
    say("|---|---|---|---|")
    t60 = sum(sig_weight.values())
    for name, w in sig_weight.most_common():
        say(f"| {name} | {sig_count[name]} | {w:,} | {100.0*w/t60:.1f}% |")
    say()

    with open(os.path.join(OUT, "RECON6_GCJIT_raw.txt"), "w") as f:
        f.write("\n".join(out_lines) + "\n")
    print("\nsaved:", os.path.join(OUT, "RECON6_GCJIT_raw.txt"))


if __name__ == "__main__":
    main()
