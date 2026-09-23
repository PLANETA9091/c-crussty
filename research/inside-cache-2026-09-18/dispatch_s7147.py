#!/usr/bin/env python3
"""dispatch_s7147.py — dispatch base-b: the census base re-run on the FIXED
fixture (S7-147 real-count topup), head 4c8f029.

Config = the preregistered census config (run 35275967738: 150000 live,
seed 42, xmx 10G, fp 4, guard 1, 300s) with ALL lever inputs explicitly 0.
The S7-147 fixture fix restores wall-time population parity (leg2/leg2'
evidence: 148k -> ~71k drain while the model-based topup reported deficit=0).
A/B pairs (leg #2'' INSIDE-CACHE, FLUSH-DIET, FLUID-FREE, ALLOC-DIET) must
compare against THIS base, not the pre-fix census.

Preregistered leg #2'' absorb (next): §156 gates vs base-b equal windows.
"""
import json
import re
import sys
import time
import urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
TOKEN = __import__("os").environ.get("CRUSSTY_GH_TOKEN", "")  # set per owner rule 1b (cron directive)


def api(url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {TOKEN}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def main():
    # S7-108 concurrency law: no in-flight world-bench run
    d = api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "summon_sweeps": "0",
        "fake_players": "4",
        "fluid_guard": "1",
        "paletted_demux": "0",
        "alloc_diet": "0",
        "inside_cache": "0",
        "flush_diet": "0",
        "fluid_free": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
    }
    api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent (base-b: all-zero levers on fixed fixture); waiting...")
    for _ in range(12):
        time.sleep(5)
        d = api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
        runs = d.get("workflow_runs", [])
        if runs:
            r = runs[0]
            print(json.dumps({"run_id": r["id"], "status": r["status"],
                              "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
            return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
