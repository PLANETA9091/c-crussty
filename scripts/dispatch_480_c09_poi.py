#!/usr/bin/env python3
"""dispatch_480_c09_poi.py — COMMANDER 480-C09: POI-плоскость окно-фид, 3-й якорь охота.

CLAIM (POI-окно 2/3, v5-FROZEN): окно [8907260,9007260], порог якоря ≤−1.99
(poi456-4 leg +18.01; 2/3 держится, нужен 3-й якорь для вердикта ноги poi456-4
+18.01, Л-477-C65 — POI-ось ампутирована АИ-мержем ×10-18, окно держится 2/3).

ЗАДАЧ: 2 ваниль-якоря world-bench-parallel @686f2258 (origin/master == HEAD,
0 код-дельт), алиасы round-480-c09-p1 / round-480-c09-p2, canon x466-C98 ЯВНЫМ
JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever=""),
band GLOB [6.0,9.5]M fast-fail. Окно В ГЕЙТ НЕ СТАВИТЬ (пост-хок фильтр по
run-env — канон Л-470-S31.1: банк-фид полным band + пост-хок idx-фильтр).

ГЕЙТЫ (preregister, закон 14a/16):
  (1) band PASS [6.0,9.5]M fast-fail; band-dead = free discard (Л188c) → ре-ролл ≤2;
  (2) hit = cpu ∈ [8907260,9007260] (ТОЛЬКО из run-env.txt, Л195) ∧ norm_v5 ≤ −1.99
      ∧ vanilla-valid Л209 → 3-й якорь poi456-4 → 3/3 → МЕРЖ-хвост +18.01;
  (3) CLEAN-ценз M1 gc.log-primary: STW ≤23.0s ∧ young avg ≤200ms (gc3-класс);
  (4) вне окна in-band CLEAN → банк-фид v5; ваниль-коридор [−8.0,+1.5] (Л143).
Runs >15 мин = DISPATCHED run-id (закон 12e/18-iii). LEDGER Л-480-C09.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master == HEAD, 0 код-дельт

W_LO, W_HI = 8907260, 9007260  # POI-окно v5-FROZEN (инфо-дельта, НЕ в band-гейт)

LEGS = ["round-480-c09-p1", "round-480-c09-p2"]

# canon x466-C98 ЯВНЫЙ JSON (ваниль-якорь, pop150k POI-канон 474-C12)
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


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, ok404=False):
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
        if e.code == 404 and ok404:
            return None
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}", ok404=True)
    cur = (r or {}).get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]}", flush=True)
        return True
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        cur = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
        print(f"{name}: moved -> {cur[:8]}", flush=True)
        return cur == sha
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    cur = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    ok = cur == sha
    print(f"{name}: created FULL-sha @ {cur[:8]} verify={'OK' if ok else 'FAIL'}", flush=True)
    return ok


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    ok = r == {}
    print(f"dispatch {ref}: {'204 OK' if ok else r}", flush=True)
    return ok


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
    print(f"origin/master live = {live}", flush=True)
    pin = live if live.startswith(PIN[:12]) else PIN
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    results = []
    for alias in LEGS:
        if not ensure_alias(tok, alias, pin):
            print(f"{alias}: REF-VERIFY FAIL — skip", flush=True)
            results.append({"alias": alias, "error": "ref-verify-fail"})
            continue
        if not dispatch(tok, alias, INPUTS):
            results.append({"alias": alias, "error": "dispatch-fail"})
            continue
        time.sleep(20)
        run = latest_run(tok, alias, mc)
        print(json.dumps({"alias": alias, **(run or {})}, indent=1), flush=True)
        results.append({"alias": alias, "window": [W_LO, W_HI],
                        "anchor_gate": "norm_v5 <= -1.99 (post-hoc run-env)",
                        **(run or {})})
        time.sleep(5)
    print("C09-DISPATCH-JSON " + json.dumps(results), flush=True)


if __name__ == "__main__":
    main()
