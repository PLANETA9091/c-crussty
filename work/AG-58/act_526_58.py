#!/usr/bin/env python3
"""act_526_58.py — AG-58 волна-526: leg-A s6000 s-мид bench-v2 (зазор 3000-9000,
0-клейм: s3000 AG-255/260, канон 9000; WBP-s отдельная ось) @PIN a9ff088f
(w/xmx/s-носитель AG-278, yml 0049e34a) + leg-B pop1M pop-край WBP за 750k
(низ: 500k raced AG-241, 750k AG-40×526; суперлинейный pop-tax / OOM-потолок)
@PIN e49e8984 (WBP-канон AG-257, wbp-yml 7c021f41). Zero-code refs-API, CAS-CLAIM
с race-recheck (CAS-SIB канон AG-250), пивоты A: xmx8G→dcp300; B: rt28→pop600k.
Сиды: 525058/526058 грязные (w525 AG-58 3dim-w512) → 527058 (grep-чист).
Д1-Д5: API-only, 0 ворктри, 0 gc/prune, 0 локальных коммитов."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
WS = "/home/z/c-crussty"
AG = "AG-58"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
PIN_BV2 = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"   # bench-v2 w/xmx/s-носитель AG-278
BV2_BLOB = "0049e34a53fb2ab1906223ac7df318c463708261"
PIN_BV2_FB = "2171d6da775975c4bee94748f549ad16f02074e1"  # sim/fp-канон AG-138 (fallback)
BV2_FB_BLOB = "b4e9e05b00000000000000000000000000000000"
PIN_WBP = "e49e89845108c0b3d00022dfbace91777bacb01e"   # WBP-канон-носитель AG-257
WBP_BLOB = "7c021f41"

BV2_BASE = {"radius_blocks": "1136", "run_seconds": "9000", "server_xmx": "10G",
            "bench_dims": "minecraft:overworld", "dim_gen_window": "256",
            "drain_cap_polls": "900", "seed": None}
WBP_CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
             "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
             "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
             "population_target": "1000000", "population_seed": "42",
             "server_xmx": "10G", "server_xms": "4G",
             "cpu_band_min": "5500000", "cpu_band_max": "13500000",
             "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}

# (клетка, race-regex, inputs-переопределения, подпись клетки в CLAIM)
A_OPTS = [
    ("s6000",  r"(?i)\bs6000\b",              {"run_seconds": "6000"}, "s6000 s-мид bench-v2 (3000-9000, 0-клейм)"),
    ("xmx8G",  r"(?i)\bxmx8g\b",              {"server_xmx": "8G"},    "xmx8G heap-floor низ-край (за канон 10G)"),
    ("dcp300", r"\bdcp300\b",                 {"drain_cap_polls": "300"}, "dcp300 dcp-мид (240-400)"),
]
B_OPTS = [
    ("pop1M",  r"(?i)\bpop[ ]?1m\b|\bpop1000k\b|\bpopulation_target[ =]1000000\b", {}, "pop1M pop-край WBP (за 750k)"),
    ("rt28",   r"\brt28\b",                   {}, None),  # WBP нет rt-инпута в каноне AG-40 -> rt через lever? нет -> поп-пивот
    ("pop600k", r"\bpop600k\b",               {"population_target": "600000"}, "pop600k pop-мид WBP (750k-500k)"),
]
# rt28 убран из опций (world-bench-parallel.yml без прямого randomtick-инпута в векторе AG-40)
B_OPTS = [B_OPTS[0], B_OPTS[2]]
SEED = "527058"
BR_A, BR_B = "swarm-526-58", "swarm-526-58b"


def token():
    url = subprocess.run(["git", "-C", WS, "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, ok404=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if ok404 and e.code == 404:
            return None
        print(f"HTTP {e.code} {url}: {e.read()[:200]}", flush=True)
        raise
    return json.loads(body) if body else {}


def board_read(tok):
    d = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    return d["sha"], base64.b64decode(d["content"]).decode("utf-8")


def raced(text, regexes, my_cells):
    """Чужой CLAIM/FAIL на мои клетки = стоп (regex-boundary, свои строки игнор)."""
    for ln in text.splitlines():
        if not (ln.startswith("CLAIM") or ln.startswith("FAIL")):
            continue
        if any(c in ln for c in my_cells) and AG in ln:
            continue
        low = ln.lower()
        for rx in regexes:
            if re.search(rx, low):
                return ln[:110]
    return None


def seed_taken(text, seed):
    return seed in text


def tree_check(tok, pin, wf_path, blob8):
    c = api(tok, f"/repos/{REPO}/commits/{pin}")
    t = c["commit"]["tree"]["sha"]
    tr = api(tok, f"/repos/{REPO}/git/trees/{t}?recursive=1")
    n = len(tr.get("tree", []))
    yb = next((x["sha"] for x in tr.get("tree", [])
               if x["path"] == wf_path), "?")
    ok = (not tr.get("truncated", False)) and n >= 3200 and str(yb).startswith(blob8)
    print(f"pin={pin[:8]} tree={n} trunc={tr.get('truncated')} yml={str(yb)[:8]} ok={ok}", flush=True)
    return ok, n


def pin_bv2(tok):
    ok, n = tree_check(tok, PIN_BV2, ".github/workflows/bench-v2.yml", "0049e34a")
    if ok:
        return PIN_BV2, n
    ok, n = tree_check(tok, PIN_BV2_FB, ".github/workflows/bench-v2.yml", "b4e9e05b")
    if ok:
        return PIN_BV2_FB, n
    sys.exit("FATAL: оба bv2-пина мертвы")


def find_runs(tok, pre_ids, branches, pages=3):
    found = {}
    for page in range(1, pages + 1):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in branches and run["id"] not in pre_ids:
                print(f"RUN {run['id']} {b} sha={run['head_sha'][:8]} {run['status']}", flush=True)
                found[b] = run["id"]
        if len(found) >= len(branches):
            break
    return found


def phase_run():
    tok = token()
    # --- 1. пины: tree FULL + yml-blob assert ---
    pinA, nA = pin_bv2(tok)
    okW, nW = tree_check(tok, PIN_WBP, ".github/workflows/world-bench-parallel.yml", WBP_BLOB)
    if not okW:
        sys.exit("FATAL: WBP-пин мёртв")
    # --- 2. сид: grep-чист по живой доске ---
    _, text0 = board_read(tok)
    seed = SEED
    while seed_taken(text0, seed):
        seed = str(int(seed) + 10000)
        print(f"seed-collision -> {seed}", flush=True)
    # --- 3. выбор пары клеток + CLAIM (CAS, race-recheck в ретраях) ---
    claim = None
    for attempt in range(4):
        _, text = board_read(tok)
        cells, regexes = [], []
        for opts, lane in ((A_OPTS, "A"), (B_OPTS, "B")):
            free = [o for o in opts if not raced(text, [o[1]], [])]
            if not free:
                sys.exit(f"RACE: лейн {lane} весь занят — цикл к шагу 1")
            cells.append(free[0])
            regexes.append(free[0][1])
        a, b = cells
        claim = (f"CLAIM | AG-58 | {a[3]} @{pinA[:8]} + {b[3]} "
                 f"seed{seed} | 2 POST")
        if len(claim) > 120:
            sys.exit(f"FATAL: claim {len(claim)} >120")
        print(f"CLAIM try: {claim}", flush=True)
        put_ok = True
        for att in range(6):
            sha, t2 = board_read(tok)
            rl = raced(t2, regexes, [a[0], b[0]])
            if rl:
                print(f"RACE-PIVOT: {rl}", flush=True)
                put_ok = False
                break
            if claim in t2:
                print("already-appended", flush=True)
                put_ok = True
                break
            new = t2.rstrip("\n") + "\n\n" + claim + "\n"
            body = {"message": "board: AG-58 CLAIM s6000+pop1M (wave-526)",
                    "content": base64.b64encode(new.encode()).decode(),
                    "sha": sha, "branch": "master"}
            try:
                r = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT", data=body)
                print(f"board-commit {r['commit']['sha'][:8]}", flush=True)
                put_ok = True
                break
            except urllib.error.HTTPError as e:
                if e.code in (409, 422) and att < 5:
                    time.sleep(6)
                    continue
                raise
        if put_ok:
            break
    if not put_ok:
        sys.exit("RACE: 4 пивота исчерпаны")
    # --- 4. снапшот ран-идов + refs zero-code (полный 40-sha) ---
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for br, pin in ((BR_A, pinA), (BR_B, PIN_WBP)):
        r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{br}", "sha": pin})
        if not r.get("ref"):
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{br}", ok404=True)
            if not g or g.get("object", {}).get("sha") != pin:
                sys.exit(f"FATAL: ref {br} занят другим sha")
        ver = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
        print(f"branch {br}: {ver[:8]} match={ver == pin}", flush=True)
        if ver != pin:
            sys.exit(f"FATAL: ref sha mismatch {br}")
        time.sleep(31)
    # --- 5. диспатчи ≤2, ≥31s между POST ---
    inA = dict(BV2_BASE)
    inA.update(a[2])
    inA["seed"] = seed
    inB = dict(WBP_CANON)
    inB.update(b[2])
    for i, (br, wf, inp) in enumerate(((BR_A, "bench-v2.yml", inA),
                                       (BR_B, "world-bench-parallel.yml", inB))):
        api(tok, f"/repos/{REPO}/actions/workflows/{wf}/dispatches",
            method="POST", data={"ref": br, "inputs": inp})
        print(f"dispatch {br} {wf}: 204", flush=True)
        if i == 0:
            time.sleep(31)
    # --- 6. GET-вериф run-id ---
    time.sleep(8)
    found = find_runs(tok, pre, {BR_A, BR_B})
    if len(found) < 2:
        time.sleep(20)
        found = {**found, **find_runs(tok, pre, {BR_A, BR_B} - set(found))}
    payload = {"pinA": pinA, "treeA": nA, "pinB": PIN_WBP, "treeB": nW,
               "claim": claim, "seed": seed, "runs": found,
               "legs": [{"branch": BR_A, "wf": "bench-v2.yml", "cell": a[0],
                         "inputs": inA},
                        {"branch": BR_B, "wf": "world-bench-parallel.yml",
                         "cell": b[0], "inputs": inB}]}
    json.dump(payload, open(f"{RD}/work/{AG}/dispatch_526_58.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {pinA[:8]}/{PIN_WBP[:8]} TREE {nA}/{nW} SEED {seed}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/{AG}/dispatch_526_58.json"))
    if len(p["runs"]) < 2:
        p["runs"] = {**p["runs"], **find_runs(tok, set(), {BR_A, BR_B})}
        json.dump(p, open(f"{RD}/work/{AG}/dispatch_526_58.json", "w"),
                  indent=1, ensure_ascii=False)
    ra, rb = p["runs"].get(BR_A, 0), p["runs"].get(BR_B, 0)
    cellA = p["legs"][0]["cell"]
    cellB = p["legs"][1]["cell"]
    lines = [
        (f"FACT | AG-58 | 2/2 204 @{p['pinA'][:8]}+{p['pinB'][:8]} "
         f"t{p['treeA']}/{p['treeB']}: {ra} {cellA} s{p['seed']} + {rb} {cellB} WBP QUEUED | api"),
        (f"DISP | AG-58 | {cellA}-мид + {cellB}-край 2/2 queued @swarm-526-58[ab] "
         f"1d/r1136 + dp3v2 s42; payload work/{AG} | 2/2 204"),
        (f"PATCH_SUMMARY | AG-58 | files=claims,work/{AG} | idea={cellA}+{cellB} "
         f"dose fill | evidence=2/2 queued"),
    ]
    for l in lines:
        assert len(l) <= 120, f"{len(l)}: {l}"
    ok = False
    for att in range(6):
        sha, t = board_read(tok)
        if all(L in t for L in lines):
            ok = True
            break
        new = t.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
        body = {"message": "board: AG-58 fact+disp+patch s6000+pop1M queued (wave-526)",
                "content": base64.b64encode(new.encode()).decode(),
                "sha": sha, "branch": "master"}
        try:
            r = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT", data=body)
            print(f"board-commit {r['commit']['sha'][:8]} (+3)", flush=True)
            ok = True
            break
        except urllib.error.HTTPError as e:
            if e.code in (409, 422) and att < 5:
                time.sleep(6)
                continue
            raise
    print("BOARD-OK" if ok else "BOARD-FAIL", flush=True)
    # --- payload: rounds + mirror в общий воркспейс (без git-коммитов!) ---
    for dst in (f"{RD}/work/{AG}", f"{WS}/work/{AG}"):
        shutil.copy(f"{RD}/work/{AG}/dispatch_526_58.json", f"{dst}/dispatch_526_58.json")
        shutil.copy(__file__, f"{dst}/act_526_58.py")
        shutil.copy(f"{RD}/work/{AG}/MEMORY.md", f"{dst}/MEMORY.md")
    for dst in (f"{RD}/claims/{AG}.md", f"{WS}/claims/{AG}.md"):
        shutil.copy(f"{RD}/work/{AG}/prereg.md", dst)
    print(f"FINAL ra={ra} rb={rb}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
