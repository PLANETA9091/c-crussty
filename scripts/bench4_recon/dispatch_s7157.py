#!/usr/bin/env python3
"""dispatch_s7157.py — dispatch the X150K REGION-THREADS A/B leg (S7-157).

Preregistered protocol (GOAL СТАТУС S7-155 §6 + S7-156 §5): A/B min-of-2
against the CUMULATIVE green baseline 35330129145 (inside_cache=1 +
flush_diet=1, fp4/300s/150k/seed42/xmx10G, radius 640). The leg adds
region_threads=4 (lever #7: region-threaded entity ticking via
RegionTickOps — W=4 quadrants over 8-chunk regions, 3 persistent TickThread
helpers + main, GO/DONE barriers, deferred FIFO EntityCallbacks) on top of
the CUMULATIVE config — everything else identical. PG1 lockstep PASSED
(research/region-threads-2026-09-18/PG1_LOCKSTEP_S7157.md).

Preregistered absorb gates (next tick, S7-157b):
  PG2 0 NoClassDefFoundError + ARMED markers (RegionTickOps defined in
      kernel loader, ServerLevel Retargeted{1}, EntityCallbacks
      Retargeted{1}+{1}, serve, retransform rc=0) + population twin 150k;
  PG3 TPS >= +25% vs CUMULATIVE (Amdahl ceiling x2.31); REFUTED if < +10%;
  PG4 young GC count <= baseline + 15% (<= 135 vs 118).
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
