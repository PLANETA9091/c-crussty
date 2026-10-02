#!/usr/bin/env python3
"""dispatch_479_b4_dpfull.py — 479-B4 dpfull-стенд перепроверка (v19.0 tick ×479).

CLAIM: 19c dpfull-стенд перепроверка на №17-мастере (quiesce resident). ×477 dpfull
  36356927274 SUCCESS @c30febfb; ×479 A9 v4-фикстура BN in-flight 36374776545.
  Задача: dpfull-повтор на новом master ed705c5df431c54a84e9899baa3ef8f06fe88cb4
  0 код-дельт (алиас round-479-b4-dpfull), canon x466-C98 pop150k ЯВНЫМ JSON
  (урок C66-C72: yml-дефолты = merge-поверхность):
  640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G/band[6000000,9500000],
  lever ∅ (0 код-дельт → нечего ARM-пруфить), world_url = workflow-default
  MineShield-3 Min (dpfull-стенд).

Гейты прегист (закон 14a/16, до диспатча; чтение пост-фактум из артефакта):
  boot-Done (server boot complete), AIOOBE-счёт (cmp420-biomes-selftest не-гейт
  Л-474-C88.2), POP-гейт (POPULATION INJECT DONE + FIXTURE-VALIDITY: VALID),
  band 6.0-9.5M, NCDFE=0, закон-5 запреты (пороги/окна v5-FROZEN не тронуты,
  ваниль-паритет датапаков).

Census (галочка 3): fn-пайп из cpu-collapsed.txt артефакта —
  ServerFunctionManager ∪ CommandFunction ∪ CallFunction ∪ fn-executor ∪
  brigadier-union ∪ jigsaw/template-dp-путь; база B6 0/348,656 (0.0000% ALL-CPU,
  ×3 ваниль-ноги Л-478-B6) → dp-ось перепроверка на №17-мастере.

Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_479_b4_dpfull.py
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-479-b4-dpfull"
EXPECTED_SHA = "ed705c5df431c54a84e9899baa3ef8f06fe88cb4"

# canon-вектор x466-C98 ЯВНЫМ JSON (world_url = workflow-default MineShield-3 Min)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-b4-dpfull-verifier"})
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


def main():
    tok = token()

    # 0) EXPECTED_SHA-гард против HEAD-гонок (метод 478-A10)
    master = api(tok, f"/repos/{REPO}/commits/master")["sha"]
    assert master == EXPECTED_SHA, f"master drift: {master} != {EXPECTED_SHA}"
    print(f"master pin OK {master[:8]} (№17-master, 0 code-delta vs dispatch)")

    # 1) alias round-479-b4-dpfull @EXPECTED_SHA (FULL-sha push, Л188a)
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")
    cur = r.get("object", {}).get("sha")
    if cur == EXPECTED_SHA:
        print(f"alias {ALIAS}: exists @ {cur[:8]}")
    elif cur:
        sys.exit(f"FATAL: alias {ALIAS} exists @ {cur[:8]} — drift, not touching")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST", data={
            "ref": f"refs/heads/{ALIAS}", "sha": EXPECTED_SHA})
        print(f"alias {ALIAS}: CREATED @ {EXPECTED_SHA[:8]}")
    got = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")["object"]["sha"]
    assert got == EXPECTED_SHA, f"GET-verify failed: {got}"
    print(f"GET-verify OK: {ALIAS} @ {got[:8]}")

    # 2) диспатч — явный JSON-канон, lever ∅ (1 реф = 1 диспатч Л188b)
    code = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
               method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    print(f"DISPATCHED ref={ALIAS} lever=∅ "
          f"canon=640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G "
          f"band=[6.0,9.5]M http={'204' if not code or '_http_error' not in code else code}")

    # 3) run-id discovery строго по head_branch (S31) + head_sha
    for _ in range(20):
        time.sleep(15)
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=15")["workflow_runs"]
        for run in runs:
            if run["head_branch"] == ALIAS and run["head_sha"] == EXPECTED_SHA:
                print(f"RUN_ID={run['id']} status={run['status']} created={run['created_at']}")
                return
        print("waiting run discovery ...")
    print("RUN_ID=NOT_FOUND (check head_branch attribution)")


if __name__ == "__main__":
    main()
