#!/usr/bin/env python3
"""act_526_60.py — AG-60 волна-526: leg-A dcp1800 dcp-мид (1500-2400, соседи dcp1500-канон /
dcp2400 AG-38; каскад-риск AG-80 C1) lane bench-v2 @PIN 2171d6da (yml-blob b4e9e05b,
AG-138-канон) + leg-B pop1M pop-фронт WBP за 750k (низ pop750k AG-40) @PIN e49e8984
(WBP-канон AG-257, wbp-yml 7c021f41). Zero-code refs-API, CAS-CLAIM, race-guard с
пивотами (xmx56G/fp176/dcp3000; pop600k/s1875/rt18), 2 dispatch POST ≥31s (AG-338),
GET-вериф head_sha==PIN. Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов.
Seed 527060 (grep=0 доска+rounds; 526060 занят wave-525 AG-60 leg-2 — урок AG-15)."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "2171d6da775975c4bee94748f549ad16f02074e1"   # bench-v2 sim/dcp-канон AG-138
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob @PIN
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
WBP_BLOB = "7c021f41"                                   # world-bench-parallel.yml blob
SEED = "527060"

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

A_OPTS = ["dcp1800", "xmx56g", "fp176", "dcp3000"]
B_OPTS = ["pop1M", "pop600k", "s1875", "rt18"]
A_DESC = {"dcp1800": "dcp1800 dcp-мид 1500-2400", "xmx56g": "xmx56G xmx-фронт за 48G",
          "fp176": "fp176 press-верх за 160", "dcp3000": "dcp3000 dcp-верх за 2400"}
B_DESC = {"pop1M": "pop1M pop-фронт WBP за 750k", "pop600k": "pop600k pop-мид WBP 500-750k",
          "s1875": "s1875 s-мид WBP 1500-2250", "rt18": "rt18 rt-мид WBP 16-20"}
A_SHORT = {"dcp1800": "dcp1800 dcp-мид", "xmx56g": "xmx56G фронт",
           "fp176": "fp176 press-верх", "dcp3000": "dcp3000 dcp-верх"}
B_SHORT = {"pop1M": "pop1M фронт WBP", "pop600k": "pop600k WBP-мид",
           "s1875": "s1875 s-мид WBP", "rt18": "rt18 rt-мид WBP"}
RACE = {"dcp1800": r"\bdcp1800\b|1500-2400", "xmx56g": r"\bxmx56g\b|за 48g",
        "fp176": r"\bfp176\b", "dcp3000": r"\bdcp3000\b|за 2400",
        "pop1M": r"\bpop1m\b|\bpop1000000\b|\bpop1000k\b|за 750k",
        "pop600k": r"\bpop600k\b|\b600000\b", "s1875": r"\bs1875\b|\b1875s\b|1500-2250",
        "rt18": r"\brt18\b"}
BR_A, BR_B = "swarm-526-60", "swarm-526-60b"


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
    """Чужой CLAIM на клетку = гонка (regex-boundary, живой GET; канон x525/AG-40)."""
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-60 |" in ln:
            continue
        low = ln.lower()
        if re.search(RACE[cell], low):
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
    assert SEED not in text, "seed collision"
    a = next((c for c in A_OPTS if not race_scan(text, c)), None)
    b = next((c for c in B_OPTS if not race_scan(text, c)), None)
    if not a or not b:
        sys.exit(f"FATAL: все клетки пивота заняты A={a} B={b}")
    return a, b


def leg_inputs(a, b):
    ia = dict(BV2_CANON)
    if a.startswith("dcp"):
        ia["drain_cap_polls"] = a[3:]
    elif a == "xmx56g":
        ia["server_xmx"] = "56G"
    elif a == "fp176":
        ia["fake_players"] = "176"
        ia["simulation_distance"] = "32"
    ia["seed"] = SEED
    ib = dict(WBP_CANON)
    if b == "pop1M":
        ib["population_target"] = "1000000"
    elif b == "pop600k":
        ib["population_target"] = "600000"
    elif b == "s1875":
        ib["seconds"] = "1875"
    elif b == "rt18":
        ib["region_threads"] = "18"
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
    claim = f"CLAIM | AG-60 | {A_DESC[a]} + {B_DESC[b]}: 1d/9000s + dp3v2 s42 | 2 POST"
    assert len(claim) <= 120
    print(f"cells: {a} + {b}", flush=True)
    board_append(tok, [claim], "board: AG-60 CLAIM dcp-mid+WBP-pop-front (wave-526)")
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
        time.sleep(31)
    ia, ib = leg_inputs(a, b)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, a), (BR_B, "world-bench-parallel.yml", ib, b))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(8)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [a, b], "seed": SEED, "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": a,
                         "seed": SEED, "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": b,
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-60/dispatch_526_60.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {a}+{b}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-60/dispatch_526_60.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-60/dispatch_526_60.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    a, b = p["cells"]
    lines = [
        (f"FACT | AG-60 | 2/2 204 @2171d6da+e49e8984: {ra} {a} s{SEED} + {rb} {b} "
         f"WBP QUEUED | api"),
        (f"DISP | AG-60 | {A_SHORT[a]} + {B_SHORT[b]} 2/2 queued @swarm-526-60[ab] "
         f"1d/9000s + dp3v2 s42; payload work/AG-60"),
        (f"PATCH_SUMMARY | AG-60 | files=work+claims/AG-60 | idea={A_SHORT[a]}+"
         f"{B_SHORT[b]} dose | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-60 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-60", "/home/z/c-crussty/work/AG-60"):
        shutil.copy(f"{RD}/work/AG-60/dispatch_526_60.json", f"{dst}/dispatch_526_60.json")
        shutil.copy(__file__, f"{dst}/act_526_60.py")
    shutil.copy(f"{RD}/claims/AG-60.md", "/home/z/c-crussty/claims/AG-60.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={a}+{b}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
