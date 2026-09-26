#!/usr/bin/env python3
"""dispatch_470_s51_r480.py — S51 / КЛИМБ-chunk (round-470): чистый r480-повтор
Решает S51-клейм: r480b −38.95@7083980 (BOTTLENECK §5) — артефакт или стратум.

Контекст (Л-470-S06.1-.5): r480b шёл на G1 через gc6-дыру базы 4c1c05a9
(фикс ac0dbad9 упал в master ПОЗЖЕ рана) → молчаливый фолбэк в G1;
r480a −3.00 = тот же вектор на gc3 (банк-канон ParallelGC). S06 уже
задиспатчил r480c (gc6-живой, run 36270314846) + r480d (gc3, run 36269778143).
S51 = независимый 3-й ролл чистого r480-вектора на ветке round-470-s51-*:
если повторы ±2пп → стратум-число r480 в банк (bank-feed gc3-канон);
если рядом с −38.95 → стратум-эффект реален (opens W8-квант).

Вектор = БАНК-V5 канон (workflow defaults, x466-C98): r480/300s/fp4/gc3/
ic1/fd1/rt4/bc1/pop150k/seed42/10G/4G, band 6.0-9.5M, lever ПУСТО
(чистая нога, armed=null — как r480a/b). База ветки = 40068dbe
(b3853246 МЕРЖ №11 + board-док, дерево бит-идентично ногам r480c/d).

Usage: dispatch_470_s51_r480.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-470-s51-r480e"
PIN_SHA = "40068dbe"  # b3853246 (МЕРЖ №11) + board-док; case-6 жив, дерево == r480c/d

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


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def find_run_id(tok, full_sha):
    """Discover the workflow_dispatch run id for head_sha created after dispatch."""
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
    live = sha_of(tok, BRANCH)
    if not live.startswith(PIN_SHA[:7]):
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live[:8]} pin={PIN_SHA}")
    print(f"preflight OK: {BRANCH} @ {live[:8]} (чистый r480-повтор, банк-канон gc3)", flush=True)
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
    print(f"=== S51 чистый r480-повтор DISPATCHED: branch={BRANCH} sha={live[:8]} run={run_id} ===", flush=True)


if __name__ == "__main__":
    main()
