#!/usr/bin/env python3
"""dispatch_480_c68.py — COMMANDER 480-C68: КЛИМБ chkclimb-5 фид-волна ×3.

CLAIM (окно 2/3 FROZEN): chkclimb-5 [6427199,6527199] порог ≤−1.60.
C24-дрейф-матем: низ пула статичен ~6.4-6.5M (V1 min 6502014 vs TAIL min
6570145 = −69k) → окно chkclimb-5 на LOW-краю пула, РЕАЛЬНО достижимо.
в-точки 3/40 in-window (r07 6439531/−0.74 best near-miss; edge ±60k ещё 2)
→ edge-зона 12.5%; N(1 хит) ≈23 ваниль-диспатчей; ×3 → P(≥1 hit) ≈ 16-20%.

ЗАДАЧ: 3 диспатча world-bench-parallel @master 686f2258 (0 код-дельт),
алиасы round-480-c68-w1/w2/w3, canon x466-C98 ЯВНЫМ JSON
(640/300s/fp4/gc3/ic1/fd1/rt4/bc1/seed42/pop150000/10G/xms4G, lever=""
ваниль), band GLOB [6000000,9500000] fast-fail, окно ПОСТ-ХОК (не в гейте,
S31.1). band-miss → ≤1 ре-ролл/точку (W3/Л188c).

ПРЕГИСТ-ГЕЙТ: hit = cpu∈[6427199,6527199] ∧ norm_v5≤−1.60 ∧ vanilla-valid
Л209 ∧ M1 STW≤23.0/young_avg≤200 → 3-й хит окна → цикл закона 18.
Вне окна in-band CLEAN → банк-фид 27→28+. Runs >15 мин → DISPATCHED
run-id (закон 12e/18-iii). LEDGER Л-480-C68.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master, 0 код-дельт

LEGS = [
    {"alias": "round-480-c68-w1"},
    {"alias": "round-480-c68-w2"},
    {"alias": "round-480-c68-w3"},
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
POP = "150000"


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
        print(f"{name}: GET-proof exists @ {sha[:12]}")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:12]}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha[:12]}")


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
        alias = leg["alias"]
        ensure_alias(tok, alias, pin)
        inputs = dict(BASE_INPUTS)
        inputs["population_target"] = POP
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, alias, inputs):
            sys.exit(2)
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1))
        results.append({"alias": alias, **(run or {})})
        time.sleep(15)  # стаггер = разные фазы пула (C24 зонд-дроу рецепт)
    print("C68-DISPATCH-JSON " + json.dumps(results))


if __name__ == "__main__":
    main()
