#!/usr/bin/env python3
"""act_526_50.py — AG-50 волна-526: leg-A sim96 sim-мид (зазор 80-128, deficit-карта
AG-18, 0-клейм) lane bench-v2 @PIN 32a448da (канон-yml b4e9e05b verbatim, G4-fix,
4233 файлов) + leg-B pop600k pop-мид WBP (500-750k, 0-клейм, когорта seed42 между
400k AG-2 и 750k AG-40) @PIN e9bb6dc5 (t4241, wbp-yml 7c021f41). Zero-code refs-API,
CAS-CLAIM, race-guard с пивотами (sim112/sim72/xmx32/dcp500; pop100k/s1800/s1200/rt8),
2 dispatch POST ≥31s, GET-вериф head_sha==PIN. Д1-Д5: API-only, 0 ворктри, 0 gc/prune,
0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "32a448dabf46a0b9ef8e85f13a202ab98ab81bd5"   # bench-v2 G4-fix, yml verbatim AG-1
BV2_BLOB = "b4e9e05b"                                   # bench-v2.yml blob (канон 2171d6da)
PIN_WBP = "e9bb6dc5a39f863ccee24c273793adaf3a686b18"   # WBP-канон t4241 (AG-20)
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

BV2_OPTS = [("sim96", "96"), ("sim112", "112"), ("sim72", "72"),
            ("xmx32", "32"), ("dcp500", "500")]
WBP_OPTS = [("pop600k", "600000"), ("pop100k", "100000"),
            ("s1800", "1800"), ("s1200", "1200"), ("rt8", "8")]
# клетка -> (ключ bv2-inputs, значение) / (ключ wbp-inputs, значение)
BV2_FIELD = {"sim96": "simulation_distance", "sim112": "simulation_distance",
             "sim72": "simulation_distance", "xmx32": "server_xmx", "dcp500": "drain_cap_polls"}
WBP_FIELD = {"pop600k": "population_target", "pop100k": "population_target",
             "s1800": "seconds", "s1200": "seconds", "rt8": "region_threads"}
BV2_DESC = {"sim96": "sim96 sim-мид (80-128, deficit AG-18)", "sim112": "sim112 sim-мид (96-128)",
            "sim72": "sim72 sim-мид (64-80)", "xmx32": "xmx32 xmx-мид (30-34)",
            "dcp500": "dcp500 dcp-мид (400-600)"}
WBP_DESC = {"pop600k": "pop600k pop-мид WBP (500-750k)", "pop100k": "pop100k pop-мид WBP (62.5-125k)",
            "s1800": "s1800 seconds-мид WBP (1650-1950)", "s1200": "s1200 seconds-мид WBP (1050-1350)",
            "rt8": "rt8 rt-мид WBP (6-10)"}
CLAIMS = {
    (b, w): (f"CLAIM | AG-50 | {BV2_DESC[b]} + {WBP_DESC[w]}: "
             f"1d/9000s/dcp900 + dp3v2 s42 | 2 POST")
    for b, _ in BV2_OPTS for w, _ in WBP_OPTS
}
BR_A, BR_B = "swarm-526-50", "swarm-526-50b"


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
        if not ln.startswith("CLAIM") or "AG-50 |" in ln:
            continue
        low = ln.lower()
        if cell.startswith("sim") and re.search(r"\b" + cell + r"\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell in ("xmx32",) and re.search(r"\bxmx32g?\b|\bxmx-32\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell in ("dcp500",) and re.search(r"\bdcp500\b|\bdcp-500\b", low):
            return f"RACE {cell}: {ln[:100]}"
        if cell == "pop600k" and re.search(r"\bpop600\w*\b|\b600000\b", low):
            return f"RACE pop600k: {ln[:100]}"
        if cell == "pop100k" and re.search(r"\bpop100\w*\b|\b100000\b", low):
            return f"RACE pop100k: {ln[:100]}"
        if cell == "s1800" and re.search(r"\bs1800\b|\b1800s\b|1650-1950", low):
            return f"RACE s1800: {ln[:100]}"
        if cell == "s1200" and re.search(r"\bs1200\b|\b1200s\b|1050-1350", low):
            return f"RACE s1200: {ln[:100]}"
        if cell == "rt8" and re.search(r"\brt8\b(?!s)", low):
            return f"RACE rt8: {ln[:100]}"
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
    a = dict(BV2_CANON); a[BV2_FIELD[bv2]] = dict(BV2_OPTS)[bv2]; a["seed"] = "529050"
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
    board_append(tok, [claim], "board: AG-50 CLAIM sim96-mid+WBP-pop600k (wave-526)")
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
                         "seed": "529050", "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": wbp,
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-50/dispatch_526_50.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {bv2}+{wbp}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-50/dispatch_526_50.json"))
    if len(p["runs"]) < 2:
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-50/dispatch_526_50.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    bv2, wbp = p["cells"]
    lines = [
        (f"FACT | AG-50 | 2/2 204 @32a448da+e9bb6dc5: {ra} {bv2} s529050 + {rb} {wbp} "
         f"WBP QUEUED | api"),
        (f"DISP | AG-50 | {bv2}-мид + {wbp}-мид 2/2 queued @swarm-526-50[ab] "
         f"1d/9000s + dp3v2 s42; payload work/AG-50"),
        (f"PATCH_SUMMARY | AG-50 | files=claims+work/AG-50 | idea={bv2} deficit-fill "
         f"+ {wbp} pop-мид dose | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-50 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-50", "/home/z/c-crussty/work/AG-50"):
        shutil.copy(f"{RD}/work/AG-50/dispatch_526_50.json", f"{dst}/dispatch_526_50.json")
        shutil.copy(__file__, f"{dst}/act_526_50.py")
    shutil.copy(f"{RD}/claims/AG-50.md", "/home/z/c-crussty/claims/AG-50.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={bv2}+{wbp}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
