#!/usr/bin/env python3
"""dispatch_477_c61_reworld.py — Task 477-C61: confinement POP-gate RE-DISPATCH ×3 мира.
Ветка round-477-c61-conf-reworld @72eed9ae (base carrier 9f6c784b + C61 harness fix).
Фикс харнесса (BenchPopulationPlugin + run_world3.sh + гейты ×2 WF): wedge-kill
attempt-cap, corruption-guard abort+re-arm, start-gate loadedChunks/forceload,
POP_TIMEOUT 1800s. Канон-вектор C75: gc3, rt4/bc1, pop150k seed42, r640/300s/fp4,
ic1/fd1, 10G/xms4G, band [6.0,9.5]M, lever_flag=cmp475_c30conf.
Мир-носители ×3 (C30-474/C75 фатал-миры): dp2/totemA/Trek.
"""
import json, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCH = "round-477-c61-conf-reworld"

LEGS = {
    "dp2":   "https://github.com/PLANETA9091/c-crussty/releases/download/v471-s74-dp/world471-s74-dp.zip",
    "totem": "https://github.com/PLANETA9091/c-crussty/releases/download/v470-s74-totem/world470-s74-totem-v1.zip",
    "trek":  "https://github.com/PLANETA9091/c-crussty/releases/download/v472-s35-dp1/world472-s35-dp1.zip",
}

def token():
    return open("/tmp/gh_token").read().strip()

def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "c61-reworld-dispatcher"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}

def inputs_for(world_url):
    return {
        "world_url": world_url,
        "radius": "640", "seconds": "300", "fake_players": "4",
        "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0",
        "region_threads": "4", "batch_collector": "1",
        "inside_bitmask": "0", "skip_store_bb": "0",
        "region_steal": "0", "bu_defer": "0",
        "population_target": "150000", "population_seed": "42",
        "server_xmx": "10G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp475_c30conf", "lever_arg": "",
    }

assert len(inputs_for("x")) <= 25, "GitHub 25-input limit!"

def main():
    tok = token()
    head = api(tok, f"/repos/{REPO}/commits/{BRANCH}")["sha"]
    print(f"branch {BRANCH} HEAD {head[:8]} (9f6c784b carrier + C61 fix 72eed9ae)")

    run_ids = {}
    for key, world in LEGS.items():
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": BRANCH, "inputs": inputs_for(world)})
        print(f"DISPATCHED leg={key} branch={BRANCH} world={world.rsplit('/', 1)[-1]} lever=cmp475_c30conf gc3 rt4/bc1 pop150k")

    # run-id discovery: 3 runs share head_sha; disambiguate by created_at order
    # per leg (dispatch order) — poll until 3 found
    found = {}
    for _ in range(36):
        time.sleep(10)
        rs = api(tok, f"/repos/{REPO}/actions/runs?head_sha={head}&per_page=30")["workflow_runs"]
        cand = [r for r in rs if (r["head_branch"] or "") == BRANCH
                and r["event"] == "workflow_dispatch" and r["id"] not in found]
        for r in cand:
            found[r["id"]] = r
        if len(found) >= 3:
            break
        print(f"... waiting run registration ({len(found)}/3)")
    ids = sorted(found.keys())
    for key, rid in zip(LEGS.keys(), ids):
        run_ids[key] = rid
        print(f"RUN-ID {rid} leg={key} status={found[rid]['status']} created={found[rid]['created_at']}")
    print("=== SUMMARY ===")
    for k, v in run_ids.items():
        print(f"{k}: {v or 'PENDING (poll by head_branch=round-477-c61-conf-reworld)'}")

if __name__ == "__main__":
    main()
