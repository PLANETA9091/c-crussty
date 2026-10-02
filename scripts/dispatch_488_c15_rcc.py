#!/usr/bin/env python3
"""dispatch_488_c15_rcc.py — RCC-зонд-нога ×488 (командир C15, закон 14a/16 prereg).

Плоскость: 19b стена 215-221k = CodeCache × pop^2.9-3.1 (C87). Нога: 215k + gc_tune=6
(ParallelGC+MetaspaceSize256M+RCC512M) @ xmx12G против 215k gc6 (C15 1.7 ×487, n=1) —
репликация стеновой клетки + первый 215k-член gc6@12G-лестницы тика (g6a-f = 165/205/210k).
Прогноз/фальсификатор: /home/z/rounds/ROUND-488/board/CLM-C15.md (prereg ДО диспатча).
env-only, 0 код-дельт: ветка = голый ref @ master 13a955a3 (push origin, master НЕ тронут).
"""
import json, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCH = "round-488-c15-rcc"
PIN = "13a955a3"  # full sha тянем из API; пин = master этого тика

WORLD = "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip"

def tok(): return open("/tmp/gh_token").read().strip()

def api(url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok()}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload: req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            b = r.read()
        return json.loads(b) if b else {}
    except urllib.error.HTTPError as e:
        return {"_http_error": e.code, "_err": e.read().decode("utf-8", "replace")[:300]}

def canon(**over):
    d = {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
        "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
        "datapack_url": "", "population_target": "150000", "population_seed": "42",
        "server_xmx": "10G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "", "lever_arg": "",
    }
    d.update(over); return d

def main():
    # 0) master pin (full sha)
    m = api(f"/repos/{REPO}/git/ref/heads/master")
    master = m["object"]["sha"]
    assert master.startswith(PIN), f"master {master} != pin {PIN}"
    print(f"master pin = {master}")

    # 0b) снапшот run-ids ветки ДО диспатча (атрибуция Л-470-S31.1)
    pre = api(f"/repos/{REPO}/actions/runs?branch={BRANCH}&per_page=100")
    pre_ids = {r["id"] for r in pre.get("workflow_runs", [])}
    print(f"pre-dispatch run-ids on {BRANCH}: {len(pre_ids)}")

    # 1) push ref: origin 13a955a3:refs/heads/round-488-c15-rcc (git-клиент, токен в remote)
    import subprocess
    r = subprocess.run(["git", "push", "origin", f"{master}:refs/heads/{BRANCH}"],
                       capture_output=True, text=True, timeout=120)
    print((r.stdout + r.stderr).strip()[-400:])
    assert r.returncode == 0, "git push ref failed"

    # 2) GET-верификация object.sha ДО dispatch (Л188a/S20-урок)
    ref = api(f"/repos/{REPO}/git/ref/heads/{BRANCH}")
    sha = ref.get("object", {}).get("sha", "")
    assert sha == master, f"branch sha {sha} != master {master}"
    print(f"branch {BRANCH} verified @ {sha}")

    # 3) inputs: 215k + gc6 + 12G, сервер-канон остальной (prereg CLM-C15.md)
    inputs = canon(gc_tune="6", server_xmx="12G", population_target="215000")
    r = api(f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": BRANCH, "inputs": inputs})
    assert "_http_error" not in r, f"dispatch failed: {r}"
    print(f"dispatch POST ok (HTTP 204): {BRANCH} inputs={ {k: inputs[k] for k in ('gc_tune','server_xmx','population_target')} }")

    # 4) двусторонняя атрибуция: GET run.head_branch И GET runs?branch= (Л-470-S31.1)
    run_id = None
    for _ in range(12):
        time.sleep(10)
        runs = api(f"/repos/{REPO}/actions/runs?branch={BRANCH}&per_page=100")
        new = [x for x in runs.get("workflow_runs", []) if x["id"] not in pre_ids]
        if new:
            x = new[0]
            if x["head_branch"] == BRANCH and x["head_sha"] == master:
                run_id = x["id"]
                print(f"RUN VERIFIED: id={run_id} head_branch={x['head_branch']} head_sha={x['head_sha'][:8]} status={x['status']} created={x['created_at']}")
                break
    if run_id is None:
        print("WARN: run not yet visible — проверить GET runs?branch= позже")

    with open("/tmp/abs488/c15_rcc_dispatch.json", "w") as f:
        json.dump({"master": master, "branch": BRANCH, "run_id": run_id,
                   "inputs": inputs}, f, ensure_ascii=False, indent=1)
    print(f"RESULT: branch={BRANCH} run_id={run_id} → /tmp/abs488/c15_rcc_dispatch.json")

if __name__ == "__main__":
    main()
