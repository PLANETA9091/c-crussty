#!/usr/bin/env python3
"""dispatch_s7176.py — STEAL lever #13 v1 gate leg (TASK-333, RECON-15):
chunked work-stealing queue replaces static region buckets.

RECON-15 (fresh clean s7169 wall profile, 900 Server-thread samples):
static-bucket phase 3 measured main bucket-0 478 wall vs 651 per helper
(total work 2432, critical path 651 => mean parallelism 3.74/4 = 93%) and
the main thread PARKED on the DONE barrier 121/900 = 13.4% of its wall —
static quadrant partitioning is workload-unaware. Lever: ONE shared
snapshot array (exact vanilla iteration order) + AtomicInteger chunk cursor
(STEAL_CHUNK=512); main and helpers all pull chunks until exhausted, so
DONE-park -> ~0 and the critical path drops to ~total/4 (~608). Per-entity
logic untouched (vanilla consumer bit-for-bit); intra-chunk order = snapshot
order; cross-chunk interleaving = the parity class already accepted for
region ticks (median-exact owner bar, S7-155 boundary).

Offline gates ALREADY PASSED (local): RegionLockstepHarness extended with a
W=4+STEAL child — LOCKSTEP PG1 PASS: W=1 == W=2 == W=4 == W=4+STEAL digests
(60 ticks, deferred adds/removals through real retargeted guard sites).

PREREGISTER GATES (absorb; bank base = CUMULATIVE v3 median 1.80):
  PG-S2 delivery: 0 NCDFE + pop 150k VALID
    + "region_threads: ARMED" + "batch_collector: defined"
    + run-env "region_steal: 1"
    + worker thread names crussty-region-worker-1..3 alive (GC log/stdout
      evidence or profile)
  PG-S3a DONE-park kill: Server-thread wall samples on
    CyclicBarrier.await (RegionTickOps DONE) <= 40/900 (base 121)
  PG-S3b balance: helper-vs-main work spread (max-min)/mean <= 0.20
    (base: 651 vs 478 => (651-478)/605 = 0.29)
  PG-S4 TPS: last-5 median >= 1.98 (bank 1.80 + 10% GREEN bar, v7 "ТОП-1
    ОБЯЗАН УПАСТЬ"); report full delta vs 1.80. NOTE: RECON-15 arithmetic
    predicts ~+4-6% from park-kill alone; if TPS < 1.98 the lever is
    REFUTED-by-TPS and rolled back — the lane is NOT closed (v7), next
    = RECON-16 on the remset/seriæ attribution axis.
  PG-S5 GC sanity: young <= 174 (s7169 clean-base), 0 Full
  CRASH-FREE: 0 crash / 0 Full, "Entity threw exception" <= 5/run
Banking: PASS  -> CUMULATIVE v4 = v3 + region_steal=1;
         FAIL  -> REFUTED + rollback region_steal=0 (bridge = infrastructure).
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

    # remote-head == local-head law (S7-164 dispatch incident): push BEFORE dispatch
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/commits/master").get("sha", "")
    if remote[:12] != local[:12]:
        print(f"HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    # concurrency guard: any in-flight world-bench run? (law S7-108)
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
        "paletted_demux": "0",
        "alloc_diet": "0",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "flat_traversal": "0",
        "zero_alloc": "0",
        "skip_store_bb": "0",
        "region_steal": "1",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "recon_diag": "0",
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
