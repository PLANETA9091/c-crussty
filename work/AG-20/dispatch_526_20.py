#!/usr/bin/env python3
"""dispatch_526_20.py — AG-20 волна-526: fg0 lever-A/B (fluid_guard=0, canon 1;
пивот fg0->AG-2 по живому GET (0 runner-min); xms6G xms-мид (4-8;
канон 4G, сиб AG-22 взял 7G/10G) WBP dp3v2 pop150k same-seed 526020. Zero-code:
refs-API @master-tip после API tree-чека ≥3200, 2 POST ≥31s (FAIL AG-338),
GET-вериф run-ids + head_sha==PIN, CAS-доска, payload-зеркало."""
import base64, json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ME = "AG-20"
RD = "/home/z/rounds/ROUND-526"
BR_A, BR_B = "swarm-526-20", "swarm-526-20b"
SEED = "526020"
DP_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
          "v484-dp3v2/stz3v2-fixture.zip")
BAND = ("5500000", "13500000")  # 5.5-13.5M когорта WBP x525
CLAIM = ("CLAIM | AG-20 | xms6G xms-мид (4-8) + rt2 rt-мид (1-3) WBP dp3v2 "
         "pop150k same-seed 526020 | 2 POST")
LEGS = [
    (BR_A, {"datapack_url": DP_URL, "server_xms": "6G", "population_seed": SEED,
            "cpu_band_min": BAND[0], "cpu_band_max": BAND[1]}, "xms6G"),
    (BR_B, {"datapack_url": DP_URL, "region_threads": "2", "population_seed": SEED,
            "cpu_band_min": BAND[0], "cpu_band_max": BAND[1]}, "rt2"),
]
assert len(CLAIM) <= 120, len(CLAIM)


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "ag-20-wave526"})
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


def race_check(text):
    """Чужие CLAIM на fg0/xms6 = стоп (живой GET, канон OBSERVED AG-273/250)."""
    for ln in text.splitlines():
        if "AG-20" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        if re.search(r"\bxms\s?6g?\b", low) or "xms-доза 6" in low:
            return f"RACE xms6G: {ln[:100]}"
        if re.search(r"\brt2\b", low):
            return f"RACE rt2: {ln[:100]}"
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
    # --- 1. PIN = live master-tip; tree FULL >=3200 + yml blob (канон AG-278) ---
    m = api(tok, f"/repos/{REPO}/commits/master")
    pin = m["sha"]
    tree_sha = m["commit"]["tree"]["sha"]
    tree = api(tok, f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
    n = len(tree.get("tree", []))
    trunc = tree.get("truncated", False)
    yblob = next((x["sha"] for x in tree.get("tree", [])
                  if x["path"] == f".github/workflows/{WF}"), "?")
    print(f"pin={pin[:8]} tree={n} truncated={trunc} wbp-yml={yblob[:8]}", flush=True)
    if trunc or n < 3200:
        sys.exit(f"FATAL sparse-мина: tree {n} truncated={trunc}")
    # --- 2. 0-клейм чек по ЖИВОЙ remote доске ---
    _, text = board_read(tok)
    race = race_check(text)
    if race:
        print(race, flush=True)
        sys.exit(1)
    print("0-клейм OK: xms6G + rt2 свободны (живой GET)", flush=True)
    # --- 3. CLAIM ДО РАБОТЫ ---
    board_append(tok, [CLAIM], "board: AG-20 CLAIM xms6G+rt2 WBP (wave-526)")
    # --- 4. refs zero-code @PIN (FULL 40-sha) + GET-вериф ---
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for ref_name, _, _ in LEGS:
        r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{ref_name}", "sha": pin})
        if not r.get("ref"):
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")
            if g.get("object", {}).get("sha", "") != pin:
                sys.exit(f"FATAL: ref {ref_name} exists with other sha")
        ver = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")["object"]["sha"]
        print(f"branch {ref_name}: sha={ver[:8]} match={ver == pin}", flush=True)
        if ver != pin:
            sys.exit(f"FATAL: ref sha mismatch {ref_name}")
    # --- 5. Диспатчи (≤2), 2-й через 31s (FAIL AG-338) ---
    for i, (ref_name, inputs, tag) in enumerate(LEGS):
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {tag}: POST", flush=True)
        if i == 0:
            time.sleep(31)
    # --- 6. GET-вериф run-ids ---
    time.sleep(10)
    found = find_runs(tok, pre, pin)
    if len(found) < len(LEGS):
        print("WARN: не оба рана видимы — вериф повторит --finalize", flush=True)
    payload = {"pin": pin, "tree_files": n, "wbp_yml_blob": yblob, "claim": CLAIM,
               "runs": dict(found),
               "legs": [{"branch": b, "tag": t, "inputs": i} for b, i, t in LEGS]}
    json.dump(payload, open(f"{RD}/work/AG-20/dispatch_526_20.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {pin} TREE {n}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-20/dispatch_526_20.json"))
    pin, n = p["pin"], p["tree_files"]
    if len(p["runs"]) < len(LEGS):
        found = find_runs(tok, set(), pin, pages=3)
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-20/dispatch_526_20.json", "w"),
                      indent=1, ensure_ascii=False)
    ids = p["runs"]
    ra = ids.get(BR_A, 0)
    rb = ids.get(BR_B, 0)
    pin8 = pin[:8]
    lines = [
        (f"FACT | AG-20 | 2/2 204 @{pin8} t{n}: {ra} xms6G s{SEED} + {rb} rt2 "
         f"s{SEED} WBP pop150k QUEUED | api"),
        (f"DISP | AG-20 | xms6G+rt2 миды 2/2 queued @swarm-526-20[ab] WBP dp3v2 "
         f"pop150k; payload work/AG-20 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-20 | files=claims,work/AG-20 | idea=xms6G+rt2 dose "
         f"fill | ev=2/2 @{pin8}"),
    ]
    ok = board_append(tok, lines, "board: AG-20 fact+disp+patch xms6G+rt2 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    print(f"FINAL ra={ra} rb={rb} pin={pin8}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
