#!/usr/bin/env python3
"""dispatch_487_c78_dps43.py — [487-C78] Commander C78 КЛИМБ ×487: population_seed=43 dp leg (Job 415026).

CLAIM (prereg, закон 14a/16 — board/CLM-C78.md, зафиксирован ДО диспатча):
  seed-робастность dp-стенда — seed43 vs seed42-канон (инжект-детерминизм гейт).
  Пул n=7 DP-VALID: якорь 0.3@150k (Л-487-C24: КВАНТ-ПЛАТО tps_exp_dp=0.30±0.1
  [6.67,8.84]M, slope≈0) — все ноги на seed42. Вопрос: 0.3-класс seed-инвариантен?
ГИПОТЕЗА: TPS_med(seed43) ∈ 0.3±0.1 → seed НЕ фактор, инжект-детерминизм гейт PASS
  (плато подтверждено кросс-seed); иначе seed = скрытая страта → инжект-шум учёт,
  pool пере-гранулировать по seed.
ВЕКТОР = x466-C98 канон ЯВНЫЙ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/
  10G/xms4G, band GLOB [6.0,9.5]M, lever ∅) + datapack_url=v484-dp3v2 (sha 16fa1a32)
  + population_target=150000 + population_seed=43. ОТ seed42-канона отличается ТОЛЬКО seed.
ПОВЕРХНОСТЬ: world-bench-parallel.yml (parallel per-ref, слот-война ЗАПРЕТ).
  1 ветка = 1 ран (Л188b).
ГЕЙТЫ рана (prereg): band PASS; DP-INSTALLED 16fa1a32; POPULATION INJECT DONE
  150000 + FIXTURE VALID; NCDFE=0; M1 CLEAN; TPS ∈ [0.2,0.4] → seed-инвариант
  CONFIRMED; runs >15 мин → DISPATCHED (закон 12e/18-iii).

Usage: dispatch_487_c78_dps43.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c78-dps43"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)

# x466-C98 канон ЯВНЫЙ JSON + datapack_url=v484-dp3v2 fixture (sha 16fa1a32);
# population_target=150000, population_seed=43 (ЕДИНСТВЕННАЯ дельта vs seed42-канон);
# lever ∅ (seed-робастность, без lever)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640",        # canon r640
    "seconds": "300",       # canon 300s
    "fake_players": "4",    # fp4
    "fluid_guard": "1",
    "gc_tune": "3",         # gc3 = ParallelGC swap
    "inside_cache": "1",    # ic1
    "flush_diet": "1",      # fd1
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4",  # rt4
    "batch_collector": "1", # bc1
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
    "datapack_url": "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip",  # sha256 16fa1a32
    "population_target": "150000",  # pop150k (миссия-явный)
    "population_seed": "43",        # МИССИЯ: seed43 leg (vs seed42-канон)
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",  # GLOB band [6.0,9.5]M
    "lever_flag": "",       # lever ∅
    "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN[:8]} (×486 учёт, миссия)", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": PIN})  # FULL-sha (урок S20)
            print(f"ref CREATED {BRANCH} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {BRANCH} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r == {}:
        print("dispatch 204-OK", flush=True)
    else:
        raise SystemExit(f"dispatch failed: {r}")

    rid = None
    deadline = time.time() + 240
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже)", flush=True)
        json.dump({"branch": BRANCH, "pin": PIN, "run_id": None,
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c78_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID: {rid} (branch {BRANCH} @ {PIN[:8]})", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c78_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
