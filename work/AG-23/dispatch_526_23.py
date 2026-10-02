#!/usr/bin/env python3
"""dispatch_526_23.py — AG-23 волна-526: dcp400+dcp600 dcp-низ leg-2 bench-v2
(зазор 240-700 вокруг канона dcp900; точки 240/700/800/900/1000/1100/1200/1500;
0-клейм census+живой GET) @r1136/s9000/dgw256/xmx10G/1d-ow, seeds 525023/526023.
Ценность: закрывает dcp-лестницу низа — чувствительность TPS к drain-пламбингу
(метод-факт AG-278: сопоставимость смешанных dcp-ног банка). Пивотов 0: клетка
свободна на живом GET (wave-526 AG-1/5/7/8/19/24/26/29 взяли sim/fp/rt/w/xms).
2-й POST через 31s (FAIL AG-338), ≤2 POST, refs-API zero-code, DISK-Д1-Д5."""
import base64, json, re, shutil, subprocess, sys, time, urllib.request, urllib.error, os

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "bench-v2.yml"
YML_BLOB = "0049e34a53fb2ab1906223ac7df318c463708261"   # канон bench-v2.yml
PIN_FALLBACK = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"  # x525-носитель dcp-ног
RD = "/home/z/rounds/ROUND-526"
CLAIM = ("CLAIM | AG-23 | dcp400+dcp600 dcp-низ leg-2 (зазор 240-700, "
         "0-клейм) r1136/s9000 bench-v2 | 2 POST")
LEGS = [
    ("swarm-526-23", "400", "525023"),
    ("swarm-526-23b", "600", "526023"),
]
assert len(CLAIM) <= 120, f"CLAIM {len(CLAIM)} ch > 120"


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
    """Чужие CLAIM на dcp-низ/ось = стоп (живой GET, канон AG-157/211)."""
    for ln in text.splitlines():
        if "AG-23" in ln or not ln.startswith("CLAIM"):
            continue
        low = ln.lower()
        if re.search(r"\bdcp(300|400|500|600)\b", low) or "dcp-ось" in low \
                or "dcp-доза" in low or "dcp-dose" in low or "dcp-низ" in low \
                or "drain_cap_polls" in low and re.search(r"(400|600)", low):
            return f"RACE dcp-низ: {ln[:100]}"
    return None


def pick_pin(tok):
    """PIN = master tip (канон AG-243) c tree FULL>=3200 + yml канон; fallback."""
    for pin in (None, PIN_FALLBACK):
        if pin is None:
            pin = api(tok, f"/repos/{REPO}/git/ref/heads/master")["object"]["sha"]
        c = api(tok, f"/repos/{REPO}/commits/{pin}")
        tree_sha = c["commit"]["tree"]["sha"]
        tree = api(tok, f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
        n = len(tree.get("tree", []))
        trunc = tree.get("truncated", False)
        yblob = next((x["sha"] for x in tree.get("tree", [])
                      if x["path"] == f".github/workflows/{WF}"), "?")
        print(f"pin={pin[:8]} tree={n} truncated={trunc} bv2-yml={yblob[:8]}", flush=True)
        if not trunc and n >= 3200 and yblob == YML_BLOB:
            return pin, n
    sys.exit("FATAL: нет валидного PIN")


def find_runs(tok, pre_ids, pin, pages=3):
    found = {}
    branches = {x for x, _, _ in LEGS}
    for page in range(1, pages + 1):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in branches and run["id"] not in pre_ids:
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
    # --- 1. PIN вериф: tree FULL + bench-v2.yml blob (канон AG-208/239/278) ---
    pin, n = pick_pin(tok)
    # --- 2. 0-клейм чек по ЖИВОЙ remote доске ---
    _, text = board_read(tok)
    race = race_check(text)
    if race:
        print(race, flush=True)
        sys.exit(3)
    print("0-клейм OK: dcp400/dcp600 свободны (живой GET)", flush=True)
    # --- 3. CLAIM ДО РАБОТЫ ---
    board_append(tok, [CLAIM], "board: AG-23 CLAIM dcp400+dcp600 low dose (wave-526)")
    # --- 4. снапшот ран-идов + refs zero-code @PIN ---
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100")
           .get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for ref_name, _, _ in LEGS:
        try:
            r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                    data={"ref": f"refs/heads/{ref_name}", "sha": pin})
            print(f"ref POST {ref_name}: 201", flush=True)
        except urllib.error.HTTPError:
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")
            if g.get("object", {}).get("sha", "") != pin:
                sys.exit(f"FATAL: ref {ref_name} exists with other sha")
        ver = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")["object"]["sha"]
        print(f"branch {ref_name}: sha={ver[:8]} match={ver == pin}", flush=True)
        if ver != pin:
            sys.exit(f"FATAL: ref sha mismatch {ref_name}")
    # --- 5. Диспатчи (≤2), 2-й через 31s (FAIL AG-338) ---
    for i, (ref_name, dcp, seed) in enumerate(LEGS):
        inputs = {"radius_blocks": "1136", "run_seconds": "9000", "seed": seed,
                  "server_xmx": "10G", "bench_dims": "minecraft:overworld",
                  "dim_gen_window": "256", "drain_cap_polls": dcp}
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} dcp={dcp} seed={seed}: POST ok", flush=True)
        if i == 0:
            time.sleep(31)
    # --- 6. GET-вериф run-ids (×3 страницы канон AG-197) ---
    time.sleep(8)
    found = find_runs(tok, pre, pin)
    if len(found) < len(LEGS):
        print("WARN: не оба рана видимы — вериф повторит --finalize", flush=True)
    payload = {"pin": pin, "tree_files": n, "claim": CLAIM,
               "runs": dict(found),
               "legs": [{"branch": b, "dcp": d, "seed": s} for b, d, s in LEGS]}
    json.dump(payload, open(f"{RD}/work/AG-23/dispatch_526_23.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {pin} TREE {n}", flush=True)


def phase_finalize():
    tok = token()
    p = json.load(open(f"{RD}/work/AG-23/dispatch_526_23.json"))
    pin, n = p["pin"], p["tree_files"]
    if len(p["runs"]) < len(LEGS):
        found = find_runs(tok, set(), pin, pages=3)
        if found:
            p["runs"] = {**p["runs"], **found}
            json.dump(p, open(f"{RD}/work/AG-23/dispatch_526_23.json", "w"),
                      indent=1, ensure_ascii=False)
    ids = p["runs"]
    ra = ids.get("swarm-526-23", 0)
    rb = ids.get("swarm-526-23b", 0)
    pin8 = pin[:8]
    lines = [
        (f"FACT | AG-23 | 2/2 204 @{pin8} t{n}: {ra} dcp400 s525023 + {rb} dcp600 "
         f"s526023 QUEUED | api"),
        (f"DISP | AG-23 | dcp400+dcp600 dcp-низ 2/2 queued @swarm-526-23[ab] "
         f"r1136/s9000; payload work/AG-23 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-23 | files=work+claims/AG-23 | idea=dcp-low dose "
         f"400/600 leg-2 ladder | ev=2/2 @{pin8}"),
    ]
    for l in lines:
        assert len(l) <= 120, f"{len(l)} ch > 120: {l}"
    ok = board_append(tok, lines, "board: AG-23 fact+disp+patch dcp400+dcp600 queued (wave-526)")
    print("BOARD-LINES-OK" if ok else "BOARD-LINES-FAIL", flush=True)
    # --- payload-файлы: rounds + c-crussty mirror (канон AG-234/239/278) ---
    for dst in (f"{RD}/work/AG-23", "/home/z/c-crussty/work/AG-23"):
        os.makedirs(dst, exist_ok=True)
        if os.path.abspath(f"{dst}/dispatch_526_23.json") != os.path.abspath(
                f"{RD}/work/AG-23/dispatch_526_23.json"):
            shutil.copy(f"{RD}/work/AG-23/dispatch_526_23.json", f"{dst}/dispatch_526_23.json")
        if not (os.path.exists(f"{dst}/dispatch_526_23.py")
                and os.path.samefile(__file__, f"{dst}/dispatch_526_23.py")):
            shutil.copy(__file__, f"{dst}/dispatch_526_23.py")
    print("PAYLOAD mirrored", flush=True)
    print(f"FINAL ra={ra} rb={rb} pin={pin8}", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
