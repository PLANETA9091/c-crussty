#!/usr/bin/env python3
"""dispatch_481_c38_stz3.py — COMMANDER C38: СТЗ-3 LITHIUM #37 fixture-validity vanilla (v19.0 ×481).

CLAIM (прегист закон 14a/16, ДО диспатча):
  Миссия СТЗ-3 Lithium #37 redstone-mechanism-mass × chunk-load, fixture 9b3a3f09 pending
  (СТЗ-3 амплитуда ≥10пп @duty 0.2 — канон ×479). C12/C37 ×2: redstone-масса в 9b3a3f09
  = ∅ (86 файлов: level.dat + пустой region + dp-драйвер forceload) → redstone-терм 0пп;
  ЭТОТ диспатч = fixture-validity ваниль-базлайн СТЗ-3-фикстуры на мастере 22919dfc (×481),
  НЕ redstone-измерение. world_url найден прошлого тика (C37-стенд): release-ассет
  v479-f2-stz-world/world479-f2-stz-v1.zip sha256 9b3a3f09… (19861 B) — СТЗ-2-прецедент
  run 36379253638 SUCCESS, C93 fresh-прецедент run-архив ROUND-480.

ФАКТ-ЧЕК (закон-канон, до диспатча): world_url существует (HTTP 206 GET-verified),
  ветка round-481-c38-stz3 @22919dfc FULL-sha GET-verified (Л188a), 0 код-дельт
  (scripts-only локально, ветка = чистый базовый sha), lever ∅ (ваниль-команды only).

ПРЕГИСТ-ГЕЙТ (пороги v5-FROZEN, СТЗ-2/C93-класс):
  G-band: runner_cpu_index ∈ [6.0,9.5]M fast-fail (band-miss → ≤1 ре-ролл same-branch W3).
  G-M1: STW ≤23.0s ∧ young_avg ≤200ms.
  G-FV (fixture-validity, dp-мир ≠ банк-фид): norm ВНЕ ваниль-коридора [−8,+1.5]
    сверху (+2..+6 прогноз; СТЗ-2-прецедент +3.56 breach-top) → vanilla_valid=FALSE,
    НЕ банк-фид — сцена-конфаунд fresh-gen-vs-pregen, fixture VALID.
  P1 young-морфология: паузы 200-280, avg 40-80ms, масса [12.5,16.5]s (константа 13.9s).
  P2 STW ∈ [18,23]s (СТЗ-2 21.16s).
  P4 dp-машина ≤0.2пп (СТЗ-2 0/117,629 сэмплов 0.0000%), worldgen tick 0.0000%.
  Redstone-lane: СТЗ-3-амплитуда ≥10пп @duty 0.2 НЕ измерима на этой фикстуре
    (H-Δ1: redstone ∅) → pending materialize one-scene-per-run (отдельный zip).

ЗАДАЧА: 1 ваниль-диспатч world-bench-parallel @22919dfc, алиас round-481-c38-stz3,
  canon x466-C98 ЯВНЫМ JSON + world_url=9b3a3f09-ассет, lever_flag="", lever_arg="".
  Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_481_c38_stz3.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-481-c38-stz3"
PIN = "22919dfc1ae0d91eb6d0d962bfed960c8d8cb884"

# СТЗ-2/СТЗ-3 fixture 9b3a3f09 (найден прошлого тика: dispatch_479_f2_stz.py / dispatch_480_c93_fresh.py)
WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v479-f2-stz-world/world479-f2-stz-v1.zip")
WORLD_SHA256 = "9b3a3f091941e90dbd22cbf4fc34ee3a2345e2655f108b2c1c14e29fc4c0bb9f"

# canon x466-C98 ЯВНЫМ JSON + world_url, lever ∅ (урок C66-C72: yml-дефолты = merge-поверхность)
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
        "User-Agent": "481-c38-stz3-dispatch"})
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


def verify_fixture(tok):
    """GET-verify релиз-ассета (s51-канон, прецедент F2/C93)."""
    req = urllib.request.Request(WORLD_URL, method="GET", headers={
        "Authorization": f"Bearer {tok}", "User-Agent": "481-c38-stz3-dispatch"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            head = r.read(64)
            ok = r.status == 200
    except urllib.error.HTTPError as e:
        ok, head = False, b""
    print(f"fixture GET-verify: {'OK' if ok else 'FAIL'} (head={head[:16]!r})")
    return ok


def branch_verified(tok):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")
    sha = r.get("object", {}).get("sha")
    ok = sha == PIN
    print(f"branch {ALIAS}: {'GET-verified @ ' + sha[:12] if ok else 'MISMATCH ' + str(sha)[:12]}")
    return ok


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} world=9b3a3f09(stz2/СТЗ-3-fixture) -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"], "url": run["html_url"]}
    return None


def main():
    if "--dry-run" in sys.argv:
        print(json.dumps({"alias": ALIAS, "pin": PIN, "world_sha256": WORLD_SHA256,
                          "inputs": INPUTS}, indent=1))
        return
    tok = token()
    if not verify_fixture(tok):
        sys.exit("FATAL: fixture 9b3a3f09 недоступна — диспатч отменён")
    if not branch_verified(tok):
        sys.exit("FATAL: ветка не GET-verified — диспатч отменён")
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    if not dispatch(tok, ALIAS):
        sys.exit(2)
    run = None
    for _ in range(6):
        time.sleep(15)
        run = latest_run(tok, ALIAS, mc)
        if run:
            break
    print("C38-DISPATCH-JSON " + json.dumps(
        {"alias": ALIAS, "pin": PIN, "world_sha256": WORLD_SHA256,
         "prereg": {"band": "[6.0,9.5]M fast-fail",
                    "M1": "STW<=23.0 ∧ young_avg<=200",
                    "FV": "norm вне [−8,+1.5] сверху (+2..+6) → НЕ банк-фид, fixture VALID",
                    "P1": "young 200-280 @ 40-80ms, масса [12.5,16.5]s",
                    "P2": "STW [18,23]s", "P4": "dp ≤0.2пп, worldgen 0.0000%",
                    "redstone_lane": "СТЗ-3 ≥10пп @duty 0.2 pending (H-Δ1 redstone ∅ в 9b3a3f09)"},
         "run": run}, indent=1))


if __name__ == "__main__":
    main()
