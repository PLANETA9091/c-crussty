#!/usr/bin/env python3
"""TASK-468-S13 (M1-микс): STRICT-OR compo carrier cmp468_c98ai_m1 into every
gate site carrying the merged carrier cmp466_c98ai (МЕРЖ №10 c98-compo
dbbffef9) — EXCEPT the P31 owner gate src/inside_batch.rs, which is the
surgical CUT this leg is about.

M1-микс = cmp466_c98ai MINUS P31 (inside_batch). Л180d: the inside_batch
blob is a vanilla pass-through whose activate() SUPERSEDES inside_cache on
the isAffectedByBlocks site — arming P31 disables the inside_cache=1
dispatch input on every P31 leg (anchor-пара гандикап). Cutting P31 from the
compo restores inside_cache (env CRUSSTY_INSIDE_CACHE=1, independent of the
lever flag) while keeping every other plane of the carrier: chunk6-sched ⊕
P32-snapreg ⊕ inside_snap ⊕ master cert stack ⊕ AI-depth N=8 (lever_arg=8,
0 code delta). Forecast leg +28.7-31.4 (Л180o), anchor threshold ≤+10.7
global band.

Union semantics: cmp468_c98ai_m1 arms EXACTLY the union of the planes armed
by cmp466_c98ai, one gate site short — the inside_batch owner gate keeps
accepting only cmp456_chunkmono_p31snap / cmp466_c98ai, so the P31 bridge
stays DORMANT under the M1 flag (fail-closed cut, zero inside_batch logs).
Pattern = add_c98ai_gates_466.py (mirror-drift lesson x451/452: rust gates +
java flag-lists are edited SYNCHRONOUSLY). Idempotent: lines already
carrying FLAG are skipped. src/lib.rs mentions carriers in COMMENTS only.
"""
import re, sys, glob

FLAG = "cmp468_c98ai_m1"
ANCHORS = ("cmp466_c98ai",)
# THE CUT: P31 owner gate must NOT accept the M1 flag (that is the leg).
SKIP_FILES = ("src/inside_batch.rs",)

rust_files = sorted(glob.glob("src/*.rs"))
java_files = [
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
    "sense/net/minecraft/world/entity/SenseOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
]

fails = []
rust_touched = java_touched = 0

# ---------------- RUST ----------------
for path in rust_files:
    if path.endswith("lib.rs"):
        continue  # mentions carriers in COMMENTS exclusively — no gate
    if path in SKIP_FILES:
        continue  # THE CUT: P31 owner gate stays M1-blind
    with open(path) as f:
        lines = f.readlines()
    changed = False
    for i, line in enumerate(lines):
        if FLAG in line:
            continue  # idempotent
        if not any(a in line for a in ANCHORS):
            continue
        last = None  # (kind, var, span_end)
        for a in ANCHORS:
            for m in re.finditer(r'\| (Ok|Some)\("' + a + r'"\)', line):
                last = (m.group(1), None, m.end())
            for m in re.finditer(r'(\w+(?:\.trim\(\))?) == "' + a + r'"', line):
                last = ("eq", m.group(1), m.end())
        if last is None:
            continue  # comment/doc/test-call — not a gate list
        kind, var, end = last
        if kind == "Ok":
            ins = f' | Ok("{FLAG}")'
        elif kind == "Some":
            ins = f' | Some("{FLAG}")'
        else:
            ins = f' || {var} == "{FLAG}"'
        lines[i] = line[:end] + ins + line[end:]
        changed = True
    if changed:
        with open(path, "w") as f:
            f.writelines(lines)
        rust_touched += 1
        print(f"rust ok: {path}")

# ---------------- JAVA ----------------
for path in java_files:
    with open(path) as f:
        lines = f.readlines()
    changed = False
    for i, line in enumerate(lines):
        if FLAG in line:
            continue
        if not any(f'"{a}"' in line for a in ANCHORS):
            continue
        last = None
        for a in ANCHORS:
            for m in re.finditer(r'(\w+(?:\.trim\(\))?)\.equals\("' + a + r'"\)', line):
                last = (m.group(1), m.end())
            for m in re.finditer(r'"' + a + r'"\.equals\((\w+(?:\.trim\(\))?)\)', line):
                last = (m.group(1), m.end())
        if last is None:
            continue
        var, end = last
        ins = f' || {var}.equals("{FLAG}")'
        lines[i] = line[:end] + ins + line[end:]
        changed = True
    if changed:
        with open(path, "w") as f:
            f.writelines(lines)
        java_touched += 1
        print(f"java ok: {path}")

print(f"\n== compo {FLAG} wired: {rust_touched} rust files, {java_touched} java files ==")

# verify 1 (THE CUT): inside_batch.rs must NOT carry the M1 flag
if FLAG in open("src/inside_batch.rs").read():
    print("CUT VIOLATION: src/inside_batch.rs carries the M1 flag")
    sys.exit(1)
print("verify: P31 cut holds — src/inside_batch.rs does NOT accept the M1 flag")

# verify 2: every file that carried the anchor (gate files only) now carries
# the flag, except the skipped cut + comments-only lib.rs
missing = []
for path in rust_files:
    if path.endswith("lib.rs") or path in SKIP_FILES:
        continue
    txt = open(path).read()
    if any(a in txt for a in ANCHORS) and FLAG not in txt:
        missing.append(("rust", path))
for path in java_files:
    txt = open(path).read()
    if any(a in txt for a in ANCHORS) and FLAG not in txt:
        missing.append(("java", path))
if missing:
    print("MISSING (anchor present, flag absent):")
    for kind, path in missing:
        print(f"  {kind}: {path}")
    sys.exit(1)
print("verify: all c98ai gate files carry the M1 flag")

# verify 3: single-token canon — no line carries the flag twice
dup = 0
for path in rust_files + java_files:
    for n, ln in enumerate(open(path), 1):
        c = ln.count(FLAG)
        if c > 1:
            print(f"DUP-TOKEN: {path}:{n} x{c}")
            dup += 1
sys.exit(1 if dup else 0)
