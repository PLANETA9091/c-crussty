#!/usr/bin/env python3
"""dispatch_480_c12_dp1.py — ROUND-480 C12 dp-контроль (ваниль, post-№17 confinement; v19.0).

CLAIM (пегист закон 14a/16, ДО диспатча):
  dp-ось 19c post-МЕРЖ №17 (RegionTickOps-confinement f66feb1b) — контроль ×1 fresh на
  master 686f2258: dp-lane должен остаться stratum-only (0.0000% ×348,656 exec канон B6
  ×3 fresh; C34: Commands-union 0.045% без dp / 0.037% с dp). Это ВАНИЛЬ-КОНТРОЛЬ
  (не dp-нога): canon x466-C98 явным JSON (все дефолты, lever ∅), мир-дефолт
  (world_sha256 afb3a0b3), band GLOB [6.0,9.5]M fast-fail.
  ОСНОВНАЯ работа командира C12 — СТЗ-абсорб: СТЗ-2 C2ME #457 run 36379253638
  (CORRIDOR-BREACH norm +3.56 @7,066,602, M1 CLEAN — datum плато-класса 19c) +
  гипотезы-дельты СТЗ-3 Lithium #37 (см. CLM-C12.md).

Гейты чтения (пост-фактум): ваниль-VALID (lever ∅ armed=0, NCDFE=0, AIOOBE=0,
FIXTURE VALIDITY, world afb3a0b3, canon-env ×12), M1 normtool_478 m1_clean, norm_v5
∈ ваниль-коридор [−8,+1.5] → банк-фид §3; STRICT-hit post-hoc [6.9,7.2]M. Профиль:
dp-lane (CommandFunction/CommandDispatcher/ServerFunctionManager) — ожидание 0.0000%
(0.05пп stratum-коридор C34). Runs >15 мин → DISPATCHED run-id (закон 18-iii).
Закон-5: пороги/окна v5-FROZEN не тронуты, ваниль-семантика датапаков не ломаем.

Usage: dispatch_480_c12_dp1.py [--dry-run] [--pin <sha>]
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-480-c12-dp1"
EXPECTED_PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72: yml-дефолты = merge-поверхность); lever ∅;
# world_url НЕ задаём → дефолт-мир банка (afb3a0b3) — ваниль-контроль dp-оси.
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


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "480-c12-dp1-dispatch"})
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
    pin = None
    if "--pin" in sys.argv:
        pin = sys.argv[sys.argv.index("--pin") + 1]
    tok = token()

    # 0) пин = --pin (shared-clone-гонка) или EXPECTED_PIN; origin/master должен совпасть
    master = api(tok, f"/repos/{REPO}/commits/master").get("sha", "")
    if pin is None:
        pin = EXPECTED_PIN
    assert master == pin, f"origin/master {master} != expected pin {pin} — дрейф/пуш"
    # 479-F2-урок: pin из аргумента/константы, не из volatile HEAD shared-клона;
    # локальный HEAD-дрейф (соседние командиры, docs/scripts-only) не блокер —
    # алиас ставится строго на pin. Прегист-скрипт вендоред в CLM-C12.md.
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    if local != pin:
        print(f"NOTE: local HEAD {local[:8]} != pin {pin[:8]} (сосед-коммит docs-only) — пин держим")
    print(f"master pin OK {master[:8]} (ваниль-контроль, 0 код-дельт)")

    if dry:
        print(json.dumps({"alias": ALIAS, "pin": master, "inputs": INPUTS}, indent=1))
        return

    # 1) алиас round-480-c12-dp1 @pin FULL-sha (Л188a)
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

    # 2) диспатч — canon JSON, lever ∅ (1 реф = 1 диспатч Л188b)
    code = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
               method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    ok = "_http_error" not in code
    print(f"DISPATCHED ref={ALIAS} world=default(afb3a0b3) "
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
