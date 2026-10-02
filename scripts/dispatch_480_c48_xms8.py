#!/usr/bin/env python3
"""dispatch_480_c48_xms8.py — COMMANDER 480-C48: young-heap-геометрия, xms8 ×1.

CLAIM (young-предзаготовка, унаследована от ×480-C05): young-масса = главный
STW-ботлнек (STW_total = young_mass + 0.954×fulls, R²=0.66; r01 fulls=0 →
STW 30.14s = young 98.2%). xms (initial heap) меняет generational-границы
+ pre-touch morphology → young-морфология (young_n / young_avg / young-масса)
должна сдвинуться при xms 4G→8G при фиксированном xmx 10G.

ЗАДАЧ: 1 диспатч world-bench-parallel @master 686f2258 (0 код-дельт: прегист
только scripts/board — diff-страж 0 bench-поверхностей), алиас
round-480-c48-xms8, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/
bc1/pop150k/seed42/10G/**xms8G**, lever="" ваниль), band GLOB [6000000,9500000]
fast-fail, band-miss → 1 ре-ролл (закон W3).

ПРЕГИСТ-ГЕЙТ (закон 14a/16, пороги v5-FROZEN не двигаются):
G1 band [6.0,9.5]M; G2 armed=null ∧ ncdfe=0 ∧ aioobe=0;
G3 young_n ∈ [100,130] ∧ young_avg ∈ [100,150] (изолят C05 G4);
G4 STW≤23s (M1); G5 norm_v5 ∈ [−8.0,+1.5] (Л143).
Вердикт-дерево: xms8 = 0 код-дельт heap-флаг → продукт = v6-датасет young-
морфологии (Δ young_n/young_avg/STW vs xms4G-твины canon-банка), НЕ merge-лег;
any-band-miss → 1 ре-ролл/точку. Runs >15 мин → DISPATCHED run-id (закон 18-iii).
LEDGER Л-480-C48.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master, 0 код-дельт

LEGS = [
    {"alias": "round-480-c48-xms8", "xms": "8G"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_seed": "42",
    "server_xmx": "10G", "server_xms": "8G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}
# population_target = canon 150000 (x466-C98 pop150k) — задаётся явно (урок C73)


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
    print(f"dispatch {ref} xms={inputs['server_xms']} -> "
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
        alias, xms = leg["alias"], leg["xms"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        inputs["server_xms"] = xms
        inputs["population_target"] = "150000"
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, "xms": xms, **(run or {})}, indent=1))
        results.append({"alias": alias, "xms": xms, **(run or {})})
        time.sleep(5)
    print("C48-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
