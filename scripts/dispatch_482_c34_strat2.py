#!/usr/bin/env python3
"""dispatch_482_c34_strat2.py — C34 / ROUND-482: r480-стратум повтор-2 (292.6-класс фиксация).

Миссия (CLM-C34 ×482): повтор ступени 19a-лестницы «r480-стратум 292.6 чанк/s»
(оригинал run 36389080708, round-480-c34-terr1 @686f2258, canon-vanilla gc3,
4096/14s, cadence 0.875s/cmd, parse-disk класс RC1). ×481: стратум-эффект
REFUTED (r480b = G1-артефакт), но 292.6 стоит как ступень — фиксация n=2.

Вектор = БАНК-V5 канон (workflow defaults x466-C98): r480/300s/fp4/gc3/
ic1/fd1/rt4/bc1/pop150k/seed42/10G/4G, band 6.0-9.5M, lever ПУСТО
(armed=null, точная реплика оригинала; gc3 НЕ gc6 — конфиг ступени 292.6
проверен по LEDGER Л-481-C13/C34: canon, ParallelGC). База ветки =
master full 3666a7931703e24a36af4887c41a585918667b7a (МЕРЖ №19).

Usage: dispatch_482_c34_strat2.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-482-c34-strat2"
PIN_SHA = "3666a7931703e24a36af4887c41a585918667b7a"

INPUTS = {
    "radius": "480", "seconds": "300", "fake_players": "4",
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
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def find_run_id(tok, full_sha):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=10")
    for r in runs.get("workflow_runs", []):
        if r.get("created_at") and r.get("head_sha") == full_sha:
            return r["id"], r["html_url"], r.get("status")
    return None


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{BRANCH}")["object"]["sha"]
    if live != PIN_SHA:
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live} pin={PIN_SHA}")
    print(f"preflight OK: {BRANCH} @ {live[:8]} (r480-стратум повтор-2, canon gc3)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": BRANCH, "inputs": INPUTS})
    print(f"dispatched: {BRANCH} radius=480 gc_tune=3 pop150k seed42 vanilla (HTTP 204)", flush=True)
    run_id = None
    for _ in range(30):
        time.sleep(5)
        hit = find_run_id(tok, live)
        if hit:
            run_id = hit[0]
            print(f"run-id discovered: {hit[0]} status={hit[2]} {hit[1]}", flush=True)
            break
    if run_id is None:
        print("run-id NOT discovered within 150s — poll actions/runs by head_sha", flush=True)
    print(f"=== C34 strat2 DISPATCHED: branch={BRANCH} sha={live[:8]} run={run_id} ===", flush=True)


if __name__ == "__main__":
    main()
