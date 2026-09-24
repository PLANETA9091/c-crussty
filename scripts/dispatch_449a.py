#!/usr/bin/env python3
"""dispatch_449a.py — TASK-449-A collide re-leg dispatch (single leg, argv-guarded).

×447 lesson: argv-guard BEFORE any action; --dry-run = preflight only.
Strict SHA pin on the vector branch (round-449a-collide-1 @7edda662 — the
×449 ConstSlot delivery fix). Inputs РОВНО world-bench-parallel canon;
NO travel_diet / fluid_dirty_ledger / concurrency guards.
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-449a-collide-1"
EXPECTED_SHA = "7edda662b16474e9383140530228f1c81582f39f"  # NCDFE-fix commit
LEVER = "cmp445_collide"

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": LEVER, "lever_arg": "1",
}


def token():
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
            print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def main():
    if "--help" in sys.argv or "-h" in sys.argv:
        print("usage: dispatch_449a.py [--dry-run]")
        raise SystemExit(0)
    dry = "--dry-run" in sys.argv
    tok = token()
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{BRANCH}")["object"]["sha"]
    if not live.startswith(EXPECTED_SHA[:12]):
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live[:12]} expected={EXPECTED_SHA[:12]}")
    print(f"preflight OK: {BRANCH} @ {live[:8]} (strict vector pin)", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatch", flush=True)
        return
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": BRANCH, "inputs": INPUTS})
    print(f"dispatched round-449a-collide-1 lever={LEVER} arg=1 sha={live[:8]}", flush=True)
    time.sleep(2)
    # echo the just-created run id (head of workflow runs for the branch)
    runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={BRANCH}&per_page=3")
    for r in runs.get("workflow_runs", [])[:3]:
        print(f"run {r['id']} {r['status']} created={r['created_at']} head={r['head_sha'][:8]}", flush=True)


if __name__ == "__main__":
    main()
