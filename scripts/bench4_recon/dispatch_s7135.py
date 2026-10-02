#!/usr/bin/env python3
"""dispatch_s7135.py — dispatch the X150K A/B lever leg #1 (INSIDE-CACHE, S7-135/TASK-271).

Config mirrors the SUCCESS census run 35275967738 (S7-134b: 150000 live,
seed 42, xmx 10G, fp 4, guard 1, 300s, demux 0, diet 0) with
inside_cache=1. The A/B delta isolates the lever on the same scene
against the FRESH master base (35275967738, profiler v3 — alloc/wall
profiles trustworthy).

Preregistered absorb gates (next tick, S7-135b):
  (1) fixture gates green (INJECT DONE 150000/150000, VALID, alive 4/4,
      0 tick-behind) — behavioral parity on the live scene;
  (2) ARMED markers: pristine sighting Entity, defined InsideBlockOps(+Recorder),
      computed patch Retargeted{1}, hook serve, retransform rc=0;
  (3) PRIMARY alloc lanes (equal windows, S7-96d): inside-blocks family
      (LongOpenHashSet/BlockPos$6-lambdas/visitor paths) down >= 30%;
      movement-geometry family (collidedWithFluid/AABB paths) down >= 25%;
  (4) PRIMARY cpu lane: PalettedContainer.get down >= 15% (base 3.1%);
  (5) young GC count down (base ~14/60s window); high-water no growth >10%;
  (6) entity phase: no regression >0.5pp.
Partial pass (one of 3/4) => calibration leg #2; miss => REFUTED-BY-ECONOMICS,
default 0 (fail-closed vanilla); pass => leg #2 min-of-2 by cpu pairs.
"""
import json, re, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_creds(path="~/.git-credentials"):
    import os
    line = open(os.path.expanduser(path)).read().strip().splitlines()[0]
    return re.match(r"^https://[^:]+:([^@]+)@github\.com$", line).group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
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
    tok = token_from_creds()

    # concurrency guard: any in-flight world-bench run? (law S7-108)
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
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
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST", data={"ref": "master", "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
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
