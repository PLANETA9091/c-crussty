#!/usr/bin/env python3
"""dispatch_s7153.py — dispatch the X150K FLUID-DIRTY A/B leg (S7-153/TASK-292).

Preregistered protocol (GOAL СТАТУС S7-150 §4, S7-152 §4): A/B min-of-2
against the CUMULATIVE green baseline 35330129145 (inside_cache=1 +
flush_diet=1, fp4/300s/150k/seed42/xmx10G, radius 640). The leg adds
fluid_dirty=1 (lever #6: fluid-scan memoization via FluidPushOps.scan +
event-driven dirty-stamp ledger on LevelChunk.setBlockState) on top of the
CUMULATIVE config — everything else identical.

Preregistered absorb gates §S7-150 (next tick, S7-153b):
  G1 fluid-push family CPU ↓>= 60% at hit-rate >= 80%;
  G2 0 NoClassDefFoundError + ARMED markers (FluidPushOps defined,
     Retargeted{sites:2 entity + 1 secWrite}, serve, rc=0);
  G3 population parity vs baseline twins (<= 1% band);
  G4 TPS not below baseline min-of-2 (regress > 10% => REFUTED, the
     demux anti-lesson);
  G6 young GC count not above baseline.
G5 (OFFLINE lockstep) already PASSED in S7-152.
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
        "fluid_dirty": "1",
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
