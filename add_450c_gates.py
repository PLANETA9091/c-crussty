#!/usr/bin/env python3
"""TASK-450-C: STRICT-OR cmp450_chunk into all gate sites (pattern =
add_chunk5_gates.py / e93bb8c2 / 2a07c491 retags) — R8 chunk-pipeline UNION
carrier ON TOP of cmp437_chunk4 (send snapshot) ⊕ cmp444_chunk5 (encode-cache):
the full chunk4+chunk5 composite union + the NEW 450-C slices (research-450-C).
Round-id hygiene for ROUND-450-C legs.

Exclusions: src/chunk_send.rs + src/chunk_send5.rs + src/chunk_parse.rs (their
enabled() gates extended manually below), src/classfile.rs (constants/guards
only), src/lib.rs (docs).
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
]


def do_rust(path):
    with open(path) as f:
        src = f.readlines()
    changed = 0
    for i, line in enumerate(src):
        if "cmp444_chunk5" not in line or "cmp450_chunk" in line:
            continue
        t = line.rstrip("\n")
        # marker-id match arm: Ok("cmp444_chunk5") => "cmp444_chunk5",
        if re.search(r'Ok\("cmp444_chunk5"\) => "cmp444_chunk5"', t):
            indent = re.match(r"\s*", t).group(0)
            marker = indent + 'Ok("cmp450_chunk") => "cmp450_chunk", // TASK-450-C: union carrier marker id (chunk4⊕chunk5⊕slices)'
            src.insert(i + 1, marker + "\n")
            changed += 1
            continue
        m = re.search(r"(\w+(?:\.trim\(\))?) == \"cmp444_chunk5\"", t)
        if m:
            nl = t.replace(m.group(0), m.group(0) + f' || {m.group(1)} == "cmp450_chunk"', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        if 'Ok("cmp444_chunk5")' in t:
            nl = t.replace('Ok("cmp444_chunk5")', 'Ok("cmp444_chunk5") | Ok("cmp450_chunk")', 1)
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
        if "cmp444_chunk5" not in line or "cmp450_chunk" in line:
            continue
        t = line.rstrip("\n")
        # gate arm: || f.trim().equals("cmp444_chunk5"));
        if 'f.trim().equals("cmp444_chunk5"))' in t:
            nl = t.replace('f.trim().equals("cmp444_chunk5"))',
                           'f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk"))', 1)
            src[i] = nl + "\n"
            changed += 1
            continue
        # reversed gate arm: || "cmp444_chunk5".equals(LEVER_FLAG);
        if '"cmp444_chunk5".equals(LEVER_FLAG);' in t:
            nl = t.replace('"cmp444_chunk5".equals(LEVER_FLAG);',
                           '"cmp444_chunk5".equals(LEVER_FLAG) || "cmp450_chunk".equals(LEVER_FLAG);', 1)
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

# chunk_parse.rs (parse plane gate — extended manually: the file was excluded
# from the glob in the chunk5 retag; union carriers ride both ids).
p = "src/chunk_parse.rs"
total += do_rust(p)

# chunk_send.rs (chunk4 send-snapshot plane): enabled() accepts
# LEVER_ID(cmp437_chunk4) | cmp444_chunk5 -> add cmp450_chunk.
p = "src/chunk_send.rs"
with open(p) as f:
    src = f.read()
if "cmp450_chunk" not in src:
    src = src.replace(
        'v == LEVER_ID || v == "cmp444_chunk5"',
        'v == LEVER_ID || v == "cmp444_chunk5" || v == "cmp450_chunk"', 1)
    # carrier-constant doc block: add CARRIER_UNION_450 next to _444
    src = src.replace(
        'const CARRIER_UNION_444: &str = "cmp444_chunk5";',
        'const CARRIER_UNION_444: &str = "cmp444_chunk5";\n'
        '/// TASK-450-C union carrier (chunk4⊕chunk5⊕slices; STRICT-OR; raw-cp marker).\n'
        'const CARRIER_UNION_450: &str = "cmp450_chunk";', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"rust ok (manual): {p} (enabled + CARRIER_UNION_450 if present)")
    total += 1

# chunk_send5.rs (encode-cache plane): STRICT own-id gate -> accept union id.
p = "src/chunk_send5.rs"
with open(p) as f:
    src = f.read()
changed = 0
if "cmp450_chunk" not in src:
    out = []
    for line in src.splitlines(keepends=True):
        t = line.rstrip("\n")
        if re.search(r'Ok\("cmp444_chunk5"\)', t) and "cmp450_chunk" not in t:
            line = t.replace('Ok("cmp444_chunk5")', 'Ok("cmp444_chunk5") | Ok("cmp450_chunk")', 1) + "\n"
            changed += 1
        elif re.search(r'== "cmp444_chunk5"', t) and "cmp450_chunk" not in t:
            line = t.replace('== "cmp444_chunk5"', '== "cmp444_chunk5" || v == "cmp450_chunk"', 1) + "\n"
            changed += 1
        out.append(line)
    src = "".join(out)
    with open(p, "w") as f:
        f.write(src)
    print(f"rust ok ({changed}): {p} (union id)")
    total += 1

# Java carrier constants (raw-cp markers for check_blobs_sync needles).
for p, anchor in [
    ("chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java",
     '    static final String CARRIER_UNION_444 = "cmp444_chunk5";'),
    ("chunksend/net/minecraft/server/network/ChunkSendOps.java",
     '    static final String CARRIER_UNION_444 = "cmp444_chunk5";'),
]:  # ChunkPacketEncodeOps has no _444 constant — handled below

    with open(p) as f:
        src = f.read()
    if "cmp450_chunk" not in src:
        assert anchor in src, f"anchor missing in {p}"
        src = src.replace(
            anchor,
            anchor + '\n'
            '    /** TASK-450-C union carrier (STRICT-OR; raw-cp marker for the\n'
            '     * check_blobs_sync gate). */\n'
            '    static final String CARRIER_UNION_450 = "cmp450_chunk";', 1)
        with open(p, "w") as f:
            f.write(src)
        print(f"java ok (1): {p} (CARRIER_UNION_450)")
        total += 1

# ChunkPacketEncodeOps: add CARRIER_UNION_450 next to the _437 constant.
p = "chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java"
with open(p) as f:
    src = f.read()
if "cmp450_chunk" not in src:
    anchor = '    static final String CARRIER_UNION_437 = "cmp437_chunk4";'
    assert anchor in src, f"anchor missing in {p}"
    src = src.replace(
        anchor,
        anchor + '\n'
        '    /** TASK-450-C union carrier (STRICT-OR; raw-cp marker for the\n'
        '     * check_blobs_sync gate). */\n'
        '    static final String CARRIER_UNION_450 = "cmp450_chunk";', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"java ok (1): {p} (CARRIER_UNION_450)")
    total += 1

# chunk_send5.rs manual: encode plane gate -> own id OR union id.
p = "src/chunk_send5.rs"
with open(p) as f:
    src = f.read()
if "cmp450_chunk" not in src:
    src = src.replace(
        '        .map(|v| v.trim() == LEVER_ID)',
        '        .map(|v| v.trim() == LEVER_ID || v.trim() == "cmp450_chunk")', 1)
    src = src.replace(
        '/// cmp435_chunk3, cmp437_chunk4) MUST NOT gain this plane (their certified',
        '/// cmp435_chunk3, cmp437_chunk4) MUST NOT gain this plane (their certified\n'
        '/// semantics are frozen) EXCEPT the TASK-450-C union carrier cmp450_chunk', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"rust ok (manual): {p} (union id)")
    total += 1

# ColpushOps: FLAG8 constant + gate reference.
p = "colpush/net/minecraft/world/entity/ColpushOps.java"
with open(p) as f:
    src = f.read()
if "cmp450_chunk" not in src:
    src = src.replace(
        '    private static final String FLAG7 = "cmp444_chunk5";',
        '    private static final String FLAG7 = "cmp444_chunk5";\n'
        '    /** TASK-450-C union carrier (chunk4⊕chunk5⊕slices). */\n'
        '    private static final String FLAG8 = "cmp450_chunk";', 1)
    src = src.replace(
        '|| f.trim().equals(FLAG7));',
        '|| f.trim().equals(FLAG7) || f.trim().equals(FLAG8));', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"java ok (1): {p} (FLAG8)")
    total += 1

print(f"TOTAL retag sites: {total}")
