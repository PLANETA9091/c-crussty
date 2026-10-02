#!/usr/bin/env python3
"""act_526_67.py — AG-67 волна-526: leg-A xmx28G xmx-мид (26-30G, соседи xmx26G AG-272 +
xmx30G AG-250 оба QUEUED, 0-клейм) lane bench-v2 @PIN 32a448da (канон-yml b4e9e05b,
G4-fix, ≥3200 файлов) seed 526067 + leg-B pop875k pop-мид WBP (800k-1M, соседи pop800k
AG-76 + pop1M AG-58 оба QUEUED, 0-клейм, dp3v2-когорта seed42) @PIN e9bb6dc5 (t4241,
wbp-yml 7c021f41). Zero-code refs-API, CAS-CLAIM, race-guard с пивотами
(sim76/s7500; rt36/s5250), 2 dispatch POST ≥31s, GET-вериф head_sha==PIN.
Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "32a448dabf46a0b9ef8e85f13a202ab98ab81bd5"   # bench-v2 G4-fix, yml verbatim AG-50
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob
PIN_WBP = "e9bb6dc5a39f863ccee24c273793adaf3a686b18"   # WBP-канон t4241 (AG-50)
WBP_BLOB = "7c021f41"                                   # world-bench-parallel.yml blob

BV2_CANON = {"radius_blocks": "1136", "run_seconds": "9000", "server_xmx": "10G",
             "bench_dims": "minecraft:overworld", "dim_gen_window": "256",
             "drain_cap_polls": "900", "fake_players": "4"}
WBP_CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
             "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
             "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
             "population_target": "150000", "population_seed": "42", "server_xmx": "10G",
             "server_xms": "4G", "cpu_band_min": "5500000", "cpu_band_max": "13500000",
             "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}

BV2_OPTS = [("xmx28G", "28G"), ("sim76", "76"), ("s7500", "7500")]
WBP_OPTS = [("pop875k", "875000"), ("rt36", "36"), ("s5250", "5250")]
BV2_FIELD = {"xmx28G": "server_xmx", "sim76": "simulation_distance", "s7500": "run_seconds"}
WBP_FIELD = {"pop875k": "population_target", "rt36": "region_threads", "s5250": "seconds"}
BV2_DESC = {"xmx28G": "xmx28G xmx-мид (26-30G)", "sim76": "sim76 sim-мид (72-80)",
            "s7500": "s7500 seconds-мид bv2 (6000-9000)"}
WBP_DESC = {"pop875k": "pop875k pop-мид WBP (800k-1M)", "rt36": "rt36 rt-мид WBP (32-40)",
            "s5250": "s5250 seconds-мид WBP (4500-6000)"}
CLAIMS = {
    (b, w): (f"CLAIM | AG-67 | {BV2_DESC[b]} + {WBP_DESC[w]}: "
             f"1d/9000s/dcp900 + dp3v2 s42 | 2 POST")
    for b, _ in BV2_OPTS for w, _ in WBP_OPTS
}
BR_A, BR_B = "swarm-526-67", "swarm-526-67b"


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
        print(f"HTTP {e.code} {url}: {e.read()[:200]}", flush=True)
        raise
    return json.loads(body) if body else {}


def board_read(tok):
    d = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    return d["sha"], base64.b64decode(d["content"]).decode("utf-8")


def board_append(tok, lines, msg):
    for l in lines:
        assert len(l) <= 120, f"{len(l)} ch > 120: {l}"
    for attempt in range(6):
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
            if e.code in (409, 422) and attempt < 5:
                time.sleep(5)
                continue
            raise
    return False


def race_scan(text, cell):
    """Чужой CLAIM на клетку = гонка (regex-boundary, живой GET; канон x525)."""
    pats = {
        "xmx28G": r"\bxmx28g?\b|\bxmx-28\b",
        "sim76": r"\bsim76\b",
        "s7500": r"\bs7500\b|\b7500s\b|6000-9000",
        "pop875k": r"\bpop875\w*\b|\b875000\b|\b875k\b",
        "rt36": r"\brt36\b(?!s)",
        "s5250": r"\bs5250\b|\b5250s\b|4500-6000",
    }
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-67 |" in ln:
            continue
        low = ln.lower()
        if re.search(pats[cell], low):
            return f"RACE {cell}: {ln[:100]}"
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
    bv2 = next((c for c, _ in BV2_OPTS if not race_scan(text, c)), None)
    wbp = next((c for c, _ in WBP_OPTS if not race_scan(text, c)), None)
    if not bv2 or not wbp:
        sys.exit(f"FATAL: все клетки пивота заняты bv2={bv2} wbp={wbp}")
    return bv2, wbp, text


def leg_inputs(bv2, wbp):
    a = dict(BV2_CANON); a[BV2_FIELD[bv2]] = dict(BV2_OPTS)[bv2]; a["seed"] = "526067"
    b = dict(WBP_CANON); b[WBP_FIELD[wbp]] = dict(WBP_OPTS)[wbp]
    return a, b


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
    bv2, wbp, _ = pick_cells(tok)
    claim = CLAIMS[(bv2, wbp)]
    assert len(claim) <= 120
    print(f"cells: {bv2} + {wbp}", flush=True)
    board_append(tok, [claim], "board: AG-67 CLAIM xmx28G-mid+WBP-pop875k (wave-526)")
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
        time.sleep(4)
    ia, ib = leg_inputs(bv2, wbp)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, bv2), (BR_B, "world-bench-parallel.yml", ib, wbp))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(8)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [bv2, wbp], "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": bv2,
                         "seed": "526067", "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": wbp,
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-67/dispatch_526_67.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {bv2}+{wbp}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-67/dispatch_526_67.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-67/dispatch_526_67.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    bv2, wbp = p["cells"]
    AX = {"xmx28G": "xmx-мид", "sim76": "sim-мид", "s7500": "seconds-мид",
          "pop875k": "pop-мид", "rt36": "rt-мид", "s5250": "seconds-мид"}
    lines = [
        (f"FACT | AG-67 | 2/2 204 @32a448da+e9bb6dc5: {ra} {bv2} s526067 + {rb} {wbp} "
         f"WBP QUEUED | api"),
        (f"DISP | AG-67 | {bv2}+{wbp} миды 2/2 queued @swarm-526-67[ab] "
         f"1d/9000s/dcp900 + dp3v2 s42; payload work/AG-67"),
        (f"PATCH_SUMMARY | AG-67 | files=work+claims/AG-67 | idea={bv2}+{wbp} миды "
         f"dose fill, пивот xmx28G | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-67 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-67", "/home/z/c-crussty/work/AG-67"):
        shutil.copy(f"{RD}/work/AG-67/dispatch_526_67.json", f"{dst}/dispatch_526_67.json")
        shutil.copy(__file__, f"{dst}/act_526_67.py")
    shutil.copy(f"{RD}/claims/AG-67.md", "/home/z/c-crussty/claims/AG-67.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={bv2}+{wbp}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
