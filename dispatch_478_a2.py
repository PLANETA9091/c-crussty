#!/usr/bin/env python3
"""dispatch_478_a2.py — Task 478-A2: confinement МЕРЖ №17 fixture-фикс РЕ-ДИСПАТЧ ×3 мира.

Ветки-алиасы round-478-a2-{dp2,totem,trek} @ <fix-sha> (base 640926e9 = carrier
9f6c784b C30 SITE-A SCHED-DEFER + C61 harness fix 72eed9ae) — алиасы против
cancel-in-progress (по ветке = своя concurrency-группа).

Фикс G3.0-fixture (478-A2) INJECT-QUIESCE: пока BenchPopulation готовит
фикстуру, RegionTickOps.forEach уходит в vanilla-serial list.forEach (воркеры
паркованы); на INJECT DONE/ABORT/disable рычаг ре-армится — измеряемое окно
не меняется (fix=0 на окно). Диагноз по артефактам фейлов: dp2 36357022841 /
trek 36356982268 / totem 36357157202 — main-thread addEntity × worker-thread
entity-callbacks = fastutil entityMap AIOOBE Index -1 (len 65537|131073,
n=65536|131072 per-world tracker) + CollectingNeighborUpdater Index 6/6
(FallingBlockEntity на воркере). C54 PER-WORLD REFUTED: мир dp2 (sha
a13b353a...) SUCCEEDED на c30febfb (c98a-dpfull 36356927274) и FAILED на
640926e9 — дискриминатор = carrier, не мир. Старт-гейт НЕ дефект: forceload
36/9216 + boot Done во всех 4 рана.

Канон-вектор C75 (как dispatch_477_c61_reworld.py): gc3, rt4/bc1, pop150k
seed42, r640/300s/fp4, ic1/fd1, 10G/xms4G, band [6.0,9.5]M, lever_flag=cmp475_c30conf.
"""
import json, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = {
    "dp2":   ("round-478-a2-dp2",
              "https://github.com/PLANETA9091/c-crussty/releases/download/v471-s74-dp/world471-s74-dp.zip"),
    "totem": ("round-478-a2-totem",
              "https://github.com/PLANETA9091/c-crussty/releases/download/v470-s74-totem/world470-s74-totem-v1.zip"),
    "trek":  ("round-478-a2-trek",
              "https://github.com/PLANETA9091/c-crussty/releases/download/v472-s35-dp1/world472-s35-dp1.zip"),
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "478-a2-fixture-dispatcher"})
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


def main():
    tok = token()
    fix_sha = api(tok, "/repos/{}/commits/{}".format(REPO, LEGS["dp2"][0]))["sha"]
    print(f"478-A2 fix HEAD {fix_sha[:8]} (640926e9 carrier + G3.0-fixture INJECT-QUIESCE)")

    run_ids = {}
    for key, (branch, world) in LEGS.items():
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs_for(world)})
        print(f"DISPATCHED leg={key} branch={branch} world={world.rsplit('/', 1)[-1]} lever=cmp475_c30conf gc3 rt4/bc1 pop150k")

    # run-id discovery: legs are on 3 DIFFERENT alias branches — match by head_branch
    seen = {}
    deadline = time.time() + 300
    while len(seen) < 3 and time.time() < deadline:
        time.sleep(15)
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=20")["workflow_runs"]
        for r in runs:
            hb = r["head_branch"]
            if hb in {b for b, _ in LEGS.values()} and hb not in seen and r["head_sha"] == fix_sha:
                seen[hb] = {"id": r["id"], "status": r["status"], "created_at": r["created_at"]}
                print(f"RUN leg={[k for k, v in LEGS.items() if v[0] == hb][0]} id={r['id']} branch={hb} status={r['status']}")
    for k, (b, _) in LEGS.items():
        if b in seen:
            run_ids[k] = seen[b]["id"]
    print("RUN_IDS_JSON=" + json.dumps(run_ids, sort_keys=True))


if __name__ == "__main__":
    main()
