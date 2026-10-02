#!/usr/bin/env python3
"""TASK-466-C98: STRICT-OR compo carrier cmp466_c98ai into every gate site
carrying the climb carrier anchor cmp456_chunkmono_p31snap (climb5-p32-1 leg
+22.99 v5 @8784563, 2/3 anchors: chunk6-sched ⊕ P31-inside-batch ⊕ P32-snapreg
planes) — the C98 compo = climb carrier ⊕ AI-depth N=8 (lever_arg=8,
MobAiOps.windowN + GoalStaggerOps/PushStaggerOps readN; leg source C15, Л168).

Union semantics: cmp466_c98ai arms EXACTLY the union of the planes armed by
cmp456_chunkmono_p31snap (gate-term parity with the certified climb carrier)
— per-plane behavior under cmp466_c98ai is identical to its behavior under
the anchor (consistency with the climb5-p32-1 leg preserved by construction).
The AI-depth leg rides lever_arg=8 (dispatch input, 0 code delta; the arg
consumers already read CRUSSTY_LEVER_ARG — Л167 javap canon).

Pattern = add_poiun_gates_466.py (mirror-drift lesson x451/452: rust gates +
java flag-lists are edited SYNCHRONOUSLY). Existing needles preserved
verbatim. Idempotent: lines already carrying FLAG are skipped.
"""
import re, sys, glob

FLAG = "cmp466_c98ai"
ANCHORS = ("cmp456_chunkmono_p31snap",)

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
print("verify: all anchor files carry the compo flag")

# verify: single-token canon — no line carries the flag twice
dup = 0
for path in rust_files + java_files:
    for n, ln in enumerate(open(path), 1):
        c = ln.count(FLAG)
        if c > 1:
            print(f"DUP-TOKEN: {path}:{n} x{c}")
            dup += 1
sys.exit(1 if dup else 0)
