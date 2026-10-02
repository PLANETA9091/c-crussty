#!/usr/bin/env python3
"""dispatch_s7163.py — dispatch the X150K S7-163 leg (FLAT-TRAVERSAL,
lever #9) on top of the BANKED CUMULATIVE v3.

Preregistered protocol: single A/B leg against the BANKED CUMULATIVE v3
(leg #2 = 35399980345: inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1, fp4/300s/150k/seed42/xmx10G, radius 640). The leg
config is the v4 CANDIDATE: v3 + flat_traversal=1, delivered through the
entity_compose stage 6:
  - TraverseOps.forEachFlat replaces the guava-iterator orchestration of
    BlockGetter.forEachBlockIntersectedBehind (the single invokestatic
    site in Entity.checkInsideBlocks(Vec3,Vec3,SBC,LongSet,int)I);
  - bit-exactness: TraverseLockstepHarness 60k+ scenarios (random +
    stationary boundary + sign-zero axes + aligned coords + marches +
    clip-empty cells) x 4 visitor policies, (posLong, step) sequence +
    return bit-in-bit; the getFurthestCorner last-component sign was
    caught and fixed by the harness (reflection oracle);
  - AABB.clip static stays VERBATIM (parity-first: only the iteration
    layer is replaced; inline-clip is a later squeeze within the lever).

Verification: cargo suite 155/0/1 (+5 traversal tests incl. full-chain
composition idempotency); TraverseLockstepHarness OFFLINE PASS.

Preregistered absorb gates (vs leg #2 = CUMULATIVE v3):
  PG2  0 NCDFE + population 150000 VALID
       + "entity_compose: ARMED chain [inside->rng->batch->traversal],
         retransform rc=0"
       + "region_threads: ARMED ... ChunkMap=0"
       + "traverse_ops: defined" + telemetry lines present
       + "batch_collector: defined" (v3 stages intact);
  PG3  TPS last-5 median >= 1.60 (worst banked leg); expectation
       +2..+8% (orchestration tail removal + LongOpenHashSet death);
  PG4  traversal-lane (forEachBlockIntersectedBetween family) reduced
       >= 50% per-work; orchestration tail (guava + BlockPos iterators +
       LongOpenHashSet init) reduced >= 70%; young GC <= bank level (154);
  CRASH-FREE: 0 tracker-NPE, 0 uuid-dup, navigatingMobs watchlist.
Banking rule: full PASS -> CUMULATIVE v4 = v3 + flat_traversal=1;
FAIL -> REFUTED + rollback flat_traversal=0 (TraverseOps stays as
infrastructure either way). INJECTS-ONLY: 1 CI-leg = preregister A/B
(sanctioned).
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
        "flat_traversal": "1",
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
