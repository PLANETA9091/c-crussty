#!/usr/bin/env python3
"""dispatch_479_a2_r0.py — 479-A2 R0 пост-мерж верификация (v19.0 tick ×479).

CLAIM: R0-фикс в мастере (70d64190): TICK2_FLAGS-mirror java-7==rust-7 STRICT
(cmp452_mega в rust-гейт) + GB4 include_str!-страж + PIN 13→28. In-vivo пруф
был на ветке (run 36362943927 SUCCESS @21f67725). Пост-мерж верификация =
canon-вектор x466-C98 ЯВНЫМ JSON (урок C66-C72: yml-дефолты неканон):
640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G/band[6000000,9500000]
lever cmp452_mega на алиасе round-479-a2-r0 @70d64190 (0 код-дельт от master).

Гейты прегист (закон 14a/16, до диспатча): GB4-зеркало 7==7 STRICT обе стороны
(python-реплика PASS) + blobgate ALL IN SYNC EXIT=0 (PIN-28 сайтов).

Гейты рана: TICK2_FLAGS-mirror ARM-пруф 4/4 in server-stdout (selfTestTickEach
→ tick2 patched → cmp452_mega brain-tick2 ARMED → EFFECT sense tick2),
PIN-28 маркеры (gate_load nested IdKey/Snapshot ×2, CACHE-пул, flat==nested
trio), NCDFE=0, AIOOBE=0 (cmp420-biomes-selftest не-гейт Л-474-C88.2),
M1 STW≤23s/young_avg≤200ms, band 6.0–9.5M, norm [−8,+1.5] vanilla-коридор
(tick2-квота 0.48%CPU ≤+0.5pp), ваниль-паритет.
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-479-a2-r0"
# master на тик; дрейф 70d64190→19b45ac5 docs/scripts-only (0 Java/Rust, проверено
# diff --name-only) — пин переехал на HEAD, R0 70d64190 = предок (A18-урок).
EXPECTED_SHA = "19b45ac5a49330a87f62a76df841be7fca896184"

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
    "lever_flag": "cmp452_mega", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-a2-r0-verifier"})
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
    print(f"master pin OK {master[:8]} (R0-fix base, 0 code-delta)")

    # 1) alias round-479-a2-r0 @EXPECTED_SHA (FULL-sha push, Л188a)
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

    # 2) диспатч — явный JSON-канон + lever cmp452_mega (1 реф = 1 диспатч Л188b)
    code = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
               method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    print(f"DISPATCHED ref={ALIAS} lever=cmp452_mega "
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
