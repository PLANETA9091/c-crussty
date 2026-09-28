#!/usr/bin/env python3
"""dispatch_480_feeder.py — ROUND-480 wave-2 feeders (закон 15b, прегист 14a/16).

Волны: (1) sensn16-leg ×3 (lever cmp466_c98ai/16, окно якорей [6.80,6.96]M post-hoc;
пары mxa-08 −3.70@6,849,418 / mxa-12 +3.32@6,905,659 / s1-d +0.59@6,888,701, Δ≤50k,
min-of-3; leg ≥+20 → МЕРЖ-кандидат №19). (2) 201k-лестница ×2 (Л-479-A8: (201k,202k]
не исключена; 201k CLEAN → колено ∈(201k,202k] = ступень; DIRTY → колено ≤201k).
(3) canary-480 ×1 ваниль (гейт norm ∈ [−6,+6], мастер-здоровье после R0/F3/doc-пушей).
(4) STRICT-волна ×12 ваниль @live-master (банк 27/30 дефицит 3; hit-rate 12.5-27%).
Пороги v5-FROZEN не двигаются; закон-5 чист.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = (
    [{"alias": f"round-480-l1{c}", "lever": ("cmp466_c98ai", "16")} for c in "abc"] +
    [{"alias": "round-480-p201a", "pop": "201000"},
     {"alias": "round-480-p201b", "pop": "201000"},
     {"alias": "round-480-canary", "canary": True}] +
    [{"alias": f"round-480-s{s:02d}", "vanilla": True} for s in range(1, 13)]
)

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
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": ref, "inputs": inputs})
    print(f"dispatch {ref} pop={inputs.get('population_target')} "
          f"lever={inputs.get('lever_flag') or '-'} -> "
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
    pin = br.get("commit", {}).get("sha", "")
    if not pin:
        print("FATAL: no master sha", file=sys.stderr)
        sys.exit(1)
    print(f"origin/master live = {pin}")
    results = []
    for leg in LEGS:
        alias = leg["alias"]
        inputs = dict(BASE_INPUTS)
        if leg.get("lever"):
            inputs["lever_flag"], inputs["lever_arg"] = leg["lever"]
        if leg.get("pop"):
            inputs["population_target"] = leg["pop"]
        ensure_alias(tok, alias, pin)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            results.append({"alias": alias, "err": "dispatch-fail"})
            time.sleep(10)
            continue
        time.sleep(18)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1))
        results.append({"alias": alias, **(run or {})})
        time.sleep(5)
    print("FEEDER-480-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
