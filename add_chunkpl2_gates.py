#!/usr/bin/env python3
"""TASK-435-C step-2: STRICT-OR cmp435_chunk3 into all gate sites
(pattern = add_chunkpl_gates.py / 8cd2028 retag) — R6 carrier flag ON TOP of
cmp434_chunkpl: same chunk-parse planes (block_states + biomes caches) +
full composite union; round-id hygiene for ROUND-435 certification legs."""
import re, sys

RUST_GLOB = [
    "src/collide_batch.rs",
    "src/colpush.rs",
    "src/entity_query.rs",
    "src/goal_selector.rs",
    "src/inside_bitmask.rs",
    "src/inside_snap.rs",
    "src/items_index.rs",
    "src/items_manager.rs",
    "src/mobs_ai.rs",
    "src/mobs_grid.rs",
    "src/mobs_manager.rs",
    "src/mobs_soa.rs",
    "src/mobs_sscan.rs",
    "src/nav_plane.rs",
    "src/noise_fill.rs",
    "src/stagger.rs",
    "src/tickplane.rs",
]

def do_rust(path):
    with open(path) as f:
        src = f.readlines()
    changed = 0
    for i, line in enumerate(src):
        if "cmp434_chunkpl" not in line or "cmp435_chunk3" in line:
            continue
        t = line.rstrip("\n")
        if 'Ok("cmp434_chunkpl") =>' in t:
            indent = re.match(r"\s*", t).group(0)
            marker = indent + 'Ok("cmp435_chunk3") => "cmp435_chunk3", // TASK-435-C: R6 carrier marker id'
            src.insert(i + 1, marker + "\n")
            changed += 1
            continue
        m = re.search(r"(\w+(?:\.trim\(\))?) == \"cmp434_chunkpl\"", t)
        if m:
            nl = t.replace(m.group(0), m.group(0) + f" || {m.group(1)} == \"cmp435_chunk3\"", 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        if 'Ok("cmp434_chunkpl")' in t:
            nl = t.replace('Ok("cmp434_chunkpl")', 'Ok("cmp434_chunkpl") | Ok("cmp435_chunk3")', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        # non-gate mention (doc comment) — leave as-is
        print(f"rust doc-mention left: {path}:{i+1}: {t.strip()[:90]}")
    with open(path, "w") as f:
        f.writelines(src)
    print(f"rust ok ({changed}): {path}")
    return changed

def do_java(path):
    with open(path) as f:
        src = f.readlines()
    changed = 0
    for i, line in enumerate(src):
        if "cmp434_chunkpl" not in line or "cmp435_chunk3" in line:
            continue
        t = line.rstrip("\n")
        # gate arm: || f.trim().equals("cmp434_chunkpl"));
        if 'f.trim().equals("cmp434_chunkpl"))' in t:
            nl = t.replace('f.trim().equals("cmp434_chunkpl"))',
                           'f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3"))', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        # marker return: if (...) return "cmp434_chunkpl";
        m = re.search(r'if \((f != null && f\.trim\(\)\.equals\("cmp434_chunkpl"\))\) return "cmp434_chunkpl";', t)
        if m:
            indent = re.match(r"\s*", t).group(0)
            marker = (indent + 'if (f != null && f.trim().equals("cmp435_chunk3")) return "cmp435_chunk3";'
                      ' // TASK-435-C')
            src.insert(i, marker + "\n")
            changed += 1
            continue
        # ternary marker: : t.equals("cmp434_chunkpl") ? "cmp434_chunkpl"
        if ': t.equals("cmp434_chunkpl") ? "cmp434_chunkpl"' in t:
            nl = t.replace(': t.equals("cmp434_chunkpl") ? "cmp434_chunkpl"',
                           ': t.equals("cmp434_chunkpl") ? "cmp434_chunkpl"'
                           ' : t.equals("cmp435_chunk3") ? "cmp435_chunk3"', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        print(f"java doc-mention left: {path}:{i+1}: {t.strip()[:90]}")
    with open(path, "w") as f:
        f.writelines(src)
    print(f"java ok ({changed}): {path}")
    return changed

total = 0
for p in RUST_GLOB:
    total += do_rust(p)
for p in ["mobai/net/minecraft/world/entity/MobAiOps.java",
          "sscan/net/minecraft/world/entity/MobScanOps.java",
          "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
          "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
          "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
          "mobpush/net/minecraft/world/entity/MobPushOps.java",
          "entityinside/net/minecraft/world/entity/ItemEntityManager.java"]:
    total += do_java(p)

# ColpushOps: FLAG5 constant + gate reference
p = "colpush/net/minecraft/world/entity/ColpushOps.java"
with open(p) as f:
    src = f.read()
if "cmp435_chunk3" not in src:
    src = src.replace(
        '    private static final String FLAG4 = "cmp434_chunkpl";',
        '    private static final String FLAG4 = "cmp434_chunkpl";\n'
        '    /** TASK-435-C: R6 carrier (STRICT-OR; raw-cp marker for the\n'
        '     * check_blobs_sync gate). */\n'
        '    private static final String FLAG5 = "cmp435_chunk3";', 1)
    src = src.replace(
        'return f != null && (f.trim().equals(FLAG) || f.trim().equals(FLAG2) || f.trim().equals(FLAG3) || f.trim().equals(FLAG4));',
        'return f != null && (f.trim().equals(FLAG) || f.trim().equals(FLAG2) || f.trim().equals(FLAG3) || f.trim().equals(FLAG4) || f.trim().equals(FLAG5));', 1)
    with open(p, "w") as f:
        f.write(src)
    total += 2
    print("colpush ok: FLAG5 added")
else:
    print("colpush skip (already)")

# ChunkParseOps: CARRIER_UNION_435 constant + keep-alive references
p = "chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java"
with open(p) as f:
    src = f.read()
if "cmp435_chunk3" not in src:
    src = src.replace(
        '    static final String CARRIER_UNION_423 = "cmp434_chunkpl";',
        '    static final String CARRIER_UNION_423 = "cmp434_chunkpl";\n\n'
        '    /**\n'
        '     * TASK-435-C chunk-pipeline R6 carrier (law 7/8): STRICT-OR successor\n'
        '     * id ON TOP of cmp434_chunkpl (same planes: block_states deep cache ⊕\n'
        '     * biomes-parse cache ⊕ full composite union; no new lever — round-id\n'
        '     * hygiene for ROUND-435 certification). Kept in the constant pool for\n'
        '     * the raw-byte blob-sync gate (check_blobs_sync.sh) — x93 lesson.\n'
        '     */\n'
        '    static final String CARRIER_UNION_435 = "cmp435_chunk3";', 1)
    src = src.replace(
        '+ CARRIER_UNION_423 + ")");',
        '+ CARRIER_UNION_423 + "/" + CARRIER_UNION_435 + ")");', 1)
    src = src.replace(
        '+ " union=" + CARRIER_UNION_423;',
        '+ " union=" + CARRIER_UNION_423 + "/" + CARRIER_UNION_435;', 1)
    with open(p, "w") as f:
        f.write(src)
    total += 3
    print("chunkparse ok: CARRIER_UNION_435 added")
else:
    print("chunkparse skip (already)")

# check_blobs_sync marker list
p = "scripts/check_blobs_sync.sh"
with open(p) as f:
    src = f.read()
if '"cmp435_chunk3"' not in src:
    src = src.replace('"cmp420_chunk2" "cmp420_colpush" "cmp434_chunkpl" "parse-cache first hit" "parse-cache selftest" \\',
                      '"cmp420_chunk2" "cmp420_colpush" "cmp434_chunkpl" "cmp435_chunk3" "parse-cache first hit" "parse-cache selftest" \\', 1)
    with open(p, "w") as f:
        f.write(src)
    total += 1
    print("check_blobs_sync ok: marker added")
else:
    print("check_blobs_sync skip (already)")

print(f"ALL GATE SITES UPDATED ({total} edits)")
