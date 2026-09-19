#!/usr/bin/env python3
"""dispatch_s7164.py — dispatch the X150K S7-163 leg (FLAT-TRAVERSAL,
lever #9) on top of the BANKED CUMULATIVE v3.

Preregistered protocol: single A/B leg against the BANKED CUMULATIVE v3
(leg #2 = 35399980345: inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1, fp4/300s/150k/seed42/xmx10G, radius 640). The leg
config is the v4 CANDIDATE: v3 + zero_alloc_inside=1 (flat_traversal=0,
rollback after the S7-163 PG4a FAIL), delivered through the
entity_compose stage 7:
  - METHOD-BODY redirects (receiver-prepended invokestatic,
    length-preserving): Entity.collidedWithFluid /
    collidedWithShapeMovingFrom / updateFluidHeightAndDoFluidPushing ->
    ZeroAllocOps scalar bodies (allocation-free, bit-exact double math).
  - Census (v3 profile): collidedAlongVector is called ONLY from
    collidedWithShapeMovingFrom and collidedWithShapeMovingFrom ONLY from
    collidedWithFluid — three Entity redirects capture the whole lane.
  - bit-exactness: ZeroAllocLockstepHarness 350k scenarios (50k
    EntityDimensions.makeBoundingBox + 300k AABB.collidedAlongVector with
    the full getCenter/add/inflate/contains/clip ladder, random +
    degenerate) OFFLINE PASS bit-in-bit; the clipPoint t-sign was caught
    and fixed by the harness (javap dcmpg ladder, lesson-of-#9 repeat).
    Redirected Entity.class verified by the HotSpot verifier (defineClass).
  - FlowingFluid.getFlow stays vanilla (one Vec3 per wet block — the
    named NEXT layer, v2 of the lever if it survives the fresh TOPring).

Verification: cargo suite 166/0/1 (+6 zeroalloc tests: 3-redirect
composite+verify, idempotency, NotFound-no-mutation, receiver-shape
enforced, slot accounting incl. cat2 doubles, emit-for-harness);
ZeroAllocLockstepHarness OFFLINE PASS (350k).

Preregistered absorb gates (vs leg #2 = CUMULATIVE v3, fresh profile
35399980345 = 128124 samples; lesson #6 calibration):
  PG2  0 NCDFE + population 150000 VALID
       + "entity_compose: stage zeroin composed (Retargeted { sites: 3 })"
       + "entity_compose: ARMED chain [inside->rng->batch->zeroin],
         retransform rc=0"
       + "zero_alloc_ops: defined" + "region_threads: ARMED ... ChunkMap=0"
       + "batch_collector: defined" + telemetry lines present;
  PG3  TPS last-5 median >= 1.60 (bank gate);
  PG4a collided lane (collidedWithFluid + collidedWithShapeMovingFrom +
       collidedAlongVector + FluidState/Fluid.getAABB under the chain;
       per-work vs v3 3829 samples: 1749+687+406+987) reduced >= 50%
       (expectation -70..-80%: List.of, fluid AABB, makeBoundingBox,
       subtract, getCenter/add/inflate/clip all scalarized; getHeight/
       getFluidState time remains);
  PG4b fluid-push lane (updateFluidHeightAndDoFluidPushing family,
       per-work vs v3 13523) reduced >= 5% (expectation -5..-10% direct;
       deflate + Vec3 scale/add/normalize chains + part of <init>
       eliminated; getFlow 1.85% and vanilla block reads remain);
  PG4c young GC <= 154 (bank) AND alloc family AABB+Vec3 share of total
       allocations <= 36.1% (v3 = 40.08%; -10% relative; expectation
       -20..-30% relative: fluid AABBs + collidedAlongVector Vec3s +
       updateFluid Vec3 chains die);
  CRASH-FREE 0 crash / 0 Full GC; "Entity threw exception" <= historical
       parallel-tick noise band (0-5/run, s7160/61/63 precedent).
Banking rule: full PASS -> CUMULATIVE v4 = v3 + zero_alloc_inside=1;
FAIL -> REFUTED + rollback zero_alloc_inside=0 (ZeroAllocOps stays as
verified infrastructure either way). INJECTS-ONLY: 1 CI-leg =
preregister A/B (sanctioned).
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
        "flat_traversal": "0",
        "zero_alloc": "1",
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
