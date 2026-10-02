#!/usr/bin/env python3
"""dispatch_s7134.py — S7-134/TASK-270 profiling-census dispatch (base scene).

Purpose: infrastructure validation + the FIRST TRUE alloc profile of the
X150K scene. The v2 harness bug (asprof `dump` does not stop the session)
meant wall/alloc events never ran — every past "alloc-collapsed.txt" was a
cumulative CPU re-dump (ap.log: 3x "already started"). v3 is stop-based.

Config = the X150K SUCCESS baseline scene (35245032701 family: 150000 live,
seed 42, xmx 10G, fp 4, guard 1, 300s) with ALL lever gates at default 0
(demux=0, diet=0) — a clean master census, not an A/B lever leg.

Preregistered absorb gates (next tick):
  (1) ap.log clean — no "already started" / "not active" errors;
  (2) alloc-collapsed leaf signature = allocation sites (<init>-dominated,
      NOT G1 oop closures / C2 compiler frames);
  (3) alloc bytes sanity — order of hundreds of GB over the 60s alloc window
      (S7-133b structural estimate: hundreds of MB/tick churn);
  (4) wall differs from cpu (GC/waiting threads present in wall only);
  (5) fixture gates green (INJECT DONE 150000/150000, FIXTURE-VALIDITY: VALID,
      alive 4/4, 0 tick-behind).
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
