#!/usr/bin/env python3
"""dispatch_s7147b.py — dispatch leg #2'' INSIDE-CACHE (S7-147b) on the FIXED
fixture, head >= 4c8f029 (topup real-count).

Config = preregistered leg #2' spec (dispatch_s7135.py/§156): the census base
config with inside_cache=1 — 150000 live, seed 42, xmx 10G, fp 4, guard 1,
300s, all other levers 0.

A/B base = base-b (run 35317176927, head 4c8f029, all-zero levers, fp4/300s,
population 148391/148193/148027 stable, TPS 0.8-0.9 — structural twin of the
pre-fix census 35275967738).

Preregistered absorb gates: §156 — fixture green, ARMED markers, alloc
families (inside-blocks ↓>=30%, movement-geometry ↓>=25%), PalettedContainer.get
↓>=15% (base 3.06%), young GC down, entity phase <= +0.5pp.
"""
import json
import sys
import time
import urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
TOKEN = __import__("os").environ.get("CRUSSTY_GH_TOKEN", "")  # owner rule 1b


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
    token = TOKEN
    if not token:
        print("set CRUSSTY_GH_TOKEN (owner rule 1b)", file=sys.stderr)
        return 3

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
        "inside_cache": "1",
        "flush_diet": "0",
        "fluid_free": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
    }
    api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent (leg #2'' INSIDE-CACHE on fixed fixture); waiting...")
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
