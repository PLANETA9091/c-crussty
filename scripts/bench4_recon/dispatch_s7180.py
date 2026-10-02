#!/usr/bin/env python3
"""dispatch_s7180.py - STEAL v2 PAIRED leg RETRY vs s7178 anchor (TASK-339).

s7179 (run 35459232032) = INFRA-FLAKE: the server thread WEDGED in
ServerChunkCache.syncLoad (chunk (-30,12), managedBlock yield loop) 11s after
"Done" - watchdog x5 (10..30s), the population inject started from console
into the wedged server, workflow stopped it at 17:56:56; BENCH-4 fixture
INVALID (no polls possible) -> gate exit 1. NO lever evidence either way:
both levers arm post-startup and the soak never ran. Same wedge class is
absent from s7169/s7177/s7178 with the same bank config -> CI runner flake
(slow class 6.72M), not a regression. This leg retries the identical paired
protocol.

RECON-19: the bank re-cal leg s7178 (bank v3 config, runner 8493973) measured
console median5 = 1.40 vs s7169's 1.60 on the SAME config class and runner
class -> the TPS axis carries ~±12% leg-to-leg noise even after runner-speed
normalization. Single-pair deltas are only interpretable with the paired
protocol and the ±5% runner-class rule.

s7180 = region_steal=1 + bu_defer=1 (STEAL v2 / BU-DEFER infrastructure,
crash-free per s7177), everything else = bank v3. PAIRING RULE (absorb):
valid only if runner_cpu_index lands within ±5% of s7178's 8493973.

PREREGISTER GATES (absorb):
  PG-P1 delivery: 0 NCDFE + pop 150k VALID + bu_defer=1 + region_steal=1 +
    "BU-DEFER composed" marker (delivery chain proven on s7177)
  PG-P2 crash-free vs s7176 incident: 0 threw/unexpected/NPE markers,
    soak complete
  PG-P3 PAIRED TPS: runner_cpu_index within ±5% of 8493973 (else leg is
    invalid for pairing -> re-dispatch, no verdict); verdict metric =
    speed-normalized delta vs anchor s7178 (1.40 @ 8493973):
      >= +10% -> candidate GREEN -> confirmation leg s7180 required
      (min-of-2, both lever legs >= +10% -> CUMULATIVE v4 banking);
      < +10% -> lane stays open with lever #14 (TRAVEL-ALLOC-DIET)
  PG-P4 GC sanity: young <= 174, 0 Full
  PG-P5 DONE-park (if wall profile present; AP-PID defect on s7178 killed
    wall/alloc windows): expect ~1.4% class (s7177 evidence), N/A tolerated
    on profiler failure — park is structural evidence, not the verdict axis.
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
