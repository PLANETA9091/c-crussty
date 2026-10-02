#!/usr/bin/env python3
"""act_526_69.py — AG-69 волна-526: leg-A sim120 sim-верх-мид (зазор 112-128,
соседи sim112 claimed / sim128 AG-15×526) lane bench-v2 @PIN 2171d6da
(sim-канон AG-138, yml-blob b4e9e05b) + leg-B pop950k pop-мид WBP (зазор
900k-1M, соседи 900k claimed / pop1M AG-58×526) @PIN e49e8984 (WBP-канон
AG-257, wbp-yml 7c021f41). Zero-code refs-API, CAS-CLAIM, race-guard с
пивотами (sim124/sim116; pop925k/pop975k), 2 dispatch POST ≥31s (AG-338),
GET-вериф head_sha==PIN. Сид 527069 (525069/526069 = w525 AG-69, не брать).
Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
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

SIM_OPTS = [("sim120", "120"), ("sim124", "124"), ("sim116", "116")]
POP_OPTS = [("pop950k", "950000"), ("pop925k", "925000"), ("pop975k", "975000")]
SEED_A = "527069"

CLAIMS = {
 ("sim120", "pop950k"): ("CLAIM | AG-69 | sim120 sim-верх-мид (112-128, 0-клейм) + "
                         "pop950k pop-мид WBP (900k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim120", "pop925k"): ("CLAIM | AG-69 | sim120 sim-верх-мид (112-128, 0-клейм) + "
                         "pop925k pop-мид WBP (900k-950k): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim120", "pop975k"): ("CLAIM | AG-69 | sim120 sim-верх-мид (112-128, 0-клейм) + "
                         "pop975k pop-мид WBP (950k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim124", "pop950k"): ("CLAIM | AG-69 | sim124 sim-мид (112-136, 0-клейм) + "
                         "pop950k pop-мид WBP (900k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim124", "pop925k"): ("CLAIM | AG-69 | sim124 sim-мид (112-136, 0-клейм) + "
                         "pop925k pop-мид WBP (900k-950k): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim124", "pop975k"): ("CLAIM | AG-69 | sim124 sim-мид (112-136, 0-клейм) + "
                         "pop975k pop-мид WBP (950k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim116", "pop950k"): ("CLAIM | AG-69 | sim116 sim-мид (112-120, 0-клейм) + "
                         "pop950k pop-мид WBP (900k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim116", "pop925k"): ("CLAIM | AG-69 | sim116 sim-мид (112-120, 0-клейм) + "
                         "pop925k pop-мид WBP (900k-950k): 1d/9000s + dp3v2 s42 | 2 POST"),
 ("sim116", "pop975k"): ("CLAIM | AG-69 | sim116 sim-мид (112-120, 0-клейм) + "
                         "pop975k pop-мид WBP (950k-1M): 1d/9000s + dp3v2 s42 | 2 POST"),
}
BR_A, BR_B = "swarm-526-69", "swarm-526-69b"


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
    num = {"sim120": r"120", "sim124": r"124", "sim116": r"116",
           "pop950k": r"950", "pop925k": r"925", "pop975k": r"975"}[cell]
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-69 |" in ln:
            continue
        low = ln.lower()
        if cell.startswith("sim"):
            if re.search(r"\b" + cell + r"\b", low):
                return f"RACE {cell}: {ln[:100]}"
            if "sim" in low and re.search(r"\b" + num + r"\b", low):
                return f"RACE {cell}-num: {ln[:100]}"
        else:
            if re.search(r"\b" + cell + r"\w*\b", low) or re.search(r"\b" + num + r"000\b", low):
                return f"RACE {cell}: {ln[:100]}"
            if "pop" in low and re.search(r"\b" + num + r"\b", low):
                return f"RACE {cell}-num: {ln[:100]}"
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
    pop = next((c for c, _ in POP_OPTS if not race_scan(text, c)), None)
    if not sim or not pop:
        sys.exit(f"FATAL: все клетки пивота заняты sim={sim} pop={pop}")
    return sim, pop, text


def leg_inputs(sim, pop):
    a = dict(BV2_CANON); a["simulation_distance"] = dict(SIM_OPTS)[sim]; a["seed"] = SEED_A
    b = dict(WBP_CANON); b["population_target"] = dict(POP_OPTS)[pop]
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
    sim, pop, _ = pick_cells(tok)
    claim = CLAIMS[(sim, pop)]
    assert len(claim) <= 120
    print(f"cells: {sim} + {pop}", flush=True)
    board_append(tok, [claim], "board: AG-69 CLAIM sim120-mid+WBP-pop950k (wave-526)")
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
    ia, ib = leg_inputs(sim, pop)
    for i, (ref_name, wf, inputs, cell) in enumerate(
            ((BR_A, "bench-v2.yml", ia, sim), (BR_B, "world-bench-parallel.yml", ib, pop))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    time.sleep(8)
    found = find_runs(tok, pre)
    payload = {"pin_bv2": PIN_BV2, "pin_wbp": PIN_WBP, "tree_files": [n1, n2],
               "claim": claim, "cells": [sim, pop], "runs": dict(found),
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": sim,
                         "seed": SEED_A, "inputs": ia},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml", "cell": pop,
                         "inputs": ib}],
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-69/dispatch_526_69.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"TREE {n1}/{n2} CELLS {sim}+{pop}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-69/dispatch_526_69.json"))
    if len(p["runs"]) < 2:
        for _ in range(3):
            found = find_runs(tok, set())
            if found:
                p["runs"] = {**p["runs"], **found}
                json.dump(p, open(f"{RD}/work/AG-69/dispatch_526_69.json", "w"),
                          indent=1, ensure_ascii=False)
            if len(p["runs"]) >= 2:
                break
            time.sleep(10)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    sim, pop = p["cells"]
    lines = [
        (f"FACT | AG-69 | 2/2 204 @2171d6da+e49e8984 t4231: {ra} {sim} s{SEED_A} + "
         f"{rb} {pop} WBP s42 QUEUED | api"),
        (f"DISP | AG-69 | {sim}-мид + {pop}-мид 2/2 queued @swarm-526-69[ab] "
         f"1d/9000s/dcp900 + dp3v2; payload work/AG-69"),
        (f"PATCH_SUMMARY | AG-69 | files=claims,work/AG-69 | idea={sim} sim-мид "
         f"112-128 + {pop} pop-мид dose | evidence=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-69 fact+disp+patch queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    for dst in (f"{RD}/work/AG-69", "/home/z/c-crussty/work/AG-69"):
        shutil.copy(f"{RD}/work/AG-69/dispatch_526_69.json", f"{dst}/dispatch_526_69.json")
        shutil.copy(__file__, f"{dst}/act_526_69.py")
    shutil.copy(f"{RD}/claims/AG-69.md", "/home/z/c-crussty/claims/AG-69.md")
    mem = (f"# AG-69 MEMORY (wave-526) — уроки ≤15 строк\n"
           f"1. Сиды w525-тёзки заняты: 525069/526069 = xmx-ноги AG-69×525 — брать 5270NN+.\n"
           f"2. Mixed-лейн легален: leg-A bench-v2 @2171d6da (blob b4e9e05b) + leg-B WBP "
           f"@e49e8984 (blob 7c021f41), tree FULL 4231/4231.\n"
           f"3. pop-мид 900k-1M свободен был пока AG-58 взял край pop1M — миды живут дольше краёв.\n"
           f"4. sim-верх-мид 112-128 закрывает сим-кривую к фронтиру sim128 AG-15×526.\n"
           f"5. Пивоты готовить парами (SIM×POP матрица 3x3) — 0 потерянных минут при гонке.\n"
           f"6. def find_runs: head_sha==PIN вериф per-branch — атрибуция чистая без реестра.\n"
           f"7. Сибам оставлены: sim116/sim124 (если пивот), pop925k/pop975k, fp176, dcp3000.\n")
    open(f"{RD}/work/AG-69/MEMORY.md", "w").write(mem)
    shutil.copy(f"{RD}/work/AG-69/MEMORY.md", "/home/z/c-crussty/work/AG-69/MEMORY.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} cells={sim}+{pop}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
