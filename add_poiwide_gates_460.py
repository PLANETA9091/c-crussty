#!/usr/bin/env python3
"""add_poiwide_gates_460.py — TASK-460-06 (L03): union-widen cmp456_poi to the
round id cmp456_poi_wide = cmp456_poi FULL era carrier ∪ brain-tick2 lane.

Rule (per line, code part = before first '//'):
  * rust  `Ok("cmp456_poi")`            -> `... | Ok("cmp456_poi_wide")`
  * rust  `Some("cmp456_poi")`          -> `... | Some("cmp456_poi_wide")`
  * rust  `X == "cmp456_poi"`           -> `... || X == "cmp456_poi_wide"`
  * java  `f.trim().equals("cmp456_poi")`     -> `... || f.trim().equals("cmp456_poi_wide")`
  * java  `"cmp456_poi".equals(LEVER_FLAG)`   -> `... || "cmp456_poi_wide".equals(LEVER_FLAG)`
Skipped: display/label arms (rust `=>`, java `? "` / `return "`), comments.
Existing needles verbatim (mirror-drift lesson x451/x452); empty/foreign flag
= vanilla bit-in-byte. cmp456_poi gate lists themselves UNTOUCHED (round-id
hygiene: the new union rides the NEW round id only).

Widening delta (beyond pure alias): brain-tick2 lane
  * src/brainhook.rs tick2_enabled()  += cmp456_poi_wide (mirror of the
    mobs_sense enabled() canon which ALREADY carries cmp456_poi — drift fix)
  * randomtick/src/BrainOps.java TICK2_FLAGS += cmp456_poi_wide (cp mirror)
Old exact-match lanes (cmp401_offthread prepare/prepare_index, cmp405_eindex,
items_oss) stay dormant — baked <clinit> gates, out of minimal-diff scope.

Usage: python3 add_poiwide_gates_460.py [--dry]
"""
import re
import subprocess
import sys

DRY = "--dry" in sys.argv
changed_files = 0
changed_lines = 0

RUST_FILES = subprocess.run(
    ["git", "grep", "-l", '"cmp456_poi"', "--", "src/*.rs"],
    capture_output=True, text=True).stdout.split()
JAVA_FILES = subprocess.run(
    ["git", "grep", "-l", '"cmp456_poi"', "--", "*.java"],
    capture_output=True, text=True).stdout.split()


def widen_rust_line(ln):
    code = ln.split("//")[0]
    if "cmp456_poi_wide" in code or "cmp456_poi" not in code:
        return None
    if re.search(r'Ok\("cmp456_poi"\)\s*=>', code):
        return None  # display arm
    out = ln
    if "Ok(\"cmp456_poi\")" in code:
        out = out.replace('Ok("cmp456_poi")',
                          'Ok("cmp456_poi") | Ok("cmp456_poi_wide")')
        return out
    if "Some(\"cmp456_poi\")" in code:
        out = out.replace('Some("cmp456_poi")',
                          'Some("cmp456_poi") | Some("cmp456_poi_wide")')
        return out
    m = re.search(r'([\w.()]+)\s*==\s*"cmp456_poi"', code)
    if m:
        var = m.group(1)
        return out.replace(f'{var} == "cmp456_poi"',
                           f'{var} == "cmp456_poi" || {var} == "cmp456_poi_wide"')
    return None


def widen_java_line(ln):
    code = ln.split("//")[0]
    if "cmp456_poi_wide" in code or "cmp456_poi" not in code:
        return None
    if '? "' in code or 'return "' in code:
        return None  # label/marker arm
    if 'f.trim().equals("cmp456_poi")' in code:
        return ln.replace('f.trim().equals("cmp456_poi")',
                          'f.trim().equals("cmp456_poi") || f.trim().equals("cmp456_poi_wide")')
    if '"cmp456_poi".equals(LEVER_FLAG)' in code:
        return ln.replace('"cmp456_poi".equals(LEVER_FLAG)',
                          '"cmp456_poi".equals(LEVER_FLAG) || "cmp456_poi_wide".equals(LEVER_FLAG)')
    return None


def process(path, widen_fn):
    global changed_files, changed_lines
    lines = open(path).read().split("\n")
    out, n = [], 0
    for ln in lines:
        new = widen_fn(ln)
        if new is None:
            out.append(ln)
        else:
            out.append(new)
            n += 1
    if n:
        changed_files += 1
        changed_lines += n
        print(f"{path}: {n} needle line(s) widened")
        if not DRY:
            open(path, "w").write("\n".join(out))
    return n


for f in RUST_FILES:
    process(f, widen_rust_line)
for f in JAVA_FILES:
    if "BrainOps.java" in f:
        continue  # tick2 handled explicitly below
    process(f, widen_java_line)

# ---- widening delta: brain-tick2 lane -------------------------------------
TICK2 = 'Ok("cmp451_senseins") // TASK-451-D: senseins composite'
TICK2_NEW = ('Ok("cmp451_senseins") | Ok("cmp456_poi_wide") '
             '// TASK-460-06: poiw widen — tick2 rides the wide round id '
             '(mirror canon: mobs_sense enabled() already carries cmp456_poi)')


def widen_brainhook():
    global changed_files, changed_lines
    path = "src/brainhook.rs"
    txt = open(path).read()
    if "cmp456_poi_wide" in txt:
        return
    # tick2_enabled(): STRICT-OR append on the matches! list
    old = ('Ok("cmp438_sense") | Ok("cmp439_sense_scan") | Ok("cmp451_senseins") '
           '// TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR)')
    new = ('Ok("cmp438_sense") | Ok("cmp439_sense_scan") | Ok("cmp451_senseins") '
           '| Ok("cmp456_poi_wide") '
           '// TASK-451-D: senseins composite (carrier ins4 + sense/brain family, STRICT OR) '
           '// TASK-460-06: poiw widen — tick2 rides the wide round id (mobs_sense mirror canon)')
    if old not in txt:
        print(f"FATAL: brainhook tick2 gate needle not found", file=sys.stderr)
        sys.exit(1)
    changed_files += 1
    changed_lines += 1
    print(f"{path}: tick2_enabled widened")
    if not DRY:
        open(path, "w").write(txt.replace(old, new))


def widen_brainops():
    global changed_files, changed_lines
    path = "randomtick/src/BrainOps.java"
    txt = open(path).read()
    if "cmp456_poi_wide" in txt:
        return
    old = 'static final String TICK2_FLAGS = "cmp438_sense|cmp439_sense_scan|cmp451_senseins|cmp452_mega";'
    new = ('static final String TICK2_FLAGS = "cmp438_sense|cmp439_sense_scan|cmp451_senseins|cmp452_mega'
           '|cmp456_poi_wide"; // TASK-460-06: poiw widen (cp mirror of rust tick2 gate)')
    if old not in txt:
        print("FATAL: BrainOps TICK2_FLAGS needle not found", file=sys.stderr)
        sys.exit(1)
    changed_files += 1
    changed_lines += 1
    print(f"{path}: TICK2_FLAGS widened")
    if not DRY:
        open(path, "w").write(txt.replace(old, new))


def widen_colpush():
    global changed_files, changed_lines
    path = "colpush/net/minecraft/world/entity/ColpushOps.java"
    txt = open(path).read()
    if "cmp456_poi_wide" in txt:
        return
    old_c = 'private static final String FLAG_POI = "cmp456_poi"; // TASK-456-B carrier (STRICT OR)'
    new_c = (old_c + "\n    "
             'private static final String FLAG_POI_WIDE = "cmp456_poi_wide"; // TASK-460-06 poiw widen (STRICT OR)')
    old_g = "f.trim().equals(FLAG_MEGA) || f.trim().equals(FLAG_DIET) || f.trim().equals(FLAG_POI))"
    new_g = ("f.trim().equals(FLAG_MEGA) || f.trim().equals(FLAG_DIET) || f.trim().equals(FLAG_POI)"
             " || f.trim().equals(FLAG_POI_WIDE))")
    if old_c not in txt or old_g not in txt:
        print("FATAL: ColpushOps FLAG_POI needles not found", file=sys.stderr)
        sys.exit(1)
    changed_files += 1
    changed_lines += 2
    print(f"{path}: FLAG_POI_WIDE constant + leverEnabled widened")
    if not DRY:
        open(path, "w").write(txt.replace(old_c, new_c).replace(old_g, new_g))


widen_brainhook()
widen_brainops()
widen_colpush()

print(f"{'DRY-RUN: ' if DRY else ''}{changed_files} files, {changed_lines} lines widened")
