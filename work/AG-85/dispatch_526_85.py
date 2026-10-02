#!/usr/bin/env python3
"""dispatch_526_85.py — AG-85 x526: r950 чанк-фронт WBP (за канон 640, 0-клейм @pop150k;
первая точка TPS(chunks) банка за 20k-порогом — S-якорь TPS@20k-chunks; ∝r²-интерп
9216@r640 → ~20.2k@r950; r-ось WBP тронута только AG-205 @pop50k — другая страта) +
s3750 — self-reject по кап-матем (окно 62.5 мин + overhead > 75 GH-кап); rt20 снят
AG-26 (живой GET, 3-й пивот, 0 runner-min) → leg-B = r800 WBP @pop150k (чанк-доза
+56%: 640→800 = 9216→~14.4k чанков; AG-205 r800 = @pop50k ДРУГАЯ страта, не клейм;
даёт с r950 трёхточечную TPS(chunks) кривую банка 640→800→950 = S-метрика).
dp3v2 canon seed42 band 5.5-13.5M.
Ценность: (1) якорь S-метрики TPS@20k-chunks на банке (OOM/timeout = честный
REFUTED-INFRA потолок xmx10G); (2) TPS(chunks)-кривая банка 640→800→950 (S-метрика).
2 POST ≥31s (FAIL AG-338); zero-code refs-API @PIN; воркри нет; диск чист (Д1-Д5)."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "e49e89845108c0b3d00022dfbace91777bacb01e"  # AG-257 WBP-канон-носитель x525
WBP_BLOB = "7c021f41"  # prefix wbp-yml blob @PIN (канон AG-257; полный sha вериф API)
DP3V2 = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
RD = "/home/z/rounds/ROUND-526"
CLAIM = ("CLAIM | AG-85 | r950+r800 WBP чанк-доза TPS(chunks) (20k-якорь+мид, "
         "0-клейм @150k) dp3v2 | 2 POST")
LEGS = [
    ("swarm-526-85", "r950", {"radius": "950"}),
    ("swarm-526-85b", "r800", {"radius": "800"}),
]
CANON = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
         "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
         "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
         "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
         "population_target": "150000", "population_seed": "42", "server_xmx": "10G",
         "server_xms": "4G", "cpu_band_min": "5500000", "cpu_band_max": "13500000",
         "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}
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


def race_check(text):
    """Чужие CLAIM на r950-чанк-фронт/s3750 = стоп (живой GET, канон OBSERVED x525).
    boundary-regex обязателен (урок AG-78 №4: substring врёт, s3750 ловит s37500)."""
    for ln in text.splitlines():
        if "AG-85 |" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        if re.search(r"\br950\b", low) or re.search(r"\bradius[=:]?\s*950\b", low) \
                or "20k-режим" in low or "чанк-фронт" in low:
            return f"RACE r950: {ln[:100]}"
        # r800: клейм только @150k-страты (AG-205 @pop50k = другая страта, не RACE)
        if re.search(r"\br800\b", low) and ("150k" in low or "pop150" in low):
            return f"RACE r800@150k: {ln[:100]}"
    return None


def find_runs(tok, pre_ids, pin, pages=3):
    found = {}
    for page in range(1, pages + 1):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in {x for x, _, _ in LEGS} and run["id"] not in pre_ids:
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
    # --- 1. PIN вериф: commit/tree FULL + world-bench-parallel.yml blob ---
    c = api(tok, f"/repos/{REPO}/commits/{PIN}")
    tree_sha = c["commit"]["tree"]["sha"]
    tree = api(tok, f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
    n = len(tree.get("tree", []))
    trunc = tree.get("truncated", False)
    yblob = next((x["sha"] for x in tree.get("tree", [])
                  if x["path"] == ".github/workflows/world-bench-parallel.yml"), "?")
    hooks = {p: len([x for x in tree.get("tree", []) if x["path"].startswith(p)])
             for p in ("native/src/", "src/")}
    print(f"pin={PIN[:8]} tree={n} trunc={trunc} wbp-yml={yblob[:8]} "
          f"native/src+src files={hooks['native/src/']+hooks['src/']}", flush=True)
    if trunc or n < 3200 or not yblob.startswith(WBP_BLOB):
        sys.exit(f"FATAL: PIN-вериф провален (tree={n} trunc={trunc} yml={yblob[:8]})")
    # --- 2. 0-клейм чек по ЖИВОЙ remote доске ---
    _, text = board_read(tok)
    race = race_check(text)
    if race:
        print(race, flush=True)
        sys.exit(1)
    print("0-клейм OK: r950+r800@150k свободны (живой GET)", flush=True)
    # --- 3. CLAIM ДО РАБОТЫ ---
    board_append(tok, [CLAIM], "board: AG-85 CLAIM r950+r800 WBP (wave-526)")
    # --- 4. снапшот ран-идов + refs zero-code @PIN ---
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for ref_name, _, _ in LEGS:
        try:
            r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                    data={"ref": f"refs/heads/{ref_name}", "sha": PIN})
            ver = r["object"]["sha"]
        except urllib.error.HTTPError as e:
            if e.code != 422:
                raise
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")
            ver = g.get("object", {}).get("sha", "")
        print(f"branch {ref_name}: sha={ver[:8]} match={ver == PIN}", flush=True)
        if ver != PIN:
            sys.exit(f"FATAL: ref sha mismatch {ref_name}")
    # --- 5. Диспатчи (≤2), 2-й через 31s (FAIL AG-338) ---
    for i, (ref_name, cell, delta) in enumerate(LEGS):
        inputs = dict(CANON)
        inputs.update(delta)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {cell}: POST 204", flush=True)
        if i == 0:
            time.sleep(31)
    # --- 6. GET-вериф run-ids (×3 страницы канон AG-197) ---
    time.sleep(8)
    found = find_runs(tok, pre, PIN)
    if len(found) < len(LEGS):
        print("WARN: не оба рана видимы — вериф повторит --finalize", flush=True)
    payload = {"pin": PIN, "tree_files": n, "claim": CLAIM, "runs": dict(found),
               "legs": [{"branch": b, "cell": c, "delta": d} for b, c, d in LEGS],
               "canon_vector": "640*/300s/fp4/guard1/gc3/ic1/fd1/rt4/bc1/pop150k/"
                               "pseed42/10G/xms4G/dp3v2/band5.5-13.5M (*leg-A radius950, "
                               "leg-B radius800)",
               "dp3v2": DP3V2}
    json.dump(payload, open(f"{RD}/work/AG-85/dispatch_526_85.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {PIN} TREE {n}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-85/dispatch_526_85.json"))
    pin, n = p["pin"], p["tree_files"]
    if len(p["runs"]) < len(LEGS):
        found = find_runs(tok, set(), pin, pages=3)
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-85/dispatch_526_85.json", "w"),
                      indent=1, ensure_ascii=False)
    ids = p["runs"]
    ra = ids.get("swarm-526-85", 0)
    rb = ids.get("swarm-526-85b", 0)
    pin8 = pin[:8]
    lines = [
        (f"FACT | AG-85 | 2/2 204 @{pin8} t{n}: {ra} r950 s42 + {rb} r800 WBP "
         f"pop150k QUEUED | api"),
        (f"DISP | AG-85 | r950+r800 чанк-доза 2/2 queued @swarm-526-85[ab] "
         f"WBP dp3v2 seed42; payload work/AG-85 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-85 | files=work+claims/AG-85 | idea=r950+r800 "
         f"TPS(chunks) curve | evidence=2/2 @{pin8}"),
    ]
    ok = board_append(tok, lines, "board: AG-85 fact+disp+patch r950+r800 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    # --- payload-файлы: rounds + c-crussty mirror (канон AG-234/239/278;
    #     SameFileError-гейт: урок AG-278 №9 / AG-2 №7) ---
    import os
    for dst in (f"{RD}/work/AG-85", "/home/z/c-crussty/work/AG-85"):
        for fn in ("dispatch_526_85.json", "dispatch_526_85.py"):
            src = f"{RD}/work/AG-85/{fn}"
            d = f"{dst}/{fn}"
            if (os.path.exists(d) and os.path.exists(src)
                    and os.path.samefile(src, d)):
                continue
            shutil.copy(src, d)
    shutil.copy(f"{RD}/claims/AG-85/prereg.md", f"{RD}/claims/AG-85.md")
    shutil.copy(f"{RD}/claims/AG-85/prereg.md", "/home/z/c-crussty/claims/AG-85.md")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} pin={pin8}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
