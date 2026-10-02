#!/usr/bin/env python3
"""TASK-426-A step-1: STRICT-OR cmp424_mobfeed into all gate sites
(pattern = cmp423_brain3 sites, 16 rust + 7 java) — SoA-feed carrier flag."""
import re, sys

RUST_SITES = [
    ("src/chunk_parse.rs", [126]),
    ("src/collide_batch.rs", [87, 238]),
    ("src/entity_query.rs", [133, 175, 969]),
    ("src/goal_selector.rs", [82]),
    ("src/items_index.rs", [298]),
    ("src/items_manager.rs", [78, 112]),
    ("src/mobs_ai.rs", [80]),
    ("src/mobs_grid.rs", [76]),
    ("src/mobs_manager.rs", [94, 452, 469]),
    ("src/mobs_soa.rs", [245, 286]),
    ("src/mobs_sscan.rs", [90]),
    ("src/nav_plane.rs", [55]),
    ("src/noise_fill.rs", [179]),
    ("src/queryplane.rs", [76]),
    ("src/stagger.rs", [83]),
    ("src/tickplane.rs", [59]),
]

JAVA_SITES = [
    ("mobai/net/minecraft/world/entity/MobAiOps.java", [69]),
    ("sscan/net/minecraft/world/entity/MobScanOps.java", [77]),
    ("entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java", [117, 138]),
    ("goalops/net/minecraft/world/entity/ai/goal/GoalOps.java", [74]),
    ("queryplane/net/minecraft/world/entity/QueryPlaneOps.java", [87]),
    ("mobpush/net/minecraft/world/entity/MobPushOps.java", [158, 198]),
    ("entityinside/net/minecraft/world/entity/ItemEntityManager.java", [109, 159]),
]

for path, lines in RUST_SITES:
    with open(path) as f:
        src = f.readlines()
    for ln in lines:
        line = src[ln - 1]
        if "cmp424_mobfeed" in line:
            continue
        if 'Ok("cmp423_brain3")' in line:
            nl = line.replace('| Ok("cmp423_brain3")',
                              '| Ok("cmp423_brain3") | Ok("cmp424_mobfeed")')
        else:
            m = re.search(r'(\w+(?:\.trim\(\))?) == "cmp423_brain3"', line)
            if not m:
                print(f"FAIL {path}:{ln}: no var pattern in: {line!r}", file=sys.stderr)
                sys.exit(1)
            var = m.group(1)
            nl = line.replace(' == "cmp423_brain3"',
                              f' == "cmp423_brain3" || {var} == "cmp424_mobfeed"')
        src[ln - 1] = nl
    with open(path, "w") as f:
        f.writelines(src)
    print(f"rust ok: {path} lines {lines}")

for path, lines in JAVA_SITES:
    with open(path) as f:
        src = f.readlines()
    for ln in lines:
        line = src[ln - 1]
        if "cmp424_mobfeed" in line:
            continue
        if '"cmp423_brain3".equals(LEVER_FLAG)' in line:
            nl = line.replace('"cmp423_brain3".equals(LEVER_FLAG)',
                              '"cmp423_brain3".equals(LEVER_FLAG) || "cmp424_mobfeed".equals(LEVER_FLAG)')
        else:
            # strip trailing ');' or ')' then re-append
            stripped = line.rstrip("\n")
            m = re.search(r'\)\s*;\s*$', stripped)
            tail = ""
            if m:
                tail = stripped[m.start():]
                stripped = stripped[:m.start()]
            if not stripped.rstrip().endswith('"cmp423_brain3")'):
                if stripped.rstrip().endswith('"cmp423_brain3"'):
                    pass  # bare string before tail on next line
                else:
                    print(f"FAIL {path}:{ln}: unexpected tail in: {line!r}", file=sys.stderr)
                    sys.exit(1)
            indent = re.match(r'\s*', line).group(0)
            nl = stripped.rstrip() + f'\n{indent}// TASK-426-A: SoA-feed carrier (STRICT OR).\n{indent}|| f.trim().equals("cmp424_mobfeed")' + tail + "\n"
        src[ln - 1] = nl
    with open(path, "w") as f:
        f.writelines(src)
    print(f"java ok: {path} lines {lines}")

print("ALL GATE SITES UPDATED")
