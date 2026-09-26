#!/usr/bin/env python3
"""TASK-463-68a (swarx-6): STRICT-OR branch-local lever cmp463_swar_hilbert
into every gate site carrying the certified swar carrier needle cmp458_swar
(mechanics = add_swar_gates_458.py; mirror-drift урок ×451/×452: rust-гейты +
java flag-lists правятся СИНХРОННО). Label/marker lines (ternary FLAG_LABEL,
`return "..."` mappings) are SKIPPED — branch-local labels are wired by hand.
H07 Hilbert-issue сам гейтится отдельно (entity_query::hilbert_issue_mode).
Existing needles preserved verbatim. Empty/foreign flag = vanilla bit-in-byte.
"""
import re, sys, glob

FLAG = "cmp463_swar_hilbert"
NEEDLE = "cmp458_swar"

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
        if NEEDLE not in line:
            continue
        last = None  # (kind, var, span_end)
        for m in re.finditer(r'\| (Ok|Some)\("' + NEEDLE + r'"\)', line):
            last = (m.group(1), None, m.end())
        for m in re.finditer(r'(\w+) == "' + NEEDLE + r'"', line):
            last = ("eq", m.group(1), m.end())
        if last is None:
            continue  # comment/doc/label — not a gate list
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
            continue  # idempotent
        if NEEDLE not in line:
            continue
        if path.endswith("BrainOps.java"):
            if 'TICK2_FLAGS' in line and NEEDLE in line:
                lines[i] = line.replace(f'|{NEEDLE}"', f'|{NEEDLE}|{FLAG}"')
                changed = True
            continue
        if path.endswith("ColpushOps.java"):
            # FLAG-constant style: swar = FLAG17 → hilbert = FLAG18 (next slot);
            # chain extension at the FLAG17 usage site.
            if re.search(r'\w+\.equals\(FLAG17\)', line):
                lines[i] = re.sub(r'(\w+)\.equals\(FLAG17\)',
                                  rf'\1.equals(FLAG17) || \1.equals(FLAG18)', line)
                changed = True
            elif re.match(r'\s*private static final String FLAG17 = "' + NEEDLE + r'";', line):
                indent = re.match(r'(\s*)', line).group(1)
                lines[i] = (line.rstrip("\n") + "\n"
                            + f'{indent}/** TASK-463-68a: H07 Hilbert-issue swarx-6 '
                              f'(STRICT-OR, branch-local lever). */\n'
                            + f'{indent}private static final String FLAG18 = "{FLAG}";\n')
                changed = True
            continue
        # label/marker lines (ternary arm or `return "..."` mapping) — skip,
        # wired by hand with the branch-local label.
        if 'return "' in line or re.search(r'\? "cmp', line):
            continue
        if NEEDLE not in line:
            continue
        last = None
        for m in re.finditer(r'(\w+(?:\.trim\(\))?)\.equals\("' + NEEDLE + r'"\)', line):
            last = (m.group(1), m.end())
        for m in re.finditer(r'"' + NEEDLE + r'"\.equals\((\w+(?:\.trim\(\))?)\)', line):
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

# verify: every file that carried the swar needle now carries the flag
missing = []
for path in rust_files + java_files:
    txt = open(path).read()
    if NEEDLE in txt and FLAG not in txt:
        missing.append(path)
if missing:
    for p in missing:
        print(f"MISSING: {p}", file=sys.stderr)
    sys.exit(1)
print(f"verify: all swar-needle gates now accept {FLAG} (STRICT-OR union)")
