#!/usr/bin/env python3
"""TASK-88 S7-32: boot-window JFR attribution (cold start, CDS-enabled).
Parses `jfr print --events jdk.ExecutionSample` text output, keeps samples
inside the boot window (before the 'Done' wall-clock second), and prints:
  - thread share
  - top leaf methods (CPU truth)
  - top 3-frame stack prefixes (paths)
Usage: analyze_boot_jfr.py <jfr_print.txt> <HH:MM:SS of Done line> [tolerance_seconds]
"""
import re
import sys
from collections import Counter

def main():
    path, done_ts = sys.argv[1], sys.argv[2]
    tol = float(sys.argv[3]) if len(sys.argv) > 3 else 0.5
    h, m, s = (float(x) for x in done_ts.split(":"))
    done_s = h * 3600 + m * 60 + s

    blocks = re.findall(
        r"jdk\.ExecutionSample \{(.*?)\n\}", open(path).read(), re.S)
    kept, dropped = [], 0
    for b in blocks:
        tsm = re.search(r"startTime = (\d+):(\d+):([\d.]+)", b)
        if not tsm:
            continue
        hh, mm, ss = float(tsm.group(1)), float(tsm.group(2)), float(tsm.group(3))
        t = hh * 3600 + mm * 60 + ss
        if t > done_s + tol:
            dropped += 1
            continue
        thr = re.search(r'sampledThread = "([^"]+)"', b)
        frames = re.findall(r"^\s{4}(\S+)\(", b, re.M)
        if frames:
            kept.append((thr.group(1) if thr else "?", frames))
    print(f"samples kept {len(kept)}, dropped-post-Done {dropped}")

    thr_c = Counter(t for t, _ in kept)
    print("\n== THREADS ==")
    for t, c in thr_c.most_common(8):
        print(f"  {t}: {c}")

    leaf = Counter(fr[0].split("(")[0].rsplit(".", 1)[-1] for _, fr in kept)
    print("\n== TOP LEAVES (method) ==")
    for f, c in leaf.most_common(20):
        print(f"  {c:4d}  {f}")

    leafcls = Counter()
    for _, fr in kept:
        parts = fr[0].split("(")[0].split(".")
        leafcls[".".join(parts[:2])] += 1
    print("\n== TOP LEAF CLASSES ==")
    for f, c in leafcls.most_common(20):
        print(f"  {c:4d}  {f}")

    path3 = Counter(tuple(
        f.split("(")[0].rsplit(".", 1)[-1] for f in fr[:3]) for _, fr in kept)
    print("\n== TOP 3-FRAME PATHS ==")
    for p, c in path3.most_common(25):
        print(f"  {c:4d}  {' <- '.join(p)}")

if __name__ == "__main__":
    main()
