#!/usr/bin/env python3
"""TASK-434-C step-1: STRICT-OR cmp434_chunkpl into all gate sites
(pattern = add_mobfeed_gates.py / 5865667 retag) — chunk-pipeline R5 carrier
flag: full composite union (cmp430_inside set) + chunk parse planes."""
import re, sys

# (path, [line numbers with a cmp430_inside GATE occurrence]) — verified by grep.
RUST_SITES = [
    ("src/chunk_parse.rs",          [131]),            # already retagged manually — idempotent
    ("src/collide_batch.rs",        [87, 238]),
    ("src/colpush.rs",              [121]),
    ("src/entity_query.rs",         [133, 175, 969]),
    ("src/goal_selector.rs",        [82]),
    ("src/inside_bitmask.rs",       [45]),
    ("src/inside_snap.rs",          [106]),
    ("src/items_index.rs",          [298]),
    ("src/items_manager.rs",        [78, 112]),
    ("src/mobs_ai.rs",              [80]),
    ("src/mobs_grid.rs",            [92]),
    ("src/mobs_manager.rs",         [94, 452, 469]),
    ("src/mobs_soa.rs",             [314, 355]),
    ("src/mobs_sscan.rs",           [90]),
    ("src/nav_plane.rs",            [55]),
    ("src/noise_fill.rs",           [179]),            # already retagged manually — idempotent
    ("src/queryplane.rs",           [76]),             # already retagged manually — idempotent
    ("src/stagger.rs",              [83]),
    ("src/tickplane.rs",            [59]),
]

JAVA_SITES = [
    ("mobai/net/minecraft/world/entity/MobAiOps.java",                 [74]),
    ("sscan/net/minecraft/world/entity/MobScanOps.java",               [79]),
    ("entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java", [119]),
    ("goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",        [76]),
    ("queryplane/net/minecraft/world/entity/QueryPlaneOps.java",       [89]),
    ("mobpush/net/minecraft/world/entity/MobPushOps.java",             [160, 202]),
    ("entityinside/net/minecraft/world/entity/ItemEntityManager.java", [109, 159, 193]),
]

MARKER_MAPS = [
    # (path, anchor line snippet) — flag->marker-id maps; add cmp434_chunkpl arm
    ("src/queryplane.rs", 'Ok("cmp430_inside") => "cmp430_inside",'),
    ("mobpush/net/minecraft/world/entity/MobPushOps.java",
     'if (f != null && f.trim().equals("cmp430_inside")) return "cmp430_inside"; // TASK-430-B'),
    ("entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
     ': t.equals("cmp430_inside") ? "cmp430_inside"'),
]

def do_rust(path, lines):
    with open(path) as f:
        src = f.readlines()
    for ln in lines:
        line = src[ln - 1]
        if "cmp434_chunkpl" in line:
            print(f"rust skip (already): {path}:{ln}")
            continue
        if "cmp430_inside" not in line:
            print(f"FAIL {path}:{ln}: no cmp430_inside in: {line!r}", file=sys.stderr)
            sys.exit(1)
        var = None
        for pat in [r'(\w+(?:\.trim\(\))?) == "cmp430_inside"',
                    r'Ok\("cmp430_inside"\)']:
            m = re.search(pat, line)
            if m:
                var = m.group(0)
                break
        if var is None:
            print(f"FAIL {path}:{ln}: no var pattern in: {line!r}", file=sys.stderr)
            sys.exit(1)
        if var.startswith("Ok("):
            nl = line.replace('Ok("cmp430_inside")',
                              'Ok("cmp430_inside") | Ok("cmp434_chunkpl")')
        else:
            nl = line.replace(' == "cmp430_inside"',
                              ' == "cmp430_inside" || ' + var.split(" ==")[0] + ' == "cmp434_chunkpl"')
        src[ln - 1] = nl
    with open(path, "w") as f:
        f.writelines(src)
    print(f"rust ok: {path} lines {lines}")

def do_java(path, lines):
    with open(path) as f:
        src = f.readlines()
    for ln in lines:
        line = src[ln - 1]
        if "cmp434_chunkpl" in line:
            print(f"java skip (already): {path}:{ln}")
            continue
        if "cmp430_inside" not in line:
            print(f"FAIL {path}:{ln}: no cmp430_inside in: {line!r}", file=sys.stderr)
            sys.exit(1)
        stripped = line.rstrip("\n")
        m = re.search(r'\)\s*;\s*$', stripped)
        tail = ""
        if m:
            tail = stripped[m.start():]
            stripped = stripped[:m.start()]
        indent = re.match(r'\s*', line).group(0)
        # insert before the tail: || f.trim().equals("cmp434_chunkpl")  /  || "cmp434_chunkpl".equals(LEVER_FLAG)
        if '"cmp430_inside".equals(LEVER_FLAG)' in stripped:
            add = ' || "cmp434_chunkpl".equals(LEVER_FLAG)'
        else:
            add = ' || f.trim().equals("cmp434_chunkpl")'
        nl = stripped + add + tail + "\n"
        src[ln - 1] = nl
    with open(path, "w") as f:
        f.writelines(src)
    print(f"java ok: {path} lines {lines}")

def do_marker(path, snippet):
    with open(path) as f:
        src = f.read()
    if "cmp434_chunkpl" in src and snippet not in src:
        print(f"marker skip (already): {path}")
        return
    indent = "        " if path.endswith(".rs") else "        "
    if path.endswith(".rs"):
        add = '\n        // TASK-434-C: chunk-pipeline R5 carrier — свой id в ARM-маркерах.\n        Ok("cmp434_chunkpl") => "cmp434_chunkpl",'
    else:
        if 'return "cmp430_inside";' in snippet:
            add = '\n        if (f != null && f.trim().equals("cmp434_chunkpl")) return "cmp434_chunkpl"; // TASK-434-C'
        else:
            add = '\n                : t.equals("cmp434_chunkpl") ? "cmp434_chunkpl"'
    src = src.replace(snippet, snippet + add, 1)
    with open(path, "w") as f:
        f.write(src)
    print(f"marker ok: {path}")

for path, lines in RUST_SITES:
    do_rust(path, lines)
for path, lines in JAVA_SITES:
    do_java(path, lines)
for path, snippet in MARKER_MAPS:
    do_marker(path, snippet)

# ColpushOps: FLAG3-style constant + gate reference
p = "colpush/net/minecraft/world/entity/ColpushOps.java"
with open(p) as f:
    src = f.read()
if "cmp434_chunkpl" not in src:
    src = src.replace(
        '    private static final String FLAG3 = "cmp430_inside";',
        '    private static final String FLAG3 = "cmp430_inside";\n'
        '    /** TASK-434-C: chunk-pipeline R5 carrier (STRICT-OR; raw-cp marker\n'
        '     * for the check_blobs_sync gate, x93). */\n'
        '    private static final String FLAG4 = "cmp434_chunkpl";', 1)
    src = src.replace(
        'return f != null && (f.trim().equals(FLAG) || f.trim().equals(FLAG2) || f.trim().equals(FLAG3));',
        'return f != null && (f.trim().equals(FLAG) || f.trim().equals(FLAG2) || f.trim().equals(FLAG3) || f.trim().equals(FLAG4));', 1)
    with open(p, "w") as f:
        f.write(src)
    print("colpush ok: FLAG4 added")
else:
    print("colpush skip (already)")

print("ALL GATE SITES UPDATED")
