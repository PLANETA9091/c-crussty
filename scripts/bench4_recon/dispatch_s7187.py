#!/usr/bin/env python3
"""dispatch_s7187.py - lever #14 TRAVEL-ALLOC-DIET v1 leg (TASK-341).

RECON-20: s7180 (run 35460026013) = CRASH-REFUTED — the BU-DEFER phase-4 replay
itself hit the s7176 race family (NPE fastutil SetIterator "wrapped is null" at
BlockUpdateOps.vanilla:117 <- RegionTickOps.drainDeferredBlockUpdates:168 <- stealTick:352,
44s post-inject). Second strike on the STEAL v2 lane (+ s7177 REFUTED-by-TPS + RECON-17
ceiling < +10%) -> lane CLOSED, rollback (yml defaults already 0).

Pivot per RECON-17/20: TOP-1 CPU axis = GC/JVM-G1 36.23% (alloc churn of entity work;
recon20_top_s7178.py). Lever #14 TRAVEL-ALLOC-DIET v1 = bank v3 + zero_alloc=1
(S7-164 ZeroAllocOps scalar bodies: collidedWithFluid / collidedWithShapeMovingFrom /
updateFluidHeightAndDoFluidPushing) + skip_store_bb=1 (S7-166 value-equal setBoundingBox
store-skip). region_steal=0, bu_defer=0.

DUAL BAR (owner directive 2026-09-20): GREEN requires BOTH speed-normalized AND
absolute median5 TPS delta >= +10% vs the same-class anchor (min-of-2 preserved).

PAIRING LAW (S7-96d): cpu_band_min/max = 7800000..9200000 -> first-step fast-fail on
slow-class runners (~1 min discard) instead of an 11-min unpairable soak. Anchor:
s7178 median5 1.40 @ runner 8493973 (fast class).

PREREGISTER GATES (absorb_s7182.py):
  PG-T1 delivery: zero_alloc=1 + skip_store_bb=1 + region_steal=0 + bu_defer=0 +
    inside_cache=1 + flush_diet=1 + region_threads=4 + batch_collector=1,
    NCDFE=0, pop 150k VALID
  PG-T2 crash-free: 0 threw/unexpected/NPE (incl. the s7180 BlockUpdateOps
    incident class MUST be absent) + soak TPS polls >= 3
  PG-T3 DUAL-BAR paired TPS: runner within ±5% of 8493973 (band-gated upstream);
    normalized delta = (med/runner)/(1.40/8493973) AND absolute delta = med/1.40;
    BOTH >= +10% -> candidate GREEN -> confirming leg s7183 (min-of-2) -> banking v4;
    else lane open -> #14 v2 (javap recon makeBoundingBox/Vec3.add/AABB.inflate ->
    TravelDietOps redirects)
  PG-T4 GC sanity: young <= 174, 0 Full
  PG-T5 DONE-park N/A tolerated (AP-PID defect precedent s7178)
INJECTS-ONLY, 1 sanctioned CI gate leg.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
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

    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("in_progress", "queued", "waiting"):
            print(f"concurrency guard: run {r['id']} is {r['status']} — dispatch blocked")
            return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "travel_diet": "1",
        "flat_traversal": "0",
        "zero_alloc": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
        "cpu_band_min": "6350000",
        "cpu_band_max": "7010000",
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
