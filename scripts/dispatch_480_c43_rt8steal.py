#!/usr/bin/env python3
"""dispatch_480_c43_rt8steal.py — COMMANDER 480-C43: чистая rt8+STEAL пара (матрица-клетка).

CLAIM (матрица region-осей ×479/×480): x08 = rt8 duty (steal=0), x09 = bu_defer+steal @rt4,
G6 = STEAL+bu_defer @rt4 честный CLEAN — norm −4.40, barrier 1.579% vs A15 2.03% (Δ−0.451пп)
→ соло REFUTED. Чистая rt8×steal клетка НЕ гонялась; rt8-пара = другая фаза (W=8, тоньше
бакеты, 8-way барьер) — G4-прегист: barrier-доля vs G6-база 1.579%; capture ≥2пп? Прегист-
ответ НЕТ (потолок ≤0.6пп C29, дивизор rt8). Прегист: /home/z/rounds/ROUND-480/c43/
PREREG_480_C43_RT8STEAL.md (закон 14a/16, ДО диспатча).

ЗАДАЧ: 1 диспатч world-bench-parallel @master 686f2258 (0 код-дельт), алиас
round-480-c43-rt8steal, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt8/bc1/
steal1/bu0/pop150k/seed42/10G/xms4G, lever="" ваниль), band GLOB [6000000,9500000]
fast-fail, band-miss → 1 ре-ролл (закон W3).

ПРЕГИСТ-ГЕЙТЫ: G1 delivery (rt8∧steal1∧bu0∧canon∧afb3a0b3); G2 NPE-FREE (s7176-риск при
bu_defer=0 — честный ценз); G3 M1 STW≤23s ∧ young_avg≤200ms (v5-FROZEN); G4 barrier-доля
once-per-stack vs 1.579% (классификация ≤1.2 / (1.2,2.0] / >2.0); G5 capture-матем ≥2пп?
G6 pair norm_v5 ≥+20 (honest P≤2%). LEDGER Л-480-C43. Runs >15 мин → DISPATCHED run-id
(закон 18-iii).
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # master live, 0 код-дельт

LEGS = [
    {"alias": "round-480-c43-rt8steal"},
]

BASE_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",          # C43-ось: rt8 (x479-x08 duty = rt8 duty)
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "1",            # C43-ось: STEAL v1, ЧИСТАЯ пара (bu_defer=0)
    "bu_defer": "0",
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
    print(f"dispatch {ref} rt={inputs['region_threads']} steal={inputs['region_steal']} -> "
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
        alias = leg["alias"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1))
        results.append({"alias": alias, **(run or {})})
        time.sleep(5)
    print("C43-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
