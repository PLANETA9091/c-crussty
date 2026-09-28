#!/usr/bin/env python3
"""dispatch_480_c29_a1.py — COMMANDER C29: ваниль-якорь в верхний коридор [7.2,7.5]M.

ЗАДАЧ (тик ×480, round-480): 1 ваниль-диспатч world-bench-parallel @686f2258
(0 код-дельт, canon x466-C98 явным JSON: 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/
pop150k/seed42/10G/xms4G, lever="" ваниль), алиас round-480-c29-a1,
band GLOB [6000000,9500000] fast-fail. Окно коридора [7200000,7500000] —
ПОСТ-ХОК (не в гейте, прецедент C24/S31.1: окна НЕ в гейте).
Назначение: банк-фид §3 (ваниль-VALID ∧ in-band CLEAN) + pair-fresh пул
(парные якоря: Δcpu ≤50k, pair-fresh Л201).
Band-miss → 1 ре-ролл (закон W3/Л188c), алиас round-480-c29-a1-r2.
Runs >15 мин → DISPATCHED run-id (закон 18-iii), абсорб ×481.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master, ваниль, 0 код-дельт

CORRIDOR = (7200000, 7500000)   # верхний коридор — ПОСТ-ХОК, не гейт
BAND = (6000000, 9500000)       # GLOB fast-fail — ЕДИНСТВЕННЫЙ гейт

LEGS = [
    {"alias": "round-480-c29-a1",    "pop": "150000"},
    {"alias": "round-480-c29-a1-r2", "pop": "150000"},  # только при band-miss
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha}")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha}")


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} pop={inputs['population_target']} -> "
          f"{'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def wait_run(tok, rid, timeout_s=2400, poll_s=30):
    t0 = time.time()
    while time.time() - t0 < timeout_s:
        r = api(tok, f"/repos/{REPO}/actions/runs/{rid}")
        if r.get("status") == "completed":
            return r
        time.sleep(poll_s)
    return None


def cpu_of_run(tok, rid, outdir):
    """cpu из артефакта world3-bench -> run-env.txt (runner_cpu_index)."""
    import os, re, zipfile
    os.makedirs(outdir, exist_ok=True)
    arts = api(tok, f"/repos/{REPO}/actions/runs/{rid}/artifacts")
    aid = next((a["id"] for a in arts.get("artifacts", [])
                if a["name"] == "world3-bench"), None)
    if not aid:
        return None
    z = f"{outdir}/a.zip"
    subprocess.run(["curl", "-sL", "-H", f"Authorization: token {tok}",
                    f"{API}/repos/{REPO}/actions/artifacts/{aid}/zip",
                    "-o", z], check=True)
    with zipfile.ZipFile(z) as zf:
        zf.extract("run-env.txt", outdir)
    os.remove(z)
    env = open(f"{outdir}/run-env.txt").read()
    m = re.search(r"runner_cpu_index:\s*(\d+)", env)
    return int(m.group(1)) if m else None


def main():
    tok = token()
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    results = []
    for leg in LEGS:
        alias, pop = leg["alias"], leg["pop"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        inputs["population_target"] = pop
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, "pop": pop, **(run or {})}, indent=1))
        results.append({"alias": alias, "pop": pop, **(run or {})})
        time.sleep(5)
    print("C29-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
