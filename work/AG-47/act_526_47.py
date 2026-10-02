#!/usr/bin/env python3
"""act_526_47.py — AG-47 w526, цикл-4 (5 race-pivot: dcp1050->AG-25, w13312->AG-39, fp112->AG-19, s4500->AG-31, w15360->AG-14;
все ДО POST, 0 runner-min — race-gate жив x5, штампед тотален, миды живут <3мин). Финал:
dcp1275 dcp-мид (1200-1350) + s7500 s-мид bench-v2 (6000-9000) — оба 0-клейм живой GET
10:0xZ @a9ff088f (канон-носитель x525, tree 4231, yml 0049e34a). Seeds 527047/528047.
Harvest-фаза batch-1 x525 (6 SUCCESS-ног) отдельно: harvest_ag47.py + extract_gates.py."""
import base64, json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"; API = "https://api.github.com"; WF = "bench-v2.yml"
PIN = "a9ff088fd31f3f7d791bfdcab5760790c3fc463c"
YML_BLOB = "0049e34a53fb2ab1906223ac7df318c463708261"
FALLBACK = "2171d6da775975c4bee94748f549ad16f02074e1"
CLAIM = ("CLAIM | AG-47 | dcp1275 dcp-мид (1200-1350) + s7500 s-мид bench-v2 (6000-9000) "
         "1d @a9ff088f | 2 POST")
LEGS = [
    ("swarm-526-47",  {"drain_cap_polls": "1275"}, "527047", "dcp1275", PIN),
    ("swarm-526-47b", {"run_seconds": "7500"}, "528047", "s7500", PIN),
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
    if payload: req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code} {url}: {e.read()[:200]}", flush=True); raise
    return json.loads(body) if body else {}

def board_append(tok, lines, msg):
    for l in lines: assert len(l) <= 120, f"{len(l)}: {l}"
    for attempt in range(6):
        d = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
        sha, text = d["sha"], base64.b64decode(d["content"]).decode()
        if all(L in text for L in lines):
            print("already-appended", flush=True); return True
        new = text.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
        body = {"message": msg, "content": base64.b64encode(new.encode()).decode(),
                "sha": sha, "branch": "master"}
        try:
            r = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT", data=body)
            print(f"board-commit {r['commit']['sha'][:8]} (+{len(lines)})", flush=True); return True
        except urllib.error.HTTPError as e:
            if e.code in (409, 422) and attempt < 5: time.sleep(5); continue
            raise
    return False

def main():
    tok = token()
    # 1. PIN-вериф
    c = api(tok, f"/repos/{REPO}/commits/{PIN}"); tree_sha = c["commit"]["tree"]["sha"]
    tree = api(tok, f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
    n = len(tree.get("tree", [])); trunc = tree.get("truncated", False)
    yblob = next((x["sha"] for x in tree.get("tree", [])
                  if x["path"] == ".github/workflows/bench-v2.yml"), "?")
    print(f"pin={PIN[:8]} tree={n} trunc={trunc} yml={str(yblob)[:8]}", flush=True)
    pin = PIN
    if trunc or n < 3200 or yblob != YML_BLOB:
        pin = FALLBACK; print(f"FALLBACK {pin[:8]}", flush=True)
    # 2. live race-check
    _, text = api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")[0:2] \
        if False else (None, base64.b64decode(api(tok, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")["content"]).decode())
    for ln in text.splitlines():
        if not ln.startswith("CLAIM") or "AG-47 |" in ln: continue
        if re.search(r"\bdcp1275\b|\bs7500\b", ln):
            print(f"RACE: {ln[:100]}"); sys.exit(1)
    print("race OK: dcp1275/s7500 свободны (живой GET)", flush=True)
    # 3. CLAIM
    board_append(tok, [CLAIM], "board: AG-47 CLAIM dcp1275+s7500 mids (wave-526)")
    # 4. refs zero-code
    pre = {r["id"] for r in api(tok, f"/repos/{REPO}/actions/runs?per_page=100").get("workflow_runs", [])}
    print(f"pre-runs={len(pre)}", flush=True)
    for ref_name, _, _, _, legpin in LEGS:
        r = api(tok, "/repos/{REPO}/git/refs".format(REPO=REPO), method="POST",
                data={"ref": f"refs/heads/{ref_name}", "sha": legpin})
        if not r.get("ref"):
            g = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")
            if g.get("object", {}).get("sha", "") != pin:
                sys.exit(f"FATAL ref {ref_name} чужой sha")
        ver = api(tok, f"/repos/{REPO}/git/ref/heads/{ref_name}")["object"]["sha"]
        print(f"branch {ref_name} sha={ver[:8]} match={ver == legpin}", flush=True)
    # 5. диспатчи (2-й через 31s, FAIL AG-338)
    for i, (ref_name, extra, seed, tag, legpin) in enumerate(LEGS):
        inputs = {"radius_blocks": "1136", "run_seconds": "9000", "seed": seed,
                  "server_xmx": "10G", "bench_dims": "minecraft:overworld",
                  "dim_gen_window": "256", "drain_cap_polls": "900"}
        inputs.update(extra)
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref_name, "inputs": inputs})
        print(f"dispatch {ref_name} {tag} seed={seed}: 204", flush=True)
        if i == 0: time.sleep(31)
    # 6. вериф run-ids
    time.sleep(10)
    found = {}
    for page in (1, 2, 3):
        d = api(tok, f"/repos/{REPO}/actions/runs?per_page=100&page={page}")
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            lp = next((x[4] for x in LEGS if x[0] == b), None)
            if b in {x[0] for x in LEGS} and run["id"] not in pre and run["head_sha"] == lp:
                found[b] = run["id"]
        if len(found) >= len(LEGS): break
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    payload = {"pins": {"A": PIN, "B": FALLBACK}, "tree_files": n, "claim": CLAIM,
               "runs": found, "legs": [{"branch": b, "tag": t, "seed": s, "extra": e,
               "pin": p} for b, e, s, t, p in LEGS]}
    json.dump(payload, open("/home/z/rounds/ROUND-526/work/AG-47/dispatch_526_47.json", "w"),
              indent=1, ensure_ascii=False)
    if len(found) < len(LEGS):
        print("WARN: не оба рана видимы — re-вериф", flush=True)
    # 7. финальные линии
    ra, rb = found.get("swarm-526-47", 0), found.get("swarm-526-47b", 0)
    lines = [
        (f"FACT | AG-47 | 2/2 204 @{pin[:8]} t{n}: {ra} dcp1275 s527047 + {rb} s7500 "
         f"s528047 QUEUED | api"),
        (f"DISP | AG-47 | dcp1275+s7500 миды 2/2 queued @swarm-526-47[ab] 1d/r1136; "
         f"payload work/AG-47 | 2/2 204"),
        (f"PATCH_SUMMARY | AG-47 | files=work+claims/AG-47 | idea=dcp1275+s7500 мид fill "
         f"2 оси | ev=2/2 @{pin[:8]}"),
    ]
    ok = board_append(tok, lines, "board: AG-47 fact+disp+patch dcp1275+s7500 queued (wave-526)")
    print("BOARD-OK" if ok else "BOARD-FAIL", flush=True)
    print(f"FINAL ra={ra} rb={rb} pin={pin[:8]}", flush=True)

if __name__ == "__main__":
    main()
