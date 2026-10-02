#!/usr/bin/env python3
"""dispatch_526_135.py — AG-135 x526: leg-A w5760 w-мид@r1136 (зазор 4352-6912,
0-клейм; 4352=AG-276, 6912=AG-256 оба queued) bench-v2 1d/9000s/dcp900/xmx10G
@swarm-526-135 = a9ff088f (G4-fix carrier, tree-4231 FULL, bv2 blob 0049e34a),
seed 528135. leg-B s7000 s-фронт за s4800 (AG-99 queued; s-ось: 300/750/1050/
1350/1500/2250/2400/3000/4800) WBP pop150k dp3v2 canon fp4 seed42 cohort
@swarm-526-135b = e49e8984 (tree-4231 FULL, wbp blob 7c021f41).
Сиды 528135/42: grep живой доски + rounds 0-клейм (42 = WBP-cohort канон
AG-240/258/268/269). 2-й POST через 31s (FAIL AG-338), ≤2 POST, refs-API
FULL-sha, CAS-board (канон AG-105/278)."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
PIN_BV2 = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"   # G4-fix carrier, tree 4231
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP carrier x525/526, tree 4231
WF_WBP = "world-bench-parallel.yml"
WF_BV2 = "bench-v2.yml"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
CLAIM = ("CLAIM | AG-135 | w5760 w-мид (4352-6912, 0-клейм) @a9ff088f + s7000 "
         "s-фронт за 4800 WBP seed42 | 2 POST")
LEGS = [
    {"branch": "swarm-526-135", "pin": PIN_BV2, "wf": WF_BV2, "tag": "w5760",
     "seed": "528135",
     "inputs": {"radius_blocks": "1136", "run_seconds": "9000", "seed": "528135",
                "server_xmx": "10G", "bench_dims": "minecraft:overworld",
                "dim_gen_window": "5760", "drain_cap_polls": "900"}},
    {"branch": "swarm-526-135b", "pin": PIN_WBP, "wf": WF_WBP, "tag": "s7000",
     "seed": "42",
     "inputs": {"radius": "640", "seconds": "7000", "fake_players": "4",
                "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
                "flush_diet": "1", "fluid_dirty": "0", "fluid_bitmask": "0",
                "region_threads": "4", "batch_collector": "1",
                "datapack_url": DP3V2, "skip_store_bb": "0", "region_steal": "0",
                "bu_defer": "0", "population_target": "150000",
                "population_seed": "42", "server_xmx": "10G", "server_xms": "4G",
                "cpu_band_min": "5500000", "cpu_band_max": "13500000"}},
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
    """CAS PUT append-only, 6 попыток (канон AG-91/197/208/278/105)."""
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
    """Чужие CLAIM на мои клетки = стоп (живой GET, канон AG-211/278/105)."""
    # regex-boundary канон ×2: дедуп ТОЛЬКО точные токены клеток — generic
    # дескрипторы («w-мид» у AG-251/w14336) не мой сигнал (2 ложных abort
    # пойманы на себе; клетки = \bw5760\b/\bs7000\b)
    for ln in text.splitlines():
        if "AG-135" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        if re.search(r"\bw5760\b", low):
            return f"RACE w5760: {ln[:100]}"
        if re.search(r"\bs7000\b", low):
            return f"RACE s7000: {ln[:100]}"
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
    # --- 1. PIN-вериф: tree FULL + workflow-блобы (канон AG-208/239/278/105) ---
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
    print("0-клейм OK: w5760 + s7000 свободны (живой GET)", flush=True)
    # --- 3. CLAIM ДО РАБОТЫ ---
    board_append(tok, [CLAIM], "board: AG-135 CLAIM w5760+s7000 (wave-526)")
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
    payload = {"pins": {"bv2": PIN_BV2, "wbp": PIN_WBP},
               "tree_files": {"bv2": 4231, "wbp": 4231},
               "claim": CLAIM, "runs": dict(found),
               "legs": [{"branch": L["branch"], "wf": L["wf"], "tag": L["tag"],
                         "seed": L["seed"]} for L in LEGS]}
    json.dump(payload, open(f"{RD}/work/AG-135/dispatch_526_135.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PINS bv2={PIN_BV2[:8]} wbp={PIN_WBP[:8]}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-135/dispatch_526_135.json"))
    if len(p["runs"]) < len(LEGS):
        found = find_runs(tok, set())
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-135/dispatch_526_135.json", "w"),
                      indent=1, ensure_ascii=False)
    ra = p["runs"].get("swarm-526-135", 0)
    rb = p["runs"].get("swarm-526-135b", 0)
    lines = [
        (f"FACT | AG-135 | 2/2 204 @a9ff088f/e49e8984: {ra} w5760 s528135 bv2 + "
         f"{rb} s7000 s42 WBP QUEUED | api"),
        (f"DISP | AG-135 | w5760 w-мид + s7000 s-фронт 2/2 queued @135[ab] "
         f"9000s/dcp900 + pop150k dp3v2; work/AG-135 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-135 | files=claims,work/AG-135 | idea=w5760 mid + "
         f"s7000 soak frontier dose fill | ev=2/2 204"),
    ]
    ok = board_append(tok, lines, "board: AG-135 fact+disp+patch w5760+s7000 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    # --- payload-файлы: rounds + c-crussty mirror (канон AG-234/239/278/105) ---
    shutil.copy(f"{RD}/claims/AG-135/prereg.md", f"{RD}/claims/AG-135.md")
    shutil.copy(f"{RD}/claims/AG-135/prereg.md", "/home/z/c-crussty/claims/AG-135.md")
    for f in ("dispatch_526_135.json", "dispatch_526_135.py"):
        src = __file__ if f.endswith(".py") else f"{RD}/work/AG-135/dispatch_526_135.json"
        shutil.copy(src, f"{RD}/work/AG-135/{f}")
        shutil.copy(src, f"/home/z/c-crussty/work/AG-135/{f}")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
