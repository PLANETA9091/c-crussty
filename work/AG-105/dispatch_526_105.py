#!/usr/bin/env python3
"""dispatch_526_105.py — AG-105 x526: leg-A fp3 WBP player-load мид (зазор 2-6 за
canon fp4; WBP fp-когорта 0/1/2/6/8/16 занята, fp3 0-клейм) @swarm-526-105 = live-tip
6bac5590 (tree-4256 FULL), world-bench-parallel.yml, canon r640/s300/gc3/rt4/dp3v2/
pop150k, seed 531105. leg-B dcp1600 dcp-мид-верх (1500-2400, 0-клейм; 1950=AG-48,
2400=AG-38) @swarm-526-105b = a9ff088f (dcp-канон-носитель x525/526, tree-4231,
bench-v2.yml blob 0049e34a), r1136/s9000/dgw256/xmx10G/1d-ow, seed 532105.
Сиды 531105/532105: grep живой доски + rounds 0-клейм. 2-й POST через 31s (FAIL
AG-338), ≤2 POST, refs-API FULL-sha, CAS-board (канон AG-278)."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
PIN_TIP = "6bac55905ca0fcc8365cc4aca77e7b281f62881e"      # live master tip, tree 4256
PIN_DCP = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"      # dcp-канон-носитель, tree 4231
WF_WBP = "world-bench-parallel.yml"
WF_BV2 = "bench-v2.yml"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
CLAIM = ("CLAIM | AG-105 | fp3 WBP player-load мид (зазор 2-6, 0-клейм) + "
         "dcp1600 dcp-мид-верх (1500-2400) bench-v2 | 2 POST")
LEGS = [
    {"branch": "swarm-526-105", "pin": PIN_TIP, "wf": WF_WBP, "tag": "fp3",
     "seed": "531105",
     "inputs": {"radius": "640", "seconds": "300", "fake_players": "3",
                "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
                "flush_diet": "1", "fluid_dirty": "0", "fluid_bitmask": "0",
                "region_threads": "4", "batch_collector": "1",
                "datapack_url": DP3V2, "skip_store_bb": "0", "region_steal": "0",
                "bu_defer": "0", "population_target": "150000",
                "population_seed": "531105", "server_xmx": "10G", "server_xms": "4G",
                "cpu_band_min": "5500000", "cpu_band_max": "13500000"}},
    {"branch": "swarm-526-105b", "pin": PIN_DCP, "wf": WF_BV2, "tag": "dcp1600",
     "seed": "532105",
     "inputs": {"radius_blocks": "1136", "run_seconds": "9000", "seed": "532105",
                "server_xmx": "10G", "bench_dims": "minecraft:overworld",
                "dim_gen_window": "256", "drain_cap_polls": "1600"}},
]
assert len(CLAIM) <= 120, len(CLAIM)


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
    """CAS PUT append-only, 6 попыток (канон AG-91/197/208/278)."""
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


def race_check(text):
    """Чужие CLAIM на мои клетки = стоп (живой GET, канон AG-211/278)."""
    for ln in text.splitlines():
        if "AG-105" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        if re.search(r"\bfp3\b", low) or "wbp fp-ось" in low or "fp-ось wbp" in low:
            return f"RACE fp3: {ln[:100]}"
        if re.search(r"\bdcp1600\b", low) or "dcp-ось" in low or "dcp-доза" in low:
            return f"RACE dcp1600: {ln[:100]}"
    return None


def find_runs(tok, pre_ids, pages=3):
    found = {}
    want = {L["branch"] for L in LEGS}
    for page in range(1, pages + 1):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in want and run["id"] not in pre_ids:
                pin = next(L["pin"] for L in LEGS if L["branch"] == b)
                ok = run["head_sha"] == pin
                print(f"RUN {run['id']} {b} sha={run['head_sha'][:8]} "
                      f"{run['status']} match={'OK' if ok else 'MISMATCH'}", flush=True)
                if ok:
                    found[b] = run["id"]
        if len(found) >= len(LEGS):
            break
    return found


def phase_run():
    tok = token()
    # --- 1. PIN-вериф: tree FULL + workflow-блобы (канон AG-208/239/278) ---
    pins = {L["pin"] for L in LEGS}
    for pin in pins:
        c = api(tok, f"/repos/{REPO}/commits/{pin}")
        tree = api(tok, f"/repos/{REPO}/git/trees/{c['commit']['tree']['sha']}?recursive=1")
        n = len(tree.get("tree", []))
        trunc = tree.get("truncated", False)
        yblobs = {x["path"].split("/")[-1]: x["sha"][:8] for x in tree.get("tree", [])
                  if x["path"].startswith(".github/workflows/")}
        print(f"pin={pin[:8]} tree={n} trunc={trunc} "
              f"wbp={yblobs.get(WF_WBP, '?')} bv2={yblobs.get(WF_BV2, '?')}", flush=True)
        if trunc or n < 3200:
            sys.exit(f"FATAL pin {pin[:8]} tree {n}")
    # --- 2. 0-клейм чек по ЖИВОЙ доске ---
    _, text = board_read(tok)
    race = race_check(text)
    if race:
        print(race, flush=True)
        sys.exit(1)
    print("0-клейм OK: fp3 + dcp1600 свободны (живой GET)", flush=True)
    # --- 3. CLAIM ДО РАБОТЫ ---
    board_append(tok, [CLAIM], "board: AG-105 CLAIM fp3 WBP + dcp1600 (wave-526)")
    # --- 4. снапшот ран-идов + refs zero-code (API, FULL-sha) ---
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for L in LEGS:
        r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{L['branch']}", "sha": L["pin"]})
        if not r.get("ref"):
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{L['branch']}")
            if g.get("object", {}).get("sha", "") != L["pin"]:
                sys.exit(f"FATAL: ref {L['branch']} exists with other sha")
        ver = api(tok, f"/repos/{REPO}/git/ref/heads/{L['branch']}")["object"]["sha"]
        print(f"branch {L['branch']}: sha={ver[:8]} match={ver == L['pin']}", flush=True)
        if ver != L["pin"]:
            sys.exit(f"FATAL: ref sha mismatch {L['branch']}")
    # --- 5. Диспатчи (≤2), 2-й через 31s (FAIL AG-338) ---
    for i, L in enumerate(LEGS):
        api(tok, f"/repos/{REPO}/actions/workflows/{L['wf']}/dispatches",
            method="POST", data={"ref": L["branch"], "inputs": L["inputs"]})
        print(f"dispatch {L['branch']} {L['tag']} seed={L['seed']} wf={L['wf']}: POST",
              flush=True)
        if i == 0:
            time.sleep(31)
    # --- 6. GET-вериф run-ids (×3 страницы канон AG-197) ---
    time.sleep(8)
    found = find_runs(tok, pre)
    if len(found) < len(LEGS):
        print("WARN: не оба рана видимы — вериф повторит --finalize", flush=True)
    payload = {"pins": {"wbp": PIN_TIP, "bv2": PIN_DCP},
               "tree_files": {"wbp": 4256, "bv2": 4231},
               "claim": CLAIM, "runs": dict(found),
               "legs": [{"branch": L["branch"], "wf": L["wf"], "tag": L["tag"],
                         "seed": L["seed"]} for L in LEGS]}
    json.dump(payload, open(f"{RD}/work/AG-105/dispatch_526_105.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PINS wbp={PIN_TIP[:8]} bv2={PIN_DCP[:8]}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-105/dispatch_526_105.json"))
    if len(p["runs"]) < len(LEGS):
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-105/dispatch_526_105.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get("swarm-526-105", 0)
    rb = p["runs"].get("swarm-526-105b", 0)
    lines = [
        (f"FACT | AG-105 | 2/2 204 @6bac5590/a9ff088f: {ra} fp3 s531105 WBP + "
         f"{rb} dcp1600 s532105 QUEUED | api"),
        (f"DISP | AG-105 | fp3 WBP + dcp1600 bv2 2/2 queued @swarm-526-105[ab] "
         f"r640/s300 + r1136/s9000; work/AG-105 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-105 | files=claims,work/AG-105 | idea=fp3 player-load "
         f"mid + dcp1600 drain-sens | ev=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-105 fact+disp+patch fp3+dcp1600 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    # --- payload-файлы: rounds + c-crussty mirror (канон AG-234/239/278) ---
    shutil.copy(f"{RD}/claims/AG-105/prereg.md", f"{RD}/claims/AG-105.md")
    for dst in (f"{RD}/work/AG-105", "/home/z/c-crussty/work/AG-105"):
        for f in ("dispatch_526_105.json", "dispatch_526_105.py"):
            src = f if f.endswith(".py") else f"{RD}/work/AG-105/dispatch_526_105.json"
            src = __file__ if f.endswith(".py") else src
            if not (dst.endswith("AG-105") and dst.startswith(RD)):
                shutil.copy(src, f"{dst}/{f}")
    shutil.copy(f"{RD}/claims/AG-105/prereg.md", "/home/z/c-crussty/claims/AG-105.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
