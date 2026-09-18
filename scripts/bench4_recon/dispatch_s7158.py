#!/usr/bin/env python3
"""dispatch_s7158.py — dispatch the X150K REGION-THREADS leg #3 (S7-158).

Preregistered protocol (GOAL СТАТУС S7-157c §4): A/B min-of-2 against the
CUMULATIVE green baseline 35330129145 (inside_cache=1 + flush_diet=1,
fp4/300s/150k/seed42/xmx10G, radius 640). Same inputs as legs #1/#2 — the
ONLY change is the FIXED BUILD: S7-158 hardening (banking prerequisites).
  S7-158a  bench harness: bounded console writes (FIFO open() hang) +
           report timeout (the 59-min post-artifact burn of leg #2);
  S7-158b  removal-safe tracker sweep (TrackerTickOps; live crash
           "entity is null" ChunkMap.java:1017 — main sweep vs worker
           removal swap-remove; retarget ChunkMap.tick()V Retargeted{1});
  S7-158c  GC diet of the RegionTickOps snapshot/partition (zero-alloc
           steady state: persistent grow-on-demand slot arrays, shared
           consumer, stale-tail hygiene; PG4 gate young GC <= 135);
  S7-158d  serialized UUID seeding (RngOps; live UUID-dup WARN — purpur
           entity-shared-random=true routes every entity ctor through ONE
           ThreadUnsafeRandom; retarget Entity ctor Retargeted{1}).
PG1 lockstep digest UNCHANGED (61e3c374...941d5); harness OFFLINE PASS
(structural/wiring/dormant/parallel + S7-158b sweep regression + S7-158d
2x2000 parallel UUID constructions 0 dups).

Preregistered absorb gates (next tick, S7-158 absorb):
  PG2 0 NoClassDefFoundError + ARMED markers (region_threads ×3 + ChunkMap
      Retargeted{1} + Entity Retargeted{1}, retransform rc=0) + population
      twin 150k;
  PG3 TPS >= +25% vs CUMULATIVE (leg #2 evidence: +66.7%); REFUTED < +10%;
  PG4 young GC <= 135 (base 118, cap +15%);
  CRASH-FREE: no tracker-race NPE, no UUID-dup WARN (watchlist: the
  navigatingMobs sendBlockUpdated race — contained by guardEntityTick;
  recurring > 1/hour => S7-159 bridge).
Banking rule (S7-157c §4): banking only on PG2+PG3+PG4 PASS without
crashes; leg #4 = the second min-of-2 sample on the same build.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    """Extract the PAT from the origin remote URL (rule 1b setup) —
    keeps this file secret-free (GitHub push protection, S7-153 lesson)."""
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL — run rule (1b) remote set-url first")
    return m.group(1)


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
    tok = token_from_remote()

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
        "flush_diet": "1",
        "fluid_free": "0",
        "fluid_dirty": "0",
        "region_threads": "4",
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
