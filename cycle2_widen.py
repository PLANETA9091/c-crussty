#!/usr/bin/env python3
"""cycle2_widen.py — TASK-455-B cycle-2 (law 3): widen cmp450_chunk to the
FULL diet-profile union.

Rule (per line): a line that arms cmp453_diet (lever needle) and does not
already arm cmp450_chunk gains the cmp450_chunk needle with the same pattern.
Skipped: match-DISPLAY arms (`Ok("cmp453_diet") => ...`), comment-only lines.
Diet CUT canon preserved (subadditivity x3):
  * BrainOps tick2 (TICK2_FLAGS contains no cmp453_diet -> untouched, dormant)
  * chunk_send5 keeps its existing cmp450_chunk arm (chk454-3 measured set)
  * noise_fill GEN axis stays armed (law 8 owner-visible axis)
Usage: python3 cycle2_widen.py [--dry]
"""
import re
import subprocess
import sys

DRY = "--dry" in sys.argv
changed = 0

RUST_FILES = subprocess.run(["git", "grep", "-l", "cmp453_diet", "--", "*.rs"],
                            capture_output=True, text=True).stdout.split()
JAVA_FILES = subprocess.run(["git", "grep", "-l", "cmp453_diet", "--", "*.java"],
                            capture_output=True, text=True).stdout.split()


def widen_rust_line(ln):
    """Return widened line or None."""
    code = ln.split("//")[0]  # needle context = before trailing comment
    if "cmp450_chunk" in code:
        return None  # already armed on this line
    if "cmp453_diet" not in code:
        return None
    if re.search(r'Ok\("cmp453_diet"\)\s*=>', code):
        return None  # display arm
    if 'Ok("cmp453_diet")' in code:
        return ln.replace('Ok("cmp453_diet")', 'Ok("cmp453_diet") | Ok("cmp450_chunk")')
    if 'f == "cmp453_diet"' in code:
        return ln.replace('f == "cmp453_diet"', 'f == "cmp453_diet" || f == "cmp450_chunk"')
    if 'v == "cmp453_diet"' in code:
        return ln.replace('v == "cmp453_diet"', 'v == "cmp453_diet" || v == "cmp450_chunk"')
    return None


def widen_java_line(ln):
    if 'equals("cmp453_diet")' not in ln or "cmp450_chunk" in ln:
        return None
    return ln.replace('equals("cmp453_diet")',
                      'equals("cmp453_diet") || f.trim().equals("cmp450_chunk")')


for f in RUST_FILES:
    lines = open(f).read().split("\n")
    n = 0
    for i, ln in enumerate(lines):
        w = widen_rust_line(ln)
        if w is not None:
            lines[i] = w
            n += 1
    if n:
        print(f"rust {f}: {n} site(s)")
        changed += n
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
        print(f"java {f}: {n} site(s)")
        changed += n
        if not DRY:
            open(f, "w").write("\n".join(lines))

print(f"total sites changed: {changed}" + (" (DRY)" if DRY else ""))
