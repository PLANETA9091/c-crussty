#!/usr/bin/env python3
"""TASK-466-C08: STRICT-OR cmp466_poiun union carrier into every gate site
carrying the era-union anchors cmp456_poi (poi+chunkmono full-era carrier,
master 88ac16a2) / cmp457_paldelta / cmp457_eqsnap2.

Union semantics: cmp466_poiun arms EXACTLY the union of the planes armed by
the three carriers — per-plane behavior under cmp466_poiun is identical to
its behavior under the carrier whose gate list it joined (consistency with
the certified poi-p22-1 leg is preserved by construction).

Pattern = add_eqsnap2_gates_457.py (mirror-drift lesson x451/452: rust gates
+ java flag-lists are edited SYNCHRONOUSLY). Existing needles preserved
verbatim. Idempotent: lines already carrying FLAG are skipped.

SPECIAL CASE ColpushOps.java: leverEnabled chain is constant-style
(FLAG..FLAG17) and carries cmp457_paldelta via FLAG15 — the chain line has no
anchor literal, so the union equals is appended to the chain explicitly.
ChunkParseOps.java / ChunkSendOps.java / ChunkPacketEncodeOps.java /
BrainOps.java are raw-cp MARKER constants only (no behavioral gate list) —
intentionally untouched; their rust gates (chunk_parse.rs / chunk_send*.rs /
brainhook.rs) receive the flag through the generic rust rule.
"""
import re, sys, glob

FLAG = "cmp466_poiun"
ANCHORS = ("cmp456_poi", "cmp457_paldelta", "cmp457_eqsnap2")

rust_files = sorted(glob.glob("src/*.rs"))
java_files = [
    "poi/net/minecraft/world/entity/ai/village/poi/PoiOps.java",
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
    "sense/net/minecraft/world/entity/SenseOps.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
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
        if not any(a in line for a in ANCHORS):
            continue
        # find LAST anchor occurrence and its form
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
        if path.endswith("ColpushOps.java"):
            # FLAG-constant style chain (FLAG..FLAG17; paldelta rides FLAG15):
            # append the union equals to the leverEnabled chain line.
            if 'f.trim().equals(FLAG17)' in line:
                lines[i] = line.replace('f.trim().equals(FLAG17))',
                                        f'f.trim().equals(FLAG17) || f.trim().equals("{FLAG}"))')
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

# verify: every file that carried an anchor now carries the flag (gate files
# only; src/lib.rs mentions the anchors in COMMENTS exclusively — no gate)
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
print("verify: all anchor files carry the union flag")
