#!/usr/bin/env python3
"""dispatch_526_157.py — AG-157 x526: r900+r1000 WBP TPS(chunks) мид+фронт за-20k @pop150k
(0-клейм, зазоры 800-950/950-1100; плотная кривая у S-якоря TPS@20k-chunks; ∝r²
18227@900 / 22502@1000 от 9216@640; r-ось WBP тронута AG-85 {800,950} — мои клетки
свободны, страта та же @150k). dp3v2 canon seed42 band 5.5-13.5M, PIN e49e8984.
Ценность: (1) мид/фронт вокруг S-якоря 20k — dose-факт или честный REFUTED-INFRA
потолок xmx10G @22k+ чанков; (2) min-of-3-когорта к r640/r800/r950 seed42.
2 POST ≥31s (FAIL AG-338); refs POST full-sha + GET-вериф (Л188a); 1 диспатч = 1
ветка (Л188b); zero-code; диск чист (Д1-Д5)."""
import base64, json, re, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "e49e89845108c0b3d00022dfbace91777bacb01e"  # AG-257/85 WBP-канон-носитель
WBP_BLOB = "7c021f41"
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
RD = "/home/z/rounds/ROUND-526"
CLAIM = ("CLAIM | AG-157 | r900+r1000 WBP TPS(chunks) (мид 800-950 + фронт за-20k, "
         "0-клейм @150k) dp3v2 s42 | 2 POST")
LEGS = [
    ("swarm-526-157", "r900", {"radius": "900"}),
    ("swarm-526-157b", "r1000", {"radius": "1000"}),
]
CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
         "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
         "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
         "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
         "population_target": "150000", "population_seed": "42", "server_xmx": "10G",
         "server_xms": "4G", "cpu_band_min": "5500000", "cpu_band_max": "13500000",
         "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}
assert len(CLAIM) <= 120, len(CLAIM)
TOK = open("/tmp/gh_token").read().strip()
HDR = {"Authorization": f"Bearer {TOK}", "Accept": "application/vnd.github+json",
       "User-Agent": "swarm-526-ag157"}


def api(url, method="GET", data=None, t=90):
    payload = json.dumps(data).encode() if data is not None else None
    req = urllib.request.Request(API + url, data=payload, method=method, headers=HDR)
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=t) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code} {url}: {e.read()[:220]}", flush=True)
        raise
    return json.loads(body) if body else {}


def board_read():
    d = api(f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    return d["sha"], base64.b64decode(d["content"]).decode("utf-8")


def board_append(lines, msg):
    for l in lines:
        assert len(l) <= 120, f"{len(l)} ch > 120: {l}"
    for attempt in range(6):
        sha, text = board_read()
        if all(L in text for L in lines):
            print("already-appended", flush=True)
            return True
        new = text.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
        body = {"message": msg, "content": base64.b64encode(new.encode()).decode(),
                "sha": sha, "branch": "master"}
        try:
            r = api(f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT", data=body)
            print(f"board-commit {r['commit']['sha'][:8]} (+{len(lines)})", flush=True)
            return True
        except urllib.error.HTTPError as e:
            if e.code in (409, 422) and attempt < 5:
                time.sleep(4)
                continue
            raise
    return False


def race_check(text):
    """Живой GET прямо перед PUT (канон OBSERVED x525). Boundary-regex + страта.
    Уроки AG-85: guard-regex только \\bклетка\\b + явный страта-контекст; чужие
    страты НЕ клейм (прецедент: AG-205 r800 @pop50k vs AG-85 r800 WBP @150k оба
    легальны). AG-154 r1000+r1040 = bench-v2 1d/w256/9000s (ch/s-страта) — НЕ WBP
    TPS(chunks) @150k. WBP-маркеры: wbp|WBP|pop150|150k|чанк-доза|TPS(chunks)."""
    WBP_CTX = re.compile(r"\bWBP\b|pop150|@150k|@150 k|чанк-доза|TPS\(chunks\)|"
                         r"world-bench-parallel", re.I)
    BV2_CTX = re.compile(r"\b1d\b|w256|9000s|dcp\d{3,4}|t4231|bv2|bench-v2|1-dim", re.I)
    for ln in text.splitlines():
        if "AG-157 |" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        hit = re.search(r"\br900\b|\br1000\b", low) or \
            re.search(r"\bradius[=: ]*(900|1000)\b", low)
        if not hit:
            continue
        if WBP_CTX.search(low):
            return f"RACE WBP-страта {hit.group(0)}: {ln[:100]}"
        if not BV2_CTX.search(low):
            # нет явного страта-маркера — консервативно считаем гонкой
            return f"RACE ambiguous {hit.group(0)}: {ln[:100]}"
    return None


def find_runs(pre_ids, pages=3):
    found = {}
    branches = {x for x, _, _ in LEGS}
    for page in range(1, pages + 1):
        d = api(f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            if run["head_branch"] in branches and run["id"] not in pre_ids:
                ok = run["head_sha"] == PIN
                print(f"RUN {run['id']} {run['head_branch']} sha={run['head_sha'][:8]} "
                      f"{run['status']} match={'OK' if ok else 'MISMATCH'}", flush=True)
                if ok:
                    found[run["head_branch"]] = run["id"]
        if len(found) >= len(LEGS):
            break
    return found


def phase_run():
    # 1. PIN вериф: tree FULL + wbp-yml blob
    c = api(f"/repos/{REPO}/commits/{PIN}")
    tree = api(f"/repos/{REPO}/git/trees/{c['commit']['tree']['sha']}?recursive=1")
    n = len(tree.get("tree", []))
    yblob = next((x["sha"] for x in tree.get("tree", [])
                  if x["path"] == ".github/workflows/world-bench-parallel.yml"), "?")
    print(f"pin={PIN[:8]} tree={n} trunc={tree.get('truncated')} wbp-yml={yblob[:8]}",
          flush=True)
    if tree.get("truncated") or n < 3200 or not yblob.startswith(WBP_BLOB):
        sys.exit(f"FATAL: PIN-вериф провален (tree={n} trunc={tree.get('truncated')})")
    # 2. 0-клейм по ЖИВОЙ доске
    _, text = board_read()
    race = race_check(text)
    if race:
        print(race, flush=True)
        sys.exit(1)
    print("0-клейм OK: r900+r1000@150k свободны (живой GET)", flush=True)
    # 3. CLAIM ДО РАБОТЫ (CAS)
    board_append([CLAIM], "board: AG-157 CLAIM r900+r1000 WBP TPS(chunks) (wave-526)")
    # 4. pre-snapshot run-ids + refs zero-code @PIN (POST full-sha, GET-вериф)
    pre = {r["id"] for r in api("/repos/{REPO}/actions/runs?per_page=100".replace(
        "{REPO}", REPO)).get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for ref_name, _, _ in LEGS:
        try:
            r = api(f"/repos/{REPO}/git/refs", method="POST",
                    data={"ref": f"refs/heads/{ref_name}", "sha": PIN})
            ver = r["object"]["sha"]
        except urllib.error.HTTPError as e:
            if e.code != 422:
                raise
            g = api(f"/repos/{REPO}/git/ref/heads/{ref_name}")
            ver = g.get("object", {}).get("sha", "")
        print(f"branch {ref_name}: sha={ver[:8]} match={ver == PIN}", flush=True)
        if ver != PIN:
            sys.exit(f"FATAL: ref sha mismatch {ref_name}")
    # 5. Диспатчи (≤2), 2-й через 31s
    for i, (ref_name, cell, delta) in enumerate(LEGS):
        inputs = dict(CANON)
        inputs.update(delta)
        api(f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    # 6. GET-вериф run-ids (×3 страницы)
    time.sleep(8)
    found = find_runs(pre)
    payload = {"pin": PIN, "tree_files": n, "claim": CLAIM, "runs": dict(found),
               "legs": [{"branch": b, "cell": c, "delta": d} for b, c, d in LEGS],
               "canon_vector": "640*/300s/fp4/guard1/gc3/ic1/fd1/rt4/bc1/pop150k/"
                               "pseed42/10G/xms4G/dp3v2/band5.5-13.5M (*leg-A radius900, "
                               "leg-B radius1000)",
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-157/dispatch_526_157.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {PIN} TREE {n}", flush=True)


def phase_finalize():
    p = json.load(open(f"{RD}/work/AG-157/dispatch_526_157.json"))
    if len(p["runs"]) < len(LEGS):
        found = find_runs(set(), pages=3)
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-157/dispatch_526_157.json", "w"),
                      indent=1, ensure_ascii=False)
    ids = p["runs"]
    ra = ids.get("swarm-526-157", 0)
    rb = ids.get("swarm-526-157b", 0)
    pin8 = p["pin"][:8]
    lines = [
        (f"FACT | AG-157 | 2/2 204 @{pin8} t{p['tree_files']}: {ra} r900 s42 + {rb} "
         f"r1000 WBP pop150k QUEUED | api"),
        (f"DISP | AG-157 | r900+r1000 TPS(chunks) 2/2 queued @157[ab] WBP dp3v2 "
         f"s42; payload work/AG-157 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-157 | files=claims,work/AG-157 | idea=r900/r1000 "
         f"TPS(chunks) mid+frontier | evidence=2/2 @{pin8}"),
    ]
    ok = board_append(lines, "board: AG-157 fact+disp+patch r900+r1000 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    import os, shutil
    for dst in (f"{RD}/work/AG-157", "/home/z/c-crussty/work/AG-157"):
        os.makedirs(dst, exist_ok=True)
        for fn in ("dispatch_526_157.json", "dispatch_526_157.py"):
            src = f"{RD}/work/AG-157/{fn}"
            if os.path.abspath(src) != os.path.abspath(f"{dst}/{fn}"):
                shutil.copyfile(src, f"{dst}/{fn}")
    for dst in ("/home/z/rounds/ROUND-526/claims", "/home/z/c-crussty/claims"):
        src = f"{RD}/claims/AG-157.md"
        if os.path.abspath(src) != os.path.abspath(f"{dst}/AG-157.md"):
            shutil.copyfile(src, f"{dst}/AG-157.md")
    print("MIRROR-OK", flush=True)


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--finalize":
        phase_finalize()
    else:
        phase_run()
