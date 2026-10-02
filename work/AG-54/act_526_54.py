#!/usr/bin/env python3
"""act_526_54.py — AG-54 волна-526: leg-A w/dcp-мид bench-v2 @PIN a9ff088f (G4-FIX
carrier, tree 4231, blob 0049e34a) + leg-B pop/s/rt-мид WBP dp3v2 @PIN e49e8984
(blob 7c021f41, канон-носитель AG-257). Zero-code refs-API, CAS-CLAIM, race-guard
с 9+6 кандидатами (миды живут <3 мин — урок AG-274), 2 dispatch POST ≥31s
(AG-338), GET-вериф head_sha==PIN. Seed 527054 (526054 = моя нога x525,
коллизия). Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"   # G4-FIX w/dcp-carrier (md5 2da1febc)
BV2_BLOB = "0049e34a"
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
WBP_BLOB = "7c021f41"
SEED_A = "527054"

BV2_CANON = {"radius_blocks": "1136", "run_seconds": "9000", "server_xmx": "10G",
             "bench_dims": "minecraft:overworld", "drain_cap_polls": "900"}
WBP_CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
             "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
             "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
             "population_target": "150000", "population_seed": "42", "server_xmx": "10G",
             "server_xms": "4G", "cpu_band_min": "5500000", "cpu_band_max": "13500000",
             "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}

A_OPTS = [  # (cell, input-key, input-val, claim-fragment, race-regex)
    ("w4800", "dim_gen_window", "4800", "w4800 w-мид (4608-4992, 0-клейм) @a9ff088f", r"\bw4800\b"),
    ("w5184", "dim_gen_window", "5184", "w5184 w-мид (4992-5376, 0-клейм) @a9ff088f", r"\bw5184\b"),
    ("w5504", "dim_gen_window", "5504", "w5504 w-мид (5376-5632, 0-клейм) @a9ff088f", r"\bw5504\b"),
    ("w5952", "dim_gen_window", "5952", "w5952 w-мид (5632-6272, 0-клейм) @a9ff088f", r"\bw5952\b"),
    ("w6656", "dim_gen_window", "6656", "w6656 w-мид (6272-6912, 0-клейм) @a9ff088f", r"\bw6656\b"),
    ("w7168", "dim_gen_window", "7168", "w7168 w-мид (6912-7680, 0-клейм) @a9ff088f", r"\bw7168\b"),
    ("w13312", "dim_gen_window", "13312", "w13312 w-мид (12288-14336, 0-клейм) @a9ff088f", r"\bw13312\b"),
    ("dcp2100", "drain_cap_polls", "2100", "dcp2100 dcp-мид (1800-2400, 0-клейм) @a9ff088f", r"\bdcp2100\b"),
    ("dcp2700", "drain_cap_polls", "2700", "dcp2700 dcp-мид (2400-3000, 0-клейм) @a9ff088f", r"\bdcp2700\b"),
]
B_OPTS = [
    ("pop700k", "population_target", "700000", "pop700k pop-мид (650-750k) WBP dp3v2 s42",
     r"\bpop700k\b|\b700k\b"),
    ("pop725k", "population_target", "725000", "pop725k pop-мид (700-750k) WBP dp3v2 s42",
     r"\bpop725k\b"),
    ("s5250", "seconds", "5250", "s5250 s-мид (4500-6000) WBP dp3v2 s42",
     r"\bs5250\b|\b5250s\b|\b4500-6000\b"),
    ("s6750", "seconds", "6750", "s6750 s-верх (6000-7500) WBP dp3v2 s42", r"\bs6750\b"),
    ("rt30", "region_threads", "30", "rt30 rt-мид (28-32) WBP dp3v2 s42", r"\brt30\b"),
    ("rt22", "region_threads", "22", "rt22 rt-мид (20-24) WBP dp3v2 s42", r"\brt22\b"),
]
BR_A, BR_B = "swarm-526-54", "swarm-526-54b"


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code} {url}: {e.read()[:160]}", flush=True)
        raise
    return json.loads(body) if body else {}


def board_read(tok):
    d = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    return d["sha"], base64.b64decode(d["content"]).decode("utf-8")


def board_append(tok, lines, msg):
    for l in lines:
        assert len(l) <= 120, f"{len(l)} ch > 120: {l}"
    for attempt in range(8):
        sha, text = board_read(tok)
        if all(L in text for L in lines):
            print("already-appended", flush=True)
            return True
        new = text.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
        body = {"message": msg, "content": base64.b64encode(new.encode()).decode(),
                "sha": sha, "branch": "master"}
        try:
            r = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT", data=body)
            print(f"board-commit {r['commit']['sha'][:8]} (+{len(lines)})", flush=True)
            return True
        except urllib.error.HTTPError as e:
            if e.code in (409, 422) and attempt < 7:
                time.sleep(5)
                continue
            raise
    return False


def file_put(tok, path, content, msg):
    """Create-or-update repo file via contents API (payload/claims файлы)."""
    sha = None
    try:
        sha = api(tok, f"/repos/{REPO}/contents/{path}?ref=master")["sha"]
    except urllib.error.HTTPError as e:
        if e.code != 404:
            raise
    body = {"message": msg, "content": base64.b64encode(content.encode()).decode(),
            "branch": "master"}
    if sha:
        body["sha"] = sha
    r = api(tok, f"/repos/{REPO}/contents/{path}", method="PUT", data=body)
    print(f"file-put {path} -> {r['commit']['sha'][:8]}", flush=True)


def race_scan(text, rx):
    """Чужой CLAIM/FAIL на клетку = гонка (regex-boundary, живой GET; канон x525)."""
    for ln in text.splitlines():
        if not ln.startswith(("CLAIM", "FAIL")) or "AG-54 |" in ln:
            continue
        if re.search(rx, ln):
            return f"RACE {rx}: {ln[:90]}"
    return None


def pin_check(tok, pin, blob_prefix, yml):
    c = api(tok, f"/repos/{REPO}/commits/{pin}")
    tree_sha = c["commit"]["tree"]["sha"]
    tree = api(tok, f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
    n = len(tree.get("tree", []))
    trunc = tree.get("truncated", False)
    yblob = next((x["sha"] for x in tree.get("tree", [])
                  if x["path"] == f".github/workflows/{yml}"), "?")
    print(f"pin={pin[:8]} tree={n} trunc={trunc} {yml}={yblob[:8]}", flush=True)
    if trunc or n < 3200 or not str(yblob).startswith(blob_prefix):
        sys.exit(f"FATAL: PIN-вериф провален {pin[:8]} (tree={n} trunc={trunc} yml={yblob[:8]})")
    return n


def pick_cells(tok):
    _, text = board_read(tok)
    a = next((c for c in A_OPTS if not race_scan(text, c[4])), None)
    b = next((c for c in B_OPTS if not race_scan(text, c[4])), None)
    if not a or not b:
        sys.exit(f"FATAL: все клетки пивота заняты A={a} B={b}")
    return a, b


def leg_inputs(a, b):
    ia = dict(BV2_CANON); ia[a[1]] = a[2]; ia["seed"] = SEED_A
    ib = dict(WBP_CANON); ib[b[1]] = b[2]
    return ia, ib


def find_runs(tok, pre_ids, pages=3):
    found = {}
    for page in range(1, pages + 1):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in (BR_A, BR_B) and run["id"] not in pre_ids:
                pin = PIN_BV2 if b == BR_A else PIN_WBP
                ok = run["head_sha"] == pin
                print(f"RUN {run['id']} {b} sha={run['head_sha'][:8]} "
                      f"{run['status']} match={'OK' if ok else 'MISMATCH'}", flush=True)
                if ok:
                    found[b] = run["id"]
        if len(found) >= 2:
            break
    return found


def phase_run():
    tok = token()
    n1 = pin_check(tok, PIN_BV2, BV2_BLOB, "bench-v2.yml")
    n2 = pin_check(tok, PIN_WBP, WBP_BLOB, "world-bench-parallel.yml")
    a, b = pick_cells(tok)
    claim = f"CLAIM | AG-54 | {a[3]} + {b[3]} | 2 POST"
    assert len(claim) <= 120, len(claim)
    print(f"cells: {a[0]} + {b[0]}", flush=True)
    board_append(tok, [claim], "board: AG-54 CLAIM w/dcp-mid + pop/s/rt-mid WBP (wave-526)")
    file_put(tok, "claims/AG-54.md",
             open(f"{RD}/claims/AG-54.md").read(), "claims: AG-54 prereg w526 (wave-526)")
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for ref_name, sha in ((BR_A, PIN_BV2), (BR_B, PIN_WBP)):
        try:
            r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                    data={"ref": f"refs/heads/{ref_name}", "sha": sha})
            ver = r["object"]["sha"]
        except urllib.error.HTTPError as e:
            if e.code != 422:
                raise
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")
            ver = g.get("object", {}).get("sha", "")
        print(f"branch {ref_name}: sha={ver[:8]} match={ver == sha}", flush=True)
        if ver != sha:
            sys.exit(f"FATAL: ref sha mismatch {ref_name}")
        time.sleep(3)
    ia, ib = leg_inputs(a, b)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, a[0]), (BR_B, "world-bench-parallel.yml", ib, b[0]))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(12)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [a[0], b[0]], "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": a[0],
                         "seed": SEED_A, "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": b[0],
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-54/dispatch_526_54.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {a[0]}+{b[0]}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-54/dispatch_526_54.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-54/dispatch_526_54.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    ca, cb = p["cells"]
    fact = f"FACT | AG-54 | 2/2 204 @a9ff088f+e49e8984: {ra} {ca} s{SEED_A} + {rb} {cb} WBP QUEUED | api"
    disp = f"DISP | AG-54 | {ca}+{cb} миды 2/2 queued @swarm-526-54[ab] 1d/9000s/dcp900 + dp3v2 s42; work/AG-54"
    patch = f"PATCH_SUMMARY | AG-54 | files=claims,work/AG-54 | idea={ca}+{cb} dose mids w/pop осей | ev=2/2 204"
    for l in (fact, disp, patch):
        assert len(l) <= 120, f"{len(l)}: {l}"
    ok = board_append(tok, [fact, disp, patch], "board: AG-54 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    body = open(f"{RD}/work/AG-54/dispatch_526_54.json").read()
    file_put(tok, "work/AG-54/dispatch_526_54.json", body, "work: AG-54 payload w526 (wave-526)")
    for dst in (f"{RD}/work/AG-54", "/home/z/c-crussty/work/AG-54"):
        shutil.copy(f"{RD}/work/AG-54/dispatch_526_54.json", f"{dst}/dispatch_526_54.json")
        shutil.copy(__file__, f"{dst}/act_526_54.py")
    shutil.copy(f"{RD}/claims/AG-54.md", "/home/z/c-crussty/claims/AG-54.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={ca}+{cb}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
