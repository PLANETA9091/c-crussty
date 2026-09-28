#!/usr/bin/env python3
"""dispatch_479_a10_stz.py — 479-A10 СТЗ-фикстура №1: item-storm Paper #13783 (v19.0 tick ×479).

CLAIM (прегист закон 14a/16, ДО диспатча; см. stress/MANIFEST.md — СТЗ-1):
  260k item-дропов в spawn-chunk окне → MSPT ↑ vs канон pop150k (Paper #13783 weak-chunks:
  260k drops → 1000+ MSPT). Сцена = датапак stz1 в world479-a10-stz-v1.zip:
  1300 summons/тик × 200 storm-тиков = 260,000 дропов/бурст, период 600 тиков, ваниль-NBT
  (stone Count 1b, PickupDelay 32767s, гравитация, merge vanilla).

Фикстура (галочка 1): релиз v479-a10-stz-world, asset world479-a10-stz-v1.zip, sha256
  d88978f16e0b0de5eae7f3a05e87469c597ff98ebf2496b5143bbb992da76b9c (71552 B),
  download-верифицирован; dirs 0755 (A9-урок), testzip clean, base level.dat v4
  (DataVersion 4556/1.21.10, seed 42, region/ пуст → fresh-gen r640).

Диспатч (галочка 2): world-bench-parallel, алиас round-479-a10-stz @HEAD (дельта
  docs/stress/scripts-only, 0 src/Java/Rust), canon x466-C98 ЯВНЫМ JSON
  (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G), band [6.0,9.5]M
  (pre-download калибровка узла — fast-fail вне band не про сцену), lever ∅, world_url = ассет.

Гейты чтения (галочка 3, пост-фактум из артефакта): FIXTURE-VALID (preflight+boot Done),
NCDFE=0, AIOOBE=0, item-entity counts >0 (шторм жил; 0 → fail-safe canon-ран →
REFUTED_CENS сцены), MSPT-профиль vs канон 392.89/415.84; band-miss workload-cpu ожидаем
(сцена добавляет CPU = claim), не ре-ролл-триггер (Л201). Runs >15 мин → DISPATCHED run-id
(закон 18-iii). Закон-5: пороги/окна v5-FROZEN не тронуты, lever ∅, ваниль-семантика дропов.

Usage: dispatch_479_a10_stz.py [--dry-run]
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-479-a10-stz"

# СТЗ-1 фикс-тура (sha-pinned, download-verified)
WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v479-a10-stz-world/world479-a10-stz-v1.zip")
WORLD_SHA256 = "d88978f16e0b0de5eae7f3a05e87469c597ff98ebf2496b5143bbb992da76b9c"

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72: yml-дефолты = merge-поверхность)
INPUTS = {
    "world_url": WORLD_URL,
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


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-a10-stz-dispatch"})
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
    dry = "--dry-run" in sys.argv
    tok = token()

    # 0) пин = локальный HEAD; гард против HEAD-гонок: origin/master должен совпасть
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    master = api(tok, f"/repos/{REPO}/commits/master")["sha"]
    assert master == local, f"origin/master {master} != local HEAD {local} — push не доехал/дрейф"
    print(f"master pin OK {master[:8]} (docs/stress/scripts-only дельта, 0 код-дельт)")

    if dry:
        print(json.dumps({"alias": ALIAS, "pin": master, "world_sha256": WORLD_SHA256,
                          "inputs": INPUTS}, indent=1))
        return

    # 1) alias round-479-a10-stz @pin FULL-sha (Л188a)
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")
    cur = r.get("object", {}).get("sha")
    if cur == master:
        print(f"alias {ALIAS}: exists @ {cur[:8]}")
    elif cur:
        sys.exit(f"FATAL: alias {ALIAS} exists @ {cur[:8]} — drift, not touching")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST", data={
            "ref": f"refs/heads/{ALIAS}", "sha": master})
        print(f"alias {ALIAS}: CREATED @ {master[:8]}")
    got = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")["object"]["sha"]
    assert got == master, f"GET-verify failed: {got}"
    print(f"GET-verify OK: {ALIAS} @ {got[:8]}")

    # 2) диспатч — world_url + canon JSON (1 реф = 1 диспатч Л188b)
    code = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
               method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    ok = "_http_error" not in code
    print(f"DISPATCHED ref={ALIAS} world=stz1@{WORLD_SHA256[:8]} "
          f"canon=640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G "
          f"band=[6.0,9.5]M lever=none http={'204' if ok else code}")

    # 3) run-id discovery строго по head_branch + head_sha (S31)
    for _ in range(20):
        time.sleep(15)
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=15")["workflow_runs"]
        for run in runs:
            if run["head_branch"] == ALIAS and run["head_sha"] == master:
                print(f"RUN_ID={run['id']} status={run['status']} created={run['created_at']}")
                return
        print("waiting run discovery ...")
    print("RUN_ID=NOT_FOUND (check head_branch attribution)")


if __name__ == "__main__":
    main()
