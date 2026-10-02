#!/usr/bin/env python3
"""TASK-444-B step: STRICT-OR cmp444_chunk5 into all gate sites (pattern =
add_chunkpl2_gates.py / e93bb8c2 retag) — R8 stage-2 carrier flag ON TOP of
cmp437_chunk4: the full composite union (parse planes + send snapshot plane +
all mob/inside/item/nav lanes) + the NEW encode-cache plane (chunk_send5.rs,
its own file — NOT touched here). Round-id hygiene for ROUND-444-B legs.

Exclusions: src/chunk_send.rs (chunk4 plane — its enabled() was extended
manually to accept the carrier), src/chunk_send5.rs (the new plane itself,
STRICT own-id), src/classfile.rs (constants/guards only), src/lib.rs (docs).
"""
import re

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
    "src/queryplane.rs",
    "src/chunk_parse.rs",
]

def do_rust(path):
    with open(path) as f:
        src = f.readlines()
    changed = 0
    for i, line in enumerate(src):
        if "cmp437_chunk4" not in line or "cmp444_chunk5" in line:
            continue
        t = line.rstrip("\n")
        # marker-id match arm: Ok("cmp437_chunk4") => "cmp437_chunk4",
        if re.search(r'Ok\("cmp437_chunk4"\) => "cmp437_chunk4"', t):
            indent = re.match(r"\s*", t).group(0)
            marker = indent + 'Ok("cmp444_chunk5") => "cmp444_chunk5", // TASK-444-B: R8 carrier marker id (encode-cache stage-2)'
            src.insert(i + 1, marker + "\n")
            changed += 1
            continue
        m = re.search(r"(\w+(?:\.trim\(\))?) == \"cmp437_chunk4\"", t)
        if m:
            nl = t.replace(m.group(0), m.group(0) + f' || {m.group(1)} == "cmp444_chunk5"', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        if 'Ok("cmp437_chunk4")' in t:
            nl = t.replace('Ok("cmp437_chunk4")', 'Ok("cmp437_chunk4") | Ok("cmp444_chunk5")', 1)
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
        if "cmp437_chunk4" not in line or "cmp444_chunk5" in line:
            continue
        t = line.rstrip("\n")
        # gate arm: || f.trim().equals("cmp437_chunk4"));
        if 'f.trim().equals("cmp437_chunk4"))' in t:
            nl = t.replace('f.trim().equals("cmp437_chunk4"))',
                           'f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5"))', 1)
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

# ChunkParseOps: CARRIER_UNION_444 constant (raw-cp marker; the parse plane's
# rust gate was extended by the script above).
p = "chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java"
with open(p) as f:
    src = f.read()
if "cmp444_chunk5" not in src:
    src = src.replace(
        '    static final String CARRIER_UNION_437 = "cmp437_chunk4";',
        '    static final String CARRIER_UNION_437 = "cmp437_chunk4";\n'
        '    /** TASK-444-B: R8 stage-2 carrier (STRICT-OR; raw-cp marker for the\n'
        '     * check_blobs_sync gate). */\n'
        '    static final String CARRIER_UNION_444 = "cmp444_chunk5";', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"java ok (1): {p} (CARRIER_UNION_444)")
    total += 1

# ColpushOps: FLAG7 constant + gate reference.
p = "colpush/net/minecraft/world/entity/ColpushOps.java"
with open(p) as f:
    src = f.read()
if "cmp444_chunk5" not in src:
    src = src.replace(
        '    private static final String FLAG6 = "cmp437_chunk4";',
        '    private static final String FLAG6 = "cmp437_chunk4";\n'
        '    /** TASK-444-B: R8 stage-2 carrier (STRICT-OR; raw-cp marker for the\n'
        '     * check_blobs_sync gate). */\n'
        '    private static final String FLAG7 = "cmp444_chunk5";', 1)
    src = src.replace(
        'f.trim().equals(FLAG6));',
        'f.trim().equals(FLAG6) || f.trim().equals(FLAG7));', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"java ok (2): {p} (FLAG7 + gate)")
    total += 2

# ChunkSendOps: CARRIER_UNION_444 constant is added by hand in the source;
# verify it landed.
p = "chunksend/net/minecraft/server/network/ChunkSendOps.java"
with open(p) as f:
    src = f.read()
assert "cmp444_chunk5" in src, "ChunkSendOps must carry the cmp444_chunk5 carrier"
print("java ok: chunksend ChunkSendOps carries cmp444_chunk5")

print(f"TOTAL gates retagged: {total}")
