#!/usr/bin/env python3
"""dispatch_487_c60_tectr.py — [487-C60] Commander C60 tect-якорь 5.1-класс rep 300s (тик ×487, Job 415026).

CLAIM (prereg, закон 14a/16 — CLM-C60.md зафиксирован ДО диспатча):
  300s-якорь-повтор tect-класса 5.1. Факт: tect1-якорь 36452553684 (tect@r480-gc6-rt4
  5.1 чанк/s, forceload 4096/804s); лестница 19a: tect 12.0 ≫ tect@r480-gc6-rt4 5.1.
  НЕ дублирует C30 LIMBO-900s (run 36507529511) — я канон-якорь @300s.
ГИПОТЕЗА-ДЕЛЬТА (prereg): бит-в-бит повтор вектора tect1-якоря при seconds=300 →
  forceload 4096 чанков НЕ доезжает за 300s (~804s истор.) → честный 300s-срез
  прогресса / LIMBO-класс; подтверждение стабильности штумма 5.1.
ВЕКТОР = tect1-якорь-класс 5.1 бит-в-бит C30-канон:
  world_url=v482-tect-v1/world482-tect-v1.zip (sha256 ced8f79d…), r480 (4096 чанков),
  seconds=300, gc6, xmx12G, xms4G (default), fp4, ic1, fd1, rt4, bc1, pop150k, seed42,
  band [6.0,9.5]M, lever_flag=cmp466_c98ai (emap-ARM), lever_arg ПУСТОЙ (stagger 16).
ПОВЕРХНОСТЬ: world-bench-parallel.yml (инфра-канон ×486 parallel per-ref; слот-война
  ЗАПРЕТ). Ветка round-487-c60-tectr создаётся git branch+push @ pin 85a06f2f (Л188b:
  1 ветка = 1 ран).
ГЕЙТЫ рана (prereg): (1) band PASS fast-fail; (2) forceload 4096 DONE или 300s-срез;
  (3) fixture sha ced8f79d; (4) ARM-эхо cmp466_c98ai + emap composed; (5) AIOOBE=0 /
  INJECT DONE=150000; (6) M1 STW ≤23.0; (7) >15 мин → DISPATCHED (12e/18-iii).
LEDGER: «## ТИК-487 ЛАБ-C60».

Usage: dispatch_487_c60_tectr.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c60-tectr"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70, LIMBO-GATE ee168ecb in)

# tect1-якорь-класс 5.1 (36452553684) бит-в-бит C30-канон; ЕДИНСТВЕННАЯ дельта vs C30: seconds=300
INPUTS = {
    "world_url": "https://github.com/PLANETA9091/c-crussty/releases/download/v482-tect-v1/world482-tect-v1.zip",
    "seconds": "300",       # ← ДЕЛЬТА ноги: 300s-якорь rep (C30 = 900s LIMBO, не дублируем)
    "radius": "480",        # r480-стратум, 4096 чанков
    "region_threads": "4",  # rt4
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "6",         # tect-класс (gc6 = ParallelGC + Metaspace 256M + RCC 512M)
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "batch_collector": "1", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp466_c98ai",  # emap-ARM (не-default мир + pop>0, Л-474-C82.2/C63)
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
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c60_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c60_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
