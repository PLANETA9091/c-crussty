#!/usr/bin/env python3
"""dispatch_s7171.py — re-dispatch of the ZERO-CURSOR lever #11 v1 gate leg (TASK-331): s7170 died pre-server on run_world3.sh line 161 unbound $6 (literal BlockPos$6 in double-quoted echo under set -u); fixed 5686c5b, same inputs, same gates
on top of BANKED CUMULATIVE v3 (head 5686c5b).

Lever: pooled bit-exact betweenCornersInDirection iterator. The redirected
factory BlockPos.lambda$betweenCornersInDirection$8 -> ZeroCursorOps.lambda8
returns a pooled, per-call-reset ZeroCursorIter (single classfile bridge pair
defined into the kernel loader). The vanilla walk allocates one BlockPos$6 +
one MutableBlockPos per betweenCornersInDirection call — one per entity per
tick per travel step; fresh profile s7169: cursor family = 29.18% of ALL
alloc samples / 6.42% CPU (the largest attackable sub-lane of the TOP-1
entity-tick-core 34.04% CPU / 68.20% alloc). Bit-exactness: offline
CursorLockstepHarness 350k scenarios / 31.7M positions identical vs the real
vanilla lambda (the redirected factory receives the same untouched 314B
computed arguments).

Workflow: world-bench.yml input zero_cursor (added TASK-330) -> env
ZERO_CURSOR -> CRUSSTY_ZERO_CURSOR (run_world3.sh). fluid_free input DROPPED
(25-input GitHub limit; requires paletted_demux=1 which never armed).

PREREGISTER GATES (absorb; A/B vs fresh same-bank profile s7169 = 35437243128; s7170 run 35440054574 = delivery-DEFECT, no data
and banked v3 legs):
  PG-Z2 delivery: 0 NCDFE + pop 150k VALID
    + "[crussty-plugin] zero_cursor: pristine sighting net/minecraft/core/BlockPos"
    + "zero_cursor: defined net/minecraft/core/ZeroCursorIter + net/minecraft/core/ZeroCursorOps in kernel loader"
    + "zero_cursor: computed redirect for net/minecraft/core/BlockPos (... Retargeted { sites: 1 })"
    + "zero_cursor: net/minecraft/core/BlockPos armed, retransform rc=0"
    + "region_threads: ARMED ... ChunkMap=0" + "batch_collector: defined"
  PG-Z3 TPS: last-5 median >= 1.60 (bank harm floor); report delta vs banked
    v3 median 1.80 — the LEVER BOOST verdict comes from this delta.
  PG-Z4a cursor-alloc lane: (BlockPos$6 | betweenCornersInDirection |
    forEachBlockIntersectedBetween family share of alloc samples) >= -50%
    per-work vs s7169 base 29.18% (expect -80..-95% on the factory kill;
    LongSet/Vec3 remain in the lane)
  PG-Z4b young GC <= 154 (bank v3); 0 Full GC
  PG-Z4c cursor-CPU lane: 6.42% baseline, report delta (expect >= -30%)
  CRASH-FREE: 0 crash / 0 Full, "Entity threw exception" <= 5/run
Banking: full PASS -> CUMULATIVE v4 = v3 + zero_cursor=1;
         FAIL          -> REFUTED + rollback zero_cursor=0 (bridges remain
         infrastructure). INJECTS-ONLY, 1 sanctioned CI gate leg.
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
        "zero_cursor": "1",
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
