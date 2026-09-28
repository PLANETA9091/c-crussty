#!/usr/bin/env python3
"""dispatch_478_b5_anchors.py — COMMANDER 478-B5: 3 vanilla anchors @master.

CLAIM chk-19 (BANK_V5_FREEZE §2, заморожено): leg_v5 +16.11, порог якоря norm ≤ −3.89,
окно GLOB 6.0-9.5M; существующие маржи пар 22.5→23.2 (a22), 22.6→23.3 (a53) — 2/3,
нужен 3-й легальный якорь → min-of-3 ≥ +20 → PAIR.

Ваниль-якоря: алиасы round-478-b5-a1..a3 @origin/master, lever_flag/arg ПУСТО
(0 код-дельт, master default = vanilla), canon-вектор x466-C98 явным JSON
(урок C73: дефолты yml = merge-поверхность): 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/
pop150k/seed42/10G/xms4G/band[6000000,9500000].
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}

ALIASES = ["round-478-b5-a1", "round-478-b5-a2", "round-478-b5-a3"]


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
        print(f"{name}: exists @ {sha[:8]}")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created @ {sha[:8]}")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def list_runs(tok, branches, min_created):
    res = {}
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        hb = run["head_branch"]
        if hb in branches and run["created_at"] > min_created:
            if hb not in res:
                res[hb] = {"id": run["id"], "status": run["status"],
                           "conclusion": run["conclusion"], "sha": run["head_sha"][:8],
                           "created": run["created_at"]}
    return res


def main():
    tok = token()
    sha = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "origin/master"],
                         capture_output=True, text=True).stdout.strip()
    print(f"master = {sha}")
    for a in ALIASES:
        ensure_alias(tok, a, sha)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    for a in ALIASES:
        dispatch(tok, a)
        time.sleep(3)
    time.sleep(25)
    runs = list_runs(tok, set(ALIASES), mc)
    print(json.dumps(runs, indent=1))


if __name__ == "__main__":
    main()
