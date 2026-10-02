#!/usr/bin/env python3
"""dispatch_483_a05_wildcells.py — COMMANDER 483-A05: WILD-клетки ×2.

CLAIM (Task 2-A05, тик ×483):
  нога1 fake_players=8 (population_seed 42) — fp-ось, канон fp4;
  нога2 population_seed=43 (fake_players 4) — seed-ковариата ±5пп (канон C49).
PIN: acffa3839b09a3388d4949d3767ab0a15b4fcd94 (master ×482-учёт, BOTTLENECK.md).
Канон x466-C98 ЯВНЫМ JSON: radius 640, seconds 300, fluid_guard 1, gc_tune 3,
inside_cache 1, flush_diet 1, fluid_dirty 0, fluid_bitmask 0, region_threads 4,
batch_collector 1, inside_bitmask 0, skip_store_bb 0, region_steal 0, bu_defer 0,
population_target 150000, server_xmx 10G, server_xms 4G (канон! не 8G),
cpu_band [6000000,9500000] fast-fail, lever_flag/arg "".
0 код-дельт: ветки-алиасы на PIN, диспатч только (Л188a/b). 1 диспатч = 1 ветка.
BAND: fast-fail "Runner calibration band gate" = band-miss → ре-ролл ≤2
(алиасы -r2/-r3). >15 мин = DISPATCHED run-id (закон 12e).
ЗАПРЕТЫ закона 5 соблюдены (нет ZGC/alloc_diet/zero_alloc/flat_traversal/
fluid_dirty-мемо/inside_bitmask#15/fluid_bitmask#16/THP/RECON-42/players-16).
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master ×482-учёт

LEGS = [
    {"alias": "round-483-a05-fp8",    "fake_players": "8", "population_seed": "42"},
    {"alias": "round-483-a05-seed43", "fake_players": "4", "population_seed": "43"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000",
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
        print(f"{name}: GET-proof exists @ {cur[:12]}")
        return True
    if cur:
        print(f"{name}: EXISTS @ {cur[:12]} != PIN {sha[:12]} — NOT moving foreign branch")
        return False
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    # GET-verify (Л188a)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    got = v.get("object", {}).get("sha", "")
    ok = got == sha
    print(f"{name}: created + verified {'OK' if ok else 'FAIL (' + got[:12] + ')'} @ {sha[:12]}")
    return ok


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} fp={inputs['fake_players']} seed={inputs['population_seed']} -> "
          f"{'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def main():
    tok = token()
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    results = []
    for leg in LEGS:
        alias = leg["alias"]
        if not ensure_alias(tok, alias, pin):
            results.append({"alias": alias, "error": "alias-conflict"})
            continue
        inputs = dict(BASE_INPUTS)
        inputs["fake_players"] = leg["fake_players"]
        inputs["population_seed"] = leg["population_seed"]
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            results.append({"alias": alias, "error": "dispatch-failed"})
            continue
        time.sleep(25)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, "fp": inputs["fake_players"],
                          "seed": inputs["population_seed"], **(run or {})}, indent=1))
        results.append({"alias": alias, "fp": inputs["fake_players"],
                        "seed": inputs["population_seed"], **(run or {})})
        time.sleep(5)
    print("A05-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
