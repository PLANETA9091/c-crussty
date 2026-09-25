#!/usr/bin/env python3
"""TASK-455-A step-2: STRICT-OR cmp455_spawn into all gate sites carrying
master needles cmp436_ins4/cmp451_senseins (pattern = ×454-C union fix
47ea8b20, 20 files/42 sites; mirror-drift урок ×451/×452: rust-гейты + java
flag-lists правятся СИНХРОННО). Existing needles preserved verbatim."""
import re, sys, glob

FLAG = "cmp455_spawn"
ANCHORS = ("cmp436_ins4", "cmp451_senseins")

rust_files = sorted(
    glob.glob("src/*.rs")
)
java_files = [
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "sense/net/minecraft/world/entity/SenseOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
    "randomtick/src/BrainOps.java",
]

fails = []
rust_touched = java_touched = 0

# ---------------- RUST ----------------
for path in rust_files:
    with open(path) as f:
        lines = f.readlines()
    changed = False
    for i, line in enumerate(lines):
        if FLAG in line:
            continue  # idempotent
        if not any(f'"{a}"' in line for a in ANCHORS):
            continue
        # find LAST anchor occurrence and its form
        last = None  # (kind, var, span_end)
        for a in ANCHORS:
            for m in re.finditer(r'\| (Ok|Some)\("' + a + r'"\)', line):
                last = (m.group(1), None, m.end())
            for m in re.finditer(r'(\w+(?:\.trim\(\))?) == "' + a + r'"', line):
                last = ("eq", m.group(1), m.end())
        if last is None:
            continue  # comment/doc or label-const — not a gate list
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
        if path.endswith("ColpushOps.java"):
            # FLAG6-constant style: extend leverEnabled chain + declare FLAG7
            if 'f.trim().equals(FLAG6)' in line:
                lines[i] = line.replace('f.trim().equals(FLAG6)',
                                        f'f.trim().equals(FLAG6) || f.trim().equals(FLAG7)')
                changed = True
            elif re.match(r'\s*private static final String FLAG6 = "cmp451_senseins";', line):
                indent = re.match(r'(\s*)', line).group(1)
                lines[i] = (line.rstrip("\n") + "\n"
                            + f'{indent}/** TASK-455-A: R4 despawn/spawn/activation scans carrier '
                              f'(STRICT-OR). */\n'
                            + f'{indent}private static final String FLAG7 = "{FLAG}";\n')
                changed = True
            continue
        if path.endswith("BrainOps.java"):
            if 'TICK2_FLAGS' in line and '"cmp451_senseins"' in line:
                lines[i] = line.replace('"cmp438_sense|cmp439_sense_scan|cmp451_senseins"',
                                        f'"cmp438_sense|cmp439_sense_scan|cmp451_senseins|{FLAG}"')
                changed = True
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

print(f"\n== union cmp455_spawn wired: {rust_touched} rust files, {java_touched} java files ==")

# verify: every file that carried a needle now carries the flag (gate files only)
missing = []
for path in rust_files:
    txt = open(path).read()
    if ("cmp436_ins4" in txt or "cmp451_senseins" in txt) and FLAG not in txt:
        missing.append(("rust", path))
for path in java_files:
    txt = open(path).read()
    if FLAG not in txt:
        missing.append(("java", path))
if missing:
    for kind, p in missing:
        print(f"MISSING {kind}: {p}", file=sys.stderr)
    sys.exit(1)
print("verify: all needle-carrying gates now accept cmp455_spawn (STRICT-OR union)")
