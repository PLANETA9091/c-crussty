#!/usr/bin/env python3
"""dispatch_s7177.py - STEAL v2 defect-fix gate leg (TASK-335, S7-168 BU-DEFER).

s7176 (run 35452002378) CRASH root-cause: a region worker tics an entity whose
travel hits FarmBlock.turnToDirt -> Level.setBlockAndUpdate ->
ServerLevel.sendBlockUpdated; the vanilla body iterates ServerLevel
.navigatingMobs (fastutil ObjectOpenHashSet) under a latch that in vanilla
only the Server thread can hold -> worker-vs-main race ->
ObjectOpenHashSet$SetIterator NPE ("wrapped is null") -> the main thread dies
on the same set (ReportedException "Exception while updating neighbours") ->
graceful shutdown mid-soak (uptime 197s).

FIX (S7-168 BU-DEFER): the single ServerLevel.sendBlockUpdated body redirects
to BlockUpdateOps.handle (javap-verbatim vanilla body, codelen=235 contract).
Worker threads execute the order-safe inline prefix (blockChanged +
pathTypesByPosCache.invalidate + shapes-delta gate) and DEFER only the
navigatingMobs pass; the main thread replays it FIFO in RegionTickOps phase 4
(after the DONE barrier, same discipline as the EntityCallbacks Mut queue).

Offline gates ALREADY PASSED: cargo 195/195 (6 new BU-DEFER guards:
no-nested-classes, single classfile, embedded bytes match, resolution
closure, redirect table = exactly sendBlockUpdated, REAL-ServerLevel-fixture
retarget sites:1 + idempotence); release build PASS; sendBlockUpdated
disassembly contract via javap_lite.py.

PREREGISTER GATES (absorb; bank base = CUMULATIVE v3 median 1.80):
  PG-V2 delivery: 0 NCDFE + pop 150k VALID + "bu_defer: 1" in run-env
    + "region_steal: 1" + "BU-DEFER composed" marker in stdout
  PG-V2a CRASH-FREE vs s7176 incident: 0 sendBlockUpdated NPE, 0
    "Encountered an unexpected exception", server ALIVE at soak end
    (alloc profiler window completes normally)
  PG-V2b DONE-park kill: main-thread wall samples on CyclicBarrier
    <= 40/900 (base 121/901 = 13.4%, RECON-16 methodology)
  PG-V2c TPS: last-5 median >= 1.98 (bank 1.80 + 10% GREEN bar). RECON-15
    arithmetic predicts ~+4-6% from park-kill alone; if TPS < 1.98 the
    lever is REFUTED-by-TPS and rolled back (bu_defer=0 + region_steal=0) -
    the lane is NOT closed (v7).
  PG-V2d GC sanity: young <= 174 (s7169 clean-base), 0 Full
Banking: PASS  -> CUMULATIVE v4 = v3 + region_steal=1 + bu_defer=1;
         FAIL  -> rollback (both 0), lane stays open (v7).
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
        "bu_defer": "1",
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
