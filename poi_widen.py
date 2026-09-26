#!/usr/bin/env python3
"""poi_widen.py — TASK-456-B: STRICT-OR widen cmp456_poi to the FULL era
carrier union (law 7 composition of certified master effects).

Rule (per line): a line arming cmp450_chunk OR cmp453_diet (the newest
full-carrier needles) and not already arming cmp456_poi gains the
cmp456_poi needle with the same pattern. Skipped: match-DISPLAY arms
(`Ok("cmp453_diet") => ...`), comment-only lines (needle context = code
before trailing comment).

Master-effect canon: cmp450_chunk legs (chk455-2 cert) and cmp453_diet legs
(diet455/diet454 monsters) = the certified composite ins4 ⊕ senseins ⊕
chunk4-send ⊕ chunk5-encode ⊕ chunkparse ⊕ noise-GEN. cmp456_poi = same
union + the new POI subsystem plane (poi_plane.rs / PoiOps.java).
Mirror-drift lesson x451/x452: prod gates and test helpers in sync —
test-helper files (fix_452c_chains.py / union_resolve.py fixture lists)
are NOT touched here (they replay historical union merges, not the lever
contract); the .rs/.java/.sh surfaces are.

Usage: python3 poi_widen.py [--dry]
"""
import re
import subprocess
import sys

DRY = "--dry" in sys.argv
changed = 0

RUST_FILES = subprocess.run(
    ["git", "grep", "-l", "-E", "cmp453_diet|cmp450_chunk", "--", "*.rs"],
    capture_output=True, text=True).stdout.split()
JAVA_FILES = subprocess.run(
    ["git", "grep", "-l", "-E", "cmp453_diet|cmp450_chunk", "--", "*.java"],
    capture_output=True, text=True).stdout.split()


def widen_rust_line(ln):
    code = ln.split("//")[0]
    if "cmp456_poi" in code:
        return None
    if "cmp453_diet" not in code and "cmp450_chunk" not in code:
        return None
    if re.search(r'Ok\("cmp45[03][^"]*"\)\s*=>', code):
        return None  # display arm
    for needle in ('Ok("cmp453_diet")', 'Ok("cmp450_chunk")'):
        if needle in code:
            return ln.replace(needle, needle[:-1] + ' | Ok("cmp456_poi")', 1)
    for pat in ('f == "cmp453_diet"', 'v == "cmp453_diet"', 't == "cmp453_diet"',
                'f == "cmp450_chunk"', 'v == "cmp450_chunk"', 't == "cmp450_chunk"',
                'v.trim() == "cmp453_diet"', 'v.trim() == "cmp450_chunk"'):
        if pat in code:
            base = pat.split(" ")[0]  # f or v
            return ln.replace(pat, pat + f' || {base} == "cmp456_poi"', 1)
    return None


def widen_java_line(ln):
    if "cmp456_poi" in ln:
        return None
    for needle in ('equals("cmp453_diet")', 'equals("cmp450_chunk")'):
        if needle in ln:
            return ln.replace(needle, needle + ' || f.trim().equals("cmp456_poi")', 1)
    return None


for f in RUST_FILES:
    lines = open(f).read().split("\n")
    n = 0
    for i, ln in enumerate(lines):
        w = widen_rust_line(ln)
        if w is not None:
            lines[i] = w
            n += 1
    if n:
        changed += n
        print(f"{f}: {n} sites")
        if not DRY:
            open(f, "w").write("\n".join(lines))

for f in JAVA_FILES:
    lines = open(f).read().split("\n")
    n = 0
    for i, ln in enumerate(lines):
        w = widen_java_line(ln)
        if w is not None:
            lines[i] = w
            n += 1
    if n:
        changed += n
        print(f"{f}: {n} sites")
        if not DRY:
            open(f, "w").write("\n".join(lines))

print(f"{'DRY-RUN: ' if DRY else ''}total widened sites: {changed}")
