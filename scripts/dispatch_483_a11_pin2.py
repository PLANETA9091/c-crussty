#!/usr/bin/env python3
"""dispatch_483_a11_pin2.py — [483-A11] Commander A11: C84 pin-codecache реплика-2 ×1 (тик ×483).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча):
  C84 pin-codecache реплика-2. Канон x466-C98 с дельтой CodeCache-reserve:
  lever_flag="pin_codecache", lever_arg="512m".

УТОЧНЕНИЕ lever (честный ценз, выполнен до диспатча):
  - c84-скрипта в /home/z/c-crussty/scripts/ НЕТ (grep: 0 dispatch_482_c84*).
  - /home/z/rounds/ROUND-482/board/CLM-C84.md НАЙДЕН, но ×482-C84 = burst73-ваниль ×8,
    lever_flag="" / lever_arg="" (c84_dispatch.json 1:1) — pin-codecache там НЕ задан.
  - Репо: CRUSSTY_LEVER_FLAG-консьюмеры = items_sweep/items_index/items_subsys2/items_oss/
    inside_* — id "pin_codecache" в мастере НЕ зарегистрирован → риск silent-vanilla
    (C80 FINAL footgun: цепь if/elif без else). RCC-ось (CodeCache-reserve) C80: REFUTED ×2.
  => lever_flag/lever_arg взяты ИЗ CLAIM-канона, помечено «по спеке C06-гипотезы».
  Диспатч честный: если lever незнаком мастеру — bench уйдёт ванилью, данные это покажут.

Вектор: канон CLAIM 1:1 (640/300s/fp4/gc3/ic1/fd1/fl0/fb0/rt4/bc1/ib0/sbb0/rs0/bd0/
  pop150k/seed42/10G/xms4G/band [6.0,9.5]M). Код-дельт НЕТ (ветка = bare pin acffa383)
  -> push-гейты cargo/javap не применимы. Запреты закона 5 не тронуты.

Алиас: round-483-a11-pin2. PIN = master acffa383 (×482-учёт, BOTTLENECK).
BAND: fast-fail = band-miss -> ре-ролл <=2.

Usage: dispatch_483_a11_pin2.py [--dry-run]
"""
import json, os, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = os.environ.get("A11_BRANCH", "round-483-a11-pin2")
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master ×482-учёт (BOTTLENECK-483)

INPUTS = {
    "radius": "640",
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3",
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0",
    "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    # по спеке C06-гипотезы (CLAIM-канон A11; в борде/репо id не подтверждён — см. шапку)
    "lever_flag": "pin_codecache", "lever_arg": "512m",
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
    print(f"origin/master live = {live_master} -> pin {PIN[:8]} (A11: C84 pin-codecache реплика-2)", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        if cur is None:
            r = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                    data={"ref": f"refs/heads/{BRANCH}", "sha": PIN})  # FULL-sha (урок S20: 422 на коротком)
            print(f"ref POST {BRANCH} @ {PIN[:8]} -> {r}", flush=True)
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
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-483/a11_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-483/a11_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
