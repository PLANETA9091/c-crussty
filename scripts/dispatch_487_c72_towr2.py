#!/usr/bin/env python3
"""dispatch_487_c72_towr2.py — [487-C72] Commander C72 towers rt2-плечо (тик ×487, Job 415026).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча, /home/z/rounds/ROUND-487/board/CLM-C72.md):
  W-лестница barrier: rt2 0.544 < rt4 1.579 < rt8 4.334 %wall (LEDGER). towers-мир:
  rt4 28.6/29.7 (Л-478-A5 36365068334 / Л-482-C61.3), rt8 26.5 REFUTED (Л-482 ×5-тест),
  rt2 = ∅ (нижняя точка НЕ снята). ПЛЕЧО: towers rt2 — снимаем недостающую точку.
  ГИПОТЕЗА: rt2 на towers — barrier-точка (прогноз <28.6, ladder-монотонность по %wall).
ВЕКТОР = канон x466-C98 явным JSON (640/300s/fp4/gc3/ic1/fd1/rt2/bc1/pop150k/seed42/10G/
  xms4G, band [6.0,9.5]M) с world_url = towers-зип (sha256 331617db1124c89a, канон
  Л-482-C61.1). ЕДИНСТВЕННАЯ дельта от C58-towr4 (бит-в-бит) = region_threads=2.
  lever ∅ (flag=''/arg='' — чистая ваниль, Л-466-C43).
ПОВЕРХНОСТЬ: world-bench-parallel.yml (инфра-канон ×486: parallel per-ref — единственная
  поверхность; world-bench.yml слот-война ЗАПРЕТ). 1 ветка = 1 ран (Л188b).
ВЕТКА: round-487-c72-towr2 @ master PIN 85a06f2f — создана ЛОКАЛЬНО git branch+push
  (миссия-канон), GET-верификация ls-remote == PIN. master НЕ трогается.
ГЕЙТЫ рана (prereg): (1) band PASS fast-fail → бесплатный дискард; (2) FIXTURE VALIDITY
  VALID (pop150k); (3) world sha 331617db; (4) ARM-эхо ∅ / NCDFE=0 / AIOOBE=0;
  (5) M1 STW ≤23.0; (6) runs >15 мин → DISPATCHED run-<id> (закон 12e/18-iii).

Usage: dispatch_487_c72_towr2.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c72-towr2"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)

# канон x466-C98 + towers-сцена; ЕДИНСТВЕННАЯ дельта vs C58 (rt4) = region_threads=2
INPUTS = {
    "world_url": "https://raw.githubusercontent.com/PLANETA9091/c-crussty/round-478-a5-tow-re/stress/world-stress-towers.zip",
    "radius": "640",        # канон 640 (~9.2k чанков)
    "seconds": "300",       # канон 300s
    "region_threads": "2",  # rt2 ← ПЛЕЧО ноги (C58 = rt4, C16 = rt8)
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3",         # канон ParallelGC swap
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",   # закон-5 чист
    "batch_collector": "1", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "",       # чистая ваниль (сцена = нагрузка, бит-в-бит C58)
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
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c72_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c72_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
