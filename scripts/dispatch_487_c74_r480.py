#!/usr/bin/env python3
"""dispatch_487_c74_r480.py — [487-C74] Commander C74 r480-стратум vanilla rep leg (тик ×487, Job 415026).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча, /home/z/rounds/ROUND-487/board/CLM-C74.md):
  r480-стратум vanilla rep на БАЗОВОМ MineShield-3 Min мире (radius=480 = 4096 чанков).
  C18 ×487 CONFIRMED: пол 273.1 структурен (sync-stall ChunkLoadTask/ChunkFullTask бит-в-бит);
  DISPATCHED-канал стратума = C74 r480-rep leg (реп-функция брикета, НЕ ≥+20-претендент).
  БРИКЕТ ×486 C75/C80: 273.1 (floor, sync-stall, slow-host 6.25M) … 292.6 (fast-host) …
  307.2 (disk-load потолок, канон pregen). ГИПОТЕЗА: r480-rep воспроизводит 273.1-292.6-брикет
  (2-я точка брикета) — ч/с = 4096/wall_forceload ∈ [273.1, 292.6].
ВЕКТОР = канон x466-C98 явным JSON (640→ЕДИНСТВЕННАЯ дельта radius=480; 300s/fp4/gc3/ic1/fd1/
  rt4/bc1/pop150k/seed42/10G/xms4G, band [6.0,9.5]M) — бит-в-бит Л-470-S51.1 чистый r480-повтор
  (r480a −3.00@6749854 класс; gc_tune=3 обязателен — банк-feed пресет, G1-дыра запрещена).
  world_url = дефолтный MineShield-3 Min (yml-канон, базовый мир — НЕ towers/terr/tect).
  lever ∅ (flag=''/arg='' — чистая ваниль, armed=null).
ПОВЕРХНОСТЬ: world-bench-parallel.yml (инфра-канон ×486: parallel per-ref — единственная
  поверхность; world-bench.yml слот-война ЗАПРЕТ). 1 ветка = 1 ран (Л188b).
ВЕТКА: round-487-c74-r480 @ master PIN 85a06f2f — создана ЛОКАЛЬНО git branch+push
  (миссия-канон), GET-верификация ls-remote == PIN. master НЕ трогается.
ГЕЙТЫ рана (prereg): (1) band PASS fast-fail → бесплатный дискард; (2) forceload 4096 DONE;
  (3) POPULATION VALID 150k/seed42 fp4; (4) ARM-эхо ∅ / NCDFE=0 / AIOOBE=0; (5) M1 STW ≤23.0
  (gc3); (6) ч/с-вердикт: ∈[273.1,292.6] → rep CONFIRMED 2-я точка; <273.1 → floor/slow-host
  повтор; >307.2 → брикет-потолок сломан (подозрительно); (7) ран >15 мин → DISPATCHED
  run-<id> (закон 12e/18-iii). norm_v5 = report-only (r480-стратум не банк-в-точка носитель).
LEDGER: «## ТИК-487 ЛАБ-C74» (append на ветку docs-only, после диспатча).

Usage: dispatch_487_c74_r480.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c74-r480"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)

# канон x466-C98; ЕДИНСТВЕННАЯ дельта = radius 640→480 (r480-стратум, 4096 чанков)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "480",        # ← ПЛЕЧО ноги (канон 640; r480-стратум C18/S51-домен)
    "seconds": "300",       # канон 300s
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3",         # канон ParallelGC swap (bank-feed, G1-дыра запрещена Л-470-S06)
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",   # закон-5 чист
    "batch_collector": "1", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "region_threads": "4",  # rt4 канон (W12 REFUTED rt8-оптимум — не трогаем)
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "",       # чистая ваниль (armed=null, реп-функция брикета)
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
    if live_master != PIN:
        print("WARNING: master moved — PIN-канон держим 85a06f2f (миссия-пин)", flush=True)

    # ветка создана локально git branch+push (миссия-канон) — верифицируем API-бит-точность
    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        raise SystemExit(f"BRANCH VERIFY FAIL: {BRANCH} @ origin = {cur} != PIN {PIN} "
                         "(сначала: git branch + git push)")
    print(f"GET-verify OK {BRANCH}.object.sha == {PIN[:8]}", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")
    print("run-snapshot clean (0 runs on branch)", flush=True)

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
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c74_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c74_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
