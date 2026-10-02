#!/usr/bin/env python3
"""dispatch_s7167.py — dispatch the FREE-HOST 2.5GB PROFILE leg (TASK-321, S7-167)
on top of BANKED CUMULATIVE v3.

Owner directive (tick 14:43 +08, recorded GOAL section 5): all tests and
optimizations must target the free-hosting profile — 2.5GB shared RAM
container (FalixNodes free class), real JVM budget ~2G heap. This leg is the
RECON phase of the two-phase track: profile the SAME living scene
(fp4/300s/150k/seed42/r640) under server_xms=2G + server_xmx=2G with
recon_diag=1 (JFR + remset/refine debug — pure observability), then re-sort
the TOP under low-heap and attack the next TOP-1 from there.

Leg config: EXACT v3 bank (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1, everything else 0), skip_store_bb=0 (#13-SBB REFUTED
TASK-320), server_xms=2G + server_xmx=2G (NEW: run_world3.sh parameterized
-Xms this tick — historical legs keep default 4G bit-exact).

Preregister (fixed BEFORE dispatch, RECON leg — NOT a banking gate):
  PG-A delivery: pop 150k VALID + ARMED [inside->rng->batch] (no sbb) +
    0 NCDFE + run-env echoes server_xms/server_xmx = 2G/2G
  Honest window (a): OOM / GC death spiral -> INFEASIBLE-BY-MEMORY verdict:
    the 150k-scale scene cannot fit the 2.5GB container class; gc.log
    evidence required (phase of death, full-GC thrash, allocation-fail rate).
    This is a VALID finding, not a tool failure.
  Honest window (b): run completes -> fresh TOP re-sort under low-heap:
    young-GC count/duty vs the 10G diagnostic base 35425246662, card-set and
    alloc lane shares (JFR), TPS series -> next TOP-1 target.
  NO banking; the future FREE-HOST A/B base = this leg itself.

S7-108: single-leg-in-flight guard; remote-head == local-head before POST.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token_from_remote():
    """Extract the PAT from the origin remote URL (rule 1b setup)."""
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
        "zero_alloc": "0",
        "skip_store_bb": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "2G",
        "server_xms": "2G",
        "recon_diag": "1",
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
