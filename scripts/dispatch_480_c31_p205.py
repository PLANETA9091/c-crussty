#!/usr/bin/env python3
"""dispatch_480_c31_p205.py — COMMANDER 480-C31: 19b entities-лестница, 205k реплика-3.

CLAIM (19b-лестница, канон BLACKBOARD/BOTTLENECK 19b): 165k PASS / 200k 20.91 CLEAN ×2 /
колено ≤202k (202k DIRTY ×2, 201k p201a/b в полёте round-480). 205k серый = 21.90s
(×476 подтверждение 1/1, run 36304204422, CLM-C22) — РЕПЛИКА-3 нужна для ступени.

ЗАДАЧ: 1 диспатч world-bench-parallel @master 686f2258 (0 код-дельт, docs/scripts-only),
алиас round-480-c31-p205 (pop 205000), canon x466-C98 ЯВНЫМ JSON
(640/300s/fp4/gc6/ic1/fd1/rt4/bc1/seed42/12G/xms4G, lever="" ваниль) — gc6@12G класс
канона 20.91-семейства (прецедент C59-реплик «gc6@12G»), band GLOB [6000000,9500000]
fast-fail, band-miss → 1 ре-ролл (закон W3/Л188c).

ПРЕГИСТ-ГЕЙТ (закон 16, пороги v5-FROZEN): M1 STW≤23s (∧ young_avg≤200ms вторично).
205k CLEAN (STW≤23) → лестница-ступень 205k подтверждена (серый 21.90 реплицирован).
205k DIRTY (STW>23) → колено подтверждено ≤202k, разрыв (202k,205k] закрыт DIRTY.
LEDGER Л-480-C31. Runs >15 мин → DISPATCHED run-id (закон 18-iii).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master live, 0 код-дельт

LEGS = [
    {"alias": "round-480-c31-p205", "pop": "205000"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
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
        attempts = 2  # 1 диспатч + 1 ре-ролл на band-miss (W3/Л188c)
        for attempt in range(1, attempts + 1):
            mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
            if not dispatch(tok, alias, inputs):
                sys.exit(2)
            time.sleep(20)
            # полл fast-fail: band GLOB [6.0,9.5]M фейлится в первые минуты
            run = None
            deadline = time.time() + 300
            while time.time() < deadline:
                run = latest_run(tok, alias, mc)
                if run and (run["conclusion"] or run["status"] != "queued"):
                    break
                time.sleep(30)
            print(json.dumps({"alias": alias, "pop": pop, "attempt": attempt,
                              **(run or {})}, indent=1))
            if run and run.get("conclusion") == "failure":
                print(f"ATTEMPT {attempt}: fast-fail (band-miss кандидат) -> "
                      f"{'ре-ролл' if attempt < attempts else 'ре-роллы исчерпаны (W3)'}")
                time.sleep(5)
                continue
            results.append({"alias": alias, "pop": pop, "attempt": attempt,
                            **(run or {})})
            break
        time.sleep(5)
    print("C31-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
