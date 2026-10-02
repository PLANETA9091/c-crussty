#!/usr/bin/env python3
"""TASK-458-I step-1: STRICT-OR cmp458_swar into all gate sites carrying the
master cert needle cmp457_paldelta (mechanics = add_paldelta_gates_457.py;
lineage 47ea8b20/fe7e7120/c25782b4/097def9d; mirror-drift урок ×451/×452:
rust-гейты + java flag-lists правятся СИНХРОННО).
NCDFE-канон ×456/×413: entity_query::flag_enabled (the EntityGoalQueryOps
EARLY-define gate) получает cmp458_swar через тот же Some-arm pass —
arm AFTER define, define-race структурно исключён.
Existing needles preserved verbatim. Empty/foreign flag = vanilla bit-in-byte.
"""
import re, sys, glob

FLAG = "cmp458_swar"
ANCHORS = ("cmp457_paldelta", "cmp436_ins4", "cmp451_senseins")

rust_files = sorted(glob.glob("src/*.rs"))
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
            # FLAG-constant style (next free slot FLAG16; chain extension at the
            # FLAG15 site — same mechanical shape as add_paldelta_gates_457.py)
            if 'f.trim().equals(FLAG15)' in line:
                lines[i] = line.replace('f.trim().equals(FLAG15)',
                                        f'f.trim().equals(FLAG15) || f.trim().equals(FLAG16)')
                changed = True
            elif re.match(r'\s*private static final String FLAG15 = "cmp457_paldelta";', line):
                indent = re.match(r'(\s*)', line).group(1)
                lines[i] = (line.rstrip("\n") + "\n"
                            + f'{indent}/** TASK-458-I: SWAR-CSR broadphase carrier '
                              f'(STRICT-OR). */\n'
                            + f'{indent}private static final String FLAG16 = "{FLAG}";\n')
                changed = True
            continue
        if path.endswith("BrainOps.java"):
            if 'TICK2_FLAGS' in line and 'cmp457_paldelta' in line and FLAG not in line:
                lines[i] = line.replace(
                    '"cmp438_sense|cmp439_sense_scan|cmp451_senseins|cmp452_mega|cmp457_paldelta"',
                    f'"cmp438_sense|cmp439_sense_scan|cmp451_senseins|cmp452_mega|cmp457_paldelta|{FLAG}"')
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

print(f"\n== union {FLAG} wired: {rust_touched} rust files, {java_touched} java files ==")

# verify: every file that carried a needle now carries the flag (gate files only)
missing = []
for path in rust_files:
    txt = open(path).read()
    if "cmp457_paldelta" in txt and FLAG not in txt:
        missing.append(("rust", path))
for path in java_files:
    txt = open(path).read()
    if "cmp457_paldelta" in txt and FLAG not in txt:
        missing.append(("java", path))
if missing:
    for kind, p in missing:
        print(f"MISSING {kind}: {p}", file=sys.stderr)
    sys.exit(1)
print(f"verify: all needle-carrying gates now accept {FLAG} (STRICT-OR union)")
