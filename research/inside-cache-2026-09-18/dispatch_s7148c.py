#!/usr/bin/env python3
"""dispatch_s7148c.py — dispatch FLUID-FREE leg (S7-148c, lever #5) on head
>= d2f063e.

Config = preregistered FLUID-FREE spec (§S7-139): fluid_free=1 + paletted_demux=1
(ff verdict cache requires the demux substrate, S7-143) + inside_cache=1 (the
Entity chain owner — compose_entity applies the fgate retarget on top of the
inside-patched Entity bytes, inside_cache.rs) — 150000 live, seed 42, xmx 10G,
fp 4, guard 1, 300s, alloc_diet=0, flush_diet=0 (isolated leg).

A/B base = base-b (35317176927, head 4c8f029, all-zero, fp4/300s).
§S7-139 gates + S7-148 protocol: fluid-share of PalettedContainer.get ↓>=60%,
entity phase <= +0.5pp, 0 NCDFE, population ~148k stable whole soak.
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
        "paletted_demux": "1",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "0",
        "fluid_free": "1",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
    }
    api(f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent (FLUID-FREE leg, demux=1, inside_cache=1 owner); waiting...")
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
