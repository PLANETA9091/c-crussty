#!/usr/bin/env python3
"""dispatch_483_a02_ic0.py — COMMANDER 483-A02 (Task 2-A02): WILD-ic0 изоляция ×2.

CLAIM: WILD-ic0 — inside_cache=0 ×2 (прецедент C46; канон ic1). Канон x466-C98 несёт
inside_cache="1"; ic0-изоляция снимает кэш → raw-checkInside всплывает → ожидаемый
минус нормы. Миссия = проверка ic-квоты канона на ×483-базе; гейт: ТОЛЬКО ЧИСЛО
Δnorm(ic0−ic1), НЕ merge. LEDGER Л-483-A02. Runs >15 мин → DISPATCHED run-id (18-iii).

ЗАДАЧ: 2 диспатча world-bench-parallel @PIN acffa3839b09a3388d4949d3767ab0a15b4fcd94
(= master ×482-учёт, 0 код-дельт), алиасы round-483-a02-ic0a / round-483-a02-ic0b,
canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic0(ДЕЛЬТА)/fd1/rt4/bc1/pop150k/
seed42/10G/xms4G, lever="" ваниль), band GLOB [6000000,9500000] fast-fail,
band-miss → ре-ролл ≤2 (алиасы -r2/-r3).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master ×482-учёт, 0 код-дельт

LEGS = [
    {"alias": "round-483-a02-ic0a"},
    {"alias": "round-483-a02-ic0b"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3",
    "inside_cache": "0",            # A02-ось: ЕДИНСТВЕННАЯ дельта от канона (ic-квота-тест)
    "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
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
        return True
    if cur:
        print(f"{name}: EXISTS @ {cur} != PIN — NOT touching (chForeign?)")
        return False
    r2 = api(tok, f"/repos/{REPO}/git/refs", method="POST",
             data={"ref": f"refs/heads/{name}", "sha": sha})
    if r2.get("_http_error"):
        return False
    print(f"{name}: created FULL-sha @ {sha}")
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    vsha = v.get("object", {}).get("sha", "")
    print(f"{name}: GET-verify {vsha} -> {'OK' if vsha == sha else 'MISMATCH'}")
    return vsha == sha


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} ic={inputs['inside_cache']} -> "
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


def main():
    tok = token()
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    pin = live if live.startswith(PIN[:12]) else PIN
    print(f"origin/master live = {live} (pin={pin})")
    results = []
    for leg in LEGS:
        alias = leg["alias"]
        if not ensure_alias(tok, alias, pin):
            print(f"{alias}: ENSURE-FAIL, skip dispatch")
            results.append({"alias": alias, "error": "ensure-fail"})
            continue
        inputs = dict(BASE_INPUTS)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            print(f"{alias}: DISPATCH-FAIL")
            results.append({"alias": alias, "error": "dispatch-fail"})
            continue
        time.sleep(45)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1))
        results.append({"alias": alias, **(run or {})})
        time.sleep(10)
    print("A02-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
