#!/usr/bin/env python3
"""dispatch_479_a8_knee.py — COMMANDER 479-A8: W11-добивка, 19b-бисекция young-колена.

CLAIM (19b-лестница, унаследован от ×478-W11 — пал дедлайном, 0 диспатчей): A3-ценз
×478 (min-of-1) young-колено slope ×95 (0.29→27.2 ms/k) на gc3@10G ∈(202k,203k];
202k norm −28.05 / 203k −34.76, оба DIRTY-класса. 200k канон 20.91 CLEAN ×2.
Требуется верификация 201k/202k (первая нога 201k, вторая 202k ×1).

ЗАДАЧ: 2 диспатча world-bench-parallel @master 70d64190 (0 код-дельт: db887807→prereg
только BLACKBOARD/scripts — diff-стат docs/scripts-only), алиасы
round-479-a8-p201 (pop 201000) / round-479-a8-p202 (pop 202000), canon x466-C98
ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/seed42/10G/xms4G, lever="" ваниль),
band GLOB [6000000,9500000] fast-fail, band-miss → 1 ре-ролл/точку (закон W3).

ПРЕГИСТ-ГЕЙТ (закон 14a/16, пороги v5-FROZEN не двигаются): M1 STW≤23s
(∧ young_avg≤200ms вторичный рид-аут). ЕСЛИ ОБЕ НОГИ DIRTY (STW>23) →
молодое-колено = HOST-факт (не лестница-ступень) → REFUTED_CENS числа-потолка
{STW201, STW202}. ЕСЛИ p201 CLEAN ∧ p202 DIRTY → колено уточнено ∈(201k,202k]
= лестница-ступень подтверждена числами. Оба CLEAN → колено держится (202k,203k].
LEDGER Л-479-A8. Runs >15 мин → DISPATCHED run-id (закон 18-iii).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "70d641907769d982497fafbb3c56fb387fe2b204"  # origin/master, 0 код-дельт

LEGS = [
    {"alias": "round-479-a8-p201", "pop": "201000"},
    {"alias": "round-479-a8-p202", "pop": "202000"},
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
    print("A8-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
