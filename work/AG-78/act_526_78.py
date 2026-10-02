#!/usr/bin/env python3
"""act_526_78.py — AG-78 волна-526: leg-A sim96 sim-мид (зазор 80-128, соседи sim80
AG-40×526 / sim128 AG-15×526, deficit-карта AG-18 §3) lane bench-v2 @PIN 2171d6da
(sim-канон-носитель AG-138, yml-blob b4e9e05b) + leg-B rt32 rt-верх WBP за 24
(rt24 AG-262×525, rt20 AG-1/20×526) @PIN e49e8984 (WBP-канон AG-257, wbp-yml
7c021f41). Zero-code refs-API, CAS-CLAIM, race-guard с пивотами (sim72/sim60;
rt28/s500), 2 dispatch POST ≥31s (AG-338), GET-вериф head_sha==PIN. Д1-Д5:
API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "2171d6da775975c4bee94748f549ad16f02074e1"   # bench-v2 sim-канон AG-138
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob @PIN
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
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

SIM_OPTS = [("sim96", "96"), ("sim72", "72"), ("sim60", "60")]
RT_OPTS = [("rt32", ("region_threads", "32")), ("rt28", ("region_threads", "28")),
           ("s500", ("seconds", "500"))]

CLAIMS = {
 ("sim96", "rt32"): ("CLAIM | AG-78 | sim96 sim-мид (80-128, 0-клейм) + rt32 rt-верх WBP "
                     "(за 24, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim96", "rt28"): ("CLAIM | AG-78 | sim96 sim-мид (80-128, 0-клейм) + rt28 rt-мид WBP "
                     "(24-32, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim96", "s500"): ("CLAIM | AG-78 | sim96 sim-мид (80-128, 0-клейм) + s500 s-мид WBP "
                     "(300-750, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim72", "rt32"): ("CLAIM | AG-78 | sim72 sim-мид (64-80, 0-клейм) + rt32 rt-верх WBP "
                     "(за 24, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim72", "rt28"): ("CLAIM | AG-78 | sim72 sim-мид (64-80, 0-клейм) + rt28 rt-мид WBP "
                     "(24-32, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim72", "s500"): ("CLAIM | AG-78 | sim72 sim-мид (64-80, 0-клейм) + s500 s-мид WBP "
                     "(300-750, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim60", "rt32"): ("CLAIM | AG-78 | sim60 sim-мид (52-64, 0-клейм) + rt32 rt-верх WBP "
                     "(за 24, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim60", "rt28"): ("CLAIM | AG-78 | sim60 sim-мид (52-64, 0-клейм) + rt28 rt-мид WBP "
                     "(24-32, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim60", "s500"): ("CLAIM | AG-78 | sim60 sim-мид (52-64, 0-клейм) + s500 s-мид WBP "
                     "(300-750, 0-клейм): 1d/9000s + dp3v2 s42 | 2 POST"),
}
BR_A, BR_B = "swarm-526-78", "swarm-526-78b"


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
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-78 |" in ln:
            continue
        low = ln.lower()
        if cell.startswith("sim") and re.search(r"\b" + cell + r"\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell.startswith("rt") and re.search(r"\b" + cell + r"\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell == "s500" and re.search(r"\bs500\b|\b500s\b|300-750", low):
            return f"RACE s500: {ln[:100]}"
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
    sim = next((c for c, _ in SIM_OPTS if not race_scan(text, c)), None)
    rt = next((c for c, _ in RT_OPTS if not race_scan(text, c)), None)
    if not sim or not rt:
        sys.exit(f"FATAL: все клетки пивота заняты sim={sim} rt={rt}")
    return sim, rt, text


def leg_inputs(sim, rt):
    a = dict(BV2_CANON); a["simulation_distance"] = dict(SIM_OPTS)[sim]; a["seed"] = "526078"
    b = dict(WBP_CANON)
    key, val = dict(RT_OPTS)[rt]
    b[key] = val
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
    sim, rt, _ = pick_cells(tok)
    claim = CLAIMS[(sim, rt)]
    assert len(claim) <= 120
    print(f"cells: {sim} + {rt}", flush=True)
    board_append(tok, [claim], "board: AG-78 CLAIM sim-mid+rt-top WBP (wave-526)")
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
    ia, ib = leg_inputs(sim, rt)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, sim), (BR_B, "world-bench-parallel.yml", ib, rt))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(12)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [sim, rt], "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": sim,
                         "seed": "526078", "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": rt,
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-78/dispatch_526_78.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {sim}+{rt}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-78/dispatch_526_78.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-78/dispatch_526_78.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    sim, rt = p["cells"]
    lines = [
        (f"FACT | AG-78 | 2/2 204 @2171d6da+e49e8984: {ra} {sim} s526078 + {rb} {rt} "
         f"WBP QUEUED | api"),
        (f"DISP | AG-78 | {sim}-мид + {rt}-верх 2/2 queued @swarm-526-78[ab] "
         f"1d/9000s/dcp900 + dp3v2 s42; payload work/AG-78"),
        (f"PATCH_SUMMARY | AG-78 | files=work+claims/AG-78 | idea={sim} sim-мид 80-128 "
         f"+ {rt} rt-верх dose | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-78 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-78", "/home/z/c-crussty/work/AG-78"):
        shutil.copy(f"{RD}/work/AG-78/dispatch_526_78.json", f"{dst}/dispatch_526_78.json")
        shutil.copy(__file__, f"{dst}/act_526_78.py")
    shutil.copy(f"{RD}/claims/AG-78.md", "/home/z/c-crussty/claims/AG-78.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={sim}+{rt}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
