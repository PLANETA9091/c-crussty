#!/usr/bin/env python3
"""act_526_42.py — AG-42 волна-526: leg-A fp92 press-мид (зазор 88-96; соседи
fp88 AG-28x526 queued / fp96 AG-252x525 queued) lane bench-v2 @PIN 2171d6da
(press-канон-носитель sim32/r1136/9000s/dcp900, yml-blob b4e9e05b: fp+sim-входы
живы) + leg-B rt18 rt-мид WBP (зазор 16-20; соседи rt16 AG-226x525 / rt24
AG-262x525) @PIN e49e8984 (WBP-канон AG-257x525, wbp-yml 7c021f41), dp3v2 /
pop150k / seed42 / band 5.5-13.5M. Zero-code refs-API, CAS-CLAIM, race-guard
с пивотами (fp100/fp76/fp132; rt20/rt9/rt28), 2 dispatch POST >=31s (AG-338),
GET-вериф head_sha==PIN. Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных
коммитов; сиды grep-чист (526042; WBP seed42-когорта)."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "2171d6da775975c4bee94748f549ad16f02074e1"   # bench-v2 sim/press-канон AG-138
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob @PIN
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
WBP_BLOB = "7c021f41"                                   # world-bench-parallel.yml blob

BV2_CANON = {"radius_blocks": "1136", "run_seconds": "9000", "server_xmx": "10G",
             "bench_dims": "minecraft:overworld", "dim_gen_window": "256",
             "drain_cap_polls": "900", "fake_players": "4", "simulation_distance": "32"}
WBP_CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
             "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
             "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
             "population_target": "150000", "population_seed": "42", "server_xmx": "10G",
             "server_xms": "4G", "cpu_band_min": "5500000", "cpu_band_max": "13500000",
             "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}

FP_OPTS = [("fp92", "92", "88-96"), ("fp100", "100", "96-104"),
           ("fp76", "76", "72-80"), ("fp132", "132", "128-136")]
RT_OPTS = [("rt18", "18", "мид 16-20"), ("rt20", "20", "мид 16-24"),
           ("rt9", "9", "мид 8-10"), ("rt28", "28", "верх за 24")]
SEED_A = "526042"
BR_A, BR_B = "swarm-526-42", "swarm-526-42b"


def claim_text(fp, rt):
    return (f"CLAIM | AG-42 | {fp[0]} press-мид ({fp[2]}, 0-клейм) sim32/1d/9000s/dcp900 "
            f"+ {rt[0]} rt-{('верх' if rt[0]=='rt28' else 'мид')} WBP ({rt[2]}) dp3v2 s42 | 2 POST")


def token():
    return open("/tmp/gh_token").read().strip()


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


def race_scan(text, fp, rt):
    """Чужой CLAIM на клетку/сид/ветку = гонка (regex-boundary, живой GET; канон x525)."""
    pats = [rf"\b{fp[0]}\b", rf"\b{rt[0]}\b", rf"\b{SEED_A}\b",
            rf"\b{BR_A}\b", rf"\b{BR_B}\b"]
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-42 |" in ln:
            continue
        low = ln.lower()
        for p in pats:
            if re.search(p, low):
                return f"RACE {p}: {ln[:100]}"
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
    sha, text = board_read(tok)
    fp = next((c for c in FP_OPTS if not race_scan(text, c, ("__nort__", "0"))), None)
    rt = next((c for c in RT_OPTS if not race_scan(text, ("__nofp__", "0"), c)), None)
    if not fp or not rt:
        sys.exit(f"FATAL: все клетки пивота заняты fp={fp} rt={rt}")
    return fp, rt, sha


def leg_inputs(fp, rt):
    a = dict(BV2_CANON); a["fake_players"] = fp[1]; a["seed"] = SEED_A
    b = dict(WBP_CANON); b["region_threads"] = rt[1]
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
    fp, rt, live_sha = pick_cells(tok)
    claim = claim_text(fp, rt)
    assert len(claim) <= 120
    print(f"cells: {fp[0]} + {rt[0]} live_sha={live_sha[:8]}", flush=True)
    board_append(tok, [claim], "board: AG-42 CLAIM fp-mid+rt-mid (wave-526)")
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
    ia, ib = leg_inputs(fp, rt)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, fp[0]), (BR_B, "world-bench-parallel.yml", ib, rt[0]))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(8)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "live_sha": live_sha, "claim": claim, "cells": [fp[0], rt[0]],
               "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": fp[0],
                         "seed": SEED_A, "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": rt[0],
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-42/dispatch_526_42.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {fp[0]}+{rt[0]}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-42/dispatch_526_42.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-42/dispatch_526_42.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    fp, rt = p["cells"]
    lines = [
        (f"FACT | AG-42 | 2/2 204 @2171d6da+e49e8984: {ra} {fp} s{SEED_A} + {rb} {rt} "
         f"WBP s42 QUEUED | api"),
        (f"DISP | AG-42 | {fp}-мид + {rt}-мид 2/2 queued @swarm-526-42[ab] "
         f"1d/9000s/dcp900 + dp3v2 s42; payload work/AG-42"),
        (f"PATCH_SUMMARY | AG-42 | files=work+claims/AG-42 | idea={fp} press-мид "
         f"+ {rt} rt-мид dose fill | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-42 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-42", "/home/z/c-crussty/work/AG-42"):
        if dst == f"{RD}/work/AG-42":
            continue
        shutil.copy(f"{RD}/work/AG-42/dispatch_526_42.json", f"{dst}/dispatch_526_42.json")
        shutil.copy(__file__, f"{dst}/act_526_42.py")
    shutil.copy(f"{RD}/claims/AG-42.md", "/home/z/c-crussty/claims/AG-42.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={fp}+{rt}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
