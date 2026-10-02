#!/usr/bin/env python3
"""TASK-452-C step-1: resolve merge conflicts round-451d-senseins ⊕ round-451c-chunk.

STRICT-OR mega-composition: BOTH families live. Gate-list hunks = token union
(HEAD sense/ins4 lines + theirs chunk lines). cmp452_mega tagging is a SEPARATE
pass (add_452c_gates.py, canon add_450c_gates.py). Special hunks handled
explicitly (flag constants, marker maps, needle lists, RESULT.json).
"""
import re, sys

W = "/home/z/rounds/ROUND-452/agent-c/"

# ---------------------------------------------------------------- helpers
def split_hunks(text):
    """Yield (kind, payload) pieces: 'text' or ('hunk', head_lines, their_lines)."""
    out, cur, mode = [], [], None
    for line in text.splitlines(keepends=True):
        if line.startswith("<<<<<<<"):
            if cur:
                out.append(("text", "".join(cur))); cur = []
            head, their = [], []
            mode = "head"
        elif line.startswith("=======") and mode == "head":
            mode = "their"
        elif line.startswith(">>>>>>>") and mode == "their":
            out.append(("hunk", head, their))
            mode = None
        elif mode == "head":
            head.append(line)
        elif mode == "their":
            their.append(line)
        else:
            cur.append(line)
    if cur:
        out.append(("text", "".join(cur)))
    return out

def tokens(lines):
    return re.findall(r'"(cmp[0-9_a-z]+)"', "".join(lines))

# ------------------------------------------------------- generic union rule
def union_gate_hunk(head, their):
    """Concatenate head+their lines; drop lines whose ONLY novel tokens are
    duplicates of tokens already present (keeps comments of first side)."""
    all_lines = head + their
    seen = set()
    out = []
    for ln in all_lines:
        tk = tokens([ln])
        if not tk:
            out.append(ln); continue
        novel = [t for t in tk if t not in seen]
        if not novel:
            # pure duplicate lever line (same token, no new info) — drop line
            continue
        seen.update(tk)
        out.append(ln)
    return "".join(out)

def resolve_file(path, hunk_fn=union_gate_hunk):
    with open(W + path) as f:
        text = f.read()
    if "<<<<<<<" not in text:
        print(f"skip (no conflicts): {path}")
        return
    pieces = split_hunks(text)
    out = []
    for kind, *payload in pieces:
        if kind == "text":
            out.append(payload[0])
        else:
            head, their = payload
            out.append(hunk_fn(head, their))
    with open(W + path, "w") as f:
        f.write("".join(out))
    n_conf = sum(1 for k, *_ in pieces if k == "hunk")
    print(f"resolved ({n_conf} hunks): {path}")

# ------------------------------------------------------------- special rules
def union_needles(head, their):
    """check_blobs_sync.sh needle lists: dedupe token-wise, keep structure."""
    return union_gate_hunk(head, their)

def colpush_flags(head, their):
    """ColpushOps FLAG constants: rename theirs FLAG4..FLAG8 -> FLAG10..FLAG14,
    keep HEAD FLAG4/5/6, add FLAG_MEGA = cmp452_mega."""
    body_t = "".join(their)
    body_t = body_t.replace('private static final String FLAG4 = "cmp434_chunkpl";',
                            'private static final String FLAG10 = "cmp434_chunkpl";')
    body_t = body_t.replace('private static final String FLAG5 = "cmp435_chunk3";',
                            'private static final String FLAG11 = "cmp435_chunk3";')
    body_t = body_t.replace('private static final String FLAG6 = "cmp437_chunk4";',
                            'private static final String FLAG12 = "cmp437_chunk4";')
    body_t = body_t.replace('private static final String FLAG7 = "cmp444_chunk5";',
                            'private static final String FLAG13 = "cmp444_chunk5";')
    body_t = body_t.replace('private static final String FLAG8 = "cmp450_chunk";',
                            'private static final String FLAG14 = "cmp450_chunk";')
    mega = ('    /** TASK-452-C mega-composite (senseins \u2295 chunk union, STRICT-OR). */\n'
            '    private static final String FLAG_MEGA = "cmp452_mega";\n')
    return "".join(head) + mega + body_t

def colpush_enabled(head, their):
    """enabled() chain: union line = HEAD body with their FLAG7/FLAG8 renamed
    FLAG13/FLAG14 + FLAG10..12 tokens + FLAG_MEGA appended."""
    h = "".join(head).rstrip("\n")
    m = re.search(r'(return f != null && \(f\.trim\(\)\.equals\(FLAG\) .*?equals\(FLAG6\))', h)
    assert m, "colpush enabled HEAD shape changed"
    base = m.group(1)
    new = (base
           + ' || f.trim().equals(FLAG10) || f.trim().equals(FLAG11) || f.trim().equals(FLAG12)'
           + ' || f.trim().equals(FLAG13) || f.trim().equals(FLAG14)'
           + ' || f.trim().equals(FLAG_MEGA));\n')
    return new

def mobpush_label(head, their):
    """eqsnapFlagLabel: keep HEAD arms, add their chunk arms with FIXED
    precedence (theirs had `f != null && eq(A) || eq(B)` NPE trap), add mega arm."""
    out = "".join(head)
    out += ('        if (f != null && (f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk"))) return "cmp437_chunk4"; // TASK-438-C R7 marker id (452c: precedence fixed vs 451c)\n'
            '        if (f != null && (f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3"))) return "cmp434_chunkpl"; // TASK-434-C\n'
            '        if (f != null && f.trim().equals("cmp452_mega")) return "cmp452_mega"; // TASK-452-C mega-composite marker id\n')
    return out

def queryplane_marker(head, their):
    """lever_id() match arms: keep HEAD (inside label + sense arms), add their
    chunk arms (dedup cmp450_chunk dup), add mega arm."""
    out = "".join(head)
    body_t = "".join(their)
    # drop the duplicated cmp450_chunk arm that theirs itself carries twice
    first = body_t.find('Ok("cmp450_chunk") => "cmp450_chunk"')
    second = body_t.find('Ok("cmp450_chunk") => "cmp450_chunk"', first + 1)
    if second != -1:
        line_start = body_t.rfind("\n", 0, second) + 1
        line_end = body_t.find("\n", second) + 1
        body_t = body_t[:line_start] + body_t[line_end:]
    out += body_t
    out += '        Ok("cmp452_mega") => "cmp452_mega", // TASK-452-C mega-composite marker id (senseins \u2295 chunk union)\n'
    return out

def goalquery_marker(head, their):
    """Ternary marker chain: head arms + their arms + mega arm."""
    return "".join(head) + "".join(their) + \
        '                : t.equals("cmp452_mega") ? "cmp452_mega" // TASK-452-C mega-composite marker id\n'

def result_json(head, their):
    """RESULT.json: composite write-through (rewritten later; union now)."""
    return '{\n  "agent": "TASK-452-C mega-composite (resolved in favor of cmp452_mega step)",\n}\n'

SPECIAL = {
    "colpush/net/minecraft/world/entity/ColpushOps.java": [colpush_flags, colpush_enabled],
    "mobpush/net/minecraft/world/entity/MobPushOps.java": [union_gate_hunk, union_gate_hunk, mobpush_label],
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java": [union_gate_hunk],
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java": [union_gate_hunk, goalquery_marker],
    "src/queryplane.rs": [union_gate_hunk, queryplane_marker],
    "scripts/check_blobs_sync.sh": [union_needles] * 7,
    "RESULT.json": [result_json],
}

FILES = [
    "src/chunk_parse.rs", "src/collide_batch.rs", "src/colpush.rs", "src/entity_query.rs",
    "src/goal_selector.rs", "src/inside_bitmask.rs", "src/inside_snap.rs", "src/items_index.rs",
    "src/items_manager.rs", "src/mobs_ai.rs", "src/mobs_grid.rs", "src/mobs_manager.rs",
    "src/mobs_soa.rs", "src/mobs_sscan.rs", "src/nav_plane.rs", "src/noise_fill.rs",
    "src/queryplane.rs", "src/stagger.rs", "src/tickplane.rs",
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
    "scripts/check_blobs_sync.sh",
    "RESULT.json",
]

def main():
    only = sys.argv[1] if len(sys.argv) > 1 else None
    for path in FILES:
        if only and only not in path:
            continue
        fns = SPECIAL.get(path, [union_gate_hunk])
        state = {"i": 0}
        def hunk_fn(head, their, _fns=fns, _state=state):
            i = _state["i"]
            fn = _fns[i] if i < len(_fns) else union_gate_hunk
            _state["i"] = i + 1
            return fn(head, their)
        resolve_file(path, hunk_fn)

if __name__ == "__main__":
    main()
