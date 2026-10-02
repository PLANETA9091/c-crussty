#!/usr/bin/env python3
"""dispatch_478_w5_gc2ab.py — COMMANDER 478-W5: gc2-канал A/B vs gc3-банк.

CLAIM: gc1 young-стена = pause-target-артефакт (Л-478-A4: gc1 638ev/44.1s → gc2
151ev/19.4s, gc2 ∈ gc3-bank [18.3,25.6]s). Задача W5: подтвердить gc2 STW-коридор
×2 репликами на canon-векторе x466-C98 (гейт STW ≤23 ∧ young ≤200ev, A4-канон M1)
→ GC2-READY {числа ×2}. B-плечо (gc3) = банк: лестница A4 [18.3,25.6]s + fresh
B5-a2/a3 (21.43/21.64s, тот же вектор). 0 код-дельт, lever gc_tune=2 обе ноги,
алиасы round-478-w5-gc2a/gc2b, 1 диспатч = 1 ветка (Л188b).
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# Канон x466-C98 ЯВНЫМ JSON (урок C66-C72), единственная дельта = gc_tune 3→2.
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "2", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}

ALIASES = ["round-478-w5-gc2a", "round-478-w5-gc2b"]


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


def verify_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    got = r.get("object", {}).get("sha")
    ok = got == sha
    print(f"{name}: full-sha verify {'OK' if ok else f'MISMATCH {got}'}")
    return ok


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
    for a in ALIASES:
        if not verify_alias(tok, a, sha):
            sys.exit(2)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    for a in ALIASES:
        dispatch(tok, a)
        time.sleep(20)  # пауза 20s (канон очереди)
    time.sleep(25)
    runs = list_runs(tok, set(ALIASES), mc)
    print(json.dumps(runs, indent=1))


if __name__ == "__main__":
    main()
