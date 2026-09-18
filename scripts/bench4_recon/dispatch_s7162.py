#!/usr/bin/env python3
"""dispatch_s7162.py — dispatch the X150K S7-162 leg (single compose-chain +
ensure-retirement + INSTANCES telemetry).

Preregistered protocol: single A/B leg against the BANKED CUMULATIVE v2
(leg #5 = 35381522360: inside_cache=1 + flush_diet=1 + region_threads=4,
fp4/300s/150k/seed42/xmx10G, radius 640). The leg config is the
v3 CANDIDATE: v2 + batch_collector=1, delivered through the S7-162
architecture:
  - entity_compose: the SINGLE owner of the Entity byte pipeline —
    5 strict stages (inside -> fluid_free -> fluid_dirty -> rng -> batch)
    on ONE accumulated buffer, one hook, one retransform (the
    supersede mechanic of leg #5 886/895 is eliminated; the
    inside_cache Entity gate dead in v2 is REVIVED — declared bundle);
  - ensure RETIRED: run 35391679176 stack-proof — all 801
    BatchCollector.<init> samples came from the per-tick ensure loop
    (re-construction for pre-arm entities, swap never stuck) + the gate
    itself 737 samples; tickBucket is vanilla-identical again;
  - INSTANCES telemetry every 600 ticks (answers the ctor-count question
    with live data; "first-swap" marker NOT expected anymore).

Verification: cargo suite 150/0/1 (+ full-chain strict test on the real
Entity fixture); RegionThreadsHarness OFFLINE PASS; BatchCollectorHarness
PASS (4000 scenarios, flushStep queues bit-identical).

Preregistered absorb gates (vs leg #5 = CUMULATIVE v2):
  PG2  0 NCDFE + population 150000 VALID
       + "entity_compose: ARMED chain [inside->rng->batch], retransform rc=0"
       + "region_threads: ARMED ... ChunkMap=0"
       + "batch_collector: defined" + telemetry lines present
       + "ARMED first-swap" ABSENT (ensure retired);
  PG3  TPS last-5 median >= 1.60 (worst banked v2 leg); expectation
       +2..+10% (revived inside gate + collector methods);
  PG4''' young GC <= 180 AND collector-family per-work <= 1290
       (55% of leg #5 2345) AND infra tail (BatchCollector.<init> + ensure)
       <= 10% of the family (the S7-161 re-attack condition);
  CRASH-FREE: 0 tracker-NPE, 0 uuid-dup, navigatingMobs watchlist.
Banking rule: full PASS -> CUMULATIVE v3 = v2 + batch_collector=1
(entity_compose stays as infrastructure, not config); FAIL -> REFUTED
(final for the collector-family axis) + rollback batch_collector=0
(entity_compose stays as the hook-coexistence infrastructure fix either
way). INJECTS-ONLY: 1 CI-leg = preregister A/B (sanctioned).
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
        "batch_collector": "1",
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
