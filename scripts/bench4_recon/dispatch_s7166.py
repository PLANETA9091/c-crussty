#!/usr/bin/env python3
"""dispatch_s7166.py — dispatch the #13-SBB SKIP-STORE-BB v5 CANDIDATE leg
(TASK-319, S7-166) on top of BANKED CUMULATIVE v3.

The leg config is the EXACT v3 bank (run 35399980345: inside_cache=1 +
flush_diet=1 + region_threads=4 + batch_collector=1, everything else 0,
fp4/300s/150k/seed42/xmx10G/r640) PLUS skip_store_bb=1 AND recon_diag=1.

recon_diag=1 is REQUIRED by the preregistered measurement-by-effect gate
(TASK-318): the remset dirty-cards A/B against the base leg 35425246662
(same identical recon_diag overhead) directly measures the bb share of the
old->young store-firehose. xmx stays 10G — the A/B base ran xmx10G, any
heap change would invalidate the preregistered comparison (the owner's new
2.5GB free-hosting profile is a SEPARATE track, planned after this
verdict).

Preregister #13-SBB gates (TASK-318, fixed BEFORE implementation):
  PG2 delivery: "stage sbb composed (Retargeted { sites: 1 })" +
    "skip_store_ops: defined net/minecraft/world/level/SkipStoreOps" +
    ARMED chain containing sbb + 0 NCDFE + pop 150k VALID
  PG3 TPS last-5 median >= 1.60
  PG4a remset dirty-cards/cycle p50 -10%+ (expectation 10-25%)
  PG4b card-set/remset CPU lane -10%+
  PG4c AABB allocation share by count -20%+ (base 16.75%)
  CRASH-FREE 0 crash / 0 Full GC / noise <= 5
  PARITY oracle >= 1M bit-exact (DONE offline: 1,100,000 PASS,
    skipped=40000, 0 mismatches — bit-parity + identity invariant +
    zero-sign strictness)
  Banking: PASS -> CUMULATIVE v5 = v3 + skip_store_bb=1;
           FAIL -> REFUTED + rollback (ops stays infrastructure).

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
        "skip_store_bb": "1",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
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
