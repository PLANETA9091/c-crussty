#!/usr/bin/env python3
"""TASK-452-C: STRICT-OR cmp452_mega into every gate/marker site of BOTH
families (pattern = add_450c_gates.py retag canon). Parent levers:
cmp451_senseins (sense/brain composite on ins6-carrier) and cmp450_chunk
(chunk4⊕chunk5⊕chunkparse union). cmp452_mega = mega-composition of the two.

Empty lever flag stays vanilla bit-in-byte (all gates equality checks).
"""
import re, glob

MEGA = "cmp452_mega"
PARENTS = ("cmp451_senseins", "cmp450_chunk")

RUST_FILES = sorted(glob.glob("src/*.rs"))
RUST_SKIP_MANUAL = {"src/chunk_parse.rs", "src/chunk_send.rs", "src/chunk_send5.rs"}
RUST_SKIP_DOC = {"src/lib.rs", "src/classfile.rs"}

def do_rust(path):
    with open(path) as f:
        lines = f.readlines()
    out, changed = [], 0
    i = 0
    while i < len(lines):
        t = lines[i].rstrip("\n")
        stripped = t.strip()
        if any(p in t for p in PARENTS) and MEGA not in t and not stripped.startswith("//") and not stripped.startswith("///") and not stripped.startswith("*"):
            indent = re.match(r"\s*", t).group(0)
            # 1) marker-id arm: Ok("cmpX") => ...
            m_arm = re.search(r'Ok\("(cmp[0-9_a-z]+)"\) =>', t)
            if m_arm and "=>" in t:
                out.append(t + "\n")
                out.append(indent + f'Ok("{MEGA}") => "{MEGA}", // TASK-452-C mega-composite (senseins \u2295 chunk union, STRICT-OR)\n')
                changed += 1
                i += 1
                continue
            # 2) matches!-style arm: Ok("cmpX") (no =>)
            if re.search(r'Ok\("(cmp[0-9_a-z]+)"\)', t):
                # append after the LAST Ok("...") on the line
                last = list(re.finditer(r'Ok\("(cmp[0-9_a-z]+)"\)', t))[-1]
                out.append(t[:last.end()] + f' | Ok("{MEGA}")' + t[last.end():] + "\n")
                changed += 1
                i += 1
                continue
            # 3) equality chain: var == "cmpX" -> append || var == mega
            m_eq = re.search(r'(\w+(?:\.trim\(\))?) == "(cmp[0-9_a-z]+)"', t)
            if m_eq:
                var = m_eq.group(1)
                out.append(t.replace(m_eq.group(0), m_eq.group(0) + f' || {var} == "{MEGA}"', 1) + "\n")
                changed += 1
                i += 1
                continue
            print(f"  rust UNHANDLED: {path}:{i+1}: {stripped[:100]}")
        out.append(lines[i])
        i += 1
    if changed:
        with open(path, "w") as f:
            f.writelines(out)
    print(f"rust ok ({changed}): {path}")
    return changed

def do_java(path):
    with open(path) as f:
        lines = f.readlines()
    out, changed = [], 0
    for i, t in enumerate(lines):
        raw = t.rstrip("\n")
        stripped = raw.strip()
        if any(p in raw for p in PARENTS) and MEGA not in raw and not stripped.startswith("//") and not stripped.startswith("*"):
            # 0) marker-map if/ternary lines carry `return` or `? "` — the
            # resolve specials already wrote proper mega arms there. SKIP.
            if "return " in raw or re.search(r'\? "cmp', raw):
                out.append(lines[i]); continue
            # 1) gate arm end: f.trim().equals("cmpX"));
            if re.search(r'equals\("cmp[0-9_a-z]+"\)\)\s*;', raw):
                last = list(re.finditer(r'equals\("(cmp[0-9_a-z]+)"\)', raw))[-1]
                raw2 = raw[:last.end()] + f' || f.trim().equals("{MEGA}")' + raw[last.end():]
                out.append(raw2 + "\n"); changed += 1; continue
            # 2) reversed arm: "cmpX".equals(LEVER_FLAG);
            if re.search(r'"cmp[0-9_a-z]+"\.equals\(LEVER_FLAG\)', raw):
                last = list(re.finditer(r'"(cmp[0-9_a-z]+)"\.equals\(LEVER_FLAG\)', raw))[-1]
                raw2 = raw[:last.end()] + f' || "{MEGA}".equals(LEVER_FLAG)' + raw[last.end():]
                out.append(raw2 + "\n"); changed += 1; continue
            # 3) pipe-string constant (TICK2_FLAGS style): append |mega — only
            # when the quoted value ALREADY has a pipe (multi-flag string).
            m_pipe = re.search(r'(=\s*"[^"]*\|[^"]*")', raw)
            if m_pipe and raw.rstrip().endswith(";"):
                raw2 = raw.replace(m_pipe.group(1), m_pipe.group(1)[:-1] + f"|{MEGA}\"", 1)
                out.append(raw2 + "\n"); changed += 1; continue
            print(f"  java UNHANDLED: {path}:{i+1}: {stripped[:100]}")
        out.append(lines[i])
    if changed:
        with open(path, "w") as f:
            f.writelines(out)
    print(f"java ok ({changed}): {path}")
    return changed

JAVA_FILES = [
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "sense/net/minecraft/world/entity/SenseOps.java",
    "randomtick/src/BrainOps.java",
]

total = 0
for p in RUST_FILES:
    if p in RUST_SKIP_DOC:
        continue
    total += do_rust(p)
for p in JAVA_FILES:
    total += do_java(p)

# ---- chunk_parse.rs / chunk_send.rs / chunk_send5.rs: LEVER_ID-style gates.
for p, gate_line, marker_anchor in [
    ("src/chunk_parse.rs",
     '                || v == "cmp437_chunk4" || v == "cmp444_chunk5" || v == "cmp450_chunk"',
     '        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),'),
    ("src/chunk_send.rs",
     '            v == LEVER_ID || v == "cmp444_chunk5" || v == "cmp450_chunk"',
     '        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),'),
]:
    with open(p) as f:
        src = f.read()
    if MEGA in src:
        print(f"rust skip (already): {p}"); continue
    assert gate_line in src, f"gate anchor missing: {p}"
    src = src.replace(gate_line, gate_line + f' || v == "{MEGA}"', 1)
    src = src.replace(marker_anchor,
        marker_anchor + f'\n        Ok("{MEGA}") => std::borrow::Cow::Owned("{MEGA}".to_string()),', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"rust ok (manual): {p}")
    total += 1

p = "src/chunk_send5.rs"
with open(p) as f:
    src = f.read()
if MEGA not in src:
    src = src.replace(
        '        .map(|v| v.trim() == LEVER_ID || v.trim() == "cmp450_chunk")',
        f'        .map(|v| v.trim() == LEVER_ID || v.trim() == "cmp450_chunk" || v.trim() == "{MEGA}")', 1)
    src = src.replace(
        '        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),',
        f'        Ok("cmp450_chunk") => std::borrow::Cow::Owned("cmp450_chunk".to_string()),\n'
        f'        Ok("{MEGA}") => std::borrow::Cow::Owned("{MEGA}".to_string()),', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"rust ok (manual): {p}")
    total += 1

# ---- Java carrier constants (raw-cp markers for check_blobs_sync needles).
for p in [
    "chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java",
    "chunksend/net/minecraft/server/network/ChunkSendOps.java",
    "chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java",
]:
    with open(p) as f:
        src = f.read()
    if MEGA in src:
        print(f"java skip (already): {p}"); continue
    anchor = '    static final String CARRIER_UNION_450 = "cmp450_chunk";'
    assert anchor in src, f"carrier anchor missing: {p}"
    src = src.replace(anchor, anchor + '\n'
        '    /** TASK-452-C mega-composite (senseins \u2295 chunk union; STRICT-OR; raw-cp\n'
        '     * marker for the check_blobs_sync gate). */\n'
        f'    static final String CARRIER_UNION_452 = "{MEGA}";', 1)
    with open(p, "w") as f:
        f.write(src)
    print(f"java ok (1): {p} (CARRIER_UNION_452)")
    total += 1

print(f"TOTAL retag sites: {total}")

# ---- check_blobs_sync.sh: append cmp452_mega needle to every class list that
# carries a parent lever needle (the rebuilt blobs will contain the mega id).
p = "scripts/check_blobs_sync.sh"
with open(p) as f:
    src = f.read()
n_sh = 0
if MEGA not in src:
    def _add_needle(m):
        global n_sh
        n_sh += 1
        return m.group(1) + f' "{MEGA}"' + m.group(2)
    src = re.sub(r'(\n  "[^"\n]*(?:cmp451_senseins|cmp450_chunk)[^"\n]*")(\s*\\)', _add_needle, src)
    src = re.sub(r'(\n  "[^"\n]*(?:cmp451_senseins|cmp450_chunk)[^"\n]*")(\n)', _add_needle, src)
    with open(p, "w") as f:
        f.write(src)
print(f"sh needles ok ({n_sh}): {p}")
total += n_sh
