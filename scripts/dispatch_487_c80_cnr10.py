#!/usr/bin/env python3
"""dispatch_487_c80_cnr10.py — [487-C80] Commander C80 canary cnr10 rep 300s (тик ×487, Job 415026).

CLAIM (prereg, закон 14a/16 — CLM-C80.md зафиксирован ДО диспатча):
  canary cnr10 rep — 2-я точка canary-коридора ×487. Ванильный MineShield-стенд,
  банк-канон x466-C98 бит-в-бит, lever_flag=∅ (ваниль). НЕ дублирует C79 cnr9 —
  своя ветка round-487-c80-cnr10, тот же канон-вектор ваниль.
ГИПОТЕЗА-ДЕЛЬТА (prereg): 0-delta canary @ pin 85a06f2f (код f0051e70, вердикт №20
  +41.44) → norm_v5 ∈ [−6,+6] коридор-OK (cnr4-класс −1.97, cnr5 SUCCESS); пара
  cnr9+cnr10 = min-of-2 коридор-гейт ×487. Breach → merge-gate сигнал.
ВЕКТОР = банк-канон x466-C98 (комментарий workflow: input defaults = canon vector):
  radius 640 / seconds 300 / fp4 / fluid_guard 1 / gc3 (ParallelGC) / ic1 / fd1 /
  rt4 / bc1 / pop150k / seed42 / xmx10G / xms4G / band [6000000,9500000] /
  world_url = MineShield-3 Min--Normal (workflow default) / lever_flag ∅.
ПОВЕРХНОСТЬ: world-bench-parallel.yml (инфра-канон ×486 parallel per-ref; слот-война
  ЗАПРЕТ). Ветка round-487-c80-cnr10 создаётся git branch+push @ pin 85a06f2f
  (Л188b: 1 ветка = 1 ран).
ГЕЙТЫ рана (prereg): (1) band PASS fast-fail; (2) fixture = default Min; (3)
  AIOOBE=0 / INJECT DONE=150000; (4) norm_v5 ∈ [−6,+6]; (5) M1 STW ≤23.0; (6)
  >15 мин → DISPATCHED (12e/18-iii).
LEDGER: «## ТИК-487 ЛАБ-C80».

Usage: dispatch_487_c80_cnr10.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c80-cnr10"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)

# банк-канон x466-C98 ваниль бит-в-бит (world_url = workflow default Min--Normal)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "seconds": "300",        # canon 300s
    "radius": "640",         # canon ~9.2k chunks
    "region_threads": "4",   # rt4
    "fake_players": "4",     # fp4
    "fluid_guard": "1",
    "gc_tune": "3",          # gc3 = ParallelGC swap
    "inside_cache": "1",     # ic1
    "flush_diet": "1",       # fd1
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "batch_collector": "1",  # bc1
    "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "",        # ВАНИЛЬ — canary 0-delta
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
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&event=workflow_dispatch&per_page=30")
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
        raise SystemExit(f"BRANCH {BRANCH} on remote != pin ({cur}) — git branch+push выполни вне скрипта; re-run")
    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit("GET-verify FAIL (Л188a)")
    print(f"GET-verify OK {BRANCH} object.sha == {got}", flush=True)

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
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c80_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c80_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
