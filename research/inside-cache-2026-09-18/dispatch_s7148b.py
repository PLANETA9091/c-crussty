#!/usr/bin/env python3
"""dispatch_s7148b.py — dispatch FLUSH-DIET leg (S7-148b, lever #4) on head
>= 1864e3d (self-contained bridge + deficit-driven topup).

Config = preregistered FLUSH-DIET spec (§S7-138): census base config with
flush_diet=1 — 150000 live, seed 42, xmx 10G, fp 4, guard 1, 300s, all other
levers 0. A/B base = base-b (35317176927, head 4c8f029, all-zero, fp4/300s).
§S7-138 gates + S7-148 protocol: 0 NCDFE, population ~148k stable whole soak.
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
    if not TOKEN:
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
        "inside_cache": "0",
        "flush_diet": "1",
        "fluid_free": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
    }
    api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent (FLUSH-DIET leg); waiting...")
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
