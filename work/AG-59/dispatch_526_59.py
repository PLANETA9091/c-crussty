#!/usr/bin/env python3
"""dispatch_526_59.py — AG-59 волна-526: доводка deficit-карты AG-18 до конца.
Остаток карты (w2240/w5376/rt20/pop750k/fp72 взяты сибами по живой доске):
  leg-A sim96 sim-верх bench-v2 (за 80, зазор 80-128; соседи sim80 AG-40,
       sim64 AG-267) @PIN 2171d6da (BV2-канон AG-138, yml b4e9e05b), seed 526059;
  leg-B s4500 seconds-верх WBP (за 3000, соседи s3000 AG-258, s2400 AG-269)
       @PIN e49e8984 (WBP-канон AG-257, yml 7c021f41), dp3v2 seed42 band 5.5-13.5M.
Ценность: (1) sim-кривая верхний брекет — где TPS@9000s ломается от sim-дистанции;
(2) soak-стена pop150k — деградация TPS к 75-й минуте (drift-лестница 300..4500s).
Zero-code: refs-API @PIN после API tree-чека ≥3200 FULL, 2 dispatch POST ≥31s
(FAIL AG-338), CAS-CLAIM по живому GET, race-guard с пивотами (sim88/104/112;
s3600/s4000). Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
ME = "AG-59"
RD = "/home/z/rounds/ROUND-526"
PIN_BV2 = "2171d6da775975c4bee94748f549ad16f02074e1"   # bench-v2 sim-канон AG-138
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob @PIN
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
WBP_BLOB = "7c021f41"                                   # world-bench-parallel.yml blob
DP3V2 = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
         "v484-dp3v2/stz3v2-fixture.zip")
BR_A, BR_B = "swarm-526-59", "swarm-526-59b"
SEED = "526059"

SIM_OPTS = [("sim96", "96"), ("sim88", "88"), ("sim104", "104"), ("sim112", "112")]
S_OPTS = [("s4500", "4500"), ("s3600", "3600"), ("s4000", "4000")]

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

SIM_LABEL = {"sim96": "sim-верх bv2 (за 80, deficit AG-18)",
             "sim88": "sim-мид (80-96)", "sim104": "sim-мид (96-112)",
             "sim112": "sim-мид (96-128)"}
S_LABEL = {"s4500": "seconds-верх WBP (за 3000)",
           "s3600": "seconds-мид WBP (3000-4500)",
           "s4000": "seconds-мид WBP (3600-4500)"}


def claim_text(sim, sec):
    return (f"CLAIM | {ME} | {sim} {SIM_LABEL[sim]} + {sec} {S_LABEL[sec]}: "
            f"zero-code | 2 POST")


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "ag-59-wave526"})
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
    """CAS PUT append-only, 6 попыток (канон AG-91/197/208)."""
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
        if not ln.startswith("CLAIM") or f"{ME} |" in ln:
            continue
        low = ln.lower()
        if cell.startswith("sim") and re.search(r"\b" + cell + r"\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell.startswith("sim") and cell == "sim96" and "за 80" in low:
            return f"RACE {cell}: {ln[:100]}"
        if cell == "s4500" and (re.search(r"\bs4500\b|\b4500s\b", low)
                                or "за 3000" in low and "4500" in low):
            return f"RACE s4500: {ln[:100]}"
        if cell == "s3600" and re.search(r"\bs3600\b|\b3600s\b|3000-4500", low):
            return f"RACE s3600: {ln[:100]}"
        if cell == "s4000" and re.search(r"\bs4000\b|\b4000s\b", low):
            return f"RACE s4000: {ln[:100]}"
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
        sys.exit(f"FATAL: PIN-вериф провален {pin[:8]} (tree={n} trunc={trunc} "
                 f"yml={yblob[:8]})")
    return n


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
    _, text = board_read(tok)
    sim = next((c for c, _ in SIM_OPTS if not race_scan(text, c)), None)
    sec = next((c for c, _ in S_OPTS if not race_scan(text, c)), None)
    if not sim or not sec:
        sys.exit(f"FATAL: все клетки пивота заняты sim={sim} s={sec}")
    claim = claim_text(sim, sec)
    assert len(claim) <= 120, len(claim)
    print(f"cells: {sim} + {sec}", flush=True)
    board_append(tok, [claim], "board: AG-59 CLAIM sim-верх bv2 + seconds-верх WBP "
                               "(wave-526)")
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
    ia = dict(BV2_CANON)
    ia["simulation_distance"] = dict(SIM_OPTS)[sim]
    ia["seed"] = SEED
    ib = dict(WBP_CANON)
    ib["seconds"] = dict(S_OPTS)[sec]
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, sim),
             (BR_B, "world-bench-parallel.yml", ib, sec))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(8)
    found = find_runs(tok, pre)
    if len(found) < 2:
        print("WARN: не оба рана видимы — вериф повторит --finalize", flush=True)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [sim, sec], "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": sim,
                         "seed": SEED, "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": sec,
                         "inputs": ib}],
               "canon_vector": "bv2: r1136/9000s/dcp900/dgw256/fp4/xmx10G/1d "
                               "(delta sim) | wbp: 640/300s*/fp4/guard1/gc3/ic1/fd1/"
                               "rt4/bc1/pop150k/pseed42/10G/4G/dp3v2/band5.5-13.5M "
                               "(*delta seconds)",
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/{ME}/dispatch_526_59.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {sim}+{sec}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/{ME}/dispatch_526_59.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set(), pages=4)
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/{ME}/dispatch_526_59.json", "w"),
                      indent=1, ensure_ascii=False)
    ids = p["runs"]
    ra = ids.get(BR_A, 0)
    rb = ids.get(BR_B, 0)
    sim, sec = p["cells"]
    lines = [
        (f"FACT | {ME} | 2/2 204 @2171d6da+e49e8984: {ra} {sim} s{SEED} + {rb} "
         f"{sec} s42 WBP QUEUED | api"),
        (f"DISP | {ME} | {sim}-верх bv2 + {sec} seconds-верх WBP 2/2 queued "
         f"@swarm-526-59[ab]; payload work/AG-59 | 2/2 204"),
        (f"PATCH_SUMMARY | {ME} | files=work+claims/AG-59 | idea={sim}+{sec} "
         f"deficit-map AG-18 fill | evidence=2/2 queued"),
    ]
    ok = board_append(tok, lines, "board: AG-59 fact+disp+patch sim96+s4500 queued "
                                  "(wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/{ME}", f"/home/z/c-crussty/work/{ME}"):
        shutil.copy(f"{RD}/work/{ME}/dispatch_526_59.json", f"{dst}/dispatch_526_59.json")
        shutil.copy(__file__, f"{dst}/dispatch_526_59.py")
    shutil.copy(f"{RD}/claims/{ME}/prereg.md", f"{RD}/claims/{ME}.md")
    shutil.copy(f"{RD}/claims/{ME}/prereg.md", f"/home/z/c-crussty/claims/{ME}.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={sim}+{sec}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
