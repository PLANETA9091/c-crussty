#!/usr/bin/env python3
"""ROUND-468-S18: STRICT-OR compo carrier cmp468_s18fluid into every gate
site carrying the merge carrier cmp466_c98ai (MERGE #10 c98-compo dbbffef9,
master c1196321) — the S18 compo = certified carrier ⊕ inside_fluid
(fluid-empty fastpath on the checkInsideBlocks visit lambda, javap bit-exact,
R468-S18 RESEARCH; new entity_compose stage 1d, InsideFluidOps bridge).

Union semantics: cmp468_s18fluid arms EXACTLY the union of the planes armed
by cmp466_c98ai (gate-term parity with the certified merge carrier) PLUS the
new inside_fluid stage — per-plane behavior under cmp468_s18fluid is
identical to its behavior under cmp466_c98ai by construction. The AI-depth
leg rides lever_arg=8 (dispatch input, 0 code delta).

Pattern = add_c98ai_gates_466.py (mirror-drift lesson x451/452: rust gates +
java flag-lists are edited SYNCHRONOUSLY; blobs rebuilt after). Existing
needles preserved verbatim. Idempotent: lines already carrying FLAG skipped.
"""
import re, sys, glob

FLAG = "cmp468_s18fluid"
ANCHORS = ("cmp466_c98ai",)

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

# verify: every file that carried an anchor now carries the flag (gate files
# only; src/lib.rs mentions carriers in COMMENTS exclusively — no gate)
missing = []
for path in rust_files:
    if path.endswith("lib.rs"):
        continue
    txt = open(path).read()
    if ANCHORS[0] in txt and "==" in txt and FLAG not in txt:
        # anchor present in a gate line pattern but flag missing
        if re.search(r'== "' + ANCHORS[0] + r'"|Ok\("' + ANCHORS[0] + r'"\)|Some\("' + ANCHORS[0] + r'"\)', txt):
            missing.append(path)
for path in java_files:
    txt = open(path).read()
    if f'"{ANCHORS[0]}"' in txt and FLAG not in txt:
        missing.append(path)
if missing:
    for m in missing:
        print(f"MISSING FLAG: {m}", file=sys.stderr)
    sys.exit(1)
print("verify: all anchor gate files carry the flag")
